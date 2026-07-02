#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_272(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign75510_e115067, assign75510_e115067_d_n0, assign75510_e115067_d_n2, assign75510_e115067_d_n4, assign75510_e115067_d_n5, assign75510_e115067_d_n6, assign75510_e115067_d_n7, assign75510_e115067_d_n8, assign75510_e115067_d_n9, assign75510_e115067_d_n10, assign75510_e115067_d_n11, assign75510_e115067_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ps0ld_ini__blk1773, locals.var_ps0ld_ini__blk1773_dn0, locals.var_ps0ld_ini__blk1773_dn2, locals.var_ps0ld_ini__blk1773_dn4, locals.var_ps0ld_ini__blk1773_dn5, locals.var_ps0ld_ini__blk1773_dn6, locals.var_ps0ld_ini__blk1773_dn7, locals.var_ps0ld_ini__blk1773_dn8, locals.var_ps0ld_ini__blk1773_dn9, locals.var_ps0ld_ini__blk1773_dn10, locals.var_ps0ld_ini__blk1773_dn11, locals.var_ps0ld_ini__blk1773_dn14,)
    }
};
        locals.var_ps0ld_ini__blk1773 = assign75510_e115067;
        locals.var_ps0ld_ini__blk1773_dn0 = assign75510_e115067_d_n0;
        locals.var_ps0ld_ini__blk1773_dn2 = assign75510_e115067_d_n2;
        locals.var_ps0ld_ini__blk1773_dn4 = assign75510_e115067_d_n4;
        locals.var_ps0ld_ini__blk1773_dn5 = assign75510_e115067_d_n5;
        locals.var_ps0ld_ini__blk1773_dn6 = assign75510_e115067_d_n6;
        locals.var_ps0ld_ini__blk1773_dn7 = assign75510_e115067_d_n7;
        locals.var_ps0ld_ini__blk1773_dn8 = assign75510_e115067_d_n8;
        locals.var_ps0ld_ini__blk1773_dn9 = assign75510_e115067_d_n9;
        locals.var_ps0ld_ini__blk1773_dn10 = assign75510_e115067_d_n10;
        locals.var_ps0ld_ini__blk1773_dn11 = assign75510_e115067_d_n11;
        locals.var_ps0ld_ini__blk1773_dn14 = assign75510_e115067_d_n14;

        let (assign75520_e115071, assign75520_e115071_d_n0, assign75520_e115071_d_n2, assign75520_e115071_d_n4, assign75520_e115071_d_n5, assign75520_e115071_d_n6, assign75520_e115071_d_n7, assign75520_e115071_d_n8, assign75520_e115071_d_n9, assign75520_e115071_d_n10, assign75520_e115071_d_n11, assign75520_e115071_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fbsq__blk1774, locals.var_fbsq__blk1774_dn0, locals.var_fbsq__blk1774_dn2, locals.var_fbsq__blk1774_dn4, locals.var_fbsq__blk1774_dn5, locals.var_fbsq__blk1774_dn6, locals.var_fbsq__blk1774_dn7, locals.var_fbsq__blk1774_dn8, locals.var_fbsq__blk1774_dn9, locals.var_fbsq__blk1774_dn10, locals.var_fbsq__blk1774_dn11, locals.var_fbsq__blk1774_dn14,)
    }
};
        locals.var_fbsq__blk1774 = assign75520_e115071;
        locals.var_fbsq__blk1774_dn0 = assign75520_e115071_d_n0;
        locals.var_fbsq__blk1774_dn2 = assign75520_e115071_d_n2;
        locals.var_fbsq__blk1774_dn4 = assign75520_e115071_d_n4;
        locals.var_fbsq__blk1774_dn5 = assign75520_e115071_d_n5;
        locals.var_fbsq__blk1774_dn6 = assign75520_e115071_d_n6;
        locals.var_fbsq__blk1774_dn7 = assign75520_e115071_d_n7;
        locals.var_fbsq__blk1774_dn8 = assign75520_e115071_d_n8;
        locals.var_fbsq__blk1774_dn9 = assign75520_e115071_d_n9;
        locals.var_fbsq__blk1774_dn10 = assign75520_e115071_d_n10;
        locals.var_fbsq__blk1774_dn11 = assign75520_e115071_d_n11;
        locals.var_fbsq__blk1774_dn14 = assign75520_e115071_d_n14;

        let (assign75530_e115082, assign75530_e115082_d_n0, assign75530_e115082_d_n2, assign75530_e115082_d_n4, assign75530_e115082_d_n5, assign75530_e115082_d_n6, assign75530_e115082_d_n7, assign75530_e115082_d_n8, assign75530_e115082_d_n9, assign75530_e115082_d_n10, assign75530_e115082_d_n11, assign75530_e115082_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75530_e115075: f64 = (2.0 * locals.var_beta_inv);
        let assign75530_e115078: f64 = (locals.var_nover_func / locals.var_nin);
        let assign75530_e115079: f64 = (assign75530_e115078).ln();
        let assign75530_e115080: f64 = (assign75530_e115075 * assign75530_e115079);
        (assign75530_e115080, (((2.0 * locals.var_beta_inv_dn0) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn2) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn4) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn5) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn6) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn7) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn8) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn9) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn10) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn11) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))), (((2.0 * locals.var_beta_inv_dn14) * assign75530_e115079) + (assign75530_e115075 * ((-((locals.var_nover_func * locals.var_nin_dn14) / (locals.var_nin * locals.var_nin))) / assign75530_e115078))),)
    } else {
        (locals.var_pb2over__blk1769, locals.var_pb2over__blk1769_dn0, locals.var_pb2over__blk1769_dn2, locals.var_pb2over__blk1769_dn4, locals.var_pb2over__blk1769_dn5, locals.var_pb2over__blk1769_dn6, locals.var_pb2over__blk1769_dn7, locals.var_pb2over__blk1769_dn8, locals.var_pb2over__blk1769_dn9, locals.var_pb2over__blk1769_dn10, locals.var_pb2over__blk1769_dn11, locals.var_pb2over__blk1769_dn14,)
    }
};
        locals.var_pb2over__blk1769 = assign75530_e115082;
        locals.var_pb2over__blk1769_dn0 = assign75530_e115082_d_n0;
        locals.var_pb2over__blk1769_dn2 = assign75530_e115082_d_n2;
        locals.var_pb2over__blk1769_dn4 = assign75530_e115082_d_n4;
        locals.var_pb2over__blk1769_dn5 = assign75530_e115082_d_n5;
        locals.var_pb2over__blk1769_dn6 = assign75530_e115082_d_n6;
        locals.var_pb2over__blk1769_dn7 = assign75530_e115082_d_n7;
        locals.var_pb2over__blk1769_dn8 = assign75530_e115082_d_n8;
        locals.var_pb2over__blk1769_dn9 = assign75530_e115082_d_n9;
        locals.var_pb2over__blk1769_dn10 = assign75530_e115082_d_n10;
        locals.var_pb2over__blk1769_dn11 = assign75530_e115082_d_n11;
        locals.var_pb2over__blk1769_dn14 = assign75530_e115082_d_n14;

        let (assign75540_e115090, assign75540_e115090_d_n0, assign75540_e115090_d_n2, assign75540_e115090_d_n4, assign75540_e115090_d_n5, assign75540_e115090_d_n6, assign75540_e115090_d_n7, assign75540_e115090_d_n8, assign75540_e115090_d_n9, assign75540_e115090_d_n10, assign75540_e115090_d_n11, assign75540_e115090_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75540_e115086: f64 = (0.8 - locals.var_pb2over__blk1769);
        let assign75540_e115088: f64 = (assign75540_e115086 - 0.1);
        (assign75540_e115088, (-locals.var_pb2over__blk1769_dn0), (-locals.var_pb2over__blk1769_dn2), (-locals.var_pb2over__blk1769_dn4), (-locals.var_pb2over__blk1769_dn5), (-locals.var_pb2over__blk1769_dn6), (-locals.var_pb2over__blk1769_dn7), (-locals.var_pb2over__blk1769_dn8), (-locals.var_pb2over__blk1769_dn9), (-locals.var_pb2over__blk1769_dn10), (-locals.var_pb2over__blk1769_dn11), (-locals.var_pb2over__blk1769_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign75540_e115090;
        locals.var_tmf1_dn0 = assign75540_e115090_d_n0;
        locals.var_tmf1_dn2 = assign75540_e115090_d_n2;
        locals.var_tmf1_dn4 = assign75540_e115090_d_n4;
        locals.var_tmf1_dn5 = assign75540_e115090_d_n5;
        locals.var_tmf1_dn6 = assign75540_e115090_d_n6;
        locals.var_tmf1_dn7 = assign75540_e115090_d_n7;
        locals.var_tmf1_dn8 = assign75540_e115090_d_n8;
        locals.var_tmf1_dn9 = assign75540_e115090_d_n9;
        locals.var_tmf1_dn10 = assign75540_e115090_d_n10;
        locals.var_tmf1_dn11 = assign75540_e115090_d_n11;
        locals.var_tmf1_dn14 = assign75540_e115090_d_n14;

        let (assign75550_e115098, assign75550_e115098_d_n0, assign75550_e115098_d_n2, assign75550_e115098_d_n4, assign75550_e115098_d_n5, assign75550_e115098_d_n6, assign75550_e115098_d_n7, assign75550_e115098_d_n8, assign75550_e115098_d_n9, assign75550_e115098_d_n10, assign75550_e115098_d_n11, assign75550_e115098_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75550_e115094: f64 = (4.0 * 0.8);
        let assign75550_e115096: f64 = (assign75550_e115094 * 0.1);
        (assign75550_e115096, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign75550_e115098;
        locals.var_tmf2_dn0 = assign75550_e115098_d_n0;
        locals.var_tmf2_dn2 = assign75550_e115098_d_n2;
        locals.var_tmf2_dn4 = assign75550_e115098_d_n4;
        locals.var_tmf2_dn5 = assign75550_e115098_d_n5;
        locals.var_tmf2_dn6 = assign75550_e115098_d_n6;
        locals.var_tmf2_dn7 = assign75550_e115098_d_n7;
        locals.var_tmf2_dn8 = assign75550_e115098_d_n8;
        locals.var_tmf2_dn9 = assign75550_e115098_d_n9;
        locals.var_tmf2_dn10 = assign75550_e115098_d_n10;
        locals.var_tmf2_dn11 = assign75550_e115098_d_n11;
        locals.var_tmf2_dn14 = assign75550_e115098_d_n14;

        let (assign75560_e115108, assign75560_e115108_d_n0, assign75560_e115108_d_n2, assign75560_e115108_d_n4, assign75560_e115108_d_n5, assign75560_e115108_d_n6, assign75560_e115108_d_n7, assign75560_e115108_d_n8, assign75560_e115108_d_n9, assign75560_e115108_d_n10, assign75560_e115108_d_n11, assign75560_e115108_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let (assign75560_e115106, assign75560_e115106_d_n0, assign75560_e115106_d_n2, assign75560_e115106_d_n4, assign75560_e115106_d_n5, assign75560_e115106_d_n6, assign75560_e115106_d_n7, assign75560_e115106_d_n8, assign75560_e115106_d_n9, assign75560_e115106_d_n10, assign75560_e115106_d_n11, assign75560_e115106_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign75560_e115105: f64 = (-locals.var_tmf2);
                (assign75560_e115105, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign75560_e115106, assign75560_e115106_d_n0, assign75560_e115106_d_n2, assign75560_e115106_d_n4, assign75560_e115106_d_n5, assign75560_e115106_d_n6, assign75560_e115106_d_n7, assign75560_e115106_d_n8, assign75560_e115106_d_n9, assign75560_e115106_d_n10, assign75560_e115106_d_n11, assign75560_e115106_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign75560_e115108;
        locals.var_tmf2_dn0 = assign75560_e115108_d_n0;
        locals.var_tmf2_dn2 = assign75560_e115108_d_n2;
        locals.var_tmf2_dn4 = assign75560_e115108_d_n4;
        locals.var_tmf2_dn5 = assign75560_e115108_d_n5;
        locals.var_tmf2_dn6 = assign75560_e115108_d_n6;
        locals.var_tmf2_dn7 = assign75560_e115108_d_n7;
        locals.var_tmf2_dn8 = assign75560_e115108_d_n8;
        locals.var_tmf2_dn9 = assign75560_e115108_d_n9;
        locals.var_tmf2_dn10 = assign75560_e115108_d_n10;
        locals.var_tmf2_dn11 = assign75560_e115108_d_n11;
        locals.var_tmf2_dn14 = assign75560_e115108_d_n14;

        let (assign75570_e115117, assign75570_e115117_d_n0, assign75570_e115117_d_n2, assign75570_e115117_d_n4, assign75570_e115117_d_n5, assign75570_e115117_d_n6, assign75570_e115117_d_n7, assign75570_e115117_d_n8, assign75570_e115117_d_n9, assign75570_e115117_d_n10, assign75570_e115117_d_n11, assign75570_e115117_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75570_e115112: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign75570_e115114: f64 = (assign75570_e115112 + locals.var_tmf2);
        let assign75570_e115115: f64 = (assign75570_e115114).sqrt();
        (assign75570_e115115, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign75570_e115115)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign75570_e115115)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign75570_e115117;
        locals.var_tmf2_dn0 = assign75570_e115117_d_n0;
        locals.var_tmf2_dn2 = assign75570_e115117_d_n2;
        locals.var_tmf2_dn4 = assign75570_e115117_d_n4;
        locals.var_tmf2_dn5 = assign75570_e115117_d_n5;
        locals.var_tmf2_dn6 = assign75570_e115117_d_n6;
        locals.var_tmf2_dn7 = assign75570_e115117_d_n7;
        locals.var_tmf2_dn8 = assign75570_e115117_d_n8;
        locals.var_tmf2_dn9 = assign75570_e115117_d_n9;
        locals.var_tmf2_dn10 = assign75570_e115117_d_n10;
        locals.var_tmf2_dn11 = assign75570_e115117_d_n11;
        locals.var_tmf2_dn14 = assign75570_e115117_d_n14;

        let (assign75580_e115127, assign75580_e115127_d_n0, assign75580_e115127_d_n2, assign75580_e115127_d_n4, assign75580_e115127_d_n5, assign75580_e115127_d_n6, assign75580_e115127_d_n7, assign75580_e115127_d_n8, assign75580_e115127_d_n9, assign75580_e115127_d_n10, assign75580_e115127_d_n11, assign75580_e115127_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75580_e115123: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign75580_e115124: f64 = (1.0 + assign75580_e115123);
        let assign75580_e115125: f64 = (0.5 * assign75580_e115124);
        (assign75580_e115125, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign75580_e115127;
        locals.var_t0_dn0 = assign75580_e115127_d_n0;
        locals.var_t0_dn2 = assign75580_e115127_d_n2;
        locals.var_t0_dn4 = assign75580_e115127_d_n4;
        locals.var_t0_dn5 = assign75580_e115127_d_n5;
        locals.var_t0_dn6 = assign75580_e115127_d_n6;
        locals.var_t0_dn7 = assign75580_e115127_d_n7;
        locals.var_t0_dn8 = assign75580_e115127_d_n8;
        locals.var_t0_dn9 = assign75580_e115127_d_n9;
        locals.var_t0_dn10 = assign75580_e115127_d_n10;
        locals.var_t0_dn11 = assign75580_e115127_d_n11;
        locals.var_t0_dn14 = assign75580_e115127_d_n14;

        let (assign75590_e115137, assign75590_e115137_d_n0, assign75590_e115137_d_n2, assign75590_e115137_d_n4, assign75590_e115137_d_n5, assign75590_e115137_d_n6, assign75590_e115137_d_n7, assign75590_e115137_d_n8, assign75590_e115137_d_n9, assign75590_e115137_d_n10, assign75590_e115137_d_n11, assign75590_e115137_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75590_e115133: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign75590_e115134: f64 = (0.5 * assign75590_e115133);
        let assign75590_e115135: f64 = (0.8 - assign75590_e115134);
        (assign75590_e115135, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_vbs_max_over__blk1770, locals.var_vbs_max_over__blk1770_dn0, locals.var_vbs_max_over__blk1770_dn2, locals.var_vbs_max_over__blk1770_dn4, locals.var_vbs_max_over__blk1770_dn5, locals.var_vbs_max_over__blk1770_dn6, locals.var_vbs_max_over__blk1770_dn7, locals.var_vbs_max_over__blk1770_dn8, locals.var_vbs_max_over__blk1770_dn9, locals.var_vbs_max_over__blk1770_dn10, locals.var_vbs_max_over__blk1770_dn11, locals.var_vbs_max_over__blk1770_dn14,)
    }
};
        locals.var_vbs_max_over__blk1770 = assign75590_e115137;
        locals.var_vbs_max_over__blk1770_dn0 = assign75590_e115137_d_n0;
        locals.var_vbs_max_over__blk1770_dn2 = assign75590_e115137_d_n2;
        locals.var_vbs_max_over__blk1770_dn4 = assign75590_e115137_d_n4;
        locals.var_vbs_max_over__blk1770_dn5 = assign75590_e115137_d_n5;
        locals.var_vbs_max_over__blk1770_dn6 = assign75590_e115137_d_n6;
        locals.var_vbs_max_over__blk1770_dn7 = assign75590_e115137_d_n7;
        locals.var_vbs_max_over__blk1770_dn8 = assign75590_e115137_d_n8;
        locals.var_vbs_max_over__blk1770_dn9 = assign75590_e115137_d_n9;
        locals.var_vbs_max_over__blk1770_dn10 = assign75590_e115137_d_n10;
        locals.var_vbs_max_over__blk1770_dn11 = assign75590_e115137_d_n11;
        locals.var_vbs_max_over__blk1770_dn14 = assign75590_e115137_d_n14;

        let assign75600_e115141: f64 = (locals.var_vbs_max_over__blk1770 * 0.5);
        let assign75600_e115142: f64 = if locals.var_vbs_bnd_over__blk1771 > assign75600_e115141 { 1.0 } else { 0.0 };
        locals.var_guard1776 = assign75600_e115142;

        let (assign75610_e115150, assign75610_e115150_d_n0, assign75610_e115150_d_n2, assign75610_e115150_d_n4, assign75610_e115150_d_n5, assign75610_e115150_d_n6, assign75610_e115150_d_n7, assign75610_e115150_d_n8, assign75610_e115150_d_n9, assign75610_e115150_d_n10, assign75610_e115150_d_n11, assign75610_e115150_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1776 != 0.0)) {
        let assign75610_e115148: f64 = (0.5 * locals.var_vbs_max_over__blk1770);
        (assign75610_e115148, (0.5 * locals.var_vbs_max_over__blk1770_dn0), (0.5 * locals.var_vbs_max_over__blk1770_dn2), (0.5 * locals.var_vbs_max_over__blk1770_dn4), (0.5 * locals.var_vbs_max_over__blk1770_dn5), (0.5 * locals.var_vbs_max_over__blk1770_dn6), (0.5 * locals.var_vbs_max_over__blk1770_dn7), (0.5 * locals.var_vbs_max_over__blk1770_dn8), (0.5 * locals.var_vbs_max_over__blk1770_dn9), (0.5 * locals.var_vbs_max_over__blk1770_dn10), (0.5 * locals.var_vbs_max_over__blk1770_dn11), (0.5 * locals.var_vbs_max_over__blk1770_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk1771, locals.var_vbs_bnd_over__blk1771_dn0, locals.var_vbs_bnd_over__blk1771_dn2, locals.var_vbs_bnd_over__blk1771_dn4, locals.var_vbs_bnd_over__blk1771_dn5, locals.var_vbs_bnd_over__blk1771_dn6, locals.var_vbs_bnd_over__blk1771_dn7, locals.var_vbs_bnd_over__blk1771_dn8, locals.var_vbs_bnd_over__blk1771_dn9, locals.var_vbs_bnd_over__blk1771_dn10, locals.var_vbs_bnd_over__blk1771_dn11, locals.var_vbs_bnd_over__blk1771_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk1771 = assign75610_e115150;
        locals.var_vbs_bnd_over__blk1771_dn0 = assign75610_e115150_d_n0;
        locals.var_vbs_bnd_over__blk1771_dn2 = assign75610_e115150_d_n2;
        locals.var_vbs_bnd_over__blk1771_dn4 = assign75610_e115150_d_n4;
        locals.var_vbs_bnd_over__blk1771_dn5 = assign75610_e115150_d_n5;
        locals.var_vbs_bnd_over__blk1771_dn6 = assign75610_e115150_d_n6;
        locals.var_vbs_bnd_over__blk1771_dn7 = assign75610_e115150_d_n7;
        locals.var_vbs_bnd_over__blk1771_dn8 = assign75610_e115150_d_n8;
        locals.var_vbs_bnd_over__blk1771_dn9 = assign75610_e115150_d_n9;
        locals.var_vbs_bnd_over__blk1771_dn10 = assign75610_e115150_d_n10;
        locals.var_vbs_bnd_over__blk1771_dn11 = assign75610_e115150_d_n11;
        locals.var_vbs_bnd_over__blk1771_dn14 = assign75610_e115150_d_n14;

        let assign75620_e115152: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard1777 = assign75620_e115152;

        let (assign75630_e115158, assign75630_e115158_d_n0, assign75630_e115158_d_n2, assign75630_e115158_d_n4, assign75630_e115158_d_n5, assign75630_e115158_d_n6, assign75630_e115158_d_n7, assign75630_e115158_d_n8, assign75630_e115158_d_n9, assign75630_e115158_d_n10, assign75630_e115158_d_n11, assign75630_e115158_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1777 != 0.0)) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_over__blk1770, locals.var_vbs_max_over__blk1770_dn0, locals.var_vbs_max_over__blk1770_dn2, locals.var_vbs_max_over__blk1770_dn4, locals.var_vbs_max_over__blk1770_dn5, locals.var_vbs_max_over__blk1770_dn6, locals.var_vbs_max_over__blk1770_dn7, locals.var_vbs_max_over__blk1770_dn8, locals.var_vbs_max_over__blk1770_dn9, locals.var_vbs_max_over__blk1770_dn10, locals.var_vbs_max_over__blk1770_dn11, locals.var_vbs_max_over__blk1770_dn14,)
    }
};
        locals.var_vbs_max_over__blk1770 = assign75630_e115158;
        locals.var_vbs_max_over__blk1770_dn0 = assign75630_e115158_d_n0;
        locals.var_vbs_max_over__blk1770_dn2 = assign75630_e115158_d_n2;
        locals.var_vbs_max_over__blk1770_dn4 = assign75630_e115158_d_n4;
        locals.var_vbs_max_over__blk1770_dn5 = assign75630_e115158_d_n5;
        locals.var_vbs_max_over__blk1770_dn6 = assign75630_e115158_d_n6;
        locals.var_vbs_max_over__blk1770_dn7 = assign75630_e115158_d_n7;
        locals.var_vbs_max_over__blk1770_dn8 = assign75630_e115158_d_n8;
        locals.var_vbs_max_over__blk1770_dn9 = assign75630_e115158_d_n9;
        locals.var_vbs_max_over__blk1770_dn10 = assign75630_e115158_d_n10;
        locals.var_vbs_max_over__blk1770_dn11 = assign75630_e115158_d_n11;
        locals.var_vbs_max_over__blk1770_dn14 = assign75630_e115158_d_n14;

        let assign75640_e115160: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard1778 = assign75640_e115160;

        let (assign75650_e115166, assign75650_e115166_d_n0, assign75650_e115166_d_n2, assign75650_e115166_d_n4, assign75650_e115166_d_n5, assign75650_e115166_d_n6, assign75650_e115166_d_n7, assign75650_e115166_d_n8, assign75650_e115166_d_n9, assign75650_e115166_d_n10, assign75650_e115166_d_n11, assign75650_e115166_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1778 != 0.0)) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_over__blk1771, locals.var_vbs_bnd_over__blk1771_dn0, locals.var_vbs_bnd_over__blk1771_dn2, locals.var_vbs_bnd_over__blk1771_dn4, locals.var_vbs_bnd_over__blk1771_dn5, locals.var_vbs_bnd_over__blk1771_dn6, locals.var_vbs_bnd_over__blk1771_dn7, locals.var_vbs_bnd_over__blk1771_dn8, locals.var_vbs_bnd_over__blk1771_dn9, locals.var_vbs_bnd_over__blk1771_dn10, locals.var_vbs_bnd_over__blk1771_dn11, locals.var_vbs_bnd_over__blk1771_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk1771 = assign75650_e115166;
        locals.var_vbs_bnd_over__blk1771_dn0 = assign75650_e115166_d_n0;
        locals.var_vbs_bnd_over__blk1771_dn2 = assign75650_e115166_d_n2;
        locals.var_vbs_bnd_over__blk1771_dn4 = assign75650_e115166_d_n4;
        locals.var_vbs_bnd_over__blk1771_dn5 = assign75650_e115166_d_n5;
        locals.var_vbs_bnd_over__blk1771_dn6 = assign75650_e115166_d_n6;
        locals.var_vbs_bnd_over__blk1771_dn7 = assign75650_e115166_d_n7;
        locals.var_vbs_bnd_over__blk1771_dn8 = assign75650_e115166_d_n8;
        locals.var_vbs_bnd_over__blk1771_dn9 = assign75650_e115166_d_n9;
        locals.var_vbs_bnd_over__blk1771_dn10 = assign75650_e115166_d_n10;
        locals.var_vbs_bnd_over__blk1771_dn11 = assign75650_e115166_d_n11;
        locals.var_vbs_bnd_over__blk1771_dn14 = assign75650_e115166_d_n14;

        let assign75660_e115168: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard1779 = assign75660_e115168;

        let (assign75670_e115179, assign75670_e115179_d_n0, assign75670_e115179_d_n2, assign75670_e115179_d_n4, assign75670_e115179_d_n5, assign75670_e115179_d_n6, assign75670_e115179_d_n7, assign75670_e115179_d_n8, assign75670_e115179_d_n9, assign75670_e115179_d_n10, assign75670_e115179_d_n11, assign75670_e115179_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1778 == 0.0)) && (locals.var_guard1779 != 0.0)) {
        let assign75670_e115177: f64 = (0.5 * locals.var_vbs_max_over__blk1770);
        (assign75670_e115177, (0.5 * locals.var_vbs_max_over__blk1770_dn0), (0.5 * locals.var_vbs_max_over__blk1770_dn2), (0.5 * locals.var_vbs_max_over__blk1770_dn4), (0.5 * locals.var_vbs_max_over__blk1770_dn5), (0.5 * locals.var_vbs_max_over__blk1770_dn6), (0.5 * locals.var_vbs_max_over__blk1770_dn7), (0.5 * locals.var_vbs_max_over__blk1770_dn8), (0.5 * locals.var_vbs_max_over__blk1770_dn9), (0.5 * locals.var_vbs_max_over__blk1770_dn10), (0.5 * locals.var_vbs_max_over__blk1770_dn11), (0.5 * locals.var_vbs_max_over__blk1770_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk1771, locals.var_vbs_bnd_over__blk1771_dn0, locals.var_vbs_bnd_over__blk1771_dn2, locals.var_vbs_bnd_over__blk1771_dn4, locals.var_vbs_bnd_over__blk1771_dn5, locals.var_vbs_bnd_over__blk1771_dn6, locals.var_vbs_bnd_over__blk1771_dn7, locals.var_vbs_bnd_over__blk1771_dn8, locals.var_vbs_bnd_over__blk1771_dn9, locals.var_vbs_bnd_over__blk1771_dn10, locals.var_vbs_bnd_over__blk1771_dn11, locals.var_vbs_bnd_over__blk1771_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk1771 = assign75670_e115179;
        locals.var_vbs_bnd_over__blk1771_dn0 = assign75670_e115179_d_n0;
        locals.var_vbs_bnd_over__blk1771_dn2 = assign75670_e115179_d_n2;
        locals.var_vbs_bnd_over__blk1771_dn4 = assign75670_e115179_d_n4;
        locals.var_vbs_bnd_over__blk1771_dn5 = assign75670_e115179_d_n5;
        locals.var_vbs_bnd_over__blk1771_dn6 = assign75670_e115179_d_n6;
        locals.var_vbs_bnd_over__blk1771_dn7 = assign75670_e115179_d_n7;
        locals.var_vbs_bnd_over__blk1771_dn8 = assign75670_e115179_d_n8;
        locals.var_vbs_bnd_over__blk1771_dn9 = assign75670_e115179_d_n9;
        locals.var_vbs_bnd_over__blk1771_dn10 = assign75670_e115179_d_n10;
        locals.var_vbs_bnd_over__blk1771_dn11 = assign75670_e115179_d_n11;
        locals.var_vbs_bnd_over__blk1771_dn14 = assign75670_e115179_d_n14;

        let assign75680_e115183: f64 = (locals.var_vbs_max_over__blk1770 * 0.5);
        let assign75680_e115184: f64 = if locals.var_vbs_bnd_over__blk1771 > assign75680_e115183 { 1.0 } else { 0.0 };
        locals.var_guard1780 = assign75680_e115184;

        let (assign75690_e115192, assign75690_e115192_d_n0, assign75690_e115192_d_n2, assign75690_e115192_d_n4, assign75690_e115192_d_n5, assign75690_e115192_d_n6, assign75690_e115192_d_n7, assign75690_e115192_d_n8, assign75690_e115192_d_n9, assign75690_e115192_d_n10, assign75690_e115192_d_n11, assign75690_e115192_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1780 != 0.0)) {
        let assign75690_e115190: f64 = (0.5 * locals.var_vbs_max_over__blk1770);
        (assign75690_e115190, (0.5 * locals.var_vbs_max_over__blk1770_dn0), (0.5 * locals.var_vbs_max_over__blk1770_dn2), (0.5 * locals.var_vbs_max_over__blk1770_dn4), (0.5 * locals.var_vbs_max_over__blk1770_dn5), (0.5 * locals.var_vbs_max_over__blk1770_dn6), (0.5 * locals.var_vbs_max_over__blk1770_dn7), (0.5 * locals.var_vbs_max_over__blk1770_dn8), (0.5 * locals.var_vbs_max_over__blk1770_dn9), (0.5 * locals.var_vbs_max_over__blk1770_dn10), (0.5 * locals.var_vbs_max_over__blk1770_dn11), (0.5 * locals.var_vbs_max_over__blk1770_dn14),)
    } else {
        (locals.var_vbs_bnd_over__blk1771, locals.var_vbs_bnd_over__blk1771_dn0, locals.var_vbs_bnd_over__blk1771_dn2, locals.var_vbs_bnd_over__blk1771_dn4, locals.var_vbs_bnd_over__blk1771_dn5, locals.var_vbs_bnd_over__blk1771_dn6, locals.var_vbs_bnd_over__blk1771_dn7, locals.var_vbs_bnd_over__blk1771_dn8, locals.var_vbs_bnd_over__blk1771_dn9, locals.var_vbs_bnd_over__blk1771_dn10, locals.var_vbs_bnd_over__blk1771_dn11, locals.var_vbs_bnd_over__blk1771_dn14,)
    }
};
        locals.var_vbs_bnd_over__blk1771 = assign75690_e115192;
        locals.var_vbs_bnd_over__blk1771_dn0 = assign75690_e115192_d_n0;
        locals.var_vbs_bnd_over__blk1771_dn2 = assign75690_e115192_d_n2;
        locals.var_vbs_bnd_over__blk1771_dn4 = assign75690_e115192_d_n4;
        locals.var_vbs_bnd_over__blk1771_dn5 = assign75690_e115192_d_n5;
        locals.var_vbs_bnd_over__blk1771_dn6 = assign75690_e115192_d_n6;
        locals.var_vbs_bnd_over__blk1771_dn7 = assign75690_e115192_d_n7;
        locals.var_vbs_bnd_over__blk1771_dn8 = assign75690_e115192_d_n8;
        locals.var_vbs_bnd_over__blk1771_dn9 = assign75690_e115192_d_n9;
        locals.var_vbs_bnd_over__blk1771_dn10 = assign75690_e115192_d_n10;
        locals.var_vbs_bnd_over__blk1771_dn11 = assign75690_e115192_d_n11;
        locals.var_vbs_bnd_over__blk1771_dn14 = assign75690_e115192_d_n14;

        let assign75700_e115195: f64 = if p.p38 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1781 = assign75700_e115195;

        let (assign75710_e115202, assign75710_e115202_d_n0, assign75710_e115202_d_n2, assign75710_e115202_d_n4, assign75710_e115202_d_n5, assign75710_e115202_d_n6, assign75710_e115202_d_n7, assign75710_e115202_d_n8, assign75710_e115202_d_n9, assign75710_e115202_d_n10, assign75710_e115202_d_n11, assign75710_e115202_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) {
        let assign75710_e115200: f64 = (-locals.var_vxbgmt);
        (assign75710_e115200, (-locals.var_vxbgmt_dn0), (-locals.var_vxbgmt_dn2), (-locals.var_vxbgmt_dn4), (-locals.var_vxbgmt_dn5), (-locals.var_vxbgmt_dn6), (-locals.var_vxbgmt_dn7), (-locals.var_vxbgmt_dn8), (-locals.var_vxbgmt_dn9), (-locals.var_vxbgmt_dn10), (-locals.var_vxbgmt_dn11), (-locals.var_vxbgmt_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign75710_e115202;
        locals.var_t0_dn0 = assign75710_e115202_d_n0;
        locals.var_t0_dn2 = assign75710_e115202_d_n2;
        locals.var_t0_dn4 = assign75710_e115202_d_n4;
        locals.var_t0_dn5 = assign75710_e115202_d_n5;
        locals.var_t0_dn6 = assign75710_e115202_d_n6;
        locals.var_t0_dn7 = assign75710_e115202_d_n7;
        locals.var_t0_dn8 = assign75710_e115202_d_n8;
        locals.var_t0_dn9 = assign75710_e115202_d_n9;
        locals.var_t0_dn10 = assign75710_e115202_d_n10;
        locals.var_t0_dn11 = assign75710_e115202_d_n11;
        locals.var_t0_dn14 = assign75710_e115202_d_n14;

        let assign75720_e115205: f64 = if locals.var_t0 > locals.var_vbs_bnd_over__blk1771 { 1.0 } else { 0.0 };
        locals.var_guard1782 = assign75720_e115205;

        let (assign75730_e115215, assign75730_e115215_d_n0, assign75730_e115215_d_n2, assign75730_e115215_d_n4, assign75730_e115215_d_n5, assign75730_e115215_d_n6, assign75730_e115215_d_n7, assign75730_e115215_d_n8, assign75730_e115215_d_n9, assign75730_e115215_d_n10, assign75730_e115215_d_n11, assign75730_e115215_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75730_e115213: f64 = (locals.var_t0 - locals.var_vbs_bnd_over__blk1771);
        (assign75730_e115213, (locals.var_t0_dn0 - locals.var_vbs_bnd_over__blk1771_dn0), (locals.var_t0_dn2 - locals.var_vbs_bnd_over__blk1771_dn2), (locals.var_t0_dn4 - locals.var_vbs_bnd_over__blk1771_dn4), (locals.var_t0_dn5 - locals.var_vbs_bnd_over__blk1771_dn5), (locals.var_t0_dn6 - locals.var_vbs_bnd_over__blk1771_dn6), (locals.var_t0_dn7 - locals.var_vbs_bnd_over__blk1771_dn7), (locals.var_t0_dn8 - locals.var_vbs_bnd_over__blk1771_dn8), (locals.var_t0_dn9 - locals.var_vbs_bnd_over__blk1771_dn9), (locals.var_t0_dn10 - locals.var_vbs_bnd_over__blk1771_dn10), (locals.var_t0_dn11 - locals.var_vbs_bnd_over__blk1771_dn11), (locals.var_t0_dn14 - locals.var_vbs_bnd_over__blk1771_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign75730_e115215;
        locals.var_t1_dn0 = assign75730_e115215_d_n0;
        locals.var_t1_dn2 = assign75730_e115215_d_n2;
        locals.var_t1_dn4 = assign75730_e115215_d_n4;
        locals.var_t1_dn5 = assign75730_e115215_d_n5;
        locals.var_t1_dn6 = assign75730_e115215_d_n6;
        locals.var_t1_dn7 = assign75730_e115215_d_n7;
        locals.var_t1_dn8 = assign75730_e115215_d_n8;
        locals.var_t1_dn9 = assign75730_e115215_d_n9;
        locals.var_t1_dn10 = assign75730_e115215_d_n10;
        locals.var_t1_dn11 = assign75730_e115215_d_n11;
        locals.var_t1_dn14 = assign75730_e115215_d_n14;

        let (assign75740_e115225, assign75740_e115225_d_n0, assign75740_e115225_d_n2, assign75740_e115225_d_n4, assign75740_e115225_d_n5, assign75740_e115225_d_n6, assign75740_e115225_d_n7, assign75740_e115225_d_n8, assign75740_e115225_d_n9, assign75740_e115225_d_n10, assign75740_e115225_d_n11, assign75740_e115225_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75740_e115223: f64 = (locals.var_vbs_max_over__blk1770 - locals.var_vbs_bnd_over__blk1771);
        (assign75740_e115223, (locals.var_vbs_max_over__blk1770_dn0 - locals.var_vbs_bnd_over__blk1771_dn0), (locals.var_vbs_max_over__blk1770_dn2 - locals.var_vbs_bnd_over__blk1771_dn2), (locals.var_vbs_max_over__blk1770_dn4 - locals.var_vbs_bnd_over__blk1771_dn4), (locals.var_vbs_max_over__blk1770_dn5 - locals.var_vbs_bnd_over__blk1771_dn5), (locals.var_vbs_max_over__blk1770_dn6 - locals.var_vbs_bnd_over__blk1771_dn6), (locals.var_vbs_max_over__blk1770_dn7 - locals.var_vbs_bnd_over__blk1771_dn7), (locals.var_vbs_max_over__blk1770_dn8 - locals.var_vbs_bnd_over__blk1771_dn8), (locals.var_vbs_max_over__blk1770_dn9 - locals.var_vbs_bnd_over__blk1771_dn9), (locals.var_vbs_max_over__blk1770_dn10 - locals.var_vbs_bnd_over__blk1771_dn10), (locals.var_vbs_max_over__blk1770_dn11 - locals.var_vbs_bnd_over__blk1771_dn11), (locals.var_vbs_max_over__blk1770_dn14 - locals.var_vbs_bnd_over__blk1771_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign75740_e115225;
        locals.var_t2_dn0 = assign75740_e115225_d_n0;
        locals.var_t2_dn2 = assign75740_e115225_d_n2;
        locals.var_t2_dn4 = assign75740_e115225_d_n4;
        locals.var_t2_dn5 = assign75740_e115225_d_n5;
        locals.var_t2_dn6 = assign75740_e115225_d_n6;
        locals.var_t2_dn7 = assign75740_e115225_d_n7;
        locals.var_t2_dn8 = assign75740_e115225_d_n8;
        locals.var_t2_dn9 = assign75740_e115225_d_n9;
        locals.var_t2_dn10 = assign75740_e115225_d_n10;
        locals.var_t2_dn11 = assign75740_e115225_d_n11;
        locals.var_t2_dn14 = assign75740_e115225_d_n14;

        let (assign75750_e115235, assign75750_e115235_d_n0, assign75750_e115235_d_n2, assign75750_e115235_d_n4, assign75750_e115235_d_n5, assign75750_e115235_d_n6, assign75750_e115235_d_n7, assign75750_e115235_d_n8, assign75750_e115235_d_n9, assign75750_e115235_d_n10, assign75750_e115235_d_n11, assign75750_e115235_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75750_e115233: f64 = (locals.var_t1 / locals.var_t2);
        (assign75750_e115233, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign75750_e115235;
        locals.var_tmf1_dn0 = assign75750_e115235_d_n0;
        locals.var_tmf1_dn2 = assign75750_e115235_d_n2;
        locals.var_tmf1_dn4 = assign75750_e115235_d_n4;
        locals.var_tmf1_dn5 = assign75750_e115235_d_n5;
        locals.var_tmf1_dn6 = assign75750_e115235_d_n6;
        locals.var_tmf1_dn7 = assign75750_e115235_d_n7;
        locals.var_tmf1_dn8 = assign75750_e115235_d_n8;
        locals.var_tmf1_dn9 = assign75750_e115235_d_n9;
        locals.var_tmf1_dn10 = assign75750_e115235_d_n10;
        locals.var_tmf1_dn11 = assign75750_e115235_d_n11;
        locals.var_tmf1_dn14 = assign75750_e115235_d_n14;

        let (assign75760_e115245, assign75760_e115245_d_n0, assign75760_e115245_d_n2, assign75760_e115245_d_n4, assign75760_e115245_d_n5, assign75760_e115245_d_n6, assign75760_e115245_d_n7, assign75760_e115245_d_n8, assign75760_e115245_d_n9, assign75760_e115245_d_n10, assign75760_e115245_d_n11, assign75760_e115245_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75760_e115243: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign75760_e115243, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign75760_e115245;
        locals.var_tmf2_dn0 = assign75760_e115245_d_n0;
        locals.var_tmf2_dn2 = assign75760_e115245_d_n2;
        locals.var_tmf2_dn4 = assign75760_e115245_d_n4;
        locals.var_tmf2_dn5 = assign75760_e115245_d_n5;
        locals.var_tmf2_dn6 = assign75760_e115245_d_n6;
        locals.var_tmf2_dn7 = assign75760_e115245_d_n7;
        locals.var_tmf2_dn8 = assign75760_e115245_d_n8;
        locals.var_tmf2_dn9 = assign75760_e115245_d_n9;
        locals.var_tmf2_dn10 = assign75760_e115245_d_n10;
        locals.var_tmf2_dn11 = assign75760_e115245_d_n11;
        locals.var_tmf2_dn14 = assign75760_e115245_d_n14;

        let (assign75770_e115255, assign75770_e115255_d_n0, assign75770_e115255_d_n2, assign75770_e115255_d_n4, assign75770_e115255_d_n5, assign75770_e115255_d_n6, assign75770_e115255_d_n7, assign75770_e115255_d_n8, assign75770_e115255_d_n9, assign75770_e115255_d_n10, assign75770_e115255_d_n11, assign75770_e115255_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75770_e115253: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign75770_e115253, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign75770_e115255;
        locals.var_tmf3_dn0 = assign75770_e115255_d_n0;
        locals.var_tmf3_dn2 = assign75770_e115255_d_n2;
        locals.var_tmf3_dn4 = assign75770_e115255_d_n4;
        locals.var_tmf3_dn5 = assign75770_e115255_d_n5;
        locals.var_tmf3_dn6 = assign75770_e115255_d_n6;
        locals.var_tmf3_dn7 = assign75770_e115255_d_n7;
        locals.var_tmf3_dn8 = assign75770_e115255_d_n8;
        locals.var_tmf3_dn9 = assign75770_e115255_d_n9;
        locals.var_tmf3_dn10 = assign75770_e115255_d_n10;
        locals.var_tmf3_dn11 = assign75770_e115255_d_n11;
        locals.var_tmf3_dn14 = assign75770_e115255_d_n14;

        let (assign75780_e115265, assign75780_e115265_d_n0, assign75780_e115265_d_n2, assign75780_e115265_d_n4, assign75780_e115265_d_n5, assign75780_e115265_d_n6, assign75780_e115265_d_n7, assign75780_e115265_d_n8, assign75780_e115265_d_n9, assign75780_e115265_d_n10, assign75780_e115265_d_n11, assign75780_e115265_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75780_e115263: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign75780_e115263, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign75780_e115265;
        locals.var_tmf4_dn0 = assign75780_e115265_d_n0;
        locals.var_tmf4_dn2 = assign75780_e115265_d_n2;
        locals.var_tmf4_dn4 = assign75780_e115265_d_n4;
        locals.var_tmf4_dn5 = assign75780_e115265_d_n5;
        locals.var_tmf4_dn6 = assign75780_e115265_d_n6;
        locals.var_tmf4_dn7 = assign75780_e115265_d_n7;
        locals.var_tmf4_dn8 = assign75780_e115265_d_n8;
        locals.var_tmf4_dn9 = assign75780_e115265_d_n9;
        locals.var_tmf4_dn10 = assign75780_e115265_d_n10;
        locals.var_tmf4_dn11 = assign75780_e115265_d_n11;
        locals.var_tmf4_dn14 = assign75780_e115265_d_n14;

        let (assign75790_e115283, assign75790_e115283_d_n0, assign75790_e115283_d_n2, assign75790_e115283_d_n4, assign75790_e115283_d_n5, assign75790_e115283_d_n6, assign75790_e115283_d_n7, assign75790_e115283_d_n8, assign75790_e115283_d_n9, assign75790_e115283_d_n10, assign75790_e115283_d_n11, assign75790_e115283_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75790_e115274: f64 = (1.0 + locals.var_tmf1);
        let assign75790_e115276: f64 = (assign75790_e115274 + locals.var_tmf2);
        let assign75790_e115278: f64 = (assign75790_e115276 + locals.var_tmf3);
        let assign75790_e115280: f64 = (assign75790_e115278 + locals.var_tmf4);
        let assign75790_e115281: f64 = (1.0 / assign75790_e115280);
        (assign75790_e115281, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign75790_e115280 * assign75790_e115280))), (-((((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) + locals.var_tmf3_dn14) + locals.var_tmf4_dn14) / (assign75790_e115280 * assign75790_e115280))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign75790_e115283;
        locals.var_tmf0_dn0 = assign75790_e115283_d_n0;
        locals.var_tmf0_dn2 = assign75790_e115283_d_n2;
        locals.var_tmf0_dn4 = assign75790_e115283_d_n4;
        locals.var_tmf0_dn5 = assign75790_e115283_d_n5;
        locals.var_tmf0_dn6 = assign75790_e115283_d_n6;
        locals.var_tmf0_dn7 = assign75790_e115283_d_n7;
        locals.var_tmf0_dn8 = assign75790_e115283_d_n8;
        locals.var_tmf0_dn9 = assign75790_e115283_d_n9;
        locals.var_tmf0_dn10 = assign75790_e115283_d_n10;
        locals.var_tmf0_dn11 = assign75790_e115283_d_n11;
        locals.var_tmf0_dn14 = assign75790_e115283_d_n14;

    }

    pub(super) fn stamp_transient_block_273(
        locals: &mut StampLocals,
    ) {
        let (assign75800_e115308, assign75800_e115308_d_n0, assign75800_e115308_d_n2, assign75800_e115308_d_n4, assign75800_e115308_d_n5, assign75800_e115308_d_n6, assign75800_e115308_d_n7, assign75800_e115308_d_n8, assign75800_e115308_d_n9, assign75800_e115308_d_n10, assign75800_e115308_d_n11, assign75800_e115308_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75800_e115292: f64 = (2.0 * locals.var_tmf1);
        let assign75800_e115293: f64 = (1.0 + assign75800_e115292);
        let assign75800_e115296: f64 = (3.0 * locals.var_tmf2);
        let assign75800_e115297: f64 = (assign75800_e115293 + assign75800_e115296);
        let assign75800_e115300: f64 = (4.0 * locals.var_tmf3);
        let assign75800_e115301: f64 = (assign75800_e115297 + assign75800_e115300);
        let assign75800_e115302: f64 = (-assign75800_e115301);
        let assign75800_e115304: f64 = (assign75800_e115302 * locals.var_tmf0);
        let assign75800_e115306: f64 = (assign75800_e115304 * locals.var_tmf0);
        (assign75800_e115306, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn11)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn11)), (((((-(((2.0 * locals.var_tmf1_dn14) + (3.0 * locals.var_tmf2_dn14)) + (4.0 * locals.var_tmf3_dn14))) * locals.var_tmf0) + (assign75800_e115302 * locals.var_tmf0_dn14)) * locals.var_tmf0) + (assign75800_e115304 * locals.var_tmf0_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign75800_e115308;
        locals.var_t11_dn0 = assign75800_e115308_d_n0;
        locals.var_t11_dn2 = assign75800_e115308_d_n2;
        locals.var_t11_dn4 = assign75800_e115308_d_n4;
        locals.var_t11_dn5 = assign75800_e115308_d_n5;
        locals.var_t11_dn6 = assign75800_e115308_d_n6;
        locals.var_t11_dn7 = assign75800_e115308_d_n7;
        locals.var_t11_dn8 = assign75800_e115308_d_n8;
        locals.var_t11_dn9 = assign75800_e115308_d_n9;
        locals.var_t11_dn10 = assign75800_e115308_d_n10;
        locals.var_t11_dn11 = assign75800_e115308_d_n11;
        locals.var_t11_dn14 = assign75800_e115308_d_n14;

        let (assign75810_e115320, assign75810_e115320_d_n0, assign75810_e115320_d_n2, assign75810_e115320_d_n4, assign75810_e115320_d_n5, assign75810_e115320_d_n6, assign75810_e115320_d_n7, assign75810_e115320_d_n8, assign75810_e115320_d_n9, assign75810_e115320_d_n10, assign75810_e115320_d_n11, assign75810_e115320_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75810_e115317: f64 = (1.0 - locals.var_tmf0);
        let assign75810_e115318: f64 = (locals.var_t2 * assign75810_e115317);
        (assign75810_e115318, ((locals.var_t2_dn0 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn11 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn11))), ((locals.var_t2_dn14 * assign75810_e115317) + (locals.var_t2 * (-locals.var_tmf0_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign75810_e115320;
        locals.var_ty_dn0 = assign75810_e115320_d_n0;
        locals.var_ty_dn2 = assign75810_e115320_d_n2;
        locals.var_ty_dn4 = assign75810_e115320_d_n4;
        locals.var_ty_dn5 = assign75810_e115320_d_n5;
        locals.var_ty_dn6 = assign75810_e115320_d_n6;
        locals.var_ty_dn7 = assign75810_e115320_d_n7;
        locals.var_ty_dn8 = assign75810_e115320_d_n8;
        locals.var_ty_dn9 = assign75810_e115320_d_n9;
        locals.var_ty_dn10 = assign75810_e115320_d_n10;
        locals.var_ty_dn11 = assign75810_e115320_d_n11;
        locals.var_ty_dn14 = assign75810_e115320_d_n14;

        let (assign75820_e115334, assign75820_e115334_d_n0, assign75820_e115334_d_n2, assign75820_e115334_d_n4, assign75820_e115334_d_n5, assign75820_e115334_d_n6, assign75820_e115334_d_n7, assign75820_e115334_d_n8, assign75820_e115334_d_n9, assign75820_e115334_d_n10, assign75820_e115334_d_n11, assign75820_e115334_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75820_e115328: f64 = (1.0 - locals.var_tmf0);
        let assign75820_e115331: f64 = (locals.var_tmf1 * locals.var_t11);
        let assign75820_e115332: f64 = (assign75820_e115328 + assign75820_e115331);
        (assign75820_e115332, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn10))), ((-locals.var_tmf0_dn11) + ((locals.var_tmf1_dn11 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn11))), ((-locals.var_tmf0_dn14) + ((locals.var_tmf1_dn14 * locals.var_t11) + (locals.var_tmf1 * locals.var_t11_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign75820_e115334;
        locals.var_t0_dn0 = assign75820_e115334_d_n0;
        locals.var_t0_dn2 = assign75820_e115334_d_n2;
        locals.var_t0_dn4 = assign75820_e115334_d_n4;
        locals.var_t0_dn5 = assign75820_e115334_d_n5;
        locals.var_t0_dn6 = assign75820_e115334_d_n6;
        locals.var_t0_dn7 = assign75820_e115334_d_n7;
        locals.var_t0_dn8 = assign75820_e115334_d_n8;
        locals.var_t0_dn9 = assign75820_e115334_d_n9;
        locals.var_t0_dn10 = assign75820_e115334_d_n10;
        locals.var_t0_dn11 = assign75820_e115334_d_n11;
        locals.var_t0_dn14 = assign75820_e115334_d_n14;

        let (assign75830_e115343, assign75830_e115343_d_n0, assign75830_e115343_d_n2, assign75830_e115343_d_n4, assign75830_e115343_d_n5, assign75830_e115343_d_n6, assign75830_e115343_d_n7, assign75830_e115343_d_n8, assign75830_e115343_d_n9, assign75830_e115343_d_n10, assign75830_e115343_d_n11, assign75830_e115343_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75830_e115341: f64 = (-locals.var_t11);
        (assign75830_e115341, (-locals.var_t11_dn0), (-locals.var_t11_dn2), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11), (-locals.var_t11_dn14),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign75830_e115343;
        locals.var_t11_dn0 = assign75830_e115343_d_n0;
        locals.var_t11_dn2 = assign75830_e115343_d_n2;
        locals.var_t11_dn4 = assign75830_e115343_d_n4;
        locals.var_t11_dn5 = assign75830_e115343_d_n5;
        locals.var_t11_dn6 = assign75830_e115343_d_n6;
        locals.var_t11_dn7 = assign75830_e115343_d_n7;
        locals.var_t11_dn8 = assign75830_e115343_d_n8;
        locals.var_t11_dn9 = assign75830_e115343_d_n9;
        locals.var_t11_dn10 = assign75830_e115343_d_n10;
        locals.var_t11_dn11 = assign75830_e115343_d_n11;
        locals.var_t11_dn14 = assign75830_e115343_d_n14;

        let (assign75840_e115353, assign75840_e115353_d_n0, assign75840_e115353_d_n2, assign75840_e115353_d_n4, assign75840_e115353_d_n5, assign75840_e115353_d_n6, assign75840_e115353_d_n7, assign75840_e115353_d_n8, assign75840_e115353_d_n9, assign75840_e115353_d_n10, assign75840_e115353_d_n11, assign75840_e115353_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 != 0.0)) {
        let assign75840_e115351: f64 = (locals.var_vbs_bnd_over__blk1771 + locals.var_ty);
        (assign75840_e115351, (locals.var_vbs_bnd_over__blk1771_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_over__blk1771_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_over__blk1771_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_over__blk1771_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_over__blk1771_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_over__blk1771_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_over__blk1771_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_over__blk1771_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_over__blk1771_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_over__blk1771_dn11 + locals.var_ty_dn11), (locals.var_vbs_bnd_over__blk1771_dn14 + locals.var_ty_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign75840_e115353;
        locals.var_t10_dn0 = assign75840_e115353_d_n0;
        locals.var_t10_dn2 = assign75840_e115353_d_n2;
        locals.var_t10_dn4 = assign75840_e115353_d_n4;
        locals.var_t10_dn5 = assign75840_e115353_d_n5;
        locals.var_t10_dn6 = assign75840_e115353_d_n6;
        locals.var_t10_dn7 = assign75840_e115353_d_n7;
        locals.var_t10_dn8 = assign75840_e115353_d_n8;
        locals.var_t10_dn9 = assign75840_e115353_d_n9;
        locals.var_t10_dn10 = assign75840_e115353_d_n10;
        locals.var_t10_dn11 = assign75840_e115353_d_n11;
        locals.var_t10_dn14 = assign75840_e115353_d_n14;

        let (assign75850_e115362, assign75850_e115362_d_n0, assign75850_e115362_d_n2, assign75850_e115362_d_n4, assign75850_e115362_d_n5, assign75850_e115362_d_n6, assign75850_e115362_d_n7, assign75850_e115362_d_n8, assign75850_e115362_d_n9, assign75850_e115362_d_n10, assign75850_e115362_d_n11, assign75850_e115362_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) && (locals.var_guard1782 == 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign75850_e115362;
        locals.var_t10_dn0 = assign75850_e115362_d_n0;
        locals.var_t10_dn2 = assign75850_e115362_d_n2;
        locals.var_t10_dn4 = assign75850_e115362_d_n4;
        locals.var_t10_dn5 = assign75850_e115362_d_n5;
        locals.var_t10_dn6 = assign75850_e115362_d_n6;
        locals.var_t10_dn7 = assign75850_e115362_d_n7;
        locals.var_t10_dn8 = assign75850_e115362_d_n8;
        locals.var_t10_dn9 = assign75850_e115362_d_n9;
        locals.var_t10_dn10 = assign75850_e115362_d_n10;
        locals.var_t10_dn11 = assign75850_e115362_d_n11;
        locals.var_t10_dn14 = assign75850_e115362_d_n14;

        let (assign75860_e115369, assign75860_e115369_d_n0, assign75860_e115369_d_n2, assign75860_e115369_d_n4, assign75860_e115369_d_n5, assign75860_e115369_d_n6, assign75860_e115369_d_n7, assign75860_e115369_d_n8, assign75860_e115369_d_n9, assign75860_e115369_d_n10, assign75860_e115369_d_n11, assign75860_e115369_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 != 0.0)) {
        let assign75860_e115367: f64 = (-locals.var_t10);
        (assign75860_e115367, (-locals.var_t10_dn0), (-locals.var_t10_dn2), (-locals.var_t10_dn4), (-locals.var_t10_dn5), (-locals.var_t10_dn6), (-locals.var_t10_dn7), (-locals.var_t10_dn8), (-locals.var_t10_dn9), (-locals.var_t10_dn10), (-locals.var_t10_dn11), (-locals.var_t10_dn14),)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign75860_e115369;
        locals.var_vxbgmtcl_dn0 = assign75860_e115369_d_n0;
        locals.var_vxbgmtcl_dn2 = assign75860_e115369_d_n2;
        locals.var_vxbgmtcl_dn4 = assign75860_e115369_d_n4;
        locals.var_vxbgmtcl_dn5 = assign75860_e115369_d_n5;
        locals.var_vxbgmtcl_dn6 = assign75860_e115369_d_n6;
        locals.var_vxbgmtcl_dn7 = assign75860_e115369_d_n7;
        locals.var_vxbgmtcl_dn8 = assign75860_e115369_d_n8;
        locals.var_vxbgmtcl_dn9 = assign75860_e115369_d_n9;
        locals.var_vxbgmtcl_dn10 = assign75860_e115369_d_n10;
        locals.var_vxbgmtcl_dn11 = assign75860_e115369_d_n11;
        locals.var_vxbgmtcl_dn14 = assign75860_e115369_d_n14;

        let (assign75870_e115376, assign75870_e115376_d_n0, assign75870_e115376_d_n2, assign75870_e115376_d_n4, assign75870_e115376_d_n5, assign75870_e115376_d_n6, assign75870_e115376_d_n7, assign75870_e115376_d_n8, assign75870_e115376_d_n9, assign75870_e115376_d_n10, assign75870_e115376_d_n11, assign75870_e115376_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1781 == 0.0)) {
        (locals.var_vxbgmt, locals.var_vxbgmt_dn0, locals.var_vxbgmt_dn2, locals.var_vxbgmt_dn4, locals.var_vxbgmt_dn5, locals.var_vxbgmt_dn6, locals.var_vxbgmt_dn7, locals.var_vxbgmt_dn8, locals.var_vxbgmt_dn9, locals.var_vxbgmt_dn10, locals.var_vxbgmt_dn11, locals.var_vxbgmt_dn14,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign75870_e115376;
        locals.var_vxbgmtcl_dn0 = assign75870_e115376_d_n0;
        locals.var_vxbgmtcl_dn2 = assign75870_e115376_d_n2;
        locals.var_vxbgmtcl_dn4 = assign75870_e115376_d_n4;
        locals.var_vxbgmtcl_dn5 = assign75870_e115376_d_n5;
        locals.var_vxbgmtcl_dn6 = assign75870_e115376_d_n6;
        locals.var_vxbgmtcl_dn7 = assign75870_e115376_d_n7;
        locals.var_vxbgmtcl_dn8 = assign75870_e115376_d_n8;
        locals.var_vxbgmtcl_dn9 = assign75870_e115376_d_n9;
        locals.var_vxbgmtcl_dn10 = assign75870_e115376_d_n10;
        locals.var_vxbgmtcl_dn11 = assign75870_e115376_d_n11;
        locals.var_vxbgmtcl_dn14 = assign75870_e115376_d_n14;

        let (assign75880_e115382, assign75880_e115382_d_n0, assign75880_e115382_d_n2, assign75880_e115382_d_n4, assign75880_e115382_d_n5, assign75880_e115382_d_n6, assign75880_e115382_d_n7, assign75880_e115382_d_n8, assign75880_e115382_d_n9, assign75880_e115382_d_n10, assign75880_e115382_d_n11, assign75880_e115382_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75880_e115380: f64 = (locals.var_cnst0over_func / locals.var_cox0_func);
        (assign75880_e115380, (locals.var_cnst0over_func_dn0 / locals.var_cox0_func), (locals.var_cnst0over_func_dn2 / locals.var_cox0_func), (locals.var_cnst0over_func_dn4 / locals.var_cox0_func), (locals.var_cnst0over_func_dn5 / locals.var_cox0_func), (locals.var_cnst0over_func_dn6 / locals.var_cox0_func), (locals.var_cnst0over_func_dn7 / locals.var_cox0_func), (locals.var_cnst0over_func_dn8 / locals.var_cox0_func), (locals.var_cnst0over_func_dn9 / locals.var_cox0_func), (locals.var_cnst0over_func_dn10 / locals.var_cox0_func), (locals.var_cnst0over_func_dn11 / locals.var_cox0_func), (locals.var_cnst0over_func_dn14 / locals.var_cox0_func),)
    } else {
        (locals.var_fac1, locals.var_fac1_dn0, locals.var_fac1_dn2, locals.var_fac1_dn4, locals.var_fac1_dn5, locals.var_fac1_dn6, locals.var_fac1_dn7, locals.var_fac1_dn8, locals.var_fac1_dn9, locals.var_fac1_dn10, locals.var_fac1_dn11, locals.var_fac1_dn14,)
    }
};
        locals.var_fac1 = assign75880_e115382;
        locals.var_fac1_dn0 = assign75880_e115382_d_n0;
        locals.var_fac1_dn2 = assign75880_e115382_d_n2;
        locals.var_fac1_dn4 = assign75880_e115382_d_n4;
        locals.var_fac1_dn5 = assign75880_e115382_d_n5;
        locals.var_fac1_dn6 = assign75880_e115382_d_n6;
        locals.var_fac1_dn7 = assign75880_e115382_d_n7;
        locals.var_fac1_dn8 = assign75880_e115382_d_n8;
        locals.var_fac1_dn9 = assign75880_e115382_d_n9;
        locals.var_fac1_dn10 = assign75880_e115382_d_n10;
        locals.var_fac1_dn11 = assign75880_e115382_d_n11;
        locals.var_fac1_dn14 = assign75880_e115382_d_n14;

        let (assign75890_e115388, assign75890_e115388_d_n0, assign75890_e115388_d_n2, assign75890_e115388_d_n4, assign75890_e115388_d_n5, assign75890_e115388_d_n6, assign75890_e115388_d_n7, assign75890_e115388_d_n8, assign75890_e115388_d_n9, assign75890_e115388_d_n10, assign75890_e115388_d_n11, assign75890_e115388_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75890_e115386: f64 = (locals.var_fac1 * locals.var_fac1);
        (assign75890_e115386, ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0)), ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2)), ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4)), ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5)), ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6)), ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7)), ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8)), ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9)), ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10)), ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11)), ((locals.var_fac1_dn14 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn14)),)
    } else {
        (locals.var_fac1p2, locals.var_fac1p2_dn0, locals.var_fac1p2_dn2, locals.var_fac1p2_dn4, locals.var_fac1p2_dn5, locals.var_fac1p2_dn6, locals.var_fac1p2_dn7, locals.var_fac1p2_dn8, locals.var_fac1p2_dn9, locals.var_fac1p2_dn10, locals.var_fac1p2_dn11, locals.var_fac1p2_dn14,)
    }
};
        locals.var_fac1p2 = assign75890_e115388;
        locals.var_fac1p2_dn0 = assign75890_e115388_d_n0;
        locals.var_fac1p2_dn2 = assign75890_e115388_d_n2;
        locals.var_fac1p2_dn4 = assign75890_e115388_d_n4;
        locals.var_fac1p2_dn5 = assign75890_e115388_d_n5;
        locals.var_fac1p2_dn6 = assign75890_e115388_d_n6;
        locals.var_fac1p2_dn7 = assign75890_e115388_d_n7;
        locals.var_fac1p2_dn8 = assign75890_e115388_d_n8;
        locals.var_fac1p2_dn9 = assign75890_e115388_d_n9;
        locals.var_fac1p2_dn10 = assign75890_e115388_d_n10;
        locals.var_fac1p2_dn11 = assign75890_e115388_d_n11;
        locals.var_fac1p2_dn14 = assign75890_e115388_d_n14;

        let (assign75900_e115395, assign75900_e115395_d_n2, assign75900_e115395_d_n7, assign75900_e115395_d_n8, assign75900_e115395_d_n9,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75900_e115391: f64 = (-locals.var_vgbgmt);
        let assign75900_e115393: f64 = (assign75900_e115391 + locals.var_uc_vfbover);
        (assign75900_e115393, (-locals.var_vgbgmt_dn2), (-locals.var_vgbgmt_dn7), (-locals.var_vgbgmt_dn8), (-locals.var_vgbgmt_dn9),)
    } else {
        (locals.var_vgpld, locals.var_vgpld_dn2, locals.var_vgpld_dn7, locals.var_vgpld_dn8, locals.var_vgpld_dn9,)
    }
};
        locals.var_vgpld = assign75900_e115395;
        locals.var_vgpld_dn2 = assign75900_e115395_d_n2;
        locals.var_vgpld_dn7 = assign75900_e115395_d_n7;
        locals.var_vgpld_dn8 = assign75900_e115395_d_n8;
        locals.var_vgpld_dn9 = assign75900_e115395_d_n9;

        let (assign75910_e115404,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75910_e115398: f64 = (-locals.var_vxbgmtcl);
        let assign75910_e115401: f64 = (10.0 * 2.220446049250313e-16);
        let assign75910_e115402: f64 = (assign75910_e115398 + assign75910_e115401);
        (assign75910_e115402,)
    } else {
        (locals.var_vgb_fb_ld,)
    }
};
        locals.var_vgb_fb_ld = assign75910_e115404;

        let (assign75920_e115408, assign75920_e115408_d_n0, assign75920_e115408_d_n2, assign75920_e115408_d_n4, assign75920_e115408_d_n5, assign75920_e115408_d_n6, assign75920_e115408_d_n7, assign75920_e115408_d_n8, assign75920_e115408_d_n9, assign75920_e115408_d_n10, assign75920_e115408_d_n11, assign75920_e115408_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_dep_ld__blk1765, locals.var_q_dep_ld__blk1765_dn0, locals.var_q_dep_ld__blk1765_dn2, locals.var_q_dep_ld__blk1765_dn4, locals.var_q_dep_ld__blk1765_dn5, locals.var_q_dep_ld__blk1765_dn6, locals.var_q_dep_ld__blk1765_dn7, locals.var_q_dep_ld__blk1765_dn8, locals.var_q_dep_ld__blk1765_dn9, locals.var_q_dep_ld__blk1765_dn10, locals.var_q_dep_ld__blk1765_dn11, locals.var_q_dep_ld__blk1765_dn14,)
    }
};
        locals.var_q_dep_ld__blk1765 = assign75920_e115408;
        locals.var_q_dep_ld__blk1765_dn0 = assign75920_e115408_d_n0;
        locals.var_q_dep_ld__blk1765_dn2 = assign75920_e115408_d_n2;
        locals.var_q_dep_ld__blk1765_dn4 = assign75920_e115408_d_n4;
        locals.var_q_dep_ld__blk1765_dn5 = assign75920_e115408_d_n5;
        locals.var_q_dep_ld__blk1765_dn6 = assign75920_e115408_d_n6;
        locals.var_q_dep_ld__blk1765_dn7 = assign75920_e115408_d_n7;
        locals.var_q_dep_ld__blk1765_dn8 = assign75920_e115408_d_n8;
        locals.var_q_dep_ld__blk1765_dn9 = assign75920_e115408_d_n9;
        locals.var_q_dep_ld__blk1765_dn10 = assign75920_e115408_d_n10;
        locals.var_q_dep_ld__blk1765_dn11 = assign75920_e115408_d_n11;
        locals.var_q_dep_ld__blk1765_dn14 = assign75920_e115408_d_n14;

        let (assign75930_e115414,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75930_e115412: f64 = (1.6021918e-19 * locals.var_nover_func);
        (assign75930_e115412,)
    } else {
        (locals.var_q_nsubld__blk1766,)
    }
};
        locals.var_q_nsubld__blk1766 = assign75930_e115414;

        let (assign75940_e115420, assign75940_e115420_d_n0, assign75940_e115420_d_n2, assign75940_e115420_d_n4, assign75940_e115420_d_n5, assign75940_e115420_d_n6, assign75940_e115420_d_n7, assign75940_e115420_d_n8, assign75940_e115420_d_n9, assign75940_e115420_d_n10, assign75940_e115420_d_n11, assign75940_e115420_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75940_e115418: f64 = (locals.var_nin / locals.var_nover_func);
        (assign75940_e115418, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn11 / locals.var_nover_func), (locals.var_nin_dn14 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign75940_e115420;
        locals.var_t0_dn0 = assign75940_e115420_d_n0;
        locals.var_t0_dn2 = assign75940_e115420_d_n2;
        locals.var_t0_dn4 = assign75940_e115420_d_n4;
        locals.var_t0_dn5 = assign75940_e115420_d_n5;
        locals.var_t0_dn6 = assign75940_e115420_d_n6;
        locals.var_t0_dn7 = assign75940_e115420_d_n7;
        locals.var_t0_dn8 = assign75940_e115420_d_n8;
        locals.var_t0_dn9 = assign75940_e115420_d_n9;
        locals.var_t0_dn10 = assign75940_e115420_d_n10;
        locals.var_t0_dn11 = assign75940_e115420_d_n11;
        locals.var_t0_dn14 = assign75940_e115420_d_n14;

        let (assign75950_e115426, assign75950_e115426_d_n0, assign75950_e115426_d_n2, assign75950_e115426_d_n4, assign75950_e115426_d_n5, assign75950_e115426_d_n6, assign75950_e115426_d_n7, assign75950_e115426_d_n8, assign75950_e115426_d_n9, assign75950_e115426_d_n10, assign75950_e115426_d_n11, assign75950_e115426_d_n14,) = {
    if (locals.var_flg_calcqover != 0.0) {
        let assign75950_e115424: f64 = (locals.var_t0 * locals.var_t0);
        (assign75950_e115424, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn14,)
    }
};
        locals.var_cnst1over = assign75950_e115426;
        locals.var_cnst1over_dn0 = assign75950_e115426_d_n0;
        locals.var_cnst1over_dn2 = assign75950_e115426_d_n2;
        locals.var_cnst1over_dn4 = assign75950_e115426_d_n4;
        locals.var_cnst1over_dn5 = assign75950_e115426_d_n5;
        locals.var_cnst1over_dn6 = assign75950_e115426_d_n6;
        locals.var_cnst1over_dn7 = assign75950_e115426_d_n7;
        locals.var_cnst1over_dn8 = assign75950_e115426_d_n8;
        locals.var_cnst1over_dn9 = assign75950_e115426_d_n9;
        locals.var_cnst1over_dn10 = assign75950_e115426_d_n10;
        locals.var_cnst1over_dn11 = assign75950_e115426_d_n11;
        locals.var_cnst1over_dn14 = assign75950_e115426_d_n14;

        let assign75960_e115429: f64 = (-locals.var_vxbgmtcl);
        let assign75960_e115430: f64 = (locals.var_beta * assign75960_e115429);
        let assign75960_e115432: f64 = if assign75960_e115430 >= 500.0 { 1.0 } else { 0.0 };
        locals.var_guard1783 = assign75960_e115432;

        let (assign75970_e115447, assign75970_e115447_d_n0, assign75970_e115447_d_n2, assign75970_e115447_d_n4, assign75970_e115447_d_n5, assign75970_e115447_d_n6, assign75970_e115447_d_n7, assign75970_e115447_d_n8, assign75970_e115447_d_n9, assign75970_e115447_d_n10, assign75970_e115447_d_n11, assign75970_e115447_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1783 != 0.0)) {
        let assign75970_e115440: f64 = (-locals.var_vxbgmtcl);
        let assign75970_e115441: f64 = (locals.var_beta * assign75970_e115440);
        let assign75970_e115442: f64 = (1.0 + assign75970_e115441);
        let assign75970_e115444: f64 = (assign75970_e115442 - 500.0);
        let assign75970_e115445: f64 = (1.403592217853e217 * assign75970_e115444);
        (assign75970_e115445, (1.403592217853e217 * ((locals.var_beta_dn0 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (1.403592217853e217 * ((locals.var_beta_dn2 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (1.403592217853e217 * ((locals.var_beta_dn4 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (1.403592217853e217 * ((locals.var_beta_dn5 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (1.403592217853e217 * ((locals.var_beta_dn6 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (1.403592217853e217 * ((locals.var_beta_dn7 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (1.403592217853e217 * ((locals.var_beta_dn8 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (1.403592217853e217 * ((locals.var_beta_dn9 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (1.403592217853e217 * ((locals.var_beta_dn10 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (1.403592217853e217 * ((locals.var_beta_dn11 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11)))), (1.403592217853e217 * ((locals.var_beta_dn14 * assign75970_e115440) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign75970_e115447;
        locals.var_exp_bvbs_dn0 = assign75970_e115447_d_n0;
        locals.var_exp_bvbs_dn2 = assign75970_e115447_d_n2;
        locals.var_exp_bvbs_dn4 = assign75970_e115447_d_n4;
        locals.var_exp_bvbs_dn5 = assign75970_e115447_d_n5;
        locals.var_exp_bvbs_dn6 = assign75970_e115447_d_n6;
        locals.var_exp_bvbs_dn7 = assign75970_e115447_d_n7;
        locals.var_exp_bvbs_dn8 = assign75970_e115447_d_n8;
        locals.var_exp_bvbs_dn9 = assign75970_e115447_d_n9;
        locals.var_exp_bvbs_dn10 = assign75970_e115447_d_n10;
        locals.var_exp_bvbs_dn11 = assign75970_e115447_d_n11;
        locals.var_exp_bvbs_dn14 = assign75970_e115447_d_n14;

        let (assign75980_e115453, assign75980_e115453_d_n0, assign75980_e115453_d_n2, assign75980_e115453_d_n4, assign75980_e115453_d_n5, assign75980_e115453_d_n6, assign75980_e115453_d_n7, assign75980_e115453_d_n8, assign75980_e115453_d_n9, assign75980_e115453_d_n10, assign75980_e115453_d_n11, assign75980_e115453_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1783 != 0.0)) {
        (1.403592217853e217, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign75980_e115453;
        locals.var_t0_dn0 = assign75980_e115453_d_n0;
        locals.var_t0_dn2 = assign75980_e115453_d_n2;
        locals.var_t0_dn4 = assign75980_e115453_d_n4;
        locals.var_t0_dn5 = assign75980_e115453_d_n5;
        locals.var_t0_dn6 = assign75980_e115453_d_n6;
        locals.var_t0_dn7 = assign75980_e115453_d_n7;
        locals.var_t0_dn8 = assign75980_e115453_d_n8;
        locals.var_t0_dn9 = assign75980_e115453_d_n9;
        locals.var_t0_dn10 = assign75980_e115453_d_n10;
        locals.var_t0_dn11 = assign75980_e115453_d_n11;
        locals.var_t0_dn14 = assign75980_e115453_d_n14;

        let (assign75990_e115463, assign75990_e115463_d_n0, assign75990_e115463_d_n2, assign75990_e115463_d_n4, assign75990_e115463_d_n5, assign75990_e115463_d_n6, assign75990_e115463_d_n7, assign75990_e115463_d_n8, assign75990_e115463_d_n9, assign75990_e115463_d_n10, assign75990_e115463_d_n11, assign75990_e115463_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1783 == 0.0)) {
        let assign75990_e115460: f64 = (-locals.var_vxbgmtcl);
        let assign75990_e115461: f64 = (locals.var_beta * assign75990_e115460);
        (assign75990_e115461, ((locals.var_beta_dn0 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign75990_e115460) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign75990_e115463;
        locals.var_tmf1_dn0 = assign75990_e115463_d_n0;
        locals.var_tmf1_dn2 = assign75990_e115463_d_n2;
        locals.var_tmf1_dn4 = assign75990_e115463_d_n4;
        locals.var_tmf1_dn5 = assign75990_e115463_d_n5;
        locals.var_tmf1_dn6 = assign75990_e115463_d_n6;
        locals.var_tmf1_dn7 = assign75990_e115463_d_n7;
        locals.var_tmf1_dn8 = assign75990_e115463_d_n8;
        locals.var_tmf1_dn9 = assign75990_e115463_d_n9;
        locals.var_tmf1_dn10 = assign75990_e115463_d_n10;
        locals.var_tmf1_dn11 = assign75990_e115463_d_n11;
        locals.var_tmf1_dn14 = assign75990_e115463_d_n14;

        let (assign76000_e115470, assign76000_e115470_d_n0, assign76000_e115470_d_n2, assign76000_e115470_d_n4, assign76000_e115470_d_n5, assign76000_e115470_d_n6, assign76000_e115470_d_n7, assign76000_e115470_d_n8, assign76000_e115470_d_n9, assign76000_e115470_d_n10, assign76000_e115470_d_n11, assign76000_e115470_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1783 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign76000_e115470;
        locals.var_exp_bvbs_dn0 = assign76000_e115470_d_n0;
        locals.var_exp_bvbs_dn2 = assign76000_e115470_d_n2;
        locals.var_exp_bvbs_dn4 = assign76000_e115470_d_n4;
        locals.var_exp_bvbs_dn5 = assign76000_e115470_d_n5;
        locals.var_exp_bvbs_dn6 = assign76000_e115470_d_n6;
        locals.var_exp_bvbs_dn7 = assign76000_e115470_d_n7;
        locals.var_exp_bvbs_dn8 = assign76000_e115470_d_n8;
        locals.var_exp_bvbs_dn9 = assign76000_e115470_d_n9;
        locals.var_exp_bvbs_dn10 = assign76000_e115470_d_n10;
        locals.var_exp_bvbs_dn11 = assign76000_e115470_d_n11;
        locals.var_exp_bvbs_dn14 = assign76000_e115470_d_n14;

        let mut assign76010_loop_guard: usize = 0;
        while {
            let assign76010_cond_e115478: f64 = if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1783 == 0.0)) && (locals.var_tmf1 >= 60.0)) { 1.0 } else { 0.0 };
            assign76010_cond_e115478 != 0.0
        } {
            assign76010_loop_guard += 1;
            assert!(assign76010_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign76010_body0_e115487, assign76010_body0_e115487_d_n0, assign76010_body0_e115487_d_n2, assign76010_body0_e115487_d_n4, assign76010_body0_e115487_d_n5, assign76010_body0_e115487_d_n6, assign76010_body0_e115487_d_n7, assign76010_body0_e115487_d_n8, assign76010_body0_e115487_d_n9, assign76010_body0_e115487_d_n10, assign76010_body0_e115487_d_n11, assign76010_body0_e115487_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1783 == 0.0)) {
        let assign76010_body0_e115485: f64 = (locals.var_exp_bvbs * 1.14200738981568e26);
        (assign76010_body0_e115485, (locals.var_exp_bvbs_dn0 * 1.14200738981568e26), (locals.var_exp_bvbs_dn2 * 1.14200738981568e26), (locals.var_exp_bvbs_dn4 * 1.14200738981568e26), (locals.var_exp_bvbs_dn5 * 1.14200738981568e26), (locals.var_exp_bvbs_dn6 * 1.14200738981568e26), (locals.var_exp_bvbs_dn7 * 1.14200738981568e26), (locals.var_exp_bvbs_dn8 * 1.14200738981568e26), (locals.var_exp_bvbs_dn9 * 1.14200738981568e26), (locals.var_exp_bvbs_dn10 * 1.14200738981568e26), (locals.var_exp_bvbs_dn11 * 1.14200738981568e26), (locals.var_exp_bvbs_dn14 * 1.14200738981568e26),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
            locals.var_exp_bvbs = assign76010_body0_e115487;
            locals.var_exp_bvbs_dn0 = assign76010_body0_e115487_d_n0;
            locals.var_exp_bvbs_dn2 = assign76010_body0_e115487_d_n2;
            locals.var_exp_bvbs_dn4 = assign76010_body0_e115487_d_n4;
            locals.var_exp_bvbs_dn5 = assign76010_body0_e115487_d_n5;
            locals.var_exp_bvbs_dn6 = assign76010_body0_e115487_d_n6;
            locals.var_exp_bvbs_dn7 = assign76010_body0_e115487_d_n7;
            locals.var_exp_bvbs_dn8 = assign76010_body0_e115487_d_n8;
            locals.var_exp_bvbs_dn9 = assign76010_body0_e115487_d_n9;
            locals.var_exp_bvbs_dn10 = assign76010_body0_e115487_d_n10;
            locals.var_exp_bvbs_dn11 = assign76010_body0_e115487_d_n11;
            locals.var_exp_bvbs_dn14 = assign76010_body0_e115487_d_n14;
            let (assign76010_body1_e115496, assign76010_body1_e115496_d_n0, assign76010_body1_e115496_d_n2, assign76010_body1_e115496_d_n4, assign76010_body1_e115496_d_n5, assign76010_body1_e115496_d_n6, assign76010_body1_e115496_d_n7, assign76010_body1_e115496_d_n8, assign76010_body1_e115496_d_n9, assign76010_body1_e115496_d_n10, assign76010_body1_e115496_d_n11, assign76010_body1_e115496_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1783 == 0.0)) {
        let assign76010_body1_e115494: f64 = (locals.var_tmf1 - 60.0);
        (assign76010_body1_e115494, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
            locals.var_tmf1 = assign76010_body1_e115496;
            locals.var_tmf1_dn0 = assign76010_body1_e115496_d_n0;
            locals.var_tmf1_dn2 = assign76010_body1_e115496_d_n2;
            locals.var_tmf1_dn4 = assign76010_body1_e115496_d_n4;
            locals.var_tmf1_dn5 = assign76010_body1_e115496_d_n5;
            locals.var_tmf1_dn6 = assign76010_body1_e115496_d_n6;
            locals.var_tmf1_dn7 = assign76010_body1_e115496_d_n7;
            locals.var_tmf1_dn8 = assign76010_body1_e115496_d_n8;
            locals.var_tmf1_dn9 = assign76010_body1_e115496_d_n9;
            locals.var_tmf1_dn10 = assign76010_body1_e115496_d_n10;
            locals.var_tmf1_dn11 = assign76010_body1_e115496_d_n11;
            locals.var_tmf1_dn14 = assign76010_body1_e115496_d_n14;
        }

        let (assign76020_e115506, assign76020_e115506_d_n0, assign76020_e115506_d_n2, assign76020_e115506_d_n4, assign76020_e115506_d_n5, assign76020_e115506_d_n6, assign76020_e115506_d_n7, assign76020_e115506_d_n8, assign76020_e115506_d_n9, assign76020_e115506_d_n10, assign76020_e115506_d_n11, assign76020_e115506_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1783 == 0.0)) {
        let assign76020_e115503: f64 = (locals.var_tmf1).exp();
        let assign76020_e115504: f64 = (locals.var_exp_bvbs * assign76020_e115503);
        (assign76020_e115504, ((locals.var_exp_bvbs_dn0 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn0))), ((locals.var_exp_bvbs_dn2 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn2))), ((locals.var_exp_bvbs_dn4 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn4))), ((locals.var_exp_bvbs_dn5 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn5))), ((locals.var_exp_bvbs_dn6 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn6))), ((locals.var_exp_bvbs_dn7 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn7))), ((locals.var_exp_bvbs_dn8 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn8))), ((locals.var_exp_bvbs_dn9 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn9))), ((locals.var_exp_bvbs_dn10 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn10))), ((locals.var_exp_bvbs_dn11 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn11))), ((locals.var_exp_bvbs_dn14 * assign76020_e115503) + (locals.var_exp_bvbs * (assign76020_e115503 * locals.var_tmf1_dn14))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign76020_e115506;
        locals.var_exp_bvbs_dn0 = assign76020_e115506_d_n0;
        locals.var_exp_bvbs_dn2 = assign76020_e115506_d_n2;
        locals.var_exp_bvbs_dn4 = assign76020_e115506_d_n4;
        locals.var_exp_bvbs_dn5 = assign76020_e115506_d_n5;
        locals.var_exp_bvbs_dn6 = assign76020_e115506_d_n6;
        locals.var_exp_bvbs_dn7 = assign76020_e115506_d_n7;
        locals.var_exp_bvbs_dn8 = assign76020_e115506_d_n8;
        locals.var_exp_bvbs_dn9 = assign76020_e115506_d_n9;
        locals.var_exp_bvbs_dn10 = assign76020_e115506_d_n10;
        locals.var_exp_bvbs_dn11 = assign76020_e115506_d_n11;
        locals.var_exp_bvbs_dn14 = assign76020_e115506_d_n14;

        let (assign76030_e115513, assign76030_e115513_d_n0, assign76030_e115513_d_n2, assign76030_e115513_d_n4, assign76030_e115513_d_n5, assign76030_e115513_d_n6, assign76030_e115513_d_n7, assign76030_e115513_d_n8, assign76030_e115513_d_n9, assign76030_e115513_d_n10, assign76030_e115513_d_n11, assign76030_e115513_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1783 == 0.0)) {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign76030_e115513;
        locals.var_t0_dn0 = assign76030_e115513_d_n0;
        locals.var_t0_dn2 = assign76030_e115513_d_n2;
        locals.var_t0_dn4 = assign76030_e115513_d_n4;
        locals.var_t0_dn5 = assign76030_e115513_d_n5;
        locals.var_t0_dn6 = assign76030_e115513_d_n6;
        locals.var_t0_dn7 = assign76030_e115513_d_n7;
        locals.var_t0_dn8 = assign76030_e115513_d_n8;
        locals.var_t0_dn9 = assign76030_e115513_d_n9;
        locals.var_t0_dn10 = assign76030_e115513_d_n10;
        locals.var_t0_dn11 = assign76030_e115513_d_n11;
        locals.var_t0_dn14 = assign76030_e115513_d_n14;

    }

    pub(super) fn stamp_transient_block_274(
        locals: &mut StampLocals,
    ) {
        let (assign76040_e115526, assign76040_e115526_d_n0, assign76040_e115526_d_n2, assign76040_e115526_d_n4, assign76040_e115526_d_n5, assign76040_e115526_d_n6, assign76040_e115526_d_n7, assign76040_e115526_d_n8, assign76040_e115526_d_n9, assign76040_e115526_d_n10, assign76040_e115526_d_n11, assign76040_e115526_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76040_e115518: f64 = (-locals.var_vgpld);
        let assign76040_e115520: f64 = (assign76040_e115518 * 0.5);
        let assign76040_e115522: f64 = (assign76040_e115520 - 0.5);
        let assign76040_e115524: f64 = (assign76040_e115522 - 1.0);
        (assign76040_e115524, 0.0, ((-locals.var_vgpld_dn2) * 0.5), 0.0, 0.0, 0.0, ((-locals.var_vgpld_dn7) * 0.5), ((-locals.var_vgpld_dn8) * 0.5), ((-locals.var_vgpld_dn9) * 0.5), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign76040_e115526;
        locals.var_tmf1_dn0 = assign76040_e115526_d_n0;
        locals.var_tmf1_dn2 = assign76040_e115526_d_n2;
        locals.var_tmf1_dn4 = assign76040_e115526_d_n4;
        locals.var_tmf1_dn5 = assign76040_e115526_d_n5;
        locals.var_tmf1_dn6 = assign76040_e115526_d_n6;
        locals.var_tmf1_dn7 = assign76040_e115526_d_n7;
        locals.var_tmf1_dn8 = assign76040_e115526_d_n8;
        locals.var_tmf1_dn9 = assign76040_e115526_d_n9;
        locals.var_tmf1_dn10 = assign76040_e115526_d_n10;
        locals.var_tmf1_dn11 = assign76040_e115526_d_n11;
        locals.var_tmf1_dn14 = assign76040_e115526_d_n14;

        let (assign76050_e115536, assign76050_e115536_d_n0, assign76050_e115536_d_n2, assign76050_e115536_d_n4, assign76050_e115536_d_n5, assign76050_e115536_d_n6, assign76050_e115536_d_n7, assign76050_e115536_d_n8, assign76050_e115536_d_n9, assign76050_e115536_d_n10, assign76050_e115536_d_n11, assign76050_e115536_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76050_e115532: f64 = (4.0 * 0.5);
        let assign76050_e115534: f64 = assign76050_e115532;
        (assign76050_e115534, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign76050_e115536;
        locals.var_tmf2_dn0 = assign76050_e115536_d_n0;
        locals.var_tmf2_dn2 = assign76050_e115536_d_n2;
        locals.var_tmf2_dn4 = assign76050_e115536_d_n4;
        locals.var_tmf2_dn5 = assign76050_e115536_d_n5;
        locals.var_tmf2_dn6 = assign76050_e115536_d_n6;
        locals.var_tmf2_dn7 = assign76050_e115536_d_n7;
        locals.var_tmf2_dn8 = assign76050_e115536_d_n8;
        locals.var_tmf2_dn9 = assign76050_e115536_d_n9;
        locals.var_tmf2_dn10 = assign76050_e115536_d_n10;
        locals.var_tmf2_dn11 = assign76050_e115536_d_n11;
        locals.var_tmf2_dn14 = assign76050_e115536_d_n14;

        let (assign76060_e115548, assign76060_e115548_d_n0, assign76060_e115548_d_n2, assign76060_e115548_d_n4, assign76060_e115548_d_n5, assign76060_e115548_d_n6, assign76060_e115548_d_n7, assign76060_e115548_d_n8, assign76060_e115548_d_n9, assign76060_e115548_d_n10, assign76060_e115548_d_n11, assign76060_e115548_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let (assign76060_e115546, assign76060_e115546_d_n0, assign76060_e115546_d_n2, assign76060_e115546_d_n4, assign76060_e115546_d_n5, assign76060_e115546_d_n6, assign76060_e115546_d_n7, assign76060_e115546_d_n8, assign76060_e115546_d_n9, assign76060_e115546_d_n10, assign76060_e115546_d_n11, assign76060_e115546_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign76060_e115545: f64 = (-locals.var_tmf2);
                (assign76060_e115545, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign76060_e115546, assign76060_e115546_d_n0, assign76060_e115546_d_n2, assign76060_e115546_d_n4, assign76060_e115546_d_n5, assign76060_e115546_d_n6, assign76060_e115546_d_n7, assign76060_e115546_d_n8, assign76060_e115546_d_n9, assign76060_e115546_d_n10, assign76060_e115546_d_n11, assign76060_e115546_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign76060_e115548;
        locals.var_tmf2_dn0 = assign76060_e115548_d_n0;
        locals.var_tmf2_dn2 = assign76060_e115548_d_n2;
        locals.var_tmf2_dn4 = assign76060_e115548_d_n4;
        locals.var_tmf2_dn5 = assign76060_e115548_d_n5;
        locals.var_tmf2_dn6 = assign76060_e115548_d_n6;
        locals.var_tmf2_dn7 = assign76060_e115548_d_n7;
        locals.var_tmf2_dn8 = assign76060_e115548_d_n8;
        locals.var_tmf2_dn9 = assign76060_e115548_d_n9;
        locals.var_tmf2_dn10 = assign76060_e115548_d_n10;
        locals.var_tmf2_dn11 = assign76060_e115548_d_n11;
        locals.var_tmf2_dn14 = assign76060_e115548_d_n14;

        let (assign76070_e115559, assign76070_e115559_d_n0, assign76070_e115559_d_n2, assign76070_e115559_d_n4, assign76070_e115559_d_n5, assign76070_e115559_d_n6, assign76070_e115559_d_n7, assign76070_e115559_d_n8, assign76070_e115559_d_n9, assign76070_e115559_d_n10, assign76070_e115559_d_n11, assign76070_e115559_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76070_e115554: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign76070_e115556: f64 = (assign76070_e115554 + locals.var_tmf2);
        let assign76070_e115557: f64 = (assign76070_e115556).sqrt();
        (assign76070_e115557, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign76070_e115557)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign76070_e115557)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign76070_e115559;
        locals.var_tmf2_dn0 = assign76070_e115559_d_n0;
        locals.var_tmf2_dn2 = assign76070_e115559_d_n2;
        locals.var_tmf2_dn4 = assign76070_e115559_d_n4;
        locals.var_tmf2_dn5 = assign76070_e115559_d_n5;
        locals.var_tmf2_dn6 = assign76070_e115559_d_n6;
        locals.var_tmf2_dn7 = assign76070_e115559_d_n7;
        locals.var_tmf2_dn8 = assign76070_e115559_d_n8;
        locals.var_tmf2_dn9 = assign76070_e115559_d_n9;
        locals.var_tmf2_dn10 = assign76070_e115559_d_n10;
        locals.var_tmf2_dn11 = assign76070_e115559_d_n11;
        locals.var_tmf2_dn14 = assign76070_e115559_d_n14;

        let (assign76080_e115571, assign76080_e115571_d_n0, assign76080_e115571_d_n2, assign76080_e115571_d_n4, assign76080_e115571_d_n5, assign76080_e115571_d_n6, assign76080_e115571_d_n7, assign76080_e115571_d_n8, assign76080_e115571_d_n9, assign76080_e115571_d_n10, assign76080_e115571_d_n11, assign76080_e115571_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76080_e115567: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign76080_e115568: f64 = (1.0 + assign76080_e115567);
        let assign76080_e115569: f64 = (0.5 * assign76080_e115568);
        (assign76080_e115569, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign76080_e115571;
        locals.var_t0_dn0 = assign76080_e115571_d_n0;
        locals.var_t0_dn2 = assign76080_e115571_d_n2;
        locals.var_t0_dn4 = assign76080_e115571_d_n4;
        locals.var_t0_dn5 = assign76080_e115571_d_n5;
        locals.var_t0_dn6 = assign76080_e115571_d_n6;
        locals.var_t0_dn7 = assign76080_e115571_d_n7;
        locals.var_t0_dn8 = assign76080_e115571_d_n8;
        locals.var_t0_dn9 = assign76080_e115571_d_n9;
        locals.var_t0_dn10 = assign76080_e115571_d_n10;
        locals.var_t0_dn11 = assign76080_e115571_d_n11;
        locals.var_t0_dn14 = assign76080_e115571_d_n14;

        let (assign76090_e115583, assign76090_e115583_d_n0, assign76090_e115583_d_n2, assign76090_e115583_d_n4, assign76090_e115583_d_n5, assign76090_e115583_d_n6, assign76090_e115583_d_n7, assign76090_e115583_d_n8, assign76090_e115583_d_n9, assign76090_e115583_d_n10, assign76090_e115583_d_n11, assign76090_e115583_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76090_e115579: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign76090_e115580: f64 = (0.5 * assign76090_e115579);
        let assign76090_e115581: f64 = (0.5 + assign76090_e115580);
        (assign76090_e115581, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign76090_e115583;
        locals.var_t1_dn0 = assign76090_e115583_d_n0;
        locals.var_t1_dn2 = assign76090_e115583_d_n2;
        locals.var_t1_dn4 = assign76090_e115583_d_n4;
        locals.var_t1_dn5 = assign76090_e115583_d_n5;
        locals.var_t1_dn6 = assign76090_e115583_d_n6;
        locals.var_t1_dn7 = assign76090_e115583_d_n7;
        locals.var_t1_dn8 = assign76090_e115583_d_n8;
        locals.var_t1_dn9 = assign76090_e115583_d_n9;
        locals.var_t1_dn10 = assign76090_e115583_d_n10;
        locals.var_t1_dn11 = assign76090_e115583_d_n11;
        locals.var_t1_dn14 = assign76090_e115583_d_n14;

        let assign76100_e115586: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76100_e115589: f64 = (-locals.var_t1);
        let assign76100_e115594: f64 = if ((assign76100_e115586 > assign76100_e115589) && (locals.var_t1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1784 = assign76100_e115594;

        let (assign76110_e115608, assign76110_e115608_d_n0, assign76110_e115608_d_n2, assign76110_e115608_d_n4, assign76110_e115608_d_n5, assign76110_e115608_d_n6, assign76110_e115608_d_n7, assign76110_e115608_d_n8, assign76110_e115608_d_n9, assign76110_e115608_d_n10, assign76110_e115608_d_n11, assign76110_e115608_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76110_e115602: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76110_e115604: f64 = assign76110_e115602;
        let assign76110_e115606: f64 = (assign76110_e115604 + locals.var_t1);
        (assign76110_e115606, (locals.var_vxbgmtcl_dn0 + locals.var_t1_dn0), ((locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2) + locals.var_t1_dn2), (locals.var_vxbgmtcl_dn4 + locals.var_t1_dn4), (locals.var_vxbgmtcl_dn5 + locals.var_t1_dn5), (locals.var_vxbgmtcl_dn6 + locals.var_t1_dn6), ((locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7) + locals.var_t1_dn7), ((locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8) + locals.var_t1_dn8), ((locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9) + locals.var_t1_dn9), (locals.var_vxbgmtcl_dn10 + locals.var_t1_dn10), (locals.var_vxbgmtcl_dn11 + locals.var_t1_dn11), (locals.var_vxbgmtcl_dn14 + locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign76110_e115608;
        locals.var_tmf1_dn0 = assign76110_e115608_d_n0;
        locals.var_tmf1_dn2 = assign76110_e115608_d_n2;
        locals.var_tmf1_dn4 = assign76110_e115608_d_n4;
        locals.var_tmf1_dn5 = assign76110_e115608_d_n5;
        locals.var_tmf1_dn6 = assign76110_e115608_d_n6;
        locals.var_tmf1_dn7 = assign76110_e115608_d_n7;
        locals.var_tmf1_dn8 = assign76110_e115608_d_n8;
        locals.var_tmf1_dn9 = assign76110_e115608_d_n9;
        locals.var_tmf1_dn10 = assign76110_e115608_d_n10;
        locals.var_tmf1_dn11 = assign76110_e115608_d_n11;
        locals.var_tmf1_dn14 = assign76110_e115608_d_n14;

        let (assign76120_e115618, assign76120_e115618_d_n0, assign76120_e115618_d_n2, assign76120_e115618_d_n4, assign76120_e115618_d_n5, assign76120_e115618_d_n6, assign76120_e115618_d_n7, assign76120_e115618_d_n8, assign76120_e115618_d_n9, assign76120_e115618_d_n10, assign76120_e115618_d_n11, assign76120_e115618_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76120_e115616: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign76120_e115616, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign76120_e115618;
        locals.var_x2_dn0 = assign76120_e115618_d_n0;
        locals.var_x2_dn2 = assign76120_e115618_d_n2;
        locals.var_x2_dn4 = assign76120_e115618_d_n4;
        locals.var_x2_dn5 = assign76120_e115618_d_n5;
        locals.var_x2_dn6 = assign76120_e115618_d_n6;
        locals.var_x2_dn7 = assign76120_e115618_d_n7;
        locals.var_x2_dn8 = assign76120_e115618_d_n8;
        locals.var_x2_dn9 = assign76120_e115618_d_n9;
        locals.var_x2_dn10 = assign76120_e115618_d_n10;
        locals.var_x2_dn11 = assign76120_e115618_d_n11;
        locals.var_x2_dn14 = assign76120_e115618_d_n14;

        let (assign76130_e115628, assign76130_e115628_d_n0, assign76130_e115628_d_n2, assign76130_e115628_d_n4, assign76130_e115628_d_n5, assign76130_e115628_d_n6, assign76130_e115628_d_n7, assign76130_e115628_d_n8, assign76130_e115628_d_n9, assign76130_e115628_d_n10, assign76130_e115628_d_n11, assign76130_e115628_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76130_e115626: f64 = (locals.var_t1 * locals.var_t1);
        (assign76130_e115626, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign76130_e115628;
        locals.var_xmax2_dn0 = assign76130_e115628_d_n0;
        locals.var_xmax2_dn2 = assign76130_e115628_d_n2;
        locals.var_xmax2_dn4 = assign76130_e115628_d_n4;
        locals.var_xmax2_dn5 = assign76130_e115628_d_n5;
        locals.var_xmax2_dn6 = assign76130_e115628_d_n6;
        locals.var_xmax2_dn7 = assign76130_e115628_d_n7;
        locals.var_xmax2_dn8 = assign76130_e115628_d_n8;
        locals.var_xmax2_dn9 = assign76130_e115628_d_n9;
        locals.var_xmax2_dn10 = assign76130_e115628_d_n10;
        locals.var_xmax2_dn11 = assign76130_e115628_d_n11;
        locals.var_xmax2_dn14 = assign76130_e115628_d_n14;

        let (assign76140_e115636, assign76140_e115636_d_n0, assign76140_e115636_d_n2, assign76140_e115636_d_n4, assign76140_e115636_d_n5, assign76140_e115636_d_n6, assign76140_e115636_d_n7, assign76140_e115636_d_n8, assign76140_e115636_d_n9, assign76140_e115636_d_n10, assign76140_e115636_d_n11, assign76140_e115636_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign76140_e115636;
        locals.var_xp_dn0 = assign76140_e115636_d_n0;
        locals.var_xp_dn2 = assign76140_e115636_d_n2;
        locals.var_xp_dn4 = assign76140_e115636_d_n4;
        locals.var_xp_dn5 = assign76140_e115636_d_n5;
        locals.var_xp_dn6 = assign76140_e115636_d_n6;
        locals.var_xp_dn7 = assign76140_e115636_d_n7;
        locals.var_xp_dn8 = assign76140_e115636_d_n8;
        locals.var_xp_dn9 = assign76140_e115636_d_n9;
        locals.var_xp_dn10 = assign76140_e115636_d_n10;
        locals.var_xp_dn11 = assign76140_e115636_d_n11;
        locals.var_xp_dn14 = assign76140_e115636_d_n14;

        let (assign76150_e115644, assign76150_e115644_d_n0, assign76150_e115644_d_n2, assign76150_e115644_d_n4, assign76150_e115644_d_n5, assign76150_e115644_d_n6, assign76150_e115644_d_n7, assign76150_e115644_d_n8, assign76150_e115644_d_n9, assign76150_e115644_d_n10, assign76150_e115644_d_n11, assign76150_e115644_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign76150_e115644;
        locals.var_xmp_dn0 = assign76150_e115644_d_n0;
        locals.var_xmp_dn2 = assign76150_e115644_d_n2;
        locals.var_xmp_dn4 = assign76150_e115644_d_n4;
        locals.var_xmp_dn5 = assign76150_e115644_d_n5;
        locals.var_xmp_dn6 = assign76150_e115644_d_n6;
        locals.var_xmp_dn7 = assign76150_e115644_d_n7;
        locals.var_xmp_dn8 = assign76150_e115644_d_n8;
        locals.var_xmp_dn9 = assign76150_e115644_d_n9;
        locals.var_xmp_dn10 = assign76150_e115644_d_n10;
        locals.var_xmp_dn11 = assign76150_e115644_d_n11;
        locals.var_xmp_dn14 = assign76150_e115644_d_n14;

        let (assign76160_e115652,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign76160_e115652;

        let (assign76170_e115660,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76170_e115660;

        let (assign76180_e115668, assign76180_e115668_d_n0, assign76180_e115668_d_n2, assign76180_e115668_d_n4, assign76180_e115668_d_n5, assign76180_e115668_d_n6, assign76180_e115668_d_n7, assign76180_e115668_d_n8, assign76180_e115668_d_n9, assign76180_e115668_d_n10, assign76180_e115668_d_n11, assign76180_e115668_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign76180_e115668;
        locals.var_arg_dn0 = assign76180_e115668_d_n0;
        locals.var_arg_dn2 = assign76180_e115668_d_n2;
        locals.var_arg_dn4 = assign76180_e115668_d_n4;
        locals.var_arg_dn5 = assign76180_e115668_d_n5;
        locals.var_arg_dn6 = assign76180_e115668_d_n6;
        locals.var_arg_dn7 = assign76180_e115668_d_n7;
        locals.var_arg_dn8 = assign76180_e115668_d_n8;
        locals.var_arg_dn9 = assign76180_e115668_d_n9;
        locals.var_arg_dn10 = assign76180_e115668_d_n10;
        locals.var_arg_dn11 = assign76180_e115668_d_n11;
        locals.var_arg_dn14 = assign76180_e115668_d_n14;

        let (assign76190_e115676, assign76190_e115676_d_n0, assign76190_e115676_d_n2, assign76190_e115676_d_n4, assign76190_e115676_d_n5, assign76190_e115676_d_n6, assign76190_e115676_d_n7, assign76190_e115676_d_n8, assign76190_e115676_d_n9, assign76190_e115676_d_n10, assign76190_e115676_d_n11, assign76190_e115676_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign76190_e115676;
        locals.var_dnm_dn0 = assign76190_e115676_d_n0;
        locals.var_dnm_dn2 = assign76190_e115676_d_n2;
        locals.var_dnm_dn4 = assign76190_e115676_d_n4;
        locals.var_dnm_dn5 = assign76190_e115676_d_n5;
        locals.var_dnm_dn6 = assign76190_e115676_d_n6;
        locals.var_dnm_dn7 = assign76190_e115676_d_n7;
        locals.var_dnm_dn8 = assign76190_e115676_d_n8;
        locals.var_dnm_dn9 = assign76190_e115676_d_n9;
        locals.var_dnm_dn10 = assign76190_e115676_d_n10;
        locals.var_dnm_dn11 = assign76190_e115676_d_n11;
        locals.var_dnm_dn14 = assign76190_e115676_d_n14;

        let (assign76200_e115686, assign76200_e115686_d_n0, assign76200_e115686_d_n2, assign76200_e115686_d_n4, assign76200_e115686_d_n5, assign76200_e115686_d_n6, assign76200_e115686_d_n7, assign76200_e115686_d_n8, assign76200_e115686_d_n9, assign76200_e115686_d_n10, assign76200_e115686_d_n11, assign76200_e115686_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76200_e115684: f64 = (locals.var_xp * locals.var_x2);
        (assign76200_e115684, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign76200_e115686;
        locals.var_xp_dn0 = assign76200_e115686_d_n0;
        locals.var_xp_dn2 = assign76200_e115686_d_n2;
        locals.var_xp_dn4 = assign76200_e115686_d_n4;
        locals.var_xp_dn5 = assign76200_e115686_d_n5;
        locals.var_xp_dn6 = assign76200_e115686_d_n6;
        locals.var_xp_dn7 = assign76200_e115686_d_n7;
        locals.var_xp_dn8 = assign76200_e115686_d_n8;
        locals.var_xp_dn9 = assign76200_e115686_d_n9;
        locals.var_xp_dn10 = assign76200_e115686_d_n10;
        locals.var_xp_dn11 = assign76200_e115686_d_n11;
        locals.var_xp_dn14 = assign76200_e115686_d_n14;

        let (assign76210_e115696, assign76210_e115696_d_n0, assign76210_e115696_d_n2, assign76210_e115696_d_n4, assign76210_e115696_d_n5, assign76210_e115696_d_n6, assign76210_e115696_d_n7, assign76210_e115696_d_n8, assign76210_e115696_d_n9, assign76210_e115696_d_n10, assign76210_e115696_d_n11, assign76210_e115696_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76210_e115694: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign76210_e115694, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign76210_e115696;
        locals.var_xmp_dn0 = assign76210_e115696_d_n0;
        locals.var_xmp_dn2 = assign76210_e115696_d_n2;
        locals.var_xmp_dn4 = assign76210_e115696_d_n4;
        locals.var_xmp_dn5 = assign76210_e115696_d_n5;
        locals.var_xmp_dn6 = assign76210_e115696_d_n6;
        locals.var_xmp_dn7 = assign76210_e115696_d_n7;
        locals.var_xmp_dn8 = assign76210_e115696_d_n8;
        locals.var_xmp_dn9 = assign76210_e115696_d_n9;
        locals.var_xmp_dn10 = assign76210_e115696_d_n10;
        locals.var_xmp_dn11 = assign76210_e115696_d_n11;
        locals.var_xmp_dn14 = assign76210_e115696_d_n14;

        let (assign76220_e115706, assign76220_e115706_d_n0, assign76220_e115706_d_n2, assign76220_e115706_d_n4, assign76220_e115706_d_n5, assign76220_e115706_d_n6, assign76220_e115706_d_n7, assign76220_e115706_d_n8, assign76220_e115706_d_n9, assign76220_e115706_d_n10, assign76220_e115706_d_n11, assign76220_e115706_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76220_e115704: f64 = (locals.var_xp + locals.var_xmp);
        (assign76220_e115704, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign76220_e115706;
        locals.var_arg_dn0 = assign76220_e115706_d_n0;
        locals.var_arg_dn2 = assign76220_e115706_d_n2;
        locals.var_arg_dn4 = assign76220_e115706_d_n4;
        locals.var_arg_dn5 = assign76220_e115706_d_n5;
        locals.var_arg_dn6 = assign76220_e115706_d_n6;
        locals.var_arg_dn7 = assign76220_e115706_d_n7;
        locals.var_arg_dn8 = assign76220_e115706_d_n8;
        locals.var_arg_dn9 = assign76220_e115706_d_n9;
        locals.var_arg_dn10 = assign76220_e115706_d_n10;
        locals.var_arg_dn11 = assign76220_e115706_d_n11;
        locals.var_arg_dn14 = assign76220_e115706_d_n14;

        let (assign76230_e115714, assign76230_e115714_d_n0, assign76230_e115714_d_n2, assign76230_e115714_d_n4, assign76230_e115714_d_n5, assign76230_e115714_d_n6, assign76230_e115714_d_n7, assign76230_e115714_d_n8, assign76230_e115714_d_n9, assign76230_e115714_d_n10, assign76230_e115714_d_n11, assign76230_e115714_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign76230_e115714;
        locals.var_dnm_dn0 = assign76230_e115714_d_n0;
        locals.var_dnm_dn2 = assign76230_e115714_d_n2;
        locals.var_dnm_dn4 = assign76230_e115714_d_n4;
        locals.var_dnm_dn5 = assign76230_e115714_d_n5;
        locals.var_dnm_dn6 = assign76230_e115714_d_n6;
        locals.var_dnm_dn7 = assign76230_e115714_d_n7;
        locals.var_dnm_dn8 = assign76230_e115714_d_n8;
        locals.var_dnm_dn9 = assign76230_e115714_d_n9;
        locals.var_dnm_dn10 = assign76230_e115714_d_n10;
        locals.var_dnm_dn11 = assign76230_e115714_d_n11;
        locals.var_dnm_dn14 = assign76230_e115714_d_n14;

        let assign76240_e115729: f64 = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1785 = assign76240_e115729;

        let assign76250_e115732: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1786 = assign76250_e115732;

        let (assign76260_e115744,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) && (locals.var_guard1786 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76260_e115744;

        let assign76270_e115747: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1787 = assign76270_e115747;

        let (assign76280_e115762,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1787 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76280_e115762;

        let assign76290_e115765: f64 = if 1.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1788 = assign76290_e115765;

        let (assign76300_e115783,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1787 == 0.0)) && (locals.var_guard1788 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76300_e115783;

        let assign76310_e115786: f64 = if 1.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1789 = assign76310_e115786;

        let (assign76320_e115807,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) && (locals.var_guard1786 == 0.0)) && (locals.var_guard1787 == 0.0)) && (locals.var_guard1788 == 0.0)) && (locals.var_guard1789 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign76320_e115807;

        let (assign76330_e115817,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign76330_e115817;

        let mut assign76340_loop_guard: usize = 0;
        while {
            let assign76340_cond_e115828: f64 = if (((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign76340_cond_e115828 != 0.0
        } {
            assign76340_loop_guard += 1;
            assert!(assign76340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign76340_body0_e115839, assign76340_body0_e115839_d_n0, assign76340_body0_e115839_d_n2, assign76340_body0_e115839_d_n4, assign76340_body0_e115839_d_n5, assign76340_body0_e115839_d_n6, assign76340_body0_e115839_d_n7, assign76340_body0_e115839_d_n8, assign76340_body0_e115839_d_n9, assign76340_body0_e115839_d_n10, assign76340_body0_e115839_d_n11, assign76340_body0_e115839_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) {
        let assign76340_body0_e115837: f64 = (locals.var_dnm).sqrt();
        (assign76340_body0_e115837, (locals.var_dnm_dn0 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn2 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn4 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn5 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn6 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn7 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn8 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn9 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn10 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn11 / (2.0 * assign76340_body0_e115837)), (locals.var_dnm_dn14 / (2.0 * assign76340_body0_e115837)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign76340_body0_e115839;
            locals.var_dnm_dn0 = assign76340_body0_e115839_d_n0;
            locals.var_dnm_dn2 = assign76340_body0_e115839_d_n2;
            locals.var_dnm_dn4 = assign76340_body0_e115839_d_n4;
            locals.var_dnm_dn5 = assign76340_body0_e115839_d_n5;
            locals.var_dnm_dn6 = assign76340_body0_e115839_d_n6;
            locals.var_dnm_dn7 = assign76340_body0_e115839_d_n7;
            locals.var_dnm_dn8 = assign76340_body0_e115839_d_n8;
            locals.var_dnm_dn9 = assign76340_body0_e115839_d_n9;
            locals.var_dnm_dn10 = assign76340_body0_e115839_d_n10;
            locals.var_dnm_dn11 = assign76340_body0_e115839_d_n11;
            locals.var_dnm_dn14 = assign76340_body0_e115839_d_n14;
            let (assign76340_body1_e115851,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 != 0.0)) {
        let assign76340_body1_e115849: f64 = (locals.var_m0 + 1.0);
        (assign76340_body1_e115849,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign76340_body1_e115851;
        }

    }

    pub(super) fn stamp_transient_block_275(
        locals: &mut StampLocals,
    ) {
        let (assign76350_e115873, assign76350_e115873_d_n0, assign76350_e115873_d_n2, assign76350_e115873_d_n4, assign76350_e115873_d_n5, assign76350_e115873_d_n6, assign76350_e115873_d_n7, assign76350_e115873_d_n8, assign76350_e115873_d_n9, assign76350_e115873_d_n10, assign76350_e115873_d_n11, assign76350_e115873_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) && (locals.var_guard1785 == 0.0)) {
        let (assign76350_e115871, assign76350_e115871_d_n0, assign76350_e115871_d_n2, assign76350_e115871_d_n4, assign76350_e115871_d_n5, assign76350_e115871_d_n6, assign76350_e115871_d_n7, assign76350_e115871_d_n8, assign76350_e115871_d_n9, assign76350_e115871_d_n10, assign76350_e115871_d_n11, assign76350_e115871_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign76350_e115868: f64 = 2.0;
                let assign76350_e115869: f64 = (1.0 / assign76350_e115868);
                let assign76350_e115870: f64 = (locals.var_dnm).powf(assign76350_e115869);
                (assign76350_e115870, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn0)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn2)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn4)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn5)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn6)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn7)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn8)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn9)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn10)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn11)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign76350_e115869) as f64).is_finite() && ((assign76350_e115869) as f64).fract() == 0.0 { if assign76350_e115869 == 0.0 { 0.0 } else { (assign76350_e115869 * ((locals.var_dnm).powf(assign76350_e115869 - 1.0) * locals.var_dnm_dn14)) } } else { (assign76350_e115870 * (assign76350_e115869 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign76350_e115871, assign76350_e115871_d_n0, assign76350_e115871_d_n2, assign76350_e115871_d_n4, assign76350_e115871_d_n5, assign76350_e115871_d_n6, assign76350_e115871_d_n7, assign76350_e115871_d_n8, assign76350_e115871_d_n9, assign76350_e115871_d_n10, assign76350_e115871_d_n11, assign76350_e115871_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign76350_e115873;
        locals.var_dnm_dn0 = assign76350_e115873_d_n0;
        locals.var_dnm_dn2 = assign76350_e115873_d_n2;
        locals.var_dnm_dn4 = assign76350_e115873_d_n4;
        locals.var_dnm_dn5 = assign76350_e115873_d_n5;
        locals.var_dnm_dn6 = assign76350_e115873_d_n6;
        locals.var_dnm_dn7 = assign76350_e115873_d_n7;
        locals.var_dnm_dn8 = assign76350_e115873_d_n8;
        locals.var_dnm_dn9 = assign76350_e115873_d_n9;
        locals.var_dnm_dn10 = assign76350_e115873_d_n10;
        locals.var_dnm_dn11 = assign76350_e115873_d_n11;
        locals.var_dnm_dn14 = assign76350_e115873_d_n14;

        let (assign76360_e115883, assign76360_e115883_d_n0, assign76360_e115883_d_n2, assign76360_e115883_d_n4, assign76360_e115883_d_n5, assign76360_e115883_d_n6, assign76360_e115883_d_n7, assign76360_e115883_d_n8, assign76360_e115883_d_n9, assign76360_e115883_d_n10, assign76360_e115883_d_n11, assign76360_e115883_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76360_e115881: f64 = (1.0 / locals.var_dnm);
        (assign76360_e115881, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign76360_e115883;
        locals.var_dnm_dn0 = assign76360_e115883_d_n0;
        locals.var_dnm_dn2 = assign76360_e115883_d_n2;
        locals.var_dnm_dn4 = assign76360_e115883_d_n4;
        locals.var_dnm_dn5 = assign76360_e115883_d_n5;
        locals.var_dnm_dn6 = assign76360_e115883_d_n6;
        locals.var_dnm_dn7 = assign76360_e115883_d_n7;
        locals.var_dnm_dn8 = assign76360_e115883_d_n8;
        locals.var_dnm_dn9 = assign76360_e115883_d_n9;
        locals.var_dnm_dn10 = assign76360_e115883_d_n10;
        locals.var_dnm_dn11 = assign76360_e115883_d_n11;
        locals.var_dnm_dn14 = assign76360_e115883_d_n14;

        let (assign76370_e115895, assign76370_e115895_d_n0, assign76370_e115895_d_n2, assign76370_e115895_d_n4, assign76370_e115895_d_n5, assign76370_e115895_d_n6, assign76370_e115895_d_n7, assign76370_e115895_d_n8, assign76370_e115895_d_n9, assign76370_e115895_d_n10, assign76370_e115895_d_n11, assign76370_e115895_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76370_e115891: f64 = (locals.var_tmf1 * locals.var_t1);
        let assign76370_e115893: f64 = (assign76370_e115891 * locals.var_dnm);
        (assign76370_e115893, ((((locals.var_tmf1_dn0 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn0)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn2)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn4)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn5)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn6)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn7)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn8)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn9)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn10)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn11)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t1) + (locals.var_tmf1 * locals.var_t1_dn14)) * locals.var_dnm) + (assign76370_e115891 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign76370_e115895;
        locals.var_tmf0_dn0 = assign76370_e115895_d_n0;
        locals.var_tmf0_dn2 = assign76370_e115895_d_n2;
        locals.var_tmf0_dn4 = assign76370_e115895_d_n4;
        locals.var_tmf0_dn5 = assign76370_e115895_d_n5;
        locals.var_tmf0_dn6 = assign76370_e115895_d_n6;
        locals.var_tmf0_dn7 = assign76370_e115895_d_n7;
        locals.var_tmf0_dn8 = assign76370_e115895_d_n8;
        locals.var_tmf0_dn9 = assign76370_e115895_d_n9;
        locals.var_tmf0_dn10 = assign76370_e115895_d_n10;
        locals.var_tmf0_dn11 = assign76370_e115895_d_n11;
        locals.var_tmf0_dn14 = assign76370_e115895_d_n14;

        let (assign76380_e115909, assign76380_e115909_d_n0, assign76380_e115909_d_n2, assign76380_e115909_d_n4, assign76380_e115909_d_n5, assign76380_e115909_d_n6, assign76380_e115909_d_n7, assign76380_e115909_d_n8, assign76380_e115909_d_n9, assign76380_e115909_d_n10, assign76380_e115909_d_n11, assign76380_e115909_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76380_e115903: f64 = (locals.var_t1 * locals.var_xmp);
        let assign76380_e115905: f64 = (assign76380_e115903 * locals.var_dnm);
        let assign76380_e115907: f64 = (assign76380_e115905 / locals.var_arg);
        (assign76380_e115907, (((((((locals.var_t1_dn0 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn0)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn2 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn2)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn4 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn4)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn5 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn5)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn6 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn6)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn7 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn7)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn8 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn8)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn9 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn9)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn10 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn10)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn11 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn11)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t1_dn14 * locals.var_xmp) + (locals.var_t1 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign76380_e115903 * locals.var_dnm_dn14)) * locals.var_arg) - (assign76380_e115905 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign76380_e115909;
        locals.var_t0_dn0 = assign76380_e115909_d_n0;
        locals.var_t0_dn2 = assign76380_e115909_d_n2;
        locals.var_t0_dn4 = assign76380_e115909_d_n4;
        locals.var_t0_dn5 = assign76380_e115909_d_n5;
        locals.var_t0_dn6 = assign76380_e115909_d_n6;
        locals.var_t0_dn7 = assign76380_e115909_d_n7;
        locals.var_t0_dn8 = assign76380_e115909_d_n8;
        locals.var_t0_dn9 = assign76380_e115909_d_n9;
        locals.var_t0_dn10 = assign76380_e115909_d_n10;
        locals.var_t0_dn11 = assign76380_e115909_d_n11;
        locals.var_t0_dn14 = assign76380_e115909_d_n14;

        let (assign76390_e115921, assign76390_e115921_d_n0, assign76390_e115921_d_n2, assign76390_e115921_d_n4, assign76390_e115921_d_n5, assign76390_e115921_d_n6, assign76390_e115921_d_n7, assign76390_e115921_d_n8, assign76390_e115921_d_n9, assign76390_e115921_d_n10, assign76390_e115921_d_n11, assign76390_e115921_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        let assign76390_e115917: f64 = (-locals.var_t1);
        let assign76390_e115919: f64 = (assign76390_e115917 + locals.var_tmf0);
        (assign76390_e115919, ((-locals.var_t1_dn0) + locals.var_tmf0_dn0), ((-locals.var_t1_dn2) + locals.var_tmf0_dn2), ((-locals.var_t1_dn4) + locals.var_tmf0_dn4), ((-locals.var_t1_dn5) + locals.var_tmf0_dn5), ((-locals.var_t1_dn6) + locals.var_tmf0_dn6), ((-locals.var_t1_dn7) + locals.var_tmf0_dn7), ((-locals.var_t1_dn8) + locals.var_tmf0_dn8), ((-locals.var_t1_dn9) + locals.var_tmf0_dn9), ((-locals.var_t1_dn10) + locals.var_tmf0_dn10), ((-locals.var_t1_dn11) + locals.var_tmf0_dn11), ((-locals.var_t1_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign76390_e115921;
        locals.var_t1_dn0 = assign76390_e115921_d_n0;
        locals.var_t1_dn2 = assign76390_e115921_d_n2;
        locals.var_t1_dn4 = assign76390_e115921_d_n4;
        locals.var_t1_dn5 = assign76390_e115921_d_n5;
        locals.var_t1_dn6 = assign76390_e115921_d_n6;
        locals.var_t1_dn7 = assign76390_e115921_d_n7;
        locals.var_t1_dn8 = assign76390_e115921_d_n8;
        locals.var_t1_dn9 = assign76390_e115921_d_n9;
        locals.var_t1_dn10 = assign76390_e115921_d_n10;
        locals.var_t1_dn11 = assign76390_e115921_d_n11;
        locals.var_t1_dn14 = assign76390_e115921_d_n14;

        let (assign76400_e115929, assign76400_e115929_d_n0, assign76400_e115929_d_n2, assign76400_e115929_d_n4, assign76400_e115929_d_n5, assign76400_e115929_d_n6, assign76400_e115929_d_n7, assign76400_e115929_d_n8, assign76400_e115929_d_n9, assign76400_e115929_d_n10, assign76400_e115929_d_n11, assign76400_e115929_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign76400_e115929;
        locals.var_t0_dn0 = assign76400_e115929_d_n0;
        locals.var_t0_dn2 = assign76400_e115929_d_n2;
        locals.var_t0_dn4 = assign76400_e115929_d_n4;
        locals.var_t0_dn5 = assign76400_e115929_d_n5;
        locals.var_t0_dn6 = assign76400_e115929_d_n6;
        locals.var_t0_dn7 = assign76400_e115929_d_n7;
        locals.var_t0_dn8 = assign76400_e115929_d_n8;
        locals.var_t0_dn9 = assign76400_e115929_d_n9;
        locals.var_t0_dn10 = assign76400_e115929_d_n10;
        locals.var_t0_dn11 = assign76400_e115929_d_n11;
        locals.var_t0_dn14 = assign76400_e115929_d_n14;

        let (assign76410_e115940, assign76410_e115940_d_n0, assign76410_e115940_d_n2, assign76410_e115940_d_n4, assign76410_e115940_d_n5, assign76410_e115940_d_n6, assign76410_e115940_d_n7, assign76410_e115940_d_n8, assign76410_e115940_d_n9, assign76410_e115940_d_n10, assign76410_e115940_d_n11, assign76410_e115940_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 == 0.0)) {
        let assign76410_e115938: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        (assign76410_e115938, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9), locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign76410_e115940;
        locals.var_t1_dn0 = assign76410_e115940_d_n0;
        locals.var_t1_dn2 = assign76410_e115940_d_n2;
        locals.var_t1_dn4 = assign76410_e115940_d_n4;
        locals.var_t1_dn5 = assign76410_e115940_d_n5;
        locals.var_t1_dn6 = assign76410_e115940_d_n6;
        locals.var_t1_dn7 = assign76410_e115940_d_n7;
        locals.var_t1_dn8 = assign76410_e115940_d_n8;
        locals.var_t1_dn9 = assign76410_e115940_d_n9;
        locals.var_t1_dn10 = assign76410_e115940_d_n10;
        locals.var_t1_dn11 = assign76410_e115940_d_n11;
        locals.var_t1_dn14 = assign76410_e115940_d_n14;

        let (assign76420_e115949, assign76420_e115949_d_n0, assign76420_e115949_d_n2, assign76420_e115949_d_n4, assign76420_e115949_d_n5, assign76420_e115949_d_n6, assign76420_e115949_d_n7, assign76420_e115949_d_n8, assign76420_e115949_d_n9, assign76420_e115949_d_n10, assign76420_e115949_d_n11, assign76420_e115949_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) && (locals.var_guard1784 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign76420_e115949;
        locals.var_t0_dn0 = assign76420_e115949_d_n0;
        locals.var_t0_dn2 = assign76420_e115949_d_n2;
        locals.var_t0_dn4 = assign76420_e115949_d_n4;
        locals.var_t0_dn5 = assign76420_e115949_d_n5;
        locals.var_t0_dn6 = assign76420_e115949_d_n6;
        locals.var_t0_dn7 = assign76420_e115949_d_n7;
        locals.var_t0_dn8 = assign76420_e115949_d_n8;
        locals.var_t0_dn9 = assign76420_e115949_d_n9;
        locals.var_t0_dn10 = assign76420_e115949_d_n10;
        locals.var_t0_dn11 = assign76420_e115949_d_n11;
        locals.var_t0_dn14 = assign76420_e115949_d_n14;

        let (assign76430_e115957, assign76430_e115957_d_n0, assign76430_e115957_d_n2, assign76430_e115957_d_n4, assign76430_e115957_d_n5, assign76430_e115957_d_n6, assign76430_e115957_d_n7, assign76430_e115957_d_n8, assign76430_e115957_d_n9, assign76430_e115957_d_n10, assign76430_e115957_d_n11, assign76430_e115957_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76430_e115955: f64 = (locals.var_t1 - locals.var_vgpld);
        (assign76430_e115955, locals.var_t1_dn0, (locals.var_t1_dn2 - locals.var_vgpld_dn2), locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, (locals.var_t1_dn7 - locals.var_vgpld_dn7), (locals.var_t1_dn8 - locals.var_vgpld_dn8), (locals.var_t1_dn9 - locals.var_vgpld_dn9), locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_vxbgmtcl, locals.var_vxbgmtcl_dn0, locals.var_vxbgmtcl_dn2, locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, locals.var_vxbgmtcl_dn7, locals.var_vxbgmtcl_dn8, locals.var_vxbgmtcl_dn9, locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    }
};
        locals.var_vxbgmtcl = assign76430_e115957;
        locals.var_vxbgmtcl_dn0 = assign76430_e115957_d_n0;
        locals.var_vxbgmtcl_dn2 = assign76430_e115957_d_n2;
        locals.var_vxbgmtcl_dn4 = assign76430_e115957_d_n4;
        locals.var_vxbgmtcl_dn5 = assign76430_e115957_d_n5;
        locals.var_vxbgmtcl_dn6 = assign76430_e115957_d_n6;
        locals.var_vxbgmtcl_dn7 = assign76430_e115957_d_n7;
        locals.var_vxbgmtcl_dn8 = assign76430_e115957_d_n8;
        locals.var_vxbgmtcl_dn9 = assign76430_e115957_d_n9;
        locals.var_vxbgmtcl_dn10 = assign76430_e115957_d_n10;
        locals.var_vxbgmtcl_dn11 = assign76430_e115957_d_n11;
        locals.var_vxbgmtcl_dn14 = assign76430_e115957_d_n14;

        let (assign76440_e115968,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_flg_never_reach_vfbover != 0.0)) {
        let assign76440_e115962: f64 = (-locals.var_vxbgmtcl);
        let assign76440_e115965: f64 = (10.0 * 2.220446049250313e-16);
        let assign76440_e115966: f64 = (assign76440_e115962 + assign76440_e115965);
        (assign76440_e115966,)
    } else {
        (locals.var_vgb_fb_ld,)
    }
};
        locals.var_vgb_fb_ld = assign76440_e115968;

        let assign76450_e115971: f64 = if locals.var_vgpld < locals.var_vgb_fb_ld { 1.0 } else { 0.0 };
        locals.var_guard1790 = assign76450_e115971;

        let (assign76470_e115992, assign76470_e115992_d_n0, assign76470_e115992_d_n2, assign76470_e115992_d_n4, assign76470_e115992_d_n5, assign76470_e115992_d_n6, assign76470_e115992_d_n7, assign76470_e115992_d_n8, assign76470_e115992_d_n9, assign76470_e115992_d_n10, assign76470_e115992_d_n11, assign76470_e115992_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76470_e115984: f64 = (2.0 * locals.var_beta_inv);
        let assign76470_e115986: f64 = (-locals.var_vgs_min);
        let assign76470_e115988: f64 = (assign76470_e115986 / locals.var_fac1);
        let assign76470_e115989: f64 = (assign76470_e115988).ln();
        let assign76470_e115990: f64 = (assign76470_e115984 * assign76470_e115989);
        (assign76470_e115990, (((2.0 * locals.var_beta_inv_dn0) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn0) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn2) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn2) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn4) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn4) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn5) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn5) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn6) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn6) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn7) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn7) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn8) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn8) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn9) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn9) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn10) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn10) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn11) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn11) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))), (((2.0 * locals.var_beta_inv_dn14) * assign76470_e115989) + (assign76470_e115984 * ((-((assign76470_e115986 * locals.var_fac1_dn14) / (locals.var_fac1 * locals.var_fac1))) / assign76470_e115988))),)
    } else {
        (locals.var_ps0_min, locals.var_ps0_min_dn0, locals.var_ps0_min_dn2, locals.var_ps0_min_dn4, locals.var_ps0_min_dn5, locals.var_ps0_min_dn6, locals.var_ps0_min_dn7, locals.var_ps0_min_dn8, locals.var_ps0_min_dn9, locals.var_ps0_min_dn10, locals.var_ps0_min_dn11, locals.var_ps0_min_dn14,)
    }
};
        locals.var_ps0_min = assign76470_e115992;
        locals.var_ps0_min_dn0 = assign76470_e115992_d_n0;
        locals.var_ps0_min_dn2 = assign76470_e115992_d_n2;
        locals.var_ps0_min_dn4 = assign76470_e115992_d_n4;
        locals.var_ps0_min_dn5 = assign76470_e115992_d_n5;
        locals.var_ps0_min_dn6 = assign76470_e115992_d_n6;
        locals.var_ps0_min_dn7 = assign76470_e115992_d_n7;
        locals.var_ps0_min_dn8 = assign76470_e115992_d_n8;
        locals.var_ps0_min_dn9 = assign76470_e115992_d_n9;
        locals.var_ps0_min_dn10 = assign76470_e115992_d_n10;
        locals.var_ps0_min_dn11 = assign76470_e115992_d_n11;
        locals.var_ps0_min_dn14 = assign76470_e115992_d_n14;

        let (assign76480_e116002, assign76480_e116002_d_n0, assign76480_e116002_d_n2, assign76480_e116002_d_n4, assign76480_e116002_d_n5, assign76480_e116002_d_n6, assign76480_e116002_d_n7, assign76480_e116002_d_n8, assign76480_e116002_d_n9, assign76480_e116002_d_n10, assign76480_e116002_d_n11, assign76480_e116002_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76480_e115999: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76480_e116000: f64 = (locals.var_beta * assign76480_e115999);
        (assign76480_e116000, ((locals.var_beta_dn0 * assign76480_e115999) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((locals.var_beta_dn2 * assign76480_e115999) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign76480_e115999) + (locals.var_beta * locals.var_vxbgmtcl_dn4)), ((locals.var_beta_dn5 * assign76480_e115999) + (locals.var_beta * locals.var_vxbgmtcl_dn5)), ((locals.var_beta_dn6 * assign76480_e115999) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((locals.var_beta_dn7 * assign76480_e115999) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign76480_e115999) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign76480_e115999) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign76480_e115999) + (locals.var_beta * locals.var_vxbgmtcl_dn10)), ((locals.var_beta_dn11 * assign76480_e115999) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((locals.var_beta_dn14 * assign76480_e115999) + (locals.var_beta * locals.var_vxbgmtcl_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign76480_e116002;
        locals.var_tx_dn0 = assign76480_e116002_d_n0;
        locals.var_tx_dn2 = assign76480_e116002_d_n2;
        locals.var_tx_dn4 = assign76480_e116002_d_n4;
        locals.var_tx_dn5 = assign76480_e116002_d_n5;
        locals.var_tx_dn6 = assign76480_e116002_d_n6;
        locals.var_tx_dn7 = assign76480_e116002_d_n7;
        locals.var_tx_dn8 = assign76480_e116002_d_n8;
        locals.var_tx_dn9 = assign76480_e116002_d_n9;
        locals.var_tx_dn10 = assign76480_e116002_d_n10;
        locals.var_tx_dn11 = assign76480_e116002_d_n11;
        locals.var_tx_dn14 = assign76480_e116002_d_n14;

        let (assign76490_e116012, assign76490_e116012_d_n0, assign76490_e116012_d_n2, assign76490_e116012_d_n4, assign76490_e116012_d_n5, assign76490_e116012_d_n6, assign76490_e116012_d_n7, assign76490_e116012_d_n8, assign76490_e116012_d_n9, assign76490_e116012_d_n10, assign76490_e116012_d_n11, assign76490_e116012_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76490_e116009: f64 = (locals.var_beta * locals.var_cnst0over_func);
        let assign76490_e116010: f64 = (1.0 / assign76490_e116009);
        (assign76490_e116010, (-(((locals.var_beta_dn0 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn0)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn2 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn2)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn4 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn4)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn5 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn5)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn6 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn6)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn7 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn7)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn8 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn8)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn9 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn9)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn10 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn10)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn11 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn11)) / (assign76490_e116009 * assign76490_e116009))), (-(((locals.var_beta_dn14 * locals.var_cnst0over_func) + (locals.var_beta * locals.var_cnst0over_func_dn14)) / (assign76490_e116009 * assign76490_e116009))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign76490_e116012;
        locals.var_t1_dn0 = assign76490_e116012_d_n0;
        locals.var_t1_dn2 = assign76490_e116012_d_n2;
        locals.var_t1_dn4 = assign76490_e116012_d_n4;
        locals.var_t1_dn5 = assign76490_e116012_d_n5;
        locals.var_t1_dn6 = assign76490_e116012_d_n6;
        locals.var_t1_dn7 = assign76490_e116012_d_n7;
        locals.var_t1_dn8 = assign76490_e116012_d_n8;
        locals.var_t1_dn9 = assign76490_e116012_d_n9;
        locals.var_t1_dn10 = assign76490_e116012_d_n10;
        locals.var_t1_dn11 = assign76490_e116012_d_n11;
        locals.var_t1_dn14 = assign76490_e116012_d_n14;

        let (assign76500_e116020, assign76500_e116020_d_n0, assign76500_e116020_d_n2, assign76500_e116020_d_n4, assign76500_e116020_d_n5, assign76500_e116020_d_n6, assign76500_e116020_d_n7, assign76500_e116020_d_n8, assign76500_e116020_d_n9, assign76500_e116020_d_n10, assign76500_e116020_d_n11, assign76500_e116020_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76500_e116018: f64 = (locals.var_t1 * locals.var_cox0_func);
        (assign76500_e116018, (locals.var_t1_dn0 * locals.var_cox0_func), (locals.var_t1_dn2 * locals.var_cox0_func), (locals.var_t1_dn4 * locals.var_cox0_func), (locals.var_t1_dn5 * locals.var_cox0_func), (locals.var_t1_dn6 * locals.var_cox0_func), (locals.var_t1_dn7 * locals.var_cox0_func), (locals.var_t1_dn8 * locals.var_cox0_func), (locals.var_t1_dn9 * locals.var_cox0_func), (locals.var_t1_dn10 * locals.var_cox0_func), (locals.var_t1_dn11 * locals.var_cox0_func), (locals.var_t1_dn14 * locals.var_cox0_func),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign76500_e116020;
        locals.var_ty_dn0 = assign76500_e116020_d_n0;
        locals.var_ty_dn2 = assign76500_e116020_d_n2;
        locals.var_ty_dn4 = assign76500_e116020_d_n4;
        locals.var_ty_dn5 = assign76500_e116020_d_n5;
        locals.var_ty_dn6 = assign76500_e116020_d_n6;
        locals.var_ty_dn7 = assign76500_e116020_d_n7;
        locals.var_ty_dn8 = assign76500_e116020_d_n8;
        locals.var_ty_dn9 = assign76500_e116020_d_n9;
        locals.var_ty_dn10 = assign76500_e116020_d_n10;
        locals.var_ty_dn11 = assign76500_e116020_d_n11;
        locals.var_ty_dn14 = assign76500_e116020_d_n14;

        let (assign76510_e116032, assign76510_e116032_d_n0, assign76510_e116032_d_n2, assign76510_e116032_d_n4, assign76510_e116032_d_n5, assign76510_e116032_d_n6, assign76510_e116032_d_n7, assign76510_e116032_d_n8, assign76510_e116032_d_n9, assign76510_e116032_d_n10, assign76510_e116032_d_n11, assign76510_e116032_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76510_e116027: f64 = (3.0 * 1.414213562373095);
        let assign76510_e116029: f64 = (assign76510_e116027 * locals.var_ty);
        let assign76510_e116030: f64 = (2.0 + assign76510_e116029);
        (assign76510_e116030, (assign76510_e116027 * locals.var_ty_dn0), (assign76510_e116027 * locals.var_ty_dn2), (assign76510_e116027 * locals.var_ty_dn4), (assign76510_e116027 * locals.var_ty_dn5), (assign76510_e116027 * locals.var_ty_dn6), (assign76510_e116027 * locals.var_ty_dn7), (assign76510_e116027 * locals.var_ty_dn8), (assign76510_e116027 * locals.var_ty_dn9), (assign76510_e116027 * locals.var_ty_dn10), (assign76510_e116027 * locals.var_ty_dn11), (assign76510_e116027 * locals.var_ty_dn14),)
    } else {
        (locals.var_ac41, locals.var_ac41_dn0, locals.var_ac41_dn2, locals.var_ac41_dn4, locals.var_ac41_dn5, locals.var_ac41_dn6, locals.var_ac41_dn7, locals.var_ac41_dn8, locals.var_ac41_dn9, locals.var_ac41_dn10, locals.var_ac41_dn11, locals.var_ac41_dn14,)
    }
};
        locals.var_ac41 = assign76510_e116032;
        locals.var_ac41_dn0 = assign76510_e116032_d_n0;
        locals.var_ac41_dn2 = assign76510_e116032_d_n2;
        locals.var_ac41_dn4 = assign76510_e116032_d_n4;
        locals.var_ac41_dn5 = assign76510_e116032_d_n5;
        locals.var_ac41_dn6 = assign76510_e116032_d_n6;
        locals.var_ac41_dn7 = assign76510_e116032_d_n7;
        locals.var_ac41_dn8 = assign76510_e116032_d_n8;
        locals.var_ac41_dn9 = assign76510_e116032_d_n9;
        locals.var_ac41_dn10 = assign76510_e116032_d_n10;
        locals.var_ac41_dn11 = assign76510_e116032_d_n11;
        locals.var_ac41_dn14 = assign76510_e116032_d_n14;

        let (assign76520_e116044, assign76520_e116044_d_n0, assign76520_e116044_d_n2, assign76520_e116044_d_n4, assign76520_e116044_d_n5, assign76520_e116044_d_n6, assign76520_e116044_d_n7, assign76520_e116044_d_n8, assign76520_e116044_d_n9, assign76520_e116044_d_n10, assign76520_e116044_d_n11, assign76520_e116044_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76520_e116038: f64 = (8.0 * locals.var_ac41);
        let assign76520_e116040: f64 = (assign76520_e116038 * locals.var_ac41);
        let assign76520_e116042: f64 = (assign76520_e116040 * locals.var_ac41);
        (assign76520_e116042, (((((8.0 * locals.var_ac41_dn0) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn0)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn0)), (((((8.0 * locals.var_ac41_dn2) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn2)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn2)), (((((8.0 * locals.var_ac41_dn4) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn4)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn4)), (((((8.0 * locals.var_ac41_dn5) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn5)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn5)), (((((8.0 * locals.var_ac41_dn6) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn6)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn6)), (((((8.0 * locals.var_ac41_dn7) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn7)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn7)), (((((8.0 * locals.var_ac41_dn8) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn8)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn8)), (((((8.0 * locals.var_ac41_dn9) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn9)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn9)), (((((8.0 * locals.var_ac41_dn10) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn10)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn10)), (((((8.0 * locals.var_ac41_dn11) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn11)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn11)), (((((8.0 * locals.var_ac41_dn14) * locals.var_ac41) + (assign76520_e116038 * locals.var_ac41_dn14)) * locals.var_ac41) + (assign76520_e116040 * locals.var_ac41_dn14)),)
    } else {
        (locals.var_ac4, locals.var_ac4_dn0, locals.var_ac4_dn2, locals.var_ac4_dn4, locals.var_ac4_dn5, locals.var_ac4_dn6, locals.var_ac4_dn7, locals.var_ac4_dn8, locals.var_ac4_dn9, locals.var_ac4_dn10, locals.var_ac4_dn11, locals.var_ac4_dn14,)
    }
};
        locals.var_ac4 = assign76520_e116044;
        locals.var_ac4_dn0 = assign76520_e116044_d_n0;
        locals.var_ac4_dn2 = assign76520_e116044_d_n2;
        locals.var_ac4_dn4 = assign76520_e116044_d_n4;
        locals.var_ac4_dn5 = assign76520_e116044_d_n5;
        locals.var_ac4_dn6 = assign76520_e116044_d_n6;
        locals.var_ac4_dn7 = assign76520_e116044_d_n7;
        locals.var_ac4_dn8 = assign76520_e116044_d_n8;
        locals.var_ac4_dn9 = assign76520_e116044_d_n9;
        locals.var_ac4_dn10 = assign76520_e116044_d_n10;
        locals.var_ac4_dn11 = assign76520_e116044_d_n11;
        locals.var_ac4_dn14 = assign76520_e116044_d_n14;

        let (assign76530_e116060, assign76530_e116060_d_n0, assign76530_e116060_d_n2, assign76530_e116060_d_n4, assign76530_e116060_d_n5, assign76530_e116060_d_n6, assign76530_e116060_d_n7, assign76530_e116060_d_n8, assign76530_e116060_d_n9, assign76530_e116060_d_n10, assign76530_e116060_d_n11, assign76530_e116060_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76530_e116050: f64 = (7.0 * 1.414213562373095);
        let assign76530_e116053: f64 = (9.0 * locals.var_ty);
        let assign76530_e116056: f64 = (locals.var_tx - 2.0);
        let assign76530_e116057: f64 = (assign76530_e116053 * assign76530_e116056);
        let assign76530_e116058: f64 = (assign76530_e116050 - assign76530_e116057);
        (assign76530_e116058, (-(((9.0 * locals.var_ty_dn0) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn0))), (-(((9.0 * locals.var_ty_dn2) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn2))), (-(((9.0 * locals.var_ty_dn4) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn4))), (-(((9.0 * locals.var_ty_dn5) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn5))), (-(((9.0 * locals.var_ty_dn6) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn6))), (-(((9.0 * locals.var_ty_dn7) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn7))), (-(((9.0 * locals.var_ty_dn8) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn8))), (-(((9.0 * locals.var_ty_dn9) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn9))), (-(((9.0 * locals.var_ty_dn10) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn10))), (-(((9.0 * locals.var_ty_dn11) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn11))), (-(((9.0 * locals.var_ty_dn14) * assign76530_e116056) + (assign76530_e116053 * locals.var_tx_dn14))),)
    } else {
        (locals.var_ac31, locals.var_ac31_dn0, locals.var_ac31_dn2, locals.var_ac31_dn4, locals.var_ac31_dn5, locals.var_ac31_dn6, locals.var_ac31_dn7, locals.var_ac31_dn8, locals.var_ac31_dn9, locals.var_ac31_dn10, locals.var_ac31_dn11, locals.var_ac31_dn14,)
    }
};
        locals.var_ac31 = assign76530_e116060;
        locals.var_ac31_dn0 = assign76530_e116060_d_n0;
        locals.var_ac31_dn2 = assign76530_e116060_d_n2;
        locals.var_ac31_dn4 = assign76530_e116060_d_n4;
        locals.var_ac31_dn5 = assign76530_e116060_d_n5;
        locals.var_ac31_dn6 = assign76530_e116060_d_n6;
        locals.var_ac31_dn7 = assign76530_e116060_d_n7;
        locals.var_ac31_dn8 = assign76530_e116060_d_n8;
        locals.var_ac31_dn9 = assign76530_e116060_d_n9;
        locals.var_ac31_dn10 = assign76530_e116060_d_n10;
        locals.var_ac31_dn11 = assign76530_e116060_d_n11;
        locals.var_ac31_dn14 = assign76530_e116060_d_n14;

        let (assign76540_e116068, assign76540_e116068_d_n0, assign76540_e116068_d_n2, assign76540_e116068_d_n4, assign76540_e116068_d_n5, assign76540_e116068_d_n6, assign76540_e116068_d_n7, assign76540_e116068_d_n8, assign76540_e116068_d_n9, assign76540_e116068_d_n10, assign76540_e116068_d_n11, assign76540_e116068_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76540_e116066: f64 = (locals.var_ac31 * locals.var_ac31);
        (assign76540_e116066, ((locals.var_ac31_dn0 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn0)), ((locals.var_ac31_dn2 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn2)), ((locals.var_ac31_dn4 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn4)), ((locals.var_ac31_dn5 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn5)), ((locals.var_ac31_dn6 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn6)), ((locals.var_ac31_dn7 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn7)), ((locals.var_ac31_dn8 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn8)), ((locals.var_ac31_dn9 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn9)), ((locals.var_ac31_dn10 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn10)), ((locals.var_ac31_dn11 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn11)), ((locals.var_ac31_dn14 * locals.var_ac31) + (locals.var_ac31 * locals.var_ac31_dn14)),)
    } else {
        (locals.var_ac3, locals.var_ac3_dn0, locals.var_ac3_dn2, locals.var_ac3_dn4, locals.var_ac3_dn5, locals.var_ac3_dn6, locals.var_ac3_dn7, locals.var_ac3_dn8, locals.var_ac3_dn9, locals.var_ac3_dn10, locals.var_ac3_dn11, locals.var_ac3_dn14,)
    }
};
        locals.var_ac3 = assign76540_e116068;
        locals.var_ac3_dn0 = assign76540_e116068_d_n0;
        locals.var_ac3_dn2 = assign76540_e116068_d_n2;
        locals.var_ac3_dn4 = assign76540_e116068_d_n4;
        locals.var_ac3_dn5 = assign76540_e116068_d_n5;
        locals.var_ac3_dn6 = assign76540_e116068_d_n6;
        locals.var_ac3_dn7 = assign76540_e116068_d_n7;
        locals.var_ac3_dn8 = assign76540_e116068_d_n8;
        locals.var_ac3_dn9 = assign76540_e116068_d_n9;
        locals.var_ac3_dn10 = assign76540_e116068_d_n10;
        locals.var_ac3_dn11 = assign76540_e116068_d_n11;
        locals.var_ac3_dn14 = assign76540_e116068_d_n14;

        let assign76550_e116072: f64 = (locals.var_ac3 * 1e-8);
        let assign76550_e116073: f64 = if locals.var_ac4 < assign76550_e116072 { 1.0 } else { 0.0 };
        locals.var_guard1791 = assign76550_e116073;

        let (assign76570_e116094, assign76570_e116094_d_n0, assign76570_e116094_d_n2, assign76570_e116094_d_n4, assign76570_e116094_d_n5, assign76570_e116094_d_n6, assign76570_e116094_d_n7, assign76570_e116094_d_n8, assign76570_e116094_d_n9, assign76570_e116094_d_n10, assign76570_e116094_d_n11, assign76570_e116094_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1791 != 0.0)) {
        let assign76570_e116090: f64 = (0.5 * locals.var_ac4);
        let assign76570_e116092: f64 = (assign76570_e116090 / locals.var_ac31);
        (assign76570_e116092, ((((0.5 * locals.var_ac4_dn0) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn0)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn2) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn2)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn4) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn4)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn5) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn5)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn6) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn6)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn7) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn7)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn8) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn8)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn9) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn9)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn10) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn10)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn11) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn11)) / (locals.var_ac31 * locals.var_ac31)), ((((0.5 * locals.var_ac4_dn14) * locals.var_ac31) - (assign76570_e116090 * locals.var_ac31_dn14)) / (locals.var_ac31 * locals.var_ac31)),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign76570_e116094;
        locals.var_ac1_dn0 = assign76570_e116094_d_n0;
        locals.var_ac1_dn2 = assign76570_e116094_d_n2;
        locals.var_ac1_dn4 = assign76570_e116094_d_n4;
        locals.var_ac1_dn5 = assign76570_e116094_d_n5;
        locals.var_ac1_dn6 = assign76570_e116094_d_n6;
        locals.var_ac1_dn7 = assign76570_e116094_d_n7;
        locals.var_ac1_dn8 = assign76570_e116094_d_n8;
        locals.var_ac1_dn9 = assign76570_e116094_d_n9;
        locals.var_ac1_dn10 = assign76570_e116094_d_n10;
        locals.var_ac1_dn11 = assign76570_e116094_d_n11;
        locals.var_ac1_dn14 = assign76570_e116094_d_n14;

        let (assign76580_e116106, assign76580_e116106_d_n0, assign76580_e116106_d_n2, assign76580_e116106_d_n4, assign76580_e116106_d_n5, assign76580_e116106_d_n6, assign76580_e116106_d_n7, assign76580_e116106_d_n8, assign76580_e116106_d_n9, assign76580_e116106_d_n10, assign76580_e116106_d_n11, assign76580_e116106_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1791 == 0.0)) {
        let assign76580_e116103: f64 = (locals.var_ac4 + locals.var_ac3);
        let assign76580_e116104: f64 = (assign76580_e116103).sqrt();
        (assign76580_e116104, ((locals.var_ac4_dn0 + locals.var_ac3_dn0) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn2 + locals.var_ac3_dn2) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn4 + locals.var_ac3_dn4) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn5 + locals.var_ac3_dn5) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn6 + locals.var_ac3_dn6) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn7 + locals.var_ac3_dn7) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn8 + locals.var_ac3_dn8) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn9 + locals.var_ac3_dn9) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn10 + locals.var_ac3_dn10) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn11 + locals.var_ac3_dn11) / (2.0 * assign76580_e116104)), ((locals.var_ac4_dn14 + locals.var_ac3_dn14) / (2.0 * assign76580_e116104)),)
    } else {
        (locals.var_ac2, locals.var_ac2_dn0, locals.var_ac2_dn2, locals.var_ac2_dn4, locals.var_ac2_dn5, locals.var_ac2_dn6, locals.var_ac2_dn7, locals.var_ac2_dn8, locals.var_ac2_dn9, locals.var_ac2_dn10, locals.var_ac2_dn11, locals.var_ac2_dn14,)
    }
};
        locals.var_ac2 = assign76580_e116106;
        locals.var_ac2_dn0 = assign76580_e116106_d_n0;
        locals.var_ac2_dn2 = assign76580_e116106_d_n2;
        locals.var_ac2_dn4 = assign76580_e116106_d_n4;
        locals.var_ac2_dn5 = assign76580_e116106_d_n5;
        locals.var_ac2_dn6 = assign76580_e116106_d_n6;
        locals.var_ac2_dn7 = assign76580_e116106_d_n7;
        locals.var_ac2_dn8 = assign76580_e116106_d_n8;
        locals.var_ac2_dn9 = assign76580_e116106_d_n9;
        locals.var_ac2_dn10 = assign76580_e116106_d_n10;
        locals.var_ac2_dn11 = assign76580_e116106_d_n11;
        locals.var_ac2_dn14 = assign76580_e116106_d_n14;

        let (assign76590_e116118, assign76590_e116118_d_n0, assign76590_e116118_d_n2, assign76590_e116118_d_n4, assign76590_e116118_d_n5, assign76590_e116118_d_n6, assign76590_e116118_d_n7, assign76590_e116118_d_n8, assign76590_e116118_d_n9, assign76590_e116118_d_n10, assign76590_e116118_d_n11, assign76590_e116118_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) && (locals.var_guard1791 == 0.0)) {
        let assign76590_e116114: f64 = (-locals.var_ac31);
        let assign76590_e116116: f64 = (assign76590_e116114 + locals.var_ac2);
        (assign76590_e116116, ((-locals.var_ac31_dn0) + locals.var_ac2_dn0), ((-locals.var_ac31_dn2) + locals.var_ac2_dn2), ((-locals.var_ac31_dn4) + locals.var_ac2_dn4), ((-locals.var_ac31_dn5) + locals.var_ac2_dn5), ((-locals.var_ac31_dn6) + locals.var_ac2_dn6), ((-locals.var_ac31_dn7) + locals.var_ac2_dn7), ((-locals.var_ac31_dn8) + locals.var_ac2_dn8), ((-locals.var_ac31_dn9) + locals.var_ac2_dn9), ((-locals.var_ac31_dn10) + locals.var_ac2_dn10), ((-locals.var_ac31_dn11) + locals.var_ac2_dn11), ((-locals.var_ac31_dn14) + locals.var_ac2_dn14),)
    } else {
        (locals.var_ac1, locals.var_ac1_dn0, locals.var_ac1_dn2, locals.var_ac1_dn4, locals.var_ac1_dn5, locals.var_ac1_dn6, locals.var_ac1_dn7, locals.var_ac1_dn8, locals.var_ac1_dn9, locals.var_ac1_dn10, locals.var_ac1_dn11, locals.var_ac1_dn14,)
    }
};
        locals.var_ac1 = assign76590_e116118;
        locals.var_ac1_dn0 = assign76590_e116118_d_n0;
        locals.var_ac1_dn2 = assign76590_e116118_d_n2;
        locals.var_ac1_dn4 = assign76590_e116118_d_n4;
        locals.var_ac1_dn5 = assign76590_e116118_d_n5;
        locals.var_ac1_dn6 = assign76590_e116118_d_n6;
        locals.var_ac1_dn7 = assign76590_e116118_d_n7;
        locals.var_ac1_dn8 = assign76590_e116118_d_n8;
        locals.var_ac1_dn9 = assign76590_e116118_d_n9;
        locals.var_ac1_dn10 = assign76590_e116118_d_n10;
        locals.var_ac1_dn11 = assign76590_e116118_d_n11;
        locals.var_ac1_dn14 = assign76590_e116118_d_n14;

        let (assign76600_e116126, assign76600_e116126_d_n0, assign76600_e116126_d_n2, assign76600_e116126_d_n4, assign76600_e116126_d_n5, assign76600_e116126_d_n6, assign76600_e116126_d_n7, assign76600_e116126_d_n8, assign76600_e116126_d_n9, assign76600_e116126_d_n10, assign76600_e116126_d_n11, assign76600_e116126_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76600_e116124: f64 = (locals.var_ac1).powf(0.3333333333333333);
        (assign76600_e116124, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn0)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn0 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn2)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn2 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn4)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn4 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn5)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn5 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn6)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn6 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn7)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn7 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn8)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn8 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn9)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn9 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn10)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn10 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn11)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn11 / locals.var_ac1))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1).powf(0.3333333333333333 - 1.0) * locals.var_ac1_dn14)) } } else { (assign76600_e116124 * (0.3333333333333333 * (locals.var_ac1_dn14 / locals.var_ac1))) },)
    } else {
        (locals.var_acd, locals.var_acd_dn0, locals.var_acd_dn2, locals.var_acd_dn4, locals.var_acd_dn5, locals.var_acd_dn6, locals.var_acd_dn7, locals.var_acd_dn8, locals.var_acd_dn9, locals.var_acd_dn10, locals.var_acd_dn11, locals.var_acd_dn14,)
    }
};
        locals.var_acd = assign76600_e116126;
        locals.var_acd_dn0 = assign76600_e116126_d_n0;
        locals.var_acd_dn2 = assign76600_e116126_d_n2;
        locals.var_acd_dn4 = assign76600_e116126_d_n4;
        locals.var_acd_dn5 = assign76600_e116126_d_n5;
        locals.var_acd_dn6 = assign76600_e116126_d_n6;
        locals.var_acd_dn7 = assign76600_e116126_d_n7;
        locals.var_acd_dn8 = assign76600_e116126_d_n8;
        locals.var_acd_dn9 = assign76600_e116126_d_n9;
        locals.var_acd_dn10 = assign76600_e116126_d_n10;
        locals.var_acd_dn11 = assign76600_e116126_d_n11;
        locals.var_acd_dn14 = assign76600_e116126_d_n14;

    }

    pub(super) fn stamp_transient_block_276(
        locals: &mut StampLocals,
    ) {
        let (assign76610_e116149, assign76610_e116149_d_n0, assign76610_e116149_d_n2, assign76610_e116149_d_n4, assign76610_e116149_d_n5, assign76610_e116149_d_n6, assign76610_e116149_d_n7, assign76610_e116149_d_n8, assign76610_e116149_d_n9, assign76610_e116149_d_n10, assign76610_e116149_d_n11, assign76610_e116149_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76610_e116131: f64 = (-4.0);
        let assign76610_e116133: f64 = (assign76610_e116131 * 1.414213562373095);
        let assign76610_e116136: f64 = (12.0 * locals.var_ty);
        let assign76610_e116137: f64 = (assign76610_e116133 - assign76610_e116136);
        let assign76610_e116140: f64 = (2.0 * locals.var_acd);
        let assign76610_e116141: f64 = (assign76610_e116137 + assign76610_e116140);
        let assign76610_e116144: f64 = (1.414213562373095 * locals.var_acd);
        let assign76610_e116146: f64 = (assign76610_e116144 * locals.var_acd);
        let assign76610_e116147: f64 = (assign76610_e116141 + assign76610_e116146);
        (assign76610_e116147, (((-(12.0 * locals.var_ty_dn0)) + (2.0 * locals.var_acd_dn0)) + (((1.414213562373095 * locals.var_acd_dn0) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn0))), (((-(12.0 * locals.var_ty_dn2)) + (2.0 * locals.var_acd_dn2)) + (((1.414213562373095 * locals.var_acd_dn2) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn2))), (((-(12.0 * locals.var_ty_dn4)) + (2.0 * locals.var_acd_dn4)) + (((1.414213562373095 * locals.var_acd_dn4) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn4))), (((-(12.0 * locals.var_ty_dn5)) + (2.0 * locals.var_acd_dn5)) + (((1.414213562373095 * locals.var_acd_dn5) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn5))), (((-(12.0 * locals.var_ty_dn6)) + (2.0 * locals.var_acd_dn6)) + (((1.414213562373095 * locals.var_acd_dn6) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn6))), (((-(12.0 * locals.var_ty_dn7)) + (2.0 * locals.var_acd_dn7)) + (((1.414213562373095 * locals.var_acd_dn7) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn7))), (((-(12.0 * locals.var_ty_dn8)) + (2.0 * locals.var_acd_dn8)) + (((1.414213562373095 * locals.var_acd_dn8) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn8))), (((-(12.0 * locals.var_ty_dn9)) + (2.0 * locals.var_acd_dn9)) + (((1.414213562373095 * locals.var_acd_dn9) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn9))), (((-(12.0 * locals.var_ty_dn10)) + (2.0 * locals.var_acd_dn10)) + (((1.414213562373095 * locals.var_acd_dn10) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn10))), (((-(12.0 * locals.var_ty_dn11)) + (2.0 * locals.var_acd_dn11)) + (((1.414213562373095 * locals.var_acd_dn11) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn11))), (((-(12.0 * locals.var_ty_dn14)) + (2.0 * locals.var_acd_dn14)) + (((1.414213562373095 * locals.var_acd_dn14) * locals.var_acd) + (assign76610_e116144 * locals.var_acd_dn14))),)
    } else {
        (locals.var_acn, locals.var_acn_dn0, locals.var_acn_dn2, locals.var_acn_dn4, locals.var_acn_dn5, locals.var_acn_dn6, locals.var_acn_dn7, locals.var_acn_dn8, locals.var_acn_dn9, locals.var_acn_dn10, locals.var_acn_dn11, locals.var_acn_dn14,)
    }
};
        locals.var_acn = assign76610_e116149;
        locals.var_acn_dn0 = assign76610_e116149_d_n0;
        locals.var_acn_dn2 = assign76610_e116149_d_n2;
        locals.var_acn_dn4 = assign76610_e116149_d_n4;
        locals.var_acn_dn5 = assign76610_e116149_d_n5;
        locals.var_acn_dn6 = assign76610_e116149_d_n6;
        locals.var_acn_dn7 = assign76610_e116149_d_n7;
        locals.var_acn_dn8 = assign76610_e116149_d_n8;
        locals.var_acn_dn9 = assign76610_e116149_d_n9;
        locals.var_acn_dn10 = assign76610_e116149_d_n10;
        locals.var_acn_dn11 = assign76610_e116149_d_n11;
        locals.var_acn_dn14 = assign76610_e116149_d_n14;

        let (assign76620_e116157, assign76620_e116157_d_n0, assign76620_e116157_d_n2, assign76620_e116157_d_n4, assign76620_e116157_d_n5, assign76620_e116157_d_n6, assign76620_e116157_d_n7, assign76620_e116157_d_n8, assign76620_e116157_d_n9, assign76620_e116157_d_n10, assign76620_e116157_d_n11, assign76620_e116157_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76620_e116155: f64 = (locals.var_acn / locals.var_acd);
        (assign76620_e116155, (((locals.var_acn_dn0 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn0)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn2 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn2)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn4 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn4)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn5 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn5)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn6 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn6)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn7 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn7)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn8 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn8)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn9 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn9)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn10 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn10)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn11 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn11)) / (locals.var_acd * locals.var_acd)), (((locals.var_acn_dn14 * locals.var_acd) - (locals.var_acn * locals.var_acd_dn14)) / (locals.var_acd * locals.var_acd)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign76620_e116157;
        locals.var_chi_dn0 = assign76620_e116157_d_n0;
        locals.var_chi_dn2 = assign76620_e116157_d_n2;
        locals.var_chi_dn4 = assign76620_e116157_d_n4;
        locals.var_chi_dn5 = assign76620_e116157_d_n5;
        locals.var_chi_dn6 = assign76620_e116157_d_n6;
        locals.var_chi_dn7 = assign76620_e116157_d_n7;
        locals.var_chi_dn8 = assign76620_e116157_d_n8;
        locals.var_chi_dn9 = assign76620_e116157_d_n9;
        locals.var_chi_dn10 = assign76620_e116157_d_n10;
        locals.var_chi_dn11 = assign76620_e116157_d_n11;
        locals.var_chi_dn14 = assign76620_e116157_d_n14;

        let (assign76630_e116165, assign76630_e116165_d_n0, assign76630_e116165_d_n2, assign76630_e116165_d_n4, assign76630_e116165_d_n5, assign76630_e116165_d_n6, assign76630_e116165_d_n7, assign76630_e116165_d_n8, assign76630_e116165_d_n9, assign76630_e116165_d_n10, assign76630_e116165_d_n11, assign76630_e116165_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76630_e116163: f64 = (locals.var_chi * locals.var_beta_inv);
        (assign76630_e116163, ((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)), ((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)), ((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)), ((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)), ((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)), ((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)), ((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)), ((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)), ((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)), ((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)), ((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign76630_e116165;
        locals.var_t1_dn0 = assign76630_e116165_d_n0;
        locals.var_t1_dn2 = assign76630_e116165_d_n2;
        locals.var_t1_dn4 = assign76630_e116165_d_n4;
        locals.var_t1_dn5 = assign76630_e116165_d_n5;
        locals.var_t1_dn6 = assign76630_e116165_d_n6;
        locals.var_t1_dn7 = assign76630_e116165_d_n7;
        locals.var_t1_dn8 = assign76630_e116165_d_n8;
        locals.var_t1_dn9 = assign76630_e116165_d_n9;
        locals.var_t1_dn10 = assign76630_e116165_d_n10;
        locals.var_t1_dn11 = assign76630_e116165_d_n11;
        locals.var_t1_dn14 = assign76630_e116165_d_n14;

        let (assign76640_e116173, assign76640_e116173_d_n0, assign76640_e116173_d_n2, assign76640_e116173_d_n4, assign76640_e116173_d_n5, assign76640_e116173_d_n6, assign76640_e116173_d_n7, assign76640_e116173_d_n8, assign76640_e116173_d_n9, assign76640_e116173_d_n10, assign76640_e116173_d_n11, assign76640_e116173_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76640_e116171: f64 = (locals.var_t1 / locals.var_ps0_min);
        (assign76640_e116171, (((locals.var_t1_dn0 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn0)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn2 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn2)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn4 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn4)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn5 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn5)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn6 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn6)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn7 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn7)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn8 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn8)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn9 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn9)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn10 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn10)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn11 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn11)) / (locals.var_ps0_min * locals.var_ps0_min)), (((locals.var_t1_dn14 * locals.var_ps0_min) - (locals.var_t1 * locals.var_ps0_min_dn14)) / (locals.var_ps0_min * locals.var_ps0_min)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign76640_e116173;
        locals.var_t2_dn0 = assign76640_e116173_d_n0;
        locals.var_t2_dn2 = assign76640_e116173_d_n2;
        locals.var_t2_dn4 = assign76640_e116173_d_n4;
        locals.var_t2_dn5 = assign76640_e116173_d_n5;
        locals.var_t2_dn6 = assign76640_e116173_d_n6;
        locals.var_t2_dn7 = assign76640_e116173_d_n7;
        locals.var_t2_dn8 = assign76640_e116173_d_n8;
        locals.var_t2_dn9 = assign76640_e116173_d_n9;
        locals.var_t2_dn10 = assign76640_e116173_d_n10;
        locals.var_t2_dn11 = assign76640_e116173_d_n11;
        locals.var_t2_dn14 = assign76640_e116173_d_n14;

        let (assign76650_e116184, assign76650_e116184_d_n0, assign76650_e116184_d_n2, assign76650_e116184_d_n4, assign76650_e116184_d_n5, assign76650_e116184_d_n6, assign76650_e116184_d_n7, assign76650_e116184_d_n8, assign76650_e116184_d_n9, assign76650_e116184_d_n10, assign76650_e116184_d_n11, assign76650_e116184_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76650_e116180: f64 = (locals.var_t2 * locals.var_t2);
        let assign76650_e116181: f64 = (1.0 + assign76650_e116180);
        let assign76650_e116182: f64 = (assign76650_e116181).sqrt();
        (assign76650_e116182, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign76650_e116182)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign76650_e116182)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign76650_e116184;
        locals.var_t3_dn0 = assign76650_e116184_d_n0;
        locals.var_t3_dn2 = assign76650_e116184_d_n2;
        locals.var_t3_dn4 = assign76650_e116184_d_n4;
        locals.var_t3_dn5 = assign76650_e116184_d_n5;
        locals.var_t3_dn6 = assign76650_e116184_d_n6;
        locals.var_t3_dn7 = assign76650_e116184_d_n7;
        locals.var_t3_dn8 = assign76650_e116184_d_n8;
        locals.var_t3_dn9 = assign76650_e116184_d_n9;
        locals.var_t3_dn10 = assign76650_e116184_d_n10;
        locals.var_t3_dn11 = assign76650_e116184_d_n11;
        locals.var_t3_dn14 = assign76650_e116184_d_n14;

        let (assign76660_e116194, assign76660_e116194_d_n0, assign76660_e116194_d_n2, assign76660_e116194_d_n4, assign76660_e116194_d_n5, assign76660_e116194_d_n6, assign76660_e116194_d_n7, assign76660_e116194_d_n8, assign76660_e116194_d_n9, assign76660_e116194_d_n10, assign76660_e116194_d_n11, assign76660_e116194_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76660_e116190: f64 = (locals.var_t1 / locals.var_t3);
        let assign76660_e116192: f64 = (assign76660_e116190 - locals.var_vxbgmtcl);
        (assign76660_e116192, ((((locals.var_t1_dn0 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn0)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1_dn2 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn2)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1_dn4 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn4)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn4), ((((locals.var_t1_dn5 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn5)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn5), ((((locals.var_t1_dn6 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn6)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1_dn7 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn7)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1_dn8 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn8)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn8), ((((locals.var_t1_dn9 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn9)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn9), ((((locals.var_t1_dn10 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn10)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1_dn11 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn11)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn11), ((((locals.var_t1_dn14 * locals.var_t3) - (locals.var_t1 * locals.var_t3_dn14)) / (locals.var_t3 * locals.var_t3)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign76660_e116194;
        locals.var_ps0ld_dn0 = assign76660_e116194_d_n0;
        locals.var_ps0ld_dn2 = assign76660_e116194_d_n2;
        locals.var_ps0ld_dn4 = assign76660_e116194_d_n4;
        locals.var_ps0ld_dn5 = assign76660_e116194_d_n5;
        locals.var_ps0ld_dn6 = assign76660_e116194_d_n6;
        locals.var_ps0ld_dn7 = assign76660_e116194_d_n7;
        locals.var_ps0ld_dn8 = assign76660_e116194_d_n8;
        locals.var_ps0ld_dn9 = assign76660_e116194_d_n9;
        locals.var_ps0ld_dn10 = assign76660_e116194_d_n10;
        locals.var_ps0ld_dn11 = assign76660_e116194_d_n11;
        locals.var_ps0ld_dn14 = assign76660_e116194_d_n14;

        let (assign76670_e116202, assign76670_e116202_d_n0, assign76670_e116202_d_n2, assign76670_e116202_d_n4, assign76670_e116202_d_n5, assign76670_e116202_d_n6, assign76670_e116202_d_n7, assign76670_e116202_d_n8, assign76670_e116202_d_n9, assign76670_e116202_d_n10, assign76670_e116202_d_n11, assign76670_e116202_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76670_e116200: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign76670_e116200, (-locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (-locals.var_ps0ld_dn4), (-locals.var_ps0ld_dn5), (-locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8), (locals.var_vgpld_dn9 - locals.var_ps0ld_dn9), (-locals.var_ps0ld_dn10), (-locals.var_ps0ld_dn11), (-locals.var_ps0ld_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign76670_e116202;
        locals.var_t2_dn0 = assign76670_e116202_d_n0;
        locals.var_t2_dn2 = assign76670_e116202_d_n2;
        locals.var_t2_dn4 = assign76670_e116202_d_n4;
        locals.var_t2_dn5 = assign76670_e116202_d_n5;
        locals.var_t2_dn6 = assign76670_e116202_d_n6;
        locals.var_t2_dn7 = assign76670_e116202_d_n7;
        locals.var_t2_dn8 = assign76670_e116202_d_n8;
        locals.var_t2_dn9 = assign76670_e116202_d_n9;
        locals.var_t2_dn10 = assign76670_e116202_d_n10;
        locals.var_t2_dn11 = assign76670_e116202_d_n11;
        locals.var_t2_dn14 = assign76670_e116202_d_n14;

        let (assign76680_e116210, assign76680_e116210_d_n0, assign76680_e116210_d_n2, assign76680_e116210_d_n4, assign76680_e116210_d_n5, assign76680_e116210_d_n6, assign76680_e116210_d_n7, assign76680_e116210_d_n8, assign76680_e116210_d_n9, assign76680_e116210_d_n10, assign76680_e116210_d_n11, assign76680_e116210_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        let assign76680_e116208: f64 = (locals.var_cox0_func * locals.var_t2);
        (assign76680_e116208, (locals.var_cox0_func * locals.var_t2_dn0), (locals.var_cox0_func * locals.var_t2_dn2), (locals.var_cox0_func * locals.var_t2_dn4), (locals.var_cox0_func * locals.var_t2_dn5), (locals.var_cox0_func * locals.var_t2_dn6), (locals.var_cox0_func * locals.var_t2_dn7), (locals.var_cox0_func * locals.var_t2_dn8), (locals.var_cox0_func * locals.var_t2_dn9), (locals.var_cox0_func * locals.var_t2_dn10), (locals.var_cox0_func * locals.var_t2_dn11), (locals.var_cox0_func * locals.var_t2_dn14),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign76680_e116210;
        locals.var_qsuld_dn0 = assign76680_e116210_d_n0;
        locals.var_qsuld_dn2 = assign76680_e116210_d_n2;
        locals.var_qsuld_dn4 = assign76680_e116210_d_n4;
        locals.var_qsuld_dn5 = assign76680_e116210_d_n5;
        locals.var_qsuld_dn6 = assign76680_e116210_d_n6;
        locals.var_qsuld_dn7 = assign76680_e116210_d_n7;
        locals.var_qsuld_dn8 = assign76680_e116210_d_n8;
        locals.var_qsuld_dn9 = assign76680_e116210_d_n9;
        locals.var_qsuld_dn10 = assign76680_e116210_d_n10;
        locals.var_qsuld_dn11 = assign76680_e116210_d_n11;
        locals.var_qsuld_dn14 = assign76680_e116210_d_n14;

        let (assign76690_e116216, assign76690_e116216_d_n0, assign76690_e116216_d_n2, assign76690_e116216_d_n4, assign76690_e116216_d_n5, assign76690_e116216_d_n6, assign76690_e116216_d_n7, assign76690_e116216_d_n8, assign76690_e116216_d_n9, assign76690_e116216_d_n10, assign76690_e116216_d_n11, assign76690_e116216_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign76690_e116216;
        locals.var_qbuld_dn0 = assign76690_e116216_d_n0;
        locals.var_qbuld_dn2 = assign76690_e116216_d_n2;
        locals.var_qbuld_dn4 = assign76690_e116216_d_n4;
        locals.var_qbuld_dn5 = assign76690_e116216_d_n5;
        locals.var_qbuld_dn6 = assign76690_e116216_d_n6;
        locals.var_qbuld_dn7 = assign76690_e116216_d_n7;
        locals.var_qbuld_dn8 = assign76690_e116216_d_n8;
        locals.var_qbuld_dn9 = assign76690_e116216_d_n9;
        locals.var_qbuld_dn10 = assign76690_e116216_d_n10;
        locals.var_qbuld_dn11 = assign76690_e116216_d_n11;
        locals.var_qbuld_dn14 = assign76690_e116216_d_n14;

        let (assign76700_e116222, assign76700_e116222_d_n0, assign76700_e116222_d_n2, assign76700_e116222_d_n4, assign76700_e116222_d_n5, assign76700_e116222_d_n6, assign76700_e116222_d_n7, assign76700_e116222_d_n8, assign76700_e116222_d_n9, assign76700_e116222_d_n10, assign76700_e116222_d_n11, assign76700_e116222_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld_ini__blk1773, locals.var_ps0ld_ini__blk1773_dn0, locals.var_ps0ld_ini__blk1773_dn2, locals.var_ps0ld_ini__blk1773_dn4, locals.var_ps0ld_ini__blk1773_dn5, locals.var_ps0ld_ini__blk1773_dn6, locals.var_ps0ld_ini__blk1773_dn7, locals.var_ps0ld_ini__blk1773_dn8, locals.var_ps0ld_ini__blk1773_dn9, locals.var_ps0ld_ini__blk1773_dn10, locals.var_ps0ld_ini__blk1773_dn11, locals.var_ps0ld_ini__blk1773_dn14,)
    }
};
        locals.var_ps0ld_ini__blk1773 = assign76700_e116222;
        locals.var_ps0ld_ini__blk1773_dn0 = assign76700_e116222_d_n0;
        locals.var_ps0ld_ini__blk1773_dn2 = assign76700_e116222_d_n2;
        locals.var_ps0ld_ini__blk1773_dn4 = assign76700_e116222_d_n4;
        locals.var_ps0ld_ini__blk1773_dn5 = assign76700_e116222_d_n5;
        locals.var_ps0ld_ini__blk1773_dn6 = assign76700_e116222_d_n6;
        locals.var_ps0ld_ini__blk1773_dn7 = assign76700_e116222_d_n7;
        locals.var_ps0ld_ini__blk1773_dn8 = assign76700_e116222_d_n8;
        locals.var_ps0ld_ini__blk1773_dn9 = assign76700_e116222_d_n9;
        locals.var_ps0ld_ini__blk1773_dn10 = assign76700_e116222_d_n10;
        locals.var_ps0ld_ini__blk1773_dn11 = assign76700_e116222_d_n11;
        locals.var_ps0ld_ini__blk1773_dn14 = assign76700_e116222_d_n14;

        let assign76710_e116226: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76710_e116227: f64 = (locals.var_beta * assign76710_e116226);
        let assign76710_e116231: f64 = (10.0 * 2.220446049250313e-16);
        let assign76710_e116233: f64 = (assign76710_e116231 - 1.0);
        let assign76710_e116235: f64 = (assign76710_e116233 * locals.var_fac1p2);
        let assign76710_e116237: f64 = (assign76710_e116235 * locals.var_beta2);
        let assign76710_e116239: f64 = (assign76710_e116237 / 4.0);
        let assign76710_e116240: f64 = (1.0 + assign76710_e116239);
        let assign76710_e116241: f64 = if assign76710_e116227 < assign76710_e116240 { 1.0 } else { 0.0 };
        locals.var_guard1792 = assign76710_e116241;

        let (assign76720_e116256, assign76720_e116256_d_n0, assign76720_e116256_d_n2, assign76720_e116256_d_n4, assign76720_e116256_d_n5, assign76720_e116256_d_n6, assign76720_e116256_d_n7, assign76720_e116256_d_n8, assign76720_e116256_d_n9, assign76720_e116256_d_n10, assign76720_e116256_d_n11, assign76720_e116256_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1792 != 0.0)) {
        let assign76720_e116251: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign76720_e116253: f64 = (assign76720_e116251 / 2.0);
        let assign76720_e116254: f64 = (locals.var_vgpld + assign76720_e116253);
        (assign76720_e116254, (((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0), (locals.var_vgpld_dn2 + (((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0)), (((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0), (((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0), (((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0), (locals.var_vgpld_dn7 + (((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0)), (locals.var_vgpld_dn8 + (((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0)), (locals.var_vgpld_dn9 + (((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0)), (((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0), (((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0), (((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign76720_e116256;
        locals.var_ps0_inia_dn0 = assign76720_e116256_d_n0;
        locals.var_ps0_inia_dn2 = assign76720_e116256_d_n2;
        locals.var_ps0_inia_dn4 = assign76720_e116256_d_n4;
        locals.var_ps0_inia_dn5 = assign76720_e116256_d_n5;
        locals.var_ps0_inia_dn6 = assign76720_e116256_d_n6;
        locals.var_ps0_inia_dn7 = assign76720_e116256_d_n7;
        locals.var_ps0_inia_dn8 = assign76720_e116256_d_n8;
        locals.var_ps0_inia_dn9 = assign76720_e116256_d_n9;
        locals.var_ps0_inia_dn10 = assign76720_e116256_d_n10;
        locals.var_ps0_inia_dn11 = assign76720_e116256_d_n11;
        locals.var_ps0_inia_dn14 = assign76720_e116256_d_n14;

        let (assign76730_e116280, assign76730_e116280_d_n0, assign76730_e116280_d_n2, assign76730_e116280_d_n4, assign76730_e116280_d_n5, assign76730_e116280_d_n6, assign76730_e116280_d_n7, assign76730_e116280_d_n8, assign76730_e116280_d_n9, assign76730_e116280_d_n10, assign76730_e116280_d_n11, assign76730_e116280_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1792 == 0.0)) {
        let assign76730_e116269: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76730_e116270: f64 = (locals.var_beta * assign76730_e116269);
        let assign76730_e116272: f64 = (assign76730_e116270 - 1.0);
        let assign76730_e116273: f64 = (4.0 * assign76730_e116272);
        let assign76730_e116276: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign76730_e116277: f64 = (assign76730_e116273 / assign76730_e116276);
        let assign76730_e116278: f64 = (1.0 + assign76730_e116277);
        (assign76730_e116278, ((((4.0 * ((locals.var_beta_dn0 * assign76730_e116269) + (locals.var_beta * locals.var_vxbgmtcl_dn0))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn2 * assign76730_e116269) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn4 * assign76730_e116269) + (locals.var_beta * locals.var_vxbgmtcl_dn4))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn5 * assign76730_e116269) + (locals.var_beta * locals.var_vxbgmtcl_dn5))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn6 * assign76730_e116269) + (locals.var_beta * locals.var_vxbgmtcl_dn6))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn7 * assign76730_e116269) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn8 * assign76730_e116269) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn9 * assign76730_e116269) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9)))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn10 * assign76730_e116269) + (locals.var_beta * locals.var_vxbgmtcl_dn10))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn11 * assign76730_e116269) + (locals.var_beta * locals.var_vxbgmtcl_dn11))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign76730_e116276 * assign76730_e116276)), ((((4.0 * ((locals.var_beta_dn14 * assign76730_e116269) + (locals.var_beta * locals.var_vxbgmtcl_dn14))) * assign76730_e116276) - (assign76730_e116273 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign76730_e116276 * assign76730_e116276)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign76730_e116280;
        locals.var_tx_dn0 = assign76730_e116280_d_n0;
        locals.var_tx_dn2 = assign76730_e116280_d_n2;
        locals.var_tx_dn4 = assign76730_e116280_d_n4;
        locals.var_tx_dn5 = assign76730_e116280_d_n5;
        locals.var_tx_dn6 = assign76730_e116280_d_n6;
        locals.var_tx_dn7 = assign76730_e116280_d_n7;
        locals.var_tx_dn8 = assign76730_e116280_d_n8;
        locals.var_tx_dn9 = assign76730_e116280_d_n9;
        locals.var_tx_dn10 = assign76730_e116280_d_n10;
        locals.var_tx_dn11 = assign76730_e116280_d_n11;
        locals.var_tx_dn14 = assign76730_e116280_d_n14;

        let (assign76740_e116301, assign76740_e116301_d_n0, assign76740_e116301_d_n2, assign76740_e116301_d_n4, assign76740_e116301_d_n5, assign76740_e116301_d_n6, assign76740_e116301_d_n7, assign76740_e116301_d_n8, assign76740_e116301_d_n9, assign76740_e116301_d_n10, assign76740_e116301_d_n11, assign76740_e116301_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1792 == 0.0)) {
        let assign76740_e116291: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign76740_e116293: f64 = (assign76740_e116291 / 2.0);
        let assign76740_e116296: f64 = (locals.var_tx).sqrt();
        let assign76740_e116297: f64 = (1.0 - assign76740_e116296);
        let assign76740_e116298: f64 = (assign76740_e116293 * assign76740_e116297);
        let assign76740_e116299: f64 = (locals.var_vgpld + assign76740_e116298);
        (assign76740_e116299, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn0 / (2.0 * assign76740_e116296))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn2 / (2.0 * assign76740_e116296)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn4 / (2.0 * assign76740_e116296))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn5 / (2.0 * assign76740_e116296))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn6 / (2.0 * assign76740_e116296))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn7 / (2.0 * assign76740_e116296)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn8 / (2.0 * assign76740_e116296)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn9 / (2.0 * assign76740_e116296)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn10 / (2.0 * assign76740_e116296))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn11 / (2.0 * assign76740_e116296))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign76740_e116297) + (assign76740_e116293 * (-(locals.var_tx_dn14 / (2.0 * assign76740_e116296))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign76740_e116301;
        locals.var_ps0_inia_dn0 = assign76740_e116301_d_n0;
        locals.var_ps0_inia_dn2 = assign76740_e116301_d_n2;
        locals.var_ps0_inia_dn4 = assign76740_e116301_d_n4;
        locals.var_ps0_inia_dn5 = assign76740_e116301_d_n5;
        locals.var_ps0_inia_dn6 = assign76740_e116301_d_n6;
        locals.var_ps0_inia_dn7 = assign76740_e116301_d_n7;
        locals.var_ps0_inia_dn8 = assign76740_e116301_d_n8;
        locals.var_ps0_inia_dn9 = assign76740_e116301_d_n9;
        locals.var_ps0_inia_dn10 = assign76740_e116301_d_n10;
        locals.var_ps0_inia_dn11 = assign76740_e116301_d_n11;
        locals.var_ps0_inia_dn14 = assign76740_e116301_d_n14;

        let (assign76750_e116312, assign76750_e116312_d_n0, assign76750_e116312_d_n2, assign76750_e116312_d_n4, assign76750_e116312_d_n5, assign76750_e116312_d_n6, assign76750_e116312_d_n7, assign76750_e116312_d_n8, assign76750_e116312_d_n9, assign76750_e116312_d_n10, assign76750_e116312_d_n11, assign76750_e116312_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) {
        let assign76750_e116309: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign76750_e116310: f64 = (locals.var_beta * assign76750_e116309);
        (assign76750_e116310, ((locals.var_beta_dn0 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign76750_e116309) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign76750_e116312;
        locals.var_chi_dn0 = assign76750_e116312_d_n0;
        locals.var_chi_dn2 = assign76750_e116312_d_n2;
        locals.var_chi_dn4 = assign76750_e116312_d_n4;
        locals.var_chi_dn5 = assign76750_e116312_d_n5;
        locals.var_chi_dn6 = assign76750_e116312_d_n6;
        locals.var_chi_dn7 = assign76750_e116312_d_n7;
        locals.var_chi_dn8 = assign76750_e116312_d_n8;
        locals.var_chi_dn9 = assign76750_e116312_d_n9;
        locals.var_chi_dn10 = assign76750_e116312_d_n10;
        locals.var_chi_dn11 = assign76750_e116312_d_n11;
        locals.var_chi_dn14 = assign76750_e116312_d_n14;

        let assign76760_e116315: f64 = if locals.var_chi >= 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1793 = assign76760_e116315;

        let (assign76780_e116335, assign76780_e116335_d_n0, assign76780_e116335_d_n2, assign76780_e116335_d_n4, assign76780_e116335_d_n5, assign76780_e116335_d_n6, assign76780_e116335_d_n7, assign76780_e116335_d_n8, assign76780_e116335_d_n9, assign76780_e116335_d_n10, assign76780_e116335_d_n11, assign76780_e116335_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign76780_e116332: f64 = (-locals.var_chi);
        let assign76780_e116333: f64 = (assign76780_e116332).exp();
        (assign76780_e116333, (assign76780_e116333 * (-locals.var_chi_dn0)), (assign76780_e116333 * (-locals.var_chi_dn2)), (assign76780_e116333 * (-locals.var_chi_dn4)), (assign76780_e116333 * (-locals.var_chi_dn5)), (assign76780_e116333 * (-locals.var_chi_dn6)), (assign76780_e116333 * (-locals.var_chi_dn7)), (assign76780_e116333 * (-locals.var_chi_dn8)), (assign76780_e116333 * (-locals.var_chi_dn9)), (assign76780_e116333 * (-locals.var_chi_dn10)), (assign76780_e116333 * (-locals.var_chi_dn11)), (assign76780_e116333 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign76780_e116335;
        locals.var_ty_dn0 = assign76780_e116335_d_n0;
        locals.var_ty_dn2 = assign76780_e116335_d_n2;
        locals.var_ty_dn4 = assign76780_e116335_d_n4;
        locals.var_ty_dn5 = assign76780_e116335_d_n5;
        locals.var_ty_dn6 = assign76780_e116335_d_n6;
        locals.var_ty_dn7 = assign76780_e116335_d_n7;
        locals.var_ty_dn8 = assign76780_e116335_d_n8;
        locals.var_ty_dn9 = assign76780_e116335_d_n9;
        locals.var_ty_dn10 = assign76780_e116335_d_n10;
        locals.var_ty_dn11 = assign76780_e116335_d_n11;
        locals.var_ty_dn14 = assign76780_e116335_d_n14;

        let (assign76790_e116360, assign76790_e116360_d_n0, assign76790_e116360_d_n2, assign76790_e116360_d_n4, assign76790_e116360_d_n5, assign76790_e116360_d_n6, assign76790_e116360_d_n7, assign76790_e116360_d_n8, assign76790_e116360_d_n9, assign76790_e116360_d_n10, assign76790_e116360_d_n11, assign76790_e116360_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign76790_e116347: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76790_e116348: f64 = (locals.var_beta * assign76790_e116347);
        let assign76790_e116350: f64 = (assign76790_e116348 - 1.0);
        let assign76790_e116352: f64 = (assign76790_e116350 + locals.var_ty);
        let assign76790_e116353: f64 = (4.0 * assign76790_e116352);
        let assign76790_e116356: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign76790_e116357: f64 = (assign76790_e116353 / assign76790_e116356);
        let assign76790_e116358: f64 = (1.0 + assign76790_e116357);
        (assign76790_e116358, ((((4.0 * (((locals.var_beta_dn0 * assign76790_e116347) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn2 * assign76790_e116347) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn4 * assign76790_e116347) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn5 * assign76790_e116347) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn6 * assign76790_e116347) + (locals.var_beta * locals.var_vxbgmtcl_dn6)) + locals.var_ty_dn6)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn7 * assign76790_e116347) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn8 * assign76790_e116347) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn9 * assign76790_e116347) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))) + locals.var_ty_dn9)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn10 * assign76790_e116347) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn11 * assign76790_e116347) + (locals.var_beta * locals.var_vxbgmtcl_dn11)) + locals.var_ty_dn11)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign76790_e116356 * assign76790_e116356)), ((((4.0 * (((locals.var_beta_dn14 * assign76790_e116347) + (locals.var_beta * locals.var_vxbgmtcl_dn14)) + locals.var_ty_dn14)) * assign76790_e116356) - (assign76790_e116353 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign76790_e116356 * assign76790_e116356)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign76790_e116360;
        locals.var_tx_dn0 = assign76790_e116360_d_n0;
        locals.var_tx_dn2 = assign76790_e116360_d_n2;
        locals.var_tx_dn4 = assign76790_e116360_d_n4;
        locals.var_tx_dn5 = assign76790_e116360_d_n5;
        locals.var_tx_dn6 = assign76790_e116360_d_n6;
        locals.var_tx_dn7 = assign76790_e116360_d_n7;
        locals.var_tx_dn8 = assign76790_e116360_d_n8;
        locals.var_tx_dn9 = assign76790_e116360_d_n9;
        locals.var_tx_dn10 = assign76790_e116360_d_n10;
        locals.var_tx_dn11 = assign76790_e116360_d_n11;
        locals.var_tx_dn14 = assign76790_e116360_d_n14;

        let (assign76800_e116380, assign76800_e116380_d_n0, assign76800_e116380_d_n2, assign76800_e116380_d_n4, assign76800_e116380_d_n5, assign76800_e116380_d_n6, assign76800_e116380_d_n7, assign76800_e116380_d_n8, assign76800_e116380_d_n9, assign76800_e116380_d_n10, assign76800_e116380_d_n11, assign76800_e116380_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign76800_e116370: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign76800_e116372: f64 = (assign76800_e116370 / 2.0);
        let assign76800_e116375: f64 = (locals.var_tx).sqrt();
        let assign76800_e116376: f64 = (1.0 - assign76800_e116375);
        let assign76800_e116377: f64 = (assign76800_e116372 * assign76800_e116376);
        let assign76800_e116378: f64 = (locals.var_vgpld + assign76800_e116377);
        (assign76800_e116378, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn0 / (2.0 * assign76800_e116375))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn2 / (2.0 * assign76800_e116375)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn4 / (2.0 * assign76800_e116375))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn5 / (2.0 * assign76800_e116375))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn6 / (2.0 * assign76800_e116375))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn7 / (2.0 * assign76800_e116375)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn8 / (2.0 * assign76800_e116375)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn9 / (2.0 * assign76800_e116375)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn10 / (2.0 * assign76800_e116375))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn11 / (2.0 * assign76800_e116375))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign76800_e116376) + (assign76800_e116372 * (-(locals.var_tx_dn14 / (2.0 * assign76800_e116375))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign76800_e116380;
        locals.var_ps0_inia_dn0 = assign76800_e116380_d_n0;
        locals.var_ps0_inia_dn2 = assign76800_e116380_d_n2;
        locals.var_ps0_inia_dn4 = assign76800_e116380_d_n4;
        locals.var_ps0_inia_dn5 = assign76800_e116380_d_n5;
        locals.var_ps0_inia_dn6 = assign76800_e116380_d_n6;
        locals.var_ps0_inia_dn7 = assign76800_e116380_d_n7;
        locals.var_ps0_inia_dn8 = assign76800_e116380_d_n8;
        locals.var_ps0_inia_dn9 = assign76800_e116380_d_n9;
        locals.var_ps0_inia_dn10 = assign76800_e116380_d_n10;
        locals.var_ps0_inia_dn11 = assign76800_e116380_d_n11;
        locals.var_ps0_inia_dn14 = assign76800_e116380_d_n14;

        let (assign76810_e116393, assign76810_e116393_d_n0, assign76810_e116393_d_n2, assign76810_e116393_d_n4, assign76810_e116393_d_n5, assign76810_e116393_d_n6, assign76810_e116393_d_n7, assign76810_e116393_d_n8, assign76810_e116393_d_n9, assign76810_e116393_d_n10, assign76810_e116393_d_n11, assign76810_e116393_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign76810_e116390: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign76810_e116391: f64 = (locals.var_beta * assign76810_e116390);
        (assign76810_e116391, ((locals.var_beta_dn0 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign76810_e116390) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign76810_e116393;
        locals.var_chi_dn0 = assign76810_e116393_d_n0;
        locals.var_chi_dn2 = assign76810_e116393_d_n2;
        locals.var_chi_dn4 = assign76810_e116393_d_n4;
        locals.var_chi_dn5 = assign76810_e116393_d_n5;
        locals.var_chi_dn6 = assign76810_e116393_d_n6;
        locals.var_chi_dn7 = assign76810_e116393_d_n7;
        locals.var_chi_dn8 = assign76810_e116393_d_n8;
        locals.var_chi_dn9 = assign76810_e116393_d_n9;
        locals.var_chi_dn10 = assign76810_e116393_d_n10;
        locals.var_chi_dn11 = assign76810_e116393_d_n11;
        locals.var_chi_dn14 = assign76810_e116393_d_n14;

        let (assign76820_e116404, assign76820_e116404_d_n0, assign76820_e116404_d_n2, assign76820_e116404_d_n4, assign76820_e116404_d_n5, assign76820_e116404_d_n6, assign76820_e116404_d_n7, assign76820_e116404_d_n8, assign76820_e116404_d_n9, assign76820_e116404_d_n10, assign76820_e116404_d_n11, assign76820_e116404_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign76820_e116401: f64 = (-locals.var_chi);
        let assign76820_e116402: f64 = (assign76820_e116401).exp();
        (assign76820_e116402, (assign76820_e116402 * (-locals.var_chi_dn0)), (assign76820_e116402 * (-locals.var_chi_dn2)), (assign76820_e116402 * (-locals.var_chi_dn4)), (assign76820_e116402 * (-locals.var_chi_dn5)), (assign76820_e116402 * (-locals.var_chi_dn6)), (assign76820_e116402 * (-locals.var_chi_dn7)), (assign76820_e116402 * (-locals.var_chi_dn8)), (assign76820_e116402 * (-locals.var_chi_dn9)), (assign76820_e116402 * (-locals.var_chi_dn10)), (assign76820_e116402 * (-locals.var_chi_dn11)), (assign76820_e116402 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign76820_e116404;
        locals.var_ty_dn0 = assign76820_e116404_d_n0;
        locals.var_ty_dn2 = assign76820_e116404_d_n2;
        locals.var_ty_dn4 = assign76820_e116404_d_n4;
        locals.var_ty_dn5 = assign76820_e116404_d_n5;
        locals.var_ty_dn6 = assign76820_e116404_d_n6;
        locals.var_ty_dn7 = assign76820_e116404_d_n7;
        locals.var_ty_dn8 = assign76820_e116404_d_n8;
        locals.var_ty_dn9 = assign76820_e116404_d_n9;
        locals.var_ty_dn10 = assign76820_e116404_d_n10;
        locals.var_ty_dn11 = assign76820_e116404_d_n11;
        locals.var_ty_dn14 = assign76820_e116404_d_n14;

        let (assign76830_e116429, assign76830_e116429_d_n0, assign76830_e116429_d_n2, assign76830_e116429_d_n4, assign76830_e116429_d_n5, assign76830_e116429_d_n6, assign76830_e116429_d_n7, assign76830_e116429_d_n8, assign76830_e116429_d_n9, assign76830_e116429_d_n10, assign76830_e116429_d_n11, assign76830_e116429_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign76830_e116416: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76830_e116417: f64 = (locals.var_beta * assign76830_e116416);
        let assign76830_e116419: f64 = (assign76830_e116417 - 1.0);
        let assign76830_e116421: f64 = (assign76830_e116419 + locals.var_ty);
        let assign76830_e116422: f64 = (4.0 * assign76830_e116421);
        let assign76830_e116425: f64 = (locals.var_fac1p2 * locals.var_beta2);
        let assign76830_e116426: f64 = (assign76830_e116422 / assign76830_e116425);
        let assign76830_e116427: f64 = (1.0 + assign76830_e116426);
        (assign76830_e116427, ((((4.0 * (((locals.var_beta_dn0 * assign76830_e116416) + (locals.var_beta * locals.var_vxbgmtcl_dn0)) + locals.var_ty_dn0)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn0 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn0)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn2 * assign76830_e116416) + (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2))) + locals.var_ty_dn2)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn2 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn2)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn4 * assign76830_e116416) + (locals.var_beta * locals.var_vxbgmtcl_dn4)) + locals.var_ty_dn4)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn4 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn4)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn5 * assign76830_e116416) + (locals.var_beta * locals.var_vxbgmtcl_dn5)) + locals.var_ty_dn5)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn5 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn5)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn6 * assign76830_e116416) + (locals.var_beta * locals.var_vxbgmtcl_dn6)) + locals.var_ty_dn6)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn6 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn6)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn7 * assign76830_e116416) + (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7))) + locals.var_ty_dn7)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn7 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn7)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn8 * assign76830_e116416) + (locals.var_beta * (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8))) + locals.var_ty_dn8)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn8 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn8)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn9 * assign76830_e116416) + (locals.var_beta * (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9))) + locals.var_ty_dn9)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn9 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn9)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn10 * assign76830_e116416) + (locals.var_beta * locals.var_vxbgmtcl_dn10)) + locals.var_ty_dn10)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn10 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn10)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn11 * assign76830_e116416) + (locals.var_beta * locals.var_vxbgmtcl_dn11)) + locals.var_ty_dn11)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn11 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn11)))) / (assign76830_e116425 * assign76830_e116425)), ((((4.0 * (((locals.var_beta_dn14 * assign76830_e116416) + (locals.var_beta * locals.var_vxbgmtcl_dn14)) + locals.var_ty_dn14)) * assign76830_e116425) - (assign76830_e116422 * ((locals.var_fac1p2_dn14 * locals.var_beta2) + (locals.var_fac1p2 * locals.var_beta2_dn14)))) / (assign76830_e116425 * assign76830_e116425)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign76830_e116429;
        locals.var_tx_dn0 = assign76830_e116429_d_n0;
        locals.var_tx_dn2 = assign76830_e116429_d_n2;
        locals.var_tx_dn4 = assign76830_e116429_d_n4;
        locals.var_tx_dn5 = assign76830_e116429_d_n5;
        locals.var_tx_dn6 = assign76830_e116429_d_n6;
        locals.var_tx_dn7 = assign76830_e116429_d_n7;
        locals.var_tx_dn8 = assign76830_e116429_d_n8;
        locals.var_tx_dn9 = assign76830_e116429_d_n9;
        locals.var_tx_dn10 = assign76830_e116429_d_n10;
        locals.var_tx_dn11 = assign76830_e116429_d_n11;
        locals.var_tx_dn14 = assign76830_e116429_d_n14;

        let (assign76840_e116449, assign76840_e116449_d_n0, assign76840_e116449_d_n2, assign76840_e116449_d_n4, assign76840_e116449_d_n5, assign76840_e116449_d_n6, assign76840_e116449_d_n7, assign76840_e116449_d_n8, assign76840_e116449_d_n9, assign76840_e116449_d_n10, assign76840_e116449_d_n11, assign76840_e116449_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign76840_e116439: f64 = (locals.var_fac1p2 * locals.var_beta);
        let assign76840_e116441: f64 = (assign76840_e116439 / 2.0);
        let assign76840_e116444: f64 = (locals.var_tx).sqrt();
        let assign76840_e116445: f64 = (1.0 - assign76840_e116444);
        let assign76840_e116446: f64 = (assign76840_e116441 * assign76840_e116445);
        let assign76840_e116447: f64 = (locals.var_vgpld + assign76840_e116446);
        (assign76840_e116447, (((((locals.var_fac1p2_dn0 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn0)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn0 / (2.0 * assign76840_e116444))))), (locals.var_vgpld_dn2 + (((((locals.var_fac1p2_dn2 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn2)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn2 / (2.0 * assign76840_e116444)))))), (((((locals.var_fac1p2_dn4 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn4)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn4 / (2.0 * assign76840_e116444))))), (((((locals.var_fac1p2_dn5 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn5)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn5 / (2.0 * assign76840_e116444))))), (((((locals.var_fac1p2_dn6 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn6)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn6 / (2.0 * assign76840_e116444))))), (locals.var_vgpld_dn7 + (((((locals.var_fac1p2_dn7 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn7)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn7 / (2.0 * assign76840_e116444)))))), (locals.var_vgpld_dn8 + (((((locals.var_fac1p2_dn8 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn8)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn8 / (2.0 * assign76840_e116444)))))), (locals.var_vgpld_dn9 + (((((locals.var_fac1p2_dn9 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn9)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn9 / (2.0 * assign76840_e116444)))))), (((((locals.var_fac1p2_dn10 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn10)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn10 / (2.0 * assign76840_e116444))))), (((((locals.var_fac1p2_dn11 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn11)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn11 / (2.0 * assign76840_e116444))))), (((((locals.var_fac1p2_dn14 * locals.var_beta) + (locals.var_fac1p2 * locals.var_beta_dn14)) / 2.0) * assign76840_e116445) + (assign76840_e116441 * (-(locals.var_tx_dn14 / (2.0 * assign76840_e116444))))),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign76840_e116449;
        locals.var_ps0_inia_dn0 = assign76840_e116449_d_n0;
        locals.var_ps0_inia_dn2 = assign76840_e116449_d_n2;
        locals.var_ps0_inia_dn4 = assign76840_e116449_d_n4;
        locals.var_ps0_inia_dn5 = assign76840_e116449_d_n5;
        locals.var_ps0_inia_dn6 = assign76840_e116449_d_n6;
        locals.var_ps0_inia_dn7 = assign76840_e116449_d_n7;
        locals.var_ps0_inia_dn8 = assign76840_e116449_d_n8;
        locals.var_ps0_inia_dn9 = assign76840_e116449_d_n9;
        locals.var_ps0_inia_dn10 = assign76840_e116449_d_n10;
        locals.var_ps0_inia_dn11 = assign76840_e116449_d_n11;
        locals.var_ps0_inia_dn14 = assign76840_e116449_d_n14;

    }

    pub(super) fn stamp_transient_block_277(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign76850_e116462, assign76850_e116462_d_n0, assign76850_e116462_d_n2, assign76850_e116462_d_n4, assign76850_e116462_d_n5, assign76850_e116462_d_n6, assign76850_e116462_d_n7, assign76850_e116462_d_n8, assign76850_e116462_d_n9, assign76850_e116462_d_n10, assign76850_e116462_d_n11, assign76850_e116462_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 != 0.0)) {
        let assign76850_e116459: f64 = (locals.var_ps0_inia + locals.var_vxbgmtcl);
        let assign76850_e116460: f64 = (locals.var_beta * assign76850_e116459);
        (assign76850_e116460, ((locals.var_beta_dn0 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn0 + locals.var_vxbgmtcl_dn0))), ((locals.var_beta_dn2 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn2 + locals.var_vxbgmtcl_dn2))), ((locals.var_beta_dn4 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn4 + locals.var_vxbgmtcl_dn4))), ((locals.var_beta_dn5 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn5 + locals.var_vxbgmtcl_dn5))), ((locals.var_beta_dn6 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn6 + locals.var_vxbgmtcl_dn6))), ((locals.var_beta_dn7 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn7 + locals.var_vxbgmtcl_dn7))), ((locals.var_beta_dn8 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn8 + locals.var_vxbgmtcl_dn8))), ((locals.var_beta_dn9 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn9 + locals.var_vxbgmtcl_dn9))), ((locals.var_beta_dn10 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn10 + locals.var_vxbgmtcl_dn10))), ((locals.var_beta_dn11 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn11 + locals.var_vxbgmtcl_dn11))), ((locals.var_beta_dn14 * assign76850_e116459) + (locals.var_beta * (locals.var_ps0_inia_dn14 + locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign76850_e116462;
        locals.var_chi_dn0 = assign76850_e116462_d_n0;
        locals.var_chi_dn2 = assign76850_e116462_d_n2;
        locals.var_chi_dn4 = assign76850_e116462_d_n4;
        locals.var_chi_dn5 = assign76850_e116462_d_n5;
        locals.var_chi_dn6 = assign76850_e116462_d_n6;
        locals.var_chi_dn7 = assign76850_e116462_d_n7;
        locals.var_chi_dn8 = assign76850_e116462_d_n8;
        locals.var_chi_dn9 = assign76850_e116462_d_n9;
        locals.var_chi_dn10 = assign76850_e116462_d_n10;
        locals.var_chi_dn11 = assign76850_e116462_d_n11;
        locals.var_chi_dn14 = assign76850_e116462_d_n14;

        let (assign76870_e116504,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76870_e116483: f64 = (2.0_f64).sqrt();
        let assign76870_e116484: f64 = (9.0 * assign76870_e116483);
        let assign76870_e116485: f64 = (1.0 / assign76870_e116484);
        let assign76870_e116489: f64 = (-3.0);
        let assign76870_e116490: f64 = (assign76870_e116489).exp();
        let assign76870_e116491: f64 = (7.0 * assign76870_e116490);
        let assign76870_e116492: f64 = (5.0 + assign76870_e116491);
        let assign76870_e116496: f64 = (-3.0);
        let assign76870_e116497: f64 = (assign76870_e116496).exp();
        let assign76870_e116498: f64 = (2.0 + assign76870_e116497);
        let assign76870_e116499: f64 = (assign76870_e116498).sqrt();
        let assign76870_e116500: f64 = (54.0 * assign76870_e116499);
        let assign76870_e116501: f64 = (assign76870_e116492 / assign76870_e116500);
        let assign76870_e116502: f64 = (assign76870_e116485 - assign76870_e116501);
        (assign76870_e116502,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign76870_e116504;

        let (assign76880_e116532,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76880_e116514: f64 = (-3.0);
        let assign76880_e116515: f64 = (assign76880_e116514).exp();
        let assign76880_e116516: f64 = (1.0 + assign76880_e116515);
        let assign76880_e116520: f64 = (-3.0);
        let assign76880_e116521: f64 = (assign76880_e116520).exp();
        let assign76880_e116522: f64 = (2.0 + assign76880_e116521);
        let assign76880_e116523: f64 = (assign76880_e116522).sqrt();
        let assign76880_e116524: f64 = (2.0 * assign76880_e116523);
        let assign76880_e116525: f64 = (assign76880_e116516 / assign76880_e116524);
        let assign76880_e116527: f64 = (2.0_f64).sqrt();
        let assign76880_e116529: f64 = (assign76880_e116527 / 3.0);
        let assign76880_e116530: f64 = (assign76880_e116525 - assign76880_e116529);
        (assign76880_e116530,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign76880_e116532;

        let (assign76890_e116551, assign76890_e116551_d_n0, assign76890_e116551_d_n2, assign76890_e116551_d_n4, assign76890_e116551_d_n5, assign76890_e116551_d_n6, assign76890_e116551_d_n7, assign76890_e116551_d_n8, assign76890_e116551_d_n9, assign76890_e116551_d_n10, assign76890_e116551_d_n11, assign76890_e116551_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76890_e116542: f64 = (2.0_f64).sqrt();
        let assign76890_e116543: f64 = (1.0 / assign76890_e116542);
        let assign76890_e116547: f64 = (locals.var_beta * locals.var_fac1);
        let assign76890_e116548: f64 = (1.0 / assign76890_e116547);
        let assign76890_e116549: f64 = (assign76890_e116543 + assign76890_e116548);
        (assign76890_e116549, (-(((locals.var_beta_dn0 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn0)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn2 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn2)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn4 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn4)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn5 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn5)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn6 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn6)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn7 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn7)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn8 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn8)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn9 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn9)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn10 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn10)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn11 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn11)) / (assign76890_e116547 * assign76890_e116547))), (-(((locals.var_beta_dn14 * locals.var_fac1) + (locals.var_beta * locals.var_fac1_dn14)) / (assign76890_e116547 * assign76890_e116547))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn4, locals.var_tc_dn5, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn8, locals.var_tc_dn9, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn14,)
    }
};
        locals.var_tc = assign76890_e116551;
        locals.var_tc_dn0 = assign76890_e116551_d_n0;
        locals.var_tc_dn2 = assign76890_e116551_d_n2;
        locals.var_tc_dn4 = assign76890_e116551_d_n4;
        locals.var_tc_dn5 = assign76890_e116551_d_n5;
        locals.var_tc_dn6 = assign76890_e116551_d_n6;
        locals.var_tc_dn7 = assign76890_e116551_d_n7;
        locals.var_tc_dn8 = assign76890_e116551_d_n8;
        locals.var_tc_dn9 = assign76890_e116551_d_n9;
        locals.var_tc_dn10 = assign76890_e116551_d_n10;
        locals.var_tc_dn11 = assign76890_e116551_d_n11;
        locals.var_tc_dn14 = assign76890_e116551_d_n14;

        let (assign76900_e116566, assign76900_e116566_d_n0, assign76900_e116566_d_n2, assign76900_e116566_d_n4, assign76900_e116566_d_n5, assign76900_e116566_d_n6, assign76900_e116566_d_n7, assign76900_e116566_d_n8, assign76900_e116566_d_n9, assign76900_e116566_d_n10, assign76900_e116566_d_n11, assign76900_e116566_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76900_e116561: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76900_e116562: f64 = (-assign76900_e116561);
        let assign76900_e116564: f64 = (assign76900_e116562 / locals.var_fac1);
        (assign76900_e116564, ((((-locals.var_vxbgmtcl_dn0) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn0)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn2)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn4) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn4)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn5) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn5)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn6) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn6)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn7)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8)) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn8)) / (locals.var_fac1 * locals.var_fac1)), ((((-(locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9)) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn9)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn10) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn10)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn11) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn11)) / (locals.var_fac1 * locals.var_fac1)), ((((-locals.var_vxbgmtcl_dn14) * locals.var_fac1) - (assign76900_e116562 * locals.var_fac1_dn14)) / (locals.var_fac1 * locals.var_fac1)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn4, locals.var_td_dn5, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn8, locals.var_td_dn9, locals.var_td_dn10, locals.var_td_dn11, locals.var_td_dn14,)
    }
};
        locals.var_td = assign76900_e116566;
        locals.var_td_dn0 = assign76900_e116566_d_n0;
        locals.var_td_dn2 = assign76900_e116566_d_n2;
        locals.var_td_dn4 = assign76900_e116566_d_n4;
        locals.var_td_dn5 = assign76900_e116566_d_n5;
        locals.var_td_dn6 = assign76900_e116566_d_n6;
        locals.var_td_dn7 = assign76900_e116566_d_n7;
        locals.var_td_dn8 = assign76900_e116566_d_n8;
        locals.var_td_dn9 = assign76900_e116566_d_n9;
        locals.var_td_dn10 = assign76900_e116566_d_n10;
        locals.var_td_dn11 = assign76900_e116566_d_n11;
        locals.var_td_dn14 = assign76900_e116566_d_n14;

        let (assign76910_e116604, assign76910_e116604_d_n0, assign76910_e116604_d_n2, assign76910_e116604_d_n4, assign76910_e116604_d_n5, assign76910_e116604_d_n6, assign76910_e116604_d_n7, assign76910_e116604_d_n8, assign76910_e116604_d_n9, assign76910_e116604_d_n10, assign76910_e116604_d_n11, assign76910_e116604_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76910_e116576: f64 = (locals.var_tb * locals.var_tb);
        let assign76910_e116578: f64 = (assign76910_e116576 * locals.var_tb);
        let assign76910_e116581: f64 = (27.0 * locals.var_ta);
        let assign76910_e116583: f64 = (assign76910_e116581 * locals.var_ta);
        let assign76910_e116585: f64 = (assign76910_e116583 * locals.var_ta);
        let assign76910_e116586: f64 = (assign76910_e116578 / assign76910_e116585);
        let assign76910_e116589: f64 = (locals.var_tb * locals.var_tc);
        let assign76910_e116592: f64 = (6.0 * locals.var_ta);
        let assign76910_e116594: f64 = (assign76910_e116592 * locals.var_ta);
        let assign76910_e116595: f64 = (assign76910_e116589 / assign76910_e116594);
        let assign76910_e116596: f64 = (assign76910_e116586 - assign76910_e116595);
        let assign76910_e116600: f64 = (2.0 * locals.var_ta);
        let assign76910_e116601: f64 = (locals.var_td / assign76910_e116600);
        let assign76910_e116602: f64 = (assign76910_e116596 + assign76910_e116601);
        (assign76910_e116602, ((-((locals.var_tb * locals.var_tc_dn0) / assign76910_e116594)) + (locals.var_td_dn0 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn2) / assign76910_e116594)) + (locals.var_td_dn2 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn4) / assign76910_e116594)) + (locals.var_td_dn4 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn5) / assign76910_e116594)) + (locals.var_td_dn5 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn6) / assign76910_e116594)) + (locals.var_td_dn6 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn7) / assign76910_e116594)) + (locals.var_td_dn7 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn8) / assign76910_e116594)) + (locals.var_td_dn8 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn9) / assign76910_e116594)) + (locals.var_td_dn9 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn10) / assign76910_e116594)) + (locals.var_td_dn10 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn11) / assign76910_e116594)) + (locals.var_td_dn11 / assign76910_e116600)), ((-((locals.var_tb * locals.var_tc_dn14) / assign76910_e116594)) + (locals.var_td_dn14 / assign76910_e116600)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn4, locals.var_tq_dn5, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn8, locals.var_tq_dn9, locals.var_tq_dn10, locals.var_tq_dn11, locals.var_tq_dn14,)
    }
};
        locals.var_tq = assign76910_e116604;
        locals.var_tq_dn0 = assign76910_e116604_d_n0;
        locals.var_tq_dn2 = assign76910_e116604_d_n2;
        locals.var_tq_dn4 = assign76910_e116604_d_n4;
        locals.var_tq_dn5 = assign76910_e116604_d_n5;
        locals.var_tq_dn6 = assign76910_e116604_d_n6;
        locals.var_tq_dn7 = assign76910_e116604_d_n7;
        locals.var_tq_dn8 = assign76910_e116604_d_n8;
        locals.var_tq_dn9 = assign76910_e116604_d_n9;
        locals.var_tq_dn10 = assign76910_e116604_d_n10;
        locals.var_tq_dn11 = assign76910_e116604_d_n11;
        locals.var_tq_dn14 = assign76910_e116604_d_n14;

        let (assign76920_e116628, assign76920_e116628_d_n0, assign76920_e116628_d_n2, assign76920_e116628_d_n4, assign76920_e116628_d_n5, assign76920_e116628_d_n6, assign76920_e116628_d_n7, assign76920_e116628_d_n8, assign76920_e116628_d_n9, assign76920_e116628_d_n10, assign76920_e116628_d_n11, assign76920_e116628_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76920_e116614: f64 = (3.0 * locals.var_ta);
        let assign76920_e116616: f64 = (assign76920_e116614 * locals.var_tc);
        let assign76920_e116619: f64 = (locals.var_tb * locals.var_tb);
        let assign76920_e116620: f64 = (assign76920_e116616 - assign76920_e116619);
        let assign76920_e116623: f64 = (9.0 * locals.var_ta);
        let assign76920_e116625: f64 = (assign76920_e116623 * locals.var_ta);
        let assign76920_e116626: f64 = (assign76920_e116620 / assign76920_e116625);
        (assign76920_e116626, ((assign76920_e116614 * locals.var_tc_dn0) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn2) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn4) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn5) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn6) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn7) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn8) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn9) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn10) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn11) / assign76920_e116625), ((assign76920_e116614 * locals.var_tc_dn14) / assign76920_e116625),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn4, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9, locals.var_tp_dn10, locals.var_tp_dn11, locals.var_tp_dn14,)
    }
};
        locals.var_tp = assign76920_e116628;
        locals.var_tp_dn0 = assign76920_e116628_d_n0;
        locals.var_tp_dn2 = assign76920_e116628_d_n2;
        locals.var_tp_dn4 = assign76920_e116628_d_n4;
        locals.var_tp_dn5 = assign76920_e116628_d_n5;
        locals.var_tp_dn6 = assign76920_e116628_d_n6;
        locals.var_tp_dn7 = assign76920_e116628_d_n7;
        locals.var_tp_dn8 = assign76920_e116628_d_n8;
        locals.var_tp_dn9 = assign76920_e116628_d_n9;
        locals.var_tp_dn10 = assign76920_e116628_d_n10;
        locals.var_tp_dn11 = assign76920_e116628_d_n11;
        locals.var_tp_dn14 = assign76920_e116628_d_n14;

        let (assign76930_e116647, assign76930_e116647_d_n0, assign76930_e116647_d_n2, assign76930_e116647_d_n4, assign76930_e116647_d_n5, assign76930_e116647_d_n6, assign76930_e116647_d_n7, assign76930_e116647_d_n8, assign76930_e116647_d_n9, assign76930_e116647_d_n10, assign76930_e116647_d_n11, assign76930_e116647_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76930_e116638: f64 = (locals.var_tq * locals.var_tq);
        let assign76930_e116641: f64 = (locals.var_tp * locals.var_tp);
        let assign76930_e116643: f64 = (assign76930_e116641 * locals.var_tp);
        let assign76930_e116644: f64 = (assign76930_e116638 + assign76930_e116643);
        let assign76930_e116645: f64 = (assign76930_e116644).sqrt();
        (assign76930_e116645, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn0))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn2))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn4 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn4)) + ((((locals.var_tp_dn4 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn4)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn4))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn5 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn5)) + ((((locals.var_tp_dn5 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn5)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn5))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn6))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn7))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn8 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn8)) + ((((locals.var_tp_dn8 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn8)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn8))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn9 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn9)) + ((((locals.var_tp_dn9 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn9)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn9))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn10))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn11 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn11)) + ((((locals.var_tp_dn11 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn11)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn11))) / (2.0 * assign76930_e116645)), ((((locals.var_tq_dn14 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn14)) + ((((locals.var_tp_dn14 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn14)) * locals.var_tp) + (assign76930_e116641 * locals.var_tp_dn14))) / (2.0 * assign76930_e116645)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign76930_e116647;
        locals.var_t5_dn0 = assign76930_e116647_d_n0;
        locals.var_t5_dn2 = assign76930_e116647_d_n2;
        locals.var_t5_dn4 = assign76930_e116647_d_n4;
        locals.var_t5_dn5 = assign76930_e116647_d_n5;
        locals.var_t5_dn6 = assign76930_e116647_d_n6;
        locals.var_t5_dn7 = assign76930_e116647_d_n7;
        locals.var_t5_dn8 = assign76930_e116647_d_n8;
        locals.var_t5_dn9 = assign76930_e116647_d_n9;
        locals.var_t5_dn10 = assign76930_e116647_d_n10;
        locals.var_t5_dn11 = assign76930_e116647_d_n11;
        locals.var_t5_dn14 = assign76930_e116647_d_n14;

        let (assign76940_e116662, assign76940_e116662_d_n0, assign76940_e116662_d_n2, assign76940_e116662_d_n4, assign76940_e116662_d_n5, assign76940_e116662_d_n6, assign76940_e116662_d_n7, assign76940_e116662_d_n8, assign76940_e116662_d_n9, assign76940_e116662_d_n10, assign76940_e116662_d_n11, assign76940_e116662_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76940_e116656: f64 = (-locals.var_tq);
        let assign76940_e116658: f64 = (assign76940_e116656 + locals.var_t5);
        let assign76940_e116660: f64 = (assign76940_e116658).powf(0.3333333333333333);
        (assign76940_e116660, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5_dn0))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5_dn0) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5_dn2))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5_dn2) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn4) + locals.var_t5_dn4))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn4) + locals.var_t5_dn4) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn5) + locals.var_t5_dn5))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn5) + locals.var_t5_dn5) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5_dn6))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5_dn6) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5_dn7))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5_dn7) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn8) + locals.var_t5_dn8))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn8) + locals.var_t5_dn8) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn9) + locals.var_t5_dn9))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn9) + locals.var_t5_dn9) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5_dn10))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5_dn10) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn11) + locals.var_t5_dn11))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn11) + locals.var_t5_dn11) / assign76940_e116658))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76940_e116658).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn14) + locals.var_t5_dn14))) } } else { (assign76940_e116660 * (0.3333333333333333 * (((-locals.var_tq_dn14) + locals.var_t5_dn14) / assign76940_e116658))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn4, locals.var_tu_dn5, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn8, locals.var_tu_dn9, locals.var_tu_dn10, locals.var_tu_dn11, locals.var_tu_dn14,)
    }
};
        locals.var_tu = assign76940_e116662;
        locals.var_tu_dn0 = assign76940_e116662_d_n0;
        locals.var_tu_dn2 = assign76940_e116662_d_n2;
        locals.var_tu_dn4 = assign76940_e116662_d_n4;
        locals.var_tu_dn5 = assign76940_e116662_d_n5;
        locals.var_tu_dn6 = assign76940_e116662_d_n6;
        locals.var_tu_dn7 = assign76940_e116662_d_n7;
        locals.var_tu_dn8 = assign76940_e116662_d_n8;
        locals.var_tu_dn9 = assign76940_e116662_d_n9;
        locals.var_tu_dn10 = assign76940_e116662_d_n10;
        locals.var_tu_dn11 = assign76940_e116662_d_n11;
        locals.var_tu_dn14 = assign76940_e116662_d_n14;

        let (assign76950_e116677, assign76950_e116677_d_n0, assign76950_e116677_d_n2, assign76950_e116677_d_n4, assign76950_e116677_d_n5, assign76950_e116677_d_n6, assign76950_e116677_d_n7, assign76950_e116677_d_n8, assign76950_e116677_d_n9, assign76950_e116677_d_n10, assign76950_e116677_d_n11, assign76950_e116677_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76950_e116672: f64 = (locals.var_tq + locals.var_t5);
        let assign76950_e116674: f64 = (assign76950_e116672).powf(0.3333333333333333);
        let assign76950_e116675: f64 = (-assign76950_e116674);
        (assign76950_e116675, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5_dn0))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5_dn0) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5_dn2))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5_dn2) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn4 + locals.var_t5_dn4))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn4 + locals.var_t5_dn4) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn5 + locals.var_t5_dn5))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn5 + locals.var_t5_dn5) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5_dn6))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5_dn6) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5_dn7))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5_dn7) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn8 + locals.var_t5_dn8))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn8 + locals.var_t5_dn8) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn9 + locals.var_t5_dn9))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn9 + locals.var_t5_dn9) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5_dn10))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5_dn10) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn11 + locals.var_t5_dn11))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn11 + locals.var_t5_dn11) / assign76950_e116672))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign76950_e116672).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn14 + locals.var_t5_dn14))) } } else { (assign76950_e116674 * (0.3333333333333333 * ((locals.var_tq_dn14 + locals.var_t5_dn14) / assign76950_e116672))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn4, locals.var_tv_dn5, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn8, locals.var_tv_dn9, locals.var_tv_dn10, locals.var_tv_dn11, locals.var_tv_dn14,)
    }
};
        locals.var_tv = assign76950_e116677;
        locals.var_tv_dn0 = assign76950_e116677_d_n0;
        locals.var_tv_dn2 = assign76950_e116677_d_n2;
        locals.var_tv_dn4 = assign76950_e116677_d_n4;
        locals.var_tv_dn5 = assign76950_e116677_d_n5;
        locals.var_tv_dn6 = assign76950_e116677_d_n6;
        locals.var_tv_dn7 = assign76950_e116677_d_n7;
        locals.var_tv_dn8 = assign76950_e116677_d_n8;
        locals.var_tv_dn9 = assign76950_e116677_d_n9;
        locals.var_tv_dn10 = assign76950_e116677_d_n10;
        locals.var_tv_dn11 = assign76950_e116677_d_n11;
        locals.var_tv_dn14 = assign76950_e116677_d_n14;

        let (assign76960_e116695, assign76960_e116695_d_n0, assign76960_e116695_d_n2, assign76960_e116695_d_n4, assign76960_e116695_d_n5, assign76960_e116695_d_n6, assign76960_e116695_d_n7, assign76960_e116695_d_n8, assign76960_e116695_d_n9, assign76960_e116695_d_n10, assign76960_e116695_d_n11, assign76960_e116695_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76960_e116687: f64 = (locals.var_tu + locals.var_tv);
        let assign76960_e116691: f64 = (3.0 * locals.var_ta);
        let assign76960_e116692: f64 = (locals.var_tb / assign76960_e116691);
        let assign76960_e116693: f64 = (assign76960_e116687 - assign76960_e116692);
        (assign76960_e116693, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn4 + locals.var_tv_dn4), (locals.var_tu_dn5 + locals.var_tv_dn5), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn8 + locals.var_tv_dn8), (locals.var_tu_dn9 + locals.var_tv_dn9), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn11 + locals.var_tv_dn11), (locals.var_tu_dn14 + locals.var_tv_dn14),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign76960_e116695;
        locals.var_chi_dn0 = assign76960_e116695_d_n0;
        locals.var_chi_dn2 = assign76960_e116695_d_n2;
        locals.var_chi_dn4 = assign76960_e116695_d_n4;
        locals.var_chi_dn5 = assign76960_e116695_d_n5;
        locals.var_chi_dn6 = assign76960_e116695_d_n6;
        locals.var_chi_dn7 = assign76960_e116695_d_n7;
        locals.var_chi_dn8 = assign76960_e116695_d_n8;
        locals.var_chi_dn9 = assign76960_e116695_d_n9;
        locals.var_chi_dn10 = assign76960_e116695_d_n10;
        locals.var_chi_dn11 = assign76960_e116695_d_n11;
        locals.var_chi_dn14 = assign76960_e116695_d_n14;

        let (assign76970_e116709, assign76970_e116709_d_n0, assign76970_e116709_d_n2, assign76970_e116709_d_n4, assign76970_e116709_d_n5, assign76970_e116709_d_n6, assign76970_e116709_d_n7, assign76970_e116709_d_n8, assign76970_e116709_d_n9, assign76970_e116709_d_n10, assign76970_e116709_d_n11, assign76970_e116709_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1793 == 0.0)) {
        let assign76970_e116705: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign76970_e116707: f64 = (assign76970_e116705 - locals.var_vxbgmtcl);
        (assign76970_e116707, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0_inia, locals.var_ps0_inia_dn0, locals.var_ps0_inia_dn2, locals.var_ps0_inia_dn4, locals.var_ps0_inia_dn5, locals.var_ps0_inia_dn6, locals.var_ps0_inia_dn7, locals.var_ps0_inia_dn8, locals.var_ps0_inia_dn9, locals.var_ps0_inia_dn10, locals.var_ps0_inia_dn11, locals.var_ps0_inia_dn14,)
    }
};
        locals.var_ps0_inia = assign76970_e116709;
        locals.var_ps0_inia_dn0 = assign76970_e116709_d_n0;
        locals.var_ps0_inia_dn2 = assign76970_e116709_d_n2;
        locals.var_ps0_inia_dn4 = assign76970_e116709_d_n4;
        locals.var_ps0_inia_dn5 = assign76970_e116709_d_n5;
        locals.var_ps0_inia_dn6 = assign76970_e116709_d_n6;
        locals.var_ps0_inia_dn7 = assign76970_e116709_d_n7;
        locals.var_ps0_inia_dn8 = assign76970_e116709_d_n8;
        locals.var_ps0_inia_dn9 = assign76970_e116709_d_n9;
        locals.var_ps0_inia_dn10 = assign76970_e116709_d_n10;
        locals.var_ps0_inia_dn11 = assign76970_e116709_d_n11;
        locals.var_ps0_inia_dn14 = assign76970_e116709_d_n14;

        let assign76980_e116712: f64 = if p.p33 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1794 = assign76980_e116712;

        let (assign76990_e116725, assign76990_e116725_d_n0, assign76990_e116725_d_n2, assign76990_e116725_d_n4, assign76990_e116725_d_n5, assign76990_e116725_d_n6, assign76990_e116725_d_n7, assign76990_e116725_d_n8, assign76990_e116725_d_n9, assign76990_e116725_d_n10, assign76990_e116725_d_n11, assign76990_e116725_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign76990_e116721: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign76990_e116723: f64 = (assign76990_e116721 + 0.1);
        (assign76990_e116723, locals.var_vxbgmtcl_dn0, (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), locals.var_vxbgmtcl_dn4, locals.var_vxbgmtcl_dn5, locals.var_vxbgmtcl_dn6, (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_vgpld_dn9 + locals.var_vxbgmtcl_dn9), locals.var_vxbgmtcl_dn10, locals.var_vxbgmtcl_dn11, locals.var_vxbgmtcl_dn14,)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn4, locals.var_vgpld_shift_dn5, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn8, locals.var_vgpld_shift_dn9, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn11, locals.var_vgpld_shift_dn14,)
    }
};
        locals.var_vgpld_shift = assign76990_e116725;
        locals.var_vgpld_shift_dn0 = assign76990_e116725_d_n0;
        locals.var_vgpld_shift_dn2 = assign76990_e116725_d_n2;
        locals.var_vgpld_shift_dn4 = assign76990_e116725_d_n4;
        locals.var_vgpld_shift_dn5 = assign76990_e116725_d_n5;
        locals.var_vgpld_shift_dn6 = assign76990_e116725_d_n6;
        locals.var_vgpld_shift_dn7 = assign76990_e116725_d_n7;
        locals.var_vgpld_shift_dn8 = assign76990_e116725_d_n8;
        locals.var_vgpld_shift_dn9 = assign76990_e116725_d_n9;
        locals.var_vgpld_shift_dn10 = assign76990_e116725_d_n10;
        locals.var_vgpld_shift_dn11 = assign76990_e116725_d_n11;
        locals.var_vgpld_shift_dn14 = assign76990_e116725_d_n14;

        let (assign77000_e116736, assign77000_e116736_d_n0, assign77000_e116736_d_n2, assign77000_e116736_d_n4, assign77000_e116736_d_n5, assign77000_e116736_d_n6, assign77000_e116736_d_n7, assign77000_e116736_d_n8, assign77000_e116736_d_n9, assign77000_e116736_d_n10, assign77000_e116736_d_n11, assign77000_e116736_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77000_e116734: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign77000_e116734, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn14,)
    }
};
        locals.var_cfs1 = assign77000_e116736;
        locals.var_cfs1_dn0 = assign77000_e116736_d_n0;
        locals.var_cfs1_dn2 = assign77000_e116736_d_n2;
        locals.var_cfs1_dn4 = assign77000_e116736_d_n4;
        locals.var_cfs1_dn5 = assign77000_e116736_d_n5;
        locals.var_cfs1_dn6 = assign77000_e116736_d_n6;
        locals.var_cfs1_dn7 = assign77000_e116736_d_n7;
        locals.var_cfs1_dn8 = assign77000_e116736_d_n8;
        locals.var_cfs1_dn9 = assign77000_e116736_d_n9;
        locals.var_cfs1_dn10 = assign77000_e116736_d_n10;
        locals.var_cfs1_dn11 = assign77000_e116736_d_n11;
        locals.var_cfs1_dn14 = assign77000_e116736_d_n14;

        let (assign77010_e116747, assign77010_e116747_d_n0, assign77010_e116747_d_n2, assign77010_e116747_d_n4, assign77010_e116747_d_n5, assign77010_e116747_d_n6, assign77010_e116747_d_n7, assign77010_e116747_d_n8, assign77010_e116747_d_n9, assign77010_e116747_d_n10, assign77010_e116747_d_n11, assign77010_e116747_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77010_e116745: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign77010_e116745, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn4, locals.var_gammachi_dn5, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn8, locals.var_gammachi_dn9, locals.var_gammachi_dn10, locals.var_gammachi_dn11, locals.var_gammachi_dn14,)
    }
};
        locals.var_gammachi = assign77010_e116747;
        locals.var_gammachi_dn0 = assign77010_e116747_d_n0;
        locals.var_gammachi_dn2 = assign77010_e116747_d_n2;
        locals.var_gammachi_dn4 = assign77010_e116747_d_n4;
        locals.var_gammachi_dn5 = assign77010_e116747_d_n5;
        locals.var_gammachi_dn6 = assign77010_e116747_d_n6;
        locals.var_gammachi_dn7 = assign77010_e116747_d_n7;
        locals.var_gammachi_dn8 = assign77010_e116747_d_n8;
        locals.var_gammachi_dn9 = assign77010_e116747_d_n9;
        locals.var_gammachi_dn10 = assign77010_e116747_d_n10;
        locals.var_gammachi_dn11 = assign77010_e116747_d_n11;
        locals.var_gammachi_dn14 = assign77010_e116747_d_n14;

        let (assign77020_e116758, assign77020_e116758_d_n0, assign77020_e116758_d_n2, assign77020_e116758_d_n4, assign77020_e116758_d_n5, assign77020_e116758_d_n6, assign77020_e116758_d_n7, assign77020_e116758_d_n8, assign77020_e116758_d_n9, assign77020_e116758_d_n10, assign77020_e116758_d_n11, assign77020_e116758_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77020_e116756: f64 = (locals.var_beta2 * locals.var_fac1p2);
        (assign77020_e116756, ((locals.var_beta2_dn0 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn0)), ((locals.var_beta2_dn2 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn2)), ((locals.var_beta2_dn4 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn4)), ((locals.var_beta2_dn5 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn5)), ((locals.var_beta2_dn6 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn6)), ((locals.var_beta2_dn7 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn7)), ((locals.var_beta2_dn8 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn8)), ((locals.var_beta2_dn9 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn9)), ((locals.var_beta2_dn10 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn10)), ((locals.var_beta2_dn11 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn11)), ((locals.var_beta2_dn14 * locals.var_fac1p2) + (locals.var_beta2 * locals.var_fac1p2_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign77020_e116758;
        locals.var_t0_dn0 = assign77020_e116758_d_n0;
        locals.var_t0_dn2 = assign77020_e116758_d_n2;
        locals.var_t0_dn4 = assign77020_e116758_d_n4;
        locals.var_t0_dn5 = assign77020_e116758_d_n5;
        locals.var_t0_dn6 = assign77020_e116758_d_n6;
        locals.var_t0_dn7 = assign77020_e116758_d_n7;
        locals.var_t0_dn8 = assign77020_e116758_d_n8;
        locals.var_t0_dn9 = assign77020_e116758_d_n9;
        locals.var_t0_dn10 = assign77020_e116758_d_n10;
        locals.var_t0_dn11 = assign77020_e116758_d_n11;
        locals.var_t0_dn14 = assign77020_e116758_d_n14;

        let (assign77030_e116769, assign77030_e116769_d_n0, assign77030_e116769_d_n2, assign77030_e116769_d_n4, assign77030_e116769_d_n5, assign77030_e116769_d_n6, assign77030_e116769_d_n7, assign77030_e116769_d_n8, assign77030_e116769_d_n9, assign77030_e116769_d_n10, assign77030_e116769_d_n11, assign77030_e116769_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77030_e116767: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign77030_e116767, ((locals.var_beta_dn0 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn0)), ((locals.var_beta_dn2 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn2)), ((locals.var_beta_dn4 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn4)), ((locals.var_beta_dn5 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn5)), ((locals.var_beta_dn6 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn6)), ((locals.var_beta_dn7 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn7)), ((locals.var_beta_dn8 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn8)), ((locals.var_beta_dn9 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn9)), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), ((locals.var_beta_dn11 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn11)), ((locals.var_beta_dn14 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn14)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign77030_e116769;
        locals.var_psi_dn0 = assign77030_e116769_d_n0;
        locals.var_psi_dn2 = assign77030_e116769_d_n2;
        locals.var_psi_dn4 = assign77030_e116769_d_n4;
        locals.var_psi_dn5 = assign77030_e116769_d_n5;
        locals.var_psi_dn6 = assign77030_e116769_d_n6;
        locals.var_psi_dn7 = assign77030_e116769_d_n7;
        locals.var_psi_dn8 = assign77030_e116769_d_n8;
        locals.var_psi_dn9 = assign77030_e116769_d_n9;
        locals.var_psi_dn10 = assign77030_e116769_d_n10;
        locals.var_psi_dn11 = assign77030_e116769_d_n11;
        locals.var_psi_dn14 = assign77030_e116769_d_n14;

        let (assign77040_e116794, assign77040_e116794_d_n0, assign77040_e116794_d_n2, assign77040_e116794_d_n4, assign77040_e116794_d_n5, assign77040_e116794_d_n6, assign77040_e116794_d_n7, assign77040_e116794_d_n8, assign77040_e116794_d_n9, assign77040_e116794_d_n10, assign77040_e116794_d_n11, assign77040_e116794_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77040_e116778: f64 = (locals.var_gammachi * locals.var_t0);
        let assign77040_e116781: f64 = (locals.var_psi * locals.var_psi);
        let assign77040_e116782: f64 = (assign77040_e116778 + assign77040_e116781);
        let assign77040_e116783: f64 = (assign77040_e116782).ln();
        let assign77040_e116786: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign77040_e116787: f64 = (assign77040_e116786).ln();
        let assign77040_e116788: f64 = (assign77040_e116783 - assign77040_e116787);
        let assign77040_e116791: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign77040_e116792: f64 = (assign77040_e116788 + assign77040_e116791);
        (assign77040_e116792, ((((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign77040_e116782) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign77040_e116786)) + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), ((((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign77040_e116782) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign77040_e116786)) + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), ((((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign77040_e116782) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign77040_e116786)) + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), ((((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign77040_e116782) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign77040_e116786)) + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), ((((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign77040_e116782) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign77040_e116786)) + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), ((((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign77040_e116782) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign77040_e116786)) + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), ((((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign77040_e116782) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign77040_e116786)) + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), ((((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign77040_e116782) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign77040_e116786)) + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), ((((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign77040_e116782) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign77040_e116786)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign77040_e116782) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign77040_e116786)) + ((locals.var_beta_dn11 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn11))), ((((((locals.var_gammachi_dn14 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn14)) + ((locals.var_psi_dn14 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn14))) / assign77040_e116782) - (((locals.var_cnst1over_dn14 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn14)) / assign77040_e116786)) + ((locals.var_beta_dn14 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign77040_e116794;
        locals.var_chi_1_dn0 = assign77040_e116794_d_n0;
        locals.var_chi_1_dn2 = assign77040_e116794_d_n2;
        locals.var_chi_1_dn4 = assign77040_e116794_d_n4;
        locals.var_chi_1_dn5 = assign77040_e116794_d_n5;
        locals.var_chi_1_dn6 = assign77040_e116794_d_n6;
        locals.var_chi_1_dn7 = assign77040_e116794_d_n7;
        locals.var_chi_1_dn8 = assign77040_e116794_d_n8;
        locals.var_chi_1_dn9 = assign77040_e116794_d_n9;
        locals.var_chi_1_dn10 = assign77040_e116794_d_n10;
        locals.var_chi_1_dn11 = assign77040_e116794_d_n11;
        locals.var_chi_1_dn14 = assign77040_e116794_d_n14;

        let assign77050_e116797: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1795 = assign77050_e116797;

        let (assign77060_e116812, assign77060_e116812_d_n0, assign77060_e116812_d_n2, assign77060_e116812_d_n4, assign77060_e116812_d_n5, assign77060_e116812_d_n6, assign77060_e116812_d_n7, assign77060_e116812_d_n8, assign77060_e116812_d_n9, assign77060_e116812_d_n10, assign77060_e116812_d_n11, assign77060_e116812_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77060_e116808: f64 = (locals.var_psi - locals.var_chi_1);
        let assign77060_e116810: f64 = (assign77060_e116808 - 1.0);
        (assign77060_e116810, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn14 - locals.var_chi_1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign77060_e116812;
        locals.var_tmf1_dn0 = assign77060_e116812_d_n0;
        locals.var_tmf1_dn2 = assign77060_e116812_d_n2;
        locals.var_tmf1_dn4 = assign77060_e116812_d_n4;
        locals.var_tmf1_dn5 = assign77060_e116812_d_n5;
        locals.var_tmf1_dn6 = assign77060_e116812_d_n6;
        locals.var_tmf1_dn7 = assign77060_e116812_d_n7;
        locals.var_tmf1_dn8 = assign77060_e116812_d_n8;
        locals.var_tmf1_dn9 = assign77060_e116812_d_n9;
        locals.var_tmf1_dn10 = assign77060_e116812_d_n10;
        locals.var_tmf1_dn11 = assign77060_e116812_d_n11;
        locals.var_tmf1_dn14 = assign77060_e116812_d_n14;

        let (assign77070_e116827, assign77070_e116827_d_n0, assign77070_e116827_d_n2, assign77070_e116827_d_n4, assign77070_e116827_d_n5, assign77070_e116827_d_n6, assign77070_e116827_d_n7, assign77070_e116827_d_n8, assign77070_e116827_d_n9, assign77070_e116827_d_n10, assign77070_e116827_d_n11, assign77070_e116827_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77070_e116823: f64 = (4.0 * locals.var_psi);
        let assign77070_e116825: f64 = assign77070_e116823;
        (assign77070_e116825, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn4), (4.0 * locals.var_psi_dn5), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn8), (4.0 * locals.var_psi_dn9), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn11), (4.0 * locals.var_psi_dn14),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign77070_e116827;
        locals.var_tmf2_dn0 = assign77070_e116827_d_n0;
        locals.var_tmf2_dn2 = assign77070_e116827_d_n2;
        locals.var_tmf2_dn4 = assign77070_e116827_d_n4;
        locals.var_tmf2_dn5 = assign77070_e116827_d_n5;
        locals.var_tmf2_dn6 = assign77070_e116827_d_n6;
        locals.var_tmf2_dn7 = assign77070_e116827_d_n7;
        locals.var_tmf2_dn8 = assign77070_e116827_d_n8;
        locals.var_tmf2_dn9 = assign77070_e116827_d_n9;
        locals.var_tmf2_dn10 = assign77070_e116827_d_n10;
        locals.var_tmf2_dn11 = assign77070_e116827_d_n11;
        locals.var_tmf2_dn14 = assign77070_e116827_d_n14;

        let (assign77080_e116844, assign77080_e116844_d_n0, assign77080_e116844_d_n2, assign77080_e116844_d_n4, assign77080_e116844_d_n5, assign77080_e116844_d_n6, assign77080_e116844_d_n7, assign77080_e116844_d_n8, assign77080_e116844_d_n9, assign77080_e116844_d_n10, assign77080_e116844_d_n11, assign77080_e116844_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let (assign77080_e116842, assign77080_e116842_d_n0, assign77080_e116842_d_n2, assign77080_e116842_d_n4, assign77080_e116842_d_n5, assign77080_e116842_d_n6, assign77080_e116842_d_n7, assign77080_e116842_d_n8, assign77080_e116842_d_n9, assign77080_e116842_d_n10, assign77080_e116842_d_n11, assign77080_e116842_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign77080_e116841: f64 = (-locals.var_tmf2);
                (assign77080_e116841, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign77080_e116842, assign77080_e116842_d_n0, assign77080_e116842_d_n2, assign77080_e116842_d_n4, assign77080_e116842_d_n5, assign77080_e116842_d_n6, assign77080_e116842_d_n7, assign77080_e116842_d_n8, assign77080_e116842_d_n9, assign77080_e116842_d_n10, assign77080_e116842_d_n11, assign77080_e116842_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign77080_e116844;
        locals.var_tmf2_dn0 = assign77080_e116844_d_n0;
        locals.var_tmf2_dn2 = assign77080_e116844_d_n2;
        locals.var_tmf2_dn4 = assign77080_e116844_d_n4;
        locals.var_tmf2_dn5 = assign77080_e116844_d_n5;
        locals.var_tmf2_dn6 = assign77080_e116844_d_n6;
        locals.var_tmf2_dn7 = assign77080_e116844_d_n7;
        locals.var_tmf2_dn8 = assign77080_e116844_d_n8;
        locals.var_tmf2_dn9 = assign77080_e116844_d_n9;
        locals.var_tmf2_dn10 = assign77080_e116844_d_n10;
        locals.var_tmf2_dn11 = assign77080_e116844_d_n11;
        locals.var_tmf2_dn14 = assign77080_e116844_d_n14;

    }

    pub(super) fn stamp_transient_block_278(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign77090_e116860, assign77090_e116860_d_n0, assign77090_e116860_d_n2, assign77090_e116860_d_n4, assign77090_e116860_d_n5, assign77090_e116860_d_n6, assign77090_e116860_d_n7, assign77090_e116860_d_n8, assign77090_e116860_d_n9, assign77090_e116860_d_n10, assign77090_e116860_d_n11, assign77090_e116860_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77090_e116855: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign77090_e116857: f64 = (assign77090_e116855 + locals.var_tmf2);
        let assign77090_e116858: f64 = (assign77090_e116857).sqrt();
        (assign77090_e116858, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign77090_e116858)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign77090_e116858)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign77090_e116860;
        locals.var_tmf2_dn0 = assign77090_e116860_d_n0;
        locals.var_tmf2_dn2 = assign77090_e116860_d_n2;
        locals.var_tmf2_dn4 = assign77090_e116860_d_n4;
        locals.var_tmf2_dn5 = assign77090_e116860_d_n5;
        locals.var_tmf2_dn6 = assign77090_e116860_d_n6;
        locals.var_tmf2_dn7 = assign77090_e116860_d_n7;
        locals.var_tmf2_dn8 = assign77090_e116860_d_n8;
        locals.var_tmf2_dn9 = assign77090_e116860_d_n9;
        locals.var_tmf2_dn10 = assign77090_e116860_d_n10;
        locals.var_tmf2_dn11 = assign77090_e116860_d_n11;
        locals.var_tmf2_dn14 = assign77090_e116860_d_n14;

        let (assign77100_e116877, assign77100_e116877_d_n0, assign77100_e116877_d_n2, assign77100_e116877_d_n4, assign77100_e116877_d_n5, assign77100_e116877_d_n6, assign77100_e116877_d_n7, assign77100_e116877_d_n8, assign77100_e116877_d_n9, assign77100_e116877_d_n10, assign77100_e116877_d_n11, assign77100_e116877_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77100_e116873: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign77100_e116874: f64 = (1.0 + assign77100_e116873);
        let assign77100_e116875: f64 = (0.5 * assign77100_e116874);
        (assign77100_e116875, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77100_e116877;
        locals.var_t1_dn0 = assign77100_e116877_d_n0;
        locals.var_t1_dn2 = assign77100_e116877_d_n2;
        locals.var_t1_dn4 = assign77100_e116877_d_n4;
        locals.var_t1_dn5 = assign77100_e116877_d_n5;
        locals.var_t1_dn6 = assign77100_e116877_d_n6;
        locals.var_t1_dn7 = assign77100_e116877_d_n7;
        locals.var_t1_dn8 = assign77100_e116877_d_n8;
        locals.var_t1_dn9 = assign77100_e116877_d_n9;
        locals.var_t1_dn10 = assign77100_e116877_d_n10;
        locals.var_t1_dn11 = assign77100_e116877_d_n11;
        locals.var_t1_dn14 = assign77100_e116877_d_n14;

        let (assign77110_e116894, assign77110_e116894_d_n0, assign77110_e116894_d_n2, assign77110_e116894_d_n4, assign77110_e116894_d_n5, assign77110_e116894_d_n6, assign77110_e116894_d_n7, assign77110_e116894_d_n8, assign77110_e116894_d_n9, assign77110_e116894_d_n10, assign77110_e116894_d_n11, assign77110_e116894_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 != 0.0)) {
        let assign77110_e116890: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign77110_e116891: f64 = (0.5 * assign77110_e116890);
        let assign77110_e116892: f64 = (locals.var_psi - assign77110_e116891);
        (assign77110_e116892, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_psi_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_psi_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign77110_e116894;
        locals.var_chi_1_dn0 = assign77110_e116894_d_n0;
        locals.var_chi_1_dn2 = assign77110_e116894_d_n2;
        locals.var_chi_1_dn4 = assign77110_e116894_d_n4;
        locals.var_chi_1_dn5 = assign77110_e116894_d_n5;
        locals.var_chi_1_dn6 = assign77110_e116894_d_n6;
        locals.var_chi_1_dn7 = assign77110_e116894_d_n7;
        locals.var_chi_1_dn8 = assign77110_e116894_d_n8;
        locals.var_chi_1_dn9 = assign77110_e116894_d_n9;
        locals.var_chi_1_dn10 = assign77110_e116894_d_n10;
        locals.var_chi_1_dn11 = assign77110_e116894_d_n11;
        locals.var_chi_1_dn14 = assign77110_e116894_d_n14;

        let (assign77120_e116911, assign77120_e116911_d_n0, assign77120_e116911_d_n2, assign77120_e116911_d_n4, assign77120_e116911_d_n5, assign77120_e116911_d_n6, assign77120_e116911_d_n7, assign77120_e116911_d_n8, assign77120_e116911_d_n9, assign77120_e116911_d_n10, assign77120_e116911_d_n11, assign77120_e116911_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1795 == 0.0)) {
        let (assign77120_e116909, assign77120_e116909_d_n0, assign77120_e116909_d_n2, assign77120_e116909_d_n4, assign77120_e116909_d_n5, assign77120_e116909_d_n6, assign77120_e116909_d_n7, assign77120_e116909_d_n8, assign77120_e116909_d_n9, assign77120_e116909_d_n10, assign77120_e116909_d_n11, assign77120_e116909_d_n14,) = {
            if (locals.var_chi_1 <= locals.var_psi) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
            } else {
                (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
            }
        };
        (assign77120_e116909, assign77120_e116909_d_n0, assign77120_e116909_d_n2, assign77120_e116909_d_n4, assign77120_e116909_d_n5, assign77120_e116909_d_n6, assign77120_e116909_d_n7, assign77120_e116909_d_n8, assign77120_e116909_d_n9, assign77120_e116909_d_n10, assign77120_e116909_d_n11, assign77120_e116909_d_n14,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign77120_e116911;
        locals.var_chi_1_dn0 = assign77120_e116911_d_n0;
        locals.var_chi_1_dn2 = assign77120_e116911_d_n2;
        locals.var_chi_1_dn4 = assign77120_e116911_d_n4;
        locals.var_chi_1_dn5 = assign77120_e116911_d_n5;
        locals.var_chi_1_dn6 = assign77120_e116911_d_n6;
        locals.var_chi_1_dn7 = assign77120_e116911_d_n7;
        locals.var_chi_1_dn8 = assign77120_e116911_d_n8;
        locals.var_chi_1_dn9 = assign77120_e116911_d_n9;
        locals.var_chi_1_dn10 = assign77120_e116911_d_n10;
        locals.var_chi_1_dn11 = assign77120_e116911_d_n11;
        locals.var_chi_1_dn14 = assign77120_e116911_d_n14;

        let (assign77130_e116925, assign77130_e116925_d_n0, assign77130_e116925_d_n2, assign77130_e116925_d_n4, assign77130_e116925_d_n5, assign77130_e116925_d_n6, assign77130_e116925_d_n7, assign77130_e116925_d_n8, assign77130_e116925_d_n9, assign77130_e116925_d_n10, assign77130_e116925_d_n11, assign77130_e116925_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let (assign77130_e116923, assign77130_e116923_d_n0, assign77130_e116923_d_n2, assign77130_e116923_d_n4, assign77130_e116923_d_n5, assign77130_e116923_d_n6, assign77130_e116923_d_n7, assign77130_e116923_d_n8, assign77130_e116923_d_n9, assign77130_e116923_d_n10, assign77130_e116923_d_n11, assign77130_e116923_d_n14,) = {
            if (locals.var_chi_1 >= 0.0) {
                (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign77130_e116923, assign77130_e116923_d_n0, assign77130_e116923_d_n2, assign77130_e116923_d_n4, assign77130_e116923_d_n5, assign77130_e116923_d_n6, assign77130_e116923_d_n7, assign77130_e116923_d_n8, assign77130_e116923_d_n9, assign77130_e116923_d_n10, assign77130_e116923_d_n11, assign77130_e116923_d_n14,)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    }
};
        locals.var_chi_1 = assign77130_e116925;
        locals.var_chi_1_dn0 = assign77130_e116925_d_n0;
        locals.var_chi_1_dn2 = assign77130_e116925_d_n2;
        locals.var_chi_1_dn4 = assign77130_e116925_d_n4;
        locals.var_chi_1_dn5 = assign77130_e116925_d_n5;
        locals.var_chi_1_dn6 = assign77130_e116925_d_n6;
        locals.var_chi_1_dn7 = assign77130_e116925_d_n7;
        locals.var_chi_1_dn8 = assign77130_e116925_d_n8;
        locals.var_chi_1_dn9 = assign77130_e116925_d_n9;
        locals.var_chi_1_dn10 = assign77130_e116925_d_n10;
        locals.var_chi_1_dn11 = assign77130_e116925_d_n11;
        locals.var_chi_1_dn14 = assign77130_e116925_d_n14;

        let (assign77140_e116936, assign77140_e116936_d_n0, assign77140_e116936_d_n2, assign77140_e116936_d_n4, assign77140_e116936_d_n5, assign77140_e116936_d_n6, assign77140_e116936_d_n7, assign77140_e116936_d_n8, assign77140_e116936_d_n9, assign77140_e116936_d_n10, assign77140_e116936_d_n11, assign77140_e116936_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77140_e116934: f64 = (locals.var_psi - locals.var_chi_1);
        (assign77140_e116934, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn4 - locals.var_chi_1_dn4), (locals.var_psi_dn5 - locals.var_chi_1_dn5), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn8 - locals.var_chi_1_dn8), (locals.var_psi_dn9 - locals.var_chi_1_dn9), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn14 - locals.var_chi_1_dn14),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign77140_e116936;
        locals.var_psi_dn0 = assign77140_e116936_d_n0;
        locals.var_psi_dn2 = assign77140_e116936_d_n2;
        locals.var_psi_dn4 = assign77140_e116936_d_n4;
        locals.var_psi_dn5 = assign77140_e116936_d_n5;
        locals.var_psi_dn6 = assign77140_e116936_d_n6;
        locals.var_psi_dn7 = assign77140_e116936_d_n7;
        locals.var_psi_dn8 = assign77140_e116936_d_n8;
        locals.var_psi_dn9 = assign77140_e116936_d_n9;
        locals.var_psi_dn10 = assign77140_e116936_d_n10;
        locals.var_psi_dn11 = assign77140_e116936_d_n11;
        locals.var_psi_dn14 = assign77140_e116936_d_n14;

        let (assign77150_e116949, assign77150_e116949_d_n0, assign77150_e116949_d_n2, assign77150_e116949_d_n4, assign77150_e116949_d_n5, assign77150_e116949_d_n6, assign77150_e116949_d_n7, assign77150_e116949_d_n8, assign77150_e116949_d_n9, assign77150_e116949_d_n10, assign77150_e116949_d_n11, assign77150_e116949_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77150_e116946: f64 = (locals.var_beta * 0.1);
        let assign77150_e116947: f64 = (locals.var_psi + assign77150_e116946);
        (assign77150_e116947, (locals.var_psi_dn0 + (locals.var_beta_dn0 * 0.1)), (locals.var_psi_dn2 + (locals.var_beta_dn2 * 0.1)), (locals.var_psi_dn4 + (locals.var_beta_dn4 * 0.1)), (locals.var_psi_dn5 + (locals.var_beta_dn5 * 0.1)), (locals.var_psi_dn6 + (locals.var_beta_dn6 * 0.1)), (locals.var_psi_dn7 + (locals.var_beta_dn7 * 0.1)), (locals.var_psi_dn8 + (locals.var_beta_dn8 * 0.1)), (locals.var_psi_dn9 + (locals.var_beta_dn9 * 0.1)), (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), (locals.var_psi_dn11 + (locals.var_beta_dn11 * 0.1)), (locals.var_psi_dn14 + (locals.var_beta_dn14 * 0.1)),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn8, locals.var_psi_dn9, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn14,)
    }
};
        locals.var_psi = assign77150_e116949;
        locals.var_psi_dn0 = assign77150_e116949_d_n0;
        locals.var_psi_dn2 = assign77150_e116949_d_n2;
        locals.var_psi_dn4 = assign77150_e116949_d_n4;
        locals.var_psi_dn5 = assign77150_e116949_d_n5;
        locals.var_psi_dn6 = assign77150_e116949_d_n6;
        locals.var_psi_dn7 = assign77150_e116949_d_n7;
        locals.var_psi_dn8 = assign77150_e116949_d_n8;
        locals.var_psi_dn9 = assign77150_e116949_d_n9;
        locals.var_psi_dn10 = assign77150_e116949_d_n10;
        locals.var_psi_dn11 = assign77150_e116949_d_n11;
        locals.var_psi_dn14 = assign77150_e116949_d_n14;

        let (assign77160_e116970, assign77160_e116970_d_n0, assign77160_e116970_d_n2, assign77160_e116970_d_n4, assign77160_e116970_d_n5, assign77160_e116970_d_n6, assign77160_e116970_d_n7, assign77160_e116970_d_n8, assign77160_e116970_d_n9, assign77160_e116970_d_n10, assign77160_e116970_d_n11, assign77160_e116970_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77160_e116958: f64 = (locals.var_gammachi * locals.var_t0);
        let assign77160_e116961: f64 = (locals.var_psi * locals.var_psi);
        let assign77160_e116962: f64 = (assign77160_e116958 + assign77160_e116961);
        let assign77160_e116963: f64 = (assign77160_e116962).ln();
        let assign77160_e116966: f64 = (locals.var_cnst1over * locals.var_t0);
        let assign77160_e116967: f64 = (assign77160_e116966).ln();
        let assign77160_e116968: f64 = (assign77160_e116963 - assign77160_e116967);
        (assign77160_e116968, (((((locals.var_gammachi_dn0 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign77160_e116962) - (((locals.var_cnst1over_dn0 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn0)) / assign77160_e116966)), (((((locals.var_gammachi_dn2 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign77160_e116962) - (((locals.var_cnst1over_dn2 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn2)) / assign77160_e116966)), (((((locals.var_gammachi_dn4 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn4)) + ((locals.var_psi_dn4 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn4))) / assign77160_e116962) - (((locals.var_cnst1over_dn4 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn4)) / assign77160_e116966)), (((((locals.var_gammachi_dn5 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn5)) + ((locals.var_psi_dn5 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn5))) / assign77160_e116962) - (((locals.var_cnst1over_dn5 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn5)) / assign77160_e116966)), (((((locals.var_gammachi_dn6 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign77160_e116962) - (((locals.var_cnst1over_dn6 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn6)) / assign77160_e116966)), (((((locals.var_gammachi_dn7 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign77160_e116962) - (((locals.var_cnst1over_dn7 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn7)) / assign77160_e116966)), (((((locals.var_gammachi_dn8 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn8)) + ((locals.var_psi_dn8 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn8))) / assign77160_e116962) - (((locals.var_cnst1over_dn8 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn8)) / assign77160_e116966)), (((((locals.var_gammachi_dn9 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn9)) + ((locals.var_psi_dn9 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn9))) / assign77160_e116962) - (((locals.var_cnst1over_dn9 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn9)) / assign77160_e116966)), (((((locals.var_gammachi_dn10 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign77160_e116962) - (((locals.var_cnst1over_dn10 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn10)) / assign77160_e116966)), (((((locals.var_gammachi_dn11 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign77160_e116962) - (((locals.var_cnst1over_dn11 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn11)) / assign77160_e116966)), (((((locals.var_gammachi_dn14 * locals.var_t0) + (locals.var_gammachi * locals.var_t0_dn14)) + ((locals.var_psi_dn14 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn14))) / assign77160_e116962) - (((locals.var_cnst1over_dn14 * locals.var_t0) + (locals.var_cnst1over * locals.var_t0_dn14)) / assign77160_e116966)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77160_e116970;
        locals.var_t1_dn0 = assign77160_e116970_d_n0;
        locals.var_t1_dn2 = assign77160_e116970_d_n2;
        locals.var_t1_dn4 = assign77160_e116970_d_n4;
        locals.var_t1_dn5 = assign77160_e116970_d_n5;
        locals.var_t1_dn6 = assign77160_e116970_d_n6;
        locals.var_t1_dn7 = assign77160_e116970_d_n7;
        locals.var_t1_dn8 = assign77160_e116970_d_n8;
        locals.var_t1_dn9 = assign77160_e116970_d_n9;
        locals.var_t1_dn10 = assign77160_e116970_d_n10;
        locals.var_t1_dn11 = assign77160_e116970_d_n11;
        locals.var_t1_dn14 = assign77160_e116970_d_n14;

        let (assign77170_e116983, assign77170_e116983_d_n0, assign77170_e116983_d_n2, assign77170_e116983_d_n4, assign77170_e116983_d_n5, assign77170_e116983_d_n6, assign77170_e116983_d_n7, assign77170_e116983_d_n8, assign77170_e116983_d_n9, assign77170_e116983_d_n10, assign77170_e116983_d_n11, assign77170_e116983_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let assign77170_e116980: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign77170_e116981: f64 = (locals.var_t1 + assign77170_e116980);
        (assign77170_e116981, (locals.var_t1_dn0 + ((locals.var_beta_dn0 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn0))), (locals.var_t1_dn2 + ((locals.var_beta_dn2 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn2))), (locals.var_t1_dn4 + ((locals.var_beta_dn4 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn4))), (locals.var_t1_dn5 + ((locals.var_beta_dn5 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn5))), (locals.var_t1_dn6 + ((locals.var_beta_dn6 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn6))), (locals.var_t1_dn7 + ((locals.var_beta_dn7 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn7))), (locals.var_t1_dn8 + ((locals.var_beta_dn8 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn8))), (locals.var_t1_dn9 + ((locals.var_beta_dn9 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn9))), (locals.var_t1_dn10 + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), (locals.var_t1_dn11 + ((locals.var_beta_dn11 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn11))), (locals.var_t1_dn14 + ((locals.var_beta_dn14 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn14))),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign77170_e116983;
        locals.var_chi_b_dn0 = assign77170_e116983_d_n0;
        locals.var_chi_b_dn2 = assign77170_e116983_d_n2;
        locals.var_chi_b_dn4 = assign77170_e116983_d_n4;
        locals.var_chi_b_dn5 = assign77170_e116983_d_n5;
        locals.var_chi_b_dn6 = assign77170_e116983_d_n6;
        locals.var_chi_b_dn7 = assign77170_e116983_d_n7;
        locals.var_chi_b_dn8 = assign77170_e116983_d_n8;
        locals.var_chi_b_dn9 = assign77170_e116983_d_n9;
        locals.var_chi_b_dn10 = assign77170_e116983_d_n10;
        locals.var_chi_b_dn11 = assign77170_e116983_d_n11;
        locals.var_chi_b_dn14 = assign77170_e116983_d_n14;

        let (assign77180_e116997, assign77180_e116997_d_n0, assign77180_e116997_d_n2, assign77180_e116997_d_n4, assign77180_e116997_d_n5, assign77180_e116997_d_n6, assign77180_e116997_d_n7, assign77180_e116997_d_n8, assign77180_e116997_d_n9, assign77180_e116997_d_n10, assign77180_e116997_d_n11, assign77180_e116997_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        let (assign77180_e116995, assign77180_e116995_d_n0, assign77180_e116995_d_n2, assign77180_e116995_d_n4, assign77180_e116995_d_n5, assign77180_e116995_d_n6, assign77180_e116995_d_n7, assign77180_e116995_d_n8, assign77180_e116995_d_n9, assign77180_e116995_d_n10, assign77180_e116995_d_n11, assign77180_e116995_d_n14,) = {
            if (locals.var_chi_b >= 0.0) {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign77180_e116995, assign77180_e116995_d_n0, assign77180_e116995_d_n2, assign77180_e116995_d_n4, assign77180_e116995_d_n5, assign77180_e116995_d_n6, assign77180_e116995_d_n7, assign77180_e116995_d_n8, assign77180_e116995_d_n9, assign77180_e116995_d_n10, assign77180_e116995_d_n11, assign77180_e116995_d_n14,)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
    }
};
        locals.var_chi_b = assign77180_e116997;
        locals.var_chi_b_dn0 = assign77180_e116997_d_n0;
        locals.var_chi_b_dn2 = assign77180_e116997_d_n2;
        locals.var_chi_b_dn4 = assign77180_e116997_d_n4;
        locals.var_chi_b_dn5 = assign77180_e116997_d_n5;
        locals.var_chi_b_dn6 = assign77180_e116997_d_n6;
        locals.var_chi_b_dn7 = assign77180_e116997_d_n7;
        locals.var_chi_b_dn8 = assign77180_e116997_d_n8;
        locals.var_chi_b_dn9 = assign77180_e116997_d_n9;
        locals.var_chi_b_dn10 = assign77180_e116997_d_n10;
        locals.var_chi_b_dn11 = assign77180_e116997_d_n11;
        locals.var_chi_b_dn14 = assign77180_e116997_d_n14;

        let (assign77190_e117006, assign77190_e117006_d_n0, assign77190_e117006_d_n2, assign77190_e117006_d_n4, assign77190_e117006_d_n5, assign77190_e117006_d_n6, assign77190_e117006_d_n7, assign77190_e117006_d_n8, assign77190_e117006_d_n9, assign77190_e117006_d_n10, assign77190_e117006_d_n11, assign77190_e117006_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    }
};
        locals.var_chi_a = assign77190_e117006;
        locals.var_chi_a_dn0 = assign77190_e117006_d_n0;
        locals.var_chi_a_dn2 = assign77190_e117006_d_n2;
        locals.var_chi_a_dn4 = assign77190_e117006_d_n4;
        locals.var_chi_a_dn5 = assign77190_e117006_d_n5;
        locals.var_chi_a_dn6 = assign77190_e117006_d_n6;
        locals.var_chi_a_dn7 = assign77190_e117006_d_n7;
        locals.var_chi_a_dn8 = assign77190_e117006_d_n8;
        locals.var_chi_a_dn9 = assign77190_e117006_d_n9;
        locals.var_chi_a_dn10 = assign77190_e117006_d_n10;
        locals.var_chi_a_dn11 = assign77190_e117006_d_n11;
        locals.var_chi_a_dn14 = assign77190_e117006_d_n14;

        let assign77200_e117009: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1796 = assign77200_e117009;

        let assign77210_e117014: f64 = (0.2 * locals.var_chi_b);
        let assign77210_e117015: f64 = (locals.var_chi_b - assign77210_e117014);
        let assign77210_e117019: f64 = (0.2 * locals.var_chi_b);
        let assign77210_e117022: f64 = if ((locals.var_chi_a > assign77210_e117015) && (assign77210_e117019 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1797 = assign77210_e117022;

        let (assign77220_e117041, assign77220_e117041_d_n0, assign77220_e117041_d_n2, assign77220_e117041_d_n4, assign77220_e117041_d_n5, assign77220_e117041_d_n6, assign77220_e117041_d_n7, assign77220_e117041_d_n8, assign77220_e117041_d_n9, assign77220_e117041_d_n10, assign77220_e117041_d_n11, assign77220_e117041_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77220_e117035: f64 = (locals.var_chi_a - locals.var_chi_b);
        let assign77220_e117038: f64 = (0.2 * locals.var_chi_b);
        let assign77220_e117039: f64 = (assign77220_e117035 + assign77220_e117038);
        (assign77220_e117039, ((locals.var_chi_a_dn0 - locals.var_chi_b_dn0) + (0.2 * locals.var_chi_b_dn0)), ((locals.var_chi_a_dn2 - locals.var_chi_b_dn2) + (0.2 * locals.var_chi_b_dn2)), ((locals.var_chi_a_dn4 - locals.var_chi_b_dn4) + (0.2 * locals.var_chi_b_dn4)), ((locals.var_chi_a_dn5 - locals.var_chi_b_dn5) + (0.2 * locals.var_chi_b_dn5)), ((locals.var_chi_a_dn6 - locals.var_chi_b_dn6) + (0.2 * locals.var_chi_b_dn6)), ((locals.var_chi_a_dn7 - locals.var_chi_b_dn7) + (0.2 * locals.var_chi_b_dn7)), ((locals.var_chi_a_dn8 - locals.var_chi_b_dn8) + (0.2 * locals.var_chi_b_dn8)), ((locals.var_chi_a_dn9 - locals.var_chi_b_dn9) + (0.2 * locals.var_chi_b_dn9)), ((locals.var_chi_a_dn10 - locals.var_chi_b_dn10) + (0.2 * locals.var_chi_b_dn10)), ((locals.var_chi_a_dn11 - locals.var_chi_b_dn11) + (0.2 * locals.var_chi_b_dn11)), ((locals.var_chi_a_dn14 - locals.var_chi_b_dn14) + (0.2 * locals.var_chi_b_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign77220_e117041;
        locals.var_tmf1_dn0 = assign77220_e117041_d_n0;
        locals.var_tmf1_dn2 = assign77220_e117041_d_n2;
        locals.var_tmf1_dn4 = assign77220_e117041_d_n4;
        locals.var_tmf1_dn5 = assign77220_e117041_d_n5;
        locals.var_tmf1_dn6 = assign77220_e117041_d_n6;
        locals.var_tmf1_dn7 = assign77220_e117041_d_n7;
        locals.var_tmf1_dn8 = assign77220_e117041_d_n8;
        locals.var_tmf1_dn9 = assign77220_e117041_d_n9;
        locals.var_tmf1_dn10 = assign77220_e117041_d_n10;
        locals.var_tmf1_dn11 = assign77220_e117041_d_n11;
        locals.var_tmf1_dn14 = assign77220_e117041_d_n14;

        let (assign77230_e117056, assign77230_e117056_d_n0, assign77230_e117056_d_n2, assign77230_e117056_d_n4, assign77230_e117056_d_n5, assign77230_e117056_d_n6, assign77230_e117056_d_n7, assign77230_e117056_d_n8, assign77230_e117056_d_n9, assign77230_e117056_d_n10, assign77230_e117056_d_n11, assign77230_e117056_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77230_e117054: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign77230_e117054, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign77230_e117056;
        locals.var_x2_dn0 = assign77230_e117056_d_n0;
        locals.var_x2_dn2 = assign77230_e117056_d_n2;
        locals.var_x2_dn4 = assign77230_e117056_d_n4;
        locals.var_x2_dn5 = assign77230_e117056_d_n5;
        locals.var_x2_dn6 = assign77230_e117056_d_n6;
        locals.var_x2_dn7 = assign77230_e117056_d_n7;
        locals.var_x2_dn8 = assign77230_e117056_d_n8;
        locals.var_x2_dn9 = assign77230_e117056_d_n9;
        locals.var_x2_dn10 = assign77230_e117056_d_n10;
        locals.var_x2_dn11 = assign77230_e117056_d_n11;
        locals.var_x2_dn14 = assign77230_e117056_d_n14;

        let (assign77240_e117075, assign77240_e117075_d_n0, assign77240_e117075_d_n2, assign77240_e117075_d_n4, assign77240_e117075_d_n5, assign77240_e117075_d_n6, assign77240_e117075_d_n7, assign77240_e117075_d_n8, assign77240_e117075_d_n9, assign77240_e117075_d_n10, assign77240_e117075_d_n11, assign77240_e117075_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77240_e117069: f64 = (0.2 * locals.var_chi_b);
        let assign77240_e117072: f64 = (0.2 * locals.var_chi_b);
        let assign77240_e117073: f64 = (assign77240_e117069 * assign77240_e117072);
        (assign77240_e117073, (((0.2 * locals.var_chi_b_dn0) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn0))), (((0.2 * locals.var_chi_b_dn2) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn2))), (((0.2 * locals.var_chi_b_dn4) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn4))), (((0.2 * locals.var_chi_b_dn5) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn5))), (((0.2 * locals.var_chi_b_dn6) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn6))), (((0.2 * locals.var_chi_b_dn7) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn7))), (((0.2 * locals.var_chi_b_dn8) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn8))), (((0.2 * locals.var_chi_b_dn9) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn9))), (((0.2 * locals.var_chi_b_dn10) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn10))), (((0.2 * locals.var_chi_b_dn11) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn11))), (((0.2 * locals.var_chi_b_dn14) * assign77240_e117072) + (assign77240_e117069 * (0.2 * locals.var_chi_b_dn14))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign77240_e117075;
        locals.var_xmax2_dn0 = assign77240_e117075_d_n0;
        locals.var_xmax2_dn2 = assign77240_e117075_d_n2;
        locals.var_xmax2_dn4 = assign77240_e117075_d_n4;
        locals.var_xmax2_dn5 = assign77240_e117075_d_n5;
        locals.var_xmax2_dn6 = assign77240_e117075_d_n6;
        locals.var_xmax2_dn7 = assign77240_e117075_d_n7;
        locals.var_xmax2_dn8 = assign77240_e117075_d_n8;
        locals.var_xmax2_dn9 = assign77240_e117075_d_n9;
        locals.var_xmax2_dn10 = assign77240_e117075_d_n10;
        locals.var_xmax2_dn11 = assign77240_e117075_d_n11;
        locals.var_xmax2_dn14 = assign77240_e117075_d_n14;

        let (assign77250_e117088, assign77250_e117088_d_n0, assign77250_e117088_d_n2, assign77250_e117088_d_n4, assign77250_e117088_d_n5, assign77250_e117088_d_n6, assign77250_e117088_d_n7, assign77250_e117088_d_n8, assign77250_e117088_d_n9, assign77250_e117088_d_n10, assign77250_e117088_d_n11, assign77250_e117088_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign77250_e117088;
        locals.var_xp_dn0 = assign77250_e117088_d_n0;
        locals.var_xp_dn2 = assign77250_e117088_d_n2;
        locals.var_xp_dn4 = assign77250_e117088_d_n4;
        locals.var_xp_dn5 = assign77250_e117088_d_n5;
        locals.var_xp_dn6 = assign77250_e117088_d_n6;
        locals.var_xp_dn7 = assign77250_e117088_d_n7;
        locals.var_xp_dn8 = assign77250_e117088_d_n8;
        locals.var_xp_dn9 = assign77250_e117088_d_n9;
        locals.var_xp_dn10 = assign77250_e117088_d_n10;
        locals.var_xp_dn11 = assign77250_e117088_d_n11;
        locals.var_xp_dn14 = assign77250_e117088_d_n14;

        let (assign77260_e117101, assign77260_e117101_d_n0, assign77260_e117101_d_n2, assign77260_e117101_d_n4, assign77260_e117101_d_n5, assign77260_e117101_d_n6, assign77260_e117101_d_n7, assign77260_e117101_d_n8, assign77260_e117101_d_n9, assign77260_e117101_d_n10, assign77260_e117101_d_n11, assign77260_e117101_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign77260_e117101;
        locals.var_xmp_dn0 = assign77260_e117101_d_n0;
        locals.var_xmp_dn2 = assign77260_e117101_d_n2;
        locals.var_xmp_dn4 = assign77260_e117101_d_n4;
        locals.var_xmp_dn5 = assign77260_e117101_d_n5;
        locals.var_xmp_dn6 = assign77260_e117101_d_n6;
        locals.var_xmp_dn7 = assign77260_e117101_d_n7;
        locals.var_xmp_dn8 = assign77260_e117101_d_n8;
        locals.var_xmp_dn9 = assign77260_e117101_d_n9;
        locals.var_xmp_dn10 = assign77260_e117101_d_n10;
        locals.var_xmp_dn11 = assign77260_e117101_d_n11;
        locals.var_xmp_dn14 = assign77260_e117101_d_n14;

        let (assign77270_e117114,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign77270_e117114;

        let (assign77280_e117127,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77280_e117127;

        let (assign77290_e117140, assign77290_e117140_d_n0, assign77290_e117140_d_n2, assign77290_e117140_d_n4, assign77290_e117140_d_n5, assign77290_e117140_d_n6, assign77290_e117140_d_n7, assign77290_e117140_d_n8, assign77290_e117140_d_n9, assign77290_e117140_d_n10, assign77290_e117140_d_n11, assign77290_e117140_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign77290_e117140;
        locals.var_arg_dn0 = assign77290_e117140_d_n0;
        locals.var_arg_dn2 = assign77290_e117140_d_n2;
        locals.var_arg_dn4 = assign77290_e117140_d_n4;
        locals.var_arg_dn5 = assign77290_e117140_d_n5;
        locals.var_arg_dn6 = assign77290_e117140_d_n6;
        locals.var_arg_dn7 = assign77290_e117140_d_n7;
        locals.var_arg_dn8 = assign77290_e117140_d_n8;
        locals.var_arg_dn9 = assign77290_e117140_d_n9;
        locals.var_arg_dn10 = assign77290_e117140_d_n10;
        locals.var_arg_dn11 = assign77290_e117140_d_n11;
        locals.var_arg_dn14 = assign77290_e117140_d_n14;

        let (assign77300_e117153, assign77300_e117153_d_n0, assign77300_e117153_d_n2, assign77300_e117153_d_n4, assign77300_e117153_d_n5, assign77300_e117153_d_n6, assign77300_e117153_d_n7, assign77300_e117153_d_n8, assign77300_e117153_d_n9, assign77300_e117153_d_n10, assign77300_e117153_d_n11, assign77300_e117153_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign77300_e117153;
        locals.var_dnm_dn0 = assign77300_e117153_d_n0;
        locals.var_dnm_dn2 = assign77300_e117153_d_n2;
        locals.var_dnm_dn4 = assign77300_e117153_d_n4;
        locals.var_dnm_dn5 = assign77300_e117153_d_n5;
        locals.var_dnm_dn6 = assign77300_e117153_d_n6;
        locals.var_dnm_dn7 = assign77300_e117153_d_n7;
        locals.var_dnm_dn8 = assign77300_e117153_d_n8;
        locals.var_dnm_dn9 = assign77300_e117153_d_n9;
        locals.var_dnm_dn10 = assign77300_e117153_d_n10;
        locals.var_dnm_dn11 = assign77300_e117153_d_n11;
        locals.var_dnm_dn14 = assign77300_e117153_d_n14;

        let (assign77310_e117168, assign77310_e117168_d_n0, assign77310_e117168_d_n2, assign77310_e117168_d_n4, assign77310_e117168_d_n5, assign77310_e117168_d_n6, assign77310_e117168_d_n7, assign77310_e117168_d_n8, assign77310_e117168_d_n9, assign77310_e117168_d_n10, assign77310_e117168_d_n11, assign77310_e117168_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77310_e117166: f64 = (locals.var_xp * locals.var_x2);
        (assign77310_e117166, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign77310_e117168;
        locals.var_xp_dn0 = assign77310_e117168_d_n0;
        locals.var_xp_dn2 = assign77310_e117168_d_n2;
        locals.var_xp_dn4 = assign77310_e117168_d_n4;
        locals.var_xp_dn5 = assign77310_e117168_d_n5;
        locals.var_xp_dn6 = assign77310_e117168_d_n6;
        locals.var_xp_dn7 = assign77310_e117168_d_n7;
        locals.var_xp_dn8 = assign77310_e117168_d_n8;
        locals.var_xp_dn9 = assign77310_e117168_d_n9;
        locals.var_xp_dn10 = assign77310_e117168_d_n10;
        locals.var_xp_dn11 = assign77310_e117168_d_n11;
        locals.var_xp_dn14 = assign77310_e117168_d_n14;

        let (assign77320_e117183, assign77320_e117183_d_n0, assign77320_e117183_d_n2, assign77320_e117183_d_n4, assign77320_e117183_d_n5, assign77320_e117183_d_n6, assign77320_e117183_d_n7, assign77320_e117183_d_n8, assign77320_e117183_d_n9, assign77320_e117183_d_n10, assign77320_e117183_d_n11, assign77320_e117183_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77320_e117181: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign77320_e117181, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign77320_e117183;
        locals.var_xmp_dn0 = assign77320_e117183_d_n0;
        locals.var_xmp_dn2 = assign77320_e117183_d_n2;
        locals.var_xmp_dn4 = assign77320_e117183_d_n4;
        locals.var_xmp_dn5 = assign77320_e117183_d_n5;
        locals.var_xmp_dn6 = assign77320_e117183_d_n6;
        locals.var_xmp_dn7 = assign77320_e117183_d_n7;
        locals.var_xmp_dn8 = assign77320_e117183_d_n8;
        locals.var_xmp_dn9 = assign77320_e117183_d_n9;
        locals.var_xmp_dn10 = assign77320_e117183_d_n10;
        locals.var_xmp_dn11 = assign77320_e117183_d_n11;
        locals.var_xmp_dn14 = assign77320_e117183_d_n14;

        let (assign77330_e117198, assign77330_e117198_d_n0, assign77330_e117198_d_n2, assign77330_e117198_d_n4, assign77330_e117198_d_n5, assign77330_e117198_d_n6, assign77330_e117198_d_n7, assign77330_e117198_d_n8, assign77330_e117198_d_n9, assign77330_e117198_d_n10, assign77330_e117198_d_n11, assign77330_e117198_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77330_e117196: f64 = (locals.var_xp * locals.var_x2);
        (assign77330_e117196, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign77330_e117198;
        locals.var_xp_dn0 = assign77330_e117198_d_n0;
        locals.var_xp_dn2 = assign77330_e117198_d_n2;
        locals.var_xp_dn4 = assign77330_e117198_d_n4;
        locals.var_xp_dn5 = assign77330_e117198_d_n5;
        locals.var_xp_dn6 = assign77330_e117198_d_n6;
        locals.var_xp_dn7 = assign77330_e117198_d_n7;
        locals.var_xp_dn8 = assign77330_e117198_d_n8;
        locals.var_xp_dn9 = assign77330_e117198_d_n9;
        locals.var_xp_dn10 = assign77330_e117198_d_n10;
        locals.var_xp_dn11 = assign77330_e117198_d_n11;
        locals.var_xp_dn14 = assign77330_e117198_d_n14;

    }

    pub(super) fn stamp_transient_block_279(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign77340_e117213, assign77340_e117213_d_n0, assign77340_e117213_d_n2, assign77340_e117213_d_n4, assign77340_e117213_d_n5, assign77340_e117213_d_n6, assign77340_e117213_d_n7, assign77340_e117213_d_n8, assign77340_e117213_d_n9, assign77340_e117213_d_n10, assign77340_e117213_d_n11, assign77340_e117213_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77340_e117211: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign77340_e117211, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign77340_e117213;
        locals.var_xmp_dn0 = assign77340_e117213_d_n0;
        locals.var_xmp_dn2 = assign77340_e117213_d_n2;
        locals.var_xmp_dn4 = assign77340_e117213_d_n4;
        locals.var_xmp_dn5 = assign77340_e117213_d_n5;
        locals.var_xmp_dn6 = assign77340_e117213_d_n6;
        locals.var_xmp_dn7 = assign77340_e117213_d_n7;
        locals.var_xmp_dn8 = assign77340_e117213_d_n8;
        locals.var_xmp_dn9 = assign77340_e117213_d_n9;
        locals.var_xmp_dn10 = assign77340_e117213_d_n10;
        locals.var_xmp_dn11 = assign77340_e117213_d_n11;
        locals.var_xmp_dn14 = assign77340_e117213_d_n14;

        let (assign77350_e117228, assign77350_e117228_d_n0, assign77350_e117228_d_n2, assign77350_e117228_d_n4, assign77350_e117228_d_n5, assign77350_e117228_d_n6, assign77350_e117228_d_n7, assign77350_e117228_d_n8, assign77350_e117228_d_n9, assign77350_e117228_d_n10, assign77350_e117228_d_n11, assign77350_e117228_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77350_e117226: f64 = (locals.var_xp + locals.var_xmp);
        (assign77350_e117226, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign77350_e117228;
        locals.var_arg_dn0 = assign77350_e117228_d_n0;
        locals.var_arg_dn2 = assign77350_e117228_d_n2;
        locals.var_arg_dn4 = assign77350_e117228_d_n4;
        locals.var_arg_dn5 = assign77350_e117228_d_n5;
        locals.var_arg_dn6 = assign77350_e117228_d_n6;
        locals.var_arg_dn7 = assign77350_e117228_d_n7;
        locals.var_arg_dn8 = assign77350_e117228_d_n8;
        locals.var_arg_dn9 = assign77350_e117228_d_n9;
        locals.var_arg_dn10 = assign77350_e117228_d_n10;
        locals.var_arg_dn11 = assign77350_e117228_d_n11;
        locals.var_arg_dn14 = assign77350_e117228_d_n14;

        let (assign77360_e117241, assign77360_e117241_d_n0, assign77360_e117241_d_n2, assign77360_e117241_d_n4, assign77360_e117241_d_n5, assign77360_e117241_d_n6, assign77360_e117241_d_n7, assign77360_e117241_d_n8, assign77360_e117241_d_n9, assign77360_e117241_d_n10, assign77360_e117241_d_n11, assign77360_e117241_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign77360_e117241;
        locals.var_dnm_dn0 = assign77360_e117241_d_n0;
        locals.var_dnm_dn2 = assign77360_e117241_d_n2;
        locals.var_dnm_dn4 = assign77360_e117241_d_n4;
        locals.var_dnm_dn5 = assign77360_e117241_d_n5;
        locals.var_dnm_dn6 = assign77360_e117241_d_n6;
        locals.var_dnm_dn7 = assign77360_e117241_d_n7;
        locals.var_dnm_dn8 = assign77360_e117241_d_n8;
        locals.var_dnm_dn9 = assign77360_e117241_d_n9;
        locals.var_dnm_dn10 = assign77360_e117241_d_n10;
        locals.var_dnm_dn11 = assign77360_e117241_d_n11;
        locals.var_dnm_dn14 = assign77360_e117241_d_n14;

        let assign77370_e117256: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1798 = assign77370_e117256;

        let assign77380_e117259: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1799 = assign77380_e117259;

        let (assign77390_e117276,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) && (locals.var_guard1798 != 0.0)) && (locals.var_guard1799 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77390_e117276;

        let assign77400_e117279: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1800 = assign77400_e117279;

        let (assign77410_e117299,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) && (locals.var_guard1798 != 0.0)) && (locals.var_guard1799 == 0.0)) && (locals.var_guard1800 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77410_e117299;

        let assign77420_e117302: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1801 = assign77420_e117302;

        let (assign77430_e117325,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) && (locals.var_guard1798 != 0.0)) && (locals.var_guard1799 == 0.0)) && (locals.var_guard1800 == 0.0)) && (locals.var_guard1801 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77430_e117325;

        let assign77440_e117328: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1802 = assign77440_e117328;

        let (assign77450_e117354,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) && (locals.var_guard1798 != 0.0)) && (locals.var_guard1799 == 0.0)) && (locals.var_guard1800 == 0.0)) && (locals.var_guard1801 == 0.0)) && (locals.var_guard1802 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign77450_e117354;

        let (assign77460_e117369,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) && (locals.var_guard1798 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign77460_e117369;

        let mut assign77470_loop_guard: usize = 0;
        while {
            let assign77470_cond_e117385: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) && (locals.var_guard1798 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign77470_cond_e117385 != 0.0
        } {
            assign77470_loop_guard += 1;
            assert!(assign77470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign77470_body0_e117401, assign77470_body0_e117401_d_n0, assign77470_body0_e117401_d_n2, assign77470_body0_e117401_d_n4, assign77470_body0_e117401_d_n5, assign77470_body0_e117401_d_n6, assign77470_body0_e117401_d_n7, assign77470_body0_e117401_d_n8, assign77470_body0_e117401_d_n9, assign77470_body0_e117401_d_n10, assign77470_body0_e117401_d_n11, assign77470_body0_e117401_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) && (locals.var_guard1798 != 0.0)) {
        let assign77470_body0_e117399: f64 = (locals.var_dnm).sqrt();
        (assign77470_body0_e117399, (locals.var_dnm_dn0 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn2 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn4 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn5 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn6 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn7 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn8 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn9 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn10 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn11 / (2.0 * assign77470_body0_e117399)), (locals.var_dnm_dn14 / (2.0 * assign77470_body0_e117399)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign77470_body0_e117401;
            locals.var_dnm_dn0 = assign77470_body0_e117401_d_n0;
            locals.var_dnm_dn2 = assign77470_body0_e117401_d_n2;
            locals.var_dnm_dn4 = assign77470_body0_e117401_d_n4;
            locals.var_dnm_dn5 = assign77470_body0_e117401_d_n5;
            locals.var_dnm_dn6 = assign77470_body0_e117401_d_n6;
            locals.var_dnm_dn7 = assign77470_body0_e117401_d_n7;
            locals.var_dnm_dn8 = assign77470_body0_e117401_d_n8;
            locals.var_dnm_dn9 = assign77470_body0_e117401_d_n9;
            locals.var_dnm_dn10 = assign77470_body0_e117401_d_n10;
            locals.var_dnm_dn11 = assign77470_body0_e117401_d_n11;
            locals.var_dnm_dn14 = assign77470_body0_e117401_d_n14;
            let (assign77470_body1_e117418,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) && (locals.var_guard1798 != 0.0)) {
        let assign77470_body1_e117416: f64 = (locals.var_m0 + 1.0);
        (assign77470_body1_e117416,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign77470_body1_e117418;
        }

        let (assign77480_e117445, assign77480_e117445_d_n0, assign77480_e117445_d_n2, assign77480_e117445_d_n4, assign77480_e117445_d_n5, assign77480_e117445_d_n6, assign77480_e117445_d_n7, assign77480_e117445_d_n8, assign77480_e117445_d_n9, assign77480_e117445_d_n10, assign77480_e117445_d_n11, assign77480_e117445_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) && (locals.var_guard1798 == 0.0)) {
        let (assign77480_e117443, assign77480_e117443_d_n0, assign77480_e117443_d_n2, assign77480_e117443_d_n4, assign77480_e117443_d_n5, assign77480_e117443_d_n6, assign77480_e117443_d_n7, assign77480_e117443_d_n8, assign77480_e117443_d_n9, assign77480_e117443_d_n10, assign77480_e117443_d_n11, assign77480_e117443_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign77480_e117440: f64 = (2.0 * 2.0);
                let assign77480_e117441: f64 = (1.0 / assign77480_e117440);
                let assign77480_e117442: f64 = (locals.var_dnm).powf(assign77480_e117441);
                (assign77480_e117442, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn0)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn2)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn4)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn5)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn6)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn7)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn8)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn9)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn10)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn11)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign77480_e117441) as f64).is_finite() && ((assign77480_e117441) as f64).fract() == 0.0 { if assign77480_e117441 == 0.0 { 0.0 } else { (assign77480_e117441 * ((locals.var_dnm).powf(assign77480_e117441 - 1.0) * locals.var_dnm_dn14)) } } else { (assign77480_e117442 * (assign77480_e117441 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign77480_e117443, assign77480_e117443_d_n0, assign77480_e117443_d_n2, assign77480_e117443_d_n4, assign77480_e117443_d_n5, assign77480_e117443_d_n6, assign77480_e117443_d_n7, assign77480_e117443_d_n8, assign77480_e117443_d_n9, assign77480_e117443_d_n10, assign77480_e117443_d_n11, assign77480_e117443_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign77480_e117445;
        locals.var_dnm_dn0 = assign77480_e117445_d_n0;
        locals.var_dnm_dn2 = assign77480_e117445_d_n2;
        locals.var_dnm_dn4 = assign77480_e117445_d_n4;
        locals.var_dnm_dn5 = assign77480_e117445_d_n5;
        locals.var_dnm_dn6 = assign77480_e117445_d_n6;
        locals.var_dnm_dn7 = assign77480_e117445_d_n7;
        locals.var_dnm_dn8 = assign77480_e117445_d_n8;
        locals.var_dnm_dn9 = assign77480_e117445_d_n9;
        locals.var_dnm_dn10 = assign77480_e117445_d_n10;
        locals.var_dnm_dn11 = assign77480_e117445_d_n11;
        locals.var_dnm_dn14 = assign77480_e117445_d_n14;

        let (assign77490_e117460, assign77490_e117460_d_n0, assign77490_e117460_d_n2, assign77490_e117460_d_n4, assign77490_e117460_d_n5, assign77490_e117460_d_n6, assign77490_e117460_d_n7, assign77490_e117460_d_n8, assign77490_e117460_d_n9, assign77490_e117460_d_n10, assign77490_e117460_d_n11, assign77490_e117460_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77490_e117458: f64 = (1.0 / locals.var_dnm);
        (assign77490_e117458, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign77490_e117460;
        locals.var_dnm_dn0 = assign77490_e117460_d_n0;
        locals.var_dnm_dn2 = assign77490_e117460_d_n2;
        locals.var_dnm_dn4 = assign77490_e117460_d_n4;
        locals.var_dnm_dn5 = assign77490_e117460_d_n5;
        locals.var_dnm_dn6 = assign77490_e117460_d_n6;
        locals.var_dnm_dn7 = assign77490_e117460_d_n7;
        locals.var_dnm_dn8 = assign77490_e117460_d_n8;
        locals.var_dnm_dn9 = assign77490_e117460_d_n9;
        locals.var_dnm_dn10 = assign77490_e117460_d_n10;
        locals.var_dnm_dn11 = assign77490_e117460_d_n11;
        locals.var_dnm_dn14 = assign77490_e117460_d_n14;

        let (assign77500_e117479, assign77500_e117479_d_n0, assign77500_e117479_d_n2, assign77500_e117479_d_n4, assign77500_e117479_d_n5, assign77500_e117479_d_n6, assign77500_e117479_d_n7, assign77500_e117479_d_n8, assign77500_e117479_d_n9, assign77500_e117479_d_n10, assign77500_e117479_d_n11, assign77500_e117479_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77500_e117474: f64 = (0.2 * locals.var_chi_b);
        let assign77500_e117475: f64 = (locals.var_tmf1 * assign77500_e117474);
        let assign77500_e117477: f64 = (assign77500_e117475 * locals.var_dnm);
        (assign77500_e117477, ((((locals.var_tmf1_dn0 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn0))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn2))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn4))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn5))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn6))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn7))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn8))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn9))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn10))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn11))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * assign77500_e117474) + (locals.var_tmf1 * (0.2 * locals.var_chi_b_dn14))) * locals.var_dnm) + (assign77500_e117475 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign77500_e117479;
        locals.var_tmf0_dn0 = assign77500_e117479_d_n0;
        locals.var_tmf0_dn2 = assign77500_e117479_d_n2;
        locals.var_tmf0_dn4 = assign77500_e117479_d_n4;
        locals.var_tmf0_dn5 = assign77500_e117479_d_n5;
        locals.var_tmf0_dn6 = assign77500_e117479_d_n6;
        locals.var_tmf0_dn7 = assign77500_e117479_d_n7;
        locals.var_tmf0_dn8 = assign77500_e117479_d_n8;
        locals.var_tmf0_dn9 = assign77500_e117479_d_n9;
        locals.var_tmf0_dn10 = assign77500_e117479_d_n10;
        locals.var_tmf0_dn11 = assign77500_e117479_d_n11;
        locals.var_tmf0_dn14 = assign77500_e117479_d_n14;

        let (assign77510_e117500, assign77510_e117500_d_n0, assign77510_e117500_d_n2, assign77510_e117500_d_n4, assign77510_e117500_d_n5, assign77510_e117500_d_n6, assign77510_e117500_d_n7, assign77510_e117500_d_n8, assign77510_e117500_d_n9, assign77510_e117500_d_n10, assign77510_e117500_d_n11, assign77510_e117500_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77510_e117492: f64 = (0.2 * locals.var_chi_b);
        let assign77510_e117494: f64 = (assign77510_e117492 * locals.var_xmp);
        let assign77510_e117496: f64 = (assign77510_e117494 * locals.var_dnm);
        let assign77510_e117498: f64 = (assign77510_e117496 / locals.var_arg);
        (assign77510_e117498, ((((((((0.2 * locals.var_chi_b_dn0) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn0)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn2) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn2)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn4) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn4)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn5) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn5)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn6) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn6)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn7) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn7)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn8) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn8)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn9) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn9)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn10) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn10)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn11) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn11)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((0.2 * locals.var_chi_b_dn14) * locals.var_xmp) + (assign77510_e117492 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign77510_e117494 * locals.var_dnm_dn14)) * locals.var_arg) - (assign77510_e117496 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77510_e117500;
        locals.var_t1_dn0 = assign77510_e117500_d_n0;
        locals.var_t1_dn2 = assign77510_e117500_d_n2;
        locals.var_t1_dn4 = assign77510_e117500_d_n4;
        locals.var_t1_dn5 = assign77510_e117500_d_n5;
        locals.var_t1_dn6 = assign77510_e117500_d_n6;
        locals.var_t1_dn7 = assign77510_e117500_d_n7;
        locals.var_t1_dn8 = assign77510_e117500_d_n8;
        locals.var_t1_dn9 = assign77510_e117500_d_n9;
        locals.var_t1_dn10 = assign77510_e117500_d_n10;
        locals.var_t1_dn11 = assign77510_e117500_d_n11;
        locals.var_t1_dn14 = assign77510_e117500_d_n14;

        let (assign77520_e117519, assign77520_e117519_d_n0, assign77520_e117519_d_n2, assign77520_e117519_d_n4, assign77520_e117519_d_n5, assign77520_e117519_d_n6, assign77520_e117519_d_n7, assign77520_e117519_d_n8, assign77520_e117519_d_n9, assign77520_e117519_d_n10, assign77520_e117519_d_n11, assign77520_e117519_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        let assign77520_e117514: f64 = (0.2 * locals.var_chi_b);
        let assign77520_e117515: f64 = (locals.var_chi_b - assign77520_e117514);
        let assign77520_e117517: f64 = (assign77520_e117515 + locals.var_tmf0);
        (assign77520_e117517, ((locals.var_chi_b_dn0 - (0.2 * locals.var_chi_b_dn0)) + locals.var_tmf0_dn0), ((locals.var_chi_b_dn2 - (0.2 * locals.var_chi_b_dn2)) + locals.var_tmf0_dn2), ((locals.var_chi_b_dn4 - (0.2 * locals.var_chi_b_dn4)) + locals.var_tmf0_dn4), ((locals.var_chi_b_dn5 - (0.2 * locals.var_chi_b_dn5)) + locals.var_tmf0_dn5), ((locals.var_chi_b_dn6 - (0.2 * locals.var_chi_b_dn6)) + locals.var_tmf0_dn6), ((locals.var_chi_b_dn7 - (0.2 * locals.var_chi_b_dn7)) + locals.var_tmf0_dn7), ((locals.var_chi_b_dn8 - (0.2 * locals.var_chi_b_dn8)) + locals.var_tmf0_dn8), ((locals.var_chi_b_dn9 - (0.2 * locals.var_chi_b_dn9)) + locals.var_tmf0_dn9), ((locals.var_chi_b_dn10 - (0.2 * locals.var_chi_b_dn10)) + locals.var_tmf0_dn10), ((locals.var_chi_b_dn11 - (0.2 * locals.var_chi_b_dn11)) + locals.var_tmf0_dn11), ((locals.var_chi_b_dn14 - (0.2 * locals.var_chi_b_dn14)) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign77520_e117519;
        locals.var_chi_dn0 = assign77520_e117519_d_n0;
        locals.var_chi_dn2 = assign77520_e117519_d_n2;
        locals.var_chi_dn4 = assign77520_e117519_d_n4;
        locals.var_chi_dn5 = assign77520_e117519_d_n5;
        locals.var_chi_dn6 = assign77520_e117519_d_n6;
        locals.var_chi_dn7 = assign77520_e117519_d_n7;
        locals.var_chi_dn8 = assign77520_e117519_d_n8;
        locals.var_chi_dn9 = assign77520_e117519_d_n9;
        locals.var_chi_dn10 = assign77520_e117519_d_n10;
        locals.var_chi_dn11 = assign77520_e117519_d_n11;
        locals.var_chi_dn14 = assign77520_e117519_d_n14;

        let (assign77530_e117532, assign77530_e117532_d_n0, assign77530_e117532_d_n2, assign77530_e117532_d_n4, assign77530_e117532_d_n5, assign77530_e117532_d_n6, assign77530_e117532_d_n7, assign77530_e117532_d_n8, assign77530_e117532_d_n9, assign77530_e117532_d_n10, assign77530_e117532_d_n11, assign77530_e117532_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77530_e117532;
        locals.var_t1_dn0 = assign77530_e117532_d_n0;
        locals.var_t1_dn2 = assign77530_e117532_d_n2;
        locals.var_t1_dn4 = assign77530_e117532_d_n4;
        locals.var_t1_dn5 = assign77530_e117532_d_n5;
        locals.var_t1_dn6 = assign77530_e117532_d_n6;
        locals.var_t1_dn7 = assign77530_e117532_d_n7;
        locals.var_t1_dn8 = assign77530_e117532_d_n8;
        locals.var_t1_dn9 = assign77530_e117532_d_n9;
        locals.var_t1_dn10 = assign77530_e117532_d_n10;
        locals.var_t1_dn11 = assign77530_e117532_d_n11;
        locals.var_t1_dn14 = assign77530_e117532_d_n14;

        let (assign77540_e117546, assign77540_e117546_d_n0, assign77540_e117546_d_n2, assign77540_e117546_d_n4, assign77540_e117546_d_n5, assign77540_e117546_d_n6, assign77540_e117546_d_n7, assign77540_e117546_d_n8, assign77540_e117546_d_n9, assign77540_e117546_d_n10, assign77540_e117546_d_n11, assign77540_e117546_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 == 0.0)) {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign77540_e117546;
        locals.var_chi_dn0 = assign77540_e117546_d_n0;
        locals.var_chi_dn2 = assign77540_e117546_d_n2;
        locals.var_chi_dn4 = assign77540_e117546_d_n4;
        locals.var_chi_dn5 = assign77540_e117546_d_n5;
        locals.var_chi_dn6 = assign77540_e117546_d_n6;
        locals.var_chi_dn7 = assign77540_e117546_d_n7;
        locals.var_chi_dn8 = assign77540_e117546_d_n8;
        locals.var_chi_dn9 = assign77540_e117546_d_n9;
        locals.var_chi_dn10 = assign77540_e117546_d_n10;
        locals.var_chi_dn11 = assign77540_e117546_d_n11;
        locals.var_chi_dn14 = assign77540_e117546_d_n14;

        let (assign77550_e117560, assign77550_e117560_d_n0, assign77550_e117560_d_n2, assign77550_e117560_d_n4, assign77550_e117560_d_n5, assign77550_e117560_d_n6, assign77550_e117560_d_n7, assign77550_e117560_d_n8, assign77550_e117560_d_n9, assign77550_e117560_d_n10, assign77550_e117560_d_n11, assign77550_e117560_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 != 0.0)) && (locals.var_guard1797 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77550_e117560;
        locals.var_t1_dn0 = assign77550_e117560_d_n0;
        locals.var_t1_dn2 = assign77550_e117560_d_n2;
        locals.var_t1_dn4 = assign77550_e117560_d_n4;
        locals.var_t1_dn5 = assign77550_e117560_d_n5;
        locals.var_t1_dn6 = assign77550_e117560_d_n6;
        locals.var_t1_dn7 = assign77550_e117560_d_n7;
        locals.var_t1_dn8 = assign77550_e117560_d_n8;
        locals.var_t1_dn9 = assign77550_e117560_d_n9;
        locals.var_t1_dn10 = assign77550_e117560_d_n10;
        locals.var_t1_dn11 = assign77550_e117560_d_n11;
        locals.var_t1_dn14 = assign77550_e117560_d_n14;

        let (assign77560_e117577, assign77560_e117577_d_n0, assign77560_e117577_d_n2, assign77560_e117577_d_n4, assign77560_e117577_d_n5, assign77560_e117577_d_n6, assign77560_e117577_d_n7, assign77560_e117577_d_n8, assign77560_e117577_d_n9, assign77560_e117577_d_n10, assign77560_e117577_d_n11, assign77560_e117577_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1794 != 0.0)) && (locals.var_guard1796 == 0.0)) {
        let (assign77560_e117575, assign77560_e117575_d_n0, assign77560_e117575_d_n2, assign77560_e117575_d_n4, assign77560_e117575_d_n5, assign77560_e117575_d_n6, assign77560_e117575_d_n7, assign77560_e117575_d_n8, assign77560_e117575_d_n9, assign77560_e117575_d_n10, assign77560_e117575_d_n11, assign77560_e117575_d_n14,) = {
            if (locals.var_chi_a <= locals.var_chi_b) {
                (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn4, locals.var_chi_a_dn5, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn8, locals.var_chi_a_dn9, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn14,)
            } else {
                (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn4, locals.var_chi_b_dn5, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn8, locals.var_chi_b_dn9, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn14,)
            }
        };
        (assign77560_e117575, assign77560_e117575_d_n0, assign77560_e117575_d_n2, assign77560_e117575_d_n4, assign77560_e117575_d_n5, assign77560_e117575_d_n6, assign77560_e117575_d_n7, assign77560_e117575_d_n8, assign77560_e117575_d_n9, assign77560_e117575_d_n10, assign77560_e117575_d_n11, assign77560_e117575_d_n14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign77560_e117577;
        locals.var_chi_dn0 = assign77560_e117577_d_n0;
        locals.var_chi_dn2 = assign77560_e117577_d_n2;
        locals.var_chi_dn4 = assign77560_e117577_d_n4;
        locals.var_chi_dn5 = assign77560_e117577_d_n5;
        locals.var_chi_dn6 = assign77560_e117577_d_n6;
        locals.var_chi_dn7 = assign77560_e117577_d_n7;
        locals.var_chi_dn8 = assign77560_e117577_d_n8;
        locals.var_chi_dn9 = assign77560_e117577_d_n9;
        locals.var_chi_dn10 = assign77560_e117577_d_n10;
        locals.var_chi_dn11 = assign77560_e117577_d_n11;
        locals.var_chi_dn14 = assign77560_e117577_d_n14;

        let assign77570_e117580: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1803 = assign77570_e117580;

        let (assign77580_e117593, assign77580_e117593_d_n0, assign77580_e117593_d_n2, assign77580_e117593_d_n4, assign77580_e117593_d_n5, assign77580_e117593_d_n6, assign77580_e117593_d_n7, assign77580_e117593_d_n8, assign77580_e117593_d_n9, assign77580_e117593_d_n10, assign77580_e117593_d_n11, assign77580_e117593_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77580_e117589: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign77580_e117591: f64 = (assign77580_e117589 - locals.var_vxbgmtcl);
        (assign77580_e117591, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign77580_e117593;
        locals.var_ps0ld_dn0 = assign77580_e117593_d_n0;
        locals.var_ps0ld_dn2 = assign77580_e117593_d_n2;
        locals.var_ps0ld_dn4 = assign77580_e117593_d_n4;
        locals.var_ps0ld_dn5 = assign77580_e117593_d_n5;
        locals.var_ps0ld_dn6 = assign77580_e117593_d_n6;
        locals.var_ps0ld_dn7 = assign77580_e117593_d_n7;
        locals.var_ps0ld_dn8 = assign77580_e117593_d_n8;
        locals.var_ps0ld_dn9 = assign77580_e117593_d_n9;
        locals.var_ps0ld_dn10 = assign77580_e117593_d_n10;
        locals.var_ps0ld_dn11 = assign77580_e117593_d_n11;
        locals.var_ps0ld_dn14 = assign77580_e117593_d_n14;

        let assign77590_e117596: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1804 = assign77590_e117596;

        let (assign77600_e117609, assign77600_e117609_d_n0, assign77600_e117609_d_n2, assign77600_e117609_d_n4, assign77600_e117609_d_n5, assign77600_e117609_d_n6, assign77600_e117609_d_n7, assign77600_e117609_d_n8, assign77600_e117609_d_n9, assign77600_e117609_d_n10, assign77600_e117609_d_n11, assign77600_e117609_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1804 != 0.0)) {
        let assign77600_e117607: f64 = (p.p334 - locals.var_wdep_func);
        (assign77600_e117607, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn11), (-locals.var_wdep_func_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77600_e117609;
        locals.var_t2_dn0 = assign77600_e117609_d_n0;
        locals.var_t2_dn2 = assign77600_e117609_d_n2;
        locals.var_t2_dn4 = assign77600_e117609_d_n4;
        locals.var_t2_dn5 = assign77600_e117609_d_n5;
        locals.var_t2_dn6 = assign77600_e117609_d_n6;
        locals.var_t2_dn7 = assign77600_e117609_d_n7;
        locals.var_t2_dn8 = assign77600_e117609_d_n8;
        locals.var_t2_dn9 = assign77600_e117609_d_n9;
        locals.var_t2_dn10 = assign77600_e117609_d_n10;
        locals.var_t2_dn11 = assign77600_e117609_d_n11;
        locals.var_t2_dn14 = assign77600_e117609_d_n14;

        let (assign77610_e117634, assign77610_e117634_d_n0, assign77610_e117634_d_n2, assign77610_e117634_d_n4, assign77610_e117634_d_n5, assign77610_e117634_d_n6, assign77610_e117634_d_n7, assign77610_e117634_d_n8, assign77610_e117634_d_n9, assign77610_e117634_d_n10, assign77610_e117634_d_n11, assign77610_e117634_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1804 == 0.0)) {
        let assign77610_e117621: f64 = (locals.var_vdsi + p.p137);
        let assign77610_e117624: f64 = (locals.var_vdsi + p.p137);
        let assign77610_e117625: f64 = (assign77610_e117621 * assign77610_e117624);
        let assign77610_e117628: f64 = (4.0 * 0.1);
        let assign77610_e117630: f64 = (assign77610_e117628 * 0.1);
        let assign77610_e117631: f64 = (assign77610_e117625 + assign77610_e117630);
        let assign77610_e117632: f64 = (assign77610_e117631).sqrt();
        (assign77610_e117632, 0.0, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn6 * assign77610_e117624) + (assign77610_e117621 * locals.var_vdsi_dn6)) / (2.0 * assign77610_e117632)), 0.0, (((locals.var_vdsi_dn8 * assign77610_e117624) + (assign77610_e117621 * locals.var_vdsi_dn8)) / (2.0 * assign77610_e117632)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign77610_e117634;
        locals.var_tmf2_dn0 = assign77610_e117634_d_n0;
        locals.var_tmf2_dn2 = assign77610_e117634_d_n2;
        locals.var_tmf2_dn4 = assign77610_e117634_d_n4;
        locals.var_tmf2_dn5 = assign77610_e117634_d_n5;
        locals.var_tmf2_dn6 = assign77610_e117634_d_n6;
        locals.var_tmf2_dn7 = assign77610_e117634_d_n7;
        locals.var_tmf2_dn8 = assign77610_e117634_d_n8;
        locals.var_tmf2_dn9 = assign77610_e117634_d_n9;
        locals.var_tmf2_dn10 = assign77610_e117634_d_n10;
        locals.var_tmf2_dn11 = assign77610_e117634_d_n11;
        locals.var_tmf2_dn14 = assign77610_e117634_d_n14;

        let (assign77620_e117654, assign77620_e117654_d_n0, assign77620_e117654_d_n2, assign77620_e117654_d_n4, assign77620_e117654_d_n5, assign77620_e117654_d_n6, assign77620_e117654_d_n7, assign77620_e117654_d_n8, assign77620_e117654_d_n9, assign77620_e117654_d_n10, assign77620_e117654_d_n11, assign77620_e117654_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1804 == 0.0)) {
        let assign77620_e117648: f64 = (locals.var_vdsi + p.p137);
        let assign77620_e117650: f64 = (assign77620_e117648 / locals.var_tmf2);
        let assign77620_e117651: f64 = (1.0 + assign77620_e117650);
        let assign77620_e117652: f64 = (0.5 * assign77620_e117651);
        (assign77620_e117652, (0.5 * (-((assign77620_e117648 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77620_e117648 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77620_e117648 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77620_e117648 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn6 * locals.var_tmf2) - (assign77620_e117648 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign77620_e117648 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn8 * locals.var_tmf2) - (assign77620_e117648 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign77620_e117648 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77620_e117648 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77620_e117648 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign77620_e117648 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign77620_e117654;
        locals.var_t9_dn0 = assign77620_e117654_d_n0;
        locals.var_t9_dn2 = assign77620_e117654_d_n2;
        locals.var_t9_dn4 = assign77620_e117654_d_n4;
        locals.var_t9_dn5 = assign77620_e117654_d_n5;
        locals.var_t9_dn6 = assign77620_e117654_d_n6;
        locals.var_t9_dn7 = assign77620_e117654_d_n7;
        locals.var_t9_dn8 = assign77620_e117654_d_n8;
        locals.var_t9_dn9 = assign77620_e117654_d_n9;
        locals.var_t9_dn10 = assign77620_e117654_d_n10;
        locals.var_t9_dn11 = assign77620_e117654_d_n11;
        locals.var_t9_dn14 = assign77620_e117654_d_n14;

        let (assign77630_e117672, assign77630_e117672_d_n0, assign77630_e117672_d_n2, assign77630_e117672_d_n4, assign77630_e117672_d_n5, assign77630_e117672_d_n6, assign77630_e117672_d_n7, assign77630_e117672_d_n8, assign77630_e117672_d_n9, assign77630_e117672_d_n10, assign77630_e117672_d_n11, assign77630_e117672_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1804 == 0.0)) {
        let assign77630_e117667: f64 = (locals.var_vdsi + p.p137);
        let assign77630_e117669: f64 = (assign77630_e117667 + locals.var_tmf2);
        let assign77630_e117670: f64 = (0.5 * assign77630_e117669);
        (assign77630_e117670, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * (locals.var_vdsi_dn6 + locals.var_tmf2_dn6)), (0.5 * locals.var_tmf2_dn7), (0.5 * (locals.var_vdsi_dn8 + locals.var_tmf2_dn8)), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77630_e117672;
        locals.var_t2_dn0 = assign77630_e117672_d_n0;
        locals.var_t2_dn2 = assign77630_e117672_d_n2;
        locals.var_t2_dn4 = assign77630_e117672_d_n4;
        locals.var_t2_dn5 = assign77630_e117672_d_n5;
        locals.var_t2_dn6 = assign77630_e117672_d_n6;
        locals.var_t2_dn7 = assign77630_e117672_d_n7;
        locals.var_t2_dn8 = assign77630_e117672_d_n8;
        locals.var_t2_dn9 = assign77630_e117672_d_n9;
        locals.var_t2_dn10 = assign77630_e117672_d_n10;
        locals.var_t2_dn11 = assign77630_e117672_d_n11;
        locals.var_t2_dn14 = assign77630_e117672_d_n14;

        let assign77640_e117675: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1805 = assign77640_e117675;

    }

    pub(super) fn stamp_transient_block_280(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign77650_e117689, assign77650_e117689_d_n0, assign77650_e117689_d_n2, assign77650_e117689_d_n4, assign77650_e117689_d_n5, assign77650_e117689_d_n6, assign77650_e117689_d_n7, assign77650_e117689_d_n8, assign77650_e117689_d_n9, assign77650_e117689_d_n10, assign77650_e117689_d_n11, assign77650_e117689_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1804 == 0.0)) && (locals.var_guard1805 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77650_e117689;
        locals.var_t2_dn0 = assign77650_e117689_d_n0;
        locals.var_t2_dn2 = assign77650_e117689_d_n2;
        locals.var_t2_dn4 = assign77650_e117689_d_n4;
        locals.var_t2_dn5 = assign77650_e117689_d_n5;
        locals.var_t2_dn6 = assign77650_e117689_d_n6;
        locals.var_t2_dn7 = assign77650_e117689_d_n7;
        locals.var_t2_dn8 = assign77650_e117689_d_n8;
        locals.var_t2_dn9 = assign77650_e117689_d_n9;
        locals.var_t2_dn10 = assign77650_e117689_d_n10;
        locals.var_t2_dn11 = assign77650_e117689_d_n11;
        locals.var_t2_dn14 = assign77650_e117689_d_n14;

        let (assign77660_e117703, assign77660_e117703_d_n0, assign77660_e117703_d_n2, assign77660_e117703_d_n4, assign77660_e117703_d_n5, assign77660_e117703_d_n6, assign77660_e117703_d_n7, assign77660_e117703_d_n8, assign77660_e117703_d_n9, assign77660_e117703_d_n10, assign77660_e117703_d_n11, assign77660_e117703_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1804 == 0.0)) && (locals.var_guard1805 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign77660_e117703;
        locals.var_t9_dn0 = assign77660_e117703_d_n0;
        locals.var_t9_dn2 = assign77660_e117703_d_n2;
        locals.var_t9_dn4 = assign77660_e117703_d_n4;
        locals.var_t9_dn5 = assign77660_e117703_d_n5;
        locals.var_t9_dn6 = assign77660_e117703_d_n6;
        locals.var_t9_dn7 = assign77660_e117703_d_n7;
        locals.var_t9_dn8 = assign77660_e117703_d_n8;
        locals.var_t9_dn9 = assign77660_e117703_d_n9;
        locals.var_t9_dn10 = assign77660_e117703_d_n10;
        locals.var_t9_dn11 = assign77660_e117703_d_n11;
        locals.var_t9_dn14 = assign77660_e117703_d_n14;

        let (assign77670_e117720, assign77670_e117720_d_n0, assign77670_e117720_d_n2, assign77670_e117720_d_n4, assign77670_e117720_d_n5, assign77670_e117720_d_n6, assign77670_e117720_d_n7, assign77670_e117720_d_n8, assign77670_e117720_d_n9, assign77670_e117720_d_n10, assign77670_e117720_d_n11, assign77670_e117720_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1804 == 0.0)) {
        let assign77670_e117715: f64 = (locals.var_kjunc * locals.var_t2);
        let assign77670_e117716: f64 = (assign77670_e117715).sqrt();
        let assign77670_e117718: f64 = (assign77670_e117716 * p.p432);
        (assign77670_e117718, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign77670_e117716)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign77670_e117716)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign77670_e117720;
        locals.var_wjunc0_dn0 = assign77670_e117720_d_n0;
        locals.var_wjunc0_dn2 = assign77670_e117720_d_n2;
        locals.var_wjunc0_dn4 = assign77670_e117720_d_n4;
        locals.var_wjunc0_dn5 = assign77670_e117720_d_n5;
        locals.var_wjunc0_dn6 = assign77670_e117720_d_n6;
        locals.var_wjunc0_dn7 = assign77670_e117720_d_n7;
        locals.var_wjunc0_dn8 = assign77670_e117720_d_n8;
        locals.var_wjunc0_dn9 = assign77670_e117720_d_n9;
        locals.var_wjunc0_dn10 = assign77670_e117720_d_n10;
        locals.var_wjunc0_dn11 = assign77670_e117720_d_n11;
        locals.var_wjunc0_dn14 = assign77670_e117720_d_n14;

        let (assign77680_e117734, assign77680_e117734_d_n0, assign77680_e117734_d_n2, assign77680_e117734_d_n4, assign77680_e117734_d_n5, assign77680_e117734_d_n6, assign77680_e117734_d_n7, assign77680_e117734_d_n8, assign77680_e117734_d_n9, assign77680_e117734_d_n10, assign77680_e117734_d_n11, assign77680_e117734_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1804 == 0.0)) {
        let assign77680_e117732: f64 = (p.p334 - locals.var_wjunc0);
        (assign77680_e117732, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77680_e117734;
        locals.var_t2_dn0 = assign77680_e117734_d_n0;
        locals.var_t2_dn2 = assign77680_e117734_d_n2;
        locals.var_t2_dn4 = assign77680_e117734_d_n4;
        locals.var_t2_dn5 = assign77680_e117734_d_n5;
        locals.var_t2_dn6 = assign77680_e117734_d_n6;
        locals.var_t2_dn7 = assign77680_e117734_d_n7;
        locals.var_t2_dn8 = assign77680_e117734_d_n8;
        locals.var_t2_dn9 = assign77680_e117734_d_n9;
        locals.var_t2_dn10 = assign77680_e117734_d_n10;
        locals.var_t2_dn11 = assign77680_e117734_d_n11;
        locals.var_t2_dn14 = assign77680_e117734_d_n14;

        let (assign77690_e117756, assign77690_e117756_d_n0, assign77690_e117756_d_n2, assign77690_e117756_d_n4, assign77690_e117756_d_n5, assign77690_e117756_d_n6, assign77690_e117756_d_n7, assign77690_e117756_d_n8, assign77690_e117756_d_n9, assign77690_e117756_d_n10, assign77690_e117756_d_n11, assign77690_e117756_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77690_e117743: f64 = (locals.var_t2 * locals.var_t2);
        let assign77690_e117747: f64 = (p.p334 * 0.01);
        let assign77690_e117748: f64 = (4.0 * assign77690_e117747);
        let assign77690_e117751: f64 = (p.p334 * 0.01);
        let assign77690_e117752: f64 = (assign77690_e117748 * assign77690_e117751);
        let assign77690_e117753: f64 = (assign77690_e117743 + assign77690_e117752);
        let assign77690_e117754: f64 = (assign77690_e117753).sqrt();
        (assign77690_e117754, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign77690_e117754)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign77690_e117754)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign77690_e117756;
        locals.var_tmf2_dn0 = assign77690_e117756_d_n0;
        locals.var_tmf2_dn2 = assign77690_e117756_d_n2;
        locals.var_tmf2_dn4 = assign77690_e117756_d_n4;
        locals.var_tmf2_dn5 = assign77690_e117756_d_n5;
        locals.var_tmf2_dn6 = assign77690_e117756_d_n6;
        locals.var_tmf2_dn7 = assign77690_e117756_d_n7;
        locals.var_tmf2_dn8 = assign77690_e117756_d_n8;
        locals.var_tmf2_dn9 = assign77690_e117756_d_n9;
        locals.var_tmf2_dn10 = assign77690_e117756_d_n10;
        locals.var_tmf2_dn11 = assign77690_e117756_d_n11;
        locals.var_tmf2_dn14 = assign77690_e117756_d_n14;

        let (assign77700_e117771, assign77700_e117771_d_n0, assign77700_e117771_d_n2, assign77700_e117771_d_n4, assign77700_e117771_d_n5, assign77700_e117771_d_n6, assign77700_e117771_d_n7, assign77700_e117771_d_n8, assign77700_e117771_d_n9, assign77700_e117771_d_n10, assign77700_e117771_d_n11, assign77700_e117771_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77700_e117767: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign77700_e117768: f64 = (1.0 + assign77700_e117767);
        let assign77700_e117769: f64 = (0.5 * assign77700_e117768);
        (assign77700_e117769, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign77700_e117771;
        locals.var_t9_dn0 = assign77700_e117771_d_n0;
        locals.var_t9_dn2 = assign77700_e117771_d_n2;
        locals.var_t9_dn4 = assign77700_e117771_d_n4;
        locals.var_t9_dn5 = assign77700_e117771_d_n5;
        locals.var_t9_dn6 = assign77700_e117771_d_n6;
        locals.var_t9_dn7 = assign77700_e117771_d_n7;
        locals.var_t9_dn8 = assign77700_e117771_d_n8;
        locals.var_t9_dn9 = assign77700_e117771_d_n9;
        locals.var_t9_dn10 = assign77700_e117771_d_n10;
        locals.var_t9_dn11 = assign77700_e117771_d_n11;
        locals.var_t9_dn14 = assign77700_e117771_d_n14;

        let (assign77710_e117784, assign77710_e117784_d_n0, assign77710_e117784_d_n2, assign77710_e117784_d_n4, assign77710_e117784_d_n5, assign77710_e117784_d_n6, assign77710_e117784_d_n7, assign77710_e117784_d_n8, assign77710_e117784_d_n9, assign77710_e117784_d_n10, assign77710_e117784_d_n11, assign77710_e117784_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77710_e117781: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign77710_e117782: f64 = (0.5 * assign77710_e117781);
        (assign77710_e117782, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77710_e117784;
        locals.var_t2_dn0 = assign77710_e117784_d_n0;
        locals.var_t2_dn2 = assign77710_e117784_d_n2;
        locals.var_t2_dn4 = assign77710_e117784_d_n4;
        locals.var_t2_dn5 = assign77710_e117784_d_n5;
        locals.var_t2_dn6 = assign77710_e117784_d_n6;
        locals.var_t2_dn7 = assign77710_e117784_d_n7;
        locals.var_t2_dn8 = assign77710_e117784_d_n8;
        locals.var_t2_dn9 = assign77710_e117784_d_n9;
        locals.var_t2_dn10 = assign77710_e117784_d_n10;
        locals.var_t2_dn11 = assign77710_e117784_d_n11;
        locals.var_t2_dn14 = assign77710_e117784_d_n14;

        let assign77720_e117787: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1806 = assign77720_e117787;

        let (assign77730_e117798, assign77730_e117798_d_n0, assign77730_e117798_d_n2, assign77730_e117798_d_n4, assign77730_e117798_d_n5, assign77730_e117798_d_n6, assign77730_e117798_d_n7, assign77730_e117798_d_n8, assign77730_e117798_d_n9, assign77730_e117798_d_n10, assign77730_e117798_d_n11, assign77730_e117798_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1806 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77730_e117798;
        locals.var_t2_dn0 = assign77730_e117798_d_n0;
        locals.var_t2_dn2 = assign77730_e117798_d_n2;
        locals.var_t2_dn4 = assign77730_e117798_d_n4;
        locals.var_t2_dn5 = assign77730_e117798_d_n5;
        locals.var_t2_dn6 = assign77730_e117798_d_n6;
        locals.var_t2_dn7 = assign77730_e117798_d_n7;
        locals.var_t2_dn8 = assign77730_e117798_d_n8;
        locals.var_t2_dn9 = assign77730_e117798_d_n9;
        locals.var_t2_dn10 = assign77730_e117798_d_n10;
        locals.var_t2_dn11 = assign77730_e117798_d_n11;
        locals.var_t2_dn14 = assign77730_e117798_d_n14;

        let (assign77740_e117809, assign77740_e117809_d_n0, assign77740_e117809_d_n2, assign77740_e117809_d_n4, assign77740_e117809_d_n5, assign77740_e117809_d_n6, assign77740_e117809_d_n7, assign77740_e117809_d_n8, assign77740_e117809_d_n9, assign77740_e117809_d_n10, assign77740_e117809_d_n11, assign77740_e117809_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1806 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign77740_e117809;
        locals.var_t9_dn0 = assign77740_e117809_d_n0;
        locals.var_t9_dn2 = assign77740_e117809_d_n2;
        locals.var_t9_dn4 = assign77740_e117809_d_n4;
        locals.var_t9_dn5 = assign77740_e117809_d_n5;
        locals.var_t9_dn6 = assign77740_e117809_d_n6;
        locals.var_t9_dn7 = assign77740_e117809_d_n7;
        locals.var_t9_dn8 = assign77740_e117809_d_n8;
        locals.var_t9_dn9 = assign77740_e117809_d_n9;
        locals.var_t9_dn10 = assign77740_e117809_d_n10;
        locals.var_t9_dn11 = assign77740_e117809_d_n11;
        locals.var_t9_dn14 = assign77740_e117809_d_n14;

        let (assign77750_e117818, assign77750_e117818_d_n0, assign77750_e117818_d_n2, assign77750_e117818_d_n4, assign77750_e117818_d_n5, assign77750_e117818_d_n6, assign77750_e117818_d_n7, assign77750_e117818_d_n8, assign77750_e117818_d_n9, assign77750_e117818_d_n10, assign77750_e117818_d_n11, assign77750_e117818_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign77750_e117818;
        locals.var_ddriftldc_dn0 = assign77750_e117818_d_n0;
        locals.var_ddriftldc_dn2 = assign77750_e117818_d_n2;
        locals.var_ddriftldc_dn4 = assign77750_e117818_d_n4;
        locals.var_ddriftldc_dn5 = assign77750_e117818_d_n5;
        locals.var_ddriftldc_dn6 = assign77750_e117818_d_n6;
        locals.var_ddriftldc_dn7 = assign77750_e117818_d_n7;
        locals.var_ddriftldc_dn8 = assign77750_e117818_d_n8;
        locals.var_ddriftldc_dn9 = assign77750_e117818_d_n9;
        locals.var_ddriftldc_dn10 = assign77750_e117818_d_n10;
        locals.var_ddriftldc_dn11 = assign77750_e117818_d_n11;
        locals.var_ddriftldc_dn14 = assign77750_e117818_d_n14;

        let (assign77760_e117835, assign77760_e117835_d_n0, assign77760_e117835_d_n2, assign77760_e117835_d_n4, assign77760_e117835_d_n5, assign77760_e117835_d_n6, assign77760_e117835_d_n7, assign77760_e117835_d_n8, assign77760_e117835_d_n9, assign77760_e117835_d_n10, assign77760_e117835_d_n11, assign77760_e117835_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77760_e117827: f64 = (locals.var_q_nsubld__blk1766 * locals.var_ddriftldc);
        let assign77760_e117829: f64 = (assign77760_e117827 * locals.var_ddriftldc);
        let assign77760_e117831: f64 = (assign77760_e117829 / 2.0);
        let assign77760_e117833: f64 = (assign77760_e117831 / 1.034943e-10);
        (assign77760_e117833, (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign77760_e117827 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign77760_e117835;
        locals.var_dphi_sb_dn0 = assign77760_e117835_d_n0;
        locals.var_dphi_sb_dn2 = assign77760_e117835_d_n2;
        locals.var_dphi_sb_dn4 = assign77760_e117835_d_n4;
        locals.var_dphi_sb_dn5 = assign77760_e117835_d_n5;
        locals.var_dphi_sb_dn6 = assign77760_e117835_d_n6;
        locals.var_dphi_sb_dn7 = assign77760_e117835_d_n7;
        locals.var_dphi_sb_dn8 = assign77760_e117835_d_n8;
        locals.var_dphi_sb_dn9 = assign77760_e117835_d_n9;
        locals.var_dphi_sb_dn10 = assign77760_e117835_d_n10;
        locals.var_dphi_sb_dn11 = assign77760_e117835_d_n11;
        locals.var_dphi_sb_dn14 = assign77760_e117835_d_n14;

        let (assign77770_e117849, assign77770_e117849_d_n0, assign77770_e117849_d_n2, assign77770_e117849_d_n4, assign77770_e117849_d_n5, assign77770_e117849_d_n6, assign77770_e117849_d_n7, assign77770_e117849_d_n8, assign77770_e117849_d_n9, assign77770_e117849_d_n10, assign77770_e117849_d_n11, assign77770_e117849_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77770_e117844: f64 = (2.0 * locals.var_beta);
        let assign77770_e117846: f64 = (assign77770_e117844 * locals.var_dphi_sb);
        let assign77770_e117847: f64 = (assign77770_e117846).sqrt();
        (assign77770_e117847, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn0)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn2)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn4)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn5)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn6)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn7)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn8)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn9)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn10)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn11)) / (2.0 * assign77770_e117847)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign77770_e117844 * locals.var_dphi_sb_dn14)) / (2.0 * assign77770_e117847)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign77770_e117849;
        locals.var_t0_dn0 = assign77770_e117849_d_n0;
        locals.var_t0_dn2 = assign77770_e117849_d_n2;
        locals.var_t0_dn4 = assign77770_e117849_d_n4;
        locals.var_t0_dn5 = assign77770_e117849_d_n5;
        locals.var_t0_dn6 = assign77770_e117849_d_n6;
        locals.var_t0_dn7 = assign77770_e117849_d_n7;
        locals.var_t0_dn8 = assign77770_e117849_d_n8;
        locals.var_t0_dn9 = assign77770_e117849_d_n9;
        locals.var_t0_dn10 = assign77770_e117849_d_n10;
        locals.var_t0_dn11 = assign77770_e117849_d_n11;
        locals.var_t0_dn14 = assign77770_e117849_d_n14;

        let (assign77780_e117865, assign77780_e117865_d_n0, assign77780_e117865_d_n2, assign77780_e117865_d_n4, assign77780_e117865_d_n5, assign77780_e117865_d_n6, assign77780_e117865_d_n7, assign77780_e117865_d_n8, assign77780_e117865_d_n9, assign77780_e117865_d_n10, assign77780_e117865_d_n11, assign77780_e117865_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77780_e117857: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign77780_e117859: f64 = (-locals.var_t0);
        let assign77780_e117860: f64 = { let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign77780_e117861: f64 = (assign77780_e117857 + assign77780_e117860);
        let assign77780_e117863: f64 = (assign77780_e117861 / 2.0);
        (assign77780_e117863, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign77780_e117859; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77780_e117865;
        locals.var_t1_dn0 = assign77780_e117865_d_n0;
        locals.var_t1_dn2 = assign77780_e117865_d_n2;
        locals.var_t1_dn4 = assign77780_e117865_d_n4;
        locals.var_t1_dn5 = assign77780_e117865_d_n5;
        locals.var_t1_dn6 = assign77780_e117865_d_n6;
        locals.var_t1_dn7 = assign77780_e117865_d_n7;
        locals.var_t1_dn8 = assign77780_e117865_d_n8;
        locals.var_t1_dn9 = assign77780_e117865_d_n9;
        locals.var_t1_dn10 = assign77780_e117865_d_n10;
        locals.var_t1_dn11 = assign77780_e117865_d_n11;
        locals.var_t1_dn14 = assign77780_e117865_d_n14;

        let (assign77790_e117877, assign77790_e117877_d_n0, assign77790_e117877_d_n2, assign77790_e117877_d_n4, assign77790_e117877_d_n5, assign77790_e117877_d_n6, assign77790_e117877_d_n7, assign77790_e117877_d_n8, assign77790_e117877_d_n9, assign77790_e117877_d_n10, assign77790_e117877_d_n11, assign77790_e117877_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77790_e117873: f64 = (locals.var_t1).ln();
        let assign77790_e117875: f64 = (assign77790_e117873 / locals.var_dphi_sb);
        (assign77790_e117875, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign77790_e117873 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign77790_e117877;
        locals.var_c_sb_dn0 = assign77790_e117877_d_n0;
        locals.var_c_sb_dn2 = assign77790_e117877_d_n2;
        locals.var_c_sb_dn4 = assign77790_e117877_d_n4;
        locals.var_c_sb_dn5 = assign77790_e117877_d_n5;
        locals.var_c_sb_dn6 = assign77790_e117877_d_n6;
        locals.var_c_sb_dn7 = assign77790_e117877_d_n7;
        locals.var_c_sb_dn8 = assign77790_e117877_d_n8;
        locals.var_c_sb_dn9 = assign77790_e117877_d_n9;
        locals.var_c_sb_dn10 = assign77790_e117877_d_n10;
        locals.var_c_sb_dn11 = assign77790_e117877_d_n11;
        locals.var_c_sb_dn14 = assign77790_e117877_d_n14;

        let (assign77800_e117888, assign77800_e117888_d_n0, assign77800_e117888_d_n2, assign77800_e117888_d_n4, assign77800_e117888_d_n5, assign77800_e117888_d_n6, assign77800_e117888_d_n7, assign77800_e117888_d_n8, assign77800_e117888_d_n9, assign77800_e117888_d_n10, assign77800_e117888_d_n11, assign77800_e117888_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77800_e117886: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign77800_e117886, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_ps0ld_dn14 + locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn11, locals.var_ps0ld_vxb_dn14,)
    }
};
        locals.var_ps0ld_vxb = assign77800_e117888;
        locals.var_ps0ld_vxb_dn0 = assign77800_e117888_d_n0;
        locals.var_ps0ld_vxb_dn2 = assign77800_e117888_d_n2;
        locals.var_ps0ld_vxb_dn4 = assign77800_e117888_d_n4;
        locals.var_ps0ld_vxb_dn5 = assign77800_e117888_d_n5;
        locals.var_ps0ld_vxb_dn6 = assign77800_e117888_d_n6;
        locals.var_ps0ld_vxb_dn7 = assign77800_e117888_d_n7;
        locals.var_ps0ld_vxb_dn8 = assign77800_e117888_d_n8;
        locals.var_ps0ld_vxb_dn9 = assign77800_e117888_d_n9;
        locals.var_ps0ld_vxb_dn10 = assign77800_e117888_d_n10;
        locals.var_ps0ld_vxb_dn11 = assign77800_e117888_d_n11;
        locals.var_ps0ld_vxb_dn14 = assign77800_e117888_d_n14;

        let (assign77810_e117901, assign77810_e117901_d_n0, assign77810_e117901_d_n2, assign77810_e117901_d_n4, assign77810_e117901_d_n5, assign77810_e117901_d_n6, assign77810_e117901_d_n7, assign77810_e117901_d_n8, assign77810_e117901_d_n9, assign77810_e117901_d_n10, assign77810_e117901_d_n11, assign77810_e117901_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77810_e117898: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign77810_e117899: f64 = (locals.var_c_sb * assign77810_e117898);
        (assign77810_e117899, ((locals.var_c_sb_dn0 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn14 * assign77810_e117898) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign77810_e117901;
        locals.var_ty_dn0 = assign77810_e117901_d_n0;
        locals.var_ty_dn2 = assign77810_e117901_d_n2;
        locals.var_ty_dn4 = assign77810_e117901_d_n4;
        locals.var_ty_dn5 = assign77810_e117901_d_n5;
        locals.var_ty_dn6 = assign77810_e117901_d_n6;
        locals.var_ty_dn7 = assign77810_e117901_d_n7;
        locals.var_ty_dn8 = assign77810_e117901_d_n8;
        locals.var_ty_dn9 = assign77810_e117901_d_n9;
        locals.var_ty_dn10 = assign77810_e117901_d_n10;
        locals.var_ty_dn11 = assign77810_e117901_d_n11;
        locals.var_ty_dn14 = assign77810_e117901_d_n14;

        let assign77820_e117904: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
        locals.var_guard1807 = assign77820_e117904;

        let (assign77830_e117916, assign77830_e117916_d_n0, assign77830_e117916_d_n2, assign77830_e117916_d_n4, assign77830_e117916_d_n5, assign77830_e117916_d_n6, assign77830_e117916_d_n7, assign77830_e117916_d_n8, assign77830_e117916_d_n9, assign77830_e117916_d_n10, assign77830_e117916_d_n11, assign77830_e117916_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1807 != 0.0)) {
        let assign77830_e117914: f64 = (locals.var_ty).exp();
        (assign77830_e117914, (assign77830_e117914 * locals.var_ty_dn0), (assign77830_e117914 * locals.var_ty_dn2), (assign77830_e117914 * locals.var_ty_dn4), (assign77830_e117914 * locals.var_ty_dn5), (assign77830_e117914 * locals.var_ty_dn6), (assign77830_e117914 * locals.var_ty_dn7), (assign77830_e117914 * locals.var_ty_dn8), (assign77830_e117914 * locals.var_ty_dn9), (assign77830_e117914 * locals.var_ty_dn10), (assign77830_e117914 * locals.var_ty_dn11), (assign77830_e117914 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77830_e117916;
        locals.var_t1_dn0 = assign77830_e117916_d_n0;
        locals.var_t1_dn2 = assign77830_e117916_d_n2;
        locals.var_t1_dn4 = assign77830_e117916_d_n4;
        locals.var_t1_dn5 = assign77830_e117916_d_n5;
        locals.var_t1_dn6 = assign77830_e117916_d_n6;
        locals.var_t1_dn7 = assign77830_e117916_d_n7;
        locals.var_t1_dn8 = assign77830_e117916_d_n8;
        locals.var_t1_dn9 = assign77830_e117916_d_n9;
        locals.var_t1_dn10 = assign77830_e117916_d_n10;
        locals.var_t1_dn11 = assign77830_e117916_d_n11;
        locals.var_t1_dn14 = assign77830_e117916_d_n14;

        let (assign77840_e117931, assign77840_e117931_d_n0, assign77840_e117931_d_n2, assign77840_e117931_d_n4, assign77840_e117931_d_n5, assign77840_e117931_d_n6, assign77840_e117931_d_n7, assign77840_e117931_d_n8, assign77840_e117931_d_n9, assign77840_e117931_d_n10, assign77840_e117931_d_n11, assign77840_e117931_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1807 != 0.0)) {
        let assign77840_e117926: f64 = (-locals.var_c_sb);
        let assign77840_e117928: f64 = (assign77840_e117926 * locals.var_dphi_sb);
        let assign77840_e117929: f64 = (assign77840_e117928).exp();
        (assign77840_e117929, (assign77840_e117929 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn0))), (assign77840_e117929 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn2))), (assign77840_e117929 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn4))), (assign77840_e117929 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn5))), (assign77840_e117929 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn6))), (assign77840_e117929 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn7))), (assign77840_e117929 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn8))), (assign77840_e117929 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn9))), (assign77840_e117929 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn10))), (assign77840_e117929 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn11))), (assign77840_e117929 * (((-locals.var_c_sb_dn14) * locals.var_dphi_sb) + (assign77840_e117926 * locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign77840_e117931;
        locals.var_t0_dn0 = assign77840_e117931_d_n0;
        locals.var_t0_dn2 = assign77840_e117931_d_n2;
        locals.var_t0_dn4 = assign77840_e117931_d_n4;
        locals.var_t0_dn5 = assign77840_e117931_d_n5;
        locals.var_t0_dn6 = assign77840_e117931_d_n6;
        locals.var_t0_dn7 = assign77840_e117931_d_n7;
        locals.var_t0_dn8 = assign77840_e117931_d_n8;
        locals.var_t0_dn9 = assign77840_e117931_d_n9;
        locals.var_t0_dn10 = assign77840_e117931_d_n10;
        locals.var_t0_dn11 = assign77840_e117931_d_n11;
        locals.var_t0_dn14 = assign77840_e117931_d_n14;

        let (assign77850_e117944, assign77850_e117944_d_n0, assign77850_e117944_d_n2, assign77850_e117944_d_n4, assign77850_e117944_d_n5, assign77850_e117944_d_n6, assign77850_e117944_d_n7, assign77850_e117944_d_n8, assign77850_e117944_d_n9, assign77850_e117944_d_n10, assign77850_e117944_d_n11, assign77850_e117944_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1807 != 0.0)) {
        let assign77850_e117942: f64 = (locals.var_t1 - locals.var_t0);
        (assign77850_e117942, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77850_e117944;
        locals.var_t2_dn0 = assign77850_e117944_d_n0;
        locals.var_t2_dn2 = assign77850_e117944_d_n2;
        locals.var_t2_dn4 = assign77850_e117944_d_n4;
        locals.var_t2_dn5 = assign77850_e117944_d_n5;
        locals.var_t2_dn6 = assign77850_e117944_d_n6;
        locals.var_t2_dn7 = assign77850_e117944_d_n7;
        locals.var_t2_dn8 = assign77850_e117944_d_n8;
        locals.var_t2_dn9 = assign77850_e117944_d_n9;
        locals.var_t2_dn10 = assign77850_e117944_d_n10;
        locals.var_t2_dn11 = assign77850_e117944_d_n11;
        locals.var_t2_dn14 = assign77850_e117944_d_n14;

        let (assign77860_e117960, assign77860_e117960_d_n0, assign77860_e117960_d_n2, assign77860_e117960_d_n4, assign77860_e117960_d_n5, assign77860_e117960_d_n6, assign77860_e117960_d_n7, assign77860_e117960_d_n8, assign77860_e117960_d_n9, assign77860_e117960_d_n10, assign77860_e117960_d_n11, assign77860_e117960_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1807 != 0.0)) {
        let assign77860_e117955: f64 = (1.0 + locals.var_t2);
        let assign77860_e117956: f64 = (assign77860_e117955).ln();
        let assign77860_e117958: f64 = (assign77860_e117956 / locals.var_c_sb);
        (assign77860_e117958, ((((locals.var_t2_dn0 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn14 / assign77860_e117955) * locals.var_c_sb) - (assign77860_e117956 * locals.var_c_sb_dn14)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
        locals.var_phi_b = assign77860_e117960;
        locals.var_phi_b_dn0 = assign77860_e117960_d_n0;
        locals.var_phi_b_dn2 = assign77860_e117960_d_n2;
        locals.var_phi_b_dn4 = assign77860_e117960_d_n4;
        locals.var_phi_b_dn5 = assign77860_e117960_d_n5;
        locals.var_phi_b_dn6 = assign77860_e117960_d_n6;
        locals.var_phi_b_dn7 = assign77860_e117960_d_n7;
        locals.var_phi_b_dn8 = assign77860_e117960_d_n8;
        locals.var_phi_b_dn9 = assign77860_e117960_d_n9;
        locals.var_phi_b_dn10 = assign77860_e117960_d_n10;
        locals.var_phi_b_dn11 = assign77860_e117960_d_n11;
        locals.var_phi_b_dn14 = assign77860_e117960_d_n14;

        let (assign77870_e117974, assign77870_e117974_d_n0, assign77870_e117974_d_n2, assign77870_e117974_d_n4, assign77870_e117974_d_n5, assign77870_e117974_d_n6, assign77870_e117974_d_n7, assign77870_e117974_d_n8, assign77870_e117974_d_n9, assign77870_e117974_d_n10, assign77870_e117974_d_n11, assign77870_e117974_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1807 == 0.0)) {
        let assign77870_e117972: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign77870_e117972, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11), (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
        locals.var_phi_b = assign77870_e117974;
        locals.var_phi_b_dn0 = assign77870_e117974_d_n0;
        locals.var_phi_b_dn2 = assign77870_e117974_d_n2;
        locals.var_phi_b_dn4 = assign77870_e117974_d_n4;
        locals.var_phi_b_dn5 = assign77870_e117974_d_n5;
        locals.var_phi_b_dn6 = assign77870_e117974_d_n6;
        locals.var_phi_b_dn7 = assign77870_e117974_d_n7;
        locals.var_phi_b_dn8 = assign77870_e117974_d_n8;
        locals.var_phi_b_dn9 = assign77870_e117974_d_n9;
        locals.var_phi_b_dn10 = assign77870_e117974_d_n10;
        locals.var_phi_b_dn11 = assign77870_e117974_d_n11;
        locals.var_phi_b_dn14 = assign77870_e117974_d_n14;

        let (assign77880_e117985, assign77880_e117985_d_n0, assign77880_e117985_d_n2, assign77880_e117985_d_n4, assign77880_e117985_d_n5, assign77880_e117985_d_n6, assign77880_e117985_d_n7, assign77880_e117985_d_n8, assign77880_e117985_d_n9, assign77880_e117985_d_n10, assign77880_e117985_d_n11, assign77880_e117985_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) {
        let assign77880_e117983: f64 = (locals.var_beta * locals.var_phi_b);
        (assign77880_e117983, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn14)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn14,)
    }
};
        locals.var_chib = assign77880_e117985;
        locals.var_chib_dn0 = assign77880_e117985_d_n0;
        locals.var_chib_dn2 = assign77880_e117985_d_n2;
        locals.var_chib_dn4 = assign77880_e117985_d_n4;
        locals.var_chib_dn5 = assign77880_e117985_d_n5;
        locals.var_chib_dn6 = assign77880_e117985_d_n6;
        locals.var_chib_dn7 = assign77880_e117985_d_n7;
        locals.var_chib_dn8 = assign77880_e117985_d_n8;
        locals.var_chib_dn9 = assign77880_e117985_d_n9;
        locals.var_chib_dn10 = assign77880_e117985_d_n10;
        locals.var_chib_dn11 = assign77880_e117985_d_n11;
        locals.var_chib_dn14 = assign77880_e117985_d_n14;

        let assign77890_e117989: f64 = (locals.var_chi / 100.0);
        let assign77890_e117994: f64 = if ((locals.var_chib > assign77890_e117989) && (locals.var_chib > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1808 = assign77890_e117994;

        let (assign77900_e118007,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1808 != 0.0)) {
        let assign77900_e118005: f64 = (locals.var_flg_fd_mode__blk1772 + 1.0);
        (assign77900_e118005,)
    } else {
        (locals.var_flg_fd_mode__blk1772,)
    }
};
        locals.var_flg_fd_mode__blk1772 = assign77900_e118007;

    }

    pub(super) fn stamp_transient_block_281(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign77910_e118018, assign77910_e118018_d_n0, assign77910_e118018_d_n2, assign77910_e118018_d_n4, assign77910_e118018_d_n5, assign77910_e118018_d_n6, assign77910_e118018_d_n7, assign77910_e118018_d_n8, assign77910_e118018_d_n9, assign77910_e118018_d_n10, assign77910_e118018_d_n11, assign77910_e118018_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1803 != 0.0)) && (locals.var_guard1808 != 0.0)) {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn4, locals.var_chi_1_dn5, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn8, locals.var_chi_1_dn9, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn14,)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
        locals.var_chi = assign77910_e118018;
        locals.var_chi_dn0 = assign77910_e118018_d_n0;
        locals.var_chi_dn2 = assign77910_e118018_d_n2;
        locals.var_chi_dn4 = assign77910_e118018_d_n4;
        locals.var_chi_dn5 = assign77910_e118018_d_n5;
        locals.var_chi_dn6 = assign77910_e118018_d_n6;
        locals.var_chi_dn7 = assign77910_e118018_d_n7;
        locals.var_chi_dn8 = assign77910_e118018_d_n8;
        locals.var_chi_dn9 = assign77910_e118018_d_n9;
        locals.var_chi_dn10 = assign77910_e118018_d_n10;
        locals.var_chi_dn11 = assign77910_e118018_d_n11;
        locals.var_chi_dn14 = assign77910_e118018_d_n14;

        let (assign77920_e118029, assign77920_e118029_d_n0, assign77920_e118029_d_n2, assign77920_e118029_d_n4, assign77920_e118029_d_n5, assign77920_e118029_d_n6, assign77920_e118029_d_n7, assign77920_e118029_d_n8, assign77920_e118029_d_n9, assign77920_e118029_d_n10, assign77920_e118029_d_n11, assign77920_e118029_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) {
        let assign77920_e118025: f64 = (locals.var_chi * locals.var_beta_inv);
        let assign77920_e118027: f64 = (assign77920_e118025 - locals.var_vxbgmtcl);
        (assign77920_e118027, (((locals.var_chi_dn0 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn0)) - locals.var_vxbgmtcl_dn0), (((locals.var_chi_dn2 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn2)) - locals.var_vxbgmtcl_dn2), (((locals.var_chi_dn4 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn4)) - locals.var_vxbgmtcl_dn4), (((locals.var_chi_dn5 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn5)) - locals.var_vxbgmtcl_dn5), (((locals.var_chi_dn6 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn6)) - locals.var_vxbgmtcl_dn6), (((locals.var_chi_dn7 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn7)) - locals.var_vxbgmtcl_dn7), (((locals.var_chi_dn8 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn8)) - locals.var_vxbgmtcl_dn8), (((locals.var_chi_dn9 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn9)) - locals.var_vxbgmtcl_dn9), (((locals.var_chi_dn10 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), (((locals.var_chi_dn11 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn11)) - locals.var_vxbgmtcl_dn11), (((locals.var_chi_dn14 * locals.var_beta_inv) + (locals.var_chi * locals.var_beta_inv_dn14)) - locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign77920_e118029;
        locals.var_ps0ld_dn0 = assign77920_e118029_d_n0;
        locals.var_ps0ld_dn2 = assign77920_e118029_d_n2;
        locals.var_ps0ld_dn4 = assign77920_e118029_d_n4;
        locals.var_ps0ld_dn5 = assign77920_e118029_d_n5;
        locals.var_ps0ld_dn6 = assign77920_e118029_d_n6;
        locals.var_ps0ld_dn7 = assign77920_e118029_d_n7;
        locals.var_ps0ld_dn8 = assign77920_e118029_d_n8;
        locals.var_ps0ld_dn9 = assign77920_e118029_d_n9;
        locals.var_ps0ld_dn10 = assign77920_e118029_d_n10;
        locals.var_ps0ld_dn11 = assign77920_e118029_d_n11;
        locals.var_ps0ld_dn14 = assign77920_e118029_d_n14;

        let assign77930_e118031: f64 = (locals.var_chi).abs();
        let assign77930_e118033: f64 = if assign77930_e118031 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1809 = assign77930_e118033;

        let (assign77940_e118048, assign77940_e118048_d_n0, assign77940_e118048_d_n2, assign77940_e118048_d_n4, assign77940_e118048_d_n5, assign77940_e118048_d_n6, assign77940_e118048_d_n7, assign77940_e118048_d_n8, assign77940_e118048_d_n9, assign77940_e118048_d_n10, assign77940_e118048_d_n11, assign77940_e118048_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1809 != 0.0)) {
        let assign77940_e118042: f64 = (locals.var_chi - 1.0);
        let assign77940_e118044: f64 = (-locals.var_chi);
        let assign77940_e118045: f64 = (assign77940_e118044).exp();
        let assign77940_e118046: f64 = (assign77940_e118042 + assign77940_e118045);
        (assign77940_e118046, (locals.var_chi_dn0 + (assign77940_e118045 * (-locals.var_chi_dn0))), (locals.var_chi_dn2 + (assign77940_e118045 * (-locals.var_chi_dn2))), (locals.var_chi_dn4 + (assign77940_e118045 * (-locals.var_chi_dn4))), (locals.var_chi_dn5 + (assign77940_e118045 * (-locals.var_chi_dn5))), (locals.var_chi_dn6 + (assign77940_e118045 * (-locals.var_chi_dn6))), (locals.var_chi_dn7 + (assign77940_e118045 * (-locals.var_chi_dn7))), (locals.var_chi_dn8 + (assign77940_e118045 * (-locals.var_chi_dn8))), (locals.var_chi_dn9 + (assign77940_e118045 * (-locals.var_chi_dn9))), (locals.var_chi_dn10 + (assign77940_e118045 * (-locals.var_chi_dn10))), (locals.var_chi_dn11 + (assign77940_e118045 * (-locals.var_chi_dn11))), (locals.var_chi_dn14 + (assign77940_e118045 * (-locals.var_chi_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign77940_e118048;
        locals.var_t1_dn0 = assign77940_e118048_d_n0;
        locals.var_t1_dn2 = assign77940_e118048_d_n2;
        locals.var_t1_dn4 = assign77940_e118048_d_n4;
        locals.var_t1_dn5 = assign77940_e118048_d_n5;
        locals.var_t1_dn6 = assign77940_e118048_d_n6;
        locals.var_t1_dn7 = assign77940_e118048_d_n7;
        locals.var_t1_dn8 = assign77940_e118048_d_n8;
        locals.var_t1_dn9 = assign77940_e118048_d_n9;
        locals.var_t1_dn10 = assign77940_e118048_d_n10;
        locals.var_t1_dn11 = assign77940_e118048_d_n11;
        locals.var_t1_dn14 = assign77940_e118048_d_n14;

        let (assign77950_e118058, assign77950_e118058_d_n0, assign77950_e118058_d_n2, assign77950_e118058_d_n4, assign77950_e118058_d_n5, assign77950_e118058_d_n6, assign77950_e118058_d_n7, assign77950_e118058_d_n8, assign77950_e118058_d_n9, assign77950_e118058_d_n10, assign77950_e118058_d_n11, assign77950_e118058_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1809 != 0.0)) {
        let assign77950_e118056: f64 = (locals.var_t1).sqrt();
        (assign77950_e118056, (locals.var_t1_dn0 / (2.0 * assign77950_e118056)), (locals.var_t1_dn2 / (2.0 * assign77950_e118056)), (locals.var_t1_dn4 / (2.0 * assign77950_e118056)), (locals.var_t1_dn5 / (2.0 * assign77950_e118056)), (locals.var_t1_dn6 / (2.0 * assign77950_e118056)), (locals.var_t1_dn7 / (2.0 * assign77950_e118056)), (locals.var_t1_dn8 / (2.0 * assign77950_e118056)), (locals.var_t1_dn9 / (2.0 * assign77950_e118056)), (locals.var_t1_dn10 / (2.0 * assign77950_e118056)), (locals.var_t1_dn11 / (2.0 * assign77950_e118056)), (locals.var_t1_dn14 / (2.0 * assign77950_e118056)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77950_e118058;
        locals.var_t2_dn0 = assign77950_e118058_d_n0;
        locals.var_t2_dn2 = assign77950_e118058_d_n2;
        locals.var_t2_dn4 = assign77950_e118058_d_n4;
        locals.var_t2_dn5 = assign77950_e118058_d_n5;
        locals.var_t2_dn6 = assign77950_e118058_d_n6;
        locals.var_t2_dn7 = assign77950_e118058_d_n7;
        locals.var_t2_dn8 = assign77950_e118058_d_n8;
        locals.var_t2_dn9 = assign77950_e118058_d_n9;
        locals.var_t2_dn10 = assign77950_e118058_d_n10;
        locals.var_t2_dn11 = assign77950_e118058_d_n11;
        locals.var_t2_dn14 = assign77950_e118058_d_n14;

        let (assign77970_e118089, assign77970_e118089_d_n0, assign77970_e118089_d_n2, assign77970_e118089_d_n4, assign77970_e118089_d_n5, assign77970_e118089_d_n6, assign77970_e118089_d_n7, assign77970_e118089_d_n8, assign77970_e118089_d_n9, assign77970_e118089_d_n10, assign77970_e118089_d_n11, assign77970_e118089_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1809 == 0.0)) {
        let assign77970_e118080: f64 = (0.7071067811865475 * locals.var_chi);
        let assign77970_e118084: f64 = (locals.var_chi * 0.3333333333333333);
        let assign77970_e118085: f64 = (1.0 - assign77970_e118084);
        let assign77970_e118086: f64 = (assign77970_e118085).sqrt();
        let assign77970_e118087: f64 = (assign77970_e118080 * assign77970_e118086);
        (assign77970_e118087, (((0.7071067811865475 * locals.var_chi_dn0) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn0 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn2) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn2 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn4) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn4 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn5) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn5 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn6) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn6 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn7) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn7 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn8) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn8 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn9) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn9 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn10) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn10 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn11) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn11 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))), (((0.7071067811865475 * locals.var_chi_dn14) * assign77970_e118086) + (assign77970_e118080 * ((-(locals.var_chi_dn14 * 0.3333333333333333)) / (2.0 * assign77970_e118086)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign77970_e118089;
        locals.var_t2_dn0 = assign77970_e118089_d_n0;
        locals.var_t2_dn2 = assign77970_e118089_d_n2;
        locals.var_t2_dn4 = assign77970_e118089_d_n4;
        locals.var_t2_dn5 = assign77970_e118089_d_n5;
        locals.var_t2_dn6 = assign77970_e118089_d_n6;
        locals.var_t2_dn7 = assign77970_e118089_d_n7;
        locals.var_t2_dn8 = assign77970_e118089_d_n8;
        locals.var_t2_dn9 = assign77970_e118089_d_n9;
        locals.var_t2_dn10 = assign77970_e118089_d_n10;
        locals.var_t2_dn11 = assign77970_e118089_d_n11;
        locals.var_t2_dn14 = assign77970_e118089_d_n14;

        let (assign77980_e118098, assign77980_e118098_d_n0, assign77980_e118098_d_n2, assign77980_e118098_d_n4, assign77980_e118098_d_n5, assign77980_e118098_d_n6, assign77980_e118098_d_n7, assign77980_e118098_d_n8, assign77980_e118098_d_n9, assign77980_e118098_d_n10, assign77980_e118098_d_n11, assign77980_e118098_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) {
        let assign77980_e118096: f64 = (locals.var_cnst0over_func * locals.var_t2);
        (assign77980_e118096, ((locals.var_cnst0over_func_dn0 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn10)), ((locals.var_cnst0over_func_dn11 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn11)), ((locals.var_cnst0over_func_dn14 * locals.var_t2) + (locals.var_cnst0over_func * locals.var_t2_dn14)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign77980_e118098;
        locals.var_qbuld_dn0 = assign77980_e118098_d_n0;
        locals.var_qbuld_dn2 = assign77980_e118098_d_n2;
        locals.var_qbuld_dn4 = assign77980_e118098_d_n4;
        locals.var_qbuld_dn5 = assign77980_e118098_d_n5;
        locals.var_qbuld_dn6 = assign77980_e118098_d_n6;
        locals.var_qbuld_dn7 = assign77980_e118098_d_n7;
        locals.var_qbuld_dn8 = assign77980_e118098_d_n8;
        locals.var_qbuld_dn9 = assign77980_e118098_d_n9;
        locals.var_qbuld_dn10 = assign77980_e118098_d_n10;
        locals.var_qbuld_dn11 = assign77980_e118098_d_n11;
        locals.var_qbuld_dn14 = assign77980_e118098_d_n14;

        let (assign77990_e118109, assign77990_e118109_d_n0, assign77990_e118109_d_n2, assign77990_e118109_d_n4, assign77990_e118109_d_n5, assign77990_e118109_d_n6, assign77990_e118109_d_n7, assign77990_e118109_d_n8, assign77990_e118109_d_n9, assign77990_e118109_d_n10, assign77990_e118109_d_n11, assign77990_e118109_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) {
        let assign77990_e118106: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign77990_e118107: f64 = (locals.var_cox0_func * assign77990_e118106);
        (assign77990_e118107, (locals.var_cox0_func * (-locals.var_ps0ld_dn0)), (locals.var_cox0_func * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0_func * (-locals.var_ps0ld_dn4)), (locals.var_cox0_func * (-locals.var_ps0ld_dn5)), (locals.var_cox0_func * (-locals.var_ps0ld_dn6)), (locals.var_cox0_func * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0_func * (locals.var_vgpld_dn8 - locals.var_ps0ld_dn8)), (locals.var_cox0_func * (locals.var_vgpld_dn9 - locals.var_ps0ld_dn9)), (locals.var_cox0_func * (-locals.var_ps0ld_dn10)), (locals.var_cox0_func * (-locals.var_ps0ld_dn11)), (locals.var_cox0_func * (-locals.var_ps0ld_dn14)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign77990_e118109;
        locals.var_qsuld_dn0 = assign77990_e118109_d_n0;
        locals.var_qsuld_dn2 = assign77990_e118109_d_n2;
        locals.var_qsuld_dn4 = assign77990_e118109_d_n4;
        locals.var_qsuld_dn5 = assign77990_e118109_d_n5;
        locals.var_qsuld_dn6 = assign77990_e118109_d_n6;
        locals.var_qsuld_dn7 = assign77990_e118109_d_n7;
        locals.var_qsuld_dn8 = assign77990_e118109_d_n8;
        locals.var_qsuld_dn9 = assign77990_e118109_d_n9;
        locals.var_qsuld_dn10 = assign77990_e118109_d_n10;
        locals.var_qsuld_dn11 = assign77990_e118109_d_n11;
        locals.var_qsuld_dn14 = assign77990_e118109_d_n14;

        let (assign78000_e118118, assign78000_e118118_d_n0, assign78000_e118118_d_n2, assign78000_e118118_d_n4, assign78000_e118118_d_n5, assign78000_e118118_d_n6, assign78000_e118118_d_n7, assign78000_e118118_d_n8, assign78000_e118118_d_n9, assign78000_e118118_d_n10, assign78000_e118118_d_n11, assign78000_e118118_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) {
        let assign78000_e118116: f64 = (locals.var_qbuld / locals.var_q_nsubld__blk1766);
        (assign78000_e118116, (locals.var_qbuld_dn0 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn2 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn4 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn5 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn6 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn7 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn8 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn9 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn10 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn11 / locals.var_q_nsubld__blk1766), (locals.var_qbuld_dn14 / locals.var_q_nsubld__blk1766),)
    } else {
        (locals.var_wdld0__blk1810, locals.var_wdld0__blk1810_dn0, locals.var_wdld0__blk1810_dn2, locals.var_wdld0__blk1810_dn4, locals.var_wdld0__blk1810_dn5, locals.var_wdld0__blk1810_dn6, locals.var_wdld0__blk1810_dn7, locals.var_wdld0__blk1810_dn8, locals.var_wdld0__blk1810_dn9, locals.var_wdld0__blk1810_dn10, locals.var_wdld0__blk1810_dn11, locals.var_wdld0__blk1810_dn14,)
    }
};
        locals.var_wdld0__blk1810 = assign78000_e118118;
        locals.var_wdld0__blk1810_dn0 = assign78000_e118118_d_n0;
        locals.var_wdld0__blk1810_dn2 = assign78000_e118118_d_n2;
        locals.var_wdld0__blk1810_dn4 = assign78000_e118118_d_n4;
        locals.var_wdld0__blk1810_dn5 = assign78000_e118118_d_n5;
        locals.var_wdld0__blk1810_dn6 = assign78000_e118118_d_n6;
        locals.var_wdld0__blk1810_dn7 = assign78000_e118118_d_n7;
        locals.var_wdld0__blk1810_dn8 = assign78000_e118118_d_n8;
        locals.var_wdld0__blk1810_dn9 = assign78000_e118118_d_n9;
        locals.var_wdld0__blk1810_dn10 = assign78000_e118118_d_n10;
        locals.var_wdld0__blk1810_dn11 = assign78000_e118118_d_n11;
        locals.var_wdld0__blk1810_dn14 = assign78000_e118118_d_n14;

        let assign78010_e118121: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1812 = assign78010_e118121;

        let assign78020_e118126: f64 = (locals.var_ddriftldc * 0.1);
        let assign78020_e118127: f64 = (locals.var_ddriftldc - assign78020_e118126);
        let assign78020_e118131: f64 = (locals.var_ddriftldc * 0.1);
        let assign78020_e118134: f64 = if ((locals.var_wdld0__blk1810 > assign78020_e118127) && (assign78020_e118131 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1813 = assign78020_e118134;

        let (assign78030_e118151, assign78030_e118151_d_n0, assign78030_e118151_d_n2, assign78030_e118151_d_n4, assign78030_e118151_d_n5, assign78030_e118151_d_n6, assign78030_e118151_d_n7, assign78030_e118151_d_n8, assign78030_e118151_d_n9, assign78030_e118151_d_n10, assign78030_e118151_d_n11, assign78030_e118151_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78030_e118145: f64 = (locals.var_wdld0__blk1810 - locals.var_ddriftldc);
        let assign78030_e118148: f64 = (locals.var_ddriftldc * 0.1);
        let assign78030_e118149: f64 = (assign78030_e118145 + assign78030_e118148);
        (assign78030_e118149, ((locals.var_wdld0__blk1810_dn0 - locals.var_ddriftldc_dn0) + (locals.var_ddriftldc_dn0 * 0.1)), ((locals.var_wdld0__blk1810_dn2 - locals.var_ddriftldc_dn2) + (locals.var_ddriftldc_dn2 * 0.1)), ((locals.var_wdld0__blk1810_dn4 - locals.var_ddriftldc_dn4) + (locals.var_ddriftldc_dn4 * 0.1)), ((locals.var_wdld0__blk1810_dn5 - locals.var_ddriftldc_dn5) + (locals.var_ddriftldc_dn5 * 0.1)), ((locals.var_wdld0__blk1810_dn6 - locals.var_ddriftldc_dn6) + (locals.var_ddriftldc_dn6 * 0.1)), ((locals.var_wdld0__blk1810_dn7 - locals.var_ddriftldc_dn7) + (locals.var_ddriftldc_dn7 * 0.1)), ((locals.var_wdld0__blk1810_dn8 - locals.var_ddriftldc_dn8) + (locals.var_ddriftldc_dn8 * 0.1)), ((locals.var_wdld0__blk1810_dn9 - locals.var_ddriftldc_dn9) + (locals.var_ddriftldc_dn9 * 0.1)), ((locals.var_wdld0__blk1810_dn10 - locals.var_ddriftldc_dn10) + (locals.var_ddriftldc_dn10 * 0.1)), ((locals.var_wdld0__blk1810_dn11 - locals.var_ddriftldc_dn11) + (locals.var_ddriftldc_dn11 * 0.1)), ((locals.var_wdld0__blk1810_dn14 - locals.var_ddriftldc_dn14) + (locals.var_ddriftldc_dn14 * 0.1)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign78030_e118151;
        locals.var_tmf1_dn0 = assign78030_e118151_d_n0;
        locals.var_tmf1_dn2 = assign78030_e118151_d_n2;
        locals.var_tmf1_dn4 = assign78030_e118151_d_n4;
        locals.var_tmf1_dn5 = assign78030_e118151_d_n5;
        locals.var_tmf1_dn6 = assign78030_e118151_d_n6;
        locals.var_tmf1_dn7 = assign78030_e118151_d_n7;
        locals.var_tmf1_dn8 = assign78030_e118151_d_n8;
        locals.var_tmf1_dn9 = assign78030_e118151_d_n9;
        locals.var_tmf1_dn10 = assign78030_e118151_d_n10;
        locals.var_tmf1_dn11 = assign78030_e118151_d_n11;
        locals.var_tmf1_dn14 = assign78030_e118151_d_n14;

        let (assign78040_e118164, assign78040_e118164_d_n0, assign78040_e118164_d_n2, assign78040_e118164_d_n4, assign78040_e118164_d_n5, assign78040_e118164_d_n6, assign78040_e118164_d_n7, assign78040_e118164_d_n8, assign78040_e118164_d_n9, assign78040_e118164_d_n10, assign78040_e118164_d_n11, assign78040_e118164_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78040_e118162: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign78040_e118162, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign78040_e118164;
        locals.var_x2_dn0 = assign78040_e118164_d_n0;
        locals.var_x2_dn2 = assign78040_e118164_d_n2;
        locals.var_x2_dn4 = assign78040_e118164_d_n4;
        locals.var_x2_dn5 = assign78040_e118164_d_n5;
        locals.var_x2_dn6 = assign78040_e118164_d_n6;
        locals.var_x2_dn7 = assign78040_e118164_d_n7;
        locals.var_x2_dn8 = assign78040_e118164_d_n8;
        locals.var_x2_dn9 = assign78040_e118164_d_n9;
        locals.var_x2_dn10 = assign78040_e118164_d_n10;
        locals.var_x2_dn11 = assign78040_e118164_d_n11;
        locals.var_x2_dn14 = assign78040_e118164_d_n14;

        let (assign78050_e118181, assign78050_e118181_d_n0, assign78050_e118181_d_n2, assign78050_e118181_d_n4, assign78050_e118181_d_n5, assign78050_e118181_d_n6, assign78050_e118181_d_n7, assign78050_e118181_d_n8, assign78050_e118181_d_n9, assign78050_e118181_d_n10, assign78050_e118181_d_n11, assign78050_e118181_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78050_e118175: f64 = (locals.var_ddriftldc * 0.1);
        let assign78050_e118178: f64 = (locals.var_ddriftldc * 0.1);
        let assign78050_e118179: f64 = (assign78050_e118175 * assign78050_e118178);
        (assign78050_e118179, (((locals.var_ddriftldc_dn0 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn0 * 0.1))), (((locals.var_ddriftldc_dn2 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn2 * 0.1))), (((locals.var_ddriftldc_dn4 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn4 * 0.1))), (((locals.var_ddriftldc_dn5 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn5 * 0.1))), (((locals.var_ddriftldc_dn6 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn6 * 0.1))), (((locals.var_ddriftldc_dn7 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn7 * 0.1))), (((locals.var_ddriftldc_dn8 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn8 * 0.1))), (((locals.var_ddriftldc_dn9 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn9 * 0.1))), (((locals.var_ddriftldc_dn10 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn10 * 0.1))), (((locals.var_ddriftldc_dn11 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn11 * 0.1))), (((locals.var_ddriftldc_dn14 * 0.1) * assign78050_e118178) + (assign78050_e118175 * (locals.var_ddriftldc_dn14 * 0.1))),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign78050_e118181;
        locals.var_xmax2_dn0 = assign78050_e118181_d_n0;
        locals.var_xmax2_dn2 = assign78050_e118181_d_n2;
        locals.var_xmax2_dn4 = assign78050_e118181_d_n4;
        locals.var_xmax2_dn5 = assign78050_e118181_d_n5;
        locals.var_xmax2_dn6 = assign78050_e118181_d_n6;
        locals.var_xmax2_dn7 = assign78050_e118181_d_n7;
        locals.var_xmax2_dn8 = assign78050_e118181_d_n8;
        locals.var_xmax2_dn9 = assign78050_e118181_d_n9;
        locals.var_xmax2_dn10 = assign78050_e118181_d_n10;
        locals.var_xmax2_dn11 = assign78050_e118181_d_n11;
        locals.var_xmax2_dn14 = assign78050_e118181_d_n14;

        let (assign78060_e118192, assign78060_e118192_d_n0, assign78060_e118192_d_n2, assign78060_e118192_d_n4, assign78060_e118192_d_n5, assign78060_e118192_d_n6, assign78060_e118192_d_n7, assign78060_e118192_d_n8, assign78060_e118192_d_n9, assign78060_e118192_d_n10, assign78060_e118192_d_n11, assign78060_e118192_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign78060_e118192;
        locals.var_xp_dn0 = assign78060_e118192_d_n0;
        locals.var_xp_dn2 = assign78060_e118192_d_n2;
        locals.var_xp_dn4 = assign78060_e118192_d_n4;
        locals.var_xp_dn5 = assign78060_e118192_d_n5;
        locals.var_xp_dn6 = assign78060_e118192_d_n6;
        locals.var_xp_dn7 = assign78060_e118192_d_n7;
        locals.var_xp_dn8 = assign78060_e118192_d_n8;
        locals.var_xp_dn9 = assign78060_e118192_d_n9;
        locals.var_xp_dn10 = assign78060_e118192_d_n10;
        locals.var_xp_dn11 = assign78060_e118192_d_n11;
        locals.var_xp_dn14 = assign78060_e118192_d_n14;

        let (assign78070_e118203, assign78070_e118203_d_n0, assign78070_e118203_d_n2, assign78070_e118203_d_n4, assign78070_e118203_d_n5, assign78070_e118203_d_n6, assign78070_e118203_d_n7, assign78070_e118203_d_n8, assign78070_e118203_d_n9, assign78070_e118203_d_n10, assign78070_e118203_d_n11, assign78070_e118203_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign78070_e118203;
        locals.var_xmp_dn0 = assign78070_e118203_d_n0;
        locals.var_xmp_dn2 = assign78070_e118203_d_n2;
        locals.var_xmp_dn4 = assign78070_e118203_d_n4;
        locals.var_xmp_dn5 = assign78070_e118203_d_n5;
        locals.var_xmp_dn6 = assign78070_e118203_d_n6;
        locals.var_xmp_dn7 = assign78070_e118203_d_n7;
        locals.var_xmp_dn8 = assign78070_e118203_d_n8;
        locals.var_xmp_dn9 = assign78070_e118203_d_n9;
        locals.var_xmp_dn10 = assign78070_e118203_d_n10;
        locals.var_xmp_dn11 = assign78070_e118203_d_n11;
        locals.var_xmp_dn14 = assign78070_e118203_d_n14;

        let (assign78080_e118214,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign78080_e118214;

        let (assign78090_e118225,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78090_e118225;

        let (assign78100_e118236, assign78100_e118236_d_n0, assign78100_e118236_d_n2, assign78100_e118236_d_n4, assign78100_e118236_d_n5, assign78100_e118236_d_n6, assign78100_e118236_d_n7, assign78100_e118236_d_n8, assign78100_e118236_d_n9, assign78100_e118236_d_n10, assign78100_e118236_d_n11, assign78100_e118236_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign78100_e118236;
        locals.var_arg_dn0 = assign78100_e118236_d_n0;
        locals.var_arg_dn2 = assign78100_e118236_d_n2;
        locals.var_arg_dn4 = assign78100_e118236_d_n4;
        locals.var_arg_dn5 = assign78100_e118236_d_n5;
        locals.var_arg_dn6 = assign78100_e118236_d_n6;
        locals.var_arg_dn7 = assign78100_e118236_d_n7;
        locals.var_arg_dn8 = assign78100_e118236_d_n8;
        locals.var_arg_dn9 = assign78100_e118236_d_n9;
        locals.var_arg_dn10 = assign78100_e118236_d_n10;
        locals.var_arg_dn11 = assign78100_e118236_d_n11;
        locals.var_arg_dn14 = assign78100_e118236_d_n14;

        let (assign78110_e118247, assign78110_e118247_d_n0, assign78110_e118247_d_n2, assign78110_e118247_d_n4, assign78110_e118247_d_n5, assign78110_e118247_d_n6, assign78110_e118247_d_n7, assign78110_e118247_d_n8, assign78110_e118247_d_n9, assign78110_e118247_d_n10, assign78110_e118247_d_n11, assign78110_e118247_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign78110_e118247;
        locals.var_dnm_dn0 = assign78110_e118247_d_n0;
        locals.var_dnm_dn2 = assign78110_e118247_d_n2;
        locals.var_dnm_dn4 = assign78110_e118247_d_n4;
        locals.var_dnm_dn5 = assign78110_e118247_d_n5;
        locals.var_dnm_dn6 = assign78110_e118247_d_n6;
        locals.var_dnm_dn7 = assign78110_e118247_d_n7;
        locals.var_dnm_dn8 = assign78110_e118247_d_n8;
        locals.var_dnm_dn9 = assign78110_e118247_d_n9;
        locals.var_dnm_dn10 = assign78110_e118247_d_n10;
        locals.var_dnm_dn11 = assign78110_e118247_d_n11;
        locals.var_dnm_dn14 = assign78110_e118247_d_n14;

        let (assign78120_e118260, assign78120_e118260_d_n0, assign78120_e118260_d_n2, assign78120_e118260_d_n4, assign78120_e118260_d_n5, assign78120_e118260_d_n6, assign78120_e118260_d_n7, assign78120_e118260_d_n8, assign78120_e118260_d_n9, assign78120_e118260_d_n10, assign78120_e118260_d_n11, assign78120_e118260_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78120_e118258: f64 = (locals.var_xp * locals.var_x2);
        (assign78120_e118258, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign78120_e118260;
        locals.var_xp_dn0 = assign78120_e118260_d_n0;
        locals.var_xp_dn2 = assign78120_e118260_d_n2;
        locals.var_xp_dn4 = assign78120_e118260_d_n4;
        locals.var_xp_dn5 = assign78120_e118260_d_n5;
        locals.var_xp_dn6 = assign78120_e118260_d_n6;
        locals.var_xp_dn7 = assign78120_e118260_d_n7;
        locals.var_xp_dn8 = assign78120_e118260_d_n8;
        locals.var_xp_dn9 = assign78120_e118260_d_n9;
        locals.var_xp_dn10 = assign78120_e118260_d_n10;
        locals.var_xp_dn11 = assign78120_e118260_d_n11;
        locals.var_xp_dn14 = assign78120_e118260_d_n14;

        let (assign78130_e118273, assign78130_e118273_d_n0, assign78130_e118273_d_n2, assign78130_e118273_d_n4, assign78130_e118273_d_n5, assign78130_e118273_d_n6, assign78130_e118273_d_n7, assign78130_e118273_d_n8, assign78130_e118273_d_n9, assign78130_e118273_d_n10, assign78130_e118273_d_n11, assign78130_e118273_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78130_e118271: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign78130_e118271, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign78130_e118273;
        locals.var_xmp_dn0 = assign78130_e118273_d_n0;
        locals.var_xmp_dn2 = assign78130_e118273_d_n2;
        locals.var_xmp_dn4 = assign78130_e118273_d_n4;
        locals.var_xmp_dn5 = assign78130_e118273_d_n5;
        locals.var_xmp_dn6 = assign78130_e118273_d_n6;
        locals.var_xmp_dn7 = assign78130_e118273_d_n7;
        locals.var_xmp_dn8 = assign78130_e118273_d_n8;
        locals.var_xmp_dn9 = assign78130_e118273_d_n9;
        locals.var_xmp_dn10 = assign78130_e118273_d_n10;
        locals.var_xmp_dn11 = assign78130_e118273_d_n11;
        locals.var_xmp_dn14 = assign78130_e118273_d_n14;

        let (assign78140_e118286, assign78140_e118286_d_n0, assign78140_e118286_d_n2, assign78140_e118286_d_n4, assign78140_e118286_d_n5, assign78140_e118286_d_n6, assign78140_e118286_d_n7, assign78140_e118286_d_n8, assign78140_e118286_d_n9, assign78140_e118286_d_n10, assign78140_e118286_d_n11, assign78140_e118286_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78140_e118284: f64 = (locals.var_xp * locals.var_x2);
        (assign78140_e118284, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign78140_e118286;
        locals.var_xp_dn0 = assign78140_e118286_d_n0;
        locals.var_xp_dn2 = assign78140_e118286_d_n2;
        locals.var_xp_dn4 = assign78140_e118286_d_n4;
        locals.var_xp_dn5 = assign78140_e118286_d_n5;
        locals.var_xp_dn6 = assign78140_e118286_d_n6;
        locals.var_xp_dn7 = assign78140_e118286_d_n7;
        locals.var_xp_dn8 = assign78140_e118286_d_n8;
        locals.var_xp_dn9 = assign78140_e118286_d_n9;
        locals.var_xp_dn10 = assign78140_e118286_d_n10;
        locals.var_xp_dn11 = assign78140_e118286_d_n11;
        locals.var_xp_dn14 = assign78140_e118286_d_n14;

        let (assign78150_e118299, assign78150_e118299_d_n0, assign78150_e118299_d_n2, assign78150_e118299_d_n4, assign78150_e118299_d_n5, assign78150_e118299_d_n6, assign78150_e118299_d_n7, assign78150_e118299_d_n8, assign78150_e118299_d_n9, assign78150_e118299_d_n10, assign78150_e118299_d_n11, assign78150_e118299_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78150_e118297: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign78150_e118297, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign78150_e118299;
        locals.var_xmp_dn0 = assign78150_e118299_d_n0;
        locals.var_xmp_dn2 = assign78150_e118299_d_n2;
        locals.var_xmp_dn4 = assign78150_e118299_d_n4;
        locals.var_xmp_dn5 = assign78150_e118299_d_n5;
        locals.var_xmp_dn6 = assign78150_e118299_d_n6;
        locals.var_xmp_dn7 = assign78150_e118299_d_n7;
        locals.var_xmp_dn8 = assign78150_e118299_d_n8;
        locals.var_xmp_dn9 = assign78150_e118299_d_n9;
        locals.var_xmp_dn10 = assign78150_e118299_d_n10;
        locals.var_xmp_dn11 = assign78150_e118299_d_n11;
        locals.var_xmp_dn14 = assign78150_e118299_d_n14;

        let (assign78160_e118312, assign78160_e118312_d_n0, assign78160_e118312_d_n2, assign78160_e118312_d_n4, assign78160_e118312_d_n5, assign78160_e118312_d_n6, assign78160_e118312_d_n7, assign78160_e118312_d_n8, assign78160_e118312_d_n9, assign78160_e118312_d_n10, assign78160_e118312_d_n11, assign78160_e118312_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78160_e118310: f64 = (locals.var_xp + locals.var_xmp);
        (assign78160_e118310, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign78160_e118312;
        locals.var_arg_dn0 = assign78160_e118312_d_n0;
        locals.var_arg_dn2 = assign78160_e118312_d_n2;
        locals.var_arg_dn4 = assign78160_e118312_d_n4;
        locals.var_arg_dn5 = assign78160_e118312_d_n5;
        locals.var_arg_dn6 = assign78160_e118312_d_n6;
        locals.var_arg_dn7 = assign78160_e118312_d_n7;
        locals.var_arg_dn8 = assign78160_e118312_d_n8;
        locals.var_arg_dn9 = assign78160_e118312_d_n9;
        locals.var_arg_dn10 = assign78160_e118312_d_n10;
        locals.var_arg_dn11 = assign78160_e118312_d_n11;
        locals.var_arg_dn14 = assign78160_e118312_d_n14;

        let (assign78170_e118323, assign78170_e118323_d_n0, assign78170_e118323_d_n2, assign78170_e118323_d_n4, assign78170_e118323_d_n5, assign78170_e118323_d_n6, assign78170_e118323_d_n7, assign78170_e118323_d_n8, assign78170_e118323_d_n9, assign78170_e118323_d_n10, assign78170_e118323_d_n11, assign78170_e118323_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign78170_e118323;
        locals.var_dnm_dn0 = assign78170_e118323_d_n0;
        locals.var_dnm_dn2 = assign78170_e118323_d_n2;
        locals.var_dnm_dn4 = assign78170_e118323_d_n4;
        locals.var_dnm_dn5 = assign78170_e118323_d_n5;
        locals.var_dnm_dn6 = assign78170_e118323_d_n6;
        locals.var_dnm_dn7 = assign78170_e118323_d_n7;
        locals.var_dnm_dn8 = assign78170_e118323_d_n8;
        locals.var_dnm_dn9 = assign78170_e118323_d_n9;
        locals.var_dnm_dn10 = assign78170_e118323_d_n10;
        locals.var_dnm_dn11 = assign78170_e118323_d_n11;
        locals.var_dnm_dn14 = assign78170_e118323_d_n14;

        let assign78180_e118338: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1814 = assign78180_e118338;

        let assign78190_e118341: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1815 = assign78190_e118341;

        let (assign78200_e118356,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) && (locals.var_guard1814 != 0.0)) && (locals.var_guard1815 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78200_e118356;

        let assign78210_e118359: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1816 = assign78210_e118359;

        let (assign78220_e118377,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) && (locals.var_guard1814 != 0.0)) && (locals.var_guard1815 == 0.0)) && (locals.var_guard1816 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78220_e118377;

        let assign78230_e118380: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1817 = assign78230_e118380;

    }

    pub(super) fn stamp_transient_block_282(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign78240_e118401,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) && (locals.var_guard1814 != 0.0)) && (locals.var_guard1815 == 0.0)) && (locals.var_guard1816 == 0.0)) && (locals.var_guard1817 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78240_e118401;

        let assign78250_e118404: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1818 = assign78250_e118404;

        let (assign78260_e118428,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) && (locals.var_guard1814 != 0.0)) && (locals.var_guard1815 == 0.0)) && (locals.var_guard1816 == 0.0)) && (locals.var_guard1817 == 0.0)) && (locals.var_guard1818 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78260_e118428;

        let (assign78270_e118441,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) && (locals.var_guard1814 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign78270_e118441;

        let mut assign78280_loop_guard: usize = 0;
        while {
            let assign78280_cond_e118455: f64 = if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) && (locals.var_guard1814 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign78280_cond_e118455 != 0.0
        } {
            assign78280_loop_guard += 1;
            assert!(assign78280_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign78280_body0_e118469, assign78280_body0_e118469_d_n0, assign78280_body0_e118469_d_n2, assign78280_body0_e118469_d_n4, assign78280_body0_e118469_d_n5, assign78280_body0_e118469_d_n6, assign78280_body0_e118469_d_n7, assign78280_body0_e118469_d_n8, assign78280_body0_e118469_d_n9, assign78280_body0_e118469_d_n10, assign78280_body0_e118469_d_n11, assign78280_body0_e118469_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) && (locals.var_guard1814 != 0.0)) {
        let assign78280_body0_e118467: f64 = (locals.var_dnm).sqrt();
        (assign78280_body0_e118467, (locals.var_dnm_dn0 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn2 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn4 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn5 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn6 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn7 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn8 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn9 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn10 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn11 / (2.0 * assign78280_body0_e118467)), (locals.var_dnm_dn14 / (2.0 * assign78280_body0_e118467)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign78280_body0_e118469;
            locals.var_dnm_dn0 = assign78280_body0_e118469_d_n0;
            locals.var_dnm_dn2 = assign78280_body0_e118469_d_n2;
            locals.var_dnm_dn4 = assign78280_body0_e118469_d_n4;
            locals.var_dnm_dn5 = assign78280_body0_e118469_d_n5;
            locals.var_dnm_dn6 = assign78280_body0_e118469_d_n6;
            locals.var_dnm_dn7 = assign78280_body0_e118469_d_n7;
            locals.var_dnm_dn8 = assign78280_body0_e118469_d_n8;
            locals.var_dnm_dn9 = assign78280_body0_e118469_d_n9;
            locals.var_dnm_dn10 = assign78280_body0_e118469_d_n10;
            locals.var_dnm_dn11 = assign78280_body0_e118469_d_n11;
            locals.var_dnm_dn14 = assign78280_body0_e118469_d_n14;
            let (assign78280_body1_e118484,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) && (locals.var_guard1814 != 0.0)) {
        let assign78280_body1_e118482: f64 = (locals.var_m0 + 1.0);
        (assign78280_body1_e118482,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign78280_body1_e118484;
        }

        let (assign78290_e118509, assign78290_e118509_d_n0, assign78290_e118509_d_n2, assign78290_e118509_d_n4, assign78290_e118509_d_n5, assign78290_e118509_d_n6, assign78290_e118509_d_n7, assign78290_e118509_d_n8, assign78290_e118509_d_n9, assign78290_e118509_d_n10, assign78290_e118509_d_n11, assign78290_e118509_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) && (locals.var_guard1814 == 0.0)) {
        let (assign78290_e118507, assign78290_e118507_d_n0, assign78290_e118507_d_n2, assign78290_e118507_d_n4, assign78290_e118507_d_n5, assign78290_e118507_d_n6, assign78290_e118507_d_n7, assign78290_e118507_d_n8, assign78290_e118507_d_n9, assign78290_e118507_d_n10, assign78290_e118507_d_n11, assign78290_e118507_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign78290_e118504: f64 = (2.0 * 2.0);
                let assign78290_e118505: f64 = (1.0 / assign78290_e118504);
                let assign78290_e118506: f64 = (locals.var_dnm).powf(assign78290_e118505);
                (assign78290_e118506, if 0.0 == 0.0 && ((assign78290_e118505) as f64).is_finite() && ((assign78290_e118505) as f64).fract() == 0.0 { if assign78290_e118505 == 0.0 { 0.0 } else { (assign78290_e118505 * ((locals.var_dnm).powf(assign78290_e118505 - 1.0) * locals.var_dnm_dn0)) } } else { (assign78290_e118506 * (assign78290_e118505 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78290_e118505) as f64).is_finite() && ((assign78290_e118505) as f64).fract() == 0.0 { if assign78290_e118505 == 0.0 { 0.0 } else { (assign78290_e118505 * ((locals.var_dnm).powf(assign78290_e118505 - 1.0) * locals.var_dnm_dn2)) } } else { (assign78290_e118506 * (assign78290_e118505 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78290_e118505) as f64).is_finite() && ((assign78290_e118505) as f64).fract() == 0.0 { if assign78290_e118505 == 0.0 { 0.0 } else { (assign78290_e118505 * ((locals.var_dnm).powf(assign78290_e118505 - 1.0) * locals.var_dnm_dn4)) } } else { (assign78290_e118506 * (assign78290_e118505 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78290_e118505) as f64).is_finite() && ((assign78290_e118505) as f64).fract() == 0.0 { if assign78290_e118505 == 0.0 { 0.0 } else { (assign78290_e118505 * ((locals.var_dnm).powf(assign78290_e118505 - 1.0) * locals.var_dnm_dn5)) } } else { (assign78290_e118506 * (assign78290_e118505 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78290_e118505) as f64).is_finite() && ((assign78290_e118505) as f64).fract() == 0.0 { if assign78290_e118505 == 0.0 { 0.0 } else { (assign78290_e118505 * ((locals.var_dnm).powf(assign78290_e118505 - 1.0) * locals.var_dnm_dn6)) } } else { (assign78290_e118506 * (assign78290_e118505 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78290_e118505) as f64).is_finite() && ((assign78290_e118505) as f64).fract() == 0.0 { if assign78290_e118505 == 0.0 { 0.0 } else { (assign78290_e118505 * ((locals.var_dnm).powf(assign78290_e118505 - 1.0) * locals.var_dnm_dn7)) } } else { (assign78290_e118506 * (assign78290_e118505 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78290_e118505) as f64).is_finite() && ((assign78290_e118505) as f64).fract() == 0.0 { if assign78290_e118505 == 0.0 { 0.0 } else { (assign78290_e118505 * ((locals.var_dnm).powf(assign78290_e118505 - 1.0) * locals.var_dnm_dn8)) } } else { (assign78290_e118506 * (assign78290_e118505 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78290_e118505) as f64).is_finite() && ((assign78290_e118505) as f64).fract() == 0.0 { if assign78290_e118505 == 0.0 { 0.0 } else { (assign78290_e118505 * ((locals.var_dnm).powf(assign78290_e118505 - 1.0) * locals.var_dnm_dn9)) } } else { (assign78290_e118506 * (assign78290_e118505 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78290_e118505) as f64).is_finite() && ((assign78290_e118505) as f64).fract() == 0.0 { if assign78290_e118505 == 0.0 { 0.0 } else { (assign78290_e118505 * ((locals.var_dnm).powf(assign78290_e118505 - 1.0) * locals.var_dnm_dn10)) } } else { (assign78290_e118506 * (assign78290_e118505 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78290_e118505) as f64).is_finite() && ((assign78290_e118505) as f64).fract() == 0.0 { if assign78290_e118505 == 0.0 { 0.0 } else { (assign78290_e118505 * ((locals.var_dnm).powf(assign78290_e118505 - 1.0) * locals.var_dnm_dn11)) } } else { (assign78290_e118506 * (assign78290_e118505 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78290_e118505) as f64).is_finite() && ((assign78290_e118505) as f64).fract() == 0.0 { if assign78290_e118505 == 0.0 { 0.0 } else { (assign78290_e118505 * ((locals.var_dnm).powf(assign78290_e118505 - 1.0) * locals.var_dnm_dn14)) } } else { (assign78290_e118506 * (assign78290_e118505 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign78290_e118507, assign78290_e118507_d_n0, assign78290_e118507_d_n2, assign78290_e118507_d_n4, assign78290_e118507_d_n5, assign78290_e118507_d_n6, assign78290_e118507_d_n7, assign78290_e118507_d_n8, assign78290_e118507_d_n9, assign78290_e118507_d_n10, assign78290_e118507_d_n11, assign78290_e118507_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign78290_e118509;
        locals.var_dnm_dn0 = assign78290_e118509_d_n0;
        locals.var_dnm_dn2 = assign78290_e118509_d_n2;
        locals.var_dnm_dn4 = assign78290_e118509_d_n4;
        locals.var_dnm_dn5 = assign78290_e118509_d_n5;
        locals.var_dnm_dn6 = assign78290_e118509_d_n6;
        locals.var_dnm_dn7 = assign78290_e118509_d_n7;
        locals.var_dnm_dn8 = assign78290_e118509_d_n8;
        locals.var_dnm_dn9 = assign78290_e118509_d_n9;
        locals.var_dnm_dn10 = assign78290_e118509_d_n10;
        locals.var_dnm_dn11 = assign78290_e118509_d_n11;
        locals.var_dnm_dn14 = assign78290_e118509_d_n14;

        let (assign78300_e118522, assign78300_e118522_d_n0, assign78300_e118522_d_n2, assign78300_e118522_d_n4, assign78300_e118522_d_n5, assign78300_e118522_d_n6, assign78300_e118522_d_n7, assign78300_e118522_d_n8, assign78300_e118522_d_n9, assign78300_e118522_d_n10, assign78300_e118522_d_n11, assign78300_e118522_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78300_e118520: f64 = (1.0 / locals.var_dnm);
        (assign78300_e118520, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign78300_e118522;
        locals.var_dnm_dn0 = assign78300_e118522_d_n0;
        locals.var_dnm_dn2 = assign78300_e118522_d_n2;
        locals.var_dnm_dn4 = assign78300_e118522_d_n4;
        locals.var_dnm_dn5 = assign78300_e118522_d_n5;
        locals.var_dnm_dn6 = assign78300_e118522_d_n6;
        locals.var_dnm_dn7 = assign78300_e118522_d_n7;
        locals.var_dnm_dn8 = assign78300_e118522_d_n8;
        locals.var_dnm_dn9 = assign78300_e118522_d_n9;
        locals.var_dnm_dn10 = assign78300_e118522_d_n10;
        locals.var_dnm_dn11 = assign78300_e118522_d_n11;
        locals.var_dnm_dn14 = assign78300_e118522_d_n14;

        let (assign78310_e118539, assign78310_e118539_d_n0, assign78310_e118539_d_n2, assign78310_e118539_d_n4, assign78310_e118539_d_n5, assign78310_e118539_d_n6, assign78310_e118539_d_n7, assign78310_e118539_d_n8, assign78310_e118539_d_n9, assign78310_e118539_d_n10, assign78310_e118539_d_n11, assign78310_e118539_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78310_e118534: f64 = (locals.var_ddriftldc * 0.1);
        let assign78310_e118535: f64 = (locals.var_tmf1 * assign78310_e118534);
        let assign78310_e118537: f64 = (assign78310_e118535 * locals.var_dnm);
        (assign78310_e118537, ((((locals.var_tmf1_dn0 * assign78310_e118534) + (locals.var_tmf1 * (locals.var_ddriftldc_dn0 * 0.1))) * locals.var_dnm) + (assign78310_e118535 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * assign78310_e118534) + (locals.var_tmf1 * (locals.var_ddriftldc_dn2 * 0.1))) * locals.var_dnm) + (assign78310_e118535 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * assign78310_e118534) + (locals.var_tmf1 * (locals.var_ddriftldc_dn4 * 0.1))) * locals.var_dnm) + (assign78310_e118535 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * assign78310_e118534) + (locals.var_tmf1 * (locals.var_ddriftldc_dn5 * 0.1))) * locals.var_dnm) + (assign78310_e118535 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * assign78310_e118534) + (locals.var_tmf1 * (locals.var_ddriftldc_dn6 * 0.1))) * locals.var_dnm) + (assign78310_e118535 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * assign78310_e118534) + (locals.var_tmf1 * (locals.var_ddriftldc_dn7 * 0.1))) * locals.var_dnm) + (assign78310_e118535 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * assign78310_e118534) + (locals.var_tmf1 * (locals.var_ddriftldc_dn8 * 0.1))) * locals.var_dnm) + (assign78310_e118535 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * assign78310_e118534) + (locals.var_tmf1 * (locals.var_ddriftldc_dn9 * 0.1))) * locals.var_dnm) + (assign78310_e118535 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * assign78310_e118534) + (locals.var_tmf1 * (locals.var_ddriftldc_dn10 * 0.1))) * locals.var_dnm) + (assign78310_e118535 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * assign78310_e118534) + (locals.var_tmf1 * (locals.var_ddriftldc_dn11 * 0.1))) * locals.var_dnm) + (assign78310_e118535 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * assign78310_e118534) + (locals.var_tmf1 * (locals.var_ddriftldc_dn14 * 0.1))) * locals.var_dnm) + (assign78310_e118535 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign78310_e118539;
        locals.var_tmf0_dn0 = assign78310_e118539_d_n0;
        locals.var_tmf0_dn2 = assign78310_e118539_d_n2;
        locals.var_tmf0_dn4 = assign78310_e118539_d_n4;
        locals.var_tmf0_dn5 = assign78310_e118539_d_n5;
        locals.var_tmf0_dn6 = assign78310_e118539_d_n6;
        locals.var_tmf0_dn7 = assign78310_e118539_d_n7;
        locals.var_tmf0_dn8 = assign78310_e118539_d_n8;
        locals.var_tmf0_dn9 = assign78310_e118539_d_n9;
        locals.var_tmf0_dn10 = assign78310_e118539_d_n10;
        locals.var_tmf0_dn11 = assign78310_e118539_d_n11;
        locals.var_tmf0_dn14 = assign78310_e118539_d_n14;

        let (assign78320_e118558, assign78320_e118558_d_n0, assign78320_e118558_d_n2, assign78320_e118558_d_n4, assign78320_e118558_d_n5, assign78320_e118558_d_n6, assign78320_e118558_d_n7, assign78320_e118558_d_n8, assign78320_e118558_d_n9, assign78320_e118558_d_n10, assign78320_e118558_d_n11, assign78320_e118558_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78320_e118550: f64 = (locals.var_ddriftldc * 0.1);
        let assign78320_e118552: f64 = (assign78320_e118550 * locals.var_xmp);
        let assign78320_e118554: f64 = (assign78320_e118552 * locals.var_dnm);
        let assign78320_e118556: f64 = (assign78320_e118554 / locals.var_arg);
        (assign78320_e118556, ((((((((locals.var_ddriftldc_dn0 * 0.1) * locals.var_xmp) + (assign78320_e118550 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign78320_e118552 * locals.var_dnm_dn0)) * locals.var_arg) - (assign78320_e118554 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn2 * 0.1) * locals.var_xmp) + (assign78320_e118550 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign78320_e118552 * locals.var_dnm_dn2)) * locals.var_arg) - (assign78320_e118554 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn4 * 0.1) * locals.var_xmp) + (assign78320_e118550 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign78320_e118552 * locals.var_dnm_dn4)) * locals.var_arg) - (assign78320_e118554 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn5 * 0.1) * locals.var_xmp) + (assign78320_e118550 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign78320_e118552 * locals.var_dnm_dn5)) * locals.var_arg) - (assign78320_e118554 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn6 * 0.1) * locals.var_xmp) + (assign78320_e118550 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign78320_e118552 * locals.var_dnm_dn6)) * locals.var_arg) - (assign78320_e118554 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn7 * 0.1) * locals.var_xmp) + (assign78320_e118550 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign78320_e118552 * locals.var_dnm_dn7)) * locals.var_arg) - (assign78320_e118554 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn8 * 0.1) * locals.var_xmp) + (assign78320_e118550 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign78320_e118552 * locals.var_dnm_dn8)) * locals.var_arg) - (assign78320_e118554 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn9 * 0.1) * locals.var_xmp) + (assign78320_e118550 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign78320_e118552 * locals.var_dnm_dn9)) * locals.var_arg) - (assign78320_e118554 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn10 * 0.1) * locals.var_xmp) + (assign78320_e118550 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign78320_e118552 * locals.var_dnm_dn10)) * locals.var_arg) - (assign78320_e118554 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn11 * 0.1) * locals.var_xmp) + (assign78320_e118550 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign78320_e118552 * locals.var_dnm_dn11)) * locals.var_arg) - (assign78320_e118554 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((((locals.var_ddriftldc_dn14 * 0.1) * locals.var_xmp) + (assign78320_e118550 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign78320_e118552 * locals.var_dnm_dn14)) * locals.var_arg) - (assign78320_e118554 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign78320_e118558;
        locals.var_t0_dn0 = assign78320_e118558_d_n0;
        locals.var_t0_dn2 = assign78320_e118558_d_n2;
        locals.var_t0_dn4 = assign78320_e118558_d_n4;
        locals.var_t0_dn5 = assign78320_e118558_d_n5;
        locals.var_t0_dn6 = assign78320_e118558_d_n6;
        locals.var_t0_dn7 = assign78320_e118558_d_n7;
        locals.var_t0_dn8 = assign78320_e118558_d_n8;
        locals.var_t0_dn9 = assign78320_e118558_d_n9;
        locals.var_t0_dn10 = assign78320_e118558_d_n10;
        locals.var_t0_dn11 = assign78320_e118558_d_n11;
        locals.var_t0_dn14 = assign78320_e118558_d_n14;

        let (assign78330_e118575, assign78330_e118575_d_n0, assign78330_e118575_d_n2, assign78330_e118575_d_n4, assign78330_e118575_d_n5, assign78330_e118575_d_n6, assign78330_e118575_d_n7, assign78330_e118575_d_n8, assign78330_e118575_d_n9, assign78330_e118575_d_n10, assign78330_e118575_d_n11, assign78330_e118575_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        let assign78330_e118570: f64 = (locals.var_ddriftldc * 0.1);
        let assign78330_e118571: f64 = (locals.var_ddriftldc - assign78330_e118570);
        let assign78330_e118573: f64 = (assign78330_e118571 + locals.var_tmf0);
        (assign78330_e118573, ((locals.var_ddriftldc_dn0 - (locals.var_ddriftldc_dn0 * 0.1)) + locals.var_tmf0_dn0), ((locals.var_ddriftldc_dn2 - (locals.var_ddriftldc_dn2 * 0.1)) + locals.var_tmf0_dn2), ((locals.var_ddriftldc_dn4 - (locals.var_ddriftldc_dn4 * 0.1)) + locals.var_tmf0_dn4), ((locals.var_ddriftldc_dn5 - (locals.var_ddriftldc_dn5 * 0.1)) + locals.var_tmf0_dn5), ((locals.var_ddriftldc_dn6 - (locals.var_ddriftldc_dn6 * 0.1)) + locals.var_tmf0_dn6), ((locals.var_ddriftldc_dn7 - (locals.var_ddriftldc_dn7 * 0.1)) + locals.var_tmf0_dn7), ((locals.var_ddriftldc_dn8 - (locals.var_ddriftldc_dn8 * 0.1)) + locals.var_tmf0_dn8), ((locals.var_ddriftldc_dn9 - (locals.var_ddriftldc_dn9 * 0.1)) + locals.var_tmf0_dn9), ((locals.var_ddriftldc_dn10 - (locals.var_ddriftldc_dn10 * 0.1)) + locals.var_tmf0_dn10), ((locals.var_ddriftldc_dn11 - (locals.var_ddriftldc_dn11 * 0.1)) + locals.var_tmf0_dn11), ((locals.var_ddriftldc_dn14 - (locals.var_ddriftldc_dn14 * 0.1)) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign78330_e118575;
        locals.var_t1_dn0 = assign78330_e118575_d_n0;
        locals.var_t1_dn2 = assign78330_e118575_d_n2;
        locals.var_t1_dn4 = assign78330_e118575_d_n4;
        locals.var_t1_dn5 = assign78330_e118575_d_n5;
        locals.var_t1_dn6 = assign78330_e118575_d_n6;
        locals.var_t1_dn7 = assign78330_e118575_d_n7;
        locals.var_t1_dn8 = assign78330_e118575_d_n8;
        locals.var_t1_dn9 = assign78330_e118575_d_n9;
        locals.var_t1_dn10 = assign78330_e118575_d_n10;
        locals.var_t1_dn11 = assign78330_e118575_d_n11;
        locals.var_t1_dn14 = assign78330_e118575_d_n14;

        let (assign78340_e118586, assign78340_e118586_d_n0, assign78340_e118586_d_n2, assign78340_e118586_d_n4, assign78340_e118586_d_n5, assign78340_e118586_d_n6, assign78340_e118586_d_n7, assign78340_e118586_d_n8, assign78340_e118586_d_n9, assign78340_e118586_d_n10, assign78340_e118586_d_n11, assign78340_e118586_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign78340_e118586;
        locals.var_t0_dn0 = assign78340_e118586_d_n0;
        locals.var_t0_dn2 = assign78340_e118586_d_n2;
        locals.var_t0_dn4 = assign78340_e118586_d_n4;
        locals.var_t0_dn5 = assign78340_e118586_d_n5;
        locals.var_t0_dn6 = assign78340_e118586_d_n6;
        locals.var_t0_dn7 = assign78340_e118586_d_n7;
        locals.var_t0_dn8 = assign78340_e118586_d_n8;
        locals.var_t0_dn9 = assign78340_e118586_d_n9;
        locals.var_t0_dn10 = assign78340_e118586_d_n10;
        locals.var_t0_dn11 = assign78340_e118586_d_n11;
        locals.var_t0_dn14 = assign78340_e118586_d_n14;

        let (assign78350_e118598, assign78350_e118598_d_n0, assign78350_e118598_d_n2, assign78350_e118598_d_n4, assign78350_e118598_d_n5, assign78350_e118598_d_n6, assign78350_e118598_d_n7, assign78350_e118598_d_n8, assign78350_e118598_d_n9, assign78350_e118598_d_n10, assign78350_e118598_d_n11, assign78350_e118598_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 == 0.0)) {
        (locals.var_wdld0__blk1810, locals.var_wdld0__blk1810_dn0, locals.var_wdld0__blk1810_dn2, locals.var_wdld0__blk1810_dn4, locals.var_wdld0__blk1810_dn5, locals.var_wdld0__blk1810_dn6, locals.var_wdld0__blk1810_dn7, locals.var_wdld0__blk1810_dn8, locals.var_wdld0__blk1810_dn9, locals.var_wdld0__blk1810_dn10, locals.var_wdld0__blk1810_dn11, locals.var_wdld0__blk1810_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign78350_e118598;
        locals.var_t1_dn0 = assign78350_e118598_d_n0;
        locals.var_t1_dn2 = assign78350_e118598_d_n2;
        locals.var_t1_dn4 = assign78350_e118598_d_n4;
        locals.var_t1_dn5 = assign78350_e118598_d_n5;
        locals.var_t1_dn6 = assign78350_e118598_d_n6;
        locals.var_t1_dn7 = assign78350_e118598_d_n7;
        locals.var_t1_dn8 = assign78350_e118598_d_n8;
        locals.var_t1_dn9 = assign78350_e118598_d_n9;
        locals.var_t1_dn10 = assign78350_e118598_d_n10;
        locals.var_t1_dn11 = assign78350_e118598_d_n11;
        locals.var_t1_dn14 = assign78350_e118598_d_n14;

        let (assign78360_e118610, assign78360_e118610_d_n0, assign78360_e118610_d_n2, assign78360_e118610_d_n4, assign78360_e118610_d_n5, assign78360_e118610_d_n6, assign78360_e118610_d_n7, assign78360_e118610_d_n8, assign78360_e118610_d_n9, assign78360_e118610_d_n10, assign78360_e118610_d_n11, assign78360_e118610_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1813 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign78360_e118610;
        locals.var_t0_dn0 = assign78360_e118610_d_n0;
        locals.var_t0_dn2 = assign78360_e118610_d_n2;
        locals.var_t0_dn4 = assign78360_e118610_d_n4;
        locals.var_t0_dn5 = assign78360_e118610_d_n5;
        locals.var_t0_dn6 = assign78360_e118610_d_n6;
        locals.var_t0_dn7 = assign78360_e118610_d_n7;
        locals.var_t0_dn8 = assign78360_e118610_d_n8;
        locals.var_t0_dn9 = assign78360_e118610_d_n9;
        locals.var_t0_dn10 = assign78360_e118610_d_n10;
        locals.var_t0_dn11 = assign78360_e118610_d_n11;
        locals.var_t0_dn14 = assign78360_e118610_d_n14;

        let assign78370_e118613: f64 = if locals.var_t0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1819 = assign78370_e118613;

        let (assign78380_e118626,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 != 0.0)) && (locals.var_guard1819 != 0.0)) {
        let assign78380_e118624: f64 = (locals.var_flg_fd_mode__blk1772 + 2.0);
        (assign78380_e118624,)
    } else {
        (locals.var_flg_fd_mode__blk1772,)
    }
};
        locals.var_flg_fd_mode__blk1772 = assign78380_e118626;

        let (assign78390_e118641, assign78390_e118641_d_n0, assign78390_e118641_d_n2, assign78390_e118641_d_n4, assign78390_e118641_d_n5, assign78390_e118641_d_n6, assign78390_e118641_d_n7, assign78390_e118641_d_n8, assign78390_e118641_d_n9, assign78390_e118641_d_n10, assign78390_e118641_d_n11, assign78390_e118641_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 == 0.0)) {
        let (assign78390_e118639, assign78390_e118639_d_n0, assign78390_e118639_d_n2, assign78390_e118639_d_n4, assign78390_e118639_d_n5, assign78390_e118639_d_n6, assign78390_e118639_d_n7, assign78390_e118639_d_n8, assign78390_e118639_d_n9, assign78390_e118639_d_n10, assign78390_e118639_d_n11, assign78390_e118639_d_n14,) = {
            if (locals.var_wdld0__blk1810 <= locals.var_ddriftldc) {
                (locals.var_wdld0__blk1810, locals.var_wdld0__blk1810_dn0, locals.var_wdld0__blk1810_dn2, locals.var_wdld0__blk1810_dn4, locals.var_wdld0__blk1810_dn5, locals.var_wdld0__blk1810_dn6, locals.var_wdld0__blk1810_dn7, locals.var_wdld0__blk1810_dn8, locals.var_wdld0__blk1810_dn9, locals.var_wdld0__blk1810_dn10, locals.var_wdld0__blk1810_dn11, locals.var_wdld0__blk1810_dn14,)
            } else {
                (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
            }
        };
        (assign78390_e118639, assign78390_e118639_d_n0, assign78390_e118639_d_n2, assign78390_e118639_d_n4, assign78390_e118639_d_n5, assign78390_e118639_d_n6, assign78390_e118639_d_n7, assign78390_e118639_d_n8, assign78390_e118639_d_n9, assign78390_e118639_d_n10, assign78390_e118639_d_n11, assign78390_e118639_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign78390_e118641;
        locals.var_t1_dn0 = assign78390_e118641_d_n0;
        locals.var_t1_dn2 = assign78390_e118641_d_n2;
        locals.var_t1_dn4 = assign78390_e118641_d_n4;
        locals.var_t1_dn5 = assign78390_e118641_d_n5;
        locals.var_t1_dn6 = assign78390_e118641_d_n6;
        locals.var_t1_dn7 = assign78390_e118641_d_n7;
        locals.var_t1_dn8 = assign78390_e118641_d_n8;
        locals.var_t1_dn9 = assign78390_e118641_d_n9;
        locals.var_t1_dn10 = assign78390_e118641_d_n10;
        locals.var_t1_dn11 = assign78390_e118641_d_n11;
        locals.var_t1_dn14 = assign78390_e118641_d_n14;

        let assign78400_e118644: f64 = if locals.var_wdld0__blk1810 >= locals.var_ddriftldc { 1.0 } else { 0.0 };
        locals.var_guard1820 = assign78400_e118644;

        let (assign78410_e118658,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1812 == 0.0)) && (locals.var_guard1820 != 0.0)) {
        let assign78410_e118656: f64 = (locals.var_flg_fd_mode__blk1772 + 2.0);
        (assign78410_e118656,)
    } else {
        (locals.var_flg_fd_mode__blk1772,)
    }
};
        locals.var_flg_fd_mode__blk1772 = assign78410_e118658;

        let assign78420_e118661: f64 = if locals.var_flg_fd_mode__blk1772 >= 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1821 = assign78420_e118661;

        let (assign78430_e118670, assign78430_e118670_d_n0, assign78430_e118670_d_n2, assign78430_e118670_d_n4, assign78430_e118670_d_n5, assign78430_e118670_d_n6, assign78430_e118670_d_n7, assign78430_e118670_d_n8, assign78430_e118670_d_n9, assign78430_e118670_d_n10, assign78430_e118670_d_n11, assign78430_e118670_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld_bef1__blk1811, locals.var_ps0ld_bef1__blk1811_dn0, locals.var_ps0ld_bef1__blk1811_dn2, locals.var_ps0ld_bef1__blk1811_dn4, locals.var_ps0ld_bef1__blk1811_dn5, locals.var_ps0ld_bef1__blk1811_dn6, locals.var_ps0ld_bef1__blk1811_dn7, locals.var_ps0ld_bef1__blk1811_dn8, locals.var_ps0ld_bef1__blk1811_dn9, locals.var_ps0ld_bef1__blk1811_dn10, locals.var_ps0ld_bef1__blk1811_dn11, locals.var_ps0ld_bef1__blk1811_dn14,)
    }
};
        locals.var_ps0ld_bef1__blk1811 = assign78430_e118670;
        locals.var_ps0ld_bef1__blk1811_dn0 = assign78430_e118670_d_n0;
        locals.var_ps0ld_bef1__blk1811_dn2 = assign78430_e118670_d_n2;
        locals.var_ps0ld_bef1__blk1811_dn4 = assign78430_e118670_d_n4;
        locals.var_ps0ld_bef1__blk1811_dn5 = assign78430_e118670_d_n5;
        locals.var_ps0ld_bef1__blk1811_dn6 = assign78430_e118670_d_n6;
        locals.var_ps0ld_bef1__blk1811_dn7 = assign78430_e118670_d_n7;
        locals.var_ps0ld_bef1__blk1811_dn8 = assign78430_e118670_d_n8;
        locals.var_ps0ld_bef1__blk1811_dn9 = assign78430_e118670_d_n9;
        locals.var_ps0ld_bef1__blk1811_dn10 = assign78430_e118670_d_n10;
        locals.var_ps0ld_bef1__blk1811_dn11 = assign78430_e118670_d_n11;
        locals.var_ps0ld_bef1__blk1811_dn14 = assign78430_e118670_d_n14;

        let (assign78440_e118681, assign78440_e118681_d_n0, assign78440_e118681_d_n2, assign78440_e118681_d_n4, assign78440_e118681_d_n5, assign78440_e118681_d_n6, assign78440_e118681_d_n7, assign78440_e118681_d_n8, assign78440_e118681_d_n9, assign78440_e118681_d_n10, assign78440_e118681_d_n11, assign78440_e118681_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) {
        let assign78440_e118679: f64 = (locals.var_t1 * locals.var_q_nsubld__blk1766);
        (assign78440_e118679, (locals.var_t1_dn0 * locals.var_q_nsubld__blk1766), (locals.var_t1_dn2 * locals.var_q_nsubld__blk1766), (locals.var_t1_dn4 * locals.var_q_nsubld__blk1766), (locals.var_t1_dn5 * locals.var_q_nsubld__blk1766), (locals.var_t1_dn6 * locals.var_q_nsubld__blk1766), (locals.var_t1_dn7 * locals.var_q_nsubld__blk1766), (locals.var_t1_dn8 * locals.var_q_nsubld__blk1766), (locals.var_t1_dn9 * locals.var_q_nsubld__blk1766), (locals.var_t1_dn10 * locals.var_q_nsubld__blk1766), (locals.var_t1_dn11 * locals.var_q_nsubld__blk1766), (locals.var_t1_dn14 * locals.var_q_nsubld__blk1766),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign78440_e118681;
        locals.var_qbuld_dn0 = assign78440_e118681_d_n0;
        locals.var_qbuld_dn2 = assign78440_e118681_d_n2;
        locals.var_qbuld_dn4 = assign78440_e118681_d_n4;
        locals.var_qbuld_dn5 = assign78440_e118681_d_n5;
        locals.var_qbuld_dn6 = assign78440_e118681_d_n6;
        locals.var_qbuld_dn7 = assign78440_e118681_d_n7;
        locals.var_qbuld_dn8 = assign78440_e118681_d_n8;
        locals.var_qbuld_dn9 = assign78440_e118681_d_n9;
        locals.var_qbuld_dn10 = assign78440_e118681_d_n10;
        locals.var_qbuld_dn11 = assign78440_e118681_d_n11;
        locals.var_qbuld_dn14 = assign78440_e118681_d_n14;

        let (assign78450_e118694, assign78450_e118694_d_n0, assign78450_e118694_d_n2, assign78450_e118694_d_n4, assign78450_e118694_d_n5, assign78450_e118694_d_n6, assign78450_e118694_d_n7, assign78450_e118694_d_n8, assign78450_e118694_d_n9, assign78450_e118694_d_n10, assign78450_e118694_d_n11, assign78450_e118694_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) {
        let assign78450_e118691: f64 = (locals.var_qbuld / locals.var_cox0_func);
        let assign78450_e118692: f64 = (locals.var_vgpld - assign78450_e118691);
        (assign78450_e118692, (-(locals.var_qbuld_dn0 / locals.var_cox0_func)), (locals.var_vgpld_dn2 - (locals.var_qbuld_dn2 / locals.var_cox0_func)), (-(locals.var_qbuld_dn4 / locals.var_cox0_func)), (-(locals.var_qbuld_dn5 / locals.var_cox0_func)), (-(locals.var_qbuld_dn6 / locals.var_cox0_func)), (locals.var_vgpld_dn7 - (locals.var_qbuld_dn7 / locals.var_cox0_func)), (locals.var_vgpld_dn8 - (locals.var_qbuld_dn8 / locals.var_cox0_func)), (locals.var_vgpld_dn9 - (locals.var_qbuld_dn9 / locals.var_cox0_func)), (-(locals.var_qbuld_dn10 / locals.var_cox0_func)), (-(locals.var_qbuld_dn11 / locals.var_cox0_func)), (-(locals.var_qbuld_dn14 / locals.var_cox0_func)),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign78450_e118694;
        locals.var_ps0ld_dn0 = assign78450_e118694_d_n0;
        locals.var_ps0ld_dn2 = assign78450_e118694_d_n2;
        locals.var_ps0ld_dn4 = assign78450_e118694_d_n4;
        locals.var_ps0ld_dn5 = assign78450_e118694_d_n5;
        locals.var_ps0ld_dn6 = assign78450_e118694_d_n6;
        locals.var_ps0ld_dn7 = assign78450_e118694_d_n7;
        locals.var_ps0ld_dn8 = assign78450_e118694_d_n8;
        locals.var_ps0ld_dn9 = assign78450_e118694_d_n9;
        locals.var_ps0ld_dn10 = assign78450_e118694_d_n10;
        locals.var_ps0ld_dn11 = assign78450_e118694_d_n11;
        locals.var_ps0ld_dn14 = assign78450_e118694_d_n14;

        let assign78460_e118697: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1822 = assign78460_e118697;

        let assign78470_e118701: f64 = (locals.var_ps0ld_bef1__blk1811 - 0.1);
        let assign78470_e118706: f64 = if ((locals.var_ps0ld > assign78470_e118701) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1823 = assign78470_e118706;

        let (assign78480_e118723, assign78480_e118723_d_n0, assign78480_e118723_d_n2, assign78480_e118723_d_n4, assign78480_e118723_d_n5, assign78480_e118723_d_n6, assign78480_e118723_d_n7, assign78480_e118723_d_n8, assign78480_e118723_d_n9, assign78480_e118723_d_n10, assign78480_e118723_d_n11, assign78480_e118723_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        let assign78480_e118719: f64 = (locals.var_ps0ld - locals.var_ps0ld_bef1__blk1811);
        let assign78480_e118721: f64 = (assign78480_e118719 + 0.1);
        (assign78480_e118721, (locals.var_ps0ld_dn0 - locals.var_ps0ld_bef1__blk1811_dn0), (locals.var_ps0ld_dn2 - locals.var_ps0ld_bef1__blk1811_dn2), (locals.var_ps0ld_dn4 - locals.var_ps0ld_bef1__blk1811_dn4), (locals.var_ps0ld_dn5 - locals.var_ps0ld_bef1__blk1811_dn5), (locals.var_ps0ld_dn6 - locals.var_ps0ld_bef1__blk1811_dn6), (locals.var_ps0ld_dn7 - locals.var_ps0ld_bef1__blk1811_dn7), (locals.var_ps0ld_dn8 - locals.var_ps0ld_bef1__blk1811_dn8), (locals.var_ps0ld_dn9 - locals.var_ps0ld_bef1__blk1811_dn9), (locals.var_ps0ld_dn10 - locals.var_ps0ld_bef1__blk1811_dn10), (locals.var_ps0ld_dn11 - locals.var_ps0ld_bef1__blk1811_dn11), (locals.var_ps0ld_dn14 - locals.var_ps0ld_bef1__blk1811_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign78480_e118723;
        locals.var_tmf1_dn0 = assign78480_e118723_d_n0;
        locals.var_tmf1_dn2 = assign78480_e118723_d_n2;
        locals.var_tmf1_dn4 = assign78480_e118723_d_n4;
        locals.var_tmf1_dn5 = assign78480_e118723_d_n5;
        locals.var_tmf1_dn6 = assign78480_e118723_d_n6;
        locals.var_tmf1_dn7 = assign78480_e118723_d_n7;
        locals.var_tmf1_dn8 = assign78480_e118723_d_n8;
        locals.var_tmf1_dn9 = assign78480_e118723_d_n9;
        locals.var_tmf1_dn10 = assign78480_e118723_d_n10;
        locals.var_tmf1_dn11 = assign78480_e118723_d_n11;
        locals.var_tmf1_dn14 = assign78480_e118723_d_n14;

        let (assign78490_e118738, assign78490_e118738_d_n0, assign78490_e118738_d_n2, assign78490_e118738_d_n4, assign78490_e118738_d_n5, assign78490_e118738_d_n6, assign78490_e118738_d_n7, assign78490_e118738_d_n8, assign78490_e118738_d_n9, assign78490_e118738_d_n10, assign78490_e118738_d_n11, assign78490_e118738_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        let assign78490_e118736: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign78490_e118736, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign78490_e118738;
        locals.var_x2_dn0 = assign78490_e118738_d_n0;
        locals.var_x2_dn2 = assign78490_e118738_d_n2;
        locals.var_x2_dn4 = assign78490_e118738_d_n4;
        locals.var_x2_dn5 = assign78490_e118738_d_n5;
        locals.var_x2_dn6 = assign78490_e118738_d_n6;
        locals.var_x2_dn7 = assign78490_e118738_d_n7;
        locals.var_x2_dn8 = assign78490_e118738_d_n8;
        locals.var_x2_dn9 = assign78490_e118738_d_n9;
        locals.var_x2_dn10 = assign78490_e118738_d_n10;
        locals.var_x2_dn11 = assign78490_e118738_d_n11;
        locals.var_x2_dn14 = assign78490_e118738_d_n14;

        let (assign78500_e118753, assign78500_e118753_d_n0, assign78500_e118753_d_n2, assign78500_e118753_d_n4, assign78500_e118753_d_n5, assign78500_e118753_d_n6, assign78500_e118753_d_n7, assign78500_e118753_d_n8, assign78500_e118753_d_n9, assign78500_e118753_d_n10, assign78500_e118753_d_n11, assign78500_e118753_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        let assign78500_e118751: f64 = (0.1 * 0.1);
        (assign78500_e118751, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign78500_e118753;
        locals.var_xmax2_dn0 = assign78500_e118753_d_n0;
        locals.var_xmax2_dn2 = assign78500_e118753_d_n2;
        locals.var_xmax2_dn4 = assign78500_e118753_d_n4;
        locals.var_xmax2_dn5 = assign78500_e118753_d_n5;
        locals.var_xmax2_dn6 = assign78500_e118753_d_n6;
        locals.var_xmax2_dn7 = assign78500_e118753_d_n7;
        locals.var_xmax2_dn8 = assign78500_e118753_d_n8;
        locals.var_xmax2_dn9 = assign78500_e118753_d_n9;
        locals.var_xmax2_dn10 = assign78500_e118753_d_n10;
        locals.var_xmax2_dn11 = assign78500_e118753_d_n11;
        locals.var_xmax2_dn14 = assign78500_e118753_d_n14;

        let (assign78510_e118766, assign78510_e118766_d_n0, assign78510_e118766_d_n2, assign78510_e118766_d_n4, assign78510_e118766_d_n5, assign78510_e118766_d_n6, assign78510_e118766_d_n7, assign78510_e118766_d_n8, assign78510_e118766_d_n9, assign78510_e118766_d_n10, assign78510_e118766_d_n11, assign78510_e118766_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign78510_e118766;
        locals.var_xp_dn0 = assign78510_e118766_d_n0;
        locals.var_xp_dn2 = assign78510_e118766_d_n2;
        locals.var_xp_dn4 = assign78510_e118766_d_n4;
        locals.var_xp_dn5 = assign78510_e118766_d_n5;
        locals.var_xp_dn6 = assign78510_e118766_d_n6;
        locals.var_xp_dn7 = assign78510_e118766_d_n7;
        locals.var_xp_dn8 = assign78510_e118766_d_n8;
        locals.var_xp_dn9 = assign78510_e118766_d_n9;
        locals.var_xp_dn10 = assign78510_e118766_d_n10;
        locals.var_xp_dn11 = assign78510_e118766_d_n11;
        locals.var_xp_dn14 = assign78510_e118766_d_n14;

        let (assign78520_e118779, assign78520_e118779_d_n0, assign78520_e118779_d_n2, assign78520_e118779_d_n4, assign78520_e118779_d_n5, assign78520_e118779_d_n6, assign78520_e118779_d_n7, assign78520_e118779_d_n8, assign78520_e118779_d_n9, assign78520_e118779_d_n10, assign78520_e118779_d_n11, assign78520_e118779_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign78520_e118779;
        locals.var_xmp_dn0 = assign78520_e118779_d_n0;
        locals.var_xmp_dn2 = assign78520_e118779_d_n2;
        locals.var_xmp_dn4 = assign78520_e118779_d_n4;
        locals.var_xmp_dn5 = assign78520_e118779_d_n5;
        locals.var_xmp_dn6 = assign78520_e118779_d_n6;
        locals.var_xmp_dn7 = assign78520_e118779_d_n7;
        locals.var_xmp_dn8 = assign78520_e118779_d_n8;
        locals.var_xmp_dn9 = assign78520_e118779_d_n9;
        locals.var_xmp_dn10 = assign78520_e118779_d_n10;
        locals.var_xmp_dn11 = assign78520_e118779_d_n11;
        locals.var_xmp_dn14 = assign78520_e118779_d_n14;

        let (assign78530_e118792,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign78530_e118792;

        let (assign78540_e118805,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78540_e118805;

    }

    pub(super) fn stamp_transient_block_283(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign78550_e118818, assign78550_e118818_d_n0, assign78550_e118818_d_n2, assign78550_e118818_d_n4, assign78550_e118818_d_n5, assign78550_e118818_d_n6, assign78550_e118818_d_n7, assign78550_e118818_d_n8, assign78550_e118818_d_n9, assign78550_e118818_d_n10, assign78550_e118818_d_n11, assign78550_e118818_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign78550_e118818;
        locals.var_arg_dn0 = assign78550_e118818_d_n0;
        locals.var_arg_dn2 = assign78550_e118818_d_n2;
        locals.var_arg_dn4 = assign78550_e118818_d_n4;
        locals.var_arg_dn5 = assign78550_e118818_d_n5;
        locals.var_arg_dn6 = assign78550_e118818_d_n6;
        locals.var_arg_dn7 = assign78550_e118818_d_n7;
        locals.var_arg_dn8 = assign78550_e118818_d_n8;
        locals.var_arg_dn9 = assign78550_e118818_d_n9;
        locals.var_arg_dn10 = assign78550_e118818_d_n10;
        locals.var_arg_dn11 = assign78550_e118818_d_n11;
        locals.var_arg_dn14 = assign78550_e118818_d_n14;

        let (assign78560_e118831, assign78560_e118831_d_n0, assign78560_e118831_d_n2, assign78560_e118831_d_n4, assign78560_e118831_d_n5, assign78560_e118831_d_n6, assign78560_e118831_d_n7, assign78560_e118831_d_n8, assign78560_e118831_d_n9, assign78560_e118831_d_n10, assign78560_e118831_d_n11, assign78560_e118831_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign78560_e118831;
        locals.var_dnm_dn0 = assign78560_e118831_d_n0;
        locals.var_dnm_dn2 = assign78560_e118831_d_n2;
        locals.var_dnm_dn4 = assign78560_e118831_d_n4;
        locals.var_dnm_dn5 = assign78560_e118831_d_n5;
        locals.var_dnm_dn6 = assign78560_e118831_d_n6;
        locals.var_dnm_dn7 = assign78560_e118831_d_n7;
        locals.var_dnm_dn8 = assign78560_e118831_d_n8;
        locals.var_dnm_dn9 = assign78560_e118831_d_n9;
        locals.var_dnm_dn10 = assign78560_e118831_d_n10;
        locals.var_dnm_dn11 = assign78560_e118831_d_n11;
        locals.var_dnm_dn14 = assign78560_e118831_d_n14;

        let (assign78570_e118846, assign78570_e118846_d_n0, assign78570_e118846_d_n2, assign78570_e118846_d_n4, assign78570_e118846_d_n5, assign78570_e118846_d_n6, assign78570_e118846_d_n7, assign78570_e118846_d_n8, assign78570_e118846_d_n9, assign78570_e118846_d_n10, assign78570_e118846_d_n11, assign78570_e118846_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        let assign78570_e118844: f64 = (locals.var_xp * locals.var_x2);
        (assign78570_e118844, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign78570_e118846;
        locals.var_xp_dn0 = assign78570_e118846_d_n0;
        locals.var_xp_dn2 = assign78570_e118846_d_n2;
        locals.var_xp_dn4 = assign78570_e118846_d_n4;
        locals.var_xp_dn5 = assign78570_e118846_d_n5;
        locals.var_xp_dn6 = assign78570_e118846_d_n6;
        locals.var_xp_dn7 = assign78570_e118846_d_n7;
        locals.var_xp_dn8 = assign78570_e118846_d_n8;
        locals.var_xp_dn9 = assign78570_e118846_d_n9;
        locals.var_xp_dn10 = assign78570_e118846_d_n10;
        locals.var_xp_dn11 = assign78570_e118846_d_n11;
        locals.var_xp_dn14 = assign78570_e118846_d_n14;

        let (assign78580_e118861, assign78580_e118861_d_n0, assign78580_e118861_d_n2, assign78580_e118861_d_n4, assign78580_e118861_d_n5, assign78580_e118861_d_n6, assign78580_e118861_d_n7, assign78580_e118861_d_n8, assign78580_e118861_d_n9, assign78580_e118861_d_n10, assign78580_e118861_d_n11, assign78580_e118861_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        let assign78580_e118859: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign78580_e118859, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign78580_e118861;
        locals.var_xmp_dn0 = assign78580_e118861_d_n0;
        locals.var_xmp_dn2 = assign78580_e118861_d_n2;
        locals.var_xmp_dn4 = assign78580_e118861_d_n4;
        locals.var_xmp_dn5 = assign78580_e118861_d_n5;
        locals.var_xmp_dn6 = assign78580_e118861_d_n6;
        locals.var_xmp_dn7 = assign78580_e118861_d_n7;
        locals.var_xmp_dn8 = assign78580_e118861_d_n8;
        locals.var_xmp_dn9 = assign78580_e118861_d_n9;
        locals.var_xmp_dn10 = assign78580_e118861_d_n10;
        locals.var_xmp_dn11 = assign78580_e118861_d_n11;
        locals.var_xmp_dn14 = assign78580_e118861_d_n14;

        let (assign78590_e118876, assign78590_e118876_d_n0, assign78590_e118876_d_n2, assign78590_e118876_d_n4, assign78590_e118876_d_n5, assign78590_e118876_d_n6, assign78590_e118876_d_n7, assign78590_e118876_d_n8, assign78590_e118876_d_n9, assign78590_e118876_d_n10, assign78590_e118876_d_n11, assign78590_e118876_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        let assign78590_e118874: f64 = (locals.var_xp * locals.var_x2);
        (assign78590_e118874, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign78590_e118876;
        locals.var_xp_dn0 = assign78590_e118876_d_n0;
        locals.var_xp_dn2 = assign78590_e118876_d_n2;
        locals.var_xp_dn4 = assign78590_e118876_d_n4;
        locals.var_xp_dn5 = assign78590_e118876_d_n5;
        locals.var_xp_dn6 = assign78590_e118876_d_n6;
        locals.var_xp_dn7 = assign78590_e118876_d_n7;
        locals.var_xp_dn8 = assign78590_e118876_d_n8;
        locals.var_xp_dn9 = assign78590_e118876_d_n9;
        locals.var_xp_dn10 = assign78590_e118876_d_n10;
        locals.var_xp_dn11 = assign78590_e118876_d_n11;
        locals.var_xp_dn14 = assign78590_e118876_d_n14;

        let (assign78600_e118891, assign78600_e118891_d_n0, assign78600_e118891_d_n2, assign78600_e118891_d_n4, assign78600_e118891_d_n5, assign78600_e118891_d_n6, assign78600_e118891_d_n7, assign78600_e118891_d_n8, assign78600_e118891_d_n9, assign78600_e118891_d_n10, assign78600_e118891_d_n11, assign78600_e118891_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        let assign78600_e118889: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign78600_e118889, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign78600_e118891;
        locals.var_xmp_dn0 = assign78600_e118891_d_n0;
        locals.var_xmp_dn2 = assign78600_e118891_d_n2;
        locals.var_xmp_dn4 = assign78600_e118891_d_n4;
        locals.var_xmp_dn5 = assign78600_e118891_d_n5;
        locals.var_xmp_dn6 = assign78600_e118891_d_n6;
        locals.var_xmp_dn7 = assign78600_e118891_d_n7;
        locals.var_xmp_dn8 = assign78600_e118891_d_n8;
        locals.var_xmp_dn9 = assign78600_e118891_d_n9;
        locals.var_xmp_dn10 = assign78600_e118891_d_n10;
        locals.var_xmp_dn11 = assign78600_e118891_d_n11;
        locals.var_xmp_dn14 = assign78600_e118891_d_n14;

        let (assign78610_e118906, assign78610_e118906_d_n0, assign78610_e118906_d_n2, assign78610_e118906_d_n4, assign78610_e118906_d_n5, assign78610_e118906_d_n6, assign78610_e118906_d_n7, assign78610_e118906_d_n8, assign78610_e118906_d_n9, assign78610_e118906_d_n10, assign78610_e118906_d_n11, assign78610_e118906_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        let assign78610_e118904: f64 = (locals.var_xp + locals.var_xmp);
        (assign78610_e118904, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign78610_e118906;
        locals.var_arg_dn0 = assign78610_e118906_d_n0;
        locals.var_arg_dn2 = assign78610_e118906_d_n2;
        locals.var_arg_dn4 = assign78610_e118906_d_n4;
        locals.var_arg_dn5 = assign78610_e118906_d_n5;
        locals.var_arg_dn6 = assign78610_e118906_d_n6;
        locals.var_arg_dn7 = assign78610_e118906_d_n7;
        locals.var_arg_dn8 = assign78610_e118906_d_n8;
        locals.var_arg_dn9 = assign78610_e118906_d_n9;
        locals.var_arg_dn10 = assign78610_e118906_d_n10;
        locals.var_arg_dn11 = assign78610_e118906_d_n11;
        locals.var_arg_dn14 = assign78610_e118906_d_n14;

        let (assign78620_e118919, assign78620_e118919_d_n0, assign78620_e118919_d_n2, assign78620_e118919_d_n4, assign78620_e118919_d_n5, assign78620_e118919_d_n6, assign78620_e118919_d_n7, assign78620_e118919_d_n8, assign78620_e118919_d_n9, assign78620_e118919_d_n10, assign78620_e118919_d_n11, assign78620_e118919_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign78620_e118919;
        locals.var_dnm_dn0 = assign78620_e118919_d_n0;
        locals.var_dnm_dn2 = assign78620_e118919_d_n2;
        locals.var_dnm_dn4 = assign78620_e118919_d_n4;
        locals.var_dnm_dn5 = assign78620_e118919_d_n5;
        locals.var_dnm_dn6 = assign78620_e118919_d_n6;
        locals.var_dnm_dn7 = assign78620_e118919_d_n7;
        locals.var_dnm_dn8 = assign78620_e118919_d_n8;
        locals.var_dnm_dn9 = assign78620_e118919_d_n9;
        locals.var_dnm_dn10 = assign78620_e118919_d_n10;
        locals.var_dnm_dn11 = assign78620_e118919_d_n11;
        locals.var_dnm_dn14 = assign78620_e118919_d_n14;

        let assign78630_e118934: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1824 = assign78630_e118934;

        let assign78640_e118937: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1825 = assign78640_e118937;

        let (assign78650_e118954,) = {
    if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) && (locals.var_guard1824 != 0.0)) && (locals.var_guard1825 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78650_e118954;

        let assign78660_e118957: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1826 = assign78660_e118957;

        let (assign78670_e118977,) = {
    if ((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) && (locals.var_guard1824 != 0.0)) && (locals.var_guard1825 == 0.0)) && (locals.var_guard1826 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78670_e118977;

        let assign78680_e118980: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1827 = assign78680_e118980;

        let (assign78690_e119003,) = {
    if (((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) && (locals.var_guard1824 != 0.0)) && (locals.var_guard1825 == 0.0)) && (locals.var_guard1826 == 0.0)) && (locals.var_guard1827 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78690_e119003;

        let assign78700_e119006: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1828 = assign78700_e119006;

        let (assign78710_e119032,) = {
    if ((((((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) && (locals.var_guard1824 != 0.0)) && (locals.var_guard1825 == 0.0)) && (locals.var_guard1826 == 0.0)) && (locals.var_guard1827 == 0.0)) && (locals.var_guard1828 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign78710_e119032;

        let (assign78720_e119047,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) && (locals.var_guard1824 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign78720_e119047;

        let mut assign78730_loop_guard: usize = 0;
        while {
            let assign78730_cond_e119063: f64 = if (((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) && (locals.var_guard1824 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign78730_cond_e119063 != 0.0
        } {
            assign78730_loop_guard += 1;
            assert!(assign78730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign78730_body0_e119079, assign78730_body0_e119079_d_n0, assign78730_body0_e119079_d_n2, assign78730_body0_e119079_d_n4, assign78730_body0_e119079_d_n5, assign78730_body0_e119079_d_n6, assign78730_body0_e119079_d_n7, assign78730_body0_e119079_d_n8, assign78730_body0_e119079_d_n9, assign78730_body0_e119079_d_n10, assign78730_body0_e119079_d_n11, assign78730_body0_e119079_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) && (locals.var_guard1824 != 0.0)) {
        let assign78730_body0_e119077: f64 = (locals.var_dnm).sqrt();
        (assign78730_body0_e119077, (locals.var_dnm_dn0 / (2.0 * assign78730_body0_e119077)), (locals.var_dnm_dn2 / (2.0 * assign78730_body0_e119077)), (locals.var_dnm_dn4 / (2.0 * assign78730_body0_e119077)), (locals.var_dnm_dn5 / (2.0 * assign78730_body0_e119077)), (locals.var_dnm_dn6 / (2.0 * assign78730_body0_e119077)), (locals.var_dnm_dn7 / (2.0 * assign78730_body0_e119077)), (locals.var_dnm_dn8 / (2.0 * assign78730_body0_e119077)), (locals.var_dnm_dn9 / (2.0 * assign78730_body0_e119077)), (locals.var_dnm_dn10 / (2.0 * assign78730_body0_e119077)), (locals.var_dnm_dn11 / (2.0 * assign78730_body0_e119077)), (locals.var_dnm_dn14 / (2.0 * assign78730_body0_e119077)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign78730_body0_e119079;
            locals.var_dnm_dn0 = assign78730_body0_e119079_d_n0;
            locals.var_dnm_dn2 = assign78730_body0_e119079_d_n2;
            locals.var_dnm_dn4 = assign78730_body0_e119079_d_n4;
            locals.var_dnm_dn5 = assign78730_body0_e119079_d_n5;
            locals.var_dnm_dn6 = assign78730_body0_e119079_d_n6;
            locals.var_dnm_dn7 = assign78730_body0_e119079_d_n7;
            locals.var_dnm_dn8 = assign78730_body0_e119079_d_n8;
            locals.var_dnm_dn9 = assign78730_body0_e119079_d_n9;
            locals.var_dnm_dn10 = assign78730_body0_e119079_d_n10;
            locals.var_dnm_dn11 = assign78730_body0_e119079_d_n11;
            locals.var_dnm_dn14 = assign78730_body0_e119079_d_n14;
            let (assign78730_body1_e119096,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) && (locals.var_guard1824 != 0.0)) {
        let assign78730_body1_e119094: f64 = (locals.var_m0 + 1.0);
        (assign78730_body1_e119094,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign78730_body1_e119096;
        }

        let (assign78740_e119123, assign78740_e119123_d_n0, assign78740_e119123_d_n2, assign78740_e119123_d_n4, assign78740_e119123_d_n5, assign78740_e119123_d_n6, assign78740_e119123_d_n7, assign78740_e119123_d_n8, assign78740_e119123_d_n9, assign78740_e119123_d_n10, assign78740_e119123_d_n11, assign78740_e119123_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) && (locals.var_guard1824 == 0.0)) {
        let (assign78740_e119121, assign78740_e119121_d_n0, assign78740_e119121_d_n2, assign78740_e119121_d_n4, assign78740_e119121_d_n5, assign78740_e119121_d_n6, assign78740_e119121_d_n7, assign78740_e119121_d_n8, assign78740_e119121_d_n9, assign78740_e119121_d_n10, assign78740_e119121_d_n11, assign78740_e119121_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign78740_e119118: f64 = (2.0 * 2.0);
                let assign78740_e119119: f64 = (1.0 / assign78740_e119118);
                let assign78740_e119120: f64 = (locals.var_dnm).powf(assign78740_e119119);
                (assign78740_e119120, if 0.0 == 0.0 && ((assign78740_e119119) as f64).is_finite() && ((assign78740_e119119) as f64).fract() == 0.0 { if assign78740_e119119 == 0.0 { 0.0 } else { (assign78740_e119119 * ((locals.var_dnm).powf(assign78740_e119119 - 1.0) * locals.var_dnm_dn0)) } } else { (assign78740_e119120 * (assign78740_e119119 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78740_e119119) as f64).is_finite() && ((assign78740_e119119) as f64).fract() == 0.0 { if assign78740_e119119 == 0.0 { 0.0 } else { (assign78740_e119119 * ((locals.var_dnm).powf(assign78740_e119119 - 1.0) * locals.var_dnm_dn2)) } } else { (assign78740_e119120 * (assign78740_e119119 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78740_e119119) as f64).is_finite() && ((assign78740_e119119) as f64).fract() == 0.0 { if assign78740_e119119 == 0.0 { 0.0 } else { (assign78740_e119119 * ((locals.var_dnm).powf(assign78740_e119119 - 1.0) * locals.var_dnm_dn4)) } } else { (assign78740_e119120 * (assign78740_e119119 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78740_e119119) as f64).is_finite() && ((assign78740_e119119) as f64).fract() == 0.0 { if assign78740_e119119 == 0.0 { 0.0 } else { (assign78740_e119119 * ((locals.var_dnm).powf(assign78740_e119119 - 1.0) * locals.var_dnm_dn5)) } } else { (assign78740_e119120 * (assign78740_e119119 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78740_e119119) as f64).is_finite() && ((assign78740_e119119) as f64).fract() == 0.0 { if assign78740_e119119 == 0.0 { 0.0 } else { (assign78740_e119119 * ((locals.var_dnm).powf(assign78740_e119119 - 1.0) * locals.var_dnm_dn6)) } } else { (assign78740_e119120 * (assign78740_e119119 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78740_e119119) as f64).is_finite() && ((assign78740_e119119) as f64).fract() == 0.0 { if assign78740_e119119 == 0.0 { 0.0 } else { (assign78740_e119119 * ((locals.var_dnm).powf(assign78740_e119119 - 1.0) * locals.var_dnm_dn7)) } } else { (assign78740_e119120 * (assign78740_e119119 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78740_e119119) as f64).is_finite() && ((assign78740_e119119) as f64).fract() == 0.0 { if assign78740_e119119 == 0.0 { 0.0 } else { (assign78740_e119119 * ((locals.var_dnm).powf(assign78740_e119119 - 1.0) * locals.var_dnm_dn8)) } } else { (assign78740_e119120 * (assign78740_e119119 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78740_e119119) as f64).is_finite() && ((assign78740_e119119) as f64).fract() == 0.0 { if assign78740_e119119 == 0.0 { 0.0 } else { (assign78740_e119119 * ((locals.var_dnm).powf(assign78740_e119119 - 1.0) * locals.var_dnm_dn9)) } } else { (assign78740_e119120 * (assign78740_e119119 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78740_e119119) as f64).is_finite() && ((assign78740_e119119) as f64).fract() == 0.0 { if assign78740_e119119 == 0.0 { 0.0 } else { (assign78740_e119119 * ((locals.var_dnm).powf(assign78740_e119119 - 1.0) * locals.var_dnm_dn10)) } } else { (assign78740_e119120 * (assign78740_e119119 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78740_e119119) as f64).is_finite() && ((assign78740_e119119) as f64).fract() == 0.0 { if assign78740_e119119 == 0.0 { 0.0 } else { (assign78740_e119119 * ((locals.var_dnm).powf(assign78740_e119119 - 1.0) * locals.var_dnm_dn11)) } } else { (assign78740_e119120 * (assign78740_e119119 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign78740_e119119) as f64).is_finite() && ((assign78740_e119119) as f64).fract() == 0.0 { if assign78740_e119119 == 0.0 { 0.0 } else { (assign78740_e119119 * ((locals.var_dnm).powf(assign78740_e119119 - 1.0) * locals.var_dnm_dn14)) } } else { (assign78740_e119120 * (assign78740_e119119 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign78740_e119121, assign78740_e119121_d_n0, assign78740_e119121_d_n2, assign78740_e119121_d_n4, assign78740_e119121_d_n5, assign78740_e119121_d_n6, assign78740_e119121_d_n7, assign78740_e119121_d_n8, assign78740_e119121_d_n9, assign78740_e119121_d_n10, assign78740_e119121_d_n11, assign78740_e119121_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign78740_e119123;
        locals.var_dnm_dn0 = assign78740_e119123_d_n0;
        locals.var_dnm_dn2 = assign78740_e119123_d_n2;
        locals.var_dnm_dn4 = assign78740_e119123_d_n4;
        locals.var_dnm_dn5 = assign78740_e119123_d_n5;
        locals.var_dnm_dn6 = assign78740_e119123_d_n6;
        locals.var_dnm_dn7 = assign78740_e119123_d_n7;
        locals.var_dnm_dn8 = assign78740_e119123_d_n8;
        locals.var_dnm_dn9 = assign78740_e119123_d_n9;
        locals.var_dnm_dn10 = assign78740_e119123_d_n10;
        locals.var_dnm_dn11 = assign78740_e119123_d_n11;
        locals.var_dnm_dn14 = assign78740_e119123_d_n14;

        let (assign78750_e119138, assign78750_e119138_d_n0, assign78750_e119138_d_n2, assign78750_e119138_d_n4, assign78750_e119138_d_n5, assign78750_e119138_d_n6, assign78750_e119138_d_n7, assign78750_e119138_d_n8, assign78750_e119138_d_n9, assign78750_e119138_d_n10, assign78750_e119138_d_n11, assign78750_e119138_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        let assign78750_e119136: f64 = (1.0 / locals.var_dnm);
        (assign78750_e119136, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign78750_e119138;
        locals.var_dnm_dn0 = assign78750_e119138_d_n0;
        locals.var_dnm_dn2 = assign78750_e119138_d_n2;
        locals.var_dnm_dn4 = assign78750_e119138_d_n4;
        locals.var_dnm_dn5 = assign78750_e119138_d_n5;
        locals.var_dnm_dn6 = assign78750_e119138_d_n6;
        locals.var_dnm_dn7 = assign78750_e119138_d_n7;
        locals.var_dnm_dn8 = assign78750_e119138_d_n8;
        locals.var_dnm_dn9 = assign78750_e119138_d_n9;
        locals.var_dnm_dn10 = assign78750_e119138_d_n10;
        locals.var_dnm_dn11 = assign78750_e119138_d_n11;
        locals.var_dnm_dn14 = assign78750_e119138_d_n14;

        let (assign78760_e119155, assign78760_e119155_d_n0, assign78760_e119155_d_n2, assign78760_e119155_d_n4, assign78760_e119155_d_n5, assign78760_e119155_d_n6, assign78760_e119155_d_n7, assign78760_e119155_d_n8, assign78760_e119155_d_n9, assign78760_e119155_d_n10, assign78760_e119155_d_n11, assign78760_e119155_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        let assign78760_e119151: f64 = (locals.var_tmf1 * 0.1);
        let assign78760_e119153: f64 = (assign78760_e119151 * locals.var_dnm);
        (assign78760_e119153, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign78760_e119151 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign78760_e119151 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign78760_e119151 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign78760_e119151 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign78760_e119151 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign78760_e119151 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign78760_e119151 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign78760_e119151 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign78760_e119151 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.1) * locals.var_dnm) + (assign78760_e119151 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.1) * locals.var_dnm) + (assign78760_e119151 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign78760_e119155;
        locals.var_tmf0_dn0 = assign78760_e119155_d_n0;
        locals.var_tmf0_dn2 = assign78760_e119155_d_n2;
        locals.var_tmf0_dn4 = assign78760_e119155_d_n4;
        locals.var_tmf0_dn5 = assign78760_e119155_d_n5;
        locals.var_tmf0_dn6 = assign78760_e119155_d_n6;
        locals.var_tmf0_dn7 = assign78760_e119155_d_n7;
        locals.var_tmf0_dn8 = assign78760_e119155_d_n8;
        locals.var_tmf0_dn9 = assign78760_e119155_d_n9;
        locals.var_tmf0_dn10 = assign78760_e119155_d_n10;
        locals.var_tmf0_dn11 = assign78760_e119155_d_n11;
        locals.var_tmf0_dn14 = assign78760_e119155_d_n14;

        let (assign78770_e119174, assign78770_e119174_d_n0, assign78770_e119174_d_n2, assign78770_e119174_d_n4, assign78770_e119174_d_n5, assign78770_e119174_d_n6, assign78770_e119174_d_n7, assign78770_e119174_d_n8, assign78770_e119174_d_n9, assign78770_e119174_d_n10, assign78770_e119174_d_n11, assign78770_e119174_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        let assign78770_e119168: f64 = (0.1 * locals.var_xmp);
        let assign78770_e119170: f64 = (assign78770_e119168 * locals.var_dnm);
        let assign78770_e119172: f64 = (assign78770_e119170 / locals.var_arg);
        (assign78770_e119172, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign78770_e119168 * locals.var_dnm_dn0)) * locals.var_arg) - (assign78770_e119170 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign78770_e119168 * locals.var_dnm_dn2)) * locals.var_arg) - (assign78770_e119170 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign78770_e119168 * locals.var_dnm_dn4)) * locals.var_arg) - (assign78770_e119170 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign78770_e119168 * locals.var_dnm_dn5)) * locals.var_arg) - (assign78770_e119170 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign78770_e119168 * locals.var_dnm_dn6)) * locals.var_arg) - (assign78770_e119170 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign78770_e119168 * locals.var_dnm_dn7)) * locals.var_arg) - (assign78770_e119170 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign78770_e119168 * locals.var_dnm_dn8)) * locals.var_arg) - (assign78770_e119170 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign78770_e119168 * locals.var_dnm_dn9)) * locals.var_arg) - (assign78770_e119170 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign78770_e119168 * locals.var_dnm_dn10)) * locals.var_arg) - (assign78770_e119170 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn11) * locals.var_dnm) + (assign78770_e119168 * locals.var_dnm_dn11)) * locals.var_arg) - (assign78770_e119170 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn14) * locals.var_dnm) + (assign78770_e119168 * locals.var_dnm_dn14)) * locals.var_arg) - (assign78770_e119170 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign78770_e119174;
        locals.var_t0_dn0 = assign78770_e119174_d_n0;
        locals.var_t0_dn2 = assign78770_e119174_d_n2;
        locals.var_t0_dn4 = assign78770_e119174_d_n4;
        locals.var_t0_dn5 = assign78770_e119174_d_n5;
        locals.var_t0_dn6 = assign78770_e119174_d_n6;
        locals.var_t0_dn7 = assign78770_e119174_d_n7;
        locals.var_t0_dn8 = assign78770_e119174_d_n8;
        locals.var_t0_dn9 = assign78770_e119174_d_n9;
        locals.var_t0_dn10 = assign78770_e119174_d_n10;
        locals.var_t0_dn11 = assign78770_e119174_d_n11;
        locals.var_t0_dn14 = assign78770_e119174_d_n14;

        let (assign78780_e119191, assign78780_e119191_d_n0, assign78780_e119191_d_n2, assign78780_e119191_d_n4, assign78780_e119191_d_n5, assign78780_e119191_d_n6, assign78780_e119191_d_n7, assign78780_e119191_d_n8, assign78780_e119191_d_n9, assign78780_e119191_d_n10, assign78780_e119191_d_n11, assign78780_e119191_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        let assign78780_e119187: f64 = (locals.var_ps0ld_bef1__blk1811 - 0.1);
        let assign78780_e119189: f64 = (assign78780_e119187 + locals.var_tmf0);
        (assign78780_e119189, (locals.var_ps0ld_bef1__blk1811_dn0 + locals.var_tmf0_dn0), (locals.var_ps0ld_bef1__blk1811_dn2 + locals.var_tmf0_dn2), (locals.var_ps0ld_bef1__blk1811_dn4 + locals.var_tmf0_dn4), (locals.var_ps0ld_bef1__blk1811_dn5 + locals.var_tmf0_dn5), (locals.var_ps0ld_bef1__blk1811_dn6 + locals.var_tmf0_dn6), (locals.var_ps0ld_bef1__blk1811_dn7 + locals.var_tmf0_dn7), (locals.var_ps0ld_bef1__blk1811_dn8 + locals.var_tmf0_dn8), (locals.var_ps0ld_bef1__blk1811_dn9 + locals.var_tmf0_dn9), (locals.var_ps0ld_bef1__blk1811_dn10 + locals.var_tmf0_dn10), (locals.var_ps0ld_bef1__blk1811_dn11 + locals.var_tmf0_dn11), (locals.var_ps0ld_bef1__blk1811_dn14 + locals.var_tmf0_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign78780_e119191;
        locals.var_ps0ld_dn0 = assign78780_e119191_d_n0;
        locals.var_ps0ld_dn2 = assign78780_e119191_d_n2;
        locals.var_ps0ld_dn4 = assign78780_e119191_d_n4;
        locals.var_ps0ld_dn5 = assign78780_e119191_d_n5;
        locals.var_ps0ld_dn6 = assign78780_e119191_d_n6;
        locals.var_ps0ld_dn7 = assign78780_e119191_d_n7;
        locals.var_ps0ld_dn8 = assign78780_e119191_d_n8;
        locals.var_ps0ld_dn9 = assign78780_e119191_d_n9;
        locals.var_ps0ld_dn10 = assign78780_e119191_d_n10;
        locals.var_ps0ld_dn11 = assign78780_e119191_d_n11;
        locals.var_ps0ld_dn14 = assign78780_e119191_d_n14;

        let (assign78790_e119204, assign78790_e119204_d_n0, assign78790_e119204_d_n2, assign78790_e119204_d_n4, assign78790_e119204_d_n5, assign78790_e119204_d_n6, assign78790_e119204_d_n7, assign78790_e119204_d_n8, assign78790_e119204_d_n9, assign78790_e119204_d_n10, assign78790_e119204_d_n11, assign78790_e119204_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign78790_e119204;
        locals.var_t0_dn0 = assign78790_e119204_d_n0;
        locals.var_t0_dn2 = assign78790_e119204_d_n2;
        locals.var_t0_dn4 = assign78790_e119204_d_n4;
        locals.var_t0_dn5 = assign78790_e119204_d_n5;
        locals.var_t0_dn6 = assign78790_e119204_d_n6;
        locals.var_t0_dn7 = assign78790_e119204_d_n7;
        locals.var_t0_dn8 = assign78790_e119204_d_n8;
        locals.var_t0_dn9 = assign78790_e119204_d_n9;
        locals.var_t0_dn10 = assign78790_e119204_d_n10;
        locals.var_t0_dn11 = assign78790_e119204_d_n11;
        locals.var_t0_dn14 = assign78790_e119204_d_n14;

        let (assign78800_e119218, assign78800_e119218_d_n0, assign78800_e119218_d_n2, assign78800_e119218_d_n4, assign78800_e119218_d_n5, assign78800_e119218_d_n6, assign78800_e119218_d_n7, assign78800_e119218_d_n8, assign78800_e119218_d_n9, assign78800_e119218_d_n10, assign78800_e119218_d_n11, assign78800_e119218_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign78800_e119218;
        locals.var_ps0ld_dn0 = assign78800_e119218_d_n0;
        locals.var_ps0ld_dn2 = assign78800_e119218_d_n2;
        locals.var_ps0ld_dn4 = assign78800_e119218_d_n4;
        locals.var_ps0ld_dn5 = assign78800_e119218_d_n5;
        locals.var_ps0ld_dn6 = assign78800_e119218_d_n6;
        locals.var_ps0ld_dn7 = assign78800_e119218_d_n7;
        locals.var_ps0ld_dn8 = assign78800_e119218_d_n8;
        locals.var_ps0ld_dn9 = assign78800_e119218_d_n9;
        locals.var_ps0ld_dn10 = assign78800_e119218_d_n10;
        locals.var_ps0ld_dn11 = assign78800_e119218_d_n11;
        locals.var_ps0ld_dn14 = assign78800_e119218_d_n14;

        let (assign78810_e119232, assign78810_e119232_d_n0, assign78810_e119232_d_n2, assign78810_e119232_d_n4, assign78810_e119232_d_n5, assign78810_e119232_d_n6, assign78810_e119232_d_n7, assign78810_e119232_d_n8, assign78810_e119232_d_n9, assign78810_e119232_d_n10, assign78810_e119232_d_n11, assign78810_e119232_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 != 0.0)) && (locals.var_guard1823 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign78810_e119232;
        locals.var_t0_dn0 = assign78810_e119232_d_n0;
        locals.var_t0_dn2 = assign78810_e119232_d_n2;
        locals.var_t0_dn4 = assign78810_e119232_d_n4;
        locals.var_t0_dn5 = assign78810_e119232_d_n5;
        locals.var_t0_dn6 = assign78810_e119232_d_n6;
        locals.var_t0_dn7 = assign78810_e119232_d_n7;
        locals.var_t0_dn8 = assign78810_e119232_d_n8;
        locals.var_t0_dn9 = assign78810_e119232_d_n9;
        locals.var_t0_dn10 = assign78810_e119232_d_n10;
        locals.var_t0_dn11 = assign78810_e119232_d_n11;
        locals.var_t0_dn14 = assign78810_e119232_d_n14;

        let (assign78820_e119249, assign78820_e119249_d_n0, assign78820_e119249_d_n2, assign78820_e119249_d_n4, assign78820_e119249_d_n5, assign78820_e119249_d_n6, assign78820_e119249_d_n7, assign78820_e119249_d_n8, assign78820_e119249_d_n9, assign78820_e119249_d_n10, assign78820_e119249_d_n11, assign78820_e119249_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1821 != 0.0)) && (locals.var_guard1822 == 0.0)) {
        let (assign78820_e119247, assign78820_e119247_d_n0, assign78820_e119247_d_n2, assign78820_e119247_d_n4, assign78820_e119247_d_n5, assign78820_e119247_d_n6, assign78820_e119247_d_n7, assign78820_e119247_d_n8, assign78820_e119247_d_n9, assign78820_e119247_d_n10, assign78820_e119247_d_n11, assign78820_e119247_d_n14,) = {
            if (locals.var_ps0ld <= locals.var_ps0ld_bef1__blk1811) {
                (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
            } else {
                (locals.var_ps0ld_bef1__blk1811, locals.var_ps0ld_bef1__blk1811_dn0, locals.var_ps0ld_bef1__blk1811_dn2, locals.var_ps0ld_bef1__blk1811_dn4, locals.var_ps0ld_bef1__blk1811_dn5, locals.var_ps0ld_bef1__blk1811_dn6, locals.var_ps0ld_bef1__blk1811_dn7, locals.var_ps0ld_bef1__blk1811_dn8, locals.var_ps0ld_bef1__blk1811_dn9, locals.var_ps0ld_bef1__blk1811_dn10, locals.var_ps0ld_bef1__blk1811_dn11, locals.var_ps0ld_bef1__blk1811_dn14,)
            }
        };
        (assign78820_e119247, assign78820_e119247_d_n0, assign78820_e119247_d_n2, assign78820_e119247_d_n4, assign78820_e119247_d_n5, assign78820_e119247_d_n6, assign78820_e119247_d_n7, assign78820_e119247_d_n8, assign78820_e119247_d_n9, assign78820_e119247_d_n10, assign78820_e119247_d_n11, assign78820_e119247_d_n14,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign78820_e119249;
        locals.var_ps0ld_dn0 = assign78820_e119249_d_n0;
        locals.var_ps0ld_dn2 = assign78820_e119249_d_n2;
        locals.var_ps0ld_dn4 = assign78820_e119249_d_n4;
        locals.var_ps0ld_dn5 = assign78820_e119249_d_n5;
        locals.var_ps0ld_dn6 = assign78820_e119249_d_n6;
        locals.var_ps0ld_dn7 = assign78820_e119249_d_n7;
        locals.var_ps0ld_dn8 = assign78820_e119249_d_n8;
        locals.var_ps0ld_dn9 = assign78820_e119249_d_n9;
        locals.var_ps0ld_dn10 = assign78820_e119249_d_n10;
        locals.var_ps0ld_dn11 = assign78820_e119249_d_n11;
        locals.var_ps0ld_dn14 = assign78820_e119249_d_n14;

        let (assign78830_e119256, assign78830_e119256_d_n0, assign78830_e119256_d_n2, assign78830_e119256_d_n4, assign78830_e119256_d_n5, assign78830_e119256_d_n6, assign78830_e119256_d_n7, assign78830_e119256_d_n8, assign78830_e119256_d_n9, assign78830_e119256_d_n10, assign78830_e119256_d_n11, assign78830_e119256_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    } else {
        (locals.var_ps0ld_ini__blk1773, locals.var_ps0ld_ini__blk1773_dn0, locals.var_ps0ld_ini__blk1773_dn2, locals.var_ps0ld_ini__blk1773_dn4, locals.var_ps0ld_ini__blk1773_dn5, locals.var_ps0ld_ini__blk1773_dn6, locals.var_ps0ld_ini__blk1773_dn7, locals.var_ps0ld_ini__blk1773_dn8, locals.var_ps0ld_ini__blk1773_dn9, locals.var_ps0ld_ini__blk1773_dn10, locals.var_ps0ld_ini__blk1773_dn11, locals.var_ps0ld_ini__blk1773_dn14,)
    }
};
        locals.var_ps0ld_ini__blk1773 = assign78830_e119256;
        locals.var_ps0ld_ini__blk1773_dn0 = assign78830_e119256_d_n0;
        locals.var_ps0ld_ini__blk1773_dn2 = assign78830_e119256_d_n2;
        locals.var_ps0ld_ini__blk1773_dn4 = assign78830_e119256_d_n4;
        locals.var_ps0ld_ini__blk1773_dn5 = assign78830_e119256_d_n5;
        locals.var_ps0ld_ini__blk1773_dn6 = assign78830_e119256_d_n6;
        locals.var_ps0ld_ini__blk1773_dn7 = assign78830_e119256_d_n7;
        locals.var_ps0ld_ini__blk1773_dn8 = assign78830_e119256_d_n8;
        locals.var_ps0ld_ini__blk1773_dn9 = assign78830_e119256_d_n9;
        locals.var_ps0ld_ini__blk1773_dn10 = assign78830_e119256_d_n10;
        locals.var_ps0ld_ini__blk1773_dn11 = assign78830_e119256_d_n11;
        locals.var_ps0ld_ini__blk1773_dn14 = assign78830_e119256_d_n14;

        let assign78840_e119259: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1829 = assign78840_e119259;

        let (assign78850_e119268,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign78850_e119268;

    }

    pub(super) fn stamp_transient_block_284(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign78860_e119284, assign78860_e119284_d_n0, assign78860_e119284_d_n2, assign78860_e119284_d_n4, assign78860_e119284_d_n5, assign78860_e119284_d_n6, assign78860_e119284_d_n7, assign78860_e119284_d_n8, assign78860_e119284_d_n9, assign78860_e119284_d_n10, assign78860_e119284_d_n11, assign78860_e119284_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign78860_e119278: f64 = (1.034943e-10 / locals.var_q_nsubld__blk1766);
        let assign78860_e119280: f64 = (assign78860_e119278 * locals.var_beta_inv);
        let assign78860_e119281: f64 = (2.0 * assign78860_e119280);
        let assign78860_e119282: f64 = (assign78860_e119281).sqrt();
        (assign78860_e119282, ((2.0 * (assign78860_e119278 * locals.var_beta_inv_dn0)) / (2.0 * assign78860_e119282)), ((2.0 * (assign78860_e119278 * locals.var_beta_inv_dn2)) / (2.0 * assign78860_e119282)), ((2.0 * (assign78860_e119278 * locals.var_beta_inv_dn4)) / (2.0 * assign78860_e119282)), ((2.0 * (assign78860_e119278 * locals.var_beta_inv_dn5)) / (2.0 * assign78860_e119282)), ((2.0 * (assign78860_e119278 * locals.var_beta_inv_dn6)) / (2.0 * assign78860_e119282)), ((2.0 * (assign78860_e119278 * locals.var_beta_inv_dn7)) / (2.0 * assign78860_e119282)), ((2.0 * (assign78860_e119278 * locals.var_beta_inv_dn8)) / (2.0 * assign78860_e119282)), ((2.0 * (assign78860_e119278 * locals.var_beta_inv_dn9)) / (2.0 * assign78860_e119282)), ((2.0 * (assign78860_e119278 * locals.var_beta_inv_dn10)) / (2.0 * assign78860_e119282)), ((2.0 * (assign78860_e119278 * locals.var_beta_inv_dn11)) / (2.0 * assign78860_e119282)), ((2.0 * (assign78860_e119278 * locals.var_beta_inv_dn14)) / (2.0 * assign78860_e119282)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn11, locals.var_c_w_ld_dn14,)
    }
};
        locals.var_c_w_ld = assign78860_e119284;
        locals.var_c_w_ld_dn0 = assign78860_e119284_d_n0;
        locals.var_c_w_ld_dn2 = assign78860_e119284_d_n2;
        locals.var_c_w_ld_dn4 = assign78860_e119284_d_n4;
        locals.var_c_w_ld_dn5 = assign78860_e119284_d_n5;
        locals.var_c_w_ld_dn6 = assign78860_e119284_d_n6;
        locals.var_c_w_ld_dn7 = assign78860_e119284_d_n7;
        locals.var_c_w_ld_dn8 = assign78860_e119284_d_n8;
        locals.var_c_w_ld_dn9 = assign78860_e119284_d_n9;
        locals.var_c_w_ld_dn10 = assign78860_e119284_d_n10;
        locals.var_c_w_ld_dn11 = assign78860_e119284_d_n11;
        locals.var_c_w_ld_dn14 = assign78860_e119284_d_n14;

        let assign78870_e119287: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1830 = assign78870_e119287;

        let (assign78880_e119300, assign78880_e119300_d_n0, assign78880_e119300_d_n2, assign78880_e119300_d_n4, assign78880_e119300_d_n5, assign78880_e119300_d_n6, assign78880_e119300_d_n7, assign78880_e119300_d_n8, assign78880_e119300_d_n9, assign78880_e119300_d_n10, assign78880_e119300_d_n11, assign78880_e119300_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1830 != 0.0)) {
        let assign78880_e119298: f64 = (p.p334 - locals.var_wdep_func);
        (assign78880_e119298, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn11), (-locals.var_wdep_func_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign78880_e119300;
        locals.var_t2_dn0 = assign78880_e119300_d_n0;
        locals.var_t2_dn2 = assign78880_e119300_d_n2;
        locals.var_t2_dn4 = assign78880_e119300_d_n4;
        locals.var_t2_dn5 = assign78880_e119300_d_n5;
        locals.var_t2_dn6 = assign78880_e119300_d_n6;
        locals.var_t2_dn7 = assign78880_e119300_d_n7;
        locals.var_t2_dn8 = assign78880_e119300_d_n8;
        locals.var_t2_dn9 = assign78880_e119300_d_n9;
        locals.var_t2_dn10 = assign78880_e119300_d_n10;
        locals.var_t2_dn11 = assign78880_e119300_d_n11;
        locals.var_t2_dn14 = assign78880_e119300_d_n14;

        let (assign78890_e119325, assign78890_e119325_d_n0, assign78890_e119325_d_n2, assign78890_e119325_d_n4, assign78890_e119325_d_n5, assign78890_e119325_d_n6, assign78890_e119325_d_n7, assign78890_e119325_d_n8, assign78890_e119325_d_n9, assign78890_e119325_d_n10, assign78890_e119325_d_n11, assign78890_e119325_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1830 == 0.0)) {
        let assign78890_e119312: f64 = (locals.var_vdsi + p.p137);
        let assign78890_e119315: f64 = (locals.var_vdsi + p.p137);
        let assign78890_e119316: f64 = (assign78890_e119312 * assign78890_e119315);
        let assign78890_e119319: f64 = (4.0 * 0.1);
        let assign78890_e119321: f64 = (assign78890_e119319 * 0.1);
        let assign78890_e119322: f64 = (assign78890_e119316 + assign78890_e119321);
        let assign78890_e119323: f64 = (assign78890_e119322).sqrt();
        (assign78890_e119323, 0.0, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn6 * assign78890_e119315) + (assign78890_e119312 * locals.var_vdsi_dn6)) / (2.0 * assign78890_e119323)), 0.0, (((locals.var_vdsi_dn8 * assign78890_e119315) + (assign78890_e119312 * locals.var_vdsi_dn8)) / (2.0 * assign78890_e119323)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign78890_e119325;
        locals.var_tmf2_dn0 = assign78890_e119325_d_n0;
        locals.var_tmf2_dn2 = assign78890_e119325_d_n2;
        locals.var_tmf2_dn4 = assign78890_e119325_d_n4;
        locals.var_tmf2_dn5 = assign78890_e119325_d_n5;
        locals.var_tmf2_dn6 = assign78890_e119325_d_n6;
        locals.var_tmf2_dn7 = assign78890_e119325_d_n7;
        locals.var_tmf2_dn8 = assign78890_e119325_d_n8;
        locals.var_tmf2_dn9 = assign78890_e119325_d_n9;
        locals.var_tmf2_dn10 = assign78890_e119325_d_n10;
        locals.var_tmf2_dn11 = assign78890_e119325_d_n11;
        locals.var_tmf2_dn14 = assign78890_e119325_d_n14;

        let (assign78900_e119345, assign78900_e119345_d_n0, assign78900_e119345_d_n2, assign78900_e119345_d_n4, assign78900_e119345_d_n5, assign78900_e119345_d_n6, assign78900_e119345_d_n7, assign78900_e119345_d_n8, assign78900_e119345_d_n9, assign78900_e119345_d_n10, assign78900_e119345_d_n11, assign78900_e119345_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1830 == 0.0)) {
        let assign78900_e119339: f64 = (locals.var_vdsi + p.p137);
        let assign78900_e119341: f64 = (assign78900_e119339 / locals.var_tmf2);
        let assign78900_e119342: f64 = (1.0 + assign78900_e119341);
        let assign78900_e119343: f64 = (0.5 * assign78900_e119342);
        (assign78900_e119343, (0.5 * (-((assign78900_e119339 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign78900_e119339 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign78900_e119339 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign78900_e119339 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn6 * locals.var_tmf2) - (assign78900_e119339 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign78900_e119339 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn8 * locals.var_tmf2) - (assign78900_e119339 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign78900_e119339 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign78900_e119339 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign78900_e119339 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign78900_e119339 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign78900_e119345;
        locals.var_t9_dn0 = assign78900_e119345_d_n0;
        locals.var_t9_dn2 = assign78900_e119345_d_n2;
        locals.var_t9_dn4 = assign78900_e119345_d_n4;
        locals.var_t9_dn5 = assign78900_e119345_d_n5;
        locals.var_t9_dn6 = assign78900_e119345_d_n6;
        locals.var_t9_dn7 = assign78900_e119345_d_n7;
        locals.var_t9_dn8 = assign78900_e119345_d_n8;
        locals.var_t9_dn9 = assign78900_e119345_d_n9;
        locals.var_t9_dn10 = assign78900_e119345_d_n10;
        locals.var_t9_dn11 = assign78900_e119345_d_n11;
        locals.var_t9_dn14 = assign78900_e119345_d_n14;

        let (assign78910_e119363, assign78910_e119363_d_n0, assign78910_e119363_d_n2, assign78910_e119363_d_n4, assign78910_e119363_d_n5, assign78910_e119363_d_n6, assign78910_e119363_d_n7, assign78910_e119363_d_n8, assign78910_e119363_d_n9, assign78910_e119363_d_n10, assign78910_e119363_d_n11, assign78910_e119363_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1830 == 0.0)) {
        let assign78910_e119358: f64 = (locals.var_vdsi + p.p137);
        let assign78910_e119360: f64 = (assign78910_e119358 + locals.var_tmf2);
        let assign78910_e119361: f64 = (0.5 * assign78910_e119360);
        (assign78910_e119361, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * (locals.var_vdsi_dn6 + locals.var_tmf2_dn6)), (0.5 * locals.var_tmf2_dn7), (0.5 * (locals.var_vdsi_dn8 + locals.var_tmf2_dn8)), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign78910_e119363;
        locals.var_t2_dn0 = assign78910_e119363_d_n0;
        locals.var_t2_dn2 = assign78910_e119363_d_n2;
        locals.var_t2_dn4 = assign78910_e119363_d_n4;
        locals.var_t2_dn5 = assign78910_e119363_d_n5;
        locals.var_t2_dn6 = assign78910_e119363_d_n6;
        locals.var_t2_dn7 = assign78910_e119363_d_n7;
        locals.var_t2_dn8 = assign78910_e119363_d_n8;
        locals.var_t2_dn9 = assign78910_e119363_d_n9;
        locals.var_t2_dn10 = assign78910_e119363_d_n10;
        locals.var_t2_dn11 = assign78910_e119363_d_n11;
        locals.var_t2_dn14 = assign78910_e119363_d_n14;

        let assign78920_e119366: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1831 = assign78920_e119366;

        let (assign78930_e119380, assign78930_e119380_d_n0, assign78930_e119380_d_n2, assign78930_e119380_d_n4, assign78930_e119380_d_n5, assign78930_e119380_d_n6, assign78930_e119380_d_n7, assign78930_e119380_d_n8, assign78930_e119380_d_n9, assign78930_e119380_d_n10, assign78930_e119380_d_n11, assign78930_e119380_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1830 == 0.0)) && (locals.var_guard1831 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign78930_e119380;
        locals.var_t2_dn0 = assign78930_e119380_d_n0;
        locals.var_t2_dn2 = assign78930_e119380_d_n2;
        locals.var_t2_dn4 = assign78930_e119380_d_n4;
        locals.var_t2_dn5 = assign78930_e119380_d_n5;
        locals.var_t2_dn6 = assign78930_e119380_d_n6;
        locals.var_t2_dn7 = assign78930_e119380_d_n7;
        locals.var_t2_dn8 = assign78930_e119380_d_n8;
        locals.var_t2_dn9 = assign78930_e119380_d_n9;
        locals.var_t2_dn10 = assign78930_e119380_d_n10;
        locals.var_t2_dn11 = assign78930_e119380_d_n11;
        locals.var_t2_dn14 = assign78930_e119380_d_n14;

        let (assign78940_e119394, assign78940_e119394_d_n0, assign78940_e119394_d_n2, assign78940_e119394_d_n4, assign78940_e119394_d_n5, assign78940_e119394_d_n6, assign78940_e119394_d_n7, assign78940_e119394_d_n8, assign78940_e119394_d_n9, assign78940_e119394_d_n10, assign78940_e119394_d_n11, assign78940_e119394_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1830 == 0.0)) && (locals.var_guard1831 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign78940_e119394;
        locals.var_t9_dn0 = assign78940_e119394_d_n0;
        locals.var_t9_dn2 = assign78940_e119394_d_n2;
        locals.var_t9_dn4 = assign78940_e119394_d_n4;
        locals.var_t9_dn5 = assign78940_e119394_d_n5;
        locals.var_t9_dn6 = assign78940_e119394_d_n6;
        locals.var_t9_dn7 = assign78940_e119394_d_n7;
        locals.var_t9_dn8 = assign78940_e119394_d_n8;
        locals.var_t9_dn9 = assign78940_e119394_d_n9;
        locals.var_t9_dn10 = assign78940_e119394_d_n10;
        locals.var_t9_dn11 = assign78940_e119394_d_n11;
        locals.var_t9_dn14 = assign78940_e119394_d_n14;

        let (assign78950_e119411, assign78950_e119411_d_n0, assign78950_e119411_d_n2, assign78950_e119411_d_n4, assign78950_e119411_d_n5, assign78950_e119411_d_n6, assign78950_e119411_d_n7, assign78950_e119411_d_n8, assign78950_e119411_d_n9, assign78950_e119411_d_n10, assign78950_e119411_d_n11, assign78950_e119411_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1830 == 0.0)) {
        let assign78950_e119406: f64 = (locals.var_kjunc * locals.var_t2);
        let assign78950_e119407: f64 = (assign78950_e119406).sqrt();
        let assign78950_e119409: f64 = (assign78950_e119407 * p.p432);
        (assign78950_e119409, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign78950_e119407)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign78950_e119407)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign78950_e119407)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign78950_e119407)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign78950_e119407)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign78950_e119407)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign78950_e119407)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign78950_e119407)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign78950_e119407)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign78950_e119407)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign78950_e119407)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign78950_e119411;
        locals.var_wjunc0_dn0 = assign78950_e119411_d_n0;
        locals.var_wjunc0_dn2 = assign78950_e119411_d_n2;
        locals.var_wjunc0_dn4 = assign78950_e119411_d_n4;
        locals.var_wjunc0_dn5 = assign78950_e119411_d_n5;
        locals.var_wjunc0_dn6 = assign78950_e119411_d_n6;
        locals.var_wjunc0_dn7 = assign78950_e119411_d_n7;
        locals.var_wjunc0_dn8 = assign78950_e119411_d_n8;
        locals.var_wjunc0_dn9 = assign78950_e119411_d_n9;
        locals.var_wjunc0_dn10 = assign78950_e119411_d_n10;
        locals.var_wjunc0_dn11 = assign78950_e119411_d_n11;
        locals.var_wjunc0_dn14 = assign78950_e119411_d_n14;

        let (assign78960_e119425, assign78960_e119425_d_n0, assign78960_e119425_d_n2, assign78960_e119425_d_n4, assign78960_e119425_d_n5, assign78960_e119425_d_n6, assign78960_e119425_d_n7, assign78960_e119425_d_n8, assign78960_e119425_d_n9, assign78960_e119425_d_n10, assign78960_e119425_d_n11, assign78960_e119425_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1830 == 0.0)) {
        let assign78960_e119423: f64 = (p.p334 - locals.var_wjunc0);
        (assign78960_e119423, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign78960_e119425;
        locals.var_t2_dn0 = assign78960_e119425_d_n0;
        locals.var_t2_dn2 = assign78960_e119425_d_n2;
        locals.var_t2_dn4 = assign78960_e119425_d_n4;
        locals.var_t2_dn5 = assign78960_e119425_d_n5;
        locals.var_t2_dn6 = assign78960_e119425_d_n6;
        locals.var_t2_dn7 = assign78960_e119425_d_n7;
        locals.var_t2_dn8 = assign78960_e119425_d_n8;
        locals.var_t2_dn9 = assign78960_e119425_d_n9;
        locals.var_t2_dn10 = assign78960_e119425_d_n10;
        locals.var_t2_dn11 = assign78960_e119425_d_n11;
        locals.var_t2_dn14 = assign78960_e119425_d_n14;

        let (assign78970_e119447, assign78970_e119447_d_n0, assign78970_e119447_d_n2, assign78970_e119447_d_n4, assign78970_e119447_d_n5, assign78970_e119447_d_n6, assign78970_e119447_d_n7, assign78970_e119447_d_n8, assign78970_e119447_d_n9, assign78970_e119447_d_n10, assign78970_e119447_d_n11, assign78970_e119447_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign78970_e119434: f64 = (locals.var_t2 * locals.var_t2);
        let assign78970_e119438: f64 = (p.p334 * 0.01);
        let assign78970_e119439: f64 = (4.0 * assign78970_e119438);
        let assign78970_e119442: f64 = (p.p334 * 0.01);
        let assign78970_e119443: f64 = (assign78970_e119439 * assign78970_e119442);
        let assign78970_e119444: f64 = (assign78970_e119434 + assign78970_e119443);
        let assign78970_e119445: f64 = (assign78970_e119444).sqrt();
        (assign78970_e119445, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign78970_e119445)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign78970_e119445)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign78970_e119445)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign78970_e119445)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign78970_e119445)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign78970_e119445)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign78970_e119445)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign78970_e119445)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign78970_e119445)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign78970_e119445)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign78970_e119445)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign78970_e119447;
        locals.var_tmf2_dn0 = assign78970_e119447_d_n0;
        locals.var_tmf2_dn2 = assign78970_e119447_d_n2;
        locals.var_tmf2_dn4 = assign78970_e119447_d_n4;
        locals.var_tmf2_dn5 = assign78970_e119447_d_n5;
        locals.var_tmf2_dn6 = assign78970_e119447_d_n6;
        locals.var_tmf2_dn7 = assign78970_e119447_d_n7;
        locals.var_tmf2_dn8 = assign78970_e119447_d_n8;
        locals.var_tmf2_dn9 = assign78970_e119447_d_n9;
        locals.var_tmf2_dn10 = assign78970_e119447_d_n10;
        locals.var_tmf2_dn11 = assign78970_e119447_d_n11;
        locals.var_tmf2_dn14 = assign78970_e119447_d_n14;

        let (assign78980_e119462, assign78980_e119462_d_n0, assign78980_e119462_d_n2, assign78980_e119462_d_n4, assign78980_e119462_d_n5, assign78980_e119462_d_n6, assign78980_e119462_d_n7, assign78980_e119462_d_n8, assign78980_e119462_d_n9, assign78980_e119462_d_n10, assign78980_e119462_d_n11, assign78980_e119462_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign78980_e119458: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign78980_e119459: f64 = (1.0 + assign78980_e119458);
        let assign78980_e119460: f64 = (0.5 * assign78980_e119459);
        (assign78980_e119460, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign78980_e119462;
        locals.var_t9_dn0 = assign78980_e119462_d_n0;
        locals.var_t9_dn2 = assign78980_e119462_d_n2;
        locals.var_t9_dn4 = assign78980_e119462_d_n4;
        locals.var_t9_dn5 = assign78980_e119462_d_n5;
        locals.var_t9_dn6 = assign78980_e119462_d_n6;
        locals.var_t9_dn7 = assign78980_e119462_d_n7;
        locals.var_t9_dn8 = assign78980_e119462_d_n8;
        locals.var_t9_dn9 = assign78980_e119462_d_n9;
        locals.var_t9_dn10 = assign78980_e119462_d_n10;
        locals.var_t9_dn11 = assign78980_e119462_d_n11;
        locals.var_t9_dn14 = assign78980_e119462_d_n14;

        let (assign78990_e119475, assign78990_e119475_d_n0, assign78990_e119475_d_n2, assign78990_e119475_d_n4, assign78990_e119475_d_n5, assign78990_e119475_d_n6, assign78990_e119475_d_n7, assign78990_e119475_d_n8, assign78990_e119475_d_n9, assign78990_e119475_d_n10, assign78990_e119475_d_n11, assign78990_e119475_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign78990_e119472: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign78990_e119473: f64 = (0.5 * assign78990_e119472);
        (assign78990_e119473, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign78990_e119475;
        locals.var_t2_dn0 = assign78990_e119475_d_n0;
        locals.var_t2_dn2 = assign78990_e119475_d_n2;
        locals.var_t2_dn4 = assign78990_e119475_d_n4;
        locals.var_t2_dn5 = assign78990_e119475_d_n5;
        locals.var_t2_dn6 = assign78990_e119475_d_n6;
        locals.var_t2_dn7 = assign78990_e119475_d_n7;
        locals.var_t2_dn8 = assign78990_e119475_d_n8;
        locals.var_t2_dn9 = assign78990_e119475_d_n9;
        locals.var_t2_dn10 = assign78990_e119475_d_n10;
        locals.var_t2_dn11 = assign78990_e119475_d_n11;
        locals.var_t2_dn14 = assign78990_e119475_d_n14;

        let assign79000_e119478: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1832 = assign79000_e119478;

        let (assign79010_e119489, assign79010_e119489_d_n0, assign79010_e119489_d_n2, assign79010_e119489_d_n4, assign79010_e119489_d_n5, assign79010_e119489_d_n6, assign79010_e119489_d_n7, assign79010_e119489_d_n8, assign79010_e119489_d_n9, assign79010_e119489_d_n10, assign79010_e119489_d_n11, assign79010_e119489_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1832 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign79010_e119489;
        locals.var_t2_dn0 = assign79010_e119489_d_n0;
        locals.var_t2_dn2 = assign79010_e119489_d_n2;
        locals.var_t2_dn4 = assign79010_e119489_d_n4;
        locals.var_t2_dn5 = assign79010_e119489_d_n5;
        locals.var_t2_dn6 = assign79010_e119489_d_n6;
        locals.var_t2_dn7 = assign79010_e119489_d_n7;
        locals.var_t2_dn8 = assign79010_e119489_d_n8;
        locals.var_t2_dn9 = assign79010_e119489_d_n9;
        locals.var_t2_dn10 = assign79010_e119489_d_n10;
        locals.var_t2_dn11 = assign79010_e119489_d_n11;
        locals.var_t2_dn14 = assign79010_e119489_d_n14;

        let (assign79020_e119500, assign79020_e119500_d_n0, assign79020_e119500_d_n2, assign79020_e119500_d_n4, assign79020_e119500_d_n5, assign79020_e119500_d_n6, assign79020_e119500_d_n7, assign79020_e119500_d_n8, assign79020_e119500_d_n9, assign79020_e119500_d_n10, assign79020_e119500_d_n11, assign79020_e119500_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1832 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign79020_e119500;
        locals.var_t9_dn0 = assign79020_e119500_d_n0;
        locals.var_t9_dn2 = assign79020_e119500_d_n2;
        locals.var_t9_dn4 = assign79020_e119500_d_n4;
        locals.var_t9_dn5 = assign79020_e119500_d_n5;
        locals.var_t9_dn6 = assign79020_e119500_d_n6;
        locals.var_t9_dn7 = assign79020_e119500_d_n7;
        locals.var_t9_dn8 = assign79020_e119500_d_n8;
        locals.var_t9_dn9 = assign79020_e119500_d_n9;
        locals.var_t9_dn10 = assign79020_e119500_d_n10;
        locals.var_t9_dn11 = assign79020_e119500_d_n11;
        locals.var_t9_dn14 = assign79020_e119500_d_n14;

        let (assign79030_e119509, assign79030_e119509_d_n0, assign79030_e119509_d_n2, assign79030_e119509_d_n4, assign79030_e119509_d_n5, assign79030_e119509_d_n6, assign79030_e119509_d_n7, assign79030_e119509_d_n8, assign79030_e119509_d_n9, assign79030_e119509_d_n10, assign79030_e119509_d_n11, assign79030_e119509_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign79030_e119509;
        locals.var_ddriftldc_dn0 = assign79030_e119509_d_n0;
        locals.var_ddriftldc_dn2 = assign79030_e119509_d_n2;
        locals.var_ddriftldc_dn4 = assign79030_e119509_d_n4;
        locals.var_ddriftldc_dn5 = assign79030_e119509_d_n5;
        locals.var_ddriftldc_dn6 = assign79030_e119509_d_n6;
        locals.var_ddriftldc_dn7 = assign79030_e119509_d_n7;
        locals.var_ddriftldc_dn8 = assign79030_e119509_d_n8;
        locals.var_ddriftldc_dn9 = assign79030_e119509_d_n9;
        locals.var_ddriftldc_dn10 = assign79030_e119509_d_n10;
        locals.var_ddriftldc_dn11 = assign79030_e119509_d_n11;
        locals.var_ddriftldc_dn14 = assign79030_e119509_d_n14;

        let (assign79040_e119526, assign79040_e119526_d_n0, assign79040_e119526_d_n2, assign79040_e119526_d_n4, assign79040_e119526_d_n5, assign79040_e119526_d_n6, assign79040_e119526_d_n7, assign79040_e119526_d_n8, assign79040_e119526_d_n9, assign79040_e119526_d_n10, assign79040_e119526_d_n11, assign79040_e119526_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign79040_e119518: f64 = (locals.var_q_nsubld__blk1766 * locals.var_ddriftldc);
        let assign79040_e119520: f64 = (assign79040_e119518 * locals.var_ddriftldc);
        let assign79040_e119522: f64 = (assign79040_e119520 / 2.0);
        let assign79040_e119524: f64 = (assign79040_e119522 / 1.034943e-10);
        (assign79040_e119524, (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign79040_e119518 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign79040_e119518 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign79040_e119518 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign79040_e119518 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign79040_e119518 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign79040_e119518 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign79040_e119518 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign79040_e119518 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign79040_e119518 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign79040_e119518 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign79040_e119518 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign79040_e119526;
        locals.var_dphi_sb_dn0 = assign79040_e119526_d_n0;
        locals.var_dphi_sb_dn2 = assign79040_e119526_d_n2;
        locals.var_dphi_sb_dn4 = assign79040_e119526_d_n4;
        locals.var_dphi_sb_dn5 = assign79040_e119526_d_n5;
        locals.var_dphi_sb_dn6 = assign79040_e119526_d_n6;
        locals.var_dphi_sb_dn7 = assign79040_e119526_d_n7;
        locals.var_dphi_sb_dn8 = assign79040_e119526_d_n8;
        locals.var_dphi_sb_dn9 = assign79040_e119526_d_n9;
        locals.var_dphi_sb_dn10 = assign79040_e119526_d_n10;
        locals.var_dphi_sb_dn11 = assign79040_e119526_d_n11;
        locals.var_dphi_sb_dn14 = assign79040_e119526_d_n14;

        let (assign79050_e119540, assign79050_e119540_d_n0, assign79050_e119540_d_n2, assign79050_e119540_d_n4, assign79050_e119540_d_n5, assign79050_e119540_d_n6, assign79050_e119540_d_n7, assign79050_e119540_d_n8, assign79050_e119540_d_n9, assign79050_e119540_d_n10, assign79050_e119540_d_n11, assign79050_e119540_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign79050_e119535: f64 = (2.0 * locals.var_beta);
        let assign79050_e119537: f64 = (assign79050_e119535 * locals.var_dphi_sb);
        let assign79050_e119538: f64 = (assign79050_e119537).sqrt();
        (assign79050_e119538, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign79050_e119535 * locals.var_dphi_sb_dn0)) / (2.0 * assign79050_e119538)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign79050_e119535 * locals.var_dphi_sb_dn2)) / (2.0 * assign79050_e119538)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign79050_e119535 * locals.var_dphi_sb_dn4)) / (2.0 * assign79050_e119538)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign79050_e119535 * locals.var_dphi_sb_dn5)) / (2.0 * assign79050_e119538)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign79050_e119535 * locals.var_dphi_sb_dn6)) / (2.0 * assign79050_e119538)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign79050_e119535 * locals.var_dphi_sb_dn7)) / (2.0 * assign79050_e119538)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign79050_e119535 * locals.var_dphi_sb_dn8)) / (2.0 * assign79050_e119538)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign79050_e119535 * locals.var_dphi_sb_dn9)) / (2.0 * assign79050_e119538)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign79050_e119535 * locals.var_dphi_sb_dn10)) / (2.0 * assign79050_e119538)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign79050_e119535 * locals.var_dphi_sb_dn11)) / (2.0 * assign79050_e119538)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign79050_e119535 * locals.var_dphi_sb_dn14)) / (2.0 * assign79050_e119538)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign79050_e119540;
        locals.var_t0_dn0 = assign79050_e119540_d_n0;
        locals.var_t0_dn2 = assign79050_e119540_d_n2;
        locals.var_t0_dn4 = assign79050_e119540_d_n4;
        locals.var_t0_dn5 = assign79050_e119540_d_n5;
        locals.var_t0_dn6 = assign79050_e119540_d_n6;
        locals.var_t0_dn7 = assign79050_e119540_d_n7;
        locals.var_t0_dn8 = assign79050_e119540_d_n8;
        locals.var_t0_dn9 = assign79050_e119540_d_n9;
        locals.var_t0_dn10 = assign79050_e119540_d_n10;
        locals.var_t0_dn11 = assign79050_e119540_d_n11;
        locals.var_t0_dn14 = assign79050_e119540_d_n14;

        let (assign79060_e119556, assign79060_e119556_d_n0, assign79060_e119556_d_n2, assign79060_e119556_d_n4, assign79060_e119556_d_n5, assign79060_e119556_d_n6, assign79060_e119556_d_n7, assign79060_e119556_d_n8, assign79060_e119556_d_n9, assign79060_e119556_d_n10, assign79060_e119556_d_n11, assign79060_e119556_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign79060_e119548: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign79060_e119550: f64 = (-locals.var_t0);
        let assign79060_e119551: f64 = { let limited_exp_arg = assign79060_e119550; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign79060_e119552: f64 = (assign79060_e119548 + assign79060_e119551);
        let assign79060_e119554: f64 = (assign79060_e119552 / 2.0);
        (assign79060_e119554, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign79060_e119550; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign79060_e119550; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign79060_e119550; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign79060_e119550; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign79060_e119550; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign79060_e119550; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign79060_e119550; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign79060_e119550; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign79060_e119550; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign79060_e119550; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign79060_e119550; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign79060_e119556;
        locals.var_t1_dn0 = assign79060_e119556_d_n0;
        locals.var_t1_dn2 = assign79060_e119556_d_n2;
        locals.var_t1_dn4 = assign79060_e119556_d_n4;
        locals.var_t1_dn5 = assign79060_e119556_d_n5;
        locals.var_t1_dn6 = assign79060_e119556_d_n6;
        locals.var_t1_dn7 = assign79060_e119556_d_n7;
        locals.var_t1_dn8 = assign79060_e119556_d_n8;
        locals.var_t1_dn9 = assign79060_e119556_d_n9;
        locals.var_t1_dn10 = assign79060_e119556_d_n10;
        locals.var_t1_dn11 = assign79060_e119556_d_n11;
        locals.var_t1_dn14 = assign79060_e119556_d_n14;

        let (assign79070_e119568, assign79070_e119568_d_n0, assign79070_e119568_d_n2, assign79070_e119568_d_n4, assign79070_e119568_d_n5, assign79070_e119568_d_n6, assign79070_e119568_d_n7, assign79070_e119568_d_n8, assign79070_e119568_d_n9, assign79070_e119568_d_n10, assign79070_e119568_d_n11, assign79070_e119568_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign79070_e119564: f64 = (locals.var_t1).ln();
        let assign79070_e119566: f64 = (assign79070_e119564 / locals.var_dphi_sb);
        (assign79070_e119566, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign79070_e119564 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign79070_e119564 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign79070_e119564 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign79070_e119564 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign79070_e119564 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign79070_e119564 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign79070_e119564 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign79070_e119564 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign79070_e119564 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign79070_e119564 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign79070_e119564 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign79070_e119568;
        locals.var_c_sb_dn0 = assign79070_e119568_d_n0;
        locals.var_c_sb_dn2 = assign79070_e119568_d_n2;
        locals.var_c_sb_dn4 = assign79070_e119568_d_n4;
        locals.var_c_sb_dn5 = assign79070_e119568_d_n5;
        locals.var_c_sb_dn6 = assign79070_e119568_d_n6;
        locals.var_c_sb_dn7 = assign79070_e119568_d_n7;
        locals.var_c_sb_dn8 = assign79070_e119568_d_n8;
        locals.var_c_sb_dn9 = assign79070_e119568_d_n9;
        locals.var_c_sb_dn10 = assign79070_e119568_d_n10;
        locals.var_c_sb_dn11 = assign79070_e119568_d_n11;
        locals.var_c_sb_dn14 = assign79070_e119568_d_n14;

        let (assign79080_e119577,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign79080_e119577;

    }

    pub(super) fn stamp_transient_block_285(
        locals: &mut StampLocals,
    ) {
        let mut assign79090_loop_guard: usize = 0;
        while {
            let assign79090_cond_e119587: f64 = (locals.var_lp_s0_max + 1.0);
            let assign79090_cond_e119589: f64 = if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_lp_s0 <= assign79090_cond_e119587)) { 1.0 } else { 0.0 };
            assign79090_cond_e119589 != 0.0
        } {
            assign79090_loop_guard += 1;
            assert!(assign79090_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign79090_body3_e119625, assign79090_body3_e119625_d_n0, assign79090_body3_e119625_d_n2, assign79090_body3_e119625_d_n4, assign79090_body3_e119625_d_n5, assign79090_body3_e119625_d_n6, assign79090_body3_e119625_d_n7, assign79090_body3_e119625_d_n8, assign79090_body3_e119625_d_n9, assign79090_body3_e119625_d_n10, assign79090_body3_e119625_d_n11, assign79090_body3_e119625_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign79090_body3_e119623: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        (assign79090_body3_e119623, (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_ps0ld_dn4 + locals.var_vxbgmtcl_dn4), (locals.var_ps0ld_dn5 + locals.var_vxbgmtcl_dn5), (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_ps0ld_dn8 + locals.var_vxbgmtcl_dn8), (locals.var_ps0ld_dn9 + locals.var_vxbgmtcl_dn9), (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_ps0ld_dn14 + locals.var_vxbgmtcl_dn14),)
    } else {
        (locals.var_ps0ld_vxb, locals.var_ps0ld_vxb_dn0, locals.var_ps0ld_vxb_dn2, locals.var_ps0ld_vxb_dn4, locals.var_ps0ld_vxb_dn5, locals.var_ps0ld_vxb_dn6, locals.var_ps0ld_vxb_dn7, locals.var_ps0ld_vxb_dn8, locals.var_ps0ld_vxb_dn9, locals.var_ps0ld_vxb_dn10, locals.var_ps0ld_vxb_dn11, locals.var_ps0ld_vxb_dn14,)
    }
};
            locals.var_ps0ld_vxb = assign79090_body3_e119625;
            locals.var_ps0ld_vxb_dn0 = assign79090_body3_e119625_d_n0;
            locals.var_ps0ld_vxb_dn2 = assign79090_body3_e119625_d_n2;
            locals.var_ps0ld_vxb_dn4 = assign79090_body3_e119625_d_n4;
            locals.var_ps0ld_vxb_dn5 = assign79090_body3_e119625_d_n5;
            locals.var_ps0ld_vxb_dn6 = assign79090_body3_e119625_d_n6;
            locals.var_ps0ld_vxb_dn7 = assign79090_body3_e119625_d_n7;
            locals.var_ps0ld_vxb_dn8 = assign79090_body3_e119625_d_n8;
            locals.var_ps0ld_vxb_dn9 = assign79090_body3_e119625_d_n9;
            locals.var_ps0ld_vxb_dn10 = assign79090_body3_e119625_d_n10;
            locals.var_ps0ld_vxb_dn11 = assign79090_body3_e119625_d_n11;
            locals.var_ps0ld_vxb_dn14 = assign79090_body3_e119625_d_n14;
            let (assign79090_body4_e119636, assign79090_body4_e119636_d_n0, assign79090_body4_e119636_d_n2, assign79090_body4_e119636_d_n4, assign79090_body4_e119636_d_n5, assign79090_body4_e119636_d_n6, assign79090_body4_e119636_d_n7, assign79090_body4_e119636_d_n8, assign79090_body4_e119636_d_n9, assign79090_body4_e119636_d_n10, assign79090_body4_e119636_d_n11, assign79090_body4_e119636_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign79090_body4_e119634: f64 = (locals.var_beta * locals.var_ps0ld_vxb);
        (assign79090_body4_e119634, ((locals.var_beta_dn0 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn0)), ((locals.var_beta_dn2 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn2)), ((locals.var_beta_dn4 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn4)), ((locals.var_beta_dn5 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn5)), ((locals.var_beta_dn6 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn6)), ((locals.var_beta_dn7 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn7)), ((locals.var_beta_dn8 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn8)), ((locals.var_beta_dn9 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn9)), ((locals.var_beta_dn10 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn10)), ((locals.var_beta_dn11 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn11)), ((locals.var_beta_dn14 * locals.var_ps0ld_vxb) + (locals.var_beta * locals.var_ps0ld_vxb_dn14)),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
            locals.var_chi = assign79090_body4_e119636;
            locals.var_chi_dn0 = assign79090_body4_e119636_d_n0;
            locals.var_chi_dn2 = assign79090_body4_e119636_d_n2;
            locals.var_chi_dn4 = assign79090_body4_e119636_d_n4;
            locals.var_chi_dn5 = assign79090_body4_e119636_d_n5;
            locals.var_chi_dn6 = assign79090_body4_e119636_d_n6;
            locals.var_chi_dn7 = assign79090_body4_e119636_d_n7;
            locals.var_chi_dn8 = assign79090_body4_e119636_d_n8;
            locals.var_chi_dn9 = assign79090_body4_e119636_d_n9;
            locals.var_chi_dn10 = assign79090_body4_e119636_d_n10;
            locals.var_chi_dn11 = assign79090_body4_e119636_d_n11;
            locals.var_chi_dn14 = assign79090_body4_e119636_d_n14;
            let (assign79090_body5_e119649, assign79090_body5_e119649_d_n0, assign79090_body5_e119649_d_n2, assign79090_body5_e119649_d_n4, assign79090_body5_e119649_d_n5, assign79090_body5_e119649_d_n6, assign79090_body5_e119649_d_n7, assign79090_body5_e119649_d_n8, assign79090_body5_e119649_d_n9, assign79090_body5_e119649_d_n10, assign79090_body5_e119649_d_n11, assign79090_body5_e119649_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign79090_body5_e119646: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        let assign79090_body5_e119647: f64 = (locals.var_c_sb * assign79090_body5_e119646);
        (assign79090_body5_e119647, ((locals.var_c_sb_dn0 * assign79090_body5_e119646) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0))), ((locals.var_c_sb_dn2 * assign79090_body5_e119646) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2))), ((locals.var_c_sb_dn4 * assign79090_body5_e119646) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4))), ((locals.var_c_sb_dn5 * assign79090_body5_e119646) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5))), ((locals.var_c_sb_dn6 * assign79090_body5_e119646) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6))), ((locals.var_c_sb_dn7 * assign79090_body5_e119646) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7))), ((locals.var_c_sb_dn8 * assign79090_body5_e119646) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8))), ((locals.var_c_sb_dn9 * assign79090_body5_e119646) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9))), ((locals.var_c_sb_dn10 * assign79090_body5_e119646) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10))), ((locals.var_c_sb_dn11 * assign79090_body5_e119646) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11))), ((locals.var_c_sb_dn14 * assign79090_body5_e119646) + (locals.var_c_sb * (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
            locals.var_ty = assign79090_body5_e119649;
            locals.var_ty_dn0 = assign79090_body5_e119649_d_n0;
            locals.var_ty_dn2 = assign79090_body5_e119649_d_n2;
            locals.var_ty_dn4 = assign79090_body5_e119649_d_n4;
            locals.var_ty_dn5 = assign79090_body5_e119649_d_n5;
            locals.var_ty_dn6 = assign79090_body5_e119649_d_n6;
            locals.var_ty_dn7 = assign79090_body5_e119649_d_n7;
            locals.var_ty_dn8 = assign79090_body5_e119649_d_n8;
            locals.var_ty_dn9 = assign79090_body5_e119649_d_n9;
            locals.var_ty_dn10 = assign79090_body5_e119649_d_n10;
            locals.var_ty_dn11 = assign79090_body5_e119649_d_n11;
            locals.var_ty_dn14 = assign79090_body5_e119649_d_n14;
            let assign79090_body6_e119652: f64 = if locals.var_ty < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1834 = assign79090_body6_e119652;
            let (assign79090_body7_e119664, assign79090_body7_e119664_d_n0, assign79090_body7_e119664_d_n2, assign79090_body7_e119664_d_n4, assign79090_body7_e119664_d_n5, assign79090_body7_e119664_d_n6, assign79090_body7_e119664_d_n7, assign79090_body7_e119664_d_n8, assign79090_body7_e119664_d_n9, assign79090_body7_e119664_d_n10, assign79090_body7_e119664_d_n11, assign79090_body7_e119664_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1834 != 0.0)) {
        let assign79090_body7_e119662: f64 = (locals.var_ty).exp();
        (assign79090_body7_e119662, (assign79090_body7_e119662 * locals.var_ty_dn0), (assign79090_body7_e119662 * locals.var_ty_dn2), (assign79090_body7_e119662 * locals.var_ty_dn4), (assign79090_body7_e119662 * locals.var_ty_dn5), (assign79090_body7_e119662 * locals.var_ty_dn6), (assign79090_body7_e119662 * locals.var_ty_dn7), (assign79090_body7_e119662 * locals.var_ty_dn8), (assign79090_body7_e119662 * locals.var_ty_dn9), (assign79090_body7_e119662 * locals.var_ty_dn10), (assign79090_body7_e119662 * locals.var_ty_dn11), (assign79090_body7_e119662 * locals.var_ty_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign79090_body7_e119664;
            locals.var_t1_dn0 = assign79090_body7_e119664_d_n0;
            locals.var_t1_dn2 = assign79090_body7_e119664_d_n2;
            locals.var_t1_dn4 = assign79090_body7_e119664_d_n4;
            locals.var_t1_dn5 = assign79090_body7_e119664_d_n5;
            locals.var_t1_dn6 = assign79090_body7_e119664_d_n6;
            locals.var_t1_dn7 = assign79090_body7_e119664_d_n7;
            locals.var_t1_dn8 = assign79090_body7_e119664_d_n8;
            locals.var_t1_dn9 = assign79090_body7_e119664_d_n9;
            locals.var_t1_dn10 = assign79090_body7_e119664_d_n10;
            locals.var_t1_dn11 = assign79090_body7_e119664_d_n11;
            locals.var_t1_dn14 = assign79090_body7_e119664_d_n14;
            let (assign79090_body8_e119679, assign79090_body8_e119679_d_n0, assign79090_body8_e119679_d_n2, assign79090_body8_e119679_d_n4, assign79090_body8_e119679_d_n5, assign79090_body8_e119679_d_n6, assign79090_body8_e119679_d_n7, assign79090_body8_e119679_d_n8, assign79090_body8_e119679_d_n9, assign79090_body8_e119679_d_n10, assign79090_body8_e119679_d_n11, assign79090_body8_e119679_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1834 != 0.0)) {
        let assign79090_body8_e119674: f64 = (-locals.var_c_sb);
        let assign79090_body8_e119676: f64 = (assign79090_body8_e119674 * locals.var_dphi_sb);
        let assign79090_body8_e119677: f64 = (assign79090_body8_e119676).exp();
        (assign79090_body8_e119677, (assign79090_body8_e119677 * (((-locals.var_c_sb_dn0) * locals.var_dphi_sb) + (assign79090_body8_e119674 * locals.var_dphi_sb_dn0))), (assign79090_body8_e119677 * (((-locals.var_c_sb_dn2) * locals.var_dphi_sb) + (assign79090_body8_e119674 * locals.var_dphi_sb_dn2))), (assign79090_body8_e119677 * (((-locals.var_c_sb_dn4) * locals.var_dphi_sb) + (assign79090_body8_e119674 * locals.var_dphi_sb_dn4))), (assign79090_body8_e119677 * (((-locals.var_c_sb_dn5) * locals.var_dphi_sb) + (assign79090_body8_e119674 * locals.var_dphi_sb_dn5))), (assign79090_body8_e119677 * (((-locals.var_c_sb_dn6) * locals.var_dphi_sb) + (assign79090_body8_e119674 * locals.var_dphi_sb_dn6))), (assign79090_body8_e119677 * (((-locals.var_c_sb_dn7) * locals.var_dphi_sb) + (assign79090_body8_e119674 * locals.var_dphi_sb_dn7))), (assign79090_body8_e119677 * (((-locals.var_c_sb_dn8) * locals.var_dphi_sb) + (assign79090_body8_e119674 * locals.var_dphi_sb_dn8))), (assign79090_body8_e119677 * (((-locals.var_c_sb_dn9) * locals.var_dphi_sb) + (assign79090_body8_e119674 * locals.var_dphi_sb_dn9))), (assign79090_body8_e119677 * (((-locals.var_c_sb_dn10) * locals.var_dphi_sb) + (assign79090_body8_e119674 * locals.var_dphi_sb_dn10))), (assign79090_body8_e119677 * (((-locals.var_c_sb_dn11) * locals.var_dphi_sb) + (assign79090_body8_e119674 * locals.var_dphi_sb_dn11))), (assign79090_body8_e119677 * (((-locals.var_c_sb_dn14) * locals.var_dphi_sb) + (assign79090_body8_e119674 * locals.var_dphi_sb_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign79090_body8_e119679;
            locals.var_t0_dn0 = assign79090_body8_e119679_d_n0;
            locals.var_t0_dn2 = assign79090_body8_e119679_d_n2;
            locals.var_t0_dn4 = assign79090_body8_e119679_d_n4;
            locals.var_t0_dn5 = assign79090_body8_e119679_d_n5;
            locals.var_t0_dn6 = assign79090_body8_e119679_d_n6;
            locals.var_t0_dn7 = assign79090_body8_e119679_d_n7;
            locals.var_t0_dn8 = assign79090_body8_e119679_d_n8;
            locals.var_t0_dn9 = assign79090_body8_e119679_d_n9;
            locals.var_t0_dn10 = assign79090_body8_e119679_d_n10;
            locals.var_t0_dn11 = assign79090_body8_e119679_d_n11;
            locals.var_t0_dn14 = assign79090_body8_e119679_d_n14;
            let (assign79090_body9_e119692, assign79090_body9_e119692_d_n0, assign79090_body9_e119692_d_n2, assign79090_body9_e119692_d_n4, assign79090_body9_e119692_d_n5, assign79090_body9_e119692_d_n6, assign79090_body9_e119692_d_n7, assign79090_body9_e119692_d_n8, assign79090_body9_e119692_d_n9, assign79090_body9_e119692_d_n10, assign79090_body9_e119692_d_n11, assign79090_body9_e119692_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1834 != 0.0)) {
        let assign79090_body9_e119690: f64 = (locals.var_t1 - locals.var_t0);
        (assign79090_body9_e119690, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign79090_body9_e119692;
            locals.var_t2_dn0 = assign79090_body9_e119692_d_n0;
            locals.var_t2_dn2 = assign79090_body9_e119692_d_n2;
            locals.var_t2_dn4 = assign79090_body9_e119692_d_n4;
            locals.var_t2_dn5 = assign79090_body9_e119692_d_n5;
            locals.var_t2_dn6 = assign79090_body9_e119692_d_n6;
            locals.var_t2_dn7 = assign79090_body9_e119692_d_n7;
            locals.var_t2_dn8 = assign79090_body9_e119692_d_n8;
            locals.var_t2_dn9 = assign79090_body9_e119692_d_n9;
            locals.var_t2_dn10 = assign79090_body9_e119692_d_n10;
            locals.var_t2_dn11 = assign79090_body9_e119692_d_n11;
            locals.var_t2_dn14 = assign79090_body9_e119692_d_n14;
            let (assign79090_body10_e119708, assign79090_body10_e119708_d_n0, assign79090_body10_e119708_d_n2, assign79090_body10_e119708_d_n4, assign79090_body10_e119708_d_n5, assign79090_body10_e119708_d_n6, assign79090_body10_e119708_d_n7, assign79090_body10_e119708_d_n8, assign79090_body10_e119708_d_n9, assign79090_body10_e119708_d_n10, assign79090_body10_e119708_d_n11, assign79090_body10_e119708_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1834 != 0.0)) {
        let assign79090_body10_e119703: f64 = (1.0 + locals.var_t2);
        let assign79090_body10_e119704: f64 = (assign79090_body10_e119703).ln();
        let assign79090_body10_e119706: f64 = (assign79090_body10_e119704 / locals.var_c_sb);
        (assign79090_body10_e119706, ((((locals.var_t2_dn0 / assign79090_body10_e119703) * locals.var_c_sb) - (assign79090_body10_e119704 * locals.var_c_sb_dn0)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn2 / assign79090_body10_e119703) * locals.var_c_sb) - (assign79090_body10_e119704 * locals.var_c_sb_dn2)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn4 / assign79090_body10_e119703) * locals.var_c_sb) - (assign79090_body10_e119704 * locals.var_c_sb_dn4)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn5 / assign79090_body10_e119703) * locals.var_c_sb) - (assign79090_body10_e119704 * locals.var_c_sb_dn5)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn6 / assign79090_body10_e119703) * locals.var_c_sb) - (assign79090_body10_e119704 * locals.var_c_sb_dn6)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn7 / assign79090_body10_e119703) * locals.var_c_sb) - (assign79090_body10_e119704 * locals.var_c_sb_dn7)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn8 / assign79090_body10_e119703) * locals.var_c_sb) - (assign79090_body10_e119704 * locals.var_c_sb_dn8)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn9 / assign79090_body10_e119703) * locals.var_c_sb) - (assign79090_body10_e119704 * locals.var_c_sb_dn9)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn10 / assign79090_body10_e119703) * locals.var_c_sb) - (assign79090_body10_e119704 * locals.var_c_sb_dn10)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn11 / assign79090_body10_e119703) * locals.var_c_sb) - (assign79090_body10_e119704 * locals.var_c_sb_dn11)) / (locals.var_c_sb * locals.var_c_sb)), ((((locals.var_t2_dn14 / assign79090_body10_e119703) * locals.var_c_sb) - (assign79090_body10_e119704 * locals.var_c_sb_dn14)) / (locals.var_c_sb * locals.var_c_sb)),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign79090_body10_e119708;
            locals.var_phi_b_dn0 = assign79090_body10_e119708_d_n0;
            locals.var_phi_b_dn2 = assign79090_body10_e119708_d_n2;
            locals.var_phi_b_dn4 = assign79090_body10_e119708_d_n4;
            locals.var_phi_b_dn5 = assign79090_body10_e119708_d_n5;
            locals.var_phi_b_dn6 = assign79090_body10_e119708_d_n6;
            locals.var_phi_b_dn7 = assign79090_body10_e119708_d_n7;
            locals.var_phi_b_dn8 = assign79090_body10_e119708_d_n8;
            locals.var_phi_b_dn9 = assign79090_body10_e119708_d_n9;
            locals.var_phi_b_dn10 = assign79090_body10_e119708_d_n10;
            locals.var_phi_b_dn11 = assign79090_body10_e119708_d_n11;
            locals.var_phi_b_dn14 = assign79090_body10_e119708_d_n14;
            let (assign79090_body11_e119723, assign79090_body11_e119723_d_n0, assign79090_body11_e119723_d_n2, assign79090_body11_e119723_d_n4, assign79090_body11_e119723_d_n5, assign79090_body11_e119723_d_n6, assign79090_body11_e119723_d_n7, assign79090_body11_e119723_d_n8, assign79090_body11_e119723_d_n9, assign79090_body11_e119723_d_n10, assign79090_body11_e119723_d_n11, assign79090_body11_e119723_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1834 != 0.0)) {
        let assign79090_body11_e119720: f64 = (1.0 + locals.var_t2);
        let assign79090_body11_e119721: f64 = (locals.var_t1 / assign79090_body11_e119720);
        (assign79090_body11_e119721, (((locals.var_t1_dn0 * assign79090_body11_e119720) - (locals.var_t1 * locals.var_t2_dn0)) / (assign79090_body11_e119720 * assign79090_body11_e119720)), (((locals.var_t1_dn2 * assign79090_body11_e119720) - (locals.var_t1 * locals.var_t2_dn2)) / (assign79090_body11_e119720 * assign79090_body11_e119720)), (((locals.var_t1_dn4 * assign79090_body11_e119720) - (locals.var_t1 * locals.var_t2_dn4)) / (assign79090_body11_e119720 * assign79090_body11_e119720)), (((locals.var_t1_dn5 * assign79090_body11_e119720) - (locals.var_t1 * locals.var_t2_dn5)) / (assign79090_body11_e119720 * assign79090_body11_e119720)), (((locals.var_t1_dn6 * assign79090_body11_e119720) - (locals.var_t1 * locals.var_t2_dn6)) / (assign79090_body11_e119720 * assign79090_body11_e119720)), (((locals.var_t1_dn7 * assign79090_body11_e119720) - (locals.var_t1 * locals.var_t2_dn7)) / (assign79090_body11_e119720 * assign79090_body11_e119720)), (((locals.var_t1_dn8 * assign79090_body11_e119720) - (locals.var_t1 * locals.var_t2_dn8)) / (assign79090_body11_e119720 * assign79090_body11_e119720)), (((locals.var_t1_dn9 * assign79090_body11_e119720) - (locals.var_t1 * locals.var_t2_dn9)) / (assign79090_body11_e119720 * assign79090_body11_e119720)), (((locals.var_t1_dn10 * assign79090_body11_e119720) - (locals.var_t1 * locals.var_t2_dn10)) / (assign79090_body11_e119720 * assign79090_body11_e119720)), (((locals.var_t1_dn11 * assign79090_body11_e119720) - (locals.var_t1 * locals.var_t2_dn11)) / (assign79090_body11_e119720 * assign79090_body11_e119720)), (((locals.var_t1_dn14 * assign79090_body11_e119720) - (locals.var_t1 * locals.var_t2_dn14)) / (assign79090_body11_e119720 * assign79090_body11_e119720)),)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign79090_body11_e119723;
            locals.var_phi_b_dpss_dn0 = assign79090_body11_e119723_d_n0;
            locals.var_phi_b_dpss_dn2 = assign79090_body11_e119723_d_n2;
            locals.var_phi_b_dpss_dn4 = assign79090_body11_e119723_d_n4;
            locals.var_phi_b_dpss_dn5 = assign79090_body11_e119723_d_n5;
            locals.var_phi_b_dpss_dn6 = assign79090_body11_e119723_d_n6;
            locals.var_phi_b_dpss_dn7 = assign79090_body11_e119723_d_n7;
            locals.var_phi_b_dpss_dn8 = assign79090_body11_e119723_d_n8;
            locals.var_phi_b_dpss_dn9 = assign79090_body11_e119723_d_n9;
            locals.var_phi_b_dpss_dn10 = assign79090_body11_e119723_d_n10;
            locals.var_phi_b_dpss_dn11 = assign79090_body11_e119723_d_n11;
            locals.var_phi_b_dpss_dn14 = assign79090_body11_e119723_d_n14;
            let (assign79090_body13_e119751, assign79090_body13_e119751_d_n0, assign79090_body13_e119751_d_n2, assign79090_body13_e119751_d_n4, assign79090_body13_e119751_d_n5, assign79090_body13_e119751_d_n6, assign79090_body13_e119751_d_n7, assign79090_body13_e119751_d_n8, assign79090_body13_e119751_d_n9, assign79090_body13_e119751_d_n10, assign79090_body13_e119751_d_n11, assign79090_body13_e119751_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1834 == 0.0)) {
        let assign79090_body13_e119749: f64 = (locals.var_ps0ld_vxb - locals.var_dphi_sb);
        (assign79090_body13_e119749, (locals.var_ps0ld_vxb_dn0 - locals.var_dphi_sb_dn0), (locals.var_ps0ld_vxb_dn2 - locals.var_dphi_sb_dn2), (locals.var_ps0ld_vxb_dn4 - locals.var_dphi_sb_dn4), (locals.var_ps0ld_vxb_dn5 - locals.var_dphi_sb_dn5), (locals.var_ps0ld_vxb_dn6 - locals.var_dphi_sb_dn6), (locals.var_ps0ld_vxb_dn7 - locals.var_dphi_sb_dn7), (locals.var_ps0ld_vxb_dn8 - locals.var_dphi_sb_dn8), (locals.var_ps0ld_vxb_dn9 - locals.var_dphi_sb_dn9), (locals.var_ps0ld_vxb_dn10 - locals.var_dphi_sb_dn10), (locals.var_ps0ld_vxb_dn11 - locals.var_dphi_sb_dn11), (locals.var_ps0ld_vxb_dn14 - locals.var_dphi_sb_dn14),)
    } else {
        (locals.var_phi_b, locals.var_phi_b_dn0, locals.var_phi_b_dn2, locals.var_phi_b_dn4, locals.var_phi_b_dn5, locals.var_phi_b_dn6, locals.var_phi_b_dn7, locals.var_phi_b_dn8, locals.var_phi_b_dn9, locals.var_phi_b_dn10, locals.var_phi_b_dn11, locals.var_phi_b_dn14,)
    }
};
            locals.var_phi_b = assign79090_body13_e119751;
            locals.var_phi_b_dn0 = assign79090_body13_e119751_d_n0;
            locals.var_phi_b_dn2 = assign79090_body13_e119751_d_n2;
            locals.var_phi_b_dn4 = assign79090_body13_e119751_d_n4;
            locals.var_phi_b_dn5 = assign79090_body13_e119751_d_n5;
            locals.var_phi_b_dn6 = assign79090_body13_e119751_d_n6;
            locals.var_phi_b_dn7 = assign79090_body13_e119751_d_n7;
            locals.var_phi_b_dn8 = assign79090_body13_e119751_d_n8;
            locals.var_phi_b_dn9 = assign79090_body13_e119751_d_n9;
            locals.var_phi_b_dn10 = assign79090_body13_e119751_d_n10;
            locals.var_phi_b_dn11 = assign79090_body13_e119751_d_n11;
            locals.var_phi_b_dn14 = assign79090_body13_e119751_d_n14;
            let (assign79090_body14_e119763, assign79090_body14_e119763_d_n0, assign79090_body14_e119763_d_n2, assign79090_body14_e119763_d_n4, assign79090_body14_e119763_d_n5, assign79090_body14_e119763_d_n6, assign79090_body14_e119763_d_n7, assign79090_body14_e119763_d_n8, assign79090_body14_e119763_d_n9, assign79090_body14_e119763_d_n10, assign79090_body14_e119763_d_n11, assign79090_body14_e119763_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1834 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b_dpss, locals.var_phi_b_dpss_dn0, locals.var_phi_b_dpss_dn2, locals.var_phi_b_dpss_dn4, locals.var_phi_b_dpss_dn5, locals.var_phi_b_dpss_dn6, locals.var_phi_b_dpss_dn7, locals.var_phi_b_dpss_dn8, locals.var_phi_b_dpss_dn9, locals.var_phi_b_dpss_dn10, locals.var_phi_b_dpss_dn11, locals.var_phi_b_dpss_dn14,)
    }
};
            locals.var_phi_b_dpss = assign79090_body14_e119763;
            locals.var_phi_b_dpss_dn0 = assign79090_body14_e119763_d_n0;
            locals.var_phi_b_dpss_dn2 = assign79090_body14_e119763_d_n2;
            locals.var_phi_b_dpss_dn4 = assign79090_body14_e119763_d_n4;
            locals.var_phi_b_dpss_dn5 = assign79090_body14_e119763_d_n5;
            locals.var_phi_b_dpss_dn6 = assign79090_body14_e119763_d_n6;
            locals.var_phi_b_dpss_dn7 = assign79090_body14_e119763_d_n7;
            locals.var_phi_b_dpss_dn8 = assign79090_body14_e119763_d_n8;
            locals.var_phi_b_dpss_dn9 = assign79090_body14_e119763_d_n9;
            locals.var_phi_b_dpss_dn10 = assign79090_body14_e119763_d_n10;
            locals.var_phi_b_dpss_dn11 = assign79090_body14_e119763_d_n11;
            locals.var_phi_b_dpss_dn14 = assign79090_body14_e119763_d_n14;
            let (assign79090_body15_e119774, assign79090_body15_e119774_d_n0, assign79090_body15_e119774_d_n2, assign79090_body15_e119774_d_n4, assign79090_body15_e119774_d_n5, assign79090_body15_e119774_d_n6, assign79090_body15_e119774_d_n7, assign79090_body15_e119774_d_n8, assign79090_body15_e119774_d_n9, assign79090_body15_e119774_d_n10, assign79090_body15_e119774_d_n11, assign79090_body15_e119774_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign79090_body15_e119772: f64 = (locals.var_beta * locals.var_phi_b);
        (assign79090_body15_e119772, ((locals.var_beta_dn0 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn0)), ((locals.var_beta_dn2 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn2)), ((locals.var_beta_dn4 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn4)), ((locals.var_beta_dn5 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn5)), ((locals.var_beta_dn6 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn6)), ((locals.var_beta_dn7 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn7)), ((locals.var_beta_dn8 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn8)), ((locals.var_beta_dn9 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn9)), ((locals.var_beta_dn10 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn10)), ((locals.var_beta_dn11 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn11)), ((locals.var_beta_dn14 * locals.var_phi_b) + (locals.var_beta * locals.var_phi_b_dn14)),)
    } else {
        (locals.var_chib, locals.var_chib_dn0, locals.var_chib_dn2, locals.var_chib_dn4, locals.var_chib_dn5, locals.var_chib_dn6, locals.var_chib_dn7, locals.var_chib_dn8, locals.var_chib_dn9, locals.var_chib_dn10, locals.var_chib_dn11, locals.var_chib_dn14,)
    }
};
            locals.var_chib = assign79090_body15_e119774;
            locals.var_chib_dn0 = assign79090_body15_e119774_d_n0;
            locals.var_chib_dn2 = assign79090_body15_e119774_d_n2;
            locals.var_chib_dn4 = assign79090_body15_e119774_d_n4;
            locals.var_chib_dn5 = assign79090_body15_e119774_d_n5;
            locals.var_chib_dn6 = assign79090_body15_e119774_d_n6;
            locals.var_chib_dn7 = assign79090_body15_e119774_d_n7;
            locals.var_chib_dn8 = assign79090_body15_e119774_d_n8;
            locals.var_chib_dn9 = assign79090_body15_e119774_d_n9;
            locals.var_chib_dn10 = assign79090_body15_e119774_d_n10;
            locals.var_chib_dn11 = assign79090_body15_e119774_d_n11;
            locals.var_chib_dn14 = assign79090_body15_e119774_d_n14;
            let assign79090_body16_e119777: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1835 = assign79090_body16_e119777;
            let (assign79090_body18_e119802, assign79090_body18_e119802_d_n0, assign79090_body18_e119802_d_n2, assign79090_body18_e119802_d_n4, assign79090_body18_e119802_d_n5, assign79090_body18_e119802_d_n6, assign79090_body18_e119802_d_n7, assign79090_body18_e119802_d_n8, assign79090_body18_e119802_d_n9, assign79090_body18_e119802_d_n10, assign79090_body18_e119802_d_n11, assign79090_body18_e119802_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1835 != 0.0)) {
        let assign79090_body18_e119800: f64 = (-0.7071067811865475);
        (assign79090_body18_e119800, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign79090_body18_e119802;
            locals.var_t0_dn0 = assign79090_body18_e119802_d_n0;
            locals.var_t0_dn2 = assign79090_body18_e119802_d_n2;
            locals.var_t0_dn4 = assign79090_body18_e119802_d_n4;
            locals.var_t0_dn5 = assign79090_body18_e119802_d_n5;
            locals.var_t0_dn6 = assign79090_body18_e119802_d_n6;
            locals.var_t0_dn7 = assign79090_body18_e119802_d_n7;
            locals.var_t0_dn8 = assign79090_body18_e119802_d_n8;
            locals.var_t0_dn9 = assign79090_body18_e119802_d_n9;
            locals.var_t0_dn10 = assign79090_body18_e119802_d_n10;
            locals.var_t0_dn11 = assign79090_body18_e119802_d_n11;
            locals.var_t0_dn14 = assign79090_body18_e119802_d_n14;
            let (assign79090_body19_e119815, assign79090_body19_e119815_d_n0, assign79090_body19_e119815_d_n2, assign79090_body19_e119815_d_n4, assign79090_body19_e119815_d_n5, assign79090_body19_e119815_d_n6, assign79090_body19_e119815_d_n7, assign79090_body19_e119815_d_n8, assign79090_body19_e119815_d_n9, assign79090_body19_e119815_d_n10, assign79090_body19_e119815_d_n11, assign79090_body19_e119815_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1835 != 0.0)) {
        let assign79090_body19_e119813: f64 = (locals.var_chi * locals.var_t0);
        (assign79090_body19_e119813, ((locals.var_chi_dn0 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn0)), ((locals.var_chi_dn2 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn2)), ((locals.var_chi_dn4 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn4)), ((locals.var_chi_dn5 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn5)), ((locals.var_chi_dn6 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn6)), ((locals.var_chi_dn7 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn7)), ((locals.var_chi_dn8 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn8)), ((locals.var_chi_dn9 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn9)), ((locals.var_chi_dn10 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn10)), ((locals.var_chi_dn11 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn11)), ((locals.var_chi_dn14 * locals.var_t0) + (locals.var_chi * locals.var_t0_dn14)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign79090_body19_e119815;
            locals.var_fb_dn0 = assign79090_body19_e119815_d_n0;
            locals.var_fb_dn2 = assign79090_body19_e119815_d_n2;
            locals.var_fb_dn4 = assign79090_body19_e119815_d_n4;
            locals.var_fb_dn5 = assign79090_body19_e119815_d_n5;
            locals.var_fb_dn6 = assign79090_body19_e119815_d_n6;
            locals.var_fb_dn7 = assign79090_body19_e119815_d_n7;
            locals.var_fb_dn8 = assign79090_body19_e119815_d_n8;
            locals.var_fb_dn9 = assign79090_body19_e119815_d_n9;
            locals.var_fb_dn10 = assign79090_body19_e119815_d_n10;
            locals.var_fb_dn11 = assign79090_body19_e119815_d_n11;
            locals.var_fb_dn14 = assign79090_body19_e119815_d_n14;
            let (assign79090_body20_e119828, assign79090_body20_e119828_d_n0, assign79090_body20_e119828_d_n2, assign79090_body20_e119828_d_n4, assign79090_body20_e119828_d_n5, assign79090_body20_e119828_d_n6, assign79090_body20_e119828_d_n7, assign79090_body20_e119828_d_n8, assign79090_body20_e119828_d_n9, assign79090_body20_e119828_d_n10, assign79090_body20_e119828_d_n11, assign79090_body20_e119828_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1835 != 0.0)) {
        let assign79090_body20_e119826: f64 = (locals.var_beta * locals.var_t0);
        (assign79090_body20_e119826, ((locals.var_beta_dn0 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn0)), ((locals.var_beta_dn2 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn2)), ((locals.var_beta_dn4 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn4)), ((locals.var_beta_dn5 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn5)), ((locals.var_beta_dn6 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn6)), ((locals.var_beta_dn7 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn7)), ((locals.var_beta_dn8 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn8)), ((locals.var_beta_dn9 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn9)), ((locals.var_beta_dn10 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn10)), ((locals.var_beta_dn11 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn11)), ((locals.var_beta_dn14 * locals.var_t0) + (locals.var_beta * locals.var_t0_dn14)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign79090_body20_e119828;
            locals.var_fb_dpss_dn0 = assign79090_body20_e119828_d_n0;
            locals.var_fb_dpss_dn2 = assign79090_body20_e119828_d_n2;
            locals.var_fb_dpss_dn4 = assign79090_body20_e119828_d_n4;
            locals.var_fb_dpss_dn5 = assign79090_body20_e119828_d_n5;
            locals.var_fb_dpss_dn6 = assign79090_body20_e119828_d_n6;
            locals.var_fb_dpss_dn7 = assign79090_body20_e119828_d_n7;
            locals.var_fb_dpss_dn8 = assign79090_body20_e119828_d_n8;
            locals.var_fb_dpss_dn9 = assign79090_body20_e119828_d_n9;
            locals.var_fb_dpss_dn10 = assign79090_body20_e119828_d_n10;
            locals.var_fb_dpss_dn11 = assign79090_body20_e119828_d_n11;
            locals.var_fb_dpss_dn14 = assign79090_body20_e119828_d_n14;
            let assign79090_body21_e119831: f64 = if locals.var_chi < 1e-6 { 1.0 } else { 0.0 };
            locals.var_guard1836 = assign79090_body21_e119831;
            let (assign79090_body23_e119883, assign79090_body23_e119883_d_n0, assign79090_body23_e119883_d_n2, assign79090_body23_e119883_d_n4, assign79090_body23_e119883_d_n5, assign79090_body23_e119883_d_n6, assign79090_body23_e119883_d_n7, assign79090_body23_e119883_d_n8, assign79090_body23_e119883_d_n9, assign79090_body23_e119883_d_n10, assign79090_body23_e119883_d_n11, assign79090_body23_e119883_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) {
        let assign79090_body23_e119861: f64 = (locals.var_chi * locals.var_chi);
        let assign79090_body23_e119863: f64 = (assign79090_body23_e119861 / 2.0);
        let assign79090_body23_e119867: f64 = (locals.var_chi / 3.0);
        let assign79090_body23_e119871: f64 = (locals.var_chi / 4.0);
        let assign79090_body23_e119875: f64 = (locals.var_chi / 5.0);
        let assign79090_body23_e119876: f64 = (1.0 - assign79090_body23_e119875);
        let assign79090_body23_e119877: f64 = (assign79090_body23_e119871 * assign79090_body23_e119876);
        let assign79090_body23_e119878: f64 = (1.0 - assign79090_body23_e119877);
        let assign79090_body23_e119879: f64 = (assign79090_body23_e119867 * assign79090_body23_e119878);
        let assign79090_body23_e119880: f64 = (1.0 - assign79090_body23_e119879);
        let assign79090_body23_e119881: f64 = (assign79090_body23_e119863 * assign79090_body23_e119880);
        (assign79090_body23_e119881, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign79090_body23_e119880) + (assign79090_body23_e119863 * (-(((locals.var_chi_dn0 / 3.0) * assign79090_body23_e119878) + (assign79090_body23_e119867 * (-(((locals.var_chi_dn0 / 4.0) * assign79090_body23_e119876) + (assign79090_body23_e119871 * (-(locals.var_chi_dn0 / 5.0)))))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign79090_body23_e119880) + (assign79090_body23_e119863 * (-(((locals.var_chi_dn2 / 3.0) * assign79090_body23_e119878) + (assign79090_body23_e119867 * (-(((locals.var_chi_dn2 / 4.0) * assign79090_body23_e119876) + (assign79090_body23_e119871 * (-(locals.var_chi_dn2 / 5.0)))))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign79090_body23_e119880) + (assign79090_body23_e119863 * (-(((locals.var_chi_dn4 / 3.0) * assign79090_body23_e119878) + (assign79090_body23_e119867 * (-(((locals.var_chi_dn4 / 4.0) * assign79090_body23_e119876) + (assign79090_body23_e119871 * (-(locals.var_chi_dn4 / 5.0)))))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign79090_body23_e119880) + (assign79090_body23_e119863 * (-(((locals.var_chi_dn5 / 3.0) * assign79090_body23_e119878) + (assign79090_body23_e119867 * (-(((locals.var_chi_dn5 / 4.0) * assign79090_body23_e119876) + (assign79090_body23_e119871 * (-(locals.var_chi_dn5 / 5.0)))))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign79090_body23_e119880) + (assign79090_body23_e119863 * (-(((locals.var_chi_dn6 / 3.0) * assign79090_body23_e119878) + (assign79090_body23_e119867 * (-(((locals.var_chi_dn6 / 4.0) * assign79090_body23_e119876) + (assign79090_body23_e119871 * (-(locals.var_chi_dn6 / 5.0)))))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign79090_body23_e119880) + (assign79090_body23_e119863 * (-(((locals.var_chi_dn7 / 3.0) * assign79090_body23_e119878) + (assign79090_body23_e119867 * (-(((locals.var_chi_dn7 / 4.0) * assign79090_body23_e119876) + (assign79090_body23_e119871 * (-(locals.var_chi_dn7 / 5.0)))))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign79090_body23_e119880) + (assign79090_body23_e119863 * (-(((locals.var_chi_dn8 / 3.0) * assign79090_body23_e119878) + (assign79090_body23_e119867 * (-(((locals.var_chi_dn8 / 4.0) * assign79090_body23_e119876) + (assign79090_body23_e119871 * (-(locals.var_chi_dn8 / 5.0)))))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign79090_body23_e119880) + (assign79090_body23_e119863 * (-(((locals.var_chi_dn9 / 3.0) * assign79090_body23_e119878) + (assign79090_body23_e119867 * (-(((locals.var_chi_dn9 / 4.0) * assign79090_body23_e119876) + (assign79090_body23_e119871 * (-(locals.var_chi_dn9 / 5.0)))))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign79090_body23_e119880) + (assign79090_body23_e119863 * (-(((locals.var_chi_dn10 / 3.0) * assign79090_body23_e119878) + (assign79090_body23_e119867 * (-(((locals.var_chi_dn10 / 4.0) * assign79090_body23_e119876) + (assign79090_body23_e119871 * (-(locals.var_chi_dn10 / 5.0)))))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign79090_body23_e119880) + (assign79090_body23_e119863 * (-(((locals.var_chi_dn11 / 3.0) * assign79090_body23_e119878) + (assign79090_body23_e119867 * (-(((locals.var_chi_dn11 / 4.0) * assign79090_body23_e119876) + (assign79090_body23_e119871 * (-(locals.var_chi_dn11 / 5.0)))))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign79090_body23_e119880) + (assign79090_body23_e119863 * (-(((locals.var_chi_dn14 / 3.0) * assign79090_body23_e119878) + (assign79090_body23_e119867 * (-(((locals.var_chi_dn14 / 4.0) * assign79090_body23_e119876) + (assign79090_body23_e119871 * (-(locals.var_chi_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign79090_body23_e119883;
            locals.var_t0_dn0 = assign79090_body23_e119883_d_n0;
            locals.var_t0_dn2 = assign79090_body23_e119883_d_n2;
            locals.var_t0_dn4 = assign79090_body23_e119883_d_n4;
            locals.var_t0_dn5 = assign79090_body23_e119883_d_n5;
            locals.var_t0_dn6 = assign79090_body23_e119883_d_n6;
            locals.var_t0_dn7 = assign79090_body23_e119883_d_n7;
            locals.var_t0_dn8 = assign79090_body23_e119883_d_n8;
            locals.var_t0_dn9 = assign79090_body23_e119883_d_n9;
            locals.var_t0_dn10 = assign79090_body23_e119883_d_n10;
            locals.var_t0_dn11 = assign79090_body23_e119883_d_n11;
            locals.var_t0_dn14 = assign79090_body23_e119883_d_n14;
            let (assign79090_body24_e119915, assign79090_body24_e119915_d_n0, assign79090_body24_e119915_d_n2, assign79090_body24_e119915_d_n4, assign79090_body24_e119915_d_n5, assign79090_body24_e119915_d_n6, assign79090_body24_e119915_d_n7, assign79090_body24_e119915_d_n8, assign79090_body24_e119915_d_n9, assign79090_body24_e119915_d_n10, assign79090_body24_e119915_d_n11, assign79090_body24_e119915_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) {
        let assign79090_body24_e119899: f64 = (locals.var_chi / 2.0);
        let assign79090_body24_e119903: f64 = (locals.var_chi / 3.0);
        let assign79090_body24_e119907: f64 = (locals.var_chi / 4.0);
        let assign79090_body24_e119908: f64 = (1.0 - assign79090_body24_e119907);
        let assign79090_body24_e119909: f64 = (assign79090_body24_e119903 * assign79090_body24_e119908);
        let assign79090_body24_e119910: f64 = (1.0 - assign79090_body24_e119909);
        let assign79090_body24_e119911: f64 = (assign79090_body24_e119899 * assign79090_body24_e119910);
        let assign79090_body24_e119912: f64 = (1.0 - assign79090_body24_e119911);
        let assign79090_body24_e119913: f64 = (locals.var_chi * assign79090_body24_e119912);
        (assign79090_body24_e119913, ((locals.var_chi_dn0 * assign79090_body24_e119912) + (locals.var_chi * (-(((locals.var_chi_dn0 / 2.0) * assign79090_body24_e119910) + (assign79090_body24_e119899 * (-(((locals.var_chi_dn0 / 3.0) * assign79090_body24_e119908) + (assign79090_body24_e119903 * (-(locals.var_chi_dn0 / 4.0)))))))))), ((locals.var_chi_dn2 * assign79090_body24_e119912) + (locals.var_chi * (-(((locals.var_chi_dn2 / 2.0) * assign79090_body24_e119910) + (assign79090_body24_e119899 * (-(((locals.var_chi_dn2 / 3.0) * assign79090_body24_e119908) + (assign79090_body24_e119903 * (-(locals.var_chi_dn2 / 4.0)))))))))), ((locals.var_chi_dn4 * assign79090_body24_e119912) + (locals.var_chi * (-(((locals.var_chi_dn4 / 2.0) * assign79090_body24_e119910) + (assign79090_body24_e119899 * (-(((locals.var_chi_dn4 / 3.0) * assign79090_body24_e119908) + (assign79090_body24_e119903 * (-(locals.var_chi_dn4 / 4.0)))))))))), ((locals.var_chi_dn5 * assign79090_body24_e119912) + (locals.var_chi * (-(((locals.var_chi_dn5 / 2.0) * assign79090_body24_e119910) + (assign79090_body24_e119899 * (-(((locals.var_chi_dn5 / 3.0) * assign79090_body24_e119908) + (assign79090_body24_e119903 * (-(locals.var_chi_dn5 / 4.0)))))))))), ((locals.var_chi_dn6 * assign79090_body24_e119912) + (locals.var_chi * (-(((locals.var_chi_dn6 / 2.0) * assign79090_body24_e119910) + (assign79090_body24_e119899 * (-(((locals.var_chi_dn6 / 3.0) * assign79090_body24_e119908) + (assign79090_body24_e119903 * (-(locals.var_chi_dn6 / 4.0)))))))))), ((locals.var_chi_dn7 * assign79090_body24_e119912) + (locals.var_chi * (-(((locals.var_chi_dn7 / 2.0) * assign79090_body24_e119910) + (assign79090_body24_e119899 * (-(((locals.var_chi_dn7 / 3.0) * assign79090_body24_e119908) + (assign79090_body24_e119903 * (-(locals.var_chi_dn7 / 4.0)))))))))), ((locals.var_chi_dn8 * assign79090_body24_e119912) + (locals.var_chi * (-(((locals.var_chi_dn8 / 2.0) * assign79090_body24_e119910) + (assign79090_body24_e119899 * (-(((locals.var_chi_dn8 / 3.0) * assign79090_body24_e119908) + (assign79090_body24_e119903 * (-(locals.var_chi_dn8 / 4.0)))))))))), ((locals.var_chi_dn9 * assign79090_body24_e119912) + (locals.var_chi * (-(((locals.var_chi_dn9 / 2.0) * assign79090_body24_e119910) + (assign79090_body24_e119899 * (-(((locals.var_chi_dn9 / 3.0) * assign79090_body24_e119908) + (assign79090_body24_e119903 * (-(locals.var_chi_dn9 / 4.0)))))))))), ((locals.var_chi_dn10 * assign79090_body24_e119912) + (locals.var_chi * (-(((locals.var_chi_dn10 / 2.0) * assign79090_body24_e119910) + (assign79090_body24_e119899 * (-(((locals.var_chi_dn10 / 3.0) * assign79090_body24_e119908) + (assign79090_body24_e119903 * (-(locals.var_chi_dn10 / 4.0)))))))))), ((locals.var_chi_dn11 * assign79090_body24_e119912) + (locals.var_chi * (-(((locals.var_chi_dn11 / 2.0) * assign79090_body24_e119910) + (assign79090_body24_e119899 * (-(((locals.var_chi_dn11 / 3.0) * assign79090_body24_e119908) + (assign79090_body24_e119903 * (-(locals.var_chi_dn11 / 4.0)))))))))), ((locals.var_chi_dn14 * assign79090_body24_e119912) + (locals.var_chi * (-(((locals.var_chi_dn14 / 2.0) * assign79090_body24_e119910) + (assign79090_body24_e119899 * (-(((locals.var_chi_dn14 / 3.0) * assign79090_body24_e119908) + (assign79090_body24_e119903 * (-(locals.var_chi_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign79090_body24_e119915;
            locals.var_t1_dn0 = assign79090_body24_e119915_d_n0;
            locals.var_t1_dn2 = assign79090_body24_e119915_d_n2;
            locals.var_t1_dn4 = assign79090_body24_e119915_d_n4;
            locals.var_t1_dn5 = assign79090_body24_e119915_d_n5;
            locals.var_t1_dn6 = assign79090_body24_e119915_d_n6;
            locals.var_t1_dn7 = assign79090_body24_e119915_d_n7;
            locals.var_t1_dn8 = assign79090_body24_e119915_d_n8;
            locals.var_t1_dn9 = assign79090_body24_e119915_d_n9;
            locals.var_t1_dn10 = assign79090_body24_e119915_d_n10;
            locals.var_t1_dn11 = assign79090_body24_e119915_d_n11;
            locals.var_t1_dn14 = assign79090_body24_e119915_d_n14;
            let (assign79090_body25_e119951, assign79090_body25_e119951_d_n0, assign79090_body25_e119951_d_n2, assign79090_body25_e119951_d_n4, assign79090_body25_e119951_d_n5, assign79090_body25_e119951_d_n6, assign79090_body25_e119951_d_n7, assign79090_body25_e119951_d_n8, assign79090_body25_e119951_d_n9, assign79090_body25_e119951_d_n10, assign79090_body25_e119951_d_n11, assign79090_body25_e119951_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) {
        let assign79090_body25_e119929: f64 = (locals.var_chib * locals.var_chib);
        let assign79090_body25_e119931: f64 = (assign79090_body25_e119929 / 2.0);
        let assign79090_body25_e119935: f64 = (locals.var_chib / 3.0);
        let assign79090_body25_e119939: f64 = (locals.var_chib / 4.0);
        let assign79090_body25_e119943: f64 = (locals.var_chib / 5.0);
        let assign79090_body25_e119944: f64 = (1.0 - assign79090_body25_e119943);
        let assign79090_body25_e119945: f64 = (assign79090_body25_e119939 * assign79090_body25_e119944);
        let assign79090_body25_e119946: f64 = (1.0 - assign79090_body25_e119945);
        let assign79090_body25_e119947: f64 = (assign79090_body25_e119935 * assign79090_body25_e119946);
        let assign79090_body25_e119948: f64 = (1.0 - assign79090_body25_e119947);
        let assign79090_body25_e119949: f64 = (assign79090_body25_e119931 * assign79090_body25_e119948);
        (assign79090_body25_e119949, (((((locals.var_chib_dn0 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn0)) / 2.0) * assign79090_body25_e119948) + (assign79090_body25_e119931 * (-(((locals.var_chib_dn0 / 3.0) * assign79090_body25_e119946) + (assign79090_body25_e119935 * (-(((locals.var_chib_dn0 / 4.0) * assign79090_body25_e119944) + (assign79090_body25_e119939 * (-(locals.var_chib_dn0 / 5.0)))))))))), (((((locals.var_chib_dn2 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn2)) / 2.0) * assign79090_body25_e119948) + (assign79090_body25_e119931 * (-(((locals.var_chib_dn2 / 3.0) * assign79090_body25_e119946) + (assign79090_body25_e119935 * (-(((locals.var_chib_dn2 / 4.0) * assign79090_body25_e119944) + (assign79090_body25_e119939 * (-(locals.var_chib_dn2 / 5.0)))))))))), (((((locals.var_chib_dn4 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn4)) / 2.0) * assign79090_body25_e119948) + (assign79090_body25_e119931 * (-(((locals.var_chib_dn4 / 3.0) * assign79090_body25_e119946) + (assign79090_body25_e119935 * (-(((locals.var_chib_dn4 / 4.0) * assign79090_body25_e119944) + (assign79090_body25_e119939 * (-(locals.var_chib_dn4 / 5.0)))))))))), (((((locals.var_chib_dn5 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn5)) / 2.0) * assign79090_body25_e119948) + (assign79090_body25_e119931 * (-(((locals.var_chib_dn5 / 3.0) * assign79090_body25_e119946) + (assign79090_body25_e119935 * (-(((locals.var_chib_dn5 / 4.0) * assign79090_body25_e119944) + (assign79090_body25_e119939 * (-(locals.var_chib_dn5 / 5.0)))))))))), (((((locals.var_chib_dn6 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn6)) / 2.0) * assign79090_body25_e119948) + (assign79090_body25_e119931 * (-(((locals.var_chib_dn6 / 3.0) * assign79090_body25_e119946) + (assign79090_body25_e119935 * (-(((locals.var_chib_dn6 / 4.0) * assign79090_body25_e119944) + (assign79090_body25_e119939 * (-(locals.var_chib_dn6 / 5.0)))))))))), (((((locals.var_chib_dn7 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn7)) / 2.0) * assign79090_body25_e119948) + (assign79090_body25_e119931 * (-(((locals.var_chib_dn7 / 3.0) * assign79090_body25_e119946) + (assign79090_body25_e119935 * (-(((locals.var_chib_dn7 / 4.0) * assign79090_body25_e119944) + (assign79090_body25_e119939 * (-(locals.var_chib_dn7 / 5.0)))))))))), (((((locals.var_chib_dn8 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn8)) / 2.0) * assign79090_body25_e119948) + (assign79090_body25_e119931 * (-(((locals.var_chib_dn8 / 3.0) * assign79090_body25_e119946) + (assign79090_body25_e119935 * (-(((locals.var_chib_dn8 / 4.0) * assign79090_body25_e119944) + (assign79090_body25_e119939 * (-(locals.var_chib_dn8 / 5.0)))))))))), (((((locals.var_chib_dn9 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn9)) / 2.0) * assign79090_body25_e119948) + (assign79090_body25_e119931 * (-(((locals.var_chib_dn9 / 3.0) * assign79090_body25_e119946) + (assign79090_body25_e119935 * (-(((locals.var_chib_dn9 / 4.0) * assign79090_body25_e119944) + (assign79090_body25_e119939 * (-(locals.var_chib_dn9 / 5.0)))))))))), (((((locals.var_chib_dn10 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn10)) / 2.0) * assign79090_body25_e119948) + (assign79090_body25_e119931 * (-(((locals.var_chib_dn10 / 3.0) * assign79090_body25_e119946) + (assign79090_body25_e119935 * (-(((locals.var_chib_dn10 / 4.0) * assign79090_body25_e119944) + (assign79090_body25_e119939 * (-(locals.var_chib_dn10 / 5.0)))))))))), (((((locals.var_chib_dn11 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn11)) / 2.0) * assign79090_body25_e119948) + (assign79090_body25_e119931 * (-(((locals.var_chib_dn11 / 3.0) * assign79090_body25_e119946) + (assign79090_body25_e119935 * (-(((locals.var_chib_dn11 / 4.0) * assign79090_body25_e119944) + (assign79090_body25_e119939 * (-(locals.var_chib_dn11 / 5.0)))))))))), (((((locals.var_chib_dn14 * locals.var_chib) + (locals.var_chib * locals.var_chib_dn14)) / 2.0) * assign79090_body25_e119948) + (assign79090_body25_e119931 * (-(((locals.var_chib_dn14 / 3.0) * assign79090_body25_e119946) + (assign79090_body25_e119935 * (-(((locals.var_chib_dn14 / 4.0) * assign79090_body25_e119944) + (assign79090_body25_e119939 * (-(locals.var_chib_dn14 / 5.0)))))))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign79090_body25_e119951;
            locals.var_t2_dn0 = assign79090_body25_e119951_d_n0;
            locals.var_t2_dn2 = assign79090_body25_e119951_d_n2;
            locals.var_t2_dn4 = assign79090_body25_e119951_d_n4;
            locals.var_t2_dn5 = assign79090_body25_e119951_d_n5;
            locals.var_t2_dn6 = assign79090_body25_e119951_d_n6;
            locals.var_t2_dn7 = assign79090_body25_e119951_d_n7;
            locals.var_t2_dn8 = assign79090_body25_e119951_d_n8;
            locals.var_t2_dn9 = assign79090_body25_e119951_d_n9;
            locals.var_t2_dn10 = assign79090_body25_e119951_d_n10;
            locals.var_t2_dn11 = assign79090_body25_e119951_d_n11;
            locals.var_t2_dn14 = assign79090_body25_e119951_d_n14;
            let (assign79090_body26_e119983, assign79090_body26_e119983_d_n0, assign79090_body26_e119983_d_n2, assign79090_body26_e119983_d_n4, assign79090_body26_e119983_d_n5, assign79090_body26_e119983_d_n6, assign79090_body26_e119983_d_n7, assign79090_body26_e119983_d_n8, assign79090_body26_e119983_d_n9, assign79090_body26_e119983_d_n10, assign79090_body26_e119983_d_n11, assign79090_body26_e119983_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) {
        let assign79090_body26_e119967: f64 = (locals.var_chib / 2.0);
        let assign79090_body26_e119971: f64 = (locals.var_chib / 3.0);
        let assign79090_body26_e119975: f64 = (locals.var_chib / 4.0);
        let assign79090_body26_e119976: f64 = (1.0 - assign79090_body26_e119975);
        let assign79090_body26_e119977: f64 = (assign79090_body26_e119971 * assign79090_body26_e119976);
        let assign79090_body26_e119978: f64 = (1.0 - assign79090_body26_e119977);
        let assign79090_body26_e119979: f64 = (assign79090_body26_e119967 * assign79090_body26_e119978);
        let assign79090_body26_e119980: f64 = (1.0 - assign79090_body26_e119979);
        let assign79090_body26_e119981: f64 = (locals.var_chib * assign79090_body26_e119980);
        (assign79090_body26_e119981, ((locals.var_chib_dn0 * assign79090_body26_e119980) + (locals.var_chib * (-(((locals.var_chib_dn0 / 2.0) * assign79090_body26_e119978) + (assign79090_body26_e119967 * (-(((locals.var_chib_dn0 / 3.0) * assign79090_body26_e119976) + (assign79090_body26_e119971 * (-(locals.var_chib_dn0 / 4.0)))))))))), ((locals.var_chib_dn2 * assign79090_body26_e119980) + (locals.var_chib * (-(((locals.var_chib_dn2 / 2.0) * assign79090_body26_e119978) + (assign79090_body26_e119967 * (-(((locals.var_chib_dn2 / 3.0) * assign79090_body26_e119976) + (assign79090_body26_e119971 * (-(locals.var_chib_dn2 / 4.0)))))))))), ((locals.var_chib_dn4 * assign79090_body26_e119980) + (locals.var_chib * (-(((locals.var_chib_dn4 / 2.0) * assign79090_body26_e119978) + (assign79090_body26_e119967 * (-(((locals.var_chib_dn4 / 3.0) * assign79090_body26_e119976) + (assign79090_body26_e119971 * (-(locals.var_chib_dn4 / 4.0)))))))))), ((locals.var_chib_dn5 * assign79090_body26_e119980) + (locals.var_chib * (-(((locals.var_chib_dn5 / 2.0) * assign79090_body26_e119978) + (assign79090_body26_e119967 * (-(((locals.var_chib_dn5 / 3.0) * assign79090_body26_e119976) + (assign79090_body26_e119971 * (-(locals.var_chib_dn5 / 4.0)))))))))), ((locals.var_chib_dn6 * assign79090_body26_e119980) + (locals.var_chib * (-(((locals.var_chib_dn6 / 2.0) * assign79090_body26_e119978) + (assign79090_body26_e119967 * (-(((locals.var_chib_dn6 / 3.0) * assign79090_body26_e119976) + (assign79090_body26_e119971 * (-(locals.var_chib_dn6 / 4.0)))))))))), ((locals.var_chib_dn7 * assign79090_body26_e119980) + (locals.var_chib * (-(((locals.var_chib_dn7 / 2.0) * assign79090_body26_e119978) + (assign79090_body26_e119967 * (-(((locals.var_chib_dn7 / 3.0) * assign79090_body26_e119976) + (assign79090_body26_e119971 * (-(locals.var_chib_dn7 / 4.0)))))))))), ((locals.var_chib_dn8 * assign79090_body26_e119980) + (locals.var_chib * (-(((locals.var_chib_dn8 / 2.0) * assign79090_body26_e119978) + (assign79090_body26_e119967 * (-(((locals.var_chib_dn8 / 3.0) * assign79090_body26_e119976) + (assign79090_body26_e119971 * (-(locals.var_chib_dn8 / 4.0)))))))))), ((locals.var_chib_dn9 * assign79090_body26_e119980) + (locals.var_chib * (-(((locals.var_chib_dn9 / 2.0) * assign79090_body26_e119978) + (assign79090_body26_e119967 * (-(((locals.var_chib_dn9 / 3.0) * assign79090_body26_e119976) + (assign79090_body26_e119971 * (-(locals.var_chib_dn9 / 4.0)))))))))), ((locals.var_chib_dn10 * assign79090_body26_e119980) + (locals.var_chib * (-(((locals.var_chib_dn10 / 2.0) * assign79090_body26_e119978) + (assign79090_body26_e119967 * (-(((locals.var_chib_dn10 / 3.0) * assign79090_body26_e119976) + (assign79090_body26_e119971 * (-(locals.var_chib_dn10 / 4.0)))))))))), ((locals.var_chib_dn11 * assign79090_body26_e119980) + (locals.var_chib * (-(((locals.var_chib_dn11 / 2.0) * assign79090_body26_e119978) + (assign79090_body26_e119967 * (-(((locals.var_chib_dn11 / 3.0) * assign79090_body26_e119976) + (assign79090_body26_e119971 * (-(locals.var_chib_dn11 / 4.0)))))))))), ((locals.var_chib_dn14 * assign79090_body26_e119980) + (locals.var_chib * (-(((locals.var_chib_dn14 / 2.0) * assign79090_body26_e119978) + (assign79090_body26_e119967 * (-(((locals.var_chib_dn14 / 3.0) * assign79090_body26_e119976) + (assign79090_body26_e119971 * (-(locals.var_chib_dn14 / 4.0)))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
            locals.var_t3 = assign79090_body26_e119983;
            locals.var_t3_dn0 = assign79090_body26_e119983_d_n0;
            locals.var_t3_dn2 = assign79090_body26_e119983_d_n2;
            locals.var_t3_dn4 = assign79090_body26_e119983_d_n4;
            locals.var_t3_dn5 = assign79090_body26_e119983_d_n5;
            locals.var_t3_dn6 = assign79090_body26_e119983_d_n6;
            locals.var_t3_dn7 = assign79090_body26_e119983_d_n7;
            locals.var_t3_dn8 = assign79090_body26_e119983_d_n8;
            locals.var_t3_dn9 = assign79090_body26_e119983_d_n9;
            locals.var_t3_dn10 = assign79090_body26_e119983_d_n10;
            locals.var_t3_dn11 = assign79090_body26_e119983_d_n11;
            locals.var_t3_dn14 = assign79090_body26_e119983_d_n14;
            let (assign79090_body27_e119999, assign79090_body27_e119999_d_n0, assign79090_body27_e119999_d_n2, assign79090_body27_e119999_d_n4, assign79090_body27_e119999_d_n5, assign79090_body27_e119999_d_n6, assign79090_body27_e119999_d_n7, assign79090_body27_e119999_d_n8, assign79090_body27_e119999_d_n9, assign79090_body27_e119999_d_n10, assign79090_body27_e119999_d_n11, assign79090_body27_e119999_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) {
        let assign79090_body27_e119997: f64 = (locals.var_t0 - locals.var_t2);
        (assign79090_body27_e119997, (locals.var_t0_dn0 - locals.var_t2_dn0), (locals.var_t0_dn2 - locals.var_t2_dn2), (locals.var_t0_dn4 - locals.var_t2_dn4), (locals.var_t0_dn5 - locals.var_t2_dn5), (locals.var_t0_dn6 - locals.var_t2_dn6), (locals.var_t0_dn7 - locals.var_t2_dn7), (locals.var_t0_dn8 - locals.var_t2_dn8), (locals.var_t0_dn9 - locals.var_t2_dn9), (locals.var_t0_dn10 - locals.var_t2_dn10), (locals.var_t0_dn11 - locals.var_t2_dn11), (locals.var_t0_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
            locals.var_t4 = assign79090_body27_e119999;
            locals.var_t4_dn0 = assign79090_body27_e119999_d_n0;
            locals.var_t4_dn2 = assign79090_body27_e119999_d_n2;
            locals.var_t4_dn4 = assign79090_body27_e119999_d_n4;
            locals.var_t4_dn5 = assign79090_body27_e119999_d_n5;
            locals.var_t4_dn6 = assign79090_body27_e119999_d_n6;
            locals.var_t4_dn7 = assign79090_body27_e119999_d_n7;
            locals.var_t4_dn8 = assign79090_body27_e119999_d_n8;
            locals.var_t4_dn9 = assign79090_body27_e119999_d_n9;
            locals.var_t4_dn10 = assign79090_body27_e119999_d_n10;
            locals.var_t4_dn11 = assign79090_body27_e119999_d_n11;
            locals.var_t4_dn14 = assign79090_body27_e119999_d_n14;
            let assign79090_body28_e120002: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1837 = assign79090_body28_e120002;
            let (assign79090_body29_e120019, assign79090_body29_e120019_d_n0, assign79090_body29_e120019_d_n2, assign79090_body29_e120019_d_n4, assign79090_body29_e120019_d_n5, assign79090_body29_e120019_d_n6, assign79090_body29_e120019_d_n7, assign79090_body29_e120019_d_n8, assign79090_body29_e120019_d_n9, assign79090_body29_e120019_d_n10, assign79090_body29_e120019_d_n11, assign79090_body29_e120019_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 != 0.0)) {
        let assign79090_body29_e120017: f64 = (locals.var_t4).sqrt();
        (assign79090_body29_e120017, (locals.var_t4_dn0 / (2.0 * assign79090_body29_e120017)), (locals.var_t4_dn2 / (2.0 * assign79090_body29_e120017)), (locals.var_t4_dn4 / (2.0 * assign79090_body29_e120017)), (locals.var_t4_dn5 / (2.0 * assign79090_body29_e120017)), (locals.var_t4_dn6 / (2.0 * assign79090_body29_e120017)), (locals.var_t4_dn7 / (2.0 * assign79090_body29_e120017)), (locals.var_t4_dn8 / (2.0 * assign79090_body29_e120017)), (locals.var_t4_dn9 / (2.0 * assign79090_body29_e120017)), (locals.var_t4_dn10 / (2.0 * assign79090_body29_e120017)), (locals.var_t4_dn11 / (2.0 * assign79090_body29_e120017)), (locals.var_t4_dn14 / (2.0 * assign79090_body29_e120017)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign79090_body29_e120019;
            locals.var_fb_dn0 = assign79090_body29_e120019_d_n0;
            locals.var_fb_dn2 = assign79090_body29_e120019_d_n2;
            locals.var_fb_dn4 = assign79090_body29_e120019_d_n4;
            locals.var_fb_dn5 = assign79090_body29_e120019_d_n5;
            locals.var_fb_dn6 = assign79090_body29_e120019_d_n6;
            locals.var_fb_dn7 = assign79090_body29_e120019_d_n7;
            locals.var_fb_dn8 = assign79090_body29_e120019_d_n8;
            locals.var_fb_dn9 = assign79090_body29_e120019_d_n9;
            locals.var_fb_dn10 = assign79090_body29_e120019_d_n10;
            locals.var_fb_dn11 = assign79090_body29_e120019_d_n11;
            locals.var_fb_dn14 = assign79090_body29_e120019_d_n14;
            let (assign79090_body30_e120045, assign79090_body30_e120045_d_n0, assign79090_body30_e120045_d_n2, assign79090_body30_e120045_d_n4, assign79090_body30_e120045_d_n5, assign79090_body30_e120045_d_n6, assign79090_body30_e120045_d_n7, assign79090_body30_e120045_d_n8, assign79090_body30_e120045_d_n9, assign79090_body30_e120045_d_n10, assign79090_body30_e120045_d_n11, assign79090_body30_e120045_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 != 0.0)) {
        let assign79090_body30_e120035: f64 = (locals.var_beta * 0.5);
        let assign79090_body30_e120039: f64 = (locals.var_phi_b_dpss * locals.var_t3);
        let assign79090_body30_e120040: f64 = (locals.var_t1 - assign79090_body30_e120039);
        let assign79090_body30_e120041: f64 = (assign79090_body30_e120035 * assign79090_body30_e120040);
        let assign79090_body30_e120043: f64 = (assign79090_body30_e120041 / locals.var_fb);
        (assign79090_body30_e120043, ((((((locals.var_beta_dn0 * 0.5) * assign79090_body30_e120040) + (assign79090_body30_e120035 * (locals.var_t1_dn0 - ((locals.var_phi_b_dpss_dn0 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn0))))) * locals.var_fb) - (assign79090_body30_e120041 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign79090_body30_e120040) + (assign79090_body30_e120035 * (locals.var_t1_dn2 - ((locals.var_phi_b_dpss_dn2 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn2))))) * locals.var_fb) - (assign79090_body30_e120041 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign79090_body30_e120040) + (assign79090_body30_e120035 * (locals.var_t1_dn4 - ((locals.var_phi_b_dpss_dn4 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn4))))) * locals.var_fb) - (assign79090_body30_e120041 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign79090_body30_e120040) + (assign79090_body30_e120035 * (locals.var_t1_dn5 - ((locals.var_phi_b_dpss_dn5 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn5))))) * locals.var_fb) - (assign79090_body30_e120041 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign79090_body30_e120040) + (assign79090_body30_e120035 * (locals.var_t1_dn6 - ((locals.var_phi_b_dpss_dn6 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn6))))) * locals.var_fb) - (assign79090_body30_e120041 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign79090_body30_e120040) + (assign79090_body30_e120035 * (locals.var_t1_dn7 - ((locals.var_phi_b_dpss_dn7 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn7))))) * locals.var_fb) - (assign79090_body30_e120041 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign79090_body30_e120040) + (assign79090_body30_e120035 * (locals.var_t1_dn8 - ((locals.var_phi_b_dpss_dn8 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn8))))) * locals.var_fb) - (assign79090_body30_e120041 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign79090_body30_e120040) + (assign79090_body30_e120035 * (locals.var_t1_dn9 - ((locals.var_phi_b_dpss_dn9 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn9))))) * locals.var_fb) - (assign79090_body30_e120041 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign79090_body30_e120040) + (assign79090_body30_e120035 * (locals.var_t1_dn10 - ((locals.var_phi_b_dpss_dn10 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn10))))) * locals.var_fb) - (assign79090_body30_e120041 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn11 * 0.5) * assign79090_body30_e120040) + (assign79090_body30_e120035 * (locals.var_t1_dn11 - ((locals.var_phi_b_dpss_dn11 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn11))))) * locals.var_fb) - (assign79090_body30_e120041 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn14 * 0.5) * assign79090_body30_e120040) + (assign79090_body30_e120035 * (locals.var_t1_dn14 - ((locals.var_phi_b_dpss_dn14 * locals.var_t3) + (locals.var_phi_b_dpss * locals.var_t3_dn14))))) * locals.var_fb) - (assign79090_body30_e120041 * locals.var_fb_dn14)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign79090_body30_e120045;
            locals.var_fb_dpss_dn0 = assign79090_body30_e120045_d_n0;
            locals.var_fb_dpss_dn2 = assign79090_body30_e120045_d_n2;
            locals.var_fb_dpss_dn4 = assign79090_body30_e120045_d_n4;
            locals.var_fb_dpss_dn5 = assign79090_body30_e120045_d_n5;
            locals.var_fb_dpss_dn6 = assign79090_body30_e120045_d_n6;
            locals.var_fb_dpss_dn7 = assign79090_body30_e120045_d_n7;
            locals.var_fb_dpss_dn8 = assign79090_body30_e120045_d_n8;
            locals.var_fb_dpss_dn9 = assign79090_body30_e120045_d_n9;
            locals.var_fb_dpss_dn10 = assign79090_body30_e120045_d_n10;
            locals.var_fb_dpss_dn11 = assign79090_body30_e120045_d_n11;
            locals.var_fb_dpss_dn14 = assign79090_body30_e120045_d_n14;
            let (assign79090_body32_e120081, assign79090_body32_e120081_d_n0, assign79090_body32_e120081_d_n2, assign79090_body32_e120081_d_n4, assign79090_body32_e120081_d_n5, assign79090_body32_e120081_d_n6, assign79090_body32_e120081_d_n7, assign79090_body32_e120081_d_n8, assign79090_body32_e120081_d_n9, assign79090_body32_e120081_d_n10, assign79090_body32_e120081_d_n11, assign79090_body32_e120081_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign79090_body32_e120081;
            locals.var_fb_dn0 = assign79090_body32_e120081_d_n0;
            locals.var_fb_dn2 = assign79090_body32_e120081_d_n2;
            locals.var_fb_dn4 = assign79090_body32_e120081_d_n4;
            locals.var_fb_dn5 = assign79090_body32_e120081_d_n5;
            locals.var_fb_dn6 = assign79090_body32_e120081_d_n6;
            locals.var_fb_dn7 = assign79090_body32_e120081_d_n7;
            locals.var_fb_dn8 = assign79090_body32_e120081_d_n8;
            locals.var_fb_dn9 = assign79090_body32_e120081_d_n9;
            locals.var_fb_dn10 = assign79090_body32_e120081_d_n10;
            locals.var_fb_dn11 = assign79090_body32_e120081_d_n11;
            locals.var_fb_dn14 = assign79090_body32_e120081_d_n14;
            let (assign79090_body33_e120098, assign79090_body33_e120098_d_n0, assign79090_body33_e120098_d_n2, assign79090_body33_e120098_d_n4, assign79090_body33_e120098_d_n5, assign79090_body33_e120098_d_n6, assign79090_body33_e120098_d_n7, assign79090_body33_e120098_d_n8, assign79090_body33_e120098_d_n9, assign79090_body33_e120098_d_n10, assign79090_body33_e120098_d_n11, assign79090_body33_e120098_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 != 0.0)) && (locals.var_guard1837 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign79090_body33_e120098;
            locals.var_fb_dpss_dn0 = assign79090_body33_e120098_d_n0;
            locals.var_fb_dpss_dn2 = assign79090_body33_e120098_d_n2;
            locals.var_fb_dpss_dn4 = assign79090_body33_e120098_d_n4;
            locals.var_fb_dpss_dn5 = assign79090_body33_e120098_d_n5;
            locals.var_fb_dpss_dn6 = assign79090_body33_e120098_d_n6;
            locals.var_fb_dpss_dn7 = assign79090_body33_e120098_d_n7;
            locals.var_fb_dpss_dn8 = assign79090_body33_e120098_d_n8;
            locals.var_fb_dpss_dn9 = assign79090_body33_e120098_d_n9;
            locals.var_fb_dpss_dn10 = assign79090_body33_e120098_d_n10;
            locals.var_fb_dpss_dn11 = assign79090_body33_e120098_d_n11;
            locals.var_fb_dpss_dn14 = assign79090_body33_e120098_d_n14;
            let (assign79090_body34_e120115, assign79090_body34_e120115_d_n0, assign79090_body34_e120115_d_n2, assign79090_body34_e120115_d_n4, assign79090_body34_e120115_d_n5, assign79090_body34_e120115_d_n6, assign79090_body34_e120115_d_n7, assign79090_body34_e120115_d_n8, assign79090_body34_e120115_d_n9, assign79090_body34_e120115_d_n10, assign79090_body34_e120115_d_n11, assign79090_body34_e120115_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 == 0.0)) {
        let assign79090_body34_e120112: f64 = (-locals.var_chi);
        let assign79090_body34_e120113: f64 = (assign79090_body34_e120112).exp();
        (assign79090_body34_e120113, (assign79090_body34_e120113 * (-locals.var_chi_dn0)), (assign79090_body34_e120113 * (-locals.var_chi_dn2)), (assign79090_body34_e120113 * (-locals.var_chi_dn4)), (assign79090_body34_e120113 * (-locals.var_chi_dn5)), (assign79090_body34_e120113 * (-locals.var_chi_dn6)), (assign79090_body34_e120113 * (-locals.var_chi_dn7)), (assign79090_body34_e120113 * (-locals.var_chi_dn8)), (assign79090_body34_e120113 * (-locals.var_chi_dn9)), (assign79090_body34_e120113 * (-locals.var_chi_dn10)), (assign79090_body34_e120113 * (-locals.var_chi_dn11)), (assign79090_body34_e120113 * (-locals.var_chi_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign79090_body34_e120115;
            locals.var_t0_dn0 = assign79090_body34_e120115_d_n0;
            locals.var_t0_dn2 = assign79090_body34_e120115_d_n2;
            locals.var_t0_dn4 = assign79090_body34_e120115_d_n4;
            locals.var_t0_dn5 = assign79090_body34_e120115_d_n5;
            locals.var_t0_dn6 = assign79090_body34_e120115_d_n6;
            locals.var_t0_dn7 = assign79090_body34_e120115_d_n7;
            locals.var_t0_dn8 = assign79090_body34_e120115_d_n8;
            locals.var_t0_dn9 = assign79090_body34_e120115_d_n9;
            locals.var_t0_dn10 = assign79090_body34_e120115_d_n10;
            locals.var_t0_dn11 = assign79090_body34_e120115_d_n11;
            locals.var_t0_dn14 = assign79090_body34_e120115_d_n14;
            let (assign79090_body35_e120132, assign79090_body35_e120132_d_n0, assign79090_body35_e120132_d_n2, assign79090_body35_e120132_d_n4, assign79090_body35_e120132_d_n5, assign79090_body35_e120132_d_n6, assign79090_body35_e120132_d_n7, assign79090_body35_e120132_d_n8, assign79090_body35_e120132_d_n9, assign79090_body35_e120132_d_n10, assign79090_body35_e120132_d_n11, assign79090_body35_e120132_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 == 0.0)) {
        let assign79090_body35_e120129: f64 = (-locals.var_chib);
        let assign79090_body35_e120130: f64 = (assign79090_body35_e120129).exp();
        (assign79090_body35_e120130, (assign79090_body35_e120130 * (-locals.var_chib_dn0)), (assign79090_body35_e120130 * (-locals.var_chib_dn2)), (assign79090_body35_e120130 * (-locals.var_chib_dn4)), (assign79090_body35_e120130 * (-locals.var_chib_dn5)), (assign79090_body35_e120130 * (-locals.var_chib_dn6)), (assign79090_body35_e120130 * (-locals.var_chib_dn7)), (assign79090_body35_e120130 * (-locals.var_chib_dn8)), (assign79090_body35_e120130 * (-locals.var_chib_dn9)), (assign79090_body35_e120130 * (-locals.var_chib_dn10)), (assign79090_body35_e120130 * (-locals.var_chib_dn11)), (assign79090_body35_e120130 * (-locals.var_chib_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign79090_body35_e120132;
            locals.var_t1_dn0 = assign79090_body35_e120132_d_n0;
            locals.var_t1_dn2 = assign79090_body35_e120132_d_n2;
            locals.var_t1_dn4 = assign79090_body35_e120132_d_n4;
            locals.var_t1_dn5 = assign79090_body35_e120132_d_n5;
            locals.var_t1_dn6 = assign79090_body35_e120132_d_n6;
            locals.var_t1_dn7 = assign79090_body35_e120132_d_n7;
            locals.var_t1_dn8 = assign79090_body35_e120132_d_n8;
            locals.var_t1_dn9 = assign79090_body35_e120132_d_n9;
            locals.var_t1_dn10 = assign79090_body35_e120132_d_n10;
            locals.var_t1_dn11 = assign79090_body35_e120132_d_n11;
            locals.var_t1_dn14 = assign79090_body35_e120132_d_n14;
            let (assign79090_body36_e120153, assign79090_body36_e120153_d_n0, assign79090_body36_e120153_d_n2, assign79090_body36_e120153_d_n4, assign79090_body36_e120153_d_n5, assign79090_body36_e120153_d_n6, assign79090_body36_e120153_d_n7, assign79090_body36_e120153_d_n8, assign79090_body36_e120153_d_n9, assign79090_body36_e120153_d_n10, assign79090_body36_e120153_d_n11, assign79090_body36_e120153_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 == 0.0)) {
        let assign79090_body36_e120147: f64 = (locals.var_chi - locals.var_chib);
        let assign79090_body36_e120150: f64 = (locals.var_t0 - locals.var_t1);
        let assign79090_body36_e120151: f64 = (assign79090_body36_e120147 + assign79090_body36_e120150);
        (assign79090_body36_e120151, ((locals.var_chi_dn0 - locals.var_chib_dn0) + (locals.var_t0_dn0 - locals.var_t1_dn0)), ((locals.var_chi_dn2 - locals.var_chib_dn2) + (locals.var_t0_dn2 - locals.var_t1_dn2)), ((locals.var_chi_dn4 - locals.var_chib_dn4) + (locals.var_t0_dn4 - locals.var_t1_dn4)), ((locals.var_chi_dn5 - locals.var_chib_dn5) + (locals.var_t0_dn5 - locals.var_t1_dn5)), ((locals.var_chi_dn6 - locals.var_chib_dn6) + (locals.var_t0_dn6 - locals.var_t1_dn6)), ((locals.var_chi_dn7 - locals.var_chib_dn7) + (locals.var_t0_dn7 - locals.var_t1_dn7)), ((locals.var_chi_dn8 - locals.var_chib_dn8) + (locals.var_t0_dn8 - locals.var_t1_dn8)), ((locals.var_chi_dn9 - locals.var_chib_dn9) + (locals.var_t0_dn9 - locals.var_t1_dn9)), ((locals.var_chi_dn10 - locals.var_chib_dn10) + (locals.var_t0_dn10 - locals.var_t1_dn10)), ((locals.var_chi_dn11 - locals.var_chib_dn11) + (locals.var_t0_dn11 - locals.var_t1_dn11)), ((locals.var_chi_dn14 - locals.var_chib_dn14) + (locals.var_t0_dn14 - locals.var_t1_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
            locals.var_t4 = assign79090_body36_e120153;
            locals.var_t4_dn0 = assign79090_body36_e120153_d_n0;
            locals.var_t4_dn2 = assign79090_body36_e120153_d_n2;
            locals.var_t4_dn4 = assign79090_body36_e120153_d_n4;
            locals.var_t4_dn5 = assign79090_body36_e120153_d_n5;
            locals.var_t4_dn6 = assign79090_body36_e120153_d_n6;
            locals.var_t4_dn7 = assign79090_body36_e120153_d_n7;
            locals.var_t4_dn8 = assign79090_body36_e120153_d_n8;
            locals.var_t4_dn9 = assign79090_body36_e120153_d_n9;
            locals.var_t4_dn10 = assign79090_body36_e120153_d_n10;
            locals.var_t4_dn11 = assign79090_body36_e120153_d_n11;
            locals.var_t4_dn14 = assign79090_body36_e120153_d_n14;
            let assign79090_body37_e120156: f64 = if locals.var_t4 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1838 = assign79090_body37_e120156;
            let (assign79090_body38_e120174, assign79090_body38_e120174_d_n0, assign79090_body38_e120174_d_n2, assign79090_body38_e120174_d_n4, assign79090_body38_e120174_d_n5, assign79090_body38_e120174_d_n6, assign79090_body38_e120174_d_n7, assign79090_body38_e120174_d_n8, assign79090_body38_e120174_d_n9, assign79090_body38_e120174_d_n10, assign79090_body38_e120174_d_n11, assign79090_body38_e120174_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 == 0.0)) && (locals.var_guard1838 != 0.0)) {
        let assign79090_body38_e120172: f64 = (locals.var_t4).sqrt();
        (assign79090_body38_e120172, (locals.var_t4_dn0 / (2.0 * assign79090_body38_e120172)), (locals.var_t4_dn2 / (2.0 * assign79090_body38_e120172)), (locals.var_t4_dn4 / (2.0 * assign79090_body38_e120172)), (locals.var_t4_dn5 / (2.0 * assign79090_body38_e120172)), (locals.var_t4_dn6 / (2.0 * assign79090_body38_e120172)), (locals.var_t4_dn7 / (2.0 * assign79090_body38_e120172)), (locals.var_t4_dn8 / (2.0 * assign79090_body38_e120172)), (locals.var_t4_dn9 / (2.0 * assign79090_body38_e120172)), (locals.var_t4_dn10 / (2.0 * assign79090_body38_e120172)), (locals.var_t4_dn11 / (2.0 * assign79090_body38_e120172)), (locals.var_t4_dn14 / (2.0 * assign79090_body38_e120172)),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign79090_body38_e120174;
            locals.var_fb_dn0 = assign79090_body38_e120174_d_n0;
            locals.var_fb_dn2 = assign79090_body38_e120174_d_n2;
            locals.var_fb_dn4 = assign79090_body38_e120174_d_n4;
            locals.var_fb_dn5 = assign79090_body38_e120174_d_n5;
            locals.var_fb_dn6 = assign79090_body38_e120174_d_n6;
            locals.var_fb_dn7 = assign79090_body38_e120174_d_n7;
            locals.var_fb_dn8 = assign79090_body38_e120174_d_n8;
            locals.var_fb_dn9 = assign79090_body38_e120174_d_n9;
            locals.var_fb_dn10 = assign79090_body38_e120174_d_n10;
            locals.var_fb_dn11 = assign79090_body38_e120174_d_n11;
            locals.var_fb_dn14 = assign79090_body38_e120174_d_n14;
            let (assign79090_body39_e120205, assign79090_body39_e120205_d_n0, assign79090_body39_e120205_d_n2, assign79090_body39_e120205_d_n4, assign79090_body39_e120205_d_n5, assign79090_body39_e120205_d_n6, assign79090_body39_e120205_d_n7, assign79090_body39_e120205_d_n8, assign79090_body39_e120205_d_n9, assign79090_body39_e120205_d_n10, assign79090_body39_e120205_d_n11, assign79090_body39_e120205_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 == 0.0)) && (locals.var_guard1838 != 0.0)) {
        let assign79090_body39_e120191: f64 = (locals.var_beta * 0.5);
        let assign79090_body39_e120194: f64 = (1.0 - locals.var_t0);
        let assign79090_body39_e120198: f64 = (1.0 - locals.var_t1);
        let assign79090_body39_e120199: f64 = (locals.var_phi_b_dpss * assign79090_body39_e120198);
        let assign79090_body39_e120200: f64 = (assign79090_body39_e120194 - assign79090_body39_e120199);
        let assign79090_body39_e120201: f64 = (assign79090_body39_e120191 * assign79090_body39_e120200);
        let assign79090_body39_e120203: f64 = (assign79090_body39_e120201 / locals.var_fb);
        (assign79090_body39_e120203, ((((((locals.var_beta_dn0 * 0.5) * assign79090_body39_e120200) + (assign79090_body39_e120191 * ((-locals.var_t0_dn0) - ((locals.var_phi_b_dpss_dn0 * assign79090_body39_e120198) + (locals.var_phi_b_dpss * (-locals.var_t1_dn0)))))) * locals.var_fb) - (assign79090_body39_e120201 * locals.var_fb_dn0)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn2 * 0.5) * assign79090_body39_e120200) + (assign79090_body39_e120191 * ((-locals.var_t0_dn2) - ((locals.var_phi_b_dpss_dn2 * assign79090_body39_e120198) + (locals.var_phi_b_dpss * (-locals.var_t1_dn2)))))) * locals.var_fb) - (assign79090_body39_e120201 * locals.var_fb_dn2)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn4 * 0.5) * assign79090_body39_e120200) + (assign79090_body39_e120191 * ((-locals.var_t0_dn4) - ((locals.var_phi_b_dpss_dn4 * assign79090_body39_e120198) + (locals.var_phi_b_dpss * (-locals.var_t1_dn4)))))) * locals.var_fb) - (assign79090_body39_e120201 * locals.var_fb_dn4)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn5 * 0.5) * assign79090_body39_e120200) + (assign79090_body39_e120191 * ((-locals.var_t0_dn5) - ((locals.var_phi_b_dpss_dn5 * assign79090_body39_e120198) + (locals.var_phi_b_dpss * (-locals.var_t1_dn5)))))) * locals.var_fb) - (assign79090_body39_e120201 * locals.var_fb_dn5)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn6 * 0.5) * assign79090_body39_e120200) + (assign79090_body39_e120191 * ((-locals.var_t0_dn6) - ((locals.var_phi_b_dpss_dn6 * assign79090_body39_e120198) + (locals.var_phi_b_dpss * (-locals.var_t1_dn6)))))) * locals.var_fb) - (assign79090_body39_e120201 * locals.var_fb_dn6)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn7 * 0.5) * assign79090_body39_e120200) + (assign79090_body39_e120191 * ((-locals.var_t0_dn7) - ((locals.var_phi_b_dpss_dn7 * assign79090_body39_e120198) + (locals.var_phi_b_dpss * (-locals.var_t1_dn7)))))) * locals.var_fb) - (assign79090_body39_e120201 * locals.var_fb_dn7)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn8 * 0.5) * assign79090_body39_e120200) + (assign79090_body39_e120191 * ((-locals.var_t0_dn8) - ((locals.var_phi_b_dpss_dn8 * assign79090_body39_e120198) + (locals.var_phi_b_dpss * (-locals.var_t1_dn8)))))) * locals.var_fb) - (assign79090_body39_e120201 * locals.var_fb_dn8)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn9 * 0.5) * assign79090_body39_e120200) + (assign79090_body39_e120191 * ((-locals.var_t0_dn9) - ((locals.var_phi_b_dpss_dn9 * assign79090_body39_e120198) + (locals.var_phi_b_dpss * (-locals.var_t1_dn9)))))) * locals.var_fb) - (assign79090_body39_e120201 * locals.var_fb_dn9)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn10 * 0.5) * assign79090_body39_e120200) + (assign79090_body39_e120191 * ((-locals.var_t0_dn10) - ((locals.var_phi_b_dpss_dn10 * assign79090_body39_e120198) + (locals.var_phi_b_dpss * (-locals.var_t1_dn10)))))) * locals.var_fb) - (assign79090_body39_e120201 * locals.var_fb_dn10)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn11 * 0.5) * assign79090_body39_e120200) + (assign79090_body39_e120191 * ((-locals.var_t0_dn11) - ((locals.var_phi_b_dpss_dn11 * assign79090_body39_e120198) + (locals.var_phi_b_dpss * (-locals.var_t1_dn11)))))) * locals.var_fb) - (assign79090_body39_e120201 * locals.var_fb_dn11)) / (locals.var_fb * locals.var_fb)), ((((((locals.var_beta_dn14 * 0.5) * assign79090_body39_e120200) + (assign79090_body39_e120191 * ((-locals.var_t0_dn14) - ((locals.var_phi_b_dpss_dn14 * assign79090_body39_e120198) + (locals.var_phi_b_dpss * (-locals.var_t1_dn14)))))) * locals.var_fb) - (assign79090_body39_e120201 * locals.var_fb_dn14)) / (locals.var_fb * locals.var_fb)),)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign79090_body39_e120205;
            locals.var_fb_dpss_dn0 = assign79090_body39_e120205_d_n0;
            locals.var_fb_dpss_dn2 = assign79090_body39_e120205_d_n2;
            locals.var_fb_dpss_dn4 = assign79090_body39_e120205_d_n4;
            locals.var_fb_dpss_dn5 = assign79090_body39_e120205_d_n5;
            locals.var_fb_dpss_dn6 = assign79090_body39_e120205_d_n6;
            locals.var_fb_dpss_dn7 = assign79090_body39_e120205_d_n7;
            locals.var_fb_dpss_dn8 = assign79090_body39_e120205_d_n8;
            locals.var_fb_dpss_dn9 = assign79090_body39_e120205_d_n9;
            locals.var_fb_dpss_dn10 = assign79090_body39_e120205_d_n10;
            locals.var_fb_dpss_dn11 = assign79090_body39_e120205_d_n11;
            locals.var_fb_dpss_dn14 = assign79090_body39_e120205_d_n14;
            let (assign79090_body41_e120243, assign79090_body41_e120243_d_n0, assign79090_body41_e120243_d_n2, assign79090_body41_e120243_d_n4, assign79090_body41_e120243_d_n5, assign79090_body41_e120243_d_n6, assign79090_body41_e120243_d_n7, assign79090_body41_e120243_d_n8, assign79090_body41_e120243_d_n9, assign79090_body41_e120243_d_n10, assign79090_body41_e120243_d_n11, assign79090_body41_e120243_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 == 0.0)) && (locals.var_guard1838 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign79090_body41_e120243;
            locals.var_fb_dn0 = assign79090_body41_e120243_d_n0;
            locals.var_fb_dn2 = assign79090_body41_e120243_d_n2;
            locals.var_fb_dn4 = assign79090_body41_e120243_d_n4;
            locals.var_fb_dn5 = assign79090_body41_e120243_d_n5;
            locals.var_fb_dn6 = assign79090_body41_e120243_d_n6;
            locals.var_fb_dn7 = assign79090_body41_e120243_d_n7;
            locals.var_fb_dn8 = assign79090_body41_e120243_d_n8;
            locals.var_fb_dn9 = assign79090_body41_e120243_d_n9;
            locals.var_fb_dn10 = assign79090_body41_e120243_d_n10;
            locals.var_fb_dn11 = assign79090_body41_e120243_d_n11;
            locals.var_fb_dn14 = assign79090_body41_e120243_d_n14;
            let (assign79090_body42_e120261, assign79090_body42_e120261_d_n0, assign79090_body42_e120261_d_n2, assign79090_body42_e120261_d_n4, assign79090_body42_e120261_d_n5, assign79090_body42_e120261_d_n6, assign79090_body42_e120261_d_n7, assign79090_body42_e120261_d_n8, assign79090_body42_e120261_d_n9, assign79090_body42_e120261_d_n10, assign79090_body42_e120261_d_n11, assign79090_body42_e120261_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1835 == 0.0)) && (locals.var_guard1836 == 0.0)) && (locals.var_guard1838 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    }
};
            locals.var_fb_dpss = assign79090_body42_e120261;
            locals.var_fb_dpss_dn0 = assign79090_body42_e120261_d_n0;
            locals.var_fb_dpss_dn2 = assign79090_body42_e120261_d_n2;
            locals.var_fb_dpss_dn4 = assign79090_body42_e120261_d_n4;
            locals.var_fb_dpss_dn5 = assign79090_body42_e120261_d_n5;
            locals.var_fb_dpss_dn6 = assign79090_body42_e120261_d_n6;
            locals.var_fb_dpss_dn7 = assign79090_body42_e120261_d_n7;
            locals.var_fb_dpss_dn8 = assign79090_body42_e120261_d_n8;
            locals.var_fb_dpss_dn9 = assign79090_body42_e120261_d_n9;
            locals.var_fb_dpss_dn10 = assign79090_body42_e120261_d_n10;
            locals.var_fb_dpss_dn11 = assign79090_body42_e120261_d_n11;
            locals.var_fb_dpss_dn14 = assign79090_body42_e120261_d_n14;
            let assign79090_body43_e120264: f64 = if locals.var_chi < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1839 = assign79090_body43_e120264;
            let (assign79090_body45_e120288, assign79090_body45_e120288_d_n0, assign79090_body45_e120288_d_n2, assign79090_body45_e120288_d_n4, assign79090_body45_e120288_d_n5, assign79090_body45_e120288_d_n6, assign79090_body45_e120288_d_n7, assign79090_body45_e120288_d_n8, assign79090_body45_e120288_d_n9, assign79090_body45_e120288_d_n10, assign79090_body45_e120288_d_n11, assign79090_body45_e120288_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1839 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign79090_body45_e120288;
            locals.var_fs01_dn0 = assign79090_body45_e120288_d_n0;
            locals.var_fs01_dn2 = assign79090_body45_e120288_d_n2;
            locals.var_fs01_dn4 = assign79090_body45_e120288_d_n4;
            locals.var_fs01_dn5 = assign79090_body45_e120288_d_n5;
            locals.var_fs01_dn6 = assign79090_body45_e120288_d_n6;
            locals.var_fs01_dn7 = assign79090_body45_e120288_d_n7;
            locals.var_fs01_dn8 = assign79090_body45_e120288_d_n8;
            locals.var_fs01_dn9 = assign79090_body45_e120288_d_n9;
            locals.var_fs01_dn10 = assign79090_body45_e120288_d_n10;
            locals.var_fs01_dn11 = assign79090_body45_e120288_d_n11;
            locals.var_fs01_dn14 = assign79090_body45_e120288_d_n14;
            let (assign79090_body46_e120299, assign79090_body46_e120299_d_n0, assign79090_body46_e120299_d_n2, assign79090_body46_e120299_d_n4, assign79090_body46_e120299_d_n5, assign79090_body46_e120299_d_n6, assign79090_body46_e120299_d_n7, assign79090_body46_e120299_d_n8, assign79090_body46_e120299_d_n9, assign79090_body46_e120299_d_n10, assign79090_body46_e120299_d_n11, assign79090_body46_e120299_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1839 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign79090_body46_e120299;
            locals.var_fs01_dps0_dn0 = assign79090_body46_e120299_d_n0;
            locals.var_fs01_dps0_dn2 = assign79090_body46_e120299_d_n2;
            locals.var_fs01_dps0_dn4 = assign79090_body46_e120299_d_n4;
            locals.var_fs01_dps0_dn5 = assign79090_body46_e120299_d_n5;
            locals.var_fs01_dps0_dn6 = assign79090_body46_e120299_d_n6;
            locals.var_fs01_dps0_dn7 = assign79090_body46_e120299_d_n7;
            locals.var_fs01_dps0_dn8 = assign79090_body46_e120299_d_n8;
            locals.var_fs01_dps0_dn9 = assign79090_body46_e120299_d_n9;
            locals.var_fs01_dps0_dn10 = assign79090_body46_e120299_d_n10;
            locals.var_fs01_dps0_dn11 = assign79090_body46_e120299_d_n11;
            locals.var_fs01_dps0_dn14 = assign79090_body46_e120299_d_n14;
            let (assign79090_body47_e120311, assign79090_body47_e120311_d_n0, assign79090_body47_e120311_d_n2, assign79090_body47_e120311_d_n4, assign79090_body47_e120311_d_n5, assign79090_body47_e120311_d_n6, assign79090_body47_e120311_d_n7, assign79090_body47_e120311_d_n8, assign79090_body47_e120311_d_n9, assign79090_body47_e120311_d_n10, assign79090_body47_e120311_d_n11, assign79090_body47_e120311_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1839 != 0.0)) {
        let assign79090_body47_e120309: f64 = (-locals.var_fb);
        (assign79090_body47_e120309, (-locals.var_fb_dn0), (-locals.var_fb_dn2), (-locals.var_fb_dn4), (-locals.var_fb_dn5), (-locals.var_fb_dn6), (-locals.var_fb_dn7), (-locals.var_fb_dn8), (-locals.var_fb_dn9), (-locals.var_fb_dn10), (-locals.var_fb_dn11), (-locals.var_fb_dn14),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign79090_body47_e120311;
            locals.var_fs02_dn0 = assign79090_body47_e120311_d_n0;
            locals.var_fs02_dn2 = assign79090_body47_e120311_d_n2;
            locals.var_fs02_dn4 = assign79090_body47_e120311_d_n4;
            locals.var_fs02_dn5 = assign79090_body47_e120311_d_n5;
            locals.var_fs02_dn6 = assign79090_body47_e120311_d_n6;
            locals.var_fs02_dn7 = assign79090_body47_e120311_d_n7;
            locals.var_fs02_dn8 = assign79090_body47_e120311_d_n8;
            locals.var_fs02_dn9 = assign79090_body47_e120311_d_n9;
            locals.var_fs02_dn10 = assign79090_body47_e120311_d_n10;
            locals.var_fs02_dn11 = assign79090_body47_e120311_d_n11;
            locals.var_fs02_dn14 = assign79090_body47_e120311_d_n14;
            let (assign79090_body48_e120323, assign79090_body48_e120323_d_n0, assign79090_body48_e120323_d_n2, assign79090_body48_e120323_d_n4, assign79090_body48_e120323_d_n5, assign79090_body48_e120323_d_n6, assign79090_body48_e120323_d_n7, assign79090_body48_e120323_d_n8, assign79090_body48_e120323_d_n9, assign79090_body48_e120323_d_n10, assign79090_body48_e120323_d_n11, assign79090_body48_e120323_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1839 != 0.0)) {
        let assign79090_body48_e120321: f64 = (-locals.var_fb_dpss);
        (assign79090_body48_e120321, (-locals.var_fb_dpss_dn0), (-locals.var_fb_dpss_dn2), (-locals.var_fb_dpss_dn4), (-locals.var_fb_dpss_dn5), (-locals.var_fb_dpss_dn6), (-locals.var_fb_dpss_dn7), (-locals.var_fb_dpss_dn8), (-locals.var_fb_dpss_dn9), (-locals.var_fb_dpss_dn10), (-locals.var_fb_dpss_dn11), (-locals.var_fb_dpss_dn14),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign79090_body48_e120323;
            locals.var_fs02_dps0_dn0 = assign79090_body48_e120323_d_n0;
            locals.var_fs02_dps0_dn2 = assign79090_body48_e120323_d_n2;
            locals.var_fs02_dps0_dn4 = assign79090_body48_e120323_d_n4;
            locals.var_fs02_dps0_dn5 = assign79090_body48_e120323_d_n5;
            locals.var_fs02_dps0_dn6 = assign79090_body48_e120323_d_n6;
            locals.var_fs02_dps0_dn7 = assign79090_body48_e120323_d_n7;
            locals.var_fs02_dps0_dn8 = assign79090_body48_e120323_d_n8;
            locals.var_fs02_dps0_dn9 = assign79090_body48_e120323_d_n9;
            locals.var_fs02_dps0_dn10 = assign79090_body48_e120323_d_n10;
            locals.var_fs02_dps0_dn11 = assign79090_body48_e120323_d_n11;
            locals.var_fs02_dps0_dn14 = assign79090_body48_e120323_d_n14;
            let assign79090_body49_e120326: f64 = if locals.var_chi < 60.0 { 1.0 } else { 0.0 };
            locals.var_guard1840 = assign79090_body49_e120326;
            let assign79090_body50_e120329: f64 = if locals.var_chi < 5e-5 { 1.0 } else { 0.0 };
            locals.var_guard1841 = assign79090_body50_e120329;
            let (assign79090_body51_e120367, assign79090_body51_e120367_d_n0, assign79090_body51_e120367_d_n2, assign79090_body51_e120367_d_n4, assign79090_body51_e120367_d_n5, assign79090_body51_e120367_d_n6, assign79090_body51_e120367_d_n7, assign79090_body51_e120367_d_n8, assign79090_body51_e120367_d_n9, assign79090_body51_e120367_d_n10, assign79090_body51_e120367_d_n11, assign79090_body51_e120367_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1839 == 0.0)) && (locals.var_guard1840 != 0.0)) && (locals.var_guard1841 != 0.0)) {
        let assign79090_body51_e120345: f64 = (locals.var_chi * locals.var_chi);
        let assign79090_body51_e120347: f64 = (assign79090_body51_e120345 / 2.0);
        let assign79090_body51_e120351: f64 = (locals.var_chi / 3.0);
        let assign79090_body51_e120355: f64 = (locals.var_chi / 4.0);
        let assign79090_body51_e120359: f64 = (locals.var_chi / 5.0);
        let assign79090_body51_e120360: f64 = (1.0 + assign79090_body51_e120359);
        let assign79090_body51_e120361: f64 = (assign79090_body51_e120355 * assign79090_body51_e120360);
        let assign79090_body51_e120362: f64 = (1.0 + assign79090_body51_e120361);
        let assign79090_body51_e120363: f64 = (assign79090_body51_e120351 * assign79090_body51_e120362);
        let assign79090_body51_e120364: f64 = (1.0 + assign79090_body51_e120363);
        let assign79090_body51_e120365: f64 = (assign79090_body51_e120347 * assign79090_body51_e120364);
        (assign79090_body51_e120365, (((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) / 2.0) * assign79090_body51_e120364) + (assign79090_body51_e120347 * (((locals.var_chi_dn0 / 3.0) * assign79090_body51_e120362) + (assign79090_body51_e120351 * (((locals.var_chi_dn0 / 4.0) * assign79090_body51_e120360) + (assign79090_body51_e120355 * (locals.var_chi_dn0 / 5.0))))))), (((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) / 2.0) * assign79090_body51_e120364) + (assign79090_body51_e120347 * (((locals.var_chi_dn2 / 3.0) * assign79090_body51_e120362) + (assign79090_body51_e120351 * (((locals.var_chi_dn2 / 4.0) * assign79090_body51_e120360) + (assign79090_body51_e120355 * (locals.var_chi_dn2 / 5.0))))))), (((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) / 2.0) * assign79090_body51_e120364) + (assign79090_body51_e120347 * (((locals.var_chi_dn4 / 3.0) * assign79090_body51_e120362) + (assign79090_body51_e120351 * (((locals.var_chi_dn4 / 4.0) * assign79090_body51_e120360) + (assign79090_body51_e120355 * (locals.var_chi_dn4 / 5.0))))))), (((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) / 2.0) * assign79090_body51_e120364) + (assign79090_body51_e120347 * (((locals.var_chi_dn5 / 3.0) * assign79090_body51_e120362) + (assign79090_body51_e120351 * (((locals.var_chi_dn5 / 4.0) * assign79090_body51_e120360) + (assign79090_body51_e120355 * (locals.var_chi_dn5 / 5.0))))))), (((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) / 2.0) * assign79090_body51_e120364) + (assign79090_body51_e120347 * (((locals.var_chi_dn6 / 3.0) * assign79090_body51_e120362) + (assign79090_body51_e120351 * (((locals.var_chi_dn6 / 4.0) * assign79090_body51_e120360) + (assign79090_body51_e120355 * (locals.var_chi_dn6 / 5.0))))))), (((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) / 2.0) * assign79090_body51_e120364) + (assign79090_body51_e120347 * (((locals.var_chi_dn7 / 3.0) * assign79090_body51_e120362) + (assign79090_body51_e120351 * (((locals.var_chi_dn7 / 4.0) * assign79090_body51_e120360) + (assign79090_body51_e120355 * (locals.var_chi_dn7 / 5.0))))))), (((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) / 2.0) * assign79090_body51_e120364) + (assign79090_body51_e120347 * (((locals.var_chi_dn8 / 3.0) * assign79090_body51_e120362) + (assign79090_body51_e120351 * (((locals.var_chi_dn8 / 4.0) * assign79090_body51_e120360) + (assign79090_body51_e120355 * (locals.var_chi_dn8 / 5.0))))))), (((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) / 2.0) * assign79090_body51_e120364) + (assign79090_body51_e120347 * (((locals.var_chi_dn9 / 3.0) * assign79090_body51_e120362) + (assign79090_body51_e120351 * (((locals.var_chi_dn9 / 4.0) * assign79090_body51_e120360) + (assign79090_body51_e120355 * (locals.var_chi_dn9 / 5.0))))))), (((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) / 2.0) * assign79090_body51_e120364) + (assign79090_body51_e120347 * (((locals.var_chi_dn10 / 3.0) * assign79090_body51_e120362) + (assign79090_body51_e120351 * (((locals.var_chi_dn10 / 4.0) * assign79090_body51_e120360) + (assign79090_body51_e120355 * (locals.var_chi_dn10 / 5.0))))))), (((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) / 2.0) * assign79090_body51_e120364) + (assign79090_body51_e120347 * (((locals.var_chi_dn11 / 3.0) * assign79090_body51_e120362) + (assign79090_body51_e120351 * (((locals.var_chi_dn11 / 4.0) * assign79090_body51_e120360) + (assign79090_body51_e120355 * (locals.var_chi_dn11 / 5.0))))))), (((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) / 2.0) * assign79090_body51_e120364) + (assign79090_body51_e120347 * (((locals.var_chi_dn14 / 3.0) * assign79090_body51_e120362) + (assign79090_body51_e120351 * (((locals.var_chi_dn14 / 4.0) * assign79090_body51_e120360) + (assign79090_body51_e120355 * (locals.var_chi_dn14 / 5.0))))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign79090_body51_e120367;
            locals.var_t0_dn0 = assign79090_body51_e120367_d_n0;
            locals.var_t0_dn2 = assign79090_body51_e120367_d_n2;
            locals.var_t0_dn4 = assign79090_body51_e120367_d_n4;
            locals.var_t0_dn5 = assign79090_body51_e120367_d_n5;
            locals.var_t0_dn6 = assign79090_body51_e120367_d_n6;
            locals.var_t0_dn7 = assign79090_body51_e120367_d_n7;
            locals.var_t0_dn8 = assign79090_body51_e120367_d_n8;
            locals.var_t0_dn9 = assign79090_body51_e120367_d_n9;
            locals.var_t0_dn10 = assign79090_body51_e120367_d_n10;
            locals.var_t0_dn11 = assign79090_body51_e120367_d_n11;
            locals.var_t0_dn14 = assign79090_body51_e120367_d_n14;
            let (assign79090_body52_e120401, assign79090_body52_e120401_d_n0, assign79090_body52_e120401_d_n2, assign79090_body52_e120401_d_n4, assign79090_body52_e120401_d_n5, assign79090_body52_e120401_d_n6, assign79090_body52_e120401_d_n7, assign79090_body52_e120401_d_n8, assign79090_body52_e120401_d_n9, assign79090_body52_e120401_d_n10, assign79090_body52_e120401_d_n11, assign79090_body52_e120401_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1839 == 0.0)) && (locals.var_guard1840 != 0.0)) && (locals.var_guard1841 != 0.0)) {
        let assign79090_body52_e120385: f64 = (locals.var_chi / 2.0);
        let assign79090_body52_e120389: f64 = (locals.var_chi / 3.0);
        let assign79090_body52_e120393: f64 = (locals.var_chi / 4.0);
        let assign79090_body52_e120394: f64 = (1.0 + assign79090_body52_e120393);
        let assign79090_body52_e120395: f64 = (assign79090_body52_e120389 * assign79090_body52_e120394);
        let assign79090_body52_e120396: f64 = (1.0 + assign79090_body52_e120395);
        let assign79090_body52_e120397: f64 = (assign79090_body52_e120385 * assign79090_body52_e120396);
        let assign79090_body52_e120398: f64 = (1.0 + assign79090_body52_e120397);
        let assign79090_body52_e120399: f64 = (locals.var_chi * assign79090_body52_e120398);
        (assign79090_body52_e120399, ((locals.var_chi_dn0 * assign79090_body52_e120398) + (locals.var_chi * (((locals.var_chi_dn0 / 2.0) * assign79090_body52_e120396) + (assign79090_body52_e120385 * (((locals.var_chi_dn0 / 3.0) * assign79090_body52_e120394) + (assign79090_body52_e120389 * (locals.var_chi_dn0 / 4.0))))))), ((locals.var_chi_dn2 * assign79090_body52_e120398) + (locals.var_chi * (((locals.var_chi_dn2 / 2.0) * assign79090_body52_e120396) + (assign79090_body52_e120385 * (((locals.var_chi_dn2 / 3.0) * assign79090_body52_e120394) + (assign79090_body52_e120389 * (locals.var_chi_dn2 / 4.0))))))), ((locals.var_chi_dn4 * assign79090_body52_e120398) + (locals.var_chi * (((locals.var_chi_dn4 / 2.0) * assign79090_body52_e120396) + (assign79090_body52_e120385 * (((locals.var_chi_dn4 / 3.0) * assign79090_body52_e120394) + (assign79090_body52_e120389 * (locals.var_chi_dn4 / 4.0))))))), ((locals.var_chi_dn5 * assign79090_body52_e120398) + (locals.var_chi * (((locals.var_chi_dn5 / 2.0) * assign79090_body52_e120396) + (assign79090_body52_e120385 * (((locals.var_chi_dn5 / 3.0) * assign79090_body52_e120394) + (assign79090_body52_e120389 * (locals.var_chi_dn5 / 4.0))))))), ((locals.var_chi_dn6 * assign79090_body52_e120398) + (locals.var_chi * (((locals.var_chi_dn6 / 2.0) * assign79090_body52_e120396) + (assign79090_body52_e120385 * (((locals.var_chi_dn6 / 3.0) * assign79090_body52_e120394) + (assign79090_body52_e120389 * (locals.var_chi_dn6 / 4.0))))))), ((locals.var_chi_dn7 * assign79090_body52_e120398) + (locals.var_chi * (((locals.var_chi_dn7 / 2.0) * assign79090_body52_e120396) + (assign79090_body52_e120385 * (((locals.var_chi_dn7 / 3.0) * assign79090_body52_e120394) + (assign79090_body52_e120389 * (locals.var_chi_dn7 / 4.0))))))), ((locals.var_chi_dn8 * assign79090_body52_e120398) + (locals.var_chi * (((locals.var_chi_dn8 / 2.0) * assign79090_body52_e120396) + (assign79090_body52_e120385 * (((locals.var_chi_dn8 / 3.0) * assign79090_body52_e120394) + (assign79090_body52_e120389 * (locals.var_chi_dn8 / 4.0))))))), ((locals.var_chi_dn9 * assign79090_body52_e120398) + (locals.var_chi * (((locals.var_chi_dn9 / 2.0) * assign79090_body52_e120396) + (assign79090_body52_e120385 * (((locals.var_chi_dn9 / 3.0) * assign79090_body52_e120394) + (assign79090_body52_e120389 * (locals.var_chi_dn9 / 4.0))))))), ((locals.var_chi_dn10 * assign79090_body52_e120398) + (locals.var_chi * (((locals.var_chi_dn10 / 2.0) * assign79090_body52_e120396) + (assign79090_body52_e120385 * (((locals.var_chi_dn10 / 3.0) * assign79090_body52_e120394) + (assign79090_body52_e120389 * (locals.var_chi_dn10 / 4.0))))))), ((locals.var_chi_dn11 * assign79090_body52_e120398) + (locals.var_chi * (((locals.var_chi_dn11 / 2.0) * assign79090_body52_e120396) + (assign79090_body52_e120385 * (((locals.var_chi_dn11 / 3.0) * assign79090_body52_e120394) + (assign79090_body52_e120389 * (locals.var_chi_dn11 / 4.0))))))), ((locals.var_chi_dn14 * assign79090_body52_e120398) + (locals.var_chi * (((locals.var_chi_dn14 / 2.0) * assign79090_body52_e120396) + (assign79090_body52_e120385 * (((locals.var_chi_dn14 / 3.0) * assign79090_body52_e120394) + (assign79090_body52_e120389 * (locals.var_chi_dn14 / 4.0))))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign79090_body52_e120401;
            locals.var_t1_dn0 = assign79090_body52_e120401_d_n0;
            locals.var_t1_dn2 = assign79090_body52_e120401_d_n2;
            locals.var_t1_dn4 = assign79090_body52_e120401_d_n4;
            locals.var_t1_dn5 = assign79090_body52_e120401_d_n5;
            locals.var_t1_dn6 = assign79090_body52_e120401_d_n6;
            locals.var_t1_dn7 = assign79090_body52_e120401_d_n7;
            locals.var_t1_dn8 = assign79090_body52_e120401_d_n8;
            locals.var_t1_dn9 = assign79090_body52_e120401_d_n9;
            locals.var_t1_dn10 = assign79090_body52_e120401_d_n10;
            locals.var_t1_dn11 = assign79090_body52_e120401_d_n11;
            locals.var_t1_dn14 = assign79090_body52_e120401_d_n14;
            let (assign79090_body53_e120419, assign79090_body53_e120419_d_n0, assign79090_body53_e120419_d_n2, assign79090_body53_e120419_d_n4, assign79090_body53_e120419_d_n5, assign79090_body53_e120419_d_n6, assign79090_body53_e120419_d_n7, assign79090_body53_e120419_d_n8, assign79090_body53_e120419_d_n9, assign79090_body53_e120419_d_n10, assign79090_body53_e120419_d_n11, assign79090_body53_e120419_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1839 == 0.0)) && (locals.var_guard1840 != 0.0)) && (locals.var_guard1841 != 0.0)) {
        let assign79090_body53_e120417: f64 = (locals.var_cfs1 * locals.var_t0);
        (assign79090_body53_e120417, ((locals.var_cfs1_dn0 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn0)), ((locals.var_cfs1_dn2 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn2)), ((locals.var_cfs1_dn4 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn4)), ((locals.var_cfs1_dn5 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn5)), ((locals.var_cfs1_dn6 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn6)), ((locals.var_cfs1_dn7 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn7)), ((locals.var_cfs1_dn8 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn8)), ((locals.var_cfs1_dn9 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn9)), ((locals.var_cfs1_dn10 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn10)), ((locals.var_cfs1_dn11 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn11)), ((locals.var_cfs1_dn14 * locals.var_t0) + (locals.var_cfs1 * locals.var_t0_dn14)),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign79090_body53_e120419;
            locals.var_fs01_dn0 = assign79090_body53_e120419_d_n0;
            locals.var_fs01_dn2 = assign79090_body53_e120419_d_n2;
            locals.var_fs01_dn4 = assign79090_body53_e120419_d_n4;
            locals.var_fs01_dn5 = assign79090_body53_e120419_d_n5;
            locals.var_fs01_dn6 = assign79090_body53_e120419_d_n6;
            locals.var_fs01_dn7 = assign79090_body53_e120419_d_n7;
            locals.var_fs01_dn8 = assign79090_body53_e120419_d_n8;
            locals.var_fs01_dn9 = assign79090_body53_e120419_d_n9;
            locals.var_fs01_dn10 = assign79090_body53_e120419_d_n10;
            locals.var_fs01_dn11 = assign79090_body53_e120419_d_n11;
            locals.var_fs01_dn14 = assign79090_body53_e120419_d_n14;
            let (assign79090_body54_e120439, assign79090_body54_e120439_d_n0, assign79090_body54_e120439_d_n2, assign79090_body54_e120439_d_n4, assign79090_body54_e120439_d_n5, assign79090_body54_e120439_d_n6, assign79090_body54_e120439_d_n7, assign79090_body54_e120439_d_n8, assign79090_body54_e120439_d_n9, assign79090_body54_e120439_d_n10, assign79090_body54_e120439_d_n11, assign79090_body54_e120439_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1839 == 0.0)) && (locals.var_guard1840 != 0.0)) && (locals.var_guard1841 != 0.0)) {
        let assign79090_body54_e120435: f64 = (locals.var_cfs1 * locals.var_t1);
        let assign79090_body54_e120437: f64 = (assign79090_body54_e120435 * locals.var_beta);
        (assign79090_body54_e120437, ((((locals.var_cfs1_dn0 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn0)) * locals.var_beta) + (assign79090_body54_e120435 * locals.var_beta_dn0)), ((((locals.var_cfs1_dn2 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn2)) * locals.var_beta) + (assign79090_body54_e120435 * locals.var_beta_dn2)), ((((locals.var_cfs1_dn4 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn4)) * locals.var_beta) + (assign79090_body54_e120435 * locals.var_beta_dn4)), ((((locals.var_cfs1_dn5 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn5)) * locals.var_beta) + (assign79090_body54_e120435 * locals.var_beta_dn5)), ((((locals.var_cfs1_dn6 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn6)) * locals.var_beta) + (assign79090_body54_e120435 * locals.var_beta_dn6)), ((((locals.var_cfs1_dn7 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn7)) * locals.var_beta) + (assign79090_body54_e120435 * locals.var_beta_dn7)), ((((locals.var_cfs1_dn8 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn8)) * locals.var_beta) + (assign79090_body54_e120435 * locals.var_beta_dn8)), ((((locals.var_cfs1_dn9 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn9)) * locals.var_beta) + (assign79090_body54_e120435 * locals.var_beta_dn9)), ((((locals.var_cfs1_dn10 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn10)) * locals.var_beta) + (assign79090_body54_e120435 * locals.var_beta_dn10)), ((((locals.var_cfs1_dn11 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn11)) * locals.var_beta) + (assign79090_body54_e120435 * locals.var_beta_dn11)), ((((locals.var_cfs1_dn14 * locals.var_t1) + (locals.var_cfs1 * locals.var_t1_dn14)) * locals.var_beta) + (assign79090_body54_e120435 * locals.var_beta_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign79090_body54_e120439;
            locals.var_fs01_dps0_dn0 = assign79090_body54_e120439_d_n0;
            locals.var_fs01_dps0_dn2 = assign79090_body54_e120439_d_n2;
            locals.var_fs01_dps0_dn4 = assign79090_body54_e120439_d_n4;
            locals.var_fs01_dps0_dn5 = assign79090_body54_e120439_d_n5;
            locals.var_fs01_dps0_dn6 = assign79090_body54_e120439_d_n6;
            locals.var_fs01_dps0_dn7 = assign79090_body54_e120439_d_n7;
            locals.var_fs01_dps0_dn8 = assign79090_body54_e120439_d_n8;
            locals.var_fs01_dps0_dn9 = assign79090_body54_e120439_d_n9;
            locals.var_fs01_dps0_dn10 = assign79090_body54_e120439_d_n10;
            locals.var_fs01_dps0_dn11 = assign79090_body54_e120439_d_n11;
            locals.var_fs01_dps0_dn14 = assign79090_body54_e120439_d_n14;
            let (assign79090_body55_e120457, assign79090_body55_e120457_d_n0, assign79090_body55_e120457_d_n2, assign79090_body55_e120457_d_n4, assign79090_body55_e120457_d_n5, assign79090_body55_e120457_d_n6, assign79090_body55_e120457_d_n7, assign79090_body55_e120457_d_n8, assign79090_body55_e120457_d_n9, assign79090_body55_e120457_d_n10, assign79090_body55_e120457_d_n11, assign79090_body55_e120457_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1839 == 0.0)) && (locals.var_guard1840 != 0.0)) && (locals.var_guard1841 == 0.0)) {
        let assign79090_body55_e120455: f64 = (locals.var_chi).exp();
        (assign79090_body55_e120455, (assign79090_body55_e120455 * locals.var_chi_dn0), (assign79090_body55_e120455 * locals.var_chi_dn2), (assign79090_body55_e120455 * locals.var_chi_dn4), (assign79090_body55_e120455 * locals.var_chi_dn5), (assign79090_body55_e120455 * locals.var_chi_dn6), (assign79090_body55_e120455 * locals.var_chi_dn7), (assign79090_body55_e120455 * locals.var_chi_dn8), (assign79090_body55_e120455 * locals.var_chi_dn9), (assign79090_body55_e120455 * locals.var_chi_dn10), (assign79090_body55_e120455 * locals.var_chi_dn11), (assign79090_body55_e120455 * locals.var_chi_dn14),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    }
};
            locals.var_exp_chi = assign79090_body55_e120457;
            locals.var_exp_chi_dn0 = assign79090_body55_e120457_d_n0;
            locals.var_exp_chi_dn2 = assign79090_body55_e120457_d_n2;
            locals.var_exp_chi_dn4 = assign79090_body55_e120457_d_n4;
            locals.var_exp_chi_dn5 = assign79090_body55_e120457_d_n5;
            locals.var_exp_chi_dn6 = assign79090_body55_e120457_d_n6;
            locals.var_exp_chi_dn7 = assign79090_body55_e120457_d_n7;
            locals.var_exp_chi_dn8 = assign79090_body55_e120457_d_n8;
            locals.var_exp_chi_dn9 = assign79090_body55_e120457_d_n9;
            locals.var_exp_chi_dn10 = assign79090_body55_e120457_d_n10;
            locals.var_exp_chi_dn11 = assign79090_body55_e120457_d_n11;
            locals.var_exp_chi_dn14 = assign79090_body55_e120457_d_n14;
            let (assign79090_body56_e120476, assign79090_body56_e120476_d_n0, assign79090_body56_e120476_d_n2, assign79090_body56_e120476_d_n4, assign79090_body56_e120476_d_n5, assign79090_body56_e120476_d_n6, assign79090_body56_e120476_d_n7, assign79090_body56_e120476_d_n8, assign79090_body56_e120476_d_n9, assign79090_body56_e120476_d_n10, assign79090_body56_e120476_d_n11, assign79090_body56_e120476_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1839 == 0.0)) && (locals.var_guard1840 != 0.0)) && (locals.var_guard1841 == 0.0)) {
        let assign79090_body56_e120474: f64 = (locals.var_exp_chi - 1.0);
        (assign79090_body56_e120474, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn4, locals.var_exp_chi_dn5, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn8, locals.var_exp_chi_dn9, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign79090_body56_e120476;
            locals.var_t1_dn0 = assign79090_body56_e120476_d_n0;
            locals.var_t1_dn2 = assign79090_body56_e120476_d_n2;
            locals.var_t1_dn4 = assign79090_body56_e120476_d_n4;
            locals.var_t1_dn5 = assign79090_body56_e120476_d_n5;
            locals.var_t1_dn6 = assign79090_body56_e120476_d_n6;
            locals.var_t1_dn7 = assign79090_body56_e120476_d_n7;
            locals.var_t1_dn8 = assign79090_body56_e120476_d_n8;
            locals.var_t1_dn9 = assign79090_body56_e120476_d_n9;
            locals.var_t1_dn10 = assign79090_body56_e120476_d_n10;
            locals.var_t1_dn11 = assign79090_body56_e120476_d_n11;
            locals.var_t1_dn14 = assign79090_body56_e120476_d_n14;
            let (assign79090_body57_e120497, assign79090_body57_e120497_d_n0, assign79090_body57_e120497_d_n2, assign79090_body57_e120497_d_n4, assign79090_body57_e120497_d_n5, assign79090_body57_e120497_d_n6, assign79090_body57_e120497_d_n7, assign79090_body57_e120497_d_n8, assign79090_body57_e120497_d_n9, assign79090_body57_e120497_d_n10, assign79090_body57_e120497_d_n11, assign79090_body57_e120497_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1839 == 0.0)) && (locals.var_guard1840 != 0.0)) && (locals.var_guard1841 == 0.0)) {
        let assign79090_body57_e120494: f64 = (locals.var_t1 - locals.var_chi);
        let assign79090_body57_e120495: f64 = (locals.var_cfs1 * assign79090_body57_e120494);
        (assign79090_body57_e120495, ((locals.var_cfs1_dn0 * assign79090_body57_e120494) + (locals.var_cfs1 * (locals.var_t1_dn0 - locals.var_chi_dn0))), ((locals.var_cfs1_dn2 * assign79090_body57_e120494) + (locals.var_cfs1 * (locals.var_t1_dn2 - locals.var_chi_dn2))), ((locals.var_cfs1_dn4 * assign79090_body57_e120494) + (locals.var_cfs1 * (locals.var_t1_dn4 - locals.var_chi_dn4))), ((locals.var_cfs1_dn5 * assign79090_body57_e120494) + (locals.var_cfs1 * (locals.var_t1_dn5 - locals.var_chi_dn5))), ((locals.var_cfs1_dn6 * assign79090_body57_e120494) + (locals.var_cfs1 * (locals.var_t1_dn6 - locals.var_chi_dn6))), ((locals.var_cfs1_dn7 * assign79090_body57_e120494) + (locals.var_cfs1 * (locals.var_t1_dn7 - locals.var_chi_dn7))), ((locals.var_cfs1_dn8 * assign79090_body57_e120494) + (locals.var_cfs1 * (locals.var_t1_dn8 - locals.var_chi_dn8))), ((locals.var_cfs1_dn9 * assign79090_body57_e120494) + (locals.var_cfs1 * (locals.var_t1_dn9 - locals.var_chi_dn9))), ((locals.var_cfs1_dn10 * assign79090_body57_e120494) + (locals.var_cfs1 * (locals.var_t1_dn10 - locals.var_chi_dn10))), ((locals.var_cfs1_dn11 * assign79090_body57_e120494) + (locals.var_cfs1 * (locals.var_t1_dn11 - locals.var_chi_dn11))), ((locals.var_cfs1_dn14 * assign79090_body57_e120494) + (locals.var_cfs1 * (locals.var_t1_dn14 - locals.var_chi_dn14))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign79090_body57_e120497;
            locals.var_fs01_dn0 = assign79090_body57_e120497_d_n0;
            locals.var_fs01_dn2 = assign79090_body57_e120497_d_n2;
            locals.var_fs01_dn4 = assign79090_body57_e120497_d_n4;
            locals.var_fs01_dn5 = assign79090_body57_e120497_d_n5;
            locals.var_fs01_dn6 = assign79090_body57_e120497_d_n6;
            locals.var_fs01_dn7 = assign79090_body57_e120497_d_n7;
            locals.var_fs01_dn8 = assign79090_body57_e120497_d_n8;
            locals.var_fs01_dn9 = assign79090_body57_e120497_d_n9;
            locals.var_fs01_dn10 = assign79090_body57_e120497_d_n10;
            locals.var_fs01_dn11 = assign79090_body57_e120497_d_n11;
            locals.var_fs01_dn14 = assign79090_body57_e120497_d_n14;
            let (assign79090_body58_e120518, assign79090_body58_e120518_d_n0, assign79090_body58_e120518_d_n2, assign79090_body58_e120518_d_n4, assign79090_body58_e120518_d_n5, assign79090_body58_e120518_d_n6, assign79090_body58_e120518_d_n7, assign79090_body58_e120518_d_n8, assign79090_body58_e120518_d_n9, assign79090_body58_e120518_d_n10, assign79090_body58_e120518_d_n11, assign79090_body58_e120518_d_n14,) = {
    if ((((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1839 == 0.0)) && (locals.var_guard1840 != 0.0)) && (locals.var_guard1841 == 0.0)) {
        let assign79090_body58_e120514: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign79090_body58_e120516: f64 = (assign79090_body58_e120514 * locals.var_t1);
        (assign79090_body58_e120516, ((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * locals.var_t1) + (assign79090_body58_e120514 * locals.var_t1_dn0)), ((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * locals.var_t1) + (assign79090_body58_e120514 * locals.var_t1_dn2)), ((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * locals.var_t1) + (assign79090_body58_e120514 * locals.var_t1_dn4)), ((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * locals.var_t1) + (assign79090_body58_e120514 * locals.var_t1_dn5)), ((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * locals.var_t1) + (assign79090_body58_e120514 * locals.var_t1_dn6)), ((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * locals.var_t1) + (assign79090_body58_e120514 * locals.var_t1_dn7)), ((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * locals.var_t1) + (assign79090_body58_e120514 * locals.var_t1_dn8)), ((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * locals.var_t1) + (assign79090_body58_e120514 * locals.var_t1_dn9)), ((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * locals.var_t1) + (assign79090_body58_e120514 * locals.var_t1_dn10)), ((((locals.var_cfs1_dn11 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn11)) * locals.var_t1) + (assign79090_body58_e120514 * locals.var_t1_dn11)), ((((locals.var_cfs1_dn14 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn14)) * locals.var_t1) + (assign79090_body58_e120514 * locals.var_t1_dn14)),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign79090_body58_e120518;
            locals.var_fs01_dps0_dn0 = assign79090_body58_e120518_d_n0;
            locals.var_fs01_dps0_dn2 = assign79090_body58_e120518_d_n2;
            locals.var_fs01_dps0_dn4 = assign79090_body58_e120518_d_n4;
            locals.var_fs01_dps0_dn5 = assign79090_body58_e120518_d_n5;
            locals.var_fs01_dps0_dn6 = assign79090_body58_e120518_d_n6;
            locals.var_fs01_dps0_dn7 = assign79090_body58_e120518_d_n7;
            locals.var_fs01_dps0_dn8 = assign79090_body58_e120518_d_n8;
            locals.var_fs01_dps0_dn9 = assign79090_body58_e120518_d_n9;
            locals.var_fs01_dps0_dn10 = assign79090_body58_e120518_d_n10;
            locals.var_fs01_dps0_dn11 = assign79090_body58_e120518_d_n11;
            locals.var_fs01_dps0_dn14 = assign79090_body58_e120518_d_n14;
            let (assign79090_body60_e120553, assign79090_body60_e120553_d_n0, assign79090_body60_e120553_d_n2, assign79090_body60_e120553_d_n4, assign79090_body60_e120553_d_n5, assign79090_body60_e120553_d_n6, assign79090_body60_e120553_d_n7, assign79090_body60_e120553_d_n8, assign79090_body60_e120553_d_n9, assign79090_body60_e120553_d_n10, assign79090_body60_e120553_d_n11, assign79090_body60_e120553_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1839 == 0.0)) && (locals.var_guard1840 == 0.0)) {
        let assign79090_body60_e120550: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign79090_body60_e120551: f64 = (assign79090_body60_e120550).exp();
        (assign79090_body60_e120551, (assign79090_body60_e120551 * ((locals.var_beta_dn0 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn0))), (assign79090_body60_e120551 * ((locals.var_beta_dn2 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn2))), (assign79090_body60_e120551 * ((locals.var_beta_dn4 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn4))), (assign79090_body60_e120551 * ((locals.var_beta_dn5 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn5))), (assign79090_body60_e120551 * ((locals.var_beta_dn6 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn6))), (assign79090_body60_e120551 * ((locals.var_beta_dn7 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn7))), (assign79090_body60_e120551 * ((locals.var_beta_dn8 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn8))), (assign79090_body60_e120551 * ((locals.var_beta_dn9 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn9))), (assign79090_body60_e120551 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign79090_body60_e120551 * ((locals.var_beta_dn11 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn11))), (assign79090_body60_e120551 * ((locals.var_beta_dn14 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn14))),)
    } else {
        (locals.var_exp_bps0, locals.var_exp_bps0_dn0, locals.var_exp_bps0_dn2, locals.var_exp_bps0_dn4, locals.var_exp_bps0_dn5, locals.var_exp_bps0_dn6, locals.var_exp_bps0_dn7, locals.var_exp_bps0_dn8, locals.var_exp_bps0_dn9, locals.var_exp_bps0_dn10, locals.var_exp_bps0_dn11, locals.var_exp_bps0_dn14,)
    }
};
            locals.var_exp_bps0 = assign79090_body60_e120553;
            locals.var_exp_bps0_dn0 = assign79090_body60_e120553_d_n0;
            locals.var_exp_bps0_dn2 = assign79090_body60_e120553_d_n2;
            locals.var_exp_bps0_dn4 = assign79090_body60_e120553_d_n4;
            locals.var_exp_bps0_dn5 = assign79090_body60_e120553_d_n5;
            locals.var_exp_bps0_dn6 = assign79090_body60_e120553_d_n6;
            locals.var_exp_bps0_dn7 = assign79090_body60_e120553_d_n7;
            locals.var_exp_bps0_dn8 = assign79090_body60_e120553_d_n8;
            locals.var_exp_bps0_dn9 = assign79090_body60_e120553_d_n9;
            locals.var_exp_bps0_dn10 = assign79090_body60_e120553_d_n10;
            locals.var_exp_bps0_dn11 = assign79090_body60_e120553_d_n11;
            locals.var_exp_bps0_dn14 = assign79090_body60_e120553_d_n14;
            let (assign79090_body61_e120576, assign79090_body61_e120576_d_n0, assign79090_body61_e120576_d_n2, assign79090_body61_e120576_d_n4, assign79090_body61_e120576_d_n5, assign79090_body61_e120576_d_n6, assign79090_body61_e120576_d_n7, assign79090_body61_e120576_d_n8, assign79090_body61_e120576_d_n9, assign79090_body61_e120576_d_n10, assign79090_body61_e120576_d_n11, assign79090_body61_e120576_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1839 == 0.0)) && (locals.var_guard1840 == 0.0)) {
        let assign79090_body61_e120571: f64 = (locals.var_chi + 1.0);
        let assign79090_body61_e120572: f64 = (locals.var_exp_bvbs * assign79090_body61_e120571);
        let assign79090_body61_e120573: f64 = (locals.var_exp_bps0 - assign79090_body61_e120572);
        let assign79090_body61_e120574: f64 = (locals.var_cnst1over * assign79090_body61_e120573);
        (assign79090_body61_e120574, ((locals.var_cnst1over_dn0 * assign79090_body61_e120573) + (locals.var_cnst1over * (locals.var_exp_bps0_dn0 - ((locals.var_exp_bvbs_dn0 * assign79090_body61_e120571) + (locals.var_exp_bvbs * locals.var_chi_dn0))))), ((locals.var_cnst1over_dn2 * assign79090_body61_e120573) + (locals.var_cnst1over * (locals.var_exp_bps0_dn2 - ((locals.var_exp_bvbs_dn2 * assign79090_body61_e120571) + (locals.var_exp_bvbs * locals.var_chi_dn2))))), ((locals.var_cnst1over_dn4 * assign79090_body61_e120573) + (locals.var_cnst1over * (locals.var_exp_bps0_dn4 - ((locals.var_exp_bvbs_dn4 * assign79090_body61_e120571) + (locals.var_exp_bvbs * locals.var_chi_dn4))))), ((locals.var_cnst1over_dn5 * assign79090_body61_e120573) + (locals.var_cnst1over * (locals.var_exp_bps0_dn5 - ((locals.var_exp_bvbs_dn5 * assign79090_body61_e120571) + (locals.var_exp_bvbs * locals.var_chi_dn5))))), ((locals.var_cnst1over_dn6 * assign79090_body61_e120573) + (locals.var_cnst1over * (locals.var_exp_bps0_dn6 - ((locals.var_exp_bvbs_dn6 * assign79090_body61_e120571) + (locals.var_exp_bvbs * locals.var_chi_dn6))))), ((locals.var_cnst1over_dn7 * assign79090_body61_e120573) + (locals.var_cnst1over * (locals.var_exp_bps0_dn7 - ((locals.var_exp_bvbs_dn7 * assign79090_body61_e120571) + (locals.var_exp_bvbs * locals.var_chi_dn7))))), ((locals.var_cnst1over_dn8 * assign79090_body61_e120573) + (locals.var_cnst1over * (locals.var_exp_bps0_dn8 - ((locals.var_exp_bvbs_dn8 * assign79090_body61_e120571) + (locals.var_exp_bvbs * locals.var_chi_dn8))))), ((locals.var_cnst1over_dn9 * assign79090_body61_e120573) + (locals.var_cnst1over * (locals.var_exp_bps0_dn9 - ((locals.var_exp_bvbs_dn9 * assign79090_body61_e120571) + (locals.var_exp_bvbs * locals.var_chi_dn9))))), ((locals.var_cnst1over_dn10 * assign79090_body61_e120573) + (locals.var_cnst1over * (locals.var_exp_bps0_dn10 - ((locals.var_exp_bvbs_dn10 * assign79090_body61_e120571) + (locals.var_exp_bvbs * locals.var_chi_dn10))))), ((locals.var_cnst1over_dn11 * assign79090_body61_e120573) + (locals.var_cnst1over * (locals.var_exp_bps0_dn11 - ((locals.var_exp_bvbs_dn11 * assign79090_body61_e120571) + (locals.var_exp_bvbs * locals.var_chi_dn11))))), ((locals.var_cnst1over_dn14 * assign79090_body61_e120573) + (locals.var_cnst1over * (locals.var_exp_bps0_dn14 - ((locals.var_exp_bvbs_dn14 * assign79090_body61_e120571) + (locals.var_exp_bvbs * locals.var_chi_dn14))))),)
    } else {
        (locals.var_fs01, locals.var_fs01_dn0, locals.var_fs01_dn2, locals.var_fs01_dn4, locals.var_fs01_dn5, locals.var_fs01_dn6, locals.var_fs01_dn7, locals.var_fs01_dn8, locals.var_fs01_dn9, locals.var_fs01_dn10, locals.var_fs01_dn11, locals.var_fs01_dn14,)
    }
};
            locals.var_fs01 = assign79090_body61_e120576;
            locals.var_fs01_dn0 = assign79090_body61_e120576_d_n0;
            locals.var_fs01_dn2 = assign79090_body61_e120576_d_n2;
            locals.var_fs01_dn4 = assign79090_body61_e120576_d_n4;
            locals.var_fs01_dn5 = assign79090_body61_e120576_d_n5;
            locals.var_fs01_dn6 = assign79090_body61_e120576_d_n6;
            locals.var_fs01_dn7 = assign79090_body61_e120576_d_n7;
            locals.var_fs01_dn8 = assign79090_body61_e120576_d_n8;
            locals.var_fs01_dn9 = assign79090_body61_e120576_d_n9;
            locals.var_fs01_dn10 = assign79090_body61_e120576_d_n10;
            locals.var_fs01_dn11 = assign79090_body61_e120576_d_n11;
            locals.var_fs01_dn14 = assign79090_body61_e120576_d_n14;
            let (assign79090_body62_e120597, assign79090_body62_e120597_d_n0, assign79090_body62_e120597_d_n2, assign79090_body62_e120597_d_n4, assign79090_body62_e120597_d_n5, assign79090_body62_e120597_d_n6, assign79090_body62_e120597_d_n7, assign79090_body62_e120597_d_n8, assign79090_body62_e120597_d_n9, assign79090_body62_e120597_d_n10, assign79090_body62_e120597_d_n11, assign79090_body62_e120597_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1839 == 0.0)) && (locals.var_guard1840 == 0.0)) {
        let assign79090_body62_e120591: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign79090_body62_e120594: f64 = (locals.var_exp_bps0 - locals.var_exp_bvbs);
        let assign79090_body62_e120595: f64 = (assign79090_body62_e120591 * assign79090_body62_e120594);
        (assign79090_body62_e120595, ((((locals.var_cnst1over_dn0 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn0)) * assign79090_body62_e120594) + (assign79090_body62_e120591 * (locals.var_exp_bps0_dn0 - locals.var_exp_bvbs_dn0))), ((((locals.var_cnst1over_dn2 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn2)) * assign79090_body62_e120594) + (assign79090_body62_e120591 * (locals.var_exp_bps0_dn2 - locals.var_exp_bvbs_dn2))), ((((locals.var_cnst1over_dn4 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn4)) * assign79090_body62_e120594) + (assign79090_body62_e120591 * (locals.var_exp_bps0_dn4 - locals.var_exp_bvbs_dn4))), ((((locals.var_cnst1over_dn5 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn5)) * assign79090_body62_e120594) + (assign79090_body62_e120591 * (locals.var_exp_bps0_dn5 - locals.var_exp_bvbs_dn5))), ((((locals.var_cnst1over_dn6 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn6)) * assign79090_body62_e120594) + (assign79090_body62_e120591 * (locals.var_exp_bps0_dn6 - locals.var_exp_bvbs_dn6))), ((((locals.var_cnst1over_dn7 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn7)) * assign79090_body62_e120594) + (assign79090_body62_e120591 * (locals.var_exp_bps0_dn7 - locals.var_exp_bvbs_dn7))), ((((locals.var_cnst1over_dn8 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn8)) * assign79090_body62_e120594) + (assign79090_body62_e120591 * (locals.var_exp_bps0_dn8 - locals.var_exp_bvbs_dn8))), ((((locals.var_cnst1over_dn9 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn9)) * assign79090_body62_e120594) + (assign79090_body62_e120591 * (locals.var_exp_bps0_dn9 - locals.var_exp_bvbs_dn9))), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * assign79090_body62_e120594) + (assign79090_body62_e120591 * (locals.var_exp_bps0_dn10 - locals.var_exp_bvbs_dn10))), ((((locals.var_cnst1over_dn11 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn11)) * assign79090_body62_e120594) + (assign79090_body62_e120591 * (locals.var_exp_bps0_dn11 - locals.var_exp_bvbs_dn11))), ((((locals.var_cnst1over_dn14 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn14)) * assign79090_body62_e120594) + (assign79090_body62_e120591 * (locals.var_exp_bps0_dn14 - locals.var_exp_bvbs_dn14))),)
    } else {
        (locals.var_fs01_dps0, locals.var_fs01_dps0_dn0, locals.var_fs01_dps0_dn2, locals.var_fs01_dps0_dn4, locals.var_fs01_dps0_dn5, locals.var_fs01_dps0_dn6, locals.var_fs01_dps0_dn7, locals.var_fs01_dps0_dn8, locals.var_fs01_dps0_dn9, locals.var_fs01_dps0_dn10, locals.var_fs01_dps0_dn11, locals.var_fs01_dps0_dn14,)
    }
};
            locals.var_fs01_dps0 = assign79090_body62_e120597;
            locals.var_fs01_dps0_dn0 = assign79090_body62_e120597_d_n0;
            locals.var_fs01_dps0_dn2 = assign79090_body62_e120597_d_n2;
            locals.var_fs01_dps0_dn4 = assign79090_body62_e120597_d_n4;
            locals.var_fs01_dps0_dn5 = assign79090_body62_e120597_d_n5;
            locals.var_fs01_dps0_dn6 = assign79090_body62_e120597_d_n6;
            locals.var_fs01_dps0_dn7 = assign79090_body62_e120597_d_n7;
            locals.var_fs01_dps0_dn8 = assign79090_body62_e120597_d_n8;
            locals.var_fs01_dps0_dn9 = assign79090_body62_e120597_d_n9;
            locals.var_fs01_dps0_dn10 = assign79090_body62_e120597_d_n10;
            locals.var_fs01_dps0_dn11 = assign79090_body62_e120597_d_n11;
            locals.var_fs01_dps0_dn14 = assign79090_body62_e120597_d_n14;
            let assign79090_body63_e120600: f64 = if locals.var_fs01 > 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1842 = assign79090_body63_e120600;
            let (assign79090_body64_e120619, assign79090_body64_e120619_d_n0, assign79090_body64_e120619_d_n2, assign79090_body64_e120619_d_n4, assign79090_body64_e120619_d_n5, assign79090_body64_e120619_d_n6, assign79090_body64_e120619_d_n7, assign79090_body64_e120619_d_n8, assign79090_body64_e120619_d_n9, assign79090_body64_e120619_d_n10, assign79090_body64_e120619_d_n11, assign79090_body64_e120619_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1839 == 0.0)) && (locals.var_guard1842 != 0.0)) {
        let assign79090_body64_e120614: f64 = (locals.var_fb * locals.var_fb);
        let assign79090_body64_e120616: f64 = (assign79090_body64_e120614 + locals.var_fs01);
        let assign79090_body64_e120617: f64 = (assign79090_body64_e120616).sqrt();
        (assign79090_body64_e120617, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fs01_dn0) / (2.0 * assign79090_body64_e120617)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fs01_dn2) / (2.0 * assign79090_body64_e120617)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fs01_dn4) / (2.0 * assign79090_body64_e120617)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fs01_dn5) / (2.0 * assign79090_body64_e120617)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fs01_dn6) / (2.0 * assign79090_body64_e120617)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fs01_dn7) / (2.0 * assign79090_body64_e120617)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fs01_dn8) / (2.0 * assign79090_body64_e120617)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fs01_dn9) / (2.0 * assign79090_body64_e120617)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fs01_dn10) / (2.0 * assign79090_body64_e120617)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fs01_dn11) / (2.0 * assign79090_body64_e120617)), ((((locals.var_fb_dn14 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn14)) + locals.var_fs01_dn14) / (2.0 * assign79090_body64_e120617)),)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign79090_body64_e120619;
            locals.var_fs02_dn0 = assign79090_body64_e120619_d_n0;
            locals.var_fs02_dn2 = assign79090_body64_e120619_d_n2;
            locals.var_fs02_dn4 = assign79090_body64_e120619_d_n4;
            locals.var_fs02_dn5 = assign79090_body64_e120619_d_n5;
            locals.var_fs02_dn6 = assign79090_body64_e120619_d_n6;
            locals.var_fs02_dn7 = assign79090_body64_e120619_d_n7;
            locals.var_fs02_dn8 = assign79090_body64_e120619_d_n8;
            locals.var_fs02_dn9 = assign79090_body64_e120619_d_n9;
            locals.var_fs02_dn10 = assign79090_body64_e120619_d_n10;
            locals.var_fs02_dn11 = assign79090_body64_e120619_d_n11;
            locals.var_fs02_dn14 = assign79090_body64_e120619_d_n14;
            let (assign79090_body65_e120643, assign79090_body65_e120643_d_n0, assign79090_body65_e120643_d_n2, assign79090_body65_e120643_d_n4, assign79090_body65_e120643_d_n5, assign79090_body65_e120643_d_n6, assign79090_body65_e120643_d_n7, assign79090_body65_e120643_d_n8, assign79090_body65_e120643_d_n9, assign79090_body65_e120643_d_n10, assign79090_body65_e120643_d_n11, assign79090_body65_e120643_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1839 == 0.0)) && (locals.var_guard1842 != 0.0)) {
        let assign79090_body65_e120634: f64 = (2.0 * locals.var_fb_dpss);
        let assign79090_body65_e120636: f64 = (assign79090_body65_e120634 * locals.var_fb);
        let assign79090_body65_e120638: f64 = (assign79090_body65_e120636 + locals.var_fs01_dps0);
        let assign79090_body65_e120639: f64 = (0.5 * assign79090_body65_e120638);
        let assign79090_body65_e120641: f64 = (assign79090_body65_e120639 / locals.var_fs02);
        (assign79090_body65_e120641, ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn0) * locals.var_fb) + (assign79090_body65_e120634 * locals.var_fb_dn0)) + locals.var_fs01_dps0_dn0)) * locals.var_fs02) - (assign79090_body65_e120639 * locals.var_fs02_dn0)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn2) * locals.var_fb) + (assign79090_body65_e120634 * locals.var_fb_dn2)) + locals.var_fs01_dps0_dn2)) * locals.var_fs02) - (assign79090_body65_e120639 * locals.var_fs02_dn2)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn4) * locals.var_fb) + (assign79090_body65_e120634 * locals.var_fb_dn4)) + locals.var_fs01_dps0_dn4)) * locals.var_fs02) - (assign79090_body65_e120639 * locals.var_fs02_dn4)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn5) * locals.var_fb) + (assign79090_body65_e120634 * locals.var_fb_dn5)) + locals.var_fs01_dps0_dn5)) * locals.var_fs02) - (assign79090_body65_e120639 * locals.var_fs02_dn5)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn6) * locals.var_fb) + (assign79090_body65_e120634 * locals.var_fb_dn6)) + locals.var_fs01_dps0_dn6)) * locals.var_fs02) - (assign79090_body65_e120639 * locals.var_fs02_dn6)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn7) * locals.var_fb) + (assign79090_body65_e120634 * locals.var_fb_dn7)) + locals.var_fs01_dps0_dn7)) * locals.var_fs02) - (assign79090_body65_e120639 * locals.var_fs02_dn7)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn8) * locals.var_fb) + (assign79090_body65_e120634 * locals.var_fb_dn8)) + locals.var_fs01_dps0_dn8)) * locals.var_fs02) - (assign79090_body65_e120639 * locals.var_fs02_dn8)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn9) * locals.var_fb) + (assign79090_body65_e120634 * locals.var_fb_dn9)) + locals.var_fs01_dps0_dn9)) * locals.var_fs02) - (assign79090_body65_e120639 * locals.var_fs02_dn9)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn10) * locals.var_fb) + (assign79090_body65_e120634 * locals.var_fb_dn10)) + locals.var_fs01_dps0_dn10)) * locals.var_fs02) - (assign79090_body65_e120639 * locals.var_fs02_dn10)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn11) * locals.var_fb) + (assign79090_body65_e120634 * locals.var_fb_dn11)) + locals.var_fs01_dps0_dn11)) * locals.var_fs02) - (assign79090_body65_e120639 * locals.var_fs02_dn11)) / (locals.var_fs02 * locals.var_fs02)), ((((0.5 * ((((2.0 * locals.var_fb_dpss_dn14) * locals.var_fb) + (assign79090_body65_e120634 * locals.var_fb_dn14)) + locals.var_fs01_dps0_dn14)) * locals.var_fs02) - (assign79090_body65_e120639 * locals.var_fs02_dn14)) / (locals.var_fs02 * locals.var_fs02)),)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign79090_body65_e120643;
            locals.var_fs02_dps0_dn0 = assign79090_body65_e120643_d_n0;
            locals.var_fs02_dps0_dn2 = assign79090_body65_e120643_d_n2;
            locals.var_fs02_dps0_dn4 = assign79090_body65_e120643_d_n4;
            locals.var_fs02_dps0_dn5 = assign79090_body65_e120643_d_n5;
            locals.var_fs02_dps0_dn6 = assign79090_body65_e120643_d_n6;
            locals.var_fs02_dps0_dn7 = assign79090_body65_e120643_d_n7;
            locals.var_fs02_dps0_dn8 = assign79090_body65_e120643_d_n8;
            locals.var_fs02_dps0_dn9 = assign79090_body65_e120643_d_n9;
            locals.var_fs02_dps0_dn10 = assign79090_body65_e120643_d_n10;
            locals.var_fs02_dps0_dn11 = assign79090_body65_e120643_d_n11;
            locals.var_fs02_dps0_dn14 = assign79090_body65_e120643_d_n14;
            let (assign79090_body67_e120675, assign79090_body67_e120675_d_n0, assign79090_body67_e120675_d_n2, assign79090_body67_e120675_d_n4, assign79090_body67_e120675_d_n5, assign79090_body67_e120675_d_n6, assign79090_body67_e120675_d_n7, assign79090_body67_e120675_d_n8, assign79090_body67_e120675_d_n9, assign79090_body67_e120675_d_n10, assign79090_body67_e120675_d_n11, assign79090_body67_e120675_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1839 == 0.0)) && (locals.var_guard1842 == 0.0)) {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    } else {
        (locals.var_fs02, locals.var_fs02_dn0, locals.var_fs02_dn2, locals.var_fs02_dn4, locals.var_fs02_dn5, locals.var_fs02_dn6, locals.var_fs02_dn7, locals.var_fs02_dn8, locals.var_fs02_dn9, locals.var_fs02_dn10, locals.var_fs02_dn11, locals.var_fs02_dn14,)
    }
};
            locals.var_fs02 = assign79090_body67_e120675;
            locals.var_fs02_dn0 = assign79090_body67_e120675_d_n0;
            locals.var_fs02_dn2 = assign79090_body67_e120675_d_n2;
            locals.var_fs02_dn4 = assign79090_body67_e120675_d_n4;
            locals.var_fs02_dn5 = assign79090_body67_e120675_d_n5;
            locals.var_fs02_dn6 = assign79090_body67_e120675_d_n6;
            locals.var_fs02_dn7 = assign79090_body67_e120675_d_n7;
            locals.var_fs02_dn8 = assign79090_body67_e120675_d_n8;
            locals.var_fs02_dn9 = assign79090_body67_e120675_d_n9;
            locals.var_fs02_dn10 = assign79090_body67_e120675_d_n10;
            locals.var_fs02_dn11 = assign79090_body67_e120675_d_n11;
            locals.var_fs02_dn14 = assign79090_body67_e120675_d_n14;
            let (assign79090_body68_e120690, assign79090_body68_e120690_d_n0, assign79090_body68_e120690_d_n2, assign79090_body68_e120690_d_n4, assign79090_body68_e120690_d_n5, assign79090_body68_e120690_d_n6, assign79090_body68_e120690_d_n7, assign79090_body68_e120690_d_n8, assign79090_body68_e120690_d_n9, assign79090_body68_e120690_d_n10, assign79090_body68_e120690_d_n11, assign79090_body68_e120690_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1839 == 0.0)) && (locals.var_guard1842 == 0.0)) {
        (locals.var_fb_dpss, locals.var_fb_dpss_dn0, locals.var_fb_dpss_dn2, locals.var_fb_dpss_dn4, locals.var_fb_dpss_dn5, locals.var_fb_dpss_dn6, locals.var_fb_dpss_dn7, locals.var_fb_dpss_dn8, locals.var_fb_dpss_dn9, locals.var_fb_dpss_dn10, locals.var_fb_dpss_dn11, locals.var_fb_dpss_dn14,)
    } else {
        (locals.var_fs02_dps0, locals.var_fs02_dps0_dn0, locals.var_fs02_dps0_dn2, locals.var_fs02_dps0_dn4, locals.var_fs02_dps0_dn5, locals.var_fs02_dps0_dn6, locals.var_fs02_dps0_dn7, locals.var_fs02_dps0_dn8, locals.var_fs02_dps0_dn9, locals.var_fs02_dps0_dn10, locals.var_fs02_dps0_dn11, locals.var_fs02_dps0_dn14,)
    }
};
            locals.var_fs02_dps0 = assign79090_body68_e120690;
            locals.var_fs02_dps0_dn0 = assign79090_body68_e120690_d_n0;
            locals.var_fs02_dps0_dn2 = assign79090_body68_e120690_d_n2;
            locals.var_fs02_dps0_dn4 = assign79090_body68_e120690_d_n4;
            locals.var_fs02_dps0_dn5 = assign79090_body68_e120690_d_n5;
            locals.var_fs02_dps0_dn6 = assign79090_body68_e120690_d_n6;
            locals.var_fs02_dps0_dn7 = assign79090_body68_e120690_d_n7;
            locals.var_fs02_dps0_dn8 = assign79090_body68_e120690_d_n8;
            locals.var_fs02_dps0_dn9 = assign79090_body68_e120690_d_n9;
            locals.var_fs02_dps0_dn10 = assign79090_body68_e120690_d_n10;
            locals.var_fs02_dps0_dn11 = assign79090_body68_e120690_d_n11;
            locals.var_fs02_dps0_dn14 = assign79090_body68_e120690_d_n14;
            let (assign79090_body69_e120706, assign79090_body69_e120706_d_n0, assign79090_body69_e120706_d_n2, assign79090_body69_e120706_d_n4, assign79090_body69_e120706_d_n5, assign79090_body69_e120706_d_n6, assign79090_body69_e120706_d_n7, assign79090_body69_e120706_d_n8, assign79090_body69_e120706_d_n9, assign79090_body69_e120706_d_n10, assign79090_body69_e120706_d_n11, assign79090_body69_e120706_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign79090_body69_e120698: f64 = (-locals.var_vgpld);
        let assign79090_body69_e120700: f64 = (assign79090_body69_e120698 + locals.var_ps0ld);
        let assign79090_body69_e120703: f64 = (locals.var_fac1 * locals.var_fs02);
        let assign79090_body69_e120704: f64 = (assign79090_body69_e120700 + assign79090_body69_e120703);
        (assign79090_body69_e120704, (locals.var_ps0ld_dn0 + ((locals.var_fac1_dn0 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn0))), (((-locals.var_vgpld_dn2) + locals.var_ps0ld_dn2) + ((locals.var_fac1_dn2 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn2))), (locals.var_ps0ld_dn4 + ((locals.var_fac1_dn4 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn4))), (locals.var_ps0ld_dn5 + ((locals.var_fac1_dn5 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn5))), (locals.var_ps0ld_dn6 + ((locals.var_fac1_dn6 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn6))), (((-locals.var_vgpld_dn7) + locals.var_ps0ld_dn7) + ((locals.var_fac1_dn7 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn7))), (((-locals.var_vgpld_dn8) + locals.var_ps0ld_dn8) + ((locals.var_fac1_dn8 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn8))), (((-locals.var_vgpld_dn9) + locals.var_ps0ld_dn9) + ((locals.var_fac1_dn9 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn9))), (locals.var_ps0ld_dn10 + ((locals.var_fac1_dn10 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn10))), (locals.var_ps0ld_dn11 + ((locals.var_fac1_dn11 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn11))), (locals.var_ps0ld_dn14 + ((locals.var_fac1_dn14 * locals.var_fs02) + (locals.var_fac1 * locals.var_fs02_dn14))),)
    } else {
        (locals.var_fs0, locals.var_fs0_dn0, locals.var_fs0_dn2, locals.var_fs0_dn4, locals.var_fs0_dn5, locals.var_fs0_dn6, locals.var_fs0_dn7, locals.var_fs0_dn8, locals.var_fs0_dn9, locals.var_fs0_dn10, locals.var_fs0_dn11, locals.var_fs0_dn14,)
    }
};
            locals.var_fs0 = assign79090_body69_e120706;
            locals.var_fs0_dn0 = assign79090_body69_e120706_d_n0;
            locals.var_fs0_dn2 = assign79090_body69_e120706_d_n2;
            locals.var_fs0_dn4 = assign79090_body69_e120706_d_n4;
            locals.var_fs0_dn5 = assign79090_body69_e120706_d_n5;
            locals.var_fs0_dn6 = assign79090_body69_e120706_d_n6;
            locals.var_fs0_dn7 = assign79090_body69_e120706_d_n7;
            locals.var_fs0_dn8 = assign79090_body69_e120706_d_n8;
            locals.var_fs0_dn9 = assign79090_body69_e120706_d_n9;
            locals.var_fs0_dn10 = assign79090_body69_e120706_d_n10;
            locals.var_fs0_dn11 = assign79090_body69_e120706_d_n11;
            locals.var_fs0_dn14 = assign79090_body69_e120706_d_n14;
            let (assign79090_body70_e120719, assign79090_body70_e120719_d_n0, assign79090_body70_e120719_d_n2, assign79090_body70_e120719_d_n4, assign79090_body70_e120719_d_n5, assign79090_body70_e120719_d_n6, assign79090_body70_e120719_d_n7, assign79090_body70_e120719_d_n8, assign79090_body70_e120719_d_n9, assign79090_body70_e120719_d_n10, assign79090_body70_e120719_d_n11, assign79090_body70_e120719_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign79090_body70_e120716: f64 = (locals.var_fac1 * locals.var_fs02_dps0);
        let assign79090_body70_e120717: f64 = (1.0 + assign79090_body70_e120716);
        (assign79090_body70_e120717, ((locals.var_fac1_dn0 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn0)), ((locals.var_fac1_dn2 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn2)), ((locals.var_fac1_dn4 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn4)), ((locals.var_fac1_dn5 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn5)), ((locals.var_fac1_dn6 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn6)), ((locals.var_fac1_dn7 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn7)), ((locals.var_fac1_dn8 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn8)), ((locals.var_fac1_dn9 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn9)), ((locals.var_fac1_dn10 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn10)), ((locals.var_fac1_dn11 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn11)), ((locals.var_fac1_dn14 * locals.var_fs02_dps0) + (locals.var_fac1 * locals.var_fs02_dps0_dn14)),)
    } else {
        (locals.var_fs0_dps0, locals.var_fs0_dps0_dn0, locals.var_fs0_dps0_dn2, locals.var_fs0_dps0_dn4, locals.var_fs0_dps0_dn5, locals.var_fs0_dps0_dn6, locals.var_fs0_dps0_dn7, locals.var_fs0_dps0_dn8, locals.var_fs0_dps0_dn9, locals.var_fs0_dps0_dn10, locals.var_fs0_dps0_dn11, locals.var_fs0_dps0_dn14,)
    }
};
            locals.var_fs0_dps0 = assign79090_body70_e120719;
            locals.var_fs0_dps0_dn0 = assign79090_body70_e120719_d_n0;
            locals.var_fs0_dps0_dn2 = assign79090_body70_e120719_d_n2;
            locals.var_fs0_dps0_dn4 = assign79090_body70_e120719_d_n4;
            locals.var_fs0_dps0_dn5 = assign79090_body70_e120719_d_n5;
            locals.var_fs0_dps0_dn6 = assign79090_body70_e120719_d_n6;
            locals.var_fs0_dps0_dn7 = assign79090_body70_e120719_d_n7;
            locals.var_fs0_dps0_dn8 = assign79090_body70_e120719_d_n8;
            locals.var_fs0_dps0_dn9 = assign79090_body70_e120719_d_n9;
            locals.var_fs0_dps0_dn10 = assign79090_body70_e120719_d_n10;
            locals.var_fs0_dps0_dn11 = assign79090_body70_e120719_d_n11;
            locals.var_fs0_dps0_dn14 = assign79090_body70_e120719_d_n14;
            let assign79090_body71_e120722: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1843 = assign79090_body71_e120722;
            let (assign79090_body72_e120735,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1843 != 0.0)) {
        let assign79090_body72_e120733: f64 = (locals.var_lp_s0_max + 1.0);
        (assign79090_body72_e120733,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign79090_body72_e120735;
            let (assign79090_body73_e120750, assign79090_body73_e120750_d_n0, assign79090_body73_e120750_d_n2, assign79090_body73_e120750_d_n4, assign79090_body73_e120750_d_n5, assign79090_body73_e120750_d_n6, assign79090_body73_e120750_d_n7, assign79090_body73_e120750_d_n8, assign79090_body73_e120750_d_n9, assign79090_body73_e120750_d_n10, assign79090_body73_e120750_d_n11, assign79090_body73_e120750_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1843 == 0.0)) {
        let assign79090_body73_e120746: f64 = (-locals.var_fs0);
        let assign79090_body73_e120748: f64 = (assign79090_body73_e120746 / locals.var_fs0_dps0);
        (assign79090_body73_e120748, ((((-locals.var_fs0_dn0) * locals.var_fs0_dps0) - (assign79090_body73_e120746 * locals.var_fs0_dps0_dn0)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn2) * locals.var_fs0_dps0) - (assign79090_body73_e120746 * locals.var_fs0_dps0_dn2)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn4) * locals.var_fs0_dps0) - (assign79090_body73_e120746 * locals.var_fs0_dps0_dn4)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn5) * locals.var_fs0_dps0) - (assign79090_body73_e120746 * locals.var_fs0_dps0_dn5)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn6) * locals.var_fs0_dps0) - (assign79090_body73_e120746 * locals.var_fs0_dps0_dn6)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn7) * locals.var_fs0_dps0) - (assign79090_body73_e120746 * locals.var_fs0_dps0_dn7)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn8) * locals.var_fs0_dps0) - (assign79090_body73_e120746 * locals.var_fs0_dps0_dn8)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn9) * locals.var_fs0_dps0) - (assign79090_body73_e120746 * locals.var_fs0_dps0_dn9)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn10) * locals.var_fs0_dps0) - (assign79090_body73_e120746 * locals.var_fs0_dps0_dn10)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn11) * locals.var_fs0_dps0) - (assign79090_body73_e120746 * locals.var_fs0_dps0_dn11)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)), ((((-locals.var_fs0_dn14) * locals.var_fs0_dps0) - (assign79090_body73_e120746 * locals.var_fs0_dps0_dn14)) / (locals.var_fs0_dps0 * locals.var_fs0_dps0)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign79090_body73_e120750;
            locals.var_dps0_dn0 = assign79090_body73_e120750_d_n0;
            locals.var_dps0_dn2 = assign79090_body73_e120750_d_n2;
            locals.var_dps0_dn4 = assign79090_body73_e120750_d_n4;
            locals.var_dps0_dn5 = assign79090_body73_e120750_d_n5;
            locals.var_dps0_dn6 = assign79090_body73_e120750_d_n6;
            locals.var_dps0_dn7 = assign79090_body73_e120750_d_n7;
            locals.var_dps0_dn8 = assign79090_body73_e120750_d_n8;
            locals.var_dps0_dn9 = assign79090_body73_e120750_d_n9;
            locals.var_dps0_dn10 = assign79090_body73_e120750_d_n10;
            locals.var_dps0_dn11 = assign79090_body73_e120750_d_n11;
            locals.var_dps0_dn14 = assign79090_body73_e120750_d_n14;
            let (assign79090_body74_e120775, assign79090_body74_e120775_d_n0, assign79090_body74_e120775_d_n2, assign79090_body74_e120775_d_n4, assign79090_body74_e120775_d_n5, assign79090_body74_e120775_d_n6, assign79090_body74_e120775_d_n7, assign79090_body74_e120775_d_n8, assign79090_body74_e120775_d_n9, assign79090_body74_e120775_d_n10, assign79090_body74_e120775_d_n11, assign79090_body74_e120775_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1843 == 0.0)) {
        let assign79090_body74_e120762: f64 = (0.5 * 0.1);
        let assign79090_body74_e120766: f64 = (locals.var_ps0ld).abs();
        let (assign79090_body74_e120771, assign79090_body74_e120771_d_n0, assign79090_body74_e120771_d_n2, assign79090_body74_e120771_d_n4, assign79090_body74_e120771_d_n5, assign79090_body74_e120771_d_n6, assign79090_body74_e120771_d_n7, assign79090_body74_e120771_d_n8, assign79090_body74_e120771_d_n9, assign79090_body74_e120771_d_n10, assign79090_body74_e120771_d_n11, assign79090_body74_e120771_d_n14,) = {
            if (1.0 >= assign79090_body74_e120766) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign79090_body74_e120770: f64 = (locals.var_ps0ld).abs();
                (assign79090_body74_e120770, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn4 } else { (-locals.var_ps0ld_dn4) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn5 } else { (-locals.var_ps0ld_dn5) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn8 } else { (-locals.var_ps0ld_dn8) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn9 } else { (-locals.var_ps0ld_dn9) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn11 } else { (-locals.var_ps0ld_dn11) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn14 } else { (-locals.var_ps0ld_dn14) },)
            }
        };
        let assign79090_body74_e120772: f64 = (1.0 + assign79090_body74_e120771);
        let assign79090_body74_e120773: f64 = (assign79090_body74_e120762 * assign79090_body74_e120772);
        (assign79090_body74_e120773, (assign79090_body74_e120762 * assign79090_body74_e120771_d_n0), (assign79090_body74_e120762 * assign79090_body74_e120771_d_n2), (assign79090_body74_e120762 * assign79090_body74_e120771_d_n4), (assign79090_body74_e120762 * assign79090_body74_e120771_d_n5), (assign79090_body74_e120762 * assign79090_body74_e120771_d_n6), (assign79090_body74_e120762 * assign79090_body74_e120771_d_n7), (assign79090_body74_e120762 * assign79090_body74_e120771_d_n8), (assign79090_body74_e120762 * assign79090_body74_e120771_d_n9), (assign79090_body74_e120762 * assign79090_body74_e120771_d_n10), (assign79090_body74_e120762 * assign79090_body74_e120771_d_n11), (assign79090_body74_e120762 * assign79090_body74_e120771_d_n14),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn14,)
    }
};
            locals.var_dplim = assign79090_body74_e120775;
            locals.var_dplim_dn0 = assign79090_body74_e120775_d_n0;
            locals.var_dplim_dn2 = assign79090_body74_e120775_d_n2;
            locals.var_dplim_dn4 = assign79090_body74_e120775_d_n4;
            locals.var_dplim_dn5 = assign79090_body74_e120775_d_n5;
            locals.var_dplim_dn6 = assign79090_body74_e120775_d_n6;
            locals.var_dplim_dn7 = assign79090_body74_e120775_d_n7;
            locals.var_dplim_dn8 = assign79090_body74_e120775_d_n8;
            locals.var_dplim_dn9 = assign79090_body74_e120775_d_n9;
            locals.var_dplim_dn10 = assign79090_body74_e120775_d_n10;
            locals.var_dplim_dn11 = assign79090_body74_e120775_d_n11;
            locals.var_dplim_dn14 = assign79090_body74_e120775_d_n14;
            let assign79090_body75_e120777: f64 = (locals.var_dps0).abs();
            let assign79090_body75_e120779: f64 = if assign79090_body75_e120777 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1844 = assign79090_body75_e120779;
            let (assign79090_body76_e120801, assign79090_body76_e120801_d_n0, assign79090_body76_e120801_d_n2, assign79090_body76_e120801_d_n4, assign79090_body76_e120801_d_n5, assign79090_body76_e120801_d_n6, assign79090_body76_e120801_d_n7, assign79090_body76_e120801_d_n8, assign79090_body76_e120801_d_n9, assign79090_body76_e120801_d_n10, assign79090_body76_e120801_d_n11, assign79090_body76_e120801_d_n14,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1843 == 0.0)) && (locals.var_guard1844 != 0.0)) {
        let (assign79090_body76_e120798,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign79090_body76_e120797: f64 = (-1.0);
                (assign79090_body76_e120797,)
            }
        };
        let assign79090_body76_e120799: f64 = (locals.var_dplim * assign79090_body76_e120798);
        (assign79090_body76_e120799, (locals.var_dplim_dn0 * assign79090_body76_e120798), (locals.var_dplim_dn2 * assign79090_body76_e120798), (locals.var_dplim_dn4 * assign79090_body76_e120798), (locals.var_dplim_dn5 * assign79090_body76_e120798), (locals.var_dplim_dn6 * assign79090_body76_e120798), (locals.var_dplim_dn7 * assign79090_body76_e120798), (locals.var_dplim_dn8 * assign79090_body76_e120798), (locals.var_dplim_dn9 * assign79090_body76_e120798), (locals.var_dplim_dn10 * assign79090_body76_e120798), (locals.var_dplim_dn11 * assign79090_body76_e120798), (locals.var_dplim_dn14 * assign79090_body76_e120798),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn4, locals.var_dps0_dn5, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn8, locals.var_dps0_dn9, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn14,)
    }
};
            locals.var_dps0 = assign79090_body76_e120801;
            locals.var_dps0_dn0 = assign79090_body76_e120801_d_n0;
            locals.var_dps0_dn2 = assign79090_body76_e120801_d_n2;
            locals.var_dps0_dn4 = assign79090_body76_e120801_d_n4;
            locals.var_dps0_dn5 = assign79090_body76_e120801_d_n5;
            locals.var_dps0_dn6 = assign79090_body76_e120801_d_n6;
            locals.var_dps0_dn7 = assign79090_body76_e120801_d_n7;
            locals.var_dps0_dn8 = assign79090_body76_e120801_d_n8;
            locals.var_dps0_dn9 = assign79090_body76_e120801_d_n9;
            locals.var_dps0_dn10 = assign79090_body76_e120801_d_n10;
            locals.var_dps0_dn11 = assign79090_body76_e120801_d_n11;
            locals.var_dps0_dn14 = assign79090_body76_e120801_d_n14;
            let (assign79090_body77_e120815, assign79090_body77_e120815_d_n0, assign79090_body77_e120815_d_n2, assign79090_body77_e120815_d_n4, assign79090_body77_e120815_d_n5, assign79090_body77_e120815_d_n6, assign79090_body77_e120815_d_n7, assign79090_body77_e120815_d_n8, assign79090_body77_e120815_d_n9, assign79090_body77_e120815_d_n10, assign79090_body77_e120815_d_n11, assign79090_body77_e120815_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1843 == 0.0)) {
        let assign79090_body77_e120813: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign79090_body77_e120813, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn4 + locals.var_dps0_dn4), (locals.var_ps0ld_dn5 + locals.var_dps0_dn5), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn8 + locals.var_dps0_dn8), (locals.var_ps0ld_dn9 + locals.var_dps0_dn9), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld_dn14 + locals.var_dps0_dn14),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
            locals.var_ps0ld = assign79090_body77_e120815;
            locals.var_ps0ld_dn0 = assign79090_body77_e120815_d_n0;
            locals.var_ps0ld_dn2 = assign79090_body77_e120815_d_n2;
            locals.var_ps0ld_dn4 = assign79090_body77_e120815_d_n4;
            locals.var_ps0ld_dn5 = assign79090_body77_e120815_d_n5;
            locals.var_ps0ld_dn6 = assign79090_body77_e120815_d_n6;
            locals.var_ps0ld_dn7 = assign79090_body77_e120815_d_n7;
            locals.var_ps0ld_dn8 = assign79090_body77_e120815_d_n8;
            locals.var_ps0ld_dn9 = assign79090_body77_e120815_d_n9;
            locals.var_ps0ld_dn10 = assign79090_body77_e120815_d_n10;
            locals.var_ps0ld_dn11 = assign79090_body77_e120815_d_n11;
            locals.var_ps0ld_dn14 = assign79090_body77_e120815_d_n14;
            let assign79090_body78_e120817: f64 = (locals.var_dps0).abs();
            let assign79090_body78_e120821: f64 = (locals.var_fs0).abs();
            let assign79090_body78_e120824: f64 = if ((assign79090_body78_e120817 <= 1e-12) && (assign79090_body78_e120821 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1845 = assign79090_body78_e120824;
            let (assign79090_body79_e120838,) = {
    if (((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) && (locals.var_guard1843 == 0.0)) && (locals.var_guard1845 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign79090_body79_e120838;
            let (assign79090_body80_e120849,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign79090_body80_e120847: f64 = (locals.var_lp_s0 + 1.0);
        (assign79090_body80_e120847,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign79090_body80_e120849;
        }

    }

    pub(super) fn stamp_transient_block_286(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign79110_e120863, assign79110_e120863_d_n0, assign79110_e120863_d_n2, assign79110_e120863_d_n4, assign79110_e120863_d_n5, assign79110_e120863_d_n6, assign79110_e120863_d_n7, assign79110_e120863_d_n8, assign79110_e120863_d_n9, assign79110_e120863_d_n10, assign79110_e120863_d_n11, assign79110_e120863_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign79110_e120861: f64 = (locals.var_c_w_ld * locals.var_fb);
        (assign79110_e120861, ((locals.var_c_w_ld_dn0 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn0)), ((locals.var_c_w_ld_dn2 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn2)), ((locals.var_c_w_ld_dn4 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn4)), ((locals.var_c_w_ld_dn5 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn5)), ((locals.var_c_w_ld_dn6 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn6)), ((locals.var_c_w_ld_dn7 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn7)), ((locals.var_c_w_ld_dn8 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn8)), ((locals.var_c_w_ld_dn9 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn9)), ((locals.var_c_w_ld_dn10 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn10)), ((locals.var_c_w_ld_dn11 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn11)), ((locals.var_c_w_ld_dn14 * locals.var_fb) + (locals.var_c_w_ld * locals.var_fb_dn14)),)
    } else {
        (locals.var_wdld__blk1764, locals.var_wdld__blk1764_dn0, locals.var_wdld__blk1764_dn2, locals.var_wdld__blk1764_dn4, locals.var_wdld__blk1764_dn5, locals.var_wdld__blk1764_dn6, locals.var_wdld__blk1764_dn7, locals.var_wdld__blk1764_dn8, locals.var_wdld__blk1764_dn9, locals.var_wdld__blk1764_dn10, locals.var_wdld__blk1764_dn11, locals.var_wdld__blk1764_dn14,)
    }
};
        locals.var_wdld__blk1764 = assign79110_e120863;
        locals.var_wdld__blk1764_dn0 = assign79110_e120863_d_n0;
        locals.var_wdld__blk1764_dn2 = assign79110_e120863_d_n2;
        locals.var_wdld__blk1764_dn4 = assign79110_e120863_d_n4;
        locals.var_wdld__blk1764_dn5 = assign79110_e120863_d_n5;
        locals.var_wdld__blk1764_dn6 = assign79110_e120863_d_n6;
        locals.var_wdld__blk1764_dn7 = assign79110_e120863_d_n7;
        locals.var_wdld__blk1764_dn8 = assign79110_e120863_d_n8;
        locals.var_wdld__blk1764_dn9 = assign79110_e120863_d_n9;
        locals.var_wdld__blk1764_dn10 = assign79110_e120863_d_n10;
        locals.var_wdld__blk1764_dn11 = assign79110_e120863_d_n11;
        locals.var_wdld__blk1764_dn14 = assign79110_e120863_d_n14;

        let (assign79120_e120874, assign79120_e120874_d_n0, assign79120_e120874_d_n2, assign79120_e120874_d_n4, assign79120_e120874_d_n5, assign79120_e120874_d_n6, assign79120_e120874_d_n7, assign79120_e120874_d_n8, assign79120_e120874_d_n9, assign79120_e120874_d_n10, assign79120_e120874_d_n11, assign79120_e120874_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign79120_e120872: f64 = (locals.var_q_nsubld__blk1766 * locals.var_wdld__blk1764);
        (assign79120_e120872, (locals.var_q_nsubld__blk1766 * locals.var_wdld__blk1764_dn0), (locals.var_q_nsubld__blk1766 * locals.var_wdld__blk1764_dn2), (locals.var_q_nsubld__blk1766 * locals.var_wdld__blk1764_dn4), (locals.var_q_nsubld__blk1766 * locals.var_wdld__blk1764_dn5), (locals.var_q_nsubld__blk1766 * locals.var_wdld__blk1764_dn6), (locals.var_q_nsubld__blk1766 * locals.var_wdld__blk1764_dn7), (locals.var_q_nsubld__blk1766 * locals.var_wdld__blk1764_dn8), (locals.var_q_nsubld__blk1766 * locals.var_wdld__blk1764_dn9), (locals.var_q_nsubld__blk1766 * locals.var_wdld__blk1764_dn10), (locals.var_q_nsubld__blk1766 * locals.var_wdld__blk1764_dn11), (locals.var_q_nsubld__blk1766 * locals.var_wdld__blk1764_dn14),)
    } else {
        (locals.var_q_dep_ld__blk1765, locals.var_q_dep_ld__blk1765_dn0, locals.var_q_dep_ld__blk1765_dn2, locals.var_q_dep_ld__blk1765_dn4, locals.var_q_dep_ld__blk1765_dn5, locals.var_q_dep_ld__blk1765_dn6, locals.var_q_dep_ld__blk1765_dn7, locals.var_q_dep_ld__blk1765_dn8, locals.var_q_dep_ld__blk1765_dn9, locals.var_q_dep_ld__blk1765_dn10, locals.var_q_dep_ld__blk1765_dn11, locals.var_q_dep_ld__blk1765_dn14,)
    }
};
        locals.var_q_dep_ld__blk1765 = assign79120_e120874;
        locals.var_q_dep_ld__blk1765_dn0 = assign79120_e120874_d_n0;
        locals.var_q_dep_ld__blk1765_dn2 = assign79120_e120874_d_n2;
        locals.var_q_dep_ld__blk1765_dn4 = assign79120_e120874_d_n4;
        locals.var_q_dep_ld__blk1765_dn5 = assign79120_e120874_d_n5;
        locals.var_q_dep_ld__blk1765_dn6 = assign79120_e120874_d_n6;
        locals.var_q_dep_ld__blk1765_dn7 = assign79120_e120874_d_n7;
        locals.var_q_dep_ld__blk1765_dn8 = assign79120_e120874_d_n8;
        locals.var_q_dep_ld__blk1765_dn9 = assign79120_e120874_d_n9;
        locals.var_q_dep_ld__blk1765_dn10 = assign79120_e120874_d_n10;
        locals.var_q_dep_ld__blk1765_dn11 = assign79120_e120874_d_n11;
        locals.var_q_dep_ld__blk1765_dn14 = assign79120_e120874_d_n14;

        let (assign79130_e120889, assign79130_e120889_d_n0, assign79130_e120889_d_n2, assign79130_e120889_d_n4, assign79130_e120889_d_n5, assign79130_e120889_d_n6, assign79130_e120889_d_n7, assign79130_e120889_d_n8, assign79130_e120889_d_n9, assign79130_e120889_d_n10, assign79130_e120889_d_n11, assign79130_e120889_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign79130_e120883: f64 = (locals.var_q_dep_ld__blk1765 / locals.var_cnst0over_func);
        let assign79130_e120886: f64 = (10.0 * 2.220446049250313e-16);
        let assign79130_e120887: f64 = (assign79130_e120883 + assign79130_e120886);
        (assign79130_e120887, (((locals.var_q_dep_ld__blk1765_dn0 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1765 * locals.var_cnst0over_func_dn0)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1765_dn2 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1765 * locals.var_cnst0over_func_dn2)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1765_dn4 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1765 * locals.var_cnst0over_func_dn4)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1765_dn5 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1765 * locals.var_cnst0over_func_dn5)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1765_dn6 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1765 * locals.var_cnst0over_func_dn6)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1765_dn7 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1765 * locals.var_cnst0over_func_dn7)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1765_dn8 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1765 * locals.var_cnst0over_func_dn8)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1765_dn9 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1765 * locals.var_cnst0over_func_dn9)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1765_dn10 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1765 * locals.var_cnst0over_func_dn10)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1765_dn11 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1765 * locals.var_cnst0over_func_dn11)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)), (((locals.var_q_dep_ld__blk1765_dn14 * locals.var_cnst0over_func) - (locals.var_q_dep_ld__blk1765 * locals.var_cnst0over_func_dn14)) / (locals.var_cnst0over_func * locals.var_cnst0over_func)),)
    } else {
        (locals.var_xi0p12, locals.var_xi0p12_dn0, locals.var_xi0p12_dn2, locals.var_xi0p12_dn4, locals.var_xi0p12_dn5, locals.var_xi0p12_dn6, locals.var_xi0p12_dn7, locals.var_xi0p12_dn8, locals.var_xi0p12_dn9, locals.var_xi0p12_dn10, locals.var_xi0p12_dn11, locals.var_xi0p12_dn14,)
    }
};
        locals.var_xi0p12 = assign79130_e120889;
        locals.var_xi0p12_dn0 = assign79130_e120889_d_n0;
        locals.var_xi0p12_dn2 = assign79130_e120889_d_n2;
        locals.var_xi0p12_dn4 = assign79130_e120889_d_n4;
        locals.var_xi0p12_dn5 = assign79130_e120889_d_n5;
        locals.var_xi0p12_dn6 = assign79130_e120889_d_n6;
        locals.var_xi0p12_dn7 = assign79130_e120889_d_n7;
        locals.var_xi0p12_dn8 = assign79130_e120889_d_n8;
        locals.var_xi0p12_dn9 = assign79130_e120889_d_n9;
        locals.var_xi0p12_dn10 = assign79130_e120889_d_n10;
        locals.var_xi0p12_dn11 = assign79130_e120889_d_n11;
        locals.var_xi0p12_dn14 = assign79130_e120889_d_n14;

        let (assign79140_e120900, assign79140_e120900_d_n0, assign79140_e120900_d_n2, assign79140_e120900_d_n4, assign79140_e120900_d_n5, assign79140_e120900_d_n6, assign79140_e120900_d_n7, assign79140_e120900_d_n8, assign79140_e120900_d_n9, assign79140_e120900_d_n10, assign79140_e120900_d_n11, assign79140_e120900_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign79140_e120898: f64 = (locals.var_cnst0over_func * locals.var_xi0p12);
        (assign79140_e120898, ((locals.var_cnst0over_func_dn0 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn0)), ((locals.var_cnst0over_func_dn2 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn2)), ((locals.var_cnst0over_func_dn4 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn4)), ((locals.var_cnst0over_func_dn5 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn5)), ((locals.var_cnst0over_func_dn6 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn6)), ((locals.var_cnst0over_func_dn7 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn7)), ((locals.var_cnst0over_func_dn8 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn8)), ((locals.var_cnst0over_func_dn9 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn9)), ((locals.var_cnst0over_func_dn10 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn10)), ((locals.var_cnst0over_func_dn11 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn11)), ((locals.var_cnst0over_func_dn14 * locals.var_xi0p12) + (locals.var_cnst0over_func * locals.var_xi0p12_dn14)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn4, locals.var_qbuld_dn5, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn8, locals.var_qbuld_dn9, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn14,)
    }
};
        locals.var_qbuld = assign79140_e120900;
        locals.var_qbuld_dn0 = assign79140_e120900_d_n0;
        locals.var_qbuld_dn2 = assign79140_e120900_d_n2;
        locals.var_qbuld_dn4 = assign79140_e120900_d_n4;
        locals.var_qbuld_dn5 = assign79140_e120900_d_n5;
        locals.var_qbuld_dn6 = assign79140_e120900_d_n6;
        locals.var_qbuld_dn7 = assign79140_e120900_d_n7;
        locals.var_qbuld_dn8 = assign79140_e120900_d_n8;
        locals.var_qbuld_dn9 = assign79140_e120900_d_n9;
        locals.var_qbuld_dn10 = assign79140_e120900_d_n10;
        locals.var_qbuld_dn11 = assign79140_e120900_d_n11;
        locals.var_qbuld_dn14 = assign79140_e120900_d_n14;

        let (assign79150_e120913, assign79150_e120913_d_n0, assign79150_e120913_d_n2, assign79150_e120913_d_n4, assign79150_e120913_d_n5, assign79150_e120913_d_n6, assign79150_e120913_d_n7, assign79150_e120913_d_n8, assign79150_e120913_d_n9, assign79150_e120913_d_n10, assign79150_e120913_d_n11, assign79150_e120913_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign79150_e120910: f64 = (locals.var_fs02 + locals.var_xi0p12);
        let assign79150_e120911: f64 = (1.0 / assign79150_e120910);
        (assign79150_e120911, (-((locals.var_fs02_dn0 + locals.var_xi0p12_dn0) / (assign79150_e120910 * assign79150_e120910))), (-((locals.var_fs02_dn2 + locals.var_xi0p12_dn2) / (assign79150_e120910 * assign79150_e120910))), (-((locals.var_fs02_dn4 + locals.var_xi0p12_dn4) / (assign79150_e120910 * assign79150_e120910))), (-((locals.var_fs02_dn5 + locals.var_xi0p12_dn5) / (assign79150_e120910 * assign79150_e120910))), (-((locals.var_fs02_dn6 + locals.var_xi0p12_dn6) / (assign79150_e120910 * assign79150_e120910))), (-((locals.var_fs02_dn7 + locals.var_xi0p12_dn7) / (assign79150_e120910 * assign79150_e120910))), (-((locals.var_fs02_dn8 + locals.var_xi0p12_dn8) / (assign79150_e120910 * assign79150_e120910))), (-((locals.var_fs02_dn9 + locals.var_xi0p12_dn9) / (assign79150_e120910 * assign79150_e120910))), (-((locals.var_fs02_dn10 + locals.var_xi0p12_dn10) / (assign79150_e120910 * assign79150_e120910))), (-((locals.var_fs02_dn11 + locals.var_xi0p12_dn11) / (assign79150_e120910 * assign79150_e120910))), (-((locals.var_fs02_dn14 + locals.var_xi0p12_dn14) / (assign79150_e120910 * assign79150_e120910))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign79150_e120913;
        locals.var_t1_dn0 = assign79150_e120913_d_n0;
        locals.var_t1_dn2 = assign79150_e120913_d_n2;
        locals.var_t1_dn4 = assign79150_e120913_d_n4;
        locals.var_t1_dn5 = assign79150_e120913_d_n5;
        locals.var_t1_dn6 = assign79150_e120913_d_n6;
        locals.var_t1_dn7 = assign79150_e120913_d_n7;
        locals.var_t1_dn8 = assign79150_e120913_d_n8;
        locals.var_t1_dn9 = assign79150_e120913_d_n9;
        locals.var_t1_dn10 = assign79150_e120913_d_n10;
        locals.var_t1_dn11 = assign79150_e120913_d_n11;
        locals.var_t1_dn14 = assign79150_e120913_d_n14;

        let (assign79160_e120926, assign79160_e120926_d_n0, assign79160_e120926_d_n2, assign79160_e120926_d_n4, assign79160_e120926_d_n5, assign79160_e120926_d_n6, assign79160_e120926_d_n7, assign79160_e120926_d_n8, assign79160_e120926_d_n9, assign79160_e120926_d_n10, assign79160_e120926_d_n11, assign79160_e120926_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign79160_e120922: f64 = (locals.var_cnst0over_func * locals.var_fs01);
        let assign79160_e120924: f64 = (assign79160_e120922 * locals.var_t1);
        (assign79160_e120924, ((((locals.var_cnst0over_func_dn0 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn0)) * locals.var_t1) + (assign79160_e120922 * locals.var_t1_dn0)), ((((locals.var_cnst0over_func_dn2 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn2)) * locals.var_t1) + (assign79160_e120922 * locals.var_t1_dn2)), ((((locals.var_cnst0over_func_dn4 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn4)) * locals.var_t1) + (assign79160_e120922 * locals.var_t1_dn4)), ((((locals.var_cnst0over_func_dn5 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn5)) * locals.var_t1) + (assign79160_e120922 * locals.var_t1_dn5)), ((((locals.var_cnst0over_func_dn6 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn6)) * locals.var_t1) + (assign79160_e120922 * locals.var_t1_dn6)), ((((locals.var_cnst0over_func_dn7 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn7)) * locals.var_t1) + (assign79160_e120922 * locals.var_t1_dn7)), ((((locals.var_cnst0over_func_dn8 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn8)) * locals.var_t1) + (assign79160_e120922 * locals.var_t1_dn8)), ((((locals.var_cnst0over_func_dn9 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn9)) * locals.var_t1) + (assign79160_e120922 * locals.var_t1_dn9)), ((((locals.var_cnst0over_func_dn10 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn10)) * locals.var_t1) + (assign79160_e120922 * locals.var_t1_dn10)), ((((locals.var_cnst0over_func_dn11 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn11)) * locals.var_t1) + (assign79160_e120922 * locals.var_t1_dn11)), ((((locals.var_cnst0over_func_dn14 * locals.var_fs01) + (locals.var_cnst0over_func * locals.var_fs01_dn14)) * locals.var_t1) + (assign79160_e120922 * locals.var_t1_dn14)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn4, locals.var_qiuld_dn5, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn8, locals.var_qiuld_dn9, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn14,)
    }
};
        locals.var_qiuld = assign79160_e120926;
        locals.var_qiuld_dn0 = assign79160_e120926_d_n0;
        locals.var_qiuld_dn2 = assign79160_e120926_d_n2;
        locals.var_qiuld_dn4 = assign79160_e120926_d_n4;
        locals.var_qiuld_dn5 = assign79160_e120926_d_n5;
        locals.var_qiuld_dn6 = assign79160_e120926_d_n6;
        locals.var_qiuld_dn7 = assign79160_e120926_d_n7;
        locals.var_qiuld_dn8 = assign79160_e120926_d_n8;
        locals.var_qiuld_dn9 = assign79160_e120926_d_n9;
        locals.var_qiuld_dn10 = assign79160_e120926_d_n10;
        locals.var_qiuld_dn11 = assign79160_e120926_d_n11;
        locals.var_qiuld_dn14 = assign79160_e120926_d_n14;

        let (assign79170_e120937, assign79170_e120937_d_n0, assign79170_e120937_d_n2, assign79170_e120937_d_n4, assign79170_e120937_d_n5, assign79170_e120937_d_n6, assign79170_e120937_d_n7, assign79170_e120937_d_n8, assign79170_e120937_d_n9, assign79170_e120937_d_n10, assign79170_e120937_d_n11, assign79170_e120937_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1790 == 0.0)) && (locals.var_guard1829 != 0.0)) {
        let assign79170_e120935: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign79170_e120935, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn4 + locals.var_qiuld_dn4), (locals.var_qbuld_dn5 + locals.var_qiuld_dn5), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn8 + locals.var_qiuld_dn8), (locals.var_qbuld_dn9 + locals.var_qiuld_dn9), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn14 + locals.var_qiuld_dn14),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn4, locals.var_qsuld_dn5, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn8, locals.var_qsuld_dn9, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn14,)
    }
};
        locals.var_qsuld = assign79170_e120937;
        locals.var_qsuld_dn0 = assign79170_e120937_d_n0;
        locals.var_qsuld_dn2 = assign79170_e120937_d_n2;
        locals.var_qsuld_dn4 = assign79170_e120937_d_n4;
        locals.var_qsuld_dn5 = assign79170_e120937_d_n5;
        locals.var_qsuld_dn6 = assign79170_e120937_d_n6;
        locals.var_qsuld_dn7 = assign79170_e120937_d_n7;
        locals.var_qsuld_dn8 = assign79170_e120937_d_n8;
        locals.var_qsuld_dn9 = assign79170_e120937_d_n9;
        locals.var_qsuld_dn10 = assign79170_e120937_d_n10;
        locals.var_qsuld_dn11 = assign79170_e120937_d_n11;
        locals.var_qsuld_dn14 = assign79170_e120937_d_n14;

        let assign79180_e120940: f64 = if p.p33 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1847 = assign79180_e120940;

        let (assign79190_e120950, assign79190_e120950_d_n0, assign79190_e120950_d_n2, assign79190_e120950_d_n4, assign79190_e120950_d_n5, assign79190_e120950_d_n6, assign79190_e120950_d_n7, assign79190_e120950_d_n8, assign79190_e120950_d_n9, assign79190_e120950_d_n10, assign79190_e120950_d_n11, assign79190_e120950_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) {
        let assign79190_e120946: f64 = (-locals.var_vxbgmtcl);
        let assign79190_e120947: f64 = (locals.var_beta * assign79190_e120946);
        let assign79190_e120948: f64 = (assign79190_e120947).exp();
        (assign79190_e120948, (assign79190_e120948 * ((locals.var_beta_dn0 * assign79190_e120946) + (locals.var_beta * (-locals.var_vxbgmtcl_dn0)))), (assign79190_e120948 * ((locals.var_beta_dn2 * assign79190_e120946) + (locals.var_beta * (-locals.var_vxbgmtcl_dn2)))), (assign79190_e120948 * ((locals.var_beta_dn4 * assign79190_e120946) + (locals.var_beta * (-locals.var_vxbgmtcl_dn4)))), (assign79190_e120948 * ((locals.var_beta_dn5 * assign79190_e120946) + (locals.var_beta * (-locals.var_vxbgmtcl_dn5)))), (assign79190_e120948 * ((locals.var_beta_dn6 * assign79190_e120946) + (locals.var_beta * (-locals.var_vxbgmtcl_dn6)))), (assign79190_e120948 * ((locals.var_beta_dn7 * assign79190_e120946) + (locals.var_beta * (-locals.var_vxbgmtcl_dn7)))), (assign79190_e120948 * ((locals.var_beta_dn8 * assign79190_e120946) + (locals.var_beta * (-locals.var_vxbgmtcl_dn8)))), (assign79190_e120948 * ((locals.var_beta_dn9 * assign79190_e120946) + (locals.var_beta * (-locals.var_vxbgmtcl_dn9)))), (assign79190_e120948 * ((locals.var_beta_dn10 * assign79190_e120946) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign79190_e120948 * ((locals.var_beta_dn11 * assign79190_e120946) + (locals.var_beta * (-locals.var_vxbgmtcl_dn11)))), (assign79190_e120948 * ((locals.var_beta_dn14 * assign79190_e120946) + (locals.var_beta * (-locals.var_vxbgmtcl_dn14)))),)
    } else {
        (locals.var_exp_bvbs, locals.var_exp_bvbs_dn0, locals.var_exp_bvbs_dn2, locals.var_exp_bvbs_dn4, locals.var_exp_bvbs_dn5, locals.var_exp_bvbs_dn6, locals.var_exp_bvbs_dn7, locals.var_exp_bvbs_dn8, locals.var_exp_bvbs_dn9, locals.var_exp_bvbs_dn10, locals.var_exp_bvbs_dn11, locals.var_exp_bvbs_dn14,)
    }
};
        locals.var_exp_bvbs = assign79190_e120950;
        locals.var_exp_bvbs_dn0 = assign79190_e120950_d_n0;
        locals.var_exp_bvbs_dn2 = assign79190_e120950_d_n2;
        locals.var_exp_bvbs_dn4 = assign79190_e120950_d_n4;
        locals.var_exp_bvbs_dn5 = assign79190_e120950_d_n5;
        locals.var_exp_bvbs_dn6 = assign79190_e120950_d_n6;
        locals.var_exp_bvbs_dn7 = assign79190_e120950_d_n7;
        locals.var_exp_bvbs_dn8 = assign79190_e120950_d_n8;
        locals.var_exp_bvbs_dn9 = assign79190_e120950_d_n9;
        locals.var_exp_bvbs_dn10 = assign79190_e120950_d_n10;
        locals.var_exp_bvbs_dn11 = assign79190_e120950_d_n11;
        locals.var_exp_bvbs_dn14 = assign79190_e120950_d_n14;

        let (assign79200_e120958, assign79200_e120958_d_n0, assign79200_e120958_d_n2, assign79200_e120958_d_n4, assign79200_e120958_d_n5, assign79200_e120958_d_n6, assign79200_e120958_d_n7, assign79200_e120958_d_n8, assign79200_e120958_d_n9, assign79200_e120958_d_n10, assign79200_e120958_d_n11, assign79200_e120958_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) {
        let assign79200_e120956: f64 = (locals.var_nin / locals.var_nover_func);
        (assign79200_e120956, (locals.var_nin_dn0 / locals.var_nover_func), (locals.var_nin_dn2 / locals.var_nover_func), (locals.var_nin_dn4 / locals.var_nover_func), (locals.var_nin_dn5 / locals.var_nover_func), (locals.var_nin_dn6 / locals.var_nover_func), (locals.var_nin_dn7 / locals.var_nover_func), (locals.var_nin_dn8 / locals.var_nover_func), (locals.var_nin_dn9 / locals.var_nover_func), (locals.var_nin_dn10 / locals.var_nover_func), (locals.var_nin_dn11 / locals.var_nover_func), (locals.var_nin_dn14 / locals.var_nover_func),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign79200_e120958;
        locals.var_t0_dn0 = assign79200_e120958_d_n0;
        locals.var_t0_dn2 = assign79200_e120958_d_n2;
        locals.var_t0_dn4 = assign79200_e120958_d_n4;
        locals.var_t0_dn5 = assign79200_e120958_d_n5;
        locals.var_t0_dn6 = assign79200_e120958_d_n6;
        locals.var_t0_dn7 = assign79200_e120958_d_n7;
        locals.var_t0_dn8 = assign79200_e120958_d_n8;
        locals.var_t0_dn9 = assign79200_e120958_d_n9;
        locals.var_t0_dn10 = assign79200_e120958_d_n10;
        locals.var_t0_dn11 = assign79200_e120958_d_n11;
        locals.var_t0_dn14 = assign79200_e120958_d_n14;

        let (assign79210_e120966, assign79210_e120966_d_n0, assign79210_e120966_d_n2, assign79210_e120966_d_n4, assign79210_e120966_d_n5, assign79210_e120966_d_n6, assign79210_e120966_d_n7, assign79210_e120966_d_n8, assign79210_e120966_d_n9, assign79210_e120966_d_n10, assign79210_e120966_d_n11, assign79210_e120966_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) {
        let assign79210_e120964: f64 = (locals.var_t0 * locals.var_t0);
        (assign79210_e120964, ((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)), ((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)), ((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)), ((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn4, locals.var_cnst1over_dn5, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn8, locals.var_cnst1over_dn9, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn14,)
    }
};
        locals.var_cnst1over = assign79210_e120966;
        locals.var_cnst1over_dn0 = assign79210_e120966_d_n0;
        locals.var_cnst1over_dn2 = assign79210_e120966_d_n2;
        locals.var_cnst1over_dn4 = assign79210_e120966_d_n4;
        locals.var_cnst1over_dn5 = assign79210_e120966_d_n5;
        locals.var_cnst1over_dn6 = assign79210_e120966_d_n6;
        locals.var_cnst1over_dn7 = assign79210_e120966_d_n7;
        locals.var_cnst1over_dn8 = assign79210_e120966_d_n8;
        locals.var_cnst1over_dn9 = assign79210_e120966_d_n9;
        locals.var_cnst1over_dn10 = assign79210_e120966_d_n10;
        locals.var_cnst1over_dn11 = assign79210_e120966_d_n11;
        locals.var_cnst1over_dn14 = assign79210_e120966_d_n14;

        let (assign79220_e120974, assign79220_e120974_d_n0, assign79220_e120974_d_n2, assign79220_e120974_d_n4, assign79220_e120974_d_n5, assign79220_e120974_d_n6, assign79220_e120974_d_n7, assign79220_e120974_d_n8, assign79220_e120974_d_n9, assign79220_e120974_d_n10, assign79220_e120974_d_n11, assign79220_e120974_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) {
        let assign79220_e120972: f64 = (locals.var_cnst1over * locals.var_exp_bvbs);
        (assign79220_e120972, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn2)), ((locals.var_cnst1over_dn4 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn4)), ((locals.var_cnst1over_dn5 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn5)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn7)), ((locals.var_cnst1over_dn8 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn8)), ((locals.var_cnst1over_dn9 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn9)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn11)), ((locals.var_cnst1over_dn14 * locals.var_exp_bvbs) + (locals.var_cnst1over * locals.var_exp_bvbs_dn14)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn14,)
    }
};
        locals.var_cfs1 = assign79220_e120974;
        locals.var_cfs1_dn0 = assign79220_e120974_d_n0;
        locals.var_cfs1_dn2 = assign79220_e120974_d_n2;
        locals.var_cfs1_dn4 = assign79220_e120974_d_n4;
        locals.var_cfs1_dn5 = assign79220_e120974_d_n5;
        locals.var_cfs1_dn6 = assign79220_e120974_d_n6;
        locals.var_cfs1_dn7 = assign79220_e120974_d_n7;
        locals.var_cfs1_dn8 = assign79220_e120974_d_n8;
        locals.var_cfs1_dn9 = assign79220_e120974_d_n9;
        locals.var_cfs1_dn10 = assign79220_e120974_d_n10;
        locals.var_cfs1_dn11 = assign79220_e120974_d_n11;
        locals.var_cfs1_dn14 = assign79220_e120974_d_n14;

        let (assign79230_e120980, assign79230_e120980_d_n0, assign79230_e120980_d_n2, assign79230_e120980_d_n4, assign79230_e120980_d_n5, assign79230_e120980_d_n6, assign79230_e120980_d_n7, assign79230_e120980_d_n8, assign79230_e120980_d_n9, assign79230_e120980_d_n10, assign79230_e120980_d_n11, assign79230_e120980_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) {
        (locals.var_ps0ld_ini__blk1773, locals.var_ps0ld_ini__blk1773_dn0, locals.var_ps0ld_ini__blk1773_dn2, locals.var_ps0ld_ini__blk1773_dn4, locals.var_ps0ld_ini__blk1773_dn5, locals.var_ps0ld_ini__blk1773_dn6, locals.var_ps0ld_ini__blk1773_dn7, locals.var_ps0ld_ini__blk1773_dn8, locals.var_ps0ld_ini__blk1773_dn9, locals.var_ps0ld_ini__blk1773_dn10, locals.var_ps0ld_ini__blk1773_dn11, locals.var_ps0ld_ini__blk1773_dn14,)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn4, locals.var_ps0ld_dn5, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn8, locals.var_ps0ld_dn9, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn14,)
    }
};
        locals.var_ps0ld = assign79230_e120980;
        locals.var_ps0ld_dn0 = assign79230_e120980_d_n0;
        locals.var_ps0ld_dn2 = assign79230_e120980_d_n2;
        locals.var_ps0ld_dn4 = assign79230_e120980_d_n4;
        locals.var_ps0ld_dn5 = assign79230_e120980_d_n5;
        locals.var_ps0ld_dn6 = assign79230_e120980_d_n6;
        locals.var_ps0ld_dn7 = assign79230_e120980_d_n7;
        locals.var_ps0ld_dn8 = assign79230_e120980_d_n8;
        locals.var_ps0ld_dn9 = assign79230_e120980_d_n9;
        locals.var_ps0ld_dn10 = assign79230_e120980_d_n10;
        locals.var_ps0ld_dn11 = assign79230_e120980_d_n11;
        locals.var_ps0ld_dn14 = assign79230_e120980_d_n14;

        let (assign79240_e120986,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign79240_e120986;

        let (assign79250_e120999, assign79250_e120999_d_n0, assign79250_e120999_d_n2, assign79250_e120999_d_n4, assign79250_e120999_d_n5, assign79250_e120999_d_n6, assign79250_e120999_d_n7, assign79250_e120999_d_n8, assign79250_e120999_d_n9, assign79250_e120999_d_n10, assign79250_e120999_d_n11, assign79250_e120999_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) {
        let assign79250_e120993: f64 = (1.034943e-10 / locals.var_q_nsubld__blk1766);
        let assign79250_e120995: f64 = (assign79250_e120993 * locals.var_beta_inv);
        let assign79250_e120996: f64 = (2.0 * assign79250_e120995);
        let assign79250_e120997: f64 = (assign79250_e120996).sqrt();
        (assign79250_e120997, ((2.0 * (assign79250_e120993 * locals.var_beta_inv_dn0)) / (2.0 * assign79250_e120997)), ((2.0 * (assign79250_e120993 * locals.var_beta_inv_dn2)) / (2.0 * assign79250_e120997)), ((2.0 * (assign79250_e120993 * locals.var_beta_inv_dn4)) / (2.0 * assign79250_e120997)), ((2.0 * (assign79250_e120993 * locals.var_beta_inv_dn5)) / (2.0 * assign79250_e120997)), ((2.0 * (assign79250_e120993 * locals.var_beta_inv_dn6)) / (2.0 * assign79250_e120997)), ((2.0 * (assign79250_e120993 * locals.var_beta_inv_dn7)) / (2.0 * assign79250_e120997)), ((2.0 * (assign79250_e120993 * locals.var_beta_inv_dn8)) / (2.0 * assign79250_e120997)), ((2.0 * (assign79250_e120993 * locals.var_beta_inv_dn9)) / (2.0 * assign79250_e120997)), ((2.0 * (assign79250_e120993 * locals.var_beta_inv_dn10)) / (2.0 * assign79250_e120997)), ((2.0 * (assign79250_e120993 * locals.var_beta_inv_dn11)) / (2.0 * assign79250_e120997)), ((2.0 * (assign79250_e120993 * locals.var_beta_inv_dn14)) / (2.0 * assign79250_e120997)),)
    } else {
        (locals.var_c_w_ld, locals.var_c_w_ld_dn0, locals.var_c_w_ld_dn2, locals.var_c_w_ld_dn4, locals.var_c_w_ld_dn5, locals.var_c_w_ld_dn6, locals.var_c_w_ld_dn7, locals.var_c_w_ld_dn8, locals.var_c_w_ld_dn9, locals.var_c_w_ld_dn10, locals.var_c_w_ld_dn11, locals.var_c_w_ld_dn14,)
    }
};
        locals.var_c_w_ld = assign79250_e120999;
        locals.var_c_w_ld_dn0 = assign79250_e120999_d_n0;
        locals.var_c_w_ld_dn2 = assign79250_e120999_d_n2;
        locals.var_c_w_ld_dn4 = assign79250_e120999_d_n4;
        locals.var_c_w_ld_dn5 = assign79250_e120999_d_n5;
        locals.var_c_w_ld_dn6 = assign79250_e120999_d_n6;
        locals.var_c_w_ld_dn7 = assign79250_e120999_d_n7;
        locals.var_c_w_ld_dn8 = assign79250_e120999_d_n8;
        locals.var_c_w_ld_dn9 = assign79250_e120999_d_n9;
        locals.var_c_w_ld_dn10 = assign79250_e120999_d_n10;
        locals.var_c_w_ld_dn11 = assign79250_e120999_d_n11;
        locals.var_c_w_ld_dn14 = assign79250_e120999_d_n14;

        let assign79260_e121002: f64 = if locals.var_wdep_func > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1848 = assign79260_e121002;

        let (assign79270_e121012, assign79270_e121012_d_n0, assign79270_e121012_d_n2, assign79270_e121012_d_n4, assign79270_e121012_d_n5, assign79270_e121012_d_n6, assign79270_e121012_d_n7, assign79270_e121012_d_n8, assign79270_e121012_d_n9, assign79270_e121012_d_n10, assign79270_e121012_d_n11, assign79270_e121012_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) && (locals.var_guard1848 != 0.0)) {
        let assign79270_e121010: f64 = (p.p334 - locals.var_wdep_func);
        (assign79270_e121010, (-locals.var_wdep_func_dn0), (-locals.var_wdep_func_dn2), (-locals.var_wdep_func_dn4), (-locals.var_wdep_func_dn5), (-locals.var_wdep_func_dn6), (-locals.var_wdep_func_dn7), (-locals.var_wdep_func_dn8), (-locals.var_wdep_func_dn9), (-locals.var_wdep_func_dn10), (-locals.var_wdep_func_dn11), (-locals.var_wdep_func_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign79270_e121012;
        locals.var_t2_dn0 = assign79270_e121012_d_n0;
        locals.var_t2_dn2 = assign79270_e121012_d_n2;
        locals.var_t2_dn4 = assign79270_e121012_d_n4;
        locals.var_t2_dn5 = assign79270_e121012_d_n5;
        locals.var_t2_dn6 = assign79270_e121012_d_n6;
        locals.var_t2_dn7 = assign79270_e121012_d_n7;
        locals.var_t2_dn8 = assign79270_e121012_d_n8;
        locals.var_t2_dn9 = assign79270_e121012_d_n9;
        locals.var_t2_dn10 = assign79270_e121012_d_n10;
        locals.var_t2_dn11 = assign79270_e121012_d_n11;
        locals.var_t2_dn14 = assign79270_e121012_d_n14;

        let (assign79280_e121034, assign79280_e121034_d_n0, assign79280_e121034_d_n2, assign79280_e121034_d_n4, assign79280_e121034_d_n5, assign79280_e121034_d_n6, assign79280_e121034_d_n7, assign79280_e121034_d_n8, assign79280_e121034_d_n9, assign79280_e121034_d_n10, assign79280_e121034_d_n11, assign79280_e121034_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) && (locals.var_guard1848 == 0.0)) {
        let assign79280_e121021: f64 = (locals.var_vdsi + p.p137);
        let assign79280_e121024: f64 = (locals.var_vdsi + p.p137);
        let assign79280_e121025: f64 = (assign79280_e121021 * assign79280_e121024);
        let assign79280_e121028: f64 = (4.0 * 0.1);
        let assign79280_e121030: f64 = (assign79280_e121028 * 0.1);
        let assign79280_e121031: f64 = (assign79280_e121025 + assign79280_e121030);
        let assign79280_e121032: f64 = (assign79280_e121031).sqrt();
        (assign79280_e121032, 0.0, 0.0, 0.0, 0.0, (((locals.var_vdsi_dn6 * assign79280_e121024) + (assign79280_e121021 * locals.var_vdsi_dn6)) / (2.0 * assign79280_e121032)), 0.0, (((locals.var_vdsi_dn8 * assign79280_e121024) + (assign79280_e121021 * locals.var_vdsi_dn8)) / (2.0 * assign79280_e121032)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign79280_e121034;
        locals.var_tmf2_dn0 = assign79280_e121034_d_n0;
        locals.var_tmf2_dn2 = assign79280_e121034_d_n2;
        locals.var_tmf2_dn4 = assign79280_e121034_d_n4;
        locals.var_tmf2_dn5 = assign79280_e121034_d_n5;
        locals.var_tmf2_dn6 = assign79280_e121034_d_n6;
        locals.var_tmf2_dn7 = assign79280_e121034_d_n7;
        locals.var_tmf2_dn8 = assign79280_e121034_d_n8;
        locals.var_tmf2_dn9 = assign79280_e121034_d_n9;
        locals.var_tmf2_dn10 = assign79280_e121034_d_n10;
        locals.var_tmf2_dn11 = assign79280_e121034_d_n11;
        locals.var_tmf2_dn14 = assign79280_e121034_d_n14;

        let (assign79290_e121051, assign79290_e121051_d_n0, assign79290_e121051_d_n2, assign79290_e121051_d_n4, assign79290_e121051_d_n5, assign79290_e121051_d_n6, assign79290_e121051_d_n7, assign79290_e121051_d_n8, assign79290_e121051_d_n9, assign79290_e121051_d_n10, assign79290_e121051_d_n11, assign79290_e121051_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) && (locals.var_guard1848 == 0.0)) {
        let assign79290_e121045: f64 = (locals.var_vdsi + p.p137);
        let assign79290_e121047: f64 = (assign79290_e121045 / locals.var_tmf2);
        let assign79290_e121048: f64 = (1.0 + assign79290_e121047);
        let assign79290_e121049: f64 = (0.5 * assign79290_e121048);
        (assign79290_e121049, (0.5 * (-((assign79290_e121045 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign79290_e121045 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign79290_e121045 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign79290_e121045 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn6 * locals.var_tmf2) - (assign79290_e121045 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign79290_e121045 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (((locals.var_vdsi_dn8 * locals.var_tmf2) - (assign79290_e121045 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (-((assign79290_e121045 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign79290_e121045 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign79290_e121045 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((assign79290_e121045 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign79290_e121051;
        locals.var_t9_dn0 = assign79290_e121051_d_n0;
        locals.var_t9_dn2 = assign79290_e121051_d_n2;
        locals.var_t9_dn4 = assign79290_e121051_d_n4;
        locals.var_t9_dn5 = assign79290_e121051_d_n5;
        locals.var_t9_dn6 = assign79290_e121051_d_n6;
        locals.var_t9_dn7 = assign79290_e121051_d_n7;
        locals.var_t9_dn8 = assign79290_e121051_d_n8;
        locals.var_t9_dn9 = assign79290_e121051_d_n9;
        locals.var_t9_dn10 = assign79290_e121051_d_n10;
        locals.var_t9_dn11 = assign79290_e121051_d_n11;
        locals.var_t9_dn14 = assign79290_e121051_d_n14;

        let (assign79300_e121066, assign79300_e121066_d_n0, assign79300_e121066_d_n2, assign79300_e121066_d_n4, assign79300_e121066_d_n5, assign79300_e121066_d_n6, assign79300_e121066_d_n7, assign79300_e121066_d_n8, assign79300_e121066_d_n9, assign79300_e121066_d_n10, assign79300_e121066_d_n11, assign79300_e121066_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) && (locals.var_guard1848 == 0.0)) {
        let assign79300_e121061: f64 = (locals.var_vdsi + p.p137);
        let assign79300_e121063: f64 = (assign79300_e121061 + locals.var_tmf2);
        let assign79300_e121064: f64 = (0.5 * assign79300_e121063);
        (assign79300_e121064, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * (locals.var_vdsi_dn6 + locals.var_tmf2_dn6)), (0.5 * locals.var_tmf2_dn7), (0.5 * (locals.var_vdsi_dn8 + locals.var_tmf2_dn8)), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign79300_e121066;
        locals.var_t2_dn0 = assign79300_e121066_d_n0;
        locals.var_t2_dn2 = assign79300_e121066_d_n2;
        locals.var_t2_dn4 = assign79300_e121066_d_n4;
        locals.var_t2_dn5 = assign79300_e121066_d_n5;
        locals.var_t2_dn6 = assign79300_e121066_d_n6;
        locals.var_t2_dn7 = assign79300_e121066_d_n7;
        locals.var_t2_dn8 = assign79300_e121066_d_n8;
        locals.var_t2_dn9 = assign79300_e121066_d_n9;
        locals.var_t2_dn10 = assign79300_e121066_d_n10;
        locals.var_t2_dn11 = assign79300_e121066_d_n11;
        locals.var_t2_dn14 = assign79300_e121066_d_n14;

        let assign79310_e121069: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1849 = assign79310_e121069;

        let (assign79320_e121080, assign79320_e121080_d_n0, assign79320_e121080_d_n2, assign79320_e121080_d_n4, assign79320_e121080_d_n5, assign79320_e121080_d_n6, assign79320_e121080_d_n7, assign79320_e121080_d_n8, assign79320_e121080_d_n9, assign79320_e121080_d_n10, assign79320_e121080_d_n11, assign79320_e121080_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) && (locals.var_guard1848 == 0.0)) && (locals.var_guard1849 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign79320_e121080;
        locals.var_t2_dn0 = assign79320_e121080_d_n0;
        locals.var_t2_dn2 = assign79320_e121080_d_n2;
        locals.var_t2_dn4 = assign79320_e121080_d_n4;
        locals.var_t2_dn5 = assign79320_e121080_d_n5;
        locals.var_t2_dn6 = assign79320_e121080_d_n6;
        locals.var_t2_dn7 = assign79320_e121080_d_n7;
        locals.var_t2_dn8 = assign79320_e121080_d_n8;
        locals.var_t2_dn9 = assign79320_e121080_d_n9;
        locals.var_t2_dn10 = assign79320_e121080_d_n10;
        locals.var_t2_dn11 = assign79320_e121080_d_n11;
        locals.var_t2_dn14 = assign79320_e121080_d_n14;

        let (assign79330_e121091, assign79330_e121091_d_n0, assign79330_e121091_d_n2, assign79330_e121091_d_n4, assign79330_e121091_d_n5, assign79330_e121091_d_n6, assign79330_e121091_d_n7, assign79330_e121091_d_n8, assign79330_e121091_d_n9, assign79330_e121091_d_n10, assign79330_e121091_d_n11, assign79330_e121091_d_n14,) = {
    if ((((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) && (locals.var_guard1848 == 0.0)) && (locals.var_guard1849 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign79330_e121091;
        locals.var_t9_dn0 = assign79330_e121091_d_n0;
        locals.var_t9_dn2 = assign79330_e121091_d_n2;
        locals.var_t9_dn4 = assign79330_e121091_d_n4;
        locals.var_t9_dn5 = assign79330_e121091_d_n5;
        locals.var_t9_dn6 = assign79330_e121091_d_n6;
        locals.var_t9_dn7 = assign79330_e121091_d_n7;
        locals.var_t9_dn8 = assign79330_e121091_d_n8;
        locals.var_t9_dn9 = assign79330_e121091_d_n9;
        locals.var_t9_dn10 = assign79330_e121091_d_n10;
        locals.var_t9_dn11 = assign79330_e121091_d_n11;
        locals.var_t9_dn14 = assign79330_e121091_d_n14;

        let (assign79340_e121105, assign79340_e121105_d_n0, assign79340_e121105_d_n2, assign79340_e121105_d_n4, assign79340_e121105_d_n5, assign79340_e121105_d_n6, assign79340_e121105_d_n7, assign79340_e121105_d_n8, assign79340_e121105_d_n9, assign79340_e121105_d_n10, assign79340_e121105_d_n11, assign79340_e121105_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) && (locals.var_guard1848 == 0.0)) {
        let assign79340_e121100: f64 = (locals.var_kjunc * locals.var_t2);
        let assign79340_e121101: f64 = (assign79340_e121100).sqrt();
        let assign79340_e121103: f64 = (assign79340_e121101 * p.p432);
        (assign79340_e121103, ((((locals.var_kjunc_dn0 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn0)) / (2.0 * assign79340_e121101)) * p.p432), ((((locals.var_kjunc_dn2 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn2)) / (2.0 * assign79340_e121101)) * p.p432), ((((locals.var_kjunc_dn4 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn4)) / (2.0 * assign79340_e121101)) * p.p432), ((((locals.var_kjunc_dn5 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn5)) / (2.0 * assign79340_e121101)) * p.p432), ((((locals.var_kjunc_dn6 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn6)) / (2.0 * assign79340_e121101)) * p.p432), ((((locals.var_kjunc_dn7 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn7)) / (2.0 * assign79340_e121101)) * p.p432), ((((locals.var_kjunc_dn8 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn8)) / (2.0 * assign79340_e121101)) * p.p432), ((((locals.var_kjunc_dn9 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn9)) / (2.0 * assign79340_e121101)) * p.p432), ((((locals.var_kjunc_dn10 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn10)) / (2.0 * assign79340_e121101)) * p.p432), ((((locals.var_kjunc_dn11 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn11)) / (2.0 * assign79340_e121101)) * p.p432), ((((locals.var_kjunc_dn14 * locals.var_t2) + (locals.var_kjunc * locals.var_t2_dn14)) / (2.0 * assign79340_e121101)) * p.p432),)
    } else {
        (locals.var_wjunc0, locals.var_wjunc0_dn0, locals.var_wjunc0_dn2, locals.var_wjunc0_dn4, locals.var_wjunc0_dn5, locals.var_wjunc0_dn6, locals.var_wjunc0_dn7, locals.var_wjunc0_dn8, locals.var_wjunc0_dn9, locals.var_wjunc0_dn10, locals.var_wjunc0_dn11, locals.var_wjunc0_dn14,)
    }
};
        locals.var_wjunc0 = assign79340_e121105;
        locals.var_wjunc0_dn0 = assign79340_e121105_d_n0;
        locals.var_wjunc0_dn2 = assign79340_e121105_d_n2;
        locals.var_wjunc0_dn4 = assign79340_e121105_d_n4;
        locals.var_wjunc0_dn5 = assign79340_e121105_d_n5;
        locals.var_wjunc0_dn6 = assign79340_e121105_d_n6;
        locals.var_wjunc0_dn7 = assign79340_e121105_d_n7;
        locals.var_wjunc0_dn8 = assign79340_e121105_d_n8;
        locals.var_wjunc0_dn9 = assign79340_e121105_d_n9;
        locals.var_wjunc0_dn10 = assign79340_e121105_d_n10;
        locals.var_wjunc0_dn11 = assign79340_e121105_d_n11;
        locals.var_wjunc0_dn14 = assign79340_e121105_d_n14;

        let (assign79350_e121116, assign79350_e121116_d_n0, assign79350_e121116_d_n2, assign79350_e121116_d_n4, assign79350_e121116_d_n5, assign79350_e121116_d_n6, assign79350_e121116_d_n7, assign79350_e121116_d_n8, assign79350_e121116_d_n9, assign79350_e121116_d_n10, assign79350_e121116_d_n11, assign79350_e121116_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) && (locals.var_guard1848 == 0.0)) {
        let assign79350_e121114: f64 = (p.p334 - locals.var_wjunc0);
        (assign79350_e121114, (-locals.var_wjunc0_dn0), (-locals.var_wjunc0_dn2), (-locals.var_wjunc0_dn4), (-locals.var_wjunc0_dn5), (-locals.var_wjunc0_dn6), (-locals.var_wjunc0_dn7), (-locals.var_wjunc0_dn8), (-locals.var_wjunc0_dn9), (-locals.var_wjunc0_dn10), (-locals.var_wjunc0_dn11), (-locals.var_wjunc0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign79350_e121116;
        locals.var_t2_dn0 = assign79350_e121116_d_n0;
        locals.var_t2_dn2 = assign79350_e121116_d_n2;
        locals.var_t2_dn4 = assign79350_e121116_d_n4;
        locals.var_t2_dn5 = assign79350_e121116_d_n5;
        locals.var_t2_dn6 = assign79350_e121116_d_n6;
        locals.var_t2_dn7 = assign79350_e121116_d_n7;
        locals.var_t2_dn8 = assign79350_e121116_d_n8;
        locals.var_t2_dn9 = assign79350_e121116_d_n9;
        locals.var_t2_dn10 = assign79350_e121116_d_n10;
        locals.var_t2_dn11 = assign79350_e121116_d_n11;
        locals.var_t2_dn14 = assign79350_e121116_d_n14;

        let (assign79360_e121135, assign79360_e121135_d_n0, assign79360_e121135_d_n2, assign79360_e121135_d_n4, assign79360_e121135_d_n5, assign79360_e121135_d_n6, assign79360_e121135_d_n7, assign79360_e121135_d_n8, assign79360_e121135_d_n9, assign79360_e121135_d_n10, assign79360_e121135_d_n11, assign79360_e121135_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) {
        let assign79360_e121122: f64 = (locals.var_t2 * locals.var_t2);
        let assign79360_e121126: f64 = (p.p334 * 0.01);
        let assign79360_e121127: f64 = (4.0 * assign79360_e121126);
        let assign79360_e121130: f64 = (p.p334 * 0.01);
        let assign79360_e121131: f64 = (assign79360_e121127 * assign79360_e121130);
        let assign79360_e121132: f64 = (assign79360_e121122 + assign79360_e121131);
        let assign79360_e121133: f64 = (assign79360_e121132).sqrt();
        (assign79360_e121133, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign79360_e121133)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign79360_e121133)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign79360_e121133)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign79360_e121133)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign79360_e121133)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign79360_e121133)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign79360_e121133)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign79360_e121133)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign79360_e121133)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign79360_e121133)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign79360_e121133)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign79360_e121135;
        locals.var_tmf2_dn0 = assign79360_e121135_d_n0;
        locals.var_tmf2_dn2 = assign79360_e121135_d_n2;
        locals.var_tmf2_dn4 = assign79360_e121135_d_n4;
        locals.var_tmf2_dn5 = assign79360_e121135_d_n5;
        locals.var_tmf2_dn6 = assign79360_e121135_d_n6;
        locals.var_tmf2_dn7 = assign79360_e121135_d_n7;
        locals.var_tmf2_dn8 = assign79360_e121135_d_n8;
        locals.var_tmf2_dn9 = assign79360_e121135_d_n9;
        locals.var_tmf2_dn10 = assign79360_e121135_d_n10;
        locals.var_tmf2_dn11 = assign79360_e121135_d_n11;
        locals.var_tmf2_dn14 = assign79360_e121135_d_n14;

    }

    pub(super) fn stamp_transient_block_287(
        locals: &mut StampLocals,
    ) {
        let (assign79370_e121147, assign79370_e121147_d_n0, assign79370_e121147_d_n2, assign79370_e121147_d_n4, assign79370_e121147_d_n5, assign79370_e121147_d_n6, assign79370_e121147_d_n7, assign79370_e121147_d_n8, assign79370_e121147_d_n9, assign79370_e121147_d_n10, assign79370_e121147_d_n11, assign79370_e121147_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) {
        let assign79370_e121143: f64 = (locals.var_t2 / locals.var_tmf2);
        let assign79370_e121144: f64 = (1.0 + assign79370_e121143);
        let assign79370_e121145: f64 = (0.5 * assign79370_e121144);
        (assign79370_e121145, (0.5 * (((locals.var_t2_dn0 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn2 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn4 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn5 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn6 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn7 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn8 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn9 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn10 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn11 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t2_dn14 * locals.var_tmf2) - (locals.var_t2 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign79370_e121147;
        locals.var_t9_dn0 = assign79370_e121147_d_n0;
        locals.var_t9_dn2 = assign79370_e121147_d_n2;
        locals.var_t9_dn4 = assign79370_e121147_d_n4;
        locals.var_t9_dn5 = assign79370_e121147_d_n5;
        locals.var_t9_dn6 = assign79370_e121147_d_n6;
        locals.var_t9_dn7 = assign79370_e121147_d_n7;
        locals.var_t9_dn8 = assign79370_e121147_d_n8;
        locals.var_t9_dn9 = assign79370_e121147_d_n9;
        locals.var_t9_dn10 = assign79370_e121147_d_n10;
        locals.var_t9_dn11 = assign79370_e121147_d_n11;
        locals.var_t9_dn14 = assign79370_e121147_d_n14;

        let (assign79380_e121157, assign79380_e121157_d_n0, assign79380_e121157_d_n2, assign79380_e121157_d_n4, assign79380_e121157_d_n5, assign79380_e121157_d_n6, assign79380_e121157_d_n7, assign79380_e121157_d_n8, assign79380_e121157_d_n9, assign79380_e121157_d_n10, assign79380_e121157_d_n11, assign79380_e121157_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) {
        let assign79380_e121154: f64 = (locals.var_t2 + locals.var_tmf2);
        let assign79380_e121155: f64 = (0.5 * assign79380_e121154);
        (assign79380_e121155, (0.5 * (locals.var_t2_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t2_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t2_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t2_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign79380_e121157;
        locals.var_t2_dn0 = assign79380_e121157_d_n0;
        locals.var_t2_dn2 = assign79380_e121157_d_n2;
        locals.var_t2_dn4 = assign79380_e121157_d_n4;
        locals.var_t2_dn5 = assign79380_e121157_d_n5;
        locals.var_t2_dn6 = assign79380_e121157_d_n6;
        locals.var_t2_dn7 = assign79380_e121157_d_n7;
        locals.var_t2_dn8 = assign79380_e121157_d_n8;
        locals.var_t2_dn9 = assign79380_e121157_d_n9;
        locals.var_t2_dn10 = assign79380_e121157_d_n10;
        locals.var_t2_dn11 = assign79380_e121157_d_n11;
        locals.var_t2_dn14 = assign79380_e121157_d_n14;

        let assign79390_e121160: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1850 = assign79390_e121160;

        let (assign79400_e121168, assign79400_e121168_d_n0, assign79400_e121168_d_n2, assign79400_e121168_d_n4, assign79400_e121168_d_n5, assign79400_e121168_d_n6, assign79400_e121168_d_n7, assign79400_e121168_d_n8, assign79400_e121168_d_n9, assign79400_e121168_d_n10, assign79400_e121168_d_n11, assign79400_e121168_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) && (locals.var_guard1850 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign79400_e121168;
        locals.var_t2_dn0 = assign79400_e121168_d_n0;
        locals.var_t2_dn2 = assign79400_e121168_d_n2;
        locals.var_t2_dn4 = assign79400_e121168_d_n4;
        locals.var_t2_dn5 = assign79400_e121168_d_n5;
        locals.var_t2_dn6 = assign79400_e121168_d_n6;
        locals.var_t2_dn7 = assign79400_e121168_d_n7;
        locals.var_t2_dn8 = assign79400_e121168_d_n8;
        locals.var_t2_dn9 = assign79400_e121168_d_n9;
        locals.var_t2_dn10 = assign79400_e121168_d_n10;
        locals.var_t2_dn11 = assign79400_e121168_d_n11;
        locals.var_t2_dn14 = assign79400_e121168_d_n14;

        let (assign79410_e121176, assign79410_e121176_d_n0, assign79410_e121176_d_n2, assign79410_e121176_d_n4, assign79410_e121176_d_n5, assign79410_e121176_d_n6, assign79410_e121176_d_n7, assign79410_e121176_d_n8, assign79410_e121176_d_n9, assign79410_e121176_d_n10, assign79410_e121176_d_n11, assign79410_e121176_d_n14,) = {
    if (((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) && (locals.var_guard1850 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign79410_e121176;
        locals.var_t9_dn0 = assign79410_e121176_d_n0;
        locals.var_t9_dn2 = assign79410_e121176_d_n2;
        locals.var_t9_dn4 = assign79410_e121176_d_n4;
        locals.var_t9_dn5 = assign79410_e121176_d_n5;
        locals.var_t9_dn6 = assign79410_e121176_d_n6;
        locals.var_t9_dn7 = assign79410_e121176_d_n7;
        locals.var_t9_dn8 = assign79410_e121176_d_n8;
        locals.var_t9_dn9 = assign79410_e121176_d_n9;
        locals.var_t9_dn10 = assign79410_e121176_d_n10;
        locals.var_t9_dn11 = assign79410_e121176_d_n11;
        locals.var_t9_dn14 = assign79410_e121176_d_n14;

        let (assign79420_e121182, assign79420_e121182_d_n0, assign79420_e121182_d_n2, assign79420_e121182_d_n4, assign79420_e121182_d_n5, assign79420_e121182_d_n6, assign79420_e121182_d_n7, assign79420_e121182_d_n8, assign79420_e121182_d_n9, assign79420_e121182_d_n10, assign79420_e121182_d_n11, assign79420_e121182_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_ddriftldc, locals.var_ddriftldc_dn0, locals.var_ddriftldc_dn2, locals.var_ddriftldc_dn4, locals.var_ddriftldc_dn5, locals.var_ddriftldc_dn6, locals.var_ddriftldc_dn7, locals.var_ddriftldc_dn8, locals.var_ddriftldc_dn9, locals.var_ddriftldc_dn10, locals.var_ddriftldc_dn11, locals.var_ddriftldc_dn14,)
    }
};
        locals.var_ddriftldc = assign79420_e121182;
        locals.var_ddriftldc_dn0 = assign79420_e121182_d_n0;
        locals.var_ddriftldc_dn2 = assign79420_e121182_d_n2;
        locals.var_ddriftldc_dn4 = assign79420_e121182_d_n4;
        locals.var_ddriftldc_dn5 = assign79420_e121182_d_n5;
        locals.var_ddriftldc_dn6 = assign79420_e121182_d_n6;
        locals.var_ddriftldc_dn7 = assign79420_e121182_d_n7;
        locals.var_ddriftldc_dn8 = assign79420_e121182_d_n8;
        locals.var_ddriftldc_dn9 = assign79420_e121182_d_n9;
        locals.var_ddriftldc_dn10 = assign79420_e121182_d_n10;
        locals.var_ddriftldc_dn11 = assign79420_e121182_d_n11;
        locals.var_ddriftldc_dn14 = assign79420_e121182_d_n14;

        let (assign79430_e121196, assign79430_e121196_d_n0, assign79430_e121196_d_n2, assign79430_e121196_d_n4, assign79430_e121196_d_n5, assign79430_e121196_d_n6, assign79430_e121196_d_n7, assign79430_e121196_d_n8, assign79430_e121196_d_n9, assign79430_e121196_d_n10, assign79430_e121196_d_n11, assign79430_e121196_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) {
        let assign79430_e121188: f64 = (locals.var_q_nsubld__blk1766 * locals.var_ddriftldc);
        let assign79430_e121190: f64 = (assign79430_e121188 * locals.var_ddriftldc);
        let assign79430_e121192: f64 = (assign79430_e121190 / 2.0);
        let assign79430_e121194: f64 = (assign79430_e121192 / 1.034943e-10);
        (assign79430_e121194, (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn0) * locals.var_ddriftldc) + (assign79430_e121188 * locals.var_ddriftldc_dn0)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn2) * locals.var_ddriftldc) + (assign79430_e121188 * locals.var_ddriftldc_dn2)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn4) * locals.var_ddriftldc) + (assign79430_e121188 * locals.var_ddriftldc_dn4)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn5) * locals.var_ddriftldc) + (assign79430_e121188 * locals.var_ddriftldc_dn5)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn6) * locals.var_ddriftldc) + (assign79430_e121188 * locals.var_ddriftldc_dn6)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn7) * locals.var_ddriftldc) + (assign79430_e121188 * locals.var_ddriftldc_dn7)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn8) * locals.var_ddriftldc) + (assign79430_e121188 * locals.var_ddriftldc_dn8)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn9) * locals.var_ddriftldc) + (assign79430_e121188 * locals.var_ddriftldc_dn9)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn10) * locals.var_ddriftldc) + (assign79430_e121188 * locals.var_ddriftldc_dn10)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn11) * locals.var_ddriftldc) + (assign79430_e121188 * locals.var_ddriftldc_dn11)) / 2.0) / 1.034943e-10), (((((locals.var_q_nsubld__blk1766 * locals.var_ddriftldc_dn14) * locals.var_ddriftldc) + (assign79430_e121188 * locals.var_ddriftldc_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb, locals.var_dphi_sb_dn0, locals.var_dphi_sb_dn2, locals.var_dphi_sb_dn4, locals.var_dphi_sb_dn5, locals.var_dphi_sb_dn6, locals.var_dphi_sb_dn7, locals.var_dphi_sb_dn8, locals.var_dphi_sb_dn9, locals.var_dphi_sb_dn10, locals.var_dphi_sb_dn11, locals.var_dphi_sb_dn14,)
    }
};
        locals.var_dphi_sb = assign79430_e121196;
        locals.var_dphi_sb_dn0 = assign79430_e121196_d_n0;
        locals.var_dphi_sb_dn2 = assign79430_e121196_d_n2;
        locals.var_dphi_sb_dn4 = assign79430_e121196_d_n4;
        locals.var_dphi_sb_dn5 = assign79430_e121196_d_n5;
        locals.var_dphi_sb_dn6 = assign79430_e121196_d_n6;
        locals.var_dphi_sb_dn7 = assign79430_e121196_d_n7;
        locals.var_dphi_sb_dn8 = assign79430_e121196_d_n8;
        locals.var_dphi_sb_dn9 = assign79430_e121196_d_n9;
        locals.var_dphi_sb_dn10 = assign79430_e121196_d_n10;
        locals.var_dphi_sb_dn11 = assign79430_e121196_d_n11;
        locals.var_dphi_sb_dn14 = assign79430_e121196_d_n14;

        let (assign79440_e121207, assign79440_e121207_d_n0, assign79440_e121207_d_n2, assign79440_e121207_d_n4, assign79440_e121207_d_n5, assign79440_e121207_d_n6, assign79440_e121207_d_n7, assign79440_e121207_d_n8, assign79440_e121207_d_n9, assign79440_e121207_d_n10, assign79440_e121207_d_n11, assign79440_e121207_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) {
        let assign79440_e121202: f64 = (2.0 * locals.var_beta);
        let assign79440_e121204: f64 = (assign79440_e121202 * locals.var_dphi_sb);
        let assign79440_e121205: f64 = (assign79440_e121204).sqrt();
        (assign79440_e121205, ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb) + (assign79440_e121202 * locals.var_dphi_sb_dn0)) / (2.0 * assign79440_e121205)), ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb) + (assign79440_e121202 * locals.var_dphi_sb_dn2)) / (2.0 * assign79440_e121205)), ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb) + (assign79440_e121202 * locals.var_dphi_sb_dn4)) / (2.0 * assign79440_e121205)), ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb) + (assign79440_e121202 * locals.var_dphi_sb_dn5)) / (2.0 * assign79440_e121205)), ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb) + (assign79440_e121202 * locals.var_dphi_sb_dn6)) / (2.0 * assign79440_e121205)), ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb) + (assign79440_e121202 * locals.var_dphi_sb_dn7)) / (2.0 * assign79440_e121205)), ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb) + (assign79440_e121202 * locals.var_dphi_sb_dn8)) / (2.0 * assign79440_e121205)), ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb) + (assign79440_e121202 * locals.var_dphi_sb_dn9)) / (2.0 * assign79440_e121205)), ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb) + (assign79440_e121202 * locals.var_dphi_sb_dn10)) / (2.0 * assign79440_e121205)), ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb) + (assign79440_e121202 * locals.var_dphi_sb_dn11)) / (2.0 * assign79440_e121205)), ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb) + (assign79440_e121202 * locals.var_dphi_sb_dn14)) / (2.0 * assign79440_e121205)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign79440_e121207;
        locals.var_t0_dn0 = assign79440_e121207_d_n0;
        locals.var_t0_dn2 = assign79440_e121207_d_n2;
        locals.var_t0_dn4 = assign79440_e121207_d_n4;
        locals.var_t0_dn5 = assign79440_e121207_d_n5;
        locals.var_t0_dn6 = assign79440_e121207_d_n6;
        locals.var_t0_dn7 = assign79440_e121207_d_n7;
        locals.var_t0_dn8 = assign79440_e121207_d_n8;
        locals.var_t0_dn9 = assign79440_e121207_d_n9;
        locals.var_t0_dn10 = assign79440_e121207_d_n10;
        locals.var_t0_dn11 = assign79440_e121207_d_n11;
        locals.var_t0_dn14 = assign79440_e121207_d_n14;

        let (assign79450_e121220, assign79450_e121220_d_n0, assign79450_e121220_d_n2, assign79450_e121220_d_n4, assign79450_e121220_d_n5, assign79450_e121220_d_n6, assign79450_e121220_d_n7, assign79450_e121220_d_n8, assign79450_e121220_d_n9, assign79450_e121220_d_n10, assign79450_e121220_d_n11, assign79450_e121220_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) {
        let assign79450_e121212: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign79450_e121214: f64 = (-locals.var_t0);
        let assign79450_e121215: f64 = { let limited_exp_arg = assign79450_e121214; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign79450_e121216: f64 = (assign79450_e121212 + assign79450_e121215);
        let assign79450_e121218: f64 = (assign79450_e121216 / 2.0);
        (assign79450_e121218, ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) + ({ let limited_exp_arg = assign79450_e121214; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) + ({ let limited_exp_arg = assign79450_e121214; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + ({ let limited_exp_arg = assign79450_e121214; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) + ({ let limited_exp_arg = assign79450_e121214; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) + ({ let limited_exp_arg = assign79450_e121214; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) + ({ let limited_exp_arg = assign79450_e121214; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) + ({ let limited_exp_arg = assign79450_e121214; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) + ({ let limited_exp_arg = assign79450_e121214; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) + ({ let limited_exp_arg = assign79450_e121214; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) + ({ let limited_exp_arg = assign79450_e121214; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))) / 2.0), ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) + ({ let limited_exp_arg = assign79450_e121214; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign79450_e121220;
        locals.var_t1_dn0 = assign79450_e121220_d_n0;
        locals.var_t1_dn2 = assign79450_e121220_d_n2;
        locals.var_t1_dn4 = assign79450_e121220_d_n4;
        locals.var_t1_dn5 = assign79450_e121220_d_n5;
        locals.var_t1_dn6 = assign79450_e121220_d_n6;
        locals.var_t1_dn7 = assign79450_e121220_d_n7;
        locals.var_t1_dn8 = assign79450_e121220_d_n8;
        locals.var_t1_dn9 = assign79450_e121220_d_n9;
        locals.var_t1_dn10 = assign79450_e121220_d_n10;
        locals.var_t1_dn11 = assign79450_e121220_d_n11;
        locals.var_t1_dn14 = assign79450_e121220_d_n14;

        let (assign79460_e121229, assign79460_e121229_d_n0, assign79460_e121229_d_n2, assign79460_e121229_d_n4, assign79460_e121229_d_n5, assign79460_e121229_d_n6, assign79460_e121229_d_n7, assign79460_e121229_d_n8, assign79460_e121229_d_n9, assign79460_e121229_d_n10, assign79460_e121229_d_n11, assign79460_e121229_d_n14,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) {
        let assign79460_e121225: f64 = (locals.var_t1).ln();
        let assign79460_e121227: f64 = (assign79460_e121225 / locals.var_dphi_sb);
        (assign79460_e121227, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb) - (assign79460_e121225 * locals.var_dphi_sb_dn0)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb) - (assign79460_e121225 * locals.var_dphi_sb_dn2)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb) - (assign79460_e121225 * locals.var_dphi_sb_dn4)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb) - (assign79460_e121225 * locals.var_dphi_sb_dn5)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb) - (assign79460_e121225 * locals.var_dphi_sb_dn6)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb) - (assign79460_e121225 * locals.var_dphi_sb_dn7)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb) - (assign79460_e121225 * locals.var_dphi_sb_dn8)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb) - (assign79460_e121225 * locals.var_dphi_sb_dn9)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb) - (assign79460_e121225 * locals.var_dphi_sb_dn10)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb) - (assign79460_e121225 * locals.var_dphi_sb_dn11)) / (locals.var_dphi_sb * locals.var_dphi_sb)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb) - (assign79460_e121225 * locals.var_dphi_sb_dn14)) / (locals.var_dphi_sb * locals.var_dphi_sb)),)
    } else {
        (locals.var_c_sb, locals.var_c_sb_dn0, locals.var_c_sb_dn2, locals.var_c_sb_dn4, locals.var_c_sb_dn5, locals.var_c_sb_dn6, locals.var_c_sb_dn7, locals.var_c_sb_dn8, locals.var_c_sb_dn9, locals.var_c_sb_dn10, locals.var_c_sb_dn11, locals.var_c_sb_dn14,)
    }
};
        locals.var_c_sb = assign79460_e121229;
        locals.var_c_sb_dn0 = assign79460_e121229_d_n0;
        locals.var_c_sb_dn2 = assign79460_e121229_d_n2;
        locals.var_c_sb_dn4 = assign79460_e121229_d_n4;
        locals.var_c_sb_dn5 = assign79460_e121229_d_n5;
        locals.var_c_sb_dn6 = assign79460_e121229_d_n6;
        locals.var_c_sb_dn7 = assign79460_e121229_d_n7;
        locals.var_c_sb_dn8 = assign79460_e121229_d_n8;
        locals.var_c_sb_dn9 = assign79460_e121229_d_n9;
        locals.var_c_sb_dn10 = assign79460_e121229_d_n10;
        locals.var_c_sb_dn11 = assign79460_e121229_d_n11;
        locals.var_c_sb_dn14 = assign79460_e121229_d_n14;

        let (assign79470_e121235,) = {
    if ((locals.var_flg_calcqover != 0.0) && (locals.var_guard1847 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign79470_e121235;

    }
}
