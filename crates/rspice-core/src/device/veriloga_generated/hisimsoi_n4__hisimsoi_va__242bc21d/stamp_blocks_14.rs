#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_96(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26860_e37288, assign26860_e37288_d_n0, assign26860_e37288_d_n2, assign26860_e37288_d_n6, assign26860_e37288_d_n7, assign26860_e37288_d_n10, assign26860_e37288_d_n11, assign26860_e37288_d_n12, assign26860_e37288_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26860_e37286: f64 = (locals.var_eg - locals.var_pb2over);
        (assign26860_e37286, (locals.var_eg_dn0 - locals.var_pb2over_dn0), (locals.var_eg_dn2 - locals.var_pb2over_dn2), (locals.var_eg_dn6 - locals.var_pb2over_dn6), (locals.var_eg_dn7 - locals.var_pb2over_dn7), (locals.var_eg_dn10 - locals.var_pb2over_dn10), (locals.var_eg_dn11 - locals.var_pb2over_dn11), (locals.var_eg_dn12 - locals.var_pb2over_dn12), (locals.var_eg_dn17 - locals.var_pb2over_dn17),)
    } else {
        (locals.var_ps0_min__blk807, locals.var_ps0_min__blk807_dn0, locals.var_ps0_min__blk807_dn2, locals.var_ps0_min__blk807_dn6, locals.var_ps0_min__blk807_dn7, locals.var_ps0_min__blk807_dn10, locals.var_ps0_min__blk807_dn11, locals.var_ps0_min__blk807_dn12, locals.var_ps0_min__blk807_dn17,)
    }
};
        locals.var_ps0_min__blk807 = assign26860_e37288;
        locals.var_ps0_min__blk807_dn0 = assign26860_e37288_d_n0;
        locals.var_ps0_min__blk807_dn2 = assign26860_e37288_d_n2;
        locals.var_ps0_min__blk807_dn6 = assign26860_e37288_d_n6;
        locals.var_ps0_min__blk807_dn7 = assign26860_e37288_d_n7;
        locals.var_ps0_min__blk807_dn10 = assign26860_e37288_d_n10;
        locals.var_ps0_min__blk807_dn11 = assign26860_e37288_d_n11;
        locals.var_ps0_min__blk807_dn12 = assign26860_e37288_d_n12;
        locals.var_ps0_min__blk807_dn17 = assign26860_e37288_d_n17;
        locals.var_ps0_min__blk807_rv = 0.0;

        let (assign26870_e37302, assign26870_e37302_d_n0, assign26870_e37302_d_n2, assign26870_e37302_d_n6, assign26870_e37302_d_n7, assign26870_e37302_d_n10, assign26870_e37302_d_n11, assign26870_e37302_d_n12, assign26870_e37302_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26870_e37299: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign26870_e37300: f64 = (locals.var_beta * assign26870_e37299);
        (assign26870_e37300, (locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign26870_e37299) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_tx__blk777, locals.var_tx__blk777_dn0, locals.var_tx__blk777_dn2, locals.var_tx__blk777_dn6, locals.var_tx__blk777_dn7, locals.var_tx__blk777_dn10, locals.var_tx__blk777_dn11, locals.var_tx__blk777_dn12, locals.var_tx__blk777_dn17,)
    }
};
        locals.var_tx__blk777 = assign26870_e37302;
        locals.var_tx__blk777_dn0 = assign26870_e37302_d_n0;
        locals.var_tx__blk777_dn2 = assign26870_e37302_d_n2;
        locals.var_tx__blk777_dn6 = assign26870_e37302_d_n6;
        locals.var_tx__blk777_dn7 = assign26870_e37302_d_n7;
        locals.var_tx__blk777_dn10 = assign26870_e37302_d_n10;
        locals.var_tx__blk777_dn11 = assign26870_e37302_d_n11;
        locals.var_tx__blk777_dn12 = assign26870_e37302_d_n12;
        locals.var_tx__blk777_dn17 = assign26870_e37302_d_n17;
        locals.var_tx__blk777_rv = 0.0;

        let (assign26880_e37322, assign26880_e37322_d_n0, assign26880_e37322_d_n2, assign26880_e37322_d_n6, assign26880_e37322_d_n7, assign26880_e37322_d_n10, assign26880_e37322_d_n11, assign26880_e37322_d_n12, assign26880_e37322_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26880_e37312: f64 = (7.0 * 1.414213562373095);
        let assign26880_e37315: f64 = (9.0 * locals.var_ty__blk778);
        let assign26880_e37318: f64 = (locals.var_tx__blk777 - 2.0);
        let assign26880_e37319: f64 = (assign26880_e37315 * assign26880_e37318);
        let assign26880_e37320: f64 = (assign26880_e37312 - assign26880_e37319);
        (assign26880_e37320, (-(((9.0 * locals.var_ty__blk778_dn0) * assign26880_e37318) + (assign26880_e37315 * locals.var_tx__blk777_dn0))), (-(((9.0 * locals.var_ty__blk778_dn2) * assign26880_e37318) + (assign26880_e37315 * locals.var_tx__blk777_dn2))), (-(((9.0 * locals.var_ty__blk778_dn6) * assign26880_e37318) + (assign26880_e37315 * locals.var_tx__blk777_dn6))), (-(((9.0 * locals.var_ty__blk778_dn7) * assign26880_e37318) + (assign26880_e37315 * locals.var_tx__blk777_dn7))), (-(((9.0 * locals.var_ty__blk778_dn10) * assign26880_e37318) + (assign26880_e37315 * locals.var_tx__blk777_dn10))), (-(((9.0 * locals.var_ty__blk778_dn11) * assign26880_e37318) + (assign26880_e37315 * locals.var_tx__blk777_dn11))), (-(((9.0 * locals.var_ty__blk778_dn12) * assign26880_e37318) + (assign26880_e37315 * locals.var_tx__blk777_dn12))), (-(((9.0 * locals.var_ty__blk778_dn17) * assign26880_e37318) + (assign26880_e37315 * locals.var_tx__blk777_dn17))),)
    } else {
        (locals.var_ac31__blk808, locals.var_ac31__blk808_dn0, locals.var_ac31__blk808_dn2, locals.var_ac31__blk808_dn6, locals.var_ac31__blk808_dn7, locals.var_ac31__blk808_dn10, locals.var_ac31__blk808_dn11, locals.var_ac31__blk808_dn12, locals.var_ac31__blk808_dn17,)
    }
};
        locals.var_ac31__blk808 = assign26880_e37322;
        locals.var_ac31__blk808_dn0 = assign26880_e37322_d_n0;
        locals.var_ac31__blk808_dn2 = assign26880_e37322_d_n2;
        locals.var_ac31__blk808_dn6 = assign26880_e37322_d_n6;
        locals.var_ac31__blk808_dn7 = assign26880_e37322_d_n7;
        locals.var_ac31__blk808_dn10 = assign26880_e37322_d_n10;
        locals.var_ac31__blk808_dn11 = assign26880_e37322_d_n11;
        locals.var_ac31__blk808_dn12 = assign26880_e37322_d_n12;
        locals.var_ac31__blk808_dn17 = assign26880_e37322_d_n17;
        locals.var_ac31__blk808_rv = 0.0;

        let (assign26890_e37334, assign26890_e37334_d_n0, assign26890_e37334_d_n2, assign26890_e37334_d_n6, assign26890_e37334_d_n7, assign26890_e37334_d_n10, assign26890_e37334_d_n11, assign26890_e37334_d_n12, assign26890_e37334_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26890_e37332: f64 = (locals.var_ac31__blk808 * locals.var_ac31__blk808);
        (assign26890_e37332, ((locals.var_ac31__blk808_dn0 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn0)), ((locals.var_ac31__blk808_dn2 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn2)), ((locals.var_ac31__blk808_dn6 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn6)), ((locals.var_ac31__blk808_dn7 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn7)), ((locals.var_ac31__blk808_dn10 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn10)), ((locals.var_ac31__blk808_dn11 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn11)), ((locals.var_ac31__blk808_dn12 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn12)), ((locals.var_ac31__blk808_dn17 * locals.var_ac31__blk808) + (locals.var_ac31__blk808 * locals.var_ac31__blk808_dn17)),)
    } else {
        (locals.var_ac3__blk809, locals.var_ac3__blk809_dn0, locals.var_ac3__blk809_dn2, locals.var_ac3__blk809_dn6, locals.var_ac3__blk809_dn7, locals.var_ac3__blk809_dn10, locals.var_ac3__blk809_dn11, locals.var_ac3__blk809_dn12, locals.var_ac3__blk809_dn17,)
    }
};
        locals.var_ac3__blk809 = assign26890_e37334;
        locals.var_ac3__blk809_dn0 = assign26890_e37334_d_n0;
        locals.var_ac3__blk809_dn2 = assign26890_e37334_d_n2;
        locals.var_ac3__blk809_dn6 = assign26890_e37334_d_n6;
        locals.var_ac3__blk809_dn7 = assign26890_e37334_d_n7;
        locals.var_ac3__blk809_dn10 = assign26890_e37334_d_n10;
        locals.var_ac3__blk809_dn11 = assign26890_e37334_d_n11;
        locals.var_ac3__blk809_dn12 = assign26890_e37334_d_n12;
        locals.var_ac3__blk809_dn17 = assign26890_e37334_d_n17;
        locals.var_ac3__blk809_rv = 0.0;

        let assign26900_e37338: f64 = (locals.var_ac3__blk809 * 1e-8);
        let assign26900_e37339: f64 = if locals.var_ac4__blk806 < assign26900_e37338 { 1.0 } else { 0.0 };
        locals.var_guard872 = assign26900_e37339;
        locals.var_guard872_rv = 0.0;

        let (assign26910_e37370, assign26910_e37370_d_n0, assign26910_e37370_d_n2, assign26910_e37370_d_n6, assign26910_e37370_d_n7, assign26910_e37370_d_n10, assign26910_e37370_d_n11, assign26910_e37370_d_n12, assign26910_e37370_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) && (locals.var_guard872 != 0.0)) {
        let assign26910_e37350: f64 = (-7.0);
        let assign26910_e37352: f64 = (assign26910_e37350 * 1.414213562373095);
        let assign26910_e37354: f64 = (assign26910_e37352 + locals.var_ac31__blk808);
        let assign26910_e37357: f64 = (0.5 * locals.var_ac4__blk806);
        let assign26910_e37359: f64 = (assign26910_e37357 / locals.var_ac31__blk808);
        let assign26910_e37360: f64 = (assign26910_e37354 + assign26910_e37359);
        let assign26910_e37363: f64 = (9.0 * locals.var_ty__blk778);
        let assign26910_e37366: f64 = (locals.var_tx__blk777 - 2.0);
        let assign26910_e37367: f64 = (assign26910_e37363 * assign26910_e37366);
        let assign26910_e37368: f64 = (assign26910_e37360 + assign26910_e37367);
        (assign26910_e37368, ((locals.var_ac31__blk808_dn0 + ((((0.5 * locals.var_ac4__blk806_dn0) * locals.var_ac31__blk808) - (assign26910_e37357 * locals.var_ac31__blk808_dn0)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn0) * assign26910_e37366) + (assign26910_e37363 * locals.var_tx__blk777_dn0))), ((locals.var_ac31__blk808_dn2 + ((((0.5 * locals.var_ac4__blk806_dn2) * locals.var_ac31__blk808) - (assign26910_e37357 * locals.var_ac31__blk808_dn2)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn2) * assign26910_e37366) + (assign26910_e37363 * locals.var_tx__blk777_dn2))), ((locals.var_ac31__blk808_dn6 + ((((0.5 * locals.var_ac4__blk806_dn6) * locals.var_ac31__blk808) - (assign26910_e37357 * locals.var_ac31__blk808_dn6)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn6) * assign26910_e37366) + (assign26910_e37363 * locals.var_tx__blk777_dn6))), ((locals.var_ac31__blk808_dn7 + ((((0.5 * locals.var_ac4__blk806_dn7) * locals.var_ac31__blk808) - (assign26910_e37357 * locals.var_ac31__blk808_dn7)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn7) * assign26910_e37366) + (assign26910_e37363 * locals.var_tx__blk777_dn7))), ((locals.var_ac31__blk808_dn10 + ((((0.5 * locals.var_ac4__blk806_dn10) * locals.var_ac31__blk808) - (assign26910_e37357 * locals.var_ac31__blk808_dn10)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn10) * assign26910_e37366) + (assign26910_e37363 * locals.var_tx__blk777_dn10))), ((locals.var_ac31__blk808_dn11 + ((((0.5 * locals.var_ac4__blk806_dn11) * locals.var_ac31__blk808) - (assign26910_e37357 * locals.var_ac31__blk808_dn11)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn11) * assign26910_e37366) + (assign26910_e37363 * locals.var_tx__blk777_dn11))), ((locals.var_ac31__blk808_dn12 + ((((0.5 * locals.var_ac4__blk806_dn12) * locals.var_ac31__blk808) - (assign26910_e37357 * locals.var_ac31__blk808_dn12)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn12) * assign26910_e37366) + (assign26910_e37363 * locals.var_tx__blk777_dn12))), ((locals.var_ac31__blk808_dn17 + ((((0.5 * locals.var_ac4__blk806_dn17) * locals.var_ac31__blk808) - (assign26910_e37357 * locals.var_ac31__blk808_dn17)) / (locals.var_ac31__blk808 * locals.var_ac31__blk808))) + (((9.0 * locals.var_ty__blk778_dn17) * assign26910_e37366) + (assign26910_e37363 * locals.var_tx__blk777_dn17))),)
    } else {
        (locals.var_ac1__blk811, locals.var_ac1__blk811_dn0, locals.var_ac1__blk811_dn2, locals.var_ac1__blk811_dn6, locals.var_ac1__blk811_dn7, locals.var_ac1__blk811_dn10, locals.var_ac1__blk811_dn11, locals.var_ac1__blk811_dn12, locals.var_ac1__blk811_dn17,)
    }
};
        locals.var_ac1__blk811 = assign26910_e37370;
        locals.var_ac1__blk811_dn0 = assign26910_e37370_d_n0;
        locals.var_ac1__blk811_dn2 = assign26910_e37370_d_n2;
        locals.var_ac1__blk811_dn6 = assign26910_e37370_d_n6;
        locals.var_ac1__blk811_dn7 = assign26910_e37370_d_n7;
        locals.var_ac1__blk811_dn10 = assign26910_e37370_d_n10;
        locals.var_ac1__blk811_dn11 = assign26910_e37370_d_n11;
        locals.var_ac1__blk811_dn12 = assign26910_e37370_d_n12;
        locals.var_ac1__blk811_dn17 = assign26910_e37370_d_n17;
        locals.var_ac1__blk811_rv = 0.0;

        let (assign26920_e37386, assign26920_e37386_d_n0, assign26920_e37386_d_n2, assign26920_e37386_d_n6, assign26920_e37386_d_n7, assign26920_e37386_d_n10, assign26920_e37386_d_n11, assign26920_e37386_d_n12, assign26920_e37386_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) && (locals.var_guard872 == 0.0)) {
        let assign26920_e37383: f64 = (locals.var_ac4__blk806 + locals.var_ac3__blk809);
        let assign26920_e37384: f64 = (assign26920_e37383).sqrt();
        (assign26920_e37384, ((locals.var_ac4__blk806_dn0 + locals.var_ac3__blk809_dn0) / (2.0 * assign26920_e37384)), ((locals.var_ac4__blk806_dn2 + locals.var_ac3__blk809_dn2) / (2.0 * assign26920_e37384)), ((locals.var_ac4__blk806_dn6 + locals.var_ac3__blk809_dn6) / (2.0 * assign26920_e37384)), ((locals.var_ac4__blk806_dn7 + locals.var_ac3__blk809_dn7) / (2.0 * assign26920_e37384)), ((locals.var_ac4__blk806_dn10 + locals.var_ac3__blk809_dn10) / (2.0 * assign26920_e37384)), ((locals.var_ac4__blk806_dn11 + locals.var_ac3__blk809_dn11) / (2.0 * assign26920_e37384)), ((locals.var_ac4__blk806_dn12 + locals.var_ac3__blk809_dn12) / (2.0 * assign26920_e37384)), ((locals.var_ac4__blk806_dn17 + locals.var_ac3__blk809_dn17) / (2.0 * assign26920_e37384)),)
    } else {
        (locals.var_ac2__blk810, locals.var_ac2__blk810_dn0, locals.var_ac2__blk810_dn2, locals.var_ac2__blk810_dn6, locals.var_ac2__blk810_dn7, locals.var_ac2__blk810_dn10, locals.var_ac2__blk810_dn11, locals.var_ac2__blk810_dn12, locals.var_ac2__blk810_dn17,)
    }
};
        locals.var_ac2__blk810 = assign26920_e37386;
        locals.var_ac2__blk810_dn0 = assign26920_e37386_d_n0;
        locals.var_ac2__blk810_dn2 = assign26920_e37386_d_n2;
        locals.var_ac2__blk810_dn6 = assign26920_e37386_d_n6;
        locals.var_ac2__blk810_dn7 = assign26920_e37386_d_n7;
        locals.var_ac2__blk810_dn10 = assign26920_e37386_d_n10;
        locals.var_ac2__blk810_dn11 = assign26920_e37386_d_n11;
        locals.var_ac2__blk810_dn12 = assign26920_e37386_d_n12;
        locals.var_ac2__blk810_dn17 = assign26920_e37386_d_n17;
        locals.var_ac2__blk810_rv = 0.0;

        let (assign26930_e37412, assign26930_e37412_d_n0, assign26930_e37412_d_n2, assign26930_e37412_d_n6, assign26930_e37412_d_n7, assign26930_e37412_d_n10, assign26930_e37412_d_n11, assign26930_e37412_d_n12, assign26930_e37412_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) && (locals.var_guard872 == 0.0)) {
        let assign26930_e37398: f64 = (-7.0);
        let assign26930_e37400: f64 = (assign26930_e37398 * 1.414213562373095);
        let assign26930_e37402: f64 = (assign26930_e37400 + locals.var_ac2__blk810);
        let assign26930_e37405: f64 = (9.0 * locals.var_ty__blk778);
        let assign26930_e37408: f64 = (locals.var_tx__blk777 - 2.0);
        let assign26930_e37409: f64 = (assign26930_e37405 * assign26930_e37408);
        let assign26930_e37410: f64 = (assign26930_e37402 + assign26930_e37409);
        (assign26930_e37410, (locals.var_ac2__blk810_dn0 + (((9.0 * locals.var_ty__blk778_dn0) * assign26930_e37408) + (assign26930_e37405 * locals.var_tx__blk777_dn0))), (locals.var_ac2__blk810_dn2 + (((9.0 * locals.var_ty__blk778_dn2) * assign26930_e37408) + (assign26930_e37405 * locals.var_tx__blk777_dn2))), (locals.var_ac2__blk810_dn6 + (((9.0 * locals.var_ty__blk778_dn6) * assign26930_e37408) + (assign26930_e37405 * locals.var_tx__blk777_dn6))), (locals.var_ac2__blk810_dn7 + (((9.0 * locals.var_ty__blk778_dn7) * assign26930_e37408) + (assign26930_e37405 * locals.var_tx__blk777_dn7))), (locals.var_ac2__blk810_dn10 + (((9.0 * locals.var_ty__blk778_dn10) * assign26930_e37408) + (assign26930_e37405 * locals.var_tx__blk777_dn10))), (locals.var_ac2__blk810_dn11 + (((9.0 * locals.var_ty__blk778_dn11) * assign26930_e37408) + (assign26930_e37405 * locals.var_tx__blk777_dn11))), (locals.var_ac2__blk810_dn12 + (((9.0 * locals.var_ty__blk778_dn12) * assign26930_e37408) + (assign26930_e37405 * locals.var_tx__blk777_dn12))), (locals.var_ac2__blk810_dn17 + (((9.0 * locals.var_ty__blk778_dn17) * assign26930_e37408) + (assign26930_e37405 * locals.var_tx__blk777_dn17))),)
    } else {
        (locals.var_ac1__blk811, locals.var_ac1__blk811_dn0, locals.var_ac1__blk811_dn2, locals.var_ac1__blk811_dn6, locals.var_ac1__blk811_dn7, locals.var_ac1__blk811_dn10, locals.var_ac1__blk811_dn11, locals.var_ac1__blk811_dn12, locals.var_ac1__blk811_dn17,)
    }
};
        locals.var_ac1__blk811 = assign26930_e37412;
        locals.var_ac1__blk811_dn0 = assign26930_e37412_d_n0;
        locals.var_ac1__blk811_dn2 = assign26930_e37412_d_n2;
        locals.var_ac1__blk811_dn6 = assign26930_e37412_d_n6;
        locals.var_ac1__blk811_dn7 = assign26930_e37412_d_n7;
        locals.var_ac1__blk811_dn10 = assign26930_e37412_d_n10;
        locals.var_ac1__blk811_dn11 = assign26930_e37412_d_n11;
        locals.var_ac1__blk811_dn12 = assign26930_e37412_d_n12;
        locals.var_ac1__blk811_dn17 = assign26930_e37412_d_n17;
        locals.var_ac1__blk811_rv = 0.0;

        let (assign26940_e37424, assign26940_e37424_d_n0, assign26940_e37424_d_n2, assign26940_e37424_d_n6, assign26940_e37424_d_n7, assign26940_e37424_d_n10, assign26940_e37424_d_n11, assign26940_e37424_d_n12, assign26940_e37424_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26940_e37422: f64 = (locals.var_ac1__blk811).powf(0.3333333333333333);
        (assign26940_e37422, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn0)) } } else { (assign26940_e37422 * (0.3333333333333333 * (locals.var_ac1__blk811_dn0 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn2)) } } else { (assign26940_e37422 * (0.3333333333333333 * (locals.var_ac1__blk811_dn2 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn6)) } } else { (assign26940_e37422 * (0.3333333333333333 * (locals.var_ac1__blk811_dn6 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn7)) } } else { (assign26940_e37422 * (0.3333333333333333 * (locals.var_ac1__blk811_dn7 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn10)) } } else { (assign26940_e37422 * (0.3333333333333333 * (locals.var_ac1__blk811_dn10 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn11)) } } else { (assign26940_e37422 * (0.3333333333333333 * (locals.var_ac1__blk811_dn11 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn12)) } } else { (assign26940_e37422 * (0.3333333333333333 * (locals.var_ac1__blk811_dn12 / locals.var_ac1__blk811))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk811).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk811_dn17)) } } else { (assign26940_e37422 * (0.3333333333333333 * (locals.var_ac1__blk811_dn17 / locals.var_ac1__blk811))) },)
    } else {
        (locals.var_acd__blk812, locals.var_acd__blk812_dn0, locals.var_acd__blk812_dn2, locals.var_acd__blk812_dn6, locals.var_acd__blk812_dn7, locals.var_acd__blk812_dn10, locals.var_acd__blk812_dn11, locals.var_acd__blk812_dn12, locals.var_acd__blk812_dn17,)
    }
};
        locals.var_acd__blk812 = assign26940_e37424;
        locals.var_acd__blk812_dn0 = assign26940_e37424_d_n0;
        locals.var_acd__blk812_dn2 = assign26940_e37424_d_n2;
        locals.var_acd__blk812_dn6 = assign26940_e37424_d_n6;
        locals.var_acd__blk812_dn7 = assign26940_e37424_d_n7;
        locals.var_acd__blk812_dn10 = assign26940_e37424_d_n10;
        locals.var_acd__blk812_dn11 = assign26940_e37424_d_n11;
        locals.var_acd__blk812_dn12 = assign26940_e37424_d_n12;
        locals.var_acd__blk812_dn17 = assign26940_e37424_d_n17;
        locals.var_acd__blk812_rv = 0.0;

        let (assign26950_e37451, assign26950_e37451_d_n0, assign26950_e37451_d_n2, assign26950_e37451_d_n6, assign26950_e37451_d_n7, assign26950_e37451_d_n10, assign26950_e37451_d_n11, assign26950_e37451_d_n12, assign26950_e37451_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26950_e37433: f64 = (-4.0);
        let assign26950_e37435: f64 = (assign26950_e37433 * 1.414213562373095);
        let assign26950_e37438: f64 = (12.0 * locals.var_ty__blk778);
        let assign26950_e37439: f64 = (assign26950_e37435 - assign26950_e37438);
        let assign26950_e37442: f64 = (2.0 * locals.var_acd__blk812);
        let assign26950_e37443: f64 = (assign26950_e37439 + assign26950_e37442);
        let assign26950_e37446: f64 = (1.414213562373095 * locals.var_acd__blk812);
        let assign26950_e37448: f64 = (assign26950_e37446 * locals.var_acd__blk812);
        let assign26950_e37449: f64 = (assign26950_e37443 + assign26950_e37448);
        (assign26950_e37449, (((-(12.0 * locals.var_ty__blk778_dn0)) + (2.0 * locals.var_acd__blk812_dn0)) + (((1.414213562373095 * locals.var_acd__blk812_dn0) * locals.var_acd__blk812) + (assign26950_e37446 * locals.var_acd__blk812_dn0))), (((-(12.0 * locals.var_ty__blk778_dn2)) + (2.0 * locals.var_acd__blk812_dn2)) + (((1.414213562373095 * locals.var_acd__blk812_dn2) * locals.var_acd__blk812) + (assign26950_e37446 * locals.var_acd__blk812_dn2))), (((-(12.0 * locals.var_ty__blk778_dn6)) + (2.0 * locals.var_acd__blk812_dn6)) + (((1.414213562373095 * locals.var_acd__blk812_dn6) * locals.var_acd__blk812) + (assign26950_e37446 * locals.var_acd__blk812_dn6))), (((-(12.0 * locals.var_ty__blk778_dn7)) + (2.0 * locals.var_acd__blk812_dn7)) + (((1.414213562373095 * locals.var_acd__blk812_dn7) * locals.var_acd__blk812) + (assign26950_e37446 * locals.var_acd__blk812_dn7))), (((-(12.0 * locals.var_ty__blk778_dn10)) + (2.0 * locals.var_acd__blk812_dn10)) + (((1.414213562373095 * locals.var_acd__blk812_dn10) * locals.var_acd__blk812) + (assign26950_e37446 * locals.var_acd__blk812_dn10))), (((-(12.0 * locals.var_ty__blk778_dn11)) + (2.0 * locals.var_acd__blk812_dn11)) + (((1.414213562373095 * locals.var_acd__blk812_dn11) * locals.var_acd__blk812) + (assign26950_e37446 * locals.var_acd__blk812_dn11))), (((-(12.0 * locals.var_ty__blk778_dn12)) + (2.0 * locals.var_acd__blk812_dn12)) + (((1.414213562373095 * locals.var_acd__blk812_dn12) * locals.var_acd__blk812) + (assign26950_e37446 * locals.var_acd__blk812_dn12))), (((-(12.0 * locals.var_ty__blk778_dn17)) + (2.0 * locals.var_acd__blk812_dn17)) + (((1.414213562373095 * locals.var_acd__blk812_dn17) * locals.var_acd__blk812) + (assign26950_e37446 * locals.var_acd__blk812_dn17))),)
    } else {
        (locals.var_acn__blk813, locals.var_acn__blk813_dn0, locals.var_acn__blk813_dn2, locals.var_acn__blk813_dn6, locals.var_acn__blk813_dn7, locals.var_acn__blk813_dn10, locals.var_acn__blk813_dn11, locals.var_acn__blk813_dn12, locals.var_acn__blk813_dn17,)
    }
};
        locals.var_acn__blk813 = assign26950_e37451;
        locals.var_acn__blk813_dn0 = assign26950_e37451_d_n0;
        locals.var_acn__blk813_dn2 = assign26950_e37451_d_n2;
        locals.var_acn__blk813_dn6 = assign26950_e37451_d_n6;
        locals.var_acn__blk813_dn7 = assign26950_e37451_d_n7;
        locals.var_acn__blk813_dn10 = assign26950_e37451_d_n10;
        locals.var_acn__blk813_dn11 = assign26950_e37451_d_n11;
        locals.var_acn__blk813_dn12 = assign26950_e37451_d_n12;
        locals.var_acn__blk813_dn17 = assign26950_e37451_d_n17;
        locals.var_acn__blk813_rv = 0.0;

        let (assign26960_e37463, assign26960_e37463_d_n0, assign26960_e37463_d_n2, assign26960_e37463_d_n6, assign26960_e37463_d_n7, assign26960_e37463_d_n10, assign26960_e37463_d_n11, assign26960_e37463_d_n12, assign26960_e37463_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26960_e37461: f64 = (locals.var_acn__blk813 / locals.var_acd__blk812);
        (assign26960_e37461, (((locals.var_acn__blk813_dn0 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn0)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn2 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn2)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn6 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn6)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn7 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn7)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn10 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn10)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn11 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn11)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn12 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn12)) / (locals.var_acd__blk812 * locals.var_acd__blk812)), (((locals.var_acn__blk813_dn17 * locals.var_acd__blk812) - (locals.var_acn__blk813 * locals.var_acd__blk812_dn17)) / (locals.var_acd__blk812 * locals.var_acd__blk812)),)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
        locals.var_chi__blk814 = assign26960_e37463;
        locals.var_chi__blk814_dn0 = assign26960_e37463_d_n0;
        locals.var_chi__blk814_dn2 = assign26960_e37463_d_n2;
        locals.var_chi__blk814_dn6 = assign26960_e37463_d_n6;
        locals.var_chi__blk814_dn7 = assign26960_e37463_d_n7;
        locals.var_chi__blk814_dn10 = assign26960_e37463_d_n10;
        locals.var_chi__blk814_dn11 = assign26960_e37463_d_n11;
        locals.var_chi__blk814_dn12 = assign26960_e37463_d_n12;
        locals.var_chi__blk814_dn17 = assign26960_e37463_d_n17;
        locals.var_chi__blk814_rv = 0.0;

        let (assign26970_e37477, assign26970_e37477_d_n0, assign26970_e37477_d_n2, assign26970_e37477_d_n6, assign26970_e37477_d_n7, assign26970_e37477_d_n10, assign26970_e37477_d_n11, assign26970_e37477_d_n12, assign26970_e37477_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26970_e37473: f64 = (locals.var_chi__blk814 * locals.var_beta_inv);
        let assign26970_e37475: f64 = (assign26970_e37473 - locals.var_vxbgmtcl);
        (assign26970_e37475, ((locals.var_chi__blk814_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk814_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk814_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk814_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn7), (((locals.var_chi__blk814_dn10 * locals.var_beta_inv) + (locals.var_chi__blk814 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk814_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk814_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk814_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_psa__blk815, locals.var_psa__blk815_dn0, locals.var_psa__blk815_dn2, locals.var_psa__blk815_dn6, locals.var_psa__blk815_dn7, locals.var_psa__blk815_dn10, locals.var_psa__blk815_dn11, locals.var_psa__blk815_dn12, locals.var_psa__blk815_dn17,)
    }
};
        locals.var_psa__blk815 = assign26970_e37477;
        locals.var_psa__blk815_dn0 = assign26970_e37477_d_n0;
        locals.var_psa__blk815_dn2 = assign26970_e37477_d_n2;
        locals.var_psa__blk815_dn6 = assign26970_e37477_d_n6;
        locals.var_psa__blk815_dn7 = assign26970_e37477_d_n7;
        locals.var_psa__blk815_dn10 = assign26970_e37477_d_n10;
        locals.var_psa__blk815_dn11 = assign26970_e37477_d_n11;
        locals.var_psa__blk815_dn12 = assign26970_e37477_d_n12;
        locals.var_psa__blk815_dn17 = assign26970_e37477_d_n17;
        locals.var_psa__blk815_rv = 0.0;

        let (assign26980_e37489, assign26980_e37489_d_n0, assign26980_e37489_d_n2, assign26980_e37489_d_n6, assign26980_e37489_d_n7, assign26980_e37489_d_n10, assign26980_e37489_d_n11, assign26980_e37489_d_n12, assign26980_e37489_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26980_e37487: f64 = (locals.var_psa__blk815 + locals.var_vxbgmtcl);
        (assign26980_e37487, (locals.var_psa__blk815_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_psa__blk815_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_psa__blk815_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_psa__blk815_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_psa__blk815_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_psa__blk815_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_psa__blk815_dn12 + locals.var_vxbgmtcl_dn12), (locals.var_psa__blk815_dn17 + locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign26980_e37489;
        locals.var_t1__blk771_dn0 = assign26980_e37489_d_n0;
        locals.var_t1__blk771_dn2 = assign26980_e37489_d_n2;
        locals.var_t1__blk771_dn6 = assign26980_e37489_d_n6;
        locals.var_t1__blk771_dn7 = assign26980_e37489_d_n7;
        locals.var_t1__blk771_dn10 = assign26980_e37489_d_n10;
        locals.var_t1__blk771_dn11 = assign26980_e37489_d_n11;
        locals.var_t1__blk771_dn12 = assign26980_e37489_d_n12;
        locals.var_t1__blk771_dn17 = assign26980_e37489_d_n17;
        locals.var_t1__blk771_rv = 0.0;

        let (assign26990_e37501, assign26990_e37501_d_n0, assign26990_e37501_d_n2, assign26990_e37501_d_n6, assign26990_e37501_d_n7, assign26990_e37501_d_n10, assign26990_e37501_d_n11, assign26990_e37501_d_n12, assign26990_e37501_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign26990_e37499: f64 = (locals.var_t1__blk771 / locals.var_ps0_min__blk807);
        (assign26990_e37499, (((locals.var_t1__blk771_dn0 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn0)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn2 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn2)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn6 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn6)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn7 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn7)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn10 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn10)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn11 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn11)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn12 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn12)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)), (((locals.var_t1__blk771_dn17 * locals.var_ps0_min__blk807) - (locals.var_t1__blk771 * locals.var_ps0_min__blk807_dn17)) / (locals.var_ps0_min__blk807 * locals.var_ps0_min__blk807)),)
    } else {
        (locals.var_t2__blk772, locals.var_t2__blk772_dn0, locals.var_t2__blk772_dn2, locals.var_t2__blk772_dn6, locals.var_t2__blk772_dn7, locals.var_t2__blk772_dn10, locals.var_t2__blk772_dn11, locals.var_t2__blk772_dn12, locals.var_t2__blk772_dn17,)
    }
};
        locals.var_t2__blk772 = assign26990_e37501;
        locals.var_t2__blk772_dn0 = assign26990_e37501_d_n0;
        locals.var_t2__blk772_dn2 = assign26990_e37501_d_n2;
        locals.var_t2__blk772_dn6 = assign26990_e37501_d_n6;
        locals.var_t2__blk772_dn7 = assign26990_e37501_d_n7;
        locals.var_t2__blk772_dn10 = assign26990_e37501_d_n10;
        locals.var_t2__blk772_dn11 = assign26990_e37501_d_n11;
        locals.var_t2__blk772_dn12 = assign26990_e37501_d_n12;
        locals.var_t2__blk772_dn17 = assign26990_e37501_d_n17;
        locals.var_t2__blk772_rv = 0.0;

        let (assign27000_e37516, assign27000_e37516_d_n0, assign27000_e37516_d_n2, assign27000_e37516_d_n6, assign27000_e37516_d_n7, assign27000_e37516_d_n10, assign27000_e37516_d_n11, assign27000_e37516_d_n12, assign27000_e37516_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign27000_e37512: f64 = (locals.var_t2__blk772 * locals.var_t2__blk772);
        let assign27000_e37513: f64 = (1.0 + assign27000_e37512);
        let assign27000_e37514: f64 = (assign27000_e37513).sqrt();
        (assign27000_e37514, (((locals.var_t2__blk772_dn0 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn0)) / (2.0 * assign27000_e37514)), (((locals.var_t2__blk772_dn2 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn2)) / (2.0 * assign27000_e37514)), (((locals.var_t2__blk772_dn6 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn6)) / (2.0 * assign27000_e37514)), (((locals.var_t2__blk772_dn7 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn7)) / (2.0 * assign27000_e37514)), (((locals.var_t2__blk772_dn10 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn10)) / (2.0 * assign27000_e37514)), (((locals.var_t2__blk772_dn11 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn11)) / (2.0 * assign27000_e37514)), (((locals.var_t2__blk772_dn12 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn12)) / (2.0 * assign27000_e37514)), (((locals.var_t2__blk772_dn17 * locals.var_t2__blk772) + (locals.var_t2__blk772 * locals.var_t2__blk772_dn17)) / (2.0 * assign27000_e37514)),)
    } else {
        (locals.var_t3__blk773, locals.var_t3__blk773_dn0, locals.var_t3__blk773_dn2, locals.var_t3__blk773_dn6, locals.var_t3__blk773_dn7, locals.var_t3__blk773_dn10, locals.var_t3__blk773_dn11, locals.var_t3__blk773_dn12, locals.var_t3__blk773_dn17,)
    }
};
        locals.var_t3__blk773 = assign27000_e37516;
        locals.var_t3__blk773_dn0 = assign27000_e37516_d_n0;
        locals.var_t3__blk773_dn2 = assign27000_e37516_d_n2;
        locals.var_t3__blk773_dn6 = assign27000_e37516_d_n6;
        locals.var_t3__blk773_dn7 = assign27000_e37516_d_n7;
        locals.var_t3__blk773_dn10 = assign27000_e37516_d_n10;
        locals.var_t3__blk773_dn11 = assign27000_e37516_d_n11;
        locals.var_t3__blk773_dn12 = assign27000_e37516_d_n12;
        locals.var_t3__blk773_dn17 = assign27000_e37516_d_n17;
        locals.var_t3__blk773_rv = 0.0;

        let (assign27010_e37530, assign27010_e37530_d_n0, assign27010_e37530_d_n2, assign27010_e37530_d_n6, assign27010_e37530_d_n7, assign27010_e37530_d_n10, assign27010_e37530_d_n11, assign27010_e37530_d_n12, assign27010_e37530_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign27010_e37526: f64 = (locals.var_t1__blk771 / locals.var_t3__blk773);
        let assign27010_e37528: f64 = (assign27010_e37526 - locals.var_vxbgmtcl);
        (assign27010_e37528, ((((locals.var_t1__blk771_dn0 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn0)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn0), ((((locals.var_t1__blk771_dn2 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn2)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn2), ((((locals.var_t1__blk771_dn6 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn6)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn6), ((((locals.var_t1__blk771_dn7 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn7)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn7), ((((locals.var_t1__blk771_dn10 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn10)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn10), ((((locals.var_t1__blk771_dn11 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn11)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn11), ((((locals.var_t1__blk771_dn12 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn12)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn12), ((((locals.var_t1__blk771_dn17 * locals.var_t3__blk773) - (locals.var_t1__blk771 * locals.var_t3__blk773_dn17)) / (locals.var_t3__blk773 * locals.var_t3__blk773)) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
        locals.var_ps0ld = assign27010_e37530;
        locals.var_ps0ld_dn0 = assign27010_e37530_d_n0;
        locals.var_ps0ld_dn2 = assign27010_e37530_d_n2;
        locals.var_ps0ld_dn6 = assign27010_e37530_d_n6;
        locals.var_ps0ld_dn7 = assign27010_e37530_d_n7;
        locals.var_ps0ld_dn10 = assign27010_e37530_d_n10;
        locals.var_ps0ld_dn11 = assign27010_e37530_d_n11;
        locals.var_ps0ld_dn12 = assign27010_e37530_d_n12;
        locals.var_ps0ld_dn17 = assign27010_e37530_d_n17;
        locals.var_ps0ld_rv = 0.0;

        let (assign27020_e37542, assign27020_e37542_d_n0, assign27020_e37542_d_n2, assign27020_e37542_d_n6, assign27020_e37542_d_n7, assign27020_e37542_d_n10, assign27020_e37542_d_n11, assign27020_e37542_d_n12, assign27020_e37542_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign27020_e37540: f64 = (locals.var_vgpld - locals.var_ps0ld);
        (assign27020_e37540, (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0), (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2), (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6), (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7), (locals.var_vgpld_dn10 - locals.var_ps0ld_dn10), (locals.var_vgpld_dn11 - locals.var_ps0ld_dn11), (locals.var_vgpld_dn12 - locals.var_ps0ld_dn12), (locals.var_vgpld_dn17 - locals.var_ps0ld_dn17),)
    } else {
        (locals.var_t2__blk772, locals.var_t2__blk772_dn0, locals.var_t2__blk772_dn2, locals.var_t2__blk772_dn6, locals.var_t2__blk772_dn7, locals.var_t2__blk772_dn10, locals.var_t2__blk772_dn11, locals.var_t2__blk772_dn12, locals.var_t2__blk772_dn17,)
    }
};
        locals.var_t2__blk772 = assign27020_e37542;
        locals.var_t2__blk772_dn0 = assign27020_e37542_d_n0;
        locals.var_t2__blk772_dn2 = assign27020_e37542_d_n2;
        locals.var_t2__blk772_dn6 = assign27020_e37542_d_n6;
        locals.var_t2__blk772_dn7 = assign27020_e37542_d_n7;
        locals.var_t2__blk772_dn10 = assign27020_e37542_d_n10;
        locals.var_t2__blk772_dn11 = assign27020_e37542_d_n11;
        locals.var_t2__blk772_dn12 = assign27020_e37542_d_n12;
        locals.var_t2__blk772_dn17 = assign27020_e37542_d_n17;
        locals.var_t2__blk772_rv = 0.0;

        let (assign27030_e37554, assign27030_e37554_d_n0, assign27030_e37554_d_n2, assign27030_e37554_d_n6, assign27030_e37554_d_n7, assign27030_e37554_d_n10, assign27030_e37554_d_n11, assign27030_e37554_d_n12, assign27030_e37554_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        let assign27030_e37552: f64 = (locals.var_cox0 * locals.var_t2__blk772);
        (assign27030_e37552, (locals.var_cox0 * locals.var_t2__blk772_dn0), (locals.var_cox0 * locals.var_t2__blk772_dn2), (locals.var_cox0 * locals.var_t2__blk772_dn6), (locals.var_cox0 * locals.var_t2__blk772_dn7), (locals.var_cox0 * locals.var_t2__blk772_dn10), (locals.var_cox0 * locals.var_t2__blk772_dn11), (locals.var_cox0 * locals.var_t2__blk772_dn12), (locals.var_cox0 * locals.var_t2__blk772_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign27030_e37554;
        locals.var_qsuld_dn0 = assign27030_e37554_d_n0;
        locals.var_qsuld_dn2 = assign27030_e37554_d_n2;
        locals.var_qsuld_dn6 = assign27030_e37554_d_n6;
        locals.var_qsuld_dn7 = assign27030_e37554_d_n7;
        locals.var_qsuld_dn10 = assign27030_e37554_d_n10;
        locals.var_qsuld_dn11 = assign27030_e37554_d_n11;
        locals.var_qsuld_dn12 = assign27030_e37554_d_n12;
        locals.var_qsuld_dn17 = assign27030_e37554_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign27040_e37564, assign27040_e37564_d_n0, assign27040_e37564_d_n2, assign27040_e37564_d_n6, assign27040_e37564_d_n7, assign27040_e37564_d_n10, assign27040_e37564_d_n11, assign27040_e37564_d_n12, assign27040_e37564_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign27040_e37564;
        locals.var_qbuld_dn0 = assign27040_e37564_d_n0;
        locals.var_qbuld_dn2 = assign27040_e37564_d_n2;
        locals.var_qbuld_dn6 = assign27040_e37564_d_n6;
        locals.var_qbuld_dn7 = assign27040_e37564_d_n7;
        locals.var_qbuld_dn10 = assign27040_e37564_d_n10;
        locals.var_qbuld_dn11 = assign27040_e37564_d_n11;
        locals.var_qbuld_dn12 = assign27040_e37564_d_n12;
        locals.var_qbuld_dn17 = assign27040_e37564_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign27060_e37586, assign27060_e37586_d_n0, assign27060_e37586_d_n2, assign27060_e37586_d_n6, assign27060_e37586_d_n7, assign27060_e37586_d_n10, assign27060_e37586_d_n11, assign27060_e37586_d_n12, assign27060_e37586_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        (3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
        locals.var_chi__blk814 = assign27060_e37586;
        locals.var_chi__blk814_dn0 = assign27060_e37586_d_n0;
        locals.var_chi__blk814_dn2 = assign27060_e37586_d_n2;
        locals.var_chi__blk814_dn6 = assign27060_e37586_d_n6;
        locals.var_chi__blk814_dn7 = assign27060_e37586_d_n7;
        locals.var_chi__blk814_dn10 = assign27060_e37586_d_n10;
        locals.var_chi__blk814_dn11 = assign27060_e37586_d_n11;
        locals.var_chi__blk814_dn12 = assign27060_e37586_d_n12;
        locals.var_chi__blk814_dn17 = assign27060_e37586_d_n17;
        locals.var_chi__blk814_rv = 0.0;

        let (assign27070_e37601, assign27070_e37601_d_n0, assign27070_e37601_d_n2, assign27070_e37601_d_n6, assign27070_e37601_d_n7, assign27070_e37601_d_n10, assign27070_e37601_d_n11, assign27070_e37601_d_n12, assign27070_e37601_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27070_e37597: f64 = (locals.var_chi__blk814 / locals.var_beta);
        let assign27070_e37599: f64 = (assign27070_e37597 - locals.var_vxbgmtcl);
        (assign27070_e37599, ((locals.var_chi__blk814_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk814_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk814_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk814_dn7 / locals.var_beta) - locals.var_vxbgmtcl_dn7), ((((locals.var_chi__blk814_dn10 * locals.var_beta) - (locals.var_chi__blk814 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk814_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk814_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk814_dn17 / locals.var_beta) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0_inia__blk817, locals.var_ps0_inia__blk817_dn0, locals.var_ps0_inia__blk817_dn2, locals.var_ps0_inia__blk817_dn6, locals.var_ps0_inia__blk817_dn7, locals.var_ps0_inia__blk817_dn10, locals.var_ps0_inia__blk817_dn11, locals.var_ps0_inia__blk817_dn12, locals.var_ps0_inia__blk817_dn17,)
    }
};
        locals.var_ps0_inia__blk817 = assign27070_e37601;
        locals.var_ps0_inia__blk817_dn0 = assign27070_e37601_d_n0;
        locals.var_ps0_inia__blk817_dn2 = assign27070_e37601_d_n2;
        locals.var_ps0_inia__blk817_dn6 = assign27070_e37601_d_n6;
        locals.var_ps0_inia__blk817_dn7 = assign27070_e37601_d_n7;
        locals.var_ps0_inia__blk817_dn10 = assign27070_e37601_d_n10;
        locals.var_ps0_inia__blk817_dn11 = assign27070_e37601_d_n11;
        locals.var_ps0_inia__blk817_dn12 = assign27070_e37601_d_n12;
        locals.var_ps0_inia__blk817_dn17 = assign27070_e37601_d_n17;
        locals.var_ps0_inia__blk817_rv = 0.0;

        let (assign27080_e37614, assign27080_e37614_d_n0, assign27080_e37614_d_n2, assign27080_e37614_d_n6, assign27080_e37614_d_n7, assign27080_e37614_d_n10, assign27080_e37614_d_n11, assign27080_e37614_d_n12, assign27080_e37614_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27080_e37611: f64 = (-locals.var_chi__blk814);
        let assign27080_e37612: f64 = (assign27080_e37611).exp();
        (assign27080_e37612, (assign27080_e37612 * (-locals.var_chi__blk814_dn0)), (assign27080_e37612 * (-locals.var_chi__blk814_dn2)), (assign27080_e37612 * (-locals.var_chi__blk814_dn6)), (assign27080_e37612 * (-locals.var_chi__blk814_dn7)), (assign27080_e37612 * (-locals.var_chi__blk814_dn10)), (assign27080_e37612 * (-locals.var_chi__blk814_dn11)), (assign27080_e37612 * (-locals.var_chi__blk814_dn12)), (assign27080_e37612 * (-locals.var_chi__blk814_dn17)),)
    } else {
        (locals.var_ty__blk778, locals.var_ty__blk778_dn0, locals.var_ty__blk778_dn2, locals.var_ty__blk778_dn6, locals.var_ty__blk778_dn7, locals.var_ty__blk778_dn10, locals.var_ty__blk778_dn11, locals.var_ty__blk778_dn12, locals.var_ty__blk778_dn17,)
    }
};
        locals.var_ty__blk778 = assign27080_e37614;
        locals.var_ty__blk778_dn0 = assign27080_e37614_d_n0;
        locals.var_ty__blk778_dn2 = assign27080_e37614_d_n2;
        locals.var_ty__blk778_dn6 = assign27080_e37614_d_n6;
        locals.var_ty__blk778_dn7 = assign27080_e37614_d_n7;
        locals.var_ty__blk778_dn10 = assign27080_e37614_d_n10;
        locals.var_ty__blk778_dn11 = assign27080_e37614_d_n11;
        locals.var_ty__blk778_dn12 = assign27080_e37614_d_n12;
        locals.var_ty__blk778_dn17 = assign27080_e37614_d_n17;
        locals.var_ty__blk778_rv = 0.0;

        let (assign27090_e37641, assign27090_e37641_d_n0, assign27090_e37641_d_n2, assign27090_e37641_d_n6, assign27090_e37641_d_n7, assign27090_e37641_d_n10, assign27090_e37641_d_n11, assign27090_e37641_d_n12, assign27090_e37641_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27090_e37628: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign27090_e37629: f64 = (locals.var_beta * assign27090_e37628);
        let assign27090_e37631: f64 = (assign27090_e37629 - 1.0);
        let assign27090_e37633: f64 = (assign27090_e37631 + locals.var_ty__blk778);
        let assign27090_e37634: f64 = (4.0 * assign27090_e37633);
        let assign27090_e37637: f64 = (locals.var_fac1p2__blk801 * locals.var_beta2);
        let assign27090_e37638: f64 = (assign27090_e37634 / assign27090_e37637);
        let assign27090_e37639: f64 = (1.0 + assign27090_e37638);
        (assign27090_e37639, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + locals.var_ty__blk778_dn0)) * assign27090_e37637) - (assign27090_e37634 * (locals.var_fac1p2__blk801_dn0 * locals.var_beta2))) / (assign27090_e37637 * assign27090_e37637)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + locals.var_ty__blk778_dn2)) * assign27090_e37637) - (assign27090_e37634 * (locals.var_fac1p2__blk801_dn2 * locals.var_beta2))) / (assign27090_e37637 * assign27090_e37637)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) + locals.var_ty__blk778_dn6)) * assign27090_e37637) - (assign27090_e37634 * (locals.var_fac1p2__blk801_dn6 * locals.var_beta2))) / (assign27090_e37637 * assign27090_e37637)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) + locals.var_ty__blk778_dn7)) * assign27090_e37637) - (assign27090_e37634 * (locals.var_fac1p2__blk801_dn7 * locals.var_beta2))) / (assign27090_e37637 * assign27090_e37637)), ((((4.0 * (((locals.var_beta_dn10 * assign27090_e37628) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))) + locals.var_ty__blk778_dn10)) * assign27090_e37637) - (assign27090_e37634 * ((locals.var_fac1p2__blk801_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk801 * locals.var_beta2_dn10)))) / (assign27090_e37637 * assign27090_e37637)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) + locals.var_ty__blk778_dn11)) * assign27090_e37637) - (assign27090_e37634 * (locals.var_fac1p2__blk801_dn11 * locals.var_beta2))) / (assign27090_e37637 * assign27090_e37637)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) + locals.var_ty__blk778_dn12)) * assign27090_e37637) - (assign27090_e37634 * (locals.var_fac1p2__blk801_dn12 * locals.var_beta2))) / (assign27090_e37637 * assign27090_e37637)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) + locals.var_ty__blk778_dn17)) * assign27090_e37637) - (assign27090_e37634 * (locals.var_fac1p2__blk801_dn17 * locals.var_beta2))) / (assign27090_e37637 * assign27090_e37637)),)
    } else {
        (locals.var_tx__blk777, locals.var_tx__blk777_dn0, locals.var_tx__blk777_dn2, locals.var_tx__blk777_dn6, locals.var_tx__blk777_dn7, locals.var_tx__blk777_dn10, locals.var_tx__blk777_dn11, locals.var_tx__blk777_dn12, locals.var_tx__blk777_dn17,)
    }
};
        locals.var_tx__blk777 = assign27090_e37641;
        locals.var_tx__blk777_dn0 = assign27090_e37641_d_n0;
        locals.var_tx__blk777_dn2 = assign27090_e37641_d_n2;
        locals.var_tx__blk777_dn6 = assign27090_e37641_d_n6;
        locals.var_tx__blk777_dn7 = assign27090_e37641_d_n7;
        locals.var_tx__blk777_dn10 = assign27090_e37641_d_n10;
        locals.var_tx__blk777_dn11 = assign27090_e37641_d_n11;
        locals.var_tx__blk777_dn12 = assign27090_e37641_d_n12;
        locals.var_tx__blk777_dn17 = assign27090_e37641_d_n17;
        locals.var_tx__blk777_rv = 0.0;

        let assign27100_e37645: f64 = (10.0 * 2.220446049250313e-16);
        let assign27100_e37646: f64 = if locals.var_tx__blk777 < assign27100_e37645 { 1.0 } else { 0.0 };
        locals.var_guard873 = assign27100_e37646;
        locals.var_guard873_rv = 0.0;

        let (assign27110_e37661, assign27110_e37661_d_n0, assign27110_e37661_d_n2, assign27110_e37661_d_n6, assign27110_e37661_d_n7, assign27110_e37661_d_n10, assign27110_e37661_d_n11, assign27110_e37661_d_n12, assign27110_e37661_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard873 != 0.0)) {
        let assign27110_e37659: f64 = (10.0 * 2.220446049250313e-16);
        (assign27110_e37659, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk777, locals.var_tx__blk777_dn0, locals.var_tx__blk777_dn2, locals.var_tx__blk777_dn6, locals.var_tx__blk777_dn7, locals.var_tx__blk777_dn10, locals.var_tx__blk777_dn11, locals.var_tx__blk777_dn12, locals.var_tx__blk777_dn17,)
    }
};
        locals.var_tx__blk777 = assign27110_e37661;
        locals.var_tx__blk777_dn0 = assign27110_e37661_d_n0;
        locals.var_tx__blk777_dn2 = assign27110_e37661_d_n2;
        locals.var_tx__blk777_dn6 = assign27110_e37661_d_n6;
        locals.var_tx__blk777_dn7 = assign27110_e37661_d_n7;
        locals.var_tx__blk777_dn10 = assign27110_e37661_d_n10;
        locals.var_tx__blk777_dn11 = assign27110_e37661_d_n11;
        locals.var_tx__blk777_dn12 = assign27110_e37661_d_n12;
        locals.var_tx__blk777_dn17 = assign27110_e37661_d_n17;
        locals.var_tx__blk777_rv = 0.0;

        let (assign27120_e37683, assign27120_e37683_d_n0, assign27120_e37683_d_n2, assign27120_e37683_d_n6, assign27120_e37683_d_n7, assign27120_e37683_d_n10, assign27120_e37683_d_n11, assign27120_e37683_d_n12, assign27120_e37683_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27120_e37673: f64 = (locals.var_fac1p2__blk801 * locals.var_beta);
        let assign27120_e37675: f64 = (assign27120_e37673 / 2.0);
        let assign27120_e37678: f64 = (locals.var_tx__blk777).sqrt();
        let assign27120_e37679: f64 = (1.0 - assign27120_e37678);
        let assign27120_e37680: f64 = (assign27120_e37675 * assign27120_e37679);
        let assign27120_e37681: f64 = (locals.var_vgpld + assign27120_e37680);
        (assign27120_e37681, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2__blk801_dn0 * locals.var_beta) / 2.0) * assign27120_e37679) + (assign27120_e37675 * (-(locals.var_tx__blk777_dn0 / (2.0 * assign27120_e37678)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2__blk801_dn2 * locals.var_beta) / 2.0) * assign27120_e37679) + (assign27120_e37675 * (-(locals.var_tx__blk777_dn2 / (2.0 * assign27120_e37678)))))), (locals.var_vgpld_dn6 + ((((locals.var_fac1p2__blk801_dn6 * locals.var_beta) / 2.0) * assign27120_e37679) + (assign27120_e37675 * (-(locals.var_tx__blk777_dn6 / (2.0 * assign27120_e37678)))))), (locals.var_vgpld_dn7 + ((((locals.var_fac1p2__blk801_dn7 * locals.var_beta) / 2.0) * assign27120_e37679) + (assign27120_e37675 * (-(locals.var_tx__blk777_dn7 / (2.0 * assign27120_e37678)))))), (locals.var_vgpld_dn10 + (((((locals.var_fac1p2__blk801_dn10 * locals.var_beta) + (locals.var_fac1p2__blk801 * locals.var_beta_dn10)) / 2.0) * assign27120_e37679) + (assign27120_e37675 * (-(locals.var_tx__blk777_dn10 / (2.0 * assign27120_e37678)))))), (locals.var_vgpld_dn11 + ((((locals.var_fac1p2__blk801_dn11 * locals.var_beta) / 2.0) * assign27120_e37679) + (assign27120_e37675 * (-(locals.var_tx__blk777_dn11 / (2.0 * assign27120_e37678)))))), (locals.var_vgpld_dn12 + ((((locals.var_fac1p2__blk801_dn12 * locals.var_beta) / 2.0) * assign27120_e37679) + (assign27120_e37675 * (-(locals.var_tx__blk777_dn12 / (2.0 * assign27120_e37678)))))), (locals.var_vgpld_dn17 + ((((locals.var_fac1p2__blk801_dn17 * locals.var_beta) / 2.0) * assign27120_e37679) + (assign27120_e37675 * (-(locals.var_tx__blk777_dn17 / (2.0 * assign27120_e37678)))))),)
    } else {
        (locals.var_ps0_inia__blk817, locals.var_ps0_inia__blk817_dn0, locals.var_ps0_inia__blk817_dn2, locals.var_ps0_inia__blk817_dn6, locals.var_ps0_inia__blk817_dn7, locals.var_ps0_inia__blk817_dn10, locals.var_ps0_inia__blk817_dn11, locals.var_ps0_inia__blk817_dn12, locals.var_ps0_inia__blk817_dn17,)
    }
};
        locals.var_ps0_inia__blk817 = assign27120_e37683;
        locals.var_ps0_inia__blk817_dn0 = assign27120_e37683_d_n0;
        locals.var_ps0_inia__blk817_dn2 = assign27120_e37683_d_n2;
        locals.var_ps0_inia__blk817_dn6 = assign27120_e37683_d_n6;
        locals.var_ps0_inia__blk817_dn7 = assign27120_e37683_d_n7;
        locals.var_ps0_inia__blk817_dn10 = assign27120_e37683_d_n10;
        locals.var_ps0_inia__blk817_dn11 = assign27120_e37683_d_n11;
        locals.var_ps0_inia__blk817_dn12 = assign27120_e37683_d_n12;
        locals.var_ps0_inia__blk817_dn17 = assign27120_e37683_d_n17;
        locals.var_ps0_inia__blk817_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_97(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27130_e37698, assign27130_e37698_d_n0, assign27130_e37698_d_n2, assign27130_e37698_d_n6, assign27130_e37698_d_n7, assign27130_e37698_d_n10, assign27130_e37698_d_n11, assign27130_e37698_d_n12, assign27130_e37698_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27130_e37695: f64 = (locals.var_ps0_inia__blk817 + locals.var_vxbgmtcl);
        let assign27130_e37696: f64 = (locals.var_beta * assign27130_e37695);
        (assign27130_e37696, (locals.var_beta * (locals.var_ps0_inia__blk817_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign27130_e37695) + (locals.var_beta * (locals.var_ps0_inia__blk817_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk817_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
        locals.var_chi__blk814 = assign27130_e37698;
        locals.var_chi__blk814_dn0 = assign27130_e37698_d_n0;
        locals.var_chi__blk814_dn2 = assign27130_e37698_d_n2;
        locals.var_chi__blk814_dn6 = assign27130_e37698_d_n6;
        locals.var_chi__blk814_dn7 = assign27130_e37698_d_n7;
        locals.var_chi__blk814_dn10 = assign27130_e37698_d_n10;
        locals.var_chi__blk814_dn11 = assign27130_e37698_d_n11;
        locals.var_chi__blk814_dn12 = assign27130_e37698_d_n12;
        locals.var_chi__blk814_dn17 = assign27130_e37698_d_n17;
        locals.var_chi__blk814_rv = 0.0;

        let (assign27140_e37711, assign27140_e37711_d_n0, assign27140_e37711_d_n2, assign27140_e37711_d_n6, assign27140_e37711_d_n7, assign27140_e37711_d_n10, assign27140_e37711_d_n11, assign27140_e37711_d_n12, assign27140_e37711_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27140_e37708: f64 = (-locals.var_chi__blk814);
        let assign27140_e37709: f64 = (assign27140_e37708).exp();
        (assign27140_e37709, (assign27140_e37709 * (-locals.var_chi__blk814_dn0)), (assign27140_e37709 * (-locals.var_chi__blk814_dn2)), (assign27140_e37709 * (-locals.var_chi__blk814_dn6)), (assign27140_e37709 * (-locals.var_chi__blk814_dn7)), (assign27140_e37709 * (-locals.var_chi__blk814_dn10)), (assign27140_e37709 * (-locals.var_chi__blk814_dn11)), (assign27140_e37709 * (-locals.var_chi__blk814_dn12)), (assign27140_e37709 * (-locals.var_chi__blk814_dn17)),)
    } else {
        (locals.var_ty__blk778, locals.var_ty__blk778_dn0, locals.var_ty__blk778_dn2, locals.var_ty__blk778_dn6, locals.var_ty__blk778_dn7, locals.var_ty__blk778_dn10, locals.var_ty__blk778_dn11, locals.var_ty__blk778_dn12, locals.var_ty__blk778_dn17,)
    }
};
        locals.var_ty__blk778 = assign27140_e37711;
        locals.var_ty__blk778_dn0 = assign27140_e37711_d_n0;
        locals.var_ty__blk778_dn2 = assign27140_e37711_d_n2;
        locals.var_ty__blk778_dn6 = assign27140_e37711_d_n6;
        locals.var_ty__blk778_dn7 = assign27140_e37711_d_n7;
        locals.var_ty__blk778_dn10 = assign27140_e37711_d_n10;
        locals.var_ty__blk778_dn11 = assign27140_e37711_d_n11;
        locals.var_ty__blk778_dn12 = assign27140_e37711_d_n12;
        locals.var_ty__blk778_dn17 = assign27140_e37711_d_n17;
        locals.var_ty__blk778_rv = 0.0;

        let (assign27150_e37738, assign27150_e37738_d_n0, assign27150_e37738_d_n2, assign27150_e37738_d_n6, assign27150_e37738_d_n7, assign27150_e37738_d_n10, assign27150_e37738_d_n11, assign27150_e37738_d_n12, assign27150_e37738_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27150_e37725: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign27150_e37726: f64 = (locals.var_beta * assign27150_e37725);
        let assign27150_e37728: f64 = (assign27150_e37726 - 1.0);
        let assign27150_e37730: f64 = (assign27150_e37728 + locals.var_ty__blk778);
        let assign27150_e37731: f64 = (4.0 * assign27150_e37730);
        let assign27150_e37734: f64 = (locals.var_fac1p2__blk801 * locals.var_beta2);
        let assign27150_e37735: f64 = (assign27150_e37731 / assign27150_e37734);
        let assign27150_e37736: f64 = (1.0 + assign27150_e37735);
        (assign27150_e37736, ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) + locals.var_ty__blk778_dn0)) * assign27150_e37734) - (assign27150_e37731 * (locals.var_fac1p2__blk801_dn0 * locals.var_beta2))) / (assign27150_e37734 * assign27150_e37734)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) + locals.var_ty__blk778_dn2)) * assign27150_e37734) - (assign27150_e37731 * (locals.var_fac1p2__blk801_dn2 * locals.var_beta2))) / (assign27150_e37734 * assign27150_e37734)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) + locals.var_ty__blk778_dn6)) * assign27150_e37734) - (assign27150_e37731 * (locals.var_fac1p2__blk801_dn6 * locals.var_beta2))) / (assign27150_e37734 * assign27150_e37734)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) + locals.var_ty__blk778_dn7)) * assign27150_e37734) - (assign27150_e37731 * (locals.var_fac1p2__blk801_dn7 * locals.var_beta2))) / (assign27150_e37734 * assign27150_e37734)), ((((4.0 * (((locals.var_beta_dn10 * assign27150_e37725) + (locals.var_beta * (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10))) + locals.var_ty__blk778_dn10)) * assign27150_e37734) - (assign27150_e37731 * ((locals.var_fac1p2__blk801_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk801 * locals.var_beta2_dn10)))) / (assign27150_e37734 * assign27150_e37734)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) + locals.var_ty__blk778_dn11)) * assign27150_e37734) - (assign27150_e37731 * (locals.var_fac1p2__blk801_dn11 * locals.var_beta2))) / (assign27150_e37734 * assign27150_e37734)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) + locals.var_ty__blk778_dn12)) * assign27150_e37734) - (assign27150_e37731 * (locals.var_fac1p2__blk801_dn12 * locals.var_beta2))) / (assign27150_e37734 * assign27150_e37734)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) + locals.var_ty__blk778_dn17)) * assign27150_e37734) - (assign27150_e37731 * (locals.var_fac1p2__blk801_dn17 * locals.var_beta2))) / (assign27150_e37734 * assign27150_e37734)),)
    } else {
        (locals.var_tx__blk777, locals.var_tx__blk777_dn0, locals.var_tx__blk777_dn2, locals.var_tx__blk777_dn6, locals.var_tx__blk777_dn7, locals.var_tx__blk777_dn10, locals.var_tx__blk777_dn11, locals.var_tx__blk777_dn12, locals.var_tx__blk777_dn17,)
    }
};
        locals.var_tx__blk777 = assign27150_e37738;
        locals.var_tx__blk777_dn0 = assign27150_e37738_d_n0;
        locals.var_tx__blk777_dn2 = assign27150_e37738_d_n2;
        locals.var_tx__blk777_dn6 = assign27150_e37738_d_n6;
        locals.var_tx__blk777_dn7 = assign27150_e37738_d_n7;
        locals.var_tx__blk777_dn10 = assign27150_e37738_d_n10;
        locals.var_tx__blk777_dn11 = assign27150_e37738_d_n11;
        locals.var_tx__blk777_dn12 = assign27150_e37738_d_n12;
        locals.var_tx__blk777_dn17 = assign27150_e37738_d_n17;
        locals.var_tx__blk777_rv = 0.0;

        let assign27160_e37742: f64 = (10.0 * 2.220446049250313e-16);
        let assign27160_e37743: f64 = if locals.var_tx__blk777 < assign27160_e37742 { 1.0 } else { 0.0 };
        locals.var_guard874 = assign27160_e37743;
        locals.var_guard874_rv = 0.0;

        let (assign27170_e37758, assign27170_e37758_d_n0, assign27170_e37758_d_n2, assign27170_e37758_d_n6, assign27170_e37758_d_n7, assign27170_e37758_d_n10, assign27170_e37758_d_n11, assign27170_e37758_d_n12, assign27170_e37758_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard874 != 0.0)) {
        let assign27170_e37756: f64 = (10.0 * 2.220446049250313e-16);
        (assign27170_e37756, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk777, locals.var_tx__blk777_dn0, locals.var_tx__blk777_dn2, locals.var_tx__blk777_dn6, locals.var_tx__blk777_dn7, locals.var_tx__blk777_dn10, locals.var_tx__blk777_dn11, locals.var_tx__blk777_dn12, locals.var_tx__blk777_dn17,)
    }
};
        locals.var_tx__blk777 = assign27170_e37758;
        locals.var_tx__blk777_dn0 = assign27170_e37758_d_n0;
        locals.var_tx__blk777_dn2 = assign27170_e37758_d_n2;
        locals.var_tx__blk777_dn6 = assign27170_e37758_d_n6;
        locals.var_tx__blk777_dn7 = assign27170_e37758_d_n7;
        locals.var_tx__blk777_dn10 = assign27170_e37758_d_n10;
        locals.var_tx__blk777_dn11 = assign27170_e37758_d_n11;
        locals.var_tx__blk777_dn12 = assign27170_e37758_d_n12;
        locals.var_tx__blk777_dn17 = assign27170_e37758_d_n17;
        locals.var_tx__blk777_rv = 0.0;

        let (assign27180_e37780, assign27180_e37780_d_n0, assign27180_e37780_d_n2, assign27180_e37780_d_n6, assign27180_e37780_d_n7, assign27180_e37780_d_n10, assign27180_e37780_d_n11, assign27180_e37780_d_n12, assign27180_e37780_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27180_e37770: f64 = (locals.var_fac1p2__blk801 * locals.var_beta);
        let assign27180_e37772: f64 = (assign27180_e37770 / 2.0);
        let assign27180_e37775: f64 = (locals.var_tx__blk777).sqrt();
        let assign27180_e37776: f64 = (1.0 - assign27180_e37775);
        let assign27180_e37777: f64 = (assign27180_e37772 * assign27180_e37776);
        let assign27180_e37778: f64 = (locals.var_vgpld + assign27180_e37777);
        (assign27180_e37778, (locals.var_vgpld_dn0 + ((((locals.var_fac1p2__blk801_dn0 * locals.var_beta) / 2.0) * assign27180_e37776) + (assign27180_e37772 * (-(locals.var_tx__blk777_dn0 / (2.0 * assign27180_e37775)))))), (locals.var_vgpld_dn2 + ((((locals.var_fac1p2__blk801_dn2 * locals.var_beta) / 2.0) * assign27180_e37776) + (assign27180_e37772 * (-(locals.var_tx__blk777_dn2 / (2.0 * assign27180_e37775)))))), (locals.var_vgpld_dn6 + ((((locals.var_fac1p2__blk801_dn6 * locals.var_beta) / 2.0) * assign27180_e37776) + (assign27180_e37772 * (-(locals.var_tx__blk777_dn6 / (2.0 * assign27180_e37775)))))), (locals.var_vgpld_dn7 + ((((locals.var_fac1p2__blk801_dn7 * locals.var_beta) / 2.0) * assign27180_e37776) + (assign27180_e37772 * (-(locals.var_tx__blk777_dn7 / (2.0 * assign27180_e37775)))))), (locals.var_vgpld_dn10 + (((((locals.var_fac1p2__blk801_dn10 * locals.var_beta) + (locals.var_fac1p2__blk801 * locals.var_beta_dn10)) / 2.0) * assign27180_e37776) + (assign27180_e37772 * (-(locals.var_tx__blk777_dn10 / (2.0 * assign27180_e37775)))))), (locals.var_vgpld_dn11 + ((((locals.var_fac1p2__blk801_dn11 * locals.var_beta) / 2.0) * assign27180_e37776) + (assign27180_e37772 * (-(locals.var_tx__blk777_dn11 / (2.0 * assign27180_e37775)))))), (locals.var_vgpld_dn12 + ((((locals.var_fac1p2__blk801_dn12 * locals.var_beta) / 2.0) * assign27180_e37776) + (assign27180_e37772 * (-(locals.var_tx__blk777_dn12 / (2.0 * assign27180_e37775)))))), (locals.var_vgpld_dn17 + ((((locals.var_fac1p2__blk801_dn17 * locals.var_beta) / 2.0) * assign27180_e37776) + (assign27180_e37772 * (-(locals.var_tx__blk777_dn17 / (2.0 * assign27180_e37775)))))),)
    } else {
        (locals.var_ps0_inia__blk817, locals.var_ps0_inia__blk817_dn0, locals.var_ps0_inia__blk817_dn2, locals.var_ps0_inia__blk817_dn6, locals.var_ps0_inia__blk817_dn7, locals.var_ps0_inia__blk817_dn10, locals.var_ps0_inia__blk817_dn11, locals.var_ps0_inia__blk817_dn12, locals.var_ps0_inia__blk817_dn17,)
    }
};
        locals.var_ps0_inia__blk817 = assign27180_e37780;
        locals.var_ps0_inia__blk817_dn0 = assign27180_e37780_d_n0;
        locals.var_ps0_inia__blk817_dn2 = assign27180_e37780_d_n2;
        locals.var_ps0_inia__blk817_dn6 = assign27180_e37780_d_n6;
        locals.var_ps0_inia__blk817_dn7 = assign27180_e37780_d_n7;
        locals.var_ps0_inia__blk817_dn10 = assign27180_e37780_d_n10;
        locals.var_ps0_inia__blk817_dn11 = assign27180_e37780_d_n11;
        locals.var_ps0_inia__blk817_dn12 = assign27180_e37780_d_n12;
        locals.var_ps0_inia__blk817_dn17 = assign27180_e37780_d_n17;
        locals.var_ps0_inia__blk817_rv = 0.0;

        let (assign27190_e37795, assign27190_e37795_d_n0, assign27190_e37795_d_n2, assign27190_e37795_d_n6, assign27190_e37795_d_n7, assign27190_e37795_d_n10, assign27190_e37795_d_n11, assign27190_e37795_d_n12, assign27190_e37795_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27190_e37792: f64 = (locals.var_ps0_inia__blk817 + locals.var_vxbgmtcl);
        let assign27190_e37793: f64 = (locals.var_beta * assign27190_e37792);
        (assign27190_e37793, (locals.var_beta * (locals.var_ps0_inia__blk817_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign27190_e37792) + (locals.var_beta * (locals.var_ps0_inia__blk817_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk817_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
        locals.var_chi__blk814 = assign27190_e37795;
        locals.var_chi__blk814_dn0 = assign27190_e37795_d_n0;
        locals.var_chi__blk814_dn2 = assign27190_e37795_d_n2;
        locals.var_chi__blk814_dn6 = assign27190_e37795_d_n6;
        locals.var_chi__blk814_dn7 = assign27190_e37795_d_n7;
        locals.var_chi__blk814_dn10 = assign27190_e37795_d_n10;
        locals.var_chi__blk814_dn11 = assign27190_e37795_d_n11;
        locals.var_chi__blk814_dn12 = assign27190_e37795_d_n12;
        locals.var_chi__blk814_dn17 = assign27190_e37795_d_n17;
        locals.var_chi__blk814_rv = 0.0;

        let assign27200_e37798: f64 = if locals.var_chi__blk814 < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard875 = assign27200_e37798;
        locals.var_guard875_rv = 0.0;

        let (assign27220_e37841,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27220_e37825: f64 = (9.0 * 1.414213562373095);
        let assign27220_e37826: f64 = (1.0 / assign27220_e37825);
        let assign27220_e37830: f64 = (7.0 * 0.049787068367863944);
        let assign27220_e37831: f64 = (5.0 + assign27220_e37830);
        let assign27220_e37835: f64 = (2.0 + 0.049787068367863944);
        let assign27220_e37836: f64 = (assign27220_e37835).sqrt();
        let assign27220_e37837: f64 = (54.0 * assign27220_e37836);
        let assign27220_e37838: f64 = (assign27220_e37831 / assign27220_e37837);
        let assign27220_e37839: f64 = (assign27220_e37826 - assign27220_e37838);
        (assign27220_e37839,)
    } else {
        (locals.var_ta,)
    }
};
        locals.var_ta = assign27220_e37841;
        locals.var_ta_rv = 0.0;

        let (assign27230_e37867,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27230_e37854: f64 = (1.0 + 0.049787068367863944);
        let assign27230_e37858: f64 = (2.0 + 0.049787068367863944);
        let assign27230_e37859: f64 = (assign27230_e37858).sqrt();
        let assign27230_e37860: f64 = (2.0 * assign27230_e37859);
        let assign27230_e37861: f64 = (assign27230_e37854 / assign27230_e37860);
        let assign27230_e37864: f64 = (1.414213562373095 / 3.0);
        let assign27230_e37865: f64 = (assign27230_e37861 - assign27230_e37864);
        (assign27230_e37865,)
    } else {
        (locals.var_tb,)
    }
};
        locals.var_tb = assign27230_e37867;
        locals.var_tb_rv = 0.0;

        let (assign27240_e37888, assign27240_e37888_d_n0, assign27240_e37888_d_n2, assign27240_e37888_d_n6, assign27240_e37888_d_n7, assign27240_e37888_d_n10, assign27240_e37888_d_n11, assign27240_e37888_d_n12, assign27240_e37888_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27240_e37880: f64 = (1.0 / 1.414213562373095);
        let assign27240_e37884: f64 = (locals.var_beta * locals.var_fac1__blk800);
        let assign27240_e37885: f64 = (1.0 / assign27240_e37884);
        let assign27240_e37886: f64 = (assign27240_e37880 + assign27240_e37885);
        (assign27240_e37886, (-((locals.var_beta * locals.var_fac1__blk800_dn0) / (assign27240_e37884 * assign27240_e37884))), (-((locals.var_beta * locals.var_fac1__blk800_dn2) / (assign27240_e37884 * assign27240_e37884))), (-((locals.var_beta * locals.var_fac1__blk800_dn6) / (assign27240_e37884 * assign27240_e37884))), (-((locals.var_beta * locals.var_fac1__blk800_dn7) / (assign27240_e37884 * assign27240_e37884))), (-(((locals.var_beta_dn10 * locals.var_fac1__blk800) + (locals.var_beta * locals.var_fac1__blk800_dn10)) / (assign27240_e37884 * assign27240_e37884))), (-((locals.var_beta * locals.var_fac1__blk800_dn11) / (assign27240_e37884 * assign27240_e37884))), (-((locals.var_beta * locals.var_fac1__blk800_dn12) / (assign27240_e37884 * assign27240_e37884))), (-((locals.var_beta * locals.var_fac1__blk800_dn17) / (assign27240_e37884 * assign27240_e37884))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn12, locals.var_tc_dn17,)
    }
};
        locals.var_tc = assign27240_e37888;
        locals.var_tc_dn0 = assign27240_e37888_d_n0;
        locals.var_tc_dn2 = assign27240_e37888_d_n2;
        locals.var_tc_dn6 = assign27240_e37888_d_n6;
        locals.var_tc_dn7 = assign27240_e37888_d_n7;
        locals.var_tc_dn10 = assign27240_e37888_d_n10;
        locals.var_tc_dn11 = assign27240_e37888_d_n11;
        locals.var_tc_dn12 = assign27240_e37888_d_n12;
        locals.var_tc_dn17 = assign27240_e37888_d_n17;
        locals.var_tc_rv = 0.0;

        let (assign27250_e37906, assign27250_e37906_d_n0, assign27250_e37906_d_n2, assign27250_e37906_d_n6, assign27250_e37906_d_n7, assign27250_e37906_d_n10, assign27250_e37906_d_n11, assign27250_e37906_d_n12, assign27250_e37906_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27250_e37901: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign27250_e37902: f64 = (-assign27250_e37901);
        let assign27250_e37904: f64 = (assign27250_e37902 / locals.var_fac1__blk800);
        (assign27250_e37904, ((((-(locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0)) * locals.var_fac1__blk800) - (assign27250_e37902 * locals.var_fac1__blk800_dn0)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2)) * locals.var_fac1__blk800) - (assign27250_e37902 * locals.var_fac1__blk800_dn2)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6)) * locals.var_fac1__blk800) - (assign27250_e37902 * locals.var_fac1__blk800_dn6)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7)) * locals.var_fac1__blk800) - (assign27250_e37902 * locals.var_fac1__blk800_dn7)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10)) * locals.var_fac1__blk800) - (assign27250_e37902 * locals.var_fac1__blk800_dn10)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11)) * locals.var_fac1__blk800) - (assign27250_e37902 * locals.var_fac1__blk800_dn11)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12)) * locals.var_fac1__blk800) - (assign27250_e37902 * locals.var_fac1__blk800_dn12)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)), ((((-(locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17)) * locals.var_fac1__blk800) - (assign27250_e37902 * locals.var_fac1__blk800_dn17)) / (locals.var_fac1__blk800 * locals.var_fac1__blk800)),)
    } else {
        (locals.var_td, locals.var_td_dn0, locals.var_td_dn2, locals.var_td_dn6, locals.var_td_dn7, locals.var_td_dn10, locals.var_td_dn11, locals.var_td_dn12, locals.var_td_dn17,)
    }
};
        locals.var_td = assign27250_e37906;
        locals.var_td_dn0 = assign27250_e37906_d_n0;
        locals.var_td_dn2 = assign27250_e37906_d_n2;
        locals.var_td_dn6 = assign27250_e37906_d_n6;
        locals.var_td_dn7 = assign27250_e37906_d_n7;
        locals.var_td_dn10 = assign27250_e37906_d_n10;
        locals.var_td_dn11 = assign27250_e37906_d_n11;
        locals.var_td_dn12 = assign27250_e37906_d_n12;
        locals.var_td_dn17 = assign27250_e37906_d_n17;
        locals.var_td_rv = 0.0;

        let (assign27260_e37947, assign27260_e37947_d_n0, assign27260_e37947_d_n2, assign27260_e37947_d_n6, assign27260_e37947_d_n7, assign27260_e37947_d_n10, assign27260_e37947_d_n11, assign27260_e37947_d_n12, assign27260_e37947_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27260_e37919: f64 = (locals.var_tb * locals.var_tb);
        let assign27260_e37921: f64 = (assign27260_e37919 * locals.var_tb);
        let assign27260_e37924: f64 = (27.0 * locals.var_ta);
        let assign27260_e37926: f64 = (assign27260_e37924 * locals.var_ta);
        let assign27260_e37928: f64 = (assign27260_e37926 * locals.var_ta);
        let assign27260_e37929: f64 = (assign27260_e37921 / assign27260_e37928);
        let assign27260_e37932: f64 = (locals.var_tb * locals.var_tc);
        let assign27260_e37935: f64 = (6.0 * locals.var_ta);
        let assign27260_e37937: f64 = (assign27260_e37935 * locals.var_ta);
        let assign27260_e37938: f64 = (assign27260_e37932 / assign27260_e37937);
        let assign27260_e37939: f64 = (assign27260_e37929 - assign27260_e37938);
        let assign27260_e37943: f64 = (2.0 * locals.var_ta);
        let assign27260_e37944: f64 = (locals.var_td / assign27260_e37943);
        let assign27260_e37945: f64 = (assign27260_e37939 + assign27260_e37944);
        (assign27260_e37945, ((-((locals.var_tb * locals.var_tc_dn0) / assign27260_e37937)) + (locals.var_td_dn0 / assign27260_e37943)), ((-((locals.var_tb * locals.var_tc_dn2) / assign27260_e37937)) + (locals.var_td_dn2 / assign27260_e37943)), ((-((locals.var_tb * locals.var_tc_dn6) / assign27260_e37937)) + (locals.var_td_dn6 / assign27260_e37943)), ((-((locals.var_tb * locals.var_tc_dn7) / assign27260_e37937)) + (locals.var_td_dn7 / assign27260_e37943)), ((-((locals.var_tb * locals.var_tc_dn10) / assign27260_e37937)) + (locals.var_td_dn10 / assign27260_e37943)), ((-((locals.var_tb * locals.var_tc_dn11) / assign27260_e37937)) + (locals.var_td_dn11 / assign27260_e37943)), ((-((locals.var_tb * locals.var_tc_dn12) / assign27260_e37937)) + (locals.var_td_dn12 / assign27260_e37943)), ((-((locals.var_tb * locals.var_tc_dn17) / assign27260_e37937)) + (locals.var_td_dn17 / assign27260_e37943)),)
    } else {
        (locals.var_tq, locals.var_tq_dn0, locals.var_tq_dn2, locals.var_tq_dn6, locals.var_tq_dn7, locals.var_tq_dn10, locals.var_tq_dn11, locals.var_tq_dn12, locals.var_tq_dn17,)
    }
};
        locals.var_tq = assign27260_e37947;
        locals.var_tq_dn0 = assign27260_e37947_d_n0;
        locals.var_tq_dn2 = assign27260_e37947_d_n2;
        locals.var_tq_dn6 = assign27260_e37947_d_n6;
        locals.var_tq_dn7 = assign27260_e37947_d_n7;
        locals.var_tq_dn10 = assign27260_e37947_d_n10;
        locals.var_tq_dn11 = assign27260_e37947_d_n11;
        locals.var_tq_dn12 = assign27260_e37947_d_n12;
        locals.var_tq_dn17 = assign27260_e37947_d_n17;
        locals.var_tq_rv = 0.0;

        let (assign27270_e37974, assign27270_e37974_d_n0, assign27270_e37974_d_n2, assign27270_e37974_d_n6, assign27270_e37974_d_n7, assign27270_e37974_d_n10, assign27270_e37974_d_n11, assign27270_e37974_d_n12, assign27270_e37974_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27270_e37960: f64 = (3.0 * locals.var_ta);
        let assign27270_e37962: f64 = (assign27270_e37960 * locals.var_tc);
        let assign27270_e37965: f64 = (locals.var_tb * locals.var_tb);
        let assign27270_e37966: f64 = (assign27270_e37962 - assign27270_e37965);
        let assign27270_e37969: f64 = (9.0 * locals.var_ta);
        let assign27270_e37971: f64 = (assign27270_e37969 * locals.var_ta);
        let assign27270_e37972: f64 = (assign27270_e37966 / assign27270_e37971);
        (assign27270_e37972, ((assign27270_e37960 * locals.var_tc_dn0) / assign27270_e37971), ((assign27270_e37960 * locals.var_tc_dn2) / assign27270_e37971), ((assign27270_e37960 * locals.var_tc_dn6) / assign27270_e37971), ((assign27270_e37960 * locals.var_tc_dn7) / assign27270_e37971), ((assign27270_e37960 * locals.var_tc_dn10) / assign27270_e37971), ((assign27270_e37960 * locals.var_tc_dn11) / assign27270_e37971), ((assign27270_e37960 * locals.var_tc_dn12) / assign27270_e37971), ((assign27270_e37960 * locals.var_tc_dn17) / assign27270_e37971),)
    } else {
        (locals.var_tp, locals.var_tp_dn0, locals.var_tp_dn2, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn10, locals.var_tp_dn11, locals.var_tp_dn12, locals.var_tp_dn17,)
    }
};
        locals.var_tp = assign27270_e37974;
        locals.var_tp_dn0 = assign27270_e37974_d_n0;
        locals.var_tp_dn2 = assign27270_e37974_d_n2;
        locals.var_tp_dn6 = assign27270_e37974_d_n6;
        locals.var_tp_dn7 = assign27270_e37974_d_n7;
        locals.var_tp_dn10 = assign27270_e37974_d_n10;
        locals.var_tp_dn11 = assign27270_e37974_d_n11;
        locals.var_tp_dn12 = assign27270_e37974_d_n12;
        locals.var_tp_dn17 = assign27270_e37974_d_n17;
        locals.var_tp_rv = 0.0;

        let (assign27280_e37996, assign27280_e37996_d_n0, assign27280_e37996_d_n2, assign27280_e37996_d_n6, assign27280_e37996_d_n7, assign27280_e37996_d_n10, assign27280_e37996_d_n11, assign27280_e37996_d_n12, assign27280_e37996_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27280_e37987: f64 = (locals.var_tq * locals.var_tq);
        let assign27280_e37990: f64 = (locals.var_tp * locals.var_tp);
        let assign27280_e37992: f64 = (assign27280_e37990 * locals.var_tp);
        let assign27280_e37993: f64 = (assign27280_e37987 + assign27280_e37992);
        let assign27280_e37994: f64 = (assign27280_e37993).sqrt();
        (assign27280_e37994, ((((locals.var_tq_dn0 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn0)) + ((((locals.var_tp_dn0 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn0)) * locals.var_tp) + (assign27280_e37990 * locals.var_tp_dn0))) / (2.0 * assign27280_e37994)), ((((locals.var_tq_dn2 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn2)) + ((((locals.var_tp_dn2 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn2)) * locals.var_tp) + (assign27280_e37990 * locals.var_tp_dn2))) / (2.0 * assign27280_e37994)), ((((locals.var_tq_dn6 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn6)) + ((((locals.var_tp_dn6 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn6)) * locals.var_tp) + (assign27280_e37990 * locals.var_tp_dn6))) / (2.0 * assign27280_e37994)), ((((locals.var_tq_dn7 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn7)) + ((((locals.var_tp_dn7 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn7)) * locals.var_tp) + (assign27280_e37990 * locals.var_tp_dn7))) / (2.0 * assign27280_e37994)), ((((locals.var_tq_dn10 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn10)) + ((((locals.var_tp_dn10 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn10)) * locals.var_tp) + (assign27280_e37990 * locals.var_tp_dn10))) / (2.0 * assign27280_e37994)), ((((locals.var_tq_dn11 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn11)) + ((((locals.var_tp_dn11 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn11)) * locals.var_tp) + (assign27280_e37990 * locals.var_tp_dn11))) / (2.0 * assign27280_e37994)), ((((locals.var_tq_dn12 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn12)) + ((((locals.var_tp_dn12 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn12)) * locals.var_tp) + (assign27280_e37990 * locals.var_tp_dn12))) / (2.0 * assign27280_e37994)), ((((locals.var_tq_dn17 * locals.var_tq) + (locals.var_tq * locals.var_tq_dn17)) + ((((locals.var_tp_dn17 * locals.var_tp) + (locals.var_tp * locals.var_tp_dn17)) * locals.var_tp) + (assign27280_e37990 * locals.var_tp_dn17))) / (2.0 * assign27280_e37994)),)
    } else {
        (locals.var_t5__blk774, locals.var_t5__blk774_dn0, locals.var_t5__blk774_dn2, locals.var_t5__blk774_dn6, locals.var_t5__blk774_dn7, locals.var_t5__blk774_dn10, locals.var_t5__blk774_dn11, locals.var_t5__blk774_dn12, locals.var_t5__blk774_dn17,)
    }
};
        locals.var_t5__blk774 = assign27280_e37996;
        locals.var_t5__blk774_dn0 = assign27280_e37996_d_n0;
        locals.var_t5__blk774_dn2 = assign27280_e37996_d_n2;
        locals.var_t5__blk774_dn6 = assign27280_e37996_d_n6;
        locals.var_t5__blk774_dn7 = assign27280_e37996_d_n7;
        locals.var_t5__blk774_dn10 = assign27280_e37996_d_n10;
        locals.var_t5__blk774_dn11 = assign27280_e37996_d_n11;
        locals.var_t5__blk774_dn12 = assign27280_e37996_d_n12;
        locals.var_t5__blk774_dn17 = assign27280_e37996_d_n17;
        locals.var_t5__blk774_rv = 0.0;

        let (assign27290_e38014, assign27290_e38014_d_n0, assign27290_e38014_d_n2, assign27290_e38014_d_n6, assign27290_e38014_d_n7, assign27290_e38014_d_n10, assign27290_e38014_d_n11, assign27290_e38014_d_n12, assign27290_e38014_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27290_e38008: f64 = (-locals.var_tq);
        let assign27290_e38010: f64 = (assign27290_e38008 + locals.var_t5__blk774);
        let assign27290_e38012: f64 = (assign27290_e38010).powf(0.3333333333333333);
        (assign27290_e38012, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27290_e38010).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn0) + locals.var_t5__blk774_dn0))) } } else { (assign27290_e38012 * (0.3333333333333333 * (((-locals.var_tq_dn0) + locals.var_t5__blk774_dn0) / assign27290_e38010))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27290_e38010).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn2) + locals.var_t5__blk774_dn2))) } } else { (assign27290_e38012 * (0.3333333333333333 * (((-locals.var_tq_dn2) + locals.var_t5__blk774_dn2) / assign27290_e38010))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27290_e38010).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn6) + locals.var_t5__blk774_dn6))) } } else { (assign27290_e38012 * (0.3333333333333333 * (((-locals.var_tq_dn6) + locals.var_t5__blk774_dn6) / assign27290_e38010))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27290_e38010).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn7) + locals.var_t5__blk774_dn7))) } } else { (assign27290_e38012 * (0.3333333333333333 * (((-locals.var_tq_dn7) + locals.var_t5__blk774_dn7) / assign27290_e38010))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27290_e38010).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn10) + locals.var_t5__blk774_dn10))) } } else { (assign27290_e38012 * (0.3333333333333333 * (((-locals.var_tq_dn10) + locals.var_t5__blk774_dn10) / assign27290_e38010))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27290_e38010).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn11) + locals.var_t5__blk774_dn11))) } } else { (assign27290_e38012 * (0.3333333333333333 * (((-locals.var_tq_dn11) + locals.var_t5__blk774_dn11) / assign27290_e38010))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27290_e38010).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn12) + locals.var_t5__blk774_dn12))) } } else { (assign27290_e38012 * (0.3333333333333333 * (((-locals.var_tq_dn12) + locals.var_t5__blk774_dn12) / assign27290_e38010))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27290_e38010).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq_dn17) + locals.var_t5__blk774_dn17))) } } else { (assign27290_e38012 * (0.3333333333333333 * (((-locals.var_tq_dn17) + locals.var_t5__blk774_dn17) / assign27290_e38010))) },)
    } else {
        (locals.var_tu, locals.var_tu_dn0, locals.var_tu_dn2, locals.var_tu_dn6, locals.var_tu_dn7, locals.var_tu_dn10, locals.var_tu_dn11, locals.var_tu_dn12, locals.var_tu_dn17,)
    }
};
        locals.var_tu = assign27290_e38014;
        locals.var_tu_dn0 = assign27290_e38014_d_n0;
        locals.var_tu_dn2 = assign27290_e38014_d_n2;
        locals.var_tu_dn6 = assign27290_e38014_d_n6;
        locals.var_tu_dn7 = assign27290_e38014_d_n7;
        locals.var_tu_dn10 = assign27290_e38014_d_n10;
        locals.var_tu_dn11 = assign27290_e38014_d_n11;
        locals.var_tu_dn12 = assign27290_e38014_d_n12;
        locals.var_tu_dn17 = assign27290_e38014_d_n17;
        locals.var_tu_rv = 0.0;

        let (assign27300_e38032, assign27300_e38032_d_n0, assign27300_e38032_d_n2, assign27300_e38032_d_n6, assign27300_e38032_d_n7, assign27300_e38032_d_n10, assign27300_e38032_d_n11, assign27300_e38032_d_n12, assign27300_e38032_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27300_e38027: f64 = (locals.var_tq + locals.var_t5__blk774);
        let assign27300_e38029: f64 = (assign27300_e38027).powf(0.3333333333333333);
        let assign27300_e38030: f64 = (-assign27300_e38029);
        (assign27300_e38030, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27300_e38027).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn0 + locals.var_t5__blk774_dn0))) } } else { (assign27300_e38029 * (0.3333333333333333 * ((locals.var_tq_dn0 + locals.var_t5__blk774_dn0) / assign27300_e38027))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27300_e38027).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn2 + locals.var_t5__blk774_dn2))) } } else { (assign27300_e38029 * (0.3333333333333333 * ((locals.var_tq_dn2 + locals.var_t5__blk774_dn2) / assign27300_e38027))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27300_e38027).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn6 + locals.var_t5__blk774_dn6))) } } else { (assign27300_e38029 * (0.3333333333333333 * ((locals.var_tq_dn6 + locals.var_t5__blk774_dn6) / assign27300_e38027))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27300_e38027).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn7 + locals.var_t5__blk774_dn7))) } } else { (assign27300_e38029 * (0.3333333333333333 * ((locals.var_tq_dn7 + locals.var_t5__blk774_dn7) / assign27300_e38027))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27300_e38027).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn10 + locals.var_t5__blk774_dn10))) } } else { (assign27300_e38029 * (0.3333333333333333 * ((locals.var_tq_dn10 + locals.var_t5__blk774_dn10) / assign27300_e38027))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27300_e38027).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn11 + locals.var_t5__blk774_dn11))) } } else { (assign27300_e38029 * (0.3333333333333333 * ((locals.var_tq_dn11 + locals.var_t5__blk774_dn11) / assign27300_e38027))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27300_e38027).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn12 + locals.var_t5__blk774_dn12))) } } else { (assign27300_e38029 * (0.3333333333333333 * ((locals.var_tq_dn12 + locals.var_t5__blk774_dn12) / assign27300_e38027))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign27300_e38027).powf(0.3333333333333333 - 1.0) * (locals.var_tq_dn17 + locals.var_t5__blk774_dn17))) } } else { (assign27300_e38029 * (0.3333333333333333 * ((locals.var_tq_dn17 + locals.var_t5__blk774_dn17) / assign27300_e38027))) }),)
    } else {
        (locals.var_tv, locals.var_tv_dn0, locals.var_tv_dn2, locals.var_tv_dn6, locals.var_tv_dn7, locals.var_tv_dn10, locals.var_tv_dn11, locals.var_tv_dn12, locals.var_tv_dn17,)
    }
};
        locals.var_tv = assign27300_e38032;
        locals.var_tv_dn0 = assign27300_e38032_d_n0;
        locals.var_tv_dn2 = assign27300_e38032_d_n2;
        locals.var_tv_dn6 = assign27300_e38032_d_n6;
        locals.var_tv_dn7 = assign27300_e38032_d_n7;
        locals.var_tv_dn10 = assign27300_e38032_d_n10;
        locals.var_tv_dn11 = assign27300_e38032_d_n11;
        locals.var_tv_dn12 = assign27300_e38032_d_n12;
        locals.var_tv_dn17 = assign27300_e38032_d_n17;
        locals.var_tv_rv = 0.0;

        let (assign27310_e38053, assign27310_e38053_d_n0, assign27310_e38053_d_n2, assign27310_e38053_d_n6, assign27310_e38053_d_n7, assign27310_e38053_d_n10, assign27310_e38053_d_n11, assign27310_e38053_d_n12, assign27310_e38053_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27310_e38045: f64 = (locals.var_tu + locals.var_tv);
        let assign27310_e38049: f64 = (3.0 * locals.var_ta);
        let assign27310_e38050: f64 = (locals.var_tb / assign27310_e38049);
        let assign27310_e38051: f64 = (assign27310_e38045 - assign27310_e38050);
        (assign27310_e38051, (locals.var_tu_dn0 + locals.var_tv_dn0), (locals.var_tu_dn2 + locals.var_tv_dn2), (locals.var_tu_dn6 + locals.var_tv_dn6), (locals.var_tu_dn7 + locals.var_tv_dn7), (locals.var_tu_dn10 + locals.var_tv_dn10), (locals.var_tu_dn11 + locals.var_tv_dn11), (locals.var_tu_dn12 + locals.var_tv_dn12), (locals.var_tu_dn17 + locals.var_tv_dn17),)
    } else {
        (locals.var_tx__blk777, locals.var_tx__blk777_dn0, locals.var_tx__blk777_dn2, locals.var_tx__blk777_dn6, locals.var_tx__blk777_dn7, locals.var_tx__blk777_dn10, locals.var_tx__blk777_dn11, locals.var_tx__blk777_dn12, locals.var_tx__blk777_dn17,)
    }
};
        locals.var_tx__blk777 = assign27310_e38053;
        locals.var_tx__blk777_dn0 = assign27310_e38053_d_n0;
        locals.var_tx__blk777_dn2 = assign27310_e38053_d_n2;
        locals.var_tx__blk777_dn6 = assign27310_e38053_d_n6;
        locals.var_tx__blk777_dn7 = assign27310_e38053_d_n7;
        locals.var_tx__blk777_dn10 = assign27310_e38053_d_n10;
        locals.var_tx__blk777_dn11 = assign27310_e38053_d_n11;
        locals.var_tx__blk777_dn12 = assign27310_e38053_d_n12;
        locals.var_tx__blk777_dn17 = assign27310_e38053_d_n17;
        locals.var_tx__blk777_rv = 0.0;

        let (assign27320_e38070, assign27320_e38070_d_n0, assign27320_e38070_d_n2, assign27320_e38070_d_n6, assign27320_e38070_d_n7, assign27320_e38070_d_n10, assign27320_e38070_d_n11, assign27320_e38070_d_n12, assign27320_e38070_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27320_e38066: f64 = (locals.var_tx__blk777 * locals.var_beta_inv);
        let assign27320_e38068: f64 = (assign27320_e38066 - locals.var_vxbgmtcl);
        (assign27320_e38068, ((locals.var_tx__blk777_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn0), ((locals.var_tx__blk777_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn2), ((locals.var_tx__blk777_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn6), ((locals.var_tx__blk777_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn7), (((locals.var_tx__blk777_dn10 * locals.var_beta_inv) + (locals.var_tx__blk777 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl_dn10), ((locals.var_tx__blk777_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn11), ((locals.var_tx__blk777_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn12), ((locals.var_tx__blk777_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0_inia__blk817, locals.var_ps0_inia__blk817_dn0, locals.var_ps0_inia__blk817_dn2, locals.var_ps0_inia__blk817_dn6, locals.var_ps0_inia__blk817_dn7, locals.var_ps0_inia__blk817_dn10, locals.var_ps0_inia__blk817_dn11, locals.var_ps0_inia__blk817_dn12, locals.var_ps0_inia__blk817_dn17,)
    }
};
        locals.var_ps0_inia__blk817 = assign27320_e38070;
        locals.var_ps0_inia__blk817_dn0 = assign27320_e38070_d_n0;
        locals.var_ps0_inia__blk817_dn2 = assign27320_e38070_d_n2;
        locals.var_ps0_inia__blk817_dn6 = assign27320_e38070_d_n6;
        locals.var_ps0_inia__blk817_dn7 = assign27320_e38070_d_n7;
        locals.var_ps0_inia__blk817_dn10 = assign27320_e38070_d_n10;
        locals.var_ps0_inia__blk817_dn11 = assign27320_e38070_d_n11;
        locals.var_ps0_inia__blk817_dn12 = assign27320_e38070_d_n12;
        locals.var_ps0_inia__blk817_dn17 = assign27320_e38070_d_n17;
        locals.var_ps0_inia__blk817_rv = 0.0;

        let (assign27330_e38087, assign27330_e38087_d_n0, assign27330_e38087_d_n2, assign27330_e38087_d_n6, assign27330_e38087_d_n7, assign27330_e38087_d_n10, assign27330_e38087_d_n11, assign27330_e38087_d_n12, assign27330_e38087_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard875 != 0.0)) {
        let assign27330_e38084: f64 = (locals.var_ps0_inia__blk817 + locals.var_vxbgmtcl);
        let assign27330_e38085: f64 = (locals.var_beta * assign27330_e38084);
        (assign27330_e38085, (locals.var_beta * (locals.var_ps0_inia__blk817_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign27330_e38084) + (locals.var_beta * (locals.var_ps0_inia__blk817_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk817_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk817_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
        locals.var_chi__blk814 = assign27330_e38087;
        locals.var_chi__blk814_dn0 = assign27330_e38087_d_n0;
        locals.var_chi__blk814_dn2 = assign27330_e38087_d_n2;
        locals.var_chi__blk814_dn6 = assign27330_e38087_d_n6;
        locals.var_chi__blk814_dn7 = assign27330_e38087_d_n7;
        locals.var_chi__blk814_dn10 = assign27330_e38087_d_n10;
        locals.var_chi__blk814_dn11 = assign27330_e38087_d_n11;
        locals.var_chi__blk814_dn12 = assign27330_e38087_d_n12;
        locals.var_chi__blk814_dn17 = assign27330_e38087_d_n17;
        locals.var_chi__blk814_rv = 0.0;

        let (assign27350_e38115, assign27350_e38115_d_n0, assign27350_e38115_d_n2, assign27350_e38115_d_n6, assign27350_e38115_d_n7, assign27350_e38115_d_n10, assign27350_e38115_d_n11, assign27350_e38115_d_n12, assign27350_e38115_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27350_e38111: f64 = (locals.var_vgpld + locals.var_vxbgmtcl);
        let assign27350_e38113: f64 = (assign27350_e38111 + 0.1);
        (assign27350_e38113, (locals.var_vgpld_dn0 + locals.var_vxbgmtcl_dn0), (locals.var_vgpld_dn2 + locals.var_vxbgmtcl_dn2), (locals.var_vgpld_dn6 + locals.var_vxbgmtcl_dn6), (locals.var_vgpld_dn7 + locals.var_vxbgmtcl_dn7), (locals.var_vgpld_dn10 + locals.var_vxbgmtcl_dn10), (locals.var_vgpld_dn11 + locals.var_vxbgmtcl_dn11), (locals.var_vgpld_dn12 + locals.var_vxbgmtcl_dn12), (locals.var_vgpld_dn17 + locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_vgpld_shift, locals.var_vgpld_shift_dn0, locals.var_vgpld_shift_dn2, locals.var_vgpld_shift_dn6, locals.var_vgpld_shift_dn7, locals.var_vgpld_shift_dn10, locals.var_vgpld_shift_dn11, locals.var_vgpld_shift_dn12, locals.var_vgpld_shift_dn17,)
    }
};
        locals.var_vgpld_shift = assign27350_e38115;
        locals.var_vgpld_shift_dn0 = assign27350_e38115_d_n0;
        locals.var_vgpld_shift_dn2 = assign27350_e38115_d_n2;
        locals.var_vgpld_shift_dn6 = assign27350_e38115_d_n6;
        locals.var_vgpld_shift_dn7 = assign27350_e38115_d_n7;
        locals.var_vgpld_shift_dn10 = assign27350_e38115_d_n10;
        locals.var_vgpld_shift_dn11 = assign27350_e38115_d_n11;
        locals.var_vgpld_shift_dn12 = assign27350_e38115_d_n12;
        locals.var_vgpld_shift_dn17 = assign27350_e38115_d_n17;
        locals.var_vgpld_shift_rv = 0.0;

        let (assign27360_e38132, assign27360_e38132_d_n0, assign27360_e38132_d_n2, assign27360_e38132_d_n6, assign27360_e38132_d_n7, assign27360_e38132_d_n10, assign27360_e38132_d_n11, assign27360_e38132_d_n12, assign27360_e38132_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27360_e38126: f64 = (-locals.var_vxbgmtcl);
        let assign27360_e38127: f64 = (locals.var_beta * assign27360_e38126);
        let assign27360_e38128: f64 = (assign27360_e38127).exp();
        let assign27360_e38130: f64 = (assign27360_e38128 + 1e-50);
        (assign27360_e38130, (assign27360_e38128 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign27360_e38128 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign27360_e38128 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign27360_e38128 * (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), (assign27360_e38128 * ((locals.var_beta_dn10 * assign27360_e38126) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign27360_e38128 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign27360_e38128 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))), (assign27360_e38128 * (locals.var_beta * (-locals.var_vxbgmtcl_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk833, locals.var_exp_bvbs__blk833_dn0, locals.var_exp_bvbs__blk833_dn2, locals.var_exp_bvbs__blk833_dn6, locals.var_exp_bvbs__blk833_dn7, locals.var_exp_bvbs__blk833_dn10, locals.var_exp_bvbs__blk833_dn11, locals.var_exp_bvbs__blk833_dn12, locals.var_exp_bvbs__blk833_dn17,)
    }
};
        locals.var_exp_bvbs__blk833 = assign27360_e38132;
        locals.var_exp_bvbs__blk833_dn0 = assign27360_e38132_d_n0;
        locals.var_exp_bvbs__blk833_dn2 = assign27360_e38132_d_n2;
        locals.var_exp_bvbs__blk833_dn6 = assign27360_e38132_d_n6;
        locals.var_exp_bvbs__blk833_dn7 = assign27360_e38132_d_n7;
        locals.var_exp_bvbs__blk833_dn10 = assign27360_e38132_d_n10;
        locals.var_exp_bvbs__blk833_dn11 = assign27360_e38132_d_n11;
        locals.var_exp_bvbs__blk833_dn12 = assign27360_e38132_d_n12;
        locals.var_exp_bvbs__blk833_dn17 = assign27360_e38132_d_n17;
        locals.var_exp_bvbs__blk833_rv = 0.0;

        let (assign27370_e38145, assign27370_e38145_d_n0, assign27370_e38145_d_n2, assign27370_e38145_d_n6, assign27370_e38145_d_n7, assign27370_e38145_d_n10, assign27370_e38145_d_n11, assign27370_e38145_d_n12, assign27370_e38145_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27370_e38143: f64 = (locals.var_nin / locals.var_uc_nsubbttub);
        (assign27370_e38143, (((locals.var_nin_dn0 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn0)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn2 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn2)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn6 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn6)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn7 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn7)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn10 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn10)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn11 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn11)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn12 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn12)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn17 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn17)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)),)
    } else {
        (locals.var_t0__blk770, locals.var_t0__blk770_dn0, locals.var_t0__blk770_dn2, locals.var_t0__blk770_dn6, locals.var_t0__blk770_dn7, locals.var_t0__blk770_dn10, locals.var_t0__blk770_dn11, locals.var_t0__blk770_dn12, locals.var_t0__blk770_dn17,)
    }
};
        locals.var_t0__blk770 = assign27370_e38145;
        locals.var_t0__blk770_dn0 = assign27370_e38145_d_n0;
        locals.var_t0__blk770_dn2 = assign27370_e38145_d_n2;
        locals.var_t0__blk770_dn6 = assign27370_e38145_d_n6;
        locals.var_t0__blk770_dn7 = assign27370_e38145_d_n7;
        locals.var_t0__blk770_dn10 = assign27370_e38145_d_n10;
        locals.var_t0__blk770_dn11 = assign27370_e38145_d_n11;
        locals.var_t0__blk770_dn12 = assign27370_e38145_d_n12;
        locals.var_t0__blk770_dn17 = assign27370_e38145_d_n17;
        locals.var_t0__blk770_rv = 0.0;

        let (assign27380_e38158, assign27380_e38158_d_n0, assign27380_e38158_d_n2, assign27380_e38158_d_n6, assign27380_e38158_d_n7, assign27380_e38158_d_n10, assign27380_e38158_d_n11, assign27380_e38158_d_n12, assign27380_e38158_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27380_e38156: f64 = (locals.var_t0__blk770 * locals.var_t0__blk770);
        (assign27380_e38156, ((locals.var_t0__blk770_dn0 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn0)), ((locals.var_t0__blk770_dn2 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn2)), ((locals.var_t0__blk770_dn6 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn6)), ((locals.var_t0__blk770_dn7 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn7)), ((locals.var_t0__blk770_dn10 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn10)), ((locals.var_t0__blk770_dn11 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn11)), ((locals.var_t0__blk770_dn12 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn12)), ((locals.var_t0__blk770_dn17 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn17)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12, locals.var_cnst1over_dn17,)
    }
};
        locals.var_cnst1over = assign27380_e38158;
        locals.var_cnst1over_dn0 = assign27380_e38158_d_n0;
        locals.var_cnst1over_dn2 = assign27380_e38158_d_n2;
        locals.var_cnst1over_dn6 = assign27380_e38158_d_n6;
        locals.var_cnst1over_dn7 = assign27380_e38158_d_n7;
        locals.var_cnst1over_dn10 = assign27380_e38158_d_n10;
        locals.var_cnst1over_dn11 = assign27380_e38158_d_n11;
        locals.var_cnst1over_dn12 = assign27380_e38158_d_n12;
        locals.var_cnst1over_dn17 = assign27380_e38158_d_n17;
        locals.var_cnst1over_rv = 0.0;

        let (assign27390_e38171, assign27390_e38171_d_n0, assign27390_e38171_d_n2, assign27390_e38171_d_n6, assign27390_e38171_d_n7, assign27390_e38171_d_n10, assign27390_e38171_d_n11, assign27390_e38171_d_n12, assign27390_e38171_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27390_e38169: f64 = (locals.var_cnst1over * locals.var_exp_bvbs__blk833);
        (assign27390_e38169, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn2)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn7)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn12)), ((locals.var_cnst1over_dn17 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn17)),)
    } else {
        (locals.var_gammachi, locals.var_gammachi_dn0, locals.var_gammachi_dn2, locals.var_gammachi_dn6, locals.var_gammachi_dn7, locals.var_gammachi_dn10, locals.var_gammachi_dn11, locals.var_gammachi_dn12, locals.var_gammachi_dn17,)
    }
};
        locals.var_gammachi = assign27390_e38171;
        locals.var_gammachi_dn0 = assign27390_e38171_d_n0;
        locals.var_gammachi_dn2 = assign27390_e38171_d_n2;
        locals.var_gammachi_dn6 = assign27390_e38171_d_n6;
        locals.var_gammachi_dn7 = assign27390_e38171_d_n7;
        locals.var_gammachi_dn10 = assign27390_e38171_d_n10;
        locals.var_gammachi_dn11 = assign27390_e38171_d_n11;
        locals.var_gammachi_dn12 = assign27390_e38171_d_n12;
        locals.var_gammachi_dn17 = assign27390_e38171_d_n17;
        locals.var_gammachi_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_98(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27400_e38184, assign27400_e38184_d_n0, assign27400_e38184_d_n2, assign27400_e38184_d_n6, assign27400_e38184_d_n7, assign27400_e38184_d_n10, assign27400_e38184_d_n11, assign27400_e38184_d_n12, assign27400_e38184_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27400_e38182: f64 = (locals.var_beta2 * locals.var_fac1p2__blk801);
        (assign27400_e38182, (locals.var_beta2 * locals.var_fac1p2__blk801_dn0), (locals.var_beta2 * locals.var_fac1p2__blk801_dn2), (locals.var_beta2 * locals.var_fac1p2__blk801_dn6), (locals.var_beta2 * locals.var_fac1p2__blk801_dn7), ((locals.var_beta2_dn10 * locals.var_fac1p2__blk801) + (locals.var_beta2 * locals.var_fac1p2__blk801_dn10)), (locals.var_beta2 * locals.var_fac1p2__blk801_dn11), (locals.var_beta2 * locals.var_fac1p2__blk801_dn12), (locals.var_beta2 * locals.var_fac1p2__blk801_dn17),)
    } else {
        (locals.var_t0__blk770, locals.var_t0__blk770_dn0, locals.var_t0__blk770_dn2, locals.var_t0__blk770_dn6, locals.var_t0__blk770_dn7, locals.var_t0__blk770_dn10, locals.var_t0__blk770_dn11, locals.var_t0__blk770_dn12, locals.var_t0__blk770_dn17,)
    }
};
        locals.var_t0__blk770 = assign27400_e38184;
        locals.var_t0__blk770_dn0 = assign27400_e38184_d_n0;
        locals.var_t0__blk770_dn2 = assign27400_e38184_d_n2;
        locals.var_t0__blk770_dn6 = assign27400_e38184_d_n6;
        locals.var_t0__blk770_dn7 = assign27400_e38184_d_n7;
        locals.var_t0__blk770_dn10 = assign27400_e38184_d_n10;
        locals.var_t0__blk770_dn11 = assign27400_e38184_d_n11;
        locals.var_t0__blk770_dn12 = assign27400_e38184_d_n12;
        locals.var_t0__blk770_dn17 = assign27400_e38184_d_n17;
        locals.var_t0__blk770_rv = 0.0;

        let (assign27410_e38197, assign27410_e38197_d_n0, assign27410_e38197_d_n2, assign27410_e38197_d_n6, assign27410_e38197_d_n7, assign27410_e38197_d_n10, assign27410_e38197_d_n11, assign27410_e38197_d_n12, assign27410_e38197_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27410_e38195: f64 = (locals.var_beta * locals.var_vgpld_shift);
        (assign27410_e38195, (locals.var_beta * locals.var_vgpld_shift_dn0), (locals.var_beta * locals.var_vgpld_shift_dn2), (locals.var_beta * locals.var_vgpld_shift_dn6), (locals.var_beta * locals.var_vgpld_shift_dn7), ((locals.var_beta_dn10 * locals.var_vgpld_shift) + (locals.var_beta * locals.var_vgpld_shift_dn10)), (locals.var_beta * locals.var_vgpld_shift_dn11), (locals.var_beta * locals.var_vgpld_shift_dn12), (locals.var_beta * locals.var_vgpld_shift_dn17),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign27410_e38197;
        locals.var_psi_dn0 = assign27410_e38197_d_n0;
        locals.var_psi_dn2 = assign27410_e38197_d_n2;
        locals.var_psi_dn6 = assign27410_e38197_d_n6;
        locals.var_psi_dn7 = assign27410_e38197_d_n7;
        locals.var_psi_dn10 = assign27410_e38197_d_n10;
        locals.var_psi_dn11 = assign27410_e38197_d_n11;
        locals.var_psi_dn12 = assign27410_e38197_d_n12;
        locals.var_psi_dn17 = assign27410_e38197_d_n17;
        locals.var_psi_rv = 0.0;

        let (assign27420_e38224, assign27420_e38224_d_n0, assign27420_e38224_d_n2, assign27420_e38224_d_n6, assign27420_e38224_d_n7, assign27420_e38224_d_n10, assign27420_e38224_d_n11, assign27420_e38224_d_n12, assign27420_e38224_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27420_e38208: f64 = (locals.var_gammachi * locals.var_t0__blk770);
        let assign27420_e38211: f64 = (locals.var_psi * locals.var_psi);
        let assign27420_e38212: f64 = (assign27420_e38208 + assign27420_e38211);
        let assign27420_e38213: f64 = (assign27420_e38212).ln();
        let assign27420_e38216: f64 = (locals.var_cnst1over * locals.var_t0__blk770);
        let assign27420_e38217: f64 = (assign27420_e38216).ln();
        let assign27420_e38218: f64 = (assign27420_e38213 - assign27420_e38217);
        let assign27420_e38221: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign27420_e38222: f64 = (assign27420_e38218 + assign27420_e38221);
        (assign27420_e38222, ((((((locals.var_gammachi_dn0 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign27420_e38212) - (((locals.var_cnst1over_dn0 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn0)) / assign27420_e38216)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign27420_e38212) - (((locals.var_cnst1over_dn2 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn2)) / assign27420_e38216)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn6 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign27420_e38212) - (((locals.var_cnst1over_dn6 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn6)) / assign27420_e38216)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn7 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign27420_e38212) - (((locals.var_cnst1over_dn7 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn7)) / assign27420_e38216)) + (locals.var_beta * locals.var_vxbgmtcl_dn7)), ((((((locals.var_gammachi_dn10 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign27420_e38212) - (((locals.var_cnst1over_dn10 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn10)) / assign27420_e38216)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign27420_e38212) - (((locals.var_cnst1over_dn11 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn11)) / assign27420_e38216)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign27420_e38212) - (((locals.var_cnst1over_dn12 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn12)) / assign27420_e38216)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)), ((((((locals.var_gammachi_dn17 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn17)) + ((locals.var_psi_dn17 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn17))) / assign27420_e38212) - (((locals.var_cnst1over_dn17 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn17)) / assign27420_e38216)) + (locals.var_beta * locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12, locals.var_chi_1_dn17,)
    }
};
        locals.var_chi_1 = assign27420_e38224;
        locals.var_chi_1_dn0 = assign27420_e38224_d_n0;
        locals.var_chi_1_dn2 = assign27420_e38224_d_n2;
        locals.var_chi_1_dn6 = assign27420_e38224_d_n6;
        locals.var_chi_1_dn7 = assign27420_e38224_d_n7;
        locals.var_chi_1_dn10 = assign27420_e38224_d_n10;
        locals.var_chi_1_dn11 = assign27420_e38224_d_n11;
        locals.var_chi_1_dn12 = assign27420_e38224_d_n12;
        locals.var_chi_1_dn17 = assign27420_e38224_d_n17;
        locals.var_chi_1_rv = 0.0;

        let (assign27430_e38239, assign27430_e38239_d_n0, assign27430_e38239_d_n2, assign27430_e38239_d_n6, assign27430_e38239_d_n7, assign27430_e38239_d_n10, assign27430_e38239_d_n11, assign27430_e38239_d_n12, assign27430_e38239_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27430_e38235: f64 = (locals.var_psi - locals.var_chi_1);
        let assign27430_e38237: f64 = (assign27430_e38235 - 1.0);
        (assign27430_e38237, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12), (locals.var_psi_dn17 - locals.var_chi_1_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign27430_e38239;
        locals.var_tmf1_dn0 = assign27430_e38239_d_n0;
        locals.var_tmf1_dn2 = assign27430_e38239_d_n2;
        locals.var_tmf1_dn6 = assign27430_e38239_d_n6;
        locals.var_tmf1_dn7 = assign27430_e38239_d_n7;
        locals.var_tmf1_dn10 = assign27430_e38239_d_n10;
        locals.var_tmf1_dn11 = assign27430_e38239_d_n11;
        locals.var_tmf1_dn12 = assign27430_e38239_d_n12;
        locals.var_tmf1_dn17 = assign27430_e38239_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign27440_e38254, assign27440_e38254_d_n0, assign27440_e38254_d_n2, assign27440_e38254_d_n6, assign27440_e38254_d_n7, assign27440_e38254_d_n10, assign27440_e38254_d_n11, assign27440_e38254_d_n12, assign27440_e38254_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27440_e38250: f64 = (4.0 * locals.var_psi);
        let assign27440_e38252: f64 = assign27440_e38250;
        (assign27440_e38252, (4.0 * locals.var_psi_dn0), (4.0 * locals.var_psi_dn2), (4.0 * locals.var_psi_dn6), (4.0 * locals.var_psi_dn7), (4.0 * locals.var_psi_dn10), (4.0 * locals.var_psi_dn11), (4.0 * locals.var_psi_dn12), (4.0 * locals.var_psi_dn17),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27440_e38254;
        locals.var_tmf2_dn0 = assign27440_e38254_d_n0;
        locals.var_tmf2_dn2 = assign27440_e38254_d_n2;
        locals.var_tmf2_dn6 = assign27440_e38254_d_n6;
        locals.var_tmf2_dn7 = assign27440_e38254_d_n7;
        locals.var_tmf2_dn10 = assign27440_e38254_d_n10;
        locals.var_tmf2_dn11 = assign27440_e38254_d_n11;
        locals.var_tmf2_dn12 = assign27440_e38254_d_n12;
        locals.var_tmf2_dn17 = assign27440_e38254_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign27450_e38271, assign27450_e38271_d_n0, assign27450_e38271_d_n2, assign27450_e38271_d_n6, assign27450_e38271_d_n7, assign27450_e38271_d_n10, assign27450_e38271_d_n11, assign27450_e38271_d_n12, assign27450_e38271_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let (assign27450_e38269, assign27450_e38269_d_n0, assign27450_e38269_d_n2, assign27450_e38269_d_n6, assign27450_e38269_d_n7, assign27450_e38269_d_n10, assign27450_e38269_d_n11, assign27450_e38269_d_n12, assign27450_e38269_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign27450_e38268: f64 = (-locals.var_tmf2);
                (assign27450_e38268, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign27450_e38269, assign27450_e38269_d_n0, assign27450_e38269_d_n2, assign27450_e38269_d_n6, assign27450_e38269_d_n7, assign27450_e38269_d_n10, assign27450_e38269_d_n11, assign27450_e38269_d_n12, assign27450_e38269_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27450_e38271;
        locals.var_tmf2_dn0 = assign27450_e38271_d_n0;
        locals.var_tmf2_dn2 = assign27450_e38271_d_n2;
        locals.var_tmf2_dn6 = assign27450_e38271_d_n6;
        locals.var_tmf2_dn7 = assign27450_e38271_d_n7;
        locals.var_tmf2_dn10 = assign27450_e38271_d_n10;
        locals.var_tmf2_dn11 = assign27450_e38271_d_n11;
        locals.var_tmf2_dn12 = assign27450_e38271_d_n12;
        locals.var_tmf2_dn17 = assign27450_e38271_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign27460_e38287, assign27460_e38287_d_n0, assign27460_e38287_d_n2, assign27460_e38287_d_n6, assign27460_e38287_d_n7, assign27460_e38287_d_n10, assign27460_e38287_d_n11, assign27460_e38287_d_n12, assign27460_e38287_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27460_e38282: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign27460_e38284: f64 = (assign27460_e38282 + locals.var_tmf2);
        let assign27460_e38285: f64 = (assign27460_e38284).sqrt();
        (assign27460_e38285, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign27460_e38285)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign27460_e38285)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign27460_e38285)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign27460_e38285)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign27460_e38285)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign27460_e38285)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign27460_e38285)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign27460_e38285)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27460_e38287;
        locals.var_tmf2_dn0 = assign27460_e38287_d_n0;
        locals.var_tmf2_dn2 = assign27460_e38287_d_n2;
        locals.var_tmf2_dn6 = assign27460_e38287_d_n6;
        locals.var_tmf2_dn7 = assign27460_e38287_d_n7;
        locals.var_tmf2_dn10 = assign27460_e38287_d_n10;
        locals.var_tmf2_dn11 = assign27460_e38287_d_n11;
        locals.var_tmf2_dn12 = assign27460_e38287_d_n12;
        locals.var_tmf2_dn17 = assign27460_e38287_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign27470_e38304, assign27470_e38304_d_n0, assign27470_e38304_d_n2, assign27470_e38304_d_n6, assign27470_e38304_d_n7, assign27470_e38304_d_n10, assign27470_e38304_d_n11, assign27470_e38304_d_n12, assign27470_e38304_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27470_e38300: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign27470_e38301: f64 = (1.0 + assign27470_e38300);
        let assign27470_e38302: f64 = (0.5 * assign27470_e38301);
        (assign27470_e38302, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign27470_e38304;
        locals.var_t1__blk771_dn0 = assign27470_e38304_d_n0;
        locals.var_t1__blk771_dn2 = assign27470_e38304_d_n2;
        locals.var_t1__blk771_dn6 = assign27470_e38304_d_n6;
        locals.var_t1__blk771_dn7 = assign27470_e38304_d_n7;
        locals.var_t1__blk771_dn10 = assign27470_e38304_d_n10;
        locals.var_t1__blk771_dn11 = assign27470_e38304_d_n11;
        locals.var_t1__blk771_dn12 = assign27470_e38304_d_n12;
        locals.var_t1__blk771_dn17 = assign27470_e38304_d_n17;
        locals.var_t1__blk771_rv = 0.0;

        let (assign27480_e38325, assign27480_e38325_d_n0, assign27480_e38325_d_n2, assign27480_e38325_d_n6, assign27480_e38325_d_n7, assign27480_e38325_d_n10, assign27480_e38325_d_n11, assign27480_e38325_d_n12, assign27480_e38325_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27480_e38318: f64 = 2.0;
        let assign27480_e38319: f64 = (locals.var_tmf1 + assign27480_e38318);
        let assign27480_e38321: f64 = (assign27480_e38319 / locals.var_tmf2);
        let assign27480_e38322: f64 = (1.0 - assign27480_e38321);
        let assign27480_e38323: f64 = (0.5 * assign27480_e38322);
        (assign27480_e38323, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign27480_e38319 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign27480_e38319 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign27480_e38319 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign27480_e38319 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign27480_e38319 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign27480_e38319 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign27480_e38319 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign27480_e38319 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk772, locals.var_t2__blk772_dn0, locals.var_t2__blk772_dn2, locals.var_t2__blk772_dn6, locals.var_t2__blk772_dn7, locals.var_t2__blk772_dn10, locals.var_t2__blk772_dn11, locals.var_t2__blk772_dn12, locals.var_t2__blk772_dn17,)
    }
};
        locals.var_t2__blk772 = assign27480_e38325;
        locals.var_t2__blk772_dn0 = assign27480_e38325_d_n0;
        locals.var_t2__blk772_dn2 = assign27480_e38325_d_n2;
        locals.var_t2__blk772_dn6 = assign27480_e38325_d_n6;
        locals.var_t2__blk772_dn7 = assign27480_e38325_d_n7;
        locals.var_t2__blk772_dn10 = assign27480_e38325_d_n10;
        locals.var_t2__blk772_dn11 = assign27480_e38325_d_n11;
        locals.var_t2__blk772_dn12 = assign27480_e38325_d_n12;
        locals.var_t2__blk772_dn17 = assign27480_e38325_d_n17;
        locals.var_t2__blk772_rv = 0.0;

        let (assign27490_e38342, assign27490_e38342_d_n0, assign27490_e38342_d_n2, assign27490_e38342_d_n6, assign27490_e38342_d_n7, assign27490_e38342_d_n10, assign27490_e38342_d_n11, assign27490_e38342_d_n12, assign27490_e38342_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27490_e38338: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign27490_e38339: f64 = (0.5 * assign27490_e38338);
        let assign27490_e38340: f64 = (locals.var_psi - assign27490_e38339);
        (assign27490_e38340, (locals.var_psi_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psi_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi_1, locals.var_chi_1_dn0, locals.var_chi_1_dn2, locals.var_chi_1_dn6, locals.var_chi_1_dn7, locals.var_chi_1_dn10, locals.var_chi_1_dn11, locals.var_chi_1_dn12, locals.var_chi_1_dn17,)
    }
};
        locals.var_chi_1 = assign27490_e38342;
        locals.var_chi_1_dn0 = assign27490_e38342_d_n0;
        locals.var_chi_1_dn2 = assign27490_e38342_d_n2;
        locals.var_chi_1_dn6 = assign27490_e38342_d_n6;
        locals.var_chi_1_dn7 = assign27490_e38342_d_n7;
        locals.var_chi_1_dn10 = assign27490_e38342_d_n10;
        locals.var_chi_1_dn11 = assign27490_e38342_d_n11;
        locals.var_chi_1_dn12 = assign27490_e38342_d_n12;
        locals.var_chi_1_dn17 = assign27490_e38342_d_n17;
        locals.var_chi_1_rv = 0.0;

        let (assign27500_e38355, assign27500_e38355_d_n0, assign27500_e38355_d_n2, assign27500_e38355_d_n6, assign27500_e38355_d_n7, assign27500_e38355_d_n10, assign27500_e38355_d_n11, assign27500_e38355_d_n12, assign27500_e38355_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27500_e38353: f64 = (locals.var_psi - locals.var_chi_1);
        (assign27500_e38353, (locals.var_psi_dn0 - locals.var_chi_1_dn0), (locals.var_psi_dn2 - locals.var_chi_1_dn2), (locals.var_psi_dn6 - locals.var_chi_1_dn6), (locals.var_psi_dn7 - locals.var_chi_1_dn7), (locals.var_psi_dn10 - locals.var_chi_1_dn10), (locals.var_psi_dn11 - locals.var_chi_1_dn11), (locals.var_psi_dn12 - locals.var_chi_1_dn12), (locals.var_psi_dn17 - locals.var_chi_1_dn17),)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign27500_e38355;
        locals.var_psi_dn0 = assign27500_e38355_d_n0;
        locals.var_psi_dn2 = assign27500_e38355_d_n2;
        locals.var_psi_dn6 = assign27500_e38355_d_n6;
        locals.var_psi_dn7 = assign27500_e38355_d_n7;
        locals.var_psi_dn10 = assign27500_e38355_d_n10;
        locals.var_psi_dn11 = assign27500_e38355_d_n11;
        locals.var_psi_dn12 = assign27500_e38355_d_n12;
        locals.var_psi_dn17 = assign27500_e38355_d_n17;
        locals.var_psi_rv = 0.0;

        let (assign27510_e38370, assign27510_e38370_d_n0, assign27510_e38370_d_n2, assign27510_e38370_d_n6, assign27510_e38370_d_n7, assign27510_e38370_d_n10, assign27510_e38370_d_n11, assign27510_e38370_d_n12, assign27510_e38370_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27510_e38367: f64 = (locals.var_beta * 0.1);
        let assign27510_e38368: f64 = (locals.var_psi + assign27510_e38367);
        (assign27510_e38368, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, (locals.var_psi_dn10 + (locals.var_beta_dn10 * 0.1)), locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    } else {
        (locals.var_psi, locals.var_psi_dn0, locals.var_psi_dn2, locals.var_psi_dn6, locals.var_psi_dn7, locals.var_psi_dn10, locals.var_psi_dn11, locals.var_psi_dn12, locals.var_psi_dn17,)
    }
};
        locals.var_psi = assign27510_e38370;
        locals.var_psi_dn0 = assign27510_e38370_d_n0;
        locals.var_psi_dn2 = assign27510_e38370_d_n2;
        locals.var_psi_dn6 = assign27510_e38370_d_n6;
        locals.var_psi_dn7 = assign27510_e38370_d_n7;
        locals.var_psi_dn10 = assign27510_e38370_d_n10;
        locals.var_psi_dn11 = assign27510_e38370_d_n11;
        locals.var_psi_dn12 = assign27510_e38370_d_n12;
        locals.var_psi_dn17 = assign27510_e38370_d_n17;
        locals.var_psi_rv = 0.0;

        let (assign27520_e38397, assign27520_e38397_d_n0, assign27520_e38397_d_n2, assign27520_e38397_d_n6, assign27520_e38397_d_n7, assign27520_e38397_d_n10, assign27520_e38397_d_n11, assign27520_e38397_d_n12, assign27520_e38397_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27520_e38381: f64 = (locals.var_gammachi * locals.var_t0__blk770);
        let assign27520_e38384: f64 = (locals.var_psi * locals.var_psi);
        let assign27520_e38385: f64 = (assign27520_e38381 + assign27520_e38384);
        let assign27520_e38386: f64 = (assign27520_e38385).ln();
        let assign27520_e38389: f64 = (locals.var_cnst1over * locals.var_t0__blk770);
        let assign27520_e38390: f64 = (assign27520_e38389).ln();
        let assign27520_e38391: f64 = (assign27520_e38386 - assign27520_e38390);
        let assign27520_e38394: f64 = (locals.var_beta * locals.var_vxbgmtcl);
        let assign27520_e38395: f64 = (assign27520_e38391 + assign27520_e38394);
        (assign27520_e38395, ((((((locals.var_gammachi_dn0 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn0)) + ((locals.var_psi_dn0 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn0))) / assign27520_e38385) - (((locals.var_cnst1over_dn0 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn0)) / assign27520_e38389)) + (locals.var_beta * locals.var_vxbgmtcl_dn0)), ((((((locals.var_gammachi_dn2 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn2)) + ((locals.var_psi_dn2 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn2))) / assign27520_e38385) - (((locals.var_cnst1over_dn2 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn2)) / assign27520_e38389)) + (locals.var_beta * locals.var_vxbgmtcl_dn2)), ((((((locals.var_gammachi_dn6 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn6)) + ((locals.var_psi_dn6 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn6))) / assign27520_e38385) - (((locals.var_cnst1over_dn6 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn6)) / assign27520_e38389)) + (locals.var_beta * locals.var_vxbgmtcl_dn6)), ((((((locals.var_gammachi_dn7 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn7)) + ((locals.var_psi_dn7 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn7))) / assign27520_e38385) - (((locals.var_cnst1over_dn7 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn7)) / assign27520_e38389)) + (locals.var_beta * locals.var_vxbgmtcl_dn7)), ((((((locals.var_gammachi_dn10 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn10)) + ((locals.var_psi_dn10 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn10))) / assign27520_e38385) - (((locals.var_cnst1over_dn10 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn10)) / assign27520_e38389)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl) + (locals.var_beta * locals.var_vxbgmtcl_dn10))), ((((((locals.var_gammachi_dn11 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn11)) + ((locals.var_psi_dn11 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn11))) / assign27520_e38385) - (((locals.var_cnst1over_dn11 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn11)) / assign27520_e38389)) + (locals.var_beta * locals.var_vxbgmtcl_dn11)), ((((((locals.var_gammachi_dn12 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn12)) + ((locals.var_psi_dn12 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn12))) / assign27520_e38385) - (((locals.var_cnst1over_dn12 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn12)) / assign27520_e38389)) + (locals.var_beta * locals.var_vxbgmtcl_dn12)), ((((((locals.var_gammachi_dn17 * locals.var_t0__blk770) + (locals.var_gammachi * locals.var_t0__blk770_dn17)) + ((locals.var_psi_dn17 * locals.var_psi) + (locals.var_psi * locals.var_psi_dn17))) / assign27520_e38385) - (((locals.var_cnst1over_dn17 * locals.var_t0__blk770) + (locals.var_cnst1over * locals.var_t0__blk770_dn17)) / assign27520_e38389)) + (locals.var_beta * locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi_b, locals.var_chi_b_dn0, locals.var_chi_b_dn2, locals.var_chi_b_dn6, locals.var_chi_b_dn7, locals.var_chi_b_dn10, locals.var_chi_b_dn11, locals.var_chi_b_dn12, locals.var_chi_b_dn17,)
    }
};
        locals.var_chi_b = assign27520_e38397;
        locals.var_chi_b_dn0 = assign27520_e38397_d_n0;
        locals.var_chi_b_dn2 = assign27520_e38397_d_n2;
        locals.var_chi_b_dn6 = assign27520_e38397_d_n6;
        locals.var_chi_b_dn7 = assign27520_e38397_d_n7;
        locals.var_chi_b_dn10 = assign27520_e38397_d_n10;
        locals.var_chi_b_dn11 = assign27520_e38397_d_n11;
        locals.var_chi_b_dn12 = assign27520_e38397_d_n12;
        locals.var_chi_b_dn17 = assign27520_e38397_d_n17;
        locals.var_chi_b_rv = 0.0;

        let (assign27530_e38408, assign27530_e38408_d_n0, assign27530_e38408_d_n2, assign27530_e38408_d_n6, assign27530_e38408_d_n7, assign27530_e38408_d_n10, assign27530_e38408_d_n11, assign27530_e38408_d_n12, assign27530_e38408_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    } else {
        (locals.var_chi_a, locals.var_chi_a_dn0, locals.var_chi_a_dn2, locals.var_chi_a_dn6, locals.var_chi_a_dn7, locals.var_chi_a_dn10, locals.var_chi_a_dn11, locals.var_chi_a_dn12, locals.var_chi_a_dn17,)
    }
};
        locals.var_chi_a = assign27530_e38408;
        locals.var_chi_a_dn0 = assign27530_e38408_d_n0;
        locals.var_chi_a_dn2 = assign27530_e38408_d_n2;
        locals.var_chi_a_dn6 = assign27530_e38408_d_n6;
        locals.var_chi_a_dn7 = assign27530_e38408_d_n7;
        locals.var_chi_a_dn10 = assign27530_e38408_d_n10;
        locals.var_chi_a_dn11 = assign27530_e38408_d_n11;
        locals.var_chi_a_dn12 = assign27530_e38408_d_n12;
        locals.var_chi_a_dn17 = assign27530_e38408_d_n17;
        locals.var_chi_a_rv = 0.0;

        let (assign27540_e38425, assign27540_e38425_d_n0, assign27540_e38425_d_n2, assign27540_e38425_d_n6, assign27540_e38425_d_n7, assign27540_e38425_d_n10, assign27540_e38425_d_n11, assign27540_e38425_d_n12, assign27540_e38425_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27540_e38419: f64 = (locals.var_chi_b - locals.var_chi_a);
        let assign27540_e38422: f64 = (0.0008 * 75.0);
        let assign27540_e38423: f64 = (assign27540_e38419 - assign27540_e38422);
        (assign27540_e38423, (locals.var_chi_b_dn0 - locals.var_chi_a_dn0), (locals.var_chi_b_dn2 - locals.var_chi_a_dn2), (locals.var_chi_b_dn6 - locals.var_chi_a_dn6), (locals.var_chi_b_dn7 - locals.var_chi_a_dn7), (locals.var_chi_b_dn10 - locals.var_chi_a_dn10), (locals.var_chi_b_dn11 - locals.var_chi_a_dn11), (locals.var_chi_b_dn12 - locals.var_chi_a_dn12), (locals.var_chi_b_dn17 - locals.var_chi_a_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign27540_e38425;
        locals.var_tmf1_dn0 = assign27540_e38425_d_n0;
        locals.var_tmf1_dn2 = assign27540_e38425_d_n2;
        locals.var_tmf1_dn6 = assign27540_e38425_d_n6;
        locals.var_tmf1_dn7 = assign27540_e38425_d_n7;
        locals.var_tmf1_dn10 = assign27540_e38425_d_n10;
        locals.var_tmf1_dn11 = assign27540_e38425_d_n11;
        locals.var_tmf1_dn12 = assign27540_e38425_d_n12;
        locals.var_tmf1_dn17 = assign27540_e38425_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign27550_e38442, assign27550_e38442_d_n0, assign27550_e38442_d_n2, assign27550_e38442_d_n6, assign27550_e38442_d_n7, assign27550_e38442_d_n10, assign27550_e38442_d_n11, assign27550_e38442_d_n12, assign27550_e38442_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27550_e38436: f64 = (4.0 * locals.var_chi_b);
        let assign27550_e38439: f64 = (0.0008 * 75.0);
        let assign27550_e38440: f64 = (assign27550_e38436 * assign27550_e38439);
        (assign27550_e38440, ((4.0 * locals.var_chi_b_dn0) * assign27550_e38439), ((4.0 * locals.var_chi_b_dn2) * assign27550_e38439), ((4.0 * locals.var_chi_b_dn6) * assign27550_e38439), ((4.0 * locals.var_chi_b_dn7) * assign27550_e38439), ((4.0 * locals.var_chi_b_dn10) * assign27550_e38439), ((4.0 * locals.var_chi_b_dn11) * assign27550_e38439), ((4.0 * locals.var_chi_b_dn12) * assign27550_e38439), ((4.0 * locals.var_chi_b_dn17) * assign27550_e38439),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27550_e38442;
        locals.var_tmf2_dn0 = assign27550_e38442_d_n0;
        locals.var_tmf2_dn2 = assign27550_e38442_d_n2;
        locals.var_tmf2_dn6 = assign27550_e38442_d_n6;
        locals.var_tmf2_dn7 = assign27550_e38442_d_n7;
        locals.var_tmf2_dn10 = assign27550_e38442_d_n10;
        locals.var_tmf2_dn11 = assign27550_e38442_d_n11;
        locals.var_tmf2_dn12 = assign27550_e38442_d_n12;
        locals.var_tmf2_dn17 = assign27550_e38442_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign27560_e38459, assign27560_e38459_d_n0, assign27560_e38459_d_n2, assign27560_e38459_d_n6, assign27560_e38459_d_n7, assign27560_e38459_d_n10, assign27560_e38459_d_n11, assign27560_e38459_d_n12, assign27560_e38459_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let (assign27560_e38457, assign27560_e38457_d_n0, assign27560_e38457_d_n2, assign27560_e38457_d_n6, assign27560_e38457_d_n7, assign27560_e38457_d_n10, assign27560_e38457_d_n11, assign27560_e38457_d_n12, assign27560_e38457_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign27560_e38456: f64 = (-locals.var_tmf2);
                (assign27560_e38456, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign27560_e38457, assign27560_e38457_d_n0, assign27560_e38457_d_n2, assign27560_e38457_d_n6, assign27560_e38457_d_n7, assign27560_e38457_d_n10, assign27560_e38457_d_n11, assign27560_e38457_d_n12, assign27560_e38457_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27560_e38459;
        locals.var_tmf2_dn0 = assign27560_e38459_d_n0;
        locals.var_tmf2_dn2 = assign27560_e38459_d_n2;
        locals.var_tmf2_dn6 = assign27560_e38459_d_n6;
        locals.var_tmf2_dn7 = assign27560_e38459_d_n7;
        locals.var_tmf2_dn10 = assign27560_e38459_d_n10;
        locals.var_tmf2_dn11 = assign27560_e38459_d_n11;
        locals.var_tmf2_dn12 = assign27560_e38459_d_n12;
        locals.var_tmf2_dn17 = assign27560_e38459_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign27570_e38475, assign27570_e38475_d_n0, assign27570_e38475_d_n2, assign27570_e38475_d_n6, assign27570_e38475_d_n7, assign27570_e38475_d_n10, assign27570_e38475_d_n11, assign27570_e38475_d_n12, assign27570_e38475_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27570_e38470: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign27570_e38472: f64 = (assign27570_e38470 + locals.var_tmf2);
        let assign27570_e38473: f64 = (assign27570_e38472).sqrt();
        (assign27570_e38473, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign27570_e38473)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign27570_e38473)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign27570_e38473)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign27570_e38473)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign27570_e38473)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign27570_e38473)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign27570_e38473)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign27570_e38473)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign27570_e38475;
        locals.var_tmf2_dn0 = assign27570_e38475_d_n0;
        locals.var_tmf2_dn2 = assign27570_e38475_d_n2;
        locals.var_tmf2_dn6 = assign27570_e38475_d_n6;
        locals.var_tmf2_dn7 = assign27570_e38475_d_n7;
        locals.var_tmf2_dn10 = assign27570_e38475_d_n10;
        locals.var_tmf2_dn11 = assign27570_e38475_d_n11;
        locals.var_tmf2_dn12 = assign27570_e38475_d_n12;
        locals.var_tmf2_dn17 = assign27570_e38475_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign27580_e38492, assign27580_e38492_d_n0, assign27580_e38492_d_n2, assign27580_e38492_d_n6, assign27580_e38492_d_n7, assign27580_e38492_d_n10, assign27580_e38492_d_n11, assign27580_e38492_d_n12, assign27580_e38492_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27580_e38488: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign27580_e38489: f64 = (1.0 + assign27580_e38488);
        let assign27580_e38490: f64 = (0.5 * assign27580_e38489);
        (assign27580_e38490, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign27580_e38492;
        locals.var_t1__blk771_dn0 = assign27580_e38492_d_n0;
        locals.var_t1__blk771_dn2 = assign27580_e38492_d_n2;
        locals.var_t1__blk771_dn6 = assign27580_e38492_d_n6;
        locals.var_t1__blk771_dn7 = assign27580_e38492_d_n7;
        locals.var_t1__blk771_dn10 = assign27580_e38492_d_n10;
        locals.var_t1__blk771_dn11 = assign27580_e38492_d_n11;
        locals.var_t1__blk771_dn12 = assign27580_e38492_d_n12;
        locals.var_t1__blk771_dn17 = assign27580_e38492_d_n17;
        locals.var_t1__blk771_rv = 0.0;

        let (assign27590_e38515, assign27590_e38515_d_n0, assign27590_e38515_d_n2, assign27590_e38515_d_n6, assign27590_e38515_d_n7, assign27590_e38515_d_n10, assign27590_e38515_d_n11, assign27590_e38515_d_n12, assign27590_e38515_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27590_e38506: f64 = (2.0 * 0.0008);
        let assign27590_e38508: f64 = (assign27590_e38506 * 75.0);
        let assign27590_e38509: f64 = (locals.var_tmf1 + assign27590_e38508);
        let assign27590_e38511: f64 = (assign27590_e38509 / locals.var_tmf2);
        let assign27590_e38512: f64 = (1.0 - assign27590_e38511);
        let assign27590_e38513: f64 = (0.5 * assign27590_e38512);
        (assign27590_e38513, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign27590_e38509 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign27590_e38509 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign27590_e38509 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign27590_e38509 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign27590_e38509 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign27590_e38509 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign27590_e38509 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign27590_e38509 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk772, locals.var_t2__blk772_dn0, locals.var_t2__blk772_dn2, locals.var_t2__blk772_dn6, locals.var_t2__blk772_dn7, locals.var_t2__blk772_dn10, locals.var_t2__blk772_dn11, locals.var_t2__blk772_dn12, locals.var_t2__blk772_dn17,)
    }
};
        locals.var_t2__blk772 = assign27590_e38515;
        locals.var_t2__blk772_dn0 = assign27590_e38515_d_n0;
        locals.var_t2__blk772_dn2 = assign27590_e38515_d_n2;
        locals.var_t2__blk772_dn6 = assign27590_e38515_d_n6;
        locals.var_t2__blk772_dn7 = assign27590_e38515_d_n7;
        locals.var_t2__blk772_dn10 = assign27590_e38515_d_n10;
        locals.var_t2__blk772_dn11 = assign27590_e38515_d_n11;
        locals.var_t2__blk772_dn12 = assign27590_e38515_d_n12;
        locals.var_t2__blk772_dn17 = assign27590_e38515_d_n17;
        locals.var_t2__blk772_rv = 0.0;

        let (assign27600_e38532, assign27600_e38532_d_n0, assign27600_e38532_d_n2, assign27600_e38532_d_n6, assign27600_e38532_d_n7, assign27600_e38532_d_n10, assign27600_e38532_d_n11, assign27600_e38532_d_n12, assign27600_e38532_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27600_e38528: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign27600_e38529: f64 = (0.5 * assign27600_e38528);
        let assign27600_e38530: f64 = (locals.var_chi_b - assign27600_e38529);
        (assign27600_e38530, (locals.var_chi_b_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_chi_b_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_chi_b_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_chi_b_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_chi_b_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_chi_b_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_chi_b_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_chi_b_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
        locals.var_chi__blk814 = assign27600_e38532;
        locals.var_chi__blk814_dn0 = assign27600_e38532_d_n0;
        locals.var_chi__blk814_dn2 = assign27600_e38532_d_n2;
        locals.var_chi__blk814_dn6 = assign27600_e38532_d_n6;
        locals.var_chi__blk814_dn7 = assign27600_e38532_d_n7;
        locals.var_chi__blk814_dn10 = assign27600_e38532_d_n10;
        locals.var_chi__blk814_dn11 = assign27600_e38532_d_n11;
        locals.var_chi__blk814_dn12 = assign27600_e38532_d_n12;
        locals.var_chi__blk814_dn17 = assign27600_e38532_d_n17;
        locals.var_chi__blk814_rv = 0.0;

        let (assign27610_e38547, assign27610_e38547_d_n0, assign27610_e38547_d_n2, assign27610_e38547_d_n6, assign27610_e38547_d_n7, assign27610_e38547_d_n10, assign27610_e38547_d_n11, assign27610_e38547_d_n12, assign27610_e38547_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27610_e38543: f64 = (locals.var_chi__blk814 / locals.var_beta);
        let assign27610_e38545: f64 = (assign27610_e38543 - locals.var_vxbgmtcl);
        (assign27610_e38545, ((locals.var_chi__blk814_dn0 / locals.var_beta) - locals.var_vxbgmtcl_dn0), ((locals.var_chi__blk814_dn2 / locals.var_beta) - locals.var_vxbgmtcl_dn2), ((locals.var_chi__blk814_dn6 / locals.var_beta) - locals.var_vxbgmtcl_dn6), ((locals.var_chi__blk814_dn7 / locals.var_beta) - locals.var_vxbgmtcl_dn7), ((((locals.var_chi__blk814_dn10 * locals.var_beta) - (locals.var_chi__blk814 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl_dn10), ((locals.var_chi__blk814_dn11 / locals.var_beta) - locals.var_vxbgmtcl_dn11), ((locals.var_chi__blk814_dn12 / locals.var_beta) - locals.var_vxbgmtcl_dn12), ((locals.var_chi__blk814_dn17 / locals.var_beta) - locals.var_vxbgmtcl_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
        locals.var_ps0ld = assign27610_e38547;
        locals.var_ps0ld_dn0 = assign27610_e38547_d_n0;
        locals.var_ps0ld_dn2 = assign27610_e38547_d_n2;
        locals.var_ps0ld_dn6 = assign27610_e38547_d_n6;
        locals.var_ps0ld_dn7 = assign27610_e38547_d_n7;
        locals.var_ps0ld_dn10 = assign27610_e38547_d_n10;
        locals.var_ps0ld_dn11 = assign27610_e38547_d_n11;
        locals.var_ps0ld_dn12 = assign27610_e38547_d_n12;
        locals.var_ps0ld_dn17 = assign27610_e38547_d_n17;
        locals.var_ps0ld_rv = 0.0;

        let (assign27620_e38564, assign27620_e38564_d_n0, assign27620_e38564_d_n2, assign27620_e38564_d_n6, assign27620_e38564_d_n7, assign27620_e38564_d_n10, assign27620_e38564_d_n11, assign27620_e38564_d_n12, assign27620_e38564_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27620_e38558: f64 = (locals.var_chi__blk814 - 1.0);
        let assign27620_e38560: f64 = (-locals.var_chi__blk814);
        let assign27620_e38561: f64 = (assign27620_e38560).exp();
        let assign27620_e38562: f64 = (assign27620_e38558 + assign27620_e38561);
        (assign27620_e38562, (locals.var_chi__blk814_dn0 + (assign27620_e38561 * (-locals.var_chi__blk814_dn0))), (locals.var_chi__blk814_dn2 + (assign27620_e38561 * (-locals.var_chi__blk814_dn2))), (locals.var_chi__blk814_dn6 + (assign27620_e38561 * (-locals.var_chi__blk814_dn6))), (locals.var_chi__blk814_dn7 + (assign27620_e38561 * (-locals.var_chi__blk814_dn7))), (locals.var_chi__blk814_dn10 + (assign27620_e38561 * (-locals.var_chi__blk814_dn10))), (locals.var_chi__blk814_dn11 + (assign27620_e38561 * (-locals.var_chi__blk814_dn11))), (locals.var_chi__blk814_dn12 + (assign27620_e38561 * (-locals.var_chi__blk814_dn12))), (locals.var_chi__blk814_dn17 + (assign27620_e38561 * (-locals.var_chi__blk814_dn17))),)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign27620_e38564;
        locals.var_t1__blk771_dn0 = assign27620_e38564_d_n0;
        locals.var_t1__blk771_dn2 = assign27620_e38564_d_n2;
        locals.var_t1__blk771_dn6 = assign27620_e38564_d_n6;
        locals.var_t1__blk771_dn7 = assign27620_e38564_d_n7;
        locals.var_t1__blk771_dn10 = assign27620_e38564_d_n10;
        locals.var_t1__blk771_dn11 = assign27620_e38564_d_n11;
        locals.var_t1__blk771_dn12 = assign27620_e38564_d_n12;
        locals.var_t1__blk771_dn17 = assign27620_e38564_d_n17;
        locals.var_t1__blk771_rv = 0.0;

        let assign27630_e38568: f64 = (10.0 * 2.220446049250313e-16);
        let assign27630_e38569: f64 = if locals.var_t1__blk771 < assign27630_e38568 { 1.0 } else { 0.0 };
        locals.var_guard876 = assign27630_e38569;
        locals.var_guard876_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_99(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27640_e38584, assign27640_e38584_d_n0, assign27640_e38584_d_n2, assign27640_e38584_d_n6, assign27640_e38584_d_n7, assign27640_e38584_d_n10, assign27640_e38584_d_n11, assign27640_e38584_d_n12, assign27640_e38584_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard876 != 0.0)) {
        let assign27640_e38582: f64 = (10.0 * 2.220446049250313e-16);
        (assign27640_e38582, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign27640_e38584;
        locals.var_t1__blk771_dn0 = assign27640_e38584_d_n0;
        locals.var_t1__blk771_dn2 = assign27640_e38584_d_n2;
        locals.var_t1__blk771_dn6 = assign27640_e38584_d_n6;
        locals.var_t1__blk771_dn7 = assign27640_e38584_d_n7;
        locals.var_t1__blk771_dn10 = assign27640_e38584_d_n10;
        locals.var_t1__blk771_dn11 = assign27640_e38584_d_n11;
        locals.var_t1__blk771_dn12 = assign27640_e38584_d_n12;
        locals.var_t1__blk771_dn17 = assign27640_e38584_d_n17;
        locals.var_t1__blk771_rv = 0.0;

        let (assign27650_e38596, assign27650_e38596_d_n0, assign27650_e38596_d_n2, assign27650_e38596_d_n6, assign27650_e38596_d_n7, assign27650_e38596_d_n10, assign27650_e38596_d_n11, assign27650_e38596_d_n12, assign27650_e38596_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27650_e38594: f64 = (locals.var_t1__blk771).sqrt();
        (assign27650_e38594, (locals.var_t1__blk771_dn0 / (2.0 * assign27650_e38594)), (locals.var_t1__blk771_dn2 / (2.0 * assign27650_e38594)), (locals.var_t1__blk771_dn6 / (2.0 * assign27650_e38594)), (locals.var_t1__blk771_dn7 / (2.0 * assign27650_e38594)), (locals.var_t1__blk771_dn10 / (2.0 * assign27650_e38594)), (locals.var_t1__blk771_dn11 / (2.0 * assign27650_e38594)), (locals.var_t1__blk771_dn12 / (2.0 * assign27650_e38594)), (locals.var_t1__blk771_dn17 / (2.0 * assign27650_e38594)),)
    } else {
        (locals.var_t2__blk772, locals.var_t2__blk772_dn0, locals.var_t2__blk772_dn2, locals.var_t2__blk772_dn6, locals.var_t2__blk772_dn7, locals.var_t2__blk772_dn10, locals.var_t2__blk772_dn11, locals.var_t2__blk772_dn12, locals.var_t2__blk772_dn17,)
    }
};
        locals.var_t2__blk772 = assign27650_e38596;
        locals.var_t2__blk772_dn0 = assign27650_e38596_d_n0;
        locals.var_t2__blk772_dn2 = assign27650_e38596_d_n2;
        locals.var_t2__blk772_dn6 = assign27650_e38596_d_n6;
        locals.var_t2__blk772_dn7 = assign27650_e38596_d_n7;
        locals.var_t2__blk772_dn10 = assign27650_e38596_d_n10;
        locals.var_t2__blk772_dn11 = assign27650_e38596_d_n11;
        locals.var_t2__blk772_dn12 = assign27650_e38596_d_n12;
        locals.var_t2__blk772_dn17 = assign27650_e38596_d_n17;
        locals.var_t2__blk772_rv = 0.0;

        let (assign27660_e38609, assign27660_e38609_d_n0, assign27660_e38609_d_n2, assign27660_e38609_d_n6, assign27660_e38609_d_n7, assign27660_e38609_d_n10, assign27660_e38609_d_n11, assign27660_e38609_d_n12, assign27660_e38609_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27660_e38607: f64 = (locals.var_cnst0over * locals.var_t2__blk772);
        (assign27660_e38607, ((locals.var_cnst0over_dn0 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn0)), ((locals.var_cnst0over_dn2 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn2)), ((locals.var_cnst0over_dn6 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn6)), ((locals.var_cnst0over_dn7 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn7)), ((locals.var_cnst0over_dn10 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn10)), ((locals.var_cnst0over_dn11 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn11)), ((locals.var_cnst0over_dn12 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn12)), ((locals.var_cnst0over_dn17 * locals.var_t2__blk772) + (locals.var_cnst0over * locals.var_t2__blk772_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign27660_e38609;
        locals.var_qbuld_dn0 = assign27660_e38609_d_n0;
        locals.var_qbuld_dn2 = assign27660_e38609_d_n2;
        locals.var_qbuld_dn6 = assign27660_e38609_d_n6;
        locals.var_qbuld_dn7 = assign27660_e38609_d_n7;
        locals.var_qbuld_dn10 = assign27660_e38609_d_n10;
        locals.var_qbuld_dn11 = assign27660_e38609_d_n11;
        locals.var_qbuld_dn12 = assign27660_e38609_d_n12;
        locals.var_qbuld_dn17 = assign27660_e38609_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign27670_e38624, assign27670_e38624_d_n0, assign27670_e38624_d_n2, assign27670_e38624_d_n6, assign27670_e38624_d_n7, assign27670_e38624_d_n10, assign27670_e38624_d_n11, assign27670_e38624_d_n12, assign27670_e38624_d_n17,) = {
    if ((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) {
        let assign27670_e38621: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign27670_e38622: f64 = (locals.var_cox0 * assign27670_e38621);
        (assign27670_e38622, (locals.var_cox0 * (locals.var_vgpld_dn0 - locals.var_ps0ld_dn0)), (locals.var_cox0 * (locals.var_vgpld_dn2 - locals.var_ps0ld_dn2)), (locals.var_cox0 * (locals.var_vgpld_dn6 - locals.var_ps0ld_dn6)), (locals.var_cox0 * (locals.var_vgpld_dn7 - locals.var_ps0ld_dn7)), (locals.var_cox0 * (locals.var_vgpld_dn10 - locals.var_ps0ld_dn10)), (locals.var_cox0 * (locals.var_vgpld_dn11 - locals.var_ps0ld_dn11)), (locals.var_cox0 * (locals.var_vgpld_dn12 - locals.var_ps0ld_dn12)), (locals.var_cox0 * (locals.var_vgpld_dn17 - locals.var_ps0ld_dn17)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign27670_e38624;
        locals.var_qsuld_dn0 = assign27670_e38624_d_n0;
        locals.var_qsuld_dn2 = assign27670_e38624_d_n2;
        locals.var_qsuld_dn6 = assign27670_e38624_d_n6;
        locals.var_qsuld_dn7 = assign27670_e38624_d_n7;
        locals.var_qsuld_dn10 = assign27670_e38624_d_n10;
        locals.var_qsuld_dn11 = assign27670_e38624_d_n11;
        locals.var_qsuld_dn12 = assign27670_e38624_d_n12;
        locals.var_qsuld_dn17 = assign27670_e38624_d_n17;
        locals.var_qsuld_rv = 0.0;

        let assign27680_e38627: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard877 = assign27680_e38627;
        locals.var_guard877_rv = 0.0;

        let (assign27690_e38644, assign27690_e38644_d_n0, assign27690_e38644_d_n2, assign27690_e38644_d_n6, assign27690_e38644_d_n7, assign27690_e38644_d_n10, assign27690_e38644_d_n11, assign27690_e38644_d_n12, assign27690_e38644_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27690_e38640: f64 = (-locals.var_vxbgmtcl);
        let assign27690_e38641: f64 = (locals.var_beta * assign27690_e38640);
        let assign27690_e38642: f64 = (assign27690_e38641).exp();
        (assign27690_e38642, (assign27690_e38642 * (locals.var_beta * (-locals.var_vxbgmtcl_dn0))), (assign27690_e38642 * (locals.var_beta * (-locals.var_vxbgmtcl_dn2))), (assign27690_e38642 * (locals.var_beta * (-locals.var_vxbgmtcl_dn6))), (assign27690_e38642 * (locals.var_beta * (-locals.var_vxbgmtcl_dn7))), (assign27690_e38642 * ((locals.var_beta_dn10 * assign27690_e38640) + (locals.var_beta * (-locals.var_vxbgmtcl_dn10)))), (assign27690_e38642 * (locals.var_beta * (-locals.var_vxbgmtcl_dn11))), (assign27690_e38642 * (locals.var_beta * (-locals.var_vxbgmtcl_dn12))), (assign27690_e38642 * (locals.var_beta * (-locals.var_vxbgmtcl_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk833, locals.var_exp_bvbs__blk833_dn0, locals.var_exp_bvbs__blk833_dn2, locals.var_exp_bvbs__blk833_dn6, locals.var_exp_bvbs__blk833_dn7, locals.var_exp_bvbs__blk833_dn10, locals.var_exp_bvbs__blk833_dn11, locals.var_exp_bvbs__blk833_dn12, locals.var_exp_bvbs__blk833_dn17,)
    }
};
        locals.var_exp_bvbs__blk833 = assign27690_e38644;
        locals.var_exp_bvbs__blk833_dn0 = assign27690_e38644_d_n0;
        locals.var_exp_bvbs__blk833_dn2 = assign27690_e38644_d_n2;
        locals.var_exp_bvbs__blk833_dn6 = assign27690_e38644_d_n6;
        locals.var_exp_bvbs__blk833_dn7 = assign27690_e38644_d_n7;
        locals.var_exp_bvbs__blk833_dn10 = assign27690_e38644_d_n10;
        locals.var_exp_bvbs__blk833_dn11 = assign27690_e38644_d_n11;
        locals.var_exp_bvbs__blk833_dn12 = assign27690_e38644_d_n12;
        locals.var_exp_bvbs__blk833_dn17 = assign27690_e38644_d_n17;
        locals.var_exp_bvbs__blk833_rv = 0.0;

        let (assign27700_e38659, assign27700_e38659_d_n0, assign27700_e38659_d_n2, assign27700_e38659_d_n6, assign27700_e38659_d_n7, assign27700_e38659_d_n10, assign27700_e38659_d_n11, assign27700_e38659_d_n12, assign27700_e38659_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27700_e38657: f64 = (locals.var_nin / locals.var_uc_nsubbttub);
        (assign27700_e38657, (((locals.var_nin_dn0 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn0)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn2 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn2)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn6 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn6)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn7 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn7)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn10 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn10)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn11 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn11)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn12 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn12)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)), (((locals.var_nin_dn17 * locals.var_uc_nsubbttub) - (locals.var_nin * locals.var_uc_nsubbttub_dn17)) / (locals.var_uc_nsubbttub * locals.var_uc_nsubbttub)),)
    } else {
        (locals.var_t0__blk770, locals.var_t0__blk770_dn0, locals.var_t0__blk770_dn2, locals.var_t0__blk770_dn6, locals.var_t0__blk770_dn7, locals.var_t0__blk770_dn10, locals.var_t0__blk770_dn11, locals.var_t0__blk770_dn12, locals.var_t0__blk770_dn17,)
    }
};
        locals.var_t0__blk770 = assign27700_e38659;
        locals.var_t0__blk770_dn0 = assign27700_e38659_d_n0;
        locals.var_t0__blk770_dn2 = assign27700_e38659_d_n2;
        locals.var_t0__blk770_dn6 = assign27700_e38659_d_n6;
        locals.var_t0__blk770_dn7 = assign27700_e38659_d_n7;
        locals.var_t0__blk770_dn10 = assign27700_e38659_d_n10;
        locals.var_t0__blk770_dn11 = assign27700_e38659_d_n11;
        locals.var_t0__blk770_dn12 = assign27700_e38659_d_n12;
        locals.var_t0__blk770_dn17 = assign27700_e38659_d_n17;
        locals.var_t0__blk770_rv = 0.0;

        let (assign27710_e38674, assign27710_e38674_d_n0, assign27710_e38674_d_n2, assign27710_e38674_d_n6, assign27710_e38674_d_n7, assign27710_e38674_d_n10, assign27710_e38674_d_n11, assign27710_e38674_d_n12, assign27710_e38674_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27710_e38672: f64 = (locals.var_t0__blk770 * locals.var_t0__blk770);
        (assign27710_e38672, ((locals.var_t0__blk770_dn0 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn0)), ((locals.var_t0__blk770_dn2 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn2)), ((locals.var_t0__blk770_dn6 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn6)), ((locals.var_t0__blk770_dn7 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn7)), ((locals.var_t0__blk770_dn10 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn10)), ((locals.var_t0__blk770_dn11 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn11)), ((locals.var_t0__blk770_dn12 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn12)), ((locals.var_t0__blk770_dn17 * locals.var_t0__blk770) + (locals.var_t0__blk770 * locals.var_t0__blk770_dn17)),)
    } else {
        (locals.var_cnst1over, locals.var_cnst1over_dn0, locals.var_cnst1over_dn2, locals.var_cnst1over_dn6, locals.var_cnst1over_dn7, locals.var_cnst1over_dn10, locals.var_cnst1over_dn11, locals.var_cnst1over_dn12, locals.var_cnst1over_dn17,)
    }
};
        locals.var_cnst1over = assign27710_e38674;
        locals.var_cnst1over_dn0 = assign27710_e38674_d_n0;
        locals.var_cnst1over_dn2 = assign27710_e38674_d_n2;
        locals.var_cnst1over_dn6 = assign27710_e38674_d_n6;
        locals.var_cnst1over_dn7 = assign27710_e38674_d_n7;
        locals.var_cnst1over_dn10 = assign27710_e38674_d_n10;
        locals.var_cnst1over_dn11 = assign27710_e38674_d_n11;
        locals.var_cnst1over_dn12 = assign27710_e38674_d_n12;
        locals.var_cnst1over_dn17 = assign27710_e38674_d_n17;
        locals.var_cnst1over_rv = 0.0;

        let (assign27720_e38689, assign27720_e38689_d_n0, assign27720_e38689_d_n2, assign27720_e38689_d_n6, assign27720_e38689_d_n7, assign27720_e38689_d_n10, assign27720_e38689_d_n11, assign27720_e38689_d_n12, assign27720_e38689_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27720_e38687: f64 = (locals.var_cnst1over * locals.var_exp_bvbs__blk833);
        (assign27720_e38687, ((locals.var_cnst1over_dn0 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn0)), ((locals.var_cnst1over_dn2 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn2)), ((locals.var_cnst1over_dn6 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn6)), ((locals.var_cnst1over_dn7 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn7)), ((locals.var_cnst1over_dn10 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn10)), ((locals.var_cnst1over_dn11 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn11)), ((locals.var_cnst1over_dn12 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn12)), ((locals.var_cnst1over_dn17 * locals.var_exp_bvbs__blk833) + (locals.var_cnst1over * locals.var_exp_bvbs__blk833_dn17)),)
    } else {
        (locals.var_cfs1__blk842, locals.var_cfs1__blk842_dn0, locals.var_cfs1__blk842_dn2, locals.var_cfs1__blk842_dn6, locals.var_cfs1__blk842_dn7, locals.var_cfs1__blk842_dn10, locals.var_cfs1__blk842_dn11, locals.var_cfs1__blk842_dn12, locals.var_cfs1__blk842_dn17,)
    }
};
        locals.var_cfs1__blk842 = assign27720_e38689;
        locals.var_cfs1__blk842_dn0 = assign27720_e38689_d_n0;
        locals.var_cfs1__blk842_dn2 = assign27720_e38689_d_n2;
        locals.var_cfs1__blk842_dn6 = assign27720_e38689_d_n6;
        locals.var_cfs1__blk842_dn7 = assign27720_e38689_d_n7;
        locals.var_cfs1__blk842_dn10 = assign27720_e38689_d_n10;
        locals.var_cfs1__blk842_dn11 = assign27720_e38689_d_n11;
        locals.var_cfs1__blk842_dn12 = assign27720_e38689_d_n12;
        locals.var_cfs1__blk842_dn17 = assign27720_e38689_d_n17;
        locals.var_cfs1__blk842_rv = 0.0;

        let (assign27730_e38702,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv__blk787,)
    }
};
        locals.var_flg_conv__blk787 = assign27730_e38702;
        locals.var_flg_conv__blk787_rv = 0.0;

        let (assign27740_e38715,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign27740_e38715;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_100(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign27750_loop_guard: usize = 0;
        while {
            let assign27750_cond_e38729: f64 = (2.0 * 20.0);
            let assign27750_cond_e38731: f64 = (assign27750_cond_e38729 + 1.0);
            let assign27750_cond_e38733: f64 = if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_lp_s0 <= assign27750_cond_e38731)) { 1.0 } else { 0.0 };
            assign27750_cond_e38733 != 0.0
        } {
            assign27750_loop_guard += 1;
            assert!(assign27750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign27750_body0_e38746, assign27750_body0_e38746_d_n0, assign27750_body0_e38746_d_n2, assign27750_body0_e38746_d_n6, assign27750_body0_e38746_d_n7, assign27750_body0_e38746_d_n10, assign27750_body0_e38746_d_n11, assign27750_body0_e38746_d_n12, assign27750_body0_e38746_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb__blk838, locals.var_fb__blk838_dn0, locals.var_fb__blk838_dn2, locals.var_fb__blk838_dn6, locals.var_fb__blk838_dn7, locals.var_fb__blk838_dn10, locals.var_fb__blk838_dn11, locals.var_fb__blk838_dn12, locals.var_fb__blk838_dn17,)
    }
};
            locals.var_fb__blk838 = assign27750_body0_e38746;
            locals.var_fb__blk838_dn0 = assign27750_body0_e38746_d_n0;
            locals.var_fb__blk838_dn2 = assign27750_body0_e38746_d_n2;
            locals.var_fb__blk838_dn6 = assign27750_body0_e38746_d_n6;
            locals.var_fb__blk838_dn7 = assign27750_body0_e38746_d_n7;
            locals.var_fb__blk838_dn10 = assign27750_body0_e38746_d_n10;
            locals.var_fb__blk838_dn11 = assign27750_body0_e38746_d_n11;
            locals.var_fb__blk838_dn12 = assign27750_body0_e38746_d_n12;
            locals.var_fb__blk838_dn17 = assign27750_body0_e38746_d_n17;
            locals.var_fb__blk838_rv = 0.0;
            let (assign27750_body1_e38763, assign27750_body1_e38763_d_n0, assign27750_body1_e38763_d_n2, assign27750_body1_e38763_d_n6, assign27750_body1_e38763_d_n7, assign27750_body1_e38763_d_n10, assign27750_body1_e38763_d_n11, assign27750_body1_e38763_d_n12, assign27750_body1_e38763_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27750_body1_e38760: f64 = (locals.var_ps0ld + locals.var_vxbgmtcl);
        let assign27750_body1_e38761: f64 = (locals.var_beta * assign27750_body1_e38760);
        (assign27750_body1_e38761, (locals.var_beta * (locals.var_ps0ld_dn0 + locals.var_vxbgmtcl_dn0)), (locals.var_beta * (locals.var_ps0ld_dn2 + locals.var_vxbgmtcl_dn2)), (locals.var_beta * (locals.var_ps0ld_dn6 + locals.var_vxbgmtcl_dn6)), (locals.var_beta * (locals.var_ps0ld_dn7 + locals.var_vxbgmtcl_dn7)), ((locals.var_beta_dn10 * assign27750_body1_e38760) + (locals.var_beta * (locals.var_ps0ld_dn10 + locals.var_vxbgmtcl_dn10))), (locals.var_beta * (locals.var_ps0ld_dn11 + locals.var_vxbgmtcl_dn11)), (locals.var_beta * (locals.var_ps0ld_dn12 + locals.var_vxbgmtcl_dn12)), (locals.var_beta * (locals.var_ps0ld_dn17 + locals.var_vxbgmtcl_dn17)),)
    } else {
        (locals.var_chi__blk814, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    }
};
            locals.var_chi__blk814 = assign27750_body1_e38763;
            locals.var_chi__blk814_dn0 = assign27750_body1_e38763_d_n0;
            locals.var_chi__blk814_dn2 = assign27750_body1_e38763_d_n2;
            locals.var_chi__blk814_dn6 = assign27750_body1_e38763_d_n6;
            locals.var_chi__blk814_dn7 = assign27750_body1_e38763_d_n7;
            locals.var_chi__blk814_dn10 = assign27750_body1_e38763_d_n10;
            locals.var_chi__blk814_dn11 = assign27750_body1_e38763_d_n11;
            locals.var_chi__blk814_dn12 = assign27750_body1_e38763_d_n12;
            locals.var_chi__blk814_dn17 = assign27750_body1_e38763_d_n17;
            locals.var_chi__blk814_rv = 0.0;
            let assign27750_body2_e38766: f64 = if locals.var_chi__blk814 < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard878 = assign27750_body2_e38766;
            locals.var_guard878_rv = 0.0;
            let (assign27750_body3_e38796, assign27750_body3_e38796_d_n0, assign27750_body3_e38796_d_n2, assign27750_body3_e38796_d_n6, assign27750_body3_e38796_d_n7, assign27750_body3_e38796_d_n10, assign27750_body3_e38796_d_n11, assign27750_body3_e38796_d_n12, assign27750_body3_e38796_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard878 != 0.0)) {
        let assign27750_body3_e38781: f64 = (locals.var_chi__blk814 * locals.var_chi__blk814);
        let assign27750_body3_e38783: f64 = (assign27750_body3_e38781 * locals.var_chi__blk814);
        let assign27750_body3_e38787: f64 = (-0.07053654284009761);
        let assign27750_body3_e38790: f64 = (locals.var_chi__blk814 * 0.006115288895133179);
        let assign27750_body3_e38791: f64 = (assign27750_body3_e38787 + assign27750_body3_e38790);
        let assign27750_body3_e38792: f64 = (locals.var_chi__blk814 * assign27750_body3_e38791);
        let assign27750_body3_e38793: f64 = (0.29693154855771 + assign27750_body3_e38792);
        let assign27750_body3_e38794: f64 = (assign27750_body3_e38783 * assign27750_body3_e38793);
        (assign27750_body3_e38794, ((((((locals.var_chi__blk814_dn0 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn0)) * locals.var_chi__blk814) + (assign27750_body3_e38781 * locals.var_chi__blk814_dn0)) * assign27750_body3_e38793) + (assign27750_body3_e38783 * ((locals.var_chi__blk814_dn0 * assign27750_body3_e38791) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn0 * 0.006115288895133179))))), ((((((locals.var_chi__blk814_dn2 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn2)) * locals.var_chi__blk814) + (assign27750_body3_e38781 * locals.var_chi__blk814_dn2)) * assign27750_body3_e38793) + (assign27750_body3_e38783 * ((locals.var_chi__blk814_dn2 * assign27750_body3_e38791) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn2 * 0.006115288895133179))))), ((((((locals.var_chi__blk814_dn6 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn6)) * locals.var_chi__blk814) + (assign27750_body3_e38781 * locals.var_chi__blk814_dn6)) * assign27750_body3_e38793) + (assign27750_body3_e38783 * ((locals.var_chi__blk814_dn6 * assign27750_body3_e38791) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn6 * 0.006115288895133179))))), ((((((locals.var_chi__blk814_dn7 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn7)) * locals.var_chi__blk814) + (assign27750_body3_e38781 * locals.var_chi__blk814_dn7)) * assign27750_body3_e38793) + (assign27750_body3_e38783 * ((locals.var_chi__blk814_dn7 * assign27750_body3_e38791) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn7 * 0.006115288895133179))))), ((((((locals.var_chi__blk814_dn10 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn10)) * locals.var_chi__blk814) + (assign27750_body3_e38781 * locals.var_chi__blk814_dn10)) * assign27750_body3_e38793) + (assign27750_body3_e38783 * ((locals.var_chi__blk814_dn10 * assign27750_body3_e38791) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn10 * 0.006115288895133179))))), ((((((locals.var_chi__blk814_dn11 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn11)) * locals.var_chi__blk814) + (assign27750_body3_e38781 * locals.var_chi__blk814_dn11)) * assign27750_body3_e38793) + (assign27750_body3_e38783 * ((locals.var_chi__blk814_dn11 * assign27750_body3_e38791) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn11 * 0.006115288895133179))))), ((((((locals.var_chi__blk814_dn12 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn12)) * locals.var_chi__blk814) + (assign27750_body3_e38781 * locals.var_chi__blk814_dn12)) * assign27750_body3_e38793) + (assign27750_body3_e38783 * ((locals.var_chi__blk814_dn12 * assign27750_body3_e38791) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn12 * 0.006115288895133179))))), ((((((locals.var_chi__blk814_dn17 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn17)) * locals.var_chi__blk814) + (assign27750_body3_e38781 * locals.var_chi__blk814_dn17)) * assign27750_body3_e38793) + (assign27750_body3_e38783 * ((locals.var_chi__blk814_dn17 * assign27750_body3_e38791) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn17 * 0.006115288895133179))))),)
    } else {
        (locals.var_fi, locals.var_fi_dn0, locals.var_fi_dn2, locals.var_fi_dn6, locals.var_fi_dn7, locals.var_fi_dn10, locals.var_fi_dn11, locals.var_fi_dn12, locals.var_fi_dn17,)
    }
};
            locals.var_fi = assign27750_body3_e38796;
            locals.var_fi_dn0 = assign27750_body3_e38796_d_n0;
            locals.var_fi_dn2 = assign27750_body3_e38796_d_n2;
            locals.var_fi_dn6 = assign27750_body3_e38796_d_n6;
            locals.var_fi_dn7 = assign27750_body3_e38796_d_n7;
            locals.var_fi_dn10 = assign27750_body3_e38796_d_n10;
            locals.var_fi_dn11 = assign27750_body3_e38796_d_n11;
            locals.var_fi_dn12 = assign27750_body3_e38796_d_n12;
            locals.var_fi_dn17 = assign27750_body3_e38796_d_n17;
            locals.var_fi_rv = 0.0;
            let (assign27750_body4_e38830, assign27750_body4_e38830_d_n0, assign27750_body4_e38830_d_n2, assign27750_body4_e38830_d_n6, assign27750_body4_e38830_d_n7, assign27750_body4_e38830_d_n10, assign27750_body4_e38830_d_n11, assign27750_body4_e38830_d_n12, assign27750_body4_e38830_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard878 != 0.0)) {
        let assign27750_body4_e38811: f64 = (locals.var_chi__blk814 * locals.var_chi__blk814);
        let assign27750_body4_e38814: f64 = (3.0 * 0.29693154855771);
        let assign27750_body4_e38818: f64 = (-0.07053654284009761);
        let assign27750_body4_e38819: f64 = (4.0 * assign27750_body4_e38818);
        let assign27750_body4_e38822: f64 = (locals.var_chi__blk814 * 5.0);
        let assign27750_body4_e38824: f64 = (assign27750_body4_e38822 * 0.006115288895133179);
        let assign27750_body4_e38825: f64 = (assign27750_body4_e38819 + assign27750_body4_e38824);
        let assign27750_body4_e38826: f64 = (locals.var_chi__blk814 * assign27750_body4_e38825);
        let assign27750_body4_e38827: f64 = (assign27750_body4_e38814 + assign27750_body4_e38826);
        let assign27750_body4_e38828: f64 = (assign27750_body4_e38811 * assign27750_body4_e38827);
        (assign27750_body4_e38828, ((((locals.var_chi__blk814_dn0 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn0)) * assign27750_body4_e38827) + (assign27750_body4_e38811 * ((locals.var_chi__blk814_dn0 * assign27750_body4_e38825) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk814_dn2 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn2)) * assign27750_body4_e38827) + (assign27750_body4_e38811 * ((locals.var_chi__blk814_dn2 * assign27750_body4_e38825) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk814_dn6 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn6)) * assign27750_body4_e38827) + (assign27750_body4_e38811 * ((locals.var_chi__blk814_dn6 * assign27750_body4_e38825) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk814_dn7 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn7)) * assign27750_body4_e38827) + (assign27750_body4_e38811 * ((locals.var_chi__blk814_dn7 * assign27750_body4_e38825) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn7 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk814_dn10 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn10)) * assign27750_body4_e38827) + (assign27750_body4_e38811 * ((locals.var_chi__blk814_dn10 * assign27750_body4_e38825) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk814_dn11 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn11)) * assign27750_body4_e38827) + (assign27750_body4_e38811 * ((locals.var_chi__blk814_dn11 * assign27750_body4_e38825) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk814_dn12 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn12)) * assign27750_body4_e38827) + (assign27750_body4_e38811 * ((locals.var_chi__blk814_dn12 * assign27750_body4_e38825) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn12 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk814_dn17 * locals.var_chi__blk814) + (locals.var_chi__blk814 * locals.var_chi__blk814_dn17)) * assign27750_body4_e38827) + (assign27750_body4_e38811 * ((locals.var_chi__blk814_dn17 * assign27750_body4_e38825) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn17 * 5.0) * 0.006115288895133179))))),)
    } else {
        (locals.var_fi_dchi, locals.var_fi_dchi_dn0, locals.var_fi_dchi_dn2, locals.var_fi_dchi_dn6, locals.var_fi_dchi_dn7, locals.var_fi_dchi_dn10, locals.var_fi_dchi_dn11, locals.var_fi_dchi_dn12, locals.var_fi_dchi_dn17,)
    }
};
            locals.var_fi_dchi = assign27750_body4_e38830;
            locals.var_fi_dchi_dn0 = assign27750_body4_e38830_d_n0;
            locals.var_fi_dchi_dn2 = assign27750_body4_e38830_d_n2;
            locals.var_fi_dchi_dn6 = assign27750_body4_e38830_d_n6;
            locals.var_fi_dchi_dn7 = assign27750_body4_e38830_d_n7;
            locals.var_fi_dchi_dn10 = assign27750_body4_e38830_d_n10;
            locals.var_fi_dchi_dn11 = assign27750_body4_e38830_d_n11;
            locals.var_fi_dchi_dn12 = assign27750_body4_e38830_d_n12;
            locals.var_fi_dchi_dn17 = assign27750_body4_e38830_d_n17;
            locals.var_fi_dchi_rv = 0.0;
            let (assign27750_body5_e38849, assign27750_body5_e38849_d_n0, assign27750_body5_e38849_d_n2, assign27750_body5_e38849_d_n6, assign27750_body5_e38849_d_n7, assign27750_body5_e38849_d_n10, assign27750_body5_e38849_d_n11, assign27750_body5_e38849_d_n12, assign27750_body5_e38849_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard878 != 0.0)) {
        let assign27750_body5_e38845: f64 = (locals.var_cfs1__blk842 * locals.var_fi);
        let assign27750_body5_e38847: f64 = (assign27750_body5_e38845 * locals.var_fi);
        (assign27750_body5_e38847, ((((locals.var_cfs1__blk842_dn0 * locals.var_fi) + (locals.var_cfs1__blk842 * locals.var_fi_dn0)) * locals.var_fi) + (assign27750_body5_e38845 * locals.var_fi_dn0)), ((((locals.var_cfs1__blk842_dn2 * locals.var_fi) + (locals.var_cfs1__blk842 * locals.var_fi_dn2)) * locals.var_fi) + (assign27750_body5_e38845 * locals.var_fi_dn2)), ((((locals.var_cfs1__blk842_dn6 * locals.var_fi) + (locals.var_cfs1__blk842 * locals.var_fi_dn6)) * locals.var_fi) + (assign27750_body5_e38845 * locals.var_fi_dn6)), ((((locals.var_cfs1__blk842_dn7 * locals.var_fi) + (locals.var_cfs1__blk842 * locals.var_fi_dn7)) * locals.var_fi) + (assign27750_body5_e38845 * locals.var_fi_dn7)), ((((locals.var_cfs1__blk842_dn10 * locals.var_fi) + (locals.var_cfs1__blk842 * locals.var_fi_dn10)) * locals.var_fi) + (assign27750_body5_e38845 * locals.var_fi_dn10)), ((((locals.var_cfs1__blk842_dn11 * locals.var_fi) + (locals.var_cfs1__blk842 * locals.var_fi_dn11)) * locals.var_fi) + (assign27750_body5_e38845 * locals.var_fi_dn11)), ((((locals.var_cfs1__blk842_dn12 * locals.var_fi) + (locals.var_cfs1__blk842 * locals.var_fi_dn12)) * locals.var_fi) + (assign27750_body5_e38845 * locals.var_fi_dn12)), ((((locals.var_cfs1__blk842_dn17 * locals.var_fi) + (locals.var_cfs1__blk842 * locals.var_fi_dn17)) * locals.var_fi) + (assign27750_body5_e38845 * locals.var_fi_dn17)),)
    } else {
        (locals.var_fs01__blk836, locals.var_fs01__blk836_dn0, locals.var_fs01__blk836_dn2, locals.var_fs01__blk836_dn6, locals.var_fs01__blk836_dn7, locals.var_fs01__blk836_dn10, locals.var_fs01__blk836_dn11, locals.var_fs01__blk836_dn12, locals.var_fs01__blk836_dn17,)
    }
};
            locals.var_fs01__blk836 = assign27750_body5_e38849;
            locals.var_fs01__blk836_dn0 = assign27750_body5_e38849_d_n0;
            locals.var_fs01__blk836_dn2 = assign27750_body5_e38849_d_n2;
            locals.var_fs01__blk836_dn6 = assign27750_body5_e38849_d_n6;
            locals.var_fs01__blk836_dn7 = assign27750_body5_e38849_d_n7;
            locals.var_fs01__blk836_dn10 = assign27750_body5_e38849_d_n10;
            locals.var_fs01__blk836_dn11 = assign27750_body5_e38849_d_n11;
            locals.var_fs01__blk836_dn12 = assign27750_body5_e38849_d_n12;
            locals.var_fs01__blk836_dn17 = assign27750_body5_e38849_d_n17;
            locals.var_fs01__blk836_rv = 0.0;
            let (assign27750_body6_e38872, assign27750_body6_e38872_d_n0, assign27750_body6_e38872_d_n2, assign27750_body6_e38872_d_n6, assign27750_body6_e38872_d_n7, assign27750_body6_e38872_d_n10, assign27750_body6_e38872_d_n11, assign27750_body6_e38872_d_n12, assign27750_body6_e38872_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard878 != 0.0)) {
        let assign27750_body6_e38864: f64 = (locals.var_cfs1__blk842 * locals.var_beta);
        let assign27750_body6_e38866: f64 = (assign27750_body6_e38864 * 2.0);
        let assign27750_body6_e38868: f64 = (assign27750_body6_e38866 * locals.var_fi);
        let assign27750_body6_e38870: f64 = (assign27750_body6_e38868 * locals.var_fi_dchi);
        (assign27750_body6_e38870, ((((((locals.var_cfs1__blk842_dn0 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27750_body6_e38866 * locals.var_fi_dn0)) * locals.var_fi_dchi) + (assign27750_body6_e38868 * locals.var_fi_dchi_dn0)), ((((((locals.var_cfs1__blk842_dn2 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27750_body6_e38866 * locals.var_fi_dn2)) * locals.var_fi_dchi) + (assign27750_body6_e38868 * locals.var_fi_dchi_dn2)), ((((((locals.var_cfs1__blk842_dn6 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27750_body6_e38866 * locals.var_fi_dn6)) * locals.var_fi_dchi) + (assign27750_body6_e38868 * locals.var_fi_dchi_dn6)), ((((((locals.var_cfs1__blk842_dn7 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27750_body6_e38866 * locals.var_fi_dn7)) * locals.var_fi_dchi) + (assign27750_body6_e38868 * locals.var_fi_dchi_dn7)), (((((((locals.var_cfs1__blk842_dn10 * locals.var_beta) + (locals.var_cfs1__blk842 * locals.var_beta_dn10)) * 2.0) * locals.var_fi) + (assign27750_body6_e38866 * locals.var_fi_dn10)) * locals.var_fi_dchi) + (assign27750_body6_e38868 * locals.var_fi_dchi_dn10)), ((((((locals.var_cfs1__blk842_dn11 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27750_body6_e38866 * locals.var_fi_dn11)) * locals.var_fi_dchi) + (assign27750_body6_e38868 * locals.var_fi_dchi_dn11)), ((((((locals.var_cfs1__blk842_dn12 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27750_body6_e38866 * locals.var_fi_dn12)) * locals.var_fi_dchi) + (assign27750_body6_e38868 * locals.var_fi_dchi_dn12)), ((((((locals.var_cfs1__blk842_dn17 * locals.var_beta) * 2.0) * locals.var_fi) + (assign27750_body6_e38866 * locals.var_fi_dn17)) * locals.var_fi_dchi) + (assign27750_body6_e38868 * locals.var_fi_dchi_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk837, locals.var_fs01_dps0__blk837_dn0, locals.var_fs01_dps0__blk837_dn2, locals.var_fs01_dps0__blk837_dn6, locals.var_fs01_dps0__blk837_dn7, locals.var_fs01_dps0__blk837_dn10, locals.var_fs01_dps0__blk837_dn11, locals.var_fs01_dps0__blk837_dn12, locals.var_fs01_dps0__blk837_dn17,)
    }
};
            locals.var_fs01_dps0__blk837 = assign27750_body6_e38872;
            locals.var_fs01_dps0__blk837_dn0 = assign27750_body6_e38872_d_n0;
            locals.var_fs01_dps0__blk837_dn2 = assign27750_body6_e38872_d_n2;
            locals.var_fs01_dps0__blk837_dn6 = assign27750_body6_e38872_d_n6;
            locals.var_fs01_dps0__blk837_dn7 = assign27750_body6_e38872_d_n7;
            locals.var_fs01_dps0__blk837_dn10 = assign27750_body6_e38872_d_n10;
            locals.var_fs01_dps0__blk837_dn11 = assign27750_body6_e38872_d_n11;
            locals.var_fs01_dps0__blk837_dn12 = assign27750_body6_e38872_d_n12;
            locals.var_fs01_dps0__blk837_dn17 = assign27750_body6_e38872_d_n17;
            locals.var_fs01_dps0__blk837_rv = 0.0;
            let (assign27750_body7_e38907, assign27750_body7_e38907_d_n0, assign27750_body7_e38907_d_n2, assign27750_body7_e38907_d_n6, assign27750_body7_e38907_d_n7, assign27750_body7_e38907_d_n10, assign27750_body7_e38907_d_n11, assign27750_body7_e38907_d_n12, assign27750_body7_e38907_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard878 != 0.0)) {
        let assign27750_body7_e38889: f64 = (-0.117851130197758);
        let assign27750_body7_e38894: f64 = (-0.00163730162779191);
        let assign27750_body7_e38897: f64 = (locals.var_chi__blk814 * 6.36964918866352e-5);
        let assign27750_body7_e38898: f64 = (assign27750_body7_e38894 + assign27750_body7_e38897);
        let assign27750_body7_e38899: f64 = (locals.var_chi__blk814 * assign27750_body7_e38898);
        let assign27750_body7_e38900: f64 = (0.0178800506338833 + assign27750_body7_e38899);
        let assign27750_body7_e38901: f64 = (locals.var_chi__blk814 * assign27750_body7_e38900);
        let assign27750_body7_e38902: f64 = (assign27750_body7_e38889 + assign27750_body7_e38901);
        let assign27750_body7_e38903: f64 = (locals.var_chi__blk814 * assign27750_body7_e38902);
        let assign27750_body7_e38904: f64 = (0.707106781186548 + assign27750_body7_e38903);
        let assign27750_body7_e38905: f64 = (locals.var_chi__blk814 * assign27750_body7_e38904);
        (assign27750_body7_e38905, ((locals.var_chi__blk814_dn0 * assign27750_body7_e38904) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn0 * assign27750_body7_e38902) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn0 * assign27750_body7_e38900) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn0 * assign27750_body7_e38898) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk814_dn2 * assign27750_body7_e38904) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn2 * assign27750_body7_e38902) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn2 * assign27750_body7_e38900) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn2 * assign27750_body7_e38898) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk814_dn6 * assign27750_body7_e38904) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn6 * assign27750_body7_e38902) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn6 * assign27750_body7_e38900) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn6 * assign27750_body7_e38898) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk814_dn7 * assign27750_body7_e38904) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn7 * assign27750_body7_e38902) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn7 * assign27750_body7_e38900) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn7 * assign27750_body7_e38898) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn7 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk814_dn10 * assign27750_body7_e38904) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn10 * assign27750_body7_e38902) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn10 * assign27750_body7_e38900) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn10 * assign27750_body7_e38898) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk814_dn11 * assign27750_body7_e38904) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn11 * assign27750_body7_e38902) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn11 * assign27750_body7_e38900) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn11 * assign27750_body7_e38898) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk814_dn12 * assign27750_body7_e38904) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn12 * assign27750_body7_e38902) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn12 * assign27750_body7_e38900) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn12 * assign27750_body7_e38898) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn12 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk814_dn17 * assign27750_body7_e38904) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn17 * assign27750_body7_e38902) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn17 * assign27750_body7_e38900) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn17 * assign27750_body7_e38898) + (locals.var_chi__blk814 * (locals.var_chi__blk814_dn17 * 6.36964918866352e-5))))))))),)
    } else {
        (locals.var_fb__blk838, locals.var_fb__blk838_dn0, locals.var_fb__blk838_dn2, locals.var_fb__blk838_dn6, locals.var_fb__blk838_dn7, locals.var_fb__blk838_dn10, locals.var_fb__blk838_dn11, locals.var_fb__blk838_dn12, locals.var_fb__blk838_dn17,)
    }
};
            locals.var_fb__blk838 = assign27750_body7_e38907;
            locals.var_fb__blk838_dn0 = assign27750_body7_e38907_d_n0;
            locals.var_fb__blk838_dn2 = assign27750_body7_e38907_d_n2;
            locals.var_fb__blk838_dn6 = assign27750_body7_e38907_d_n6;
            locals.var_fb__blk838_dn7 = assign27750_body7_e38907_d_n7;
            locals.var_fb__blk838_dn10 = assign27750_body7_e38907_d_n10;
            locals.var_fb__blk838_dn11 = assign27750_body7_e38907_d_n11;
            locals.var_fb__blk838_dn12 = assign27750_body7_e38907_d_n12;
            locals.var_fb__blk838_dn17 = assign27750_body7_e38907_d_n17;
            locals.var_fb__blk838_rv = 0.0;
            let (assign27750_body8_e38948, assign27750_body8_e38948_d_n0, assign27750_body8_e38948_d_n2, assign27750_body8_e38948_d_n6, assign27750_body8_e38948_d_n7, assign27750_body8_e38948_d_n10, assign27750_body8_e38948_d_n11, assign27750_body8_e38948_d_n12, assign27750_body8_e38948_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard878 != 0.0)) {
        let assign27750_body8_e38924: f64 = (-0.117851130197758);
        let assign27750_body8_e38925: f64 = (2.0 * assign27750_body8_e38924);
        let assign27750_body8_e38929: f64 = (3.0 * 0.0178800506338833);
        let assign27750_body8_e38933: f64 = (-0.00163730162779191);
        let assign27750_body8_e38934: f64 = (4.0 * assign27750_body8_e38933);
        let assign27750_body8_e38937: f64 = (locals.var_chi__blk814 * 5.0);
        let assign27750_body8_e38939: f64 = (assign27750_body8_e38937 * 6.36964918866352e-5);
        let assign27750_body8_e38940: f64 = (assign27750_body8_e38934 + assign27750_body8_e38939);
        let assign27750_body8_e38941: f64 = (locals.var_chi__blk814 * assign27750_body8_e38940);
        let assign27750_body8_e38942: f64 = (assign27750_body8_e38929 + assign27750_body8_e38941);
        let assign27750_body8_e38943: f64 = (locals.var_chi__blk814 * assign27750_body8_e38942);
        let assign27750_body8_e38944: f64 = (assign27750_body8_e38925 + assign27750_body8_e38943);
        let assign27750_body8_e38945: f64 = (locals.var_chi__blk814 * assign27750_body8_e38944);
        let assign27750_body8_e38946: f64 = (0.707106781186548 + assign27750_body8_e38945);
        (assign27750_body8_e38946, ((locals.var_chi__blk814_dn0 * assign27750_body8_e38944) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn0 * assign27750_body8_e38942) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn0 * assign27750_body8_e38940) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk814_dn2 * assign27750_body8_e38944) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn2 * assign27750_body8_e38942) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn2 * assign27750_body8_e38940) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk814_dn6 * assign27750_body8_e38944) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn6 * assign27750_body8_e38942) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn6 * assign27750_body8_e38940) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk814_dn7 * assign27750_body8_e38944) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn7 * assign27750_body8_e38942) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn7 * assign27750_body8_e38940) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn7 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk814_dn10 * assign27750_body8_e38944) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn10 * assign27750_body8_e38942) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn10 * assign27750_body8_e38940) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk814_dn11 * assign27750_body8_e38944) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn11 * assign27750_body8_e38942) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn11 * assign27750_body8_e38940) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk814_dn12 * assign27750_body8_e38944) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn12 * assign27750_body8_e38942) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn12 * assign27750_body8_e38940) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn12 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk814_dn17 * assign27750_body8_e38944) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn17 * assign27750_body8_e38942) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn17 * assign27750_body8_e38940) + (locals.var_chi__blk814 * ((locals.var_chi__blk814_dn17 * 5.0) * 6.36964918866352e-5))))))),)
    } else {
        (locals.var_fb_dchi, locals.var_fb_dchi_dn0, locals.var_fb_dchi_dn2, locals.var_fb_dchi_dn6, locals.var_fb_dchi_dn7, locals.var_fb_dchi_dn10, locals.var_fb_dchi_dn11, locals.var_fb_dchi_dn12, locals.var_fb_dchi_dn17,)
    }
};
            locals.var_fb_dchi = assign27750_body8_e38948;
            locals.var_fb_dchi_dn0 = assign27750_body8_e38948_d_n0;
            locals.var_fb_dchi_dn2 = assign27750_body8_e38948_d_n2;
            locals.var_fb_dchi_dn6 = assign27750_body8_e38948_d_n6;
            locals.var_fb_dchi_dn7 = assign27750_body8_e38948_d_n7;
            locals.var_fb_dchi_dn10 = assign27750_body8_e38948_d_n10;
            locals.var_fb_dchi_dn11 = assign27750_body8_e38948_d_n11;
            locals.var_fb_dchi_dn12 = assign27750_body8_e38948_d_n12;
            locals.var_fb_dchi_dn17 = assign27750_body8_e38948_d_n17;
            locals.var_fb_dchi_rv = 0.0;
            let (assign27750_body9_e38970, assign27750_body9_e38970_d_n0, assign27750_body9_e38970_d_n2, assign27750_body9_e38970_d_n6, assign27750_body9_e38970_d_n7, assign27750_body9_e38970_d_n10, assign27750_body9_e38970_d_n11, assign27750_body9_e38970_d_n12, assign27750_body9_e38970_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard878 != 0.0)) {
        let assign27750_body9_e38963: f64 = (locals.var_fb__blk838 * locals.var_fb__blk838);
        let assign27750_body9_e38965: f64 = (assign27750_body9_e38963 + locals.var_fs01__blk836);
        let assign27750_body9_e38967: f64 = (assign27750_body9_e38965 + 1e-50);
        let assign27750_body9_e38968: f64 = (assign27750_body9_e38967).sqrt();
        (assign27750_body9_e38968, ((((locals.var_fb__blk838_dn0 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn0)) + locals.var_fs01__blk836_dn0) / (2.0 * assign27750_body9_e38968)), ((((locals.var_fb__blk838_dn2 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn2)) + locals.var_fs01__blk836_dn2) / (2.0 * assign27750_body9_e38968)), ((((locals.var_fb__blk838_dn6 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn6)) + locals.var_fs01__blk836_dn6) / (2.0 * assign27750_body9_e38968)), ((((locals.var_fb__blk838_dn7 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn7)) + locals.var_fs01__blk836_dn7) / (2.0 * assign27750_body9_e38968)), ((((locals.var_fb__blk838_dn10 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn10)) + locals.var_fs01__blk836_dn10) / (2.0 * assign27750_body9_e38968)), ((((locals.var_fb__blk838_dn11 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn11)) + locals.var_fs01__blk836_dn11) / (2.0 * assign27750_body9_e38968)), ((((locals.var_fb__blk838_dn12 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn12)) + locals.var_fs01__blk836_dn12) / (2.0 * assign27750_body9_e38968)), ((((locals.var_fb__blk838_dn17 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn17)) + locals.var_fs01__blk836_dn17) / (2.0 * assign27750_body9_e38968)),)
    } else {
        (locals.var_fs02__blk840, locals.var_fs02__blk840_dn0, locals.var_fs02__blk840_dn2, locals.var_fs02__blk840_dn6, locals.var_fs02__blk840_dn7, locals.var_fs02__blk840_dn10, locals.var_fs02__blk840_dn11, locals.var_fs02__blk840_dn12, locals.var_fs02__blk840_dn17,)
    }
};
            locals.var_fs02__blk840 = assign27750_body9_e38970;
            locals.var_fs02__blk840_dn0 = assign27750_body9_e38970_d_n0;
            locals.var_fs02__blk840_dn2 = assign27750_body9_e38970_d_n2;
            locals.var_fs02__blk840_dn6 = assign27750_body9_e38970_d_n6;
            locals.var_fs02__blk840_dn7 = assign27750_body9_e38970_d_n7;
            locals.var_fs02__blk840_dn10 = assign27750_body9_e38970_d_n10;
            locals.var_fs02__blk840_dn11 = assign27750_body9_e38970_d_n11;
            locals.var_fs02__blk840_dn12 = assign27750_body9_e38970_d_n12;
            locals.var_fs02__blk840_dn17 = assign27750_body9_e38970_d_n17;
            locals.var_fs02__blk840_rv = 0.0;
            let (assign27750_body10_e38997, assign27750_body10_e38997_d_n0, assign27750_body10_e38997_d_n2, assign27750_body10_e38997_d_n6, assign27750_body10_e38997_d_n7, assign27750_body10_e38997_d_n10, assign27750_body10_e38997_d_n11, assign27750_body10_e38997_d_n12, assign27750_body10_e38997_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard878 != 0.0)) {
        let assign27750_body10_e38985: f64 = (locals.var_beta * locals.var_fb_dchi);
        let assign27750_body10_e38987: f64 = (assign27750_body10_e38985 * 2.0);
        let assign27750_body10_e38989: f64 = (assign27750_body10_e38987 * locals.var_fb__blk838);
        let assign27750_body10_e38991: f64 = (assign27750_body10_e38989 + locals.var_fs01_dps0__blk837);
        let assign27750_body10_e38994: f64 = (locals.var_fs02__blk840 + locals.var_fs02__blk840);
        let assign27750_body10_e38995: f64 = (assign27750_body10_e38991 / assign27750_body10_e38994);
        (assign27750_body10_e38995, ((((((((locals.var_beta * locals.var_fb_dchi_dn0) * 2.0) * locals.var_fb__blk838) + (assign27750_body10_e38987 * locals.var_fb__blk838_dn0)) + locals.var_fs01_dps0__blk837_dn0) * assign27750_body10_e38994) - (assign27750_body10_e38991 * (locals.var_fs02__blk840_dn0 + locals.var_fs02__blk840_dn0))) / (assign27750_body10_e38994 * assign27750_body10_e38994)), ((((((((locals.var_beta * locals.var_fb_dchi_dn2) * 2.0) * locals.var_fb__blk838) + (assign27750_body10_e38987 * locals.var_fb__blk838_dn2)) + locals.var_fs01_dps0__blk837_dn2) * assign27750_body10_e38994) - (assign27750_body10_e38991 * (locals.var_fs02__blk840_dn2 + locals.var_fs02__blk840_dn2))) / (assign27750_body10_e38994 * assign27750_body10_e38994)), ((((((((locals.var_beta * locals.var_fb_dchi_dn6) * 2.0) * locals.var_fb__blk838) + (assign27750_body10_e38987 * locals.var_fb__blk838_dn6)) + locals.var_fs01_dps0__blk837_dn6) * assign27750_body10_e38994) - (assign27750_body10_e38991 * (locals.var_fs02__blk840_dn6 + locals.var_fs02__blk840_dn6))) / (assign27750_body10_e38994 * assign27750_body10_e38994)), ((((((((locals.var_beta * locals.var_fb_dchi_dn7) * 2.0) * locals.var_fb__blk838) + (assign27750_body10_e38987 * locals.var_fb__blk838_dn7)) + locals.var_fs01_dps0__blk837_dn7) * assign27750_body10_e38994) - (assign27750_body10_e38991 * (locals.var_fs02__blk840_dn7 + locals.var_fs02__blk840_dn7))) / (assign27750_body10_e38994 * assign27750_body10_e38994)), (((((((((locals.var_beta_dn10 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn10)) * 2.0) * locals.var_fb__blk838) + (assign27750_body10_e38987 * locals.var_fb__blk838_dn10)) + locals.var_fs01_dps0__blk837_dn10) * assign27750_body10_e38994) - (assign27750_body10_e38991 * (locals.var_fs02__blk840_dn10 + locals.var_fs02__blk840_dn10))) / (assign27750_body10_e38994 * assign27750_body10_e38994)), ((((((((locals.var_beta * locals.var_fb_dchi_dn11) * 2.0) * locals.var_fb__blk838) + (assign27750_body10_e38987 * locals.var_fb__blk838_dn11)) + locals.var_fs01_dps0__blk837_dn11) * assign27750_body10_e38994) - (assign27750_body10_e38991 * (locals.var_fs02__blk840_dn11 + locals.var_fs02__blk840_dn11))) / (assign27750_body10_e38994 * assign27750_body10_e38994)), ((((((((locals.var_beta * locals.var_fb_dchi_dn12) * 2.0) * locals.var_fb__blk838) + (assign27750_body10_e38987 * locals.var_fb__blk838_dn12)) + locals.var_fs01_dps0__blk837_dn12) * assign27750_body10_e38994) - (assign27750_body10_e38991 * (locals.var_fs02__blk840_dn12 + locals.var_fs02__blk840_dn12))) / (assign27750_body10_e38994 * assign27750_body10_e38994)), ((((((((locals.var_beta * locals.var_fb_dchi_dn17) * 2.0) * locals.var_fb__blk838) + (assign27750_body10_e38987 * locals.var_fb__blk838_dn17)) + locals.var_fs01_dps0__blk837_dn17) * assign27750_body10_e38994) - (assign27750_body10_e38991 * (locals.var_fs02__blk840_dn17 + locals.var_fs02__blk840_dn17))) / (assign27750_body10_e38994 * assign27750_body10_e38994)),)
    } else {
        (locals.var_fs02_dps0__blk841, locals.var_fs02_dps0__blk841_dn0, locals.var_fs02_dps0__blk841_dn2, locals.var_fs02_dps0__blk841_dn6, locals.var_fs02_dps0__blk841_dn7, locals.var_fs02_dps0__blk841_dn10, locals.var_fs02_dps0__blk841_dn11, locals.var_fs02_dps0__blk841_dn12, locals.var_fs02_dps0__blk841_dn17,)
    }
};
            locals.var_fs02_dps0__blk841 = assign27750_body10_e38997;
            locals.var_fs02_dps0__blk841_dn0 = assign27750_body10_e38997_d_n0;
            locals.var_fs02_dps0__blk841_dn2 = assign27750_body10_e38997_d_n2;
            locals.var_fs02_dps0__blk841_dn6 = assign27750_body10_e38997_d_n6;
            locals.var_fs02_dps0__blk841_dn7 = assign27750_body10_e38997_d_n7;
            locals.var_fs02_dps0__blk841_dn10 = assign27750_body10_e38997_d_n10;
            locals.var_fs02_dps0__blk841_dn11 = assign27750_body10_e38997_d_n11;
            locals.var_fs02_dps0__blk841_dn12 = assign27750_body10_e38997_d_n12;
            locals.var_fs02_dps0__blk841_dn17 = assign27750_body10_e38997_d_n17;
            locals.var_fs02_dps0__blk841_rv = 0.0;
            let assign27750_body11_e39000: f64 = if locals.var_chi__blk814 < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard879 = assign27750_body11_e39000;
            locals.var_guard879_rv = 0.0;
            let (assign27750_body12_e39019, assign27750_body12_e39019_d_n0, assign27750_body12_e39019_d_n2, assign27750_body12_e39019_d_n6, assign27750_body12_e39019_d_n7, assign27750_body12_e39019_d_n10, assign27750_body12_e39019_d_n11, assign27750_body12_e39019_d_n12, assign27750_body12_e39019_d_n17,) = {
    if (((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard878 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27750_body12_e39017: f64 = (locals.var_chi__blk814).exp();
        (assign27750_body12_e39017, (assign27750_body12_e39017 * locals.var_chi__blk814_dn0), (assign27750_body12_e39017 * locals.var_chi__blk814_dn2), (assign27750_body12_e39017 * locals.var_chi__blk814_dn6), (assign27750_body12_e39017 * locals.var_chi__blk814_dn7), (assign27750_body12_e39017 * locals.var_chi__blk814_dn10), (assign27750_body12_e39017 * locals.var_chi__blk814_dn11), (assign27750_body12_e39017 * locals.var_chi__blk814_dn12), (assign27750_body12_e39017 * locals.var_chi__blk814_dn17),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12, locals.var_exp_chi_dn17,)
    }
};
            locals.var_exp_chi = assign27750_body12_e39019;
            locals.var_exp_chi_dn0 = assign27750_body12_e39019_d_n0;
            locals.var_exp_chi_dn2 = assign27750_body12_e39019_d_n2;
            locals.var_exp_chi_dn6 = assign27750_body12_e39019_d_n6;
            locals.var_exp_chi_dn7 = assign27750_body12_e39019_d_n7;
            locals.var_exp_chi_dn10 = assign27750_body12_e39019_d_n10;
            locals.var_exp_chi_dn11 = assign27750_body12_e39019_d_n11;
            locals.var_exp_chi_dn12 = assign27750_body12_e39019_d_n12;
            locals.var_exp_chi_dn17 = assign27750_body12_e39019_d_n17;
            locals.var_exp_chi_rv = 0.0;
            let (assign27750_body13_e39041, assign27750_body13_e39041_d_n0, assign27750_body13_e39041_d_n2, assign27750_body13_e39041_d_n6, assign27750_body13_e39041_d_n7, assign27750_body13_e39041_d_n10, assign27750_body13_e39041_d_n11, assign27750_body13_e39041_d_n12, assign27750_body13_e39041_d_n17,) = {
    if (((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard878 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27750_body13_e39038: f64 = (locals.var_exp_chi - 1.0);
        let assign27750_body13_e39039: f64 = (locals.var_cfs1__blk842 * assign27750_body13_e39038);
        (assign27750_body13_e39039, ((locals.var_cfs1__blk842_dn0 * assign27750_body13_e39038) + (locals.var_cfs1__blk842 * locals.var_exp_chi_dn0)), ((locals.var_cfs1__blk842_dn2 * assign27750_body13_e39038) + (locals.var_cfs1__blk842 * locals.var_exp_chi_dn2)), ((locals.var_cfs1__blk842_dn6 * assign27750_body13_e39038) + (locals.var_cfs1__blk842 * locals.var_exp_chi_dn6)), ((locals.var_cfs1__blk842_dn7 * assign27750_body13_e39038) + (locals.var_cfs1__blk842 * locals.var_exp_chi_dn7)), ((locals.var_cfs1__blk842_dn10 * assign27750_body13_e39038) + (locals.var_cfs1__blk842 * locals.var_exp_chi_dn10)), ((locals.var_cfs1__blk842_dn11 * assign27750_body13_e39038) + (locals.var_cfs1__blk842 * locals.var_exp_chi_dn11)), ((locals.var_cfs1__blk842_dn12 * assign27750_body13_e39038) + (locals.var_cfs1__blk842 * locals.var_exp_chi_dn12)), ((locals.var_cfs1__blk842_dn17 * assign27750_body13_e39038) + (locals.var_cfs1__blk842 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01__blk836, locals.var_fs01__blk836_dn0, locals.var_fs01__blk836_dn2, locals.var_fs01__blk836_dn6, locals.var_fs01__blk836_dn7, locals.var_fs01__blk836_dn10, locals.var_fs01__blk836_dn11, locals.var_fs01__blk836_dn12, locals.var_fs01__blk836_dn17,)
    }
};
            locals.var_fs01__blk836 = assign27750_body13_e39041;
            locals.var_fs01__blk836_dn0 = assign27750_body13_e39041_d_n0;
            locals.var_fs01__blk836_dn2 = assign27750_body13_e39041_d_n2;
            locals.var_fs01__blk836_dn6 = assign27750_body13_e39041_d_n6;
            locals.var_fs01__blk836_dn7 = assign27750_body13_e39041_d_n7;
            locals.var_fs01__blk836_dn10 = assign27750_body13_e39041_d_n10;
            locals.var_fs01__blk836_dn11 = assign27750_body13_e39041_d_n11;
            locals.var_fs01__blk836_dn12 = assign27750_body13_e39041_d_n12;
            locals.var_fs01__blk836_dn17 = assign27750_body13_e39041_d_n17;
            locals.var_fs01__blk836_rv = 0.0;
            let (assign27750_body14_e39063, assign27750_body14_e39063_d_n0, assign27750_body14_e39063_d_n2, assign27750_body14_e39063_d_n6, assign27750_body14_e39063_d_n7, assign27750_body14_e39063_d_n10, assign27750_body14_e39063_d_n11, assign27750_body14_e39063_d_n12, assign27750_body14_e39063_d_n17,) = {
    if (((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard878 == 0.0)) && (locals.var_guard879 != 0.0)) {
        let assign27750_body14_e39059: f64 = (locals.var_cfs1__blk842 * locals.var_beta);
        let assign27750_body14_e39061: f64 = (assign27750_body14_e39059 * locals.var_exp_chi);
        (assign27750_body14_e39061, (((locals.var_cfs1__blk842_dn0 * locals.var_beta) * locals.var_exp_chi) + (assign27750_body14_e39059 * locals.var_exp_chi_dn0)), (((locals.var_cfs1__blk842_dn2 * locals.var_beta) * locals.var_exp_chi) + (assign27750_body14_e39059 * locals.var_exp_chi_dn2)), (((locals.var_cfs1__blk842_dn6 * locals.var_beta) * locals.var_exp_chi) + (assign27750_body14_e39059 * locals.var_exp_chi_dn6)), (((locals.var_cfs1__blk842_dn7 * locals.var_beta) * locals.var_exp_chi) + (assign27750_body14_e39059 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1__blk842_dn10 * locals.var_beta) + (locals.var_cfs1__blk842 * locals.var_beta_dn10)) * locals.var_exp_chi) + (assign27750_body14_e39059 * locals.var_exp_chi_dn10)), (((locals.var_cfs1__blk842_dn11 * locals.var_beta) * locals.var_exp_chi) + (assign27750_body14_e39059 * locals.var_exp_chi_dn11)), (((locals.var_cfs1__blk842_dn12 * locals.var_beta) * locals.var_exp_chi) + (assign27750_body14_e39059 * locals.var_exp_chi_dn12)), (((locals.var_cfs1__blk842_dn17 * locals.var_beta) * locals.var_exp_chi) + (assign27750_body14_e39059 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk837, locals.var_fs01_dps0__blk837_dn0, locals.var_fs01_dps0__blk837_dn2, locals.var_fs01_dps0__blk837_dn6, locals.var_fs01_dps0__blk837_dn7, locals.var_fs01_dps0__blk837_dn10, locals.var_fs01_dps0__blk837_dn11, locals.var_fs01_dps0__blk837_dn12, locals.var_fs01_dps0__blk837_dn17,)
    }
};
            locals.var_fs01_dps0__blk837 = assign27750_body14_e39063;
            locals.var_fs01_dps0__blk837_dn0 = assign27750_body14_e39063_d_n0;
            locals.var_fs01_dps0__blk837_dn2 = assign27750_body14_e39063_d_n2;
            locals.var_fs01_dps0__blk837_dn6 = assign27750_body14_e39063_d_n6;
            locals.var_fs01_dps0__blk837_dn7 = assign27750_body14_e39063_d_n7;
            locals.var_fs01_dps0__blk837_dn10 = assign27750_body14_e39063_d_n10;
            locals.var_fs01_dps0__blk837_dn11 = assign27750_body14_e39063_d_n11;
            locals.var_fs01_dps0__blk837_dn12 = assign27750_body14_e39063_d_n12;
            locals.var_fs01_dps0__blk837_dn17 = assign27750_body14_e39063_d_n17;
            locals.var_fs01_dps0__blk837_rv = 0.0;
            let (assign27750_body15_e39085, assign27750_body15_e39085_d_n0, assign27750_body15_e39085_d_n2, assign27750_body15_e39085_d_n6, assign27750_body15_e39085_d_n7, assign27750_body15_e39085_d_n10, assign27750_body15_e39085_d_n11, assign27750_body15_e39085_d_n12, assign27750_body15_e39085_d_n17,) = {
    if (((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard878 == 0.0)) && (locals.var_guard879 == 0.0)) {
        let assign27750_body15_e39082: f64 = (locals.var_beta * locals.var_ps0ld);
        let assign27750_body15_e39083: f64 = (assign27750_body15_e39082).exp();
        (assign27750_body15_e39083, (assign27750_body15_e39083 * (locals.var_beta * locals.var_ps0ld_dn0)), (assign27750_body15_e39083 * (locals.var_beta * locals.var_ps0ld_dn2)), (assign27750_body15_e39083 * (locals.var_beta * locals.var_ps0ld_dn6)), (assign27750_body15_e39083 * (locals.var_beta * locals.var_ps0ld_dn7)), (assign27750_body15_e39083 * ((locals.var_beta_dn10 * locals.var_ps0ld) + (locals.var_beta * locals.var_ps0ld_dn10))), (assign27750_body15_e39083 * (locals.var_beta * locals.var_ps0ld_dn11)), (assign27750_body15_e39083 * (locals.var_beta * locals.var_ps0ld_dn12)), (assign27750_body15_e39083 * (locals.var_beta * locals.var_ps0ld_dn17)),)
    } else {
        (locals.var_exp_bps0__blk843, locals.var_exp_bps0__blk843_dn0, locals.var_exp_bps0__blk843_dn2, locals.var_exp_bps0__blk843_dn6, locals.var_exp_bps0__blk843_dn7, locals.var_exp_bps0__blk843_dn10, locals.var_exp_bps0__blk843_dn11, locals.var_exp_bps0__blk843_dn12, locals.var_exp_bps0__blk843_dn17,)
    }
};
            locals.var_exp_bps0__blk843 = assign27750_body15_e39085;
            locals.var_exp_bps0__blk843_dn0 = assign27750_body15_e39085_d_n0;
            locals.var_exp_bps0__blk843_dn2 = assign27750_body15_e39085_d_n2;
            locals.var_exp_bps0__blk843_dn6 = assign27750_body15_e39085_d_n6;
            locals.var_exp_bps0__blk843_dn7 = assign27750_body15_e39085_d_n7;
            locals.var_exp_bps0__blk843_dn10 = assign27750_body15_e39085_d_n10;
            locals.var_exp_bps0__blk843_dn11 = assign27750_body15_e39085_d_n11;
            locals.var_exp_bps0__blk843_dn12 = assign27750_body15_e39085_d_n12;
            locals.var_exp_bps0__blk843_dn17 = assign27750_body15_e39085_d_n17;
            locals.var_exp_bps0__blk843_rv = 0.0;
            let (assign27750_body16_e39108, assign27750_body16_e39108_d_n0, assign27750_body16_e39108_d_n2, assign27750_body16_e39108_d_n6, assign27750_body16_e39108_d_n7, assign27750_body16_e39108_d_n10, assign27750_body16_e39108_d_n11, assign27750_body16_e39108_d_n12, assign27750_body16_e39108_d_n17,) = {
    if (((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard878 == 0.0)) && (locals.var_guard879 == 0.0)) {
        let assign27750_body16_e39105: f64 = (locals.var_exp_bps0__blk843 - locals.var_exp_bvbs__blk833);
        let assign27750_body16_e39106: f64 = (locals.var_cnst1over * assign27750_body16_e39105);
        (assign27750_body16_e39106, ((locals.var_cnst1over_dn0 * assign27750_body16_e39105) + (locals.var_cnst1over * (locals.var_exp_bps0__blk843_dn0 - locals.var_exp_bvbs__blk833_dn0))), ((locals.var_cnst1over_dn2 * assign27750_body16_e39105) + (locals.var_cnst1over * (locals.var_exp_bps0__blk843_dn2 - locals.var_exp_bvbs__blk833_dn2))), ((locals.var_cnst1over_dn6 * assign27750_body16_e39105) + (locals.var_cnst1over * (locals.var_exp_bps0__blk843_dn6 - locals.var_exp_bvbs__blk833_dn6))), ((locals.var_cnst1over_dn7 * assign27750_body16_e39105) + (locals.var_cnst1over * (locals.var_exp_bps0__blk843_dn7 - locals.var_exp_bvbs__blk833_dn7))), ((locals.var_cnst1over_dn10 * assign27750_body16_e39105) + (locals.var_cnst1over * (locals.var_exp_bps0__blk843_dn10 - locals.var_exp_bvbs__blk833_dn10))), ((locals.var_cnst1over_dn11 * assign27750_body16_e39105) + (locals.var_cnst1over * (locals.var_exp_bps0__blk843_dn11 - locals.var_exp_bvbs__blk833_dn11))), ((locals.var_cnst1over_dn12 * assign27750_body16_e39105) + (locals.var_cnst1over * (locals.var_exp_bps0__blk843_dn12 - locals.var_exp_bvbs__blk833_dn12))), ((locals.var_cnst1over_dn17 * assign27750_body16_e39105) + (locals.var_cnst1over * (locals.var_exp_bps0__blk843_dn17 - locals.var_exp_bvbs__blk833_dn17))),)
    } else {
        (locals.var_fs01__blk836, locals.var_fs01__blk836_dn0, locals.var_fs01__blk836_dn2, locals.var_fs01__blk836_dn6, locals.var_fs01__blk836_dn7, locals.var_fs01__blk836_dn10, locals.var_fs01__blk836_dn11, locals.var_fs01__blk836_dn12, locals.var_fs01__blk836_dn17,)
    }
};
            locals.var_fs01__blk836 = assign27750_body16_e39108;
            locals.var_fs01__blk836_dn0 = assign27750_body16_e39108_d_n0;
            locals.var_fs01__blk836_dn2 = assign27750_body16_e39108_d_n2;
            locals.var_fs01__blk836_dn6 = assign27750_body16_e39108_d_n6;
            locals.var_fs01__blk836_dn7 = assign27750_body16_e39108_d_n7;
            locals.var_fs01__blk836_dn10 = assign27750_body16_e39108_d_n10;
            locals.var_fs01__blk836_dn11 = assign27750_body16_e39108_d_n11;
            locals.var_fs01__blk836_dn12 = assign27750_body16_e39108_d_n12;
            locals.var_fs01__blk836_dn17 = assign27750_body16_e39108_d_n17;
            locals.var_fs01__blk836_rv = 0.0;
            let (assign27750_body17_e39131, assign27750_body17_e39131_d_n0, assign27750_body17_e39131_d_n2, assign27750_body17_e39131_d_n6, assign27750_body17_e39131_d_n7, assign27750_body17_e39131_d_n10, assign27750_body17_e39131_d_n11, assign27750_body17_e39131_d_n12, assign27750_body17_e39131_d_n17,) = {
    if (((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard878 == 0.0)) && (locals.var_guard879 == 0.0)) {
        let assign27750_body17_e39127: f64 = (locals.var_cnst1over * locals.var_beta);
        let assign27750_body17_e39129: f64 = (assign27750_body17_e39127 * locals.var_exp_bps0__blk843);
        (assign27750_body17_e39129, (((locals.var_cnst1over_dn0 * locals.var_beta) * locals.var_exp_bps0__blk843) + (assign27750_body17_e39127 * locals.var_exp_bps0__blk843_dn0)), (((locals.var_cnst1over_dn2 * locals.var_beta) * locals.var_exp_bps0__blk843) + (assign27750_body17_e39127 * locals.var_exp_bps0__blk843_dn2)), (((locals.var_cnst1over_dn6 * locals.var_beta) * locals.var_exp_bps0__blk843) + (assign27750_body17_e39127 * locals.var_exp_bps0__blk843_dn6)), (((locals.var_cnst1over_dn7 * locals.var_beta) * locals.var_exp_bps0__blk843) + (assign27750_body17_e39127 * locals.var_exp_bps0__blk843_dn7)), ((((locals.var_cnst1over_dn10 * locals.var_beta) + (locals.var_cnst1over * locals.var_beta_dn10)) * locals.var_exp_bps0__blk843) + (assign27750_body17_e39127 * locals.var_exp_bps0__blk843_dn10)), (((locals.var_cnst1over_dn11 * locals.var_beta) * locals.var_exp_bps0__blk843) + (assign27750_body17_e39127 * locals.var_exp_bps0__blk843_dn11)), (((locals.var_cnst1over_dn12 * locals.var_beta) * locals.var_exp_bps0__blk843) + (assign27750_body17_e39127 * locals.var_exp_bps0__blk843_dn12)), (((locals.var_cnst1over_dn17 * locals.var_beta) * locals.var_exp_bps0__blk843) + (assign27750_body17_e39127 * locals.var_exp_bps0__blk843_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk837, locals.var_fs01_dps0__blk837_dn0, locals.var_fs01_dps0__blk837_dn2, locals.var_fs01_dps0__blk837_dn6, locals.var_fs01_dps0__blk837_dn7, locals.var_fs01_dps0__blk837_dn10, locals.var_fs01_dps0__blk837_dn11, locals.var_fs01_dps0__blk837_dn12, locals.var_fs01_dps0__blk837_dn17,)
    }
};
            locals.var_fs01_dps0__blk837 = assign27750_body17_e39131;
            locals.var_fs01_dps0__blk837_dn0 = assign27750_body17_e39131_d_n0;
            locals.var_fs01_dps0__blk837_dn2 = assign27750_body17_e39131_d_n2;
            locals.var_fs01_dps0__blk837_dn6 = assign27750_body17_e39131_d_n6;
            locals.var_fs01_dps0__blk837_dn7 = assign27750_body17_e39131_d_n7;
            locals.var_fs01_dps0__blk837_dn10 = assign27750_body17_e39131_d_n10;
            locals.var_fs01_dps0__blk837_dn11 = assign27750_body17_e39131_d_n11;
            locals.var_fs01_dps0__blk837_dn12 = assign27750_body17_e39131_d_n12;
            locals.var_fs01_dps0__blk837_dn17 = assign27750_body17_e39131_d_n17;
            locals.var_fs01_dps0__blk837_rv = 0.0;
            let (assign27750_body18_e39152, assign27750_body18_e39152_d_n0, assign27750_body18_e39152_d_n2, assign27750_body18_e39152_d_n6, assign27750_body18_e39152_d_n7, assign27750_body18_e39152_d_n10, assign27750_body18_e39152_d_n11, assign27750_body18_e39152_d_n12, assign27750_body18_e39152_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard878 == 0.0)) {
        let assign27750_body18_e39147: f64 = (locals.var_chi__blk814 - 1.0);
        let assign27750_body18_e39149: f64 = (assign27750_body18_e39147 + locals.var_fs01__blk836);
        let assign27750_body18_e39150: f64 = (assign27750_body18_e39149).sqrt();
        (assign27750_body18_e39150, ((locals.var_chi__blk814_dn0 + locals.var_fs01__blk836_dn0) / (2.0 * assign27750_body18_e39150)), ((locals.var_chi__blk814_dn2 + locals.var_fs01__blk836_dn2) / (2.0 * assign27750_body18_e39150)), ((locals.var_chi__blk814_dn6 + locals.var_fs01__blk836_dn6) / (2.0 * assign27750_body18_e39150)), ((locals.var_chi__blk814_dn7 + locals.var_fs01__blk836_dn7) / (2.0 * assign27750_body18_e39150)), ((locals.var_chi__blk814_dn10 + locals.var_fs01__blk836_dn10) / (2.0 * assign27750_body18_e39150)), ((locals.var_chi__blk814_dn11 + locals.var_fs01__blk836_dn11) / (2.0 * assign27750_body18_e39150)), ((locals.var_chi__blk814_dn12 + locals.var_fs01__blk836_dn12) / (2.0 * assign27750_body18_e39150)), ((locals.var_chi__blk814_dn17 + locals.var_fs01__blk836_dn17) / (2.0 * assign27750_body18_e39150)),)
    } else {
        (locals.var_fs02__blk840, locals.var_fs02__blk840_dn0, locals.var_fs02__blk840_dn2, locals.var_fs02__blk840_dn6, locals.var_fs02__blk840_dn7, locals.var_fs02__blk840_dn10, locals.var_fs02__blk840_dn11, locals.var_fs02__blk840_dn12, locals.var_fs02__blk840_dn17,)
    }
};
            locals.var_fs02__blk840 = assign27750_body18_e39152;
            locals.var_fs02__blk840_dn0 = assign27750_body18_e39152_d_n0;
            locals.var_fs02__blk840_dn2 = assign27750_body18_e39152_d_n2;
            locals.var_fs02__blk840_dn6 = assign27750_body18_e39152_d_n6;
            locals.var_fs02__blk840_dn7 = assign27750_body18_e39152_d_n7;
            locals.var_fs02__blk840_dn10 = assign27750_body18_e39152_d_n10;
            locals.var_fs02__blk840_dn11 = assign27750_body18_e39152_d_n11;
            locals.var_fs02__blk840_dn12 = assign27750_body18_e39152_d_n12;
            locals.var_fs02__blk840_dn17 = assign27750_body18_e39152_d_n17;
            locals.var_fs02__blk840_rv = 0.0;
            let (assign27750_body19_e39174, assign27750_body19_e39174_d_n0, assign27750_body19_e39174_d_n2, assign27750_body19_e39174_d_n6, assign27750_body19_e39174_d_n7, assign27750_body19_e39174_d_n10, assign27750_body19_e39174_d_n11, assign27750_body19_e39174_d_n12, assign27750_body19_e39174_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard878 == 0.0)) {
        let assign27750_body19_e39168: f64 = (locals.var_beta + locals.var_fs01_dps0__blk837);
        let assign27750_body19_e39170: f64 = (assign27750_body19_e39168 / locals.var_fs02__blk840);
        let assign27750_body19_e39172: f64 = (assign27750_body19_e39170 * 0.5);
        (assign27750_body19_e39172, ((((locals.var_fs01_dps0__blk837_dn0 * locals.var_fs02__blk840) - (assign27750_body19_e39168 * locals.var_fs02__blk840_dn0)) / (locals.var_fs02__blk840 * locals.var_fs02__blk840)) * 0.5), ((((locals.var_fs01_dps0__blk837_dn2 * locals.var_fs02__blk840) - (assign27750_body19_e39168 * locals.var_fs02__blk840_dn2)) / (locals.var_fs02__blk840 * locals.var_fs02__blk840)) * 0.5), ((((locals.var_fs01_dps0__blk837_dn6 * locals.var_fs02__blk840) - (assign27750_body19_e39168 * locals.var_fs02__blk840_dn6)) / (locals.var_fs02__blk840 * locals.var_fs02__blk840)) * 0.5), ((((locals.var_fs01_dps0__blk837_dn7 * locals.var_fs02__blk840) - (assign27750_body19_e39168 * locals.var_fs02__blk840_dn7)) / (locals.var_fs02__blk840 * locals.var_fs02__blk840)) * 0.5), (((((locals.var_beta_dn10 + locals.var_fs01_dps0__blk837_dn10) * locals.var_fs02__blk840) - (assign27750_body19_e39168 * locals.var_fs02__blk840_dn10)) / (locals.var_fs02__blk840 * locals.var_fs02__blk840)) * 0.5), ((((locals.var_fs01_dps0__blk837_dn11 * locals.var_fs02__blk840) - (assign27750_body19_e39168 * locals.var_fs02__blk840_dn11)) / (locals.var_fs02__blk840 * locals.var_fs02__blk840)) * 0.5), ((((locals.var_fs01_dps0__blk837_dn12 * locals.var_fs02__blk840) - (assign27750_body19_e39168 * locals.var_fs02__blk840_dn12)) / (locals.var_fs02__blk840 * locals.var_fs02__blk840)) * 0.5), ((((locals.var_fs01_dps0__blk837_dn17 * locals.var_fs02__blk840) - (assign27750_body19_e39168 * locals.var_fs02__blk840_dn17)) / (locals.var_fs02__blk840 * locals.var_fs02__blk840)) * 0.5),)
    } else {
        (locals.var_fs02_dps0__blk841, locals.var_fs02_dps0__blk841_dn0, locals.var_fs02_dps0__blk841_dn2, locals.var_fs02_dps0__blk841_dn6, locals.var_fs02_dps0__blk841_dn7, locals.var_fs02_dps0__blk841_dn10, locals.var_fs02_dps0__blk841_dn11, locals.var_fs02_dps0__blk841_dn12, locals.var_fs02_dps0__blk841_dn17,)
    }
};
            locals.var_fs02_dps0__blk841 = assign27750_body19_e39174;
            locals.var_fs02_dps0__blk841_dn0 = assign27750_body19_e39174_d_n0;
            locals.var_fs02_dps0__blk841_dn2 = assign27750_body19_e39174_d_n2;
            locals.var_fs02_dps0__blk841_dn6 = assign27750_body19_e39174_d_n6;
            locals.var_fs02_dps0__blk841_dn7 = assign27750_body19_e39174_d_n7;
            locals.var_fs02_dps0__blk841_dn10 = assign27750_body19_e39174_d_n10;
            locals.var_fs02_dps0__blk841_dn11 = assign27750_body19_e39174_d_n11;
            locals.var_fs02_dps0__blk841_dn12 = assign27750_body19_e39174_d_n12;
            locals.var_fs02_dps0__blk841_dn17 = assign27750_body19_e39174_d_n17;
            locals.var_fs02_dps0__blk841_rv = 0.0;
            let (assign27750_body20_e39193, assign27750_body20_e39193_d_n0, assign27750_body20_e39193_d_n2, assign27750_body20_e39193_d_n6, assign27750_body20_e39193_d_n7, assign27750_body20_e39193_d_n10, assign27750_body20_e39193_d_n11, assign27750_body20_e39193_d_n12, assign27750_body20_e39193_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27750_body20_e39187: f64 = (locals.var_vgpld - locals.var_ps0ld);
        let assign27750_body20_e39190: f64 = (locals.var_fac1__blk800 * locals.var_fs02__blk840);
        let assign27750_body20_e39191: f64 = (assign27750_body20_e39187 - assign27750_body20_e39190);
        (assign27750_body20_e39191, ((locals.var_vgpld_dn0 - locals.var_ps0ld_dn0) - ((locals.var_fac1__blk800_dn0 * locals.var_fs02__blk840) + (locals.var_fac1__blk800 * locals.var_fs02__blk840_dn0))), ((locals.var_vgpld_dn2 - locals.var_ps0ld_dn2) - ((locals.var_fac1__blk800_dn2 * locals.var_fs02__blk840) + (locals.var_fac1__blk800 * locals.var_fs02__blk840_dn2))), ((locals.var_vgpld_dn6 - locals.var_ps0ld_dn6) - ((locals.var_fac1__blk800_dn6 * locals.var_fs02__blk840) + (locals.var_fac1__blk800 * locals.var_fs02__blk840_dn6))), ((locals.var_vgpld_dn7 - locals.var_ps0ld_dn7) - ((locals.var_fac1__blk800_dn7 * locals.var_fs02__blk840) + (locals.var_fac1__blk800 * locals.var_fs02__blk840_dn7))), ((locals.var_vgpld_dn10 - locals.var_ps0ld_dn10) - ((locals.var_fac1__blk800_dn10 * locals.var_fs02__blk840) + (locals.var_fac1__blk800 * locals.var_fs02__blk840_dn10))), ((locals.var_vgpld_dn11 - locals.var_ps0ld_dn11) - ((locals.var_fac1__blk800_dn11 * locals.var_fs02__blk840) + (locals.var_fac1__blk800 * locals.var_fs02__blk840_dn11))), ((locals.var_vgpld_dn12 - locals.var_ps0ld_dn12) - ((locals.var_fac1__blk800_dn12 * locals.var_fs02__blk840) + (locals.var_fac1__blk800 * locals.var_fs02__blk840_dn12))), ((locals.var_vgpld_dn17 - locals.var_ps0ld_dn17) - ((locals.var_fac1__blk800_dn17 * locals.var_fs02__blk840) + (locals.var_fac1__blk800 * locals.var_fs02__blk840_dn17))),)
    } else {
        (locals.var_fs0__blk844, locals.var_fs0__blk844_dn0, locals.var_fs0__blk844_dn2, locals.var_fs0__blk844_dn6, locals.var_fs0__blk844_dn7, locals.var_fs0__blk844_dn10, locals.var_fs0__blk844_dn11, locals.var_fs0__blk844_dn12, locals.var_fs0__blk844_dn17,)
    }
};
            locals.var_fs0__blk844 = assign27750_body20_e39193;
            locals.var_fs0__blk844_dn0 = assign27750_body20_e39193_d_n0;
            locals.var_fs0__blk844_dn2 = assign27750_body20_e39193_d_n2;
            locals.var_fs0__blk844_dn6 = assign27750_body20_e39193_d_n6;
            locals.var_fs0__blk844_dn7 = assign27750_body20_e39193_d_n7;
            locals.var_fs0__blk844_dn10 = assign27750_body20_e39193_d_n10;
            locals.var_fs0__blk844_dn11 = assign27750_body20_e39193_d_n11;
            locals.var_fs0__blk844_dn12 = assign27750_body20_e39193_d_n12;
            locals.var_fs0__blk844_dn17 = assign27750_body20_e39193_d_n17;
            locals.var_fs0__blk844_rv = 0.0;
            let (assign27750_body21_e39211, assign27750_body21_e39211_d_n0, assign27750_body21_e39211_d_n2, assign27750_body21_e39211_d_n6, assign27750_body21_e39211_d_n7, assign27750_body21_e39211_d_n10, assign27750_body21_e39211_d_n11, assign27750_body21_e39211_d_n12, assign27750_body21_e39211_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27750_body21_e39205: f64 = (-1.0);
        let assign27750_body21_e39208: f64 = (locals.var_fac1__blk800 * locals.var_fs02_dps0__blk841);
        let assign27750_body21_e39209: f64 = (assign27750_body21_e39205 - assign27750_body21_e39208);
        (assign27750_body21_e39209, (-((locals.var_fac1__blk800_dn0 * locals.var_fs02_dps0__blk841) + (locals.var_fac1__blk800 * locals.var_fs02_dps0__blk841_dn0))), (-((locals.var_fac1__blk800_dn2 * locals.var_fs02_dps0__blk841) + (locals.var_fac1__blk800 * locals.var_fs02_dps0__blk841_dn2))), (-((locals.var_fac1__blk800_dn6 * locals.var_fs02_dps0__blk841) + (locals.var_fac1__blk800 * locals.var_fs02_dps0__blk841_dn6))), (-((locals.var_fac1__blk800_dn7 * locals.var_fs02_dps0__blk841) + (locals.var_fac1__blk800 * locals.var_fs02_dps0__blk841_dn7))), (-((locals.var_fac1__blk800_dn10 * locals.var_fs02_dps0__blk841) + (locals.var_fac1__blk800 * locals.var_fs02_dps0__blk841_dn10))), (-((locals.var_fac1__blk800_dn11 * locals.var_fs02_dps0__blk841) + (locals.var_fac1__blk800 * locals.var_fs02_dps0__blk841_dn11))), (-((locals.var_fac1__blk800_dn12 * locals.var_fs02_dps0__blk841) + (locals.var_fac1__blk800 * locals.var_fs02_dps0__blk841_dn12))), (-((locals.var_fac1__blk800_dn17 * locals.var_fs02_dps0__blk841) + (locals.var_fac1__blk800 * locals.var_fs02_dps0__blk841_dn17))),)
    } else {
        (locals.var_fs0_dps0__blk845, locals.var_fs0_dps0__blk845_dn0, locals.var_fs0_dps0__blk845_dn2, locals.var_fs0_dps0__blk845_dn6, locals.var_fs0_dps0__blk845_dn7, locals.var_fs0_dps0__blk845_dn10, locals.var_fs0_dps0__blk845_dn11, locals.var_fs0_dps0__blk845_dn12, locals.var_fs0_dps0__blk845_dn17,)
    }
};
            locals.var_fs0_dps0__blk845 = assign27750_body21_e39211;
            locals.var_fs0_dps0__blk845_dn0 = assign27750_body21_e39211_d_n0;
            locals.var_fs0_dps0__blk845_dn2 = assign27750_body21_e39211_d_n2;
            locals.var_fs0_dps0__blk845_dn6 = assign27750_body21_e39211_d_n6;
            locals.var_fs0_dps0__blk845_dn7 = assign27750_body21_e39211_d_n7;
            locals.var_fs0_dps0__blk845_dn10 = assign27750_body21_e39211_d_n10;
            locals.var_fs0_dps0__blk845_dn11 = assign27750_body21_e39211_d_n11;
            locals.var_fs0_dps0__blk845_dn12 = assign27750_body21_e39211_d_n12;
            locals.var_fs0_dps0__blk845_dn17 = assign27750_body21_e39211_d_n17;
            locals.var_fs0_dps0__blk845_rv = 0.0;
            let assign27750_body22_e39214: f64 = if locals.var_flg_conv__blk787 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard880 = assign27750_body22_e39214;
            locals.var_guard880_rv = 0.0;
            let (assign27750_body23_e39233,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard880 != 0.0)) {
        let assign27750_body23_e39229: f64 = (2.0 * 20.0);
        let assign27750_body23_e39231: f64 = (assign27750_body23_e39229 + 1.0);
        (assign27750_body23_e39231,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign27750_body23_e39233;
            locals.var_lp_s0_rv = 0.0;
            let (assign27750_body24_e39252, assign27750_body24_e39252_d_n0, assign27750_body24_e39252_d_n2, assign27750_body24_e39252_d_n6, assign27750_body24_e39252_d_n7, assign27750_body24_e39252_d_n10, assign27750_body24_e39252_d_n11, assign27750_body24_e39252_d_n12, assign27750_body24_e39252_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard880 == 0.0)) {
        let assign27750_body24_e39248: f64 = (-locals.var_fs0__blk844);
        let assign27750_body24_e39250: f64 = (assign27750_body24_e39248 / locals.var_fs0_dps0__blk845);
        (assign27750_body24_e39250, ((((-locals.var_fs0__blk844_dn0) * locals.var_fs0_dps0__blk845) - (assign27750_body24_e39248 * locals.var_fs0_dps0__blk845_dn0)) / (locals.var_fs0_dps0__blk845 * locals.var_fs0_dps0__blk845)), ((((-locals.var_fs0__blk844_dn2) * locals.var_fs0_dps0__blk845) - (assign27750_body24_e39248 * locals.var_fs0_dps0__blk845_dn2)) / (locals.var_fs0_dps0__blk845 * locals.var_fs0_dps0__blk845)), ((((-locals.var_fs0__blk844_dn6) * locals.var_fs0_dps0__blk845) - (assign27750_body24_e39248 * locals.var_fs0_dps0__blk845_dn6)) / (locals.var_fs0_dps0__blk845 * locals.var_fs0_dps0__blk845)), ((((-locals.var_fs0__blk844_dn7) * locals.var_fs0_dps0__blk845) - (assign27750_body24_e39248 * locals.var_fs0_dps0__blk845_dn7)) / (locals.var_fs0_dps0__blk845 * locals.var_fs0_dps0__blk845)), ((((-locals.var_fs0__blk844_dn10) * locals.var_fs0_dps0__blk845) - (assign27750_body24_e39248 * locals.var_fs0_dps0__blk845_dn10)) / (locals.var_fs0_dps0__blk845 * locals.var_fs0_dps0__blk845)), ((((-locals.var_fs0__blk844_dn11) * locals.var_fs0_dps0__blk845) - (assign27750_body24_e39248 * locals.var_fs0_dps0__blk845_dn11)) / (locals.var_fs0_dps0__blk845 * locals.var_fs0_dps0__blk845)), ((((-locals.var_fs0__blk844_dn12) * locals.var_fs0_dps0__blk845) - (assign27750_body24_e39248 * locals.var_fs0_dps0__blk845_dn12)) / (locals.var_fs0_dps0__blk845 * locals.var_fs0_dps0__blk845)), ((((-locals.var_fs0__blk844_dn17) * locals.var_fs0_dps0__blk845) - (assign27750_body24_e39248 * locals.var_fs0_dps0__blk845_dn17)) / (locals.var_fs0_dps0__blk845 * locals.var_fs0_dps0__blk845)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign27750_body24_e39252;
            locals.var_dps0_dn0 = assign27750_body24_e39252_d_n0;
            locals.var_dps0_dn2 = assign27750_body24_e39252_d_n2;
            locals.var_dps0_dn6 = assign27750_body24_e39252_d_n6;
            locals.var_dps0_dn7 = assign27750_body24_e39252_d_n7;
            locals.var_dps0_dn10 = assign27750_body24_e39252_d_n10;
            locals.var_dps0_dn11 = assign27750_body24_e39252_d_n11;
            locals.var_dps0_dn12 = assign27750_body24_e39252_d_n12;
            locals.var_dps0_dn17 = assign27750_body24_e39252_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign27750_body25_e39281, assign27750_body25_e39281_d_n0, assign27750_body25_e39281_d_n2, assign27750_body25_e39281_d_n6, assign27750_body25_e39281_d_n7, assign27750_body25_e39281_d_n10, assign27750_body25_e39281_d_n11, assign27750_body25_e39281_d_n12, assign27750_body25_e39281_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard880 == 0.0)) {
        let assign27750_body25_e39268: f64 = (0.5 * 0.1);
        let assign27750_body25_e39272: f64 = (locals.var_ps0ld).abs();
        let (assign27750_body25_e39277, assign27750_body25_e39277_d_n0, assign27750_body25_e39277_d_n2, assign27750_body25_e39277_d_n6, assign27750_body25_e39277_d_n7, assign27750_body25_e39277_d_n10, assign27750_body25_e39277_d_n11, assign27750_body25_e39277_d_n12, assign27750_body25_e39277_d_n17,) = {
            if (1.0 >= assign27750_body25_e39272) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign27750_body25_e39276: f64 = (locals.var_ps0ld).abs();
                (assign27750_body25_e39276, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn0 } else { (-locals.var_ps0ld_dn0) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn2 } else { (-locals.var_ps0ld_dn2) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn6 } else { (-locals.var_ps0ld_dn6) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn7 } else { (-locals.var_ps0ld_dn7) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn10 } else { (-locals.var_ps0ld_dn10) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn11 } else { (-locals.var_ps0ld_dn11) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn12 } else { (-locals.var_ps0ld_dn12) }, if locals.var_ps0ld >= 0.0 { locals.var_ps0ld_dn17 } else { (-locals.var_ps0ld_dn17) },)
            }
        };
        let assign27750_body25_e39278: f64 = (1.0 + assign27750_body25_e39277);
        let assign27750_body25_e39279: f64 = (assign27750_body25_e39268 * assign27750_body25_e39278);
        (assign27750_body25_e39279, (assign27750_body25_e39268 * assign27750_body25_e39277_d_n0), (assign27750_body25_e39268 * assign27750_body25_e39277_d_n2), (assign27750_body25_e39268 * assign27750_body25_e39277_d_n6), (assign27750_body25_e39268 * assign27750_body25_e39277_d_n7), (assign27750_body25_e39268 * assign27750_body25_e39277_d_n10), (assign27750_body25_e39268 * assign27750_body25_e39277_d_n11), (assign27750_body25_e39268 * assign27750_body25_e39277_d_n12), (assign27750_body25_e39268 * assign27750_body25_e39277_d_n17),)
    } else {
        (locals.var_dplim__blk846, locals.var_dplim__blk846_dn0, locals.var_dplim__blk846_dn2, locals.var_dplim__blk846_dn6, locals.var_dplim__blk846_dn7, locals.var_dplim__blk846_dn10, locals.var_dplim__blk846_dn11, locals.var_dplim__blk846_dn12, locals.var_dplim__blk846_dn17,)
    }
};
            locals.var_dplim__blk846 = assign27750_body25_e39281;
            locals.var_dplim__blk846_dn0 = assign27750_body25_e39281_d_n0;
            locals.var_dplim__blk846_dn2 = assign27750_body25_e39281_d_n2;
            locals.var_dplim__blk846_dn6 = assign27750_body25_e39281_d_n6;
            locals.var_dplim__blk846_dn7 = assign27750_body25_e39281_d_n7;
            locals.var_dplim__blk846_dn10 = assign27750_body25_e39281_d_n10;
            locals.var_dplim__blk846_dn11 = assign27750_body25_e39281_d_n11;
            locals.var_dplim__blk846_dn12 = assign27750_body25_e39281_d_n12;
            locals.var_dplim__blk846_dn17 = assign27750_body25_e39281_d_n17;
            locals.var_dplim__blk846_rv = 0.0;
            let assign27750_body26_e39283: f64 = (locals.var_dps0).abs();
            let assign27750_body26_e39285: f64 = if assign27750_body26_e39283 > locals.var_dplim__blk846 { 1.0 } else { 0.0 };
            locals.var_guard881 = assign27750_body26_e39285;
            locals.var_guard881_rv = 0.0;
            let (assign27750_body27_e39311, assign27750_body27_e39311_d_n0, assign27750_body27_e39311_d_n2, assign27750_body27_e39311_d_n6, assign27750_body27_e39311_d_n7, assign27750_body27_e39311_d_n10, assign27750_body27_e39311_d_n11, assign27750_body27_e39311_d_n12, assign27750_body27_e39311_d_n17,) = {
    if (((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard880 == 0.0)) && (locals.var_guard881 != 0.0)) {
        let (assign27750_body27_e39308,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign27750_body27_e39307: f64 = (-1.0);
                (assign27750_body27_e39307,)
            }
        };
        let assign27750_body27_e39309: f64 = (locals.var_dplim__blk846 * assign27750_body27_e39308);
        (assign27750_body27_e39309, (locals.var_dplim__blk846_dn0 * assign27750_body27_e39308), (locals.var_dplim__blk846_dn2 * assign27750_body27_e39308), (locals.var_dplim__blk846_dn6 * assign27750_body27_e39308), (locals.var_dplim__blk846_dn7 * assign27750_body27_e39308), (locals.var_dplim__blk846_dn10 * assign27750_body27_e39308), (locals.var_dplim__blk846_dn11 * assign27750_body27_e39308), (locals.var_dplim__blk846_dn12 * assign27750_body27_e39308), (locals.var_dplim__blk846_dn17 * assign27750_body27_e39308),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign27750_body27_e39311;
            locals.var_dps0_dn0 = assign27750_body27_e39311_d_n0;
            locals.var_dps0_dn2 = assign27750_body27_e39311_d_n2;
            locals.var_dps0_dn6 = assign27750_body27_e39311_d_n6;
            locals.var_dps0_dn7 = assign27750_body27_e39311_d_n7;
            locals.var_dps0_dn10 = assign27750_body27_e39311_d_n10;
            locals.var_dps0_dn11 = assign27750_body27_e39311_d_n11;
            locals.var_dps0_dn12 = assign27750_body27_e39311_d_n12;
            locals.var_dps0_dn17 = assign27750_body27_e39311_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign27750_body28_e39329, assign27750_body28_e39329_d_n0, assign27750_body28_e39329_d_n2, assign27750_body28_e39329_d_n6, assign27750_body28_e39329_d_n7, assign27750_body28_e39329_d_n10, assign27750_body28_e39329_d_n11, assign27750_body28_e39329_d_n12, assign27750_body28_e39329_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard880 == 0.0)) {
        let assign27750_body28_e39327: f64 = (locals.var_ps0ld + locals.var_dps0);
        (assign27750_body28_e39327, (locals.var_ps0ld_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld_dn12 + locals.var_dps0_dn12), (locals.var_ps0ld_dn17 + locals.var_dps0_dn17),)
    } else {
        (locals.var_ps0ld, locals.var_ps0ld_dn0, locals.var_ps0ld_dn2, locals.var_ps0ld_dn6, locals.var_ps0ld_dn7, locals.var_ps0ld_dn10, locals.var_ps0ld_dn11, locals.var_ps0ld_dn12, locals.var_ps0ld_dn17,)
    }
};
            locals.var_ps0ld = assign27750_body28_e39329;
            locals.var_ps0ld_dn0 = assign27750_body28_e39329_d_n0;
            locals.var_ps0ld_dn2 = assign27750_body28_e39329_d_n2;
            locals.var_ps0ld_dn6 = assign27750_body28_e39329_d_n6;
            locals.var_ps0ld_dn7 = assign27750_body28_e39329_d_n7;
            locals.var_ps0ld_dn10 = assign27750_body28_e39329_d_n10;
            locals.var_ps0ld_dn11 = assign27750_body28_e39329_d_n11;
            locals.var_ps0ld_dn12 = assign27750_body28_e39329_d_n12;
            locals.var_ps0ld_dn17 = assign27750_body28_e39329_d_n17;
            locals.var_ps0ld_rv = 0.0;
            let assign27750_body29_e39331: f64 = (locals.var_dps0).abs();
            let assign27750_body29_e39335: f64 = (locals.var_fs0__blk844).abs();
            let assign27750_body29_e39338: f64 = if ((assign27750_body29_e39331 <= 5e-12) && (assign27750_body29_e39335 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard882 = assign27750_body29_e39338;
            locals.var_guard882_rv = 0.0;
            let (assign27750_body30_e39356,) = {
    if (((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard880 == 0.0)) && (locals.var_guard882 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv__blk787,)
    }
};
            locals.var_flg_conv__blk787 = assign27750_body30_e39356;
            locals.var_flg_conv__blk787_rv = 0.0;
            let (assign27750_body31_e39371,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27750_body31_e39369: f64 = (locals.var_lp_s0 + 1.0);
        (assign27750_body31_e39369,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign27750_body31_e39371;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_101(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign27770_e39377: f64 = if locals.var_chi__blk814 < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard884 = assign27770_e39377;
        locals.var_guard884_rv = 0.0;

        let (assign27810_e39436, assign27810_e39436_d_n0, assign27810_e39436_d_n2, assign27810_e39436_d_n6, assign27810_e39436_d_n7, assign27810_e39436_d_n10, assign27810_e39436_d_n11, assign27810_e39436_d_n12, assign27810_e39436_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard884 != 0.0)) {
        let assign27810_e39430: f64 = (locals.var_fb__blk838 * locals.var_fb__blk838);
        let assign27810_e39433: f64 = (10.0 * 2.220446049250313e-16);
        let assign27810_e39434: f64 = (assign27810_e39430 + assign27810_e39433);
        (assign27810_e39434, ((locals.var_fb__blk838_dn0 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn0)), ((locals.var_fb__blk838_dn2 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn2)), ((locals.var_fb__blk838_dn6 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn6)), ((locals.var_fb__blk838_dn7 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn7)), ((locals.var_fb__blk838_dn10 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn10)), ((locals.var_fb__blk838_dn11 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn11)), ((locals.var_fb__blk838_dn12 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn12)), ((locals.var_fb__blk838_dn17 * locals.var_fb__blk838) + (locals.var_fb__blk838 * locals.var_fb__blk838_dn17)),)
    } else {
        (locals.var_xi0__blk847, locals.var_xi0__blk847_dn0, locals.var_xi0__blk847_dn2, locals.var_xi0__blk847_dn6, locals.var_xi0__blk847_dn7, locals.var_xi0__blk847_dn10, locals.var_xi0__blk847_dn11, locals.var_xi0__blk847_dn12, locals.var_xi0__blk847_dn17,)
    }
};
        locals.var_xi0__blk847 = assign27810_e39436;
        locals.var_xi0__blk847_dn0 = assign27810_e39436_d_n0;
        locals.var_xi0__blk847_dn2 = assign27810_e39436_d_n2;
        locals.var_xi0__blk847_dn6 = assign27810_e39436_d_n6;
        locals.var_xi0__blk847_dn7 = assign27810_e39436_d_n7;
        locals.var_xi0__blk847_dn10 = assign27810_e39436_d_n10;
        locals.var_xi0__blk847_dn11 = assign27810_e39436_d_n11;
        locals.var_xi0__blk847_dn12 = assign27810_e39436_d_n12;
        locals.var_xi0__blk847_dn17 = assign27810_e39436_d_n17;
        locals.var_xi0__blk847_rv = 0.0;

        let (assign27820_e39455, assign27820_e39455_d_n0, assign27820_e39455_d_n2, assign27820_e39455_d_n6, assign27820_e39455_d_n7, assign27820_e39455_d_n10, assign27820_e39455_d_n11, assign27820_e39455_d_n12, assign27820_e39455_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard884 != 0.0)) {
        let assign27820_e39452: f64 = (10.0 * 2.220446049250313e-16);
        let assign27820_e39453: f64 = (locals.var_fb__blk838 + assign27820_e39452);
        (assign27820_e39453, locals.var_fb__blk838_dn0, locals.var_fb__blk838_dn2, locals.var_fb__blk838_dn6, locals.var_fb__blk838_dn7, locals.var_fb__blk838_dn10, locals.var_fb__blk838_dn11, locals.var_fb__blk838_dn12, locals.var_fb__blk838_dn17,)
    } else {
        (locals.var_xi0p12__blk848, locals.var_xi0p12__blk848_dn0, locals.var_xi0p12__blk848_dn2, locals.var_xi0p12__blk848_dn6, locals.var_xi0p12__blk848_dn7, locals.var_xi0p12__blk848_dn10, locals.var_xi0p12__blk848_dn11, locals.var_xi0p12__blk848_dn12, locals.var_xi0p12__blk848_dn17,)
    }
};
        locals.var_xi0p12__blk848 = assign27820_e39455;
        locals.var_xi0p12__blk848_dn0 = assign27820_e39455_d_n0;
        locals.var_xi0p12__blk848_dn2 = assign27820_e39455_d_n2;
        locals.var_xi0p12__blk848_dn6 = assign27820_e39455_d_n6;
        locals.var_xi0p12__blk848_dn7 = assign27820_e39455_d_n7;
        locals.var_xi0p12__blk848_dn10 = assign27820_e39455_d_n10;
        locals.var_xi0p12__blk848_dn11 = assign27820_e39455_d_n11;
        locals.var_xi0p12__blk848_dn12 = assign27820_e39455_d_n12;
        locals.var_xi0p12__blk848_dn17 = assign27820_e39455_d_n17;
        locals.var_xi0p12__blk848_rv = 0.0;

        let (assign27840_e39489, assign27840_e39489_d_n0, assign27840_e39489_d_n2, assign27840_e39489_d_n6, assign27840_e39489_d_n7, assign27840_e39489_d_n10, assign27840_e39489_d_n11, assign27840_e39489_d_n12, assign27840_e39489_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard884 == 0.0)) {
        let assign27840_e39487: f64 = (locals.var_chi__blk814 - 1.0);
        (assign27840_e39487, locals.var_chi__blk814_dn0, locals.var_chi__blk814_dn2, locals.var_chi__blk814_dn6, locals.var_chi__blk814_dn7, locals.var_chi__blk814_dn10, locals.var_chi__blk814_dn11, locals.var_chi__blk814_dn12, locals.var_chi__blk814_dn17,)
    } else {
        (locals.var_xi0__blk847, locals.var_xi0__blk847_dn0, locals.var_xi0__blk847_dn2, locals.var_xi0__blk847_dn6, locals.var_xi0__blk847_dn7, locals.var_xi0__blk847_dn10, locals.var_xi0__blk847_dn11, locals.var_xi0__blk847_dn12, locals.var_xi0__blk847_dn17,)
    }
};
        locals.var_xi0__blk847 = assign27840_e39489;
        locals.var_xi0__blk847_dn0 = assign27840_e39489_d_n0;
        locals.var_xi0__blk847_dn2 = assign27840_e39489_d_n2;
        locals.var_xi0__blk847_dn6 = assign27840_e39489_d_n6;
        locals.var_xi0__blk847_dn7 = assign27840_e39489_d_n7;
        locals.var_xi0__blk847_dn10 = assign27840_e39489_d_n10;
        locals.var_xi0__blk847_dn11 = assign27840_e39489_d_n11;
        locals.var_xi0__blk847_dn12 = assign27840_e39489_d_n12;
        locals.var_xi0__blk847_dn17 = assign27840_e39489_d_n17;
        locals.var_xi0__blk847_rv = 0.0;

        let (assign27850_e39506, assign27850_e39506_d_n0, assign27850_e39506_d_n2, assign27850_e39506_d_n6, assign27850_e39506_d_n7, assign27850_e39506_d_n10, assign27850_e39506_d_n11, assign27850_e39506_d_n12, assign27850_e39506_d_n17,) = {
    if ((((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) && (locals.var_guard884 == 0.0)) {
        let assign27850_e39504: f64 = (locals.var_xi0__blk847).sqrt();
        (assign27850_e39504, (locals.var_xi0__blk847_dn0 / (2.0 * assign27850_e39504)), (locals.var_xi0__blk847_dn2 / (2.0 * assign27850_e39504)), (locals.var_xi0__blk847_dn6 / (2.0 * assign27850_e39504)), (locals.var_xi0__blk847_dn7 / (2.0 * assign27850_e39504)), (locals.var_xi0__blk847_dn10 / (2.0 * assign27850_e39504)), (locals.var_xi0__blk847_dn11 / (2.0 * assign27850_e39504)), (locals.var_xi0__blk847_dn12 / (2.0 * assign27850_e39504)), (locals.var_xi0__blk847_dn17 / (2.0 * assign27850_e39504)),)
    } else {
        (locals.var_xi0p12__blk848, locals.var_xi0p12__blk848_dn0, locals.var_xi0p12__blk848_dn2, locals.var_xi0p12__blk848_dn6, locals.var_xi0p12__blk848_dn7, locals.var_xi0p12__blk848_dn10, locals.var_xi0p12__blk848_dn11, locals.var_xi0p12__blk848_dn12, locals.var_xi0p12__blk848_dn17,)
    }
};
        locals.var_xi0p12__blk848 = assign27850_e39506;
        locals.var_xi0p12__blk848_dn0 = assign27850_e39506_d_n0;
        locals.var_xi0p12__blk848_dn2 = assign27850_e39506_d_n2;
        locals.var_xi0p12__blk848_dn6 = assign27850_e39506_d_n6;
        locals.var_xi0p12__blk848_dn7 = assign27850_e39506_d_n7;
        locals.var_xi0p12__blk848_dn10 = assign27850_e39506_d_n10;
        locals.var_xi0p12__blk848_dn11 = assign27850_e39506_d_n11;
        locals.var_xi0p12__blk848_dn12 = assign27850_e39506_d_n12;
        locals.var_xi0p12__blk848_dn17 = assign27850_e39506_d_n17;
        locals.var_xi0p12__blk848_rv = 0.0;

        let (assign27860_e39521, assign27860_e39521_d_n0, assign27860_e39521_d_n2, assign27860_e39521_d_n6, assign27860_e39521_d_n7, assign27860_e39521_d_n10, assign27860_e39521_d_n11, assign27860_e39521_d_n12, assign27860_e39521_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27860_e39519: f64 = (locals.var_cnst0over * locals.var_xi0p12__blk848);
        (assign27860_e39519, ((locals.var_cnst0over_dn0 * locals.var_xi0p12__blk848) + (locals.var_cnst0over * locals.var_xi0p12__blk848_dn0)), ((locals.var_cnst0over_dn2 * locals.var_xi0p12__blk848) + (locals.var_cnst0over * locals.var_xi0p12__blk848_dn2)), ((locals.var_cnst0over_dn6 * locals.var_xi0p12__blk848) + (locals.var_cnst0over * locals.var_xi0p12__blk848_dn6)), ((locals.var_cnst0over_dn7 * locals.var_xi0p12__blk848) + (locals.var_cnst0over * locals.var_xi0p12__blk848_dn7)), ((locals.var_cnst0over_dn10 * locals.var_xi0p12__blk848) + (locals.var_cnst0over * locals.var_xi0p12__blk848_dn10)), ((locals.var_cnst0over_dn11 * locals.var_xi0p12__blk848) + (locals.var_cnst0over * locals.var_xi0p12__blk848_dn11)), ((locals.var_cnst0over_dn12 * locals.var_xi0p12__blk848) + (locals.var_cnst0over * locals.var_xi0p12__blk848_dn12)), ((locals.var_cnst0over_dn17 * locals.var_xi0p12__blk848) + (locals.var_cnst0over * locals.var_xi0p12__blk848_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign27860_e39521;
        locals.var_qbuld_dn0 = assign27860_e39521_d_n0;
        locals.var_qbuld_dn2 = assign27860_e39521_d_n2;
        locals.var_qbuld_dn6 = assign27860_e39521_d_n6;
        locals.var_qbuld_dn7 = assign27860_e39521_d_n7;
        locals.var_qbuld_dn10 = assign27860_e39521_d_n10;
        locals.var_qbuld_dn11 = assign27860_e39521_d_n11;
        locals.var_qbuld_dn12 = assign27860_e39521_d_n12;
        locals.var_qbuld_dn17 = assign27860_e39521_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign27870_e39538, assign27870_e39538_d_n0, assign27870_e39538_d_n2, assign27870_e39538_d_n6, assign27870_e39538_d_n7, assign27870_e39538_d_n10, assign27870_e39538_d_n11, assign27870_e39538_d_n12, assign27870_e39538_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27870_e39535: f64 = (locals.var_fs02__blk840 + locals.var_xi0p12__blk848);
        let assign27870_e39536: f64 = (1.0 / assign27870_e39535);
        (assign27870_e39536, (-((locals.var_fs02__blk840_dn0 + locals.var_xi0p12__blk848_dn0) / (assign27870_e39535 * assign27870_e39535))), (-((locals.var_fs02__blk840_dn2 + locals.var_xi0p12__blk848_dn2) / (assign27870_e39535 * assign27870_e39535))), (-((locals.var_fs02__blk840_dn6 + locals.var_xi0p12__blk848_dn6) / (assign27870_e39535 * assign27870_e39535))), (-((locals.var_fs02__blk840_dn7 + locals.var_xi0p12__blk848_dn7) / (assign27870_e39535 * assign27870_e39535))), (-((locals.var_fs02__blk840_dn10 + locals.var_xi0p12__blk848_dn10) / (assign27870_e39535 * assign27870_e39535))), (-((locals.var_fs02__blk840_dn11 + locals.var_xi0p12__blk848_dn11) / (assign27870_e39535 * assign27870_e39535))), (-((locals.var_fs02__blk840_dn12 + locals.var_xi0p12__blk848_dn12) / (assign27870_e39535 * assign27870_e39535))), (-((locals.var_fs02__blk840_dn17 + locals.var_xi0p12__blk848_dn17) / (assign27870_e39535 * assign27870_e39535))),)
    } else {
        (locals.var_t1__blk771, locals.var_t1__blk771_dn0, locals.var_t1__blk771_dn2, locals.var_t1__blk771_dn6, locals.var_t1__blk771_dn7, locals.var_t1__blk771_dn10, locals.var_t1__blk771_dn11, locals.var_t1__blk771_dn12, locals.var_t1__blk771_dn17,)
    }
};
        locals.var_t1__blk771 = assign27870_e39538;
        locals.var_t1__blk771_dn0 = assign27870_e39538_d_n0;
        locals.var_t1__blk771_dn2 = assign27870_e39538_d_n2;
        locals.var_t1__blk771_dn6 = assign27870_e39538_d_n6;
        locals.var_t1__blk771_dn7 = assign27870_e39538_d_n7;
        locals.var_t1__blk771_dn10 = assign27870_e39538_d_n10;
        locals.var_t1__blk771_dn11 = assign27870_e39538_d_n11;
        locals.var_t1__blk771_dn12 = assign27870_e39538_d_n12;
        locals.var_t1__blk771_dn17 = assign27870_e39538_d_n17;
        locals.var_t1__blk771_rv = 0.0;

        let (assign27880_e39555, assign27880_e39555_d_n0, assign27880_e39555_d_n2, assign27880_e39555_d_n6, assign27880_e39555_d_n7, assign27880_e39555_d_n10, assign27880_e39555_d_n11, assign27880_e39555_d_n12, assign27880_e39555_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27880_e39551: f64 = (locals.var_cnst0over * locals.var_fs01__blk836);
        let assign27880_e39553: f64 = (assign27880_e39551 * locals.var_t1__blk771);
        (assign27880_e39553, ((((locals.var_cnst0over_dn0 * locals.var_fs01__blk836) + (locals.var_cnst0over * locals.var_fs01__blk836_dn0)) * locals.var_t1__blk771) + (assign27880_e39551 * locals.var_t1__blk771_dn0)), ((((locals.var_cnst0over_dn2 * locals.var_fs01__blk836) + (locals.var_cnst0over * locals.var_fs01__blk836_dn2)) * locals.var_t1__blk771) + (assign27880_e39551 * locals.var_t1__blk771_dn2)), ((((locals.var_cnst0over_dn6 * locals.var_fs01__blk836) + (locals.var_cnst0over * locals.var_fs01__blk836_dn6)) * locals.var_t1__blk771) + (assign27880_e39551 * locals.var_t1__blk771_dn6)), ((((locals.var_cnst0over_dn7 * locals.var_fs01__blk836) + (locals.var_cnst0over * locals.var_fs01__blk836_dn7)) * locals.var_t1__blk771) + (assign27880_e39551 * locals.var_t1__blk771_dn7)), ((((locals.var_cnst0over_dn10 * locals.var_fs01__blk836) + (locals.var_cnst0over * locals.var_fs01__blk836_dn10)) * locals.var_t1__blk771) + (assign27880_e39551 * locals.var_t1__blk771_dn10)), ((((locals.var_cnst0over_dn11 * locals.var_fs01__blk836) + (locals.var_cnst0over * locals.var_fs01__blk836_dn11)) * locals.var_t1__blk771) + (assign27880_e39551 * locals.var_t1__blk771_dn11)), ((((locals.var_cnst0over_dn12 * locals.var_fs01__blk836) + (locals.var_cnst0over * locals.var_fs01__blk836_dn12)) * locals.var_t1__blk771) + (assign27880_e39551 * locals.var_t1__blk771_dn12)), ((((locals.var_cnst0over_dn17 * locals.var_fs01__blk836) + (locals.var_cnst0over * locals.var_fs01__blk836_dn17)) * locals.var_t1__blk771) + (assign27880_e39551 * locals.var_t1__blk771_dn17)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign27880_e39555;
        locals.var_qiuld_dn0 = assign27880_e39555_d_n0;
        locals.var_qiuld_dn2 = assign27880_e39555_d_n2;
        locals.var_qiuld_dn6 = assign27880_e39555_d_n6;
        locals.var_qiuld_dn7 = assign27880_e39555_d_n7;
        locals.var_qiuld_dn10 = assign27880_e39555_d_n10;
        locals.var_qiuld_dn11 = assign27880_e39555_d_n11;
        locals.var_qiuld_dn12 = assign27880_e39555_d_n12;
        locals.var_qiuld_dn17 = assign27880_e39555_d_n17;
        locals.var_qiuld_rv = 0.0;

        let (assign27890_e39570, assign27890_e39570_d_n0, assign27890_e39570_d_n2, assign27890_e39570_d_n6, assign27890_e39570_d_n7, assign27890_e39570_d_n10, assign27890_e39570_d_n11, assign27890_e39570_d_n12, assign27890_e39570_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard871 == 0.0)) && (locals.var_guard877 != 0.0)) {
        let assign27890_e39568: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign27890_e39568, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn12 + locals.var_qiuld_dn12), (locals.var_qbuld_dn17 + locals.var_qiuld_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign27890_e39570;
        locals.var_qsuld_dn0 = assign27890_e39570_d_n0;
        locals.var_qsuld_dn2 = assign27890_e39570_d_n2;
        locals.var_qsuld_dn6 = assign27890_e39570_d_n6;
        locals.var_qsuld_dn7 = assign27890_e39570_d_n7;
        locals.var_qsuld_dn10 = assign27890_e39570_d_n10;
        locals.var_qsuld_dn11 = assign27890_e39570_d_n11;
        locals.var_qsuld_dn12 = assign27890_e39570_d_n12;
        locals.var_qsuld_dn17 = assign27890_e39570_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign27900_e39580, assign27900_e39580_d_n0, assign27900_e39580_d_n2, assign27900_e39580_d_n6, assign27900_e39580_d_n7, assign27900_e39580_d_n10, assign27900_e39580_d_n11, assign27900_e39580_d_n12, assign27900_e39580_d_n17,) = {
    if (((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) {
        let assign27900_e39578: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign27900_e39578, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn12 - locals.var_qbuld_dn12), (locals.var_qsuld_dn17 - locals.var_qbuld_dn17),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign27900_e39580;
        locals.var_qiuld_dn0 = assign27900_e39580_d_n0;
        locals.var_qiuld_dn2 = assign27900_e39580_d_n2;
        locals.var_qiuld_dn6 = assign27900_e39580_d_n6;
        locals.var_qiuld_dn7 = assign27900_e39580_d_n7;
        locals.var_qiuld_dn10 = assign27900_e39580_d_n10;
        locals.var_qiuld_dn11 = assign27900_e39580_d_n11;
        locals.var_qiuld_dn12 = assign27900_e39580_d_n12;
        locals.var_qiuld_dn17 = assign27900_e39580_d_n17;
        locals.var_qiuld_rv = 0.0;

        let assign27910_e39583: f64 = if 1.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard886 = assign27910_e39583;
        locals.var_guard886_rv = 0.0;

        let assign27920_e39586: f64 = if 1.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard887 = assign27920_e39586;
        locals.var_guard887_rv = 0.0;

        let (assign27930_e39601, assign27930_e39601_d_n0, assign27930_e39601_d_n2, assign27930_e39601_d_n6, assign27930_e39601_d_n7, assign27930_e39601_d_n10, assign27930_e39601_d_n11, assign27930_e39601_d_n12, assign27930_e39601_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard886 != 0.0)) && (locals.var_flg_ovloops != 0.0)) {
        let assign27930_e39597: f64 = (-locals.var_uc_areabt);
        let assign27930_e39599: f64 = (assign27930_e39597 * locals.var_qsuld);
        (assign27930_e39599, (assign27930_e39597 * locals.var_qsuld_dn0), (assign27930_e39597 * locals.var_qsuld_dn2), (assign27930_e39597 * locals.var_qsuld_dn6), (assign27930_e39597 * locals.var_qsuld_dn7), (assign27930_e39597 * locals.var_qsuld_dn10), (assign27930_e39597 * locals.var_qsuld_dn11), (assign27930_e39597 * locals.var_qsuld_dn12), (assign27930_e39597 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_sus, locals.var_qbody_bt_p_sus_dn0, locals.var_qbody_bt_p_sus_dn2, locals.var_qbody_bt_p_sus_dn6, locals.var_qbody_bt_p_sus_dn7, locals.var_qbody_bt_p_sus_dn10, locals.var_qbody_bt_p_sus_dn11, locals.var_qbody_bt_p_sus_dn12, locals.var_qbody_bt_p_sus_dn17,)
    }
};
        locals.var_qbody_bt_p_sus = assign27930_e39601;
        locals.var_qbody_bt_p_sus_dn0 = assign27930_e39601_d_n0;
        locals.var_qbody_bt_p_sus_dn2 = assign27930_e39601_d_n2;
        locals.var_qbody_bt_p_sus_dn6 = assign27930_e39601_d_n6;
        locals.var_qbody_bt_p_sus_dn7 = assign27930_e39601_d_n7;
        locals.var_qbody_bt_p_sus_dn10 = assign27930_e39601_d_n10;
        locals.var_qbody_bt_p_sus_dn11 = assign27930_e39601_d_n11;
        locals.var_qbody_bt_p_sus_dn12 = assign27930_e39601_d_n12;
        locals.var_qbody_bt_p_sus_dn17 = assign27930_e39601_d_n17;
        locals.var_qbody_bt_p_sus_rv = 0.0;

        let (assign27940_e39616, assign27940_e39616_d_n0, assign27940_e39616_d_n2, assign27940_e39616_d_n6, assign27940_e39616_d_n7, assign27940_e39616_d_n10, assign27940_e39616_d_n11, assign27940_e39616_d_n12, assign27940_e39616_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard886 != 0.0)) && (locals.var_flg_ovloops != 0.0)) {
        let assign27940_e39612: f64 = (-locals.var_uc_areabt);
        let assign27940_e39614: f64 = (assign27940_e39612 * locals.var_qiuld);
        (assign27940_e39614, (assign27940_e39612 * locals.var_qiuld_dn0), (assign27940_e39612 * locals.var_qiuld_dn2), (assign27940_e39612 * locals.var_qiuld_dn6), (assign27940_e39612 * locals.var_qiuld_dn7), (assign27940_e39612 * locals.var_qiuld_dn10), (assign27940_e39612 * locals.var_qiuld_dn11), (assign27940_e39612 * locals.var_qiuld_dn12), (assign27940_e39612 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_ius, locals.var_qbody_bt_p_ius_dn0, locals.var_qbody_bt_p_ius_dn2, locals.var_qbody_bt_p_ius_dn6, locals.var_qbody_bt_p_ius_dn7, locals.var_qbody_bt_p_ius_dn10, locals.var_qbody_bt_p_ius_dn11, locals.var_qbody_bt_p_ius_dn12, locals.var_qbody_bt_p_ius_dn17,)
    }
};
        locals.var_qbody_bt_p_ius = assign27940_e39616;
        locals.var_qbody_bt_p_ius_dn0 = assign27940_e39616_d_n0;
        locals.var_qbody_bt_p_ius_dn2 = assign27940_e39616_d_n2;
        locals.var_qbody_bt_p_ius_dn6 = assign27940_e39616_d_n6;
        locals.var_qbody_bt_p_ius_dn7 = assign27940_e39616_d_n7;
        locals.var_qbody_bt_p_ius_dn10 = assign27940_e39616_d_n10;
        locals.var_qbody_bt_p_ius_dn11 = assign27940_e39616_d_n11;
        locals.var_qbody_bt_p_ius_dn12 = assign27940_e39616_d_n12;
        locals.var_qbody_bt_p_ius_dn17 = assign27940_e39616_d_n17;
        locals.var_qbody_bt_p_ius_rv = 0.0;

        let (assign27950_e39631, assign27950_e39631_d_n0, assign27950_e39631_d_n2, assign27950_e39631_d_n6, assign27950_e39631_d_n7, assign27950_e39631_d_n10, assign27950_e39631_d_n11, assign27950_e39631_d_n12, assign27950_e39631_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard886 != 0.0)) && (locals.var_flg_ovloopd != 0.0)) {
        let assign27950_e39627: f64 = (-locals.var_uc_areabt);
        let assign27950_e39629: f64 = (assign27950_e39627 * locals.var_qsuld);
        (assign27950_e39629, (assign27950_e39627 * locals.var_qsuld_dn0), (assign27950_e39627 * locals.var_qsuld_dn2), (assign27950_e39627 * locals.var_qsuld_dn6), (assign27950_e39627 * locals.var_qsuld_dn7), (assign27950_e39627 * locals.var_qsuld_dn10), (assign27950_e39627 * locals.var_qsuld_dn11), (assign27950_e39627 * locals.var_qsuld_dn12), (assign27950_e39627 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_sud, locals.var_qbody_bt_p_sud_dn0, locals.var_qbody_bt_p_sud_dn2, locals.var_qbody_bt_p_sud_dn6, locals.var_qbody_bt_p_sud_dn7, locals.var_qbody_bt_p_sud_dn10, locals.var_qbody_bt_p_sud_dn11, locals.var_qbody_bt_p_sud_dn12, locals.var_qbody_bt_p_sud_dn17,)
    }
};
        locals.var_qbody_bt_p_sud = assign27950_e39631;
        locals.var_qbody_bt_p_sud_dn0 = assign27950_e39631_d_n0;
        locals.var_qbody_bt_p_sud_dn2 = assign27950_e39631_d_n2;
        locals.var_qbody_bt_p_sud_dn6 = assign27950_e39631_d_n6;
        locals.var_qbody_bt_p_sud_dn7 = assign27950_e39631_d_n7;
        locals.var_qbody_bt_p_sud_dn10 = assign27950_e39631_d_n10;
        locals.var_qbody_bt_p_sud_dn11 = assign27950_e39631_d_n11;
        locals.var_qbody_bt_p_sud_dn12 = assign27950_e39631_d_n12;
        locals.var_qbody_bt_p_sud_dn17 = assign27950_e39631_d_n17;
        locals.var_qbody_bt_p_sud_rv = 0.0;

        let (assign27960_e39646, assign27960_e39646_d_n0, assign27960_e39646_d_n2, assign27960_e39646_d_n6, assign27960_e39646_d_n7, assign27960_e39646_d_n10, assign27960_e39646_d_n11, assign27960_e39646_d_n12, assign27960_e39646_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && (locals.var_guard886 != 0.0)) && (locals.var_flg_ovloopd != 0.0)) {
        let assign27960_e39642: f64 = (-locals.var_uc_areabt);
        let assign27960_e39644: f64 = (assign27960_e39642 * locals.var_qiuld);
        (assign27960_e39644, (assign27960_e39642 * locals.var_qiuld_dn0), (assign27960_e39642 * locals.var_qiuld_dn2), (assign27960_e39642 * locals.var_qiuld_dn6), (assign27960_e39642 * locals.var_qiuld_dn7), (assign27960_e39642 * locals.var_qiuld_dn10), (assign27960_e39642 * locals.var_qiuld_dn11), (assign27960_e39642 * locals.var_qiuld_dn12), (assign27960_e39642 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_p_iud, locals.var_qbody_bt_p_iud_dn0, locals.var_qbody_bt_p_iud_dn2, locals.var_qbody_bt_p_iud_dn6, locals.var_qbody_bt_p_iud_dn7, locals.var_qbody_bt_p_iud_dn10, locals.var_qbody_bt_p_iud_dn11, locals.var_qbody_bt_p_iud_dn12, locals.var_qbody_bt_p_iud_dn17,)
    }
};
        locals.var_qbody_bt_p_iud = assign27960_e39646;
        locals.var_qbody_bt_p_iud_dn0 = assign27960_e39646_d_n0;
        locals.var_qbody_bt_p_iud_dn2 = assign27960_e39646_d_n2;
        locals.var_qbody_bt_p_iud_dn6 = assign27960_e39646_d_n6;
        locals.var_qbody_bt_p_iud_dn7 = assign27960_e39646_d_n7;
        locals.var_qbody_bt_p_iud_dn10 = assign27960_e39646_d_n10;
        locals.var_qbody_bt_p_iud_dn11 = assign27960_e39646_d_n11;
        locals.var_qbody_bt_p_iud_dn12 = assign27960_e39646_d_n12;
        locals.var_qbody_bt_p_iud_dn17 = assign27960_e39646_d_n17;
        locals.var_qbody_bt_p_iud_rv = 0.0;

        let (assign27970_e39664, assign27970_e39664_d_n0, assign27970_e39664_d_n2, assign27970_e39664_d_n6, assign27970_e39664_d_n7, assign27970_e39664_d_n10, assign27970_e39664_d_n11, assign27970_e39664_d_n12, assign27970_e39664_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && ((locals.var_guard887 != 0.0) && (locals.var_guard886 == 0.0))) && (locals.var_flg_ovloops != 0.0)) {
        let assign27970_e39660: f64 = (-locals.var_uc_areabt);
        let assign27970_e39662: f64 = (assign27970_e39660 * locals.var_qsuld);
        (assign27970_e39662, (assign27970_e39660 * locals.var_qsuld_dn0), (assign27970_e39660 * locals.var_qsuld_dn2), (assign27970_e39660 * locals.var_qsuld_dn6), (assign27970_e39660 * locals.var_qsuld_dn7), (assign27970_e39660 * locals.var_qsuld_dn10), (assign27970_e39660 * locals.var_qsuld_dn11), (assign27970_e39660 * locals.var_qsuld_dn12), (assign27970_e39660 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_sus, locals.var_qbody_bt_n_sus_dn0, locals.var_qbody_bt_n_sus_dn2, locals.var_qbody_bt_n_sus_dn6, locals.var_qbody_bt_n_sus_dn7, locals.var_qbody_bt_n_sus_dn10, locals.var_qbody_bt_n_sus_dn11, locals.var_qbody_bt_n_sus_dn12, locals.var_qbody_bt_n_sus_dn17,)
    }
};
        locals.var_qbody_bt_n_sus = assign27970_e39664;
        locals.var_qbody_bt_n_sus_dn0 = assign27970_e39664_d_n0;
        locals.var_qbody_bt_n_sus_dn2 = assign27970_e39664_d_n2;
        locals.var_qbody_bt_n_sus_dn6 = assign27970_e39664_d_n6;
        locals.var_qbody_bt_n_sus_dn7 = assign27970_e39664_d_n7;
        locals.var_qbody_bt_n_sus_dn10 = assign27970_e39664_d_n10;
        locals.var_qbody_bt_n_sus_dn11 = assign27970_e39664_d_n11;
        locals.var_qbody_bt_n_sus_dn12 = assign27970_e39664_d_n12;
        locals.var_qbody_bt_n_sus_dn17 = assign27970_e39664_d_n17;
        locals.var_qbody_bt_n_sus_rv = 0.0;

        let (assign27980_e39682, assign27980_e39682_d_n0, assign27980_e39682_d_n2, assign27980_e39682_d_n6, assign27980_e39682_d_n7, assign27980_e39682_d_n10, assign27980_e39682_d_n11, assign27980_e39682_d_n12, assign27980_e39682_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && ((locals.var_guard887 != 0.0) && (locals.var_guard886 == 0.0))) && (locals.var_flg_ovloops != 0.0)) {
        let assign27980_e39678: f64 = (-locals.var_uc_areabt);
        let assign27980_e39680: f64 = (assign27980_e39678 * locals.var_qiuld);
        (assign27980_e39680, (assign27980_e39678 * locals.var_qiuld_dn0), (assign27980_e39678 * locals.var_qiuld_dn2), (assign27980_e39678 * locals.var_qiuld_dn6), (assign27980_e39678 * locals.var_qiuld_dn7), (assign27980_e39678 * locals.var_qiuld_dn10), (assign27980_e39678 * locals.var_qiuld_dn11), (assign27980_e39678 * locals.var_qiuld_dn12), (assign27980_e39678 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_ius, locals.var_qbody_bt_n_ius_dn0, locals.var_qbody_bt_n_ius_dn2, locals.var_qbody_bt_n_ius_dn6, locals.var_qbody_bt_n_ius_dn7, locals.var_qbody_bt_n_ius_dn10, locals.var_qbody_bt_n_ius_dn11, locals.var_qbody_bt_n_ius_dn12, locals.var_qbody_bt_n_ius_dn17,)
    }
};
        locals.var_qbody_bt_n_ius = assign27980_e39682;
        locals.var_qbody_bt_n_ius_dn0 = assign27980_e39682_d_n0;
        locals.var_qbody_bt_n_ius_dn2 = assign27980_e39682_d_n2;
        locals.var_qbody_bt_n_ius_dn6 = assign27980_e39682_d_n6;
        locals.var_qbody_bt_n_ius_dn7 = assign27980_e39682_d_n7;
        locals.var_qbody_bt_n_ius_dn10 = assign27980_e39682_d_n10;
        locals.var_qbody_bt_n_ius_dn11 = assign27980_e39682_d_n11;
        locals.var_qbody_bt_n_ius_dn12 = assign27980_e39682_d_n12;
        locals.var_qbody_bt_n_ius_dn17 = assign27980_e39682_d_n17;
        locals.var_qbody_bt_n_ius_rv = 0.0;

        let (assign27990_e39700, assign27990_e39700_d_n0, assign27990_e39700_d_n2, assign27990_e39700_d_n6, assign27990_e39700_d_n7, assign27990_e39700_d_n10, assign27990_e39700_d_n11, assign27990_e39700_d_n12, assign27990_e39700_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && ((locals.var_guard887 != 0.0) && (locals.var_guard886 == 0.0))) && (locals.var_flg_ovloopd != 0.0)) {
        let assign27990_e39696: f64 = (-locals.var_uc_areabt);
        let assign27990_e39698: f64 = (assign27990_e39696 * locals.var_qsuld);
        (assign27990_e39698, (assign27990_e39696 * locals.var_qsuld_dn0), (assign27990_e39696 * locals.var_qsuld_dn2), (assign27990_e39696 * locals.var_qsuld_dn6), (assign27990_e39696 * locals.var_qsuld_dn7), (assign27990_e39696 * locals.var_qsuld_dn10), (assign27990_e39696 * locals.var_qsuld_dn11), (assign27990_e39696 * locals.var_qsuld_dn12), (assign27990_e39696 * locals.var_qsuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_sud, locals.var_qbody_bt_n_sud_dn0, locals.var_qbody_bt_n_sud_dn2, locals.var_qbody_bt_n_sud_dn6, locals.var_qbody_bt_n_sud_dn7, locals.var_qbody_bt_n_sud_dn10, locals.var_qbody_bt_n_sud_dn11, locals.var_qbody_bt_n_sud_dn12, locals.var_qbody_bt_n_sud_dn17,)
    }
};
        locals.var_qbody_bt_n_sud = assign27990_e39700;
        locals.var_qbody_bt_n_sud_dn0 = assign27990_e39700_d_n0;
        locals.var_qbody_bt_n_sud_dn2 = assign27990_e39700_d_n2;
        locals.var_qbody_bt_n_sud_dn6 = assign27990_e39700_d_n6;
        locals.var_qbody_bt_n_sud_dn7 = assign27990_e39700_d_n7;
        locals.var_qbody_bt_n_sud_dn10 = assign27990_e39700_d_n10;
        locals.var_qbody_bt_n_sud_dn11 = assign27990_e39700_d_n11;
        locals.var_qbody_bt_n_sud_dn12 = assign27990_e39700_d_n12;
        locals.var_qbody_bt_n_sud_dn17 = assign27990_e39700_d_n17;
        locals.var_qbody_bt_n_sud_rv = 0.0;

        let (assign28000_e39718, assign28000_e39718_d_n0, assign28000_e39718_d_n2, assign28000_e39718_d_n6, assign28000_e39718_d_n7, assign28000_e39718_d_n10, assign28000_e39718_d_n11, assign28000_e39718_d_n12, assign28000_e39718_d_n17,) = {
    if (((((locals.var_guard769 != 0.0) && (p.p24 != 0.0)) && (locals.var_guard851 != 0.0)) && ((locals.var_guard887 != 0.0) && (locals.var_guard886 == 0.0))) && (locals.var_flg_ovloopd != 0.0)) {
        let assign28000_e39714: f64 = (-locals.var_uc_areabt);
        let assign28000_e39716: f64 = (assign28000_e39714 * locals.var_qiuld);
        (assign28000_e39716, (assign28000_e39714 * locals.var_qiuld_dn0), (assign28000_e39714 * locals.var_qiuld_dn2), (assign28000_e39714 * locals.var_qiuld_dn6), (assign28000_e39714 * locals.var_qiuld_dn7), (assign28000_e39714 * locals.var_qiuld_dn10), (assign28000_e39714 * locals.var_qiuld_dn11), (assign28000_e39714 * locals.var_qiuld_dn12), (assign28000_e39714 * locals.var_qiuld_dn17),)
    } else {
        (locals.var_qbody_bt_n_iud, locals.var_qbody_bt_n_iud_dn0, locals.var_qbody_bt_n_iud_dn2, locals.var_qbody_bt_n_iud_dn6, locals.var_qbody_bt_n_iud_dn7, locals.var_qbody_bt_n_iud_dn10, locals.var_qbody_bt_n_iud_dn11, locals.var_qbody_bt_n_iud_dn12, locals.var_qbody_bt_n_iud_dn17,)
    }
};
        locals.var_qbody_bt_n_iud = assign28000_e39718;
        locals.var_qbody_bt_n_iud_dn0 = assign28000_e39718_d_n0;
        locals.var_qbody_bt_n_iud_dn2 = assign28000_e39718_d_n2;
        locals.var_qbody_bt_n_iud_dn6 = assign28000_e39718_d_n6;
        locals.var_qbody_bt_n_iud_dn7 = assign28000_e39718_d_n7;
        locals.var_qbody_bt_n_iud_dn10 = assign28000_e39718_d_n10;
        locals.var_qbody_bt_n_iud_dn11 = assign28000_e39718_d_n11;
        locals.var_qbody_bt_n_iud_dn12 = assign28000_e39718_d_n12;
        locals.var_qbody_bt_n_iud_dn17 = assign28000_e39718_d_n17;
        locals.var_qbody_bt_n_iud_rv = 0.0;

        locals.var_aclm = p.p189;
        locals.var_aclm_rv = 0.0;

        let assign28020_e39722: f64 = if locals.var_flg_noqi != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard890 = assign28020_e39722;
        locals.var_guard890_rv = 0.0;

        let (assign28030_e39728, assign28030_e39728_d_n0, assign28030_e39728_d_n2, assign28030_e39728_d_n6, assign28030_e39728_d_n7, assign28030_e39728_d_n10, assign28030_e39728_d_n11, assign28030_e39728_d_n12, assign28030_e39728_d_n17,) = {
    if (locals.var_guard890 != 0.0) {
        let assign28030_e39726: f64 = (locals.var_vds + locals.var_ps0);
        (assign28030_e39726, (locals.var_vds_dn0 + locals.var_ps0_dn0), (locals.var_vds_dn2 + locals.var_ps0_dn2), (locals.var_vds_dn6 + locals.var_ps0_dn6), (locals.var_vds_dn7 + locals.var_ps0_dn7), (locals.var_vds_dn10 + locals.var_ps0_dn10), (locals.var_vds_dn11 + locals.var_ps0_dn11), (locals.var_vds_dn12 + locals.var_ps0_dn12), (locals.var_vds_dn17 + locals.var_ps0_dn17),)
    } else {
        (locals.var_t2__blk889, locals.var_t2__blk889_dn0, locals.var_t2__blk889_dn2, locals.var_t2__blk889_dn6, locals.var_t2__blk889_dn7, locals.var_t2__blk889_dn10, locals.var_t2__blk889_dn11, locals.var_t2__blk889_dn12, locals.var_t2__blk889_dn17,)
    }
};
        locals.var_t2__blk889 = assign28030_e39728;
        locals.var_t2__blk889_dn0 = assign28030_e39728_d_n0;
        locals.var_t2__blk889_dn2 = assign28030_e39728_d_n2;
        locals.var_t2__blk889_dn6 = assign28030_e39728_d_n6;
        locals.var_t2__blk889_dn7 = assign28030_e39728_d_n7;
        locals.var_t2__blk889_dn10 = assign28030_e39728_d_n10;
        locals.var_t2__blk889_dn11 = assign28030_e39728_d_n11;
        locals.var_t2__blk889_dn12 = assign28030_e39728_d_n12;
        locals.var_t2__blk889_dn17 = assign28030_e39728_d_n17;
        locals.var_t2__blk889_rv = 0.0;

        let (assign28040_e39740, assign28040_e39740_d_n0, assign28040_e39740_d_n2, assign28040_e39740_d_n6, assign28040_e39740_d_n7, assign28040_e39740_d_n10, assign28040_e39740_d_n11, assign28040_e39740_d_n12, assign28040_e39740_d_n17,) = {
    if (locals.var_guard890 != 0.0) {
        let assign28040_e39732: f64 = (locals.var_aclm * locals.var_t2__blk889);
        let assign28040_e39735: f64 = (1.0 - locals.var_aclm);
        let assign28040_e39737: f64 = (assign28040_e39735 * locals.var_psl);
        let assign28040_e39738: f64 = (assign28040_e39732 + assign28040_e39737);
        (assign28040_e39738, ((locals.var_aclm * locals.var_t2__blk889_dn0) + (assign28040_e39735 * locals.var_psl_dn0)), ((locals.var_aclm * locals.var_t2__blk889_dn2) + (assign28040_e39735 * locals.var_psl_dn2)), ((locals.var_aclm * locals.var_t2__blk889_dn6) + (assign28040_e39735 * locals.var_psl_dn6)), ((locals.var_aclm * locals.var_t2__blk889_dn7) + (assign28040_e39735 * locals.var_psl_dn7)), ((locals.var_aclm * locals.var_t2__blk889_dn10) + (assign28040_e39735 * locals.var_psl_dn10)), ((locals.var_aclm * locals.var_t2__blk889_dn11) + (assign28040_e39735 * locals.var_psl_dn11)), ((locals.var_aclm * locals.var_t2__blk889_dn12) + (assign28040_e39735 * locals.var_psl_dn12)), ((locals.var_aclm * locals.var_t2__blk889_dn17) + (assign28040_e39735 * locals.var_psl_dn17)),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign28040_e39740;
        locals.var_psdl_dn0 = assign28040_e39740_d_n0;
        locals.var_psdl_dn2 = assign28040_e39740_d_n2;
        locals.var_psdl_dn6 = assign28040_e39740_d_n6;
        locals.var_psdl_dn7 = assign28040_e39740_d_n7;
        locals.var_psdl_dn10 = assign28040_e39740_d_n10;
        locals.var_psdl_dn11 = assign28040_e39740_d_n11;
        locals.var_psdl_dn12 = assign28040_e39740_d_n12;
        locals.var_psdl_dn17 = assign28040_e39740_d_n17;
        locals.var_psdl_rv = 0.0;

        let assign28050_e39743: f64 = if p.p64 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard891 = assign28050_e39743;
        locals.var_guard891_rv = 0.0;

        let (assign28060_e39749, assign28060_e39749_d_n0, assign28060_e39749_d_n2, assign28060_e39749_d_n6, assign28060_e39749_d_n7, assign28060_e39749_d_n10, assign28060_e39749_d_n11, assign28060_e39749_d_n12, assign28060_e39749_d_n17,) = {
    if ((locals.var_guard890 != 0.0) && (locals.var_guard891 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn12, locals.var_ec_dn17,)
    }
};
        locals.var_ec = assign28060_e39749;
        locals.var_ec_dn0 = assign28060_e39749_d_n0;
        locals.var_ec_dn2 = assign28060_e39749_d_n2;
        locals.var_ec_dn6 = assign28060_e39749_d_n6;
        locals.var_ec_dn7 = assign28060_e39749_d_n7;
        locals.var_ec_dn10 = assign28060_e39749_d_n10;
        locals.var_ec_dn11 = assign28060_e39749_d_n11;
        locals.var_ec_dn12 = assign28060_e39749_d_n12;
        locals.var_ec_dn17 = assign28060_e39749_d_n17;
        locals.var_ec_rv = 0.0;

        let assign28070_e39753: f64 = (locals.var_ps0 + locals.var_vds);
        let assign28070_e39756: f64 = (10.0 * 2.220446049250313e-16);
        let assign28070_e39757: f64 = (assign28070_e39753 - assign28070_e39756);
        let assign28070_e39758: f64 = if locals.var_psdl > assign28070_e39757 { 1.0 } else { 0.0 };
        locals.var_guard892 = assign28070_e39758;
        locals.var_guard892_rv = 0.0;

        let (assign28080_e39770, assign28080_e39770_d_n0, assign28080_e39770_d_n2, assign28080_e39770_d_n6, assign28080_e39770_d_n7, assign28080_e39770_d_n10, assign28080_e39770_d_n11, assign28080_e39770_d_n12, assign28080_e39770_d_n17,) = {
    if ((locals.var_guard890 != 0.0) && (locals.var_guard892 != 0.0)) {
        let assign28080_e39764: f64 = (locals.var_ps0 + locals.var_vds);
        let assign28080_e39767: f64 = (10.0 * 2.220446049250313e-16);
        let assign28080_e39768: f64 = (assign28080_e39764 - assign28080_e39767);
        (assign28080_e39768, (locals.var_ps0_dn0 + locals.var_vds_dn0), (locals.var_ps0_dn2 + locals.var_vds_dn2), (locals.var_ps0_dn6 + locals.var_vds_dn6), (locals.var_ps0_dn7 + locals.var_vds_dn7), (locals.var_ps0_dn10 + locals.var_vds_dn10), (locals.var_ps0_dn11 + locals.var_vds_dn11), (locals.var_ps0_dn12 + locals.var_vds_dn12), (locals.var_ps0_dn17 + locals.var_vds_dn17),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn12, locals.var_psdl_dn17,)
    }
};
        locals.var_psdl = assign28080_e39770;
        locals.var_psdl_dn0 = assign28080_e39770_d_n0;
        locals.var_psdl_dn2 = assign28080_e39770_d_n2;
        locals.var_psdl_dn6 = assign28080_e39770_d_n6;
        locals.var_psdl_dn7 = assign28080_e39770_d_n7;
        locals.var_psdl_dn10 = assign28080_e39770_d_n10;
        locals.var_psdl_dn11 = assign28080_e39770_d_n11;
        locals.var_psdl_dn12 = assign28080_e39770_d_n12;
        locals.var_psdl_dn17 = assign28080_e39770_d_n17;
        locals.var_psdl_rv = 0.0;

        let assign28090_e39773: f64 = if p.p64 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard893 = assign28090_e39773;
        locals.var_guard893_rv = 0.0;

        let assign28100_e39776: f64 = if locals.var_idd < 1e-15 { 1.0 } else { 0.0 };
        locals.var_guard894 = assign28100_e39776;
        locals.var_guard894_rv = 0.0;

        let (assign28110_e39785, assign28110_e39785_d_n0, assign28110_e39785_d_n2, assign28110_e39785_d_n6, assign28110_e39785_d_n7, assign28110_e39785_d_n10, assign28110_e39785_d_n11, assign28110_e39785_d_n12, assign28110_e39785_d_n17,) = {
    if (((locals.var_guard890 == 0.0) && (locals.var_guard893 != 0.0)) && (locals.var_guard894 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn12, locals.var_ec_dn17,)
    }
};
        locals.var_ec = assign28110_e39785;
        locals.var_ec_dn0 = assign28110_e39785_d_n0;
        locals.var_ec_dn2 = assign28110_e39785_d_n2;
        locals.var_ec_dn6 = assign28110_e39785_d_n6;
        locals.var_ec_dn7 = assign28110_e39785_d_n7;
        locals.var_ec_dn10 = assign28110_e39785_d_n10;
        locals.var_ec_dn11 = assign28110_e39785_d_n11;
        locals.var_ec_dn12 = assign28110_e39785_d_n12;
        locals.var_ec_dn17 = assign28110_e39785_d_n17;
        locals.var_ec_rv = 0.0;

        let (assign28120_e39797, assign28120_e39797_d_n10,) = {
    if (((locals.var_guard890 == 0.0) && (locals.var_guard893 != 0.0)) && (locals.var_guard894 == 0.0)) {
        let assign28120_e39795: f64 = (locals.var_beta_inv / locals.var_leff);
        (assign28120_e39795, (locals.var_beta_inv_dn10 / locals.var_leff),)
    } else {
        (locals.var_t1__blk888, locals.var_t1__blk888_dn10,)
    }
};
        locals.var_t1__blk888 = assign28120_e39797;
        locals.var_t1__blk888_dn10 = assign28120_e39797_d_n10;
        locals.var_t1__blk888_rv = 0.0;

        let (assign28130_e39809, assign28130_e39809_d_n0, assign28130_e39809_d_n2, assign28130_e39809_d_n6, assign28130_e39809_d_n7, assign28130_e39809_d_n10, assign28130_e39809_d_n11, assign28130_e39809_d_n12, assign28130_e39809_d_n17,) = {
    if (((locals.var_guard890 == 0.0) && (locals.var_guard893 != 0.0)) && (locals.var_guard894 == 0.0)) {
        let assign28130_e39807: f64 = (1.0 / locals.var_qn0);
        (assign28130_e39807, (-(locals.var_qn0_dn0 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn2 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn6 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn7 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn10 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn11 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn12 / (locals.var_qn0 * locals.var_qn0))), (-(locals.var_qn0_dn17 / (locals.var_qn0 * locals.var_qn0))),)
    } else {
        (locals.var_t2__blk889, locals.var_t2__blk889_dn0, locals.var_t2__blk889_dn2, locals.var_t2__blk889_dn6, locals.var_t2__blk889_dn7, locals.var_t2__blk889_dn10, locals.var_t2__blk889_dn11, locals.var_t2__blk889_dn12, locals.var_t2__blk889_dn17,)
    }
};
        locals.var_t2__blk889 = assign28130_e39809;
        locals.var_t2__blk889_dn0 = assign28130_e39809_d_n0;
        locals.var_t2__blk889_dn2 = assign28130_e39809_d_n2;
        locals.var_t2__blk889_dn6 = assign28130_e39809_d_n6;
        locals.var_t2__blk889_dn7 = assign28130_e39809_d_n7;
        locals.var_t2__blk889_dn10 = assign28130_e39809_d_n10;
        locals.var_t2__blk889_dn11 = assign28130_e39809_d_n11;
        locals.var_t2__blk889_dn12 = assign28130_e39809_d_n12;
        locals.var_t2__blk889_dn17 = assign28130_e39809_d_n17;
        locals.var_t2__blk889_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_102(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28140_e39823, assign28140_e39823_d_n0, assign28140_e39823_d_n2, assign28140_e39823_d_n6, assign28140_e39823_d_n7, assign28140_e39823_d_n10, assign28140_e39823_d_n11, assign28140_e39823_d_n12, assign28140_e39823_d_n17,) = {
    if (((locals.var_guard890 == 0.0) && (locals.var_guard893 != 0.0)) && (locals.var_guard894 == 0.0)) {
        let assign28140_e39819: f64 = (locals.var_idd * locals.var_t1__blk888);
        let assign28140_e39821: f64 = (assign28140_e39819 * locals.var_t2__blk889);
        (assign28140_e39821, (((locals.var_idd_dn0 * locals.var_t1__blk888) * locals.var_t2__blk889) + (assign28140_e39819 * locals.var_t2__blk889_dn0)), (((locals.var_idd_dn2 * locals.var_t1__blk888) * locals.var_t2__blk889) + (assign28140_e39819 * locals.var_t2__blk889_dn2)), (((locals.var_idd_dn6 * locals.var_t1__blk888) * locals.var_t2__blk889) + (assign28140_e39819 * locals.var_t2__blk889_dn6)), (((locals.var_idd_dn7 * locals.var_t1__blk888) * locals.var_t2__blk889) + (assign28140_e39819 * locals.var_t2__blk889_dn7)), ((((locals.var_idd_dn10 * locals.var_t1__blk888) + (locals.var_idd * locals.var_t1__blk888_dn10)) * locals.var_t2__blk889) + (assign28140_e39819 * locals.var_t2__blk889_dn10)), (((locals.var_idd_dn11 * locals.var_t1__blk888) * locals.var_t2__blk889) + (assign28140_e39819 * locals.var_t2__blk889_dn11)), (((locals.var_idd_dn12 * locals.var_t1__blk888) * locals.var_t2__blk889) + (assign28140_e39819 * locals.var_t2__blk889_dn12)), (((locals.var_idd_dn17 * locals.var_t1__blk888) * locals.var_t2__blk889) + (assign28140_e39819 * locals.var_t2__blk889_dn17)),)
    } else {
        (locals.var_ec, locals.var_ec_dn0, locals.var_ec_dn2, locals.var_ec_dn6, locals.var_ec_dn7, locals.var_ec_dn10, locals.var_ec_dn11, locals.var_ec_dn12, locals.var_ec_dn17,)
    }
};
        locals.var_ec = assign28140_e39823;
        locals.var_ec_dn0 = assign28140_e39823_d_n0;
        locals.var_ec_dn2 = assign28140_e39823_d_n2;
        locals.var_ec_dn6 = assign28140_e39823_d_n6;
        locals.var_ec_dn7 = assign28140_e39823_d_n7;
        locals.var_ec_dn10 = assign28140_e39823_d_n10;
        locals.var_ec_dn11 = assign28140_e39823_d_n11;
        locals.var_ec_dn12 = assign28140_e39823_d_n12;
        locals.var_ec_dn17 = assign28140_e39823_d_n17;
        locals.var_ec_rv = 0.0;

        locals.var_cox0__blk906 = locals.var_c_fox0;
        locals.var_cox0__blk906_rv = 0.0;

        let assign28160_e39827: f64 = (1.0 / locals.var_cox0__blk906);
        locals.var_cox0_inv__blk907 = assign28160_e39827;
        locals.var_cox0_inv__blk907_rv = 0.0;

        locals.var_vgbgmt__blk927 = 0.0;
        locals.var_vgbgmt__blk927_dn0 = 0.0;
        locals.var_vgbgmt__blk927_dn2 = 0.0;
        locals.var_vgbgmt__blk927_dn6 = 0.0;
        locals.var_vgbgmt__blk927_dn7 = 0.0;
        locals.var_vgbgmt__blk927_dn10 = 0.0;
        locals.var_vgbgmt__blk927_dn11 = 0.0;
        locals.var_vgbgmt__blk927_dn12 = 0.0;
        locals.var_vgbgmt__blk927_dn17 = 0.0;
        locals.var_vgbgmt__blk927_rv = 0.0;

        locals.var_fb__blk967 = 0.0;
        locals.var_fb__blk967_dn0 = 0.0;
        locals.var_fb__blk967_dn2 = 0.0;
        locals.var_fb__blk967_dn6 = 0.0;
        locals.var_fb__blk967_dn7 = 0.0;
        locals.var_fb__blk967_dn10 = 0.0;
        locals.var_fb__blk967_dn11 = 0.0;
        locals.var_fb__blk967_dn12 = 0.0;
        locals.var_fb__blk967_dn17 = 0.0;
        locals.var_fb__blk967_rv = 0.0;

        locals.var_fs01__blk965 = 0.0;
        locals.var_fs01__blk965_dn0 = 0.0;
        locals.var_fs01__blk965_dn2 = 0.0;
        locals.var_fs01__blk965_dn6 = 0.0;
        locals.var_fs01__blk965_dn7 = 0.0;
        locals.var_fs01__blk965_dn10 = 0.0;
        locals.var_fs01__blk965_dn11 = 0.0;
        locals.var_fs01__blk965_dn12 = 0.0;
        locals.var_fs01__blk965_dn17 = 0.0;
        locals.var_fs01__blk965_rv = 0.0;

        locals.var_fs02__blk969 = 0.0;
        locals.var_fs02__blk969_dn0 = 0.0;
        locals.var_fs02__blk969_dn2 = 0.0;
        locals.var_fs02__blk969_dn6 = 0.0;
        locals.var_fs02__blk969_dn7 = 0.0;
        locals.var_fs02__blk969_dn10 = 0.0;
        locals.var_fs02__blk969_dn11 = 0.0;
        locals.var_fs02__blk969_dn12 = 0.0;
        locals.var_fs02__blk969_dn17 = 0.0;
        locals.var_fs02__blk969_rv = 0.0;

        let assign28210_e39838: f64 = if ((p.p29 >= 1.0) && (p.p188 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard978 = assign28210_e39838;
        locals.var_guard978_rv = 0.0;

        let (assign28220_e39844,) = {
    if ((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) {
        (p.p171,)
    } else {
        (locals.var_cov_slp,)
    }
};
        locals.var_cov_slp = assign28220_e39844;
        locals.var_cov_slp_rv = 0.0;

        let (assign28230_e39850,) = {
    if ((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) {
        (p.p172,)
    } else {
        (locals.var_cov_mag,)
    }
};
        locals.var_cov_mag = assign28230_e39850;
        locals.var_cov_mag_rv = 0.0;

        let (assign28240_e39856, assign28240_e39856_d_n6, assign28240_e39856_d_n7, assign28240_e39856_d_n11,) = {
    if ((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) {
        (locals.var_vgs, locals.var_vgs_dn6, locals.var_vgs_dn7, locals.var_vgs_dn11,)
    } else {
        (locals.var_covvg, locals.var_covvg_dn6, locals.var_covvg_dn7, locals.var_covvg_dn11,)
    }
};
        locals.var_covvg = assign28240_e39856;
        locals.var_covvg_dn6 = assign28240_e39856_d_n6;
        locals.var_covvg_dn7 = assign28240_e39856_d_n7;
        locals.var_covvg_dn11 = assign28240_e39856_d_n11;
        locals.var_covvg_rv = 0.0;

        let (assign28250_e39862,) = {
    if ((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) {
        (p.p188,)
    } else {
        (locals.var_lov,)
    }
};
        locals.var_lov = assign28250_e39862;
        locals.var_lov_rv = 0.0;

        let assign28260_e39869: f64 = if ((locals.var_mks_nover == 0.0) && (p.p188 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard979 = assign28260_e39869;
        locals.var_guard979_rv = 0.0;

        let (assign28270_e39886, assign28270_e39886_d_n0, assign28270_e39886_d_n2, assign28270_e39886_d_n6, assign28270_e39886_d_n7, assign28270_e39886_d_n10, assign28270_e39886_d_n11, assign28270_e39886_d_n12, assign28270_e39886_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 != 0.0)) {
        let (assign28270_e39884,) = {
            if (p.p43 == 1.0) {
                let assign28270_e39880: f64 = (locals.var_w_dioscv * locals.var_cox0__blk906);
                (assign28270_e39880,)
            } else {
                let assign28270_e39883: f64 = (locals.var_weffcv_nf * locals.var_cox0__blk906);
                (assign28270_e39883,)
            }
        };
        (assign28270_e39884, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign28270_e39886;
        locals.var_t1__blk896_dn0 = assign28270_e39886_d_n0;
        locals.var_t1__blk896_dn2 = assign28270_e39886_d_n2;
        locals.var_t1__blk896_dn6 = assign28270_e39886_d_n6;
        locals.var_t1__blk896_dn7 = assign28270_e39886_d_n7;
        locals.var_t1__blk896_dn10 = assign28270_e39886_d_n10;
        locals.var_t1__blk896_dn11 = assign28270_e39886_d_n11;
        locals.var_t1__blk896_dn12 = assign28270_e39886_d_n12;
        locals.var_t1__blk896_dn17 = assign28270_e39886_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let (assign28280_e39900, assign28280_e39900_d_n0, assign28280_e39900_d_n2, assign28280_e39900_d_n6, assign28280_e39900_d_n7, assign28280_e39900_d_n10, assign28280_e39900_d_n11, assign28280_e39900_d_n12, assign28280_e39900_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 != 0.0)) {
        let assign28280_e39894: f64 = (locals.var_cov_slp * locals.var_t1__blk896);
        let assign28280_e39897: f64 = (locals.var_cov_mag + locals.var_covvg);
        let assign28280_e39898: f64 = (assign28280_e39894 * assign28280_e39897);
        (assign28280_e39898, ((locals.var_cov_slp * locals.var_t1__blk896_dn0) * assign28280_e39897), ((locals.var_cov_slp * locals.var_t1__blk896_dn2) * assign28280_e39897), (((locals.var_cov_slp * locals.var_t1__blk896_dn6) * assign28280_e39897) + (assign28280_e39894 * locals.var_covvg_dn6)), (((locals.var_cov_slp * locals.var_t1__blk896_dn7) * assign28280_e39897) + (assign28280_e39894 * locals.var_covvg_dn7)), ((locals.var_cov_slp * locals.var_t1__blk896_dn10) * assign28280_e39897), (((locals.var_cov_slp * locals.var_t1__blk896_dn11) * assign28280_e39897) + (assign28280_e39894 * locals.var_covvg_dn11)), ((locals.var_cov_slp * locals.var_t1__blk896_dn12) * assign28280_e39897), ((locals.var_cov_slp * locals.var_t1__blk896_dn17) * assign28280_e39897),)
    } else {
        (locals.var_t4__blk899, locals.var_t4__blk899_dn0, locals.var_t4__blk899_dn2, locals.var_t4__blk899_dn6, locals.var_t4__blk899_dn7, locals.var_t4__blk899_dn10, locals.var_t4__blk899_dn11, locals.var_t4__blk899_dn12, locals.var_t4__blk899_dn17,)
    }
};
        locals.var_t4__blk899 = assign28280_e39900;
        locals.var_t4__blk899_dn0 = assign28280_e39900_d_n0;
        locals.var_t4__blk899_dn2 = assign28280_e39900_d_n2;
        locals.var_t4__blk899_dn6 = assign28280_e39900_d_n6;
        locals.var_t4__blk899_dn7 = assign28280_e39900_d_n7;
        locals.var_t4__blk899_dn10 = assign28280_e39900_d_n10;
        locals.var_t4__blk899_dn11 = assign28280_e39900_d_n11;
        locals.var_t4__blk899_dn12 = assign28280_e39900_d_n12;
        locals.var_t4__blk899_dn17 = assign28280_e39900_d_n17;
        locals.var_t4__blk899_rv = 0.0;

        let (assign28290_e39910, assign28290_e39910_d_n0, assign28290_e39910_d_n2, assign28290_e39910_d_n6, assign28290_e39910_d_n7, assign28290_e39910_d_n10, assign28290_e39910_d_n11, assign28290_e39910_d_n12, assign28290_e39910_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 != 0.0)) {
        let assign28290_e39908: f64 = (locals.var_lov * locals.var_t1__blk896);
        (assign28290_e39908, (locals.var_lov * locals.var_t1__blk896_dn0), (locals.var_lov * locals.var_t1__blk896_dn2), (locals.var_lov * locals.var_t1__blk896_dn6), (locals.var_lov * locals.var_t1__blk896_dn7), (locals.var_lov * locals.var_t1__blk896_dn10), (locals.var_lov * locals.var_t1__blk896_dn11), (locals.var_lov * locals.var_t1__blk896_dn12), (locals.var_lov * locals.var_t1__blk896_dn17),)
    } else {
        (locals.var_t5__blk900, locals.var_t5__blk900_dn0, locals.var_t5__blk900_dn2, locals.var_t5__blk900_dn6, locals.var_t5__blk900_dn7, locals.var_t5__blk900_dn10, locals.var_t5__blk900_dn11, locals.var_t5__blk900_dn12, locals.var_t5__blk900_dn17,)
    }
};
        locals.var_t5__blk900 = assign28290_e39910;
        locals.var_t5__blk900_dn0 = assign28290_e39910_d_n0;
        locals.var_t5__blk900_dn2 = assign28290_e39910_d_n2;
        locals.var_t5__blk900_dn6 = assign28290_e39910_d_n6;
        locals.var_t5__blk900_dn7 = assign28290_e39910_d_n7;
        locals.var_t5__blk900_dn10 = assign28290_e39910_d_n10;
        locals.var_t5__blk900_dn11 = assign28290_e39910_d_n11;
        locals.var_t5__blk900_dn12 = assign28290_e39910_d_n12;
        locals.var_t5__blk900_dn17 = assign28290_e39910_d_n17;
        locals.var_t5__blk900_rv = 0.0;

        let (assign28300_e39918, assign28300_e39918_d_n0, assign28300_e39918_d_n2, assign28300_e39918_d_n6, assign28300_e39918_d_n7, assign28300_e39918_d_n10, assign28300_e39918_d_n11, assign28300_e39918_d_n12, assign28300_e39918_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn12, locals.var_ps0_dn17,)
    } else {
        (locals.var_tx__blk904, locals.var_tx__blk904_dn0, locals.var_tx__blk904_dn2, locals.var_tx__blk904_dn6, locals.var_tx__blk904_dn7, locals.var_tx__blk904_dn10, locals.var_tx__blk904_dn11, locals.var_tx__blk904_dn12, locals.var_tx__blk904_dn17,)
    }
};
        locals.var_tx__blk904 = assign28300_e39918;
        locals.var_tx__blk904_dn0 = assign28300_e39918_d_n0;
        locals.var_tx__blk904_dn2 = assign28300_e39918_d_n2;
        locals.var_tx__blk904_dn6 = assign28300_e39918_d_n6;
        locals.var_tx__blk904_dn7 = assign28300_e39918_d_n7;
        locals.var_tx__blk904_dn10 = assign28300_e39918_d_n10;
        locals.var_tx__blk904_dn11 = assign28300_e39918_d_n11;
        locals.var_tx__blk904_dn12 = assign28300_e39918_d_n12;
        locals.var_tx__blk904_dn17 = assign28300_e39918_d_n17;
        locals.var_tx__blk904_rv = 0.0;

        let (assign28310_e39928, assign28310_e39928_d_n0, assign28310_e39928_d_n2, assign28310_e39928_d_n6, assign28310_e39928_d_n7, assign28310_e39928_d_n10, assign28310_e39928_d_n11, assign28310_e39928_d_n12, assign28310_e39928_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 != 0.0)) {
        let assign28310_e39926: f64 = (1.2 - locals.var_tx__blk904);
        (assign28310_e39926, (-locals.var_tx__blk904_dn0), (-locals.var_tx__blk904_dn2), (-locals.var_tx__blk904_dn6), (-locals.var_tx__blk904_dn7), (-locals.var_tx__blk904_dn10), (-locals.var_tx__blk904_dn11), (-locals.var_tx__blk904_dn12), (-locals.var_tx__blk904_dn17),)
    } else {
        (locals.var_t9__blk901, locals.var_t9__blk901_dn0, locals.var_t9__blk901_dn2, locals.var_t9__blk901_dn6, locals.var_t9__blk901_dn7, locals.var_t9__blk901_dn10, locals.var_t9__blk901_dn11, locals.var_t9__blk901_dn12, locals.var_t9__blk901_dn17,)
    }
};
        locals.var_t9__blk901 = assign28310_e39928;
        locals.var_t9__blk901_dn0 = assign28310_e39928_d_n0;
        locals.var_t9__blk901_dn2 = assign28310_e39928_d_n2;
        locals.var_t9__blk901_dn6 = assign28310_e39928_d_n6;
        locals.var_t9__blk901_dn7 = assign28310_e39928_d_n7;
        locals.var_t9__blk901_dn10 = assign28310_e39928_d_n10;
        locals.var_t9__blk901_dn11 = assign28310_e39928_d_n11;
        locals.var_t9__blk901_dn12 = assign28310_e39928_d_n12;
        locals.var_t9__blk901_dn17 = assign28310_e39928_d_n17;
        locals.var_t9__blk901_rv = 0.0;

        let (assign28320_e39942, assign28320_e39942_d_n0, assign28320_e39942_d_n2, assign28320_e39942_d_n6, assign28320_e39942_d_n7, assign28320_e39942_d_n10, assign28320_e39942_d_n11, assign28320_e39942_d_n12, assign28320_e39942_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 != 0.0)) {
        let assign28320_e39936: f64 = (locals.var_vgs * locals.var_t5__blk900);
        let assign28320_e39939: f64 = (locals.var_t9__blk901 * locals.var_t4__blk899);
        let assign28320_e39940: f64 = (assign28320_e39936 - assign28320_e39939);
        (assign28320_e39940, ((locals.var_vgs * locals.var_t5__blk900_dn0) - ((locals.var_t9__blk901_dn0 * locals.var_t4__blk899) + (locals.var_t9__blk901 * locals.var_t4__blk899_dn0))), ((locals.var_vgs * locals.var_t5__blk900_dn2) - ((locals.var_t9__blk901_dn2 * locals.var_t4__blk899) + (locals.var_t9__blk901 * locals.var_t4__blk899_dn2))), (((locals.var_vgs_dn6 * locals.var_t5__blk900) + (locals.var_vgs * locals.var_t5__blk900_dn6)) - ((locals.var_t9__blk901_dn6 * locals.var_t4__blk899) + (locals.var_t9__blk901 * locals.var_t4__blk899_dn6))), (((locals.var_vgs_dn7 * locals.var_t5__blk900) + (locals.var_vgs * locals.var_t5__blk900_dn7)) - ((locals.var_t9__blk901_dn7 * locals.var_t4__blk899) + (locals.var_t9__blk901 * locals.var_t4__blk899_dn7))), ((locals.var_vgs * locals.var_t5__blk900_dn10) - ((locals.var_t9__blk901_dn10 * locals.var_t4__blk899) + (locals.var_t9__blk901 * locals.var_t4__blk899_dn10))), (((locals.var_vgs_dn11 * locals.var_t5__blk900) + (locals.var_vgs * locals.var_t5__blk900_dn11)) - ((locals.var_t9__blk901_dn11 * locals.var_t4__blk899) + (locals.var_t9__blk901 * locals.var_t4__blk899_dn11))), ((locals.var_vgs * locals.var_t5__blk900_dn12) - ((locals.var_t9__blk901_dn12 * locals.var_t4__blk899) + (locals.var_t9__blk901 * locals.var_t4__blk899_dn12))), ((locals.var_vgs * locals.var_t5__blk900_dn17) - ((locals.var_t9__blk901_dn17 * locals.var_t4__blk899) + (locals.var_t9__blk901 * locals.var_t4__blk899_dn17))),)
    } else {
        (locals.var_qgos, locals.var_qgos_dn0, locals.var_qgos_dn2, locals.var_qgos_dn6, locals.var_qgos_dn7, locals.var_qgos_dn10, locals.var_qgos_dn11, locals.var_qgos_dn12, locals.var_qgos_dn17,)
    }
};
        locals.var_qgos = assign28320_e39942;
        locals.var_qgos_dn0 = assign28320_e39942_d_n0;
        locals.var_qgos_dn2 = assign28320_e39942_d_n2;
        locals.var_qgos_dn6 = assign28320_e39942_d_n6;
        locals.var_qgos_dn7 = assign28320_e39942_d_n7;
        locals.var_qgos_dn10 = assign28320_e39942_d_n10;
        locals.var_qgos_dn11 = assign28320_e39942_d_n11;
        locals.var_qgos_dn12 = assign28320_e39942_d_n12;
        locals.var_qgos_dn17 = assign28320_e39942_d_n17;
        locals.var_qgos_rv = 0.0;

        let (assign28330_e39958, assign28330_e39958_d_n0, assign28330_e39958_d_n2, assign28330_e39958_d_n6, assign28330_e39958_d_n7, assign28330_e39958_d_n10, assign28330_e39958_d_n11, assign28330_e39958_d_n12, assign28330_e39958_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 != 0.0)) {
        let assign28330_e39950: f64 = (locals.var_cov_slp * locals.var_t1__blk896);
        let assign28330_e39953: f64 = (locals.var_cov_mag + locals.var_covvg);
        let assign28330_e39955: f64 = (assign28330_e39953 - locals.var_vds);
        let assign28330_e39956: f64 = (assign28330_e39950 * assign28330_e39955);
        (assign28330_e39956, (((locals.var_cov_slp * locals.var_t1__blk896_dn0) * assign28330_e39955) + (assign28330_e39950 * (-locals.var_vds_dn0))), (((locals.var_cov_slp * locals.var_t1__blk896_dn2) * assign28330_e39955) + (assign28330_e39950 * (-locals.var_vds_dn2))), (((locals.var_cov_slp * locals.var_t1__blk896_dn6) * assign28330_e39955) + (assign28330_e39950 * (locals.var_covvg_dn6 - locals.var_vds_dn6))), (((locals.var_cov_slp * locals.var_t1__blk896_dn7) * assign28330_e39955) + (assign28330_e39950 * (locals.var_covvg_dn7 - locals.var_vds_dn7))), (((locals.var_cov_slp * locals.var_t1__blk896_dn10) * assign28330_e39955) + (assign28330_e39950 * (-locals.var_vds_dn10))), (((locals.var_cov_slp * locals.var_t1__blk896_dn11) * assign28330_e39955) + (assign28330_e39950 * (locals.var_covvg_dn11 - locals.var_vds_dn11))), (((locals.var_cov_slp * locals.var_t1__blk896_dn12) * assign28330_e39955) + (assign28330_e39950 * (-locals.var_vds_dn12))), (((locals.var_cov_slp * locals.var_t1__blk896_dn17) * assign28330_e39955) + (assign28330_e39950 * (-locals.var_vds_dn17))),)
    } else {
        (locals.var_t4__blk899, locals.var_t4__blk899_dn0, locals.var_t4__blk899_dn2, locals.var_t4__blk899_dn6, locals.var_t4__blk899_dn7, locals.var_t4__blk899_dn10, locals.var_t4__blk899_dn11, locals.var_t4__blk899_dn12, locals.var_t4__blk899_dn17,)
    }
};
        locals.var_t4__blk899 = assign28330_e39958;
        locals.var_t4__blk899_dn0 = assign28330_e39958_d_n0;
        locals.var_t4__blk899_dn2 = assign28330_e39958_d_n2;
        locals.var_t4__blk899_dn6 = assign28330_e39958_d_n6;
        locals.var_t4__blk899_dn7 = assign28330_e39958_d_n7;
        locals.var_t4__blk899_dn10 = assign28330_e39958_d_n10;
        locals.var_t4__blk899_dn11 = assign28330_e39958_d_n11;
        locals.var_t4__blk899_dn12 = assign28330_e39958_d_n12;
        locals.var_t4__blk899_dn17 = assign28330_e39958_d_n17;
        locals.var_t4__blk899_rv = 0.0;

        let (assign28340_e39968, assign28340_e39968_d_n0, assign28340_e39968_d_n2, assign28340_e39968_d_n6, assign28340_e39968_d_n7, assign28340_e39968_d_n10, assign28340_e39968_d_n11, assign28340_e39968_d_n12, assign28340_e39968_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 != 0.0)) {
        let assign28340_e39966: f64 = (locals.var_psl - locals.var_vds);
        (assign28340_e39966, (locals.var_psl_dn0 - locals.var_vds_dn0), (locals.var_psl_dn2 - locals.var_vds_dn2), (locals.var_psl_dn6 - locals.var_vds_dn6), (locals.var_psl_dn7 - locals.var_vds_dn7), (locals.var_psl_dn10 - locals.var_vds_dn10), (locals.var_psl_dn11 - locals.var_vds_dn11), (locals.var_psl_dn12 - locals.var_vds_dn12), (locals.var_psl_dn17 - locals.var_vds_dn17),)
    } else {
        (locals.var_tx__blk904, locals.var_tx__blk904_dn0, locals.var_tx__blk904_dn2, locals.var_tx__blk904_dn6, locals.var_tx__blk904_dn7, locals.var_tx__blk904_dn10, locals.var_tx__blk904_dn11, locals.var_tx__blk904_dn12, locals.var_tx__blk904_dn17,)
    }
};
        locals.var_tx__blk904 = assign28340_e39968;
        locals.var_tx__blk904_dn0 = assign28340_e39968_d_n0;
        locals.var_tx__blk904_dn2 = assign28340_e39968_d_n2;
        locals.var_tx__blk904_dn6 = assign28340_e39968_d_n6;
        locals.var_tx__blk904_dn7 = assign28340_e39968_d_n7;
        locals.var_tx__blk904_dn10 = assign28340_e39968_d_n10;
        locals.var_tx__blk904_dn11 = assign28340_e39968_d_n11;
        locals.var_tx__blk904_dn12 = assign28340_e39968_d_n12;
        locals.var_tx__blk904_dn17 = assign28340_e39968_d_n17;
        locals.var_tx__blk904_rv = 0.0;

        let (assign28350_e39978, assign28350_e39978_d_n0, assign28350_e39978_d_n2, assign28350_e39978_d_n6, assign28350_e39978_d_n7, assign28350_e39978_d_n10, assign28350_e39978_d_n11, assign28350_e39978_d_n12, assign28350_e39978_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 != 0.0)) {
        let assign28350_e39976: f64 = (1.2 - locals.var_tx__blk904);
        (assign28350_e39976, (-locals.var_tx__blk904_dn0), (-locals.var_tx__blk904_dn2), (-locals.var_tx__blk904_dn6), (-locals.var_tx__blk904_dn7), (-locals.var_tx__blk904_dn10), (-locals.var_tx__blk904_dn11), (-locals.var_tx__blk904_dn12), (-locals.var_tx__blk904_dn17),)
    } else {
        (locals.var_t9__blk901, locals.var_t9__blk901_dn0, locals.var_t9__blk901_dn2, locals.var_t9__blk901_dn6, locals.var_t9__blk901_dn7, locals.var_t9__blk901_dn10, locals.var_t9__blk901_dn11, locals.var_t9__blk901_dn12, locals.var_t9__blk901_dn17,)
    }
};
        locals.var_t9__blk901 = assign28350_e39978;
        locals.var_t9__blk901_dn0 = assign28350_e39978_d_n0;
        locals.var_t9__blk901_dn2 = assign28350_e39978_d_n2;
        locals.var_t9__blk901_dn6 = assign28350_e39978_d_n6;
        locals.var_t9__blk901_dn7 = assign28350_e39978_d_n7;
        locals.var_t9__blk901_dn10 = assign28350_e39978_d_n10;
        locals.var_t9__blk901_dn11 = assign28350_e39978_d_n11;
        locals.var_t9__blk901_dn12 = assign28350_e39978_d_n12;
        locals.var_t9__blk901_dn17 = assign28350_e39978_d_n17;
        locals.var_t9__blk901_rv = 0.0;

        let (assign28360_e39994, assign28360_e39994_d_n0, assign28360_e39994_d_n2, assign28360_e39994_d_n6, assign28360_e39994_d_n7, assign28360_e39994_d_n10, assign28360_e39994_d_n11, assign28360_e39994_d_n12, assign28360_e39994_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 != 0.0)) {
        let assign28360_e39986: f64 = (locals.var_vgs - locals.var_vds);
        let assign28360_e39988: f64 = (assign28360_e39986 * locals.var_t5__blk900);
        let assign28360_e39991: f64 = (locals.var_t4__blk899 * locals.var_t9__blk901);
        let assign28360_e39992: f64 = (assign28360_e39988 - assign28360_e39991);
        (assign28360_e39992, ((((-locals.var_vds_dn0) * locals.var_t5__blk900) + (assign28360_e39986 * locals.var_t5__blk900_dn0)) - ((locals.var_t4__blk899_dn0 * locals.var_t9__blk901) + (locals.var_t4__blk899 * locals.var_t9__blk901_dn0))), ((((-locals.var_vds_dn2) * locals.var_t5__blk900) + (assign28360_e39986 * locals.var_t5__blk900_dn2)) - ((locals.var_t4__blk899_dn2 * locals.var_t9__blk901) + (locals.var_t4__blk899 * locals.var_t9__blk901_dn2))), ((((locals.var_vgs_dn6 - locals.var_vds_dn6) * locals.var_t5__blk900) + (assign28360_e39986 * locals.var_t5__blk900_dn6)) - ((locals.var_t4__blk899_dn6 * locals.var_t9__blk901) + (locals.var_t4__blk899 * locals.var_t9__blk901_dn6))), ((((locals.var_vgs_dn7 - locals.var_vds_dn7) * locals.var_t5__blk900) + (assign28360_e39986 * locals.var_t5__blk900_dn7)) - ((locals.var_t4__blk899_dn7 * locals.var_t9__blk901) + (locals.var_t4__blk899 * locals.var_t9__blk901_dn7))), ((((-locals.var_vds_dn10) * locals.var_t5__blk900) + (assign28360_e39986 * locals.var_t5__blk900_dn10)) - ((locals.var_t4__blk899_dn10 * locals.var_t9__blk901) + (locals.var_t4__blk899 * locals.var_t9__blk901_dn10))), ((((locals.var_vgs_dn11 - locals.var_vds_dn11) * locals.var_t5__blk900) + (assign28360_e39986 * locals.var_t5__blk900_dn11)) - ((locals.var_t4__blk899_dn11 * locals.var_t9__blk901) + (locals.var_t4__blk899 * locals.var_t9__blk901_dn11))), ((((-locals.var_vds_dn12) * locals.var_t5__blk900) + (assign28360_e39986 * locals.var_t5__blk900_dn12)) - ((locals.var_t4__blk899_dn12 * locals.var_t9__blk901) + (locals.var_t4__blk899 * locals.var_t9__blk901_dn12))), ((((-locals.var_vds_dn17) * locals.var_t5__blk900) + (assign28360_e39986 * locals.var_t5__blk900_dn17)) - ((locals.var_t4__blk899_dn17 * locals.var_t9__blk901) + (locals.var_t4__blk899 * locals.var_t9__blk901_dn17))),)
    } else {
        (locals.var_qgod, locals.var_qgod_dn0, locals.var_qgod_dn2, locals.var_qgod_dn6, locals.var_qgod_dn7, locals.var_qgod_dn10, locals.var_qgod_dn11, locals.var_qgod_dn12, locals.var_qgod_dn17,)
    }
};
        locals.var_qgod = assign28360_e39994;
        locals.var_qgod_dn0 = assign28360_e39994_d_n0;
        locals.var_qgod_dn2 = assign28360_e39994_d_n2;
        locals.var_qgod_dn6 = assign28360_e39994_d_n6;
        locals.var_qgod_dn7 = assign28360_e39994_d_n7;
        locals.var_qgod_dn10 = assign28360_e39994_d_n10;
        locals.var_qgod_dn11 = assign28360_e39994_d_n11;
        locals.var_qgod_dn12 = assign28360_e39994_d_n12;
        locals.var_qgod_dn17 = assign28360_e39994_d_n17;
        locals.var_qgod_rv = 0.0;

        let (assign28370_e40008, assign28370_e40008_d_n0, assign28370_e40008_d_n2, assign28370_e40008_d_n6, assign28370_e40008_d_n7, assign28370_e40008_d_n10, assign28370_e40008_d_n11, assign28370_e40008_d_n12, assign28370_e40008_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign28370_e40004: f64 = (locals.var_mks_nover / locals.var_nsub);
        let assign28370_e40005: f64 = (assign28370_e40004).sqrt();
        let assign28370_e40006: f64 = (locals.var_cnst0soi * assign28370_e40005);
        (assign28370_e40006, ((locals.var_cnst0soi_dn0 * assign28370_e40005) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28370_e40005)))), ((locals.var_cnst0soi_dn2 * assign28370_e40005) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28370_e40005)))), ((locals.var_cnst0soi_dn6 * assign28370_e40005) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28370_e40005)))), ((locals.var_cnst0soi_dn7 * assign28370_e40005) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28370_e40005)))), ((locals.var_cnst0soi_dn10 * assign28370_e40005) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28370_e40005)))), ((locals.var_cnst0soi_dn11 * assign28370_e40005) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn11) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28370_e40005)))), ((locals.var_cnst0soi_dn12 * assign28370_e40005) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn12) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28370_e40005)))), ((locals.var_cnst0soi_dn17 * assign28370_e40005) + (locals.var_cnst0soi * ((-((locals.var_mks_nover * locals.var_nsub_dn17) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign28370_e40005)))),)
    } else {
        (locals.var_cnst0over__blk928, locals.var_cnst0over__blk928_dn0, locals.var_cnst0over__blk928_dn2, locals.var_cnst0over__blk928_dn6, locals.var_cnst0over__blk928_dn7, locals.var_cnst0over__blk928_dn10, locals.var_cnst0over__blk928_dn11, locals.var_cnst0over__blk928_dn12, locals.var_cnst0over__blk928_dn17,)
    }
};
        locals.var_cnst0over__blk928 = assign28370_e40008;
        locals.var_cnst0over__blk928_dn0 = assign28370_e40008_d_n0;
        locals.var_cnst0over__blk928_dn2 = assign28370_e40008_d_n2;
        locals.var_cnst0over__blk928_dn6 = assign28370_e40008_d_n6;
        locals.var_cnst0over__blk928_dn7 = assign28370_e40008_d_n7;
        locals.var_cnst0over__blk928_dn10 = assign28370_e40008_d_n10;
        locals.var_cnst0over__blk928_dn11 = assign28370_e40008_d_n11;
        locals.var_cnst0over__blk928_dn12 = assign28370_e40008_d_n12;
        locals.var_cnst0over__blk928_dn17 = assign28370_e40008_d_n17;
        locals.var_cnst0over__blk928_rv = 0.0;

        let (assign28380_e40021,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign28380_e40017: f64 = (1.0 - -1.0);
        let assign28380_e40019: f64 = (assign28380_e40017 / 2.0);
        (assign28380_e40019,)
    } else {
        (locals.var_flg_ovloops__blk912,)
    }
};
        locals.var_flg_ovloops__blk912 = assign28380_e40021;
        locals.var_flg_ovloops__blk912_rv = 0.0;

        let (assign28390_e40034,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign28390_e40030: f64 = (1.0 + -1.0);
        let assign28390_e40032: f64 = (assign28390_e40030 / 2.0);
        (assign28390_e40032,)
    } else {
        (locals.var_flg_ovloopd__blk913,)
    }
};
        locals.var_flg_ovloopd__blk913 = assign28390_e40034;
        locals.var_flg_ovloopd__blk913_rv = 0.0;

        let assign28400_e40037: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard980 = assign28400_e40037;
        locals.var_guard980_rv = 0.0;

        let (assign28410_e40056, assign28410_e40056_d_n0, assign28410_e40056_d_n2, assign28410_e40056_d_n6, assign28410_e40056_d_n7, assign28410_e40056_d_n10, assign28410_e40056_d_n11, assign28410_e40056_d_n12, assign28410_e40056_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard980 != 0.0)) {
        let assign28410_e40048: f64 = (locals.var_modenml * locals.var_vbs);
        let assign28410_e40052: f64 = (locals.var_vbs - locals.var_vds);
        let assign28410_e40053: f64 = (locals.var_modervs * assign28410_e40052);
        let assign28410_e40054: f64 = (assign28410_e40048 + assign28410_e40053);
        (assign28410_e40054, ((locals.var_modenml * locals.var_vbs_dn0) + (locals.var_modervs * (locals.var_vbs_dn0 - locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vbs_dn2) + (locals.var_modervs * (locals.var_vbs_dn2 - locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vbs_dn6) + (locals.var_modervs * (locals.var_vbs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vbs_dn7) + (locals.var_modervs * (locals.var_vbs_dn7 - locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vbs_dn10) + (locals.var_modervs * (locals.var_vbs_dn10 - locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vbs_dn11) + (locals.var_modervs * (locals.var_vbs_dn11 - locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vbs_dn12) + (locals.var_modervs * (locals.var_vbs_dn12 - locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vbs_dn17) + (locals.var_modervs * (locals.var_vbs_dn17 - locals.var_vds_dn17))),)
    } else {
        (locals.var_vbsgmt__blk922, locals.var_vbsgmt__blk922_dn0, locals.var_vbsgmt__blk922_dn2, locals.var_vbsgmt__blk922_dn6, locals.var_vbsgmt__blk922_dn7, locals.var_vbsgmt__blk922_dn10, locals.var_vbsgmt__blk922_dn11, locals.var_vbsgmt__blk922_dn12, locals.var_vbsgmt__blk922_dn17,)
    }
};
        locals.var_vbsgmt__blk922 = assign28410_e40056;
        locals.var_vbsgmt__blk922_dn0 = assign28410_e40056_d_n0;
        locals.var_vbsgmt__blk922_dn2 = assign28410_e40056_d_n2;
        locals.var_vbsgmt__blk922_dn6 = assign28410_e40056_d_n6;
        locals.var_vbsgmt__blk922_dn7 = assign28410_e40056_d_n7;
        locals.var_vbsgmt__blk922_dn10 = assign28410_e40056_d_n10;
        locals.var_vbsgmt__blk922_dn11 = assign28410_e40056_d_n11;
        locals.var_vbsgmt__blk922_dn12 = assign28410_e40056_d_n12;
        locals.var_vbsgmt__blk922_dn17 = assign28410_e40056_d_n17;
        locals.var_vbsgmt__blk922_rv = 0.0;

        let (assign28420_e40074, assign28420_e40074_d_n0, assign28420_e40074_d_n2, assign28420_e40074_d_n6, assign28420_e40074_d_n7, assign28420_e40074_d_n10, assign28420_e40074_d_n11, assign28420_e40074_d_n12, assign28420_e40074_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard980 != 0.0)) {
        let assign28420_e40067: f64 = (locals.var_modenml * locals.var_vds);
        let assign28420_e40070: f64 = (-locals.var_vds);
        let assign28420_e40071: f64 = (locals.var_modervs * assign28420_e40070);
        let assign28420_e40072: f64 = (assign28420_e40067 + assign28420_e40071);
        (assign28420_e40072, ((locals.var_modenml * locals.var_vds_dn0) + (locals.var_modervs * (-locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vds_dn2) + (locals.var_modervs * (-locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vds_dn6) + (locals.var_modervs * (-locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vds_dn7) + (locals.var_modervs * (-locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vds_dn10) + (locals.var_modervs * (-locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vds_dn11) + (locals.var_modervs * (-locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vds_dn12) + (locals.var_modervs * (-locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vds_dn17) + (locals.var_modervs * (-locals.var_vds_dn17))),)
    } else {
        (locals.var_vdsgmt__blk923, locals.var_vdsgmt__blk923_dn0, locals.var_vdsgmt__blk923_dn2, locals.var_vdsgmt__blk923_dn6, locals.var_vdsgmt__blk923_dn7, locals.var_vdsgmt__blk923_dn10, locals.var_vdsgmt__blk923_dn11, locals.var_vdsgmt__blk923_dn12, locals.var_vdsgmt__blk923_dn17,)
    }
};
        locals.var_vdsgmt__blk923 = assign28420_e40074;
        locals.var_vdsgmt__blk923_dn0 = assign28420_e40074_d_n0;
        locals.var_vdsgmt__blk923_dn2 = assign28420_e40074_d_n2;
        locals.var_vdsgmt__blk923_dn6 = assign28420_e40074_d_n6;
        locals.var_vdsgmt__blk923_dn7 = assign28420_e40074_d_n7;
        locals.var_vdsgmt__blk923_dn10 = assign28420_e40074_d_n10;
        locals.var_vdsgmt__blk923_dn11 = assign28420_e40074_d_n11;
        locals.var_vdsgmt__blk923_dn12 = assign28420_e40074_d_n12;
        locals.var_vdsgmt__blk923_dn17 = assign28420_e40074_d_n17;
        locals.var_vdsgmt__blk923_rv = 0.0;

        let (assign28430_e40093, assign28430_e40093_d_n0, assign28430_e40093_d_n2, assign28430_e40093_d_n6, assign28430_e40093_d_n7, assign28430_e40093_d_n10, assign28430_e40093_d_n11, assign28430_e40093_d_n12, assign28430_e40093_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard980 != 0.0)) {
        let assign28430_e40085: f64 = (locals.var_modenml * locals.var_vgs);
        let assign28430_e40089: f64 = (locals.var_vgs - locals.var_vds);
        let assign28430_e40090: f64 = (locals.var_modervs * assign28430_e40089);
        let assign28430_e40091: f64 = (assign28430_e40085 + assign28430_e40090);
        (assign28430_e40091, (locals.var_modervs * (-locals.var_vds_dn0)), (locals.var_modervs * (-locals.var_vds_dn2)), ((locals.var_modenml * locals.var_vgs_dn6) + (locals.var_modervs * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vgs_dn7) + (locals.var_modervs * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modervs * (-locals.var_vds_dn10)), ((locals.var_modenml * locals.var_vgs_dn11) + (locals.var_modervs * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modervs * (-locals.var_vds_dn12)), (locals.var_modervs * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgsgmt__blk924, locals.var_vgsgmt__blk924_dn0, locals.var_vgsgmt__blk924_dn2, locals.var_vgsgmt__blk924_dn6, locals.var_vgsgmt__blk924_dn7, locals.var_vgsgmt__blk924_dn10, locals.var_vgsgmt__blk924_dn11, locals.var_vgsgmt__blk924_dn12, locals.var_vgsgmt__blk924_dn17,)
    }
};
        locals.var_vgsgmt__blk924 = assign28430_e40093;
        locals.var_vgsgmt__blk924_dn0 = assign28430_e40093_d_n0;
        locals.var_vgsgmt__blk924_dn2 = assign28430_e40093_d_n2;
        locals.var_vgsgmt__blk924_dn6 = assign28430_e40093_d_n6;
        locals.var_vgsgmt__blk924_dn7 = assign28430_e40093_d_n7;
        locals.var_vgsgmt__blk924_dn10 = assign28430_e40093_d_n10;
        locals.var_vgsgmt__blk924_dn11 = assign28430_e40093_d_n11;
        locals.var_vgsgmt__blk924_dn12 = assign28430_e40093_d_n12;
        locals.var_vgsgmt__blk924_dn17 = assign28430_e40093_d_n17;
        locals.var_vgsgmt__blk924_rv = 0.0;

        let (assign28440_e40106, assign28440_e40106_d_n0, assign28440_e40106_d_n2, assign28440_e40106_d_n6, assign28440_e40106_d_n7, assign28440_e40106_d_n10, assign28440_e40106_d_n11, assign28440_e40106_d_n12, assign28440_e40106_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard980 != 0.0)) {
        let assign28440_e40104: f64 = (locals.var_vdsgmt__blk923 - locals.var_vbsgmt__blk922);
        (assign28440_e40104, (locals.var_vdsgmt__blk923_dn0 - locals.var_vbsgmt__blk922_dn0), (locals.var_vdsgmt__blk923_dn2 - locals.var_vbsgmt__blk922_dn2), (locals.var_vdsgmt__blk923_dn6 - locals.var_vbsgmt__blk922_dn6), (locals.var_vdsgmt__blk923_dn7 - locals.var_vbsgmt__blk922_dn7), (locals.var_vdsgmt__blk923_dn10 - locals.var_vbsgmt__blk922_dn10), (locals.var_vdsgmt__blk923_dn11 - locals.var_vbsgmt__blk922_dn11), (locals.var_vdsgmt__blk923_dn12 - locals.var_vbsgmt__blk922_dn12), (locals.var_vdsgmt__blk923_dn17 - locals.var_vbsgmt__blk922_dn17),)
    } else {
        (locals.var_vdbgmt__blk925, locals.var_vdbgmt__blk925_dn0, locals.var_vdbgmt__blk925_dn2, locals.var_vdbgmt__blk925_dn6, locals.var_vdbgmt__blk925_dn7, locals.var_vdbgmt__blk925_dn10, locals.var_vdbgmt__blk925_dn11, locals.var_vdbgmt__blk925_dn12, locals.var_vdbgmt__blk925_dn17,)
    }
};
        locals.var_vdbgmt__blk925 = assign28440_e40106;
        locals.var_vdbgmt__blk925_dn0 = assign28440_e40106_d_n0;
        locals.var_vdbgmt__blk925_dn2 = assign28440_e40106_d_n2;
        locals.var_vdbgmt__blk925_dn6 = assign28440_e40106_d_n6;
        locals.var_vdbgmt__blk925_dn7 = assign28440_e40106_d_n7;
        locals.var_vdbgmt__blk925_dn10 = assign28440_e40106_d_n10;
        locals.var_vdbgmt__blk925_dn11 = assign28440_e40106_d_n11;
        locals.var_vdbgmt__blk925_dn12 = assign28440_e40106_d_n12;
        locals.var_vdbgmt__blk925_dn17 = assign28440_e40106_d_n17;
        locals.var_vdbgmt__blk925_rv = 0.0;

        let (assign28450_e40119, assign28450_e40119_d_n0, assign28450_e40119_d_n2, assign28450_e40119_d_n6, assign28450_e40119_d_n7, assign28450_e40119_d_n10, assign28450_e40119_d_n11, assign28450_e40119_d_n12, assign28450_e40119_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard980 != 0.0)) {
        let assign28450_e40117: f64 = (locals.var_vgsgmt__blk924 - locals.var_vbsgmt__blk922);
        (assign28450_e40117, (locals.var_vgsgmt__blk924_dn0 - locals.var_vbsgmt__blk922_dn0), (locals.var_vgsgmt__blk924_dn2 - locals.var_vbsgmt__blk922_dn2), (locals.var_vgsgmt__blk924_dn6 - locals.var_vbsgmt__blk922_dn6), (locals.var_vgsgmt__blk924_dn7 - locals.var_vbsgmt__blk922_dn7), (locals.var_vgsgmt__blk924_dn10 - locals.var_vbsgmt__blk922_dn10), (locals.var_vgsgmt__blk924_dn11 - locals.var_vbsgmt__blk922_dn11), (locals.var_vgsgmt__blk924_dn12 - locals.var_vbsgmt__blk922_dn12), (locals.var_vgsgmt__blk924_dn17 - locals.var_vbsgmt__blk922_dn17),)
    } else {
        (locals.var_vgbgmt__blk927, locals.var_vgbgmt__blk927_dn0, locals.var_vgbgmt__blk927_dn2, locals.var_vgbgmt__blk927_dn6, locals.var_vgbgmt__blk927_dn7, locals.var_vgbgmt__blk927_dn10, locals.var_vgbgmt__blk927_dn11, locals.var_vgbgmt__blk927_dn12, locals.var_vgbgmt__blk927_dn17,)
    }
};
        locals.var_vgbgmt__blk927 = assign28450_e40119;
        locals.var_vgbgmt__blk927_dn0 = assign28450_e40119_d_n0;
        locals.var_vgbgmt__blk927_dn2 = assign28450_e40119_d_n2;
        locals.var_vgbgmt__blk927_dn6 = assign28450_e40119_d_n6;
        locals.var_vgbgmt__blk927_dn7 = assign28450_e40119_d_n7;
        locals.var_vgbgmt__blk927_dn10 = assign28450_e40119_d_n10;
        locals.var_vgbgmt__blk927_dn11 = assign28450_e40119_d_n11;
        locals.var_vgbgmt__blk927_dn12 = assign28450_e40119_d_n12;
        locals.var_vgbgmt__blk927_dn17 = assign28450_e40119_d_n17;
        locals.var_vgbgmt__blk927_rv = 0.0;

        let (assign28460_e40131, assign28460_e40131_d_n0, assign28460_e40131_d_n2, assign28460_e40131_d_n6, assign28460_e40131_d_n7, assign28460_e40131_d_n10, assign28460_e40131_d_n11, assign28460_e40131_d_n12, assign28460_e40131_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard980 != 0.0)) {
        let assign28460_e40129: f64 = (-locals.var_vbsgmt__blk922);
        (assign28460_e40129, (-locals.var_vbsgmt__blk922_dn0), (-locals.var_vbsgmt__blk922_dn2), (-locals.var_vbsgmt__blk922_dn6), (-locals.var_vbsgmt__blk922_dn7), (-locals.var_vbsgmt__blk922_dn10), (-locals.var_vbsgmt__blk922_dn11), (-locals.var_vbsgmt__blk922_dn12), (-locals.var_vbsgmt__blk922_dn17),)
    } else {
        (locals.var_vsbgmt__blk926, locals.var_vsbgmt__blk926_dn0, locals.var_vsbgmt__blk926_dn2, locals.var_vsbgmt__blk926_dn6, locals.var_vsbgmt__blk926_dn7, locals.var_vsbgmt__blk926_dn10, locals.var_vsbgmt__blk926_dn11, locals.var_vsbgmt__blk926_dn12, locals.var_vsbgmt__blk926_dn17,)
    }
};
        locals.var_vsbgmt__blk926 = assign28460_e40131;
        locals.var_vsbgmt__blk926_dn0 = assign28460_e40131_d_n0;
        locals.var_vsbgmt__blk926_dn2 = assign28460_e40131_d_n2;
        locals.var_vsbgmt__blk926_dn6 = assign28460_e40131_d_n6;
        locals.var_vsbgmt__blk926_dn7 = assign28460_e40131_d_n7;
        locals.var_vsbgmt__blk926_dn10 = assign28460_e40131_d_n10;
        locals.var_vsbgmt__blk926_dn11 = assign28460_e40131_d_n11;
        locals.var_vsbgmt__blk926_dn12 = assign28460_e40131_d_n12;
        locals.var_vsbgmt__blk926_dn17 = assign28460_e40131_d_n17;
        locals.var_vsbgmt__blk926_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_103(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28470_e40148,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard980 != 0.0)) {
        let assign28470_e40142: f64 = (locals.var_flg_ovloops__blk912 * locals.var_modenml);
        let assign28470_e40145: f64 = (locals.var_flg_ovloopd__blk913 * locals.var_modervs);
        let assign28470_e40146: f64 = (assign28470_e40142 + assign28470_e40145);
        (assign28470_e40146,)
    } else {
        (locals.var_flg_overs__blk914,)
    }
};
        locals.var_flg_overs__blk914 = assign28470_e40148;
        locals.var_flg_overs__blk914_rv = 0.0;

        let (assign28480_e40165,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard980 != 0.0)) {
        let assign28480_e40159: f64 = (locals.var_flg_ovloops__blk912 * locals.var_modervs);
        let assign28480_e40162: f64 = (locals.var_flg_ovloopd__blk913 * locals.var_modenml);
        let assign28480_e40163: f64 = (assign28480_e40159 + assign28480_e40162);
        (assign28480_e40163,)
    } else {
        (locals.var_flg_overd__blk915,)
    }
};
        locals.var_flg_overd__blk915 = assign28480_e40165;
        locals.var_flg_overd__blk915_rv = 0.0;

        let (assign28490_e40186, assign28490_e40186_d_n0, assign28490_e40186_d_n2, assign28490_e40186_d_n6, assign28490_e40186_d_n7, assign28490_e40186_d_n10, assign28490_e40186_d_n11, assign28490_e40186_d_n12, assign28490_e40186_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard980 != 0.0)) {
        let assign28490_e40176: f64 = (locals.var_flg_overs__blk914 * locals.var_vsbgmt__blk926);
        let assign28490_e40179: f64 = (locals.var_flg_overd__blk915 * locals.var_vdbgmt__blk925);
        let assign28490_e40180: f64 = (assign28490_e40176 + assign28490_e40179);
        let assign28490_e40183: f64 = (10.0 * 2.220446049250313e-16);
        let assign28490_e40184: f64 = (assign28490_e40180 + assign28490_e40183);
        (assign28490_e40184, ((locals.var_flg_overs__blk914 * locals.var_vsbgmt__blk926_dn0) + (locals.var_flg_overd__blk915 * locals.var_vdbgmt__blk925_dn0)), ((locals.var_flg_overs__blk914 * locals.var_vsbgmt__blk926_dn2) + (locals.var_flg_overd__blk915 * locals.var_vdbgmt__blk925_dn2)), ((locals.var_flg_overs__blk914 * locals.var_vsbgmt__blk926_dn6) + (locals.var_flg_overd__blk915 * locals.var_vdbgmt__blk925_dn6)), ((locals.var_flg_overs__blk914 * locals.var_vsbgmt__blk926_dn7) + (locals.var_flg_overd__blk915 * locals.var_vdbgmt__blk925_dn7)), ((locals.var_flg_overs__blk914 * locals.var_vsbgmt__blk926_dn10) + (locals.var_flg_overd__blk915 * locals.var_vdbgmt__blk925_dn10)), ((locals.var_flg_overs__blk914 * locals.var_vsbgmt__blk926_dn11) + (locals.var_flg_overd__blk915 * locals.var_vdbgmt__blk925_dn11)), ((locals.var_flg_overs__blk914 * locals.var_vsbgmt__blk926_dn12) + (locals.var_flg_overd__blk915 * locals.var_vdbgmt__blk925_dn12)), ((locals.var_flg_overs__blk914 * locals.var_vsbgmt__blk926_dn17) + (locals.var_flg_overd__blk915 * locals.var_vdbgmt__blk925_dn17)),)
    } else {
        (locals.var_vxbgmt__blk920, locals.var_vxbgmt__blk920_dn0, locals.var_vxbgmt__blk920_dn2, locals.var_vxbgmt__blk920_dn6, locals.var_vxbgmt__blk920_dn7, locals.var_vxbgmt__blk920_dn10, locals.var_vxbgmt__blk920_dn11, locals.var_vxbgmt__blk920_dn12, locals.var_vxbgmt__blk920_dn17,)
    }
};
        locals.var_vxbgmt__blk920 = assign28490_e40186;
        locals.var_vxbgmt__blk920_dn0 = assign28490_e40186_d_n0;
        locals.var_vxbgmt__blk920_dn2 = assign28490_e40186_d_n2;
        locals.var_vxbgmt__blk920_dn6 = assign28490_e40186_d_n6;
        locals.var_vxbgmt__blk920_dn7 = assign28490_e40186_d_n7;
        locals.var_vxbgmt__blk920_dn10 = assign28490_e40186_d_n10;
        locals.var_vxbgmt__blk920_dn11 = assign28490_e40186_d_n11;
        locals.var_vxbgmt__blk920_dn12 = assign28490_e40186_d_n12;
        locals.var_vxbgmt__blk920_dn17 = assign28490_e40186_d_n17;
        locals.var_vxbgmt__blk920_rv = 0.0;

        let (assign28500_e40204,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard980 == 0.0)) {
        let assign28500_e40198: f64 = (locals.var_flg_ovloops__blk912 * locals.var_modenml);
        let assign28500_e40201: f64 = (locals.var_flg_ovloopd__blk913 * locals.var_modervs);
        let assign28500_e40202: f64 = (assign28500_e40198 + assign28500_e40201);
        (assign28500_e40202,)
    } else {
        (locals.var_flg_overs__blk914,)
    }
};
        locals.var_flg_overs__blk914 = assign28500_e40204;
        locals.var_flg_overs__blk914_rv = 0.0;

        let (assign28510_e40222,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard980 == 0.0)) {
        let assign28510_e40216: f64 = (locals.var_flg_ovloops__blk912 * locals.var_modervs);
        let assign28510_e40219: f64 = (locals.var_flg_ovloopd__blk913 * locals.var_modenml);
        let assign28510_e40220: f64 = (assign28510_e40216 + assign28510_e40219);
        (assign28510_e40220,)
    } else {
        (locals.var_flg_overd__blk915,)
    }
};
        locals.var_flg_overd__blk915 = assign28510_e40222;
        locals.var_flg_overd__blk915_rv = 0.0;

        let (assign28520_e40244, assign28520_e40244_d_n0, assign28520_e40244_d_n2, assign28520_e40244_d_n6, assign28520_e40244_d_n7, assign28520_e40244_d_n10, assign28520_e40244_d_n11, assign28520_e40244_d_n12, assign28520_e40244_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard980 == 0.0)) && (locals.var_flg_ovloops__blk912 != 0.0)) {
        let assign28520_e40236: f64 = (locals.var_modenml * locals.var_vgs);
        let assign28520_e40240: f64 = (locals.var_vgs - locals.var_vds);
        let assign28520_e40241: f64 = (locals.var_modervs * assign28520_e40240);
        let assign28520_e40242: f64 = (assign28520_e40236 + assign28520_e40241);
        (assign28520_e40242, (locals.var_modervs * (-locals.var_vds_dn0)), (locals.var_modervs * (-locals.var_vds_dn2)), ((locals.var_modenml * locals.var_vgs_dn6) + (locals.var_modervs * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vgs_dn7) + (locals.var_modervs * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modervs * (-locals.var_vds_dn10)), ((locals.var_modenml * locals.var_vgs_dn11) + (locals.var_modervs * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modervs * (-locals.var_vds_dn12)), (locals.var_modervs * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgbgmt__blk927, locals.var_vgbgmt__blk927_dn0, locals.var_vgbgmt__blk927_dn2, locals.var_vgbgmt__blk927_dn6, locals.var_vgbgmt__blk927_dn7, locals.var_vgbgmt__blk927_dn10, locals.var_vgbgmt__blk927_dn11, locals.var_vgbgmt__blk927_dn12, locals.var_vgbgmt__blk927_dn17,)
    }
};
        locals.var_vgbgmt__blk927 = assign28520_e40244;
        locals.var_vgbgmt__blk927_dn0 = assign28520_e40244_d_n0;
        locals.var_vgbgmt__blk927_dn2 = assign28520_e40244_d_n2;
        locals.var_vgbgmt__blk927_dn6 = assign28520_e40244_d_n6;
        locals.var_vgbgmt__blk927_dn7 = assign28520_e40244_d_n7;
        locals.var_vgbgmt__blk927_dn10 = assign28520_e40244_d_n10;
        locals.var_vgbgmt__blk927_dn11 = assign28520_e40244_d_n11;
        locals.var_vgbgmt__blk927_dn12 = assign28520_e40244_d_n12;
        locals.var_vgbgmt__blk927_dn17 = assign28520_e40244_d_n17;
        locals.var_vgbgmt__blk927_rv = 0.0;

        let (assign28530_e40266, assign28530_e40266_d_n0, assign28530_e40266_d_n2, assign28530_e40266_d_n6, assign28530_e40266_d_n7, assign28530_e40266_d_n10, assign28530_e40266_d_n11, assign28530_e40266_d_n12, assign28530_e40266_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard980 == 0.0)) && (locals.var_flg_ovloopd__blk913 != 0.0)) {
        let assign28530_e40258: f64 = (locals.var_modervs * locals.var_vgs);
        let assign28530_e40262: f64 = (locals.var_vgs - locals.var_vds);
        let assign28530_e40263: f64 = (locals.var_modenml * assign28530_e40262);
        let assign28530_e40264: f64 = (assign28530_e40258 + assign28530_e40263);
        (assign28530_e40264, (locals.var_modenml * (-locals.var_vds_dn0)), (locals.var_modenml * (-locals.var_vds_dn2)), ((locals.var_modervs * locals.var_vgs_dn6) + (locals.var_modenml * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modervs * locals.var_vgs_dn7) + (locals.var_modenml * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modenml * (-locals.var_vds_dn10)), ((locals.var_modervs * locals.var_vgs_dn11) + (locals.var_modenml * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modenml * (-locals.var_vds_dn12)), (locals.var_modenml * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgbgmt__blk927, locals.var_vgbgmt__blk927_dn0, locals.var_vgbgmt__blk927_dn2, locals.var_vgbgmt__blk927_dn6, locals.var_vgbgmt__blk927_dn7, locals.var_vgbgmt__blk927_dn10, locals.var_vgbgmt__blk927_dn11, locals.var_vgbgmt__blk927_dn12, locals.var_vgbgmt__blk927_dn17,)
    }
};
        locals.var_vgbgmt__blk927 = assign28530_e40266;
        locals.var_vgbgmt__blk927_dn0 = assign28530_e40266_d_n0;
        locals.var_vgbgmt__blk927_dn2 = assign28530_e40266_d_n2;
        locals.var_vgbgmt__blk927_dn6 = assign28530_e40266_d_n6;
        locals.var_vgbgmt__blk927_dn7 = assign28530_e40266_d_n7;
        locals.var_vgbgmt__blk927_dn10 = assign28530_e40266_d_n10;
        locals.var_vgbgmt__blk927_dn11 = assign28530_e40266_d_n11;
        locals.var_vgbgmt__blk927_dn12 = assign28530_e40266_d_n12;
        locals.var_vgbgmt__blk927_dn17 = assign28530_e40266_d_n17;
        locals.var_vgbgmt__blk927_rv = 0.0;

        let (assign28540_e40278, assign28540_e40278_d_n0, assign28540_e40278_d_n2, assign28540_e40278_d_n6, assign28540_e40278_d_n7, assign28540_e40278_d_n10, assign28540_e40278_d_n11, assign28540_e40278_d_n12, assign28540_e40278_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard980 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt__blk920, locals.var_vxbgmt__blk920_dn0, locals.var_vxbgmt__blk920_dn2, locals.var_vxbgmt__blk920_dn6, locals.var_vxbgmt__blk920_dn7, locals.var_vxbgmt__blk920_dn10, locals.var_vxbgmt__blk920_dn11, locals.var_vxbgmt__blk920_dn12, locals.var_vxbgmt__blk920_dn17,)
    }
};
        locals.var_vxbgmt__blk920 = assign28540_e40278;
        locals.var_vxbgmt__blk920_dn0 = assign28540_e40278_d_n0;
        locals.var_vxbgmt__blk920_dn2 = assign28540_e40278_d_n2;
        locals.var_vxbgmt__blk920_dn6 = assign28540_e40278_d_n6;
        locals.var_vxbgmt__blk920_dn7 = assign28540_e40278_d_n7;
        locals.var_vxbgmt__blk920_dn10 = assign28540_e40278_d_n10;
        locals.var_vxbgmt__blk920_dn11 = assign28540_e40278_d_n11;
        locals.var_vxbgmt__blk920_dn12 = assign28540_e40278_d_n12;
        locals.var_vxbgmt__blk920_dn17 = assign28540_e40278_d_n17;
        locals.var_vxbgmt__blk920_rv = 0.0;

        let (assign28550_e40288, assign28550_e40288_d_n0, assign28550_e40288_d_n2, assign28550_e40288_d_n6, assign28550_e40288_d_n7, assign28550_e40288_d_n10, assign28550_e40288_d_n11, assign28550_e40288_d_n12, assign28550_e40288_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign28550_e40286: f64 = (-locals.var_vxbgmt__blk920);
        (assign28550_e40286, (-locals.var_vxbgmt__blk920_dn0), (-locals.var_vxbgmt__blk920_dn2), (-locals.var_vxbgmt__blk920_dn6), (-locals.var_vxbgmt__blk920_dn7), (-locals.var_vxbgmt__blk920_dn10), (-locals.var_vxbgmt__blk920_dn11), (-locals.var_vxbgmt__blk920_dn12), (-locals.var_vxbgmt__blk920_dn17),)
    } else {
        (locals.var_t0__blk895, locals.var_t0__blk895_dn0, locals.var_t0__blk895_dn2, locals.var_t0__blk895_dn6, locals.var_t0__blk895_dn7, locals.var_t0__blk895_dn10, locals.var_t0__blk895_dn11, locals.var_t0__blk895_dn12, locals.var_t0__blk895_dn17,)
    }
};
        locals.var_t0__blk895 = assign28550_e40288;
        locals.var_t0__blk895_dn0 = assign28550_e40288_d_n0;
        locals.var_t0__blk895_dn2 = assign28550_e40288_d_n2;
        locals.var_t0__blk895_dn6 = assign28550_e40288_d_n6;
        locals.var_t0__blk895_dn7 = assign28550_e40288_d_n7;
        locals.var_t0__blk895_dn10 = assign28550_e40288_d_n10;
        locals.var_t0__blk895_dn11 = assign28550_e40288_d_n11;
        locals.var_t0__blk895_dn12 = assign28550_e40288_d_n12;
        locals.var_t0__blk895_dn17 = assign28550_e40288_d_n17;
        locals.var_t0__blk895_rv = 0.0;

        let assign28560_e40291: f64 = if locals.var_t0__blk895 > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard981 = assign28560_e40291;
        locals.var_guard981_rv = 0.0;

        let (assign28570_e40304, assign28570_e40304_d_n0, assign28570_e40304_d_n2, assign28570_e40304_d_n6, assign28570_e40304_d_n7, assign28570_e40304_d_n10, assign28570_e40304_d_n11, assign28570_e40304_d_n12, assign28570_e40304_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard981 != 0.0)) {
        let assign28570_e40302: f64 = (locals.var_t0__blk895 - locals.var_vbs_bnd);
        (assign28570_e40302, locals.var_t0__blk895_dn0, locals.var_t0__blk895_dn2, locals.var_t0__blk895_dn6, locals.var_t0__blk895_dn7, locals.var_t0__blk895_dn10, locals.var_t0__blk895_dn11, locals.var_t0__blk895_dn12, locals.var_t0__blk895_dn17,)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign28570_e40304;
        locals.var_t1__blk896_dn0 = assign28570_e40304_d_n0;
        locals.var_t1__blk896_dn2 = assign28570_e40304_d_n2;
        locals.var_t1__blk896_dn6 = assign28570_e40304_d_n6;
        locals.var_t1__blk896_dn7 = assign28570_e40304_d_n7;
        locals.var_t1__blk896_dn10 = assign28570_e40304_d_n10;
        locals.var_t1__blk896_dn11 = assign28570_e40304_d_n11;
        locals.var_t1__blk896_dn12 = assign28570_e40304_d_n12;
        locals.var_t1__blk896_dn17 = assign28570_e40304_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let (assign28580_e40317, assign28580_e40317_d_n0, assign28580_e40317_d_n2, assign28580_e40317_d_n6, assign28580_e40317_d_n7, assign28580_e40317_d_n10, assign28580_e40317_d_n11, assign28580_e40317_d_n12, assign28580_e40317_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard981 != 0.0)) {
        let assign28580_e40315: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign28580_e40315, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk897, locals.var_t2__blk897_dn0, locals.var_t2__blk897_dn2, locals.var_t2__blk897_dn6, locals.var_t2__blk897_dn7, locals.var_t2__blk897_dn10, locals.var_t2__blk897_dn11, locals.var_t2__blk897_dn12, locals.var_t2__blk897_dn17,)
    }
};
        locals.var_t2__blk897 = assign28580_e40317;
        locals.var_t2__blk897_dn0 = assign28580_e40317_d_n0;
        locals.var_t2__blk897_dn2 = assign28580_e40317_d_n2;
        locals.var_t2__blk897_dn6 = assign28580_e40317_d_n6;
        locals.var_t2__blk897_dn7 = assign28580_e40317_d_n7;
        locals.var_t2__blk897_dn10 = assign28580_e40317_d_n10;
        locals.var_t2__blk897_dn11 = assign28580_e40317_d_n11;
        locals.var_t2__blk897_dn12 = assign28580_e40317_d_n12;
        locals.var_t2__blk897_dn17 = assign28580_e40317_d_n17;
        locals.var_t2__blk897_rv = 0.0;

        let (assign28590_e40330, assign28590_e40330_d_n0, assign28590_e40330_d_n2, assign28590_e40330_d_n6, assign28590_e40330_d_n7, assign28590_e40330_d_n10, assign28590_e40330_d_n11, assign28590_e40330_d_n12, assign28590_e40330_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard981 != 0.0)) {
        let assign28590_e40328: f64 = (locals.var_t1__blk896 / locals.var_t2__blk897);
        (assign28590_e40328, (((locals.var_t1__blk896_dn0 * locals.var_t2__blk897) - (locals.var_t1__blk896 * locals.var_t2__blk897_dn0)) / (locals.var_t2__blk897 * locals.var_t2__blk897)), (((locals.var_t1__blk896_dn2 * locals.var_t2__blk897) - (locals.var_t1__blk896 * locals.var_t2__blk897_dn2)) / (locals.var_t2__blk897 * locals.var_t2__blk897)), (((locals.var_t1__blk896_dn6 * locals.var_t2__blk897) - (locals.var_t1__blk896 * locals.var_t2__blk897_dn6)) / (locals.var_t2__blk897 * locals.var_t2__blk897)), (((locals.var_t1__blk896_dn7 * locals.var_t2__blk897) - (locals.var_t1__blk896 * locals.var_t2__blk897_dn7)) / (locals.var_t2__blk897 * locals.var_t2__blk897)), (((locals.var_t1__blk896_dn10 * locals.var_t2__blk897) - (locals.var_t1__blk896 * locals.var_t2__blk897_dn10)) / (locals.var_t2__blk897 * locals.var_t2__blk897)), (((locals.var_t1__blk896_dn11 * locals.var_t2__blk897) - (locals.var_t1__blk896 * locals.var_t2__blk897_dn11)) / (locals.var_t2__blk897 * locals.var_t2__blk897)), (((locals.var_t1__blk896_dn12 * locals.var_t2__blk897) - (locals.var_t1__blk896 * locals.var_t2__blk897_dn12)) / (locals.var_t2__blk897 * locals.var_t2__blk897)), (((locals.var_t1__blk896_dn17 * locals.var_t2__blk897) - (locals.var_t1__blk896 * locals.var_t2__blk897_dn17)) / (locals.var_t2__blk897 * locals.var_t2__blk897)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign28590_e40330;
        locals.var_tmf1_dn0 = assign28590_e40330_d_n0;
        locals.var_tmf1_dn2 = assign28590_e40330_d_n2;
        locals.var_tmf1_dn6 = assign28590_e40330_d_n6;
        locals.var_tmf1_dn7 = assign28590_e40330_d_n7;
        locals.var_tmf1_dn10 = assign28590_e40330_d_n10;
        locals.var_tmf1_dn11 = assign28590_e40330_d_n11;
        locals.var_tmf1_dn12 = assign28590_e40330_d_n12;
        locals.var_tmf1_dn17 = assign28590_e40330_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign28600_e40343, assign28600_e40343_d_n0, assign28600_e40343_d_n2, assign28600_e40343_d_n6, assign28600_e40343_d_n7, assign28600_e40343_d_n10, assign28600_e40343_d_n11, assign28600_e40343_d_n12, assign28600_e40343_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard981 != 0.0)) {
        let assign28600_e40341: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign28600_e40341, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign28600_e40343;
        locals.var_tmf2_dn0 = assign28600_e40343_d_n0;
        locals.var_tmf2_dn2 = assign28600_e40343_d_n2;
        locals.var_tmf2_dn6 = assign28600_e40343_d_n6;
        locals.var_tmf2_dn7 = assign28600_e40343_d_n7;
        locals.var_tmf2_dn10 = assign28600_e40343_d_n10;
        locals.var_tmf2_dn11 = assign28600_e40343_d_n11;
        locals.var_tmf2_dn12 = assign28600_e40343_d_n12;
        locals.var_tmf2_dn17 = assign28600_e40343_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign28610_e40356, assign28610_e40356_d_n0, assign28610_e40356_d_n2, assign28610_e40356_d_n6, assign28610_e40356_d_n7, assign28610_e40356_d_n10, assign28610_e40356_d_n11, assign28610_e40356_d_n12, assign28610_e40356_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard981 != 0.0)) {
        let assign28610_e40354: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign28610_e40354, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn12, locals.var_tmf3_dn17,)
    }
};
        locals.var_tmf3 = assign28610_e40356;
        locals.var_tmf3_dn0 = assign28610_e40356_d_n0;
        locals.var_tmf3_dn2 = assign28610_e40356_d_n2;
        locals.var_tmf3_dn6 = assign28610_e40356_d_n6;
        locals.var_tmf3_dn7 = assign28610_e40356_d_n7;
        locals.var_tmf3_dn10 = assign28610_e40356_d_n10;
        locals.var_tmf3_dn11 = assign28610_e40356_d_n11;
        locals.var_tmf3_dn12 = assign28610_e40356_d_n12;
        locals.var_tmf3_dn17 = assign28610_e40356_d_n17;
        locals.var_tmf3_rv = 0.0;

        let (assign28620_e40369, assign28620_e40369_d_n0, assign28620_e40369_d_n2, assign28620_e40369_d_n6, assign28620_e40369_d_n7, assign28620_e40369_d_n10, assign28620_e40369_d_n11, assign28620_e40369_d_n12, assign28620_e40369_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard981 != 0.0)) {
        let assign28620_e40367: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign28620_e40367, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn17)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn12, locals.var_tmf4_dn17,)
    }
};
        locals.var_tmf4 = assign28620_e40369;
        locals.var_tmf4_dn0 = assign28620_e40369_d_n0;
        locals.var_tmf4_dn2 = assign28620_e40369_d_n2;
        locals.var_tmf4_dn6 = assign28620_e40369_d_n6;
        locals.var_tmf4_dn7 = assign28620_e40369_d_n7;
        locals.var_tmf4_dn10 = assign28620_e40369_d_n10;
        locals.var_tmf4_dn11 = assign28620_e40369_d_n11;
        locals.var_tmf4_dn12 = assign28620_e40369_d_n12;
        locals.var_tmf4_dn17 = assign28620_e40369_d_n17;
        locals.var_tmf4_rv = 0.0;

        let (assign28630_e40390, assign28630_e40390_d_n0, assign28630_e40390_d_n2, assign28630_e40390_d_n6, assign28630_e40390_d_n7, assign28630_e40390_d_n10, assign28630_e40390_d_n11, assign28630_e40390_d_n12, assign28630_e40390_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard981 != 0.0)) {
        let assign28630_e40381: f64 = (1.0 + locals.var_tmf1);
        let assign28630_e40383: f64 = (assign28630_e40381 + locals.var_tmf2);
        let assign28630_e40385: f64 = (assign28630_e40383 + locals.var_tmf3);
        let assign28630_e40387: f64 = (assign28630_e40385 + locals.var_tmf4);
        let assign28630_e40388: f64 = (1.0 / assign28630_e40387);
        (assign28630_e40388, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign28630_e40387 * assign28630_e40387))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign28630_e40387 * assign28630_e40387))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign28630_e40387 * assign28630_e40387))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign28630_e40387 * assign28630_e40387))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign28630_e40387 * assign28630_e40387))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign28630_e40387 * assign28630_e40387))), (-((((locals.var_tmf1_dn12 + locals.var_tmf2_dn12) + locals.var_tmf3_dn12) + locals.var_tmf4_dn12) / (assign28630_e40387 * assign28630_e40387))), (-((((locals.var_tmf1_dn17 + locals.var_tmf2_dn17) + locals.var_tmf3_dn17) + locals.var_tmf4_dn17) / (assign28630_e40387 * assign28630_e40387))),)
    } else {
        (locals.var_ty__blk905, locals.var_ty__blk905_dn0, locals.var_ty__blk905_dn2, locals.var_ty__blk905_dn6, locals.var_ty__blk905_dn7, locals.var_ty__blk905_dn10, locals.var_ty__blk905_dn11, locals.var_ty__blk905_dn12, locals.var_ty__blk905_dn17,)
    }
};
        locals.var_ty__blk905 = assign28630_e40390;
        locals.var_ty__blk905_dn0 = assign28630_e40390_d_n0;
        locals.var_ty__blk905_dn2 = assign28630_e40390_d_n2;
        locals.var_ty__blk905_dn6 = assign28630_e40390_d_n6;
        locals.var_ty__blk905_dn7 = assign28630_e40390_d_n7;
        locals.var_ty__blk905_dn10 = assign28630_e40390_d_n10;
        locals.var_ty__blk905_dn11 = assign28630_e40390_d_n11;
        locals.var_ty__blk905_dn12 = assign28630_e40390_d_n12;
        locals.var_ty__blk905_dn17 = assign28630_e40390_d_n17;
        locals.var_ty__blk905_rv = 0.0;

        let (assign28650_e40433, assign28650_e40433_d_n0, assign28650_e40433_d_n2, assign28650_e40433_d_n6, assign28650_e40433_d_n7, assign28650_e40433_d_n10, assign28650_e40433_d_n11, assign28650_e40433_d_n12, assign28650_e40433_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard981 != 0.0)) {
        let assign28650_e40430: f64 = (1.0 - locals.var_ty__blk905);
        let assign28650_e40431: f64 = (locals.var_t2__blk897 * assign28650_e40430);
        (assign28650_e40431, ((locals.var_t2__blk897_dn0 * assign28650_e40430) + (locals.var_t2__blk897 * (-locals.var_ty__blk905_dn0))), ((locals.var_t2__blk897_dn2 * assign28650_e40430) + (locals.var_t2__blk897 * (-locals.var_ty__blk905_dn2))), ((locals.var_t2__blk897_dn6 * assign28650_e40430) + (locals.var_t2__blk897 * (-locals.var_ty__blk905_dn6))), ((locals.var_t2__blk897_dn7 * assign28650_e40430) + (locals.var_t2__blk897 * (-locals.var_ty__blk905_dn7))), ((locals.var_t2__blk897_dn10 * assign28650_e40430) + (locals.var_t2__blk897 * (-locals.var_ty__blk905_dn10))), ((locals.var_t2__blk897_dn11 * assign28650_e40430) + (locals.var_t2__blk897 * (-locals.var_ty__blk905_dn11))), ((locals.var_t2__blk897_dn12 * assign28650_e40430) + (locals.var_t2__blk897 * (-locals.var_ty__blk905_dn12))), ((locals.var_t2__blk897_dn17 * assign28650_e40430) + (locals.var_t2__blk897 * (-locals.var_ty__blk905_dn17))),)
    } else {
        (locals.var_ty__blk905, locals.var_ty__blk905_dn0, locals.var_ty__blk905_dn2, locals.var_ty__blk905_dn6, locals.var_ty__blk905_dn7, locals.var_ty__blk905_dn10, locals.var_ty__blk905_dn11, locals.var_ty__blk905_dn12, locals.var_ty__blk905_dn17,)
    }
};
        locals.var_ty__blk905 = assign28650_e40433;
        locals.var_ty__blk905_dn0 = assign28650_e40433_d_n0;
        locals.var_ty__blk905_dn2 = assign28650_e40433_d_n2;
        locals.var_ty__blk905_dn6 = assign28650_e40433_d_n6;
        locals.var_ty__blk905_dn7 = assign28650_e40433_d_n7;
        locals.var_ty__blk905_dn10 = assign28650_e40433_d_n10;
        locals.var_ty__blk905_dn11 = assign28650_e40433_d_n11;
        locals.var_ty__blk905_dn12 = assign28650_e40433_d_n12;
        locals.var_ty__blk905_dn17 = assign28650_e40433_d_n17;
        locals.var_ty__blk905_rv = 0.0;

        let (assign28670_e40458, assign28670_e40458_d_n0, assign28670_e40458_d_n2, assign28670_e40458_d_n6, assign28670_e40458_d_n7, assign28670_e40458_d_n10, assign28670_e40458_d_n11, assign28670_e40458_d_n12, assign28670_e40458_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard981 != 0.0)) {
        let assign28670_e40456: f64 = (locals.var_vbs_bnd + locals.var_ty__blk905);
        (assign28670_e40456, locals.var_ty__blk905_dn0, locals.var_ty__blk905_dn2, locals.var_ty__blk905_dn6, locals.var_ty__blk905_dn7, locals.var_ty__blk905_dn10, locals.var_ty__blk905_dn11, locals.var_ty__blk905_dn12, locals.var_ty__blk905_dn17,)
    } else {
        (locals.var_t10__blk902, locals.var_t10__blk902_dn0, locals.var_t10__blk902_dn2, locals.var_t10__blk902_dn6, locals.var_t10__blk902_dn7, locals.var_t10__blk902_dn10, locals.var_t10__blk902_dn11, locals.var_t10__blk902_dn12, locals.var_t10__blk902_dn17,)
    }
};
        locals.var_t10__blk902 = assign28670_e40458;
        locals.var_t10__blk902_dn0 = assign28670_e40458_d_n0;
        locals.var_t10__blk902_dn2 = assign28670_e40458_d_n2;
        locals.var_t10__blk902_dn6 = assign28670_e40458_d_n6;
        locals.var_t10__blk902_dn7 = assign28670_e40458_d_n7;
        locals.var_t10__blk902_dn10 = assign28670_e40458_d_n10;
        locals.var_t10__blk902_dn11 = assign28670_e40458_d_n11;
        locals.var_t10__blk902_dn12 = assign28670_e40458_d_n12;
        locals.var_t10__blk902_dn17 = assign28670_e40458_d_n17;
        locals.var_t10__blk902_rv = 0.0;

        let (assign28680_e40470, assign28680_e40470_d_n0, assign28680_e40470_d_n2, assign28680_e40470_d_n6, assign28680_e40470_d_n7, assign28680_e40470_d_n10, assign28680_e40470_d_n11, assign28680_e40470_d_n12, assign28680_e40470_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard981 == 0.0)) {
        (locals.var_t0__blk895, locals.var_t0__blk895_dn0, locals.var_t0__blk895_dn2, locals.var_t0__blk895_dn6, locals.var_t0__blk895_dn7, locals.var_t0__blk895_dn10, locals.var_t0__blk895_dn11, locals.var_t0__blk895_dn12, locals.var_t0__blk895_dn17,)
    } else {
        (locals.var_t10__blk902, locals.var_t10__blk902_dn0, locals.var_t10__blk902_dn2, locals.var_t10__blk902_dn6, locals.var_t10__blk902_dn7, locals.var_t10__blk902_dn10, locals.var_t10__blk902_dn11, locals.var_t10__blk902_dn12, locals.var_t10__blk902_dn17,)
    }
};
        locals.var_t10__blk902 = assign28680_e40470;
        locals.var_t10__blk902_dn0 = assign28680_e40470_d_n0;
        locals.var_t10__blk902_dn2 = assign28680_e40470_d_n2;
        locals.var_t10__blk902_dn6 = assign28680_e40470_d_n6;
        locals.var_t10__blk902_dn7 = assign28680_e40470_d_n7;
        locals.var_t10__blk902_dn10 = assign28680_e40470_d_n10;
        locals.var_t10__blk902_dn11 = assign28680_e40470_d_n11;
        locals.var_t10__blk902_dn12 = assign28680_e40470_d_n12;
        locals.var_t10__blk902_dn17 = assign28680_e40470_d_n17;
        locals.var_t10__blk902_rv = 0.0;

        let (assign28700_e40494, assign28700_e40494_d_n0, assign28700_e40494_d_n2, assign28700_e40494_d_n6, assign28700_e40494_d_n7, assign28700_e40494_d_n10, assign28700_e40494_d_n11, assign28700_e40494_d_n12, assign28700_e40494_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign28700_e40490: f64 = (-locals.var_t10__blk902);
        let assign28700_e40492: f64 = (assign28700_e40490 - 1e-12);
        (assign28700_e40492, (-locals.var_t10__blk902_dn0), (-locals.var_t10__blk902_dn2), (-locals.var_t10__blk902_dn6), (-locals.var_t10__blk902_dn7), (-locals.var_t10__blk902_dn10), (-locals.var_t10__blk902_dn11), (-locals.var_t10__blk902_dn12), (-locals.var_t10__blk902_dn17),)
    } else {
        (locals.var_vxbgmtcl__blk921, locals.var_vxbgmtcl__blk921_dn0, locals.var_vxbgmtcl__blk921_dn2, locals.var_vxbgmtcl__blk921_dn6, locals.var_vxbgmtcl__blk921_dn7, locals.var_vxbgmtcl__blk921_dn10, locals.var_vxbgmtcl__blk921_dn11, locals.var_vxbgmtcl__blk921_dn12, locals.var_vxbgmtcl__blk921_dn17,)
    }
};
        locals.var_vxbgmtcl__blk921 = assign28700_e40494;
        locals.var_vxbgmtcl__blk921_dn0 = assign28700_e40494_d_n0;
        locals.var_vxbgmtcl__blk921_dn2 = assign28700_e40494_d_n2;
        locals.var_vxbgmtcl__blk921_dn6 = assign28700_e40494_d_n6;
        locals.var_vxbgmtcl__blk921_dn7 = assign28700_e40494_d_n7;
        locals.var_vxbgmtcl__blk921_dn10 = assign28700_e40494_d_n10;
        locals.var_vxbgmtcl__blk921_dn11 = assign28700_e40494_d_n11;
        locals.var_vxbgmtcl__blk921_dn12 = assign28700_e40494_d_n12;
        locals.var_vxbgmtcl__blk921_dn17 = assign28700_e40494_d_n17;
        locals.var_vxbgmtcl__blk921_rv = 0.0;

        let (assign28710_e40505, assign28710_e40505_d_n0, assign28710_e40505_d_n2, assign28710_e40505_d_n6, assign28710_e40505_d_n7, assign28710_e40505_d_n10, assign28710_e40505_d_n11, assign28710_e40505_d_n12, assign28710_e40505_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign28710_e40503: f64 = (locals.var_cnst0over__blk928 * locals.var_cox0_inv__blk907);
        (assign28710_e40503, (locals.var_cnst0over__blk928_dn0 * locals.var_cox0_inv__blk907), (locals.var_cnst0over__blk928_dn2 * locals.var_cox0_inv__blk907), (locals.var_cnst0over__blk928_dn6 * locals.var_cox0_inv__blk907), (locals.var_cnst0over__blk928_dn7 * locals.var_cox0_inv__blk907), (locals.var_cnst0over__blk928_dn10 * locals.var_cox0_inv__blk907), (locals.var_cnst0over__blk928_dn11 * locals.var_cox0_inv__blk907), (locals.var_cnst0over__blk928_dn12 * locals.var_cox0_inv__blk907), (locals.var_cnst0over__blk928_dn17 * locals.var_cox0_inv__blk907),)
    } else {
        (locals.var_fac1__blk929, locals.var_fac1__blk929_dn0, locals.var_fac1__blk929_dn2, locals.var_fac1__blk929_dn6, locals.var_fac1__blk929_dn7, locals.var_fac1__blk929_dn10, locals.var_fac1__blk929_dn11, locals.var_fac1__blk929_dn12, locals.var_fac1__blk929_dn17,)
    }
};
        locals.var_fac1__blk929 = assign28710_e40505;
        locals.var_fac1__blk929_dn0 = assign28710_e40505_d_n0;
        locals.var_fac1__blk929_dn2 = assign28710_e40505_d_n2;
        locals.var_fac1__blk929_dn6 = assign28710_e40505_d_n6;
        locals.var_fac1__blk929_dn7 = assign28710_e40505_d_n7;
        locals.var_fac1__blk929_dn10 = assign28710_e40505_d_n10;
        locals.var_fac1__blk929_dn11 = assign28710_e40505_d_n11;
        locals.var_fac1__blk929_dn12 = assign28710_e40505_d_n12;
        locals.var_fac1__blk929_dn17 = assign28710_e40505_d_n17;
        locals.var_fac1__blk929_rv = 0.0;

        let (assign28720_e40516, assign28720_e40516_d_n0, assign28720_e40516_d_n2, assign28720_e40516_d_n6, assign28720_e40516_d_n7, assign28720_e40516_d_n10, assign28720_e40516_d_n11, assign28720_e40516_d_n12, assign28720_e40516_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign28720_e40514: f64 = (locals.var_fac1__blk929 * locals.var_fac1__blk929);
        (assign28720_e40514, ((locals.var_fac1__blk929_dn0 * locals.var_fac1__blk929) + (locals.var_fac1__blk929 * locals.var_fac1__blk929_dn0)), ((locals.var_fac1__blk929_dn2 * locals.var_fac1__blk929) + (locals.var_fac1__blk929 * locals.var_fac1__blk929_dn2)), ((locals.var_fac1__blk929_dn6 * locals.var_fac1__blk929) + (locals.var_fac1__blk929 * locals.var_fac1__blk929_dn6)), ((locals.var_fac1__blk929_dn7 * locals.var_fac1__blk929) + (locals.var_fac1__blk929 * locals.var_fac1__blk929_dn7)), ((locals.var_fac1__blk929_dn10 * locals.var_fac1__blk929) + (locals.var_fac1__blk929 * locals.var_fac1__blk929_dn10)), ((locals.var_fac1__blk929_dn11 * locals.var_fac1__blk929) + (locals.var_fac1__blk929 * locals.var_fac1__blk929_dn11)), ((locals.var_fac1__blk929_dn12 * locals.var_fac1__blk929) + (locals.var_fac1__blk929 * locals.var_fac1__blk929_dn12)), ((locals.var_fac1__blk929_dn17 * locals.var_fac1__blk929) + (locals.var_fac1__blk929 * locals.var_fac1__blk929_dn17)),)
    } else {
        (locals.var_fac1p2__blk930, locals.var_fac1p2__blk930_dn0, locals.var_fac1p2__blk930_dn2, locals.var_fac1p2__blk930_dn6, locals.var_fac1p2__blk930_dn7, locals.var_fac1p2__blk930_dn10, locals.var_fac1p2__blk930_dn11, locals.var_fac1p2__blk930_dn12, locals.var_fac1p2__blk930_dn17,)
    }
};
        locals.var_fac1p2__blk930 = assign28720_e40516;
        locals.var_fac1p2__blk930_dn0 = assign28720_e40516_d_n0;
        locals.var_fac1p2__blk930_dn2 = assign28720_e40516_d_n2;
        locals.var_fac1p2__blk930_dn6 = assign28720_e40516_d_n6;
        locals.var_fac1p2__blk930_dn7 = assign28720_e40516_d_n7;
        locals.var_fac1p2__blk930_dn10 = assign28720_e40516_d_n10;
        locals.var_fac1p2__blk930_dn11 = assign28720_e40516_d_n11;
        locals.var_fac1p2__blk930_dn12 = assign28720_e40516_d_n12;
        locals.var_fac1p2__blk930_dn17 = assign28720_e40516_d_n17;
        locals.var_fac1p2__blk930_rv = 0.0;

        let (assign28730_e40528, assign28730_e40528_d_n0, assign28730_e40528_d_n2, assign28730_e40528_d_n6, assign28730_e40528_d_n7, assign28730_e40528_d_n10, assign28730_e40528_d_n11, assign28730_e40528_d_n12, assign28730_e40528_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign28730_e40524: f64 = (-locals.var_vgbgmt__blk927);
        let assign28730_e40526: f64 = (assign28730_e40524 + locals.var_uc_vfbover);
        (assign28730_e40526, (-locals.var_vgbgmt__blk927_dn0), (-locals.var_vgbgmt__blk927_dn2), (-locals.var_vgbgmt__blk927_dn6), (-locals.var_vgbgmt__blk927_dn7), (-locals.var_vgbgmt__blk927_dn10), (-locals.var_vgbgmt__blk927_dn11), (-locals.var_vgbgmt__blk927_dn12), (-locals.var_vgbgmt__blk927_dn17),)
    } else {
        (locals.var_vgpld__blk931, locals.var_vgpld__blk931_dn0, locals.var_vgpld__blk931_dn2, locals.var_vgpld__blk931_dn6, locals.var_vgpld__blk931_dn7, locals.var_vgpld__blk931_dn10, locals.var_vgpld__blk931_dn11, locals.var_vgpld__blk931_dn12, locals.var_vgpld__blk931_dn17,)
    }
};
        locals.var_vgpld__blk931 = assign28730_e40528;
        locals.var_vgpld__blk931_dn0 = assign28730_e40528_d_n0;
        locals.var_vgpld__blk931_dn2 = assign28730_e40528_d_n2;
        locals.var_vgpld__blk931_dn6 = assign28730_e40528_d_n6;
        locals.var_vgpld__blk931_dn7 = assign28730_e40528_d_n7;
        locals.var_vgpld__blk931_dn10 = assign28730_e40528_d_n10;
        locals.var_vgpld__blk931_dn11 = assign28730_e40528_d_n11;
        locals.var_vgpld__blk931_dn12 = assign28730_e40528_d_n12;
        locals.var_vgpld__blk931_dn17 = assign28730_e40528_d_n17;
        locals.var_vgpld__blk931_rv = 0.0;

        let (assign28740_e40539, assign28740_e40539_d_n0, assign28740_e40539_d_n2, assign28740_e40539_d_n6, assign28740_e40539_d_n7, assign28740_e40539_d_n10, assign28740_e40539_d_n11, assign28740_e40539_d_n12, assign28740_e40539_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign28740_e40537: f64 = (locals.var_mks_nover / locals.var_nin);
        (assign28740_e40537, (-((locals.var_mks_nover * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn12) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn17) / (locals.var_nin * locals.var_nin))),)
    } else {
        (locals.var_t0__blk895, locals.var_t0__blk895_dn0, locals.var_t0__blk895_dn2, locals.var_t0__blk895_dn6, locals.var_t0__blk895_dn7, locals.var_t0__blk895_dn10, locals.var_t0__blk895_dn11, locals.var_t0__blk895_dn12, locals.var_t0__blk895_dn17,)
    }
};
        locals.var_t0__blk895 = assign28740_e40539;
        locals.var_t0__blk895_dn0 = assign28740_e40539_d_n0;
        locals.var_t0__blk895_dn2 = assign28740_e40539_d_n2;
        locals.var_t0__blk895_dn6 = assign28740_e40539_d_n6;
        locals.var_t0__blk895_dn7 = assign28740_e40539_d_n7;
        locals.var_t0__blk895_dn10 = assign28740_e40539_d_n10;
        locals.var_t0__blk895_dn11 = assign28740_e40539_d_n11;
        locals.var_t0__blk895_dn12 = assign28740_e40539_d_n12;
        locals.var_t0__blk895_dn17 = assign28740_e40539_d_n17;
        locals.var_t0__blk895_rv = 0.0;

        let (assign28750_e40553, assign28750_e40553_d_n0, assign28750_e40553_d_n2, assign28750_e40553_d_n6, assign28750_e40553_d_n7, assign28750_e40553_d_n10, assign28750_e40553_d_n11, assign28750_e40553_d_n12, assign28750_e40553_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign28750_e40548: f64 = (2.0 / locals.var_beta);
        let assign28750_e40550: f64 = (locals.var_t0__blk895).ln();
        let assign28750_e40551: f64 = (assign28750_e40548 * assign28750_e40550);
        (assign28750_e40551, (assign28750_e40548 * (locals.var_t0__blk895_dn0 / locals.var_t0__blk895)), (assign28750_e40548 * (locals.var_t0__blk895_dn2 / locals.var_t0__blk895)), (assign28750_e40548 * (locals.var_t0__blk895_dn6 / locals.var_t0__blk895)), (assign28750_e40548 * (locals.var_t0__blk895_dn7 / locals.var_t0__blk895)), (((-((2.0 * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign28750_e40550) + (assign28750_e40548 * (locals.var_t0__blk895_dn10 / locals.var_t0__blk895))), (assign28750_e40548 * (locals.var_t0__blk895_dn11 / locals.var_t0__blk895)), (assign28750_e40548 * (locals.var_t0__blk895_dn12 / locals.var_t0__blk895)), (assign28750_e40548 * (locals.var_t0__blk895_dn17 / locals.var_t0__blk895)),)
    } else {
        (locals.var_pb2over__blk932, locals.var_pb2over__blk932_dn0, locals.var_pb2over__blk932_dn2, locals.var_pb2over__blk932_dn6, locals.var_pb2over__blk932_dn7, locals.var_pb2over__blk932_dn10, locals.var_pb2over__blk932_dn11, locals.var_pb2over__blk932_dn12, locals.var_pb2over__blk932_dn17,)
    }
};
        locals.var_pb2over__blk932 = assign28750_e40553;
        locals.var_pb2over__blk932_dn0 = assign28750_e40553_d_n0;
        locals.var_pb2over__blk932_dn2 = assign28750_e40553_d_n2;
        locals.var_pb2over__blk932_dn6 = assign28750_e40553_d_n6;
        locals.var_pb2over__blk932_dn7 = assign28750_e40553_d_n7;
        locals.var_pb2over__blk932_dn10 = assign28750_e40553_d_n10;
        locals.var_pb2over__blk932_dn11 = assign28750_e40553_d_n11;
        locals.var_pb2over__blk932_dn12 = assign28750_e40553_d_n12;
        locals.var_pb2over__blk932_dn17 = assign28750_e40553_d_n17;
        locals.var_pb2over__blk932_rv = 0.0;

        let (assign28760_e40563, assign28760_e40563_d_n0, assign28760_e40563_d_n2, assign28760_e40563_d_n6, assign28760_e40563_d_n7, assign28760_e40563_d_n10, assign28760_e40563_d_n11, assign28760_e40563_d_n12, assign28760_e40563_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign28760_e40561: f64 = (-locals.var_vxbgmtcl__blk921);
        (assign28760_e40561, (-locals.var_vxbgmtcl__blk921_dn0), (-locals.var_vxbgmtcl__blk921_dn2), (-locals.var_vxbgmtcl__blk921_dn6), (-locals.var_vxbgmtcl__blk921_dn7), (-locals.var_vxbgmtcl__blk921_dn10), (-locals.var_vxbgmtcl__blk921_dn11), (-locals.var_vxbgmtcl__blk921_dn12), (-locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_vgb_fb_ld__blk933, locals.var_vgb_fb_ld__blk933_dn0, locals.var_vgb_fb_ld__blk933_dn2, locals.var_vgb_fb_ld__blk933_dn6, locals.var_vgb_fb_ld__blk933_dn7, locals.var_vgb_fb_ld__blk933_dn10, locals.var_vgb_fb_ld__blk933_dn11, locals.var_vgb_fb_ld__blk933_dn12, locals.var_vgb_fb_ld__blk933_dn17,)
    }
};
        locals.var_vgb_fb_ld__blk933 = assign28760_e40563;
        locals.var_vgb_fb_ld__blk933_dn0 = assign28760_e40563_d_n0;
        locals.var_vgb_fb_ld__blk933_dn2 = assign28760_e40563_d_n2;
        locals.var_vgb_fb_ld__blk933_dn6 = assign28760_e40563_d_n6;
        locals.var_vgb_fb_ld__blk933_dn7 = assign28760_e40563_d_n7;
        locals.var_vgb_fb_ld__blk933_dn10 = assign28760_e40563_d_n10;
        locals.var_vgb_fb_ld__blk933_dn11 = assign28760_e40563_d_n11;
        locals.var_vgb_fb_ld__blk933_dn12 = assign28760_e40563_d_n12;
        locals.var_vgb_fb_ld__blk933_dn17 = assign28760_e40563_d_n17;
        locals.var_vgb_fb_ld__blk933_rv = 0.0;

        let assign28770_e40566: f64 = if locals.var_vgpld__blk931 < locals.var_vgb_fb_ld__blk933 { 1.0 } else { 0.0 };
        locals.var_guard982 = assign28770_e40566;
        locals.var_guard982_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_104(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign28790_e40593, assign28790_e40593_d_n0, assign28790_e40593_d_n2, assign28790_e40593_d_n6, assign28790_e40593_d_n7, assign28790_e40593_d_n10, assign28790_e40593_d_n11, assign28790_e40593_d_n12, assign28790_e40593_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28790_e40590: f64 = (locals.var_beta * locals.var_cnst0over__blk928);
        let assign28790_e40591: f64 = (1.0 / assign28790_e40590);
        (assign28790_e40591, (-((locals.var_beta * locals.var_cnst0over__blk928_dn0) / (assign28790_e40590 * assign28790_e40590))), (-((locals.var_beta * locals.var_cnst0over__blk928_dn2) / (assign28790_e40590 * assign28790_e40590))), (-((locals.var_beta * locals.var_cnst0over__blk928_dn6) / (assign28790_e40590 * assign28790_e40590))), (-((locals.var_beta * locals.var_cnst0over__blk928_dn7) / (assign28790_e40590 * assign28790_e40590))), (-(((locals.var_beta_dn10 * locals.var_cnst0over__blk928) + (locals.var_beta * locals.var_cnst0over__blk928_dn10)) / (assign28790_e40590 * assign28790_e40590))), (-((locals.var_beta * locals.var_cnst0over__blk928_dn11) / (assign28790_e40590 * assign28790_e40590))), (-((locals.var_beta * locals.var_cnst0over__blk928_dn12) / (assign28790_e40590 * assign28790_e40590))), (-((locals.var_beta * locals.var_cnst0over__blk928_dn17) / (assign28790_e40590 * assign28790_e40590))),)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign28790_e40593;
        locals.var_t1__blk896_dn0 = assign28790_e40593_d_n0;
        locals.var_t1__blk896_dn2 = assign28790_e40593_d_n2;
        locals.var_t1__blk896_dn6 = assign28790_e40593_d_n6;
        locals.var_t1__blk896_dn7 = assign28790_e40593_d_n7;
        locals.var_t1__blk896_dn10 = assign28790_e40593_d_n10;
        locals.var_t1__blk896_dn11 = assign28790_e40593_d_n11;
        locals.var_t1__blk896_dn12 = assign28790_e40593_d_n12;
        locals.var_t1__blk896_dn17 = assign28790_e40593_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let (assign28800_e40606, assign28800_e40606_d_n0, assign28800_e40606_d_n2, assign28800_e40606_d_n6, assign28800_e40606_d_n7, assign28800_e40606_d_n10, assign28800_e40606_d_n11, assign28800_e40606_d_n12, assign28800_e40606_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28800_e40604: f64 = (locals.var_t1__blk896 * locals.var_cox0__blk906);
        (assign28800_e40604, (locals.var_t1__blk896_dn0 * locals.var_cox0__blk906), (locals.var_t1__blk896_dn2 * locals.var_cox0__blk906), (locals.var_t1__blk896_dn6 * locals.var_cox0__blk906), (locals.var_t1__blk896_dn7 * locals.var_cox0__blk906), (locals.var_t1__blk896_dn10 * locals.var_cox0__blk906), (locals.var_t1__blk896_dn11 * locals.var_cox0__blk906), (locals.var_t1__blk896_dn12 * locals.var_cox0__blk906), (locals.var_t1__blk896_dn17 * locals.var_cox0__blk906),)
    } else {
        (locals.var_ty__blk905, locals.var_ty__blk905_dn0, locals.var_ty__blk905_dn2, locals.var_ty__blk905_dn6, locals.var_ty__blk905_dn7, locals.var_ty__blk905_dn10, locals.var_ty__blk905_dn11, locals.var_ty__blk905_dn12, locals.var_ty__blk905_dn17,)
    }
};
        locals.var_ty__blk905 = assign28800_e40606;
        locals.var_ty__blk905_dn0 = assign28800_e40606_d_n0;
        locals.var_ty__blk905_dn2 = assign28800_e40606_d_n2;
        locals.var_ty__blk905_dn6 = assign28800_e40606_d_n6;
        locals.var_ty__blk905_dn7 = assign28800_e40606_d_n7;
        locals.var_ty__blk905_dn10 = assign28800_e40606_d_n10;
        locals.var_ty__blk905_dn11 = assign28800_e40606_d_n11;
        locals.var_ty__blk905_dn12 = assign28800_e40606_d_n12;
        locals.var_ty__blk905_dn17 = assign28800_e40606_d_n17;
        locals.var_ty__blk905_rv = 0.0;

        let (assign28810_e40623, assign28810_e40623_d_n0, assign28810_e40623_d_n2, assign28810_e40623_d_n6, assign28810_e40623_d_n7, assign28810_e40623_d_n10, assign28810_e40623_d_n11, assign28810_e40623_d_n12, assign28810_e40623_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28810_e40618: f64 = (3.0 * 1.414213562373095);
        let assign28810_e40620: f64 = (assign28810_e40618 * locals.var_ty__blk905);
        let assign28810_e40621: f64 = (2.0 + assign28810_e40620);
        (assign28810_e40621, (assign28810_e40618 * locals.var_ty__blk905_dn0), (assign28810_e40618 * locals.var_ty__blk905_dn2), (assign28810_e40618 * locals.var_ty__blk905_dn6), (assign28810_e40618 * locals.var_ty__blk905_dn7), (assign28810_e40618 * locals.var_ty__blk905_dn10), (assign28810_e40618 * locals.var_ty__blk905_dn11), (assign28810_e40618 * locals.var_ty__blk905_dn12), (assign28810_e40618 * locals.var_ty__blk905_dn17),)
    } else {
        (locals.var_ac41__blk934, locals.var_ac41__blk934_dn0, locals.var_ac41__blk934_dn2, locals.var_ac41__blk934_dn6, locals.var_ac41__blk934_dn7, locals.var_ac41__blk934_dn10, locals.var_ac41__blk934_dn11, locals.var_ac41__blk934_dn12, locals.var_ac41__blk934_dn17,)
    }
};
        locals.var_ac41__blk934 = assign28810_e40623;
        locals.var_ac41__blk934_dn0 = assign28810_e40623_d_n0;
        locals.var_ac41__blk934_dn2 = assign28810_e40623_d_n2;
        locals.var_ac41__blk934_dn6 = assign28810_e40623_d_n6;
        locals.var_ac41__blk934_dn7 = assign28810_e40623_d_n7;
        locals.var_ac41__blk934_dn10 = assign28810_e40623_d_n10;
        locals.var_ac41__blk934_dn11 = assign28810_e40623_d_n11;
        locals.var_ac41__blk934_dn12 = assign28810_e40623_d_n12;
        locals.var_ac41__blk934_dn17 = assign28810_e40623_d_n17;
        locals.var_ac41__blk934_rv = 0.0;

        let (assign28820_e40640, assign28820_e40640_d_n0, assign28820_e40640_d_n2, assign28820_e40640_d_n6, assign28820_e40640_d_n7, assign28820_e40640_d_n10, assign28820_e40640_d_n11, assign28820_e40640_d_n12, assign28820_e40640_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28820_e40634: f64 = (8.0 * locals.var_ac41__blk934);
        let assign28820_e40636: f64 = (assign28820_e40634 * locals.var_ac41__blk934);
        let assign28820_e40638: f64 = (assign28820_e40636 * locals.var_ac41__blk934);
        (assign28820_e40638, (((((8.0 * locals.var_ac41__blk934_dn0) * locals.var_ac41__blk934) + (assign28820_e40634 * locals.var_ac41__blk934_dn0)) * locals.var_ac41__blk934) + (assign28820_e40636 * locals.var_ac41__blk934_dn0)), (((((8.0 * locals.var_ac41__blk934_dn2) * locals.var_ac41__blk934) + (assign28820_e40634 * locals.var_ac41__blk934_dn2)) * locals.var_ac41__blk934) + (assign28820_e40636 * locals.var_ac41__blk934_dn2)), (((((8.0 * locals.var_ac41__blk934_dn6) * locals.var_ac41__blk934) + (assign28820_e40634 * locals.var_ac41__blk934_dn6)) * locals.var_ac41__blk934) + (assign28820_e40636 * locals.var_ac41__blk934_dn6)), (((((8.0 * locals.var_ac41__blk934_dn7) * locals.var_ac41__blk934) + (assign28820_e40634 * locals.var_ac41__blk934_dn7)) * locals.var_ac41__blk934) + (assign28820_e40636 * locals.var_ac41__blk934_dn7)), (((((8.0 * locals.var_ac41__blk934_dn10) * locals.var_ac41__blk934) + (assign28820_e40634 * locals.var_ac41__blk934_dn10)) * locals.var_ac41__blk934) + (assign28820_e40636 * locals.var_ac41__blk934_dn10)), (((((8.0 * locals.var_ac41__blk934_dn11) * locals.var_ac41__blk934) + (assign28820_e40634 * locals.var_ac41__blk934_dn11)) * locals.var_ac41__blk934) + (assign28820_e40636 * locals.var_ac41__blk934_dn11)), (((((8.0 * locals.var_ac41__blk934_dn12) * locals.var_ac41__blk934) + (assign28820_e40634 * locals.var_ac41__blk934_dn12)) * locals.var_ac41__blk934) + (assign28820_e40636 * locals.var_ac41__blk934_dn12)), (((((8.0 * locals.var_ac41__blk934_dn17) * locals.var_ac41__blk934) + (assign28820_e40634 * locals.var_ac41__blk934_dn17)) * locals.var_ac41__blk934) + (assign28820_e40636 * locals.var_ac41__blk934_dn17)),)
    } else {
        (locals.var_ac4__blk935, locals.var_ac4__blk935_dn0, locals.var_ac4__blk935_dn2, locals.var_ac4__blk935_dn6, locals.var_ac4__blk935_dn7, locals.var_ac4__blk935_dn10, locals.var_ac4__blk935_dn11, locals.var_ac4__blk935_dn12, locals.var_ac4__blk935_dn17,)
    }
};
        locals.var_ac4__blk935 = assign28820_e40640;
        locals.var_ac4__blk935_dn0 = assign28820_e40640_d_n0;
        locals.var_ac4__blk935_dn2 = assign28820_e40640_d_n2;
        locals.var_ac4__blk935_dn6 = assign28820_e40640_d_n6;
        locals.var_ac4__blk935_dn7 = assign28820_e40640_d_n7;
        locals.var_ac4__blk935_dn10 = assign28820_e40640_d_n10;
        locals.var_ac4__blk935_dn11 = assign28820_e40640_d_n11;
        locals.var_ac4__blk935_dn12 = assign28820_e40640_d_n12;
        locals.var_ac4__blk935_dn17 = assign28820_e40640_d_n17;
        locals.var_ac4__blk935_rv = 0.0;

        let (assign28830_e40653, assign28830_e40653_d_n0, assign28830_e40653_d_n2, assign28830_e40653_d_n6, assign28830_e40653_d_n7, assign28830_e40653_d_n10, assign28830_e40653_d_n11, assign28830_e40653_d_n12, assign28830_e40653_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28830_e40651: f64 = (locals.var_eg - locals.var_pb2over__blk932);
        (assign28830_e40651, (locals.var_eg_dn0 - locals.var_pb2over__blk932_dn0), (locals.var_eg_dn2 - locals.var_pb2over__blk932_dn2), (locals.var_eg_dn6 - locals.var_pb2over__blk932_dn6), (locals.var_eg_dn7 - locals.var_pb2over__blk932_dn7), (locals.var_eg_dn10 - locals.var_pb2over__blk932_dn10), (locals.var_eg_dn11 - locals.var_pb2over__blk932_dn11), (locals.var_eg_dn12 - locals.var_pb2over__blk932_dn12), (locals.var_eg_dn17 - locals.var_pb2over__blk932_dn17),)
    } else {
        (locals.var_ps0_min__blk936, locals.var_ps0_min__blk936_dn0, locals.var_ps0_min__blk936_dn2, locals.var_ps0_min__blk936_dn6, locals.var_ps0_min__blk936_dn7, locals.var_ps0_min__blk936_dn10, locals.var_ps0_min__blk936_dn11, locals.var_ps0_min__blk936_dn12, locals.var_ps0_min__blk936_dn17,)
    }
};
        locals.var_ps0_min__blk936 = assign28830_e40653;
        locals.var_ps0_min__blk936_dn0 = assign28830_e40653_d_n0;
        locals.var_ps0_min__blk936_dn2 = assign28830_e40653_d_n2;
        locals.var_ps0_min__blk936_dn6 = assign28830_e40653_d_n6;
        locals.var_ps0_min__blk936_dn7 = assign28830_e40653_d_n7;
        locals.var_ps0_min__blk936_dn10 = assign28830_e40653_d_n10;
        locals.var_ps0_min__blk936_dn11 = assign28830_e40653_d_n11;
        locals.var_ps0_min__blk936_dn12 = assign28830_e40653_d_n12;
        locals.var_ps0_min__blk936_dn17 = assign28830_e40653_d_n17;
        locals.var_ps0_min__blk936_rv = 0.0;

        let (assign28840_e40668, assign28840_e40668_d_n0, assign28840_e40668_d_n2, assign28840_e40668_d_n6, assign28840_e40668_d_n7, assign28840_e40668_d_n10, assign28840_e40668_d_n11, assign28840_e40668_d_n12, assign28840_e40668_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28840_e40665: f64 = (locals.var_vgpld__blk931 + locals.var_vxbgmtcl__blk921);
        let assign28840_e40666: f64 = (locals.var_beta * assign28840_e40665);
        (assign28840_e40666, (locals.var_beta * (locals.var_vgpld__blk931_dn0 + locals.var_vxbgmtcl__blk921_dn0)), (locals.var_beta * (locals.var_vgpld__blk931_dn2 + locals.var_vxbgmtcl__blk921_dn2)), (locals.var_beta * (locals.var_vgpld__blk931_dn6 + locals.var_vxbgmtcl__blk921_dn6)), (locals.var_beta * (locals.var_vgpld__blk931_dn7 + locals.var_vxbgmtcl__blk921_dn7)), ((locals.var_beta_dn10 * assign28840_e40665) + (locals.var_beta * (locals.var_vgpld__blk931_dn10 + locals.var_vxbgmtcl__blk921_dn10))), (locals.var_beta * (locals.var_vgpld__blk931_dn11 + locals.var_vxbgmtcl__blk921_dn11)), (locals.var_beta * (locals.var_vgpld__blk931_dn12 + locals.var_vxbgmtcl__blk921_dn12)), (locals.var_beta * (locals.var_vgpld__blk931_dn17 + locals.var_vxbgmtcl__blk921_dn17)),)
    } else {
        (locals.var_tx__blk904, locals.var_tx__blk904_dn0, locals.var_tx__blk904_dn2, locals.var_tx__blk904_dn6, locals.var_tx__blk904_dn7, locals.var_tx__blk904_dn10, locals.var_tx__blk904_dn11, locals.var_tx__blk904_dn12, locals.var_tx__blk904_dn17,)
    }
};
        locals.var_tx__blk904 = assign28840_e40668;
        locals.var_tx__blk904_dn0 = assign28840_e40668_d_n0;
        locals.var_tx__blk904_dn2 = assign28840_e40668_d_n2;
        locals.var_tx__blk904_dn6 = assign28840_e40668_d_n6;
        locals.var_tx__blk904_dn7 = assign28840_e40668_d_n7;
        locals.var_tx__blk904_dn10 = assign28840_e40668_d_n10;
        locals.var_tx__blk904_dn11 = assign28840_e40668_d_n11;
        locals.var_tx__blk904_dn12 = assign28840_e40668_d_n12;
        locals.var_tx__blk904_dn17 = assign28840_e40668_d_n17;
        locals.var_tx__blk904_rv = 0.0;

        let (assign28850_e40689, assign28850_e40689_d_n0, assign28850_e40689_d_n2, assign28850_e40689_d_n6, assign28850_e40689_d_n7, assign28850_e40689_d_n10, assign28850_e40689_d_n11, assign28850_e40689_d_n12, assign28850_e40689_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28850_e40679: f64 = (7.0 * 1.414213562373095);
        let assign28850_e40682: f64 = (9.0 * locals.var_ty__blk905);
        let assign28850_e40685: f64 = (locals.var_tx__blk904 - 2.0);
        let assign28850_e40686: f64 = (assign28850_e40682 * assign28850_e40685);
        let assign28850_e40687: f64 = (assign28850_e40679 - assign28850_e40686);
        (assign28850_e40687, (-(((9.0 * locals.var_ty__blk905_dn0) * assign28850_e40685) + (assign28850_e40682 * locals.var_tx__blk904_dn0))), (-(((9.0 * locals.var_ty__blk905_dn2) * assign28850_e40685) + (assign28850_e40682 * locals.var_tx__blk904_dn2))), (-(((9.0 * locals.var_ty__blk905_dn6) * assign28850_e40685) + (assign28850_e40682 * locals.var_tx__blk904_dn6))), (-(((9.0 * locals.var_ty__blk905_dn7) * assign28850_e40685) + (assign28850_e40682 * locals.var_tx__blk904_dn7))), (-(((9.0 * locals.var_ty__blk905_dn10) * assign28850_e40685) + (assign28850_e40682 * locals.var_tx__blk904_dn10))), (-(((9.0 * locals.var_ty__blk905_dn11) * assign28850_e40685) + (assign28850_e40682 * locals.var_tx__blk904_dn11))), (-(((9.0 * locals.var_ty__blk905_dn12) * assign28850_e40685) + (assign28850_e40682 * locals.var_tx__blk904_dn12))), (-(((9.0 * locals.var_ty__blk905_dn17) * assign28850_e40685) + (assign28850_e40682 * locals.var_tx__blk904_dn17))),)
    } else {
        (locals.var_ac31__blk937, locals.var_ac31__blk937_dn0, locals.var_ac31__blk937_dn2, locals.var_ac31__blk937_dn6, locals.var_ac31__blk937_dn7, locals.var_ac31__blk937_dn10, locals.var_ac31__blk937_dn11, locals.var_ac31__blk937_dn12, locals.var_ac31__blk937_dn17,)
    }
};
        locals.var_ac31__blk937 = assign28850_e40689;
        locals.var_ac31__blk937_dn0 = assign28850_e40689_d_n0;
        locals.var_ac31__blk937_dn2 = assign28850_e40689_d_n2;
        locals.var_ac31__blk937_dn6 = assign28850_e40689_d_n6;
        locals.var_ac31__blk937_dn7 = assign28850_e40689_d_n7;
        locals.var_ac31__blk937_dn10 = assign28850_e40689_d_n10;
        locals.var_ac31__blk937_dn11 = assign28850_e40689_d_n11;
        locals.var_ac31__blk937_dn12 = assign28850_e40689_d_n12;
        locals.var_ac31__blk937_dn17 = assign28850_e40689_d_n17;
        locals.var_ac31__blk937_rv = 0.0;

        let (assign28860_e40702, assign28860_e40702_d_n0, assign28860_e40702_d_n2, assign28860_e40702_d_n6, assign28860_e40702_d_n7, assign28860_e40702_d_n10, assign28860_e40702_d_n11, assign28860_e40702_d_n12, assign28860_e40702_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28860_e40700: f64 = (locals.var_ac31__blk937 * locals.var_ac31__blk937);
        (assign28860_e40700, ((locals.var_ac31__blk937_dn0 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn0)), ((locals.var_ac31__blk937_dn2 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn2)), ((locals.var_ac31__blk937_dn6 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn6)), ((locals.var_ac31__blk937_dn7 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn7)), ((locals.var_ac31__blk937_dn10 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn10)), ((locals.var_ac31__blk937_dn11 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn11)), ((locals.var_ac31__blk937_dn12 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn12)), ((locals.var_ac31__blk937_dn17 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn17)),)
    } else {
        (locals.var_ac3__blk938, locals.var_ac3__blk938_dn0, locals.var_ac3__blk938_dn2, locals.var_ac3__blk938_dn6, locals.var_ac3__blk938_dn7, locals.var_ac3__blk938_dn10, locals.var_ac3__blk938_dn11, locals.var_ac3__blk938_dn12, locals.var_ac3__blk938_dn17,)
    }
};
        locals.var_ac3__blk938 = assign28860_e40702;
        locals.var_ac3__blk938_dn0 = assign28860_e40702_d_n0;
        locals.var_ac3__blk938_dn2 = assign28860_e40702_d_n2;
        locals.var_ac3__blk938_dn6 = assign28860_e40702_d_n6;
        locals.var_ac3__blk938_dn7 = assign28860_e40702_d_n7;
        locals.var_ac3__blk938_dn10 = assign28860_e40702_d_n10;
        locals.var_ac3__blk938_dn11 = assign28860_e40702_d_n11;
        locals.var_ac3__blk938_dn12 = assign28860_e40702_d_n12;
        locals.var_ac3__blk938_dn17 = assign28860_e40702_d_n17;
        locals.var_ac3__blk938_rv = 0.0;

        let assign28870_e40706: f64 = (locals.var_ac3__blk938 * 1e-8);
        let assign28870_e40707: f64 = if locals.var_ac4__blk935 < assign28870_e40706 { 1.0 } else { 0.0 };
        locals.var_guard983 = assign28870_e40707;
        locals.var_guard983_rv = 0.0;

        let (assign28880_e40739, assign28880_e40739_d_n0, assign28880_e40739_d_n2, assign28880_e40739_d_n6, assign28880_e40739_d_n7, assign28880_e40739_d_n10, assign28880_e40739_d_n11, assign28880_e40739_d_n12, assign28880_e40739_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 != 0.0)) {
        let assign28880_e40719: f64 = (-7.0);
        let assign28880_e40721: f64 = (assign28880_e40719 * 1.414213562373095);
        let assign28880_e40723: f64 = (assign28880_e40721 + locals.var_ac31__blk937);
        let assign28880_e40726: f64 = (0.5 * locals.var_ac4__blk935);
        let assign28880_e40728: f64 = (assign28880_e40726 / locals.var_ac31__blk937);
        let assign28880_e40729: f64 = (assign28880_e40723 + assign28880_e40728);
        let assign28880_e40732: f64 = (9.0 * locals.var_ty__blk905);
        let assign28880_e40735: f64 = (locals.var_tx__blk904 - 2.0);
        let assign28880_e40736: f64 = (assign28880_e40732 * assign28880_e40735);
        let assign28880_e40737: f64 = (assign28880_e40729 + assign28880_e40736);
        (assign28880_e40737, ((locals.var_ac31__blk937_dn0 + ((((0.5 * locals.var_ac4__blk935_dn0) * locals.var_ac31__blk937) - (assign28880_e40726 * locals.var_ac31__blk937_dn0)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn0) * assign28880_e40735) + (assign28880_e40732 * locals.var_tx__blk904_dn0))), ((locals.var_ac31__blk937_dn2 + ((((0.5 * locals.var_ac4__blk935_dn2) * locals.var_ac31__blk937) - (assign28880_e40726 * locals.var_ac31__blk937_dn2)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn2) * assign28880_e40735) + (assign28880_e40732 * locals.var_tx__blk904_dn2))), ((locals.var_ac31__blk937_dn6 + ((((0.5 * locals.var_ac4__blk935_dn6) * locals.var_ac31__blk937) - (assign28880_e40726 * locals.var_ac31__blk937_dn6)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn6) * assign28880_e40735) + (assign28880_e40732 * locals.var_tx__blk904_dn6))), ((locals.var_ac31__blk937_dn7 + ((((0.5 * locals.var_ac4__blk935_dn7) * locals.var_ac31__blk937) - (assign28880_e40726 * locals.var_ac31__blk937_dn7)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn7) * assign28880_e40735) + (assign28880_e40732 * locals.var_tx__blk904_dn7))), ((locals.var_ac31__blk937_dn10 + ((((0.5 * locals.var_ac4__blk935_dn10) * locals.var_ac31__blk937) - (assign28880_e40726 * locals.var_ac31__blk937_dn10)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn10) * assign28880_e40735) + (assign28880_e40732 * locals.var_tx__blk904_dn10))), ((locals.var_ac31__blk937_dn11 + ((((0.5 * locals.var_ac4__blk935_dn11) * locals.var_ac31__blk937) - (assign28880_e40726 * locals.var_ac31__blk937_dn11)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn11) * assign28880_e40735) + (assign28880_e40732 * locals.var_tx__blk904_dn11))), ((locals.var_ac31__blk937_dn12 + ((((0.5 * locals.var_ac4__blk935_dn12) * locals.var_ac31__blk937) - (assign28880_e40726 * locals.var_ac31__blk937_dn12)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn12) * assign28880_e40735) + (assign28880_e40732 * locals.var_tx__blk904_dn12))), ((locals.var_ac31__blk937_dn17 + ((((0.5 * locals.var_ac4__blk935_dn17) * locals.var_ac31__blk937) - (assign28880_e40726 * locals.var_ac31__blk937_dn17)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn17) * assign28880_e40735) + (assign28880_e40732 * locals.var_tx__blk904_dn17))),)
    } else {
        (locals.var_ac1__blk940, locals.var_ac1__blk940_dn0, locals.var_ac1__blk940_dn2, locals.var_ac1__blk940_dn6, locals.var_ac1__blk940_dn7, locals.var_ac1__blk940_dn10, locals.var_ac1__blk940_dn11, locals.var_ac1__blk940_dn12, locals.var_ac1__blk940_dn17,)
    }
};
        locals.var_ac1__blk940 = assign28880_e40739;
        locals.var_ac1__blk940_dn0 = assign28880_e40739_d_n0;
        locals.var_ac1__blk940_dn2 = assign28880_e40739_d_n2;
        locals.var_ac1__blk940_dn6 = assign28880_e40739_d_n6;
        locals.var_ac1__blk940_dn7 = assign28880_e40739_d_n7;
        locals.var_ac1__blk940_dn10 = assign28880_e40739_d_n10;
        locals.var_ac1__blk940_dn11 = assign28880_e40739_d_n11;
        locals.var_ac1__blk940_dn12 = assign28880_e40739_d_n12;
        locals.var_ac1__blk940_dn17 = assign28880_e40739_d_n17;
        locals.var_ac1__blk940_rv = 0.0;

        let (assign28890_e40756, assign28890_e40756_d_n0, assign28890_e40756_d_n2, assign28890_e40756_d_n6, assign28890_e40756_d_n7, assign28890_e40756_d_n10, assign28890_e40756_d_n11, assign28890_e40756_d_n12, assign28890_e40756_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign28890_e40753: f64 = (locals.var_ac4__blk935 + locals.var_ac3__blk938);
        let assign28890_e40754: f64 = (assign28890_e40753).sqrt();
        (assign28890_e40754, ((locals.var_ac4__blk935_dn0 + locals.var_ac3__blk938_dn0) / (2.0 * assign28890_e40754)), ((locals.var_ac4__blk935_dn2 + locals.var_ac3__blk938_dn2) / (2.0 * assign28890_e40754)), ((locals.var_ac4__blk935_dn6 + locals.var_ac3__blk938_dn6) / (2.0 * assign28890_e40754)), ((locals.var_ac4__blk935_dn7 + locals.var_ac3__blk938_dn7) / (2.0 * assign28890_e40754)), ((locals.var_ac4__blk935_dn10 + locals.var_ac3__blk938_dn10) / (2.0 * assign28890_e40754)), ((locals.var_ac4__blk935_dn11 + locals.var_ac3__blk938_dn11) / (2.0 * assign28890_e40754)), ((locals.var_ac4__blk935_dn12 + locals.var_ac3__blk938_dn12) / (2.0 * assign28890_e40754)), ((locals.var_ac4__blk935_dn17 + locals.var_ac3__blk938_dn17) / (2.0 * assign28890_e40754)),)
    } else {
        (locals.var_ac2__blk939, locals.var_ac2__blk939_dn0, locals.var_ac2__blk939_dn2, locals.var_ac2__blk939_dn6, locals.var_ac2__blk939_dn7, locals.var_ac2__blk939_dn10, locals.var_ac2__blk939_dn11, locals.var_ac2__blk939_dn12, locals.var_ac2__blk939_dn17,)
    }
};
        locals.var_ac2__blk939 = assign28890_e40756;
        locals.var_ac2__blk939_dn0 = assign28890_e40756_d_n0;
        locals.var_ac2__blk939_dn2 = assign28890_e40756_d_n2;
        locals.var_ac2__blk939_dn6 = assign28890_e40756_d_n6;
        locals.var_ac2__blk939_dn7 = assign28890_e40756_d_n7;
        locals.var_ac2__blk939_dn10 = assign28890_e40756_d_n10;
        locals.var_ac2__blk939_dn11 = assign28890_e40756_d_n11;
        locals.var_ac2__blk939_dn12 = assign28890_e40756_d_n12;
        locals.var_ac2__blk939_dn17 = assign28890_e40756_d_n17;
        locals.var_ac2__blk939_rv = 0.0;

        let (assign28900_e40783, assign28900_e40783_d_n0, assign28900_e40783_d_n2, assign28900_e40783_d_n6, assign28900_e40783_d_n7, assign28900_e40783_d_n10, assign28900_e40783_d_n11, assign28900_e40783_d_n12, assign28900_e40783_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) && (locals.var_guard983 == 0.0)) {
        let assign28900_e40769: f64 = (-7.0);
        let assign28900_e40771: f64 = (assign28900_e40769 * 1.414213562373095);
        let assign28900_e40773: f64 = (assign28900_e40771 + locals.var_ac2__blk939);
        let assign28900_e40776: f64 = (9.0 * locals.var_ty__blk905);
        let assign28900_e40779: f64 = (locals.var_tx__blk904 - 2.0);
        let assign28900_e40780: f64 = (assign28900_e40776 * assign28900_e40779);
        let assign28900_e40781: f64 = (assign28900_e40773 + assign28900_e40780);
        (assign28900_e40781, (locals.var_ac2__blk939_dn0 + (((9.0 * locals.var_ty__blk905_dn0) * assign28900_e40779) + (assign28900_e40776 * locals.var_tx__blk904_dn0))), (locals.var_ac2__blk939_dn2 + (((9.0 * locals.var_ty__blk905_dn2) * assign28900_e40779) + (assign28900_e40776 * locals.var_tx__blk904_dn2))), (locals.var_ac2__blk939_dn6 + (((9.0 * locals.var_ty__blk905_dn6) * assign28900_e40779) + (assign28900_e40776 * locals.var_tx__blk904_dn6))), (locals.var_ac2__blk939_dn7 + (((9.0 * locals.var_ty__blk905_dn7) * assign28900_e40779) + (assign28900_e40776 * locals.var_tx__blk904_dn7))), (locals.var_ac2__blk939_dn10 + (((9.0 * locals.var_ty__blk905_dn10) * assign28900_e40779) + (assign28900_e40776 * locals.var_tx__blk904_dn10))), (locals.var_ac2__blk939_dn11 + (((9.0 * locals.var_ty__blk905_dn11) * assign28900_e40779) + (assign28900_e40776 * locals.var_tx__blk904_dn11))), (locals.var_ac2__blk939_dn12 + (((9.0 * locals.var_ty__blk905_dn12) * assign28900_e40779) + (assign28900_e40776 * locals.var_tx__blk904_dn12))), (locals.var_ac2__blk939_dn17 + (((9.0 * locals.var_ty__blk905_dn17) * assign28900_e40779) + (assign28900_e40776 * locals.var_tx__blk904_dn17))),)
    } else {
        (locals.var_ac1__blk940, locals.var_ac1__blk940_dn0, locals.var_ac1__blk940_dn2, locals.var_ac1__blk940_dn6, locals.var_ac1__blk940_dn7, locals.var_ac1__blk940_dn10, locals.var_ac1__blk940_dn11, locals.var_ac1__blk940_dn12, locals.var_ac1__blk940_dn17,)
    }
};
        locals.var_ac1__blk940 = assign28900_e40783;
        locals.var_ac1__blk940_dn0 = assign28900_e40783_d_n0;
        locals.var_ac1__blk940_dn2 = assign28900_e40783_d_n2;
        locals.var_ac1__blk940_dn6 = assign28900_e40783_d_n6;
        locals.var_ac1__blk940_dn7 = assign28900_e40783_d_n7;
        locals.var_ac1__blk940_dn10 = assign28900_e40783_d_n10;
        locals.var_ac1__blk940_dn11 = assign28900_e40783_d_n11;
        locals.var_ac1__blk940_dn12 = assign28900_e40783_d_n12;
        locals.var_ac1__blk940_dn17 = assign28900_e40783_d_n17;
        locals.var_ac1__blk940_rv = 0.0;

        let (assign28910_e40796, assign28910_e40796_d_n0, assign28910_e40796_d_n2, assign28910_e40796_d_n6, assign28910_e40796_d_n7, assign28910_e40796_d_n10, assign28910_e40796_d_n11, assign28910_e40796_d_n12, assign28910_e40796_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28910_e40794: f64 = (locals.var_ac1__blk940).powf(0.3333333333333333);
        (assign28910_e40794, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn0)) } } else { (assign28910_e40794 * (0.3333333333333333 * (locals.var_ac1__blk940_dn0 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn2)) } } else { (assign28910_e40794 * (0.3333333333333333 * (locals.var_ac1__blk940_dn2 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn6)) } } else { (assign28910_e40794 * (0.3333333333333333 * (locals.var_ac1__blk940_dn6 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn7)) } } else { (assign28910_e40794 * (0.3333333333333333 * (locals.var_ac1__blk940_dn7 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn10)) } } else { (assign28910_e40794 * (0.3333333333333333 * (locals.var_ac1__blk940_dn10 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn11)) } } else { (assign28910_e40794 * (0.3333333333333333 * (locals.var_ac1__blk940_dn11 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn12)) } } else { (assign28910_e40794 * (0.3333333333333333 * (locals.var_ac1__blk940_dn12 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn17)) } } else { (assign28910_e40794 * (0.3333333333333333 * (locals.var_ac1__blk940_dn17 / locals.var_ac1__blk940))) },)
    } else {
        (locals.var_acd__blk941, locals.var_acd__blk941_dn0, locals.var_acd__blk941_dn2, locals.var_acd__blk941_dn6, locals.var_acd__blk941_dn7, locals.var_acd__blk941_dn10, locals.var_acd__blk941_dn11, locals.var_acd__blk941_dn12, locals.var_acd__blk941_dn17,)
    }
};
        locals.var_acd__blk941 = assign28910_e40796;
        locals.var_acd__blk941_dn0 = assign28910_e40796_d_n0;
        locals.var_acd__blk941_dn2 = assign28910_e40796_d_n2;
        locals.var_acd__blk941_dn6 = assign28910_e40796_d_n6;
        locals.var_acd__blk941_dn7 = assign28910_e40796_d_n7;
        locals.var_acd__blk941_dn10 = assign28910_e40796_d_n10;
        locals.var_acd__blk941_dn11 = assign28910_e40796_d_n11;
        locals.var_acd__blk941_dn12 = assign28910_e40796_d_n12;
        locals.var_acd__blk941_dn17 = assign28910_e40796_d_n17;
        locals.var_acd__blk941_rv = 0.0;

        let (assign28920_e40824, assign28920_e40824_d_n0, assign28920_e40824_d_n2, assign28920_e40824_d_n6, assign28920_e40824_d_n7, assign28920_e40824_d_n10, assign28920_e40824_d_n11, assign28920_e40824_d_n12, assign28920_e40824_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28920_e40806: f64 = (-4.0);
        let assign28920_e40808: f64 = (assign28920_e40806 * 1.414213562373095);
        let assign28920_e40811: f64 = (12.0 * locals.var_ty__blk905);
        let assign28920_e40812: f64 = (assign28920_e40808 - assign28920_e40811);
        let assign28920_e40815: f64 = (2.0 * locals.var_acd__blk941);
        let assign28920_e40816: f64 = (assign28920_e40812 + assign28920_e40815);
        let assign28920_e40819: f64 = (1.414213562373095 * locals.var_acd__blk941);
        let assign28920_e40821: f64 = (assign28920_e40819 * locals.var_acd__blk941);
        let assign28920_e40822: f64 = (assign28920_e40816 + assign28920_e40821);
        (assign28920_e40822, (((-(12.0 * locals.var_ty__blk905_dn0)) + (2.0 * locals.var_acd__blk941_dn0)) + (((1.414213562373095 * locals.var_acd__blk941_dn0) * locals.var_acd__blk941) + (assign28920_e40819 * locals.var_acd__blk941_dn0))), (((-(12.0 * locals.var_ty__blk905_dn2)) + (2.0 * locals.var_acd__blk941_dn2)) + (((1.414213562373095 * locals.var_acd__blk941_dn2) * locals.var_acd__blk941) + (assign28920_e40819 * locals.var_acd__blk941_dn2))), (((-(12.0 * locals.var_ty__blk905_dn6)) + (2.0 * locals.var_acd__blk941_dn6)) + (((1.414213562373095 * locals.var_acd__blk941_dn6) * locals.var_acd__blk941) + (assign28920_e40819 * locals.var_acd__blk941_dn6))), (((-(12.0 * locals.var_ty__blk905_dn7)) + (2.0 * locals.var_acd__blk941_dn7)) + (((1.414213562373095 * locals.var_acd__blk941_dn7) * locals.var_acd__blk941) + (assign28920_e40819 * locals.var_acd__blk941_dn7))), (((-(12.0 * locals.var_ty__blk905_dn10)) + (2.0 * locals.var_acd__blk941_dn10)) + (((1.414213562373095 * locals.var_acd__blk941_dn10) * locals.var_acd__blk941) + (assign28920_e40819 * locals.var_acd__blk941_dn10))), (((-(12.0 * locals.var_ty__blk905_dn11)) + (2.0 * locals.var_acd__blk941_dn11)) + (((1.414213562373095 * locals.var_acd__blk941_dn11) * locals.var_acd__blk941) + (assign28920_e40819 * locals.var_acd__blk941_dn11))), (((-(12.0 * locals.var_ty__blk905_dn12)) + (2.0 * locals.var_acd__blk941_dn12)) + (((1.414213562373095 * locals.var_acd__blk941_dn12) * locals.var_acd__blk941) + (assign28920_e40819 * locals.var_acd__blk941_dn12))), (((-(12.0 * locals.var_ty__blk905_dn17)) + (2.0 * locals.var_acd__blk941_dn17)) + (((1.414213562373095 * locals.var_acd__blk941_dn17) * locals.var_acd__blk941) + (assign28920_e40819 * locals.var_acd__blk941_dn17))),)
    } else {
        (locals.var_acn__blk942, locals.var_acn__blk942_dn0, locals.var_acn__blk942_dn2, locals.var_acn__blk942_dn6, locals.var_acn__blk942_dn7, locals.var_acn__blk942_dn10, locals.var_acn__blk942_dn11, locals.var_acn__blk942_dn12, locals.var_acn__blk942_dn17,)
    }
};
        locals.var_acn__blk942 = assign28920_e40824;
        locals.var_acn__blk942_dn0 = assign28920_e40824_d_n0;
        locals.var_acn__blk942_dn2 = assign28920_e40824_d_n2;
        locals.var_acn__blk942_dn6 = assign28920_e40824_d_n6;
        locals.var_acn__blk942_dn7 = assign28920_e40824_d_n7;
        locals.var_acn__blk942_dn10 = assign28920_e40824_d_n10;
        locals.var_acn__blk942_dn11 = assign28920_e40824_d_n11;
        locals.var_acn__blk942_dn12 = assign28920_e40824_d_n12;
        locals.var_acn__blk942_dn17 = assign28920_e40824_d_n17;
        locals.var_acn__blk942_rv = 0.0;

        let (assign28930_e40837, assign28930_e40837_d_n0, assign28930_e40837_d_n2, assign28930_e40837_d_n6, assign28930_e40837_d_n7, assign28930_e40837_d_n10, assign28930_e40837_d_n11, assign28930_e40837_d_n12, assign28930_e40837_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28930_e40835: f64 = (locals.var_acn__blk942 / locals.var_acd__blk941);
        (assign28930_e40835, (((locals.var_acn__blk942_dn0 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn0)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn2 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn2)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn6 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn6)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn7 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn7)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn10 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn10)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn11 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn11)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn12 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn12)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn17 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn17)) / (locals.var_acd__blk941 * locals.var_acd__blk941)),)
    } else {
        (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    }
};
        locals.var_chi__blk943 = assign28930_e40837;
        locals.var_chi__blk943_dn0 = assign28930_e40837_d_n0;
        locals.var_chi__blk943_dn2 = assign28930_e40837_d_n2;
        locals.var_chi__blk943_dn6 = assign28930_e40837_d_n6;
        locals.var_chi__blk943_dn7 = assign28930_e40837_d_n7;
        locals.var_chi__blk943_dn10 = assign28930_e40837_d_n10;
        locals.var_chi__blk943_dn11 = assign28930_e40837_d_n11;
        locals.var_chi__blk943_dn12 = assign28930_e40837_d_n12;
        locals.var_chi__blk943_dn17 = assign28930_e40837_d_n17;
        locals.var_chi__blk943_rv = 0.0;

        let (assign28940_e40852, assign28940_e40852_d_n0, assign28940_e40852_d_n2, assign28940_e40852_d_n6, assign28940_e40852_d_n7, assign28940_e40852_d_n10, assign28940_e40852_d_n11, assign28940_e40852_d_n12, assign28940_e40852_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28940_e40848: f64 = (locals.var_chi__blk943 * locals.var_beta_inv);
        let assign28940_e40850: f64 = (assign28940_e40848 - locals.var_vxbgmtcl__blk921);
        (assign28940_e40850, ((locals.var_chi__blk943_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn0), ((locals.var_chi__blk943_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn2), ((locals.var_chi__blk943_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn6), ((locals.var_chi__blk943_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn7), (((locals.var_chi__blk943_dn10 * locals.var_beta_inv) + (locals.var_chi__blk943 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl__blk921_dn10), ((locals.var_chi__blk943_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn11), ((locals.var_chi__blk943_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn12), ((locals.var_chi__blk943_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_psa__blk944, locals.var_psa__blk944_dn0, locals.var_psa__blk944_dn2, locals.var_psa__blk944_dn6, locals.var_psa__blk944_dn7, locals.var_psa__blk944_dn10, locals.var_psa__blk944_dn11, locals.var_psa__blk944_dn12, locals.var_psa__blk944_dn17,)
    }
};
        locals.var_psa__blk944 = assign28940_e40852;
        locals.var_psa__blk944_dn0 = assign28940_e40852_d_n0;
        locals.var_psa__blk944_dn2 = assign28940_e40852_d_n2;
        locals.var_psa__blk944_dn6 = assign28940_e40852_d_n6;
        locals.var_psa__blk944_dn7 = assign28940_e40852_d_n7;
        locals.var_psa__blk944_dn10 = assign28940_e40852_d_n10;
        locals.var_psa__blk944_dn11 = assign28940_e40852_d_n11;
        locals.var_psa__blk944_dn12 = assign28940_e40852_d_n12;
        locals.var_psa__blk944_dn17 = assign28940_e40852_d_n17;
        locals.var_psa__blk944_rv = 0.0;

        let (assign28950_e40865, assign28950_e40865_d_n0, assign28950_e40865_d_n2, assign28950_e40865_d_n6, assign28950_e40865_d_n7, assign28950_e40865_d_n10, assign28950_e40865_d_n11, assign28950_e40865_d_n12, assign28950_e40865_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28950_e40863: f64 = (locals.var_psa__blk944 + locals.var_vxbgmtcl__blk921);
        (assign28950_e40863, (locals.var_psa__blk944_dn0 + locals.var_vxbgmtcl__blk921_dn0), (locals.var_psa__blk944_dn2 + locals.var_vxbgmtcl__blk921_dn2), (locals.var_psa__blk944_dn6 + locals.var_vxbgmtcl__blk921_dn6), (locals.var_psa__blk944_dn7 + locals.var_vxbgmtcl__blk921_dn7), (locals.var_psa__blk944_dn10 + locals.var_vxbgmtcl__blk921_dn10), (locals.var_psa__blk944_dn11 + locals.var_vxbgmtcl__blk921_dn11), (locals.var_psa__blk944_dn12 + locals.var_vxbgmtcl__blk921_dn12), (locals.var_psa__blk944_dn17 + locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign28950_e40865;
        locals.var_t1__blk896_dn0 = assign28950_e40865_d_n0;
        locals.var_t1__blk896_dn2 = assign28950_e40865_d_n2;
        locals.var_t1__blk896_dn6 = assign28950_e40865_d_n6;
        locals.var_t1__blk896_dn7 = assign28950_e40865_d_n7;
        locals.var_t1__blk896_dn10 = assign28950_e40865_d_n10;
        locals.var_t1__blk896_dn11 = assign28950_e40865_d_n11;
        locals.var_t1__blk896_dn12 = assign28950_e40865_d_n12;
        locals.var_t1__blk896_dn17 = assign28950_e40865_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let (assign28960_e40878, assign28960_e40878_d_n0, assign28960_e40878_d_n2, assign28960_e40878_d_n6, assign28960_e40878_d_n7, assign28960_e40878_d_n10, assign28960_e40878_d_n11, assign28960_e40878_d_n12, assign28960_e40878_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28960_e40876: f64 = (locals.var_t1__blk896 / locals.var_ps0_min__blk936);
        (assign28960_e40876, (((locals.var_t1__blk896_dn0 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn0)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn2 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn2)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn6 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn6)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn7 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn7)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn10 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn10)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn11 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn11)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn12 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn12)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn17 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn17)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)),)
    } else {
        (locals.var_t2__blk897, locals.var_t2__blk897_dn0, locals.var_t2__blk897_dn2, locals.var_t2__blk897_dn6, locals.var_t2__blk897_dn7, locals.var_t2__blk897_dn10, locals.var_t2__blk897_dn11, locals.var_t2__blk897_dn12, locals.var_t2__blk897_dn17,)
    }
};
        locals.var_t2__blk897 = assign28960_e40878;
        locals.var_t2__blk897_dn0 = assign28960_e40878_d_n0;
        locals.var_t2__blk897_dn2 = assign28960_e40878_d_n2;
        locals.var_t2__blk897_dn6 = assign28960_e40878_d_n6;
        locals.var_t2__blk897_dn7 = assign28960_e40878_d_n7;
        locals.var_t2__blk897_dn10 = assign28960_e40878_d_n10;
        locals.var_t2__blk897_dn11 = assign28960_e40878_d_n11;
        locals.var_t2__blk897_dn12 = assign28960_e40878_d_n12;
        locals.var_t2__blk897_dn17 = assign28960_e40878_d_n17;
        locals.var_t2__blk897_rv = 0.0;

        let (assign28970_e40894, assign28970_e40894_d_n0, assign28970_e40894_d_n2, assign28970_e40894_d_n6, assign28970_e40894_d_n7, assign28970_e40894_d_n10, assign28970_e40894_d_n11, assign28970_e40894_d_n12, assign28970_e40894_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28970_e40890: f64 = (locals.var_t2__blk897 * locals.var_t2__blk897);
        let assign28970_e40891: f64 = (1.0 + assign28970_e40890);
        let assign28970_e40892: f64 = (assign28970_e40891).sqrt();
        (assign28970_e40892, (((locals.var_t2__blk897_dn0 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn0)) / (2.0 * assign28970_e40892)), (((locals.var_t2__blk897_dn2 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn2)) / (2.0 * assign28970_e40892)), (((locals.var_t2__blk897_dn6 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn6)) / (2.0 * assign28970_e40892)), (((locals.var_t2__blk897_dn7 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn7)) / (2.0 * assign28970_e40892)), (((locals.var_t2__blk897_dn10 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn10)) / (2.0 * assign28970_e40892)), (((locals.var_t2__blk897_dn11 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn11)) / (2.0 * assign28970_e40892)), (((locals.var_t2__blk897_dn12 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn12)) / (2.0 * assign28970_e40892)), (((locals.var_t2__blk897_dn17 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn17)) / (2.0 * assign28970_e40892)),)
    } else {
        (locals.var_t3__blk898, locals.var_t3__blk898_dn0, locals.var_t3__blk898_dn2, locals.var_t3__blk898_dn6, locals.var_t3__blk898_dn7, locals.var_t3__blk898_dn10, locals.var_t3__blk898_dn11, locals.var_t3__blk898_dn12, locals.var_t3__blk898_dn17,)
    }
};
        locals.var_t3__blk898 = assign28970_e40894;
        locals.var_t3__blk898_dn0 = assign28970_e40894_d_n0;
        locals.var_t3__blk898_dn2 = assign28970_e40894_d_n2;
        locals.var_t3__blk898_dn6 = assign28970_e40894_d_n6;
        locals.var_t3__blk898_dn7 = assign28970_e40894_d_n7;
        locals.var_t3__blk898_dn10 = assign28970_e40894_d_n10;
        locals.var_t3__blk898_dn11 = assign28970_e40894_d_n11;
        locals.var_t3__blk898_dn12 = assign28970_e40894_d_n12;
        locals.var_t3__blk898_dn17 = assign28970_e40894_d_n17;
        locals.var_t3__blk898_rv = 0.0;

        let (assign28980_e40909, assign28980_e40909_d_n0, assign28980_e40909_d_n2, assign28980_e40909_d_n6, assign28980_e40909_d_n7, assign28980_e40909_d_n10, assign28980_e40909_d_n11, assign28980_e40909_d_n12, assign28980_e40909_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28980_e40905: f64 = (locals.var_t1__blk896 / locals.var_t3__blk898);
        let assign28980_e40907: f64 = (assign28980_e40905 - locals.var_vxbgmtcl__blk921);
        (assign28980_e40907, ((((locals.var_t1__blk896_dn0 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn0)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn0), ((((locals.var_t1__blk896_dn2 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn2)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn2), ((((locals.var_t1__blk896_dn6 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn6)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn6), ((((locals.var_t1__blk896_dn7 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn7)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn7), ((((locals.var_t1__blk896_dn10 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn10)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn10), ((((locals.var_t1__blk896_dn11 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn11)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn11), ((((locals.var_t1__blk896_dn12 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn12)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn12), ((((locals.var_t1__blk896_dn17 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn17)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_ps0ld__blk945, locals.var_ps0ld__blk945_dn0, locals.var_ps0ld__blk945_dn2, locals.var_ps0ld__blk945_dn6, locals.var_ps0ld__blk945_dn7, locals.var_ps0ld__blk945_dn10, locals.var_ps0ld__blk945_dn11, locals.var_ps0ld__blk945_dn12, locals.var_ps0ld__blk945_dn17,)
    }
};
        locals.var_ps0ld__blk945 = assign28980_e40909;
        locals.var_ps0ld__blk945_dn0 = assign28980_e40909_d_n0;
        locals.var_ps0ld__blk945_dn2 = assign28980_e40909_d_n2;
        locals.var_ps0ld__blk945_dn6 = assign28980_e40909_d_n6;
        locals.var_ps0ld__blk945_dn7 = assign28980_e40909_d_n7;
        locals.var_ps0ld__blk945_dn10 = assign28980_e40909_d_n10;
        locals.var_ps0ld__blk945_dn11 = assign28980_e40909_d_n11;
        locals.var_ps0ld__blk945_dn12 = assign28980_e40909_d_n12;
        locals.var_ps0ld__blk945_dn17 = assign28980_e40909_d_n17;
        locals.var_ps0ld__blk945_rv = 0.0;

        let (assign28990_e40922, assign28990_e40922_d_n0, assign28990_e40922_d_n2, assign28990_e40922_d_n6, assign28990_e40922_d_n7, assign28990_e40922_d_n10, assign28990_e40922_d_n11, assign28990_e40922_d_n12, assign28990_e40922_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign28990_e40920: f64 = (locals.var_vgpld__blk931 - locals.var_ps0ld__blk945);
        (assign28990_e40920, (locals.var_vgpld__blk931_dn0 - locals.var_ps0ld__blk945_dn0), (locals.var_vgpld__blk931_dn2 - locals.var_ps0ld__blk945_dn2), (locals.var_vgpld__blk931_dn6 - locals.var_ps0ld__blk945_dn6), (locals.var_vgpld__blk931_dn7 - locals.var_ps0ld__blk945_dn7), (locals.var_vgpld__blk931_dn10 - locals.var_ps0ld__blk945_dn10), (locals.var_vgpld__blk931_dn11 - locals.var_ps0ld__blk945_dn11), (locals.var_vgpld__blk931_dn12 - locals.var_ps0ld__blk945_dn12), (locals.var_vgpld__blk931_dn17 - locals.var_ps0ld__blk945_dn17),)
    } else {
        (locals.var_t2__blk897, locals.var_t2__blk897_dn0, locals.var_t2__blk897_dn2, locals.var_t2__blk897_dn6, locals.var_t2__blk897_dn7, locals.var_t2__blk897_dn10, locals.var_t2__blk897_dn11, locals.var_t2__blk897_dn12, locals.var_t2__blk897_dn17,)
    }
};
        locals.var_t2__blk897 = assign28990_e40922;
        locals.var_t2__blk897_dn0 = assign28990_e40922_d_n0;
        locals.var_t2__blk897_dn2 = assign28990_e40922_d_n2;
        locals.var_t2__blk897_dn6 = assign28990_e40922_d_n6;
        locals.var_t2__blk897_dn7 = assign28990_e40922_d_n7;
        locals.var_t2__blk897_dn10 = assign28990_e40922_d_n10;
        locals.var_t2__blk897_dn11 = assign28990_e40922_d_n11;
        locals.var_t2__blk897_dn12 = assign28990_e40922_d_n12;
        locals.var_t2__blk897_dn17 = assign28990_e40922_d_n17;
        locals.var_t2__blk897_rv = 0.0;

        let (assign29000_e40935, assign29000_e40935_d_n0, assign29000_e40935_d_n2, assign29000_e40935_d_n6, assign29000_e40935_d_n7, assign29000_e40935_d_n10, assign29000_e40935_d_n11, assign29000_e40935_d_n12, assign29000_e40935_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) {
        let assign29000_e40933: f64 = (locals.var_cox0__blk906 * locals.var_t2__blk897);
        (assign29000_e40933, (locals.var_cox0__blk906 * locals.var_t2__blk897_dn0), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn2), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn6), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn7), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn10), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn11), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn12), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign29000_e40935;
        locals.var_qsuld_dn0 = assign29000_e40935_d_n0;
        locals.var_qsuld_dn2 = assign29000_e40935_d_n2;
        locals.var_qsuld_dn6 = assign29000_e40935_d_n6;
        locals.var_qsuld_dn7 = assign29000_e40935_d_n7;
        locals.var_qsuld_dn10 = assign29000_e40935_d_n10;
        locals.var_qsuld_dn11 = assign29000_e40935_d_n11;
        locals.var_qsuld_dn12 = assign29000_e40935_d_n12;
        locals.var_qsuld_dn17 = assign29000_e40935_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign29010_e40946, assign29010_e40946_d_n0, assign29010_e40946_d_n2, assign29010_e40946_d_n6, assign29010_e40946_d_n7, assign29010_e40946_d_n10, assign29010_e40946_d_n11, assign29010_e40946_d_n12, assign29010_e40946_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign29010_e40946;
        locals.var_qbuld_dn0 = assign29010_e40946_d_n0;
        locals.var_qbuld_dn2 = assign29010_e40946_d_n2;
        locals.var_qbuld_dn6 = assign29010_e40946_d_n6;
        locals.var_qbuld_dn7 = assign29010_e40946_d_n7;
        locals.var_qbuld_dn10 = assign29010_e40946_d_n10;
        locals.var_qbuld_dn11 = assign29010_e40946_d_n11;
        locals.var_qbuld_dn12 = assign29010_e40946_d_n12;
        locals.var_qbuld_dn17 = assign29010_e40946_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign29030_e40970, assign29030_e40970_d_n0, assign29030_e40970_d_n2, assign29030_e40970_d_n6, assign29030_e40970_d_n7, assign29030_e40970_d_n10, assign29030_e40970_d_n11, assign29030_e40970_d_n12, assign29030_e40970_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) {
        (3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    }
};
        locals.var_chi__blk943 = assign29030_e40970;
        locals.var_chi__blk943_dn0 = assign29030_e40970_d_n0;
        locals.var_chi__blk943_dn2 = assign29030_e40970_d_n2;
        locals.var_chi__blk943_dn6 = assign29030_e40970_d_n6;
        locals.var_chi__blk943_dn7 = assign29030_e40970_d_n7;
        locals.var_chi__blk943_dn10 = assign29030_e40970_d_n10;
        locals.var_chi__blk943_dn11 = assign29030_e40970_d_n11;
        locals.var_chi__blk943_dn12 = assign29030_e40970_d_n12;
        locals.var_chi__blk943_dn17 = assign29030_e40970_d_n17;
        locals.var_chi__blk943_rv = 0.0;

        let (assign29040_e40986, assign29040_e40986_d_n0, assign29040_e40986_d_n2, assign29040_e40986_d_n6, assign29040_e40986_d_n7, assign29040_e40986_d_n10, assign29040_e40986_d_n11, assign29040_e40986_d_n12, assign29040_e40986_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) {
        let assign29040_e40982: f64 = (locals.var_chi__blk943 / locals.var_beta);
        let assign29040_e40984: f64 = (assign29040_e40982 - locals.var_vxbgmtcl__blk921);
        (assign29040_e40984, ((locals.var_chi__blk943_dn0 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn0), ((locals.var_chi__blk943_dn2 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn2), ((locals.var_chi__blk943_dn6 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn6), ((locals.var_chi__blk943_dn7 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn7), ((((locals.var_chi__blk943_dn10 * locals.var_beta) - (locals.var_chi__blk943 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl__blk921_dn10), ((locals.var_chi__blk943_dn11 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn11), ((locals.var_chi__blk943_dn12 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn12), ((locals.var_chi__blk943_dn17 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_ps0_inia__blk946, locals.var_ps0_inia__blk946_dn0, locals.var_ps0_inia__blk946_dn2, locals.var_ps0_inia__blk946_dn6, locals.var_ps0_inia__blk946_dn7, locals.var_ps0_inia__blk946_dn10, locals.var_ps0_inia__blk946_dn11, locals.var_ps0_inia__blk946_dn12, locals.var_ps0_inia__blk946_dn17,)
    }
};
        locals.var_ps0_inia__blk946 = assign29040_e40986;
        locals.var_ps0_inia__blk946_dn0 = assign29040_e40986_d_n0;
        locals.var_ps0_inia__blk946_dn2 = assign29040_e40986_d_n2;
        locals.var_ps0_inia__blk946_dn6 = assign29040_e40986_d_n6;
        locals.var_ps0_inia__blk946_dn7 = assign29040_e40986_d_n7;
        locals.var_ps0_inia__blk946_dn10 = assign29040_e40986_d_n10;
        locals.var_ps0_inia__blk946_dn11 = assign29040_e40986_d_n11;
        locals.var_ps0_inia__blk946_dn12 = assign29040_e40986_d_n12;
        locals.var_ps0_inia__blk946_dn17 = assign29040_e40986_d_n17;
        locals.var_ps0_inia__blk946_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_105(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29050_e41000, assign29050_e41000_d_n0, assign29050_e41000_d_n2, assign29050_e41000_d_n6, assign29050_e41000_d_n7, assign29050_e41000_d_n10, assign29050_e41000_d_n11, assign29050_e41000_d_n12, assign29050_e41000_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) {
        let assign29050_e40997: f64 = (-locals.var_chi__blk943);
        let assign29050_e40998: f64 = (assign29050_e40997).exp();
        (assign29050_e40998, (assign29050_e40998 * (-locals.var_chi__blk943_dn0)), (assign29050_e40998 * (-locals.var_chi__blk943_dn2)), (assign29050_e40998 * (-locals.var_chi__blk943_dn6)), (assign29050_e40998 * (-locals.var_chi__blk943_dn7)), (assign29050_e40998 * (-locals.var_chi__blk943_dn10)), (assign29050_e40998 * (-locals.var_chi__blk943_dn11)), (assign29050_e40998 * (-locals.var_chi__blk943_dn12)), (assign29050_e40998 * (-locals.var_chi__blk943_dn17)),)
    } else {
        (locals.var_ty__blk905, locals.var_ty__blk905_dn0, locals.var_ty__blk905_dn2, locals.var_ty__blk905_dn6, locals.var_ty__blk905_dn7, locals.var_ty__blk905_dn10, locals.var_ty__blk905_dn11, locals.var_ty__blk905_dn12, locals.var_ty__blk905_dn17,)
    }
};
        locals.var_ty__blk905 = assign29050_e41000;
        locals.var_ty__blk905_dn0 = assign29050_e41000_d_n0;
        locals.var_ty__blk905_dn2 = assign29050_e41000_d_n2;
        locals.var_ty__blk905_dn6 = assign29050_e41000_d_n6;
        locals.var_ty__blk905_dn7 = assign29050_e41000_d_n7;
        locals.var_ty__blk905_dn10 = assign29050_e41000_d_n10;
        locals.var_ty__blk905_dn11 = assign29050_e41000_d_n11;
        locals.var_ty__blk905_dn12 = assign29050_e41000_d_n12;
        locals.var_ty__blk905_dn17 = assign29050_e41000_d_n17;
        locals.var_ty__blk905_rv = 0.0;

        let (assign29060_e41028, assign29060_e41028_d_n0, assign29060_e41028_d_n2, assign29060_e41028_d_n6, assign29060_e41028_d_n7, assign29060_e41028_d_n10, assign29060_e41028_d_n11, assign29060_e41028_d_n12, assign29060_e41028_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) {
        let assign29060_e41015: f64 = (locals.var_vgpld__blk931 + locals.var_vxbgmtcl__blk921);
        let assign29060_e41016: f64 = (locals.var_beta * assign29060_e41015);
        let assign29060_e41018: f64 = (assign29060_e41016 - 1.0);
        let assign29060_e41020: f64 = (assign29060_e41018 + locals.var_ty__blk905);
        let assign29060_e41021: f64 = (4.0 * assign29060_e41020);
        let assign29060_e41024: f64 = (locals.var_fac1p2__blk930 * locals.var_beta2);
        let assign29060_e41025: f64 = (assign29060_e41021 / assign29060_e41024);
        let assign29060_e41026: f64 = (1.0 + assign29060_e41025);
        (assign29060_e41026, ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn0 + locals.var_vxbgmtcl__blk921_dn0)) + locals.var_ty__blk905_dn0)) * assign29060_e41024) - (assign29060_e41021 * (locals.var_fac1p2__blk930_dn0 * locals.var_beta2))) / (assign29060_e41024 * assign29060_e41024)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn2 + locals.var_vxbgmtcl__blk921_dn2)) + locals.var_ty__blk905_dn2)) * assign29060_e41024) - (assign29060_e41021 * (locals.var_fac1p2__blk930_dn2 * locals.var_beta2))) / (assign29060_e41024 * assign29060_e41024)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn6 + locals.var_vxbgmtcl__blk921_dn6)) + locals.var_ty__blk905_dn6)) * assign29060_e41024) - (assign29060_e41021 * (locals.var_fac1p2__blk930_dn6 * locals.var_beta2))) / (assign29060_e41024 * assign29060_e41024)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn7 + locals.var_vxbgmtcl__blk921_dn7)) + locals.var_ty__blk905_dn7)) * assign29060_e41024) - (assign29060_e41021 * (locals.var_fac1p2__blk930_dn7 * locals.var_beta2))) / (assign29060_e41024 * assign29060_e41024)), ((((4.0 * (((locals.var_beta_dn10 * assign29060_e41015) + (locals.var_beta * (locals.var_vgpld__blk931_dn10 + locals.var_vxbgmtcl__blk921_dn10))) + locals.var_ty__blk905_dn10)) * assign29060_e41024) - (assign29060_e41021 * ((locals.var_fac1p2__blk930_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk930 * locals.var_beta2_dn10)))) / (assign29060_e41024 * assign29060_e41024)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn11 + locals.var_vxbgmtcl__blk921_dn11)) + locals.var_ty__blk905_dn11)) * assign29060_e41024) - (assign29060_e41021 * (locals.var_fac1p2__blk930_dn11 * locals.var_beta2))) / (assign29060_e41024 * assign29060_e41024)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn12 + locals.var_vxbgmtcl__blk921_dn12)) + locals.var_ty__blk905_dn12)) * assign29060_e41024) - (assign29060_e41021 * (locals.var_fac1p2__blk930_dn12 * locals.var_beta2))) / (assign29060_e41024 * assign29060_e41024)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn17 + locals.var_vxbgmtcl__blk921_dn17)) + locals.var_ty__blk905_dn17)) * assign29060_e41024) - (assign29060_e41021 * (locals.var_fac1p2__blk930_dn17 * locals.var_beta2))) / (assign29060_e41024 * assign29060_e41024)),)
    } else {
        (locals.var_tx__blk904, locals.var_tx__blk904_dn0, locals.var_tx__blk904_dn2, locals.var_tx__blk904_dn6, locals.var_tx__blk904_dn7, locals.var_tx__blk904_dn10, locals.var_tx__blk904_dn11, locals.var_tx__blk904_dn12, locals.var_tx__blk904_dn17,)
    }
};
        locals.var_tx__blk904 = assign29060_e41028;
        locals.var_tx__blk904_dn0 = assign29060_e41028_d_n0;
        locals.var_tx__blk904_dn2 = assign29060_e41028_d_n2;
        locals.var_tx__blk904_dn6 = assign29060_e41028_d_n6;
        locals.var_tx__blk904_dn7 = assign29060_e41028_d_n7;
        locals.var_tx__blk904_dn10 = assign29060_e41028_d_n10;
        locals.var_tx__blk904_dn11 = assign29060_e41028_d_n11;
        locals.var_tx__blk904_dn12 = assign29060_e41028_d_n12;
        locals.var_tx__blk904_dn17 = assign29060_e41028_d_n17;
        locals.var_tx__blk904_rv = 0.0;

        let assign29070_e41032: f64 = (10.0 * 2.220446049250313e-16);
        let assign29070_e41033: f64 = if locals.var_tx__blk904 < assign29070_e41032 { 1.0 } else { 0.0 };
        locals.var_guard984 = assign29070_e41033;
        locals.var_guard984_rv = 0.0;

        let (assign29080_e41049, assign29080_e41049_d_n0, assign29080_e41049_d_n2, assign29080_e41049_d_n6, assign29080_e41049_d_n7, assign29080_e41049_d_n10, assign29080_e41049_d_n11, assign29080_e41049_d_n12, assign29080_e41049_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard984 != 0.0)) {
        let assign29080_e41047: f64 = (10.0 * 2.220446049250313e-16);
        (assign29080_e41047, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk904, locals.var_tx__blk904_dn0, locals.var_tx__blk904_dn2, locals.var_tx__blk904_dn6, locals.var_tx__blk904_dn7, locals.var_tx__blk904_dn10, locals.var_tx__blk904_dn11, locals.var_tx__blk904_dn12, locals.var_tx__blk904_dn17,)
    }
};
        locals.var_tx__blk904 = assign29080_e41049;
        locals.var_tx__blk904_dn0 = assign29080_e41049_d_n0;
        locals.var_tx__blk904_dn2 = assign29080_e41049_d_n2;
        locals.var_tx__blk904_dn6 = assign29080_e41049_d_n6;
        locals.var_tx__blk904_dn7 = assign29080_e41049_d_n7;
        locals.var_tx__blk904_dn10 = assign29080_e41049_d_n10;
        locals.var_tx__blk904_dn11 = assign29080_e41049_d_n11;
        locals.var_tx__blk904_dn12 = assign29080_e41049_d_n12;
        locals.var_tx__blk904_dn17 = assign29080_e41049_d_n17;
        locals.var_tx__blk904_rv = 0.0;

        let (assign29090_e41072, assign29090_e41072_d_n0, assign29090_e41072_d_n2, assign29090_e41072_d_n6, assign29090_e41072_d_n7, assign29090_e41072_d_n10, assign29090_e41072_d_n11, assign29090_e41072_d_n12, assign29090_e41072_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) {
        let assign29090_e41062: f64 = (locals.var_fac1p2__blk930 * locals.var_beta);
        let assign29090_e41064: f64 = (assign29090_e41062 / 2.0);
        let assign29090_e41067: f64 = (locals.var_tx__blk904).sqrt();
        let assign29090_e41068: f64 = (1.0 - assign29090_e41067);
        let assign29090_e41069: f64 = (assign29090_e41064 * assign29090_e41068);
        let assign29090_e41070: f64 = (locals.var_vgpld__blk931 + assign29090_e41069);
        (assign29090_e41070, (locals.var_vgpld__blk931_dn0 + ((((locals.var_fac1p2__blk930_dn0 * locals.var_beta) / 2.0) * assign29090_e41068) + (assign29090_e41064 * (-(locals.var_tx__blk904_dn0 / (2.0 * assign29090_e41067)))))), (locals.var_vgpld__blk931_dn2 + ((((locals.var_fac1p2__blk930_dn2 * locals.var_beta) / 2.0) * assign29090_e41068) + (assign29090_e41064 * (-(locals.var_tx__blk904_dn2 / (2.0 * assign29090_e41067)))))), (locals.var_vgpld__blk931_dn6 + ((((locals.var_fac1p2__blk930_dn6 * locals.var_beta) / 2.0) * assign29090_e41068) + (assign29090_e41064 * (-(locals.var_tx__blk904_dn6 / (2.0 * assign29090_e41067)))))), (locals.var_vgpld__blk931_dn7 + ((((locals.var_fac1p2__blk930_dn7 * locals.var_beta) / 2.0) * assign29090_e41068) + (assign29090_e41064 * (-(locals.var_tx__blk904_dn7 / (2.0 * assign29090_e41067)))))), (locals.var_vgpld__blk931_dn10 + (((((locals.var_fac1p2__blk930_dn10 * locals.var_beta) + (locals.var_fac1p2__blk930 * locals.var_beta_dn10)) / 2.0) * assign29090_e41068) + (assign29090_e41064 * (-(locals.var_tx__blk904_dn10 / (2.0 * assign29090_e41067)))))), (locals.var_vgpld__blk931_dn11 + ((((locals.var_fac1p2__blk930_dn11 * locals.var_beta) / 2.0) * assign29090_e41068) + (assign29090_e41064 * (-(locals.var_tx__blk904_dn11 / (2.0 * assign29090_e41067)))))), (locals.var_vgpld__blk931_dn12 + ((((locals.var_fac1p2__blk930_dn12 * locals.var_beta) / 2.0) * assign29090_e41068) + (assign29090_e41064 * (-(locals.var_tx__blk904_dn12 / (2.0 * assign29090_e41067)))))), (locals.var_vgpld__blk931_dn17 + ((((locals.var_fac1p2__blk930_dn17 * locals.var_beta) / 2.0) * assign29090_e41068) + (assign29090_e41064 * (-(locals.var_tx__blk904_dn17 / (2.0 * assign29090_e41067)))))),)
    } else {
        (locals.var_ps0_inia__blk946, locals.var_ps0_inia__blk946_dn0, locals.var_ps0_inia__blk946_dn2, locals.var_ps0_inia__blk946_dn6, locals.var_ps0_inia__blk946_dn7, locals.var_ps0_inia__blk946_dn10, locals.var_ps0_inia__blk946_dn11, locals.var_ps0_inia__blk946_dn12, locals.var_ps0_inia__blk946_dn17,)
    }
};
        locals.var_ps0_inia__blk946 = assign29090_e41072;
        locals.var_ps0_inia__blk946_dn0 = assign29090_e41072_d_n0;
        locals.var_ps0_inia__blk946_dn2 = assign29090_e41072_d_n2;
        locals.var_ps0_inia__blk946_dn6 = assign29090_e41072_d_n6;
        locals.var_ps0_inia__blk946_dn7 = assign29090_e41072_d_n7;
        locals.var_ps0_inia__blk946_dn10 = assign29090_e41072_d_n10;
        locals.var_ps0_inia__blk946_dn11 = assign29090_e41072_d_n11;
        locals.var_ps0_inia__blk946_dn12 = assign29090_e41072_d_n12;
        locals.var_ps0_inia__blk946_dn17 = assign29090_e41072_d_n17;
        locals.var_ps0_inia__blk946_rv = 0.0;

        let (assign29100_e41088, assign29100_e41088_d_n0, assign29100_e41088_d_n2, assign29100_e41088_d_n6, assign29100_e41088_d_n7, assign29100_e41088_d_n10, assign29100_e41088_d_n11, assign29100_e41088_d_n12, assign29100_e41088_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) {
        let assign29100_e41085: f64 = (locals.var_ps0_inia__blk946 + locals.var_vxbgmtcl__blk921);
        let assign29100_e41086: f64 = (locals.var_beta * assign29100_e41085);
        (assign29100_e41086, (locals.var_beta * (locals.var_ps0_inia__blk946_dn0 + locals.var_vxbgmtcl__blk921_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn2 + locals.var_vxbgmtcl__blk921_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn6 + locals.var_vxbgmtcl__blk921_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn7 + locals.var_vxbgmtcl__blk921_dn7)), ((locals.var_beta_dn10 * assign29100_e41085) + (locals.var_beta * (locals.var_ps0_inia__blk946_dn10 + locals.var_vxbgmtcl__blk921_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk946_dn11 + locals.var_vxbgmtcl__blk921_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn12 + locals.var_vxbgmtcl__blk921_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn17 + locals.var_vxbgmtcl__blk921_dn17)),)
    } else {
        (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    }
};
        locals.var_chi__blk943 = assign29100_e41088;
        locals.var_chi__blk943_dn0 = assign29100_e41088_d_n0;
        locals.var_chi__blk943_dn2 = assign29100_e41088_d_n2;
        locals.var_chi__blk943_dn6 = assign29100_e41088_d_n6;
        locals.var_chi__blk943_dn7 = assign29100_e41088_d_n7;
        locals.var_chi__blk943_dn10 = assign29100_e41088_d_n10;
        locals.var_chi__blk943_dn11 = assign29100_e41088_d_n11;
        locals.var_chi__blk943_dn12 = assign29100_e41088_d_n12;
        locals.var_chi__blk943_dn17 = assign29100_e41088_d_n17;
        locals.var_chi__blk943_rv = 0.0;

        let (assign29110_e41102, assign29110_e41102_d_n0, assign29110_e41102_d_n2, assign29110_e41102_d_n6, assign29110_e41102_d_n7, assign29110_e41102_d_n10, assign29110_e41102_d_n11, assign29110_e41102_d_n12, assign29110_e41102_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) {
        let assign29110_e41099: f64 = (-locals.var_chi__blk943);
        let assign29110_e41100: f64 = (assign29110_e41099).exp();
        (assign29110_e41100, (assign29110_e41100 * (-locals.var_chi__blk943_dn0)), (assign29110_e41100 * (-locals.var_chi__blk943_dn2)), (assign29110_e41100 * (-locals.var_chi__blk943_dn6)), (assign29110_e41100 * (-locals.var_chi__blk943_dn7)), (assign29110_e41100 * (-locals.var_chi__blk943_dn10)), (assign29110_e41100 * (-locals.var_chi__blk943_dn11)), (assign29110_e41100 * (-locals.var_chi__blk943_dn12)), (assign29110_e41100 * (-locals.var_chi__blk943_dn17)),)
    } else {
        (locals.var_ty__blk905, locals.var_ty__blk905_dn0, locals.var_ty__blk905_dn2, locals.var_ty__blk905_dn6, locals.var_ty__blk905_dn7, locals.var_ty__blk905_dn10, locals.var_ty__blk905_dn11, locals.var_ty__blk905_dn12, locals.var_ty__blk905_dn17,)
    }
};
        locals.var_ty__blk905 = assign29110_e41102;
        locals.var_ty__blk905_dn0 = assign29110_e41102_d_n0;
        locals.var_ty__blk905_dn2 = assign29110_e41102_d_n2;
        locals.var_ty__blk905_dn6 = assign29110_e41102_d_n6;
        locals.var_ty__blk905_dn7 = assign29110_e41102_d_n7;
        locals.var_ty__blk905_dn10 = assign29110_e41102_d_n10;
        locals.var_ty__blk905_dn11 = assign29110_e41102_d_n11;
        locals.var_ty__blk905_dn12 = assign29110_e41102_d_n12;
        locals.var_ty__blk905_dn17 = assign29110_e41102_d_n17;
        locals.var_ty__blk905_rv = 0.0;

        let (assign29120_e41130, assign29120_e41130_d_n0, assign29120_e41130_d_n2, assign29120_e41130_d_n6, assign29120_e41130_d_n7, assign29120_e41130_d_n10, assign29120_e41130_d_n11, assign29120_e41130_d_n12, assign29120_e41130_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) {
        let assign29120_e41117: f64 = (locals.var_vgpld__blk931 + locals.var_vxbgmtcl__blk921);
        let assign29120_e41118: f64 = (locals.var_beta * assign29120_e41117);
        let assign29120_e41120: f64 = (assign29120_e41118 - 1.0);
        let assign29120_e41122: f64 = (assign29120_e41120 + locals.var_ty__blk905);
        let assign29120_e41123: f64 = (4.0 * assign29120_e41122);
        let assign29120_e41126: f64 = (locals.var_fac1p2__blk930 * locals.var_beta2);
        let assign29120_e41127: f64 = (assign29120_e41123 / assign29120_e41126);
        let assign29120_e41128: f64 = (1.0 + assign29120_e41127);
        (assign29120_e41128, ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn0 + locals.var_vxbgmtcl__blk921_dn0)) + locals.var_ty__blk905_dn0)) * assign29120_e41126) - (assign29120_e41123 * (locals.var_fac1p2__blk930_dn0 * locals.var_beta2))) / (assign29120_e41126 * assign29120_e41126)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn2 + locals.var_vxbgmtcl__blk921_dn2)) + locals.var_ty__blk905_dn2)) * assign29120_e41126) - (assign29120_e41123 * (locals.var_fac1p2__blk930_dn2 * locals.var_beta2))) / (assign29120_e41126 * assign29120_e41126)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn6 + locals.var_vxbgmtcl__blk921_dn6)) + locals.var_ty__blk905_dn6)) * assign29120_e41126) - (assign29120_e41123 * (locals.var_fac1p2__blk930_dn6 * locals.var_beta2))) / (assign29120_e41126 * assign29120_e41126)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn7 + locals.var_vxbgmtcl__blk921_dn7)) + locals.var_ty__blk905_dn7)) * assign29120_e41126) - (assign29120_e41123 * (locals.var_fac1p2__blk930_dn7 * locals.var_beta2))) / (assign29120_e41126 * assign29120_e41126)), ((((4.0 * (((locals.var_beta_dn10 * assign29120_e41117) + (locals.var_beta * (locals.var_vgpld__blk931_dn10 + locals.var_vxbgmtcl__blk921_dn10))) + locals.var_ty__blk905_dn10)) * assign29120_e41126) - (assign29120_e41123 * ((locals.var_fac1p2__blk930_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk930 * locals.var_beta2_dn10)))) / (assign29120_e41126 * assign29120_e41126)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn11 + locals.var_vxbgmtcl__blk921_dn11)) + locals.var_ty__blk905_dn11)) * assign29120_e41126) - (assign29120_e41123 * (locals.var_fac1p2__blk930_dn11 * locals.var_beta2))) / (assign29120_e41126 * assign29120_e41126)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn12 + locals.var_vxbgmtcl__blk921_dn12)) + locals.var_ty__blk905_dn12)) * assign29120_e41126) - (assign29120_e41123 * (locals.var_fac1p2__blk930_dn12 * locals.var_beta2))) / (assign29120_e41126 * assign29120_e41126)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn17 + locals.var_vxbgmtcl__blk921_dn17)) + locals.var_ty__blk905_dn17)) * assign29120_e41126) - (assign29120_e41123 * (locals.var_fac1p2__blk930_dn17 * locals.var_beta2))) / (assign29120_e41126 * assign29120_e41126)),)
    } else {
        (locals.var_tx__blk904, locals.var_tx__blk904_dn0, locals.var_tx__blk904_dn2, locals.var_tx__blk904_dn6, locals.var_tx__blk904_dn7, locals.var_tx__blk904_dn10, locals.var_tx__blk904_dn11, locals.var_tx__blk904_dn12, locals.var_tx__blk904_dn17,)
    }
};
        locals.var_tx__blk904 = assign29120_e41130;
        locals.var_tx__blk904_dn0 = assign29120_e41130_d_n0;
        locals.var_tx__blk904_dn2 = assign29120_e41130_d_n2;
        locals.var_tx__blk904_dn6 = assign29120_e41130_d_n6;
        locals.var_tx__blk904_dn7 = assign29120_e41130_d_n7;
        locals.var_tx__blk904_dn10 = assign29120_e41130_d_n10;
        locals.var_tx__blk904_dn11 = assign29120_e41130_d_n11;
        locals.var_tx__blk904_dn12 = assign29120_e41130_d_n12;
        locals.var_tx__blk904_dn17 = assign29120_e41130_d_n17;
        locals.var_tx__blk904_rv = 0.0;

        let assign29130_e41134: f64 = (10.0 * 2.220446049250313e-16);
        let assign29130_e41135: f64 = if locals.var_tx__blk904 < assign29130_e41134 { 1.0 } else { 0.0 };
        locals.var_guard985 = assign29130_e41135;
        locals.var_guard985_rv = 0.0;

        let (assign29140_e41151, assign29140_e41151_d_n0, assign29140_e41151_d_n2, assign29140_e41151_d_n6, assign29140_e41151_d_n7, assign29140_e41151_d_n10, assign29140_e41151_d_n11, assign29140_e41151_d_n12, assign29140_e41151_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard985 != 0.0)) {
        let assign29140_e41149: f64 = (10.0 * 2.220446049250313e-16);
        (assign29140_e41149, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk904, locals.var_tx__blk904_dn0, locals.var_tx__blk904_dn2, locals.var_tx__blk904_dn6, locals.var_tx__blk904_dn7, locals.var_tx__blk904_dn10, locals.var_tx__blk904_dn11, locals.var_tx__blk904_dn12, locals.var_tx__blk904_dn17,)
    }
};
        locals.var_tx__blk904 = assign29140_e41151;
        locals.var_tx__blk904_dn0 = assign29140_e41151_d_n0;
        locals.var_tx__blk904_dn2 = assign29140_e41151_d_n2;
        locals.var_tx__blk904_dn6 = assign29140_e41151_d_n6;
        locals.var_tx__blk904_dn7 = assign29140_e41151_d_n7;
        locals.var_tx__blk904_dn10 = assign29140_e41151_d_n10;
        locals.var_tx__blk904_dn11 = assign29140_e41151_d_n11;
        locals.var_tx__blk904_dn12 = assign29140_e41151_d_n12;
        locals.var_tx__blk904_dn17 = assign29140_e41151_d_n17;
        locals.var_tx__blk904_rv = 0.0;

        let (assign29150_e41174, assign29150_e41174_d_n0, assign29150_e41174_d_n2, assign29150_e41174_d_n6, assign29150_e41174_d_n7, assign29150_e41174_d_n10, assign29150_e41174_d_n11, assign29150_e41174_d_n12, assign29150_e41174_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) {
        let assign29150_e41164: f64 = (locals.var_fac1p2__blk930 * locals.var_beta);
        let assign29150_e41166: f64 = (assign29150_e41164 / 2.0);
        let assign29150_e41169: f64 = (locals.var_tx__blk904).sqrt();
        let assign29150_e41170: f64 = (1.0 - assign29150_e41169);
        let assign29150_e41171: f64 = (assign29150_e41166 * assign29150_e41170);
        let assign29150_e41172: f64 = (locals.var_vgpld__blk931 + assign29150_e41171);
        (assign29150_e41172, (locals.var_vgpld__blk931_dn0 + ((((locals.var_fac1p2__blk930_dn0 * locals.var_beta) / 2.0) * assign29150_e41170) + (assign29150_e41166 * (-(locals.var_tx__blk904_dn0 / (2.0 * assign29150_e41169)))))), (locals.var_vgpld__blk931_dn2 + ((((locals.var_fac1p2__blk930_dn2 * locals.var_beta) / 2.0) * assign29150_e41170) + (assign29150_e41166 * (-(locals.var_tx__blk904_dn2 / (2.0 * assign29150_e41169)))))), (locals.var_vgpld__blk931_dn6 + ((((locals.var_fac1p2__blk930_dn6 * locals.var_beta) / 2.0) * assign29150_e41170) + (assign29150_e41166 * (-(locals.var_tx__blk904_dn6 / (2.0 * assign29150_e41169)))))), (locals.var_vgpld__blk931_dn7 + ((((locals.var_fac1p2__blk930_dn7 * locals.var_beta) / 2.0) * assign29150_e41170) + (assign29150_e41166 * (-(locals.var_tx__blk904_dn7 / (2.0 * assign29150_e41169)))))), (locals.var_vgpld__blk931_dn10 + (((((locals.var_fac1p2__blk930_dn10 * locals.var_beta) + (locals.var_fac1p2__blk930 * locals.var_beta_dn10)) / 2.0) * assign29150_e41170) + (assign29150_e41166 * (-(locals.var_tx__blk904_dn10 / (2.0 * assign29150_e41169)))))), (locals.var_vgpld__blk931_dn11 + ((((locals.var_fac1p2__blk930_dn11 * locals.var_beta) / 2.0) * assign29150_e41170) + (assign29150_e41166 * (-(locals.var_tx__blk904_dn11 / (2.0 * assign29150_e41169)))))), (locals.var_vgpld__blk931_dn12 + ((((locals.var_fac1p2__blk930_dn12 * locals.var_beta) / 2.0) * assign29150_e41170) + (assign29150_e41166 * (-(locals.var_tx__blk904_dn12 / (2.0 * assign29150_e41169)))))), (locals.var_vgpld__blk931_dn17 + ((((locals.var_fac1p2__blk930_dn17 * locals.var_beta) / 2.0) * assign29150_e41170) + (assign29150_e41166 * (-(locals.var_tx__blk904_dn17 / (2.0 * assign29150_e41169)))))),)
    } else {
        (locals.var_ps0_inia__blk946, locals.var_ps0_inia__blk946_dn0, locals.var_ps0_inia__blk946_dn2, locals.var_ps0_inia__blk946_dn6, locals.var_ps0_inia__blk946_dn7, locals.var_ps0_inia__blk946_dn10, locals.var_ps0_inia__blk946_dn11, locals.var_ps0_inia__blk946_dn12, locals.var_ps0_inia__blk946_dn17,)
    }
};
        locals.var_ps0_inia__blk946 = assign29150_e41174;
        locals.var_ps0_inia__blk946_dn0 = assign29150_e41174_d_n0;
        locals.var_ps0_inia__blk946_dn2 = assign29150_e41174_d_n2;
        locals.var_ps0_inia__blk946_dn6 = assign29150_e41174_d_n6;
        locals.var_ps0_inia__blk946_dn7 = assign29150_e41174_d_n7;
        locals.var_ps0_inia__blk946_dn10 = assign29150_e41174_d_n10;
        locals.var_ps0_inia__blk946_dn11 = assign29150_e41174_d_n11;
        locals.var_ps0_inia__blk946_dn12 = assign29150_e41174_d_n12;
        locals.var_ps0_inia__blk946_dn17 = assign29150_e41174_d_n17;
        locals.var_ps0_inia__blk946_rv = 0.0;

        let (assign29160_e41190, assign29160_e41190_d_n0, assign29160_e41190_d_n2, assign29160_e41190_d_n6, assign29160_e41190_d_n7, assign29160_e41190_d_n10, assign29160_e41190_d_n11, assign29160_e41190_d_n12, assign29160_e41190_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) {
        let assign29160_e41187: f64 = (locals.var_ps0_inia__blk946 + locals.var_vxbgmtcl__blk921);
        let assign29160_e41188: f64 = (locals.var_beta * assign29160_e41187);
        (assign29160_e41188, (locals.var_beta * (locals.var_ps0_inia__blk946_dn0 + locals.var_vxbgmtcl__blk921_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn2 + locals.var_vxbgmtcl__blk921_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn6 + locals.var_vxbgmtcl__blk921_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn7 + locals.var_vxbgmtcl__blk921_dn7)), ((locals.var_beta_dn10 * assign29160_e41187) + (locals.var_beta * (locals.var_ps0_inia__blk946_dn10 + locals.var_vxbgmtcl__blk921_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk946_dn11 + locals.var_vxbgmtcl__blk921_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn12 + locals.var_vxbgmtcl__blk921_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn17 + locals.var_vxbgmtcl__blk921_dn17)),)
    } else {
        (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    }
};
        locals.var_chi__blk943 = assign29160_e41190;
        locals.var_chi__blk943_dn0 = assign29160_e41190_d_n0;
        locals.var_chi__blk943_dn2 = assign29160_e41190_d_n2;
        locals.var_chi__blk943_dn6 = assign29160_e41190_d_n6;
        locals.var_chi__blk943_dn7 = assign29160_e41190_d_n7;
        locals.var_chi__blk943_dn10 = assign29160_e41190_d_n10;
        locals.var_chi__blk943_dn11 = assign29160_e41190_d_n11;
        locals.var_chi__blk943_dn12 = assign29160_e41190_d_n12;
        locals.var_chi__blk943_dn17 = assign29160_e41190_d_n17;
        locals.var_chi__blk943_rv = 0.0;

        let assign29170_e41193: f64 = if locals.var_chi__blk943 < 3.0 { 1.0 } else { 0.0 };
        locals.var_guard986 = assign29170_e41193;
        locals.var_guard986_rv = 0.0;

        let (assign29190_e41238,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign29190_e41222: f64 = (9.0 * 1.414213562373095);
        let assign29190_e41223: f64 = (1.0 / assign29190_e41222);
        let assign29190_e41227: f64 = (7.0 * 0.049787068367863944);
        let assign29190_e41228: f64 = (5.0 + assign29190_e41227);
        let assign29190_e41232: f64 = (2.0 + 0.049787068367863944);
        let assign29190_e41233: f64 = (assign29190_e41232).sqrt();
        let assign29190_e41234: f64 = (54.0 * assign29190_e41233);
        let assign29190_e41235: f64 = (assign29190_e41228 / assign29190_e41234);
        let assign29190_e41236: f64 = (assign29190_e41223 - assign29190_e41235);
        (assign29190_e41236,)
    } else {
        (locals.var_ta__blk947,)
    }
};
        locals.var_ta__blk947 = assign29190_e41238;
        locals.var_ta__blk947_rv = 0.0;

        let (assign29200_e41265,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign29200_e41252: f64 = (1.0 + 0.049787068367863944);
        let assign29200_e41256: f64 = (2.0 + 0.049787068367863944);
        let assign29200_e41257: f64 = (assign29200_e41256).sqrt();
        let assign29200_e41258: f64 = (2.0 * assign29200_e41257);
        let assign29200_e41259: f64 = (assign29200_e41252 / assign29200_e41258);
        let assign29200_e41262: f64 = (1.414213562373095 / 3.0);
        let assign29200_e41263: f64 = (assign29200_e41259 - assign29200_e41262);
        (assign29200_e41263,)
    } else {
        (locals.var_tb__blk948,)
    }
};
        locals.var_tb__blk948 = assign29200_e41265;
        locals.var_tb__blk948_rv = 0.0;

        let (assign29210_e41287, assign29210_e41287_d_n0, assign29210_e41287_d_n2, assign29210_e41287_d_n6, assign29210_e41287_d_n7, assign29210_e41287_d_n10, assign29210_e41287_d_n11, assign29210_e41287_d_n12, assign29210_e41287_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign29210_e41279: f64 = (1.0 / 1.414213562373095);
        let assign29210_e41283: f64 = (locals.var_beta * locals.var_fac1__blk929);
        let assign29210_e41284: f64 = (1.0 / assign29210_e41283);
        let assign29210_e41285: f64 = (assign29210_e41279 + assign29210_e41284);
        (assign29210_e41285, (-((locals.var_beta * locals.var_fac1__blk929_dn0) / (assign29210_e41283 * assign29210_e41283))), (-((locals.var_beta * locals.var_fac1__blk929_dn2) / (assign29210_e41283 * assign29210_e41283))), (-((locals.var_beta * locals.var_fac1__blk929_dn6) / (assign29210_e41283 * assign29210_e41283))), (-((locals.var_beta * locals.var_fac1__blk929_dn7) / (assign29210_e41283 * assign29210_e41283))), (-(((locals.var_beta_dn10 * locals.var_fac1__blk929) + (locals.var_beta * locals.var_fac1__blk929_dn10)) / (assign29210_e41283 * assign29210_e41283))), (-((locals.var_beta * locals.var_fac1__blk929_dn11) / (assign29210_e41283 * assign29210_e41283))), (-((locals.var_beta * locals.var_fac1__blk929_dn12) / (assign29210_e41283 * assign29210_e41283))), (-((locals.var_beta * locals.var_fac1__blk929_dn17) / (assign29210_e41283 * assign29210_e41283))),)
    } else {
        (locals.var_tc__blk949, locals.var_tc__blk949_dn0, locals.var_tc__blk949_dn2, locals.var_tc__blk949_dn6, locals.var_tc__blk949_dn7, locals.var_tc__blk949_dn10, locals.var_tc__blk949_dn11, locals.var_tc__blk949_dn12, locals.var_tc__blk949_dn17,)
    }
};
        locals.var_tc__blk949 = assign29210_e41287;
        locals.var_tc__blk949_dn0 = assign29210_e41287_d_n0;
        locals.var_tc__blk949_dn2 = assign29210_e41287_d_n2;
        locals.var_tc__blk949_dn6 = assign29210_e41287_d_n6;
        locals.var_tc__blk949_dn7 = assign29210_e41287_d_n7;
        locals.var_tc__blk949_dn10 = assign29210_e41287_d_n10;
        locals.var_tc__blk949_dn11 = assign29210_e41287_d_n11;
        locals.var_tc__blk949_dn12 = assign29210_e41287_d_n12;
        locals.var_tc__blk949_dn17 = assign29210_e41287_d_n17;
        locals.var_tc__blk949_rv = 0.0;

        let (assign29220_e41306, assign29220_e41306_d_n0, assign29220_e41306_d_n2, assign29220_e41306_d_n6, assign29220_e41306_d_n7, assign29220_e41306_d_n10, assign29220_e41306_d_n11, assign29220_e41306_d_n12, assign29220_e41306_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign29220_e41301: f64 = (locals.var_vgpld__blk931 + locals.var_vxbgmtcl__blk921);
        let assign29220_e41302: f64 = (-assign29220_e41301);
        let assign29220_e41304: f64 = (assign29220_e41302 / locals.var_fac1__blk929);
        (assign29220_e41304, ((((-(locals.var_vgpld__blk931_dn0 + locals.var_vxbgmtcl__blk921_dn0)) * locals.var_fac1__blk929) - (assign29220_e41302 * locals.var_fac1__blk929_dn0)) / (locals.var_fac1__blk929 * locals.var_fac1__blk929)), ((((-(locals.var_vgpld__blk931_dn2 + locals.var_vxbgmtcl__blk921_dn2)) * locals.var_fac1__blk929) - (assign29220_e41302 * locals.var_fac1__blk929_dn2)) / (locals.var_fac1__blk929 * locals.var_fac1__blk929)), ((((-(locals.var_vgpld__blk931_dn6 + locals.var_vxbgmtcl__blk921_dn6)) * locals.var_fac1__blk929) - (assign29220_e41302 * locals.var_fac1__blk929_dn6)) / (locals.var_fac1__blk929 * locals.var_fac1__blk929)), ((((-(locals.var_vgpld__blk931_dn7 + locals.var_vxbgmtcl__blk921_dn7)) * locals.var_fac1__blk929) - (assign29220_e41302 * locals.var_fac1__blk929_dn7)) / (locals.var_fac1__blk929 * locals.var_fac1__blk929)), ((((-(locals.var_vgpld__blk931_dn10 + locals.var_vxbgmtcl__blk921_dn10)) * locals.var_fac1__blk929) - (assign29220_e41302 * locals.var_fac1__blk929_dn10)) / (locals.var_fac1__blk929 * locals.var_fac1__blk929)), ((((-(locals.var_vgpld__blk931_dn11 + locals.var_vxbgmtcl__blk921_dn11)) * locals.var_fac1__blk929) - (assign29220_e41302 * locals.var_fac1__blk929_dn11)) / (locals.var_fac1__blk929 * locals.var_fac1__blk929)), ((((-(locals.var_vgpld__blk931_dn12 + locals.var_vxbgmtcl__blk921_dn12)) * locals.var_fac1__blk929) - (assign29220_e41302 * locals.var_fac1__blk929_dn12)) / (locals.var_fac1__blk929 * locals.var_fac1__blk929)), ((((-(locals.var_vgpld__blk931_dn17 + locals.var_vxbgmtcl__blk921_dn17)) * locals.var_fac1__blk929) - (assign29220_e41302 * locals.var_fac1__blk929_dn17)) / (locals.var_fac1__blk929 * locals.var_fac1__blk929)),)
    } else {
        (locals.var_td__blk950, locals.var_td__blk950_dn0, locals.var_td__blk950_dn2, locals.var_td__blk950_dn6, locals.var_td__blk950_dn7, locals.var_td__blk950_dn10, locals.var_td__blk950_dn11, locals.var_td__blk950_dn12, locals.var_td__blk950_dn17,)
    }
};
        locals.var_td__blk950 = assign29220_e41306;
        locals.var_td__blk950_dn0 = assign29220_e41306_d_n0;
        locals.var_td__blk950_dn2 = assign29220_e41306_d_n2;
        locals.var_td__blk950_dn6 = assign29220_e41306_d_n6;
        locals.var_td__blk950_dn7 = assign29220_e41306_d_n7;
        locals.var_td__blk950_dn10 = assign29220_e41306_d_n10;
        locals.var_td__blk950_dn11 = assign29220_e41306_d_n11;
        locals.var_td__blk950_dn12 = assign29220_e41306_d_n12;
        locals.var_td__blk950_dn17 = assign29220_e41306_d_n17;
        locals.var_td__blk950_rv = 0.0;

        let (assign29230_e41348, assign29230_e41348_d_n0, assign29230_e41348_d_n2, assign29230_e41348_d_n6, assign29230_e41348_d_n7, assign29230_e41348_d_n10, assign29230_e41348_d_n11, assign29230_e41348_d_n12, assign29230_e41348_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign29230_e41320: f64 = (locals.var_tb__blk948 * locals.var_tb__blk948);
        let assign29230_e41322: f64 = (assign29230_e41320 * locals.var_tb__blk948);
        let assign29230_e41325: f64 = (27.0 * locals.var_ta__blk947);
        let assign29230_e41327: f64 = (assign29230_e41325 * locals.var_ta__blk947);
        let assign29230_e41329: f64 = (assign29230_e41327 * locals.var_ta__blk947);
        let assign29230_e41330: f64 = (assign29230_e41322 / assign29230_e41329);
        let assign29230_e41333: f64 = (locals.var_tb__blk948 * locals.var_tc__blk949);
        let assign29230_e41336: f64 = (6.0 * locals.var_ta__blk947);
        let assign29230_e41338: f64 = (assign29230_e41336 * locals.var_ta__blk947);
        let assign29230_e41339: f64 = (assign29230_e41333 / assign29230_e41338);
        let assign29230_e41340: f64 = (assign29230_e41330 - assign29230_e41339);
        let assign29230_e41344: f64 = (2.0 * locals.var_ta__blk947);
        let assign29230_e41345: f64 = (locals.var_td__blk950 / assign29230_e41344);
        let assign29230_e41346: f64 = (assign29230_e41340 + assign29230_e41345);
        (assign29230_e41346, ((-((locals.var_tb__blk948 * locals.var_tc__blk949_dn0) / assign29230_e41338)) + (locals.var_td__blk950_dn0 / assign29230_e41344)), ((-((locals.var_tb__blk948 * locals.var_tc__blk949_dn2) / assign29230_e41338)) + (locals.var_td__blk950_dn2 / assign29230_e41344)), ((-((locals.var_tb__blk948 * locals.var_tc__blk949_dn6) / assign29230_e41338)) + (locals.var_td__blk950_dn6 / assign29230_e41344)), ((-((locals.var_tb__blk948 * locals.var_tc__blk949_dn7) / assign29230_e41338)) + (locals.var_td__blk950_dn7 / assign29230_e41344)), ((-((locals.var_tb__blk948 * locals.var_tc__blk949_dn10) / assign29230_e41338)) + (locals.var_td__blk950_dn10 / assign29230_e41344)), ((-((locals.var_tb__blk948 * locals.var_tc__blk949_dn11) / assign29230_e41338)) + (locals.var_td__blk950_dn11 / assign29230_e41344)), ((-((locals.var_tb__blk948 * locals.var_tc__blk949_dn12) / assign29230_e41338)) + (locals.var_td__blk950_dn12 / assign29230_e41344)), ((-((locals.var_tb__blk948 * locals.var_tc__blk949_dn17) / assign29230_e41338)) + (locals.var_td__blk950_dn17 / assign29230_e41344)),)
    } else {
        (locals.var_tq__blk951, locals.var_tq__blk951_dn0, locals.var_tq__blk951_dn2, locals.var_tq__blk951_dn6, locals.var_tq__blk951_dn7, locals.var_tq__blk951_dn10, locals.var_tq__blk951_dn11, locals.var_tq__blk951_dn12, locals.var_tq__blk951_dn17,)
    }
};
        locals.var_tq__blk951 = assign29230_e41348;
        locals.var_tq__blk951_dn0 = assign29230_e41348_d_n0;
        locals.var_tq__blk951_dn2 = assign29230_e41348_d_n2;
        locals.var_tq__blk951_dn6 = assign29230_e41348_d_n6;
        locals.var_tq__blk951_dn7 = assign29230_e41348_d_n7;
        locals.var_tq__blk951_dn10 = assign29230_e41348_d_n10;
        locals.var_tq__blk951_dn11 = assign29230_e41348_d_n11;
        locals.var_tq__blk951_dn12 = assign29230_e41348_d_n12;
        locals.var_tq__blk951_dn17 = assign29230_e41348_d_n17;
        locals.var_tq__blk951_rv = 0.0;

        let (assign29240_e41376, assign29240_e41376_d_n0, assign29240_e41376_d_n2, assign29240_e41376_d_n6, assign29240_e41376_d_n7, assign29240_e41376_d_n10, assign29240_e41376_d_n11, assign29240_e41376_d_n12, assign29240_e41376_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign29240_e41362: f64 = (3.0 * locals.var_ta__blk947);
        let assign29240_e41364: f64 = (assign29240_e41362 * locals.var_tc__blk949);
        let assign29240_e41367: f64 = (locals.var_tb__blk948 * locals.var_tb__blk948);
        let assign29240_e41368: f64 = (assign29240_e41364 - assign29240_e41367);
        let assign29240_e41371: f64 = (9.0 * locals.var_ta__blk947);
        let assign29240_e41373: f64 = (assign29240_e41371 * locals.var_ta__blk947);
        let assign29240_e41374: f64 = (assign29240_e41368 / assign29240_e41373);
        (assign29240_e41374, ((assign29240_e41362 * locals.var_tc__blk949_dn0) / assign29240_e41373), ((assign29240_e41362 * locals.var_tc__blk949_dn2) / assign29240_e41373), ((assign29240_e41362 * locals.var_tc__blk949_dn6) / assign29240_e41373), ((assign29240_e41362 * locals.var_tc__blk949_dn7) / assign29240_e41373), ((assign29240_e41362 * locals.var_tc__blk949_dn10) / assign29240_e41373), ((assign29240_e41362 * locals.var_tc__blk949_dn11) / assign29240_e41373), ((assign29240_e41362 * locals.var_tc__blk949_dn12) / assign29240_e41373), ((assign29240_e41362 * locals.var_tc__blk949_dn17) / assign29240_e41373),)
    } else {
        (locals.var_tp__blk952, locals.var_tp__blk952_dn0, locals.var_tp__blk952_dn2, locals.var_tp__blk952_dn6, locals.var_tp__blk952_dn7, locals.var_tp__blk952_dn10, locals.var_tp__blk952_dn11, locals.var_tp__blk952_dn12, locals.var_tp__blk952_dn17,)
    }
};
        locals.var_tp__blk952 = assign29240_e41376;
        locals.var_tp__blk952_dn0 = assign29240_e41376_d_n0;
        locals.var_tp__blk952_dn2 = assign29240_e41376_d_n2;
        locals.var_tp__blk952_dn6 = assign29240_e41376_d_n6;
        locals.var_tp__blk952_dn7 = assign29240_e41376_d_n7;
        locals.var_tp__blk952_dn10 = assign29240_e41376_d_n10;
        locals.var_tp__blk952_dn11 = assign29240_e41376_d_n11;
        locals.var_tp__blk952_dn12 = assign29240_e41376_d_n12;
        locals.var_tp__blk952_dn17 = assign29240_e41376_d_n17;
        locals.var_tp__blk952_rv = 0.0;

        let (assign29250_e41399, assign29250_e41399_d_n0, assign29250_e41399_d_n2, assign29250_e41399_d_n6, assign29250_e41399_d_n7, assign29250_e41399_d_n10, assign29250_e41399_d_n11, assign29250_e41399_d_n12, assign29250_e41399_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign29250_e41390: f64 = (locals.var_tq__blk951 * locals.var_tq__blk951);
        let assign29250_e41393: f64 = (locals.var_tp__blk952 * locals.var_tp__blk952);
        let assign29250_e41395: f64 = (assign29250_e41393 * locals.var_tp__blk952);
        let assign29250_e41396: f64 = (assign29250_e41390 + assign29250_e41395);
        let assign29250_e41397: f64 = (assign29250_e41396).sqrt();
        (assign29250_e41397, ((((locals.var_tq__blk951_dn0 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn0)) + ((((locals.var_tp__blk952_dn0 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn0)) * locals.var_tp__blk952) + (assign29250_e41393 * locals.var_tp__blk952_dn0))) / (2.0 * assign29250_e41397)), ((((locals.var_tq__blk951_dn2 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn2)) + ((((locals.var_tp__blk952_dn2 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn2)) * locals.var_tp__blk952) + (assign29250_e41393 * locals.var_tp__blk952_dn2))) / (2.0 * assign29250_e41397)), ((((locals.var_tq__blk951_dn6 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn6)) + ((((locals.var_tp__blk952_dn6 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn6)) * locals.var_tp__blk952) + (assign29250_e41393 * locals.var_tp__blk952_dn6))) / (2.0 * assign29250_e41397)), ((((locals.var_tq__blk951_dn7 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn7)) + ((((locals.var_tp__blk952_dn7 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn7)) * locals.var_tp__blk952) + (assign29250_e41393 * locals.var_tp__blk952_dn7))) / (2.0 * assign29250_e41397)), ((((locals.var_tq__blk951_dn10 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn10)) + ((((locals.var_tp__blk952_dn10 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn10)) * locals.var_tp__blk952) + (assign29250_e41393 * locals.var_tp__blk952_dn10))) / (2.0 * assign29250_e41397)), ((((locals.var_tq__blk951_dn11 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn11)) + ((((locals.var_tp__blk952_dn11 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn11)) * locals.var_tp__blk952) + (assign29250_e41393 * locals.var_tp__blk952_dn11))) / (2.0 * assign29250_e41397)), ((((locals.var_tq__blk951_dn12 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn12)) + ((((locals.var_tp__blk952_dn12 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn12)) * locals.var_tp__blk952) + (assign29250_e41393 * locals.var_tp__blk952_dn12))) / (2.0 * assign29250_e41397)), ((((locals.var_tq__blk951_dn17 * locals.var_tq__blk951) + (locals.var_tq__blk951 * locals.var_tq__blk951_dn17)) + ((((locals.var_tp__blk952_dn17 * locals.var_tp__blk952) + (locals.var_tp__blk952 * locals.var_tp__blk952_dn17)) * locals.var_tp__blk952) + (assign29250_e41393 * locals.var_tp__blk952_dn17))) / (2.0 * assign29250_e41397)),)
    } else {
        (locals.var_t5__blk900, locals.var_t5__blk900_dn0, locals.var_t5__blk900_dn2, locals.var_t5__blk900_dn6, locals.var_t5__blk900_dn7, locals.var_t5__blk900_dn10, locals.var_t5__blk900_dn11, locals.var_t5__blk900_dn12, locals.var_t5__blk900_dn17,)
    }
};
        locals.var_t5__blk900 = assign29250_e41399;
        locals.var_t5__blk900_dn0 = assign29250_e41399_d_n0;
        locals.var_t5__blk900_dn2 = assign29250_e41399_d_n2;
        locals.var_t5__blk900_dn6 = assign29250_e41399_d_n6;
        locals.var_t5__blk900_dn7 = assign29250_e41399_d_n7;
        locals.var_t5__blk900_dn10 = assign29250_e41399_d_n10;
        locals.var_t5__blk900_dn11 = assign29250_e41399_d_n11;
        locals.var_t5__blk900_dn12 = assign29250_e41399_d_n12;
        locals.var_t5__blk900_dn17 = assign29250_e41399_d_n17;
        locals.var_t5__blk900_rv = 0.0;

        let (assign29260_e41418, assign29260_e41418_d_n0, assign29260_e41418_d_n2, assign29260_e41418_d_n6, assign29260_e41418_d_n7, assign29260_e41418_d_n10, assign29260_e41418_d_n11, assign29260_e41418_d_n12, assign29260_e41418_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign29260_e41412: f64 = (-locals.var_tq__blk951);
        let assign29260_e41414: f64 = (assign29260_e41412 + locals.var_t5__blk900);
        let assign29260_e41416: f64 = (assign29260_e41414).powf(0.3333333333333333);
        (assign29260_e41416, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29260_e41414).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn0) + locals.var_t5__blk900_dn0))) } } else { (assign29260_e41416 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn0) + locals.var_t5__blk900_dn0) / assign29260_e41414))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29260_e41414).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn2) + locals.var_t5__blk900_dn2))) } } else { (assign29260_e41416 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn2) + locals.var_t5__blk900_dn2) / assign29260_e41414))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29260_e41414).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn6) + locals.var_t5__blk900_dn6))) } } else { (assign29260_e41416 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn6) + locals.var_t5__blk900_dn6) / assign29260_e41414))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29260_e41414).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn7) + locals.var_t5__blk900_dn7))) } } else { (assign29260_e41416 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn7) + locals.var_t5__blk900_dn7) / assign29260_e41414))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29260_e41414).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn10) + locals.var_t5__blk900_dn10))) } } else { (assign29260_e41416 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn10) + locals.var_t5__blk900_dn10) / assign29260_e41414))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29260_e41414).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn11) + locals.var_t5__blk900_dn11))) } } else { (assign29260_e41416 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn11) + locals.var_t5__blk900_dn11) / assign29260_e41414))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29260_e41414).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn12) + locals.var_t5__blk900_dn12))) } } else { (assign29260_e41416 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn12) + locals.var_t5__blk900_dn12) / assign29260_e41414))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29260_e41414).powf(0.3333333333333333 - 1.0) * ((-locals.var_tq__blk951_dn17) + locals.var_t5__blk900_dn17))) } } else { (assign29260_e41416 * (0.3333333333333333 * (((-locals.var_tq__blk951_dn17) + locals.var_t5__blk900_dn17) / assign29260_e41414))) },)
    } else {
        (locals.var_tu__blk953, locals.var_tu__blk953_dn0, locals.var_tu__blk953_dn2, locals.var_tu__blk953_dn6, locals.var_tu__blk953_dn7, locals.var_tu__blk953_dn10, locals.var_tu__blk953_dn11, locals.var_tu__blk953_dn12, locals.var_tu__blk953_dn17,)
    }
};
        locals.var_tu__blk953 = assign29260_e41418;
        locals.var_tu__blk953_dn0 = assign29260_e41418_d_n0;
        locals.var_tu__blk953_dn2 = assign29260_e41418_d_n2;
        locals.var_tu__blk953_dn6 = assign29260_e41418_d_n6;
        locals.var_tu__blk953_dn7 = assign29260_e41418_d_n7;
        locals.var_tu__blk953_dn10 = assign29260_e41418_d_n10;
        locals.var_tu__blk953_dn11 = assign29260_e41418_d_n11;
        locals.var_tu__blk953_dn12 = assign29260_e41418_d_n12;
        locals.var_tu__blk953_dn17 = assign29260_e41418_d_n17;
        locals.var_tu__blk953_rv = 0.0;

        let (assign29270_e41437, assign29270_e41437_d_n0, assign29270_e41437_d_n2, assign29270_e41437_d_n6, assign29270_e41437_d_n7, assign29270_e41437_d_n10, assign29270_e41437_d_n11, assign29270_e41437_d_n12, assign29270_e41437_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign29270_e41432: f64 = (locals.var_tq__blk951 + locals.var_t5__blk900);
        let assign29270_e41434: f64 = (assign29270_e41432).powf(0.3333333333333333);
        let assign29270_e41435: f64 = (-assign29270_e41434);
        (assign29270_e41435, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29270_e41432).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn0 + locals.var_t5__blk900_dn0))) } } else { (assign29270_e41434 * (0.3333333333333333 * ((locals.var_tq__blk951_dn0 + locals.var_t5__blk900_dn0) / assign29270_e41432))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29270_e41432).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn2 + locals.var_t5__blk900_dn2))) } } else { (assign29270_e41434 * (0.3333333333333333 * ((locals.var_tq__blk951_dn2 + locals.var_t5__blk900_dn2) / assign29270_e41432))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29270_e41432).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn6 + locals.var_t5__blk900_dn6))) } } else { (assign29270_e41434 * (0.3333333333333333 * ((locals.var_tq__blk951_dn6 + locals.var_t5__blk900_dn6) / assign29270_e41432))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29270_e41432).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn7 + locals.var_t5__blk900_dn7))) } } else { (assign29270_e41434 * (0.3333333333333333 * ((locals.var_tq__blk951_dn7 + locals.var_t5__blk900_dn7) / assign29270_e41432))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29270_e41432).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn10 + locals.var_t5__blk900_dn10))) } } else { (assign29270_e41434 * (0.3333333333333333 * ((locals.var_tq__blk951_dn10 + locals.var_t5__blk900_dn10) / assign29270_e41432))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29270_e41432).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn11 + locals.var_t5__blk900_dn11))) } } else { (assign29270_e41434 * (0.3333333333333333 * ((locals.var_tq__blk951_dn11 + locals.var_t5__blk900_dn11) / assign29270_e41432))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29270_e41432).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn12 + locals.var_t5__blk900_dn12))) } } else { (assign29270_e41434 * (0.3333333333333333 * ((locals.var_tq__blk951_dn12 + locals.var_t5__blk900_dn12) / assign29270_e41432))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign29270_e41432).powf(0.3333333333333333 - 1.0) * (locals.var_tq__blk951_dn17 + locals.var_t5__blk900_dn17))) } } else { (assign29270_e41434 * (0.3333333333333333 * ((locals.var_tq__blk951_dn17 + locals.var_t5__blk900_dn17) / assign29270_e41432))) }),)
    } else {
        (locals.var_tv__blk954, locals.var_tv__blk954_dn0, locals.var_tv__blk954_dn2, locals.var_tv__blk954_dn6, locals.var_tv__blk954_dn7, locals.var_tv__blk954_dn10, locals.var_tv__blk954_dn11, locals.var_tv__blk954_dn12, locals.var_tv__blk954_dn17,)
    }
};
        locals.var_tv__blk954 = assign29270_e41437;
        locals.var_tv__blk954_dn0 = assign29270_e41437_d_n0;
        locals.var_tv__blk954_dn2 = assign29270_e41437_d_n2;
        locals.var_tv__blk954_dn6 = assign29270_e41437_d_n6;
        locals.var_tv__blk954_dn7 = assign29270_e41437_d_n7;
        locals.var_tv__blk954_dn10 = assign29270_e41437_d_n10;
        locals.var_tv__blk954_dn11 = assign29270_e41437_d_n11;
        locals.var_tv__blk954_dn12 = assign29270_e41437_d_n12;
        locals.var_tv__blk954_dn17 = assign29270_e41437_d_n17;
        locals.var_tv__blk954_rv = 0.0;

        let (assign29280_e41459, assign29280_e41459_d_n0, assign29280_e41459_d_n2, assign29280_e41459_d_n6, assign29280_e41459_d_n7, assign29280_e41459_d_n10, assign29280_e41459_d_n11, assign29280_e41459_d_n12, assign29280_e41459_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign29280_e41451: f64 = (locals.var_tu__blk953 + locals.var_tv__blk954);
        let assign29280_e41455: f64 = (3.0 * locals.var_ta__blk947);
        let assign29280_e41456: f64 = (locals.var_tb__blk948 / assign29280_e41455);
        let assign29280_e41457: f64 = (assign29280_e41451 - assign29280_e41456);
        (assign29280_e41457, (locals.var_tu__blk953_dn0 + locals.var_tv__blk954_dn0), (locals.var_tu__blk953_dn2 + locals.var_tv__blk954_dn2), (locals.var_tu__blk953_dn6 + locals.var_tv__blk954_dn6), (locals.var_tu__blk953_dn7 + locals.var_tv__blk954_dn7), (locals.var_tu__blk953_dn10 + locals.var_tv__blk954_dn10), (locals.var_tu__blk953_dn11 + locals.var_tv__blk954_dn11), (locals.var_tu__blk953_dn12 + locals.var_tv__blk954_dn12), (locals.var_tu__blk953_dn17 + locals.var_tv__blk954_dn17),)
    } else {
        (locals.var_tx__blk904, locals.var_tx__blk904_dn0, locals.var_tx__blk904_dn2, locals.var_tx__blk904_dn6, locals.var_tx__blk904_dn7, locals.var_tx__blk904_dn10, locals.var_tx__blk904_dn11, locals.var_tx__blk904_dn12, locals.var_tx__blk904_dn17,)
    }
};
        locals.var_tx__blk904 = assign29280_e41459;
        locals.var_tx__blk904_dn0 = assign29280_e41459_d_n0;
        locals.var_tx__blk904_dn2 = assign29280_e41459_d_n2;
        locals.var_tx__blk904_dn6 = assign29280_e41459_d_n6;
        locals.var_tx__blk904_dn7 = assign29280_e41459_d_n7;
        locals.var_tx__blk904_dn10 = assign29280_e41459_d_n10;
        locals.var_tx__blk904_dn11 = assign29280_e41459_d_n11;
        locals.var_tx__blk904_dn12 = assign29280_e41459_d_n12;
        locals.var_tx__blk904_dn17 = assign29280_e41459_d_n17;
        locals.var_tx__blk904_rv = 0.0;

        let (assign29290_e41477, assign29290_e41477_d_n0, assign29290_e41477_d_n2, assign29290_e41477_d_n6, assign29290_e41477_d_n7, assign29290_e41477_d_n10, assign29290_e41477_d_n11, assign29290_e41477_d_n12, assign29290_e41477_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign29290_e41473: f64 = (locals.var_tx__blk904 * locals.var_beta_inv);
        let assign29290_e41475: f64 = (assign29290_e41473 - locals.var_vxbgmtcl__blk921);
        (assign29290_e41475, ((locals.var_tx__blk904_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn0), ((locals.var_tx__blk904_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn2), ((locals.var_tx__blk904_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn6), ((locals.var_tx__blk904_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn7), (((locals.var_tx__blk904_dn10 * locals.var_beta_inv) + (locals.var_tx__blk904 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl__blk921_dn10), ((locals.var_tx__blk904_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn11), ((locals.var_tx__blk904_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn12), ((locals.var_tx__blk904_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_ps0_inia__blk946, locals.var_ps0_inia__blk946_dn0, locals.var_ps0_inia__blk946_dn2, locals.var_ps0_inia__blk946_dn6, locals.var_ps0_inia__blk946_dn7, locals.var_ps0_inia__blk946_dn10, locals.var_ps0_inia__blk946_dn11, locals.var_ps0_inia__blk946_dn12, locals.var_ps0_inia__blk946_dn17,)
    }
};
        locals.var_ps0_inia__blk946 = assign29290_e41477;
        locals.var_ps0_inia__blk946_dn0 = assign29290_e41477_d_n0;
        locals.var_ps0_inia__blk946_dn2 = assign29290_e41477_d_n2;
        locals.var_ps0_inia__blk946_dn6 = assign29290_e41477_d_n6;
        locals.var_ps0_inia__blk946_dn7 = assign29290_e41477_d_n7;
        locals.var_ps0_inia__blk946_dn10 = assign29290_e41477_d_n10;
        locals.var_ps0_inia__blk946_dn11 = assign29290_e41477_d_n11;
        locals.var_ps0_inia__blk946_dn12 = assign29290_e41477_d_n12;
        locals.var_ps0_inia__blk946_dn17 = assign29290_e41477_d_n17;
        locals.var_ps0_inia__blk946_rv = 0.0;

        let (assign29300_e41495, assign29300_e41495_d_n0, assign29300_e41495_d_n2, assign29300_e41495_d_n6, assign29300_e41495_d_n7, assign29300_e41495_d_n10, assign29300_e41495_d_n11, assign29300_e41495_d_n12, assign29300_e41495_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard986 != 0.0)) {
        let assign29300_e41492: f64 = (locals.var_ps0_inia__blk946 + locals.var_vxbgmtcl__blk921);
        let assign29300_e41493: f64 = (locals.var_beta * assign29300_e41492);
        (assign29300_e41493, (locals.var_beta * (locals.var_ps0_inia__blk946_dn0 + locals.var_vxbgmtcl__blk921_dn0)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn2 + locals.var_vxbgmtcl__blk921_dn2)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn6 + locals.var_vxbgmtcl__blk921_dn6)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn7 + locals.var_vxbgmtcl__blk921_dn7)), ((locals.var_beta_dn10 * assign29300_e41492) + (locals.var_beta * (locals.var_ps0_inia__blk946_dn10 + locals.var_vxbgmtcl__blk921_dn10))), (locals.var_beta * (locals.var_ps0_inia__blk946_dn11 + locals.var_vxbgmtcl__blk921_dn11)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn12 + locals.var_vxbgmtcl__blk921_dn12)), (locals.var_beta * (locals.var_ps0_inia__blk946_dn17 + locals.var_vxbgmtcl__blk921_dn17)),)
    } else {
        (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    }
};
        locals.var_chi__blk943 = assign29300_e41495;
        locals.var_chi__blk943_dn0 = assign29300_e41495_d_n0;
        locals.var_chi__blk943_dn2 = assign29300_e41495_d_n2;
        locals.var_chi__blk943_dn6 = assign29300_e41495_d_n6;
        locals.var_chi__blk943_dn7 = assign29300_e41495_d_n7;
        locals.var_chi__blk943_dn10 = assign29300_e41495_d_n10;
        locals.var_chi__blk943_dn11 = assign29300_e41495_d_n11;
        locals.var_chi__blk943_dn12 = assign29300_e41495_d_n12;
        locals.var_chi__blk943_dn17 = assign29300_e41495_d_n17;
        locals.var_chi__blk943_rv = 0.0;

        let assign29310_e41498: f64 = if p.p41 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard987 = assign29310_e41498;
        locals.var_guard987_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_106(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29330_e41532, assign29330_e41532_d_n0, assign29330_e41532_d_n2, assign29330_e41532_d_n6, assign29330_e41532_d_n7, assign29330_e41532_d_n10, assign29330_e41532_d_n11, assign29330_e41532_d_n12, assign29330_e41532_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29330_e41528: f64 = (locals.var_vgpld__blk931 + locals.var_vxbgmtcl__blk921);
        let assign29330_e41530: f64 = (assign29330_e41528 + 0.1);
        (assign29330_e41530, (locals.var_vgpld__blk931_dn0 + locals.var_vxbgmtcl__blk921_dn0), (locals.var_vgpld__blk931_dn2 + locals.var_vxbgmtcl__blk921_dn2), (locals.var_vgpld__blk931_dn6 + locals.var_vxbgmtcl__blk921_dn6), (locals.var_vgpld__blk931_dn7 + locals.var_vxbgmtcl__blk921_dn7), (locals.var_vgpld__blk931_dn10 + locals.var_vxbgmtcl__blk921_dn10), (locals.var_vgpld__blk931_dn11 + locals.var_vxbgmtcl__blk921_dn11), (locals.var_vgpld__blk931_dn12 + locals.var_vxbgmtcl__blk921_dn12), (locals.var_vgpld__blk931_dn17 + locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_vgpld_shift__blk955, locals.var_vgpld_shift__blk955_dn0, locals.var_vgpld_shift__blk955_dn2, locals.var_vgpld_shift__blk955_dn6, locals.var_vgpld_shift__blk955_dn7, locals.var_vgpld_shift__blk955_dn10, locals.var_vgpld_shift__blk955_dn11, locals.var_vgpld_shift__blk955_dn12, locals.var_vgpld_shift__blk955_dn17,)
    }
};
        locals.var_vgpld_shift__blk955 = assign29330_e41532;
        locals.var_vgpld_shift__blk955_dn0 = assign29330_e41532_d_n0;
        locals.var_vgpld_shift__blk955_dn2 = assign29330_e41532_d_n2;
        locals.var_vgpld_shift__blk955_dn6 = assign29330_e41532_d_n6;
        locals.var_vgpld_shift__blk955_dn7 = assign29330_e41532_d_n7;
        locals.var_vgpld_shift__blk955_dn10 = assign29330_e41532_d_n10;
        locals.var_vgpld_shift__blk955_dn11 = assign29330_e41532_d_n11;
        locals.var_vgpld_shift__blk955_dn12 = assign29330_e41532_d_n12;
        locals.var_vgpld_shift__blk955_dn17 = assign29330_e41532_d_n17;
        locals.var_vgpld_shift__blk955_rv = 0.0;

        let (assign29340_e41552, assign29340_e41552_d_n0, assign29340_e41552_d_n2, assign29340_e41552_d_n6, assign29340_e41552_d_n7, assign29340_e41552_d_n10, assign29340_e41552_d_n11, assign29340_e41552_d_n12, assign29340_e41552_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29340_e41546: f64 = (-locals.var_vxbgmtcl__blk921);
        let assign29340_e41547: f64 = (locals.var_beta * assign29340_e41546);
        let assign29340_e41548: f64 = (assign29340_e41547).exp();
        let assign29340_e41550: f64 = (assign29340_e41548 + 1e-50);
        (assign29340_e41550, (assign29340_e41548 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn0))), (assign29340_e41548 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn2))), (assign29340_e41548 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn6))), (assign29340_e41548 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn7))), (assign29340_e41548 * ((locals.var_beta_dn10 * assign29340_e41546) + (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn10)))), (assign29340_e41548 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn11))), (assign29340_e41548 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn12))), (assign29340_e41548 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk962, locals.var_exp_bvbs__blk962_dn0, locals.var_exp_bvbs__blk962_dn2, locals.var_exp_bvbs__blk962_dn6, locals.var_exp_bvbs__blk962_dn7, locals.var_exp_bvbs__blk962_dn10, locals.var_exp_bvbs__blk962_dn11, locals.var_exp_bvbs__blk962_dn12, locals.var_exp_bvbs__blk962_dn17,)
    }
};
        locals.var_exp_bvbs__blk962 = assign29340_e41552;
        locals.var_exp_bvbs__blk962_dn0 = assign29340_e41552_d_n0;
        locals.var_exp_bvbs__blk962_dn2 = assign29340_e41552_d_n2;
        locals.var_exp_bvbs__blk962_dn6 = assign29340_e41552_d_n6;
        locals.var_exp_bvbs__blk962_dn7 = assign29340_e41552_d_n7;
        locals.var_exp_bvbs__blk962_dn10 = assign29340_e41552_d_n10;
        locals.var_exp_bvbs__blk962_dn11 = assign29340_e41552_d_n11;
        locals.var_exp_bvbs__blk962_dn12 = assign29340_e41552_d_n12;
        locals.var_exp_bvbs__blk962_dn17 = assign29340_e41552_d_n17;
        locals.var_exp_bvbs__blk962_rv = 0.0;

        let (assign29350_e41568, assign29350_e41568_d_n0, assign29350_e41568_d_n2, assign29350_e41568_d_n6, assign29350_e41568_d_n7, assign29350_e41568_d_n10, assign29350_e41568_d_n11, assign29350_e41568_d_n12, assign29350_e41568_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29350_e41566: f64 = (locals.var_nin / locals.var_mks_nover);
        (assign29350_e41566, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn7 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover), (locals.var_nin_dn17 / locals.var_mks_nover),)
    } else {
        (locals.var_t0__blk895, locals.var_t0__blk895_dn0, locals.var_t0__blk895_dn2, locals.var_t0__blk895_dn6, locals.var_t0__blk895_dn7, locals.var_t0__blk895_dn10, locals.var_t0__blk895_dn11, locals.var_t0__blk895_dn12, locals.var_t0__blk895_dn17,)
    }
};
        locals.var_t0__blk895 = assign29350_e41568;
        locals.var_t0__blk895_dn0 = assign29350_e41568_d_n0;
        locals.var_t0__blk895_dn2 = assign29350_e41568_d_n2;
        locals.var_t0__blk895_dn6 = assign29350_e41568_d_n6;
        locals.var_t0__blk895_dn7 = assign29350_e41568_d_n7;
        locals.var_t0__blk895_dn10 = assign29350_e41568_d_n10;
        locals.var_t0__blk895_dn11 = assign29350_e41568_d_n11;
        locals.var_t0__blk895_dn12 = assign29350_e41568_d_n12;
        locals.var_t0__blk895_dn17 = assign29350_e41568_d_n17;
        locals.var_t0__blk895_rv = 0.0;

        let (assign29360_e41584, assign29360_e41584_d_n0, assign29360_e41584_d_n2, assign29360_e41584_d_n6, assign29360_e41584_d_n7, assign29360_e41584_d_n10, assign29360_e41584_d_n11, assign29360_e41584_d_n12, assign29360_e41584_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29360_e41582: f64 = (locals.var_t0__blk895 * locals.var_t0__blk895);
        (assign29360_e41582, ((locals.var_t0__blk895_dn0 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn0)), ((locals.var_t0__blk895_dn2 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn2)), ((locals.var_t0__blk895_dn6 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn6)), ((locals.var_t0__blk895_dn7 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn7)), ((locals.var_t0__blk895_dn10 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn10)), ((locals.var_t0__blk895_dn11 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn11)), ((locals.var_t0__blk895_dn12 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn12)), ((locals.var_t0__blk895_dn17 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn17)),)
    } else {
        (locals.var_cnst1over__blk956, locals.var_cnst1over__blk956_dn0, locals.var_cnst1over__blk956_dn2, locals.var_cnst1over__blk956_dn6, locals.var_cnst1over__blk956_dn7, locals.var_cnst1over__blk956_dn10, locals.var_cnst1over__blk956_dn11, locals.var_cnst1over__blk956_dn12, locals.var_cnst1over__blk956_dn17,)
    }
};
        locals.var_cnst1over__blk956 = assign29360_e41584;
        locals.var_cnst1over__blk956_dn0 = assign29360_e41584_d_n0;
        locals.var_cnst1over__blk956_dn2 = assign29360_e41584_d_n2;
        locals.var_cnst1over__blk956_dn6 = assign29360_e41584_d_n6;
        locals.var_cnst1over__blk956_dn7 = assign29360_e41584_d_n7;
        locals.var_cnst1over__blk956_dn10 = assign29360_e41584_d_n10;
        locals.var_cnst1over__blk956_dn11 = assign29360_e41584_d_n11;
        locals.var_cnst1over__blk956_dn12 = assign29360_e41584_d_n12;
        locals.var_cnst1over__blk956_dn17 = assign29360_e41584_d_n17;
        locals.var_cnst1over__blk956_rv = 0.0;

        let (assign29370_e41600, assign29370_e41600_d_n0, assign29370_e41600_d_n2, assign29370_e41600_d_n6, assign29370_e41600_d_n7, assign29370_e41600_d_n10, assign29370_e41600_d_n11, assign29370_e41600_d_n12, assign29370_e41600_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29370_e41598: f64 = (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962);
        (assign29370_e41598, ((locals.var_cnst1over__blk956_dn0 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn0)), ((locals.var_cnst1over__blk956_dn2 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn2)), ((locals.var_cnst1over__blk956_dn6 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn6)), ((locals.var_cnst1over__blk956_dn7 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn7)), ((locals.var_cnst1over__blk956_dn10 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn10)), ((locals.var_cnst1over__blk956_dn11 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn11)), ((locals.var_cnst1over__blk956_dn12 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn12)), ((locals.var_cnst1over__blk956_dn17 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn17)),)
    } else {
        (locals.var_gammachi__blk957, locals.var_gammachi__blk957_dn0, locals.var_gammachi__blk957_dn2, locals.var_gammachi__blk957_dn6, locals.var_gammachi__blk957_dn7, locals.var_gammachi__blk957_dn10, locals.var_gammachi__blk957_dn11, locals.var_gammachi__blk957_dn12, locals.var_gammachi__blk957_dn17,)
    }
};
        locals.var_gammachi__blk957 = assign29370_e41600;
        locals.var_gammachi__blk957_dn0 = assign29370_e41600_d_n0;
        locals.var_gammachi__blk957_dn2 = assign29370_e41600_d_n2;
        locals.var_gammachi__blk957_dn6 = assign29370_e41600_d_n6;
        locals.var_gammachi__blk957_dn7 = assign29370_e41600_d_n7;
        locals.var_gammachi__blk957_dn10 = assign29370_e41600_d_n10;
        locals.var_gammachi__blk957_dn11 = assign29370_e41600_d_n11;
        locals.var_gammachi__blk957_dn12 = assign29370_e41600_d_n12;
        locals.var_gammachi__blk957_dn17 = assign29370_e41600_d_n17;
        locals.var_gammachi__blk957_rv = 0.0;

        let (assign29380_e41616, assign29380_e41616_d_n0, assign29380_e41616_d_n2, assign29380_e41616_d_n6, assign29380_e41616_d_n7, assign29380_e41616_d_n10, assign29380_e41616_d_n11, assign29380_e41616_d_n12, assign29380_e41616_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29380_e41614: f64 = (locals.var_beta2 * locals.var_fac1p2__blk930);
        (assign29380_e41614, (locals.var_beta2 * locals.var_fac1p2__blk930_dn0), (locals.var_beta2 * locals.var_fac1p2__blk930_dn2), (locals.var_beta2 * locals.var_fac1p2__blk930_dn6), (locals.var_beta2 * locals.var_fac1p2__blk930_dn7), ((locals.var_beta2_dn10 * locals.var_fac1p2__blk930) + (locals.var_beta2 * locals.var_fac1p2__blk930_dn10)), (locals.var_beta2 * locals.var_fac1p2__blk930_dn11), (locals.var_beta2 * locals.var_fac1p2__blk930_dn12), (locals.var_beta2 * locals.var_fac1p2__blk930_dn17),)
    } else {
        (locals.var_t0__blk895, locals.var_t0__blk895_dn0, locals.var_t0__blk895_dn2, locals.var_t0__blk895_dn6, locals.var_t0__blk895_dn7, locals.var_t0__blk895_dn10, locals.var_t0__blk895_dn11, locals.var_t0__blk895_dn12, locals.var_t0__blk895_dn17,)
    }
};
        locals.var_t0__blk895 = assign29380_e41616;
        locals.var_t0__blk895_dn0 = assign29380_e41616_d_n0;
        locals.var_t0__blk895_dn2 = assign29380_e41616_d_n2;
        locals.var_t0__blk895_dn6 = assign29380_e41616_d_n6;
        locals.var_t0__blk895_dn7 = assign29380_e41616_d_n7;
        locals.var_t0__blk895_dn10 = assign29380_e41616_d_n10;
        locals.var_t0__blk895_dn11 = assign29380_e41616_d_n11;
        locals.var_t0__blk895_dn12 = assign29380_e41616_d_n12;
        locals.var_t0__blk895_dn17 = assign29380_e41616_d_n17;
        locals.var_t0__blk895_rv = 0.0;

        let (assign29390_e41632, assign29390_e41632_d_n0, assign29390_e41632_d_n2, assign29390_e41632_d_n6, assign29390_e41632_d_n7, assign29390_e41632_d_n10, assign29390_e41632_d_n11, assign29390_e41632_d_n12, assign29390_e41632_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29390_e41630: f64 = (locals.var_beta * locals.var_vgpld_shift__blk955);
        (assign29390_e41630, (locals.var_beta * locals.var_vgpld_shift__blk955_dn0), (locals.var_beta * locals.var_vgpld_shift__blk955_dn2), (locals.var_beta * locals.var_vgpld_shift__blk955_dn6), (locals.var_beta * locals.var_vgpld_shift__blk955_dn7), ((locals.var_beta_dn10 * locals.var_vgpld_shift__blk955) + (locals.var_beta * locals.var_vgpld_shift__blk955_dn10)), (locals.var_beta * locals.var_vgpld_shift__blk955_dn11), (locals.var_beta * locals.var_vgpld_shift__blk955_dn12), (locals.var_beta * locals.var_vgpld_shift__blk955_dn17),)
    } else {
        (locals.var_psi__blk958, locals.var_psi__blk958_dn0, locals.var_psi__blk958_dn2, locals.var_psi__blk958_dn6, locals.var_psi__blk958_dn7, locals.var_psi__blk958_dn10, locals.var_psi__blk958_dn11, locals.var_psi__blk958_dn12, locals.var_psi__blk958_dn17,)
    }
};
        locals.var_psi__blk958 = assign29390_e41632;
        locals.var_psi__blk958_dn0 = assign29390_e41632_d_n0;
        locals.var_psi__blk958_dn2 = assign29390_e41632_d_n2;
        locals.var_psi__blk958_dn6 = assign29390_e41632_d_n6;
        locals.var_psi__blk958_dn7 = assign29390_e41632_d_n7;
        locals.var_psi__blk958_dn10 = assign29390_e41632_d_n10;
        locals.var_psi__blk958_dn11 = assign29390_e41632_d_n11;
        locals.var_psi__blk958_dn12 = assign29390_e41632_d_n12;
        locals.var_psi__blk958_dn17 = assign29390_e41632_d_n17;
        locals.var_psi__blk958_rv = 0.0;

        let (assign29400_e41662, assign29400_e41662_d_n0, assign29400_e41662_d_n2, assign29400_e41662_d_n6, assign29400_e41662_d_n7, assign29400_e41662_d_n10, assign29400_e41662_d_n11, assign29400_e41662_d_n12, assign29400_e41662_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29400_e41646: f64 = (locals.var_gammachi__blk957 * locals.var_t0__blk895);
        let assign29400_e41649: f64 = (locals.var_psi__blk958 * locals.var_psi__blk958);
        let assign29400_e41650: f64 = (assign29400_e41646 + assign29400_e41649);
        let assign29400_e41651: f64 = (assign29400_e41650).ln();
        let assign29400_e41654: f64 = (locals.var_cnst1over__blk956 * locals.var_t0__blk895);
        let assign29400_e41655: f64 = (assign29400_e41654).ln();
        let assign29400_e41656: f64 = (assign29400_e41651 - assign29400_e41655);
        let assign29400_e41659: f64 = (locals.var_beta * locals.var_vxbgmtcl__blk921);
        let assign29400_e41660: f64 = (assign29400_e41656 + assign29400_e41659);
        (assign29400_e41660, ((((((locals.var_gammachi__blk957_dn0 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn0)) + ((locals.var_psi__blk958_dn0 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn0))) / assign29400_e41650) - (((locals.var_cnst1over__blk956_dn0 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn0)) / assign29400_e41654)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn0)), ((((((locals.var_gammachi__blk957_dn2 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn2)) + ((locals.var_psi__blk958_dn2 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn2))) / assign29400_e41650) - (((locals.var_cnst1over__blk956_dn2 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn2)) / assign29400_e41654)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn2)), ((((((locals.var_gammachi__blk957_dn6 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn6)) + ((locals.var_psi__blk958_dn6 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn6))) / assign29400_e41650) - (((locals.var_cnst1over__blk956_dn6 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn6)) / assign29400_e41654)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn6)), ((((((locals.var_gammachi__blk957_dn7 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn7)) + ((locals.var_psi__blk958_dn7 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn7))) / assign29400_e41650) - (((locals.var_cnst1over__blk956_dn7 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn7)) / assign29400_e41654)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn7)), ((((((locals.var_gammachi__blk957_dn10 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn10)) + ((locals.var_psi__blk958_dn10 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn10))) / assign29400_e41650) - (((locals.var_cnst1over__blk956_dn10 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn10)) / assign29400_e41654)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl__blk921) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn10))), ((((((locals.var_gammachi__blk957_dn11 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn11)) + ((locals.var_psi__blk958_dn11 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn11))) / assign29400_e41650) - (((locals.var_cnst1over__blk956_dn11 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn11)) / assign29400_e41654)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn11)), ((((((locals.var_gammachi__blk957_dn12 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn12)) + ((locals.var_psi__blk958_dn12 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn12))) / assign29400_e41650) - (((locals.var_cnst1over__blk956_dn12 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn12)) / assign29400_e41654)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn12)), ((((((locals.var_gammachi__blk957_dn17 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn17)) + ((locals.var_psi__blk958_dn17 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn17))) / assign29400_e41650) - (((locals.var_cnst1over__blk956_dn17 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn17)) / assign29400_e41654)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn17)),)
    } else {
        (locals.var_chi_1__blk959, locals.var_chi_1__blk959_dn0, locals.var_chi_1__blk959_dn2, locals.var_chi_1__blk959_dn6, locals.var_chi_1__blk959_dn7, locals.var_chi_1__blk959_dn10, locals.var_chi_1__blk959_dn11, locals.var_chi_1__blk959_dn12, locals.var_chi_1__blk959_dn17,)
    }
};
        locals.var_chi_1__blk959 = assign29400_e41662;
        locals.var_chi_1__blk959_dn0 = assign29400_e41662_d_n0;
        locals.var_chi_1__blk959_dn2 = assign29400_e41662_d_n2;
        locals.var_chi_1__blk959_dn6 = assign29400_e41662_d_n6;
        locals.var_chi_1__blk959_dn7 = assign29400_e41662_d_n7;
        locals.var_chi_1__blk959_dn10 = assign29400_e41662_d_n10;
        locals.var_chi_1__blk959_dn11 = assign29400_e41662_d_n11;
        locals.var_chi_1__blk959_dn12 = assign29400_e41662_d_n12;
        locals.var_chi_1__blk959_dn17 = assign29400_e41662_d_n17;
        locals.var_chi_1__blk959_rv = 0.0;

        let (assign29410_e41680, assign29410_e41680_d_n0, assign29410_e41680_d_n2, assign29410_e41680_d_n6, assign29410_e41680_d_n7, assign29410_e41680_d_n10, assign29410_e41680_d_n11, assign29410_e41680_d_n12, assign29410_e41680_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29410_e41676: f64 = (locals.var_psi__blk958 - locals.var_chi_1__blk959);
        let assign29410_e41678: f64 = (assign29410_e41676 - 1.0);
        (assign29410_e41678, (locals.var_psi__blk958_dn0 - locals.var_chi_1__blk959_dn0), (locals.var_psi__blk958_dn2 - locals.var_chi_1__blk959_dn2), (locals.var_psi__blk958_dn6 - locals.var_chi_1__blk959_dn6), (locals.var_psi__blk958_dn7 - locals.var_chi_1__blk959_dn7), (locals.var_psi__blk958_dn10 - locals.var_chi_1__blk959_dn10), (locals.var_psi__blk958_dn11 - locals.var_chi_1__blk959_dn11), (locals.var_psi__blk958_dn12 - locals.var_chi_1__blk959_dn12), (locals.var_psi__blk958_dn17 - locals.var_chi_1__blk959_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign29410_e41680;
        locals.var_tmf1_dn0 = assign29410_e41680_d_n0;
        locals.var_tmf1_dn2 = assign29410_e41680_d_n2;
        locals.var_tmf1_dn6 = assign29410_e41680_d_n6;
        locals.var_tmf1_dn7 = assign29410_e41680_d_n7;
        locals.var_tmf1_dn10 = assign29410_e41680_d_n10;
        locals.var_tmf1_dn11 = assign29410_e41680_d_n11;
        locals.var_tmf1_dn12 = assign29410_e41680_d_n12;
        locals.var_tmf1_dn17 = assign29410_e41680_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign29420_e41698, assign29420_e41698_d_n0, assign29420_e41698_d_n2, assign29420_e41698_d_n6, assign29420_e41698_d_n7, assign29420_e41698_d_n10, assign29420_e41698_d_n11, assign29420_e41698_d_n12, assign29420_e41698_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29420_e41694: f64 = (4.0 * locals.var_psi__blk958);
        let assign29420_e41696: f64 = assign29420_e41694;
        (assign29420_e41696, (4.0 * locals.var_psi__blk958_dn0), (4.0 * locals.var_psi__blk958_dn2), (4.0 * locals.var_psi__blk958_dn6), (4.0 * locals.var_psi__blk958_dn7), (4.0 * locals.var_psi__blk958_dn10), (4.0 * locals.var_psi__blk958_dn11), (4.0 * locals.var_psi__blk958_dn12), (4.0 * locals.var_psi__blk958_dn17),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign29420_e41698;
        locals.var_tmf2_dn0 = assign29420_e41698_d_n0;
        locals.var_tmf2_dn2 = assign29420_e41698_d_n2;
        locals.var_tmf2_dn6 = assign29420_e41698_d_n6;
        locals.var_tmf2_dn7 = assign29420_e41698_d_n7;
        locals.var_tmf2_dn10 = assign29420_e41698_d_n10;
        locals.var_tmf2_dn11 = assign29420_e41698_d_n11;
        locals.var_tmf2_dn12 = assign29420_e41698_d_n12;
        locals.var_tmf2_dn17 = assign29420_e41698_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign29430_e41718, assign29430_e41718_d_n0, assign29430_e41718_d_n2, assign29430_e41718_d_n6, assign29430_e41718_d_n7, assign29430_e41718_d_n10, assign29430_e41718_d_n11, assign29430_e41718_d_n12, assign29430_e41718_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let (assign29430_e41716, assign29430_e41716_d_n0, assign29430_e41716_d_n2, assign29430_e41716_d_n6, assign29430_e41716_d_n7, assign29430_e41716_d_n10, assign29430_e41716_d_n11, assign29430_e41716_d_n12, assign29430_e41716_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign29430_e41715: f64 = (-locals.var_tmf2);
                (assign29430_e41715, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign29430_e41716, assign29430_e41716_d_n0, assign29430_e41716_d_n2, assign29430_e41716_d_n6, assign29430_e41716_d_n7, assign29430_e41716_d_n10, assign29430_e41716_d_n11, assign29430_e41716_d_n12, assign29430_e41716_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign29430_e41718;
        locals.var_tmf2_dn0 = assign29430_e41718_d_n0;
        locals.var_tmf2_dn2 = assign29430_e41718_d_n2;
        locals.var_tmf2_dn6 = assign29430_e41718_d_n6;
        locals.var_tmf2_dn7 = assign29430_e41718_d_n7;
        locals.var_tmf2_dn10 = assign29430_e41718_d_n10;
        locals.var_tmf2_dn11 = assign29430_e41718_d_n11;
        locals.var_tmf2_dn12 = assign29430_e41718_d_n12;
        locals.var_tmf2_dn17 = assign29430_e41718_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign29440_e41737, assign29440_e41737_d_n0, assign29440_e41737_d_n2, assign29440_e41737_d_n6, assign29440_e41737_d_n7, assign29440_e41737_d_n10, assign29440_e41737_d_n11, assign29440_e41737_d_n12, assign29440_e41737_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29440_e41732: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign29440_e41734: f64 = (assign29440_e41732 + locals.var_tmf2);
        let assign29440_e41735: f64 = (assign29440_e41734).sqrt();
        (assign29440_e41735, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign29440_e41735)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign29440_e41735)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign29440_e41735)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign29440_e41735)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign29440_e41735)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign29440_e41735)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign29440_e41735)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign29440_e41735)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign29440_e41737;
        locals.var_tmf2_dn0 = assign29440_e41737_d_n0;
        locals.var_tmf2_dn2 = assign29440_e41737_d_n2;
        locals.var_tmf2_dn6 = assign29440_e41737_d_n6;
        locals.var_tmf2_dn7 = assign29440_e41737_d_n7;
        locals.var_tmf2_dn10 = assign29440_e41737_d_n10;
        locals.var_tmf2_dn11 = assign29440_e41737_d_n11;
        locals.var_tmf2_dn12 = assign29440_e41737_d_n12;
        locals.var_tmf2_dn17 = assign29440_e41737_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign29450_e41757, assign29450_e41757_d_n0, assign29450_e41757_d_n2, assign29450_e41757_d_n6, assign29450_e41757_d_n7, assign29450_e41757_d_n10, assign29450_e41757_d_n11, assign29450_e41757_d_n12, assign29450_e41757_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29450_e41753: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign29450_e41754: f64 = (1.0 + assign29450_e41753);
        let assign29450_e41755: f64 = (0.5 * assign29450_e41754);
        (assign29450_e41755, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign29450_e41757;
        locals.var_t1__blk896_dn0 = assign29450_e41757_d_n0;
        locals.var_t1__blk896_dn2 = assign29450_e41757_d_n2;
        locals.var_t1__blk896_dn6 = assign29450_e41757_d_n6;
        locals.var_t1__blk896_dn7 = assign29450_e41757_d_n7;
        locals.var_t1__blk896_dn10 = assign29450_e41757_d_n10;
        locals.var_t1__blk896_dn11 = assign29450_e41757_d_n11;
        locals.var_t1__blk896_dn12 = assign29450_e41757_d_n12;
        locals.var_t1__blk896_dn17 = assign29450_e41757_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let (assign29460_e41781, assign29460_e41781_d_n0, assign29460_e41781_d_n2, assign29460_e41781_d_n6, assign29460_e41781_d_n7, assign29460_e41781_d_n10, assign29460_e41781_d_n11, assign29460_e41781_d_n12, assign29460_e41781_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29460_e41774: f64 = 2.0;
        let assign29460_e41775: f64 = (locals.var_tmf1 + assign29460_e41774);
        let assign29460_e41777: f64 = (assign29460_e41775 / locals.var_tmf2);
        let assign29460_e41778: f64 = (1.0 - assign29460_e41777);
        let assign29460_e41779: f64 = (0.5 * assign29460_e41778);
        (assign29460_e41779, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign29460_e41775 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign29460_e41775 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign29460_e41775 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign29460_e41775 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign29460_e41775 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign29460_e41775 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign29460_e41775 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign29460_e41775 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk897, locals.var_t2__blk897_dn0, locals.var_t2__blk897_dn2, locals.var_t2__blk897_dn6, locals.var_t2__blk897_dn7, locals.var_t2__blk897_dn10, locals.var_t2__blk897_dn11, locals.var_t2__blk897_dn12, locals.var_t2__blk897_dn17,)
    }
};
        locals.var_t2__blk897 = assign29460_e41781;
        locals.var_t2__blk897_dn0 = assign29460_e41781_d_n0;
        locals.var_t2__blk897_dn2 = assign29460_e41781_d_n2;
        locals.var_t2__blk897_dn6 = assign29460_e41781_d_n6;
        locals.var_t2__blk897_dn7 = assign29460_e41781_d_n7;
        locals.var_t2__blk897_dn10 = assign29460_e41781_d_n10;
        locals.var_t2__blk897_dn11 = assign29460_e41781_d_n11;
        locals.var_t2__blk897_dn12 = assign29460_e41781_d_n12;
        locals.var_t2__blk897_dn17 = assign29460_e41781_d_n17;
        locals.var_t2__blk897_rv = 0.0;

        let (assign29470_e41801, assign29470_e41801_d_n0, assign29470_e41801_d_n2, assign29470_e41801_d_n6, assign29470_e41801_d_n7, assign29470_e41801_d_n10, assign29470_e41801_d_n11, assign29470_e41801_d_n12, assign29470_e41801_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29470_e41797: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign29470_e41798: f64 = (0.5 * assign29470_e41797);
        let assign29470_e41799: f64 = (locals.var_psi__blk958 - assign29470_e41798);
        (assign29470_e41799, (locals.var_psi__blk958_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psi__blk958_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psi__blk958_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psi__blk958_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_psi__blk958_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psi__blk958_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psi__blk958_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_psi__blk958_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi_1__blk959, locals.var_chi_1__blk959_dn0, locals.var_chi_1__blk959_dn2, locals.var_chi_1__blk959_dn6, locals.var_chi_1__blk959_dn7, locals.var_chi_1__blk959_dn10, locals.var_chi_1__blk959_dn11, locals.var_chi_1__blk959_dn12, locals.var_chi_1__blk959_dn17,)
    }
};
        locals.var_chi_1__blk959 = assign29470_e41801;
        locals.var_chi_1__blk959_dn0 = assign29470_e41801_d_n0;
        locals.var_chi_1__blk959_dn2 = assign29470_e41801_d_n2;
        locals.var_chi_1__blk959_dn6 = assign29470_e41801_d_n6;
        locals.var_chi_1__blk959_dn7 = assign29470_e41801_d_n7;
        locals.var_chi_1__blk959_dn10 = assign29470_e41801_d_n10;
        locals.var_chi_1__blk959_dn11 = assign29470_e41801_d_n11;
        locals.var_chi_1__blk959_dn12 = assign29470_e41801_d_n12;
        locals.var_chi_1__blk959_dn17 = assign29470_e41801_d_n17;
        locals.var_chi_1__blk959_rv = 0.0;

        let (assign29480_e41817, assign29480_e41817_d_n0, assign29480_e41817_d_n2, assign29480_e41817_d_n6, assign29480_e41817_d_n7, assign29480_e41817_d_n10, assign29480_e41817_d_n11, assign29480_e41817_d_n12, assign29480_e41817_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29480_e41815: f64 = (locals.var_psi__blk958 - locals.var_chi_1__blk959);
        (assign29480_e41815, (locals.var_psi__blk958_dn0 - locals.var_chi_1__blk959_dn0), (locals.var_psi__blk958_dn2 - locals.var_chi_1__blk959_dn2), (locals.var_psi__blk958_dn6 - locals.var_chi_1__blk959_dn6), (locals.var_psi__blk958_dn7 - locals.var_chi_1__blk959_dn7), (locals.var_psi__blk958_dn10 - locals.var_chi_1__blk959_dn10), (locals.var_psi__blk958_dn11 - locals.var_chi_1__blk959_dn11), (locals.var_psi__blk958_dn12 - locals.var_chi_1__blk959_dn12), (locals.var_psi__blk958_dn17 - locals.var_chi_1__blk959_dn17),)
    } else {
        (locals.var_psi__blk958, locals.var_psi__blk958_dn0, locals.var_psi__blk958_dn2, locals.var_psi__blk958_dn6, locals.var_psi__blk958_dn7, locals.var_psi__blk958_dn10, locals.var_psi__blk958_dn11, locals.var_psi__blk958_dn12, locals.var_psi__blk958_dn17,)
    }
};
        locals.var_psi__blk958 = assign29480_e41817;
        locals.var_psi__blk958_dn0 = assign29480_e41817_d_n0;
        locals.var_psi__blk958_dn2 = assign29480_e41817_d_n2;
        locals.var_psi__blk958_dn6 = assign29480_e41817_d_n6;
        locals.var_psi__blk958_dn7 = assign29480_e41817_d_n7;
        locals.var_psi__blk958_dn10 = assign29480_e41817_d_n10;
        locals.var_psi__blk958_dn11 = assign29480_e41817_d_n11;
        locals.var_psi__blk958_dn12 = assign29480_e41817_d_n12;
        locals.var_psi__blk958_dn17 = assign29480_e41817_d_n17;
        locals.var_psi__blk958_rv = 0.0;

        let (assign29490_e41835, assign29490_e41835_d_n0, assign29490_e41835_d_n2, assign29490_e41835_d_n6, assign29490_e41835_d_n7, assign29490_e41835_d_n10, assign29490_e41835_d_n11, assign29490_e41835_d_n12, assign29490_e41835_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29490_e41832: f64 = (locals.var_beta * 0.1);
        let assign29490_e41833: f64 = (locals.var_psi__blk958 + assign29490_e41832);
        (assign29490_e41833, locals.var_psi__blk958_dn0, locals.var_psi__blk958_dn2, locals.var_psi__blk958_dn6, locals.var_psi__blk958_dn7, (locals.var_psi__blk958_dn10 + (locals.var_beta_dn10 * 0.1)), locals.var_psi__blk958_dn11, locals.var_psi__blk958_dn12, locals.var_psi__blk958_dn17,)
    } else {
        (locals.var_psi__blk958, locals.var_psi__blk958_dn0, locals.var_psi__blk958_dn2, locals.var_psi__blk958_dn6, locals.var_psi__blk958_dn7, locals.var_psi__blk958_dn10, locals.var_psi__blk958_dn11, locals.var_psi__blk958_dn12, locals.var_psi__blk958_dn17,)
    }
};
        locals.var_psi__blk958 = assign29490_e41835;
        locals.var_psi__blk958_dn0 = assign29490_e41835_d_n0;
        locals.var_psi__blk958_dn2 = assign29490_e41835_d_n2;
        locals.var_psi__blk958_dn6 = assign29490_e41835_d_n6;
        locals.var_psi__blk958_dn7 = assign29490_e41835_d_n7;
        locals.var_psi__blk958_dn10 = assign29490_e41835_d_n10;
        locals.var_psi__blk958_dn11 = assign29490_e41835_d_n11;
        locals.var_psi__blk958_dn12 = assign29490_e41835_d_n12;
        locals.var_psi__blk958_dn17 = assign29490_e41835_d_n17;
        locals.var_psi__blk958_rv = 0.0;

        let (assign29500_e41865, assign29500_e41865_d_n0, assign29500_e41865_d_n2, assign29500_e41865_d_n6, assign29500_e41865_d_n7, assign29500_e41865_d_n10, assign29500_e41865_d_n11, assign29500_e41865_d_n12, assign29500_e41865_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29500_e41849: f64 = (locals.var_gammachi__blk957 * locals.var_t0__blk895);
        let assign29500_e41852: f64 = (locals.var_psi__blk958 * locals.var_psi__blk958);
        let assign29500_e41853: f64 = (assign29500_e41849 + assign29500_e41852);
        let assign29500_e41854: f64 = (assign29500_e41853).ln();
        let assign29500_e41857: f64 = (locals.var_cnst1over__blk956 * locals.var_t0__blk895);
        let assign29500_e41858: f64 = (assign29500_e41857).ln();
        let assign29500_e41859: f64 = (assign29500_e41854 - assign29500_e41858);
        let assign29500_e41862: f64 = (locals.var_beta * locals.var_vxbgmtcl__blk921);
        let assign29500_e41863: f64 = (assign29500_e41859 + assign29500_e41862);
        (assign29500_e41863, ((((((locals.var_gammachi__blk957_dn0 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn0)) + ((locals.var_psi__blk958_dn0 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn0))) / assign29500_e41853) - (((locals.var_cnst1over__blk956_dn0 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn0)) / assign29500_e41857)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn0)), ((((((locals.var_gammachi__blk957_dn2 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn2)) + ((locals.var_psi__blk958_dn2 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn2))) / assign29500_e41853) - (((locals.var_cnst1over__blk956_dn2 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn2)) / assign29500_e41857)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn2)), ((((((locals.var_gammachi__blk957_dn6 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn6)) + ((locals.var_psi__blk958_dn6 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn6))) / assign29500_e41853) - (((locals.var_cnst1over__blk956_dn6 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn6)) / assign29500_e41857)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn6)), ((((((locals.var_gammachi__blk957_dn7 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn7)) + ((locals.var_psi__blk958_dn7 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn7))) / assign29500_e41853) - (((locals.var_cnst1over__blk956_dn7 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn7)) / assign29500_e41857)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn7)), ((((((locals.var_gammachi__blk957_dn10 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn10)) + ((locals.var_psi__blk958_dn10 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn10))) / assign29500_e41853) - (((locals.var_cnst1over__blk956_dn10 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn10)) / assign29500_e41857)) + ((locals.var_beta_dn10 * locals.var_vxbgmtcl__blk921) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn10))), ((((((locals.var_gammachi__blk957_dn11 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn11)) + ((locals.var_psi__blk958_dn11 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn11))) / assign29500_e41853) - (((locals.var_cnst1over__blk956_dn11 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn11)) / assign29500_e41857)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn11)), ((((((locals.var_gammachi__blk957_dn12 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn12)) + ((locals.var_psi__blk958_dn12 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn12))) / assign29500_e41853) - (((locals.var_cnst1over__blk956_dn12 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn12)) / assign29500_e41857)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn12)), ((((((locals.var_gammachi__blk957_dn17 * locals.var_t0__blk895) + (locals.var_gammachi__blk957 * locals.var_t0__blk895_dn17)) + ((locals.var_psi__blk958_dn17 * locals.var_psi__blk958) + (locals.var_psi__blk958 * locals.var_psi__blk958_dn17))) / assign29500_e41853) - (((locals.var_cnst1over__blk956_dn17 * locals.var_t0__blk895) + (locals.var_cnst1over__blk956 * locals.var_t0__blk895_dn17)) / assign29500_e41857)) + (locals.var_beta * locals.var_vxbgmtcl__blk921_dn17)),)
    } else {
        (locals.var_chi_b__blk960, locals.var_chi_b__blk960_dn0, locals.var_chi_b__blk960_dn2, locals.var_chi_b__blk960_dn6, locals.var_chi_b__blk960_dn7, locals.var_chi_b__blk960_dn10, locals.var_chi_b__blk960_dn11, locals.var_chi_b__blk960_dn12, locals.var_chi_b__blk960_dn17,)
    }
};
        locals.var_chi_b__blk960 = assign29500_e41865;
        locals.var_chi_b__blk960_dn0 = assign29500_e41865_d_n0;
        locals.var_chi_b__blk960_dn2 = assign29500_e41865_d_n2;
        locals.var_chi_b__blk960_dn6 = assign29500_e41865_d_n6;
        locals.var_chi_b__blk960_dn7 = assign29500_e41865_d_n7;
        locals.var_chi_b__blk960_dn10 = assign29500_e41865_d_n10;
        locals.var_chi_b__blk960_dn11 = assign29500_e41865_d_n11;
        locals.var_chi_b__blk960_dn12 = assign29500_e41865_d_n12;
        locals.var_chi_b__blk960_dn17 = assign29500_e41865_d_n17;
        locals.var_chi_b__blk960_rv = 0.0;

        let (assign29510_e41879, assign29510_e41879_d_n0, assign29510_e41879_d_n2, assign29510_e41879_d_n6, assign29510_e41879_d_n7, assign29510_e41879_d_n10, assign29510_e41879_d_n11, assign29510_e41879_d_n12, assign29510_e41879_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    } else {
        (locals.var_chi_a__blk961, locals.var_chi_a__blk961_dn0, locals.var_chi_a__blk961_dn2, locals.var_chi_a__blk961_dn6, locals.var_chi_a__blk961_dn7, locals.var_chi_a__blk961_dn10, locals.var_chi_a__blk961_dn11, locals.var_chi_a__blk961_dn12, locals.var_chi_a__blk961_dn17,)
    }
};
        locals.var_chi_a__blk961 = assign29510_e41879;
        locals.var_chi_a__blk961_dn0 = assign29510_e41879_d_n0;
        locals.var_chi_a__blk961_dn2 = assign29510_e41879_d_n2;
        locals.var_chi_a__blk961_dn6 = assign29510_e41879_d_n6;
        locals.var_chi_a__blk961_dn7 = assign29510_e41879_d_n7;
        locals.var_chi_a__blk961_dn10 = assign29510_e41879_d_n10;
        locals.var_chi_a__blk961_dn11 = assign29510_e41879_d_n11;
        locals.var_chi_a__blk961_dn12 = assign29510_e41879_d_n12;
        locals.var_chi_a__blk961_dn17 = assign29510_e41879_d_n17;
        locals.var_chi_a__blk961_rv = 0.0;

        let (assign29520_e41899, assign29520_e41899_d_n0, assign29520_e41899_d_n2, assign29520_e41899_d_n6, assign29520_e41899_d_n7, assign29520_e41899_d_n10, assign29520_e41899_d_n11, assign29520_e41899_d_n12, assign29520_e41899_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29520_e41893: f64 = (locals.var_chi_b__blk960 - locals.var_chi_a__blk961);
        let assign29520_e41896: f64 = (0.0008 * 75.0);
        let assign29520_e41897: f64 = (assign29520_e41893 - assign29520_e41896);
        (assign29520_e41897, (locals.var_chi_b__blk960_dn0 - locals.var_chi_a__blk961_dn0), (locals.var_chi_b__blk960_dn2 - locals.var_chi_a__blk961_dn2), (locals.var_chi_b__blk960_dn6 - locals.var_chi_a__blk961_dn6), (locals.var_chi_b__blk960_dn7 - locals.var_chi_a__blk961_dn7), (locals.var_chi_b__blk960_dn10 - locals.var_chi_a__blk961_dn10), (locals.var_chi_b__blk960_dn11 - locals.var_chi_a__blk961_dn11), (locals.var_chi_b__blk960_dn12 - locals.var_chi_a__blk961_dn12), (locals.var_chi_b__blk960_dn17 - locals.var_chi_a__blk961_dn17),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign29520_e41899;
        locals.var_tmf1_dn0 = assign29520_e41899_d_n0;
        locals.var_tmf1_dn2 = assign29520_e41899_d_n2;
        locals.var_tmf1_dn6 = assign29520_e41899_d_n6;
        locals.var_tmf1_dn7 = assign29520_e41899_d_n7;
        locals.var_tmf1_dn10 = assign29520_e41899_d_n10;
        locals.var_tmf1_dn11 = assign29520_e41899_d_n11;
        locals.var_tmf1_dn12 = assign29520_e41899_d_n12;
        locals.var_tmf1_dn17 = assign29520_e41899_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign29530_e41919, assign29530_e41919_d_n0, assign29530_e41919_d_n2, assign29530_e41919_d_n6, assign29530_e41919_d_n7, assign29530_e41919_d_n10, assign29530_e41919_d_n11, assign29530_e41919_d_n12, assign29530_e41919_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29530_e41913: f64 = (4.0 * locals.var_chi_b__blk960);
        let assign29530_e41916: f64 = (0.0008 * 75.0);
        let assign29530_e41917: f64 = (assign29530_e41913 * assign29530_e41916);
        (assign29530_e41917, ((4.0 * locals.var_chi_b__blk960_dn0) * assign29530_e41916), ((4.0 * locals.var_chi_b__blk960_dn2) * assign29530_e41916), ((4.0 * locals.var_chi_b__blk960_dn6) * assign29530_e41916), ((4.0 * locals.var_chi_b__blk960_dn7) * assign29530_e41916), ((4.0 * locals.var_chi_b__blk960_dn10) * assign29530_e41916), ((4.0 * locals.var_chi_b__blk960_dn11) * assign29530_e41916), ((4.0 * locals.var_chi_b__blk960_dn12) * assign29530_e41916), ((4.0 * locals.var_chi_b__blk960_dn17) * assign29530_e41916),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign29530_e41919;
        locals.var_tmf2_dn0 = assign29530_e41919_d_n0;
        locals.var_tmf2_dn2 = assign29530_e41919_d_n2;
        locals.var_tmf2_dn6 = assign29530_e41919_d_n6;
        locals.var_tmf2_dn7 = assign29530_e41919_d_n7;
        locals.var_tmf2_dn10 = assign29530_e41919_d_n10;
        locals.var_tmf2_dn11 = assign29530_e41919_d_n11;
        locals.var_tmf2_dn12 = assign29530_e41919_d_n12;
        locals.var_tmf2_dn17 = assign29530_e41919_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign29540_e41939, assign29540_e41939_d_n0, assign29540_e41939_d_n2, assign29540_e41939_d_n6, assign29540_e41939_d_n7, assign29540_e41939_d_n10, assign29540_e41939_d_n11, assign29540_e41939_d_n12, assign29540_e41939_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let (assign29540_e41937, assign29540_e41937_d_n0, assign29540_e41937_d_n2, assign29540_e41937_d_n6, assign29540_e41937_d_n7, assign29540_e41937_d_n10, assign29540_e41937_d_n11, assign29540_e41937_d_n12, assign29540_e41937_d_n17,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
            } else {
                let assign29540_e41936: f64 = (-locals.var_tmf2);
                (assign29540_e41936, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12), (-locals.var_tmf2_dn17),)
            }
        };
        (assign29540_e41937, assign29540_e41937_d_n0, assign29540_e41937_d_n2, assign29540_e41937_d_n6, assign29540_e41937_d_n7, assign29540_e41937_d_n10, assign29540_e41937_d_n11, assign29540_e41937_d_n12, assign29540_e41937_d_n17,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign29540_e41939;
        locals.var_tmf2_dn0 = assign29540_e41939_d_n0;
        locals.var_tmf2_dn2 = assign29540_e41939_d_n2;
        locals.var_tmf2_dn6 = assign29540_e41939_d_n6;
        locals.var_tmf2_dn7 = assign29540_e41939_d_n7;
        locals.var_tmf2_dn10 = assign29540_e41939_d_n10;
        locals.var_tmf2_dn11 = assign29540_e41939_d_n11;
        locals.var_tmf2_dn12 = assign29540_e41939_d_n12;
        locals.var_tmf2_dn17 = assign29540_e41939_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign29550_e41958, assign29550_e41958_d_n0, assign29550_e41958_d_n2, assign29550_e41958_d_n6, assign29550_e41958_d_n7, assign29550_e41958_d_n10, assign29550_e41958_d_n11, assign29550_e41958_d_n12, assign29550_e41958_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29550_e41953: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign29550_e41955: f64 = (assign29550_e41953 + locals.var_tmf2);
        let assign29550_e41956: f64 = (assign29550_e41955).sqrt();
        (assign29550_e41956, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign29550_e41956)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign29550_e41956)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign29550_e41956)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign29550_e41956)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign29550_e41956)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign29550_e41956)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign29550_e41956)), ((((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)) + locals.var_tmf2_dn17) / (2.0 * assign29550_e41956)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign29550_e41958;
        locals.var_tmf2_dn0 = assign29550_e41958_d_n0;
        locals.var_tmf2_dn2 = assign29550_e41958_d_n2;
        locals.var_tmf2_dn6 = assign29550_e41958_d_n6;
        locals.var_tmf2_dn7 = assign29550_e41958_d_n7;
        locals.var_tmf2_dn10 = assign29550_e41958_d_n10;
        locals.var_tmf2_dn11 = assign29550_e41958_d_n11;
        locals.var_tmf2_dn12 = assign29550_e41958_d_n12;
        locals.var_tmf2_dn17 = assign29550_e41958_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign29560_e41978, assign29560_e41978_d_n0, assign29560_e41978_d_n2, assign29560_e41978_d_n6, assign29560_e41978_d_n7, assign29560_e41978_d_n10, assign29560_e41978_d_n11, assign29560_e41978_d_n12, assign29560_e41978_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29560_e41974: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign29560_e41975: f64 = (1.0 + assign29560_e41974);
        let assign29560_e41976: f64 = (0.5 * assign29560_e41975);
        (assign29560_e41976, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn17 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign29560_e41978;
        locals.var_t1__blk896_dn0 = assign29560_e41978_d_n0;
        locals.var_t1__blk896_dn2 = assign29560_e41978_d_n2;
        locals.var_t1__blk896_dn6 = assign29560_e41978_d_n6;
        locals.var_t1__blk896_dn7 = assign29560_e41978_d_n7;
        locals.var_t1__blk896_dn10 = assign29560_e41978_d_n10;
        locals.var_t1__blk896_dn11 = assign29560_e41978_d_n11;
        locals.var_t1__blk896_dn12 = assign29560_e41978_d_n12;
        locals.var_t1__blk896_dn17 = assign29560_e41978_d_n17;
        locals.var_t1__blk896_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_107(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign29570_e42004, assign29570_e42004_d_n0, assign29570_e42004_d_n2, assign29570_e42004_d_n6, assign29570_e42004_d_n7, assign29570_e42004_d_n10, assign29570_e42004_d_n11, assign29570_e42004_d_n12, assign29570_e42004_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29570_e41995: f64 = (2.0 * 0.0008);
        let assign29570_e41997: f64 = (assign29570_e41995 * 75.0);
        let assign29570_e41998: f64 = (locals.var_tmf1 + assign29570_e41997);
        let assign29570_e42000: f64 = (assign29570_e41998 / locals.var_tmf2);
        let assign29570_e42001: f64 = (1.0 - assign29570_e42000);
        let assign29570_e42002: f64 = (0.5 * assign29570_e42001);
        (assign29570_e42002, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign29570_e41998 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign29570_e41998 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign29570_e41998 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign29570_e41998 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign29570_e41998 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign29570_e41998 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn12 * locals.var_tmf2) - (assign29570_e41998 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn17 * locals.var_tmf2) - (assign29570_e41998 * locals.var_tmf2_dn17)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t2__blk897, locals.var_t2__blk897_dn0, locals.var_t2__blk897_dn2, locals.var_t2__blk897_dn6, locals.var_t2__blk897_dn7, locals.var_t2__blk897_dn10, locals.var_t2__blk897_dn11, locals.var_t2__blk897_dn12, locals.var_t2__blk897_dn17,)
    }
};
        locals.var_t2__blk897 = assign29570_e42004;
        locals.var_t2__blk897_dn0 = assign29570_e42004_d_n0;
        locals.var_t2__blk897_dn2 = assign29570_e42004_d_n2;
        locals.var_t2__blk897_dn6 = assign29570_e42004_d_n6;
        locals.var_t2__blk897_dn7 = assign29570_e42004_d_n7;
        locals.var_t2__blk897_dn10 = assign29570_e42004_d_n10;
        locals.var_t2__blk897_dn11 = assign29570_e42004_d_n11;
        locals.var_t2__blk897_dn12 = assign29570_e42004_d_n12;
        locals.var_t2__blk897_dn17 = assign29570_e42004_d_n17;
        locals.var_t2__blk897_rv = 0.0;

        let (assign29580_e42024, assign29580_e42024_d_n0, assign29580_e42024_d_n2, assign29580_e42024_d_n6, assign29580_e42024_d_n7, assign29580_e42024_d_n10, assign29580_e42024_d_n11, assign29580_e42024_d_n12, assign29580_e42024_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard987 != 0.0)) {
        let assign29580_e42020: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign29580_e42021: f64 = (0.5 * assign29580_e42020);
        let assign29580_e42022: f64 = (locals.var_chi_b__blk960 - assign29580_e42021);
        (assign29580_e42022, (locals.var_chi_b__blk960_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_chi_b__blk960_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_chi_b__blk960_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_chi_b__blk960_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_chi_b__blk960_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_chi_b__blk960_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_chi_b__blk960_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))), (locals.var_chi_b__blk960_dn17 - (0.5 * (locals.var_tmf1_dn17 + locals.var_tmf2_dn17))),)
    } else {
        (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    }
};
        locals.var_chi__blk943 = assign29580_e42024;
        locals.var_chi__blk943_dn0 = assign29580_e42024_d_n0;
        locals.var_chi__blk943_dn2 = assign29580_e42024_d_n2;
        locals.var_chi__blk943_dn6 = assign29580_e42024_d_n6;
        locals.var_chi__blk943_dn7 = assign29580_e42024_d_n7;
        locals.var_chi__blk943_dn10 = assign29580_e42024_d_n10;
        locals.var_chi__blk943_dn11 = assign29580_e42024_d_n11;
        locals.var_chi__blk943_dn12 = assign29580_e42024_d_n12;
        locals.var_chi__blk943_dn17 = assign29580_e42024_d_n17;
        locals.var_chi__blk943_rv = 0.0;

        let (assign29590_e42040, assign29590_e42040_d_n0, assign29590_e42040_d_n2, assign29590_e42040_d_n6, assign29590_e42040_d_n7, assign29590_e42040_d_n10, assign29590_e42040_d_n11, assign29590_e42040_d_n12, assign29590_e42040_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) {
        let assign29590_e42036: f64 = (locals.var_chi__blk943 / locals.var_beta);
        let assign29590_e42038: f64 = (assign29590_e42036 - locals.var_vxbgmtcl__blk921);
        (assign29590_e42038, ((locals.var_chi__blk943_dn0 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn0), ((locals.var_chi__blk943_dn2 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn2), ((locals.var_chi__blk943_dn6 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn6), ((locals.var_chi__blk943_dn7 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn7), ((((locals.var_chi__blk943_dn10 * locals.var_beta) - (locals.var_chi__blk943 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl__blk921_dn10), ((locals.var_chi__blk943_dn11 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn11), ((locals.var_chi__blk943_dn12 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn12), ((locals.var_chi__blk943_dn17 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_ps0ld__blk945, locals.var_ps0ld__blk945_dn0, locals.var_ps0ld__blk945_dn2, locals.var_ps0ld__blk945_dn6, locals.var_ps0ld__blk945_dn7, locals.var_ps0ld__blk945_dn10, locals.var_ps0ld__blk945_dn11, locals.var_ps0ld__blk945_dn12, locals.var_ps0ld__blk945_dn17,)
    }
};
        locals.var_ps0ld__blk945 = assign29590_e42040;
        locals.var_ps0ld__blk945_dn0 = assign29590_e42040_d_n0;
        locals.var_ps0ld__blk945_dn2 = assign29590_e42040_d_n2;
        locals.var_ps0ld__blk945_dn6 = assign29590_e42040_d_n6;
        locals.var_ps0ld__blk945_dn7 = assign29590_e42040_d_n7;
        locals.var_ps0ld__blk945_dn10 = assign29590_e42040_d_n10;
        locals.var_ps0ld__blk945_dn11 = assign29590_e42040_d_n11;
        locals.var_ps0ld__blk945_dn12 = assign29590_e42040_d_n12;
        locals.var_ps0ld__blk945_dn17 = assign29590_e42040_d_n17;
        locals.var_ps0ld__blk945_rv = 0.0;

        let (assign29600_e42058, assign29600_e42058_d_n0, assign29600_e42058_d_n2, assign29600_e42058_d_n6, assign29600_e42058_d_n7, assign29600_e42058_d_n10, assign29600_e42058_d_n11, assign29600_e42058_d_n12, assign29600_e42058_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) {
        let assign29600_e42052: f64 = (locals.var_chi__blk943 - 1.0);
        let assign29600_e42054: f64 = (-locals.var_chi__blk943);
        let assign29600_e42055: f64 = (assign29600_e42054).exp();
        let assign29600_e42056: f64 = (assign29600_e42052 + assign29600_e42055);
        (assign29600_e42056, (locals.var_chi__blk943_dn0 + (assign29600_e42055 * (-locals.var_chi__blk943_dn0))), (locals.var_chi__blk943_dn2 + (assign29600_e42055 * (-locals.var_chi__blk943_dn2))), (locals.var_chi__blk943_dn6 + (assign29600_e42055 * (-locals.var_chi__blk943_dn6))), (locals.var_chi__blk943_dn7 + (assign29600_e42055 * (-locals.var_chi__blk943_dn7))), (locals.var_chi__blk943_dn10 + (assign29600_e42055 * (-locals.var_chi__blk943_dn10))), (locals.var_chi__blk943_dn11 + (assign29600_e42055 * (-locals.var_chi__blk943_dn11))), (locals.var_chi__blk943_dn12 + (assign29600_e42055 * (-locals.var_chi__blk943_dn12))), (locals.var_chi__blk943_dn17 + (assign29600_e42055 * (-locals.var_chi__blk943_dn17))),)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign29600_e42058;
        locals.var_t1__blk896_dn0 = assign29600_e42058_d_n0;
        locals.var_t1__blk896_dn2 = assign29600_e42058_d_n2;
        locals.var_t1__blk896_dn6 = assign29600_e42058_d_n6;
        locals.var_t1__blk896_dn7 = assign29600_e42058_d_n7;
        locals.var_t1__blk896_dn10 = assign29600_e42058_d_n10;
        locals.var_t1__blk896_dn11 = assign29600_e42058_d_n11;
        locals.var_t1__blk896_dn12 = assign29600_e42058_d_n12;
        locals.var_t1__blk896_dn17 = assign29600_e42058_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let assign29610_e42062: f64 = (10.0 * 2.220446049250313e-16);
        let assign29610_e42063: f64 = if locals.var_t1__blk896 < assign29610_e42062 { 1.0 } else { 0.0 };
        locals.var_guard988 = assign29610_e42063;
        locals.var_guard988_rv = 0.0;

        let (assign29620_e42079, assign29620_e42079_d_n0, assign29620_e42079_d_n2, assign29620_e42079_d_n6, assign29620_e42079_d_n7, assign29620_e42079_d_n10, assign29620_e42079_d_n11, assign29620_e42079_d_n12, assign29620_e42079_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard988 != 0.0)) {
        let assign29620_e42077: f64 = (10.0 * 2.220446049250313e-16);
        (assign29620_e42077, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign29620_e42079;
        locals.var_t1__blk896_dn0 = assign29620_e42079_d_n0;
        locals.var_t1__blk896_dn2 = assign29620_e42079_d_n2;
        locals.var_t1__blk896_dn6 = assign29620_e42079_d_n6;
        locals.var_t1__blk896_dn7 = assign29620_e42079_d_n7;
        locals.var_t1__blk896_dn10 = assign29620_e42079_d_n10;
        locals.var_t1__blk896_dn11 = assign29620_e42079_d_n11;
        locals.var_t1__blk896_dn12 = assign29620_e42079_d_n12;
        locals.var_t1__blk896_dn17 = assign29620_e42079_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let (assign29630_e42092, assign29630_e42092_d_n0, assign29630_e42092_d_n2, assign29630_e42092_d_n6, assign29630_e42092_d_n7, assign29630_e42092_d_n10, assign29630_e42092_d_n11, assign29630_e42092_d_n12, assign29630_e42092_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) {
        let assign29630_e42090: f64 = (locals.var_t1__blk896).sqrt();
        (assign29630_e42090, (locals.var_t1__blk896_dn0 / (2.0 * assign29630_e42090)), (locals.var_t1__blk896_dn2 / (2.0 * assign29630_e42090)), (locals.var_t1__blk896_dn6 / (2.0 * assign29630_e42090)), (locals.var_t1__blk896_dn7 / (2.0 * assign29630_e42090)), (locals.var_t1__blk896_dn10 / (2.0 * assign29630_e42090)), (locals.var_t1__blk896_dn11 / (2.0 * assign29630_e42090)), (locals.var_t1__blk896_dn12 / (2.0 * assign29630_e42090)), (locals.var_t1__blk896_dn17 / (2.0 * assign29630_e42090)),)
    } else {
        (locals.var_t2__blk897, locals.var_t2__blk897_dn0, locals.var_t2__blk897_dn2, locals.var_t2__blk897_dn6, locals.var_t2__blk897_dn7, locals.var_t2__blk897_dn10, locals.var_t2__blk897_dn11, locals.var_t2__blk897_dn12, locals.var_t2__blk897_dn17,)
    }
};
        locals.var_t2__blk897 = assign29630_e42092;
        locals.var_t2__blk897_dn0 = assign29630_e42092_d_n0;
        locals.var_t2__blk897_dn2 = assign29630_e42092_d_n2;
        locals.var_t2__blk897_dn6 = assign29630_e42092_d_n6;
        locals.var_t2__blk897_dn7 = assign29630_e42092_d_n7;
        locals.var_t2__blk897_dn10 = assign29630_e42092_d_n10;
        locals.var_t2__blk897_dn11 = assign29630_e42092_d_n11;
        locals.var_t2__blk897_dn12 = assign29630_e42092_d_n12;
        locals.var_t2__blk897_dn17 = assign29630_e42092_d_n17;
        locals.var_t2__blk897_rv = 0.0;

        let (assign29640_e42106, assign29640_e42106_d_n0, assign29640_e42106_d_n2, assign29640_e42106_d_n6, assign29640_e42106_d_n7, assign29640_e42106_d_n10, assign29640_e42106_d_n11, assign29640_e42106_d_n12, assign29640_e42106_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) {
        let assign29640_e42104: f64 = (locals.var_cnst0over__blk928 * locals.var_t2__blk897);
        (assign29640_e42104, ((locals.var_cnst0over__blk928_dn0 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn0)), ((locals.var_cnst0over__blk928_dn2 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn2)), ((locals.var_cnst0over__blk928_dn6 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn6)), ((locals.var_cnst0over__blk928_dn7 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn7)), ((locals.var_cnst0over__blk928_dn10 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn10)), ((locals.var_cnst0over__blk928_dn11 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn11)), ((locals.var_cnst0over__blk928_dn12 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn12)), ((locals.var_cnst0over__blk928_dn17 * locals.var_t2__blk897) + (locals.var_cnst0over__blk928 * locals.var_t2__blk897_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign29640_e42106;
        locals.var_qbuld_dn0 = assign29640_e42106_d_n0;
        locals.var_qbuld_dn2 = assign29640_e42106_d_n2;
        locals.var_qbuld_dn6 = assign29640_e42106_d_n6;
        locals.var_qbuld_dn7 = assign29640_e42106_d_n7;
        locals.var_qbuld_dn10 = assign29640_e42106_d_n10;
        locals.var_qbuld_dn11 = assign29640_e42106_d_n11;
        locals.var_qbuld_dn12 = assign29640_e42106_d_n12;
        locals.var_qbuld_dn17 = assign29640_e42106_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign29650_e42122, assign29650_e42122_d_n0, assign29650_e42122_d_n2, assign29650_e42122_d_n6, assign29650_e42122_d_n7, assign29650_e42122_d_n10, assign29650_e42122_d_n11, assign29650_e42122_d_n12, assign29650_e42122_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) {
        let assign29650_e42119: f64 = (locals.var_vgpld__blk931 - locals.var_ps0ld__blk945);
        let assign29650_e42120: f64 = (locals.var_cox0__blk906 * assign29650_e42119);
        (assign29650_e42120, (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn0 - locals.var_ps0ld__blk945_dn0)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn2 - locals.var_ps0ld__blk945_dn2)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn6 - locals.var_ps0ld__blk945_dn6)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn7 - locals.var_ps0ld__blk945_dn7)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn10 - locals.var_ps0ld__blk945_dn10)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn11 - locals.var_ps0ld__blk945_dn11)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn12 - locals.var_ps0ld__blk945_dn12)), (locals.var_cox0__blk906 * (locals.var_vgpld__blk931_dn17 - locals.var_ps0ld__blk945_dn17)),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign29650_e42122;
        locals.var_qsuld_dn0 = assign29650_e42122_d_n0;
        locals.var_qsuld_dn2 = assign29650_e42122_d_n2;
        locals.var_qsuld_dn6 = assign29650_e42122_d_n6;
        locals.var_qsuld_dn7 = assign29650_e42122_d_n7;
        locals.var_qsuld_dn10 = assign29650_e42122_d_n10;
        locals.var_qsuld_dn11 = assign29650_e42122_d_n11;
        locals.var_qsuld_dn12 = assign29650_e42122_d_n12;
        locals.var_qsuld_dn17 = assign29650_e42122_d_n17;
        locals.var_qsuld_rv = 0.0;

        let assign29660_e42125: f64 = if p.p41 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard989 = assign29660_e42125;
        locals.var_guard989_rv = 0.0;

        let (assign29670_e42143, assign29670_e42143_d_n0, assign29670_e42143_d_n2, assign29670_e42143_d_n6, assign29670_e42143_d_n7, assign29670_e42143_d_n10, assign29670_e42143_d_n11, assign29670_e42143_d_n12, assign29670_e42143_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29670_e42139: f64 = (-locals.var_vxbgmtcl__blk921);
        let assign29670_e42140: f64 = (locals.var_beta * assign29670_e42139);
        let assign29670_e42141: f64 = (assign29670_e42140).exp();
        (assign29670_e42141, (assign29670_e42141 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn0))), (assign29670_e42141 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn2))), (assign29670_e42141 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn6))), (assign29670_e42141 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn7))), (assign29670_e42141 * ((locals.var_beta_dn10 * assign29670_e42139) + (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn10)))), (assign29670_e42141 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn11))), (assign29670_e42141 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn12))), (assign29670_e42141 * (locals.var_beta * (-locals.var_vxbgmtcl__blk921_dn17))),)
    } else {
        (locals.var_exp_bvbs__blk962, locals.var_exp_bvbs__blk962_dn0, locals.var_exp_bvbs__blk962_dn2, locals.var_exp_bvbs__blk962_dn6, locals.var_exp_bvbs__blk962_dn7, locals.var_exp_bvbs__blk962_dn10, locals.var_exp_bvbs__blk962_dn11, locals.var_exp_bvbs__blk962_dn12, locals.var_exp_bvbs__blk962_dn17,)
    }
};
        locals.var_exp_bvbs__blk962 = assign29670_e42143;
        locals.var_exp_bvbs__blk962_dn0 = assign29670_e42143_d_n0;
        locals.var_exp_bvbs__blk962_dn2 = assign29670_e42143_d_n2;
        locals.var_exp_bvbs__blk962_dn6 = assign29670_e42143_d_n6;
        locals.var_exp_bvbs__blk962_dn7 = assign29670_e42143_d_n7;
        locals.var_exp_bvbs__blk962_dn10 = assign29670_e42143_d_n10;
        locals.var_exp_bvbs__blk962_dn11 = assign29670_e42143_d_n11;
        locals.var_exp_bvbs__blk962_dn12 = assign29670_e42143_d_n12;
        locals.var_exp_bvbs__blk962_dn17 = assign29670_e42143_d_n17;
        locals.var_exp_bvbs__blk962_rv = 0.0;

        let (assign29680_e42159, assign29680_e42159_d_n0, assign29680_e42159_d_n2, assign29680_e42159_d_n6, assign29680_e42159_d_n7, assign29680_e42159_d_n10, assign29680_e42159_d_n11, assign29680_e42159_d_n12, assign29680_e42159_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29680_e42157: f64 = (locals.var_nin / locals.var_mks_nover);
        (assign29680_e42157, (locals.var_nin_dn0 / locals.var_mks_nover), (locals.var_nin_dn2 / locals.var_mks_nover), (locals.var_nin_dn6 / locals.var_mks_nover), (locals.var_nin_dn7 / locals.var_mks_nover), (locals.var_nin_dn10 / locals.var_mks_nover), (locals.var_nin_dn11 / locals.var_mks_nover), (locals.var_nin_dn12 / locals.var_mks_nover), (locals.var_nin_dn17 / locals.var_mks_nover),)
    } else {
        (locals.var_t0__blk895, locals.var_t0__blk895_dn0, locals.var_t0__blk895_dn2, locals.var_t0__blk895_dn6, locals.var_t0__blk895_dn7, locals.var_t0__blk895_dn10, locals.var_t0__blk895_dn11, locals.var_t0__blk895_dn12, locals.var_t0__blk895_dn17,)
    }
};
        locals.var_t0__blk895 = assign29680_e42159;
        locals.var_t0__blk895_dn0 = assign29680_e42159_d_n0;
        locals.var_t0__blk895_dn2 = assign29680_e42159_d_n2;
        locals.var_t0__blk895_dn6 = assign29680_e42159_d_n6;
        locals.var_t0__blk895_dn7 = assign29680_e42159_d_n7;
        locals.var_t0__blk895_dn10 = assign29680_e42159_d_n10;
        locals.var_t0__blk895_dn11 = assign29680_e42159_d_n11;
        locals.var_t0__blk895_dn12 = assign29680_e42159_d_n12;
        locals.var_t0__blk895_dn17 = assign29680_e42159_d_n17;
        locals.var_t0__blk895_rv = 0.0;

        let (assign29690_e42175, assign29690_e42175_d_n0, assign29690_e42175_d_n2, assign29690_e42175_d_n6, assign29690_e42175_d_n7, assign29690_e42175_d_n10, assign29690_e42175_d_n11, assign29690_e42175_d_n12, assign29690_e42175_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29690_e42173: f64 = (locals.var_t0__blk895 * locals.var_t0__blk895);
        (assign29690_e42173, ((locals.var_t0__blk895_dn0 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn0)), ((locals.var_t0__blk895_dn2 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn2)), ((locals.var_t0__blk895_dn6 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn6)), ((locals.var_t0__blk895_dn7 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn7)), ((locals.var_t0__blk895_dn10 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn10)), ((locals.var_t0__blk895_dn11 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn11)), ((locals.var_t0__blk895_dn12 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn12)), ((locals.var_t0__blk895_dn17 * locals.var_t0__blk895) + (locals.var_t0__blk895 * locals.var_t0__blk895_dn17)),)
    } else {
        (locals.var_cnst1over__blk956, locals.var_cnst1over__blk956_dn0, locals.var_cnst1over__blk956_dn2, locals.var_cnst1over__blk956_dn6, locals.var_cnst1over__blk956_dn7, locals.var_cnst1over__blk956_dn10, locals.var_cnst1over__blk956_dn11, locals.var_cnst1over__blk956_dn12, locals.var_cnst1over__blk956_dn17,)
    }
};
        locals.var_cnst1over__blk956 = assign29690_e42175;
        locals.var_cnst1over__blk956_dn0 = assign29690_e42175_d_n0;
        locals.var_cnst1over__blk956_dn2 = assign29690_e42175_d_n2;
        locals.var_cnst1over__blk956_dn6 = assign29690_e42175_d_n6;
        locals.var_cnst1over__blk956_dn7 = assign29690_e42175_d_n7;
        locals.var_cnst1over__blk956_dn10 = assign29690_e42175_d_n10;
        locals.var_cnst1over__blk956_dn11 = assign29690_e42175_d_n11;
        locals.var_cnst1over__blk956_dn12 = assign29690_e42175_d_n12;
        locals.var_cnst1over__blk956_dn17 = assign29690_e42175_d_n17;
        locals.var_cnst1over__blk956_rv = 0.0;

        let (assign29700_e42191, assign29700_e42191_d_n0, assign29700_e42191_d_n2, assign29700_e42191_d_n6, assign29700_e42191_d_n7, assign29700_e42191_d_n10, assign29700_e42191_d_n11, assign29700_e42191_d_n12, assign29700_e42191_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29700_e42189: f64 = (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962);
        (assign29700_e42189, ((locals.var_cnst1over__blk956_dn0 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn0)), ((locals.var_cnst1over__blk956_dn2 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn2)), ((locals.var_cnst1over__blk956_dn6 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn6)), ((locals.var_cnst1over__blk956_dn7 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn7)), ((locals.var_cnst1over__blk956_dn10 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn10)), ((locals.var_cnst1over__blk956_dn11 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn11)), ((locals.var_cnst1over__blk956_dn12 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn12)), ((locals.var_cnst1over__blk956_dn17 * locals.var_exp_bvbs__blk962) + (locals.var_cnst1over__blk956 * locals.var_exp_bvbs__blk962_dn17)),)
    } else {
        (locals.var_cfs1__blk971, locals.var_cfs1__blk971_dn0, locals.var_cfs1__blk971_dn2, locals.var_cfs1__blk971_dn6, locals.var_cfs1__blk971_dn7, locals.var_cfs1__blk971_dn10, locals.var_cfs1__blk971_dn11, locals.var_cfs1__blk971_dn12, locals.var_cfs1__blk971_dn17,)
    }
};
        locals.var_cfs1__blk971 = assign29700_e42191;
        locals.var_cfs1__blk971_dn0 = assign29700_e42191_d_n0;
        locals.var_cfs1__blk971_dn2 = assign29700_e42191_d_n2;
        locals.var_cfs1__blk971_dn6 = assign29700_e42191_d_n6;
        locals.var_cfs1__blk971_dn7 = assign29700_e42191_d_n7;
        locals.var_cfs1__blk971_dn10 = assign29700_e42191_d_n10;
        locals.var_cfs1__blk971_dn11 = assign29700_e42191_d_n11;
        locals.var_cfs1__blk971_dn12 = assign29700_e42191_d_n12;
        locals.var_cfs1__blk971_dn17 = assign29700_e42191_d_n17;
        locals.var_cfs1__blk971_rv = 0.0;

        let (assign29710_e42205,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv__blk918,)
    }
};
        locals.var_flg_conv__blk918 = assign29710_e42205;
        locals.var_flg_conv__blk918_rv = 0.0;

        let (assign29720_e42219, assign29720_e42219_d_n0, assign29720_e42219_d_n2, assign29720_e42219_d_n6, assign29720_e42219_d_n7, assign29720_e42219_d_n10, assign29720_e42219_d_n11, assign29720_e42219_d_n12, assign29720_e42219_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs01__blk965, locals.var_fs01__blk965_dn0, locals.var_fs01__blk965_dn2, locals.var_fs01__blk965_dn6, locals.var_fs01__blk965_dn7, locals.var_fs01__blk965_dn10, locals.var_fs01__blk965_dn11, locals.var_fs01__blk965_dn12, locals.var_fs01__blk965_dn17,)
    }
};
        locals.var_fs01__blk965 = assign29720_e42219;
        locals.var_fs01__blk965_dn0 = assign29720_e42219_d_n0;
        locals.var_fs01__blk965_dn2 = assign29720_e42219_d_n2;
        locals.var_fs01__blk965_dn6 = assign29720_e42219_d_n6;
        locals.var_fs01__blk965_dn7 = assign29720_e42219_d_n7;
        locals.var_fs01__blk965_dn10 = assign29720_e42219_d_n10;
        locals.var_fs01__blk965_dn11 = assign29720_e42219_d_n11;
        locals.var_fs01__blk965_dn12 = assign29720_e42219_d_n12;
        locals.var_fs01__blk965_dn17 = assign29720_e42219_d_n17;
        locals.var_fs01__blk965_rv = 0.0;

        let (assign29730_e42233, assign29730_e42233_d_n0, assign29730_e42233_d_n2, assign29730_e42233_d_n6, assign29730_e42233_d_n7, assign29730_e42233_d_n10, assign29730_e42233_d_n11, assign29730_e42233_d_n12, assign29730_e42233_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fs02__blk969, locals.var_fs02__blk969_dn0, locals.var_fs02__blk969_dn2, locals.var_fs02__blk969_dn6, locals.var_fs02__blk969_dn7, locals.var_fs02__blk969_dn10, locals.var_fs02__blk969_dn11, locals.var_fs02__blk969_dn12, locals.var_fs02__blk969_dn17,)
    }
};
        locals.var_fs02__blk969 = assign29730_e42233;
        locals.var_fs02__blk969_dn0 = assign29730_e42233_d_n0;
        locals.var_fs02__blk969_dn2 = assign29730_e42233_d_n2;
        locals.var_fs02__blk969_dn6 = assign29730_e42233_d_n6;
        locals.var_fs02__blk969_dn7 = assign29730_e42233_d_n7;
        locals.var_fs02__blk969_dn10 = assign29730_e42233_d_n10;
        locals.var_fs02__blk969_dn11 = assign29730_e42233_d_n11;
        locals.var_fs02__blk969_dn12 = assign29730_e42233_d_n12;
        locals.var_fs02__blk969_dn17 = assign29730_e42233_d_n17;
        locals.var_fs02__blk969_rv = 0.0;

        let (assign29740_e42247,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign29740_e42247;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_108(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign29750_loop_guard: usize = 0;
        while {
            let assign29750_cond_e42262: f64 = (2.0 * 20.0);
            let assign29750_cond_e42264: f64 = (assign29750_cond_e42262 + 1.0);
            let assign29750_cond_e42266: f64 = if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_lp_s0 <= assign29750_cond_e42264)) { 1.0 } else { 0.0 };
            assign29750_cond_e42266 != 0.0
        } {
            assign29750_loop_guard += 1;
            assert!(assign29750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign29750_body0_e42280, assign29750_body0_e42280_d_n0, assign29750_body0_e42280_d_n2, assign29750_body0_e42280_d_n6, assign29750_body0_e42280_d_n7, assign29750_body0_e42280_d_n10, assign29750_body0_e42280_d_n11, assign29750_body0_e42280_d_n12, assign29750_body0_e42280_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fb__blk967, locals.var_fb__blk967_dn0, locals.var_fb__blk967_dn2, locals.var_fb__blk967_dn6, locals.var_fb__blk967_dn7, locals.var_fb__blk967_dn10, locals.var_fb__blk967_dn11, locals.var_fb__blk967_dn12, locals.var_fb__blk967_dn17,)
    }
};
            locals.var_fb__blk967 = assign29750_body0_e42280;
            locals.var_fb__blk967_dn0 = assign29750_body0_e42280_d_n0;
            locals.var_fb__blk967_dn2 = assign29750_body0_e42280_d_n2;
            locals.var_fb__blk967_dn6 = assign29750_body0_e42280_d_n6;
            locals.var_fb__blk967_dn7 = assign29750_body0_e42280_d_n7;
            locals.var_fb__blk967_dn10 = assign29750_body0_e42280_d_n10;
            locals.var_fb__blk967_dn11 = assign29750_body0_e42280_d_n11;
            locals.var_fb__blk967_dn12 = assign29750_body0_e42280_d_n12;
            locals.var_fb__blk967_dn17 = assign29750_body0_e42280_d_n17;
            locals.var_fb__blk967_rv = 0.0;
            let (assign29750_body1_e42298, assign29750_body1_e42298_d_n0, assign29750_body1_e42298_d_n2, assign29750_body1_e42298_d_n6, assign29750_body1_e42298_d_n7, assign29750_body1_e42298_d_n10, assign29750_body1_e42298_d_n11, assign29750_body1_e42298_d_n12, assign29750_body1_e42298_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29750_body1_e42295: f64 = (locals.var_ps0ld__blk945 + locals.var_vxbgmtcl__blk921);
        let assign29750_body1_e42296: f64 = (locals.var_beta * assign29750_body1_e42295);
        (assign29750_body1_e42296, (locals.var_beta * (locals.var_ps0ld__blk945_dn0 + locals.var_vxbgmtcl__blk921_dn0)), (locals.var_beta * (locals.var_ps0ld__blk945_dn2 + locals.var_vxbgmtcl__blk921_dn2)), (locals.var_beta * (locals.var_ps0ld__blk945_dn6 + locals.var_vxbgmtcl__blk921_dn6)), (locals.var_beta * (locals.var_ps0ld__blk945_dn7 + locals.var_vxbgmtcl__blk921_dn7)), ((locals.var_beta_dn10 * assign29750_body1_e42295) + (locals.var_beta * (locals.var_ps0ld__blk945_dn10 + locals.var_vxbgmtcl__blk921_dn10))), (locals.var_beta * (locals.var_ps0ld__blk945_dn11 + locals.var_vxbgmtcl__blk921_dn11)), (locals.var_beta * (locals.var_ps0ld__blk945_dn12 + locals.var_vxbgmtcl__blk921_dn12)), (locals.var_beta * (locals.var_ps0ld__blk945_dn17 + locals.var_vxbgmtcl__blk921_dn17)),)
    } else {
        (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    }
};
            locals.var_chi__blk943 = assign29750_body1_e42298;
            locals.var_chi__blk943_dn0 = assign29750_body1_e42298_d_n0;
            locals.var_chi__blk943_dn2 = assign29750_body1_e42298_d_n2;
            locals.var_chi__blk943_dn6 = assign29750_body1_e42298_d_n6;
            locals.var_chi__blk943_dn7 = assign29750_body1_e42298_d_n7;
            locals.var_chi__blk943_dn10 = assign29750_body1_e42298_d_n10;
            locals.var_chi__blk943_dn11 = assign29750_body1_e42298_d_n11;
            locals.var_chi__blk943_dn12 = assign29750_body1_e42298_d_n12;
            locals.var_chi__blk943_dn17 = assign29750_body1_e42298_d_n17;
            locals.var_chi__blk943_rv = 0.0;
            let assign29750_body2_e42301: f64 = if locals.var_chi__blk943 < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard990 = assign29750_body2_e42301;
            locals.var_guard990_rv = 0.0;
            let (assign29750_body3_e42332, assign29750_body3_e42332_d_n0, assign29750_body3_e42332_d_n2, assign29750_body3_e42332_d_n6, assign29750_body3_e42332_d_n7, assign29750_body3_e42332_d_n10, assign29750_body3_e42332_d_n11, assign29750_body3_e42332_d_n12, assign29750_body3_e42332_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29750_body3_e42317: f64 = (locals.var_chi__blk943 * locals.var_chi__blk943);
        let assign29750_body3_e42319: f64 = (assign29750_body3_e42317 * locals.var_chi__blk943);
        let assign29750_body3_e42323: f64 = (-0.07053654284009761);
        let assign29750_body3_e42326: f64 = (locals.var_chi__blk943 * 0.006115288895133179);
        let assign29750_body3_e42327: f64 = (assign29750_body3_e42323 + assign29750_body3_e42326);
        let assign29750_body3_e42328: f64 = (locals.var_chi__blk943 * assign29750_body3_e42327);
        let assign29750_body3_e42329: f64 = (0.29693154855771 + assign29750_body3_e42328);
        let assign29750_body3_e42330: f64 = (assign29750_body3_e42319 * assign29750_body3_e42329);
        (assign29750_body3_e42330, ((((((locals.var_chi__blk943_dn0 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn0)) * locals.var_chi__blk943) + (assign29750_body3_e42317 * locals.var_chi__blk943_dn0)) * assign29750_body3_e42329) + (assign29750_body3_e42319 * ((locals.var_chi__blk943_dn0 * assign29750_body3_e42327) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn0 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn2 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn2)) * locals.var_chi__blk943) + (assign29750_body3_e42317 * locals.var_chi__blk943_dn2)) * assign29750_body3_e42329) + (assign29750_body3_e42319 * ((locals.var_chi__blk943_dn2 * assign29750_body3_e42327) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn2 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn6 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn6)) * locals.var_chi__blk943) + (assign29750_body3_e42317 * locals.var_chi__blk943_dn6)) * assign29750_body3_e42329) + (assign29750_body3_e42319 * ((locals.var_chi__blk943_dn6 * assign29750_body3_e42327) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn6 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn7 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn7)) * locals.var_chi__blk943) + (assign29750_body3_e42317 * locals.var_chi__blk943_dn7)) * assign29750_body3_e42329) + (assign29750_body3_e42319 * ((locals.var_chi__blk943_dn7 * assign29750_body3_e42327) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn7 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn10 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn10)) * locals.var_chi__blk943) + (assign29750_body3_e42317 * locals.var_chi__blk943_dn10)) * assign29750_body3_e42329) + (assign29750_body3_e42319 * ((locals.var_chi__blk943_dn10 * assign29750_body3_e42327) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn10 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn11 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn11)) * locals.var_chi__blk943) + (assign29750_body3_e42317 * locals.var_chi__blk943_dn11)) * assign29750_body3_e42329) + (assign29750_body3_e42319 * ((locals.var_chi__blk943_dn11 * assign29750_body3_e42327) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn11 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn12 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn12)) * locals.var_chi__blk943) + (assign29750_body3_e42317 * locals.var_chi__blk943_dn12)) * assign29750_body3_e42329) + (assign29750_body3_e42319 * ((locals.var_chi__blk943_dn12 * assign29750_body3_e42327) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn12 * 0.006115288895133179))))), ((((((locals.var_chi__blk943_dn17 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn17)) * locals.var_chi__blk943) + (assign29750_body3_e42317 * locals.var_chi__blk943_dn17)) * assign29750_body3_e42329) + (assign29750_body3_e42319 * ((locals.var_chi__blk943_dn17 * assign29750_body3_e42327) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn17 * 0.006115288895133179))))),)
    } else {
        (locals.var_fi__blk963, locals.var_fi__blk963_dn0, locals.var_fi__blk963_dn2, locals.var_fi__blk963_dn6, locals.var_fi__blk963_dn7, locals.var_fi__blk963_dn10, locals.var_fi__blk963_dn11, locals.var_fi__blk963_dn12, locals.var_fi__blk963_dn17,)
    }
};
            locals.var_fi__blk963 = assign29750_body3_e42332;
            locals.var_fi__blk963_dn0 = assign29750_body3_e42332_d_n0;
            locals.var_fi__blk963_dn2 = assign29750_body3_e42332_d_n2;
            locals.var_fi__blk963_dn6 = assign29750_body3_e42332_d_n6;
            locals.var_fi__blk963_dn7 = assign29750_body3_e42332_d_n7;
            locals.var_fi__blk963_dn10 = assign29750_body3_e42332_d_n10;
            locals.var_fi__blk963_dn11 = assign29750_body3_e42332_d_n11;
            locals.var_fi__blk963_dn12 = assign29750_body3_e42332_d_n12;
            locals.var_fi__blk963_dn17 = assign29750_body3_e42332_d_n17;
            locals.var_fi__blk963_rv = 0.0;
            let (assign29750_body4_e42367, assign29750_body4_e42367_d_n0, assign29750_body4_e42367_d_n2, assign29750_body4_e42367_d_n6, assign29750_body4_e42367_d_n7, assign29750_body4_e42367_d_n10, assign29750_body4_e42367_d_n11, assign29750_body4_e42367_d_n12, assign29750_body4_e42367_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29750_body4_e42348: f64 = (locals.var_chi__blk943 * locals.var_chi__blk943);
        let assign29750_body4_e42351: f64 = (3.0 * 0.29693154855771);
        let assign29750_body4_e42355: f64 = (-0.07053654284009761);
        let assign29750_body4_e42356: f64 = (4.0 * assign29750_body4_e42355);
        let assign29750_body4_e42359: f64 = (locals.var_chi__blk943 * 5.0);
        let assign29750_body4_e42361: f64 = (assign29750_body4_e42359 * 0.006115288895133179);
        let assign29750_body4_e42362: f64 = (assign29750_body4_e42356 + assign29750_body4_e42361);
        let assign29750_body4_e42363: f64 = (locals.var_chi__blk943 * assign29750_body4_e42362);
        let assign29750_body4_e42364: f64 = (assign29750_body4_e42351 + assign29750_body4_e42363);
        let assign29750_body4_e42365: f64 = (assign29750_body4_e42348 * assign29750_body4_e42364);
        (assign29750_body4_e42365, ((((locals.var_chi__blk943_dn0 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn0)) * assign29750_body4_e42364) + (assign29750_body4_e42348 * ((locals.var_chi__blk943_dn0 * assign29750_body4_e42362) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn2 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn2)) * assign29750_body4_e42364) + (assign29750_body4_e42348 * ((locals.var_chi__blk943_dn2 * assign29750_body4_e42362) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn6 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn6)) * assign29750_body4_e42364) + (assign29750_body4_e42348 * ((locals.var_chi__blk943_dn6 * assign29750_body4_e42362) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn7 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn7)) * assign29750_body4_e42364) + (assign29750_body4_e42348 * ((locals.var_chi__blk943_dn7 * assign29750_body4_e42362) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn10 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn10)) * assign29750_body4_e42364) + (assign29750_body4_e42348 * ((locals.var_chi__blk943_dn10 * assign29750_body4_e42362) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn11 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn11)) * assign29750_body4_e42364) + (assign29750_body4_e42348 * ((locals.var_chi__blk943_dn11 * assign29750_body4_e42362) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn12 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn12)) * assign29750_body4_e42364) + (assign29750_body4_e42348 * ((locals.var_chi__blk943_dn12 * assign29750_body4_e42362) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi__blk943_dn17 * locals.var_chi__blk943) + (locals.var_chi__blk943 * locals.var_chi__blk943_dn17)) * assign29750_body4_e42364) + (assign29750_body4_e42348 * ((locals.var_chi__blk943_dn17 * assign29750_body4_e42362) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * 5.0) * 0.006115288895133179))))),)
    } else {
        (locals.var_fi_dchi__blk964, locals.var_fi_dchi__blk964_dn0, locals.var_fi_dchi__blk964_dn2, locals.var_fi_dchi__blk964_dn6, locals.var_fi_dchi__blk964_dn7, locals.var_fi_dchi__blk964_dn10, locals.var_fi_dchi__blk964_dn11, locals.var_fi_dchi__blk964_dn12, locals.var_fi_dchi__blk964_dn17,)
    }
};
            locals.var_fi_dchi__blk964 = assign29750_body4_e42367;
            locals.var_fi_dchi__blk964_dn0 = assign29750_body4_e42367_d_n0;
            locals.var_fi_dchi__blk964_dn2 = assign29750_body4_e42367_d_n2;
            locals.var_fi_dchi__blk964_dn6 = assign29750_body4_e42367_d_n6;
            locals.var_fi_dchi__blk964_dn7 = assign29750_body4_e42367_d_n7;
            locals.var_fi_dchi__blk964_dn10 = assign29750_body4_e42367_d_n10;
            locals.var_fi_dchi__blk964_dn11 = assign29750_body4_e42367_d_n11;
            locals.var_fi_dchi__blk964_dn12 = assign29750_body4_e42367_d_n12;
            locals.var_fi_dchi__blk964_dn17 = assign29750_body4_e42367_d_n17;
            locals.var_fi_dchi__blk964_rv = 0.0;
            let (assign29750_body5_e42387, assign29750_body5_e42387_d_n0, assign29750_body5_e42387_d_n2, assign29750_body5_e42387_d_n6, assign29750_body5_e42387_d_n7, assign29750_body5_e42387_d_n10, assign29750_body5_e42387_d_n11, assign29750_body5_e42387_d_n12, assign29750_body5_e42387_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29750_body5_e42383: f64 = (locals.var_cfs1__blk971 * locals.var_fi__blk963);
        let assign29750_body5_e42385: f64 = (assign29750_body5_e42383 * locals.var_fi__blk963);
        (assign29750_body5_e42385, ((((locals.var_cfs1__blk971_dn0 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn0)) * locals.var_fi__blk963) + (assign29750_body5_e42383 * locals.var_fi__blk963_dn0)), ((((locals.var_cfs1__blk971_dn2 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn2)) * locals.var_fi__blk963) + (assign29750_body5_e42383 * locals.var_fi__blk963_dn2)), ((((locals.var_cfs1__blk971_dn6 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn6)) * locals.var_fi__blk963) + (assign29750_body5_e42383 * locals.var_fi__blk963_dn6)), ((((locals.var_cfs1__blk971_dn7 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn7)) * locals.var_fi__blk963) + (assign29750_body5_e42383 * locals.var_fi__blk963_dn7)), ((((locals.var_cfs1__blk971_dn10 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn10)) * locals.var_fi__blk963) + (assign29750_body5_e42383 * locals.var_fi__blk963_dn10)), ((((locals.var_cfs1__blk971_dn11 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn11)) * locals.var_fi__blk963) + (assign29750_body5_e42383 * locals.var_fi__blk963_dn11)), ((((locals.var_cfs1__blk971_dn12 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn12)) * locals.var_fi__blk963) + (assign29750_body5_e42383 * locals.var_fi__blk963_dn12)), ((((locals.var_cfs1__blk971_dn17 * locals.var_fi__blk963) + (locals.var_cfs1__blk971 * locals.var_fi__blk963_dn17)) * locals.var_fi__blk963) + (assign29750_body5_e42383 * locals.var_fi__blk963_dn17)),)
    } else {
        (locals.var_fs01__blk965, locals.var_fs01__blk965_dn0, locals.var_fs01__blk965_dn2, locals.var_fs01__blk965_dn6, locals.var_fs01__blk965_dn7, locals.var_fs01__blk965_dn10, locals.var_fs01__blk965_dn11, locals.var_fs01__blk965_dn12, locals.var_fs01__blk965_dn17,)
    }
};
            locals.var_fs01__blk965 = assign29750_body5_e42387;
            locals.var_fs01__blk965_dn0 = assign29750_body5_e42387_d_n0;
            locals.var_fs01__blk965_dn2 = assign29750_body5_e42387_d_n2;
            locals.var_fs01__blk965_dn6 = assign29750_body5_e42387_d_n6;
            locals.var_fs01__blk965_dn7 = assign29750_body5_e42387_d_n7;
            locals.var_fs01__blk965_dn10 = assign29750_body5_e42387_d_n10;
            locals.var_fs01__blk965_dn11 = assign29750_body5_e42387_d_n11;
            locals.var_fs01__blk965_dn12 = assign29750_body5_e42387_d_n12;
            locals.var_fs01__blk965_dn17 = assign29750_body5_e42387_d_n17;
            locals.var_fs01__blk965_rv = 0.0;
            let (assign29750_body6_e42411, assign29750_body6_e42411_d_n0, assign29750_body6_e42411_d_n2, assign29750_body6_e42411_d_n6, assign29750_body6_e42411_d_n7, assign29750_body6_e42411_d_n10, assign29750_body6_e42411_d_n11, assign29750_body6_e42411_d_n12, assign29750_body6_e42411_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29750_body6_e42403: f64 = (locals.var_cfs1__blk971 * locals.var_beta);
        let assign29750_body6_e42405: f64 = (assign29750_body6_e42403 * 2.0);
        let assign29750_body6_e42407: f64 = (assign29750_body6_e42405 * locals.var_fi__blk963);
        let assign29750_body6_e42409: f64 = (assign29750_body6_e42407 * locals.var_fi_dchi__blk964);
        (assign29750_body6_e42409, ((((((locals.var_cfs1__blk971_dn0 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign29750_body6_e42405 * locals.var_fi__blk963_dn0)) * locals.var_fi_dchi__blk964) + (assign29750_body6_e42407 * locals.var_fi_dchi__blk964_dn0)), ((((((locals.var_cfs1__blk971_dn2 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign29750_body6_e42405 * locals.var_fi__blk963_dn2)) * locals.var_fi_dchi__blk964) + (assign29750_body6_e42407 * locals.var_fi_dchi__blk964_dn2)), ((((((locals.var_cfs1__blk971_dn6 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign29750_body6_e42405 * locals.var_fi__blk963_dn6)) * locals.var_fi_dchi__blk964) + (assign29750_body6_e42407 * locals.var_fi_dchi__blk964_dn6)), ((((((locals.var_cfs1__blk971_dn7 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign29750_body6_e42405 * locals.var_fi__blk963_dn7)) * locals.var_fi_dchi__blk964) + (assign29750_body6_e42407 * locals.var_fi_dchi__blk964_dn7)), (((((((locals.var_cfs1__blk971_dn10 * locals.var_beta) + (locals.var_cfs1__blk971 * locals.var_beta_dn10)) * 2.0) * locals.var_fi__blk963) + (assign29750_body6_e42405 * locals.var_fi__blk963_dn10)) * locals.var_fi_dchi__blk964) + (assign29750_body6_e42407 * locals.var_fi_dchi__blk964_dn10)), ((((((locals.var_cfs1__blk971_dn11 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign29750_body6_e42405 * locals.var_fi__blk963_dn11)) * locals.var_fi_dchi__blk964) + (assign29750_body6_e42407 * locals.var_fi_dchi__blk964_dn11)), ((((((locals.var_cfs1__blk971_dn12 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign29750_body6_e42405 * locals.var_fi__blk963_dn12)) * locals.var_fi_dchi__blk964) + (assign29750_body6_e42407 * locals.var_fi_dchi__blk964_dn12)), ((((((locals.var_cfs1__blk971_dn17 * locals.var_beta) * 2.0) * locals.var_fi__blk963) + (assign29750_body6_e42405 * locals.var_fi__blk963_dn17)) * locals.var_fi_dchi__blk964) + (assign29750_body6_e42407 * locals.var_fi_dchi__blk964_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk966, locals.var_fs01_dps0__blk966_dn0, locals.var_fs01_dps0__blk966_dn2, locals.var_fs01_dps0__blk966_dn6, locals.var_fs01_dps0__blk966_dn7, locals.var_fs01_dps0__blk966_dn10, locals.var_fs01_dps0__blk966_dn11, locals.var_fs01_dps0__blk966_dn12, locals.var_fs01_dps0__blk966_dn17,)
    }
};
            locals.var_fs01_dps0__blk966 = assign29750_body6_e42411;
            locals.var_fs01_dps0__blk966_dn0 = assign29750_body6_e42411_d_n0;
            locals.var_fs01_dps0__blk966_dn2 = assign29750_body6_e42411_d_n2;
            locals.var_fs01_dps0__blk966_dn6 = assign29750_body6_e42411_d_n6;
            locals.var_fs01_dps0__blk966_dn7 = assign29750_body6_e42411_d_n7;
            locals.var_fs01_dps0__blk966_dn10 = assign29750_body6_e42411_d_n10;
            locals.var_fs01_dps0__blk966_dn11 = assign29750_body6_e42411_d_n11;
            locals.var_fs01_dps0__blk966_dn12 = assign29750_body6_e42411_d_n12;
            locals.var_fs01_dps0__blk966_dn17 = assign29750_body6_e42411_d_n17;
            locals.var_fs01_dps0__blk966_rv = 0.0;
            let (assign29750_body7_e42447, assign29750_body7_e42447_d_n0, assign29750_body7_e42447_d_n2, assign29750_body7_e42447_d_n6, assign29750_body7_e42447_d_n7, assign29750_body7_e42447_d_n10, assign29750_body7_e42447_d_n11, assign29750_body7_e42447_d_n12, assign29750_body7_e42447_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29750_body7_e42429: f64 = (-0.117851130197758);
        let assign29750_body7_e42434: f64 = (-0.00163730162779191);
        let assign29750_body7_e42437: f64 = (locals.var_chi__blk943 * 6.36964918866352e-5);
        let assign29750_body7_e42438: f64 = (assign29750_body7_e42434 + assign29750_body7_e42437);
        let assign29750_body7_e42439: f64 = (locals.var_chi__blk943 * assign29750_body7_e42438);
        let assign29750_body7_e42440: f64 = (0.0178800506338833 + assign29750_body7_e42439);
        let assign29750_body7_e42441: f64 = (locals.var_chi__blk943 * assign29750_body7_e42440);
        let assign29750_body7_e42442: f64 = (assign29750_body7_e42429 + assign29750_body7_e42441);
        let assign29750_body7_e42443: f64 = (locals.var_chi__blk943 * assign29750_body7_e42442);
        let assign29750_body7_e42444: f64 = (0.707106781186548 + assign29750_body7_e42443);
        let assign29750_body7_e42445: f64 = (locals.var_chi__blk943 * assign29750_body7_e42444);
        (assign29750_body7_e42445, ((locals.var_chi__blk943_dn0 * assign29750_body7_e42444) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * assign29750_body7_e42442) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * assign29750_body7_e42440) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * assign29750_body7_e42438) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn2 * assign29750_body7_e42444) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * assign29750_body7_e42442) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * assign29750_body7_e42440) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * assign29750_body7_e42438) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn6 * assign29750_body7_e42444) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * assign29750_body7_e42442) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * assign29750_body7_e42440) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * assign29750_body7_e42438) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn7 * assign29750_body7_e42444) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * assign29750_body7_e42442) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * assign29750_body7_e42440) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * assign29750_body7_e42438) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn7 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn10 * assign29750_body7_e42444) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * assign29750_body7_e42442) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * assign29750_body7_e42440) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * assign29750_body7_e42438) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn11 * assign29750_body7_e42444) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * assign29750_body7_e42442) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * assign29750_body7_e42440) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * assign29750_body7_e42438) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn12 * assign29750_body7_e42444) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * assign29750_body7_e42442) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * assign29750_body7_e42440) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * assign29750_body7_e42438) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn12 * 6.36964918866352e-5))))))))), ((locals.var_chi__blk943_dn17 * assign29750_body7_e42444) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * assign29750_body7_e42442) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * assign29750_body7_e42440) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * assign29750_body7_e42438) + (locals.var_chi__blk943 * (locals.var_chi__blk943_dn17 * 6.36964918866352e-5))))))))),)
    } else {
        (locals.var_fb__blk967, locals.var_fb__blk967_dn0, locals.var_fb__blk967_dn2, locals.var_fb__blk967_dn6, locals.var_fb__blk967_dn7, locals.var_fb__blk967_dn10, locals.var_fb__blk967_dn11, locals.var_fb__blk967_dn12, locals.var_fb__blk967_dn17,)
    }
};
            locals.var_fb__blk967 = assign29750_body7_e42447;
            locals.var_fb__blk967_dn0 = assign29750_body7_e42447_d_n0;
            locals.var_fb__blk967_dn2 = assign29750_body7_e42447_d_n2;
            locals.var_fb__blk967_dn6 = assign29750_body7_e42447_d_n6;
            locals.var_fb__blk967_dn7 = assign29750_body7_e42447_d_n7;
            locals.var_fb__blk967_dn10 = assign29750_body7_e42447_d_n10;
            locals.var_fb__blk967_dn11 = assign29750_body7_e42447_d_n11;
            locals.var_fb__blk967_dn12 = assign29750_body7_e42447_d_n12;
            locals.var_fb__blk967_dn17 = assign29750_body7_e42447_d_n17;
            locals.var_fb__blk967_rv = 0.0;
            let (assign29750_body8_e42489, assign29750_body8_e42489_d_n0, assign29750_body8_e42489_d_n2, assign29750_body8_e42489_d_n6, assign29750_body8_e42489_d_n7, assign29750_body8_e42489_d_n10, assign29750_body8_e42489_d_n11, assign29750_body8_e42489_d_n12, assign29750_body8_e42489_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29750_body8_e42465: f64 = (-0.117851130197758);
        let assign29750_body8_e42466: f64 = (2.0 * assign29750_body8_e42465);
        let assign29750_body8_e42470: f64 = (3.0 * 0.0178800506338833);
        let assign29750_body8_e42474: f64 = (-0.00163730162779191);
        let assign29750_body8_e42475: f64 = (4.0 * assign29750_body8_e42474);
        let assign29750_body8_e42478: f64 = (locals.var_chi__blk943 * 5.0);
        let assign29750_body8_e42480: f64 = (assign29750_body8_e42478 * 6.36964918866352e-5);
        let assign29750_body8_e42481: f64 = (assign29750_body8_e42475 + assign29750_body8_e42480);
        let assign29750_body8_e42482: f64 = (locals.var_chi__blk943 * assign29750_body8_e42481);
        let assign29750_body8_e42483: f64 = (assign29750_body8_e42470 + assign29750_body8_e42482);
        let assign29750_body8_e42484: f64 = (locals.var_chi__blk943 * assign29750_body8_e42483);
        let assign29750_body8_e42485: f64 = (assign29750_body8_e42466 + assign29750_body8_e42484);
        let assign29750_body8_e42486: f64 = (locals.var_chi__blk943 * assign29750_body8_e42485);
        let assign29750_body8_e42487: f64 = (0.707106781186548 + assign29750_body8_e42486);
        (assign29750_body8_e42487, ((locals.var_chi__blk943_dn0 * assign29750_body8_e42485) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * assign29750_body8_e42483) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * assign29750_body8_e42481) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn2 * assign29750_body8_e42485) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * assign29750_body8_e42483) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * assign29750_body8_e42481) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn6 * assign29750_body8_e42485) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * assign29750_body8_e42483) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * assign29750_body8_e42481) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn7 * assign29750_body8_e42485) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * assign29750_body8_e42483) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * assign29750_body8_e42481) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn7 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn10 * assign29750_body8_e42485) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * assign29750_body8_e42483) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * assign29750_body8_e42481) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn11 * assign29750_body8_e42485) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * assign29750_body8_e42483) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * assign29750_body8_e42481) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn12 * assign29750_body8_e42485) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * assign29750_body8_e42483) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * assign29750_body8_e42481) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn12 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi__blk943_dn17 * assign29750_body8_e42485) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * assign29750_body8_e42483) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * assign29750_body8_e42481) + (locals.var_chi__blk943 * ((locals.var_chi__blk943_dn17 * 5.0) * 6.36964918866352e-5))))))),)
    } else {
        (locals.var_fb_dchi__blk968, locals.var_fb_dchi__blk968_dn0, locals.var_fb_dchi__blk968_dn2, locals.var_fb_dchi__blk968_dn6, locals.var_fb_dchi__blk968_dn7, locals.var_fb_dchi__blk968_dn10, locals.var_fb_dchi__blk968_dn11, locals.var_fb_dchi__blk968_dn12, locals.var_fb_dchi__blk968_dn17,)
    }
};
            locals.var_fb_dchi__blk968 = assign29750_body8_e42489;
            locals.var_fb_dchi__blk968_dn0 = assign29750_body8_e42489_d_n0;
            locals.var_fb_dchi__blk968_dn2 = assign29750_body8_e42489_d_n2;
            locals.var_fb_dchi__blk968_dn6 = assign29750_body8_e42489_d_n6;
            locals.var_fb_dchi__blk968_dn7 = assign29750_body8_e42489_d_n7;
            locals.var_fb_dchi__blk968_dn10 = assign29750_body8_e42489_d_n10;
            locals.var_fb_dchi__blk968_dn11 = assign29750_body8_e42489_d_n11;
            locals.var_fb_dchi__blk968_dn12 = assign29750_body8_e42489_d_n12;
            locals.var_fb_dchi__blk968_dn17 = assign29750_body8_e42489_d_n17;
            locals.var_fb_dchi__blk968_rv = 0.0;
            let (assign29750_body9_e42512, assign29750_body9_e42512_d_n0, assign29750_body9_e42512_d_n2, assign29750_body9_e42512_d_n6, assign29750_body9_e42512_d_n7, assign29750_body9_e42512_d_n10, assign29750_body9_e42512_d_n11, assign29750_body9_e42512_d_n12, assign29750_body9_e42512_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29750_body9_e42505: f64 = (locals.var_fb__blk967 * locals.var_fb__blk967);
        let assign29750_body9_e42507: f64 = (assign29750_body9_e42505 + locals.var_fs01__blk965);
        let assign29750_body9_e42509: f64 = (assign29750_body9_e42507 + 1e-50);
        let assign29750_body9_e42510: f64 = (assign29750_body9_e42509).sqrt();
        (assign29750_body9_e42510, ((((locals.var_fb__blk967_dn0 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn0)) + locals.var_fs01__blk965_dn0) / (2.0 * assign29750_body9_e42510)), ((((locals.var_fb__blk967_dn2 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn2)) + locals.var_fs01__blk965_dn2) / (2.0 * assign29750_body9_e42510)), ((((locals.var_fb__blk967_dn6 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn6)) + locals.var_fs01__blk965_dn6) / (2.0 * assign29750_body9_e42510)), ((((locals.var_fb__blk967_dn7 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn7)) + locals.var_fs01__blk965_dn7) / (2.0 * assign29750_body9_e42510)), ((((locals.var_fb__blk967_dn10 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn10)) + locals.var_fs01__blk965_dn10) / (2.0 * assign29750_body9_e42510)), ((((locals.var_fb__blk967_dn11 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn11)) + locals.var_fs01__blk965_dn11) / (2.0 * assign29750_body9_e42510)), ((((locals.var_fb__blk967_dn12 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn12)) + locals.var_fs01__blk965_dn12) / (2.0 * assign29750_body9_e42510)), ((((locals.var_fb__blk967_dn17 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn17)) + locals.var_fs01__blk965_dn17) / (2.0 * assign29750_body9_e42510)),)
    } else {
        (locals.var_fs02__blk969, locals.var_fs02__blk969_dn0, locals.var_fs02__blk969_dn2, locals.var_fs02__blk969_dn6, locals.var_fs02__blk969_dn7, locals.var_fs02__blk969_dn10, locals.var_fs02__blk969_dn11, locals.var_fs02__blk969_dn12, locals.var_fs02__blk969_dn17,)
    }
};
            locals.var_fs02__blk969 = assign29750_body9_e42512;
            locals.var_fs02__blk969_dn0 = assign29750_body9_e42512_d_n0;
            locals.var_fs02__blk969_dn2 = assign29750_body9_e42512_d_n2;
            locals.var_fs02__blk969_dn6 = assign29750_body9_e42512_d_n6;
            locals.var_fs02__blk969_dn7 = assign29750_body9_e42512_d_n7;
            locals.var_fs02__blk969_dn10 = assign29750_body9_e42512_d_n10;
            locals.var_fs02__blk969_dn11 = assign29750_body9_e42512_d_n11;
            locals.var_fs02__blk969_dn12 = assign29750_body9_e42512_d_n12;
            locals.var_fs02__blk969_dn17 = assign29750_body9_e42512_d_n17;
            locals.var_fs02__blk969_rv = 0.0;
            let (assign29750_body10_e42540, assign29750_body10_e42540_d_n0, assign29750_body10_e42540_d_n2, assign29750_body10_e42540_d_n6, assign29750_body10_e42540_d_n7, assign29750_body10_e42540_d_n10, assign29750_body10_e42540_d_n11, assign29750_body10_e42540_d_n12, assign29750_body10_e42540_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard990 != 0.0)) {
        let assign29750_body10_e42528: f64 = (locals.var_beta * locals.var_fb_dchi__blk968);
        let assign29750_body10_e42530: f64 = (assign29750_body10_e42528 * 2.0);
        let assign29750_body10_e42532: f64 = (assign29750_body10_e42530 * locals.var_fb__blk967);
        let assign29750_body10_e42534: f64 = (assign29750_body10_e42532 + locals.var_fs01_dps0__blk966);
        let assign29750_body10_e42537: f64 = (locals.var_fs02__blk969 + locals.var_fs02__blk969);
        let assign29750_body10_e42538: f64 = (assign29750_body10_e42534 / assign29750_body10_e42537);
        (assign29750_body10_e42538, ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn0) * 2.0) * locals.var_fb__blk967) + (assign29750_body10_e42530 * locals.var_fb__blk967_dn0)) + locals.var_fs01_dps0__blk966_dn0) * assign29750_body10_e42537) - (assign29750_body10_e42534 * (locals.var_fs02__blk969_dn0 + locals.var_fs02__blk969_dn0))) / (assign29750_body10_e42537 * assign29750_body10_e42537)), ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn2) * 2.0) * locals.var_fb__blk967) + (assign29750_body10_e42530 * locals.var_fb__blk967_dn2)) + locals.var_fs01_dps0__blk966_dn2) * assign29750_body10_e42537) - (assign29750_body10_e42534 * (locals.var_fs02__blk969_dn2 + locals.var_fs02__blk969_dn2))) / (assign29750_body10_e42537 * assign29750_body10_e42537)), ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn6) * 2.0) * locals.var_fb__blk967) + (assign29750_body10_e42530 * locals.var_fb__blk967_dn6)) + locals.var_fs01_dps0__blk966_dn6) * assign29750_body10_e42537) - (assign29750_body10_e42534 * (locals.var_fs02__blk969_dn6 + locals.var_fs02__blk969_dn6))) / (assign29750_body10_e42537 * assign29750_body10_e42537)), ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn7) * 2.0) * locals.var_fb__blk967) + (assign29750_body10_e42530 * locals.var_fb__blk967_dn7)) + locals.var_fs01_dps0__blk966_dn7) * assign29750_body10_e42537) - (assign29750_body10_e42534 * (locals.var_fs02__blk969_dn7 + locals.var_fs02__blk969_dn7))) / (assign29750_body10_e42537 * assign29750_body10_e42537)), (((((((((locals.var_beta_dn10 * locals.var_fb_dchi__blk968) + (locals.var_beta * locals.var_fb_dchi__blk968_dn10)) * 2.0) * locals.var_fb__blk967) + (assign29750_body10_e42530 * locals.var_fb__blk967_dn10)) + locals.var_fs01_dps0__blk966_dn10) * assign29750_body10_e42537) - (assign29750_body10_e42534 * (locals.var_fs02__blk969_dn10 + locals.var_fs02__blk969_dn10))) / (assign29750_body10_e42537 * assign29750_body10_e42537)), ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn11) * 2.0) * locals.var_fb__blk967) + (assign29750_body10_e42530 * locals.var_fb__blk967_dn11)) + locals.var_fs01_dps0__blk966_dn11) * assign29750_body10_e42537) - (assign29750_body10_e42534 * (locals.var_fs02__blk969_dn11 + locals.var_fs02__blk969_dn11))) / (assign29750_body10_e42537 * assign29750_body10_e42537)), ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn12) * 2.0) * locals.var_fb__blk967) + (assign29750_body10_e42530 * locals.var_fb__blk967_dn12)) + locals.var_fs01_dps0__blk966_dn12) * assign29750_body10_e42537) - (assign29750_body10_e42534 * (locals.var_fs02__blk969_dn12 + locals.var_fs02__blk969_dn12))) / (assign29750_body10_e42537 * assign29750_body10_e42537)), ((((((((locals.var_beta * locals.var_fb_dchi__blk968_dn17) * 2.0) * locals.var_fb__blk967) + (assign29750_body10_e42530 * locals.var_fb__blk967_dn17)) + locals.var_fs01_dps0__blk966_dn17) * assign29750_body10_e42537) - (assign29750_body10_e42534 * (locals.var_fs02__blk969_dn17 + locals.var_fs02__blk969_dn17))) / (assign29750_body10_e42537 * assign29750_body10_e42537)),)
    } else {
        (locals.var_fs02_dps0__blk970, locals.var_fs02_dps0__blk970_dn0, locals.var_fs02_dps0__blk970_dn2, locals.var_fs02_dps0__blk970_dn6, locals.var_fs02_dps0__blk970_dn7, locals.var_fs02_dps0__blk970_dn10, locals.var_fs02_dps0__blk970_dn11, locals.var_fs02_dps0__blk970_dn12, locals.var_fs02_dps0__blk970_dn17,)
    }
};
            locals.var_fs02_dps0__blk970 = assign29750_body10_e42540;
            locals.var_fs02_dps0__blk970_dn0 = assign29750_body10_e42540_d_n0;
            locals.var_fs02_dps0__blk970_dn2 = assign29750_body10_e42540_d_n2;
            locals.var_fs02_dps0__blk970_dn6 = assign29750_body10_e42540_d_n6;
            locals.var_fs02_dps0__blk970_dn7 = assign29750_body10_e42540_d_n7;
            locals.var_fs02_dps0__blk970_dn10 = assign29750_body10_e42540_d_n10;
            locals.var_fs02_dps0__blk970_dn11 = assign29750_body10_e42540_d_n11;
            locals.var_fs02_dps0__blk970_dn12 = assign29750_body10_e42540_d_n12;
            locals.var_fs02_dps0__blk970_dn17 = assign29750_body10_e42540_d_n17;
            locals.var_fs02_dps0__blk970_rv = 0.0;
            let assign29750_body11_e42543: f64 = if locals.var_chi__blk943 < 80.0 { 1.0 } else { 0.0 };
            locals.var_guard991 = assign29750_body11_e42543;
            locals.var_guard991_rv = 0.0;
            let (assign29750_body12_e42563, assign29750_body12_e42563_d_n0, assign29750_body12_e42563_d_n2, assign29750_body12_e42563_d_n6, assign29750_body12_e42563_d_n7, assign29750_body12_e42563_d_n10, assign29750_body12_e42563_d_n11, assign29750_body12_e42563_d_n12, assign29750_body12_e42563_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard990 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29750_body12_e42561: f64 = (locals.var_chi__blk943).exp();
        (assign29750_body12_e42561, (assign29750_body12_e42561 * locals.var_chi__blk943_dn0), (assign29750_body12_e42561 * locals.var_chi__blk943_dn2), (assign29750_body12_e42561 * locals.var_chi__blk943_dn6), (assign29750_body12_e42561 * locals.var_chi__blk943_dn7), (assign29750_body12_e42561 * locals.var_chi__blk943_dn10), (assign29750_body12_e42561 * locals.var_chi__blk943_dn11), (assign29750_body12_e42561 * locals.var_chi__blk943_dn12), (assign29750_body12_e42561 * locals.var_chi__blk943_dn17),)
    } else {
        (locals.var_exp_chi, locals.var_exp_chi_dn0, locals.var_exp_chi_dn2, locals.var_exp_chi_dn6, locals.var_exp_chi_dn7, locals.var_exp_chi_dn10, locals.var_exp_chi_dn11, locals.var_exp_chi_dn12, locals.var_exp_chi_dn17,)
    }
};
            locals.var_exp_chi = assign29750_body12_e42563;
            locals.var_exp_chi_dn0 = assign29750_body12_e42563_d_n0;
            locals.var_exp_chi_dn2 = assign29750_body12_e42563_d_n2;
            locals.var_exp_chi_dn6 = assign29750_body12_e42563_d_n6;
            locals.var_exp_chi_dn7 = assign29750_body12_e42563_d_n7;
            locals.var_exp_chi_dn10 = assign29750_body12_e42563_d_n10;
            locals.var_exp_chi_dn11 = assign29750_body12_e42563_d_n11;
            locals.var_exp_chi_dn12 = assign29750_body12_e42563_d_n12;
            locals.var_exp_chi_dn17 = assign29750_body12_e42563_d_n17;
            locals.var_exp_chi_rv = 0.0;
            let (assign29750_body13_e42586, assign29750_body13_e42586_d_n0, assign29750_body13_e42586_d_n2, assign29750_body13_e42586_d_n6, assign29750_body13_e42586_d_n7, assign29750_body13_e42586_d_n10, assign29750_body13_e42586_d_n11, assign29750_body13_e42586_d_n12, assign29750_body13_e42586_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard990 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29750_body13_e42583: f64 = (locals.var_exp_chi - 1.0);
        let assign29750_body13_e42584: f64 = (locals.var_cfs1__blk971 * assign29750_body13_e42583);
        (assign29750_body13_e42584, ((locals.var_cfs1__blk971_dn0 * assign29750_body13_e42583) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn0)), ((locals.var_cfs1__blk971_dn2 * assign29750_body13_e42583) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn2)), ((locals.var_cfs1__blk971_dn6 * assign29750_body13_e42583) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn6)), ((locals.var_cfs1__blk971_dn7 * assign29750_body13_e42583) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn7)), ((locals.var_cfs1__blk971_dn10 * assign29750_body13_e42583) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn10)), ((locals.var_cfs1__blk971_dn11 * assign29750_body13_e42583) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn11)), ((locals.var_cfs1__blk971_dn12 * assign29750_body13_e42583) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn12)), ((locals.var_cfs1__blk971_dn17 * assign29750_body13_e42583) + (locals.var_cfs1__blk971 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01__blk965, locals.var_fs01__blk965_dn0, locals.var_fs01__blk965_dn2, locals.var_fs01__blk965_dn6, locals.var_fs01__blk965_dn7, locals.var_fs01__blk965_dn10, locals.var_fs01__blk965_dn11, locals.var_fs01__blk965_dn12, locals.var_fs01__blk965_dn17,)
    }
};
            locals.var_fs01__blk965 = assign29750_body13_e42586;
            locals.var_fs01__blk965_dn0 = assign29750_body13_e42586_d_n0;
            locals.var_fs01__blk965_dn2 = assign29750_body13_e42586_d_n2;
            locals.var_fs01__blk965_dn6 = assign29750_body13_e42586_d_n6;
            locals.var_fs01__blk965_dn7 = assign29750_body13_e42586_d_n7;
            locals.var_fs01__blk965_dn10 = assign29750_body13_e42586_d_n10;
            locals.var_fs01__blk965_dn11 = assign29750_body13_e42586_d_n11;
            locals.var_fs01__blk965_dn12 = assign29750_body13_e42586_d_n12;
            locals.var_fs01__blk965_dn17 = assign29750_body13_e42586_d_n17;
            locals.var_fs01__blk965_rv = 0.0;
            let (assign29750_body14_e42609, assign29750_body14_e42609_d_n0, assign29750_body14_e42609_d_n2, assign29750_body14_e42609_d_n6, assign29750_body14_e42609_d_n7, assign29750_body14_e42609_d_n10, assign29750_body14_e42609_d_n11, assign29750_body14_e42609_d_n12, assign29750_body14_e42609_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard990 == 0.0)) && (locals.var_guard991 != 0.0)) {
        let assign29750_body14_e42605: f64 = (locals.var_cfs1__blk971 * locals.var_beta);
        let assign29750_body14_e42607: f64 = (assign29750_body14_e42605 * locals.var_exp_chi);
        (assign29750_body14_e42607, (((locals.var_cfs1__blk971_dn0 * locals.var_beta) * locals.var_exp_chi) + (assign29750_body14_e42605 * locals.var_exp_chi_dn0)), (((locals.var_cfs1__blk971_dn2 * locals.var_beta) * locals.var_exp_chi) + (assign29750_body14_e42605 * locals.var_exp_chi_dn2)), (((locals.var_cfs1__blk971_dn6 * locals.var_beta) * locals.var_exp_chi) + (assign29750_body14_e42605 * locals.var_exp_chi_dn6)), (((locals.var_cfs1__blk971_dn7 * locals.var_beta) * locals.var_exp_chi) + (assign29750_body14_e42605 * locals.var_exp_chi_dn7)), ((((locals.var_cfs1__blk971_dn10 * locals.var_beta) + (locals.var_cfs1__blk971 * locals.var_beta_dn10)) * locals.var_exp_chi) + (assign29750_body14_e42605 * locals.var_exp_chi_dn10)), (((locals.var_cfs1__blk971_dn11 * locals.var_beta) * locals.var_exp_chi) + (assign29750_body14_e42605 * locals.var_exp_chi_dn11)), (((locals.var_cfs1__blk971_dn12 * locals.var_beta) * locals.var_exp_chi) + (assign29750_body14_e42605 * locals.var_exp_chi_dn12)), (((locals.var_cfs1__blk971_dn17 * locals.var_beta) * locals.var_exp_chi) + (assign29750_body14_e42605 * locals.var_exp_chi_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk966, locals.var_fs01_dps0__blk966_dn0, locals.var_fs01_dps0__blk966_dn2, locals.var_fs01_dps0__blk966_dn6, locals.var_fs01_dps0__blk966_dn7, locals.var_fs01_dps0__blk966_dn10, locals.var_fs01_dps0__blk966_dn11, locals.var_fs01_dps0__blk966_dn12, locals.var_fs01_dps0__blk966_dn17,)
    }
};
            locals.var_fs01_dps0__blk966 = assign29750_body14_e42609;
            locals.var_fs01_dps0__blk966_dn0 = assign29750_body14_e42609_d_n0;
            locals.var_fs01_dps0__blk966_dn2 = assign29750_body14_e42609_d_n2;
            locals.var_fs01_dps0__blk966_dn6 = assign29750_body14_e42609_d_n6;
            locals.var_fs01_dps0__blk966_dn7 = assign29750_body14_e42609_d_n7;
            locals.var_fs01_dps0__blk966_dn10 = assign29750_body14_e42609_d_n10;
            locals.var_fs01_dps0__blk966_dn11 = assign29750_body14_e42609_d_n11;
            locals.var_fs01_dps0__blk966_dn12 = assign29750_body14_e42609_d_n12;
            locals.var_fs01_dps0__blk966_dn17 = assign29750_body14_e42609_d_n17;
            locals.var_fs01_dps0__blk966_rv = 0.0;
            let (assign29750_body15_e42632, assign29750_body15_e42632_d_n0, assign29750_body15_e42632_d_n2, assign29750_body15_e42632_d_n6, assign29750_body15_e42632_d_n7, assign29750_body15_e42632_d_n10, assign29750_body15_e42632_d_n11, assign29750_body15_e42632_d_n12, assign29750_body15_e42632_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard990 == 0.0)) && (locals.var_guard991 == 0.0)) {
        let assign29750_body15_e42629: f64 = (locals.var_beta * locals.var_ps0ld__blk945);
        let assign29750_body15_e42630: f64 = (assign29750_body15_e42629).exp();
        (assign29750_body15_e42630, (assign29750_body15_e42630 * (locals.var_beta * locals.var_ps0ld__blk945_dn0)), (assign29750_body15_e42630 * (locals.var_beta * locals.var_ps0ld__blk945_dn2)), (assign29750_body15_e42630 * (locals.var_beta * locals.var_ps0ld__blk945_dn6)), (assign29750_body15_e42630 * (locals.var_beta * locals.var_ps0ld__blk945_dn7)), (assign29750_body15_e42630 * ((locals.var_beta_dn10 * locals.var_ps0ld__blk945) + (locals.var_beta * locals.var_ps0ld__blk945_dn10))), (assign29750_body15_e42630 * (locals.var_beta * locals.var_ps0ld__blk945_dn11)), (assign29750_body15_e42630 * (locals.var_beta * locals.var_ps0ld__blk945_dn12)), (assign29750_body15_e42630 * (locals.var_beta * locals.var_ps0ld__blk945_dn17)),)
    } else {
        (locals.var_exp_bps0__blk972, locals.var_exp_bps0__blk972_dn0, locals.var_exp_bps0__blk972_dn2, locals.var_exp_bps0__blk972_dn6, locals.var_exp_bps0__blk972_dn7, locals.var_exp_bps0__blk972_dn10, locals.var_exp_bps0__blk972_dn11, locals.var_exp_bps0__blk972_dn12, locals.var_exp_bps0__blk972_dn17,)
    }
};
            locals.var_exp_bps0__blk972 = assign29750_body15_e42632;
            locals.var_exp_bps0__blk972_dn0 = assign29750_body15_e42632_d_n0;
            locals.var_exp_bps0__blk972_dn2 = assign29750_body15_e42632_d_n2;
            locals.var_exp_bps0__blk972_dn6 = assign29750_body15_e42632_d_n6;
            locals.var_exp_bps0__blk972_dn7 = assign29750_body15_e42632_d_n7;
            locals.var_exp_bps0__blk972_dn10 = assign29750_body15_e42632_d_n10;
            locals.var_exp_bps0__blk972_dn11 = assign29750_body15_e42632_d_n11;
            locals.var_exp_bps0__blk972_dn12 = assign29750_body15_e42632_d_n12;
            locals.var_exp_bps0__blk972_dn17 = assign29750_body15_e42632_d_n17;
            locals.var_exp_bps0__blk972_rv = 0.0;
            let (assign29750_body16_e42656, assign29750_body16_e42656_d_n0, assign29750_body16_e42656_d_n2, assign29750_body16_e42656_d_n6, assign29750_body16_e42656_d_n7, assign29750_body16_e42656_d_n10, assign29750_body16_e42656_d_n11, assign29750_body16_e42656_d_n12, assign29750_body16_e42656_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard990 == 0.0)) && (locals.var_guard991 == 0.0)) {
        let assign29750_body16_e42653: f64 = (locals.var_exp_bps0__blk972 - locals.var_exp_bvbs__blk962);
        let assign29750_body16_e42654: f64 = (locals.var_cnst1over__blk956 * assign29750_body16_e42653);
        (assign29750_body16_e42654, ((locals.var_cnst1over__blk956_dn0 * assign29750_body16_e42653) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn0 - locals.var_exp_bvbs__blk962_dn0))), ((locals.var_cnst1over__blk956_dn2 * assign29750_body16_e42653) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn2 - locals.var_exp_bvbs__blk962_dn2))), ((locals.var_cnst1over__blk956_dn6 * assign29750_body16_e42653) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn6 - locals.var_exp_bvbs__blk962_dn6))), ((locals.var_cnst1over__blk956_dn7 * assign29750_body16_e42653) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn7 - locals.var_exp_bvbs__blk962_dn7))), ((locals.var_cnst1over__blk956_dn10 * assign29750_body16_e42653) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn10 - locals.var_exp_bvbs__blk962_dn10))), ((locals.var_cnst1over__blk956_dn11 * assign29750_body16_e42653) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn11 - locals.var_exp_bvbs__blk962_dn11))), ((locals.var_cnst1over__blk956_dn12 * assign29750_body16_e42653) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn12 - locals.var_exp_bvbs__blk962_dn12))), ((locals.var_cnst1over__blk956_dn17 * assign29750_body16_e42653) + (locals.var_cnst1over__blk956 * (locals.var_exp_bps0__blk972_dn17 - locals.var_exp_bvbs__blk962_dn17))),)
    } else {
        (locals.var_fs01__blk965, locals.var_fs01__blk965_dn0, locals.var_fs01__blk965_dn2, locals.var_fs01__blk965_dn6, locals.var_fs01__blk965_dn7, locals.var_fs01__blk965_dn10, locals.var_fs01__blk965_dn11, locals.var_fs01__blk965_dn12, locals.var_fs01__blk965_dn17,)
    }
};
            locals.var_fs01__blk965 = assign29750_body16_e42656;
            locals.var_fs01__blk965_dn0 = assign29750_body16_e42656_d_n0;
            locals.var_fs01__blk965_dn2 = assign29750_body16_e42656_d_n2;
            locals.var_fs01__blk965_dn6 = assign29750_body16_e42656_d_n6;
            locals.var_fs01__blk965_dn7 = assign29750_body16_e42656_d_n7;
            locals.var_fs01__blk965_dn10 = assign29750_body16_e42656_d_n10;
            locals.var_fs01__blk965_dn11 = assign29750_body16_e42656_d_n11;
            locals.var_fs01__blk965_dn12 = assign29750_body16_e42656_d_n12;
            locals.var_fs01__blk965_dn17 = assign29750_body16_e42656_d_n17;
            locals.var_fs01__blk965_rv = 0.0;
            let (assign29750_body17_e42680, assign29750_body17_e42680_d_n0, assign29750_body17_e42680_d_n2, assign29750_body17_e42680_d_n6, assign29750_body17_e42680_d_n7, assign29750_body17_e42680_d_n10, assign29750_body17_e42680_d_n11, assign29750_body17_e42680_d_n12, assign29750_body17_e42680_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard990 == 0.0)) && (locals.var_guard991 == 0.0)) {
        let assign29750_body17_e42676: f64 = (locals.var_cnst1over__blk956 * locals.var_beta);
        let assign29750_body17_e42678: f64 = (assign29750_body17_e42676 * locals.var_exp_bps0__blk972);
        (assign29750_body17_e42678, (((locals.var_cnst1over__blk956_dn0 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign29750_body17_e42676 * locals.var_exp_bps0__blk972_dn0)), (((locals.var_cnst1over__blk956_dn2 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign29750_body17_e42676 * locals.var_exp_bps0__blk972_dn2)), (((locals.var_cnst1over__blk956_dn6 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign29750_body17_e42676 * locals.var_exp_bps0__blk972_dn6)), (((locals.var_cnst1over__blk956_dn7 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign29750_body17_e42676 * locals.var_exp_bps0__blk972_dn7)), ((((locals.var_cnst1over__blk956_dn10 * locals.var_beta) + (locals.var_cnst1over__blk956 * locals.var_beta_dn10)) * locals.var_exp_bps0__blk972) + (assign29750_body17_e42676 * locals.var_exp_bps0__blk972_dn10)), (((locals.var_cnst1over__blk956_dn11 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign29750_body17_e42676 * locals.var_exp_bps0__blk972_dn11)), (((locals.var_cnst1over__blk956_dn12 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign29750_body17_e42676 * locals.var_exp_bps0__blk972_dn12)), (((locals.var_cnst1over__blk956_dn17 * locals.var_beta) * locals.var_exp_bps0__blk972) + (assign29750_body17_e42676 * locals.var_exp_bps0__blk972_dn17)),)
    } else {
        (locals.var_fs01_dps0__blk966, locals.var_fs01_dps0__blk966_dn0, locals.var_fs01_dps0__blk966_dn2, locals.var_fs01_dps0__blk966_dn6, locals.var_fs01_dps0__blk966_dn7, locals.var_fs01_dps0__blk966_dn10, locals.var_fs01_dps0__blk966_dn11, locals.var_fs01_dps0__blk966_dn12, locals.var_fs01_dps0__blk966_dn17,)
    }
};
            locals.var_fs01_dps0__blk966 = assign29750_body17_e42680;
            locals.var_fs01_dps0__blk966_dn0 = assign29750_body17_e42680_d_n0;
            locals.var_fs01_dps0__blk966_dn2 = assign29750_body17_e42680_d_n2;
            locals.var_fs01_dps0__blk966_dn6 = assign29750_body17_e42680_d_n6;
            locals.var_fs01_dps0__blk966_dn7 = assign29750_body17_e42680_d_n7;
            locals.var_fs01_dps0__blk966_dn10 = assign29750_body17_e42680_d_n10;
            locals.var_fs01_dps0__blk966_dn11 = assign29750_body17_e42680_d_n11;
            locals.var_fs01_dps0__blk966_dn12 = assign29750_body17_e42680_d_n12;
            locals.var_fs01_dps0__blk966_dn17 = assign29750_body17_e42680_d_n17;
            locals.var_fs01_dps0__blk966_rv = 0.0;
            let (assign29750_body18_e42702, assign29750_body18_e42702_d_n0, assign29750_body18_e42702_d_n2, assign29750_body18_e42702_d_n6, assign29750_body18_e42702_d_n7, assign29750_body18_e42702_d_n10, assign29750_body18_e42702_d_n11, assign29750_body18_e42702_d_n12, assign29750_body18_e42702_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard990 == 0.0)) {
        let assign29750_body18_e42697: f64 = (locals.var_chi__blk943 - 1.0);
        let assign29750_body18_e42699: f64 = (assign29750_body18_e42697 + locals.var_fs01__blk965);
        let assign29750_body18_e42700: f64 = (assign29750_body18_e42699).sqrt();
        (assign29750_body18_e42700, ((locals.var_chi__blk943_dn0 + locals.var_fs01__blk965_dn0) / (2.0 * assign29750_body18_e42700)), ((locals.var_chi__blk943_dn2 + locals.var_fs01__blk965_dn2) / (2.0 * assign29750_body18_e42700)), ((locals.var_chi__blk943_dn6 + locals.var_fs01__blk965_dn6) / (2.0 * assign29750_body18_e42700)), ((locals.var_chi__blk943_dn7 + locals.var_fs01__blk965_dn7) / (2.0 * assign29750_body18_e42700)), ((locals.var_chi__blk943_dn10 + locals.var_fs01__blk965_dn10) / (2.0 * assign29750_body18_e42700)), ((locals.var_chi__blk943_dn11 + locals.var_fs01__blk965_dn11) / (2.0 * assign29750_body18_e42700)), ((locals.var_chi__blk943_dn12 + locals.var_fs01__blk965_dn12) / (2.0 * assign29750_body18_e42700)), ((locals.var_chi__blk943_dn17 + locals.var_fs01__blk965_dn17) / (2.0 * assign29750_body18_e42700)),)
    } else {
        (locals.var_fs02__blk969, locals.var_fs02__blk969_dn0, locals.var_fs02__blk969_dn2, locals.var_fs02__blk969_dn6, locals.var_fs02__blk969_dn7, locals.var_fs02__blk969_dn10, locals.var_fs02__blk969_dn11, locals.var_fs02__blk969_dn12, locals.var_fs02__blk969_dn17,)
    }
};
            locals.var_fs02__blk969 = assign29750_body18_e42702;
            locals.var_fs02__blk969_dn0 = assign29750_body18_e42702_d_n0;
            locals.var_fs02__blk969_dn2 = assign29750_body18_e42702_d_n2;
            locals.var_fs02__blk969_dn6 = assign29750_body18_e42702_d_n6;
            locals.var_fs02__blk969_dn7 = assign29750_body18_e42702_d_n7;
            locals.var_fs02__blk969_dn10 = assign29750_body18_e42702_d_n10;
            locals.var_fs02__blk969_dn11 = assign29750_body18_e42702_d_n11;
            locals.var_fs02__blk969_dn12 = assign29750_body18_e42702_d_n12;
            locals.var_fs02__blk969_dn17 = assign29750_body18_e42702_d_n17;
            locals.var_fs02__blk969_rv = 0.0;
            let (assign29750_body19_e42725, assign29750_body19_e42725_d_n0, assign29750_body19_e42725_d_n2, assign29750_body19_e42725_d_n6, assign29750_body19_e42725_d_n7, assign29750_body19_e42725_d_n10, assign29750_body19_e42725_d_n11, assign29750_body19_e42725_d_n12, assign29750_body19_e42725_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard990 == 0.0)) {
        let assign29750_body19_e42719: f64 = (locals.var_beta + locals.var_fs01_dps0__blk966);
        let assign29750_body19_e42721: f64 = (assign29750_body19_e42719 / locals.var_fs02__blk969);
        let assign29750_body19_e42723: f64 = (assign29750_body19_e42721 * 0.5);
        (assign29750_body19_e42723, ((((locals.var_fs01_dps0__blk966_dn0 * locals.var_fs02__blk969) - (assign29750_body19_e42719 * locals.var_fs02__blk969_dn0)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), ((((locals.var_fs01_dps0__blk966_dn2 * locals.var_fs02__blk969) - (assign29750_body19_e42719 * locals.var_fs02__blk969_dn2)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), ((((locals.var_fs01_dps0__blk966_dn6 * locals.var_fs02__blk969) - (assign29750_body19_e42719 * locals.var_fs02__blk969_dn6)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), ((((locals.var_fs01_dps0__blk966_dn7 * locals.var_fs02__blk969) - (assign29750_body19_e42719 * locals.var_fs02__blk969_dn7)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), (((((locals.var_beta_dn10 + locals.var_fs01_dps0__blk966_dn10) * locals.var_fs02__blk969) - (assign29750_body19_e42719 * locals.var_fs02__blk969_dn10)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), ((((locals.var_fs01_dps0__blk966_dn11 * locals.var_fs02__blk969) - (assign29750_body19_e42719 * locals.var_fs02__blk969_dn11)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), ((((locals.var_fs01_dps0__blk966_dn12 * locals.var_fs02__blk969) - (assign29750_body19_e42719 * locals.var_fs02__blk969_dn12)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5), ((((locals.var_fs01_dps0__blk966_dn17 * locals.var_fs02__blk969) - (assign29750_body19_e42719 * locals.var_fs02__blk969_dn17)) / (locals.var_fs02__blk969 * locals.var_fs02__blk969)) * 0.5),)
    } else {
        (locals.var_fs02_dps0__blk970, locals.var_fs02_dps0__blk970_dn0, locals.var_fs02_dps0__blk970_dn2, locals.var_fs02_dps0__blk970_dn6, locals.var_fs02_dps0__blk970_dn7, locals.var_fs02_dps0__blk970_dn10, locals.var_fs02_dps0__blk970_dn11, locals.var_fs02_dps0__blk970_dn12, locals.var_fs02_dps0__blk970_dn17,)
    }
};
            locals.var_fs02_dps0__blk970 = assign29750_body19_e42725;
            locals.var_fs02_dps0__blk970_dn0 = assign29750_body19_e42725_d_n0;
            locals.var_fs02_dps0__blk970_dn2 = assign29750_body19_e42725_d_n2;
            locals.var_fs02_dps0__blk970_dn6 = assign29750_body19_e42725_d_n6;
            locals.var_fs02_dps0__blk970_dn7 = assign29750_body19_e42725_d_n7;
            locals.var_fs02_dps0__blk970_dn10 = assign29750_body19_e42725_d_n10;
            locals.var_fs02_dps0__blk970_dn11 = assign29750_body19_e42725_d_n11;
            locals.var_fs02_dps0__blk970_dn12 = assign29750_body19_e42725_d_n12;
            locals.var_fs02_dps0__blk970_dn17 = assign29750_body19_e42725_d_n17;
            locals.var_fs02_dps0__blk970_rv = 0.0;
            let (assign29750_body20_e42745, assign29750_body20_e42745_d_n0, assign29750_body20_e42745_d_n2, assign29750_body20_e42745_d_n6, assign29750_body20_e42745_d_n7, assign29750_body20_e42745_d_n10, assign29750_body20_e42745_d_n11, assign29750_body20_e42745_d_n12, assign29750_body20_e42745_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29750_body20_e42739: f64 = (locals.var_vgpld__blk931 - locals.var_ps0ld__blk945);
        let assign29750_body20_e42742: f64 = (locals.var_fac1__blk929 * locals.var_fs02__blk969);
        let assign29750_body20_e42743: f64 = (assign29750_body20_e42739 - assign29750_body20_e42742);
        (assign29750_body20_e42743, ((locals.var_vgpld__blk931_dn0 - locals.var_ps0ld__blk945_dn0) - ((locals.var_fac1__blk929_dn0 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn0))), ((locals.var_vgpld__blk931_dn2 - locals.var_ps0ld__blk945_dn2) - ((locals.var_fac1__blk929_dn2 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn2))), ((locals.var_vgpld__blk931_dn6 - locals.var_ps0ld__blk945_dn6) - ((locals.var_fac1__blk929_dn6 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn6))), ((locals.var_vgpld__blk931_dn7 - locals.var_ps0ld__blk945_dn7) - ((locals.var_fac1__blk929_dn7 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn7))), ((locals.var_vgpld__blk931_dn10 - locals.var_ps0ld__blk945_dn10) - ((locals.var_fac1__blk929_dn10 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn10))), ((locals.var_vgpld__blk931_dn11 - locals.var_ps0ld__blk945_dn11) - ((locals.var_fac1__blk929_dn11 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn11))), ((locals.var_vgpld__blk931_dn12 - locals.var_ps0ld__blk945_dn12) - ((locals.var_fac1__blk929_dn12 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn12))), ((locals.var_vgpld__blk931_dn17 - locals.var_ps0ld__blk945_dn17) - ((locals.var_fac1__blk929_dn17 * locals.var_fs02__blk969) + (locals.var_fac1__blk929 * locals.var_fs02__blk969_dn17))),)
    } else {
        (locals.var_fs0__blk973, locals.var_fs0__blk973_dn0, locals.var_fs0__blk973_dn2, locals.var_fs0__blk973_dn6, locals.var_fs0__blk973_dn7, locals.var_fs0__blk973_dn10, locals.var_fs0__blk973_dn11, locals.var_fs0__blk973_dn12, locals.var_fs0__blk973_dn17,)
    }
};
            locals.var_fs0__blk973 = assign29750_body20_e42745;
            locals.var_fs0__blk973_dn0 = assign29750_body20_e42745_d_n0;
            locals.var_fs0__blk973_dn2 = assign29750_body20_e42745_d_n2;
            locals.var_fs0__blk973_dn6 = assign29750_body20_e42745_d_n6;
            locals.var_fs0__blk973_dn7 = assign29750_body20_e42745_d_n7;
            locals.var_fs0__blk973_dn10 = assign29750_body20_e42745_d_n10;
            locals.var_fs0__blk973_dn11 = assign29750_body20_e42745_d_n11;
            locals.var_fs0__blk973_dn12 = assign29750_body20_e42745_d_n12;
            locals.var_fs0__blk973_dn17 = assign29750_body20_e42745_d_n17;
            locals.var_fs0__blk973_rv = 0.0;
            let (assign29750_body21_e42764, assign29750_body21_e42764_d_n0, assign29750_body21_e42764_d_n2, assign29750_body21_e42764_d_n6, assign29750_body21_e42764_d_n7, assign29750_body21_e42764_d_n10, assign29750_body21_e42764_d_n11, assign29750_body21_e42764_d_n12, assign29750_body21_e42764_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29750_body21_e42758: f64 = (-1.0);
        let assign29750_body21_e42761: f64 = (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970);
        let assign29750_body21_e42762: f64 = (assign29750_body21_e42758 - assign29750_body21_e42761);
        (assign29750_body21_e42762, (-((locals.var_fac1__blk929_dn0 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn0))), (-((locals.var_fac1__blk929_dn2 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn2))), (-((locals.var_fac1__blk929_dn6 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn6))), (-((locals.var_fac1__blk929_dn7 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn7))), (-((locals.var_fac1__blk929_dn10 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn10))), (-((locals.var_fac1__blk929_dn11 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn11))), (-((locals.var_fac1__blk929_dn12 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn12))), (-((locals.var_fac1__blk929_dn17 * locals.var_fs02_dps0__blk970) + (locals.var_fac1__blk929 * locals.var_fs02_dps0__blk970_dn17))),)
    } else {
        (locals.var_fs0_dps0__blk974, locals.var_fs0_dps0__blk974_dn0, locals.var_fs0_dps0__blk974_dn2, locals.var_fs0_dps0__blk974_dn6, locals.var_fs0_dps0__blk974_dn7, locals.var_fs0_dps0__blk974_dn10, locals.var_fs0_dps0__blk974_dn11, locals.var_fs0_dps0__blk974_dn12, locals.var_fs0_dps0__blk974_dn17,)
    }
};
            locals.var_fs0_dps0__blk974 = assign29750_body21_e42764;
            locals.var_fs0_dps0__blk974_dn0 = assign29750_body21_e42764_d_n0;
            locals.var_fs0_dps0__blk974_dn2 = assign29750_body21_e42764_d_n2;
            locals.var_fs0_dps0__blk974_dn6 = assign29750_body21_e42764_d_n6;
            locals.var_fs0_dps0__blk974_dn7 = assign29750_body21_e42764_d_n7;
            locals.var_fs0_dps0__blk974_dn10 = assign29750_body21_e42764_d_n10;
            locals.var_fs0_dps0__blk974_dn11 = assign29750_body21_e42764_d_n11;
            locals.var_fs0_dps0__blk974_dn12 = assign29750_body21_e42764_d_n12;
            locals.var_fs0_dps0__blk974_dn17 = assign29750_body21_e42764_d_n17;
            locals.var_fs0_dps0__blk974_rv = 0.0;
            let assign29750_body22_e42767: f64 = if locals.var_flg_conv__blk918 == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard992 = assign29750_body22_e42767;
            locals.var_guard992_rv = 0.0;
            let (assign29750_body23_e42787,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard992 != 0.0)) {
        let assign29750_body23_e42783: f64 = (2.0 * 20.0);
        let assign29750_body23_e42785: f64 = (assign29750_body23_e42783 + 1.0);
        (assign29750_body23_e42785,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign29750_body23_e42787;
            locals.var_lp_s0_rv = 0.0;
            let (assign29750_body24_e42807, assign29750_body24_e42807_d_n0, assign29750_body24_e42807_d_n2, assign29750_body24_e42807_d_n6, assign29750_body24_e42807_d_n7, assign29750_body24_e42807_d_n10, assign29750_body24_e42807_d_n11, assign29750_body24_e42807_d_n12, assign29750_body24_e42807_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard992 == 0.0)) {
        let assign29750_body24_e42803: f64 = (-locals.var_fs0__blk973);
        let assign29750_body24_e42805: f64 = (assign29750_body24_e42803 / locals.var_fs0_dps0__blk974);
        (assign29750_body24_e42805, ((((-locals.var_fs0__blk973_dn0) * locals.var_fs0_dps0__blk974) - (assign29750_body24_e42803 * locals.var_fs0_dps0__blk974_dn0)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn2) * locals.var_fs0_dps0__blk974) - (assign29750_body24_e42803 * locals.var_fs0_dps0__blk974_dn2)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn6) * locals.var_fs0_dps0__blk974) - (assign29750_body24_e42803 * locals.var_fs0_dps0__blk974_dn6)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn7) * locals.var_fs0_dps0__blk974) - (assign29750_body24_e42803 * locals.var_fs0_dps0__blk974_dn7)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn10) * locals.var_fs0_dps0__blk974) - (assign29750_body24_e42803 * locals.var_fs0_dps0__blk974_dn10)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn11) * locals.var_fs0_dps0__blk974) - (assign29750_body24_e42803 * locals.var_fs0_dps0__blk974_dn11)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn12) * locals.var_fs0_dps0__blk974) - (assign29750_body24_e42803 * locals.var_fs0_dps0__blk974_dn12)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)), ((((-locals.var_fs0__blk973_dn17) * locals.var_fs0_dps0__blk974) - (assign29750_body24_e42803 * locals.var_fs0_dps0__blk974_dn17)) / (locals.var_fs0_dps0__blk974 * locals.var_fs0_dps0__blk974)),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign29750_body24_e42807;
            locals.var_dps0_dn0 = assign29750_body24_e42807_d_n0;
            locals.var_dps0_dn2 = assign29750_body24_e42807_d_n2;
            locals.var_dps0_dn6 = assign29750_body24_e42807_d_n6;
            locals.var_dps0_dn7 = assign29750_body24_e42807_d_n7;
            locals.var_dps0_dn10 = assign29750_body24_e42807_d_n10;
            locals.var_dps0_dn11 = assign29750_body24_e42807_d_n11;
            locals.var_dps0_dn12 = assign29750_body24_e42807_d_n12;
            locals.var_dps0_dn17 = assign29750_body24_e42807_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign29750_body25_e42837, assign29750_body25_e42837_d_n0, assign29750_body25_e42837_d_n2, assign29750_body25_e42837_d_n6, assign29750_body25_e42837_d_n7, assign29750_body25_e42837_d_n10, assign29750_body25_e42837_d_n11, assign29750_body25_e42837_d_n12, assign29750_body25_e42837_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard992 == 0.0)) {
        let assign29750_body25_e42824: f64 = (0.5 * 0.1);
        let assign29750_body25_e42828: f64 = (locals.var_ps0ld__blk945).abs();
        let (assign29750_body25_e42833, assign29750_body25_e42833_d_n0, assign29750_body25_e42833_d_n2, assign29750_body25_e42833_d_n6, assign29750_body25_e42833_d_n7, assign29750_body25_e42833_d_n10, assign29750_body25_e42833_d_n11, assign29750_body25_e42833_d_n12, assign29750_body25_e42833_d_n17,) = {
            if (1.0 >= assign29750_body25_e42828) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign29750_body25_e42832: f64 = (locals.var_ps0ld__blk945).abs();
                (assign29750_body25_e42832, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn0 } else { (-locals.var_ps0ld__blk945_dn0) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn2 } else { (-locals.var_ps0ld__blk945_dn2) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn6 } else { (-locals.var_ps0ld__blk945_dn6) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn7 } else { (-locals.var_ps0ld__blk945_dn7) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn10 } else { (-locals.var_ps0ld__blk945_dn10) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn11 } else { (-locals.var_ps0ld__blk945_dn11) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn12 } else { (-locals.var_ps0ld__blk945_dn12) }, if locals.var_ps0ld__blk945 >= 0.0 { locals.var_ps0ld__blk945_dn17 } else { (-locals.var_ps0ld__blk945_dn17) },)
            }
        };
        let assign29750_body25_e42834: f64 = (1.0 + assign29750_body25_e42833);
        let assign29750_body25_e42835: f64 = (assign29750_body25_e42824 * assign29750_body25_e42834);
        (assign29750_body25_e42835, (assign29750_body25_e42824 * assign29750_body25_e42833_d_n0), (assign29750_body25_e42824 * assign29750_body25_e42833_d_n2), (assign29750_body25_e42824 * assign29750_body25_e42833_d_n6), (assign29750_body25_e42824 * assign29750_body25_e42833_d_n7), (assign29750_body25_e42824 * assign29750_body25_e42833_d_n10), (assign29750_body25_e42824 * assign29750_body25_e42833_d_n11), (assign29750_body25_e42824 * assign29750_body25_e42833_d_n12), (assign29750_body25_e42824 * assign29750_body25_e42833_d_n17),)
    } else {
        (locals.var_dplim__blk975, locals.var_dplim__blk975_dn0, locals.var_dplim__blk975_dn2, locals.var_dplim__blk975_dn6, locals.var_dplim__blk975_dn7, locals.var_dplim__blk975_dn10, locals.var_dplim__blk975_dn11, locals.var_dplim__blk975_dn12, locals.var_dplim__blk975_dn17,)
    }
};
            locals.var_dplim__blk975 = assign29750_body25_e42837;
            locals.var_dplim__blk975_dn0 = assign29750_body25_e42837_d_n0;
            locals.var_dplim__blk975_dn2 = assign29750_body25_e42837_d_n2;
            locals.var_dplim__blk975_dn6 = assign29750_body25_e42837_d_n6;
            locals.var_dplim__blk975_dn7 = assign29750_body25_e42837_d_n7;
            locals.var_dplim__blk975_dn10 = assign29750_body25_e42837_d_n10;
            locals.var_dplim__blk975_dn11 = assign29750_body25_e42837_d_n11;
            locals.var_dplim__blk975_dn12 = assign29750_body25_e42837_d_n12;
            locals.var_dplim__blk975_dn17 = assign29750_body25_e42837_d_n17;
            locals.var_dplim__blk975_rv = 0.0;
            let assign29750_body26_e42839: f64 = (locals.var_dps0).abs();
            let assign29750_body26_e42841: f64 = if assign29750_body26_e42839 > locals.var_dplim__blk975 { 1.0 } else { 0.0 };
            locals.var_guard993 = assign29750_body26_e42841;
            locals.var_guard993_rv = 0.0;
            let (assign29750_body27_e42868, assign29750_body27_e42868_d_n0, assign29750_body27_e42868_d_n2, assign29750_body27_e42868_d_n6, assign29750_body27_e42868_d_n7, assign29750_body27_e42868_d_n10, assign29750_body27_e42868_d_n11, assign29750_body27_e42868_d_n12, assign29750_body27_e42868_d_n17,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard992 == 0.0)) && (locals.var_guard993 != 0.0)) {
        let (assign29750_body27_e42865,) = {
            if (locals.var_dps0 >= 0.0) {
                (1.0,)
            } else {
                let assign29750_body27_e42864: f64 = (-1.0);
                (assign29750_body27_e42864,)
            }
        };
        let assign29750_body27_e42866: f64 = (locals.var_dplim__blk975 * assign29750_body27_e42865);
        (assign29750_body27_e42866, (locals.var_dplim__blk975_dn0 * assign29750_body27_e42865), (locals.var_dplim__blk975_dn2 * assign29750_body27_e42865), (locals.var_dplim__blk975_dn6 * assign29750_body27_e42865), (locals.var_dplim__blk975_dn7 * assign29750_body27_e42865), (locals.var_dplim__blk975_dn10 * assign29750_body27_e42865), (locals.var_dplim__blk975_dn11 * assign29750_body27_e42865), (locals.var_dplim__blk975_dn12 * assign29750_body27_e42865), (locals.var_dplim__blk975_dn17 * assign29750_body27_e42865),)
    } else {
        (locals.var_dps0, locals.var_dps0_dn0, locals.var_dps0_dn2, locals.var_dps0_dn6, locals.var_dps0_dn7, locals.var_dps0_dn10, locals.var_dps0_dn11, locals.var_dps0_dn12, locals.var_dps0_dn17,)
    }
};
            locals.var_dps0 = assign29750_body27_e42868;
            locals.var_dps0_dn0 = assign29750_body27_e42868_d_n0;
            locals.var_dps0_dn2 = assign29750_body27_e42868_d_n2;
            locals.var_dps0_dn6 = assign29750_body27_e42868_d_n6;
            locals.var_dps0_dn7 = assign29750_body27_e42868_d_n7;
            locals.var_dps0_dn10 = assign29750_body27_e42868_d_n10;
            locals.var_dps0_dn11 = assign29750_body27_e42868_d_n11;
            locals.var_dps0_dn12 = assign29750_body27_e42868_d_n12;
            locals.var_dps0_dn17 = assign29750_body27_e42868_d_n17;
            locals.var_dps0_rv = 0.0;
            let (assign29750_body28_e42887, assign29750_body28_e42887_d_n0, assign29750_body28_e42887_d_n2, assign29750_body28_e42887_d_n6, assign29750_body28_e42887_d_n7, assign29750_body28_e42887_d_n10, assign29750_body28_e42887_d_n11, assign29750_body28_e42887_d_n12, assign29750_body28_e42887_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard992 == 0.0)) {
        let assign29750_body28_e42885: f64 = (locals.var_ps0ld__blk945 + locals.var_dps0);
        (assign29750_body28_e42885, (locals.var_ps0ld__blk945_dn0 + locals.var_dps0_dn0), (locals.var_ps0ld__blk945_dn2 + locals.var_dps0_dn2), (locals.var_ps0ld__blk945_dn6 + locals.var_dps0_dn6), (locals.var_ps0ld__blk945_dn7 + locals.var_dps0_dn7), (locals.var_ps0ld__blk945_dn10 + locals.var_dps0_dn10), (locals.var_ps0ld__blk945_dn11 + locals.var_dps0_dn11), (locals.var_ps0ld__blk945_dn12 + locals.var_dps0_dn12), (locals.var_ps0ld__blk945_dn17 + locals.var_dps0_dn17),)
    } else {
        (locals.var_ps0ld__blk945, locals.var_ps0ld__blk945_dn0, locals.var_ps0ld__blk945_dn2, locals.var_ps0ld__blk945_dn6, locals.var_ps0ld__blk945_dn7, locals.var_ps0ld__blk945_dn10, locals.var_ps0ld__blk945_dn11, locals.var_ps0ld__blk945_dn12, locals.var_ps0ld__blk945_dn17,)
    }
};
            locals.var_ps0ld__blk945 = assign29750_body28_e42887;
            locals.var_ps0ld__blk945_dn0 = assign29750_body28_e42887_d_n0;
            locals.var_ps0ld__blk945_dn2 = assign29750_body28_e42887_d_n2;
            locals.var_ps0ld__blk945_dn6 = assign29750_body28_e42887_d_n6;
            locals.var_ps0ld__blk945_dn7 = assign29750_body28_e42887_d_n7;
            locals.var_ps0ld__blk945_dn10 = assign29750_body28_e42887_d_n10;
            locals.var_ps0ld__blk945_dn11 = assign29750_body28_e42887_d_n11;
            locals.var_ps0ld__blk945_dn12 = assign29750_body28_e42887_d_n12;
            locals.var_ps0ld__blk945_dn17 = assign29750_body28_e42887_d_n17;
            locals.var_ps0ld__blk945_rv = 0.0;
            let assign29750_body29_e42889: f64 = (locals.var_dps0).abs();
            let assign29750_body29_e42893: f64 = (locals.var_fs0__blk973).abs();
            let assign29750_body29_e42896: f64 = if ((assign29750_body29_e42889 <= 5e-12) && (assign29750_body29_e42893 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard994 = assign29750_body29_e42896;
            locals.var_guard994_rv = 0.0;
            let (assign29750_body30_e42915,) = {
    if (((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard992 == 0.0)) && (locals.var_guard994 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv__blk918,)
    }
};
            locals.var_flg_conv__blk918 = assign29750_body30_e42915;
            locals.var_flg_conv__blk918_rv = 0.0;
            let (assign29750_body31_e42931,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29750_body31_e42929: f64 = (locals.var_lp_s0 + 1.0);
        (assign29750_body31_e42929,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign29750_body31_e42931;
            locals.var_lp_s0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_109(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign29770_e42937: f64 = if locals.var_chi__blk943 < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard996 = assign29770_e42937;
        locals.var_guard996_rv = 0.0;

        let (assign29810_e42999, assign29810_e42999_d_n0, assign29810_e42999_d_n2, assign29810_e42999_d_n6, assign29810_e42999_d_n7, assign29810_e42999_d_n10, assign29810_e42999_d_n11, assign29810_e42999_d_n12, assign29810_e42999_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard996 != 0.0)) {
        let assign29810_e42993: f64 = (locals.var_fb__blk967 * locals.var_fb__blk967);
        let assign29810_e42996: f64 = (10.0 * 2.220446049250313e-16);
        let assign29810_e42997: f64 = (assign29810_e42993 + assign29810_e42996);
        (assign29810_e42997, ((locals.var_fb__blk967_dn0 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn0)), ((locals.var_fb__blk967_dn2 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn2)), ((locals.var_fb__blk967_dn6 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn6)), ((locals.var_fb__blk967_dn7 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn7)), ((locals.var_fb__blk967_dn10 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn10)), ((locals.var_fb__blk967_dn11 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn11)), ((locals.var_fb__blk967_dn12 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn12)), ((locals.var_fb__blk967_dn17 * locals.var_fb__blk967) + (locals.var_fb__blk967 * locals.var_fb__blk967_dn17)),)
    } else {
        (locals.var_xi0__blk976, locals.var_xi0__blk976_dn0, locals.var_xi0__blk976_dn2, locals.var_xi0__blk976_dn6, locals.var_xi0__blk976_dn7, locals.var_xi0__blk976_dn10, locals.var_xi0__blk976_dn11, locals.var_xi0__blk976_dn12, locals.var_xi0__blk976_dn17,)
    }
};
        locals.var_xi0__blk976 = assign29810_e42999;
        locals.var_xi0__blk976_dn0 = assign29810_e42999_d_n0;
        locals.var_xi0__blk976_dn2 = assign29810_e42999_d_n2;
        locals.var_xi0__blk976_dn6 = assign29810_e42999_d_n6;
        locals.var_xi0__blk976_dn7 = assign29810_e42999_d_n7;
        locals.var_xi0__blk976_dn10 = assign29810_e42999_d_n10;
        locals.var_xi0__blk976_dn11 = assign29810_e42999_d_n11;
        locals.var_xi0__blk976_dn12 = assign29810_e42999_d_n12;
        locals.var_xi0__blk976_dn17 = assign29810_e42999_d_n17;
        locals.var_xi0__blk976_rv = 0.0;

        let (assign29820_e43019, assign29820_e43019_d_n0, assign29820_e43019_d_n2, assign29820_e43019_d_n6, assign29820_e43019_d_n7, assign29820_e43019_d_n10, assign29820_e43019_d_n11, assign29820_e43019_d_n12, assign29820_e43019_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard996 != 0.0)) {
        let assign29820_e43016: f64 = (10.0 * 2.220446049250313e-16);
        let assign29820_e43017: f64 = (locals.var_fb__blk967 + assign29820_e43016);
        (assign29820_e43017, locals.var_fb__blk967_dn0, locals.var_fb__blk967_dn2, locals.var_fb__blk967_dn6, locals.var_fb__blk967_dn7, locals.var_fb__blk967_dn10, locals.var_fb__blk967_dn11, locals.var_fb__blk967_dn12, locals.var_fb__blk967_dn17,)
    } else {
        (locals.var_xi0p12__blk977, locals.var_xi0p12__blk977_dn0, locals.var_xi0p12__blk977_dn2, locals.var_xi0p12__blk977_dn6, locals.var_xi0p12__blk977_dn7, locals.var_xi0p12__blk977_dn10, locals.var_xi0p12__blk977_dn11, locals.var_xi0p12__blk977_dn12, locals.var_xi0p12__blk977_dn17,)
    }
};
        locals.var_xi0p12__blk977 = assign29820_e43019;
        locals.var_xi0p12__blk977_dn0 = assign29820_e43019_d_n0;
        locals.var_xi0p12__blk977_dn2 = assign29820_e43019_d_n2;
        locals.var_xi0p12__blk977_dn6 = assign29820_e43019_d_n6;
        locals.var_xi0p12__blk977_dn7 = assign29820_e43019_d_n7;
        locals.var_xi0p12__blk977_dn10 = assign29820_e43019_d_n10;
        locals.var_xi0p12__blk977_dn11 = assign29820_e43019_d_n11;
        locals.var_xi0p12__blk977_dn12 = assign29820_e43019_d_n12;
        locals.var_xi0p12__blk977_dn17 = assign29820_e43019_d_n17;
        locals.var_xi0p12__blk977_rv = 0.0;

        let (assign29840_e43055, assign29840_e43055_d_n0, assign29840_e43055_d_n2, assign29840_e43055_d_n6, assign29840_e43055_d_n7, assign29840_e43055_d_n10, assign29840_e43055_d_n11, assign29840_e43055_d_n12, assign29840_e43055_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard996 == 0.0)) {
        let assign29840_e43053: f64 = (locals.var_chi__blk943 - 1.0);
        (assign29840_e43053, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    } else {
        (locals.var_xi0__blk976, locals.var_xi0__blk976_dn0, locals.var_xi0__blk976_dn2, locals.var_xi0__blk976_dn6, locals.var_xi0__blk976_dn7, locals.var_xi0__blk976_dn10, locals.var_xi0__blk976_dn11, locals.var_xi0__blk976_dn12, locals.var_xi0__blk976_dn17,)
    }
};
        locals.var_xi0__blk976 = assign29840_e43055;
        locals.var_xi0__blk976_dn0 = assign29840_e43055_d_n0;
        locals.var_xi0__blk976_dn2 = assign29840_e43055_d_n2;
        locals.var_xi0__blk976_dn6 = assign29840_e43055_d_n6;
        locals.var_xi0__blk976_dn7 = assign29840_e43055_d_n7;
        locals.var_xi0__blk976_dn10 = assign29840_e43055_d_n10;
        locals.var_xi0__blk976_dn11 = assign29840_e43055_d_n11;
        locals.var_xi0__blk976_dn12 = assign29840_e43055_d_n12;
        locals.var_xi0__blk976_dn17 = assign29840_e43055_d_n17;
        locals.var_xi0__blk976_rv = 0.0;

        let (assign29850_e43073, assign29850_e43073_d_n0, assign29850_e43073_d_n2, assign29850_e43073_d_n6, assign29850_e43073_d_n7, assign29850_e43073_d_n10, assign29850_e43073_d_n11, assign29850_e43073_d_n12, assign29850_e43073_d_n17,) = {
    if ((((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) && (locals.var_guard996 == 0.0)) {
        let assign29850_e43071: f64 = (locals.var_xi0__blk976).sqrt();
        (assign29850_e43071, (locals.var_xi0__blk976_dn0 / (2.0 * assign29850_e43071)), (locals.var_xi0__blk976_dn2 / (2.0 * assign29850_e43071)), (locals.var_xi0__blk976_dn6 / (2.0 * assign29850_e43071)), (locals.var_xi0__blk976_dn7 / (2.0 * assign29850_e43071)), (locals.var_xi0__blk976_dn10 / (2.0 * assign29850_e43071)), (locals.var_xi0__blk976_dn11 / (2.0 * assign29850_e43071)), (locals.var_xi0__blk976_dn12 / (2.0 * assign29850_e43071)), (locals.var_xi0__blk976_dn17 / (2.0 * assign29850_e43071)),)
    } else {
        (locals.var_xi0p12__blk977, locals.var_xi0p12__blk977_dn0, locals.var_xi0p12__blk977_dn2, locals.var_xi0p12__blk977_dn6, locals.var_xi0p12__blk977_dn7, locals.var_xi0p12__blk977_dn10, locals.var_xi0p12__blk977_dn11, locals.var_xi0p12__blk977_dn12, locals.var_xi0p12__blk977_dn17,)
    }
};
        locals.var_xi0p12__blk977 = assign29850_e43073;
        locals.var_xi0p12__blk977_dn0 = assign29850_e43073_d_n0;
        locals.var_xi0p12__blk977_dn2 = assign29850_e43073_d_n2;
        locals.var_xi0p12__blk977_dn6 = assign29850_e43073_d_n6;
        locals.var_xi0p12__blk977_dn7 = assign29850_e43073_d_n7;
        locals.var_xi0p12__blk977_dn10 = assign29850_e43073_d_n10;
        locals.var_xi0p12__blk977_dn11 = assign29850_e43073_d_n11;
        locals.var_xi0p12__blk977_dn12 = assign29850_e43073_d_n12;
        locals.var_xi0p12__blk977_dn17 = assign29850_e43073_d_n17;
        locals.var_xi0p12__blk977_rv = 0.0;

        let (assign29860_e43089, assign29860_e43089_d_n0, assign29860_e43089_d_n2, assign29860_e43089_d_n6, assign29860_e43089_d_n7, assign29860_e43089_d_n10, assign29860_e43089_d_n11, assign29860_e43089_d_n12, assign29860_e43089_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29860_e43087: f64 = (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977);
        (assign29860_e43087, ((locals.var_cnst0over__blk928_dn0 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn0)), ((locals.var_cnst0over__blk928_dn2 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn2)), ((locals.var_cnst0over__blk928_dn6 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn6)), ((locals.var_cnst0over__blk928_dn7 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn7)), ((locals.var_cnst0over__blk928_dn10 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn10)), ((locals.var_cnst0over__blk928_dn11 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn11)), ((locals.var_cnst0over__blk928_dn12 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn12)), ((locals.var_cnst0over__blk928_dn17 * locals.var_xi0p12__blk977) + (locals.var_cnst0over__blk928 * locals.var_xi0p12__blk977_dn17)),)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign29860_e43089;
        locals.var_qbuld_dn0 = assign29860_e43089_d_n0;
        locals.var_qbuld_dn2 = assign29860_e43089_d_n2;
        locals.var_qbuld_dn6 = assign29860_e43089_d_n6;
        locals.var_qbuld_dn7 = assign29860_e43089_d_n7;
        locals.var_qbuld_dn10 = assign29860_e43089_d_n10;
        locals.var_qbuld_dn11 = assign29860_e43089_d_n11;
        locals.var_qbuld_dn12 = assign29860_e43089_d_n12;
        locals.var_qbuld_dn17 = assign29860_e43089_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign29870_e43107, assign29870_e43107_d_n0, assign29870_e43107_d_n2, assign29870_e43107_d_n6, assign29870_e43107_d_n7, assign29870_e43107_d_n10, assign29870_e43107_d_n11, assign29870_e43107_d_n12, assign29870_e43107_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29870_e43104: f64 = (locals.var_fs02__blk969 + locals.var_xi0p12__blk977);
        let assign29870_e43105: f64 = (1.0 / assign29870_e43104);
        (assign29870_e43105, (-((locals.var_fs02__blk969_dn0 + locals.var_xi0p12__blk977_dn0) / (assign29870_e43104 * assign29870_e43104))), (-((locals.var_fs02__blk969_dn2 + locals.var_xi0p12__blk977_dn2) / (assign29870_e43104 * assign29870_e43104))), (-((locals.var_fs02__blk969_dn6 + locals.var_xi0p12__blk977_dn6) / (assign29870_e43104 * assign29870_e43104))), (-((locals.var_fs02__blk969_dn7 + locals.var_xi0p12__blk977_dn7) / (assign29870_e43104 * assign29870_e43104))), (-((locals.var_fs02__blk969_dn10 + locals.var_xi0p12__blk977_dn10) / (assign29870_e43104 * assign29870_e43104))), (-((locals.var_fs02__blk969_dn11 + locals.var_xi0p12__blk977_dn11) / (assign29870_e43104 * assign29870_e43104))), (-((locals.var_fs02__blk969_dn12 + locals.var_xi0p12__blk977_dn12) / (assign29870_e43104 * assign29870_e43104))), (-((locals.var_fs02__blk969_dn17 + locals.var_xi0p12__blk977_dn17) / (assign29870_e43104 * assign29870_e43104))),)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign29870_e43107;
        locals.var_t1__blk896_dn0 = assign29870_e43107_d_n0;
        locals.var_t1__blk896_dn2 = assign29870_e43107_d_n2;
        locals.var_t1__blk896_dn6 = assign29870_e43107_d_n6;
        locals.var_t1__blk896_dn7 = assign29870_e43107_d_n7;
        locals.var_t1__blk896_dn10 = assign29870_e43107_d_n10;
        locals.var_t1__blk896_dn11 = assign29870_e43107_d_n11;
        locals.var_t1__blk896_dn12 = assign29870_e43107_d_n12;
        locals.var_t1__blk896_dn17 = assign29870_e43107_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let (assign29880_e43125, assign29880_e43125_d_n0, assign29880_e43125_d_n2, assign29880_e43125_d_n6, assign29880_e43125_d_n7, assign29880_e43125_d_n10, assign29880_e43125_d_n11, assign29880_e43125_d_n12, assign29880_e43125_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29880_e43121: f64 = (locals.var_cnst0over__blk928 * locals.var_fs01__blk965);
        let assign29880_e43123: f64 = (assign29880_e43121 * locals.var_t1__blk896);
        (assign29880_e43123, ((((locals.var_cnst0over__blk928_dn0 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn0)) * locals.var_t1__blk896) + (assign29880_e43121 * locals.var_t1__blk896_dn0)), ((((locals.var_cnst0over__blk928_dn2 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn2)) * locals.var_t1__blk896) + (assign29880_e43121 * locals.var_t1__blk896_dn2)), ((((locals.var_cnst0over__blk928_dn6 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn6)) * locals.var_t1__blk896) + (assign29880_e43121 * locals.var_t1__blk896_dn6)), ((((locals.var_cnst0over__blk928_dn7 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn7)) * locals.var_t1__blk896) + (assign29880_e43121 * locals.var_t1__blk896_dn7)), ((((locals.var_cnst0over__blk928_dn10 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn10)) * locals.var_t1__blk896) + (assign29880_e43121 * locals.var_t1__blk896_dn10)), ((((locals.var_cnst0over__blk928_dn11 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn11)) * locals.var_t1__blk896) + (assign29880_e43121 * locals.var_t1__blk896_dn11)), ((((locals.var_cnst0over__blk928_dn12 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn12)) * locals.var_t1__blk896) + (assign29880_e43121 * locals.var_t1__blk896_dn12)), ((((locals.var_cnst0over__blk928_dn17 * locals.var_fs01__blk965) + (locals.var_cnst0over__blk928 * locals.var_fs01__blk965_dn17)) * locals.var_t1__blk896) + (assign29880_e43121 * locals.var_t1__blk896_dn17)),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign29880_e43125;
        locals.var_qiuld_dn0 = assign29880_e43125_d_n0;
        locals.var_qiuld_dn2 = assign29880_e43125_d_n2;
        locals.var_qiuld_dn6 = assign29880_e43125_d_n6;
        locals.var_qiuld_dn7 = assign29880_e43125_d_n7;
        locals.var_qiuld_dn10 = assign29880_e43125_d_n10;
        locals.var_qiuld_dn11 = assign29880_e43125_d_n11;
        locals.var_qiuld_dn12 = assign29880_e43125_d_n12;
        locals.var_qiuld_dn17 = assign29880_e43125_d_n17;
        locals.var_qiuld_rv = 0.0;

        let (assign29890_e43141, assign29890_e43141_d_n0, assign29890_e43141_d_n2, assign29890_e43141_d_n6, assign29890_e43141_d_n7, assign29890_e43141_d_n10, assign29890_e43141_d_n11, assign29890_e43141_d_n12, assign29890_e43141_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard982 == 0.0)) && (locals.var_guard989 != 0.0)) {
        let assign29890_e43139: f64 = (locals.var_qbuld + locals.var_qiuld);
        (assign29890_e43139, (locals.var_qbuld_dn0 + locals.var_qiuld_dn0), (locals.var_qbuld_dn2 + locals.var_qiuld_dn2), (locals.var_qbuld_dn6 + locals.var_qiuld_dn6), (locals.var_qbuld_dn7 + locals.var_qiuld_dn7), (locals.var_qbuld_dn10 + locals.var_qiuld_dn10), (locals.var_qbuld_dn11 + locals.var_qiuld_dn11), (locals.var_qbuld_dn12 + locals.var_qiuld_dn12), (locals.var_qbuld_dn17 + locals.var_qiuld_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign29890_e43141;
        locals.var_qsuld_dn0 = assign29890_e43141_d_n0;
        locals.var_qsuld_dn2 = assign29890_e43141_d_n2;
        locals.var_qsuld_dn6 = assign29890_e43141_d_n6;
        locals.var_qsuld_dn7 = assign29890_e43141_d_n7;
        locals.var_qsuld_dn10 = assign29890_e43141_d_n10;
        locals.var_qsuld_dn11 = assign29890_e43141_d_n11;
        locals.var_qsuld_dn12 = assign29890_e43141_d_n12;
        locals.var_qsuld_dn17 = assign29890_e43141_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign29900_e43152, assign29900_e43152_d_n0, assign29900_e43152_d_n2, assign29900_e43152_d_n6, assign29900_e43152_d_n7, assign29900_e43152_d_n10, assign29900_e43152_d_n11, assign29900_e43152_d_n12, assign29900_e43152_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign29900_e43150: f64 = (locals.var_qsuld - locals.var_qbuld);
        (assign29900_e43150, (locals.var_qsuld_dn0 - locals.var_qbuld_dn0), (locals.var_qsuld_dn2 - locals.var_qbuld_dn2), (locals.var_qsuld_dn6 - locals.var_qbuld_dn6), (locals.var_qsuld_dn7 - locals.var_qbuld_dn7), (locals.var_qsuld_dn10 - locals.var_qbuld_dn10), (locals.var_qsuld_dn11 - locals.var_qbuld_dn11), (locals.var_qsuld_dn12 - locals.var_qbuld_dn12), (locals.var_qsuld_dn17 - locals.var_qbuld_dn17),)
    } else {
        (locals.var_qiuld, locals.var_qiuld_dn0, locals.var_qiuld_dn2, locals.var_qiuld_dn6, locals.var_qiuld_dn7, locals.var_qiuld_dn10, locals.var_qiuld_dn11, locals.var_qiuld_dn12, locals.var_qiuld_dn17,)
    }
};
        locals.var_qiuld = assign29900_e43152;
        locals.var_qiuld_dn0 = assign29900_e43152_d_n0;
        locals.var_qiuld_dn2 = assign29900_e43152_d_n2;
        locals.var_qiuld_dn6 = assign29900_e43152_d_n6;
        locals.var_qiuld_dn7 = assign29900_e43152_d_n7;
        locals.var_qiuld_dn10 = assign29900_e43152_d_n10;
        locals.var_qiuld_dn11 = assign29900_e43152_d_n11;
        locals.var_qiuld_dn12 = assign29900_e43152_d_n12;
        locals.var_qiuld_dn17 = assign29900_e43152_d_n17;
        locals.var_qiuld_rv = 0.0;

        let (assign29910_e43170, assign29910_e43170_d_n0, assign29910_e43170_d_n2, assign29910_e43170_d_n6, assign29910_e43170_d_n7, assign29910_e43170_d_n10, assign29910_e43170_d_n11, assign29910_e43170_d_n12, assign29910_e43170_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let (assign29910_e43168,) = {
            if (p.p43 == 1.0) {
                let assign29910_e43164: f64 = (locals.var_w_dioscv * locals.var_lov);
                (assign29910_e43164,)
            } else {
                let assign29910_e43167: f64 = (locals.var_weffcv_nf * locals.var_lov);
                (assign29910_e43167,)
            }
        };
        (assign29910_e43168, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk899, locals.var_t4__blk899_dn0, locals.var_t4__blk899_dn2, locals.var_t4__blk899_dn6, locals.var_t4__blk899_dn7, locals.var_t4__blk899_dn10, locals.var_t4__blk899_dn11, locals.var_t4__blk899_dn12, locals.var_t4__blk899_dn17,)
    }
};
        locals.var_t4__blk899 = assign29910_e43170;
        locals.var_t4__blk899_dn0 = assign29910_e43170_d_n0;
        locals.var_t4__blk899_dn2 = assign29910_e43170_d_n2;
        locals.var_t4__blk899_dn6 = assign29910_e43170_d_n6;
        locals.var_t4__blk899_dn7 = assign29910_e43170_d_n7;
        locals.var_t4__blk899_dn10 = assign29910_e43170_d_n10;
        locals.var_t4__blk899_dn11 = assign29910_e43170_d_n11;
        locals.var_t4__blk899_dn12 = assign29910_e43170_d_n12;
        locals.var_t4__blk899_dn17 = assign29910_e43170_d_n17;
        locals.var_t4__blk899_rv = 0.0;

        let assign29920_e43181: f64 = if (((locals.var_flg_overs__blk914 != 0.0) && (p.p43 == 0.0)) || ((locals.var_flg_ovloops__blk912 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        locals.var_guard998 = assign29920_e43181;
        locals.var_guard998_rv = 0.0;

        let (assign29930_e43194, assign29930_e43194_d_n0, assign29930_e43194_d_n2, assign29930_e43194_d_n6, assign29930_e43194_d_n7, assign29930_e43194_d_n10, assign29930_e43194_d_n11, assign29930_e43194_d_n12, assign29930_e43194_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard998 != 0.0)) {
        let assign29930_e43192: f64 = (locals.var_t4__blk899 * locals.var_qsuld);
        (assign29930_e43192, ((locals.var_t4__blk899_dn0 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn0)), ((locals.var_t4__blk899_dn2 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn2)), ((locals.var_t4__blk899_dn6 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn6)), ((locals.var_t4__blk899_dn7 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn7)), ((locals.var_t4__blk899_dn10 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn10)), ((locals.var_t4__blk899_dn11 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn11)), ((locals.var_t4__blk899_dn12 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn12)), ((locals.var_t4__blk899_dn17 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn17)),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12, locals.var_qovs_dn17,)
    }
};
        locals.var_qovs = assign29930_e43194;
        locals.var_qovs_dn0 = assign29930_e43194_d_n0;
        locals.var_qovs_dn2 = assign29930_e43194_d_n2;
        locals.var_qovs_dn6 = assign29930_e43194_d_n6;
        locals.var_qovs_dn7 = assign29930_e43194_d_n7;
        locals.var_qovs_dn10 = assign29930_e43194_d_n10;
        locals.var_qovs_dn11 = assign29930_e43194_d_n11;
        locals.var_qovs_dn12 = assign29930_e43194_d_n12;
        locals.var_qovs_dn17 = assign29930_e43194_d_n17;
        locals.var_qovs_rv = 0.0;

        let (assign29940_e43207, assign29940_e43207_d_n0, assign29940_e43207_d_n2, assign29940_e43207_d_n6, assign29940_e43207_d_n7, assign29940_e43207_d_n10, assign29940_e43207_d_n11, assign29940_e43207_d_n12, assign29940_e43207_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard998 != 0.0)) {
        let assign29940_e43205: f64 = (locals.var_t4__blk899 * locals.var_qbuld);
        (assign29940_e43205, ((locals.var_t4__blk899_dn0 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn0)), ((locals.var_t4__blk899_dn2 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn2)), ((locals.var_t4__blk899_dn6 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn6)), ((locals.var_t4__blk899_dn7 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn7)), ((locals.var_t4__blk899_dn10 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn10)), ((locals.var_t4__blk899_dn11 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn11)), ((locals.var_t4__blk899_dn12 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn12)), ((locals.var_t4__blk899_dn17 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn17)),)
    } else {
        (locals.var_qbsld, locals.var_qbsld_dn0, locals.var_qbsld_dn2, locals.var_qbsld_dn6, locals.var_qbsld_dn7, locals.var_qbsld_dn10, locals.var_qbsld_dn11, locals.var_qbsld_dn12, locals.var_qbsld_dn17,)
    }
};
        locals.var_qbsld = assign29940_e43207;
        locals.var_qbsld_dn0 = assign29940_e43207_d_n0;
        locals.var_qbsld_dn2 = assign29940_e43207_d_n2;
        locals.var_qbsld_dn6 = assign29940_e43207_d_n6;
        locals.var_qbsld_dn7 = assign29940_e43207_d_n7;
        locals.var_qbsld_dn10 = assign29940_e43207_d_n10;
        locals.var_qbsld_dn11 = assign29940_e43207_d_n11;
        locals.var_qbsld_dn12 = assign29940_e43207_d_n12;
        locals.var_qbsld_dn17 = assign29940_e43207_d_n17;
        locals.var_qbsld_rv = 0.0;

        let assign29950_e43218: f64 = if (((locals.var_flg_overd__blk915 != 0.0) && (p.p43 == 0.0)) || ((locals.var_flg_ovloopd__blk913 != 0.0) && (p.p43 == 1.0))) { 1.0 } else { 0.0 };
        locals.var_guard999 = assign29950_e43218;
        locals.var_guard999_rv = 0.0;

        let (assign29960_e43231, assign29960_e43231_d_n0, assign29960_e43231_d_n2, assign29960_e43231_d_n6, assign29960_e43231_d_n7, assign29960_e43231_d_n10, assign29960_e43231_d_n11, assign29960_e43231_d_n12, assign29960_e43231_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard999 != 0.0)) {
        let assign29960_e43229: f64 = (locals.var_t4__blk899 * locals.var_qsuld);
        (assign29960_e43229, ((locals.var_t4__blk899_dn0 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn0)), ((locals.var_t4__blk899_dn2 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn2)), ((locals.var_t4__blk899_dn6 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn6)), ((locals.var_t4__blk899_dn7 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn7)), ((locals.var_t4__blk899_dn10 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn10)), ((locals.var_t4__blk899_dn11 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn11)), ((locals.var_t4__blk899_dn12 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn12)), ((locals.var_t4__blk899_dn17 * locals.var_qsuld) + (locals.var_t4__blk899 * locals.var_qsuld_dn17)),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12, locals.var_qovd_dn17,)
    }
};
        locals.var_qovd = assign29960_e43231;
        locals.var_qovd_dn0 = assign29960_e43231_d_n0;
        locals.var_qovd_dn2 = assign29960_e43231_d_n2;
        locals.var_qovd_dn6 = assign29960_e43231_d_n6;
        locals.var_qovd_dn7 = assign29960_e43231_d_n7;
        locals.var_qovd_dn10 = assign29960_e43231_d_n10;
        locals.var_qovd_dn11 = assign29960_e43231_d_n11;
        locals.var_qovd_dn12 = assign29960_e43231_d_n12;
        locals.var_qovd_dn17 = assign29960_e43231_d_n17;
        locals.var_qovd_rv = 0.0;

        let (assign29970_e43244, assign29970_e43244_d_n0, assign29970_e43244_d_n2, assign29970_e43244_d_n6, assign29970_e43244_d_n7, assign29970_e43244_d_n10, assign29970_e43244_d_n11, assign29970_e43244_d_n12, assign29970_e43244_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard999 != 0.0)) {
        let assign29970_e43242: f64 = (locals.var_t4__blk899 * locals.var_qbuld);
        (assign29970_e43242, ((locals.var_t4__blk899_dn0 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn0)), ((locals.var_t4__blk899_dn2 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn2)), ((locals.var_t4__blk899_dn6 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn6)), ((locals.var_t4__blk899_dn7 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn7)), ((locals.var_t4__blk899_dn10 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn10)), ((locals.var_t4__blk899_dn11 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn11)), ((locals.var_t4__blk899_dn12 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn12)), ((locals.var_t4__blk899_dn17 * locals.var_qbuld) + (locals.var_t4__blk899 * locals.var_qbuld_dn17)),)
    } else {
        (locals.var_qbdld, locals.var_qbdld_dn0, locals.var_qbdld_dn2, locals.var_qbdld_dn6, locals.var_qbdld_dn7, locals.var_qbdld_dn10, locals.var_qbdld_dn11, locals.var_qbdld_dn12, locals.var_qbdld_dn17,)
    }
};
        locals.var_qbdld = assign29970_e43244;
        locals.var_qbdld_dn0 = assign29970_e43244_d_n0;
        locals.var_qbdld_dn2 = assign29970_e43244_d_n2;
        locals.var_qbdld_dn6 = assign29970_e43244_d_n6;
        locals.var_qbdld_dn7 = assign29970_e43244_d_n7;
        locals.var_qbdld_dn10 = assign29970_e43244_d_n10;
        locals.var_qbdld_dn11 = assign29970_e43244_d_n11;
        locals.var_qbdld_dn12 = assign29970_e43244_d_n12;
        locals.var_qbdld_dn17 = assign29970_e43244_d_n17;
        locals.var_qbdld_rv = 0.0;

        let (assign29980_e43257,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign29980_e43253: f64 = (1.0 - 1.0);
        let assign29980_e43255: f64 = (assign29980_e43253 / 2.0);
        (assign29980_e43255,)
    } else {
        (locals.var_flg_ovloops__blk912,)
    }
};
        locals.var_flg_ovloops__blk912 = assign29980_e43257;
        locals.var_flg_ovloops__blk912_rv = 0.0;

        let (assign29990_e43270,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign29990_e43266: f64 = (1.0 + 1.0);
        let assign29990_e43268: f64 = (assign29990_e43266 / 2.0);
        (assign29990_e43268,)
    } else {
        (locals.var_flg_ovloopd__blk913,)
    }
};
        locals.var_flg_ovloopd__blk913 = assign29990_e43270;
        locals.var_flg_ovloopd__blk913_rv = 0.0;

        let assign30000_e43273: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1000 = assign30000_e43273;
        locals.var_guard1000_rv = 0.0;

        let (assign30010_e43292, assign30010_e43292_d_n0, assign30010_e43292_d_n2, assign30010_e43292_d_n6, assign30010_e43292_d_n7, assign30010_e43292_d_n10, assign30010_e43292_d_n11, assign30010_e43292_d_n12, assign30010_e43292_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1000 != 0.0)) {
        let assign30010_e43284: f64 = (locals.var_modenml * locals.var_vbs);
        let assign30010_e43288: f64 = (locals.var_vbs - locals.var_vds);
        let assign30010_e43289: f64 = (locals.var_modervs * assign30010_e43288);
        let assign30010_e43290: f64 = (assign30010_e43284 + assign30010_e43289);
        (assign30010_e43290, ((locals.var_modenml * locals.var_vbs_dn0) + (locals.var_modervs * (locals.var_vbs_dn0 - locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vbs_dn2) + (locals.var_modervs * (locals.var_vbs_dn2 - locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vbs_dn6) + (locals.var_modervs * (locals.var_vbs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vbs_dn7) + (locals.var_modervs * (locals.var_vbs_dn7 - locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vbs_dn10) + (locals.var_modervs * (locals.var_vbs_dn10 - locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vbs_dn11) + (locals.var_modervs * (locals.var_vbs_dn11 - locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vbs_dn12) + (locals.var_modervs * (locals.var_vbs_dn12 - locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vbs_dn17) + (locals.var_modervs * (locals.var_vbs_dn17 - locals.var_vds_dn17))),)
    } else {
        (locals.var_vbsgmt__blk922, locals.var_vbsgmt__blk922_dn0, locals.var_vbsgmt__blk922_dn2, locals.var_vbsgmt__blk922_dn6, locals.var_vbsgmt__blk922_dn7, locals.var_vbsgmt__blk922_dn10, locals.var_vbsgmt__blk922_dn11, locals.var_vbsgmt__blk922_dn12, locals.var_vbsgmt__blk922_dn17,)
    }
};
        locals.var_vbsgmt__blk922 = assign30010_e43292;
        locals.var_vbsgmt__blk922_dn0 = assign30010_e43292_d_n0;
        locals.var_vbsgmt__blk922_dn2 = assign30010_e43292_d_n2;
        locals.var_vbsgmt__blk922_dn6 = assign30010_e43292_d_n6;
        locals.var_vbsgmt__blk922_dn7 = assign30010_e43292_d_n7;
        locals.var_vbsgmt__blk922_dn10 = assign30010_e43292_d_n10;
        locals.var_vbsgmt__blk922_dn11 = assign30010_e43292_d_n11;
        locals.var_vbsgmt__blk922_dn12 = assign30010_e43292_d_n12;
        locals.var_vbsgmt__blk922_dn17 = assign30010_e43292_d_n17;
        locals.var_vbsgmt__blk922_rv = 0.0;

        let (assign30020_e43310, assign30020_e43310_d_n0, assign30020_e43310_d_n2, assign30020_e43310_d_n6, assign30020_e43310_d_n7, assign30020_e43310_d_n10, assign30020_e43310_d_n11, assign30020_e43310_d_n12, assign30020_e43310_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1000 != 0.0)) {
        let assign30020_e43303: f64 = (locals.var_modenml * locals.var_vds);
        let assign30020_e43306: f64 = (-locals.var_vds);
        let assign30020_e43307: f64 = (locals.var_modervs * assign30020_e43306);
        let assign30020_e43308: f64 = (assign30020_e43303 + assign30020_e43307);
        (assign30020_e43308, ((locals.var_modenml * locals.var_vds_dn0) + (locals.var_modervs * (-locals.var_vds_dn0))), ((locals.var_modenml * locals.var_vds_dn2) + (locals.var_modervs * (-locals.var_vds_dn2))), ((locals.var_modenml * locals.var_vds_dn6) + (locals.var_modervs * (-locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vds_dn7) + (locals.var_modervs * (-locals.var_vds_dn7))), ((locals.var_modenml * locals.var_vds_dn10) + (locals.var_modervs * (-locals.var_vds_dn10))), ((locals.var_modenml * locals.var_vds_dn11) + (locals.var_modervs * (-locals.var_vds_dn11))), ((locals.var_modenml * locals.var_vds_dn12) + (locals.var_modervs * (-locals.var_vds_dn12))), ((locals.var_modenml * locals.var_vds_dn17) + (locals.var_modervs * (-locals.var_vds_dn17))),)
    } else {
        (locals.var_vdsgmt__blk923, locals.var_vdsgmt__blk923_dn0, locals.var_vdsgmt__blk923_dn2, locals.var_vdsgmt__blk923_dn6, locals.var_vdsgmt__blk923_dn7, locals.var_vdsgmt__blk923_dn10, locals.var_vdsgmt__blk923_dn11, locals.var_vdsgmt__blk923_dn12, locals.var_vdsgmt__blk923_dn17,)
    }
};
        locals.var_vdsgmt__blk923 = assign30020_e43310;
        locals.var_vdsgmt__blk923_dn0 = assign30020_e43310_d_n0;
        locals.var_vdsgmt__blk923_dn2 = assign30020_e43310_d_n2;
        locals.var_vdsgmt__blk923_dn6 = assign30020_e43310_d_n6;
        locals.var_vdsgmt__blk923_dn7 = assign30020_e43310_d_n7;
        locals.var_vdsgmt__blk923_dn10 = assign30020_e43310_d_n10;
        locals.var_vdsgmt__blk923_dn11 = assign30020_e43310_d_n11;
        locals.var_vdsgmt__blk923_dn12 = assign30020_e43310_d_n12;
        locals.var_vdsgmt__blk923_dn17 = assign30020_e43310_d_n17;
        locals.var_vdsgmt__blk923_rv = 0.0;

        let (assign30030_e43329, assign30030_e43329_d_n0, assign30030_e43329_d_n2, assign30030_e43329_d_n6, assign30030_e43329_d_n7, assign30030_e43329_d_n10, assign30030_e43329_d_n11, assign30030_e43329_d_n12, assign30030_e43329_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1000 != 0.0)) {
        let assign30030_e43321: f64 = (locals.var_modenml * locals.var_vgs);
        let assign30030_e43325: f64 = (locals.var_vgs - locals.var_vds);
        let assign30030_e43326: f64 = (locals.var_modervs * assign30030_e43325);
        let assign30030_e43327: f64 = (assign30030_e43321 + assign30030_e43326);
        (assign30030_e43327, (locals.var_modervs * (-locals.var_vds_dn0)), (locals.var_modervs * (-locals.var_vds_dn2)), ((locals.var_modenml * locals.var_vgs_dn6) + (locals.var_modervs * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vgs_dn7) + (locals.var_modervs * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modervs * (-locals.var_vds_dn10)), ((locals.var_modenml * locals.var_vgs_dn11) + (locals.var_modervs * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modervs * (-locals.var_vds_dn12)), (locals.var_modervs * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgsgmt__blk924, locals.var_vgsgmt__blk924_dn0, locals.var_vgsgmt__blk924_dn2, locals.var_vgsgmt__blk924_dn6, locals.var_vgsgmt__blk924_dn7, locals.var_vgsgmt__blk924_dn10, locals.var_vgsgmt__blk924_dn11, locals.var_vgsgmt__blk924_dn12, locals.var_vgsgmt__blk924_dn17,)
    }
};
        locals.var_vgsgmt__blk924 = assign30030_e43329;
        locals.var_vgsgmt__blk924_dn0 = assign30030_e43329_d_n0;
        locals.var_vgsgmt__blk924_dn2 = assign30030_e43329_d_n2;
        locals.var_vgsgmt__blk924_dn6 = assign30030_e43329_d_n6;
        locals.var_vgsgmt__blk924_dn7 = assign30030_e43329_d_n7;
        locals.var_vgsgmt__blk924_dn10 = assign30030_e43329_d_n10;
        locals.var_vgsgmt__blk924_dn11 = assign30030_e43329_d_n11;
        locals.var_vgsgmt__blk924_dn12 = assign30030_e43329_d_n12;
        locals.var_vgsgmt__blk924_dn17 = assign30030_e43329_d_n17;
        locals.var_vgsgmt__blk924_rv = 0.0;

        let (assign30040_e43342, assign30040_e43342_d_n0, assign30040_e43342_d_n2, assign30040_e43342_d_n6, assign30040_e43342_d_n7, assign30040_e43342_d_n10, assign30040_e43342_d_n11, assign30040_e43342_d_n12, assign30040_e43342_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1000 != 0.0)) {
        let assign30040_e43340: f64 = (locals.var_vdsgmt__blk923 - locals.var_vbsgmt__blk922);
        (assign30040_e43340, (locals.var_vdsgmt__blk923_dn0 - locals.var_vbsgmt__blk922_dn0), (locals.var_vdsgmt__blk923_dn2 - locals.var_vbsgmt__blk922_dn2), (locals.var_vdsgmt__blk923_dn6 - locals.var_vbsgmt__blk922_dn6), (locals.var_vdsgmt__blk923_dn7 - locals.var_vbsgmt__blk922_dn7), (locals.var_vdsgmt__blk923_dn10 - locals.var_vbsgmt__blk922_dn10), (locals.var_vdsgmt__blk923_dn11 - locals.var_vbsgmt__blk922_dn11), (locals.var_vdsgmt__blk923_dn12 - locals.var_vbsgmt__blk922_dn12), (locals.var_vdsgmt__blk923_dn17 - locals.var_vbsgmt__blk922_dn17),)
    } else {
        (locals.var_vdbgmt__blk925, locals.var_vdbgmt__blk925_dn0, locals.var_vdbgmt__blk925_dn2, locals.var_vdbgmt__blk925_dn6, locals.var_vdbgmt__blk925_dn7, locals.var_vdbgmt__blk925_dn10, locals.var_vdbgmt__blk925_dn11, locals.var_vdbgmt__blk925_dn12, locals.var_vdbgmt__blk925_dn17,)
    }
};
        locals.var_vdbgmt__blk925 = assign30040_e43342;
        locals.var_vdbgmt__blk925_dn0 = assign30040_e43342_d_n0;
        locals.var_vdbgmt__blk925_dn2 = assign30040_e43342_d_n2;
        locals.var_vdbgmt__blk925_dn6 = assign30040_e43342_d_n6;
        locals.var_vdbgmt__blk925_dn7 = assign30040_e43342_d_n7;
        locals.var_vdbgmt__blk925_dn10 = assign30040_e43342_d_n10;
        locals.var_vdbgmt__blk925_dn11 = assign30040_e43342_d_n11;
        locals.var_vdbgmt__blk925_dn12 = assign30040_e43342_d_n12;
        locals.var_vdbgmt__blk925_dn17 = assign30040_e43342_d_n17;
        locals.var_vdbgmt__blk925_rv = 0.0;

        let (assign30050_e43355, assign30050_e43355_d_n0, assign30050_e43355_d_n2, assign30050_e43355_d_n6, assign30050_e43355_d_n7, assign30050_e43355_d_n10, assign30050_e43355_d_n11, assign30050_e43355_d_n12, assign30050_e43355_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1000 != 0.0)) {
        let assign30050_e43353: f64 = (locals.var_vgsgmt__blk924 - locals.var_vbsgmt__blk922);
        (assign30050_e43353, (locals.var_vgsgmt__blk924_dn0 - locals.var_vbsgmt__blk922_dn0), (locals.var_vgsgmt__blk924_dn2 - locals.var_vbsgmt__blk922_dn2), (locals.var_vgsgmt__blk924_dn6 - locals.var_vbsgmt__blk922_dn6), (locals.var_vgsgmt__blk924_dn7 - locals.var_vbsgmt__blk922_dn7), (locals.var_vgsgmt__blk924_dn10 - locals.var_vbsgmt__blk922_dn10), (locals.var_vgsgmt__blk924_dn11 - locals.var_vbsgmt__blk922_dn11), (locals.var_vgsgmt__blk924_dn12 - locals.var_vbsgmt__blk922_dn12), (locals.var_vgsgmt__blk924_dn17 - locals.var_vbsgmt__blk922_dn17),)
    } else {
        (locals.var_vgbgmt__blk927, locals.var_vgbgmt__blk927_dn0, locals.var_vgbgmt__blk927_dn2, locals.var_vgbgmt__blk927_dn6, locals.var_vgbgmt__blk927_dn7, locals.var_vgbgmt__blk927_dn10, locals.var_vgbgmt__blk927_dn11, locals.var_vgbgmt__blk927_dn12, locals.var_vgbgmt__blk927_dn17,)
    }
};
        locals.var_vgbgmt__blk927 = assign30050_e43355;
        locals.var_vgbgmt__blk927_dn0 = assign30050_e43355_d_n0;
        locals.var_vgbgmt__blk927_dn2 = assign30050_e43355_d_n2;
        locals.var_vgbgmt__blk927_dn6 = assign30050_e43355_d_n6;
        locals.var_vgbgmt__blk927_dn7 = assign30050_e43355_d_n7;
        locals.var_vgbgmt__blk927_dn10 = assign30050_e43355_d_n10;
        locals.var_vgbgmt__blk927_dn11 = assign30050_e43355_d_n11;
        locals.var_vgbgmt__blk927_dn12 = assign30050_e43355_d_n12;
        locals.var_vgbgmt__blk927_dn17 = assign30050_e43355_d_n17;
        locals.var_vgbgmt__blk927_rv = 0.0;

        let (assign30060_e43367, assign30060_e43367_d_n0, assign30060_e43367_d_n2, assign30060_e43367_d_n6, assign30060_e43367_d_n7, assign30060_e43367_d_n10, assign30060_e43367_d_n11, assign30060_e43367_d_n12, assign30060_e43367_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1000 != 0.0)) {
        let assign30060_e43365: f64 = (-locals.var_vbsgmt__blk922);
        (assign30060_e43365, (-locals.var_vbsgmt__blk922_dn0), (-locals.var_vbsgmt__blk922_dn2), (-locals.var_vbsgmt__blk922_dn6), (-locals.var_vbsgmt__blk922_dn7), (-locals.var_vbsgmt__blk922_dn10), (-locals.var_vbsgmt__blk922_dn11), (-locals.var_vbsgmt__blk922_dn12), (-locals.var_vbsgmt__blk922_dn17),)
    } else {
        (locals.var_vsbgmt__blk926, locals.var_vsbgmt__blk926_dn0, locals.var_vsbgmt__blk926_dn2, locals.var_vsbgmt__blk926_dn6, locals.var_vsbgmt__blk926_dn7, locals.var_vsbgmt__blk926_dn10, locals.var_vsbgmt__blk926_dn11, locals.var_vsbgmt__blk926_dn12, locals.var_vsbgmt__blk926_dn17,)
    }
};
        locals.var_vsbgmt__blk926 = assign30060_e43367;
        locals.var_vsbgmt__blk926_dn0 = assign30060_e43367_d_n0;
        locals.var_vsbgmt__blk926_dn2 = assign30060_e43367_d_n2;
        locals.var_vsbgmt__blk926_dn6 = assign30060_e43367_d_n6;
        locals.var_vsbgmt__blk926_dn7 = assign30060_e43367_d_n7;
        locals.var_vsbgmt__blk926_dn10 = assign30060_e43367_d_n10;
        locals.var_vsbgmt__blk926_dn11 = assign30060_e43367_d_n11;
        locals.var_vsbgmt__blk926_dn12 = assign30060_e43367_d_n12;
        locals.var_vsbgmt__blk926_dn17 = assign30060_e43367_d_n17;
        locals.var_vsbgmt__blk926_rv = 0.0;

        let (assign30070_e43384,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1000 != 0.0)) {
        let assign30070_e43378: f64 = (locals.var_flg_ovloops__blk912 * locals.var_modenml);
        let assign30070_e43381: f64 = (locals.var_flg_ovloopd__blk913 * locals.var_modervs);
        let assign30070_e43382: f64 = (assign30070_e43378 + assign30070_e43381);
        (assign30070_e43382,)
    } else {
        (locals.var_flg_overs__blk914,)
    }
};
        locals.var_flg_overs__blk914 = assign30070_e43384;
        locals.var_flg_overs__blk914_rv = 0.0;

        let (assign30080_e43401,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1000 != 0.0)) {
        let assign30080_e43395: f64 = (locals.var_flg_ovloops__blk912 * locals.var_modervs);
        let assign30080_e43398: f64 = (locals.var_flg_ovloopd__blk913 * locals.var_modenml);
        let assign30080_e43399: f64 = (assign30080_e43395 + assign30080_e43398);
        (assign30080_e43399,)
    } else {
        (locals.var_flg_overd__blk915,)
    }
};
        locals.var_flg_overd__blk915 = assign30080_e43401;
        locals.var_flg_overd__blk915_rv = 0.0;

        let (assign30090_e43422, assign30090_e43422_d_n0, assign30090_e43422_d_n2, assign30090_e43422_d_n6, assign30090_e43422_d_n7, assign30090_e43422_d_n10, assign30090_e43422_d_n11, assign30090_e43422_d_n12, assign30090_e43422_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1000 != 0.0)) {
        let assign30090_e43412: f64 = (locals.var_flg_overs__blk914 * locals.var_vsbgmt__blk926);
        let assign30090_e43415: f64 = (locals.var_flg_overd__blk915 * locals.var_vdbgmt__blk925);
        let assign30090_e43416: f64 = (assign30090_e43412 + assign30090_e43415);
        let assign30090_e43419: f64 = (10.0 * 2.220446049250313e-16);
        let assign30090_e43420: f64 = (assign30090_e43416 + assign30090_e43419);
        (assign30090_e43420, ((locals.var_flg_overs__blk914 * locals.var_vsbgmt__blk926_dn0) + (locals.var_flg_overd__blk915 * locals.var_vdbgmt__blk925_dn0)), ((locals.var_flg_overs__blk914 * locals.var_vsbgmt__blk926_dn2) + (locals.var_flg_overd__blk915 * locals.var_vdbgmt__blk925_dn2)), ((locals.var_flg_overs__blk914 * locals.var_vsbgmt__blk926_dn6) + (locals.var_flg_overd__blk915 * locals.var_vdbgmt__blk925_dn6)), ((locals.var_flg_overs__blk914 * locals.var_vsbgmt__blk926_dn7) + (locals.var_flg_overd__blk915 * locals.var_vdbgmt__blk925_dn7)), ((locals.var_flg_overs__blk914 * locals.var_vsbgmt__blk926_dn10) + (locals.var_flg_overd__blk915 * locals.var_vdbgmt__blk925_dn10)), ((locals.var_flg_overs__blk914 * locals.var_vsbgmt__blk926_dn11) + (locals.var_flg_overd__blk915 * locals.var_vdbgmt__blk925_dn11)), ((locals.var_flg_overs__blk914 * locals.var_vsbgmt__blk926_dn12) + (locals.var_flg_overd__blk915 * locals.var_vdbgmt__blk925_dn12)), ((locals.var_flg_overs__blk914 * locals.var_vsbgmt__blk926_dn17) + (locals.var_flg_overd__blk915 * locals.var_vdbgmt__blk925_dn17)),)
    } else {
        (locals.var_vxbgmt__blk920, locals.var_vxbgmt__blk920_dn0, locals.var_vxbgmt__blk920_dn2, locals.var_vxbgmt__blk920_dn6, locals.var_vxbgmt__blk920_dn7, locals.var_vxbgmt__blk920_dn10, locals.var_vxbgmt__blk920_dn11, locals.var_vxbgmt__blk920_dn12, locals.var_vxbgmt__blk920_dn17,)
    }
};
        locals.var_vxbgmt__blk920 = assign30090_e43422;
        locals.var_vxbgmt__blk920_dn0 = assign30090_e43422_d_n0;
        locals.var_vxbgmt__blk920_dn2 = assign30090_e43422_d_n2;
        locals.var_vxbgmt__blk920_dn6 = assign30090_e43422_d_n6;
        locals.var_vxbgmt__blk920_dn7 = assign30090_e43422_d_n7;
        locals.var_vxbgmt__blk920_dn10 = assign30090_e43422_d_n10;
        locals.var_vxbgmt__blk920_dn11 = assign30090_e43422_d_n11;
        locals.var_vxbgmt__blk920_dn12 = assign30090_e43422_d_n12;
        locals.var_vxbgmt__blk920_dn17 = assign30090_e43422_d_n17;
        locals.var_vxbgmt__blk920_rv = 0.0;

        let (assign30100_e43440,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1000 == 0.0)) {
        let assign30100_e43434: f64 = (locals.var_flg_ovloops__blk912 * locals.var_modenml);
        let assign30100_e43437: f64 = (locals.var_flg_ovloopd__blk913 * locals.var_modervs);
        let assign30100_e43438: f64 = (assign30100_e43434 + assign30100_e43437);
        (assign30100_e43438,)
    } else {
        (locals.var_flg_overs__blk914,)
    }
};
        locals.var_flg_overs__blk914 = assign30100_e43440;
        locals.var_flg_overs__blk914_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_110(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30110_e43458,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1000 == 0.0)) {
        let assign30110_e43452: f64 = (locals.var_flg_ovloops__blk912 * locals.var_modervs);
        let assign30110_e43455: f64 = (locals.var_flg_ovloopd__blk913 * locals.var_modenml);
        let assign30110_e43456: f64 = (assign30110_e43452 + assign30110_e43455);
        (assign30110_e43456,)
    } else {
        (locals.var_flg_overd__blk915,)
    }
};
        locals.var_flg_overd__blk915 = assign30110_e43458;
        locals.var_flg_overd__blk915_rv = 0.0;

        let (assign30120_e43480, assign30120_e43480_d_n0, assign30120_e43480_d_n2, assign30120_e43480_d_n6, assign30120_e43480_d_n7, assign30120_e43480_d_n10, assign30120_e43480_d_n11, assign30120_e43480_d_n12, assign30120_e43480_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1000 == 0.0)) && (locals.var_flg_ovloops__blk912 != 0.0)) {
        let assign30120_e43472: f64 = (locals.var_modenml * locals.var_vgs);
        let assign30120_e43476: f64 = (locals.var_vgs - locals.var_vds);
        let assign30120_e43477: f64 = (locals.var_modervs * assign30120_e43476);
        let assign30120_e43478: f64 = (assign30120_e43472 + assign30120_e43477);
        (assign30120_e43478, (locals.var_modervs * (-locals.var_vds_dn0)), (locals.var_modervs * (-locals.var_vds_dn2)), ((locals.var_modenml * locals.var_vgs_dn6) + (locals.var_modervs * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modenml * locals.var_vgs_dn7) + (locals.var_modervs * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modervs * (-locals.var_vds_dn10)), ((locals.var_modenml * locals.var_vgs_dn11) + (locals.var_modervs * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modervs * (-locals.var_vds_dn12)), (locals.var_modervs * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgbgmt__blk927, locals.var_vgbgmt__blk927_dn0, locals.var_vgbgmt__blk927_dn2, locals.var_vgbgmt__blk927_dn6, locals.var_vgbgmt__blk927_dn7, locals.var_vgbgmt__blk927_dn10, locals.var_vgbgmt__blk927_dn11, locals.var_vgbgmt__blk927_dn12, locals.var_vgbgmt__blk927_dn17,)
    }
};
        locals.var_vgbgmt__blk927 = assign30120_e43480;
        locals.var_vgbgmt__blk927_dn0 = assign30120_e43480_d_n0;
        locals.var_vgbgmt__blk927_dn2 = assign30120_e43480_d_n2;
        locals.var_vgbgmt__blk927_dn6 = assign30120_e43480_d_n6;
        locals.var_vgbgmt__blk927_dn7 = assign30120_e43480_d_n7;
        locals.var_vgbgmt__blk927_dn10 = assign30120_e43480_d_n10;
        locals.var_vgbgmt__blk927_dn11 = assign30120_e43480_d_n11;
        locals.var_vgbgmt__blk927_dn12 = assign30120_e43480_d_n12;
        locals.var_vgbgmt__blk927_dn17 = assign30120_e43480_d_n17;
        locals.var_vgbgmt__blk927_rv = 0.0;

        let (assign30130_e43502, assign30130_e43502_d_n0, assign30130_e43502_d_n2, assign30130_e43502_d_n6, assign30130_e43502_d_n7, assign30130_e43502_d_n10, assign30130_e43502_d_n11, assign30130_e43502_d_n12, assign30130_e43502_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1000 == 0.0)) && (locals.var_flg_ovloopd__blk913 != 0.0)) {
        let assign30130_e43494: f64 = (locals.var_modervs * locals.var_vgs);
        let assign30130_e43498: f64 = (locals.var_vgs - locals.var_vds);
        let assign30130_e43499: f64 = (locals.var_modenml * assign30130_e43498);
        let assign30130_e43500: f64 = (assign30130_e43494 + assign30130_e43499);
        (assign30130_e43500, (locals.var_modenml * (-locals.var_vds_dn0)), (locals.var_modenml * (-locals.var_vds_dn2)), ((locals.var_modervs * locals.var_vgs_dn6) + (locals.var_modenml * (locals.var_vgs_dn6 - locals.var_vds_dn6))), ((locals.var_modervs * locals.var_vgs_dn7) + (locals.var_modenml * (locals.var_vgs_dn7 - locals.var_vds_dn7))), (locals.var_modenml * (-locals.var_vds_dn10)), ((locals.var_modervs * locals.var_vgs_dn11) + (locals.var_modenml * (locals.var_vgs_dn11 - locals.var_vds_dn11))), (locals.var_modenml * (-locals.var_vds_dn12)), (locals.var_modenml * (-locals.var_vds_dn17)),)
    } else {
        (locals.var_vgbgmt__blk927, locals.var_vgbgmt__blk927_dn0, locals.var_vgbgmt__blk927_dn2, locals.var_vgbgmt__blk927_dn6, locals.var_vgbgmt__blk927_dn7, locals.var_vgbgmt__blk927_dn10, locals.var_vgbgmt__blk927_dn11, locals.var_vgbgmt__blk927_dn12, locals.var_vgbgmt__blk927_dn17,)
    }
};
        locals.var_vgbgmt__blk927 = assign30130_e43502;
        locals.var_vgbgmt__blk927_dn0 = assign30130_e43502_d_n0;
        locals.var_vgbgmt__blk927_dn2 = assign30130_e43502_d_n2;
        locals.var_vgbgmt__blk927_dn6 = assign30130_e43502_d_n6;
        locals.var_vgbgmt__blk927_dn7 = assign30130_e43502_d_n7;
        locals.var_vgbgmt__blk927_dn10 = assign30130_e43502_d_n10;
        locals.var_vgbgmt__blk927_dn11 = assign30130_e43502_d_n11;
        locals.var_vgbgmt__blk927_dn12 = assign30130_e43502_d_n12;
        locals.var_vgbgmt__blk927_dn17 = assign30130_e43502_d_n17;
        locals.var_vgbgmt__blk927_rv = 0.0;

        let (assign30140_e43514, assign30140_e43514_d_n0, assign30140_e43514_d_n2, assign30140_e43514_d_n6, assign30140_e43514_d_n7, assign30140_e43514_d_n10, assign30140_e43514_d_n11, assign30140_e43514_d_n12, assign30140_e43514_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1000 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vxbgmt__blk920, locals.var_vxbgmt__blk920_dn0, locals.var_vxbgmt__blk920_dn2, locals.var_vxbgmt__blk920_dn6, locals.var_vxbgmt__blk920_dn7, locals.var_vxbgmt__blk920_dn10, locals.var_vxbgmt__blk920_dn11, locals.var_vxbgmt__blk920_dn12, locals.var_vxbgmt__blk920_dn17,)
    }
};
        locals.var_vxbgmt__blk920 = assign30140_e43514;
        locals.var_vxbgmt__blk920_dn0 = assign30140_e43514_d_n0;
        locals.var_vxbgmt__blk920_dn2 = assign30140_e43514_d_n2;
        locals.var_vxbgmt__blk920_dn6 = assign30140_e43514_d_n6;
        locals.var_vxbgmt__blk920_dn7 = assign30140_e43514_d_n7;
        locals.var_vxbgmt__blk920_dn10 = assign30140_e43514_d_n10;
        locals.var_vxbgmt__blk920_dn11 = assign30140_e43514_d_n11;
        locals.var_vxbgmt__blk920_dn12 = assign30140_e43514_d_n12;
        locals.var_vxbgmt__blk920_dn17 = assign30140_e43514_d_n17;
        locals.var_vxbgmt__blk920_rv = 0.0;

        let (assign30150_e43524, assign30150_e43524_d_n0, assign30150_e43524_d_n2, assign30150_e43524_d_n6, assign30150_e43524_d_n7, assign30150_e43524_d_n10, assign30150_e43524_d_n11, assign30150_e43524_d_n12, assign30150_e43524_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign30150_e43522: f64 = (-locals.var_vxbgmt__blk920);
        (assign30150_e43522, (-locals.var_vxbgmt__blk920_dn0), (-locals.var_vxbgmt__blk920_dn2), (-locals.var_vxbgmt__blk920_dn6), (-locals.var_vxbgmt__blk920_dn7), (-locals.var_vxbgmt__blk920_dn10), (-locals.var_vxbgmt__blk920_dn11), (-locals.var_vxbgmt__blk920_dn12), (-locals.var_vxbgmt__blk920_dn17),)
    } else {
        (locals.var_t0__blk895, locals.var_t0__blk895_dn0, locals.var_t0__blk895_dn2, locals.var_t0__blk895_dn6, locals.var_t0__blk895_dn7, locals.var_t0__blk895_dn10, locals.var_t0__blk895_dn11, locals.var_t0__blk895_dn12, locals.var_t0__blk895_dn17,)
    }
};
        locals.var_t0__blk895 = assign30150_e43524;
        locals.var_t0__blk895_dn0 = assign30150_e43524_d_n0;
        locals.var_t0__blk895_dn2 = assign30150_e43524_d_n2;
        locals.var_t0__blk895_dn6 = assign30150_e43524_d_n6;
        locals.var_t0__blk895_dn7 = assign30150_e43524_d_n7;
        locals.var_t0__blk895_dn10 = assign30150_e43524_d_n10;
        locals.var_t0__blk895_dn11 = assign30150_e43524_d_n11;
        locals.var_t0__blk895_dn12 = assign30150_e43524_d_n12;
        locals.var_t0__blk895_dn17 = assign30150_e43524_d_n17;
        locals.var_t0__blk895_rv = 0.0;

        let assign30160_e43527: f64 = if locals.var_t0__blk895 > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard1001 = assign30160_e43527;
        locals.var_guard1001_rv = 0.0;

        let (assign30170_e43540, assign30170_e43540_d_n0, assign30170_e43540_d_n2, assign30170_e43540_d_n6, assign30170_e43540_d_n7, assign30170_e43540_d_n10, assign30170_e43540_d_n11, assign30170_e43540_d_n12, assign30170_e43540_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1001 != 0.0)) {
        let assign30170_e43538: f64 = (locals.var_t0__blk895 - locals.var_vbs_bnd);
        (assign30170_e43538, locals.var_t0__blk895_dn0, locals.var_t0__blk895_dn2, locals.var_t0__blk895_dn6, locals.var_t0__blk895_dn7, locals.var_t0__blk895_dn10, locals.var_t0__blk895_dn11, locals.var_t0__blk895_dn12, locals.var_t0__blk895_dn17,)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign30170_e43540;
        locals.var_t1__blk896_dn0 = assign30170_e43540_d_n0;
        locals.var_t1__blk896_dn2 = assign30170_e43540_d_n2;
        locals.var_t1__blk896_dn6 = assign30170_e43540_d_n6;
        locals.var_t1__blk896_dn7 = assign30170_e43540_d_n7;
        locals.var_t1__blk896_dn10 = assign30170_e43540_d_n10;
        locals.var_t1__blk896_dn11 = assign30170_e43540_d_n11;
        locals.var_t1__blk896_dn12 = assign30170_e43540_d_n12;
        locals.var_t1__blk896_dn17 = assign30170_e43540_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let (assign30180_e43553, assign30180_e43553_d_n0, assign30180_e43553_d_n2, assign30180_e43553_d_n6, assign30180_e43553_d_n7, assign30180_e43553_d_n10, assign30180_e43553_d_n11, assign30180_e43553_d_n12, assign30180_e43553_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1001 != 0.0)) {
        let assign30180_e43551: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign30180_e43551, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk897, locals.var_t2__blk897_dn0, locals.var_t2__blk897_dn2, locals.var_t2__blk897_dn6, locals.var_t2__blk897_dn7, locals.var_t2__blk897_dn10, locals.var_t2__blk897_dn11, locals.var_t2__blk897_dn12, locals.var_t2__blk897_dn17,)
    }
};
        locals.var_t2__blk897 = assign30180_e43553;
        locals.var_t2__blk897_dn0 = assign30180_e43553_d_n0;
        locals.var_t2__blk897_dn2 = assign30180_e43553_d_n2;
        locals.var_t2__blk897_dn6 = assign30180_e43553_d_n6;
        locals.var_t2__blk897_dn7 = assign30180_e43553_d_n7;
        locals.var_t2__blk897_dn10 = assign30180_e43553_d_n10;
        locals.var_t2__blk897_dn11 = assign30180_e43553_d_n11;
        locals.var_t2__blk897_dn12 = assign30180_e43553_d_n12;
        locals.var_t2__blk897_dn17 = assign30180_e43553_d_n17;
        locals.var_t2__blk897_rv = 0.0;

        let (assign30190_e43566, assign30190_e43566_d_n0, assign30190_e43566_d_n2, assign30190_e43566_d_n6, assign30190_e43566_d_n7, assign30190_e43566_d_n10, assign30190_e43566_d_n11, assign30190_e43566_d_n12, assign30190_e43566_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1001 != 0.0)) {
        let assign30190_e43564: f64 = (locals.var_t1__blk896 / locals.var_t2__blk897);
        (assign30190_e43564, (((locals.var_t1__blk896_dn0 * locals.var_t2__blk897) - (locals.var_t1__blk896 * locals.var_t2__blk897_dn0)) / (locals.var_t2__blk897 * locals.var_t2__blk897)), (((locals.var_t1__blk896_dn2 * locals.var_t2__blk897) - (locals.var_t1__blk896 * locals.var_t2__blk897_dn2)) / (locals.var_t2__blk897 * locals.var_t2__blk897)), (((locals.var_t1__blk896_dn6 * locals.var_t2__blk897) - (locals.var_t1__blk896 * locals.var_t2__blk897_dn6)) / (locals.var_t2__blk897 * locals.var_t2__blk897)), (((locals.var_t1__blk896_dn7 * locals.var_t2__blk897) - (locals.var_t1__blk896 * locals.var_t2__blk897_dn7)) / (locals.var_t2__blk897 * locals.var_t2__blk897)), (((locals.var_t1__blk896_dn10 * locals.var_t2__blk897) - (locals.var_t1__blk896 * locals.var_t2__blk897_dn10)) / (locals.var_t2__blk897 * locals.var_t2__blk897)), (((locals.var_t1__blk896_dn11 * locals.var_t2__blk897) - (locals.var_t1__blk896 * locals.var_t2__blk897_dn11)) / (locals.var_t2__blk897 * locals.var_t2__blk897)), (((locals.var_t1__blk896_dn12 * locals.var_t2__blk897) - (locals.var_t1__blk896 * locals.var_t2__blk897_dn12)) / (locals.var_t2__blk897 * locals.var_t2__blk897)), (((locals.var_t1__blk896_dn17 * locals.var_t2__blk897) - (locals.var_t1__blk896 * locals.var_t2__blk897_dn17)) / (locals.var_t2__blk897 * locals.var_t2__blk897)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12, locals.var_tmf1_dn17,)
    }
};
        locals.var_tmf1 = assign30190_e43566;
        locals.var_tmf1_dn0 = assign30190_e43566_d_n0;
        locals.var_tmf1_dn2 = assign30190_e43566_d_n2;
        locals.var_tmf1_dn6 = assign30190_e43566_d_n6;
        locals.var_tmf1_dn7 = assign30190_e43566_d_n7;
        locals.var_tmf1_dn10 = assign30190_e43566_d_n10;
        locals.var_tmf1_dn11 = assign30190_e43566_d_n11;
        locals.var_tmf1_dn12 = assign30190_e43566_d_n12;
        locals.var_tmf1_dn17 = assign30190_e43566_d_n17;
        locals.var_tmf1_rv = 0.0;

        let (assign30200_e43579, assign30200_e43579_d_n0, assign30200_e43579_d_n2, assign30200_e43579_d_n6, assign30200_e43579_d_n7, assign30200_e43579_d_n10, assign30200_e43579_d_n11, assign30200_e43579_d_n12, assign30200_e43579_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1001 != 0.0)) {
        let assign30200_e43577: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign30200_e43577, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)), ((locals.var_tmf1_dn17 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12, locals.var_tmf2_dn17,)
    }
};
        locals.var_tmf2 = assign30200_e43579;
        locals.var_tmf2_dn0 = assign30200_e43579_d_n0;
        locals.var_tmf2_dn2 = assign30200_e43579_d_n2;
        locals.var_tmf2_dn6 = assign30200_e43579_d_n6;
        locals.var_tmf2_dn7 = assign30200_e43579_d_n7;
        locals.var_tmf2_dn10 = assign30200_e43579_d_n10;
        locals.var_tmf2_dn11 = assign30200_e43579_d_n11;
        locals.var_tmf2_dn12 = assign30200_e43579_d_n12;
        locals.var_tmf2_dn17 = assign30200_e43579_d_n17;
        locals.var_tmf2_rv = 0.0;

        let (assign30210_e43592, assign30210_e43592_d_n0, assign30210_e43592_d_n2, assign30210_e43592_d_n6, assign30210_e43592_d_n7, assign30210_e43592_d_n10, assign30210_e43592_d_n11, assign30210_e43592_d_n12, assign30210_e43592_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1001 != 0.0)) {
        let assign30210_e43590: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign30210_e43590, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn17)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn12, locals.var_tmf3_dn17,)
    }
};
        locals.var_tmf3 = assign30210_e43592;
        locals.var_tmf3_dn0 = assign30210_e43592_d_n0;
        locals.var_tmf3_dn2 = assign30210_e43592_d_n2;
        locals.var_tmf3_dn6 = assign30210_e43592_d_n6;
        locals.var_tmf3_dn7 = assign30210_e43592_d_n7;
        locals.var_tmf3_dn10 = assign30210_e43592_d_n10;
        locals.var_tmf3_dn11 = assign30210_e43592_d_n11;
        locals.var_tmf3_dn12 = assign30210_e43592_d_n12;
        locals.var_tmf3_dn17 = assign30210_e43592_d_n17;
        locals.var_tmf3_rv = 0.0;

        let (assign30220_e43605, assign30220_e43605_d_n0, assign30220_e43605_d_n2, assign30220_e43605_d_n6, assign30220_e43605_d_n7, assign30220_e43605_d_n10, assign30220_e43605_d_n11, assign30220_e43605_d_n12, assign30220_e43605_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1001 != 0.0)) {
        let assign30220_e43603: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign30220_e43603, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12)), ((locals.var_tmf2_dn17 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn17)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn12, locals.var_tmf4_dn17,)
    }
};
        locals.var_tmf4 = assign30220_e43605;
        locals.var_tmf4_dn0 = assign30220_e43605_d_n0;
        locals.var_tmf4_dn2 = assign30220_e43605_d_n2;
        locals.var_tmf4_dn6 = assign30220_e43605_d_n6;
        locals.var_tmf4_dn7 = assign30220_e43605_d_n7;
        locals.var_tmf4_dn10 = assign30220_e43605_d_n10;
        locals.var_tmf4_dn11 = assign30220_e43605_d_n11;
        locals.var_tmf4_dn12 = assign30220_e43605_d_n12;
        locals.var_tmf4_dn17 = assign30220_e43605_d_n17;
        locals.var_tmf4_rv = 0.0;

        let (assign30230_e43626, assign30230_e43626_d_n0, assign30230_e43626_d_n2, assign30230_e43626_d_n6, assign30230_e43626_d_n7, assign30230_e43626_d_n10, assign30230_e43626_d_n11, assign30230_e43626_d_n12, assign30230_e43626_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1001 != 0.0)) {
        let assign30230_e43617: f64 = (1.0 + locals.var_tmf1);
        let assign30230_e43619: f64 = (assign30230_e43617 + locals.var_tmf2);
        let assign30230_e43621: f64 = (assign30230_e43619 + locals.var_tmf3);
        let assign30230_e43623: f64 = (assign30230_e43621 + locals.var_tmf4);
        let assign30230_e43624: f64 = (1.0 / assign30230_e43623);
        (assign30230_e43624, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign30230_e43623 * assign30230_e43623))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign30230_e43623 * assign30230_e43623))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign30230_e43623 * assign30230_e43623))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign30230_e43623 * assign30230_e43623))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign30230_e43623 * assign30230_e43623))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign30230_e43623 * assign30230_e43623))), (-((((locals.var_tmf1_dn12 + locals.var_tmf2_dn12) + locals.var_tmf3_dn12) + locals.var_tmf4_dn12) / (assign30230_e43623 * assign30230_e43623))), (-((((locals.var_tmf1_dn17 + locals.var_tmf2_dn17) + locals.var_tmf3_dn17) + locals.var_tmf4_dn17) / (assign30230_e43623 * assign30230_e43623))),)
    } else {
        (locals.var_ty__blk905, locals.var_ty__blk905_dn0, locals.var_ty__blk905_dn2, locals.var_ty__blk905_dn6, locals.var_ty__blk905_dn7, locals.var_ty__blk905_dn10, locals.var_ty__blk905_dn11, locals.var_ty__blk905_dn12, locals.var_ty__blk905_dn17,)
    }
};
        locals.var_ty__blk905 = assign30230_e43626;
        locals.var_ty__blk905_dn0 = assign30230_e43626_d_n0;
        locals.var_ty__blk905_dn2 = assign30230_e43626_d_n2;
        locals.var_ty__blk905_dn6 = assign30230_e43626_d_n6;
        locals.var_ty__blk905_dn7 = assign30230_e43626_d_n7;
        locals.var_ty__blk905_dn10 = assign30230_e43626_d_n10;
        locals.var_ty__blk905_dn11 = assign30230_e43626_d_n11;
        locals.var_ty__blk905_dn12 = assign30230_e43626_d_n12;
        locals.var_ty__blk905_dn17 = assign30230_e43626_d_n17;
        locals.var_ty__blk905_rv = 0.0;

        let (assign30250_e43669, assign30250_e43669_d_n0, assign30250_e43669_d_n2, assign30250_e43669_d_n6, assign30250_e43669_d_n7, assign30250_e43669_d_n10, assign30250_e43669_d_n11, assign30250_e43669_d_n12, assign30250_e43669_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1001 != 0.0)) {
        let assign30250_e43666: f64 = (1.0 - locals.var_ty__blk905);
        let assign30250_e43667: f64 = (locals.var_t2__blk897 * assign30250_e43666);
        (assign30250_e43667, ((locals.var_t2__blk897_dn0 * assign30250_e43666) + (locals.var_t2__blk897 * (-locals.var_ty__blk905_dn0))), ((locals.var_t2__blk897_dn2 * assign30250_e43666) + (locals.var_t2__blk897 * (-locals.var_ty__blk905_dn2))), ((locals.var_t2__blk897_dn6 * assign30250_e43666) + (locals.var_t2__blk897 * (-locals.var_ty__blk905_dn6))), ((locals.var_t2__blk897_dn7 * assign30250_e43666) + (locals.var_t2__blk897 * (-locals.var_ty__blk905_dn7))), ((locals.var_t2__blk897_dn10 * assign30250_e43666) + (locals.var_t2__blk897 * (-locals.var_ty__blk905_dn10))), ((locals.var_t2__blk897_dn11 * assign30250_e43666) + (locals.var_t2__blk897 * (-locals.var_ty__blk905_dn11))), ((locals.var_t2__blk897_dn12 * assign30250_e43666) + (locals.var_t2__blk897 * (-locals.var_ty__blk905_dn12))), ((locals.var_t2__blk897_dn17 * assign30250_e43666) + (locals.var_t2__blk897 * (-locals.var_ty__blk905_dn17))),)
    } else {
        (locals.var_ty__blk905, locals.var_ty__blk905_dn0, locals.var_ty__blk905_dn2, locals.var_ty__blk905_dn6, locals.var_ty__blk905_dn7, locals.var_ty__blk905_dn10, locals.var_ty__blk905_dn11, locals.var_ty__blk905_dn12, locals.var_ty__blk905_dn17,)
    }
};
        locals.var_ty__blk905 = assign30250_e43669;
        locals.var_ty__blk905_dn0 = assign30250_e43669_d_n0;
        locals.var_ty__blk905_dn2 = assign30250_e43669_d_n2;
        locals.var_ty__blk905_dn6 = assign30250_e43669_d_n6;
        locals.var_ty__blk905_dn7 = assign30250_e43669_d_n7;
        locals.var_ty__blk905_dn10 = assign30250_e43669_d_n10;
        locals.var_ty__blk905_dn11 = assign30250_e43669_d_n11;
        locals.var_ty__blk905_dn12 = assign30250_e43669_d_n12;
        locals.var_ty__blk905_dn17 = assign30250_e43669_d_n17;
        locals.var_ty__blk905_rv = 0.0;

        let (assign30270_e43694, assign30270_e43694_d_n0, assign30270_e43694_d_n2, assign30270_e43694_d_n6, assign30270_e43694_d_n7, assign30270_e43694_d_n10, assign30270_e43694_d_n11, assign30270_e43694_d_n12, assign30270_e43694_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1001 != 0.0)) {
        let assign30270_e43692: f64 = (locals.var_vbs_bnd + locals.var_ty__blk905);
        (assign30270_e43692, locals.var_ty__blk905_dn0, locals.var_ty__blk905_dn2, locals.var_ty__blk905_dn6, locals.var_ty__blk905_dn7, locals.var_ty__blk905_dn10, locals.var_ty__blk905_dn11, locals.var_ty__blk905_dn12, locals.var_ty__blk905_dn17,)
    } else {
        (locals.var_t10__blk902, locals.var_t10__blk902_dn0, locals.var_t10__blk902_dn2, locals.var_t10__blk902_dn6, locals.var_t10__blk902_dn7, locals.var_t10__blk902_dn10, locals.var_t10__blk902_dn11, locals.var_t10__blk902_dn12, locals.var_t10__blk902_dn17,)
    }
};
        locals.var_t10__blk902 = assign30270_e43694;
        locals.var_t10__blk902_dn0 = assign30270_e43694_d_n0;
        locals.var_t10__blk902_dn2 = assign30270_e43694_d_n2;
        locals.var_t10__blk902_dn6 = assign30270_e43694_d_n6;
        locals.var_t10__blk902_dn7 = assign30270_e43694_d_n7;
        locals.var_t10__blk902_dn10 = assign30270_e43694_d_n10;
        locals.var_t10__blk902_dn11 = assign30270_e43694_d_n11;
        locals.var_t10__blk902_dn12 = assign30270_e43694_d_n12;
        locals.var_t10__blk902_dn17 = assign30270_e43694_d_n17;
        locals.var_t10__blk902_rv = 0.0;

        let (assign30280_e43706, assign30280_e43706_d_n0, assign30280_e43706_d_n2, assign30280_e43706_d_n6, assign30280_e43706_d_n7, assign30280_e43706_d_n10, assign30280_e43706_d_n11, assign30280_e43706_d_n12, assign30280_e43706_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1001 == 0.0)) {
        (locals.var_t0__blk895, locals.var_t0__blk895_dn0, locals.var_t0__blk895_dn2, locals.var_t0__blk895_dn6, locals.var_t0__blk895_dn7, locals.var_t0__blk895_dn10, locals.var_t0__blk895_dn11, locals.var_t0__blk895_dn12, locals.var_t0__blk895_dn17,)
    } else {
        (locals.var_t10__blk902, locals.var_t10__blk902_dn0, locals.var_t10__blk902_dn2, locals.var_t10__blk902_dn6, locals.var_t10__blk902_dn7, locals.var_t10__blk902_dn10, locals.var_t10__blk902_dn11, locals.var_t10__blk902_dn12, locals.var_t10__blk902_dn17,)
    }
};
        locals.var_t10__blk902 = assign30280_e43706;
        locals.var_t10__blk902_dn0 = assign30280_e43706_d_n0;
        locals.var_t10__blk902_dn2 = assign30280_e43706_d_n2;
        locals.var_t10__blk902_dn6 = assign30280_e43706_d_n6;
        locals.var_t10__blk902_dn7 = assign30280_e43706_d_n7;
        locals.var_t10__blk902_dn10 = assign30280_e43706_d_n10;
        locals.var_t10__blk902_dn11 = assign30280_e43706_d_n11;
        locals.var_t10__blk902_dn12 = assign30280_e43706_d_n12;
        locals.var_t10__blk902_dn17 = assign30280_e43706_d_n17;
        locals.var_t10__blk902_rv = 0.0;

        let (assign30300_e43730, assign30300_e43730_d_n0, assign30300_e43730_d_n2, assign30300_e43730_d_n6, assign30300_e43730_d_n7, assign30300_e43730_d_n10, assign30300_e43730_d_n11, assign30300_e43730_d_n12, assign30300_e43730_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign30300_e43726: f64 = (-locals.var_t10__blk902);
        let assign30300_e43728: f64 = (assign30300_e43726 - 1e-12);
        (assign30300_e43728, (-locals.var_t10__blk902_dn0), (-locals.var_t10__blk902_dn2), (-locals.var_t10__blk902_dn6), (-locals.var_t10__blk902_dn7), (-locals.var_t10__blk902_dn10), (-locals.var_t10__blk902_dn11), (-locals.var_t10__blk902_dn12), (-locals.var_t10__blk902_dn17),)
    } else {
        (locals.var_vxbgmtcl__blk921, locals.var_vxbgmtcl__blk921_dn0, locals.var_vxbgmtcl__blk921_dn2, locals.var_vxbgmtcl__blk921_dn6, locals.var_vxbgmtcl__blk921_dn7, locals.var_vxbgmtcl__blk921_dn10, locals.var_vxbgmtcl__blk921_dn11, locals.var_vxbgmtcl__blk921_dn12, locals.var_vxbgmtcl__blk921_dn17,)
    }
};
        locals.var_vxbgmtcl__blk921 = assign30300_e43730;
        locals.var_vxbgmtcl__blk921_dn0 = assign30300_e43730_d_n0;
        locals.var_vxbgmtcl__blk921_dn2 = assign30300_e43730_d_n2;
        locals.var_vxbgmtcl__blk921_dn6 = assign30300_e43730_d_n6;
        locals.var_vxbgmtcl__blk921_dn7 = assign30300_e43730_d_n7;
        locals.var_vxbgmtcl__blk921_dn10 = assign30300_e43730_d_n10;
        locals.var_vxbgmtcl__blk921_dn11 = assign30300_e43730_d_n11;
        locals.var_vxbgmtcl__blk921_dn12 = assign30300_e43730_d_n12;
        locals.var_vxbgmtcl__blk921_dn17 = assign30300_e43730_d_n17;
        locals.var_vxbgmtcl__blk921_rv = 0.0;

        let (assign30310_e43741, assign30310_e43741_d_n0, assign30310_e43741_d_n2, assign30310_e43741_d_n6, assign30310_e43741_d_n7, assign30310_e43741_d_n10, assign30310_e43741_d_n11, assign30310_e43741_d_n12, assign30310_e43741_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign30310_e43739: f64 = (locals.var_cnst0over__blk928 * locals.var_cox0_inv__blk907);
        (assign30310_e43739, (locals.var_cnst0over__blk928_dn0 * locals.var_cox0_inv__blk907), (locals.var_cnst0over__blk928_dn2 * locals.var_cox0_inv__blk907), (locals.var_cnst0over__blk928_dn6 * locals.var_cox0_inv__blk907), (locals.var_cnst0over__blk928_dn7 * locals.var_cox0_inv__blk907), (locals.var_cnst0over__blk928_dn10 * locals.var_cox0_inv__blk907), (locals.var_cnst0over__blk928_dn11 * locals.var_cox0_inv__blk907), (locals.var_cnst0over__blk928_dn12 * locals.var_cox0_inv__blk907), (locals.var_cnst0over__blk928_dn17 * locals.var_cox0_inv__blk907),)
    } else {
        (locals.var_fac1__blk929, locals.var_fac1__blk929_dn0, locals.var_fac1__blk929_dn2, locals.var_fac1__blk929_dn6, locals.var_fac1__blk929_dn7, locals.var_fac1__blk929_dn10, locals.var_fac1__blk929_dn11, locals.var_fac1__blk929_dn12, locals.var_fac1__blk929_dn17,)
    }
};
        locals.var_fac1__blk929 = assign30310_e43741;
        locals.var_fac1__blk929_dn0 = assign30310_e43741_d_n0;
        locals.var_fac1__blk929_dn2 = assign30310_e43741_d_n2;
        locals.var_fac1__blk929_dn6 = assign30310_e43741_d_n6;
        locals.var_fac1__blk929_dn7 = assign30310_e43741_d_n7;
        locals.var_fac1__blk929_dn10 = assign30310_e43741_d_n10;
        locals.var_fac1__blk929_dn11 = assign30310_e43741_d_n11;
        locals.var_fac1__blk929_dn12 = assign30310_e43741_d_n12;
        locals.var_fac1__blk929_dn17 = assign30310_e43741_d_n17;
        locals.var_fac1__blk929_rv = 0.0;

        let (assign30320_e43752, assign30320_e43752_d_n0, assign30320_e43752_d_n2, assign30320_e43752_d_n6, assign30320_e43752_d_n7, assign30320_e43752_d_n10, assign30320_e43752_d_n11, assign30320_e43752_d_n12, assign30320_e43752_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign30320_e43750: f64 = (locals.var_fac1__blk929 * locals.var_fac1__blk929);
        (assign30320_e43750, ((locals.var_fac1__blk929_dn0 * locals.var_fac1__blk929) + (locals.var_fac1__blk929 * locals.var_fac1__blk929_dn0)), ((locals.var_fac1__blk929_dn2 * locals.var_fac1__blk929) + (locals.var_fac1__blk929 * locals.var_fac1__blk929_dn2)), ((locals.var_fac1__blk929_dn6 * locals.var_fac1__blk929) + (locals.var_fac1__blk929 * locals.var_fac1__blk929_dn6)), ((locals.var_fac1__blk929_dn7 * locals.var_fac1__blk929) + (locals.var_fac1__blk929 * locals.var_fac1__blk929_dn7)), ((locals.var_fac1__blk929_dn10 * locals.var_fac1__blk929) + (locals.var_fac1__blk929 * locals.var_fac1__blk929_dn10)), ((locals.var_fac1__blk929_dn11 * locals.var_fac1__blk929) + (locals.var_fac1__blk929 * locals.var_fac1__blk929_dn11)), ((locals.var_fac1__blk929_dn12 * locals.var_fac1__blk929) + (locals.var_fac1__blk929 * locals.var_fac1__blk929_dn12)), ((locals.var_fac1__blk929_dn17 * locals.var_fac1__blk929) + (locals.var_fac1__blk929 * locals.var_fac1__blk929_dn17)),)
    } else {
        (locals.var_fac1p2__blk930, locals.var_fac1p2__blk930_dn0, locals.var_fac1p2__blk930_dn2, locals.var_fac1p2__blk930_dn6, locals.var_fac1p2__blk930_dn7, locals.var_fac1p2__blk930_dn10, locals.var_fac1p2__blk930_dn11, locals.var_fac1p2__blk930_dn12, locals.var_fac1p2__blk930_dn17,)
    }
};
        locals.var_fac1p2__blk930 = assign30320_e43752;
        locals.var_fac1p2__blk930_dn0 = assign30320_e43752_d_n0;
        locals.var_fac1p2__blk930_dn2 = assign30320_e43752_d_n2;
        locals.var_fac1p2__blk930_dn6 = assign30320_e43752_d_n6;
        locals.var_fac1p2__blk930_dn7 = assign30320_e43752_d_n7;
        locals.var_fac1p2__blk930_dn10 = assign30320_e43752_d_n10;
        locals.var_fac1p2__blk930_dn11 = assign30320_e43752_d_n11;
        locals.var_fac1p2__blk930_dn12 = assign30320_e43752_d_n12;
        locals.var_fac1p2__blk930_dn17 = assign30320_e43752_d_n17;
        locals.var_fac1p2__blk930_rv = 0.0;

        let (assign30330_e43764, assign30330_e43764_d_n0, assign30330_e43764_d_n2, assign30330_e43764_d_n6, assign30330_e43764_d_n7, assign30330_e43764_d_n10, assign30330_e43764_d_n11, assign30330_e43764_d_n12, assign30330_e43764_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign30330_e43760: f64 = (-locals.var_vgbgmt__blk927);
        let assign30330_e43762: f64 = (assign30330_e43760 + locals.var_uc_vfbover);
        (assign30330_e43762, (-locals.var_vgbgmt__blk927_dn0), (-locals.var_vgbgmt__blk927_dn2), (-locals.var_vgbgmt__blk927_dn6), (-locals.var_vgbgmt__blk927_dn7), (-locals.var_vgbgmt__blk927_dn10), (-locals.var_vgbgmt__blk927_dn11), (-locals.var_vgbgmt__blk927_dn12), (-locals.var_vgbgmt__blk927_dn17),)
    } else {
        (locals.var_vgpld__blk931, locals.var_vgpld__blk931_dn0, locals.var_vgpld__blk931_dn2, locals.var_vgpld__blk931_dn6, locals.var_vgpld__blk931_dn7, locals.var_vgpld__blk931_dn10, locals.var_vgpld__blk931_dn11, locals.var_vgpld__blk931_dn12, locals.var_vgpld__blk931_dn17,)
    }
};
        locals.var_vgpld__blk931 = assign30330_e43764;
        locals.var_vgpld__blk931_dn0 = assign30330_e43764_d_n0;
        locals.var_vgpld__blk931_dn2 = assign30330_e43764_d_n2;
        locals.var_vgpld__blk931_dn6 = assign30330_e43764_d_n6;
        locals.var_vgpld__blk931_dn7 = assign30330_e43764_d_n7;
        locals.var_vgpld__blk931_dn10 = assign30330_e43764_d_n10;
        locals.var_vgpld__blk931_dn11 = assign30330_e43764_d_n11;
        locals.var_vgpld__blk931_dn12 = assign30330_e43764_d_n12;
        locals.var_vgpld__blk931_dn17 = assign30330_e43764_d_n17;
        locals.var_vgpld__blk931_rv = 0.0;

        let (assign30340_e43775, assign30340_e43775_d_n0, assign30340_e43775_d_n2, assign30340_e43775_d_n6, assign30340_e43775_d_n7, assign30340_e43775_d_n10, assign30340_e43775_d_n11, assign30340_e43775_d_n12, assign30340_e43775_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign30340_e43773: f64 = (locals.var_mks_nover / locals.var_nin);
        (assign30340_e43773, (-((locals.var_mks_nover * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn12) / (locals.var_nin * locals.var_nin))), (-((locals.var_mks_nover * locals.var_nin_dn17) / (locals.var_nin * locals.var_nin))),)
    } else {
        (locals.var_t0__blk895, locals.var_t0__blk895_dn0, locals.var_t0__blk895_dn2, locals.var_t0__blk895_dn6, locals.var_t0__blk895_dn7, locals.var_t0__blk895_dn10, locals.var_t0__blk895_dn11, locals.var_t0__blk895_dn12, locals.var_t0__blk895_dn17,)
    }
};
        locals.var_t0__blk895 = assign30340_e43775;
        locals.var_t0__blk895_dn0 = assign30340_e43775_d_n0;
        locals.var_t0__blk895_dn2 = assign30340_e43775_d_n2;
        locals.var_t0__blk895_dn6 = assign30340_e43775_d_n6;
        locals.var_t0__blk895_dn7 = assign30340_e43775_d_n7;
        locals.var_t0__blk895_dn10 = assign30340_e43775_d_n10;
        locals.var_t0__blk895_dn11 = assign30340_e43775_d_n11;
        locals.var_t0__blk895_dn12 = assign30340_e43775_d_n12;
        locals.var_t0__blk895_dn17 = assign30340_e43775_d_n17;
        locals.var_t0__blk895_rv = 0.0;

        let (assign30350_e43789, assign30350_e43789_d_n0, assign30350_e43789_d_n2, assign30350_e43789_d_n6, assign30350_e43789_d_n7, assign30350_e43789_d_n10, assign30350_e43789_d_n11, assign30350_e43789_d_n12, assign30350_e43789_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign30350_e43784: f64 = (2.0 / locals.var_beta);
        let assign30350_e43786: f64 = (locals.var_t0__blk895).ln();
        let assign30350_e43787: f64 = (assign30350_e43784 * assign30350_e43786);
        (assign30350_e43787, (assign30350_e43784 * (locals.var_t0__blk895_dn0 / locals.var_t0__blk895)), (assign30350_e43784 * (locals.var_t0__blk895_dn2 / locals.var_t0__blk895)), (assign30350_e43784 * (locals.var_t0__blk895_dn6 / locals.var_t0__blk895)), (assign30350_e43784 * (locals.var_t0__blk895_dn7 / locals.var_t0__blk895)), (((-((2.0 * locals.var_beta_dn10) / (locals.var_beta * locals.var_beta))) * assign30350_e43786) + (assign30350_e43784 * (locals.var_t0__blk895_dn10 / locals.var_t0__blk895))), (assign30350_e43784 * (locals.var_t0__blk895_dn11 / locals.var_t0__blk895)), (assign30350_e43784 * (locals.var_t0__blk895_dn12 / locals.var_t0__blk895)), (assign30350_e43784 * (locals.var_t0__blk895_dn17 / locals.var_t0__blk895)),)
    } else {
        (locals.var_pb2over__blk932, locals.var_pb2over__blk932_dn0, locals.var_pb2over__blk932_dn2, locals.var_pb2over__blk932_dn6, locals.var_pb2over__blk932_dn7, locals.var_pb2over__blk932_dn10, locals.var_pb2over__blk932_dn11, locals.var_pb2over__blk932_dn12, locals.var_pb2over__blk932_dn17,)
    }
};
        locals.var_pb2over__blk932 = assign30350_e43789;
        locals.var_pb2over__blk932_dn0 = assign30350_e43789_d_n0;
        locals.var_pb2over__blk932_dn2 = assign30350_e43789_d_n2;
        locals.var_pb2over__blk932_dn6 = assign30350_e43789_d_n6;
        locals.var_pb2over__blk932_dn7 = assign30350_e43789_d_n7;
        locals.var_pb2over__blk932_dn10 = assign30350_e43789_d_n10;
        locals.var_pb2over__blk932_dn11 = assign30350_e43789_d_n11;
        locals.var_pb2over__blk932_dn12 = assign30350_e43789_d_n12;
        locals.var_pb2over__blk932_dn17 = assign30350_e43789_d_n17;
        locals.var_pb2over__blk932_rv = 0.0;

        let (assign30360_e43799, assign30360_e43799_d_n0, assign30360_e43799_d_n2, assign30360_e43799_d_n6, assign30360_e43799_d_n7, assign30360_e43799_d_n10, assign30360_e43799_d_n11, assign30360_e43799_d_n12, assign30360_e43799_d_n17,) = {
    if (((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) {
        let assign30360_e43797: f64 = (-locals.var_vxbgmtcl__blk921);
        (assign30360_e43797, (-locals.var_vxbgmtcl__blk921_dn0), (-locals.var_vxbgmtcl__blk921_dn2), (-locals.var_vxbgmtcl__blk921_dn6), (-locals.var_vxbgmtcl__blk921_dn7), (-locals.var_vxbgmtcl__blk921_dn10), (-locals.var_vxbgmtcl__blk921_dn11), (-locals.var_vxbgmtcl__blk921_dn12), (-locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_vgb_fb_ld__blk933, locals.var_vgb_fb_ld__blk933_dn0, locals.var_vgb_fb_ld__blk933_dn2, locals.var_vgb_fb_ld__blk933_dn6, locals.var_vgb_fb_ld__blk933_dn7, locals.var_vgb_fb_ld__blk933_dn10, locals.var_vgb_fb_ld__blk933_dn11, locals.var_vgb_fb_ld__blk933_dn12, locals.var_vgb_fb_ld__blk933_dn17,)
    }
};
        locals.var_vgb_fb_ld__blk933 = assign30360_e43799;
        locals.var_vgb_fb_ld__blk933_dn0 = assign30360_e43799_d_n0;
        locals.var_vgb_fb_ld__blk933_dn2 = assign30360_e43799_d_n2;
        locals.var_vgb_fb_ld__blk933_dn6 = assign30360_e43799_d_n6;
        locals.var_vgb_fb_ld__blk933_dn7 = assign30360_e43799_d_n7;
        locals.var_vgb_fb_ld__blk933_dn10 = assign30360_e43799_d_n10;
        locals.var_vgb_fb_ld__blk933_dn11 = assign30360_e43799_d_n11;
        locals.var_vgb_fb_ld__blk933_dn12 = assign30360_e43799_d_n12;
        locals.var_vgb_fb_ld__blk933_dn17 = assign30360_e43799_d_n17;
        locals.var_vgb_fb_ld__blk933_rv = 0.0;

        let assign30370_e43802: f64 = if locals.var_vgpld__blk931 < locals.var_vgb_fb_ld__blk933 { 1.0 } else { 0.0 };
        locals.var_guard1002 = assign30370_e43802;
        locals.var_guard1002_rv = 0.0;

        let (assign30390_e43829, assign30390_e43829_d_n0, assign30390_e43829_d_n2, assign30390_e43829_d_n6, assign30390_e43829_d_n7, assign30390_e43829_d_n10, assign30390_e43829_d_n11, assign30390_e43829_d_n12, assign30390_e43829_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30390_e43826: f64 = (locals.var_beta * locals.var_cnst0over__blk928);
        let assign30390_e43827: f64 = (1.0 / assign30390_e43826);
        (assign30390_e43827, (-((locals.var_beta * locals.var_cnst0over__blk928_dn0) / (assign30390_e43826 * assign30390_e43826))), (-((locals.var_beta * locals.var_cnst0over__blk928_dn2) / (assign30390_e43826 * assign30390_e43826))), (-((locals.var_beta * locals.var_cnst0over__blk928_dn6) / (assign30390_e43826 * assign30390_e43826))), (-((locals.var_beta * locals.var_cnst0over__blk928_dn7) / (assign30390_e43826 * assign30390_e43826))), (-(((locals.var_beta_dn10 * locals.var_cnst0over__blk928) + (locals.var_beta * locals.var_cnst0over__blk928_dn10)) / (assign30390_e43826 * assign30390_e43826))), (-((locals.var_beta * locals.var_cnst0over__blk928_dn11) / (assign30390_e43826 * assign30390_e43826))), (-((locals.var_beta * locals.var_cnst0over__blk928_dn12) / (assign30390_e43826 * assign30390_e43826))), (-((locals.var_beta * locals.var_cnst0over__blk928_dn17) / (assign30390_e43826 * assign30390_e43826))),)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign30390_e43829;
        locals.var_t1__blk896_dn0 = assign30390_e43829_d_n0;
        locals.var_t1__blk896_dn2 = assign30390_e43829_d_n2;
        locals.var_t1__blk896_dn6 = assign30390_e43829_d_n6;
        locals.var_t1__blk896_dn7 = assign30390_e43829_d_n7;
        locals.var_t1__blk896_dn10 = assign30390_e43829_d_n10;
        locals.var_t1__blk896_dn11 = assign30390_e43829_d_n11;
        locals.var_t1__blk896_dn12 = assign30390_e43829_d_n12;
        locals.var_t1__blk896_dn17 = assign30390_e43829_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let (assign30400_e43842, assign30400_e43842_d_n0, assign30400_e43842_d_n2, assign30400_e43842_d_n6, assign30400_e43842_d_n7, assign30400_e43842_d_n10, assign30400_e43842_d_n11, assign30400_e43842_d_n12, assign30400_e43842_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30400_e43840: f64 = (locals.var_t1__blk896 * locals.var_cox0__blk906);
        (assign30400_e43840, (locals.var_t1__blk896_dn0 * locals.var_cox0__blk906), (locals.var_t1__blk896_dn2 * locals.var_cox0__blk906), (locals.var_t1__blk896_dn6 * locals.var_cox0__blk906), (locals.var_t1__blk896_dn7 * locals.var_cox0__blk906), (locals.var_t1__blk896_dn10 * locals.var_cox0__blk906), (locals.var_t1__blk896_dn11 * locals.var_cox0__blk906), (locals.var_t1__blk896_dn12 * locals.var_cox0__blk906), (locals.var_t1__blk896_dn17 * locals.var_cox0__blk906),)
    } else {
        (locals.var_ty__blk905, locals.var_ty__blk905_dn0, locals.var_ty__blk905_dn2, locals.var_ty__blk905_dn6, locals.var_ty__blk905_dn7, locals.var_ty__blk905_dn10, locals.var_ty__blk905_dn11, locals.var_ty__blk905_dn12, locals.var_ty__blk905_dn17,)
    }
};
        locals.var_ty__blk905 = assign30400_e43842;
        locals.var_ty__blk905_dn0 = assign30400_e43842_d_n0;
        locals.var_ty__blk905_dn2 = assign30400_e43842_d_n2;
        locals.var_ty__blk905_dn6 = assign30400_e43842_d_n6;
        locals.var_ty__blk905_dn7 = assign30400_e43842_d_n7;
        locals.var_ty__blk905_dn10 = assign30400_e43842_d_n10;
        locals.var_ty__blk905_dn11 = assign30400_e43842_d_n11;
        locals.var_ty__blk905_dn12 = assign30400_e43842_d_n12;
        locals.var_ty__blk905_dn17 = assign30400_e43842_d_n17;
        locals.var_ty__blk905_rv = 0.0;

        let (assign30410_e43859, assign30410_e43859_d_n0, assign30410_e43859_d_n2, assign30410_e43859_d_n6, assign30410_e43859_d_n7, assign30410_e43859_d_n10, assign30410_e43859_d_n11, assign30410_e43859_d_n12, assign30410_e43859_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30410_e43854: f64 = (3.0 * 1.414213562373095);
        let assign30410_e43856: f64 = (assign30410_e43854 * locals.var_ty__blk905);
        let assign30410_e43857: f64 = (2.0 + assign30410_e43856);
        (assign30410_e43857, (assign30410_e43854 * locals.var_ty__blk905_dn0), (assign30410_e43854 * locals.var_ty__blk905_dn2), (assign30410_e43854 * locals.var_ty__blk905_dn6), (assign30410_e43854 * locals.var_ty__blk905_dn7), (assign30410_e43854 * locals.var_ty__blk905_dn10), (assign30410_e43854 * locals.var_ty__blk905_dn11), (assign30410_e43854 * locals.var_ty__blk905_dn12), (assign30410_e43854 * locals.var_ty__blk905_dn17),)
    } else {
        (locals.var_ac41__blk934, locals.var_ac41__blk934_dn0, locals.var_ac41__blk934_dn2, locals.var_ac41__blk934_dn6, locals.var_ac41__blk934_dn7, locals.var_ac41__blk934_dn10, locals.var_ac41__blk934_dn11, locals.var_ac41__blk934_dn12, locals.var_ac41__blk934_dn17,)
    }
};
        locals.var_ac41__blk934 = assign30410_e43859;
        locals.var_ac41__blk934_dn0 = assign30410_e43859_d_n0;
        locals.var_ac41__blk934_dn2 = assign30410_e43859_d_n2;
        locals.var_ac41__blk934_dn6 = assign30410_e43859_d_n6;
        locals.var_ac41__blk934_dn7 = assign30410_e43859_d_n7;
        locals.var_ac41__blk934_dn10 = assign30410_e43859_d_n10;
        locals.var_ac41__blk934_dn11 = assign30410_e43859_d_n11;
        locals.var_ac41__blk934_dn12 = assign30410_e43859_d_n12;
        locals.var_ac41__blk934_dn17 = assign30410_e43859_d_n17;
        locals.var_ac41__blk934_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_111(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign30420_e43876, assign30420_e43876_d_n0, assign30420_e43876_d_n2, assign30420_e43876_d_n6, assign30420_e43876_d_n7, assign30420_e43876_d_n10, assign30420_e43876_d_n11, assign30420_e43876_d_n12, assign30420_e43876_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30420_e43870: f64 = (8.0 * locals.var_ac41__blk934);
        let assign30420_e43872: f64 = (assign30420_e43870 * locals.var_ac41__blk934);
        let assign30420_e43874: f64 = (assign30420_e43872 * locals.var_ac41__blk934);
        (assign30420_e43874, (((((8.0 * locals.var_ac41__blk934_dn0) * locals.var_ac41__blk934) + (assign30420_e43870 * locals.var_ac41__blk934_dn0)) * locals.var_ac41__blk934) + (assign30420_e43872 * locals.var_ac41__blk934_dn0)), (((((8.0 * locals.var_ac41__blk934_dn2) * locals.var_ac41__blk934) + (assign30420_e43870 * locals.var_ac41__blk934_dn2)) * locals.var_ac41__blk934) + (assign30420_e43872 * locals.var_ac41__blk934_dn2)), (((((8.0 * locals.var_ac41__blk934_dn6) * locals.var_ac41__blk934) + (assign30420_e43870 * locals.var_ac41__blk934_dn6)) * locals.var_ac41__blk934) + (assign30420_e43872 * locals.var_ac41__blk934_dn6)), (((((8.0 * locals.var_ac41__blk934_dn7) * locals.var_ac41__blk934) + (assign30420_e43870 * locals.var_ac41__blk934_dn7)) * locals.var_ac41__blk934) + (assign30420_e43872 * locals.var_ac41__blk934_dn7)), (((((8.0 * locals.var_ac41__blk934_dn10) * locals.var_ac41__blk934) + (assign30420_e43870 * locals.var_ac41__blk934_dn10)) * locals.var_ac41__blk934) + (assign30420_e43872 * locals.var_ac41__blk934_dn10)), (((((8.0 * locals.var_ac41__blk934_dn11) * locals.var_ac41__blk934) + (assign30420_e43870 * locals.var_ac41__blk934_dn11)) * locals.var_ac41__blk934) + (assign30420_e43872 * locals.var_ac41__blk934_dn11)), (((((8.0 * locals.var_ac41__blk934_dn12) * locals.var_ac41__blk934) + (assign30420_e43870 * locals.var_ac41__blk934_dn12)) * locals.var_ac41__blk934) + (assign30420_e43872 * locals.var_ac41__blk934_dn12)), (((((8.0 * locals.var_ac41__blk934_dn17) * locals.var_ac41__blk934) + (assign30420_e43870 * locals.var_ac41__blk934_dn17)) * locals.var_ac41__blk934) + (assign30420_e43872 * locals.var_ac41__blk934_dn17)),)
    } else {
        (locals.var_ac4__blk935, locals.var_ac4__blk935_dn0, locals.var_ac4__blk935_dn2, locals.var_ac4__blk935_dn6, locals.var_ac4__blk935_dn7, locals.var_ac4__blk935_dn10, locals.var_ac4__blk935_dn11, locals.var_ac4__blk935_dn12, locals.var_ac4__blk935_dn17,)
    }
};
        locals.var_ac4__blk935 = assign30420_e43876;
        locals.var_ac4__blk935_dn0 = assign30420_e43876_d_n0;
        locals.var_ac4__blk935_dn2 = assign30420_e43876_d_n2;
        locals.var_ac4__blk935_dn6 = assign30420_e43876_d_n6;
        locals.var_ac4__blk935_dn7 = assign30420_e43876_d_n7;
        locals.var_ac4__blk935_dn10 = assign30420_e43876_d_n10;
        locals.var_ac4__blk935_dn11 = assign30420_e43876_d_n11;
        locals.var_ac4__blk935_dn12 = assign30420_e43876_d_n12;
        locals.var_ac4__blk935_dn17 = assign30420_e43876_d_n17;
        locals.var_ac4__blk935_rv = 0.0;

        let (assign30430_e43889, assign30430_e43889_d_n0, assign30430_e43889_d_n2, assign30430_e43889_d_n6, assign30430_e43889_d_n7, assign30430_e43889_d_n10, assign30430_e43889_d_n11, assign30430_e43889_d_n12, assign30430_e43889_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30430_e43887: f64 = (locals.var_eg - locals.var_pb2over__blk932);
        (assign30430_e43887, (locals.var_eg_dn0 - locals.var_pb2over__blk932_dn0), (locals.var_eg_dn2 - locals.var_pb2over__blk932_dn2), (locals.var_eg_dn6 - locals.var_pb2over__blk932_dn6), (locals.var_eg_dn7 - locals.var_pb2over__blk932_dn7), (locals.var_eg_dn10 - locals.var_pb2over__blk932_dn10), (locals.var_eg_dn11 - locals.var_pb2over__blk932_dn11), (locals.var_eg_dn12 - locals.var_pb2over__blk932_dn12), (locals.var_eg_dn17 - locals.var_pb2over__blk932_dn17),)
    } else {
        (locals.var_ps0_min__blk936, locals.var_ps0_min__blk936_dn0, locals.var_ps0_min__blk936_dn2, locals.var_ps0_min__blk936_dn6, locals.var_ps0_min__blk936_dn7, locals.var_ps0_min__blk936_dn10, locals.var_ps0_min__blk936_dn11, locals.var_ps0_min__blk936_dn12, locals.var_ps0_min__blk936_dn17,)
    }
};
        locals.var_ps0_min__blk936 = assign30430_e43889;
        locals.var_ps0_min__blk936_dn0 = assign30430_e43889_d_n0;
        locals.var_ps0_min__blk936_dn2 = assign30430_e43889_d_n2;
        locals.var_ps0_min__blk936_dn6 = assign30430_e43889_d_n6;
        locals.var_ps0_min__blk936_dn7 = assign30430_e43889_d_n7;
        locals.var_ps0_min__blk936_dn10 = assign30430_e43889_d_n10;
        locals.var_ps0_min__blk936_dn11 = assign30430_e43889_d_n11;
        locals.var_ps0_min__blk936_dn12 = assign30430_e43889_d_n12;
        locals.var_ps0_min__blk936_dn17 = assign30430_e43889_d_n17;
        locals.var_ps0_min__blk936_rv = 0.0;

        let (assign30440_e43904, assign30440_e43904_d_n0, assign30440_e43904_d_n2, assign30440_e43904_d_n6, assign30440_e43904_d_n7, assign30440_e43904_d_n10, assign30440_e43904_d_n11, assign30440_e43904_d_n12, assign30440_e43904_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30440_e43901: f64 = (locals.var_vgpld__blk931 + locals.var_vxbgmtcl__blk921);
        let assign30440_e43902: f64 = (locals.var_beta * assign30440_e43901);
        (assign30440_e43902, (locals.var_beta * (locals.var_vgpld__blk931_dn0 + locals.var_vxbgmtcl__blk921_dn0)), (locals.var_beta * (locals.var_vgpld__blk931_dn2 + locals.var_vxbgmtcl__blk921_dn2)), (locals.var_beta * (locals.var_vgpld__blk931_dn6 + locals.var_vxbgmtcl__blk921_dn6)), (locals.var_beta * (locals.var_vgpld__blk931_dn7 + locals.var_vxbgmtcl__blk921_dn7)), ((locals.var_beta_dn10 * assign30440_e43901) + (locals.var_beta * (locals.var_vgpld__blk931_dn10 + locals.var_vxbgmtcl__blk921_dn10))), (locals.var_beta * (locals.var_vgpld__blk931_dn11 + locals.var_vxbgmtcl__blk921_dn11)), (locals.var_beta * (locals.var_vgpld__blk931_dn12 + locals.var_vxbgmtcl__blk921_dn12)), (locals.var_beta * (locals.var_vgpld__blk931_dn17 + locals.var_vxbgmtcl__blk921_dn17)),)
    } else {
        (locals.var_tx__blk904, locals.var_tx__blk904_dn0, locals.var_tx__blk904_dn2, locals.var_tx__blk904_dn6, locals.var_tx__blk904_dn7, locals.var_tx__blk904_dn10, locals.var_tx__blk904_dn11, locals.var_tx__blk904_dn12, locals.var_tx__blk904_dn17,)
    }
};
        locals.var_tx__blk904 = assign30440_e43904;
        locals.var_tx__blk904_dn0 = assign30440_e43904_d_n0;
        locals.var_tx__blk904_dn2 = assign30440_e43904_d_n2;
        locals.var_tx__blk904_dn6 = assign30440_e43904_d_n6;
        locals.var_tx__blk904_dn7 = assign30440_e43904_d_n7;
        locals.var_tx__blk904_dn10 = assign30440_e43904_d_n10;
        locals.var_tx__blk904_dn11 = assign30440_e43904_d_n11;
        locals.var_tx__blk904_dn12 = assign30440_e43904_d_n12;
        locals.var_tx__blk904_dn17 = assign30440_e43904_d_n17;
        locals.var_tx__blk904_rv = 0.0;

        let (assign30450_e43925, assign30450_e43925_d_n0, assign30450_e43925_d_n2, assign30450_e43925_d_n6, assign30450_e43925_d_n7, assign30450_e43925_d_n10, assign30450_e43925_d_n11, assign30450_e43925_d_n12, assign30450_e43925_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30450_e43915: f64 = (7.0 * 1.414213562373095);
        let assign30450_e43918: f64 = (9.0 * locals.var_ty__blk905);
        let assign30450_e43921: f64 = (locals.var_tx__blk904 - 2.0);
        let assign30450_e43922: f64 = (assign30450_e43918 * assign30450_e43921);
        let assign30450_e43923: f64 = (assign30450_e43915 - assign30450_e43922);
        (assign30450_e43923, (-(((9.0 * locals.var_ty__blk905_dn0) * assign30450_e43921) + (assign30450_e43918 * locals.var_tx__blk904_dn0))), (-(((9.0 * locals.var_ty__blk905_dn2) * assign30450_e43921) + (assign30450_e43918 * locals.var_tx__blk904_dn2))), (-(((9.0 * locals.var_ty__blk905_dn6) * assign30450_e43921) + (assign30450_e43918 * locals.var_tx__blk904_dn6))), (-(((9.0 * locals.var_ty__blk905_dn7) * assign30450_e43921) + (assign30450_e43918 * locals.var_tx__blk904_dn7))), (-(((9.0 * locals.var_ty__blk905_dn10) * assign30450_e43921) + (assign30450_e43918 * locals.var_tx__blk904_dn10))), (-(((9.0 * locals.var_ty__blk905_dn11) * assign30450_e43921) + (assign30450_e43918 * locals.var_tx__blk904_dn11))), (-(((9.0 * locals.var_ty__blk905_dn12) * assign30450_e43921) + (assign30450_e43918 * locals.var_tx__blk904_dn12))), (-(((9.0 * locals.var_ty__blk905_dn17) * assign30450_e43921) + (assign30450_e43918 * locals.var_tx__blk904_dn17))),)
    } else {
        (locals.var_ac31__blk937, locals.var_ac31__blk937_dn0, locals.var_ac31__blk937_dn2, locals.var_ac31__blk937_dn6, locals.var_ac31__blk937_dn7, locals.var_ac31__blk937_dn10, locals.var_ac31__blk937_dn11, locals.var_ac31__blk937_dn12, locals.var_ac31__blk937_dn17,)
    }
};
        locals.var_ac31__blk937 = assign30450_e43925;
        locals.var_ac31__blk937_dn0 = assign30450_e43925_d_n0;
        locals.var_ac31__blk937_dn2 = assign30450_e43925_d_n2;
        locals.var_ac31__blk937_dn6 = assign30450_e43925_d_n6;
        locals.var_ac31__blk937_dn7 = assign30450_e43925_d_n7;
        locals.var_ac31__blk937_dn10 = assign30450_e43925_d_n10;
        locals.var_ac31__blk937_dn11 = assign30450_e43925_d_n11;
        locals.var_ac31__blk937_dn12 = assign30450_e43925_d_n12;
        locals.var_ac31__blk937_dn17 = assign30450_e43925_d_n17;
        locals.var_ac31__blk937_rv = 0.0;

        let (assign30460_e43938, assign30460_e43938_d_n0, assign30460_e43938_d_n2, assign30460_e43938_d_n6, assign30460_e43938_d_n7, assign30460_e43938_d_n10, assign30460_e43938_d_n11, assign30460_e43938_d_n12, assign30460_e43938_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30460_e43936: f64 = (locals.var_ac31__blk937 * locals.var_ac31__blk937);
        (assign30460_e43936, ((locals.var_ac31__blk937_dn0 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn0)), ((locals.var_ac31__blk937_dn2 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn2)), ((locals.var_ac31__blk937_dn6 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn6)), ((locals.var_ac31__blk937_dn7 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn7)), ((locals.var_ac31__blk937_dn10 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn10)), ((locals.var_ac31__blk937_dn11 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn11)), ((locals.var_ac31__blk937_dn12 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn12)), ((locals.var_ac31__blk937_dn17 * locals.var_ac31__blk937) + (locals.var_ac31__blk937 * locals.var_ac31__blk937_dn17)),)
    } else {
        (locals.var_ac3__blk938, locals.var_ac3__blk938_dn0, locals.var_ac3__blk938_dn2, locals.var_ac3__blk938_dn6, locals.var_ac3__blk938_dn7, locals.var_ac3__blk938_dn10, locals.var_ac3__blk938_dn11, locals.var_ac3__blk938_dn12, locals.var_ac3__blk938_dn17,)
    }
};
        locals.var_ac3__blk938 = assign30460_e43938;
        locals.var_ac3__blk938_dn0 = assign30460_e43938_d_n0;
        locals.var_ac3__blk938_dn2 = assign30460_e43938_d_n2;
        locals.var_ac3__blk938_dn6 = assign30460_e43938_d_n6;
        locals.var_ac3__blk938_dn7 = assign30460_e43938_d_n7;
        locals.var_ac3__blk938_dn10 = assign30460_e43938_d_n10;
        locals.var_ac3__blk938_dn11 = assign30460_e43938_d_n11;
        locals.var_ac3__blk938_dn12 = assign30460_e43938_d_n12;
        locals.var_ac3__blk938_dn17 = assign30460_e43938_d_n17;
        locals.var_ac3__blk938_rv = 0.0;

        let assign30470_e43942: f64 = (locals.var_ac3__blk938 * 1e-8);
        let assign30470_e43943: f64 = if locals.var_ac4__blk935 < assign30470_e43942 { 1.0 } else { 0.0 };
        locals.var_guard1003 = assign30470_e43943;
        locals.var_guard1003_rv = 0.0;

        let (assign30480_e43975, assign30480_e43975_d_n0, assign30480_e43975_d_n2, assign30480_e43975_d_n6, assign30480_e43975_d_n7, assign30480_e43975_d_n10, assign30480_e43975_d_n11, assign30480_e43975_d_n12, assign30480_e43975_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) && (locals.var_guard1003 != 0.0)) {
        let assign30480_e43955: f64 = (-7.0);
        let assign30480_e43957: f64 = (assign30480_e43955 * 1.414213562373095);
        let assign30480_e43959: f64 = (assign30480_e43957 + locals.var_ac31__blk937);
        let assign30480_e43962: f64 = (0.5 * locals.var_ac4__blk935);
        let assign30480_e43964: f64 = (assign30480_e43962 / locals.var_ac31__blk937);
        let assign30480_e43965: f64 = (assign30480_e43959 + assign30480_e43964);
        let assign30480_e43968: f64 = (9.0 * locals.var_ty__blk905);
        let assign30480_e43971: f64 = (locals.var_tx__blk904 - 2.0);
        let assign30480_e43972: f64 = (assign30480_e43968 * assign30480_e43971);
        let assign30480_e43973: f64 = (assign30480_e43965 + assign30480_e43972);
        (assign30480_e43973, ((locals.var_ac31__blk937_dn0 + ((((0.5 * locals.var_ac4__blk935_dn0) * locals.var_ac31__blk937) - (assign30480_e43962 * locals.var_ac31__blk937_dn0)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn0) * assign30480_e43971) + (assign30480_e43968 * locals.var_tx__blk904_dn0))), ((locals.var_ac31__blk937_dn2 + ((((0.5 * locals.var_ac4__blk935_dn2) * locals.var_ac31__blk937) - (assign30480_e43962 * locals.var_ac31__blk937_dn2)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn2) * assign30480_e43971) + (assign30480_e43968 * locals.var_tx__blk904_dn2))), ((locals.var_ac31__blk937_dn6 + ((((0.5 * locals.var_ac4__blk935_dn6) * locals.var_ac31__blk937) - (assign30480_e43962 * locals.var_ac31__blk937_dn6)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn6) * assign30480_e43971) + (assign30480_e43968 * locals.var_tx__blk904_dn6))), ((locals.var_ac31__blk937_dn7 + ((((0.5 * locals.var_ac4__blk935_dn7) * locals.var_ac31__blk937) - (assign30480_e43962 * locals.var_ac31__blk937_dn7)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn7) * assign30480_e43971) + (assign30480_e43968 * locals.var_tx__blk904_dn7))), ((locals.var_ac31__blk937_dn10 + ((((0.5 * locals.var_ac4__blk935_dn10) * locals.var_ac31__blk937) - (assign30480_e43962 * locals.var_ac31__blk937_dn10)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn10) * assign30480_e43971) + (assign30480_e43968 * locals.var_tx__blk904_dn10))), ((locals.var_ac31__blk937_dn11 + ((((0.5 * locals.var_ac4__blk935_dn11) * locals.var_ac31__blk937) - (assign30480_e43962 * locals.var_ac31__blk937_dn11)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn11) * assign30480_e43971) + (assign30480_e43968 * locals.var_tx__blk904_dn11))), ((locals.var_ac31__blk937_dn12 + ((((0.5 * locals.var_ac4__blk935_dn12) * locals.var_ac31__blk937) - (assign30480_e43962 * locals.var_ac31__blk937_dn12)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn12) * assign30480_e43971) + (assign30480_e43968 * locals.var_tx__blk904_dn12))), ((locals.var_ac31__blk937_dn17 + ((((0.5 * locals.var_ac4__blk935_dn17) * locals.var_ac31__blk937) - (assign30480_e43962 * locals.var_ac31__blk937_dn17)) / (locals.var_ac31__blk937 * locals.var_ac31__blk937))) + (((9.0 * locals.var_ty__blk905_dn17) * assign30480_e43971) + (assign30480_e43968 * locals.var_tx__blk904_dn17))),)
    } else {
        (locals.var_ac1__blk940, locals.var_ac1__blk940_dn0, locals.var_ac1__blk940_dn2, locals.var_ac1__blk940_dn6, locals.var_ac1__blk940_dn7, locals.var_ac1__blk940_dn10, locals.var_ac1__blk940_dn11, locals.var_ac1__blk940_dn12, locals.var_ac1__blk940_dn17,)
    }
};
        locals.var_ac1__blk940 = assign30480_e43975;
        locals.var_ac1__blk940_dn0 = assign30480_e43975_d_n0;
        locals.var_ac1__blk940_dn2 = assign30480_e43975_d_n2;
        locals.var_ac1__blk940_dn6 = assign30480_e43975_d_n6;
        locals.var_ac1__blk940_dn7 = assign30480_e43975_d_n7;
        locals.var_ac1__blk940_dn10 = assign30480_e43975_d_n10;
        locals.var_ac1__blk940_dn11 = assign30480_e43975_d_n11;
        locals.var_ac1__blk940_dn12 = assign30480_e43975_d_n12;
        locals.var_ac1__blk940_dn17 = assign30480_e43975_d_n17;
        locals.var_ac1__blk940_rv = 0.0;

        let (assign30490_e43992, assign30490_e43992_d_n0, assign30490_e43992_d_n2, assign30490_e43992_d_n6, assign30490_e43992_d_n7, assign30490_e43992_d_n10, assign30490_e43992_d_n11, assign30490_e43992_d_n12, assign30490_e43992_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) && (locals.var_guard1003 == 0.0)) {
        let assign30490_e43989: f64 = (locals.var_ac4__blk935 + locals.var_ac3__blk938);
        let assign30490_e43990: f64 = (assign30490_e43989).sqrt();
        (assign30490_e43990, ((locals.var_ac4__blk935_dn0 + locals.var_ac3__blk938_dn0) / (2.0 * assign30490_e43990)), ((locals.var_ac4__blk935_dn2 + locals.var_ac3__blk938_dn2) / (2.0 * assign30490_e43990)), ((locals.var_ac4__blk935_dn6 + locals.var_ac3__blk938_dn6) / (2.0 * assign30490_e43990)), ((locals.var_ac4__blk935_dn7 + locals.var_ac3__blk938_dn7) / (2.0 * assign30490_e43990)), ((locals.var_ac4__blk935_dn10 + locals.var_ac3__blk938_dn10) / (2.0 * assign30490_e43990)), ((locals.var_ac4__blk935_dn11 + locals.var_ac3__blk938_dn11) / (2.0 * assign30490_e43990)), ((locals.var_ac4__blk935_dn12 + locals.var_ac3__blk938_dn12) / (2.0 * assign30490_e43990)), ((locals.var_ac4__blk935_dn17 + locals.var_ac3__blk938_dn17) / (2.0 * assign30490_e43990)),)
    } else {
        (locals.var_ac2__blk939, locals.var_ac2__blk939_dn0, locals.var_ac2__blk939_dn2, locals.var_ac2__blk939_dn6, locals.var_ac2__blk939_dn7, locals.var_ac2__blk939_dn10, locals.var_ac2__blk939_dn11, locals.var_ac2__blk939_dn12, locals.var_ac2__blk939_dn17,)
    }
};
        locals.var_ac2__blk939 = assign30490_e43992;
        locals.var_ac2__blk939_dn0 = assign30490_e43992_d_n0;
        locals.var_ac2__blk939_dn2 = assign30490_e43992_d_n2;
        locals.var_ac2__blk939_dn6 = assign30490_e43992_d_n6;
        locals.var_ac2__blk939_dn7 = assign30490_e43992_d_n7;
        locals.var_ac2__blk939_dn10 = assign30490_e43992_d_n10;
        locals.var_ac2__blk939_dn11 = assign30490_e43992_d_n11;
        locals.var_ac2__blk939_dn12 = assign30490_e43992_d_n12;
        locals.var_ac2__blk939_dn17 = assign30490_e43992_d_n17;
        locals.var_ac2__blk939_rv = 0.0;

        let (assign30500_e44019, assign30500_e44019_d_n0, assign30500_e44019_d_n2, assign30500_e44019_d_n6, assign30500_e44019_d_n7, assign30500_e44019_d_n10, assign30500_e44019_d_n11, assign30500_e44019_d_n12, assign30500_e44019_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) && (locals.var_guard1003 == 0.0)) {
        let assign30500_e44005: f64 = (-7.0);
        let assign30500_e44007: f64 = (assign30500_e44005 * 1.414213562373095);
        let assign30500_e44009: f64 = (assign30500_e44007 + locals.var_ac2__blk939);
        let assign30500_e44012: f64 = (9.0 * locals.var_ty__blk905);
        let assign30500_e44015: f64 = (locals.var_tx__blk904 - 2.0);
        let assign30500_e44016: f64 = (assign30500_e44012 * assign30500_e44015);
        let assign30500_e44017: f64 = (assign30500_e44009 + assign30500_e44016);
        (assign30500_e44017, (locals.var_ac2__blk939_dn0 + (((9.0 * locals.var_ty__blk905_dn0) * assign30500_e44015) + (assign30500_e44012 * locals.var_tx__blk904_dn0))), (locals.var_ac2__blk939_dn2 + (((9.0 * locals.var_ty__blk905_dn2) * assign30500_e44015) + (assign30500_e44012 * locals.var_tx__blk904_dn2))), (locals.var_ac2__blk939_dn6 + (((9.0 * locals.var_ty__blk905_dn6) * assign30500_e44015) + (assign30500_e44012 * locals.var_tx__blk904_dn6))), (locals.var_ac2__blk939_dn7 + (((9.0 * locals.var_ty__blk905_dn7) * assign30500_e44015) + (assign30500_e44012 * locals.var_tx__blk904_dn7))), (locals.var_ac2__blk939_dn10 + (((9.0 * locals.var_ty__blk905_dn10) * assign30500_e44015) + (assign30500_e44012 * locals.var_tx__blk904_dn10))), (locals.var_ac2__blk939_dn11 + (((9.0 * locals.var_ty__blk905_dn11) * assign30500_e44015) + (assign30500_e44012 * locals.var_tx__blk904_dn11))), (locals.var_ac2__blk939_dn12 + (((9.0 * locals.var_ty__blk905_dn12) * assign30500_e44015) + (assign30500_e44012 * locals.var_tx__blk904_dn12))), (locals.var_ac2__blk939_dn17 + (((9.0 * locals.var_ty__blk905_dn17) * assign30500_e44015) + (assign30500_e44012 * locals.var_tx__blk904_dn17))),)
    } else {
        (locals.var_ac1__blk940, locals.var_ac1__blk940_dn0, locals.var_ac1__blk940_dn2, locals.var_ac1__blk940_dn6, locals.var_ac1__blk940_dn7, locals.var_ac1__blk940_dn10, locals.var_ac1__blk940_dn11, locals.var_ac1__blk940_dn12, locals.var_ac1__blk940_dn17,)
    }
};
        locals.var_ac1__blk940 = assign30500_e44019;
        locals.var_ac1__blk940_dn0 = assign30500_e44019_d_n0;
        locals.var_ac1__blk940_dn2 = assign30500_e44019_d_n2;
        locals.var_ac1__blk940_dn6 = assign30500_e44019_d_n6;
        locals.var_ac1__blk940_dn7 = assign30500_e44019_d_n7;
        locals.var_ac1__blk940_dn10 = assign30500_e44019_d_n10;
        locals.var_ac1__blk940_dn11 = assign30500_e44019_d_n11;
        locals.var_ac1__blk940_dn12 = assign30500_e44019_d_n12;
        locals.var_ac1__blk940_dn17 = assign30500_e44019_d_n17;
        locals.var_ac1__blk940_rv = 0.0;

        let (assign30510_e44032, assign30510_e44032_d_n0, assign30510_e44032_d_n2, assign30510_e44032_d_n6, assign30510_e44032_d_n7, assign30510_e44032_d_n10, assign30510_e44032_d_n11, assign30510_e44032_d_n12, assign30510_e44032_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30510_e44030: f64 = (locals.var_ac1__blk940).powf(0.3333333333333333);
        (assign30510_e44030, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn0)) } } else { (assign30510_e44030 * (0.3333333333333333 * (locals.var_ac1__blk940_dn0 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn2)) } } else { (assign30510_e44030 * (0.3333333333333333 * (locals.var_ac1__blk940_dn2 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn6)) } } else { (assign30510_e44030 * (0.3333333333333333 * (locals.var_ac1__blk940_dn6 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn7)) } } else { (assign30510_e44030 * (0.3333333333333333 * (locals.var_ac1__blk940_dn7 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn10)) } } else { (assign30510_e44030 * (0.3333333333333333 * (locals.var_ac1__blk940_dn10 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn11)) } } else { (assign30510_e44030 * (0.3333333333333333 * (locals.var_ac1__blk940_dn11 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn12)) } } else { (assign30510_e44030 * (0.3333333333333333 * (locals.var_ac1__blk940_dn12 / locals.var_ac1__blk940))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_ac1__blk940).powf(0.3333333333333333 - 1.0) * locals.var_ac1__blk940_dn17)) } } else { (assign30510_e44030 * (0.3333333333333333 * (locals.var_ac1__blk940_dn17 / locals.var_ac1__blk940))) },)
    } else {
        (locals.var_acd__blk941, locals.var_acd__blk941_dn0, locals.var_acd__blk941_dn2, locals.var_acd__blk941_dn6, locals.var_acd__blk941_dn7, locals.var_acd__blk941_dn10, locals.var_acd__blk941_dn11, locals.var_acd__blk941_dn12, locals.var_acd__blk941_dn17,)
    }
};
        locals.var_acd__blk941 = assign30510_e44032;
        locals.var_acd__blk941_dn0 = assign30510_e44032_d_n0;
        locals.var_acd__blk941_dn2 = assign30510_e44032_d_n2;
        locals.var_acd__blk941_dn6 = assign30510_e44032_d_n6;
        locals.var_acd__blk941_dn7 = assign30510_e44032_d_n7;
        locals.var_acd__blk941_dn10 = assign30510_e44032_d_n10;
        locals.var_acd__blk941_dn11 = assign30510_e44032_d_n11;
        locals.var_acd__blk941_dn12 = assign30510_e44032_d_n12;
        locals.var_acd__blk941_dn17 = assign30510_e44032_d_n17;
        locals.var_acd__blk941_rv = 0.0;

        let (assign30520_e44060, assign30520_e44060_d_n0, assign30520_e44060_d_n2, assign30520_e44060_d_n6, assign30520_e44060_d_n7, assign30520_e44060_d_n10, assign30520_e44060_d_n11, assign30520_e44060_d_n12, assign30520_e44060_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30520_e44042: f64 = (-4.0);
        let assign30520_e44044: f64 = (assign30520_e44042 * 1.414213562373095);
        let assign30520_e44047: f64 = (12.0 * locals.var_ty__blk905);
        let assign30520_e44048: f64 = (assign30520_e44044 - assign30520_e44047);
        let assign30520_e44051: f64 = (2.0 * locals.var_acd__blk941);
        let assign30520_e44052: f64 = (assign30520_e44048 + assign30520_e44051);
        let assign30520_e44055: f64 = (1.414213562373095 * locals.var_acd__blk941);
        let assign30520_e44057: f64 = (assign30520_e44055 * locals.var_acd__blk941);
        let assign30520_e44058: f64 = (assign30520_e44052 + assign30520_e44057);
        (assign30520_e44058, (((-(12.0 * locals.var_ty__blk905_dn0)) + (2.0 * locals.var_acd__blk941_dn0)) + (((1.414213562373095 * locals.var_acd__blk941_dn0) * locals.var_acd__blk941) + (assign30520_e44055 * locals.var_acd__blk941_dn0))), (((-(12.0 * locals.var_ty__blk905_dn2)) + (2.0 * locals.var_acd__blk941_dn2)) + (((1.414213562373095 * locals.var_acd__blk941_dn2) * locals.var_acd__blk941) + (assign30520_e44055 * locals.var_acd__blk941_dn2))), (((-(12.0 * locals.var_ty__blk905_dn6)) + (2.0 * locals.var_acd__blk941_dn6)) + (((1.414213562373095 * locals.var_acd__blk941_dn6) * locals.var_acd__blk941) + (assign30520_e44055 * locals.var_acd__blk941_dn6))), (((-(12.0 * locals.var_ty__blk905_dn7)) + (2.0 * locals.var_acd__blk941_dn7)) + (((1.414213562373095 * locals.var_acd__blk941_dn7) * locals.var_acd__blk941) + (assign30520_e44055 * locals.var_acd__blk941_dn7))), (((-(12.0 * locals.var_ty__blk905_dn10)) + (2.0 * locals.var_acd__blk941_dn10)) + (((1.414213562373095 * locals.var_acd__blk941_dn10) * locals.var_acd__blk941) + (assign30520_e44055 * locals.var_acd__blk941_dn10))), (((-(12.0 * locals.var_ty__blk905_dn11)) + (2.0 * locals.var_acd__blk941_dn11)) + (((1.414213562373095 * locals.var_acd__blk941_dn11) * locals.var_acd__blk941) + (assign30520_e44055 * locals.var_acd__blk941_dn11))), (((-(12.0 * locals.var_ty__blk905_dn12)) + (2.0 * locals.var_acd__blk941_dn12)) + (((1.414213562373095 * locals.var_acd__blk941_dn12) * locals.var_acd__blk941) + (assign30520_e44055 * locals.var_acd__blk941_dn12))), (((-(12.0 * locals.var_ty__blk905_dn17)) + (2.0 * locals.var_acd__blk941_dn17)) + (((1.414213562373095 * locals.var_acd__blk941_dn17) * locals.var_acd__blk941) + (assign30520_e44055 * locals.var_acd__blk941_dn17))),)
    } else {
        (locals.var_acn__blk942, locals.var_acn__blk942_dn0, locals.var_acn__blk942_dn2, locals.var_acn__blk942_dn6, locals.var_acn__blk942_dn7, locals.var_acn__blk942_dn10, locals.var_acn__blk942_dn11, locals.var_acn__blk942_dn12, locals.var_acn__blk942_dn17,)
    }
};
        locals.var_acn__blk942 = assign30520_e44060;
        locals.var_acn__blk942_dn0 = assign30520_e44060_d_n0;
        locals.var_acn__blk942_dn2 = assign30520_e44060_d_n2;
        locals.var_acn__blk942_dn6 = assign30520_e44060_d_n6;
        locals.var_acn__blk942_dn7 = assign30520_e44060_d_n7;
        locals.var_acn__blk942_dn10 = assign30520_e44060_d_n10;
        locals.var_acn__blk942_dn11 = assign30520_e44060_d_n11;
        locals.var_acn__blk942_dn12 = assign30520_e44060_d_n12;
        locals.var_acn__blk942_dn17 = assign30520_e44060_d_n17;
        locals.var_acn__blk942_rv = 0.0;

        let (assign30530_e44073, assign30530_e44073_d_n0, assign30530_e44073_d_n2, assign30530_e44073_d_n6, assign30530_e44073_d_n7, assign30530_e44073_d_n10, assign30530_e44073_d_n11, assign30530_e44073_d_n12, assign30530_e44073_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30530_e44071: f64 = (locals.var_acn__blk942 / locals.var_acd__blk941);
        (assign30530_e44071, (((locals.var_acn__blk942_dn0 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn0)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn2 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn2)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn6 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn6)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn7 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn7)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn10 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn10)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn11 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn11)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn12 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn12)) / (locals.var_acd__blk941 * locals.var_acd__blk941)), (((locals.var_acn__blk942_dn17 * locals.var_acd__blk941) - (locals.var_acn__blk942 * locals.var_acd__blk941_dn17)) / (locals.var_acd__blk941 * locals.var_acd__blk941)),)
    } else {
        (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    }
};
        locals.var_chi__blk943 = assign30530_e44073;
        locals.var_chi__blk943_dn0 = assign30530_e44073_d_n0;
        locals.var_chi__blk943_dn2 = assign30530_e44073_d_n2;
        locals.var_chi__blk943_dn6 = assign30530_e44073_d_n6;
        locals.var_chi__blk943_dn7 = assign30530_e44073_d_n7;
        locals.var_chi__blk943_dn10 = assign30530_e44073_d_n10;
        locals.var_chi__blk943_dn11 = assign30530_e44073_d_n11;
        locals.var_chi__blk943_dn12 = assign30530_e44073_d_n12;
        locals.var_chi__blk943_dn17 = assign30530_e44073_d_n17;
        locals.var_chi__blk943_rv = 0.0;

        let (assign30540_e44088, assign30540_e44088_d_n0, assign30540_e44088_d_n2, assign30540_e44088_d_n6, assign30540_e44088_d_n7, assign30540_e44088_d_n10, assign30540_e44088_d_n11, assign30540_e44088_d_n12, assign30540_e44088_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30540_e44084: f64 = (locals.var_chi__blk943 * locals.var_beta_inv);
        let assign30540_e44086: f64 = (assign30540_e44084 - locals.var_vxbgmtcl__blk921);
        (assign30540_e44086, ((locals.var_chi__blk943_dn0 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn0), ((locals.var_chi__blk943_dn2 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn2), ((locals.var_chi__blk943_dn6 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn6), ((locals.var_chi__blk943_dn7 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn7), (((locals.var_chi__blk943_dn10 * locals.var_beta_inv) + (locals.var_chi__blk943 * locals.var_beta_inv_dn10)) - locals.var_vxbgmtcl__blk921_dn10), ((locals.var_chi__blk943_dn11 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn11), ((locals.var_chi__blk943_dn12 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn12), ((locals.var_chi__blk943_dn17 * locals.var_beta_inv) - locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_psa__blk944, locals.var_psa__blk944_dn0, locals.var_psa__blk944_dn2, locals.var_psa__blk944_dn6, locals.var_psa__blk944_dn7, locals.var_psa__blk944_dn10, locals.var_psa__blk944_dn11, locals.var_psa__blk944_dn12, locals.var_psa__blk944_dn17,)
    }
};
        locals.var_psa__blk944 = assign30540_e44088;
        locals.var_psa__blk944_dn0 = assign30540_e44088_d_n0;
        locals.var_psa__blk944_dn2 = assign30540_e44088_d_n2;
        locals.var_psa__blk944_dn6 = assign30540_e44088_d_n6;
        locals.var_psa__blk944_dn7 = assign30540_e44088_d_n7;
        locals.var_psa__blk944_dn10 = assign30540_e44088_d_n10;
        locals.var_psa__blk944_dn11 = assign30540_e44088_d_n11;
        locals.var_psa__blk944_dn12 = assign30540_e44088_d_n12;
        locals.var_psa__blk944_dn17 = assign30540_e44088_d_n17;
        locals.var_psa__blk944_rv = 0.0;

        let (assign30550_e44101, assign30550_e44101_d_n0, assign30550_e44101_d_n2, assign30550_e44101_d_n6, assign30550_e44101_d_n7, assign30550_e44101_d_n10, assign30550_e44101_d_n11, assign30550_e44101_d_n12, assign30550_e44101_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30550_e44099: f64 = (locals.var_psa__blk944 + locals.var_vxbgmtcl__blk921);
        (assign30550_e44099, (locals.var_psa__blk944_dn0 + locals.var_vxbgmtcl__blk921_dn0), (locals.var_psa__blk944_dn2 + locals.var_vxbgmtcl__blk921_dn2), (locals.var_psa__blk944_dn6 + locals.var_vxbgmtcl__blk921_dn6), (locals.var_psa__blk944_dn7 + locals.var_vxbgmtcl__blk921_dn7), (locals.var_psa__blk944_dn10 + locals.var_vxbgmtcl__blk921_dn10), (locals.var_psa__blk944_dn11 + locals.var_vxbgmtcl__blk921_dn11), (locals.var_psa__blk944_dn12 + locals.var_vxbgmtcl__blk921_dn12), (locals.var_psa__blk944_dn17 + locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_t1__blk896, locals.var_t1__blk896_dn0, locals.var_t1__blk896_dn2, locals.var_t1__blk896_dn6, locals.var_t1__blk896_dn7, locals.var_t1__blk896_dn10, locals.var_t1__blk896_dn11, locals.var_t1__blk896_dn12, locals.var_t1__blk896_dn17,)
    }
};
        locals.var_t1__blk896 = assign30550_e44101;
        locals.var_t1__blk896_dn0 = assign30550_e44101_d_n0;
        locals.var_t1__blk896_dn2 = assign30550_e44101_d_n2;
        locals.var_t1__blk896_dn6 = assign30550_e44101_d_n6;
        locals.var_t1__blk896_dn7 = assign30550_e44101_d_n7;
        locals.var_t1__blk896_dn10 = assign30550_e44101_d_n10;
        locals.var_t1__blk896_dn11 = assign30550_e44101_d_n11;
        locals.var_t1__blk896_dn12 = assign30550_e44101_d_n12;
        locals.var_t1__blk896_dn17 = assign30550_e44101_d_n17;
        locals.var_t1__blk896_rv = 0.0;

        let (assign30560_e44114, assign30560_e44114_d_n0, assign30560_e44114_d_n2, assign30560_e44114_d_n6, assign30560_e44114_d_n7, assign30560_e44114_d_n10, assign30560_e44114_d_n11, assign30560_e44114_d_n12, assign30560_e44114_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30560_e44112: f64 = (locals.var_t1__blk896 / locals.var_ps0_min__blk936);
        (assign30560_e44112, (((locals.var_t1__blk896_dn0 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn0)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn2 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn2)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn6 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn6)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn7 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn7)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn10 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn10)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn11 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn11)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn12 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn12)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)), (((locals.var_t1__blk896_dn17 * locals.var_ps0_min__blk936) - (locals.var_t1__blk896 * locals.var_ps0_min__blk936_dn17)) / (locals.var_ps0_min__blk936 * locals.var_ps0_min__blk936)),)
    } else {
        (locals.var_t2__blk897, locals.var_t2__blk897_dn0, locals.var_t2__blk897_dn2, locals.var_t2__blk897_dn6, locals.var_t2__blk897_dn7, locals.var_t2__blk897_dn10, locals.var_t2__blk897_dn11, locals.var_t2__blk897_dn12, locals.var_t2__blk897_dn17,)
    }
};
        locals.var_t2__blk897 = assign30560_e44114;
        locals.var_t2__blk897_dn0 = assign30560_e44114_d_n0;
        locals.var_t2__blk897_dn2 = assign30560_e44114_d_n2;
        locals.var_t2__blk897_dn6 = assign30560_e44114_d_n6;
        locals.var_t2__blk897_dn7 = assign30560_e44114_d_n7;
        locals.var_t2__blk897_dn10 = assign30560_e44114_d_n10;
        locals.var_t2__blk897_dn11 = assign30560_e44114_d_n11;
        locals.var_t2__blk897_dn12 = assign30560_e44114_d_n12;
        locals.var_t2__blk897_dn17 = assign30560_e44114_d_n17;
        locals.var_t2__blk897_rv = 0.0;

        let (assign30570_e44130, assign30570_e44130_d_n0, assign30570_e44130_d_n2, assign30570_e44130_d_n6, assign30570_e44130_d_n7, assign30570_e44130_d_n10, assign30570_e44130_d_n11, assign30570_e44130_d_n12, assign30570_e44130_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30570_e44126: f64 = (locals.var_t2__blk897 * locals.var_t2__blk897);
        let assign30570_e44127: f64 = (1.0 + assign30570_e44126);
        let assign30570_e44128: f64 = (assign30570_e44127).sqrt();
        (assign30570_e44128, (((locals.var_t2__blk897_dn0 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn0)) / (2.0 * assign30570_e44128)), (((locals.var_t2__blk897_dn2 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn2)) / (2.0 * assign30570_e44128)), (((locals.var_t2__blk897_dn6 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn6)) / (2.0 * assign30570_e44128)), (((locals.var_t2__blk897_dn7 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn7)) / (2.0 * assign30570_e44128)), (((locals.var_t2__blk897_dn10 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn10)) / (2.0 * assign30570_e44128)), (((locals.var_t2__blk897_dn11 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn11)) / (2.0 * assign30570_e44128)), (((locals.var_t2__blk897_dn12 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn12)) / (2.0 * assign30570_e44128)), (((locals.var_t2__blk897_dn17 * locals.var_t2__blk897) + (locals.var_t2__blk897 * locals.var_t2__blk897_dn17)) / (2.0 * assign30570_e44128)),)
    } else {
        (locals.var_t3__blk898, locals.var_t3__blk898_dn0, locals.var_t3__blk898_dn2, locals.var_t3__blk898_dn6, locals.var_t3__blk898_dn7, locals.var_t3__blk898_dn10, locals.var_t3__blk898_dn11, locals.var_t3__blk898_dn12, locals.var_t3__blk898_dn17,)
    }
};
        locals.var_t3__blk898 = assign30570_e44130;
        locals.var_t3__blk898_dn0 = assign30570_e44130_d_n0;
        locals.var_t3__blk898_dn2 = assign30570_e44130_d_n2;
        locals.var_t3__blk898_dn6 = assign30570_e44130_d_n6;
        locals.var_t3__blk898_dn7 = assign30570_e44130_d_n7;
        locals.var_t3__blk898_dn10 = assign30570_e44130_d_n10;
        locals.var_t3__blk898_dn11 = assign30570_e44130_d_n11;
        locals.var_t3__blk898_dn12 = assign30570_e44130_d_n12;
        locals.var_t3__blk898_dn17 = assign30570_e44130_d_n17;
        locals.var_t3__blk898_rv = 0.0;

        let (assign30580_e44145, assign30580_e44145_d_n0, assign30580_e44145_d_n2, assign30580_e44145_d_n6, assign30580_e44145_d_n7, assign30580_e44145_d_n10, assign30580_e44145_d_n11, assign30580_e44145_d_n12, assign30580_e44145_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30580_e44141: f64 = (locals.var_t1__blk896 / locals.var_t3__blk898);
        let assign30580_e44143: f64 = (assign30580_e44141 - locals.var_vxbgmtcl__blk921);
        (assign30580_e44143, ((((locals.var_t1__blk896_dn0 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn0)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn0), ((((locals.var_t1__blk896_dn2 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn2)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn2), ((((locals.var_t1__blk896_dn6 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn6)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn6), ((((locals.var_t1__blk896_dn7 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn7)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn7), ((((locals.var_t1__blk896_dn10 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn10)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn10), ((((locals.var_t1__blk896_dn11 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn11)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn11), ((((locals.var_t1__blk896_dn12 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn12)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn12), ((((locals.var_t1__blk896_dn17 * locals.var_t3__blk898) - (locals.var_t1__blk896 * locals.var_t3__blk898_dn17)) / (locals.var_t3__blk898 * locals.var_t3__blk898)) - locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_ps0ld__blk945, locals.var_ps0ld__blk945_dn0, locals.var_ps0ld__blk945_dn2, locals.var_ps0ld__blk945_dn6, locals.var_ps0ld__blk945_dn7, locals.var_ps0ld__blk945_dn10, locals.var_ps0ld__blk945_dn11, locals.var_ps0ld__blk945_dn12, locals.var_ps0ld__blk945_dn17,)
    }
};
        locals.var_ps0ld__blk945 = assign30580_e44145;
        locals.var_ps0ld__blk945_dn0 = assign30580_e44145_d_n0;
        locals.var_ps0ld__blk945_dn2 = assign30580_e44145_d_n2;
        locals.var_ps0ld__blk945_dn6 = assign30580_e44145_d_n6;
        locals.var_ps0ld__blk945_dn7 = assign30580_e44145_d_n7;
        locals.var_ps0ld__blk945_dn10 = assign30580_e44145_d_n10;
        locals.var_ps0ld__blk945_dn11 = assign30580_e44145_d_n11;
        locals.var_ps0ld__blk945_dn12 = assign30580_e44145_d_n12;
        locals.var_ps0ld__blk945_dn17 = assign30580_e44145_d_n17;
        locals.var_ps0ld__blk945_rv = 0.0;

        let (assign30590_e44158, assign30590_e44158_d_n0, assign30590_e44158_d_n2, assign30590_e44158_d_n6, assign30590_e44158_d_n7, assign30590_e44158_d_n10, assign30590_e44158_d_n11, assign30590_e44158_d_n12, assign30590_e44158_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30590_e44156: f64 = (locals.var_vgpld__blk931 - locals.var_ps0ld__blk945);
        (assign30590_e44156, (locals.var_vgpld__blk931_dn0 - locals.var_ps0ld__blk945_dn0), (locals.var_vgpld__blk931_dn2 - locals.var_ps0ld__blk945_dn2), (locals.var_vgpld__blk931_dn6 - locals.var_ps0ld__blk945_dn6), (locals.var_vgpld__blk931_dn7 - locals.var_ps0ld__blk945_dn7), (locals.var_vgpld__blk931_dn10 - locals.var_ps0ld__blk945_dn10), (locals.var_vgpld__blk931_dn11 - locals.var_ps0ld__blk945_dn11), (locals.var_vgpld__blk931_dn12 - locals.var_ps0ld__blk945_dn12), (locals.var_vgpld__blk931_dn17 - locals.var_ps0ld__blk945_dn17),)
    } else {
        (locals.var_t2__blk897, locals.var_t2__blk897_dn0, locals.var_t2__blk897_dn2, locals.var_t2__blk897_dn6, locals.var_t2__blk897_dn7, locals.var_t2__blk897_dn10, locals.var_t2__blk897_dn11, locals.var_t2__blk897_dn12, locals.var_t2__blk897_dn17,)
    }
};
        locals.var_t2__blk897 = assign30590_e44158;
        locals.var_t2__blk897_dn0 = assign30590_e44158_d_n0;
        locals.var_t2__blk897_dn2 = assign30590_e44158_d_n2;
        locals.var_t2__blk897_dn6 = assign30590_e44158_d_n6;
        locals.var_t2__blk897_dn7 = assign30590_e44158_d_n7;
        locals.var_t2__blk897_dn10 = assign30590_e44158_d_n10;
        locals.var_t2__blk897_dn11 = assign30590_e44158_d_n11;
        locals.var_t2__blk897_dn12 = assign30590_e44158_d_n12;
        locals.var_t2__blk897_dn17 = assign30590_e44158_d_n17;
        locals.var_t2__blk897_rv = 0.0;

        let (assign30600_e44171, assign30600_e44171_d_n0, assign30600_e44171_d_n2, assign30600_e44171_d_n6, assign30600_e44171_d_n7, assign30600_e44171_d_n10, assign30600_e44171_d_n11, assign30600_e44171_d_n12, assign30600_e44171_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        let assign30600_e44169: f64 = (locals.var_cox0__blk906 * locals.var_t2__blk897);
        (assign30600_e44169, (locals.var_cox0__blk906 * locals.var_t2__blk897_dn0), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn2), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn6), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn7), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn10), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn11), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn12), (locals.var_cox0__blk906 * locals.var_t2__blk897_dn17),)
    } else {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    }
};
        locals.var_qsuld = assign30600_e44171;
        locals.var_qsuld_dn0 = assign30600_e44171_d_n0;
        locals.var_qsuld_dn2 = assign30600_e44171_d_n2;
        locals.var_qsuld_dn6 = assign30600_e44171_d_n6;
        locals.var_qsuld_dn7 = assign30600_e44171_d_n7;
        locals.var_qsuld_dn10 = assign30600_e44171_d_n10;
        locals.var_qsuld_dn11 = assign30600_e44171_d_n11;
        locals.var_qsuld_dn12 = assign30600_e44171_d_n12;
        locals.var_qsuld_dn17 = assign30600_e44171_d_n17;
        locals.var_qsuld_rv = 0.0;

        let (assign30610_e44182, assign30610_e44182_d_n0, assign30610_e44182_d_n2, assign30610_e44182_d_n6, assign30610_e44182_d_n7, assign30610_e44182_d_n10, assign30610_e44182_d_n11, assign30610_e44182_d_n12, assign30610_e44182_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 != 0.0)) {
        (locals.var_qsuld, locals.var_qsuld_dn0, locals.var_qsuld_dn2, locals.var_qsuld_dn6, locals.var_qsuld_dn7, locals.var_qsuld_dn10, locals.var_qsuld_dn11, locals.var_qsuld_dn12, locals.var_qsuld_dn17,)
    } else {
        (locals.var_qbuld, locals.var_qbuld_dn0, locals.var_qbuld_dn2, locals.var_qbuld_dn6, locals.var_qbuld_dn7, locals.var_qbuld_dn10, locals.var_qbuld_dn11, locals.var_qbuld_dn12, locals.var_qbuld_dn17,)
    }
};
        locals.var_qbuld = assign30610_e44182;
        locals.var_qbuld_dn0 = assign30610_e44182_d_n0;
        locals.var_qbuld_dn2 = assign30610_e44182_d_n2;
        locals.var_qbuld_dn6 = assign30610_e44182_d_n6;
        locals.var_qbuld_dn7 = assign30610_e44182_d_n7;
        locals.var_qbuld_dn10 = assign30610_e44182_d_n10;
        locals.var_qbuld_dn11 = assign30610_e44182_d_n11;
        locals.var_qbuld_dn12 = assign30610_e44182_d_n12;
        locals.var_qbuld_dn17 = assign30610_e44182_d_n17;
        locals.var_qbuld_rv = 0.0;

        let (assign30630_e44206, assign30630_e44206_d_n0, assign30630_e44206_d_n2, assign30630_e44206_d_n6, assign30630_e44206_d_n7, assign30630_e44206_d_n10, assign30630_e44206_d_n11, assign30630_e44206_d_n12, assign30630_e44206_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        (3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_chi__blk943, locals.var_chi__blk943_dn0, locals.var_chi__blk943_dn2, locals.var_chi__blk943_dn6, locals.var_chi__blk943_dn7, locals.var_chi__blk943_dn10, locals.var_chi__blk943_dn11, locals.var_chi__blk943_dn12, locals.var_chi__blk943_dn17,)
    }
};
        locals.var_chi__blk943 = assign30630_e44206;
        locals.var_chi__blk943_dn0 = assign30630_e44206_d_n0;
        locals.var_chi__blk943_dn2 = assign30630_e44206_d_n2;
        locals.var_chi__blk943_dn6 = assign30630_e44206_d_n6;
        locals.var_chi__blk943_dn7 = assign30630_e44206_d_n7;
        locals.var_chi__blk943_dn10 = assign30630_e44206_d_n10;
        locals.var_chi__blk943_dn11 = assign30630_e44206_d_n11;
        locals.var_chi__blk943_dn12 = assign30630_e44206_d_n12;
        locals.var_chi__blk943_dn17 = assign30630_e44206_d_n17;
        locals.var_chi__blk943_rv = 0.0;

        let (assign30640_e44222, assign30640_e44222_d_n0, assign30640_e44222_d_n2, assign30640_e44222_d_n6, assign30640_e44222_d_n7, assign30640_e44222_d_n10, assign30640_e44222_d_n11, assign30640_e44222_d_n12, assign30640_e44222_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        let assign30640_e44218: f64 = (locals.var_chi__blk943 / locals.var_beta);
        let assign30640_e44220: f64 = (assign30640_e44218 - locals.var_vxbgmtcl__blk921);
        (assign30640_e44220, ((locals.var_chi__blk943_dn0 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn0), ((locals.var_chi__blk943_dn2 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn2), ((locals.var_chi__blk943_dn6 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn6), ((locals.var_chi__blk943_dn7 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn7), ((((locals.var_chi__blk943_dn10 * locals.var_beta) - (locals.var_chi__blk943 * locals.var_beta_dn10)) / (locals.var_beta * locals.var_beta)) - locals.var_vxbgmtcl__blk921_dn10), ((locals.var_chi__blk943_dn11 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn11), ((locals.var_chi__blk943_dn12 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn12), ((locals.var_chi__blk943_dn17 / locals.var_beta) - locals.var_vxbgmtcl__blk921_dn17),)
    } else {
        (locals.var_ps0_inia__blk946, locals.var_ps0_inia__blk946_dn0, locals.var_ps0_inia__blk946_dn2, locals.var_ps0_inia__blk946_dn6, locals.var_ps0_inia__blk946_dn7, locals.var_ps0_inia__blk946_dn10, locals.var_ps0_inia__blk946_dn11, locals.var_ps0_inia__blk946_dn12, locals.var_ps0_inia__blk946_dn17,)
    }
};
        locals.var_ps0_inia__blk946 = assign30640_e44222;
        locals.var_ps0_inia__blk946_dn0 = assign30640_e44222_d_n0;
        locals.var_ps0_inia__blk946_dn2 = assign30640_e44222_d_n2;
        locals.var_ps0_inia__blk946_dn6 = assign30640_e44222_d_n6;
        locals.var_ps0_inia__blk946_dn7 = assign30640_e44222_d_n7;
        locals.var_ps0_inia__blk946_dn10 = assign30640_e44222_d_n10;
        locals.var_ps0_inia__blk946_dn11 = assign30640_e44222_d_n11;
        locals.var_ps0_inia__blk946_dn12 = assign30640_e44222_d_n12;
        locals.var_ps0_inia__blk946_dn17 = assign30640_e44222_d_n17;
        locals.var_ps0_inia__blk946_rv = 0.0;

        let (assign30650_e44236, assign30650_e44236_d_n0, assign30650_e44236_d_n2, assign30650_e44236_d_n6, assign30650_e44236_d_n7, assign30650_e44236_d_n10, assign30650_e44236_d_n11, assign30650_e44236_d_n12, assign30650_e44236_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        let assign30650_e44233: f64 = (-locals.var_chi__blk943);
        let assign30650_e44234: f64 = (assign30650_e44233).exp();
        (assign30650_e44234, (assign30650_e44234 * (-locals.var_chi__blk943_dn0)), (assign30650_e44234 * (-locals.var_chi__blk943_dn2)), (assign30650_e44234 * (-locals.var_chi__blk943_dn6)), (assign30650_e44234 * (-locals.var_chi__blk943_dn7)), (assign30650_e44234 * (-locals.var_chi__blk943_dn10)), (assign30650_e44234 * (-locals.var_chi__blk943_dn11)), (assign30650_e44234 * (-locals.var_chi__blk943_dn12)), (assign30650_e44234 * (-locals.var_chi__blk943_dn17)),)
    } else {
        (locals.var_ty__blk905, locals.var_ty__blk905_dn0, locals.var_ty__blk905_dn2, locals.var_ty__blk905_dn6, locals.var_ty__blk905_dn7, locals.var_ty__blk905_dn10, locals.var_ty__blk905_dn11, locals.var_ty__blk905_dn12, locals.var_ty__blk905_dn17,)
    }
};
        locals.var_ty__blk905 = assign30650_e44236;
        locals.var_ty__blk905_dn0 = assign30650_e44236_d_n0;
        locals.var_ty__blk905_dn2 = assign30650_e44236_d_n2;
        locals.var_ty__blk905_dn6 = assign30650_e44236_d_n6;
        locals.var_ty__blk905_dn7 = assign30650_e44236_d_n7;
        locals.var_ty__blk905_dn10 = assign30650_e44236_d_n10;
        locals.var_ty__blk905_dn11 = assign30650_e44236_d_n11;
        locals.var_ty__blk905_dn12 = assign30650_e44236_d_n12;
        locals.var_ty__blk905_dn17 = assign30650_e44236_d_n17;
        locals.var_ty__blk905_rv = 0.0;

        let (assign30660_e44264, assign30660_e44264_d_n0, assign30660_e44264_d_n2, assign30660_e44264_d_n6, assign30660_e44264_d_n7, assign30660_e44264_d_n10, assign30660_e44264_d_n11, assign30660_e44264_d_n12, assign30660_e44264_d_n17,) = {
    if ((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) {
        let assign30660_e44251: f64 = (locals.var_vgpld__blk931 + locals.var_vxbgmtcl__blk921);
        let assign30660_e44252: f64 = (locals.var_beta * assign30660_e44251);
        let assign30660_e44254: f64 = (assign30660_e44252 - 1.0);
        let assign30660_e44256: f64 = (assign30660_e44254 + locals.var_ty__blk905);
        let assign30660_e44257: f64 = (4.0 * assign30660_e44256);
        let assign30660_e44260: f64 = (locals.var_fac1p2__blk930 * locals.var_beta2);
        let assign30660_e44261: f64 = (assign30660_e44257 / assign30660_e44260);
        let assign30660_e44262: f64 = (1.0 + assign30660_e44261);
        (assign30660_e44262, ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn0 + locals.var_vxbgmtcl__blk921_dn0)) + locals.var_ty__blk905_dn0)) * assign30660_e44260) - (assign30660_e44257 * (locals.var_fac1p2__blk930_dn0 * locals.var_beta2))) / (assign30660_e44260 * assign30660_e44260)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn2 + locals.var_vxbgmtcl__blk921_dn2)) + locals.var_ty__blk905_dn2)) * assign30660_e44260) - (assign30660_e44257 * (locals.var_fac1p2__blk930_dn2 * locals.var_beta2))) / (assign30660_e44260 * assign30660_e44260)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn6 + locals.var_vxbgmtcl__blk921_dn6)) + locals.var_ty__blk905_dn6)) * assign30660_e44260) - (assign30660_e44257 * (locals.var_fac1p2__blk930_dn6 * locals.var_beta2))) / (assign30660_e44260 * assign30660_e44260)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn7 + locals.var_vxbgmtcl__blk921_dn7)) + locals.var_ty__blk905_dn7)) * assign30660_e44260) - (assign30660_e44257 * (locals.var_fac1p2__blk930_dn7 * locals.var_beta2))) / (assign30660_e44260 * assign30660_e44260)), ((((4.0 * (((locals.var_beta_dn10 * assign30660_e44251) + (locals.var_beta * (locals.var_vgpld__blk931_dn10 + locals.var_vxbgmtcl__blk921_dn10))) + locals.var_ty__blk905_dn10)) * assign30660_e44260) - (assign30660_e44257 * ((locals.var_fac1p2__blk930_dn10 * locals.var_beta2) + (locals.var_fac1p2__blk930 * locals.var_beta2_dn10)))) / (assign30660_e44260 * assign30660_e44260)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn11 + locals.var_vxbgmtcl__blk921_dn11)) + locals.var_ty__blk905_dn11)) * assign30660_e44260) - (assign30660_e44257 * (locals.var_fac1p2__blk930_dn11 * locals.var_beta2))) / (assign30660_e44260 * assign30660_e44260)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn12 + locals.var_vxbgmtcl__blk921_dn12)) + locals.var_ty__blk905_dn12)) * assign30660_e44260) - (assign30660_e44257 * (locals.var_fac1p2__blk930_dn12 * locals.var_beta2))) / (assign30660_e44260 * assign30660_e44260)), ((((4.0 * ((locals.var_beta * (locals.var_vgpld__blk931_dn17 + locals.var_vxbgmtcl__blk921_dn17)) + locals.var_ty__blk905_dn17)) * assign30660_e44260) - (assign30660_e44257 * (locals.var_fac1p2__blk930_dn17 * locals.var_beta2))) / (assign30660_e44260 * assign30660_e44260)),)
    } else {
        (locals.var_tx__blk904, locals.var_tx__blk904_dn0, locals.var_tx__blk904_dn2, locals.var_tx__blk904_dn6, locals.var_tx__blk904_dn7, locals.var_tx__blk904_dn10, locals.var_tx__blk904_dn11, locals.var_tx__blk904_dn12, locals.var_tx__blk904_dn17,)
    }
};
        locals.var_tx__blk904 = assign30660_e44264;
        locals.var_tx__blk904_dn0 = assign30660_e44264_d_n0;
        locals.var_tx__blk904_dn2 = assign30660_e44264_d_n2;
        locals.var_tx__blk904_dn6 = assign30660_e44264_d_n6;
        locals.var_tx__blk904_dn7 = assign30660_e44264_d_n7;
        locals.var_tx__blk904_dn10 = assign30660_e44264_d_n10;
        locals.var_tx__blk904_dn11 = assign30660_e44264_d_n11;
        locals.var_tx__blk904_dn12 = assign30660_e44264_d_n12;
        locals.var_tx__blk904_dn17 = assign30660_e44264_d_n17;
        locals.var_tx__blk904_rv = 0.0;

        let assign30670_e44268: f64 = (10.0 * 2.220446049250313e-16);
        let assign30670_e44269: f64 = if locals.var_tx__blk904 < assign30670_e44268 { 1.0 } else { 0.0 };
        locals.var_guard1004 = assign30670_e44269;
        locals.var_guard1004_rv = 0.0;

        let (assign30680_e44285, assign30680_e44285_d_n0, assign30680_e44285_d_n2, assign30680_e44285_d_n6, assign30680_e44285_d_n7, assign30680_e44285_d_n10, assign30680_e44285_d_n11, assign30680_e44285_d_n12, assign30680_e44285_d_n17,) = {
    if (((((p.p24 != 0.0) && (locals.var_guard978 != 0.0)) && (locals.var_guard979 == 0.0)) && (locals.var_guard1002 == 0.0)) && (locals.var_guard1004 != 0.0)) {
        let assign30680_e44283: f64 = (10.0 * 2.220446049250313e-16);
        (assign30680_e44283, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx__blk904, locals.var_tx__blk904_dn0, locals.var_tx__blk904_dn2, locals.var_tx__blk904_dn6, locals.var_tx__blk904_dn7, locals.var_tx__blk904_dn10, locals.var_tx__blk904_dn11, locals.var_tx__blk904_dn12, locals.var_tx__blk904_dn17,)
    }
};
        locals.var_tx__blk904 = assign30680_e44285;
        locals.var_tx__blk904_dn0 = assign30680_e44285_d_n0;
        locals.var_tx__blk904_dn2 = assign30680_e44285_d_n2;
        locals.var_tx__blk904_dn6 = assign30680_e44285_d_n6;
        locals.var_tx__blk904_dn7 = assign30680_e44285_d_n7;
        locals.var_tx__blk904_dn10 = assign30680_e44285_d_n10;
        locals.var_tx__blk904_dn11 = assign30680_e44285_d_n11;
        locals.var_tx__blk904_dn12 = assign30680_e44285_d_n12;
        locals.var_tx__blk904_dn17 = assign30680_e44285_d_n17;
        locals.var_tx__blk904_rv = 0.0;

    }
}
