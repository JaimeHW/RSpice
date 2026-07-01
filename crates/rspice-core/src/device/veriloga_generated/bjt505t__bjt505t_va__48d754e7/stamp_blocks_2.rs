#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3990_e3710, assign3990_e3710_d_n0, assign3990_e3710_d_n1, assign3990_e3710_d_n3, assign3990_e3710_d_n4, assign3990_e3710_d_n5, assign3990_e3710_d_n6, assign3990_e3710_d_n7, assign3990_e3710_d_n8, assign3990_e3710_d_n9, assign3990_e3710_d_n10, assign3990_e3710_d_n11,) = {
    if ((locals.var_guard63 != 0.0) && (locals.var_guard64 == 0.0)) {
        let assign3990_e3702: f64 = (locals.var_vb2e1 - locals.var_vknbr_t);
        let assign3990_e3704: f64 = (assign3990_e3702 * locals.var_vtinv);
        let assign3990_e3706: f64 = (assign3990_e3704 - p.p151);
        let assign3990_e3707: f64 = (1.0 + assign3990_e3706);
        let assign3990_e3708: f64 = (locals.var_expl * assign3990_e3707);
        (assign3990_e3708, (locals.var_expl * ((-locals.var_vknbr_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn1) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn3) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vknbr_t_dn4) * locals.var_vtinv) + (assign3990_e3702 * locals.var_vtinv_dn4))), (locals.var_expl * ((locals.var_vb2e1_dn5 - locals.var_vknbr_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb2e1_dn7 - locals.var_vknbr_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn9) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn10) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn11) * locals.var_vtinv)),)
    } else {
        (locals.var_tmpexp1, locals.var_tmpexp1_dn0, locals.var_tmpexp1_dn1, locals.var_tmpexp1_dn3, locals.var_tmpexp1_dn4, locals.var_tmpexp1_dn5, locals.var_tmpexp1_dn6, locals.var_tmpexp1_dn7, locals.var_tmpexp1_dn8, locals.var_tmpexp1_dn9, locals.var_tmpexp1_dn10, locals.var_tmpexp1_dn11,)
    }
};
        locals.var_tmpexp1 = assign3990_e3710;
        locals.var_tmpexp1_dn0 = assign3990_e3710_d_n0;
        locals.var_tmpexp1_dn1 = assign3990_e3710_d_n1;
        locals.var_tmpexp1_dn3 = assign3990_e3710_d_n3;
        locals.var_tmpexp1_dn4 = assign3990_e3710_d_n4;
        locals.var_tmpexp1_dn5 = assign3990_e3710_d_n5;
        locals.var_tmpexp1_dn6 = assign3990_e3710_d_n6;
        locals.var_tmpexp1_dn7 = assign3990_e3710_d_n7;
        locals.var_tmpexp1_dn8 = assign3990_e3710_d_n8;
        locals.var_tmpexp1_dn9 = assign3990_e3710_d_n9;
        locals.var_tmpexp1_dn10 = assign3990_e3710_d_n10;
        locals.var_tmpexp1_dn11 = assign3990_e3710_d_n11;
        locals.var_tmpexp1_rv = 0.0;

        let assign4000_e3713: f64 = (locals.var_in_ / locals.var_is_t);
        let assign4000_e3715: f64 = (assign4000_e3713 - 1000.0);
        let assign4000_e3717: f64 = if assign4000_e3715 < 40.0 { 1.0 } else { 0.0 };
        locals.var_guard65 = assign4000_e3717;
        locals.var_guard65_rv = 0.0;

        let (assign4020_e3736,) = {
    if ((locals.var_guard63 != 0.0) && (locals.var_guard65 == 0.0)) {
        let assign4020_e3734: f64 = (40.0_f64).exp();
        (assign4020_e3734,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4020_e3736;
        locals.var_expl_rv = 0.0;

        let assign4080_e3843: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4080_e3845: f64 = (assign4080_e3843 / p.p19);
        let assign4080_e3847: f64 = if assign4080_e3845 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard67 = assign4080_e3847;
        locals.var_guard67_rv = 0.0;

        let (assign4090_e3856, assign4090_e3856_d_n0, assign4090_e3856_d_n1, assign4090_e3856_d_n3, assign4090_e3856_d_n4, assign4090_e3856_d_n5, assign4090_e3856_d_n6, assign4090_e3856_d_n7, assign4090_e3856_d_n8, assign4090_e3856_d_n9, assign4090_e3856_d_n10, assign4090_e3856_d_n11,) = {
    if (locals.var_guard67 != 0.0) {
        let assign4090_e3851: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4090_e3853: f64 = (assign4090_e3851 / p.p19);
        let assign4090_e3854: f64 = (assign4090_e3853).exp();
        (assign4090_e3854, 0.0, 0.0, 0.0, (assign4090_e3854 * ((locals.var_vb1e1 * locals.var_vtinv_dn4) / p.p19)), (assign4090_e3854 * ((locals.var_vb1e1_dn5 * locals.var_vtinv) / p.p19)), (assign4090_e3854 * ((locals.var_vb1e1_dn6 * locals.var_vtinv) / p.p19)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4090_e3856;
        locals.var_tmpexp_dn0 = assign4090_e3856_d_n0;
        locals.var_tmpexp_dn1 = assign4090_e3856_d_n1;
        locals.var_tmpexp_dn3 = assign4090_e3856_d_n3;
        locals.var_tmpexp_dn4 = assign4090_e3856_d_n4;
        locals.var_tmpexp_dn5 = assign4090_e3856_d_n5;
        locals.var_tmpexp_dn6 = assign4090_e3856_d_n6;
        locals.var_tmpexp_dn7 = assign4090_e3856_d_n7;
        locals.var_tmpexp_dn8 = assign4090_e3856_d_n8;
        locals.var_tmpexp_dn9 = assign4090_e3856_d_n9;
        locals.var_tmpexp_dn10 = assign4090_e3856_d_n10;
        locals.var_tmpexp_dn11 = assign4090_e3856_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let (assign4100_e3862,) = {
    if (locals.var_guard67 == 0.0) {
        let assign4100_e3860: f64 = (p.p151).exp();
        (assign4100_e3860,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4100_e3862;
        locals.var_expl_rv = 0.0;

        let (assign4110_e3877, assign4110_e3877_d_n0, assign4110_e3877_d_n1, assign4110_e3877_d_n3, assign4110_e3877_d_n4, assign4110_e3877_d_n5, assign4110_e3877_d_n6, assign4110_e3877_d_n7, assign4110_e3877_d_n8, assign4110_e3877_d_n9, assign4110_e3877_d_n10, assign4110_e3877_d_n11,) = {
    if (locals.var_guard67 == 0.0) {
        let assign4110_e3869: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4110_e3871: f64 = (assign4110_e3869 / p.p19);
        let assign4110_e3873: f64 = (assign4110_e3871 - p.p151);
        let assign4110_e3874: f64 = (1.0 + assign4110_e3873);
        let assign4110_e3875: f64 = (locals.var_expl * assign4110_e3874);
        (assign4110_e3875, 0.0, 0.0, 0.0, (locals.var_expl * ((locals.var_vb1e1 * locals.var_vtinv_dn4) / p.p19)), (locals.var_expl * ((locals.var_vb1e1_dn5 * locals.var_vtinv) / p.p19)), (locals.var_expl * ((locals.var_vb1e1_dn6 * locals.var_vtinv) / p.p19)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4110_e3877;
        locals.var_tmpexp_dn0 = assign4110_e3877_d_n0;
        locals.var_tmpexp_dn1 = assign4110_e3877_d_n1;
        locals.var_tmpexp_dn3 = assign4110_e3877_d_n3;
        locals.var_tmpexp_dn4 = assign4110_e3877_d_n4;
        locals.var_tmpexp_dn5 = assign4110_e3877_d_n5;
        locals.var_tmpexp_dn6 = assign4110_e3877_d_n6;
        locals.var_tmpexp_dn7 = assign4110_e3877_d_n7;
        locals.var_tmpexp_dn8 = assign4110_e3877_d_n8;
        locals.var_tmpexp_dn9 = assign4110_e3877_d_n9;
        locals.var_tmpexp_dn10 = assign4110_e3877_d_n10;
        locals.var_tmpexp_dn11 = assign4110_e3877_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let assign4120_e3880: f64 = if p.p24 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard68 = assign4120_e3880;
        locals.var_guard68_rv = 0.0;

        let assign4130_e3883: f64 = (locals.var_vb1e1 - locals.var_vknbr_t);
        let assign4130_e3885: f64 = (assign4130_e3883 * locals.var_vtinv);
        let assign4130_e3887: f64 = if assign4130_e3885 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard69 = assign4130_e3887;
        locals.var_guard69_rv = 0.0;

        let (assign4140_e3898, assign4140_e3898_d_n0, assign4140_e3898_d_n1, assign4140_e3898_d_n3, assign4140_e3898_d_n4, assign4140_e3898_d_n5, assign4140_e3898_d_n6, assign4140_e3898_d_n7, assign4140_e3898_d_n8, assign4140_e3898_d_n9, assign4140_e3898_d_n10, assign4140_e3898_d_n11,) = {
    if ((locals.var_guard68 != 0.0) && (locals.var_guard69 != 0.0)) {
        let assign4140_e3893: f64 = (locals.var_vb1e1 - locals.var_vknbr_t);
        let assign4140_e3895: f64 = (assign4140_e3893 * locals.var_vtinv);
        let assign4140_e3896: f64 = (assign4140_e3895).exp();
        (assign4140_e3896, (assign4140_e3896 * ((-locals.var_vknbr_t_dn0) * locals.var_vtinv)), (assign4140_e3896 * ((-locals.var_vknbr_t_dn1) * locals.var_vtinv)), (assign4140_e3896 * ((-locals.var_vknbr_t_dn3) * locals.var_vtinv)), (assign4140_e3896 * (((-locals.var_vknbr_t_dn4) * locals.var_vtinv) + (assign4140_e3893 * locals.var_vtinv_dn4))), (assign4140_e3896 * ((locals.var_vb1e1_dn5 - locals.var_vknbr_t_dn5) * locals.var_vtinv)), (assign4140_e3896 * ((locals.var_vb1e1_dn6 - locals.var_vknbr_t_dn6) * locals.var_vtinv)), (assign4140_e3896 * ((-locals.var_vknbr_t_dn7) * locals.var_vtinv)), (assign4140_e3896 * ((-locals.var_vknbr_t_dn8) * locals.var_vtinv)), (assign4140_e3896 * ((-locals.var_vknbr_t_dn9) * locals.var_vtinv)), (assign4140_e3896 * ((-locals.var_vknbr_t_dn10) * locals.var_vtinv)), (assign4140_e3896 * ((-locals.var_vknbr_t_dn11) * locals.var_vtinv)),)
    } else {
        (locals.var_tmpexp1, locals.var_tmpexp1_dn0, locals.var_tmpexp1_dn1, locals.var_tmpexp1_dn3, locals.var_tmpexp1_dn4, locals.var_tmpexp1_dn5, locals.var_tmpexp1_dn6, locals.var_tmpexp1_dn7, locals.var_tmpexp1_dn8, locals.var_tmpexp1_dn9, locals.var_tmpexp1_dn10, locals.var_tmpexp1_dn11,)
    }
};
        locals.var_tmpexp1 = assign4140_e3898;
        locals.var_tmpexp1_dn0 = assign4140_e3898_d_n0;
        locals.var_tmpexp1_dn1 = assign4140_e3898_d_n1;
        locals.var_tmpexp1_dn3 = assign4140_e3898_d_n3;
        locals.var_tmpexp1_dn4 = assign4140_e3898_d_n4;
        locals.var_tmpexp1_dn5 = assign4140_e3898_d_n5;
        locals.var_tmpexp1_dn6 = assign4140_e3898_d_n6;
        locals.var_tmpexp1_dn7 = assign4140_e3898_d_n7;
        locals.var_tmpexp1_dn8 = assign4140_e3898_d_n8;
        locals.var_tmpexp1_dn9 = assign4140_e3898_d_n9;
        locals.var_tmpexp1_dn10 = assign4140_e3898_d_n10;
        locals.var_tmpexp1_dn11 = assign4140_e3898_d_n11;
        locals.var_tmpexp1_rv = 0.0;

        let (assign4150_e3906,) = {
    if ((locals.var_guard68 != 0.0) && (locals.var_guard69 == 0.0)) {
        let assign4150_e3904: f64 = (p.p151).exp();
        (assign4150_e3904,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4150_e3906;
        locals.var_expl_rv = 0.0;

        let (assign4160_e3923, assign4160_e3923_d_n0, assign4160_e3923_d_n1, assign4160_e3923_d_n3, assign4160_e3923_d_n4, assign4160_e3923_d_n5, assign4160_e3923_d_n6, assign4160_e3923_d_n7, assign4160_e3923_d_n8, assign4160_e3923_d_n9, assign4160_e3923_d_n10, assign4160_e3923_d_n11,) = {
    if ((locals.var_guard68 != 0.0) && (locals.var_guard69 == 0.0)) {
        let assign4160_e3915: f64 = (locals.var_vb1e1 - locals.var_vknbr_t);
        let assign4160_e3917: f64 = (assign4160_e3915 * locals.var_vtinv);
        let assign4160_e3919: f64 = (assign4160_e3917 - p.p151);
        let assign4160_e3920: f64 = (1.0 + assign4160_e3919);
        let assign4160_e3921: f64 = (locals.var_expl * assign4160_e3920);
        (assign4160_e3921, (locals.var_expl * ((-locals.var_vknbr_t_dn0) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn1) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn3) * locals.var_vtinv)), (locals.var_expl * (((-locals.var_vknbr_t_dn4) * locals.var_vtinv) + (assign4160_e3915 * locals.var_vtinv_dn4))), (locals.var_expl * ((locals.var_vb1e1_dn5 - locals.var_vknbr_t_dn5) * locals.var_vtinv)), (locals.var_expl * ((locals.var_vb1e1_dn6 - locals.var_vknbr_t_dn6) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn7) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn8) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn9) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn10) * locals.var_vtinv)), (locals.var_expl * ((-locals.var_vknbr_t_dn11) * locals.var_vtinv)),)
    } else {
        (locals.var_tmpexp1, locals.var_tmpexp1_dn0, locals.var_tmpexp1_dn1, locals.var_tmpexp1_dn3, locals.var_tmpexp1_dn4, locals.var_tmpexp1_dn5, locals.var_tmpexp1_dn6, locals.var_tmpexp1_dn7, locals.var_tmpexp1_dn8, locals.var_tmpexp1_dn9, locals.var_tmpexp1_dn10, locals.var_tmpexp1_dn11,)
    }
};
        locals.var_tmpexp1 = assign4160_e3923;
        locals.var_tmpexp1_dn0 = assign4160_e3923_d_n0;
        locals.var_tmpexp1_dn1 = assign4160_e3923_d_n1;
        locals.var_tmpexp1_dn3 = assign4160_e3923_d_n3;
        locals.var_tmpexp1_dn4 = assign4160_e3923_d_n4;
        locals.var_tmpexp1_dn5 = assign4160_e3923_d_n5;
        locals.var_tmpexp1_dn6 = assign4160_e3923_d_n6;
        locals.var_tmpexp1_dn7 = assign4160_e3923_d_n7;
        locals.var_tmpexp1_dn8 = assign4160_e3923_d_n8;
        locals.var_tmpexp1_dn9 = assign4160_e3923_d_n9;
        locals.var_tmpexp1_dn10 = assign4160_e3923_d_n10;
        locals.var_tmpexp1_dn11 = assign4160_e3923_d_n11;
        locals.var_tmpexp1_rv = 0.0;

        let assign4190_e3960: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign4190_e3962: f64 = (assign4190_e3960 / p.p21);
        let assign4190_e3964: f64 = if assign4190_e3962 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard70 = assign4190_e3964;
        locals.var_guard70_rv = 0.0;

        let (assign4200_e3973, assign4200_e3973_d_n0, assign4200_e3973_d_n1, assign4200_e3973_d_n3, assign4200_e3973_d_n4, assign4200_e3973_d_n5, assign4200_e3973_d_n6, assign4200_e3973_d_n7, assign4200_e3973_d_n8, assign4200_e3973_d_n9, assign4200_e3973_d_n10, assign4200_e3973_d_n11,) = {
    if (locals.var_guard70 != 0.0) {
        let assign4200_e3968: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign4200_e3970: f64 = (assign4200_e3968 / p.p21);
        let assign4200_e3971: f64 = (assign4200_e3970).exp();
        (assign4200_e3971, 0.0, 0.0, 0.0, (assign4200_e3971 * ((locals.var_vb2e1 * locals.var_vtinv_dn4) / p.p21)), (assign4200_e3971 * ((locals.var_vb2e1_dn5 * locals.var_vtinv) / p.p21)), 0.0, (assign4200_e3971 * ((locals.var_vb2e1_dn7 * locals.var_vtinv) / p.p21)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4200_e3973;
        locals.var_tmpexp_dn0 = assign4200_e3973_d_n0;
        locals.var_tmpexp_dn1 = assign4200_e3973_d_n1;
        locals.var_tmpexp_dn3 = assign4200_e3973_d_n3;
        locals.var_tmpexp_dn4 = assign4200_e3973_d_n4;
        locals.var_tmpexp_dn5 = assign4200_e3973_d_n5;
        locals.var_tmpexp_dn6 = assign4200_e3973_d_n6;
        locals.var_tmpexp_dn7 = assign4200_e3973_d_n7;
        locals.var_tmpexp_dn8 = assign4200_e3973_d_n8;
        locals.var_tmpexp_dn9 = assign4200_e3973_d_n9;
        locals.var_tmpexp_dn10 = assign4200_e3973_d_n10;
        locals.var_tmpexp_dn11 = assign4200_e3973_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let (assign4210_e3979,) = {
    if (locals.var_guard70 == 0.0) {
        let assign4210_e3977: f64 = (p.p151).exp();
        (assign4210_e3977,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4210_e3979;
        locals.var_expl_rv = 0.0;

        let (assign4220_e3994, assign4220_e3994_d_n0, assign4220_e3994_d_n1, assign4220_e3994_d_n3, assign4220_e3994_d_n4, assign4220_e3994_d_n5, assign4220_e3994_d_n6, assign4220_e3994_d_n7, assign4220_e3994_d_n8, assign4220_e3994_d_n9, assign4220_e3994_d_n10, assign4220_e3994_d_n11,) = {
    if (locals.var_guard70 == 0.0) {
        let assign4220_e3986: f64 = (locals.var_vb2e1 * locals.var_vtinv);
        let assign4220_e3988: f64 = (assign4220_e3986 / p.p21);
        let assign4220_e3990: f64 = (assign4220_e3988 - p.p151);
        let assign4220_e3991: f64 = (1.0 + assign4220_e3990);
        let assign4220_e3992: f64 = (locals.var_expl * assign4220_e3991);
        (assign4220_e3992, 0.0, 0.0, 0.0, (locals.var_expl * ((locals.var_vb2e1 * locals.var_vtinv_dn4) / p.p21)), (locals.var_expl * ((locals.var_vb2e1_dn5 * locals.var_vtinv) / p.p21)), 0.0, (locals.var_expl * ((locals.var_vb2e1_dn7 * locals.var_vtinv) / p.p21)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4220_e3994;
        locals.var_tmpexp_dn0 = assign4220_e3994_d_n0;
        locals.var_tmpexp_dn1 = assign4220_e3994_d_n1;
        locals.var_tmpexp_dn3 = assign4220_e3994_d_n3;
        locals.var_tmpexp_dn4 = assign4220_e3994_d_n4;
        locals.var_tmpexp_dn5 = assign4220_e3994_d_n5;
        locals.var_tmpexp_dn6 = assign4220_e3994_d_n6;
        locals.var_tmpexp_dn7 = assign4220_e3994_d_n7;
        locals.var_tmpexp_dn8 = assign4220_e3994_d_n8;
        locals.var_tmpexp_dn9 = assign4220_e3994_d_n9;
        locals.var_tmpexp_dn10 = assign4220_e3994_d_n10;
        locals.var_tmpexp_dn11 = assign4220_e3994_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let assign4240_e4002: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4240_e4004: f64 = (assign4240_e4002 / p.p23);
        let assign4240_e4006: f64 = if assign4240_e4004 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard71 = assign4240_e4006;
        locals.var_guard71_rv = 0.0;

        let (assign4250_e4015, assign4250_e4015_d_n0, assign4250_e4015_d_n1, assign4250_e4015_d_n3, assign4250_e4015_d_n4, assign4250_e4015_d_n5, assign4250_e4015_d_n6, assign4250_e4015_d_n7, assign4250_e4015_d_n8, assign4250_e4015_d_n9, assign4250_e4015_d_n10, assign4250_e4015_d_n11,) = {
    if (locals.var_guard71 != 0.0) {
        let assign4250_e4010: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4250_e4012: f64 = (assign4250_e4010 / p.p23);
        let assign4250_e4013: f64 = (assign4250_e4012).exp();
        (assign4250_e4013, 0.0, 0.0, 0.0, (assign4250_e4013 * ((locals.var_vb1e1 * locals.var_vtinv_dn4) / p.p23)), (assign4250_e4013 * ((locals.var_vb1e1_dn5 * locals.var_vtinv) / p.p23)), (assign4250_e4013 * ((locals.var_vb1e1_dn6 * locals.var_vtinv) / p.p23)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4250_e4015;
        locals.var_tmpexp_dn0 = assign4250_e4015_d_n0;
        locals.var_tmpexp_dn1 = assign4250_e4015_d_n1;
        locals.var_tmpexp_dn3 = assign4250_e4015_d_n3;
        locals.var_tmpexp_dn4 = assign4250_e4015_d_n4;
        locals.var_tmpexp_dn5 = assign4250_e4015_d_n5;
        locals.var_tmpexp_dn6 = assign4250_e4015_d_n6;
        locals.var_tmpexp_dn7 = assign4250_e4015_d_n7;
        locals.var_tmpexp_dn8 = assign4250_e4015_d_n8;
        locals.var_tmpexp_dn9 = assign4250_e4015_d_n9;
        locals.var_tmpexp_dn10 = assign4250_e4015_d_n10;
        locals.var_tmpexp_dn11 = assign4250_e4015_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let (assign4260_e4021,) = {
    if (locals.var_guard71 == 0.0) {
        let assign4260_e4019: f64 = (p.p151).exp();
        (assign4260_e4019,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4260_e4021;
        locals.var_expl_rv = 0.0;

        let (assign4270_e4036, assign4270_e4036_d_n0, assign4270_e4036_d_n1, assign4270_e4036_d_n3, assign4270_e4036_d_n4, assign4270_e4036_d_n5, assign4270_e4036_d_n6, assign4270_e4036_d_n7, assign4270_e4036_d_n8, assign4270_e4036_d_n9, assign4270_e4036_d_n10, assign4270_e4036_d_n11,) = {
    if (locals.var_guard71 == 0.0) {
        let assign4270_e4028: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4270_e4030: f64 = (assign4270_e4028 / p.p23);
        let assign4270_e4032: f64 = (assign4270_e4030 - p.p151);
        let assign4270_e4033: f64 = (1.0 + assign4270_e4032);
        let assign4270_e4034: f64 = (locals.var_expl * assign4270_e4033);
        (assign4270_e4034, 0.0, 0.0, 0.0, (locals.var_expl * ((locals.var_vb1e1 * locals.var_vtinv_dn4) / p.p23)), (locals.var_expl * ((locals.var_vb1e1_dn5 * locals.var_vtinv) / p.p23)), (locals.var_expl * ((locals.var_vb1e1_dn6 * locals.var_vtinv) / p.p23)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4270_e4036;
        locals.var_tmpexp_dn0 = assign4270_e4036_d_n0;
        locals.var_tmpexp_dn1 = assign4270_e4036_d_n1;
        locals.var_tmpexp_dn3 = assign4270_e4036_d_n3;
        locals.var_tmpexp_dn4 = assign4270_e4036_d_n4;
        locals.var_tmpexp_dn5 = assign4270_e4036_d_n5;
        locals.var_tmpexp_dn6 = assign4270_e4036_d_n6;
        locals.var_tmpexp_dn7 = assign4270_e4036_d_n7;
        locals.var_tmpexp_dn8 = assign4270_e4036_d_n8;
        locals.var_tmpexp_dn9 = assign4270_e4036_d_n9;
        locals.var_tmpexp_dn10 = assign4270_e4036_d_n10;
        locals.var_tmpexp_dn11 = assign4270_e4036_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let assign4290_e4044: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign4290_e4046: f64 = (assign4290_e4044 / p.p32);
        let assign4290_e4048: f64 = if assign4290_e4046 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard72 = assign4290_e4048;
        locals.var_guard72_rv = 0.0;

        let (assign4300_e4057, assign4300_e4057_d_n0, assign4300_e4057_d_n1, assign4300_e4057_d_n3, assign4300_e4057_d_n4, assign4300_e4057_d_n5, assign4300_e4057_d_n6, assign4300_e4057_d_n7, assign4300_e4057_d_n8, assign4300_e4057_d_n9, assign4300_e4057_d_n10, assign4300_e4057_d_n11,) = {
    if (locals.var_guard72 != 0.0) {
        let assign4300_e4052: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign4300_e4054: f64 = (assign4300_e4052 / p.p32);
        let assign4300_e4055: f64 = (assign4300_e4054).exp();
        (assign4300_e4055, 0.0, 0.0, 0.0, (assign4300_e4055 * ((locals.var_vb1c4 * locals.var_vtinv_dn4) / p.p32)), 0.0, (assign4300_e4055 * ((locals.var_vb1c4_dn6 * locals.var_vtinv) / p.p32)), (assign4300_e4055 * ((locals.var_vb1c4_dn7 * locals.var_vtinv) / p.p32)), (assign4300_e4055 * ((locals.var_vb1c4_dn8 * locals.var_vtinv) / p.p32)), (assign4300_e4055 * ((locals.var_vb1c4_dn9 * locals.var_vtinv) / p.p32)), 0.0, (assign4300_e4055 * ((locals.var_vb1c4_dn11 * locals.var_vtinv) / p.p32)),)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4300_e4057;
        locals.var_tmpexp_dn0 = assign4300_e4057_d_n0;
        locals.var_tmpexp_dn1 = assign4300_e4057_d_n1;
        locals.var_tmpexp_dn3 = assign4300_e4057_d_n3;
        locals.var_tmpexp_dn4 = assign4300_e4057_d_n4;
        locals.var_tmpexp_dn5 = assign4300_e4057_d_n5;
        locals.var_tmpexp_dn6 = assign4300_e4057_d_n6;
        locals.var_tmpexp_dn7 = assign4300_e4057_d_n7;
        locals.var_tmpexp_dn8 = assign4300_e4057_d_n8;
        locals.var_tmpexp_dn9 = assign4300_e4057_d_n9;
        locals.var_tmpexp_dn10 = assign4300_e4057_d_n10;
        locals.var_tmpexp_dn11 = assign4300_e4057_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let (assign4310_e4063,) = {
    if (locals.var_guard72 == 0.0) {
        let assign4310_e4061: f64 = (p.p151).exp();
        (assign4310_e4061,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4310_e4063;
        locals.var_expl_rv = 0.0;

        let (assign4320_e4078, assign4320_e4078_d_n0, assign4320_e4078_d_n1, assign4320_e4078_d_n3, assign4320_e4078_d_n4, assign4320_e4078_d_n5, assign4320_e4078_d_n6, assign4320_e4078_d_n7, assign4320_e4078_d_n8, assign4320_e4078_d_n9, assign4320_e4078_d_n10, assign4320_e4078_d_n11,) = {
    if (locals.var_guard72 == 0.0) {
        let assign4320_e4070: f64 = (locals.var_vb1c4 * locals.var_vtinv);
        let assign4320_e4072: f64 = (assign4320_e4070 / p.p32);
        let assign4320_e4074: f64 = (assign4320_e4072 - p.p151);
        let assign4320_e4075: f64 = (1.0 + assign4320_e4074);
        let assign4320_e4076: f64 = (locals.var_expl * assign4320_e4075);
        (assign4320_e4076, 0.0, 0.0, 0.0, (locals.var_expl * ((locals.var_vb1c4 * locals.var_vtinv_dn4) / p.p32)), 0.0, (locals.var_expl * ((locals.var_vb1c4_dn6 * locals.var_vtinv) / p.p32)), (locals.var_expl * ((locals.var_vb1c4_dn7 * locals.var_vtinv) / p.p32)), (locals.var_expl * ((locals.var_vb1c4_dn8 * locals.var_vtinv) / p.p32)), (locals.var_expl * ((locals.var_vb1c4_dn9 * locals.var_vtinv) / p.p32)), 0.0, (locals.var_expl * ((locals.var_vb1c4_dn11 * locals.var_vtinv) / p.p32)),)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4320_e4078;
        locals.var_tmpexp_dn0 = assign4320_e4078_d_n0;
        locals.var_tmpexp_dn1 = assign4320_e4078_d_n1;
        locals.var_tmpexp_dn3 = assign4320_e4078_d_n3;
        locals.var_tmpexp_dn4 = assign4320_e4078_d_n4;
        locals.var_tmpexp_dn5 = assign4320_e4078_d_n5;
        locals.var_tmpexp_dn6 = assign4320_e4078_d_n6;
        locals.var_tmpexp_dn7 = assign4320_e4078_d_n7;
        locals.var_tmpexp_dn8 = assign4320_e4078_d_n8;
        locals.var_tmpexp_dn9 = assign4320_e4078_d_n9;
        locals.var_tmpexp_dn10 = assign4320_e4078_d_n10;
        locals.var_tmpexp_dn11 = assign4320_e4078_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let assign4340_e4086: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4340_e4088: f64 = (assign4340_e4086 / p.p150);
        let assign4340_e4090: f64 = if assign4340_e4088 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard73 = assign4340_e4090;
        locals.var_guard73_rv = 0.0;

        let (assign4350_e4099, assign4350_e4099_d_n0, assign4350_e4099_d_n1, assign4350_e4099_d_n3, assign4350_e4099_d_n4, assign4350_e4099_d_n5, assign4350_e4099_d_n6, assign4350_e4099_d_n7, assign4350_e4099_d_n8, assign4350_e4099_d_n9, assign4350_e4099_d_n10, assign4350_e4099_d_n11,) = {
    if (locals.var_guard73 != 0.0) {
        let assign4350_e4094: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4350_e4096: f64 = (assign4350_e4094 / p.p150);
        let assign4350_e4097: f64 = (assign4350_e4096).exp();
        (assign4350_e4097, 0.0, 0.0, 0.0, (assign4350_e4097 * ((locals.var_vb1e1 * locals.var_vtinv_dn4) / p.p150)), (assign4350_e4097 * ((locals.var_vb1e1_dn5 * locals.var_vtinv) / p.p150)), (assign4350_e4097 * ((locals.var_vb1e1_dn6 * locals.var_vtinv) / p.p150)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4350_e4099;
        locals.var_tmpexp_dn0 = assign4350_e4099_d_n0;
        locals.var_tmpexp_dn1 = assign4350_e4099_d_n1;
        locals.var_tmpexp_dn3 = assign4350_e4099_d_n3;
        locals.var_tmpexp_dn4 = assign4350_e4099_d_n4;
        locals.var_tmpexp_dn5 = assign4350_e4099_d_n5;
        locals.var_tmpexp_dn6 = assign4350_e4099_d_n6;
        locals.var_tmpexp_dn7 = assign4350_e4099_d_n7;
        locals.var_tmpexp_dn8 = assign4350_e4099_d_n8;
        locals.var_tmpexp_dn9 = assign4350_e4099_d_n9;
        locals.var_tmpexp_dn10 = assign4350_e4099_d_n10;
        locals.var_tmpexp_dn11 = assign4350_e4099_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let (assign4360_e4105,) = {
    if (locals.var_guard73 == 0.0) {
        let assign4360_e4103: f64 = (p.p151).exp();
        (assign4360_e4103,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4360_e4105;
        locals.var_expl_rv = 0.0;

        let (assign4370_e4120, assign4370_e4120_d_n0, assign4370_e4120_d_n1, assign4370_e4120_d_n3, assign4370_e4120_d_n4, assign4370_e4120_d_n5, assign4370_e4120_d_n6, assign4370_e4120_d_n7, assign4370_e4120_d_n8, assign4370_e4120_d_n9, assign4370_e4120_d_n10, assign4370_e4120_d_n11,) = {
    if (locals.var_guard73 == 0.0) {
        let assign4370_e4112: f64 = (locals.var_vb1e1 * locals.var_vtinv);
        let assign4370_e4114: f64 = (assign4370_e4112 / p.p150);
        let assign4370_e4116: f64 = (assign4370_e4114 - p.p151);
        let assign4370_e4117: f64 = (1.0 + assign4370_e4116);
        let assign4370_e4118: f64 = (locals.var_expl * assign4370_e4117);
        (assign4370_e4118, 0.0, 0.0, 0.0, (locals.var_expl * ((locals.var_vb1e1 * locals.var_vtinv_dn4) / p.p150)), (locals.var_expl * ((locals.var_vb1e1_dn5 * locals.var_vtinv) / p.p150)), (locals.var_expl * ((locals.var_vb1e1_dn6 * locals.var_vtinv) / p.p150)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpexp, locals.var_tmpexp_dn0, locals.var_tmpexp_dn1, locals.var_tmpexp_dn3, locals.var_tmpexp_dn4, locals.var_tmpexp_dn5, locals.var_tmpexp_dn6, locals.var_tmpexp_dn7, locals.var_tmpexp_dn8, locals.var_tmpexp_dn9, locals.var_tmpexp_dn10, locals.var_tmpexp_dn11,)
    }
};
        locals.var_tmpexp = assign4370_e4120;
        locals.var_tmpexp_dn0 = assign4370_e4120_d_n0;
        locals.var_tmpexp_dn1 = assign4370_e4120_d_n1;
        locals.var_tmpexp_dn3 = assign4370_e4120_d_n3;
        locals.var_tmpexp_dn4 = assign4370_e4120_d_n4;
        locals.var_tmpexp_dn5 = assign4370_e4120_d_n5;
        locals.var_tmpexp_dn6 = assign4370_e4120_d_n6;
        locals.var_tmpexp_dn7 = assign4370_e4120_d_n7;
        locals.var_tmpexp_dn8 = assign4370_e4120_d_n8;
        locals.var_tmpexp_dn9 = assign4370_e4120_d_n9;
        locals.var_tmpexp_dn10 = assign4370_e4120_d_n10;
        locals.var_tmpexp_dn11 = assign4370_e4120_d_n11;
        locals.var_tmpexp_rv = 0.0;

        let assign4390_e4136: f64 = if (((p.p34 > 0.0) && (p.p35 > 0.0)) && (locals.var_vb2e1 < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard74 = assign4390_e4136;
        locals.var_guard74_rv = 0.0;

        let assign4400_e4142: f64 = (2.0 * locals.var_e0eb);
        let assign4400_e4143: f64 = (locals.var_pow2_2m_pe / assign4400_e4142);
        let assign4400_e4144: f64 = (1.0 - assign4400_e4143);
        let assign4400_e4145: f64 = (locals.var_nzeb_t * assign4400_e4144);
        let assign4400_e4147: f64 = if assign4400_e4145 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard75 = assign4400_e4147;
        locals.var_guard75_rv = 0.0;

        let (assign4420_e4170,) = {
    if ((locals.var_guard74 != 0.0) && (locals.var_guard75 == 0.0)) {
        let assign4420_e4168: f64 = (p.p151).exp();
        (assign4420_e4168,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4420_e4170;
        locals.var_expl_rv = 0.0;

        let (assign4440_e4197, assign4440_e4197_d_n0, assign4440_e4197_d_n1, assign4440_e4197_d_n3, assign4440_e4197_d_n4, assign4440_e4197_d_n5, assign4440_e4197_d_n6, assign4440_e4197_d_n7, assign4440_e4197_d_n8, assign4440_e4197_d_n9, assign4440_e4197_d_n10, assign4440_e4197_d_n11,) = {
    if (locals.var_guard74 != 0.0) {
        let assign4440_e4195: f64 = (locals.var_vb2e1 * locals.var_inv_vde_t);
        (assign4440_e4195, (locals.var_vb2e1 * locals.var_inv_vde_t_dn0), (locals.var_vb2e1 * locals.var_inv_vde_t_dn1), (locals.var_vb2e1 * locals.var_inv_vde_t_dn3), (locals.var_vb2e1 * locals.var_inv_vde_t_dn4), ((locals.var_vb2e1_dn5 * locals.var_inv_vde_t) + (locals.var_vb2e1 * locals.var_inv_vde_t_dn5)), (locals.var_vb2e1 * locals.var_inv_vde_t_dn6), ((locals.var_vb2e1_dn7 * locals.var_inv_vde_t) + (locals.var_vb2e1 * locals.var_inv_vde_t_dn7)), (locals.var_vb2e1 * locals.var_inv_vde_t_dn8), (locals.var_vb2e1 * locals.var_inv_vde_t_dn9), (locals.var_vb2e1 * locals.var_inv_vde_t_dn10), (locals.var_vb2e1 * locals.var_inv_vde_t_dn11),)
    } else {
        (locals.var_x, locals.var_x_dn0, locals.var_x_dn1, locals.var_x_dn3, locals.var_x_dn4, locals.var_x_dn5, locals.var_x_dn6, locals.var_x_dn7, locals.var_x_dn8, locals.var_x_dn9, locals.var_x_dn10, locals.var_x_dn11,)
    }
};
        locals.var_x = assign4440_e4197;
        locals.var_x_dn0 = assign4440_e4197_d_n0;
        locals.var_x_dn1 = assign4440_e4197_d_n1;
        locals.var_x_dn3 = assign4440_e4197_d_n3;
        locals.var_x_dn4 = assign4440_e4197_d_n4;
        locals.var_x_dn5 = assign4440_e4197_d_n5;
        locals.var_x_dn6 = assign4440_e4197_d_n6;
        locals.var_x_dn7 = assign4440_e4197_d_n7;
        locals.var_x_dn8 = assign4440_e4197_d_n8;
        locals.var_x_dn9 = assign4440_e4197_d_n9;
        locals.var_x_dn10 = assign4440_e4197_d_n10;
        locals.var_x_dn11 = assign4440_e4197_d_n11;
        locals.var_x_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4450_e4241, assign4450_e4241_d_n0, assign4450_e4241_d_n1, assign4450_e4241_d_n3, assign4450_e4241_d_n4, assign4450_e4241_d_n5, assign4450_e4241_d_n6, assign4450_e4241_d_n7, assign4450_e4241_d_n8, assign4450_e4241_d_n9, assign4450_e4241_d_n10, assign4450_e4241_d_n11,) = {
    if (locals.var_guard74 != 0.0) {
        let assign4450_e4201: f64 = (locals.var_x * locals.var_x);
        let assign4450_e4203: f64 = (assign4450_e4201 + 1e-30);
        let assign4450_e4204: f64 = (assign4450_e4203).sqrt();
        let assign4450_e4206: f64 = (-2.0);
        let assign4450_e4208: f64 = (assign4450_e4206 - p.p67);
        let assign4450_e4209: f64 = (assign4450_e4204).powf(assign4450_e4208);
        let assign4450_e4214: f64 = (p.p67 * p.p67);
        let assign4450_e4215: f64 = (1.0 - assign4450_e4214);
        let assign4450_e4218: f64 = (3.0 * locals.var_x);
        let assign4450_e4221: f64 = (p.p67 - 1.0);
        let assign4450_e4222: f64 = (assign4450_e4218 * assign4450_e4221);
        let assign4450_e4223: f64 = (assign4450_e4215 - assign4450_e4222);
        let assign4450_e4224: f64 = (p.p67 * assign4450_e4223);
        let assign4450_e4227: f64 = (6.0 * locals.var_x);
        let assign4450_e4229: f64 = (assign4450_e4227 * locals.var_x);
        let assign4450_e4232: f64 = (p.p67 - 1.0);
        let assign4450_e4234: f64 = (assign4450_e4232 + locals.var_x);
        let assign4450_e4235: f64 = (assign4450_e4229 * assign4450_e4234);
        let assign4450_e4236: f64 = (assign4450_e4224 - assign4450_e4235);
        let assign4450_e4237: f64 = (assign4450_e4209 * assign4450_e4236);
        let assign4450_e4239: f64 = (assign4450_e4237 * 0.16666666666666666);
        (assign4450_e4239, (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn0 * locals.var_x) + (locals.var_x * locals.var_x_dn0)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn0 * locals.var_x) + (locals.var_x * locals.var_x_dn0)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn0) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn0) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn0)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn0))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn1 * locals.var_x) + (locals.var_x * locals.var_x_dn1)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn1 * locals.var_x) + (locals.var_x * locals.var_x_dn1)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn1) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn1) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn1)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn1))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn3 * locals.var_x) + (locals.var_x * locals.var_x_dn3)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn3 * locals.var_x) + (locals.var_x * locals.var_x_dn3)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn3) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn3) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn3)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn3))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn4 * locals.var_x) + (locals.var_x * locals.var_x_dn4)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn4 * locals.var_x) + (locals.var_x * locals.var_x_dn4)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn4) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn4) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn4)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn4))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn5 * locals.var_x) + (locals.var_x * locals.var_x_dn5)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn5 * locals.var_x) + (locals.var_x * locals.var_x_dn5)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn5) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn5) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn5)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn5))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn6 * locals.var_x) + (locals.var_x * locals.var_x_dn6)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn6 * locals.var_x) + (locals.var_x * locals.var_x_dn6)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn6) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn6) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn6)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn6))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn7 * locals.var_x) + (locals.var_x * locals.var_x_dn7)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn7 * locals.var_x) + (locals.var_x * locals.var_x_dn7)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn7) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn7) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn7)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn7))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn8 * locals.var_x) + (locals.var_x * locals.var_x_dn8)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn8 * locals.var_x) + (locals.var_x * locals.var_x_dn8)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn8) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn8) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn8)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn8))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn9 * locals.var_x) + (locals.var_x * locals.var_x_dn9)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn9 * locals.var_x) + (locals.var_x * locals.var_x_dn9)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn9) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn9) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn9)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn9))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn10 * locals.var_x) + (locals.var_x * locals.var_x_dn10)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn10 * locals.var_x) + (locals.var_x * locals.var_x_dn10)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn10) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn10) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn10)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn10))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4450_e4208) as f64).is_finite() && ((assign4450_e4208) as f64).fract() == 0.0 { if assign4450_e4208 == 0.0 { 0.0 } else { (assign4450_e4208 * ((assign4450_e4204).powf(assign4450_e4208 - 1.0) * (((locals.var_x_dn11 * locals.var_x) + (locals.var_x * locals.var_x_dn11)) / (2.0 * assign4450_e4204)))) } } else { (assign4450_e4209 * (assign4450_e4208 * ((((locals.var_x_dn11 * locals.var_x) + (locals.var_x * locals.var_x_dn11)) / (2.0 * assign4450_e4204)) / assign4450_e4204))) } * assign4450_e4236) + (assign4450_e4209 * ((p.p67 * (-((3.0 * locals.var_x_dn11) * assign4450_e4221))) - (((((6.0 * locals.var_x_dn11) * locals.var_x) + (assign4450_e4227 * locals.var_x_dn11)) * assign4450_e4234) + (assign4450_e4229 * locals.var_x_dn11))))) * 0.16666666666666666),)
    } else {
        (locals.var_de0eb, locals.var_de0eb_dn0, locals.var_de0eb_dn1, locals.var_de0eb_dn3, locals.var_de0eb_dn4, locals.var_de0eb_dn5, locals.var_de0eb_dn6, locals.var_de0eb_dn7, locals.var_de0eb_dn8, locals.var_de0eb_dn9, locals.var_de0eb_dn10, locals.var_de0eb_dn11,)
    }
};
        locals.var_de0eb = assign4450_e4241;
        locals.var_de0eb_dn0 = assign4450_e4241_d_n0;
        locals.var_de0eb_dn1 = assign4450_e4241_d_n1;
        locals.var_de0eb_dn3 = assign4450_e4241_d_n3;
        locals.var_de0eb_dn4 = assign4450_e4241_d_n4;
        locals.var_de0eb_dn5 = assign4450_e4241_d_n5;
        locals.var_de0eb_dn6 = assign4450_e4241_d_n6;
        locals.var_de0eb_dn7 = assign4450_e4241_d_n7;
        locals.var_de0eb_dn8 = assign4450_e4241_d_n8;
        locals.var_de0eb_dn9 = assign4450_e4241_d_n9;
        locals.var_de0eb_dn10 = assign4450_e4241_d_n10;
        locals.var_de0eb_dn11 = assign4450_e4241_d_n11;
        locals.var_de0eb_rv = 0.0;

        let (assign4460_e4253, assign4460_e4253_d_n0, assign4460_e4253_d_n1, assign4460_e4253_d_n3, assign4460_e4253_d_n4, assign4460_e4253_d_n5, assign4460_e4253_d_n6, assign4460_e4253_d_n7, assign4460_e4253_d_n8, assign4460_e4253_d_n9, assign4460_e4253_d_n10, assign4460_e4253_d_n11,) = {
    if (locals.var_guard74 != 0.0) {
        let assign4460_e4245: f64 = (locals.var_vb2e1 * locals.var_pow2_2m_pe);
        let assign4460_e4247: f64 = (assign4460_e4245 * locals.var_nzeb_t);
        let assign4460_e4250: f64 = (locals.var_vgzeb_t * locals.var_de0eb);
        let assign4460_e4251: f64 = (assign4460_e4247 / assign4460_e4250);
        (assign4460_e4251, ((((assign4460_e4245 * locals.var_nzeb_t_dn0) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn0 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn0)))) / (assign4460_e4250 * assign4460_e4250)), ((((assign4460_e4245 * locals.var_nzeb_t_dn1) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn1 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn1)))) / (assign4460_e4250 * assign4460_e4250)), ((((assign4460_e4245 * locals.var_nzeb_t_dn3) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn3 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn3)))) / (assign4460_e4250 * assign4460_e4250)), ((((assign4460_e4245 * locals.var_nzeb_t_dn4) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn4 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn4)))) / (assign4460_e4250 * assign4460_e4250)), ((((((locals.var_vb2e1_dn5 * locals.var_pow2_2m_pe) * locals.var_nzeb_t) + (assign4460_e4245 * locals.var_nzeb_t_dn5)) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn5 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn5)))) / (assign4460_e4250 * assign4460_e4250)), ((((assign4460_e4245 * locals.var_nzeb_t_dn6) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn6 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn6)))) / (assign4460_e4250 * assign4460_e4250)), ((((((locals.var_vb2e1_dn7 * locals.var_pow2_2m_pe) * locals.var_nzeb_t) + (assign4460_e4245 * locals.var_nzeb_t_dn7)) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn7 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn7)))) / (assign4460_e4250 * assign4460_e4250)), ((((assign4460_e4245 * locals.var_nzeb_t_dn8) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn8 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn8)))) / (assign4460_e4250 * assign4460_e4250)), ((((assign4460_e4245 * locals.var_nzeb_t_dn9) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn9 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn9)))) / (assign4460_e4250 * assign4460_e4250)), ((((assign4460_e4245 * locals.var_nzeb_t_dn10) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn10 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn10)))) / (assign4460_e4250 * assign4460_e4250)), ((((assign4460_e4245 * locals.var_nzeb_t_dn11) * assign4460_e4250) - (assign4460_e4247 * ((locals.var_vgzeb_t_dn11 * locals.var_de0eb) + (locals.var_vgzeb_t * locals.var_de0eb_dn11)))) / (assign4460_e4250 * assign4460_e4250)),)
    } else {
        (locals.var_x, locals.var_x_dn0, locals.var_x_dn1, locals.var_x_dn3, locals.var_x_dn4, locals.var_x_dn5, locals.var_x_dn6, locals.var_x_dn7, locals.var_x_dn8, locals.var_x_dn9, locals.var_x_dn10, locals.var_x_dn11,)
    }
};
        locals.var_x = assign4460_e4253;
        locals.var_x_dn0 = assign4460_e4253_d_n0;
        locals.var_x_dn1 = assign4460_e4253_d_n1;
        locals.var_x_dn3 = assign4460_e4253_d_n3;
        locals.var_x_dn4 = assign4460_e4253_d_n4;
        locals.var_x_dn5 = assign4460_e4253_d_n5;
        locals.var_x_dn6 = assign4460_e4253_d_n6;
        locals.var_x_dn7 = assign4460_e4253_d_n7;
        locals.var_x_dn8 = assign4460_e4253_d_n8;
        locals.var_x_dn9 = assign4460_e4253_d_n9;
        locals.var_x_dn10 = assign4460_e4253_d_n10;
        locals.var_x_dn11 = assign4460_e4253_d_n11;
        locals.var_x_rv = 0.0;

        let assign4470_e4256: f64 = (-0.001);
        let assign4470_e4257: f64 = if locals.var_x < assign4470_e4256 { 1.0 } else { 0.0 };
        locals.var_guard76 = assign4470_e4257;
        locals.var_guard76_rv = 0.0;

        let assign4480_e4260: f64 = if locals.var_x < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard77 = assign4480_e4260;
        locals.var_guard77_rv = 0.0;

        let (assign4500_e4279,) = {
    if (((locals.var_guard74 != 0.0) && (locals.var_guard76 != 0.0)) && (locals.var_guard77 == 0.0)) {
        let assign4500_e4277: f64 = (p.p151).exp();
        (assign4500_e4277,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4500_e4279;
        locals.var_expl_rv = 0.0;

        let assign4570_e4369: f64 = if (((p.p36 > 0.0) && (p.p37 > 0.0)) && (locals.var_vb2c1 < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard78 = assign4570_e4369;
        locals.var_guard78_rv = 0.0;

        let (assign4580_e4381, assign4580_e4381_d_n0, assign4580_e4381_d_n1, assign4580_e4381_d_n3, assign4580_e4381_d_n4, assign4580_e4381_d_n5, assign4580_e4381_d_n6, assign4580_e4381_d_n7, assign4580_e4381_d_n8, assign4580_e4381_d_n9, assign4580_e4381_d_n10, assign4580_e4381_d_n11,) = {
    if (locals.var_guard78 != 0.0) {
        let assign4580_e4374: f64 = (locals.var_vb2c1 * locals.var_inv_vdc_zener_t);
        let assign4580_e4375: f64 = (1.0 - assign4580_e4374);
        let assign4580_e4378: f64 = (1.0 - locals.var_pc_zener);
        let assign4580_e4379: f64 = (assign4580_e4375).powf(assign4580_e4378);
        (assign4580_e4379, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn0)))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn0)) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn1)))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn1)) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn3)))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn3)) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn4)))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn4)) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn5)))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn5)) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn6)))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn6)) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-((locals.var_vb2c1_dn7 * locals.var_inv_vdc_zener_t) + (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn7))))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-((locals.var_vb2c1_dn7 * locals.var_inv_vdc_zener_t) + (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn7))) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-((locals.var_vb2c1_dn8 * locals.var_inv_vdc_zener_t) + (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn8))))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-((locals.var_vb2c1_dn8 * locals.var_inv_vdc_zener_t) + (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn8))) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn9)))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn9)) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn10)))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn10)) / assign4580_e4375))) }, if 0.0 == 0.0 && ((assign4580_e4378) as f64).is_finite() && ((assign4580_e4378) as f64).fract() == 0.0 { if assign4580_e4378 == 0.0 { 0.0 } else { (assign4580_e4378 * ((assign4580_e4375).powf(assign4580_e4378 - 1.0) * (-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn11)))) } } else { (assign4580_e4379 * (assign4580_e4378 * ((-(locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn11)) / assign4580_e4375))) },)
    } else {
        (locals.var_e0cb, locals.var_e0cb_dn0, locals.var_e0cb_dn1, locals.var_e0cb_dn3, locals.var_e0cb_dn4, locals.var_e0cb_dn5, locals.var_e0cb_dn6, locals.var_e0cb_dn7, locals.var_e0cb_dn8, locals.var_e0cb_dn9, locals.var_e0cb_dn10, locals.var_e0cb_dn11,)
    }
};
        locals.var_e0cb = assign4580_e4381;
        locals.var_e0cb_dn0 = assign4580_e4381_d_n0;
        locals.var_e0cb_dn1 = assign4580_e4381_d_n1;
        locals.var_e0cb_dn3 = assign4580_e4381_d_n3;
        locals.var_e0cb_dn4 = assign4580_e4381_d_n4;
        locals.var_e0cb_dn5 = assign4580_e4381_d_n5;
        locals.var_e0cb_dn6 = assign4580_e4381_d_n6;
        locals.var_e0cb_dn7 = assign4580_e4381_d_n7;
        locals.var_e0cb_dn8 = assign4580_e4381_d_n8;
        locals.var_e0cb_dn9 = assign4580_e4381_d_n9;
        locals.var_e0cb_dn10 = assign4580_e4381_d_n10;
        locals.var_e0cb_dn11 = assign4580_e4381_d_n11;
        locals.var_e0cb_rv = 0.0;

        let assign4590_e4387: f64 = (2.0 * locals.var_e0cb);
        let assign4590_e4388: f64 = (locals.var_pow2_2m_pc / assign4590_e4387);
        let assign4590_e4389: f64 = (1.0 - assign4590_e4388);
        let assign4590_e4390: f64 = (locals.var_nzcb_t * assign4590_e4389);
        let assign4590_e4392: f64 = if assign4590_e4390 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard79 = assign4590_e4392;
        locals.var_guard79_rv = 0.0;

        let (assign4610_e4415,) = {
    if ((locals.var_guard78 != 0.0) && (locals.var_guard79 == 0.0)) {
        let assign4610_e4413: f64 = (p.p151).exp();
        (assign4610_e4413,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4610_e4415;
        locals.var_expl_rv = 0.0;

        let (assign4630_e4442, assign4630_e4442_d_n0, assign4630_e4442_d_n1, assign4630_e4442_d_n3, assign4630_e4442_d_n4, assign4630_e4442_d_n5, assign4630_e4442_d_n6, assign4630_e4442_d_n7, assign4630_e4442_d_n8, assign4630_e4442_d_n9, assign4630_e4442_d_n10, assign4630_e4442_d_n11,) = {
    if (locals.var_guard78 != 0.0) {
        let assign4630_e4440: f64 = (locals.var_vb2c1 * locals.var_inv_vdc_zener_t);
        (assign4630_e4440, (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn0), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn1), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn3), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn4), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn5), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn6), ((locals.var_vb2c1_dn7 * locals.var_inv_vdc_zener_t) + (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn7)), ((locals.var_vb2c1_dn8 * locals.var_inv_vdc_zener_t) + (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn8)), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn9), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn10), (locals.var_vb2c1 * locals.var_inv_vdc_zener_t_dn11),)
    } else {
        (locals.var_xx, locals.var_xx_dn0, locals.var_xx_dn1, locals.var_xx_dn3, locals.var_xx_dn4, locals.var_xx_dn5, locals.var_xx_dn6, locals.var_xx_dn7, locals.var_xx_dn8, locals.var_xx_dn9, locals.var_xx_dn10, locals.var_xx_dn11,)
    }
};
        locals.var_xx = assign4630_e4442;
        locals.var_xx_dn0 = assign4630_e4442_d_n0;
        locals.var_xx_dn1 = assign4630_e4442_d_n1;
        locals.var_xx_dn3 = assign4630_e4442_d_n3;
        locals.var_xx_dn4 = assign4630_e4442_d_n4;
        locals.var_xx_dn5 = assign4630_e4442_d_n5;
        locals.var_xx_dn6 = assign4630_e4442_d_n6;
        locals.var_xx_dn7 = assign4630_e4442_d_n7;
        locals.var_xx_dn8 = assign4630_e4442_d_n8;
        locals.var_xx_dn9 = assign4630_e4442_d_n9;
        locals.var_xx_dn10 = assign4630_e4442_d_n10;
        locals.var_xx_dn11 = assign4630_e4442_d_n11;
        locals.var_xx_rv = 0.0;

        let (assign4640_e4486, assign4640_e4486_d_n0, assign4640_e4486_d_n1, assign4640_e4486_d_n3, assign4640_e4486_d_n4, assign4640_e4486_d_n5, assign4640_e4486_d_n6, assign4640_e4486_d_n7, assign4640_e4486_d_n8, assign4640_e4486_d_n9, assign4640_e4486_d_n10, assign4640_e4486_d_n11,) = {
    if (locals.var_guard78 != 0.0) {
        let assign4640_e4446: f64 = (locals.var_xx * locals.var_xx);
        let assign4640_e4448: f64 = (assign4640_e4446 + 1e-30);
        let assign4640_e4449: f64 = (assign4640_e4448).sqrt();
        let assign4640_e4451: f64 = (-2.0);
        let assign4640_e4453: f64 = (assign4640_e4451 - locals.var_pc_zener);
        let assign4640_e4454: f64 = (assign4640_e4449).powf(assign4640_e4453);
        let assign4640_e4459: f64 = (locals.var_pc_zener * locals.var_pc_zener);
        let assign4640_e4460: f64 = (1.0 - assign4640_e4459);
        let assign4640_e4463: f64 = (3.0 * locals.var_xx);
        let assign4640_e4466: f64 = (locals.var_pc_zener - 1.0);
        let assign4640_e4467: f64 = (assign4640_e4463 * assign4640_e4466);
        let assign4640_e4468: f64 = (assign4640_e4460 - assign4640_e4467);
        let assign4640_e4469: f64 = (locals.var_pc_zener * assign4640_e4468);
        let assign4640_e4472: f64 = (6.0 * locals.var_xx);
        let assign4640_e4474: f64 = (assign4640_e4472 * locals.var_xx);
        let assign4640_e4477: f64 = (locals.var_pc_zener - 1.0);
        let assign4640_e4479: f64 = (assign4640_e4477 + locals.var_xx);
        let assign4640_e4480: f64 = (assign4640_e4474 * assign4640_e4479);
        let assign4640_e4481: f64 = (assign4640_e4469 - assign4640_e4480);
        let assign4640_e4482: f64 = (assign4640_e4454 * assign4640_e4481);
        let assign4640_e4484: f64 = (assign4640_e4482 * 0.16666666666666666);
        (assign4640_e4484, (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn0 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn0)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn0 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn0)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn0) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn0) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn0)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn0))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn1 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn1)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn1 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn1)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn1) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn1) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn1)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn1))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn3 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn3)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn3 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn3)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn3) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn3) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn3)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn3))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn4 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn4)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn4 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn4)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn4) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn4) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn4)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn4))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn5 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn5)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn5 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn5)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn5) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn5) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn5)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn5))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn6 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn6)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn6 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn6)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn6) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn6) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn6)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn6))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn7 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn7)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn7 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn7)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn7) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn7) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn7)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn7))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn8 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn8)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn8 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn8)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn8) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn8) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn8)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn8))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn9 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn9)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn9 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn9)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn9) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn9) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn9)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn9))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn10 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn10)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn10 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn10)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn10) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn10) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn10)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn10))))) * 0.16666666666666666), (((if 0.0 == 0.0 && ((assign4640_e4453) as f64).is_finite() && ((assign4640_e4453) as f64).fract() == 0.0 { if assign4640_e4453 == 0.0 { 0.0 } else { (assign4640_e4453 * ((assign4640_e4449).powf(assign4640_e4453 - 1.0) * (((locals.var_xx_dn11 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn11)) / (2.0 * assign4640_e4449)))) } } else { (assign4640_e4454 * (assign4640_e4453 * ((((locals.var_xx_dn11 * locals.var_xx) + (locals.var_xx * locals.var_xx_dn11)) / (2.0 * assign4640_e4449)) / assign4640_e4449))) } * assign4640_e4481) + (assign4640_e4454 * ((locals.var_pc_zener * (-((3.0 * locals.var_xx_dn11) * assign4640_e4466))) - (((((6.0 * locals.var_xx_dn11) * locals.var_xx) + (assign4640_e4472 * locals.var_xx_dn11)) * assign4640_e4479) + (assign4640_e4474 * locals.var_xx_dn11))))) * 0.16666666666666666),)
    } else {
        (locals.var_de0cb, locals.var_de0cb_dn0, locals.var_de0cb_dn1, locals.var_de0cb_dn3, locals.var_de0cb_dn4, locals.var_de0cb_dn5, locals.var_de0cb_dn6, locals.var_de0cb_dn7, locals.var_de0cb_dn8, locals.var_de0cb_dn9, locals.var_de0cb_dn10, locals.var_de0cb_dn11,)
    }
};
        locals.var_de0cb = assign4640_e4486;
        locals.var_de0cb_dn0 = assign4640_e4486_d_n0;
        locals.var_de0cb_dn1 = assign4640_e4486_d_n1;
        locals.var_de0cb_dn3 = assign4640_e4486_d_n3;
        locals.var_de0cb_dn4 = assign4640_e4486_d_n4;
        locals.var_de0cb_dn5 = assign4640_e4486_d_n5;
        locals.var_de0cb_dn6 = assign4640_e4486_d_n6;
        locals.var_de0cb_dn7 = assign4640_e4486_d_n7;
        locals.var_de0cb_dn8 = assign4640_e4486_d_n8;
        locals.var_de0cb_dn9 = assign4640_e4486_d_n9;
        locals.var_de0cb_dn10 = assign4640_e4486_d_n10;
        locals.var_de0cb_dn11 = assign4640_e4486_d_n11;
        locals.var_de0cb_rv = 0.0;

        let (assign4650_e4498, assign4650_e4498_d_n0, assign4650_e4498_d_n1, assign4650_e4498_d_n3, assign4650_e4498_d_n4, assign4650_e4498_d_n5, assign4650_e4498_d_n6, assign4650_e4498_d_n7, assign4650_e4498_d_n8, assign4650_e4498_d_n9, assign4650_e4498_d_n10, assign4650_e4498_d_n11,) = {
    if (locals.var_guard78 != 0.0) {
        let assign4650_e4490: f64 = (locals.var_vb2c1 * locals.var_pow2_2m_pc);
        let assign4650_e4492: f64 = (assign4650_e4490 * locals.var_nzcb_t);
        let assign4650_e4495: f64 = (locals.var_vgzcb_t * locals.var_de0cb);
        let assign4650_e4496: f64 = (assign4650_e4492 / assign4650_e4495);
        (assign4650_e4496, ((((assign4650_e4490 * locals.var_nzcb_t_dn0) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn0 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn0)))) / (assign4650_e4495 * assign4650_e4495)), ((((assign4650_e4490 * locals.var_nzcb_t_dn1) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn1 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn1)))) / (assign4650_e4495 * assign4650_e4495)), ((((assign4650_e4490 * locals.var_nzcb_t_dn3) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn3 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn3)))) / (assign4650_e4495 * assign4650_e4495)), ((((assign4650_e4490 * locals.var_nzcb_t_dn4) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn4 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn4)))) / (assign4650_e4495 * assign4650_e4495)), ((((assign4650_e4490 * locals.var_nzcb_t_dn5) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn5 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn5)))) / (assign4650_e4495 * assign4650_e4495)), ((((assign4650_e4490 * locals.var_nzcb_t_dn6) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn6 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn6)))) / (assign4650_e4495 * assign4650_e4495)), ((((((locals.var_vb2c1_dn7 * locals.var_pow2_2m_pc) * locals.var_nzcb_t) + (assign4650_e4490 * locals.var_nzcb_t_dn7)) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn7 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn7)))) / (assign4650_e4495 * assign4650_e4495)), ((((((locals.var_vb2c1_dn8 * locals.var_pow2_2m_pc) * locals.var_nzcb_t) + (assign4650_e4490 * locals.var_nzcb_t_dn8)) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn8 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn8)))) / (assign4650_e4495 * assign4650_e4495)), ((((assign4650_e4490 * locals.var_nzcb_t_dn9) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn9 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn9)))) / (assign4650_e4495 * assign4650_e4495)), ((((assign4650_e4490 * locals.var_nzcb_t_dn10) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn10 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn10)))) / (assign4650_e4495 * assign4650_e4495)), ((((assign4650_e4490 * locals.var_nzcb_t_dn11) * assign4650_e4495) - (assign4650_e4492 * ((locals.var_vgzcb_t_dn11 * locals.var_de0cb) + (locals.var_vgzcb_t * locals.var_de0cb_dn11)))) / (assign4650_e4495 * assign4650_e4495)),)
    } else {
        (locals.var_xx, locals.var_xx_dn0, locals.var_xx_dn1, locals.var_xx_dn3, locals.var_xx_dn4, locals.var_xx_dn5, locals.var_xx_dn6, locals.var_xx_dn7, locals.var_xx_dn8, locals.var_xx_dn9, locals.var_xx_dn10, locals.var_xx_dn11,)
    }
};
        locals.var_xx = assign4650_e4498;
        locals.var_xx_dn0 = assign4650_e4498_d_n0;
        locals.var_xx_dn1 = assign4650_e4498_d_n1;
        locals.var_xx_dn3 = assign4650_e4498_d_n3;
        locals.var_xx_dn4 = assign4650_e4498_d_n4;
        locals.var_xx_dn5 = assign4650_e4498_d_n5;
        locals.var_xx_dn6 = assign4650_e4498_d_n6;
        locals.var_xx_dn7 = assign4650_e4498_d_n7;
        locals.var_xx_dn8 = assign4650_e4498_d_n8;
        locals.var_xx_dn9 = assign4650_e4498_d_n9;
        locals.var_xx_dn10 = assign4650_e4498_d_n10;
        locals.var_xx_dn11 = assign4650_e4498_d_n11;
        locals.var_xx_rv = 0.0;

        let assign4660_e4501: f64 = (-0.001);
        let assign4660_e4502: f64 = if locals.var_xx < assign4660_e4501 { 1.0 } else { 0.0 };
        locals.var_guard80 = assign4660_e4502;
        locals.var_guard80_rv = 0.0;

        let assign4670_e4505: f64 = if locals.var_xx < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard81 = assign4670_e4505;
        locals.var_guard81_rv = 0.0;

        let (assign4690_e4524,) = {
    if (((locals.var_guard78 != 0.0) && (locals.var_guard80 != 0.0)) && (locals.var_guard81 == 0.0)) {
        let assign4690_e4522: f64 = (p.p151).exp();
        (assign4690_e4522,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4690_e4524;
        locals.var_expl_rv = 0.0;

        let assign4760_e4606: f64 = (locals.var_if0 * locals.var_evb1c4);
        locals.var_g1 = assign4760_e4606;
        locals.var_g1_dn0 = (locals.var_if0_dn0 * locals.var_evb1c4);
        locals.var_g1_dn1 = (locals.var_if0_dn1 * locals.var_evb1c4);
        locals.var_g1_dn3 = (locals.var_if0_dn3 * locals.var_evb1c4);
        locals.var_g1_dn4 = ((locals.var_if0_dn4 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn4));
        locals.var_g1_dn5 = (locals.var_if0_dn5 * locals.var_evb1c4);
        locals.var_g1_dn6 = ((locals.var_if0_dn6 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn6));
        locals.var_g1_dn7 = ((locals.var_if0_dn7 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn7));
        locals.var_g1_dn8 = ((locals.var_if0_dn8 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn8));
        locals.var_g1_dn9 = ((locals.var_if0_dn9 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn9));
        locals.var_g1_dn10 = (locals.var_if0_dn10 * locals.var_evb1c4);
        locals.var_g1_dn11 = ((locals.var_if0_dn11 * locals.var_evb1c4) + (locals.var_if0 * locals.var_evb1c4_dn11));
        locals.var_g1_rv = 0.0;

        let assign4770_e4609: f64 = (4.0 * locals.var_evb1c4vdc);
        locals.var_g2 = assign4770_e4609;
        locals.var_g2_dn0 = (4.0 * locals.var_evb1c4vdc_dn0);
        locals.var_g2_dn1 = (4.0 * locals.var_evb1c4vdc_dn1);
        locals.var_g2_dn3 = (4.0 * locals.var_evb1c4vdc_dn3);
        locals.var_g2_dn4 = (4.0 * locals.var_evb1c4vdc_dn4);
        locals.var_g2_dn5 = (4.0 * locals.var_evb1c4vdc_dn5);
        locals.var_g2_dn6 = (4.0 * locals.var_evb1c4vdc_dn6);
        locals.var_g2_dn7 = (4.0 * locals.var_evb1c4vdc_dn7);
        locals.var_g2_dn8 = (4.0 * locals.var_evb1c4vdc_dn8);
        locals.var_g2_dn9 = (4.0 * locals.var_evb1c4vdc_dn9);
        locals.var_g2_dn10 = (4.0 * locals.var_evb1c4vdc_dn10);
        locals.var_g2_dn11 = (4.0 * locals.var_evb1c4vdc_dn11);
        locals.var_g2_rv = 0.0;

        let assign4780_e4612: f64 = (locals.var_g1 - locals.var_if0);
        let assign4780_e4616: f64 = (1.0 + locals.var_g1);
        let assign4780_e4617: f64 = (assign4780_e4616).sqrt();
        let assign4780_e4618: f64 = (1.0 + assign4780_e4617);
        let assign4780_e4619: f64 = (assign4780_e4612 / assign4780_e4618);
        locals.var_nbex = assign4780_e4619;
        locals.var_nbex_dn0 = ((((locals.var_g1_dn0 - locals.var_if0_dn0) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn0 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn1 = ((((locals.var_g1_dn1 - locals.var_if0_dn1) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn1 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn3 = ((((locals.var_g1_dn3 - locals.var_if0_dn3) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn3 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn4 = ((((locals.var_g1_dn4 - locals.var_if0_dn4) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn4 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn5 = ((((locals.var_g1_dn5 - locals.var_if0_dn5) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn5 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn6 = ((((locals.var_g1_dn6 - locals.var_if0_dn6) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn6 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn7 = ((((locals.var_g1_dn7 - locals.var_if0_dn7) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn7 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn8 = ((((locals.var_g1_dn8 - locals.var_if0_dn8) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn8 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn9 = ((((locals.var_g1_dn9 - locals.var_if0_dn9) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn9 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn10 = ((((locals.var_g1_dn10 - locals.var_if0_dn10) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn10 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_dn11 = ((((locals.var_g1_dn11 - locals.var_if0_dn11) * assign4780_e4618) - (assign4780_e4612 * (locals.var_g1_dn11 / (2.0 * assign4780_e4617)))) / (assign4780_e4618 * assign4780_e4618));
        locals.var_nbex_rv = 0.0;

        let assign4790_e4624: f64 = (1.0 + locals.var_g2);
        let assign4790_e4625: f64 = (assign4790_e4624).sqrt();
        let assign4790_e4626: f64 = (1.0 + assign4790_e4625);
        let assign4790_e4627: f64 = (locals.var_g2 / assign4790_e4626);
        locals.var_pwex = assign4790_e4627;
        locals.var_pwex_dn0 = (((locals.var_g2_dn0 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn0 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn1 = (((locals.var_g2_dn1 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn1 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn3 = (((locals.var_g2_dn3 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn3 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn4 = (((locals.var_g2_dn4 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn4 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn5 = (((locals.var_g2_dn5 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn5 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn6 = (((locals.var_g2_dn6 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn6 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn7 = (((locals.var_g2_dn7 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn7 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn8 = (((locals.var_g2_dn8 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn8 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn9 = (((locals.var_g2_dn9 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn9 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn10 = (((locals.var_g2_dn10 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn10 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_dn11 = (((locals.var_g2_dn11 * assign4790_e4626) - (locals.var_g2 * (locals.var_g2_dn11 / (2.0 * assign4790_e4625)))) / (assign4790_e4626 * assign4790_e4626));
        locals.var_pwex_rv = 0.0;

        let assign4880_e4798: f64 = if ((p.p5 > 0.0) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard83 = assign4880_e4798;
        locals.var_guard83_rv = 0.0;

        let (assign4910_e4835, assign4910_e4835_d_n0, assign4910_e4835_d_n1, assign4910_e4835_d_n4, assign4910_e4835_d_n6, assign4910_e4835_d_n7, assign4910_e4835_d_n8, assign4910_e4835_d_n9, assign4910_e4835_d_n10, assign4910_e4835_d_n11,) = {
    if (locals.var_guard83 != 0.0) {
        let assign4910_e4814: f64 = (p.p33 * 2.0);
        let assign4910_e4816: f64 = (assign4910_e4814 * locals.var_ibx_t);
        let assign4910_e4819: f64 = (locals.var_evbc3 - 1.0);
        let assign4910_e4820: f64 = (assign4910_e4816 * assign4910_e4819);
        let assign4910_e4825: f64 = (4.0 * locals.var_ibx_t);
        let assign4910_e4827: f64 = (assign4910_e4825 / locals.var_ikbx_t);
        let assign4910_e4829: f64 = (assign4910_e4827 * locals.var_evbc3);
        let assign4910_e4830: f64 = (1.0 + assign4910_e4829);
        let assign4910_e4831: f64 = (assign4910_e4830).sqrt();
        let assign4910_e4832: f64 = (1.0 + assign4910_e4831);
        let assign4910_e4833: f64 = (assign4910_e4820 / assign4910_e4832);
        (assign4910_e4833, ((((assign4910_e4816 * locals.var_evbc3_dn0) * assign4910_e4832) - (assign4910_e4820 * ((assign4910_e4827 * locals.var_evbc3_dn0) / (2.0 * assign4910_e4831)))) / (assign4910_e4832 * assign4910_e4832)), ((((assign4910_e4816 * locals.var_evbc3_dn1) * assign4910_e4832) - (assign4910_e4820 * ((assign4910_e4827 * locals.var_evbc3_dn1) / (2.0 * assign4910_e4831)))) / (assign4910_e4832 * assign4910_e4832)), ((((((assign4910_e4814 * locals.var_ibx_t_dn4) * assign4910_e4819) + (assign4910_e4816 * locals.var_evbc3_dn4)) * assign4910_e4832) - (assign4910_e4820 * (((((((4.0 * locals.var_ibx_t_dn4) * locals.var_ikbx_t) - (assign4910_e4825 * locals.var_ikbx_t_dn4)) / (locals.var_ikbx_t * locals.var_ikbx_t)) * locals.var_evbc3) + (assign4910_e4827 * locals.var_evbc3_dn4)) / (2.0 * assign4910_e4831)))) / (assign4910_e4832 * assign4910_e4832)), ((((assign4910_e4816 * locals.var_evbc3_dn6) * assign4910_e4832) - (assign4910_e4820 * ((assign4910_e4827 * locals.var_evbc3_dn6) / (2.0 * assign4910_e4831)))) / (assign4910_e4832 * assign4910_e4832)), ((((assign4910_e4816 * locals.var_evbc3_dn7) * assign4910_e4832) - (assign4910_e4820 * ((assign4910_e4827 * locals.var_evbc3_dn7) / (2.0 * assign4910_e4831)))) / (assign4910_e4832 * assign4910_e4832)), ((((assign4910_e4816 * locals.var_evbc3_dn8) * assign4910_e4832) - (assign4910_e4820 * ((assign4910_e4827 * locals.var_evbc3_dn8) / (2.0 * assign4910_e4831)))) / (assign4910_e4832 * assign4910_e4832)), ((((assign4910_e4816 * locals.var_evbc3_dn9) * assign4910_e4832) - (assign4910_e4820 * ((assign4910_e4827 * locals.var_evbc3_dn9) / (2.0 * assign4910_e4831)))) / (assign4910_e4832 * assign4910_e4832)), ((((assign4910_e4816 * locals.var_evbc3_dn10) * assign4910_e4832) - (assign4910_e4820 * ((assign4910_e4827 * locals.var_evbc3_dn10) / (2.0 * assign4910_e4831)))) / (assign4910_e4832 * assign4910_e4832)), ((((assign4910_e4816 * locals.var_evbc3_dn11) * assign4910_e4832) - (assign4910_e4820 * ((assign4910_e4827 * locals.var_evbc3_dn11) / (2.0 * assign4910_e4831)))) / (assign4910_e4832 * assign4910_e4832)),)
    } else {
        (locals.var_ximex, locals.var_ximex_dn0, locals.var_ximex_dn1, locals.var_ximex_dn4, locals.var_ximex_dn6, locals.var_ximex_dn7, locals.var_ximex_dn8, locals.var_ximex_dn9, locals.var_ximex_dn10, locals.var_ximex_dn11,)
    }
};
        locals.var_ximex = assign4910_e4835;
        locals.var_ximex_dn0 = assign4910_e4835_d_n0;
        locals.var_ximex_dn1 = assign4910_e4835_d_n1;
        locals.var_ximex_dn4 = assign4910_e4835_d_n4;
        locals.var_ximex_dn6 = assign4910_e4835_d_n6;
        locals.var_ximex_dn7 = assign4910_e4835_d_n7;
        locals.var_ximex_dn8 = assign4910_e4835_d_n8;
        locals.var_ximex_dn9 = assign4910_e4835_d_n9;
        locals.var_ximex_dn10 = assign4910_e4835_d_n10;
        locals.var_ximex_dn11 = assign4910_e4835_d_n11;
        locals.var_ximex_rv = 0.0;

        let assign4920_e4838: f64 = if p.p8 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard84 = assign4920_e4838;
        locals.var_guard84_rv = 0.0;

        let (assign4930_e4873, assign4930_e4873_d_n0, assign4930_e4873_d_n1, assign4930_e4873_d_n3, assign4930_e4873_d_n4, assign4930_e4873_d_n6, assign4930_e4873_d_n7, assign4930_e4873_d_n8, assign4930_e4873_d_n9, assign4930_e4873_d_n10, assign4930_e4873_d_n11,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard84 != 0.0)) {
        let assign4930_e4844: f64 = (1.0 - p.p143);
        let assign4930_e4846: f64 = (assign4930_e4844 * p.p33);
        let assign4930_e4848: f64 = (assign4930_e4846 * 2.0);
        let assign4930_e4850: f64 = (assign4930_e4848 * locals.var_iss_t);
        let assign4930_e4853: f64 = (locals.var_evbc3 - locals.var_evsc3);
        let assign4930_e4854: f64 = (assign4930_e4850 * assign4930_e4853);
        let assign4930_e4859: f64 = (4.0 * locals.var_iss_t);
        let assign4930_e4861: f64 = (assign4930_e4859 / locals.var_iks_t);
        let assign4930_e4865: f64 = (p.p144 * locals.var_evsc3);
        let assign4930_e4866: f64 = (locals.var_evbc3 + assign4930_e4865);
        let assign4930_e4867: f64 = (assign4930_e4861 * assign4930_e4866);
        let assign4930_e4868: f64 = (1.0 + assign4930_e4867);
        let assign4930_e4869: f64 = (assign4930_e4868).sqrt();
        let assign4930_e4870: f64 = (1.0 + assign4930_e4869);
        let assign4930_e4871: f64 = (assign4930_e4854 / assign4930_e4870);
        (assign4930_e4871, ((((assign4930_e4850 * locals.var_evbc3_dn0) * assign4930_e4870) - (assign4930_e4854 * ((assign4930_e4861 * locals.var_evbc3_dn0) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)), ((((assign4930_e4850 * locals.var_evbc3_dn1) * assign4930_e4870) - (assign4930_e4854 * ((assign4930_e4861 * locals.var_evbc3_dn1) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)), ((((assign4930_e4850 * (-locals.var_evsc3_dn3)) * assign4930_e4870) - (assign4930_e4854 * ((assign4930_e4861 * (p.p144 * locals.var_evsc3_dn3)) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)), ((((((assign4930_e4848 * locals.var_iss_t_dn4) * assign4930_e4853) + (assign4930_e4850 * (locals.var_evbc3_dn4 - locals.var_evsc3_dn4))) * assign4930_e4870) - (assign4930_e4854 * (((((((4.0 * locals.var_iss_t_dn4) * locals.var_iks_t) - (assign4930_e4859 * locals.var_iks_t_dn4)) / (locals.var_iks_t * locals.var_iks_t)) * assign4930_e4866) + (assign4930_e4861 * (locals.var_evbc3_dn4 + (p.p144 * locals.var_evsc3_dn4)))) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)), ((((assign4930_e4850 * locals.var_evbc3_dn6) * assign4930_e4870) - (assign4930_e4854 * ((assign4930_e4861 * locals.var_evbc3_dn6) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)), ((((assign4930_e4850 * locals.var_evbc3_dn7) * assign4930_e4870) - (assign4930_e4854 * ((assign4930_e4861 * locals.var_evbc3_dn7) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)), ((((assign4930_e4850 * (locals.var_evbc3_dn8 - locals.var_evsc3_dn8)) * assign4930_e4870) - (assign4930_e4854 * ((assign4930_e4861 * (locals.var_evbc3_dn8 + (p.p144 * locals.var_evsc3_dn8))) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)), ((((assign4930_e4850 * locals.var_evbc3_dn9) * assign4930_e4870) - (assign4930_e4854 * ((assign4930_e4861 * locals.var_evbc3_dn9) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)), ((((assign4930_e4850 * (locals.var_evbc3_dn10 - locals.var_evsc3_dn10)) * assign4930_e4870) - (assign4930_e4854 * ((assign4930_e4861 * (locals.var_evbc3_dn10 + (p.p144 * locals.var_evsc3_dn10))) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)), ((((assign4930_e4850 * (locals.var_evbc3_dn11 - locals.var_evsc3_dn11)) * assign4930_e4870) - (assign4930_e4854 * ((assign4930_e4861 * (locals.var_evbc3_dn11 + (p.p144 * locals.var_evsc3_dn11))) / (2.0 * assign4930_e4869)))) / (assign4930_e4870 * assign4930_e4870)),)
    } else {
        (locals.var_ximsub, locals.var_ximsub_dn0, locals.var_ximsub_dn1, locals.var_ximsub_dn3, locals.var_ximsub_dn4, locals.var_ximsub_dn6, locals.var_ximsub_dn7, locals.var_ximsub_dn8, locals.var_ximsub_dn9, locals.var_ximsub_dn10, locals.var_ximsub_dn11,)
    }
};
        locals.var_ximsub = assign4930_e4873;
        locals.var_ximsub_dn0 = assign4930_e4873_d_n0;
        locals.var_ximsub_dn1 = assign4930_e4873_d_n1;
        locals.var_ximsub_dn3 = assign4930_e4873_d_n3;
        locals.var_ximsub_dn4 = assign4930_e4873_d_n4;
        locals.var_ximsub_dn6 = assign4930_e4873_d_n6;
        locals.var_ximsub_dn7 = assign4930_e4873_d_n7;
        locals.var_ximsub_dn8 = assign4930_e4873_d_n8;
        locals.var_ximsub_dn9 = assign4930_e4873_d_n9;
        locals.var_ximsub_dn10 = assign4930_e4873_d_n10;
        locals.var_ximsub_dn11 = assign4930_e4873_d_n11;
        locals.var_ximsub_rv = 0.0;

        let (assign4940_e4905, assign4940_e4905_d_n0, assign4940_e4905_d_n1, assign4940_e4905_d_n3, assign4940_e4905_d_n4, assign4940_e4905_d_n6, assign4940_e4905_d_n7, assign4940_e4905_d_n8, assign4940_e4905_d_n9, assign4940_e4905_d_n10, assign4940_e4905_d_n11,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard84 == 0.0)) {
        let assign4940_e4880: f64 = (1.0 - p.p143);
        let assign4940_e4882: f64 = (assign4940_e4880 * p.p33);
        let assign4940_e4884: f64 = (assign4940_e4882 * 2.0);
        let assign4940_e4886: f64 = (assign4940_e4884 * locals.var_iss_t);
        let assign4940_e4889: f64 = (locals.var_evbc3 - 1.0);
        let assign4940_e4890: f64 = (assign4940_e4886 * assign4940_e4889);
        let assign4940_e4895: f64 = (4.0 * locals.var_iss_t);
        let assign4940_e4897: f64 = (assign4940_e4895 / locals.var_iks_t);
        let assign4940_e4899: f64 = (assign4940_e4897 * locals.var_evbc3);
        let assign4940_e4900: f64 = (1.0 + assign4940_e4899);
        let assign4940_e4901: f64 = (assign4940_e4900).sqrt();
        let assign4940_e4902: f64 = (1.0 + assign4940_e4901);
        let assign4940_e4903: f64 = (assign4940_e4890 / assign4940_e4902);
        (assign4940_e4903, ((((assign4940_e4886 * locals.var_evbc3_dn0) * assign4940_e4902) - (assign4940_e4890 * ((assign4940_e4897 * locals.var_evbc3_dn0) / (2.0 * assign4940_e4901)))) / (assign4940_e4902 * assign4940_e4902)), ((((assign4940_e4886 * locals.var_evbc3_dn1) * assign4940_e4902) - (assign4940_e4890 * ((assign4940_e4897 * locals.var_evbc3_dn1) / (2.0 * assign4940_e4901)))) / (assign4940_e4902 * assign4940_e4902)), 0.0, ((((((assign4940_e4884 * locals.var_iss_t_dn4) * assign4940_e4889) + (assign4940_e4886 * locals.var_evbc3_dn4)) * assign4940_e4902) - (assign4940_e4890 * (((((((4.0 * locals.var_iss_t_dn4) * locals.var_iks_t) - (assign4940_e4895 * locals.var_iks_t_dn4)) / (locals.var_iks_t * locals.var_iks_t)) * locals.var_evbc3) + (assign4940_e4897 * locals.var_evbc3_dn4)) / (2.0 * assign4940_e4901)))) / (assign4940_e4902 * assign4940_e4902)), ((((assign4940_e4886 * locals.var_evbc3_dn6) * assign4940_e4902) - (assign4940_e4890 * ((assign4940_e4897 * locals.var_evbc3_dn6) / (2.0 * assign4940_e4901)))) / (assign4940_e4902 * assign4940_e4902)), ((((assign4940_e4886 * locals.var_evbc3_dn7) * assign4940_e4902) - (assign4940_e4890 * ((assign4940_e4897 * locals.var_evbc3_dn7) / (2.0 * assign4940_e4901)))) / (assign4940_e4902 * assign4940_e4902)), ((((assign4940_e4886 * locals.var_evbc3_dn8) * assign4940_e4902) - (assign4940_e4890 * ((assign4940_e4897 * locals.var_evbc3_dn8) / (2.0 * assign4940_e4901)))) / (assign4940_e4902 * assign4940_e4902)), ((((assign4940_e4886 * locals.var_evbc3_dn9) * assign4940_e4902) - (assign4940_e4890 * ((assign4940_e4897 * locals.var_evbc3_dn9) / (2.0 * assign4940_e4901)))) / (assign4940_e4902 * assign4940_e4902)), ((((assign4940_e4886 * locals.var_evbc3_dn10) * assign4940_e4902) - (assign4940_e4890 * ((assign4940_e4897 * locals.var_evbc3_dn10) / (2.0 * assign4940_e4901)))) / (assign4940_e4902 * assign4940_e4902)), ((((assign4940_e4886 * locals.var_evbc3_dn11) * assign4940_e4902) - (assign4940_e4890 * ((assign4940_e4897 * locals.var_evbc3_dn11) / (2.0 * assign4940_e4901)))) / (assign4940_e4902 * assign4940_e4902)),)
    } else {
        (locals.var_ximsub, locals.var_ximsub_dn0, locals.var_ximsub_dn1, locals.var_ximsub_dn3, locals.var_ximsub_dn4, locals.var_ximsub_dn6, locals.var_ximsub_dn7, locals.var_ximsub_dn8, locals.var_ximsub_dn9, locals.var_ximsub_dn10, locals.var_ximsub_dn11,)
    }
};
        locals.var_ximsub = assign4940_e4905;
        locals.var_ximsub_dn0 = assign4940_e4905_d_n0;
        locals.var_ximsub_dn1 = assign4940_e4905_d_n1;
        locals.var_ximsub_dn3 = assign4940_e4905_d_n3;
        locals.var_ximsub_dn4 = assign4940_e4905_d_n4;
        locals.var_ximsub_dn6 = assign4940_e4905_d_n6;
        locals.var_ximsub_dn7 = assign4940_e4905_d_n7;
        locals.var_ximsub_dn8 = assign4940_e4905_d_n8;
        locals.var_ximsub_dn9 = assign4940_e4905_d_n9;
        locals.var_ximsub_dn10 = assign4940_e4905_d_n10;
        locals.var_ximsub_dn11 = assign4940_e4905_d_n11;
        locals.var_ximsub_rv = 0.0;

        let assign4950_e4908: f64 = if p.p5 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard85 = assign4950_e4908;
        locals.var_guard85_rv = 0.0;

        let (assign4960_e4920, assign4960_e4920_d_n4,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign4960_e4915: f64 = (locals.var_ibx_t + locals.var_iss_t);
        let assign4960_e4916: f64 = (p.p33 * assign4960_e4915);
        let assign4960_e4918: f64 = (assign4960_e4916 * locals.var_rcc_xx_t);
        (assign4960_e4918, (((p.p33 * (locals.var_ibx_t_dn4 + locals.var_iss_t_dn4)) * locals.var_rcc_xx_t) + (assign4960_e4916 * locals.var_rcc_xx_t_dn4)),)
    } else {
        (locals.var_vex_bias, locals.var_vex_bias_dn4,)
    }
};
        locals.var_vex_bias = assign4960_e4920;
        locals.var_vex_bias_dn4 = assign4960_e4920_d_n4;
        locals.var_vex_bias_rv = 0.0;

        let (assign4970_e4933, assign4970_e4933_d_n4,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign4970_e4928: f64 = (locals.var_vex_bias * locals.var_vtinv);
        let assign4970_e4929: f64 = (assign4970_e4928).ln();
        let assign4970_e4930: f64 = (2.0 - assign4970_e4929);
        let assign4970_e4931: f64 = (locals.var_vt * assign4970_e4930);
        (assign4970_e4931, ((locals.var_vt_dn4 * assign4970_e4930) + (locals.var_vt * (-(((locals.var_vex_bias_dn4 * locals.var_vtinv) + (locals.var_vex_bias * locals.var_vtinv_dn4)) / assign4970_e4928)))),)
    } else {
        (locals.var_vex, locals.var_vex_dn4,)
    }
};
        locals.var_vex = assign4970_e4933;
        locals.var_vex_dn4 = assign4970_e4933_d_n4;
        locals.var_vex_rv = 0.0;

        let (assign4980_e4941, assign4980_e4941_d_n0, assign4980_e4941_d_n1, assign4980_e4941_d_n4, assign4980_e4941_d_n6, assign4980_e4941_d_n7, assign4980_e4941_d_n8, assign4980_e4941_d_n9, assign4980_e4941_d_n10, assign4980_e4941_d_n11,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign4980_e4939: f64 = (locals.var_vbc3 - locals.var_vex);
        (assign4980_e4939, locals.var_vbc3_dn0, locals.var_vbc3_dn1, (-locals.var_vex_dn4), locals.var_vbc3_dn6, locals.var_vbc3_dn7, locals.var_vbc3_dn8, locals.var_vbc3_dn9, locals.var_vbc3_dn10, locals.var_vbc3_dn11,)
    } else {
        (locals.var_vdif, locals.var_vdif_dn0, locals.var_vdif_dn1, locals.var_vdif_dn4, locals.var_vdif_dn6, locals.var_vdif_dn7, locals.var_vdif_dn8, locals.var_vdif_dn9, locals.var_vdif_dn10, locals.var_vdif_dn11,)
    }
};
        locals.var_vdif = assign4980_e4941;
        locals.var_vdif_dn0 = assign4980_e4941_d_n0;
        locals.var_vdif_dn1 = assign4980_e4941_d_n1;
        locals.var_vdif_dn4 = assign4980_e4941_d_n4;
        locals.var_vdif_dn6 = assign4980_e4941_d_n6;
        locals.var_vdif_dn7 = assign4980_e4941_d_n7;
        locals.var_vdif_dn8 = assign4980_e4941_d_n8;
        locals.var_vdif_dn9 = assign4980_e4941_d_n9;
        locals.var_vdif_dn10 = assign4980_e4941_d_n10;
        locals.var_vdif_dn11 = assign4980_e4941_d_n11;
        locals.var_vdif_rv = 0.0;

        let (assign4990_e4949, assign4990_e4949_d_n0, assign4990_e4949_d_n1, assign4990_e4949_d_n3, assign4990_e4949_d_n4, assign4990_e4949_d_n5, assign4990_e4949_d_n6, assign4990_e4949_d_n7, assign4990_e4949_d_n8, assign4990_e4949_d_n9, assign4990_e4949_d_n10, assign4990_e4949_d_n11,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign4990_e4947: f64 = (0.11 * 0.11);
        (assign4990_e4947, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eps2, locals.var_eps2_dn0, locals.var_eps2_dn1, locals.var_eps2_dn3, locals.var_eps2_dn4, locals.var_eps2_dn5, locals.var_eps2_dn6, locals.var_eps2_dn7, locals.var_eps2_dn8, locals.var_eps2_dn9, locals.var_eps2_dn10, locals.var_eps2_dn11,)
    }
};
        locals.var_eps2 = assign4990_e4949;
        locals.var_eps2_dn0 = assign4990_e4949_d_n0;
        locals.var_eps2_dn1 = assign4990_e4949_d_n1;
        locals.var_eps2_dn3 = assign4990_e4949_d_n3;
        locals.var_eps2_dn4 = assign4990_e4949_d_n4;
        locals.var_eps2_dn5 = assign4990_e4949_d_n5;
        locals.var_eps2_dn6 = assign4990_e4949_d_n6;
        locals.var_eps2_dn7 = assign4990_e4949_d_n7;
        locals.var_eps2_dn8 = assign4990_e4949_d_n8;
        locals.var_eps2_dn9 = assign4990_e4949_d_n9;
        locals.var_eps2_dn10 = assign4990_e4949_d_n10;
        locals.var_eps2_dn11 = assign4990_e4949_d_n11;
        locals.var_eps2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5000_e4957, assign5000_e4957_d_n0, assign5000_e4957_d_n1, assign5000_e4957_d_n3, assign5000_e4957_d_n4, assign5000_e4957_d_n5, assign5000_e4957_d_n6, assign5000_e4957_d_n7, assign5000_e4957_d_n8, assign5000_e4957_d_n9, assign5000_e4957_d_n10, assign5000_e4957_d_n11,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign5000_e4955: f64 = (locals.var_vdif * locals.var_vdif);
        (assign5000_e4955, ((locals.var_vdif_dn0 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn0)), ((locals.var_vdif_dn1 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn1)), 0.0, ((locals.var_vdif_dn4 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn4)), 0.0, ((locals.var_vdif_dn6 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn6)), ((locals.var_vdif_dn7 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn7)), ((locals.var_vdif_dn8 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn8)), ((locals.var_vdif_dn9 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn9)), ((locals.var_vdif_dn10 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn10)), ((locals.var_vdif_dn11 * locals.var_vdif) + (locals.var_vdif * locals.var_vdif_dn11)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn1, locals.var_x2_dn3, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11,)
    }
};
        locals.var_x2 = assign5000_e4957;
        locals.var_x2_dn0 = assign5000_e4957_d_n0;
        locals.var_x2_dn1 = assign5000_e4957_d_n1;
        locals.var_x2_dn3 = assign5000_e4957_d_n3;
        locals.var_x2_dn4 = assign5000_e4957_d_n4;
        locals.var_x2_dn5 = assign5000_e4957_d_n5;
        locals.var_x2_dn6 = assign5000_e4957_d_n6;
        locals.var_x2_dn7 = assign5000_e4957_d_n7;
        locals.var_x2_dn8 = assign5000_e4957_d_n8;
        locals.var_x2_dn9 = assign5000_e4957_d_n9;
        locals.var_x2_dn10 = assign5000_e4957_d_n10;
        locals.var_x2_dn11 = assign5000_e4957_d_n11;
        locals.var_x2_rv = 0.0;

        let assign5010_e4960: f64 = if locals.var_vdif < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard86 = assign5010_e4960;
        locals.var_guard86_rv = 0.0;

        let (assign5020_e4977, assign5020_e4977_d_n0, assign5020_e4977_d_n1, assign5020_e4977_d_n3, assign5020_e4977_d_n4, assign5020_e4977_d_n5, assign5020_e4977_d_n6, assign5020_e4977_d_n7, assign5020_e4977_d_n8, assign5020_e4977_d_n9, assign5020_e4977_d_n10, assign5020_e4977_d_n11,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) && (locals.var_guard86 != 0.0)) {
        let assign5020_e4968: f64 = (0.5 * locals.var_eps2);
        let assign5020_e4971: f64 = (locals.var_x2 + locals.var_eps2);
        let assign5020_e4972: f64 = (assign5020_e4971).sqrt();
        let assign5020_e4974: f64 = (assign5020_e4972 - locals.var_vdif);
        let assign5020_e4975: f64 = (assign5020_e4968 / assign5020_e4974);
        (assign5020_e4975, ((((0.5 * locals.var_eps2_dn0) * assign5020_e4974) - (assign5020_e4968 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign5020_e4972)) - locals.var_vdif_dn0))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn1) * assign5020_e4974) - (assign5020_e4968 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign5020_e4972)) - locals.var_vdif_dn1))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn3) * assign5020_e4974) - (assign5020_e4968 * ((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign5020_e4972)))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn4) * assign5020_e4974) - (assign5020_e4968 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign5020_e4972)) - locals.var_vdif_dn4))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn5) * assign5020_e4974) - (assign5020_e4968 * ((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign5020_e4972)))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn6) * assign5020_e4974) - (assign5020_e4968 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign5020_e4972)) - locals.var_vdif_dn6))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn7) * assign5020_e4974) - (assign5020_e4968 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign5020_e4972)) - locals.var_vdif_dn7))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn8) * assign5020_e4974) - (assign5020_e4968 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign5020_e4972)) - locals.var_vdif_dn8))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn9) * assign5020_e4974) - (assign5020_e4968 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign5020_e4972)) - locals.var_vdif_dn9))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn10) * assign5020_e4974) - (assign5020_e4968 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign5020_e4972)) - locals.var_vdif_dn10))) / (assign5020_e4974 * assign5020_e4974)), ((((0.5 * locals.var_eps2_dn11) * assign5020_e4974) - (assign5020_e4968 * (((locals.var_x2_dn11 + locals.var_eps2_dn11) / (2.0 * assign5020_e4972)) - locals.var_vdif_dn11))) / (assign5020_e4974 * assign5020_e4974)),)
    } else {
        (locals.var_vbex, locals.var_vbex_dn0, locals.var_vbex_dn1, locals.var_vbex_dn3, locals.var_vbex_dn4, locals.var_vbex_dn5, locals.var_vbex_dn6, locals.var_vbex_dn7, locals.var_vbex_dn8, locals.var_vbex_dn9, locals.var_vbex_dn10, locals.var_vbex_dn11,)
    }
};
        locals.var_vbex = assign5020_e4977;
        locals.var_vbex_dn0 = assign5020_e4977_d_n0;
        locals.var_vbex_dn1 = assign5020_e4977_d_n1;
        locals.var_vbex_dn3 = assign5020_e4977_d_n3;
        locals.var_vbex_dn4 = assign5020_e4977_d_n4;
        locals.var_vbex_dn5 = assign5020_e4977_d_n5;
        locals.var_vbex_dn6 = assign5020_e4977_d_n6;
        locals.var_vbex_dn7 = assign5020_e4977_d_n7;
        locals.var_vbex_dn8 = assign5020_e4977_d_n8;
        locals.var_vbex_dn9 = assign5020_e4977_d_n9;
        locals.var_vbex_dn10 = assign5020_e4977_d_n10;
        locals.var_vbex_dn11 = assign5020_e4977_d_n11;
        locals.var_vbex_rv = 0.0;

        let (assign5030_e4993, assign5030_e4993_d_n0, assign5030_e4993_d_n1, assign5030_e4993_d_n3, assign5030_e4993_d_n4, assign5030_e4993_d_n5, assign5030_e4993_d_n6, assign5030_e4993_d_n7, assign5030_e4993_d_n8, assign5030_e4993_d_n9, assign5030_e4993_d_n10, assign5030_e4993_d_n11,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) && (locals.var_guard86 == 0.0)) {
        let assign5030_e4987: f64 = (locals.var_x2 + locals.var_eps2);
        let assign5030_e4988: f64 = (assign5030_e4987).sqrt();
        let assign5030_e4990: f64 = (assign5030_e4988 + locals.var_vdif);
        let assign5030_e4991: f64 = (0.5 * assign5030_e4990);
        (assign5030_e4991, (0.5 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign5030_e4988)) + locals.var_vdif_dn0)), (0.5 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign5030_e4988)) + locals.var_vdif_dn1)), (0.5 * ((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign5030_e4988))), (0.5 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign5030_e4988)) + locals.var_vdif_dn4)), (0.5 * ((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign5030_e4988))), (0.5 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign5030_e4988)) + locals.var_vdif_dn6)), (0.5 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign5030_e4988)) + locals.var_vdif_dn7)), (0.5 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign5030_e4988)) + locals.var_vdif_dn8)), (0.5 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign5030_e4988)) + locals.var_vdif_dn9)), (0.5 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign5030_e4988)) + locals.var_vdif_dn10)), (0.5 * (((locals.var_x2_dn11 + locals.var_eps2_dn11) / (2.0 * assign5030_e4988)) + locals.var_vdif_dn11)),)
    } else {
        (locals.var_vbex, locals.var_vbex_dn0, locals.var_vbex_dn1, locals.var_vbex_dn3, locals.var_vbex_dn4, locals.var_vbex_dn5, locals.var_vbex_dn6, locals.var_vbex_dn7, locals.var_vbex_dn8, locals.var_vbex_dn9, locals.var_vbex_dn10, locals.var_vbex_dn11,)
    }
};
        locals.var_vbex = assign5030_e4993;
        locals.var_vbex_dn0 = assign5030_e4993_d_n0;
        locals.var_vbex_dn1 = assign5030_e4993_d_n1;
        locals.var_vbex_dn3 = assign5030_e4993_d_n3;
        locals.var_vbex_dn4 = assign5030_e4993_d_n4;
        locals.var_vbex_dn5 = assign5030_e4993_d_n5;
        locals.var_vbex_dn6 = assign5030_e4993_d_n6;
        locals.var_vbex_dn7 = assign5030_e4993_d_n7;
        locals.var_vbex_dn8 = assign5030_e4993_d_n8;
        locals.var_vbex_dn9 = assign5030_e4993_d_n9;
        locals.var_vbex_dn10 = assign5030_e4993_d_n10;
        locals.var_vbex_dn11 = assign5030_e4993_d_n11;
        locals.var_vbex_rv = 0.0;

        let (assign5040_e5009, assign5040_e5009_d_n0, assign5040_e5009_d_n1, assign5040_e5009_d_n3, assign5040_e5009_d_n4, assign5040_e5009_d_n5, assign5040_e5009_d_n6, assign5040_e5009_d_n7, assign5040_e5009_d_n8, assign5040_e5009_d_n9, assign5040_e5009_d_n10, assign5040_e5009_d_n11,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign5040_e5001: f64 = (locals.var_ximex + locals.var_ximsub);
        let assign5040_e5003: f64 = (assign5040_e5001 * locals.var_rcc_xx_t);
        let assign5040_e5004: f64 = (locals.var_vex_bias + assign5040_e5003);
        let assign5040_e5006: f64 = (assign5040_e5004 + locals.var_vbex);
        let assign5040_e5007: f64 = (locals.var_vbex / assign5040_e5006);
        (assign5040_e5007, (((locals.var_vbex_dn0 * assign5040_e5006) - (locals.var_vbex * (((locals.var_ximex_dn0 + locals.var_ximsub_dn0) * locals.var_rcc_xx_t) + locals.var_vbex_dn0))) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn1 * assign5040_e5006) - (locals.var_vbex * (((locals.var_ximex_dn1 + locals.var_ximsub_dn1) * locals.var_rcc_xx_t) + locals.var_vbex_dn1))) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn3 * assign5040_e5006) - (locals.var_vbex * ((locals.var_ximsub_dn3 * locals.var_rcc_xx_t) + locals.var_vbex_dn3))) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn4 * assign5040_e5006) - (locals.var_vbex * ((locals.var_vex_bias_dn4 + (((locals.var_ximex_dn4 + locals.var_ximsub_dn4) * locals.var_rcc_xx_t) + (assign5040_e5001 * locals.var_rcc_xx_t_dn4))) + locals.var_vbex_dn4))) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn5 * assign5040_e5006) - (locals.var_vbex * locals.var_vbex_dn5)) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn6 * assign5040_e5006) - (locals.var_vbex * (((locals.var_ximex_dn6 + locals.var_ximsub_dn6) * locals.var_rcc_xx_t) + locals.var_vbex_dn6))) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn7 * assign5040_e5006) - (locals.var_vbex * (((locals.var_ximex_dn7 + locals.var_ximsub_dn7) * locals.var_rcc_xx_t) + locals.var_vbex_dn7))) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn8 * assign5040_e5006) - (locals.var_vbex * (((locals.var_ximex_dn8 + locals.var_ximsub_dn8) * locals.var_rcc_xx_t) + locals.var_vbex_dn8))) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn9 * assign5040_e5006) - (locals.var_vbex * (((locals.var_ximex_dn9 + locals.var_ximsub_dn9) * locals.var_rcc_xx_t) + locals.var_vbex_dn9))) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn10 * assign5040_e5006) - (locals.var_vbex * (((locals.var_ximex_dn10 + locals.var_ximsub_dn10) * locals.var_rcc_xx_t) + locals.var_vbex_dn10))) / (assign5040_e5006 * assign5040_e5006)), (((locals.var_vbex_dn11 * assign5040_e5006) - (locals.var_vbex * (((locals.var_ximex_dn11 + locals.var_ximsub_dn11) * locals.var_rcc_xx_t) + locals.var_vbex_dn11))) / (assign5040_e5006 * assign5040_e5006)),)
    } else {
        (locals.var_fex, locals.var_fex_dn0, locals.var_fex_dn1, locals.var_fex_dn3, locals.var_fex_dn4, locals.var_fex_dn5, locals.var_fex_dn6, locals.var_fex_dn7, locals.var_fex_dn8, locals.var_fex_dn9, locals.var_fex_dn10, locals.var_fex_dn11,)
    }
};
        locals.var_fex = assign5040_e5009;
        locals.var_fex_dn0 = assign5040_e5009_d_n0;
        locals.var_fex_dn1 = assign5040_e5009_d_n1;
        locals.var_fex_dn3 = assign5040_e5009_d_n3;
        locals.var_fex_dn4 = assign5040_e5009_d_n4;
        locals.var_fex_dn5 = assign5040_e5009_d_n5;
        locals.var_fex_dn6 = assign5040_e5009_d_n6;
        locals.var_fex_dn7 = assign5040_e5009_d_n7;
        locals.var_fex_dn8 = assign5040_e5009_d_n8;
        locals.var_fex_dn9 = assign5040_e5009_d_n9;
        locals.var_fex_dn10 = assign5040_e5009_d_n10;
        locals.var_fex_dn11 = assign5040_e5009_d_n11;
        locals.var_fex_rv = 0.0;

        let (assign5050_e5016, assign5050_e5016_d_n4,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 == 0.0)) {
        (0.0, 0.0,)
    } else {
        (locals.var_vex, locals.var_vex_dn4,)
    }
};
        locals.var_vex = assign5050_e5016;
        locals.var_vex_dn4 = assign5050_e5016_d_n4;
        locals.var_vex_rv = 0.0;

        let (assign5060_e5023, assign5060_e5023_d_n0, assign5060_e5023_d_n1, assign5060_e5023_d_n4, assign5060_e5023_d_n6, assign5060_e5023_d_n7, assign5060_e5023_d_n8, assign5060_e5023_d_n9, assign5060_e5023_d_n10, assign5060_e5023_d_n11,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vdif, locals.var_vdif_dn0, locals.var_vdif_dn1, locals.var_vdif_dn4, locals.var_vdif_dn6, locals.var_vdif_dn7, locals.var_vdif_dn8, locals.var_vdif_dn9, locals.var_vdif_dn10, locals.var_vdif_dn11,)
    }
};
        locals.var_vdif = assign5060_e5023;
        locals.var_vdif_dn0 = assign5060_e5023_d_n0;
        locals.var_vdif_dn1 = assign5060_e5023_d_n1;
        locals.var_vdif_dn4 = assign5060_e5023_d_n4;
        locals.var_vdif_dn6 = assign5060_e5023_d_n6;
        locals.var_vdif_dn7 = assign5060_e5023_d_n7;
        locals.var_vdif_dn8 = assign5060_e5023_d_n8;
        locals.var_vdif_dn9 = assign5060_e5023_d_n9;
        locals.var_vdif_dn10 = assign5060_e5023_d_n10;
        locals.var_vdif_dn11 = assign5060_e5023_d_n11;
        locals.var_vdif_rv = 0.0;

        let (assign5070_e5030, assign5070_e5030_d_n0, assign5070_e5030_d_n1, assign5070_e5030_d_n3, assign5070_e5030_d_n4, assign5070_e5030_d_n5, assign5070_e5030_d_n6, assign5070_e5030_d_n7, assign5070_e5030_d_n8, assign5070_e5030_d_n9, assign5070_e5030_d_n10, assign5070_e5030_d_n11,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbex, locals.var_vbex_dn0, locals.var_vbex_dn1, locals.var_vbex_dn3, locals.var_vbex_dn4, locals.var_vbex_dn5, locals.var_vbex_dn6, locals.var_vbex_dn7, locals.var_vbex_dn8, locals.var_vbex_dn9, locals.var_vbex_dn10, locals.var_vbex_dn11,)
    }
};
        locals.var_vbex = assign5070_e5030;
        locals.var_vbex_dn0 = assign5070_e5030_d_n0;
        locals.var_vbex_dn1 = assign5070_e5030_d_n1;
        locals.var_vbex_dn3 = assign5070_e5030_d_n3;
        locals.var_vbex_dn4 = assign5070_e5030_d_n4;
        locals.var_vbex_dn5 = assign5070_e5030_d_n5;
        locals.var_vbex_dn6 = assign5070_e5030_d_n6;
        locals.var_vbex_dn7 = assign5070_e5030_d_n7;
        locals.var_vbex_dn8 = assign5070_e5030_d_n8;
        locals.var_vbex_dn9 = assign5070_e5030_d_n9;
        locals.var_vbex_dn10 = assign5070_e5030_d_n10;
        locals.var_vbex_dn11 = assign5070_e5030_d_n11;
        locals.var_vbex_rv = 0.0;

        let (assign5080_e5037, assign5080_e5037_d_n0, assign5080_e5037_d_n1, assign5080_e5037_d_n3, assign5080_e5037_d_n4, assign5080_e5037_d_n5, assign5080_e5037_d_n6, assign5080_e5037_d_n7, assign5080_e5037_d_n8, assign5080_e5037_d_n9, assign5080_e5037_d_n10, assign5080_e5037_d_n11,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fex, locals.var_fex_dn0, locals.var_fex_dn1, locals.var_fex_dn3, locals.var_fex_dn4, locals.var_fex_dn5, locals.var_fex_dn6, locals.var_fex_dn7, locals.var_fex_dn8, locals.var_fex_dn9, locals.var_fex_dn10, locals.var_fex_dn11,)
    }
};
        locals.var_fex = assign5080_e5037;
        locals.var_fex_dn0 = assign5080_e5037_d_n0;
        locals.var_fex_dn1 = assign5080_e5037_d_n1;
        locals.var_fex_dn3 = assign5080_e5037_d_n3;
        locals.var_fex_dn4 = assign5080_e5037_d_n4;
        locals.var_fex_dn5 = assign5080_e5037_d_n5;
        locals.var_fex_dn6 = assign5080_e5037_d_n6;
        locals.var_fex_dn7 = assign5080_e5037_d_n7;
        locals.var_fex_dn8 = assign5080_e5037_d_n8;
        locals.var_fex_dn9 = assign5080_e5037_d_n9;
        locals.var_fex_dn10 = assign5080_e5037_d_n10;
        locals.var_fex_dn11 = assign5080_e5037_d_n11;
        locals.var_fex_rv = 0.0;

        let assign5110_e5052: f64 = if p.p84 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard87 = assign5110_e5052;
        locals.var_guard87_rv = 0.0;

        let (assign5120_e5058, assign5120_e5058_d_n6, assign5120_e5058_d_n7, assign5120_e5058_d_n8,) = {
    if (locals.var_guard87 != 0.0) {
        let assign5120_e5056: f64 = (locals.var_vb1b2 + locals.var_vb2c1);
        (assign5120_e5056, locals.var_vb1b2_dn6, (locals.var_vb1b2_dn7 + locals.var_vb2c1_dn7), locals.var_vb2c1_dn8,)
    } else {
        (locals.var_vb1c1, locals.var_vb1c1_dn6, locals.var_vb1c1_dn7, locals.var_vb1c1_dn8,)
    }
};
        locals.var_vb1c1 = assign5120_e5058;
        locals.var_vb1c1_dn6 = assign5120_e5058_d_n6;
        locals.var_vb1c1_dn7 = assign5120_e5058_d_n7;
        locals.var_vb1c1_dn8 = assign5120_e5058_d_n8;
        locals.var_vb1c1_rv = 0.0;

        let (assign5130_e5064, assign5130_e5064_d_n0, assign5130_e5064_d_n1, assign5130_e5064_d_n3, assign5130_e5064_d_n4, assign5130_e5064_d_n5, assign5130_e5064_d_n6, assign5130_e5064_d_n7, assign5130_e5064_d_n8, assign5130_e5064_d_n9, assign5130_e5064_d_n10, assign5130_e5064_d_n11,) = {
    if (locals.var_guard87 != 0.0) {
        let assign5130_e5062: f64 = (1e-6 * 1e-6);
        (assign5130_e5062, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eps2, locals.var_eps2_dn0, locals.var_eps2_dn1, locals.var_eps2_dn3, locals.var_eps2_dn4, locals.var_eps2_dn5, locals.var_eps2_dn6, locals.var_eps2_dn7, locals.var_eps2_dn8, locals.var_eps2_dn9, locals.var_eps2_dn10, locals.var_eps2_dn11,)
    }
};
        locals.var_eps2 = assign5130_e5064;
        locals.var_eps2_dn0 = assign5130_e5064_d_n0;
        locals.var_eps2_dn1 = assign5130_e5064_d_n1;
        locals.var_eps2_dn3 = assign5130_e5064_d_n3;
        locals.var_eps2_dn4 = assign5130_e5064_d_n4;
        locals.var_eps2_dn5 = assign5130_e5064_d_n5;
        locals.var_eps2_dn6 = assign5130_e5064_d_n6;
        locals.var_eps2_dn7 = assign5130_e5064_d_n7;
        locals.var_eps2_dn8 = assign5130_e5064_d_n8;
        locals.var_eps2_dn9 = assign5130_e5064_d_n9;
        locals.var_eps2_dn10 = assign5130_e5064_d_n10;
        locals.var_eps2_dn11 = assign5130_e5064_d_n11;
        locals.var_eps2_rv = 0.0;

        let (assign5140_e5076, assign5140_e5076_d_n0, assign5140_e5076_d_n1, assign5140_e5076_d_n3, assign5140_e5076_d_n4, assign5140_e5076_d_n5, assign5140_e5076_d_n6, assign5140_e5076_d_n7, assign5140_e5076_d_n8, assign5140_e5076_d_n9, assign5140_e5076_d_n10, assign5140_e5076_d_n11,) = {
    if (locals.var_guard87 != 0.0) {
        let assign5140_e5067: f64 = (-1.0);
        let assign5140_e5069: f64 = (assign5140_e5067 * locals.var_vb1c1);
        let assign5140_e5071: f64 = (-1.0);
        let assign5140_e5072: f64 = (assign5140_e5069 * assign5140_e5071);
        let assign5140_e5074: f64 = (assign5140_e5072 * locals.var_vb1c1);
        (assign5140_e5074, 0.0, 0.0, 0.0, 0.0, 0.0, ((((assign5140_e5067 * locals.var_vb1c1_dn6) * assign5140_e5071) * locals.var_vb1c1) + (assign5140_e5072 * locals.var_vb1c1_dn6)), ((((assign5140_e5067 * locals.var_vb1c1_dn7) * assign5140_e5071) * locals.var_vb1c1) + (assign5140_e5072 * locals.var_vb1c1_dn7)), ((((assign5140_e5067 * locals.var_vb1c1_dn8) * assign5140_e5071) * locals.var_vb1c1) + (assign5140_e5072 * locals.var_vb1c1_dn8)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn1, locals.var_x2_dn3, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11,)
    }
};
        locals.var_x2 = assign5140_e5076;
        locals.var_x2_dn0 = assign5140_e5076_d_n0;
        locals.var_x2_dn1 = assign5140_e5076_d_n1;
        locals.var_x2_dn3 = assign5140_e5076_d_n3;
        locals.var_x2_dn4 = assign5140_e5076_d_n4;
        locals.var_x2_dn5 = assign5140_e5076_d_n5;
        locals.var_x2_dn6 = assign5140_e5076_d_n6;
        locals.var_x2_dn7 = assign5140_e5076_d_n7;
        locals.var_x2_dn8 = assign5140_e5076_d_n8;
        locals.var_x2_dn9 = assign5140_e5076_d_n9;
        locals.var_x2_dn10 = assign5140_e5076_d_n10;
        locals.var_x2_dn11 = assign5140_e5076_d_n11;
        locals.var_x2_rv = 0.0;

        let assign5290_e5200: f64 = (locals.var_vte / locals.var_ver_t);
        let assign5290_e5201: f64 = (1.0 + assign5290_e5200);
        let assign5290_e5204: f64 = (locals.var_vtc / locals.var_vef_t);
        let assign5290_e5205: f64 = (assign5290_e5201 + assign5290_e5204);
        locals.var_q0q = assign5290_e5205;
        locals.var_q0q_dn0 = ((((locals.var_vte_dn0 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn0)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn0 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn0)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn1 = ((((locals.var_vte_dn1 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn1)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn1 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn1)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn3 = ((((locals.var_vte_dn3 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn3)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn3 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn3)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn4 = ((((locals.var_vte_dn4 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn4)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn4 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn4)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn5 = ((((locals.var_vte_dn5 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn5)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn5 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn5)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn6 = ((((locals.var_vte_dn6 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn6)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn6 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn6)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn7 = ((((locals.var_vte_dn7 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn7)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn7 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn7)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn8 = ((((locals.var_vte_dn8 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn8)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn8 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn8)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn9 = ((((locals.var_vte_dn9 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn9)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn9 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn9)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn10 = ((((locals.var_vte_dn10 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn10)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn10 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn10)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_dn11 = ((((locals.var_vte_dn11 * locals.var_ver_t) - (locals.var_vte * locals.var_ver_t_dn11)) / (locals.var_ver_t * locals.var_ver_t)) + (((locals.var_vtc_dn11 * locals.var_vef_t) - (locals.var_vtc * locals.var_vef_t_dn11)) / (locals.var_vef_t * locals.var_vef_t)));
        locals.var_q0q_rv = 0.0;

        let assign5300_e5208: f64 = (0.1 * 0.1);
        locals.var_eps2 = assign5300_e5208;
        locals.var_eps2_dn0 = 0.0;
        locals.var_eps2_dn1 = 0.0;
        locals.var_eps2_dn3 = 0.0;
        locals.var_eps2_dn4 = 0.0;
        locals.var_eps2_dn5 = 0.0;
        locals.var_eps2_dn6 = 0.0;
        locals.var_eps2_dn7 = 0.0;
        locals.var_eps2_dn8 = 0.0;
        locals.var_eps2_dn9 = 0.0;
        locals.var_eps2_dn10 = 0.0;
        locals.var_eps2_dn11 = 0.0;
        locals.var_eps2_rv = 0.0;

        let assign5310_e5211: f64 = (locals.var_q0q * locals.var_q0q);
        locals.var_x2 = assign5310_e5211;
        locals.var_x2_dn0 = ((locals.var_q0q_dn0 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn0));
        locals.var_x2_dn1 = ((locals.var_q0q_dn1 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn1));
        locals.var_x2_dn3 = ((locals.var_q0q_dn3 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn3));
        locals.var_x2_dn4 = ((locals.var_q0q_dn4 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn4));
        locals.var_x2_dn5 = ((locals.var_q0q_dn5 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn5));
        locals.var_x2_dn6 = ((locals.var_q0q_dn6 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn6));
        locals.var_x2_dn7 = ((locals.var_q0q_dn7 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn7));
        locals.var_x2_dn8 = ((locals.var_q0q_dn8 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn8));
        locals.var_x2_dn9 = ((locals.var_q0q_dn9 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn9));
        locals.var_x2_dn10 = ((locals.var_q0q_dn10 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn10));
        locals.var_x2_dn11 = ((locals.var_q0q_dn11 * locals.var_q0q) + (locals.var_q0q * locals.var_q0q_dn11));
        locals.var_x2_rv = 0.0;

        let assign5320_e5214: f64 = if locals.var_q0q < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard90 = assign5320_e5214;
        locals.var_guard90_rv = 0.0;

        let (assign5330_e5227, assign5330_e5227_d_n0, assign5330_e5227_d_n1, assign5330_e5227_d_n3, assign5330_e5227_d_n4, assign5330_e5227_d_n5, assign5330_e5227_d_n6, assign5330_e5227_d_n7, assign5330_e5227_d_n8, assign5330_e5227_d_n9, assign5330_e5227_d_n10, assign5330_e5227_d_n11,) = {
    if (locals.var_guard90 != 0.0) {
        let assign5330_e5218: f64 = (0.5 * locals.var_eps2);
        let assign5330_e5221: f64 = (locals.var_x2 + locals.var_eps2);
        let assign5330_e5222: f64 = (assign5330_e5221).sqrt();
        let assign5330_e5224: f64 = (assign5330_e5222 - locals.var_q0q);
        let assign5330_e5225: f64 = (assign5330_e5218 / assign5330_e5224);
        (assign5330_e5225, ((((0.5 * locals.var_eps2_dn0) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn0))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn1) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn1))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn3) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn3))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn4) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn4))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn5) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn5))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn6) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn6))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn7) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn7))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn8) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn8))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn9) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn9))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn10) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn10))) / (assign5330_e5224 * assign5330_e5224)), ((((0.5 * locals.var_eps2_dn11) * assign5330_e5224) - (assign5330_e5218 * (((locals.var_x2_dn11 + locals.var_eps2_dn11) / (2.0 * assign5330_e5222)) - locals.var_q0q_dn11))) / (assign5330_e5224 * assign5330_e5224)),)
    } else {
        (locals.var_q1q, locals.var_q1q_dn0, locals.var_q1q_dn1, locals.var_q1q_dn3, locals.var_q1q_dn4, locals.var_q1q_dn5, locals.var_q1q_dn6, locals.var_q1q_dn7, locals.var_q1q_dn8, locals.var_q1q_dn9, locals.var_q1q_dn10, locals.var_q1q_dn11,)
    }
};
        locals.var_q1q = assign5330_e5227;
        locals.var_q1q_dn0 = assign5330_e5227_d_n0;
        locals.var_q1q_dn1 = assign5330_e5227_d_n1;
        locals.var_q1q_dn3 = assign5330_e5227_d_n3;
        locals.var_q1q_dn4 = assign5330_e5227_d_n4;
        locals.var_q1q_dn5 = assign5330_e5227_d_n5;
        locals.var_q1q_dn6 = assign5330_e5227_d_n6;
        locals.var_q1q_dn7 = assign5330_e5227_d_n7;
        locals.var_q1q_dn8 = assign5330_e5227_d_n8;
        locals.var_q1q_dn9 = assign5330_e5227_d_n9;
        locals.var_q1q_dn10 = assign5330_e5227_d_n10;
        locals.var_q1q_dn11 = assign5330_e5227_d_n11;
        locals.var_q1q_rv = 0.0;

        let (assign5340_e5239, assign5340_e5239_d_n0, assign5340_e5239_d_n1, assign5340_e5239_d_n3, assign5340_e5239_d_n4, assign5340_e5239_d_n5, assign5340_e5239_d_n6, assign5340_e5239_d_n7, assign5340_e5239_d_n8, assign5340_e5239_d_n9, assign5340_e5239_d_n10, assign5340_e5239_d_n11,) = {
    if (locals.var_guard90 == 0.0) {
        let assign5340_e5233: f64 = (locals.var_x2 + locals.var_eps2);
        let assign5340_e5234: f64 = (assign5340_e5233).sqrt();
        let assign5340_e5236: f64 = (assign5340_e5234 + locals.var_q0q);
        let assign5340_e5237: f64 = (0.5 * assign5340_e5236);
        (assign5340_e5237, (0.5 * (((locals.var_x2_dn0 + locals.var_eps2_dn0) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn0)), (0.5 * (((locals.var_x2_dn1 + locals.var_eps2_dn1) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn1)), (0.5 * (((locals.var_x2_dn3 + locals.var_eps2_dn3) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn3)), (0.5 * (((locals.var_x2_dn4 + locals.var_eps2_dn4) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn4)), (0.5 * (((locals.var_x2_dn5 + locals.var_eps2_dn5) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn5)), (0.5 * (((locals.var_x2_dn6 + locals.var_eps2_dn6) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn6)), (0.5 * (((locals.var_x2_dn7 + locals.var_eps2_dn7) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn7)), (0.5 * (((locals.var_x2_dn8 + locals.var_eps2_dn8) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn8)), (0.5 * (((locals.var_x2_dn9 + locals.var_eps2_dn9) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn9)), (0.5 * (((locals.var_x2_dn10 + locals.var_eps2_dn10) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn10)), (0.5 * (((locals.var_x2_dn11 + locals.var_eps2_dn11) / (2.0 * assign5340_e5234)) + locals.var_q0q_dn11)),)
    } else {
        (locals.var_q1q, locals.var_q1q_dn0, locals.var_q1q_dn1, locals.var_q1q_dn3, locals.var_q1q_dn4, locals.var_q1q_dn5, locals.var_q1q_dn6, locals.var_q1q_dn7, locals.var_q1q_dn8, locals.var_q1q_dn9, locals.var_q1q_dn10, locals.var_q1q_dn11,)
    }
};
        locals.var_q1q = assign5340_e5239;
        locals.var_q1q_dn0 = assign5340_e5239_d_n0;
        locals.var_q1q_dn1 = assign5340_e5239_d_n1;
        locals.var_q1q_dn3 = assign5340_e5239_d_n3;
        locals.var_q1q_dn4 = assign5340_e5239_d_n4;
        locals.var_q1q_dn5 = assign5340_e5239_d_n5;
        locals.var_q1q_dn6 = assign5340_e5239_d_n6;
        locals.var_q1q_dn7 = assign5340_e5239_d_n7;
        locals.var_q1q_dn8 = assign5340_e5239_d_n8;
        locals.var_q1q_dn9 = assign5340_e5239_d_n9;
        locals.var_q1q_dn10 = assign5340_e5239_d_n10;
        locals.var_q1q_dn11 = assign5340_e5239_d_n11;
        locals.var_q1q_rv = 0.0;

        let assign5350_e5245: f64 = (locals.var_n0 + locals.var_nb);
        let assign5350_e5246: f64 = (0.5 * assign5350_e5245);
        let assign5350_e5247: f64 = (1.0 + assign5350_e5246);
        let assign5350_e5248: f64 = (locals.var_q1q * assign5350_e5247);
        locals.var_qbq = assign5350_e5248;
        locals.var_qbq_dn0 = ((locals.var_q1q_dn0 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn0 + locals.var_nb_dn0))));
        locals.var_qbq_dn1 = ((locals.var_q1q_dn1 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn1 + locals.var_nb_dn1))));
        locals.var_qbq_dn3 = ((locals.var_q1q_dn3 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn3 + locals.var_nb_dn3))));
        locals.var_qbq_dn4 = ((locals.var_q1q_dn4 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn4 + locals.var_nb_dn4))));
        locals.var_qbq_dn5 = ((locals.var_q1q_dn5 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn5 + locals.var_nb_dn5))));
        locals.var_qbq_dn6 = ((locals.var_q1q_dn6 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn6 + locals.var_nb_dn6))));
        locals.var_qbq_dn7 = ((locals.var_q1q_dn7 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn7 + locals.var_nb_dn7))));
        locals.var_qbq_dn8 = ((locals.var_q1q_dn8 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn8 + locals.var_nb_dn8))));
        locals.var_qbq_dn9 = ((locals.var_q1q_dn9 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn9 + locals.var_nb_dn9))));
        locals.var_qbq_dn10 = ((locals.var_q1q_dn10 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn10 + locals.var_nb_dn10))));
        locals.var_qbq_dn11 = ((locals.var_q1q_dn11 * assign5350_e5247) + (locals.var_q1q * (0.5 * (locals.var_n0_dn11 + locals.var_nb_dn11))));
        locals.var_qbq_rv = 0.0;

        let assign5360_e5251: f64 = (locals.var_rbv_t / locals.var_qbq);
        locals.var_rbvtemp = assign5360_e5251;
        locals.var_rbvtemp_dn0 = (-((locals.var_rbv_t * locals.var_qbq_dn0) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn1 = (-((locals.var_rbv_t * locals.var_qbq_dn1) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn3 = (-((locals.var_rbv_t * locals.var_qbq_dn3) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn4 = (((locals.var_rbv_t_dn4 * locals.var_qbq) - (locals.var_rbv_t * locals.var_qbq_dn4)) / (locals.var_qbq * locals.var_qbq));
        locals.var_rbvtemp_dn5 = (-((locals.var_rbv_t * locals.var_qbq_dn5) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn6 = (-((locals.var_rbv_t * locals.var_qbq_dn6) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn7 = (-((locals.var_rbv_t * locals.var_qbq_dn7) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn8 = (-((locals.var_rbv_t * locals.var_qbq_dn8) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn9 = (-((locals.var_rbv_t * locals.var_qbq_dn9) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn10 = (-((locals.var_rbv_t * locals.var_qbq_dn10) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_dn11 = (-((locals.var_rbv_t * locals.var_qbq_dn11) / (locals.var_qbq * locals.var_qbq)));
        locals.var_rbvtemp_rv = 0.0;

        let assign5370_e5254: f64 = if locals.var_rbvtemp < locals.var_minr_m { 1.0 } else { 0.0 };
        locals.var_guard91 = assign5370_e5254;
        locals.var_guard91_rv = 0.0;

        let (assign5380_e5258, assign5380_e5258_d_n0, assign5380_e5258_d_n1, assign5380_e5258_d_n3, assign5380_e5258_d_n4, assign5380_e5258_d_n5, assign5380_e5258_d_n6, assign5380_e5258_d_n7, assign5380_e5258_d_n8, assign5380_e5258_d_n9, assign5380_e5258_d_n10, assign5380_e5258_d_n11,) = {
    if (locals.var_guard91 != 0.0) {
        (locals.var_minr_m, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rbvtemp, locals.var_rbvtemp_dn0, locals.var_rbvtemp_dn1, locals.var_rbvtemp_dn3, locals.var_rbvtemp_dn4, locals.var_rbvtemp_dn5, locals.var_rbvtemp_dn6, locals.var_rbvtemp_dn7, locals.var_rbvtemp_dn8, locals.var_rbvtemp_dn9, locals.var_rbvtemp_dn10, locals.var_rbvtemp_dn11,)
    }
};
        locals.var_rbvtemp = assign5380_e5258;
        locals.var_rbvtemp_dn0 = assign5380_e5258_d_n0;
        locals.var_rbvtemp_dn1 = assign5380_e5258_d_n1;
        locals.var_rbvtemp_dn3 = assign5380_e5258_d_n3;
        locals.var_rbvtemp_dn4 = assign5380_e5258_d_n4;
        locals.var_rbvtemp_dn5 = assign5380_e5258_d_n5;
        locals.var_rbvtemp_dn6 = assign5380_e5258_d_n6;
        locals.var_rbvtemp_dn7 = assign5380_e5258_d_n7;
        locals.var_rbvtemp_dn8 = assign5380_e5258_d_n8;
        locals.var_rbvtemp_dn9 = assign5380_e5258_d_n9;
        locals.var_rbvtemp_dn10 = assign5380_e5258_d_n10;
        locals.var_rbvtemp_dn11 = assign5380_e5258_d_n11;
        locals.var_rbvtemp_rv = 0.0;

        let assign5390_e5261: f64 = (3.0 * locals.var_rbvtemp);
        locals.var_rb2 = assign5390_e5261;
        locals.var_rb2_dn0 = (3.0 * locals.var_rbvtemp_dn0);
        locals.var_rb2_dn1 = (3.0 * locals.var_rbvtemp_dn1);
        locals.var_rb2_dn3 = (3.0 * locals.var_rbvtemp_dn3);
        locals.var_rb2_dn4 = (3.0 * locals.var_rbvtemp_dn4);
        locals.var_rb2_dn5 = (3.0 * locals.var_rbvtemp_dn5);
        locals.var_rb2_dn6 = (3.0 * locals.var_rbvtemp_dn6);
        locals.var_rb2_dn7 = (3.0 * locals.var_rbvtemp_dn7);
        locals.var_rb2_dn8 = (3.0 * locals.var_rbvtemp_dn8);
        locals.var_rb2_dn9 = (3.0 * locals.var_rbvtemp_dn9);
        locals.var_rb2_dn10 = (3.0 * locals.var_rbvtemp_dn10);
        locals.var_rb2_dn11 = (3.0 * locals.var_rbvtemp_dn11);
        locals.var_rb2_rv = 0.0;

        let assign5410_e5275: f64 = if locals.var_in_ > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard92 = assign5410_e5275;
        locals.var_guard92_rv = 0.0;

        let assign5420_e5278: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign5420_e5278;
        locals.var_guard93_rv = 0.0;

        let assign5430_e5281: f64 = if locals.var_vb2c1 < p.p44 { 1.0 } else { 0.0 };
        locals.var_guard94 = assign5430_e5281;
        locals.var_guard94_rv = 0.0;

        let assign5440_e5283: f64 = (-locals.var_in_);
        let assign5440_e5285: f64 = (assign5440_e5283 / p.p42);
        let assign5440_e5287: f64 = if assign5440_e5285 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign5440_e5287;
        locals.var_guard95_rv = 0.0;

        let (assign5450_e5301, assign5450_e5301_d_n0, assign5450_e5301_d_n1, assign5450_e5301_d_n3, assign5450_e5301_d_n4, assign5450_e5301_d_n5, assign5450_e5301_d_n6, assign5450_e5301_d_n7, assign5450_e5301_d_n8, assign5450_e5301_d_n9, assign5450_e5301_d_n10, assign5450_e5301_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard95 != 0.0)) {
        let assign5450_e5296: f64 = (-locals.var_in_);
        let assign5450_e5298: f64 = (assign5450_e5296 / p.p42);
        let assign5450_e5299: f64 = (assign5450_e5298).exp();
        (assign5450_e5299, (assign5450_e5299 * ((-locals.var_in__dn0) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn1) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn3) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn4) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn5) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn6) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn7) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn8) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn9) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn10) / p.p42)), (assign5450_e5299 * ((-locals.var_in__dn11) / p.p42)),)
    } else {
        (locals.var_expin, locals.var_expin_dn0, locals.var_expin_dn1, locals.var_expin_dn3, locals.var_expin_dn4, locals.var_expin_dn5, locals.var_expin_dn6, locals.var_expin_dn7, locals.var_expin_dn8, locals.var_expin_dn9, locals.var_expin_dn10, locals.var_expin_dn11,)
    }
};
        locals.var_expin = assign5450_e5301;
        locals.var_expin_dn0 = assign5450_e5301_d_n0;
        locals.var_expin_dn1 = assign5450_e5301_d_n1;
        locals.var_expin_dn3 = assign5450_e5301_d_n3;
        locals.var_expin_dn4 = assign5450_e5301_d_n4;
        locals.var_expin_dn5 = assign5450_e5301_d_n5;
        locals.var_expin_dn6 = assign5450_e5301_d_n6;
        locals.var_expin_dn7 = assign5450_e5301_d_n7;
        locals.var_expin_dn8 = assign5450_e5301_d_n8;
        locals.var_expin_dn9 = assign5450_e5301_d_n9;
        locals.var_expin_dn10 = assign5450_e5301_d_n10;
        locals.var_expin_dn11 = assign5450_e5301_d_n11;
        locals.var_expin_rv = 0.0;

        let (assign5460_e5313,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard95 == 0.0)) {
        let assign5460_e5311: f64 = (p.p151).exp();
        (assign5460_e5311,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign5460_e5313;
        locals.var_expl_rv = 0.0;

        let (assign5470_e5333, assign5470_e5333_d_n0, assign5470_e5333_d_n1, assign5470_e5333_d_n3, assign5470_e5333_d_n4, assign5470_e5333_d_n5, assign5470_e5333_d_n6, assign5470_e5333_d_n7, assign5470_e5333_d_n8, assign5470_e5333_d_n9, assign5470_e5333_d_n10, assign5470_e5333_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard95 == 0.0)) {
        let assign5470_e5325: f64 = (-locals.var_in_);
        let assign5470_e5327: f64 = (assign5470_e5325 / p.p42);
        let assign5470_e5329: f64 = (assign5470_e5327 - p.p151);
        let assign5470_e5330: f64 = (1.0 + assign5470_e5329);
        let assign5470_e5331: f64 = (locals.var_expl * assign5470_e5330);
        (assign5470_e5331, (locals.var_expl * ((-locals.var_in__dn0) / p.p42)), (locals.var_expl * ((-locals.var_in__dn1) / p.p42)), (locals.var_expl * ((-locals.var_in__dn3) / p.p42)), (locals.var_expl * ((-locals.var_in__dn4) / p.p42)), (locals.var_expl * ((-locals.var_in__dn5) / p.p42)), (locals.var_expl * ((-locals.var_in__dn6) / p.p42)), (locals.var_expl * ((-locals.var_in__dn7) / p.p42)), (locals.var_expl * ((-locals.var_in__dn8) / p.p42)), (locals.var_expl * ((-locals.var_in__dn9) / p.p42)), (locals.var_expl * ((-locals.var_in__dn10) / p.p42)), (locals.var_expl * ((-locals.var_in__dn11) / p.p42)),)
    } else {
        (locals.var_expin, locals.var_expin_dn0, locals.var_expin_dn1, locals.var_expin_dn3, locals.var_expin_dn4, locals.var_expin_dn5, locals.var_expin_dn6, locals.var_expin_dn7, locals.var_expin_dn8, locals.var_expin_dn9, locals.var_expin_dn10, locals.var_expin_dn11,)
    }
};
        locals.var_expin = assign5470_e5333;
        locals.var_expin_dn0 = assign5470_e5333_d_n0;
        locals.var_expin_dn1 = assign5470_e5333_d_n1;
        locals.var_expin_dn3 = assign5470_e5333_d_n3;
        locals.var_expin_dn4 = assign5470_e5333_d_n4;
        locals.var_expin_dn5 = assign5470_e5333_d_n5;
        locals.var_expin_dn6 = assign5470_e5333_d_n6;
        locals.var_expin_dn7 = assign5470_e5333_d_n7;
        locals.var_expin_dn8 = assign5470_e5333_d_n8;
        locals.var_expin_dn9 = assign5470_e5333_d_n9;
        locals.var_expin_dn10 = assign5470_e5333_d_n10;
        locals.var_expin_dn11 = assign5470_e5333_d_n11;
        locals.var_expin_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5480_e5345, assign5480_e5345_d_n0, assign5480_e5345_d_n1, assign5480_e5345_d_n3, assign5480_e5345_d_n4, assign5480_e5345_d_n5, assign5480_e5345_d_n6, assign5480_e5345_d_n7, assign5480_e5345_d_n8, assign5480_e5345_d_n9, assign5480_e5345_d_n10, assign5480_e5345_d_n11,) = {
    if (((locals.var_guard92 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) {
        let assign5480_e5341: f64 = (p.p44 - locals.var_vb2c1);
        let assign5480_e5343: f64 = (assign5480_e5341 * locals.var_expin);
        (assign5480_e5343, (assign5480_e5341 * locals.var_expin_dn0), (assign5480_e5341 * locals.var_expin_dn1), (assign5480_e5341 * locals.var_expin_dn3), (assign5480_e5341 * locals.var_expin_dn4), (assign5480_e5341 * locals.var_expin_dn5), (assign5480_e5341 * locals.var_expin_dn6), (((-locals.var_vb2c1_dn7) * locals.var_expin) + (assign5480_e5341 * locals.var_expin_dn7)), (((-locals.var_vb2c1_dn8) * locals.var_expin) + (assign5480_e5341 * locals.var_expin_dn8)), (assign5480_e5341 * locals.var_expin_dn9), (assign5480_e5341 * locals.var_expin_dn10), (assign5480_e5341 * locals.var_expin_dn11),)
    } else {
        (locals.var_vl, locals.var_vl_dn0, locals.var_vl_dn1, locals.var_vl_dn3, locals.var_vl_dn4, locals.var_vl_dn5, locals.var_vl_dn6, locals.var_vl_dn7, locals.var_vl_dn8, locals.var_vl_dn9, locals.var_vl_dn10, locals.var_vl_dn11,)
    }
};
        locals.var_vl = assign5480_e5345;
        locals.var_vl_dn0 = assign5480_e5345_d_n0;
        locals.var_vl_dn1 = assign5480_e5345_d_n1;
        locals.var_vl_dn3 = assign5480_e5345_d_n3;
        locals.var_vl_dn4 = assign5480_e5345_d_n4;
        locals.var_vl_dn5 = assign5480_e5345_d_n5;
        locals.var_vl_dn6 = assign5480_e5345_d_n6;
        locals.var_vl_dn7 = assign5480_e5345_d_n7;
        locals.var_vl_dn8 = assign5480_e5345_d_n8;
        locals.var_vl_dn9 = assign5480_e5345_d_n9;
        locals.var_vl_dn10 = assign5480_e5345_d_n10;
        locals.var_vl_dn11 = assign5480_e5345_d_n11;
        locals.var_vl_rv = 0.0;

        let assign5490_e5347: f64 = (-locals.var_bavl_t);
        let assign5490_e5350: f64 = (locals.var_vl).powf(p.p41);
        let assign5490_e5351: f64 = (assign5490_e5347 * assign5490_e5350);
        let assign5490_e5353: f64 = if assign5490_e5351 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard96 = assign5490_e5353;
        locals.var_guard96_rv = 0.0;

        let (assign5500_e5369, assign5500_e5369_d_n0, assign5500_e5369_d_n1, assign5500_e5369_d_n3, assign5500_e5369_d_n4, assign5500_e5369_d_n5, assign5500_e5369_d_n6, assign5500_e5369_d_n7, assign5500_e5369_d_n8, assign5500_e5369_d_n9, assign5500_e5369_d_n10, assign5500_e5369_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard96 != 0.0)) {
        let assign5500_e5362: f64 = (-locals.var_bavl_t);
        let assign5500_e5365: f64 = (locals.var_vl).powf(p.p41);
        let assign5500_e5366: f64 = (assign5500_e5362 * assign5500_e5365);
        let assign5500_e5367: f64 = (assign5500_e5366).exp();
        (assign5500_e5367, (assign5500_e5367 * (((-locals.var_bavl_t_dn0) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn0)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn0 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn1) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn1)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn1 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn3) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn3)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn3 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn4) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn4)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn4 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn5) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn5)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn5 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn6) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn6)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn6 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn7) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn7)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn7 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn8) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn8)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn8 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn9) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn9)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn9 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn10) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn10)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn10 / locals.var_vl))) }))), (assign5500_e5367 * (((-locals.var_bavl_t_dn11) * assign5500_e5365) + (assign5500_e5362 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn11)) } } else { (assign5500_e5365 * (p.p41 * (locals.var_vl_dn11 / locals.var_vl))) }))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9, locals.var_expmm1_dn10, locals.var_expmm1_dn11,)
    }
};
        locals.var_expmm1 = assign5500_e5369;
        locals.var_expmm1_dn0 = assign5500_e5369_d_n0;
        locals.var_expmm1_dn1 = assign5500_e5369_d_n1;
        locals.var_expmm1_dn3 = assign5500_e5369_d_n3;
        locals.var_expmm1_dn4 = assign5500_e5369_d_n4;
        locals.var_expmm1_dn5 = assign5500_e5369_d_n5;
        locals.var_expmm1_dn6 = assign5500_e5369_d_n6;
        locals.var_expmm1_dn7 = assign5500_e5369_d_n7;
        locals.var_expmm1_dn8 = assign5500_e5369_d_n8;
        locals.var_expmm1_dn9 = assign5500_e5369_d_n9;
        locals.var_expmm1_dn10 = assign5500_e5369_d_n10;
        locals.var_expmm1_dn11 = assign5500_e5369_d_n11;
        locals.var_expmm1_rv = 0.0;

        let (assign5510_e5381,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard96 == 0.0)) {
        let assign5510_e5379: f64 = (p.p151).exp();
        (assign5510_e5379,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign5510_e5381;
        locals.var_expl_rv = 0.0;

        let (assign5520_e5403, assign5520_e5403_d_n0, assign5520_e5403_d_n1, assign5520_e5403_d_n3, assign5520_e5403_d_n4, assign5520_e5403_d_n5, assign5520_e5403_d_n6, assign5520_e5403_d_n7, assign5520_e5403_d_n8, assign5520_e5403_d_n9, assign5520_e5403_d_n10, assign5520_e5403_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) && (locals.var_guard96 == 0.0)) {
        let assign5520_e5393: f64 = (-locals.var_bavl_t);
        let assign5520_e5396: f64 = (locals.var_vl).powf(p.p41);
        let assign5520_e5397: f64 = (assign5520_e5393 * assign5520_e5396);
        let assign5520_e5399: f64 = (assign5520_e5397 - p.p151);
        let assign5520_e5400: f64 = (1.0 + assign5520_e5399);
        let assign5520_e5401: f64 = (locals.var_expl * assign5520_e5400);
        (assign5520_e5401, (locals.var_expl * (((-locals.var_bavl_t_dn0) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn0)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn0 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn1) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn1)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn1 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn3) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn3)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn3 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn4) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn4)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn4 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn5) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn5)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn5 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn6) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn6)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn6 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn7) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn7)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn7 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn8) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn8)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn8 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn9) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn9)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn9 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn10) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn10)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn10 / locals.var_vl))) }))), (locals.var_expl * (((-locals.var_bavl_t_dn11) * assign5520_e5396) + (assign5520_e5393 * if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((locals.var_vl).powf(p.p41 - 1.0) * locals.var_vl_dn11)) } } else { (assign5520_e5396 * (p.p41 * (locals.var_vl_dn11 / locals.var_vl))) }))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9, locals.var_expmm1_dn10, locals.var_expmm1_dn11,)
    }
};
        locals.var_expmm1 = assign5520_e5403;
        locals.var_expmm1_dn0 = assign5520_e5403_d_n0;
        locals.var_expmm1_dn1 = assign5520_e5403_d_n1;
        locals.var_expmm1_dn3 = assign5520_e5403_d_n3;
        locals.var_expmm1_dn4 = assign5520_e5403_d_n4;
        locals.var_expmm1_dn5 = assign5520_e5403_d_n5;
        locals.var_expmm1_dn6 = assign5520_e5403_d_n6;
        locals.var_expmm1_dn7 = assign5520_e5403_d_n7;
        locals.var_expmm1_dn8 = assign5520_e5403_d_n8;
        locals.var_expmm1_dn9 = assign5520_e5403_d_n9;
        locals.var_expmm1_dn10 = assign5520_e5403_d_n10;
        locals.var_expmm1_dn11 = assign5520_e5403_d_n11;
        locals.var_expmm1_rv = 0.0;

        let (assign5530_e5417, assign5530_e5417_d_n0, assign5530_e5417_d_n1, assign5530_e5417_d_n3, assign5530_e5417_d_n4, assign5530_e5417_d_n5, assign5530_e5417_d_n6, assign5530_e5417_d_n7, assign5530_e5417_d_n8, assign5530_e5417_d_n9, assign5530_e5417_d_n10, assign5530_e5417_d_n11,) = {
    if (((locals.var_guard92 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) {
        let assign5530_e5411: f64 = (p.p40 / locals.var_bavl_t);
        let assign5530_e5413: f64 = (assign5530_e5411 * locals.var_vl);
        let assign5530_e5415: f64 = (assign5530_e5413 * locals.var_expmm1);
        (assign5530_e5415, (((((-((p.p40 * locals.var_bavl_t_dn0) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn0)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn0)), (((((-((p.p40 * locals.var_bavl_t_dn1) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn1)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn1)), (((((-((p.p40 * locals.var_bavl_t_dn3) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn3)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn3)), (((((-((p.p40 * locals.var_bavl_t_dn4) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn4)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn4)), (((((-((p.p40 * locals.var_bavl_t_dn5) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn5)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn5)), (((((-((p.p40 * locals.var_bavl_t_dn6) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn6)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn6)), (((((-((p.p40 * locals.var_bavl_t_dn7) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn7)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn7)), (((((-((p.p40 * locals.var_bavl_t_dn8) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn8)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn8)), (((((-((p.p40 * locals.var_bavl_t_dn9) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn9)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn9)), (((((-((p.p40 * locals.var_bavl_t_dn10) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn10)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn10)), (((((-((p.p40 * locals.var_bavl_t_dn11) / (locals.var_bavl_t * locals.var_bavl_t))) * locals.var_vl) + (assign5530_e5411 * locals.var_vl_dn11)) * locals.var_expmm1) + (assign5530_e5413 * locals.var_expmm1_dn11)),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10, locals.var_gem_dn11,)
    }
};
        locals.var_gem = assign5530_e5417;
        locals.var_gem_dn0 = assign5530_e5417_d_n0;
        locals.var_gem_dn1 = assign5530_e5417_d_n1;
        locals.var_gem_dn3 = assign5530_e5417_d_n3;
        locals.var_gem_dn4 = assign5530_e5417_d_n4;
        locals.var_gem_dn5 = assign5530_e5417_d_n5;
        locals.var_gem_dn6 = assign5530_e5417_d_n6;
        locals.var_gem_dn7 = assign5530_e5417_d_n7;
        locals.var_gem_dn8 = assign5530_e5417_d_n8;
        locals.var_gem_dn9 = assign5530_e5417_d_n9;
        locals.var_gem_dn10 = assign5530_e5417_d_n10;
        locals.var_gem_dn11 = assign5530_e5417_d_n11;
        locals.var_gem_rv = 0.0;

        let assign5540_e5420: f64 = if p.p39 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard97 = assign5540_e5420;
        locals.var_guard97_rv = 0.0;

        let assign5550_e5423: f64 = if locals.var_vb2c1 < locals.var_vdc_t { 1.0 } else { 0.0 };
        locals.var_guard98 = assign5550_e5423;
        locals.var_guard98_rv = 0.0;

        let (assign5560_e5440,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign5560_e5434: f64 = (2.0 * p.p46);
        let assign5560_e5437: f64 = (p.p45 * p.p45);
        let assign5560_e5438: f64 = (assign5560_e5434 / assign5560_e5437);
        (assign5560_e5438,)
    } else {
        (locals.var_dedx0,)
    }
};
        locals.var_dedx0 = assign5560_e5440;
        locals.var_dedx0_rv = 0.0;

        let (assign5570_e5455, assign5570_e5455_d_n0, assign5570_e5455_d_n1, assign5570_e5455_d_n3, assign5570_e5455_d_n4, assign5570_e5455_d_n5, assign5570_e5455_d_n6, assign5570_e5455_d_n7, assign5570_e5455_d_n8, assign5570_e5455_d_n9, assign5570_e5455_d_n10, assign5570_e5455_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign5570_e5451: f64 = (locals.var_vdc_t - locals.var_vb2c1);
        let assign5570_e5453: f64 = (assign5570_e5451 / locals.var_icap_ihc);
        (assign5570_e5453, (((locals.var_vdc_t_dn0 * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn0)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn1 * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn1)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn3 * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn3)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn4 * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn4)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn5 * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn5)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn6 * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn6)) / (locals.var_icap_ihc * locals.var_icap_ihc)), ((((locals.var_vdc_t_dn7 - locals.var_vb2c1_dn7) * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn7)) / (locals.var_icap_ihc * locals.var_icap_ihc)), ((((locals.var_vdc_t_dn8 - locals.var_vb2c1_dn8) * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn8)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn9 * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn9)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn10 * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn10)) / (locals.var_icap_ihc * locals.var_icap_ihc)), (((locals.var_vdc_t_dn11 * locals.var_icap_ihc) - (assign5570_e5451 * locals.var_icap_ihc_dn11)) / (locals.var_icap_ihc * locals.var_icap_ihc)),)
    } else {
        (locals.var_sqr_arg, locals.var_sqr_arg_dn0, locals.var_sqr_arg_dn1, locals.var_sqr_arg_dn3, locals.var_sqr_arg_dn4, locals.var_sqr_arg_dn5, locals.var_sqr_arg_dn6, locals.var_sqr_arg_dn7, locals.var_sqr_arg_dn8, locals.var_sqr_arg_dn9, locals.var_sqr_arg_dn10, locals.var_sqr_arg_dn11,)
    }
};
        locals.var_sqr_arg = assign5570_e5455;
        locals.var_sqr_arg_dn0 = assign5570_e5455_d_n0;
        locals.var_sqr_arg_dn1 = assign5570_e5455_d_n1;
        locals.var_sqr_arg_dn3 = assign5570_e5455_d_n3;
        locals.var_sqr_arg_dn4 = assign5570_e5455_d_n4;
        locals.var_sqr_arg_dn5 = assign5570_e5455_d_n5;
        locals.var_sqr_arg_dn6 = assign5570_e5455_d_n6;
        locals.var_sqr_arg_dn7 = assign5570_e5455_d_n7;
        locals.var_sqr_arg_dn8 = assign5570_e5455_d_n8;
        locals.var_sqr_arg_dn9 = assign5570_e5455_d_n9;
        locals.var_sqr_arg_dn10 = assign5570_e5455_d_n10;
        locals.var_sqr_arg_dn11 = assign5570_e5455_d_n11;
        locals.var_sqr_arg_rv = 0.0;

        let (assign5580_e5471, assign5580_e5471_d_n0, assign5580_e5471_d_n1, assign5580_e5471_d_n3, assign5580_e5471_d_n4, assign5580_e5471_d_n5, assign5580_e5471_d_n6, assign5580_e5471_d_n7, assign5580_e5471_d_n8, assign5580_e5471_d_n9, assign5580_e5471_d_n10, assign5580_e5471_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign5580_e5466: f64 = (2.0 * locals.var_sqr_arg);
        let assign5580_e5468: f64 = (assign5580_e5466 / locals.var_dedx0);
        let assign5580_e5469: f64 = (assign5580_e5468).sqrt();
        (assign5580_e5469, (((2.0 * locals.var_sqr_arg_dn0) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn1) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn3) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn4) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn5) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn6) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn7) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn8) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn9) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn10) / locals.var_dedx0) / (2.0 * assign5580_e5469)), (((2.0 * locals.var_sqr_arg_dn11) / locals.var_dedx0) / (2.0 * assign5580_e5469)),)
    } else {
        (locals.var_xd, locals.var_xd_dn0, locals.var_xd_dn1, locals.var_xd_dn3, locals.var_xd_dn4, locals.var_xd_dn5, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn8, locals.var_xd_dn9, locals.var_xd_dn10, locals.var_xd_dn11,)
    }
};
        locals.var_xd = assign5580_e5471;
        locals.var_xd_dn0 = assign5580_e5471_d_n0;
        locals.var_xd_dn1 = assign5580_e5471_d_n1;
        locals.var_xd_dn3 = assign5580_e5471_d_n3;
        locals.var_xd_dn4 = assign5580_e5471_d_n4;
        locals.var_xd_dn5 = assign5580_e5471_d_n5;
        locals.var_xd_dn6 = assign5580_e5471_d_n6;
        locals.var_xd_dn7 = assign5580_e5471_d_n7;
        locals.var_xd_dn8 = assign5580_e5471_d_n8;
        locals.var_xd_dn9 = assign5580_e5471_d_n9;
        locals.var_xd_dn10 = assign5580_e5471_d_n10;
        locals.var_xd_dn11 = assign5580_e5471_d_n11;
        locals.var_xd_rv = 0.0;

        let assign5590_e5474: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard99 = assign5590_e5474;
        locals.var_guard99_rv = 0.0;

        let (assign5600_e5487, assign5600_e5487_d_n0, assign5600_e5487_d_n1, assign5600_e5487_d_n3, assign5600_e5487_d_n4, assign5600_e5487_d_n5, assign5600_e5487_d_n6, assign5600_e5487_d_n7, assign5600_e5487_d_n8, assign5600_e5487_d_n9, assign5600_e5487_d_n10, assign5600_e5487_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 != 0.0)) {
        (p.p45, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_weff, locals.var_weff_dn0, locals.var_weff_dn1, locals.var_weff_dn3, locals.var_weff_dn4, locals.var_weff_dn5, locals.var_weff_dn6, locals.var_weff_dn7, locals.var_weff_dn8, locals.var_weff_dn9, locals.var_weff_dn10, locals.var_weff_dn11,)
    }
};
        locals.var_weff = assign5600_e5487;
        locals.var_weff_dn0 = assign5600_e5487_d_n0;
        locals.var_weff_dn1 = assign5600_e5487_d_n1;
        locals.var_weff_dn3 = assign5600_e5487_d_n3;
        locals.var_weff_dn4 = assign5600_e5487_d_n4;
        locals.var_weff_dn5 = assign5600_e5487_d_n5;
        locals.var_weff_dn6 = assign5600_e5487_d_n6;
        locals.var_weff_dn7 = assign5600_e5487_d_n7;
        locals.var_weff_dn8 = assign5600_e5487_d_n8;
        locals.var_weff_dn9 = assign5600_e5487_d_n9;
        locals.var_weff_dn10 = assign5600_e5487_d_n10;
        locals.var_weff_dn11 = assign5600_e5487_d_n11;
        locals.var_weff_rv = 0.0;

        let (assign5610_e5505, assign5610_e5505_d_n0, assign5610_e5505_d_n1, assign5610_e5505_d_n3, assign5610_e5505_d_n4, assign5610_e5505_d_n5, assign5610_e5505_d_n6, assign5610_e5505_d_n7, assign5610_e5505_d_n8, assign5610_e5505_d_n9, assign5610_e5505_d_n10, assign5610_e5505_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 == 0.0)) {
        let assign5610_e5502: f64 = (0.5 * locals.var_xi_w);
        let assign5610_e5503: f64 = (1.0 - assign5610_e5502);
        (assign5610_e5503, (-(0.5 * locals.var_xi_w_dn0)), (-(0.5 * locals.var_xi_w_dn1)), (-(0.5 * locals.var_xi_w_dn3)), (-(0.5 * locals.var_xi_w_dn4)), (-(0.5 * locals.var_xi_w_dn5)), (-(0.5 * locals.var_xi_w_dn6)), (-(0.5 * locals.var_xi_w_dn7)), (-(0.5 * locals.var_xi_w_dn8)), (-(0.5 * locals.var_xi_w_dn9)), (-(0.5 * locals.var_xi_w_dn10)), (-(0.5 * locals.var_xi_w_dn11)),)
    } else {
        (locals.var_xi_w1, locals.var_xi_w1_dn0, locals.var_xi_w1_dn1, locals.var_xi_w1_dn3, locals.var_xi_w1_dn4, locals.var_xi_w1_dn5, locals.var_xi_w1_dn6, locals.var_xi_w1_dn7, locals.var_xi_w1_dn8, locals.var_xi_w1_dn9, locals.var_xi_w1_dn10, locals.var_xi_w1_dn11,)
    }
};
        locals.var_xi_w1 = assign5610_e5505;
        locals.var_xi_w1_dn0 = assign5610_e5505_d_n0;
        locals.var_xi_w1_dn1 = assign5610_e5505_d_n1;
        locals.var_xi_w1_dn3 = assign5610_e5505_d_n3;
        locals.var_xi_w1_dn4 = assign5610_e5505_d_n4;
        locals.var_xi_w1_dn5 = assign5610_e5505_d_n5;
        locals.var_xi_w1_dn6 = assign5610_e5505_d_n6;
        locals.var_xi_w1_dn7 = assign5610_e5505_d_n7;
        locals.var_xi_w1_dn8 = assign5610_e5505_d_n8;
        locals.var_xi_w1_dn9 = assign5610_e5505_d_n9;
        locals.var_xi_w1_dn10 = assign5610_e5505_d_n10;
        locals.var_xi_w1_dn11 = assign5610_e5505_d_n11;
        locals.var_xi_w1_rv = 0.0;

        let (assign5620_e5523, assign5620_e5523_d_n0, assign5620_e5523_d_n1, assign5620_e5523_d_n3, assign5620_e5523_d_n4, assign5620_e5523_d_n5, assign5620_e5523_d_n6, assign5620_e5523_d_n7, assign5620_e5523_d_n8, assign5620_e5523_d_n9, assign5620_e5523_d_n10, assign5620_e5523_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 == 0.0)) {
        let assign5620_e5519: f64 = (p.p45 * locals.var_xi_w1);
        let assign5620_e5521: f64 = (assign5620_e5519 * locals.var_xi_w1);
        (assign5620_e5521, (((p.p45 * locals.var_xi_w1_dn0) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn0)), (((p.p45 * locals.var_xi_w1_dn1) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn1)), (((p.p45 * locals.var_xi_w1_dn3) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn3)), (((p.p45 * locals.var_xi_w1_dn4) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn4)), (((p.p45 * locals.var_xi_w1_dn5) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn5)), (((p.p45 * locals.var_xi_w1_dn6) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn6)), (((p.p45 * locals.var_xi_w1_dn7) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn7)), (((p.p45 * locals.var_xi_w1_dn8) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn8)), (((p.p45 * locals.var_xi_w1_dn9) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn9)), (((p.p45 * locals.var_xi_w1_dn10) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn10)), (((p.p45 * locals.var_xi_w1_dn11) * locals.var_xi_w1) + (assign5620_e5519 * locals.var_xi_w1_dn11)),)
    } else {
        (locals.var_weff, locals.var_weff_dn0, locals.var_weff_dn1, locals.var_weff_dn3, locals.var_weff_dn4, locals.var_weff_dn5, locals.var_weff_dn6, locals.var_weff_dn7, locals.var_weff_dn8, locals.var_weff_dn9, locals.var_weff_dn10, locals.var_weff_dn11,)
    }
};
        locals.var_weff = assign5620_e5523;
        locals.var_weff_dn0 = assign5620_e5523_d_n0;
        locals.var_weff_dn1 = assign5620_e5523_d_n1;
        locals.var_weff_dn3 = assign5620_e5523_d_n3;
        locals.var_weff_dn4 = assign5620_e5523_d_n4;
        locals.var_weff_dn5 = assign5620_e5523_d_n5;
        locals.var_weff_dn6 = assign5620_e5523_d_n6;
        locals.var_weff_dn7 = assign5620_e5523_d_n7;
        locals.var_weff_dn8 = assign5620_e5523_d_n8;
        locals.var_weff_dn9 = assign5620_e5523_d_n9;
        locals.var_weff_dn10 = assign5620_e5523_d_n10;
        locals.var_weff_dn11 = assign5620_e5523_d_n11;
        locals.var_weff_rv = 0.0;

        let (assign5630_e5545, assign5630_e5545_d_n0, assign5630_e5545_d_n1, assign5630_e5545_d_n3, assign5630_e5545_d_n4, assign5630_e5545_d_n5, assign5630_e5545_d_n6, assign5630_e5545_d_n7, assign5630_e5545_d_n8, assign5630_e5545_d_n9, assign5630_e5545_d_n10, assign5630_e5545_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign5630_e5534: f64 = (locals.var_xd * locals.var_weff);
        let assign5630_e5537: f64 = (locals.var_xd * locals.var_xd);
        let assign5630_e5540: f64 = (locals.var_weff * locals.var_weff);
        let assign5630_e5541: f64 = (assign5630_e5537 + assign5630_e5540);
        let assign5630_e5542: f64 = (assign5630_e5541).sqrt();
        let assign5630_e5543: f64 = (assign5630_e5534 / assign5630_e5542);
        (assign5630_e5543, (((((locals.var_xd_dn0 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn0)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn0 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn0)) + ((locals.var_weff_dn0 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn0))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn1 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn1)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn1 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn1)) + ((locals.var_weff_dn1 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn1))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn3 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn3)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn3 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn3)) + ((locals.var_weff_dn3 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn3))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn4 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn4)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn4 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn4)) + ((locals.var_weff_dn4 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn4))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn5 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn5)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn5 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn5)) + ((locals.var_weff_dn5 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn5))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn6 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn6)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn6 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn6)) + ((locals.var_weff_dn6 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn6))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn7 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn7)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn7 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn7)) + ((locals.var_weff_dn7 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn7))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn8 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn8)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn8 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn8)) + ((locals.var_weff_dn8 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn8))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn9 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn9)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn9 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn9)) + ((locals.var_weff_dn9 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn9))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn10 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn10)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn10 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn10)) + ((locals.var_weff_dn10 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn10))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)), (((((locals.var_xd_dn11 * locals.var_weff) + (locals.var_xd * locals.var_weff_dn11)) * assign5630_e5542) - (assign5630_e5534 * ((((locals.var_xd_dn11 * locals.var_xd) + (locals.var_xd * locals.var_xd_dn11)) + ((locals.var_weff_dn11 * locals.var_weff) + (locals.var_weff * locals.var_weff_dn11))) / (2.0 * assign5630_e5542)))) / (assign5630_e5542 * assign5630_e5542)),)
    } else {
        (locals.var_wd, locals.var_wd_dn0, locals.var_wd_dn1, locals.var_wd_dn3, locals.var_wd_dn4, locals.var_wd_dn5, locals.var_wd_dn6, locals.var_wd_dn7, locals.var_wd_dn8, locals.var_wd_dn9, locals.var_wd_dn10, locals.var_wd_dn11,)
    }
};
        locals.var_wd = assign5630_e5545;
        locals.var_wd_dn0 = assign5630_e5545_d_n0;
        locals.var_wd_dn1 = assign5630_e5545_d_n1;
        locals.var_wd_dn3 = assign5630_e5545_d_n3;
        locals.var_wd_dn4 = assign5630_e5545_d_n4;
        locals.var_wd_dn5 = assign5630_e5545_d_n5;
        locals.var_wd_dn6 = assign5630_e5545_d_n6;
        locals.var_wd_dn7 = assign5630_e5545_d_n7;
        locals.var_wd_dn8 = assign5630_e5545_d_n8;
        locals.var_wd_dn9 = assign5630_e5545_d_n9;
        locals.var_wd_dn10 = assign5630_e5545_d_n10;
        locals.var_wd_dn11 = assign5630_e5545_d_n11;
        locals.var_wd_rv = 0.0;

        let (assign5640_e5560, assign5640_e5560_d_n0, assign5640_e5560_d_n1, assign5640_e5560_d_n3, assign5640_e5560_d_n4, assign5640_e5560_d_n5, assign5640_e5560_d_n6, assign5640_e5560_d_n7, assign5640_e5560_d_n8, assign5640_e5560_d_n9, assign5640_e5560_d_n10, assign5640_e5560_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign5640_e5556: f64 = (locals.var_vdc_t - locals.var_vb2c1);
        let assign5640_e5558: f64 = (assign5640_e5556 / locals.var_wd);
        (assign5640_e5558, (((locals.var_vdc_t_dn0 * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn0)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn1 * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn1)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn3 * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn3)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn4 * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn4)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn5 * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn5)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn6 * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn6)) / (locals.var_wd * locals.var_wd)), ((((locals.var_vdc_t_dn7 - locals.var_vb2c1_dn7) * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn7)) / (locals.var_wd * locals.var_wd)), ((((locals.var_vdc_t_dn8 - locals.var_vb2c1_dn8) * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn8)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn9 * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn9)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn10 * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn10)) / (locals.var_wd * locals.var_wd)), (((locals.var_vdc_t_dn11 * locals.var_wd) - (assign5640_e5556 * locals.var_wd_dn11)) / (locals.var_wd * locals.var_wd)),)
    } else {
        (locals.var_eav, locals.var_eav_dn0, locals.var_eav_dn1, locals.var_eav_dn3, locals.var_eav_dn4, locals.var_eav_dn5, locals.var_eav_dn6, locals.var_eav_dn7, locals.var_eav_dn8, locals.var_eav_dn9, locals.var_eav_dn10, locals.var_eav_dn11,)
    }
};
        locals.var_eav = assign5640_e5560;
        locals.var_eav_dn0 = assign5640_e5560_d_n0;
        locals.var_eav_dn1 = assign5640_e5560_d_n1;
        locals.var_eav_dn3 = assign5640_e5560_d_n3;
        locals.var_eav_dn4 = assign5640_e5560_d_n4;
        locals.var_eav_dn5 = assign5640_e5560_d_n5;
        locals.var_eav_dn6 = assign5640_e5560_d_n6;
        locals.var_eav_dn7 = assign5640_e5560_d_n7;
        locals.var_eav_dn8 = assign5640_e5560_d_n8;
        locals.var_eav_dn9 = assign5640_e5560_d_n9;
        locals.var_eav_dn10 = assign5640_e5560_d_n10;
        locals.var_eav_dn11 = assign5640_e5560_d_n11;
        locals.var_eav_rv = 0.0;

        let (assign5650_e5579, assign5650_e5579_d_n0, assign5650_e5579_d_n1, assign5650_e5579_d_n3, assign5650_e5579_d_n4, assign5650_e5579_d_n5, assign5650_e5579_d_n6, assign5650_e5579_d_n7, assign5650_e5579_d_n8, assign5650_e5579_d_n9, assign5650_e5579_d_n10, assign5650_e5579_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign5650_e5572: f64 = (0.5 * locals.var_wd);
        let assign5650_e5574: f64 = (assign5650_e5572 * locals.var_dedx0);
        let assign5650_e5576: f64 = (assign5650_e5574 * locals.var_icap_ihc);
        let assign5650_e5577: f64 = (locals.var_eav + assign5650_e5576);
        (assign5650_e5577, (locals.var_eav_dn0 + ((((0.5 * locals.var_wd_dn0) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn0))), (locals.var_eav_dn1 + ((((0.5 * locals.var_wd_dn1) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn1))), (locals.var_eav_dn3 + ((((0.5 * locals.var_wd_dn3) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn3))), (locals.var_eav_dn4 + ((((0.5 * locals.var_wd_dn4) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn4))), (locals.var_eav_dn5 + ((((0.5 * locals.var_wd_dn5) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn5))), (locals.var_eav_dn6 + ((((0.5 * locals.var_wd_dn6) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn6))), (locals.var_eav_dn7 + ((((0.5 * locals.var_wd_dn7) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn7))), (locals.var_eav_dn8 + ((((0.5 * locals.var_wd_dn8) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn8))), (locals.var_eav_dn9 + ((((0.5 * locals.var_wd_dn9) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn9))), (locals.var_eav_dn10 + ((((0.5 * locals.var_wd_dn10) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn10))), (locals.var_eav_dn11 + ((((0.5 * locals.var_wd_dn11) * locals.var_dedx0) * locals.var_icap_ihc) + (assign5650_e5574 * locals.var_icap_ihc_dn11))),)
    } else {
        (locals.var_e0, locals.var_e0_dn0, locals.var_e0_dn1, locals.var_e0_dn3, locals.var_e0_dn4, locals.var_e0_dn5, locals.var_e0_dn6, locals.var_e0_dn7, locals.var_e0_dn8, locals.var_e0_dn9, locals.var_e0_dn10, locals.var_e0_dn11,)
    }
};
        locals.var_e0 = assign5650_e5579;
        locals.var_e0_dn0 = assign5650_e5579_d_n0;
        locals.var_e0_dn1 = assign5650_e5579_d_n1;
        locals.var_e0_dn3 = assign5650_e5579_d_n3;
        locals.var_e0_dn4 = assign5650_e5579_d_n4;
        locals.var_e0_dn5 = assign5650_e5579_d_n5;
        locals.var_e0_dn6 = assign5650_e5579_d_n6;
        locals.var_e0_dn7 = assign5650_e5579_d_n7;
        locals.var_e0_dn8 = assign5650_e5579_d_n8;
        locals.var_e0_dn9 = assign5650_e5579_d_n9;
        locals.var_e0_dn10 = assign5650_e5579_d_n10;
        locals.var_e0_dn11 = assign5650_e5579_d_n11;
        locals.var_e0_rv = 0.0;

        let assign5660_e5582: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard100 = assign5660_e5582;
        locals.var_guard100_rv = 0.0;

        let (assign5670_e5595, assign5670_e5595_d_n0, assign5670_e5595_d_n1, assign5670_e5595_d_n3, assign5670_e5595_d_n4, assign5670_e5595_d_n5, assign5670_e5595_d_n6, assign5670_e5595_d_n7, assign5670_e5595_d_n8, assign5670_e5595_d_n9, assign5670_e5595_d_n10, assign5670_e5595_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 != 0.0)) {
        (locals.var_e0, locals.var_e0_dn0, locals.var_e0_dn1, locals.var_e0_dn3, locals.var_e0_dn4, locals.var_e0_dn5, locals.var_e0_dn6, locals.var_e0_dn7, locals.var_e0_dn8, locals.var_e0_dn9, locals.var_e0_dn10, locals.var_e0_dn11,)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn1, locals.var_em_dn3, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10, locals.var_em_dn11,)
    }
};
        locals.var_em = assign5670_e5595;
        locals.var_em_dn0 = assign5670_e5595_d_n0;
        locals.var_em_dn1 = assign5670_e5595_d_n1;
        locals.var_em_dn3 = assign5670_e5595_d_n3;
        locals.var_em_dn4 = assign5670_e5595_d_n4;
        locals.var_em_dn5 = assign5670_e5595_d_n5;
        locals.var_em_dn6 = assign5670_e5595_d_n6;
        locals.var_em_dn7 = assign5670_e5595_d_n7;
        locals.var_em_dn8 = assign5670_e5595_d_n8;
        locals.var_em_dn9 = assign5670_e5595_d_n9;
        locals.var_em_dn10 = assign5670_e5595_d_n10;
        locals.var_em_dn11 = assign5670_e5595_d_n11;
        locals.var_em_rv = 0.0;

        let (assign5680_e5619, assign5680_e5619_d_n0, assign5680_e5619_d_n1, assign5680_e5619_d_n3, assign5680_e5619_d_n4, assign5680_e5619_d_n5, assign5680_e5619_d_n6, assign5680_e5619_d_n7, assign5680_e5619_d_n8, assign5680_e5619_d_n9, assign5680_e5619_d_n10, assign5680_e5619_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 == 0.0)) {
        let assign5680_e5610: f64 = (2.0 * p.p47);
        let assign5680_e5614: f64 = (2.0 * locals.var_xi_w);
        let assign5680_e5615: f64 = (1.0 + assign5680_e5614);
        let assign5680_e5616: f64 = (assign5680_e5610 * assign5680_e5615);
        let assign5680_e5617: f64 = (1.0 + assign5680_e5616);
        (assign5680_e5617, (assign5680_e5610 * (2.0 * locals.var_xi_w_dn0)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn1)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn3)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn4)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn5)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn6)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn7)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn8)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn9)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn10)), (assign5680_e5610 * (2.0 * locals.var_xi_w_dn11)),)
    } else {
        (locals.var_shw, locals.var_shw_dn0, locals.var_shw_dn1, locals.var_shw_dn3, locals.var_shw_dn4, locals.var_shw_dn5, locals.var_shw_dn6, locals.var_shw_dn7, locals.var_shw_dn8, locals.var_shw_dn9, locals.var_shw_dn10, locals.var_shw_dn11,)
    }
};
        locals.var_shw = assign5680_e5619;
        locals.var_shw_dn0 = assign5680_e5619_d_n0;
        locals.var_shw_dn1 = assign5680_e5619_d_n1;
        locals.var_shw_dn3 = assign5680_e5619_d_n3;
        locals.var_shw_dn4 = assign5680_e5619_d_n4;
        locals.var_shw_dn5 = assign5680_e5619_d_n5;
        locals.var_shw_dn6 = assign5680_e5619_d_n6;
        locals.var_shw_dn7 = assign5680_e5619_d_n7;
        locals.var_shw_dn8 = assign5680_e5619_d_n8;
        locals.var_shw_dn9 = assign5680_e5619_d_n9;
        locals.var_shw_dn10 = assign5680_e5619_d_n10;
        locals.var_shw_dn11 = assign5680_e5619_d_n11;
        locals.var_shw_rv = 0.0;

        let (assign5690_e5641,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 == 0.0)) {
        let assign5690_e5633: f64 = (1.0 + p.p47);
        let assign5690_e5637: f64 = (2.0 * p.p47);
        let assign5690_e5638: f64 = (1.0 + assign5690_e5637);
        let assign5690_e5639: f64 = (assign5690_e5633 / assign5690_e5638);
        (assign5690_e5639,)
    } else {
        (locals.var_efi,)
    }
};
        locals.var_efi = assign5690_e5641;
        locals.var_efi_rv = 0.0;

        let (assign5700_e5669, assign5700_e5669_d_n0, assign5700_e5669_d_n1, assign5700_e5669_d_n3, assign5700_e5669_d_n4, assign5700_e5669_d_n5, assign5700_e5669_d_n6, assign5700_e5669_d_n7, assign5700_e5669_d_n8, assign5700_e5669_d_n9, assign5700_e5669_d_n10, assign5700_e5669_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 == 0.0)) {
        let assign5700_e5656: f64 = (0.5 * locals.var_wd);
        let assign5700_e5658: f64 = (assign5700_e5656 * locals.var_dedx0);
        let assign5700_e5663: f64 = (p.p62 * locals.var_shw);
        let assign5700_e5664: f64 = (locals.var_in_ / assign5700_e5663);
        let assign5700_e5665: f64 = (locals.var_efi - assign5700_e5664);
        let assign5700_e5666: f64 = (assign5700_e5658 * assign5700_e5665);
        let assign5700_e5667: f64 = (locals.var_eav - assign5700_e5666);
        (assign5700_e5667, (locals.var_eav_dn0 - ((((0.5 * locals.var_wd_dn0) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn0 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn0))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn1 - ((((0.5 * locals.var_wd_dn1) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn1 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn1))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn3 - ((((0.5 * locals.var_wd_dn3) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn3 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn3))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn4 - ((((0.5 * locals.var_wd_dn4) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn4 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn4))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn5 - ((((0.5 * locals.var_wd_dn5) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn5 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn5))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn6 - ((((0.5 * locals.var_wd_dn6) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn6 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn6))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn7 - ((((0.5 * locals.var_wd_dn7) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn7 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn7))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn8 - ((((0.5 * locals.var_wd_dn8) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn8 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn8))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn9 - ((((0.5 * locals.var_wd_dn9) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn9 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn9))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn10 - ((((0.5 * locals.var_wd_dn10) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn10 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn10))) / (assign5700_e5663 * assign5700_e5663)))))), (locals.var_eav_dn11 - ((((0.5 * locals.var_wd_dn11) * locals.var_dedx0) * assign5700_e5665) + (assign5700_e5658 * (-(((locals.var_in__dn11 * assign5700_e5663) - (locals.var_in_ * (p.p62 * locals.var_shw_dn11))) / (assign5700_e5663 * assign5700_e5663)))))),)
    } else {
        (locals.var_ew, locals.var_ew_dn0, locals.var_ew_dn1, locals.var_ew_dn3, locals.var_ew_dn4, locals.var_ew_dn5, locals.var_ew_dn6, locals.var_ew_dn7, locals.var_ew_dn8, locals.var_ew_dn9, locals.var_ew_dn10, locals.var_ew_dn11,)
    }
};
        locals.var_ew = assign5700_e5669;
        locals.var_ew_dn0 = assign5700_e5669_d_n0;
        locals.var_ew_dn1 = assign5700_e5669_d_n1;
        locals.var_ew_dn3 = assign5700_e5669_d_n3;
        locals.var_ew_dn4 = assign5700_e5669_d_n4;
        locals.var_ew_dn5 = assign5700_e5669_d_n5;
        locals.var_ew_dn6 = assign5700_e5669_d_n6;
        locals.var_ew_dn7 = assign5700_e5669_d_n7;
        locals.var_ew_dn8 = assign5700_e5669_d_n8;
        locals.var_ew_dn9 = assign5700_e5669_d_n9;
        locals.var_ew_dn10 = assign5700_e5669_d_n10;
        locals.var_ew_dn11 = assign5700_e5669_d_n11;
        locals.var_ew_rv = 0.0;

        let (assign5710_e5699, assign5710_e5699_d_n0, assign5710_e5699_d_n1, assign5710_e5699_d_n3, assign5710_e5699_d_n4, assign5710_e5699_d_n5, assign5710_e5699_d_n6, assign5710_e5699_d_n7, assign5710_e5699_d_n8, assign5710_e5699_d_n9, assign5710_e5699_d_n10, assign5710_e5699_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 == 0.0)) {
        let assign5710_e5683: f64 = (locals.var_ew - locals.var_e0);
        let assign5710_e5686: f64 = (locals.var_ew - locals.var_e0);
        let assign5710_e5687: f64 = (assign5710_e5683 * assign5710_e5686);
        let assign5710_e5690: f64 = (0.1 * locals.var_eav);
        let assign5710_e5692: f64 = (assign5710_e5690 * locals.var_eav);
        let assign5710_e5694: f64 = (assign5710_e5692 * locals.var_icap);
        let assign5710_e5696: f64 = (assign5710_e5694 / p.p62);
        let assign5710_e5697: f64 = (assign5710_e5687 + assign5710_e5696);
        (assign5710_e5697, ((((locals.var_ew_dn0 - locals.var_e0_dn0) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn0 - locals.var_e0_dn0))) + ((((((0.1 * locals.var_eav_dn0) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn0)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn0)) / p.p62)), ((((locals.var_ew_dn1 - locals.var_e0_dn1) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn1 - locals.var_e0_dn1))) + ((((((0.1 * locals.var_eav_dn1) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn1)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn1)) / p.p62)), ((((locals.var_ew_dn3 - locals.var_e0_dn3) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn3 - locals.var_e0_dn3))) + ((((((0.1 * locals.var_eav_dn3) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn3)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn3)) / p.p62)), ((((locals.var_ew_dn4 - locals.var_e0_dn4) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn4 - locals.var_e0_dn4))) + ((((((0.1 * locals.var_eav_dn4) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn4)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn4)) / p.p62)), ((((locals.var_ew_dn5 - locals.var_e0_dn5) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn5 - locals.var_e0_dn5))) + ((((((0.1 * locals.var_eav_dn5) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn5)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn5)) / p.p62)), ((((locals.var_ew_dn6 - locals.var_e0_dn6) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn6 - locals.var_e0_dn6))) + ((((((0.1 * locals.var_eav_dn6) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn6)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn6)) / p.p62)), ((((locals.var_ew_dn7 - locals.var_e0_dn7) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn7 - locals.var_e0_dn7))) + ((((((0.1 * locals.var_eav_dn7) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn7)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn7)) / p.p62)), ((((locals.var_ew_dn8 - locals.var_e0_dn8) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn8 - locals.var_e0_dn8))) + ((((((0.1 * locals.var_eav_dn8) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn8)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn8)) / p.p62)), ((((locals.var_ew_dn9 - locals.var_e0_dn9) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn9 - locals.var_e0_dn9))) + ((((((0.1 * locals.var_eav_dn9) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn9)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn9)) / p.p62)), ((((locals.var_ew_dn10 - locals.var_e0_dn10) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn10 - locals.var_e0_dn10))) + ((((((0.1 * locals.var_eav_dn10) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn10)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn10)) / p.p62)), ((((locals.var_ew_dn11 - locals.var_e0_dn11) * assign5710_e5686) + (assign5710_e5683 * (locals.var_ew_dn11 - locals.var_e0_dn11))) + ((((((0.1 * locals.var_eav_dn11) * locals.var_eav) + (assign5710_e5690 * locals.var_eav_dn11)) * locals.var_icap) + (assign5710_e5692 * locals.var_icap_dn11)) / p.p62)),)
    } else {
        (locals.var_sqr_arg, locals.var_sqr_arg_dn0, locals.var_sqr_arg_dn1, locals.var_sqr_arg_dn3, locals.var_sqr_arg_dn4, locals.var_sqr_arg_dn5, locals.var_sqr_arg_dn6, locals.var_sqr_arg_dn7, locals.var_sqr_arg_dn8, locals.var_sqr_arg_dn9, locals.var_sqr_arg_dn10, locals.var_sqr_arg_dn11,)
    }
};
        locals.var_sqr_arg = assign5710_e5699;
        locals.var_sqr_arg_dn0 = assign5710_e5699_d_n0;
        locals.var_sqr_arg_dn1 = assign5710_e5699_d_n1;
        locals.var_sqr_arg_dn3 = assign5710_e5699_d_n3;
        locals.var_sqr_arg_dn4 = assign5710_e5699_d_n4;
        locals.var_sqr_arg_dn5 = assign5710_e5699_d_n5;
        locals.var_sqr_arg_dn6 = assign5710_e5699_d_n6;
        locals.var_sqr_arg_dn7 = assign5710_e5699_d_n7;
        locals.var_sqr_arg_dn8 = assign5710_e5699_d_n8;
        locals.var_sqr_arg_dn9 = assign5710_e5699_d_n9;
        locals.var_sqr_arg_dn10 = assign5710_e5699_d_n10;
        locals.var_sqr_arg_dn11 = assign5710_e5699_d_n11;
        locals.var_sqr_arg_rv = 0.0;

        let (assign5720_e5720, assign5720_e5720_d_n0, assign5720_e5720_d_n1, assign5720_e5720_d_n3, assign5720_e5720_d_n4, assign5720_e5720_d_n5, assign5720_e5720_d_n6, assign5720_e5720_d_n7, assign5720_e5720_d_n8, assign5720_e5720_d_n9, assign5720_e5720_d_n10, assign5720_e5720_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 == 0.0)) {
        let assign5720_e5714: f64 = (locals.var_ew + locals.var_e0);
        let assign5720_e5716: f64 = (locals.var_sqr_arg).sqrt();
        let assign5720_e5717: f64 = (assign5720_e5714 + assign5720_e5716);
        let assign5720_e5718: f64 = (0.5 * assign5720_e5717);
        (assign5720_e5718, (0.5 * ((locals.var_ew_dn0 + locals.var_e0_dn0) + (locals.var_sqr_arg_dn0 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn1 + locals.var_e0_dn1) + (locals.var_sqr_arg_dn1 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn3 + locals.var_e0_dn3) + (locals.var_sqr_arg_dn3 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn4 + locals.var_e0_dn4) + (locals.var_sqr_arg_dn4 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn5 + locals.var_e0_dn5) + (locals.var_sqr_arg_dn5 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn6 + locals.var_e0_dn6) + (locals.var_sqr_arg_dn6 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn7 + locals.var_e0_dn7) + (locals.var_sqr_arg_dn7 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn8 + locals.var_e0_dn8) + (locals.var_sqr_arg_dn8 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn9 + locals.var_e0_dn9) + (locals.var_sqr_arg_dn9 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn10 + locals.var_e0_dn10) + (locals.var_sqr_arg_dn10 / (2.0 * assign5720_e5716)))), (0.5 * ((locals.var_ew_dn11 + locals.var_e0_dn11) + (locals.var_sqr_arg_dn11 / (2.0 * assign5720_e5716)))),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn1, locals.var_em_dn3, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10, locals.var_em_dn11,)
    }
};
        locals.var_em = assign5720_e5720;
        locals.var_em_dn0 = assign5720_e5720_d_n0;
        locals.var_em_dn1 = assign5720_e5720_d_n1;
        locals.var_em_dn3 = assign5720_e5720_d_n3;
        locals.var_em_dn4 = assign5720_e5720_d_n4;
        locals.var_em_dn5 = assign5720_e5720_d_n5;
        locals.var_em_dn6 = assign5720_e5720_d_n6;
        locals.var_em_dn7 = assign5720_e5720_d_n7;
        locals.var_em_dn8 = assign5720_e5720_d_n8;
        locals.var_em_dn9 = assign5720_e5720_d_n9;
        locals.var_em_dn10 = assign5720_e5720_d_n10;
        locals.var_em_dn11 = assign5720_e5720_d_n11;
        locals.var_em_rv = 0.0;

        let (assign5730_e5735, assign5730_e5735_d_n0, assign5730_e5735_d_n1, assign5730_e5735_d_n3, assign5730_e5735_d_n4, assign5730_e5735_d_n5, assign5730_e5735_d_n6, assign5730_e5735_d_n7, assign5730_e5735_d_n8, assign5730_e5735_d_n9, assign5730_e5735_d_n10, assign5730_e5735_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign5730_e5731: f64 = (locals.var_em - locals.var_eav);
        let assign5730_e5733: f64 = (assign5730_e5731 / locals.var_em);
        (assign5730_e5733, ((((locals.var_em_dn0 - locals.var_eav_dn0) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn0)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn1 - locals.var_eav_dn1) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn1)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn3 - locals.var_eav_dn3) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn3)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn4 - locals.var_eav_dn4) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn4)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn5 - locals.var_eav_dn5) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn5)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn6 - locals.var_eav_dn6) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn6)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn7 - locals.var_eav_dn7) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn7)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn8 - locals.var_eav_dn8) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn8)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn9 - locals.var_eav_dn9) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn9)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn10 - locals.var_eav_dn10) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn10)) / (locals.var_em * locals.var_em)), ((((locals.var_em_dn11 - locals.var_eav_dn11) * locals.var_em) - (assign5730_e5731 * locals.var_em_dn11)) / (locals.var_em * locals.var_em)),)
    } else {
        (locals.var_emeav_em, locals.var_emeav_em_dn0, locals.var_emeav_em_dn1, locals.var_emeav_em_dn3, locals.var_emeav_em_dn4, locals.var_emeav_em_dn5, locals.var_emeav_em_dn6, locals.var_emeav_em_dn7, locals.var_emeav_em_dn8, locals.var_emeav_em_dn9, locals.var_emeav_em_dn10, locals.var_emeav_em_dn11,)
    }
};
        locals.var_emeav_em = assign5730_e5735;
        locals.var_emeav_em_dn0 = assign5730_e5735_d_n0;
        locals.var_emeav_em_dn1 = assign5730_e5735_d_n1;
        locals.var_emeav_em_dn3 = assign5730_e5735_d_n3;
        locals.var_emeav_em_dn4 = assign5730_e5735_d_n4;
        locals.var_emeav_em_dn5 = assign5730_e5735_d_n5;
        locals.var_emeav_em_dn6 = assign5730_e5735_d_n6;
        locals.var_emeav_em_dn7 = assign5730_e5735_d_n7;
        locals.var_emeav_em_dn8 = assign5730_e5735_d_n8;
        locals.var_emeav_em_dn9 = assign5730_e5735_d_n9;
        locals.var_emeav_em_dn10 = assign5730_e5735_d_n10;
        locals.var_emeav_em_dn11 = assign5730_e5735_d_n11;
        locals.var_emeav_em_rv = 0.0;

        let assign5740_e5737: f64 = (locals.var_emeav_em).abs();
        let assign5740_e5739: f64 = if assign5740_e5737 > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard101 = assign5740_e5739;
        locals.var_guard101_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5750_e5756, assign5750_e5756_d_n0, assign5750_e5756_d_n1, assign5750_e5756_d_n3, assign5750_e5756_d_n4, assign5750_e5756_d_n5, assign5750_e5756_d_n6, assign5750_e5756_d_n7, assign5750_e5756_d_n8, assign5750_e5756_d_n9, assign5750_e5756_d_n10, assign5750_e5756_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard101 != 0.0)) {
        let assign5750_e5752: f64 = (0.5 * locals.var_wd);
        let assign5750_e5754: f64 = (assign5750_e5752 / locals.var_emeav_em);
        (assign5750_e5754, ((((0.5 * locals.var_wd_dn0) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn0)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn1) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn1)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn3) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn3)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn4) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn4)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn5) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn5)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn6) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn6)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn7) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn7)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn8) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn8)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn9) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn9)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn10) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn10)) / (locals.var_emeav_em * locals.var_emeav_em)), ((((0.5 * locals.var_wd_dn11) * locals.var_emeav_em) - (assign5750_e5752 * locals.var_emeav_em_dn11)) / (locals.var_emeav_em * locals.var_emeav_em)),)
    } else {
        (locals.var_lambda, locals.var_lambda_dn0, locals.var_lambda_dn1, locals.var_lambda_dn3, locals.var_lambda_dn4, locals.var_lambda_dn5, locals.var_lambda_dn6, locals.var_lambda_dn7, locals.var_lambda_dn8, locals.var_lambda_dn9, locals.var_lambda_dn10, locals.var_lambda_dn11,)
    }
};
        locals.var_lambda = assign5750_e5756;
        locals.var_lambda_dn0 = assign5750_e5756_d_n0;
        locals.var_lambda_dn1 = assign5750_e5756_d_n1;
        locals.var_lambda_dn3 = assign5750_e5756_d_n3;
        locals.var_lambda_dn4 = assign5750_e5756_d_n4;
        locals.var_lambda_dn5 = assign5750_e5756_d_n5;
        locals.var_lambda_dn6 = assign5750_e5756_d_n6;
        locals.var_lambda_dn7 = assign5750_e5756_d_n7;
        locals.var_lambda_dn8 = assign5750_e5756_d_n8;
        locals.var_lambda_dn9 = assign5750_e5756_d_n9;
        locals.var_lambda_dn10 = assign5750_e5756_d_n10;
        locals.var_lambda_dn11 = assign5750_e5756_d_n11;
        locals.var_lambda_rv = 0.0;

        let (assign5760_e5793, assign5760_e5793_d_n0, assign5760_e5793_d_n1, assign5760_e5793_d_n3, assign5760_e5793_d_n4, assign5760_e5793_d_n5, assign5760_e5793_d_n6, assign5760_e5793_d_n7, assign5760_e5793_d_n8, assign5760_e5793_d_n9, assign5760_e5793_d_n10, assign5760_e5793_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard101 != 0.0)) {
        let assign5760_e5769: f64 = (locals.var_an / locals.var_bnt);
        let assign5760_e5771: f64 = (assign5760_e5769 * locals.var_em);
        let assign5760_e5773: f64 = (assign5760_e5771 * locals.var_lambda);
        let assign5760_e5775: f64 = (-locals.var_bnt);
        let assign5760_e5777: f64 = (assign5760_e5775 / locals.var_em);
        let assign5760_e5778: f64 = (assign5760_e5777).exp();
        let assign5760_e5780: f64 = (-locals.var_bnt);
        let assign5760_e5782: f64 = (assign5760_e5780 / locals.var_em);
        let assign5760_e5786: f64 = (locals.var_weff / locals.var_lambda);
        let assign5760_e5787: f64 = (1.0 + assign5760_e5786);
        let assign5760_e5788: f64 = (assign5760_e5782 * assign5760_e5787);
        let assign5760_e5789: f64 = (assign5760_e5788).exp();
        let assign5760_e5790: f64 = (assign5760_e5778 - assign5760_e5789);
        let assign5760_e5791: f64 = (assign5760_e5773 * assign5760_e5790);
        (assign5760_e5791, (((((assign5760_e5769 * locals.var_em_dn0) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn0)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn0) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn0) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn0 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn0)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5760_e5769 * locals.var_em_dn1) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn1)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn1) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn1) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn1 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn1)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5760_e5769 * locals.var_em_dn3) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn3)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn3) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn3) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn3 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn3)) / (locals.var_lambda * locals.var_lambda)))))))), (((((((-((locals.var_an * locals.var_bnt_dn4) / (locals.var_bnt * locals.var_bnt))) * locals.var_em) + (assign5760_e5769 * locals.var_em_dn4)) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn4)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * ((((-locals.var_bnt_dn4) * locals.var_em) - (assign5760_e5775 * locals.var_em_dn4)) / (locals.var_em * locals.var_em))) - (assign5760_e5789 * ((((((-locals.var_bnt_dn4) * locals.var_em) - (assign5760_e5780 * locals.var_em_dn4)) / (locals.var_em * locals.var_em)) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn4 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn4)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5760_e5769 * locals.var_em_dn5) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn5)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn5) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn5) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn5 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn5)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5760_e5769 * locals.var_em_dn6) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn6)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn6) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn6) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn6 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn6)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5760_e5769 * locals.var_em_dn7) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn7)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn7) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn7) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn7 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn7)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5760_e5769 * locals.var_em_dn8) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn8)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn8) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn8) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn8 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn8)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5760_e5769 * locals.var_em_dn9) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn9)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn9) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn9) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn9 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn9)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5760_e5769 * locals.var_em_dn10) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn10)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn10) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn10) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn10 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn10)) / (locals.var_lambda * locals.var_lambda)))))))), (((((assign5760_e5769 * locals.var_em_dn11) * locals.var_lambda) + (assign5760_e5771 * locals.var_lambda_dn11)) * assign5760_e5790) + (assign5760_e5773 * ((assign5760_e5778 * (-((assign5760_e5775 * locals.var_em_dn11) / (locals.var_em * locals.var_em)))) - (assign5760_e5789 * (((-((assign5760_e5780 * locals.var_em_dn11) / (locals.var_em * locals.var_em))) * assign5760_e5787) + (assign5760_e5782 * (((locals.var_weff_dn11 * locals.var_lambda) - (locals.var_weff * locals.var_lambda_dn11)) / (locals.var_lambda * locals.var_lambda)))))))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10, locals.var_gem_dn11,)
    }
};
        locals.var_gem = assign5760_e5793;
        locals.var_gem_dn0 = assign5760_e5793_d_n0;
        locals.var_gem_dn1 = assign5760_e5793_d_n1;
        locals.var_gem_dn3 = assign5760_e5793_d_n3;
        locals.var_gem_dn4 = assign5760_e5793_d_n4;
        locals.var_gem_dn5 = assign5760_e5793_d_n5;
        locals.var_gem_dn6 = assign5760_e5793_d_n6;
        locals.var_gem_dn7 = assign5760_e5793_d_n7;
        locals.var_gem_dn8 = assign5760_e5793_d_n8;
        locals.var_gem_dn9 = assign5760_e5793_d_n9;
        locals.var_gem_dn10 = assign5760_e5793_d_n10;
        locals.var_gem_dn11 = assign5760_e5793_d_n11;
        locals.var_gem_rv = 0.0;

        let (assign5770_e5815, assign5770_e5815_d_n0, assign5770_e5815_d_n1, assign5770_e5815_d_n3, assign5770_e5815_d_n4, assign5770_e5815_d_n5, assign5770_e5815_d_n6, assign5770_e5815_d_n7, assign5770_e5815_d_n8, assign5770_e5815_d_n9, assign5770_e5815_d_n10, assign5770_e5815_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard101 == 0.0)) {
        let assign5770_e5807: f64 = (locals.var_an * locals.var_weff);
        let assign5770_e5809: f64 = (-locals.var_bnt);
        let assign5770_e5811: f64 = (assign5770_e5809 / locals.var_em);
        let assign5770_e5812: f64 = (assign5770_e5811).exp();
        let assign5770_e5813: f64 = (assign5770_e5807 * assign5770_e5812);
        (assign5770_e5813, (((locals.var_an * locals.var_weff_dn0) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn0) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn1) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn1) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn3) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn3) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn4) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * ((((-locals.var_bnt_dn4) * locals.var_em) - (assign5770_e5809 * locals.var_em_dn4)) / (locals.var_em * locals.var_em))))), (((locals.var_an * locals.var_weff_dn5) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn5) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn6) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn6) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn7) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn7) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn8) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn8) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn9) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn9) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn10) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn10) / (locals.var_em * locals.var_em)))))), (((locals.var_an * locals.var_weff_dn11) * assign5770_e5812) + (assign5770_e5807 * (assign5770_e5812 * (-((assign5770_e5809 * locals.var_em_dn11) / (locals.var_em * locals.var_em)))))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10, locals.var_gem_dn11,)
    }
};
        locals.var_gem = assign5770_e5815;
        locals.var_gem_dn0 = assign5770_e5815_d_n0;
        locals.var_gem_dn1 = assign5770_e5815_d_n1;
        locals.var_gem_dn3 = assign5770_e5815_d_n3;
        locals.var_gem_dn4 = assign5770_e5815_d_n4;
        locals.var_gem_dn5 = assign5770_e5815_d_n5;
        locals.var_gem_dn6 = assign5770_e5815_d_n6;
        locals.var_gem_dn7 = assign5770_e5815_d_n7;
        locals.var_gem_dn8 = assign5770_e5815_d_n8;
        locals.var_gem_dn9 = assign5770_e5815_d_n9;
        locals.var_gem_dn10 = assign5770_e5815_d_n10;
        locals.var_gem_dn11 = assign5770_e5815_d_n11;
        locals.var_gem_rv = 0.0;

        let assign5780_e5818: f64 = if p.p39 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard102 = assign5780_e5818;
        locals.var_guard102_rv = 0.0;

        let assign5790_e5821: f64 = if locals.var_vb2c1 < p.p44 { 1.0 } else { 0.0 };
        locals.var_guard103 = assign5790_e5821;
        locals.var_guard103_rv = 0.0;

        let (assign5800_e5849, assign5800_e5849_d_n0, assign5800_e5849_d_n1, assign5800_e5849_d_n3, assign5800_e5849_d_n4, assign5800_e5849_d_n5, assign5800_e5849_d_n6, assign5800_e5849_d_n7, assign5800_e5849_d_n8, assign5800_e5849_d_n9, assign5800_e5849_d_n10, assign5800_e5849_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) {
        let assign5800_e5835: f64 = (p.p44 - locals.var_vb2c1);
        let assign5800_e5837: f64 = (assign5800_e5835).powf(p.p41);
        let assign5800_e5842: f64 = (p.p48 + locals.var_in_);
        let assign5800_e5843: f64 = (locals.var_in_ / assign5800_e5842);
        let assign5800_e5844: f64 = (1.0 - assign5800_e5843);
        let assign5800_e5846: f64 = (assign5800_e5844).powf(p.p49);
        let assign5800_e5847: f64 = (assign5800_e5837 * assign5800_e5846);
        (assign5800_e5847, (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn0 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn0)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn0 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn0)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) }), (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn1 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn1)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn1 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn1)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) }), (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn3 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn3)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn3 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn3)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) }), (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn4 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn4)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn4 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn4)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) }), (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn5 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn5)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn5 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn5)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) }), (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn6 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn6)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn6 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn6)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) }), ((if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((assign5800_e5835).powf(p.p41 - 1.0) * (-locals.var_vb2c1_dn7))) } } else { (assign5800_e5837 * (p.p41 * ((-locals.var_vb2c1_dn7) / assign5800_e5835))) } * assign5800_e5846) + (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn7 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn7)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn7 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn7)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) })), ((if 0.0 == 0.0 && ((p.p41) as f64).is_finite() && ((p.p41) as f64).fract() == 0.0 { if p.p41 == 0.0 { 0.0 } else { (p.p41 * ((assign5800_e5835).powf(p.p41 - 1.0) * (-locals.var_vb2c1_dn8))) } } else { (assign5800_e5837 * (p.p41 * ((-locals.var_vb2c1_dn8) / assign5800_e5835))) } * assign5800_e5846) + (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn8 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn8)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn8 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn8)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) })), (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn9 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn9)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn9 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn9)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) }), (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn10 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn10)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn10 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn10)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) }), (assign5800_e5837 * if 0.0 == 0.0 && ((p.p49) as f64).is_finite() && ((p.p49) as f64).fract() == 0.0 { if p.p49 == 0.0 { 0.0 } else { (p.p49 * ((assign5800_e5844).powf(p.p49 - 1.0) * (-(((locals.var_in__dn11 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn11)) / (assign5800_e5842 * assign5800_e5842))))) } } else { (assign5800_e5846 * (p.p49 * ((-(((locals.var_in__dn11 * assign5800_e5842) - (locals.var_in_ * locals.var_in__dn11)) / (assign5800_e5842 * assign5800_e5842))) / assign5800_e5844))) }),)
    } else {
        (locals.var_vdeptmp, locals.var_vdeptmp_dn0, locals.var_vdeptmp_dn1, locals.var_vdeptmp_dn3, locals.var_vdeptmp_dn4, locals.var_vdeptmp_dn5, locals.var_vdeptmp_dn6, locals.var_vdeptmp_dn7, locals.var_vdeptmp_dn8, locals.var_vdeptmp_dn9, locals.var_vdeptmp_dn10, locals.var_vdeptmp_dn11,)
    }
};
        locals.var_vdeptmp = assign5800_e5849;
        locals.var_vdeptmp_dn0 = assign5800_e5849_d_n0;
        locals.var_vdeptmp_dn1 = assign5800_e5849_d_n1;
        locals.var_vdeptmp_dn3 = assign5800_e5849_d_n3;
        locals.var_vdeptmp_dn4 = assign5800_e5849_d_n4;
        locals.var_vdeptmp_dn5 = assign5800_e5849_d_n5;
        locals.var_vdeptmp_dn6 = assign5800_e5849_d_n6;
        locals.var_vdeptmp_dn7 = assign5800_e5849_d_n7;
        locals.var_vdeptmp_dn8 = assign5800_e5849_d_n8;
        locals.var_vdeptmp_dn9 = assign5800_e5849_d_n9;
        locals.var_vdeptmp_dn10 = assign5800_e5849_d_n10;
        locals.var_vdeptmp_dn11 = assign5800_e5849_d_n11;
        locals.var_vdeptmp_rv = 0.0;

        let assign5810_e5852: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard104 = assign5810_e5852;
        locals.var_guard104_rv = 0.0;

        let (assign5820_e5868, assign5820_e5868_d_n0, assign5820_e5868_d_n1, assign5820_e5868_d_n3, assign5820_e5868_d_n4, assign5820_e5868_d_n5, assign5820_e5868_d_n6, assign5820_e5868_d_n7, assign5820_e5868_d_n8, assign5820_e5868_d_n9, assign5820_e5868_d_n10, assign5820_e5868_d_n11,) = {
    if ((((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard104 != 0.0)) {
        (locals.var_vdeptmp, locals.var_vdeptmp_dn0, locals.var_vdeptmp_dn1, locals.var_vdeptmp_dn3, locals.var_vdeptmp_dn4, locals.var_vdeptmp_dn5, locals.var_vdeptmp_dn6, locals.var_vdeptmp_dn7, locals.var_vdeptmp_dn8, locals.var_vdeptmp_dn9, locals.var_vdeptmp_dn10, locals.var_vdeptmp_dn11,)
    } else {
        (locals.var_vdep, locals.var_vdep_dn0, locals.var_vdep_dn1, locals.var_vdep_dn3, locals.var_vdep_dn4, locals.var_vdep_dn5, locals.var_vdep_dn6, locals.var_vdep_dn7, locals.var_vdep_dn8, locals.var_vdep_dn9, locals.var_vdep_dn10, locals.var_vdep_dn11,)
    }
};
        locals.var_vdep = assign5820_e5868;
        locals.var_vdep_dn0 = assign5820_e5868_d_n0;
        locals.var_vdep_dn1 = assign5820_e5868_d_n1;
        locals.var_vdep_dn3 = assign5820_e5868_d_n3;
        locals.var_vdep_dn4 = assign5820_e5868_d_n4;
        locals.var_vdep_dn5 = assign5820_e5868_d_n5;
        locals.var_vdep_dn6 = assign5820_e5868_d_n6;
        locals.var_vdep_dn7 = assign5820_e5868_d_n7;
        locals.var_vdep_dn8 = assign5820_e5868_d_n8;
        locals.var_vdep_dn9 = assign5820_e5868_d_n9;
        locals.var_vdep_dn10 = assign5820_e5868_d_n10;
        locals.var_vdep_dn11 = assign5820_e5868_d_n11;
        locals.var_vdep_rv = 0.0;

        let (assign5830_e5889, assign5830_e5889_d_n0, assign5830_e5889_d_n1, assign5830_e5889_d_n3, assign5830_e5889_d_n4, assign5830_e5889_d_n5, assign5830_e5889_d_n6, assign5830_e5889_d_n7, assign5830_e5889_d_n8, assign5830_e5889_d_n9, assign5830_e5889_d_n10, assign5830_e5889_d_n11,) = {
    if ((((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard104 == 0.0)) {
        let assign5830_e5885: f64 = (locals.var_in_ - p.p52);
        let assign5830_e5887: f64 = (assign5830_e5885 / p.p48);
        (assign5830_e5887, (locals.var_in__dn0 / p.p48), (locals.var_in__dn1 / p.p48), (locals.var_in__dn3 / p.p48), (locals.var_in__dn4 / p.p48), (locals.var_in__dn5 / p.p48), (locals.var_in__dn6 / p.p48), (locals.var_in__dn7 / p.p48), (locals.var_in__dn8 / p.p48), (locals.var_in__dn9 / p.p48), (locals.var_in__dn10 / p.p48), (locals.var_in__dn11 / p.p48),)
    } else {
        (locals.var_in_shift_ihcavl, locals.var_in_shift_ihcavl_dn0, locals.var_in_shift_ihcavl_dn1, locals.var_in_shift_ihcavl_dn3, locals.var_in_shift_ihcavl_dn4, locals.var_in_shift_ihcavl_dn5, locals.var_in_shift_ihcavl_dn6, locals.var_in_shift_ihcavl_dn7, locals.var_in_shift_ihcavl_dn8, locals.var_in_shift_ihcavl_dn9, locals.var_in_shift_ihcavl_dn10, locals.var_in_shift_ihcavl_dn11,)
    }
};
        locals.var_in_shift_ihcavl = assign5830_e5889;
        locals.var_in_shift_ihcavl_dn0 = assign5830_e5889_d_n0;
        locals.var_in_shift_ihcavl_dn1 = assign5830_e5889_d_n1;
        locals.var_in_shift_ihcavl_dn3 = assign5830_e5889_d_n3;
        locals.var_in_shift_ihcavl_dn4 = assign5830_e5889_d_n4;
        locals.var_in_shift_ihcavl_dn5 = assign5830_e5889_d_n5;
        locals.var_in_shift_ihcavl_dn6 = assign5830_e5889_d_n6;
        locals.var_in_shift_ihcavl_dn7 = assign5830_e5889_d_n7;
        locals.var_in_shift_ihcavl_dn8 = assign5830_e5889_d_n8;
        locals.var_in_shift_ihcavl_dn9 = assign5830_e5889_d_n9;
        locals.var_in_shift_ihcavl_dn10 = assign5830_e5889_d_n10;
        locals.var_in_shift_ihcavl_dn11 = assign5830_e5889_d_n11;
        locals.var_in_shift_ihcavl_rv = 0.0;

        let (assign5840_e5910, assign5840_e5910_d_n0, assign5840_e5910_d_n1, assign5840_e5910_d_n3, assign5840_e5910_d_n4, assign5840_e5910_d_n5, assign5840_e5910_d_n6, assign5840_e5910_d_n7, assign5840_e5910_d_n8, assign5840_e5910_d_n9, assign5840_e5910_d_n10, assign5840_e5910_d_n11,) = {
    if ((((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard104 == 0.0)) {
        let assign5840_e5906: f64 = (locals.var_in_shift_ihcavl - 1.0);
        let assign5840_e5908: f64 = (assign5840_e5906 / p.p51);
        (assign5840_e5908, (locals.var_in_shift_ihcavl_dn0 / p.p51), (locals.var_in_shift_ihcavl_dn1 / p.p51), (locals.var_in_shift_ihcavl_dn3 / p.p51), (locals.var_in_shift_ihcavl_dn4 / p.p51), (locals.var_in_shift_ihcavl_dn5 / p.p51), (locals.var_in_shift_ihcavl_dn6 / p.p51), (locals.var_in_shift_ihcavl_dn7 / p.p51), (locals.var_in_shift_ihcavl_dn8 / p.p51), (locals.var_in_shift_ihcavl_dn9 / p.p51), (locals.var_in_shift_ihcavl_dn10 / p.p51), (locals.var_in_shift_ihcavl_dn11 / p.p51),)
    } else {
        (locals.var_dxa, locals.var_dxa_dn0, locals.var_dxa_dn1, locals.var_dxa_dn3, locals.var_dxa_dn4, locals.var_dxa_dn5, locals.var_dxa_dn6, locals.var_dxa_dn7, locals.var_dxa_dn8, locals.var_dxa_dn9, locals.var_dxa_dn10, locals.var_dxa_dn11,)
    }
};
        locals.var_dxa = assign5840_e5910;
        locals.var_dxa_dn0 = assign5840_e5910_d_n0;
        locals.var_dxa_dn1 = assign5840_e5910_d_n1;
        locals.var_dxa_dn3 = assign5840_e5910_d_n3;
        locals.var_dxa_dn4 = assign5840_e5910_d_n4;
        locals.var_dxa_dn5 = assign5840_e5910_d_n5;
        locals.var_dxa_dn6 = assign5840_e5910_d_n6;
        locals.var_dxa_dn7 = assign5840_e5910_d_n7;
        locals.var_dxa_dn8 = assign5840_e5910_d_n8;
        locals.var_dxa_dn9 = assign5840_e5910_d_n9;
        locals.var_dxa_dn10 = assign5840_e5910_d_n10;
        locals.var_dxa_dn11 = assign5840_e5910_d_n11;
        locals.var_dxa_rv = 0.0;

        let assign5850_e5913: f64 = if locals.var_in_shift_ihcavl < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign5850_e5913;
        locals.var_guard105_rv = 0.0;

        let (assign5860_e5940, assign5860_e5940_d_n0, assign5860_e5940_d_n1, assign5860_e5940_d_n3, assign5860_e5940_d_n4, assign5860_e5940_d_n5, assign5860_e5940_d_n6, assign5860_e5940_d_n7, assign5860_e5940_d_n8, assign5860_e5940_d_n9, assign5860_e5940_d_n10, assign5860_e5940_d_n11,) = {
    if (((((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard104 == 0.0)) && (locals.var_guard105 != 0.0)) {
        let assign5860_e5934: f64 = (locals.var_dxa).exp();
        let assign5860_e5935: f64 = (1.0 + assign5860_e5934);
        let assign5860_e5936: f64 = (assign5860_e5935).ln();
        let assign5860_e5937: f64 = (p.p51 * assign5860_e5936);
        let assign5860_e5938: f64 = (1.0 + assign5860_e5937);
        (assign5860_e5938, (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn0) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn1) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn3) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn4) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn5) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn6) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn7) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn8) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn9) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn10) / assign5860_e5935)), (p.p51 * ((assign5860_e5934 * locals.var_dxa_dn11) / assign5860_e5935)),)
    } else {
        (locals.var_in_shift_n, locals.var_in_shift_n_dn0, locals.var_in_shift_n_dn1, locals.var_in_shift_n_dn3, locals.var_in_shift_n_dn4, locals.var_in_shift_n_dn5, locals.var_in_shift_n_dn6, locals.var_in_shift_n_dn7, locals.var_in_shift_n_dn8, locals.var_in_shift_n_dn9, locals.var_in_shift_n_dn10, locals.var_in_shift_n_dn11,)
    }
};
        locals.var_in_shift_n = assign5860_e5940;
        locals.var_in_shift_n_dn0 = assign5860_e5940_d_n0;
        locals.var_in_shift_n_dn1 = assign5860_e5940_d_n1;
        locals.var_in_shift_n_dn3 = assign5860_e5940_d_n3;
        locals.var_in_shift_n_dn4 = assign5860_e5940_d_n4;
        locals.var_in_shift_n_dn5 = assign5860_e5940_d_n5;
        locals.var_in_shift_n_dn6 = assign5860_e5940_d_n6;
        locals.var_in_shift_n_dn7 = assign5860_e5940_d_n7;
        locals.var_in_shift_n_dn8 = assign5860_e5940_d_n8;
        locals.var_in_shift_n_dn9 = assign5860_e5940_d_n9;
        locals.var_in_shift_n_dn10 = assign5860_e5940_d_n10;
        locals.var_in_shift_n_dn11 = assign5860_e5940_d_n11;
        locals.var_in_shift_n_rv = 0.0;

        let (assign5870_e5969, assign5870_e5969_d_n0, assign5870_e5969_d_n1, assign5870_e5969_d_n3, assign5870_e5969_d_n4, assign5870_e5969_d_n5, assign5870_e5969_d_n6, assign5870_e5969_d_n7, assign5870_e5969_d_n8, assign5870_e5969_d_n9, assign5870_e5969_d_n10, assign5870_e5969_d_n11,) = {
    if (((((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard104 == 0.0)) && (locals.var_guard105 == 0.0)) {
        let assign5870_e5962: f64 = (-locals.var_dxa);
        let assign5870_e5963: f64 = (assign5870_e5962).exp();
        let assign5870_e5964: f64 = (1.0 + assign5870_e5963);
        let assign5870_e5965: f64 = (assign5870_e5964).ln();
        let assign5870_e5966: f64 = (p.p51 * assign5870_e5965);
        let assign5870_e5967: f64 = (locals.var_in_shift_ihcavl + assign5870_e5966);
        (assign5870_e5967, (locals.var_in_shift_ihcavl_dn0 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn0)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn1 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn1)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn3 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn3)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn4 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn4)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn5 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn5)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn6 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn6)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn7 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn7)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn8 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn8)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn9 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn9)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn10 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn10)) / assign5870_e5964))), (locals.var_in_shift_ihcavl_dn11 + (p.p51 * ((assign5870_e5963 * (-locals.var_dxa_dn11)) / assign5870_e5964))),)
    } else {
        (locals.var_in_shift_n, locals.var_in_shift_n_dn0, locals.var_in_shift_n_dn1, locals.var_in_shift_n_dn3, locals.var_in_shift_n_dn4, locals.var_in_shift_n_dn5, locals.var_in_shift_n_dn6, locals.var_in_shift_n_dn7, locals.var_in_shift_n_dn8, locals.var_in_shift_n_dn9, locals.var_in_shift_n_dn10, locals.var_in_shift_n_dn11,)
    }
};
        locals.var_in_shift_n = assign5870_e5969;
        locals.var_in_shift_n_dn0 = assign5870_e5969_d_n0;
        locals.var_in_shift_n_dn1 = assign5870_e5969_d_n1;
        locals.var_in_shift_n_dn3 = assign5870_e5969_d_n3;
        locals.var_in_shift_n_dn4 = assign5870_e5969_d_n4;
        locals.var_in_shift_n_dn5 = assign5870_e5969_d_n5;
        locals.var_in_shift_n_dn6 = assign5870_e5969_d_n6;
        locals.var_in_shift_n_dn7 = assign5870_e5969_d_n7;
        locals.var_in_shift_n_dn8 = assign5870_e5969_d_n8;
        locals.var_in_shift_n_dn9 = assign5870_e5969_d_n9;
        locals.var_in_shift_n_dn10 = assign5870_e5969_d_n10;
        locals.var_in_shift_n_dn11 = assign5870_e5969_d_n11;
        locals.var_in_shift_n_rv = 0.0;

        let (assign5880_e5990, assign5880_e5990_d_n0, assign5880_e5990_d_n1, assign5880_e5990_d_n3, assign5880_e5990_d_n4, assign5880_e5990_d_n5, assign5880_e5990_d_n6, assign5880_e5990_d_n7, assign5880_e5990_d_n8, assign5880_e5990_d_n9, assign5880_e5990_d_n10, assign5880_e5990_d_n11,) = {
    if ((((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard104 == 0.0)) {
        let assign5880_e5987: f64 = (locals.var_in_shift_n).powf(p.p50);
        let assign5880_e5988: f64 = (locals.var_vdeptmp * assign5880_e5987);
        (assign5880_e5988, ((locals.var_vdeptmp_dn0 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn0)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn0 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn1 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn1)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn1 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn3 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn3)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn3 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn4 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn4)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn4 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn5 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn5)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn5 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn6 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn6)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn6 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn7 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn7)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn7 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn8 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn8)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn8 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn9 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn9)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn9 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn10 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn10)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn10 / locals.var_in_shift_n))) })), ((locals.var_vdeptmp_dn11 * assign5880_e5987) + (locals.var_vdeptmp * if 0.0 == 0.0 && ((p.p50) as f64).is_finite() && ((p.p50) as f64).fract() == 0.0 { if p.p50 == 0.0 { 0.0 } else { (p.p50 * ((locals.var_in_shift_n).powf(p.p50 - 1.0) * locals.var_in_shift_n_dn11)) } } else { (assign5880_e5987 * (p.p50 * (locals.var_in_shift_n_dn11 / locals.var_in_shift_n))) })),)
    } else {
        (locals.var_vdep, locals.var_vdep_dn0, locals.var_vdep_dn1, locals.var_vdep_dn3, locals.var_vdep_dn4, locals.var_vdep_dn5, locals.var_vdep_dn6, locals.var_vdep_dn7, locals.var_vdep_dn8, locals.var_vdep_dn9, locals.var_vdep_dn10, locals.var_vdep_dn11,)
    }
};
        locals.var_vdep = assign5880_e5990;
        locals.var_vdep_dn0 = assign5880_e5990_d_n0;
        locals.var_vdep_dn1 = assign5880_e5990_d_n1;
        locals.var_vdep_dn3 = assign5880_e5990_d_n3;
        locals.var_vdep_dn4 = assign5880_e5990_d_n4;
        locals.var_vdep_dn5 = assign5880_e5990_d_n5;
        locals.var_vdep_dn6 = assign5880_e5990_d_n6;
        locals.var_vdep_dn7 = assign5880_e5990_d_n7;
        locals.var_vdep_dn8 = assign5880_e5990_d_n8;
        locals.var_vdep_dn9 = assign5880_e5990_d_n9;
        locals.var_vdep_dn10 = assign5880_e5990_d_n10;
        locals.var_vdep_dn11 = assign5880_e5990_d_n11;
        locals.var_vdep_rv = 0.0;

        let assign5890_e5992: f64 = (-locals.var_bavl_t);
        let assign5890_e5994: f64 = (assign5890_e5992 * locals.var_vdep);
        let assign5890_e5996: f64 = if assign5890_e5994 < p.p151 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign5890_e5996;
        locals.var_guard106_rv = 0.0;

        let (assign5900_e6016, assign5900_e6016_d_n0, assign5900_e6016_d_n1, assign5900_e6016_d_n3, assign5900_e6016_d_n4, assign5900_e6016_d_n5, assign5900_e6016_d_n6, assign5900_e6016_d_n7, assign5900_e6016_d_n8, assign5900_e6016_d_n9, assign5900_e6016_d_n10, assign5900_e6016_d_n11,) = {
    if ((((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard106 != 0.0)) {
        let assign5900_e6011: f64 = (-locals.var_bavl_t);
        let assign5900_e6013: f64 = (assign5900_e6011 * locals.var_vdep);
        let assign5900_e6014: f64 = (assign5900_e6013).exp();
        (assign5900_e6014, (assign5900_e6014 * (((-locals.var_bavl_t_dn0) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn0))), (assign5900_e6014 * (((-locals.var_bavl_t_dn1) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn1))), (assign5900_e6014 * (((-locals.var_bavl_t_dn3) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn3))), (assign5900_e6014 * (((-locals.var_bavl_t_dn4) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn4))), (assign5900_e6014 * (((-locals.var_bavl_t_dn5) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn5))), (assign5900_e6014 * (((-locals.var_bavl_t_dn6) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn6))), (assign5900_e6014 * (((-locals.var_bavl_t_dn7) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn7))), (assign5900_e6014 * (((-locals.var_bavl_t_dn8) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn8))), (assign5900_e6014 * (((-locals.var_bavl_t_dn9) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn9))), (assign5900_e6014 * (((-locals.var_bavl_t_dn10) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn10))), (assign5900_e6014 * (((-locals.var_bavl_t_dn11) * locals.var_vdep) + (assign5900_e6011 * locals.var_vdep_dn11))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9, locals.var_expmm1_dn10, locals.var_expmm1_dn11,)
    }
};
        locals.var_expmm1 = assign5900_e6016;
        locals.var_expmm1_dn0 = assign5900_e6016_d_n0;
        locals.var_expmm1_dn1 = assign5900_e6016_d_n1;
        locals.var_expmm1_dn3 = assign5900_e6016_d_n3;
        locals.var_expmm1_dn4 = assign5900_e6016_d_n4;
        locals.var_expmm1_dn5 = assign5900_e6016_d_n5;
        locals.var_expmm1_dn6 = assign5900_e6016_d_n6;
        locals.var_expmm1_dn7 = assign5900_e6016_d_n7;
        locals.var_expmm1_dn8 = assign5900_e6016_d_n8;
        locals.var_expmm1_dn9 = assign5900_e6016_d_n9;
        locals.var_expmm1_dn10 = assign5900_e6016_d_n10;
        locals.var_expmm1_dn11 = assign5900_e6016_d_n11;
        locals.var_expmm1_rv = 0.0;

        let (assign5910_e6034,) = {
    if ((((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard106 == 0.0)) {
        let assign5910_e6032: f64 = (p.p151).exp();
        (assign5910_e6032,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign5910_e6034;
        locals.var_expl_rv = 0.0;

        let (assign5920_e6060, assign5920_e6060_d_n0, assign5920_e6060_d_n1, assign5920_e6060_d_n3, assign5920_e6060_d_n4, assign5920_e6060_d_n5, assign5920_e6060_d_n6, assign5920_e6060_d_n7, assign5920_e6060_d_n8, assign5920_e6060_d_n9, assign5920_e6060_d_n10, assign5920_e6060_d_n11,) = {
    if ((((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard106 == 0.0)) {
        let assign5920_e6052: f64 = (-locals.var_bavl_t);
        let assign5920_e6054: f64 = (assign5920_e6052 * locals.var_vdep);
        let assign5920_e6056: f64 = (assign5920_e6054 - p.p151);
        let assign5920_e6057: f64 = (1.0 + assign5920_e6056);
        let assign5920_e6058: f64 = (locals.var_expl * assign5920_e6057);
        (assign5920_e6058, (locals.var_expl * (((-locals.var_bavl_t_dn0) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn0))), (locals.var_expl * (((-locals.var_bavl_t_dn1) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn1))), (locals.var_expl * (((-locals.var_bavl_t_dn3) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn3))), (locals.var_expl * (((-locals.var_bavl_t_dn4) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn4))), (locals.var_expl * (((-locals.var_bavl_t_dn5) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn5))), (locals.var_expl * (((-locals.var_bavl_t_dn6) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn6))), (locals.var_expl * (((-locals.var_bavl_t_dn7) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn7))), (locals.var_expl * (((-locals.var_bavl_t_dn8) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn8))), (locals.var_expl * (((-locals.var_bavl_t_dn9) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn9))), (locals.var_expl * (((-locals.var_bavl_t_dn10) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn10))), (locals.var_expl * (((-locals.var_bavl_t_dn11) * locals.var_vdep) + (assign5920_e6052 * locals.var_vdep_dn11))),)
    } else {
        (locals.var_expmm1, locals.var_expmm1_dn0, locals.var_expmm1_dn1, locals.var_expmm1_dn3, locals.var_expmm1_dn4, locals.var_expmm1_dn5, locals.var_expmm1_dn6, locals.var_expmm1_dn7, locals.var_expmm1_dn8, locals.var_expmm1_dn9, locals.var_expmm1_dn10, locals.var_expmm1_dn11,)
    }
};
        locals.var_expmm1 = assign5920_e6060;
        locals.var_expmm1_dn0 = assign5920_e6060_d_n0;
        locals.var_expmm1_dn1 = assign5920_e6060_d_n1;
        locals.var_expmm1_dn3 = assign5920_e6060_d_n3;
        locals.var_expmm1_dn4 = assign5920_e6060_d_n4;
        locals.var_expmm1_dn5 = assign5920_e6060_d_n5;
        locals.var_expmm1_dn6 = assign5920_e6060_d_n6;
        locals.var_expmm1_dn7 = assign5920_e6060_d_n7;
        locals.var_expmm1_dn8 = assign5920_e6060_d_n8;
        locals.var_expmm1_dn9 = assign5920_e6060_d_n9;
        locals.var_expmm1_dn10 = assign5920_e6060_d_n10;
        locals.var_expmm1_dn11 = assign5920_e6060_d_n11;
        locals.var_expmm1_rv = 0.0;

        let (assign5930_e6082, assign5930_e6082_d_n0, assign5930_e6082_d_n1, assign5930_e6082_d_n3, assign5930_e6082_d_n4, assign5930_e6082_d_n5, assign5930_e6082_d_n6, assign5930_e6082_d_n7, assign5930_e6082_d_n8, assign5930_e6082_d_n9, assign5930_e6082_d_n10, assign5930_e6082_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard93 == 0.0)) && (locals.var_guard97 == 0.0)) && (locals.var_guard102 != 0.0)) && (locals.var_guard103 != 0.0)) {
        let assign5930_e6074: f64 = (p.p40 / locals.var_bavl_t);
        let assign5930_e6077: f64 = (p.p44 - locals.var_vb2c1);
        let assign5930_e6078: f64 = (assign5930_e6074 * assign5930_e6077);
        let assign5930_e6080: f64 = (assign5930_e6078 * locals.var_expmm1);
        (assign5930_e6080, ((((-((p.p40 * locals.var_bavl_t_dn0) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn0)), ((((-((p.p40 * locals.var_bavl_t_dn1) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn1)), ((((-((p.p40 * locals.var_bavl_t_dn3) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn3)), ((((-((p.p40 * locals.var_bavl_t_dn4) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn4)), ((((-((p.p40 * locals.var_bavl_t_dn5) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn5)), ((((-((p.p40 * locals.var_bavl_t_dn6) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn6)), (((((-((p.p40 * locals.var_bavl_t_dn7) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) + (assign5930_e6074 * (-locals.var_vb2c1_dn7))) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn7)), (((((-((p.p40 * locals.var_bavl_t_dn8) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) + (assign5930_e6074 * (-locals.var_vb2c1_dn8))) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn8)), ((((-((p.p40 * locals.var_bavl_t_dn9) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn9)), ((((-((p.p40 * locals.var_bavl_t_dn10) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn10)), ((((-((p.p40 * locals.var_bavl_t_dn11) / (locals.var_bavl_t * locals.var_bavl_t))) * assign5930_e6077) * locals.var_expmm1) + (assign5930_e6078 * locals.var_expmm1_dn11)),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10, locals.var_gem_dn11,)
    }
};
        locals.var_gem = assign5930_e6082;
        locals.var_gem_dn0 = assign5930_e6082_d_n0;
        locals.var_gem_dn1 = assign5930_e6082_d_n1;
        locals.var_gem_dn3 = assign5930_e6082_d_n3;
        locals.var_gem_dn4 = assign5930_e6082_d_n4;
        locals.var_gem_dn5 = assign5930_e6082_d_n5;
        locals.var_gem_dn6 = assign5930_e6082_d_n6;
        locals.var_gem_dn7 = assign5930_e6082_d_n7;
        locals.var_gem_dn8 = assign5930_e6082_d_n8;
        locals.var_gem_dn9 = assign5930_e6082_d_n9;
        locals.var_gem_dn10 = assign5930_e6082_d_n10;
        locals.var_gem_dn11 = assign5930_e6082_d_n11;
        locals.var_gem_rv = 0.0;

        let assign5940_e6085: f64 = if locals.var_gem > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign5940_e6085;
        locals.var_guard107_rv = 0.0;

        let assign5950_e6088: f64 = if p.p53 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign5950_e6088;
        locals.var_guard108_rv = 0.0;

        let (assign5960_e6114, assign5960_e6114_d_n0, assign5960_e6114_d_n1, assign5960_e6114_d_n3, assign5960_e6114_d_n4, assign5960_e6114_d_n5, assign5960_e6114_d_n6, assign5960_e6114_d_n7, assign5960_e6114_d_n8, assign5960_e6114_d_n9, assign5960_e6114_d_n10, assign5960_e6114_d_n11,) = {
    if (((locals.var_guard92 != 0.0) && (locals.var_guard107 != 0.0)) && (locals.var_guard108 != 0.0)) {
        let assign5960_e6098: f64 = (locals.var_rbc_t + locals.var_rb2);
        let assign5960_e6099: f64 = (locals.var_in_ * assign5960_e6098);
        let assign5960_e6100: f64 = (locals.var_vt / assign5960_e6099);
        let assign5960_e6103: f64 = (locals.var_qbi / locals.var_is_t);
        let assign5960_e6105: f64 = (assign5960_e6103 * locals.var_ibi_t);
        let assign5960_e6106: f64 = (assign5960_e6100 + assign5960_e6105);
        let assign5960_e6110: f64 = (locals.var_rbc_t + locals.var_rb2);
        let assign5960_e6111: f64 = (locals.var_re_t / assign5960_e6110);
        let assign5960_e6112: f64 = (assign5960_e6106 + assign5960_e6111);
        (assign5960_e6112, (((-((locals.var_vt * ((locals.var_in__dn0 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn0))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn0 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn0)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn0) / (assign5960_e6110 * assign5960_e6110)))), (((-((locals.var_vt * ((locals.var_in__dn1 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn1))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn1 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn1)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn1) / (assign5960_e6110 * assign5960_e6110)))), (((-((locals.var_vt * ((locals.var_in__dn3 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn3))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn3 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn3)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn3) / (assign5960_e6110 * assign5960_e6110)))), (((((locals.var_vt_dn4 * assign5960_e6099) - (locals.var_vt * ((locals.var_in__dn4 * assign5960_e6098) + (locals.var_in_ * (locals.var_rbc_t_dn4 + locals.var_rb2_dn4))))) / (assign5960_e6099 * assign5960_e6099)) + (((((locals.var_qbi_dn4 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn4)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t) + (assign5960_e6103 * locals.var_ibi_t_dn4))) + (((locals.var_re_t_dn4 * assign5960_e6110) - (locals.var_re_t * (locals.var_rbc_t_dn4 + locals.var_rb2_dn4))) / (assign5960_e6110 * assign5960_e6110))), (((-((locals.var_vt * ((locals.var_in__dn5 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn5))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn5 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn5)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn5) / (assign5960_e6110 * assign5960_e6110)))), (((-((locals.var_vt * ((locals.var_in__dn6 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn6))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn6 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn6)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn6) / (assign5960_e6110 * assign5960_e6110)))), (((-((locals.var_vt * ((locals.var_in__dn7 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn7))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn7 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn7)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn7) / (assign5960_e6110 * assign5960_e6110)))), (((-((locals.var_vt * ((locals.var_in__dn8 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn8))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn8 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn8)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn8) / (assign5960_e6110 * assign5960_e6110)))), (((-((locals.var_vt * ((locals.var_in__dn9 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn9))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn9 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn9)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn9) / (assign5960_e6110 * assign5960_e6110)))), (((-((locals.var_vt * ((locals.var_in__dn10 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn10))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn10 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn10)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn10) / (assign5960_e6110 * assign5960_e6110)))), (((-((locals.var_vt * ((locals.var_in__dn11 * assign5960_e6098) + (locals.var_in_ * locals.var_rb2_dn11))) / (assign5960_e6099 * assign5960_e6099))) + ((((locals.var_qbi_dn11 * locals.var_is_t) - (locals.var_qbi * locals.var_is_t_dn11)) / (locals.var_is_t * locals.var_is_t)) * locals.var_ibi_t)) + (-((locals.var_re_t * locals.var_rb2_dn11) / (assign5960_e6110 * assign5960_e6110)))),)
    } else {
        (locals.var_gmax, locals.var_gmax_dn0, locals.var_gmax_dn1, locals.var_gmax_dn3, locals.var_gmax_dn4, locals.var_gmax_dn5, locals.var_gmax_dn6, locals.var_gmax_dn7, locals.var_gmax_dn8, locals.var_gmax_dn9, locals.var_gmax_dn10, locals.var_gmax_dn11,)
    }
};
        locals.var_gmax = assign5960_e6114;
        locals.var_gmax_dn0 = assign5960_e6114_d_n0;
        locals.var_gmax_dn1 = assign5960_e6114_d_n1;
        locals.var_gmax_dn3 = assign5960_e6114_d_n3;
        locals.var_gmax_dn4 = assign5960_e6114_d_n4;
        locals.var_gmax_dn5 = assign5960_e6114_d_n5;
        locals.var_gmax_dn6 = assign5960_e6114_d_n6;
        locals.var_gmax_dn7 = assign5960_e6114_d_n7;
        locals.var_gmax_dn8 = assign5960_e6114_d_n8;
        locals.var_gmax_dn9 = assign5960_e6114_d_n9;
        locals.var_gmax_dn10 = assign5960_e6114_d_n10;
        locals.var_gmax_dn11 = assign5960_e6114_d_n11;
        locals.var_gmax_rv = 0.0;

        let assign5970_e6117: f64 = if p.p39 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard109 = assign5970_e6117;
        locals.var_guard109_rv = 0.0;

        let (assign5980_e6131, assign5980_e6131_d_n0, assign5980_e6131_d_n1, assign5980_e6131_d_n3, assign5980_e6131_d_n4, assign5980_e6131_d_n5, assign5980_e6131_d_n6, assign5980_e6131_d_n7, assign5980_e6131_d_n8, assign5980_e6131_d_n9, assign5980_e6131_d_n10, assign5980_e6131_d_n11,) = {
    if ((((locals.var_guard92 != 0.0) && (locals.var_guard107 != 0.0)) && (locals.var_guard108 != 0.0)) && (locals.var_guard109 != 0.0)) {
        let assign5980_e6127: f64 = (locals.var_gem - locals.var_gmax);
        let assign5980_e6129: f64 = (assign5980_e6127 / 1e-6);
        (assign5980_e6129, ((locals.var_gem_dn0 - locals.var_gmax_dn0) / 1e-6), ((locals.var_gem_dn1 - locals.var_gmax_dn1) / 1e-6), ((locals.var_gem_dn3 - locals.var_gmax_dn3) / 1e-6), ((locals.var_gem_dn4 - locals.var_gmax_dn4) / 1e-6), ((locals.var_gem_dn5 - locals.var_gmax_dn5) / 1e-6), ((locals.var_gem_dn6 - locals.var_gmax_dn6) / 1e-6), ((locals.var_gem_dn7 - locals.var_gmax_dn7) / 1e-6), ((locals.var_gem_dn8 - locals.var_gmax_dn8) / 1e-6), ((locals.var_gem_dn9 - locals.var_gmax_dn9) / 1e-6), ((locals.var_gem_dn10 - locals.var_gmax_dn10) / 1e-6), ((locals.var_gem_dn11 - locals.var_gmax_dn11) / 1e-6),)
    } else {
        (locals.var_dxa, locals.var_dxa_dn0, locals.var_dxa_dn1, locals.var_dxa_dn3, locals.var_dxa_dn4, locals.var_dxa_dn5, locals.var_dxa_dn6, locals.var_dxa_dn7, locals.var_dxa_dn8, locals.var_dxa_dn9, locals.var_dxa_dn10, locals.var_dxa_dn11,)
    }
};
        locals.var_dxa = assign5980_e6131;
        locals.var_dxa_dn0 = assign5980_e6131_d_n0;
        locals.var_dxa_dn1 = assign5980_e6131_d_n1;
        locals.var_dxa_dn3 = assign5980_e6131_d_n3;
        locals.var_dxa_dn4 = assign5980_e6131_d_n4;
        locals.var_dxa_dn5 = assign5980_e6131_d_n5;
        locals.var_dxa_dn6 = assign5980_e6131_d_n6;
        locals.var_dxa_dn7 = assign5980_e6131_d_n7;
        locals.var_dxa_dn8 = assign5980_e6131_d_n8;
        locals.var_dxa_dn9 = assign5980_e6131_d_n9;
        locals.var_dxa_dn10 = assign5980_e6131_d_n10;
        locals.var_dxa_dn11 = assign5980_e6131_d_n11;
        locals.var_dxa_rv = 0.0;

        let assign5990_e6134: f64 = if locals.var_gem < locals.var_gmax { 1.0 } else { 0.0 };
        locals.var_guard110 = assign5990_e6134;
        locals.var_guard110_rv = 0.0;

        let (assign6000_e6154, assign6000_e6154_d_n0, assign6000_e6154_d_n1, assign6000_e6154_d_n3, assign6000_e6154_d_n4, assign6000_e6154_d_n5, assign6000_e6154_d_n6, assign6000_e6154_d_n7, assign6000_e6154_d_n8, assign6000_e6154_d_n9, assign6000_e6154_d_n10, assign6000_e6154_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard107 != 0.0)) && (locals.var_guard108 != 0.0)) && (locals.var_guard109 != 0.0)) && (locals.var_guard110 != 0.0)) {
        let assign6000_e6148: f64 = (locals.var_dxa).exp();
        let assign6000_e6149: f64 = (1.0 + assign6000_e6148);
        let assign6000_e6150: f64 = (assign6000_e6149).ln();
        let assign6000_e6151: f64 = (1e-6 * assign6000_e6150);
        let assign6000_e6152: f64 = (locals.var_gem - assign6000_e6151);
        (assign6000_e6152, (locals.var_gem_dn0 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn0) / assign6000_e6149))), (locals.var_gem_dn1 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn1) / assign6000_e6149))), (locals.var_gem_dn3 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn3) / assign6000_e6149))), (locals.var_gem_dn4 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn4) / assign6000_e6149))), (locals.var_gem_dn5 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn5) / assign6000_e6149))), (locals.var_gem_dn6 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn6) / assign6000_e6149))), (locals.var_gem_dn7 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn7) / assign6000_e6149))), (locals.var_gem_dn8 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn8) / assign6000_e6149))), (locals.var_gem_dn9 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn9) / assign6000_e6149))), (locals.var_gem_dn10 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn10) / assign6000_e6149))), (locals.var_gem_dn11 - (1e-6 * ((assign6000_e6148 * locals.var_dxa_dn11) / assign6000_e6149))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10, locals.var_gem_dn11,)
    }
};
        locals.var_gem = assign6000_e6154;
        locals.var_gem_dn0 = assign6000_e6154_d_n0;
        locals.var_gem_dn1 = assign6000_e6154_d_n1;
        locals.var_gem_dn3 = assign6000_e6154_d_n3;
        locals.var_gem_dn4 = assign6000_e6154_d_n4;
        locals.var_gem_dn5 = assign6000_e6154_d_n5;
        locals.var_gem_dn6 = assign6000_e6154_d_n6;
        locals.var_gem_dn7 = assign6000_e6154_d_n7;
        locals.var_gem_dn8 = assign6000_e6154_d_n8;
        locals.var_gem_dn9 = assign6000_e6154_d_n9;
        locals.var_gem_dn10 = assign6000_e6154_d_n10;
        locals.var_gem_dn11 = assign6000_e6154_d_n11;
        locals.var_gem_rv = 0.0;

        let (assign6010_e6176, assign6010_e6176_d_n0, assign6010_e6176_d_n1, assign6010_e6176_d_n3, assign6010_e6176_d_n4, assign6010_e6176_d_n5, assign6010_e6176_d_n6, assign6010_e6176_d_n7, assign6010_e6176_d_n8, assign6010_e6176_d_n9, assign6010_e6176_d_n10, assign6010_e6176_d_n11,) = {
    if (((((locals.var_guard92 != 0.0) && (locals.var_guard107 != 0.0)) && (locals.var_guard108 != 0.0)) && (locals.var_guard109 != 0.0)) && (locals.var_guard110 == 0.0)) {
        let assign6010_e6169: f64 = (-locals.var_dxa);
        let assign6010_e6170: f64 = (assign6010_e6169).exp();
        let assign6010_e6171: f64 = (1.0 + assign6010_e6170);
        let assign6010_e6172: f64 = (assign6010_e6171).ln();
        let assign6010_e6173: f64 = (1e-6 * assign6010_e6172);
        let assign6010_e6174: f64 = (locals.var_gmax - assign6010_e6173);
        (assign6010_e6174, (locals.var_gmax_dn0 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn0)) / assign6010_e6171))), (locals.var_gmax_dn1 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn1)) / assign6010_e6171))), (locals.var_gmax_dn3 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn3)) / assign6010_e6171))), (locals.var_gmax_dn4 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn4)) / assign6010_e6171))), (locals.var_gmax_dn5 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn5)) / assign6010_e6171))), (locals.var_gmax_dn6 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn6)) / assign6010_e6171))), (locals.var_gmax_dn7 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn7)) / assign6010_e6171))), (locals.var_gmax_dn8 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn8)) / assign6010_e6171))), (locals.var_gmax_dn9 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn9)) / assign6010_e6171))), (locals.var_gmax_dn10 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn10)) / assign6010_e6171))), (locals.var_gmax_dn11 - (1e-6 * ((assign6010_e6170 * (-locals.var_dxa_dn11)) / assign6010_e6171))),)
    } else {
        (locals.var_gem, locals.var_gem_dn0, locals.var_gem_dn1, locals.var_gem_dn3, locals.var_gem_dn4, locals.var_gem_dn5, locals.var_gem_dn6, locals.var_gem_dn7, locals.var_gem_dn8, locals.var_gem_dn9, locals.var_gem_dn10, locals.var_gem_dn11,)
    }
};
        locals.var_gem = assign6010_e6176;
        locals.var_gem_dn0 = assign6010_e6176_d_n0;
        locals.var_gem_dn1 = assign6010_e6176_d_n1;
        locals.var_gem_dn3 = assign6010_e6176_d_n3;
        locals.var_gem_dn4 = assign6010_e6176_d_n4;
        locals.var_gem_dn5 = assign6010_e6176_d_n5;
        locals.var_gem_dn6 = assign6010_e6176_d_n6;
        locals.var_gem_dn7 = assign6010_e6176_d_n7;
        locals.var_gem_dn8 = assign6010_e6176_d_n8;
        locals.var_gem_dn9 = assign6010_e6176_d_n9;
        locals.var_gem_dn10 = assign6010_e6176_d_n10;
        locals.var_gem_dn11 = assign6010_e6176_d_n11;
        locals.var_gem_rv = 0.0;

        let assign6120_e6361: f64 = (1.0 - p.p68);
        let assign6120_e6363: f64 = (assign6120_e6361 * locals.var_cje_t);
        let assign6120_e6365: f64 = (assign6120_e6363 * locals.var_vte);
        locals.var_qte = assign6120_e6365;
        locals.var_qte_dn0 = (((assign6120_e6361 * locals.var_cje_t_dn0) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn0));
        locals.var_qte_dn1 = (((assign6120_e6361 * locals.var_cje_t_dn1) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn1));
        locals.var_qte_dn3 = (((assign6120_e6361 * locals.var_cje_t_dn3) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn3));
        locals.var_qte_dn4 = (((assign6120_e6361 * locals.var_cje_t_dn4) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn4));
        locals.var_qte_dn5 = (((assign6120_e6361 * locals.var_cje_t_dn5) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn5));
        locals.var_qte_dn6 = (((assign6120_e6361 * locals.var_cje_t_dn6) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn6));
        locals.var_qte_dn7 = (((assign6120_e6361 * locals.var_cje_t_dn7) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn7));
        locals.var_qte_dn8 = (((assign6120_e6361 * locals.var_cje_t_dn8) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn8));
        locals.var_qte_dn9 = (((assign6120_e6361 * locals.var_cje_t_dn9) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn9));
        locals.var_qte_dn10 = (((assign6120_e6361 * locals.var_cje_t_dn10) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn10));
        locals.var_qte_dn11 = (((assign6120_e6361 * locals.var_cje_t_dn11) * locals.var_vte) + (assign6120_e6363 * locals.var_vte_dn11));
        locals.var_qte_rv = 0.0;

    }

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
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let eq0_e167: f64 = (p.p3 * locals.var_ic1c2);
        let eq0_e167_d_n0: f64 = (p.p3 * locals.var_ic1c2_dn0);
        let eq0_e167_d_n1: f64 = (p.p3 * locals.var_ic1c2_dn1);
        let eq0_e167_d_n3: f64 = (p.p3 * locals.var_ic1c2_dn3);
        let eq0_e167_d_n4: f64 = (p.p3 * locals.var_ic1c2_dn4);
        let eq0_e167_d_n5: f64 = (p.p3 * locals.var_ic1c2_dn5);
        let eq0_e167_d_n6: f64 = (p.p3 * locals.var_ic1c2_dn6);
        let eq0_e167_d_n7: f64 = (p.p3 * locals.var_ic1c2_dn7);
        let eq0_e167_d_n8: f64 = (p.p3 * locals.var_ic1c2_dn8);
        let eq0_e167_d_n9: f64 = (p.p3 * locals.var_ic1c2_dn9);
        let eq0_e167_d_n10: f64 = (p.p3 * locals.var_ic1c2_dn10);
        let eq0_e167_d_n11: f64 = (p.p3 * locals.var_ic1c2_dn11);
        let eq0_e169: f64 = (eq0_e167 * p.p1);
        let eq0_e169_d_n0: f64 = (eq0_e167_d_n0 * p.p1);
        let eq0_e169_d_n1: f64 = (eq0_e167_d_n1 * p.p1);
        let eq0_e169_d_n3: f64 = (eq0_e167_d_n3 * p.p1);
        let eq0_e169_d_n4: f64 = (eq0_e167_d_n4 * p.p1);
        let eq0_e169_d_n5: f64 = (eq0_e167_d_n5 * p.p1);
        let eq0_e169_d_n6: f64 = (eq0_e167_d_n6 * p.p1);
        let eq0_e169_d_n7: f64 = (eq0_e167_d_n7 * p.p1);
        let eq0_e169_d_n8: f64 = (eq0_e167_d_n8 * p.p1);
        let eq0_e169_d_n9: f64 = (eq0_e167_d_n9 * p.p1);
        let eq0_e169_d_n10: f64 = (eq0_e167_d_n10 * p.p1);
        let eq0_e169_d_n11: f64 = (eq0_e167_d_n11 * p.p1);
        let eq0_value: f64 = eq0_e169;
        let eq0_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq0_node_derivatives: [f64; 11] = [eq0_e169_d_n0, eq0_e169_d_n1, eq0_e169_d_n3, eq0_e169_d_n4, eq0_e169_d_n5, eq0_e169_d_n6, eq0_e169_d_n7, eq0_e169_d_n8, eq0_e169_d_n9, eq0_e169_d_n10, eq0_e169_d_n11];
        let eq0_branch_derivative_indices: [usize; 0] = [];
        let eq0_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq0_value),
            &eq0_node_derivative_indices,
            &eq0_node_derivatives,
            &eq0_branch_derivative_indices,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let eq1_e172: f64 = (p.p3 * locals.var_in_);
        let eq1_e172_d_n0: f64 = (p.p3 * locals.var_in__dn0);
        let eq1_e172_d_n1: f64 = (p.p3 * locals.var_in__dn1);
        let eq1_e172_d_n3: f64 = (p.p3 * locals.var_in__dn3);
        let eq1_e172_d_n4: f64 = (p.p3 * locals.var_in__dn4);
        let eq1_e172_d_n5: f64 = (p.p3 * locals.var_in__dn5);
        let eq1_e172_d_n6: f64 = (p.p3 * locals.var_in__dn6);
        let eq1_e172_d_n7: f64 = (p.p3 * locals.var_in__dn7);
        let eq1_e172_d_n8: f64 = (p.p3 * locals.var_in__dn8);
        let eq1_e172_d_n9: f64 = (p.p3 * locals.var_in__dn9);
        let eq1_e172_d_n10: f64 = (p.p3 * locals.var_in__dn10);
        let eq1_e172_d_n11: f64 = (p.p3 * locals.var_in__dn11);
        let eq1_e174: f64 = (eq1_e172 * p.p1);
        let eq1_e174_d_n0: f64 = (eq1_e172_d_n0 * p.p1);
        let eq1_e174_d_n1: f64 = (eq1_e172_d_n1 * p.p1);
        let eq1_e174_d_n3: f64 = (eq1_e172_d_n3 * p.p1);
        let eq1_e174_d_n4: f64 = (eq1_e172_d_n4 * p.p1);
        let eq1_e174_d_n5: f64 = (eq1_e172_d_n5 * p.p1);
        let eq1_e174_d_n6: f64 = (eq1_e172_d_n6 * p.p1);
        let eq1_e174_d_n7: f64 = (eq1_e172_d_n7 * p.p1);
        let eq1_e174_d_n8: f64 = (eq1_e172_d_n8 * p.p1);
        let eq1_e174_d_n9: f64 = (eq1_e172_d_n9 * p.p1);
        let eq1_e174_d_n10: f64 = (eq1_e172_d_n10 * p.p1);
        let eq1_e174_d_n11: f64 = (eq1_e172_d_n11 * p.p1);
        let eq1_value: f64 = eq1_e174;
        let eq1_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq1_node_derivatives: [f64; 11] = [eq1_e174_d_n0, eq1_e174_d_n1, eq1_e174_d_n3, eq1_e174_d_n4, eq1_e174_d_n5, eq1_e174_d_n6, eq1_e174_d_n7, eq1_e174_d_n8, eq1_e174_d_n9, eq1_e174_d_n10, eq1_e174_d_n11];
        let eq1_branch_derivative_indices: [usize; 0] = [];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq1_value),
            &eq1_node_derivative_indices,
            &eq1_node_derivatives,
            &eq1_branch_derivative_indices,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let eq2_e178: f64 = (locals.var_ib1_s + locals.var_ib2_s);
        let eq2_e178_d_n0: f64 = (locals.var_ib1_s_dn0 + locals.var_ib2_s_dn0);
        let eq2_e178_d_n1: f64 = (locals.var_ib1_s_dn1 + locals.var_ib2_s_dn1);
        let eq2_e178_d_n3: f64 = (locals.var_ib1_s_dn3 + locals.var_ib2_s_dn3);
        let eq2_e178_d_n4: f64 = (locals.var_ib1_s_dn4 + locals.var_ib2_s_dn4);
        let eq2_e178_d_n5: f64 = (locals.var_ib1_s_dn5 + locals.var_ib2_s_dn5);
        let eq2_e178_d_n6: f64 = (locals.var_ib1_s_dn6 + locals.var_ib2_s_dn6);
        let eq2_e178_d_n7: f64 = (locals.var_ib1_s_dn7 + locals.var_ib2_s_dn7);
        let eq2_e178_d_n8: f64 = (locals.var_ib1_s_dn8 + locals.var_ib2_s_dn8);
        let eq2_e178_d_n9: f64 = (locals.var_ib1_s_dn9 + locals.var_ib2_s_dn9);
        let eq2_e178_d_n10: f64 = (locals.var_ib1_s_dn10 + locals.var_ib2_s_dn10);
        let eq2_e178_d_n11: f64 = (locals.var_ib1_s_dn11 + locals.var_ib2_s_dn11);
        let eq2_e180: f64 = (eq2_e178 + locals.var_ibrel);
        let eq2_e180_d_n0: f64 = (eq2_e178_d_n0 + locals.var_ibrel_dn0);
        let eq2_e180_d_n1: f64 = (eq2_e178_d_n1 + locals.var_ibrel_dn1);
        let eq2_e180_d_n3: f64 = (eq2_e178_d_n3 + locals.var_ibrel_dn3);
        let eq2_e180_d_n4: f64 = (eq2_e178_d_n4 + locals.var_ibrel_dn4);
        let eq2_e180_d_n5: f64 = (eq2_e178_d_n5 + locals.var_ibrel_dn5);
        let eq2_e180_d_n6: f64 = (eq2_e178_d_n6 + locals.var_ibrel_dn6);
        let eq2_e180_d_n7: f64 = (eq2_e178_d_n7 + locals.var_ibrel_dn7);
        let eq2_e180_d_n8: f64 = (eq2_e178_d_n8 + locals.var_ibrel_dn8);
        let eq2_e180_d_n9: f64 = (eq2_e178_d_n9 + locals.var_ibrel_dn9);
        let eq2_e180_d_n10: f64 = (eq2_e178_d_n10 + locals.var_ibrel_dn10);
        let eq2_e180_d_n11: f64 = (eq2_e178_d_n11 + locals.var_ibrel_dn11);
        let eq2_e181: f64 = (p.p3 * eq2_e180);
        let eq2_e181_d_n0: f64 = (p.p3 * eq2_e180_d_n0);
        let eq2_e181_d_n1: f64 = (p.p3 * eq2_e180_d_n1);
        let eq2_e181_d_n3: f64 = (p.p3 * eq2_e180_d_n3);
        let eq2_e181_d_n4: f64 = (p.p3 * eq2_e180_d_n4);
        let eq2_e181_d_n5: f64 = (p.p3 * eq2_e180_d_n5);
        let eq2_e181_d_n6: f64 = (p.p3 * eq2_e180_d_n6);
        let eq2_e181_d_n7: f64 = (p.p3 * eq2_e180_d_n7);
        let eq2_e181_d_n8: f64 = (p.p3 * eq2_e180_d_n8);
        let eq2_e181_d_n9: f64 = (p.p3 * eq2_e180_d_n9);
        let eq2_e181_d_n10: f64 = (p.p3 * eq2_e180_d_n10);
        let eq2_e181_d_n11: f64 = (p.p3 * eq2_e180_d_n11);
        let eq2_e183: f64 = (eq2_e181 * p.p1);
        let eq2_e183_d_n0: f64 = (eq2_e181_d_n0 * p.p1);
        let eq2_e183_d_n1: f64 = (eq2_e181_d_n1 * p.p1);
        let eq2_e183_d_n3: f64 = (eq2_e181_d_n3 * p.p1);
        let eq2_e183_d_n4: f64 = (eq2_e181_d_n4 * p.p1);
        let eq2_e183_d_n5: f64 = (eq2_e181_d_n5 * p.p1);
        let eq2_e183_d_n6: f64 = (eq2_e181_d_n6 * p.p1);
        let eq2_e183_d_n7: f64 = (eq2_e181_d_n7 * p.p1);
        let eq2_e183_d_n8: f64 = (eq2_e181_d_n8 * p.p1);
        let eq2_e183_d_n9: f64 = (eq2_e181_d_n9 * p.p1);
        let eq2_e183_d_n10: f64 = (eq2_e181_d_n10 * p.p1);
        let eq2_e183_d_n11: f64 = (eq2_e181_d_n11 * p.p1);
        let eq2_value: f64 = eq2_e183;
        let eq2_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq2_node_derivatives: [f64; 11] = [eq2_e183_d_n0, eq2_e183_d_n1, eq2_e183_d_n3, eq2_e183_d_n4, eq2_e183_d_n5, eq2_e183_d_n6, eq2_e183_d_n7, eq2_e183_d_n8, eq2_e183_d_n9, eq2_e183_d_n10, eq2_e183_d_n11];
        let eq2_branch_derivative_indices: [usize; 0] = [];
        let eq2_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq2_value),
            &eq2_node_derivative_indices,
            &eq2_node_derivatives,
            &eq2_branch_derivative_indices,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let eq3_e187: f64 = (locals.var_ib1 + locals.var_ib2);
        let eq3_e187_d_n0: f64 = (locals.var_ib1_dn0 + locals.var_ib2_dn0);
        let eq3_e187_d_n1: f64 = (locals.var_ib1_dn1 + locals.var_ib2_dn1);
        let eq3_e187_d_n3: f64 = (locals.var_ib1_dn3 + locals.var_ib2_dn3);
        let eq3_e187_d_n4: f64 = (locals.var_ib1_dn4 + locals.var_ib2_dn4);
        let eq3_e187_d_n5: f64 = (locals.var_ib1_dn5 + locals.var_ib2_dn5);
        let eq3_e187_d_n6: f64 = (locals.var_ib1_dn6 + locals.var_ib2_dn6);
        let eq3_e187_d_n7: f64 = (locals.var_ib1_dn7 + locals.var_ib2_dn7);
        let eq3_e187_d_n8: f64 = (locals.var_ib1_dn8 + locals.var_ib2_dn8);
        let eq3_e187_d_n9: f64 = (locals.var_ib1_dn9 + locals.var_ib2_dn9);
        let eq3_e187_d_n10: f64 = (locals.var_ib1_dn10 + locals.var_ib2_dn10);
        let eq3_e187_d_n11: f64 = (locals.var_ib1_dn11 + locals.var_ib2_dn11);
        let eq3_e190: f64 = (locals.var_gmin * locals.var_vb2e1);
        let eq3_e190_d_n5: f64 = (locals.var_gmin * locals.var_vb2e1_dn5);
        let eq3_e190_d_n7: f64 = (locals.var_gmin * locals.var_vb2e1_dn7);
        let eq3_e191: f64 = (eq3_e187 + eq3_e190);
        let eq3_e191_d_n5: f64 = (eq3_e187_d_n5 + eq3_e190_d_n5);
        let eq3_e191_d_n7: f64 = (eq3_e187_d_n7 + eq3_e190_d_n7);
        let eq3_e193: f64 = (eq3_e191 - locals.var_izteb);
        let eq3_e193_d_n0: f64 = (eq3_e187_d_n0 - locals.var_izteb_dn0);
        let eq3_e193_d_n1: f64 = (eq3_e187_d_n1 - locals.var_izteb_dn1);
        let eq3_e193_d_n3: f64 = (eq3_e187_d_n3 - locals.var_izteb_dn3);
        let eq3_e193_d_n4: f64 = (eq3_e187_d_n4 - locals.var_izteb_dn4);
        let eq3_e193_d_n5: f64 = (eq3_e191_d_n5 - locals.var_izteb_dn5);
        let eq3_e193_d_n6: f64 = (eq3_e187_d_n6 - locals.var_izteb_dn6);
        let eq3_e193_d_n7: f64 = (eq3_e191_d_n7 - locals.var_izteb_dn7);
        let eq3_e193_d_n8: f64 = (eq3_e187_d_n8 - locals.var_izteb_dn8);
        let eq3_e193_d_n9: f64 = (eq3_e187_d_n9 - locals.var_izteb_dn9);
        let eq3_e193_d_n10: f64 = (eq3_e187_d_n10 - locals.var_izteb_dn10);
        let eq3_e193_d_n11: f64 = (eq3_e187_d_n11 - locals.var_izteb_dn11);
        let eq3_e195: f64 = (eq3_e193 + locals.var_ibtbt);
        let eq3_e195_d_n0: f64 = (eq3_e193_d_n0 + locals.var_ibtbt_dn0);
        let eq3_e195_d_n1: f64 = (eq3_e193_d_n1 + locals.var_ibtbt_dn1);
        let eq3_e195_d_n3: f64 = (eq3_e193_d_n3 + locals.var_ibtbt_dn3);
        let eq3_e195_d_n4: f64 = (eq3_e193_d_n4 + locals.var_ibtbt_dn4);
        let eq3_e195_d_n5: f64 = (eq3_e193_d_n5 + locals.var_ibtbt_dn5);
        let eq3_e195_d_n6: f64 = (eq3_e193_d_n6 + locals.var_ibtbt_dn6);
        let eq3_e195_d_n7: f64 = (eq3_e193_d_n7 + locals.var_ibtbt_dn7);
        let eq3_e195_d_n8: f64 = (eq3_e193_d_n8 + locals.var_ibtbt_dn8);
        let eq3_e195_d_n9: f64 = (eq3_e193_d_n9 + locals.var_ibtbt_dn9);
        let eq3_e195_d_n10: f64 = (eq3_e193_d_n10 + locals.var_ibtbt_dn10);
        let eq3_e195_d_n11: f64 = (eq3_e193_d_n11 + locals.var_ibtbt_dn11);
        let eq3_e197: f64 = (eq3_e195 + locals.var_itat);
        let eq3_e197_d_n0: f64 = (eq3_e195_d_n0 + locals.var_itat_dn0);
        let eq3_e197_d_n1: f64 = (eq3_e195_d_n1 + locals.var_itat_dn1);
        let eq3_e197_d_n3: f64 = (eq3_e195_d_n3 + locals.var_itat_dn3);
        let eq3_e197_d_n4: f64 = (eq3_e195_d_n4 + locals.var_itat_dn4);
        let eq3_e197_d_n5: f64 = (eq3_e195_d_n5 + locals.var_itat_dn5);
        let eq3_e197_d_n6: f64 = (eq3_e195_d_n6 + locals.var_itat_dn6);
        let eq3_e197_d_n7: f64 = (eq3_e195_d_n7 + locals.var_itat_dn7);
        let eq3_e197_d_n8: f64 = (eq3_e195_d_n8 + locals.var_itat_dn8);
        let eq3_e197_d_n9: f64 = (eq3_e195_d_n9 + locals.var_itat_dn9);
        let eq3_e197_d_n10: f64 = (eq3_e195_d_n10 + locals.var_itat_dn10);
        let eq3_e197_d_n11: f64 = (eq3_e195_d_n11 + locals.var_itat_dn11);
        let eq3_e198: f64 = (p.p3 * eq3_e197);
        let eq3_e198_d_n0: f64 = (p.p3 * eq3_e197_d_n0);
        let eq3_e198_d_n1: f64 = (p.p3 * eq3_e197_d_n1);
        let eq3_e198_d_n3: f64 = (p.p3 * eq3_e197_d_n3);
        let eq3_e198_d_n4: f64 = (p.p3 * eq3_e197_d_n4);
        let eq3_e198_d_n5: f64 = (p.p3 * eq3_e197_d_n5);
        let eq3_e198_d_n6: f64 = (p.p3 * eq3_e197_d_n6);
        let eq3_e198_d_n7: f64 = (p.p3 * eq3_e197_d_n7);
        let eq3_e198_d_n8: f64 = (p.p3 * eq3_e197_d_n8);
        let eq3_e198_d_n9: f64 = (p.p3 * eq3_e197_d_n9);
        let eq3_e198_d_n10: f64 = (p.p3 * eq3_e197_d_n10);
        let eq3_e198_d_n11: f64 = (p.p3 * eq3_e197_d_n11);
        let eq3_e200: f64 = (eq3_e198 * p.p1);
        let eq3_e200_d_n0: f64 = (eq3_e198_d_n0 * p.p1);
        let eq3_e200_d_n1: f64 = (eq3_e198_d_n1 * p.p1);
        let eq3_e200_d_n3: f64 = (eq3_e198_d_n3 * p.p1);
        let eq3_e200_d_n4: f64 = (eq3_e198_d_n4 * p.p1);
        let eq3_e200_d_n5: f64 = (eq3_e198_d_n5 * p.p1);
        let eq3_e200_d_n6: f64 = (eq3_e198_d_n6 * p.p1);
        let eq3_e200_d_n7: f64 = (eq3_e198_d_n7 * p.p1);
        let eq3_e200_d_n8: f64 = (eq3_e198_d_n8 * p.p1);
        let eq3_e200_d_n9: f64 = (eq3_e198_d_n9 * p.p1);
        let eq3_e200_d_n10: f64 = (eq3_e198_d_n10 * p.p1);
        let eq3_e200_d_n11: f64 = (eq3_e198_d_n11 * p.p1);
        let eq3_value: f64 = eq3_e200;
        let eq3_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq3_node_derivatives: [f64; 11] = [eq3_e200_d_n0, eq3_e200_d_n1, eq3_e200_d_n3, eq3_e200_d_n4, eq3_e200_d_n5, eq3_e200_d_n6, eq3_e200_d_n7, eq3_e200_d_n8, eq3_e200_d_n9, eq3_e200_d_n10, eq3_e200_d_n11];
        let eq3_branch_derivative_indices: [usize; 0] = [];
        let eq3_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq3_value),
            &eq3_node_derivative_indices,
            &eq3_node_derivatives,
            &eq3_branch_derivative_indices,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e209, eq4_e209_d_n0, eq4_e209_d_n1, eq4_e209_d_n3, eq4_e209_d_n4, eq4_e209_d_n5, eq4_e209_d_n6, eq4_e209_d_n7, eq4_e209_d_n8, eq4_e209_d_n9, eq4_e209_d_n10, eq4_e209_d_n11,) = {
    if (locals.var_guard125 != 0.0) {
        let eq4_e204: f64 = (-locals.var_iztcb);
        let eq4_e205: f64 = (p.p3 * eq4_e204);
        let eq4_e205_d_n0: f64 = (p.p3 * (-locals.var_iztcb_dn0));
        let eq4_e205_d_n1: f64 = (p.p3 * (-locals.var_iztcb_dn1));
        let eq4_e205_d_n3: f64 = (p.p3 * (-locals.var_iztcb_dn3));
        let eq4_e205_d_n4: f64 = (p.p3 * (-locals.var_iztcb_dn4));
        let eq4_e205_d_n5: f64 = (p.p3 * (-locals.var_iztcb_dn5));
        let eq4_e205_d_n6: f64 = (p.p3 * (-locals.var_iztcb_dn6));
        let eq4_e205_d_n7: f64 = (p.p3 * (-locals.var_iztcb_dn7));
        let eq4_e205_d_n8: f64 = (p.p3 * (-locals.var_iztcb_dn8));
        let eq4_e205_d_n9: f64 = (p.p3 * (-locals.var_iztcb_dn9));
        let eq4_e205_d_n10: f64 = (p.p3 * (-locals.var_iztcb_dn10));
        let eq4_e205_d_n11: f64 = (p.p3 * (-locals.var_iztcb_dn11));
        let eq4_e207: f64 = (eq4_e205 * p.p1);
        let eq4_e207_d_n0: f64 = (eq4_e205_d_n0 * p.p1);
        let eq4_e207_d_n1: f64 = (eq4_e205_d_n1 * p.p1);
        let eq4_e207_d_n3: f64 = (eq4_e205_d_n3 * p.p1);
        let eq4_e207_d_n4: f64 = (eq4_e205_d_n4 * p.p1);
        let eq4_e207_d_n5: f64 = (eq4_e205_d_n5 * p.p1);
        let eq4_e207_d_n6: f64 = (eq4_e205_d_n6 * p.p1);
        let eq4_e207_d_n7: f64 = (eq4_e205_d_n7 * p.p1);
        let eq4_e207_d_n8: f64 = (eq4_e205_d_n8 * p.p1);
        let eq4_e207_d_n9: f64 = (eq4_e205_d_n9 * p.p1);
        let eq4_e207_d_n10: f64 = (eq4_e205_d_n10 * p.p1);
        let eq4_e207_d_n11: f64 = (eq4_e205_d_n11 * p.p1);
        (eq4_e207, eq4_e207_d_n0, eq4_e207_d_n1, eq4_e207_d_n3, eq4_e207_d_n4, eq4_e207_d_n5, eq4_e207_d_n6, eq4_e207_d_n7, eq4_e207_d_n8, eq4_e207_d_n9, eq4_e207_d_n10, eq4_e207_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e209;
        let eq4_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq4_node_derivatives: [f64; 11] = [eq4_e209_d_n0, eq4_e209_d_n1, eq4_e209_d_n3, eq4_e209_d_n4, eq4_e209_d_n5, eq4_e209_d_n6, eq4_e209_d_n7, eq4_e209_d_n8, eq4_e209_d_n9, eq4_e209_d_n10, eq4_e209_d_n11];
        let eq4_branch_derivative_indices: [usize; 0] = [];
        let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq4_value),
            &eq4_node_derivative_indices,
            &eq4_node_derivatives,
            &eq4_branch_derivative_indices,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e219, eq5_e219_d_n0, eq5_e219_d_n1, eq5_e219_d_n3, eq5_e219_d_n4, eq5_e219_d_n5, eq5_e219_d_n6, eq5_e219_d_n7, eq5_e219_d_n8, eq5_e219_d_n9, eq5_e219_d_n10, eq5_e219_d_n11,) = {
    if (locals.var_guard125 == 0.0) {
        let eq5_e214: f64 = (-locals.var_iztcb);
        let eq5_e215: f64 = (p.p3 * eq5_e214);
        let eq5_e215_d_n0: f64 = (p.p3 * (-locals.var_iztcb_dn0));
        let eq5_e215_d_n1: f64 = (p.p3 * (-locals.var_iztcb_dn1));
        let eq5_e215_d_n3: f64 = (p.p3 * (-locals.var_iztcb_dn3));
        let eq5_e215_d_n4: f64 = (p.p3 * (-locals.var_iztcb_dn4));
        let eq5_e215_d_n5: f64 = (p.p3 * (-locals.var_iztcb_dn5));
        let eq5_e215_d_n6: f64 = (p.p3 * (-locals.var_iztcb_dn6));
        let eq5_e215_d_n7: f64 = (p.p3 * (-locals.var_iztcb_dn7));
        let eq5_e215_d_n8: f64 = (p.p3 * (-locals.var_iztcb_dn8));
        let eq5_e215_d_n9: f64 = (p.p3 * (-locals.var_iztcb_dn9));
        let eq5_e215_d_n10: f64 = (p.p3 * (-locals.var_iztcb_dn10));
        let eq5_e215_d_n11: f64 = (p.p3 * (-locals.var_iztcb_dn11));
        let eq5_e217: f64 = (eq5_e215 * p.p1);
        let eq5_e217_d_n0: f64 = (eq5_e215_d_n0 * p.p1);
        let eq5_e217_d_n1: f64 = (eq5_e215_d_n1 * p.p1);
        let eq5_e217_d_n3: f64 = (eq5_e215_d_n3 * p.p1);
        let eq5_e217_d_n4: f64 = (eq5_e215_d_n4 * p.p1);
        let eq5_e217_d_n5: f64 = (eq5_e215_d_n5 * p.p1);
        let eq5_e217_d_n6: f64 = (eq5_e215_d_n6 * p.p1);
        let eq5_e217_d_n7: f64 = (eq5_e215_d_n7 * p.p1);
        let eq5_e217_d_n8: f64 = (eq5_e215_d_n8 * p.p1);
        let eq5_e217_d_n9: f64 = (eq5_e215_d_n9 * p.p1);
        let eq5_e217_d_n10: f64 = (eq5_e215_d_n10 * p.p1);
        let eq5_e217_d_n11: f64 = (eq5_e215_d_n11 * p.p1);
        (eq5_e217, eq5_e217_d_n0, eq5_e217_d_n1, eq5_e217_d_n3, eq5_e217_d_n4, eq5_e217_d_n5, eq5_e217_d_n6, eq5_e217_d_n7, eq5_e217_d_n8, eq5_e217_d_n9, eq5_e217_d_n10, eq5_e217_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e219;
        let eq5_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq5_node_derivatives: [f64; 11] = [eq5_e219_d_n0, eq5_e219_d_n1, eq5_e219_d_n3, eq5_e219_d_n4, eq5_e219_d_n5, eq5_e219_d_n6, eq5_e219_d_n7, eq5_e219_d_n8, eq5_e219_d_n9, eq5_e219_d_n10, eq5_e219_d_n11];
        let eq5_branch_derivative_indices: [usize; 0] = [];
        let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq5_value),
            &eq5_node_derivative_indices,
            &eq5_node_derivatives,
            &eq5_branch_derivative_indices,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let eq6_e222: f64 = (p.p3 * locals.var_isub);
        let eq6_e222_d_n3: f64 = (p.p3 * locals.var_isub_dn3);
        let eq6_e222_d_n4: f64 = (p.p3 * locals.var_isub_dn4);
        let eq6_e222_d_n6: f64 = (p.p3 * locals.var_isub_dn6);
        let eq6_e222_d_n7: f64 = (p.p3 * locals.var_isub_dn7);
        let eq6_e222_d_n8: f64 = (p.p3 * locals.var_isub_dn8);
        let eq6_e222_d_n9: f64 = (p.p3 * locals.var_isub_dn9);
        let eq6_e222_d_n11: f64 = (p.p3 * locals.var_isub_dn11);
        let eq6_e224: f64 = (eq6_e222 * p.p1);
        let eq6_e224_d_n3: f64 = (eq6_e222_d_n3 * p.p1);
        let eq6_e224_d_n4: f64 = (eq6_e222_d_n4 * p.p1);
        let eq6_e224_d_n6: f64 = (eq6_e222_d_n6 * p.p1);
        let eq6_e224_d_n7: f64 = (eq6_e222_d_n7 * p.p1);
        let eq6_e224_d_n8: f64 = (eq6_e222_d_n8 * p.p1);
        let eq6_e224_d_n9: f64 = (eq6_e222_d_n9 * p.p1);
        let eq6_e224_d_n11: f64 = (eq6_e222_d_n11 * p.p1);
        let eq6_value: f64 = eq6_e224;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(3),
            multiplicity * (eq6_value),
            [3, 4, 6, 7, 8, 9, 11],
            [multiplicity * (eq6_e224_d_n3), multiplicity * (eq6_e224_d_n4), multiplicity * (eq6_e224_d_n6), multiplicity * (eq6_e224_d_n7), multiplicity * (eq6_e224_d_n8), multiplicity * (eq6_e224_d_n9), multiplicity * (eq6_e224_d_n11)],
            [],
            [],
            1.0,
        );
        let eq7_e227: f64 = (p.p3 * locals.var_isub_int);
        let eq7_e227_d_n3: f64 = (p.p3 * locals.var_isub_int_dn3);
        let eq7_e227_d_n4: f64 = (p.p3 * locals.var_isub_int_dn4);
        let eq7_e227_d_n7: f64 = (p.p3 * locals.var_isub_int_dn7);
        let eq7_e227_d_n8: f64 = (p.p3 * locals.var_isub_int_dn8);
        let eq7_e227_d_n9: f64 = (p.p3 * locals.var_isub_int_dn9);
        let eq7_e229: f64 = (eq7_e227 * p.p1);
        let eq7_e229_d_n3: f64 = (eq7_e227_d_n3 * p.p1);
        let eq7_e229_d_n4: f64 = (eq7_e227_d_n4 * p.p1);
        let eq7_e229_d_n7: f64 = (eq7_e227_d_n7 * p.p1);
        let eq7_e229_d_n8: f64 = (eq7_e227_d_n8 * p.p1);
        let eq7_e229_d_n9: f64 = (eq7_e227_d_n9 * p.p1);
        let eq7_value: f64 = eq7_e229;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(3),
            multiplicity * (eq7_value),
            [3, 4, 7, 8, 9],
            [multiplicity * (eq7_e229_d_n3), multiplicity * (eq7_e229_d_n4), multiplicity * (eq7_e229_d_n7), multiplicity * (eq7_e229_d_n8), multiplicity * (eq7_e229_d_n9)],
            [],
            [],
            1.0,
        );
        let eq8_e232: f64 = (p.p3 * locals.var_xisub);
        let eq8_e232_d_n0: f64 = (p.p3 * locals.var_xisub_dn0);
        let eq8_e232_d_n1: f64 = (p.p3 * locals.var_xisub_dn1);
        let eq8_e232_d_n3: f64 = (p.p3 * locals.var_xisub_dn3);
        let eq8_e232_d_n4: f64 = (p.p3 * locals.var_xisub_dn4);
        let eq8_e232_d_n5: f64 = (p.p3 * locals.var_xisub_dn5);
        let eq8_e232_d_n6: f64 = (p.p3 * locals.var_xisub_dn6);
        let eq8_e232_d_n7: f64 = (p.p3 * locals.var_xisub_dn7);
        let eq8_e232_d_n8: f64 = (p.p3 * locals.var_xisub_dn8);
        let eq8_e232_d_n9: f64 = (p.p3 * locals.var_xisub_dn9);
        let eq8_e232_d_n10: f64 = (p.p3 * locals.var_xisub_dn10);
        let eq8_e232_d_n11: f64 = (p.p3 * locals.var_xisub_dn11);
        let eq8_e234: f64 = (eq8_e232 * p.p1);
        let eq8_e234_d_n0: f64 = (eq8_e232_d_n0 * p.p1);
        let eq8_e234_d_n1: f64 = (eq8_e232_d_n1 * p.p1);
        let eq8_e234_d_n3: f64 = (eq8_e232_d_n3 * p.p1);
        let eq8_e234_d_n4: f64 = (eq8_e232_d_n4 * p.p1);
        let eq8_e234_d_n5: f64 = (eq8_e232_d_n5 * p.p1);
        let eq8_e234_d_n6: f64 = (eq8_e232_d_n6 * p.p1);
        let eq8_e234_d_n7: f64 = (eq8_e232_d_n7 * p.p1);
        let eq8_e234_d_n8: f64 = (eq8_e232_d_n8 * p.p1);
        let eq8_e234_d_n9: f64 = (eq8_e232_d_n9 * p.p1);
        let eq8_e234_d_n10: f64 = (eq8_e232_d_n10 * p.p1);
        let eq8_e234_d_n11: f64 = (eq8_e232_d_n11 * p.p1);
        let eq8_value: f64 = eq8_e234;
        let eq8_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq8_node_derivatives: [f64; 11] = [eq8_e234_d_n0, eq8_e234_d_n1, eq8_e234_d_n3, eq8_e234_d_n4, eq8_e234_d_n5, eq8_e234_d_n6, eq8_e234_d_n7, eq8_e234_d_n8, eq8_e234_d_n9, eq8_e234_d_n10, eq8_e234_d_n11];
        let eq8_branch_derivative_indices: [usize; 0] = [];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(1),
            Some(3),
            multiplicity * (eq8_value),
            &eq8_node_derivative_indices,
            &eq8_node_derivatives,
            &eq8_branch_derivative_indices,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e237: f64 = (p.p3 * locals.var_isf);
        let eq9_e237_d_n3: f64 = (p.p3 * locals.var_isf_dn3);
        let eq9_e237_d_n4: f64 = (p.p3 * locals.var_isf_dn4);
        let eq9_e237_d_n8: f64 = (p.p3 * locals.var_isf_dn8);
        let eq9_e239: f64 = (eq9_e237 * p.p1);
        let eq9_e239_d_n3: f64 = (eq9_e237_d_n3 * p.p1);
        let eq9_e239_d_n4: f64 = (eq9_e237_d_n4 * p.p1);
        let eq9_e239_d_n8: f64 = (eq9_e237_d_n8 * p.p1);
        let eq9_value: f64 = eq9_e239;
        stamper.stamp_current_node3_local(
            Some(3),
            Some(8),
            multiplicity * (eq9_value),
            3,
            multiplicity * (eq9_e239_d_n3),
            4,
            multiplicity * (eq9_e239_d_n4),
            8,
            multiplicity * (eq9_e239_d_n8),
        );
        let eq10_e242: f64 = (p.p3 * locals.var_ib1b2);
        let eq10_e242_d_n0: f64 = (p.p3 * locals.var_ib1b2_dn0);
        let eq10_e242_d_n1: f64 = (p.p3 * locals.var_ib1b2_dn1);
        let eq10_e242_d_n3: f64 = (p.p3 * locals.var_ib1b2_dn3);
        let eq10_e242_d_n4: f64 = (p.p3 * locals.var_ib1b2_dn4);
        let eq10_e242_d_n5: f64 = (p.p3 * locals.var_ib1b2_dn5);
        let eq10_e242_d_n6: f64 = (p.p3 * locals.var_ib1b2_dn6);
        let eq10_e242_d_n7: f64 = (p.p3 * locals.var_ib1b2_dn7);
        let eq10_e242_d_n8: f64 = (p.p3 * locals.var_ib1b2_dn8);
        let eq10_e242_d_n9: f64 = (p.p3 * locals.var_ib1b2_dn9);
        let eq10_e242_d_n10: f64 = (p.p3 * locals.var_ib1b2_dn10);
        let eq10_e242_d_n11: f64 = (p.p3 * locals.var_ib1b2_dn11);
        let eq10_e244: f64 = (eq10_e242 * p.p1);
        let eq10_e244_d_n0: f64 = (eq10_e242_d_n0 * p.p1);
        let eq10_e244_d_n1: f64 = (eq10_e242_d_n1 * p.p1);
        let eq10_e244_d_n3: f64 = (eq10_e242_d_n3 * p.p1);
        let eq10_e244_d_n4: f64 = (eq10_e242_d_n4 * p.p1);
        let eq10_e244_d_n5: f64 = (eq10_e242_d_n5 * p.p1);
        let eq10_e244_d_n6: f64 = (eq10_e242_d_n6 * p.p1);
        let eq10_e244_d_n7: f64 = (eq10_e242_d_n7 * p.p1);
        let eq10_e244_d_n8: f64 = (eq10_e242_d_n8 * p.p1);
        let eq10_e244_d_n9: f64 = (eq10_e242_d_n9 * p.p1);
        let eq10_e244_d_n10: f64 = (eq10_e242_d_n10 * p.p1);
        let eq10_e244_d_n11: f64 = (eq10_e242_d_n11 * p.p1);
        let eq10_value: f64 = eq10_e244;
        let eq10_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq10_node_derivatives: [f64; 11] = [eq10_e244_d_n0, eq10_e244_d_n1, eq10_e244_d_n3, eq10_e244_d_n4, eq10_e244_d_n5, eq10_e244_d_n6, eq10_e244_d_n7, eq10_e244_d_n8, eq10_e244_d_n9, eq10_e244_d_n10, eq10_e244_d_n11];
        let eq10_branch_derivative_indices: [usize; 0] = [];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivative_indices,
            &eq10_node_derivatives,
            &eq10_branch_derivative_indices,
            &eq10_branch_derivatives,
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
        let eq11_e247: f64 = (-1.0);
        let eq11_e249: f64 = (eq11_e247 * locals.var_iavl);
        let eq11_e249_d_n0: f64 = (eq11_e247 * locals.var_iavl_dn0);
        let eq11_e249_d_n1: f64 = (eq11_e247 * locals.var_iavl_dn1);
        let eq11_e249_d_n3: f64 = (eq11_e247 * locals.var_iavl_dn3);
        let eq11_e249_d_n4: f64 = (eq11_e247 * locals.var_iavl_dn4);
        let eq11_e249_d_n5: f64 = (eq11_e247 * locals.var_iavl_dn5);
        let eq11_e249_d_n6: f64 = (eq11_e247 * locals.var_iavl_dn6);
        let eq11_e249_d_n7: f64 = (eq11_e247 * locals.var_iavl_dn7);
        let eq11_e249_d_n8: f64 = (eq11_e247 * locals.var_iavl_dn8);
        let eq11_e249_d_n9: f64 = (eq11_e247 * locals.var_iavl_dn9);
        let eq11_e249_d_n10: f64 = (eq11_e247 * locals.var_iavl_dn10);
        let eq11_e249_d_n11: f64 = (eq11_e247 * locals.var_iavl_dn11);
        let eq11_e250: f64 = (p.p3 * eq11_e249);
        let eq11_e250_d_n0: f64 = (p.p3 * eq11_e249_d_n0);
        let eq11_e250_d_n1: f64 = (p.p3 * eq11_e249_d_n1);
        let eq11_e250_d_n3: f64 = (p.p3 * eq11_e249_d_n3);
        let eq11_e250_d_n4: f64 = (p.p3 * eq11_e249_d_n4);
        let eq11_e250_d_n5: f64 = (p.p3 * eq11_e249_d_n5);
        let eq11_e250_d_n6: f64 = (p.p3 * eq11_e249_d_n6);
        let eq11_e250_d_n7: f64 = (p.p3 * eq11_e249_d_n7);
        let eq11_e250_d_n8: f64 = (p.p3 * eq11_e249_d_n8);
        let eq11_e250_d_n9: f64 = (p.p3 * eq11_e249_d_n9);
        let eq11_e250_d_n10: f64 = (p.p3 * eq11_e249_d_n10);
        let eq11_e250_d_n11: f64 = (p.p3 * eq11_e249_d_n11);
        let eq11_e252: f64 = (eq11_e250 * p.p1);
        let eq11_e252_d_n0: f64 = (eq11_e250_d_n0 * p.p1);
        let eq11_e252_d_n1: f64 = (eq11_e250_d_n1 * p.p1);
        let eq11_e252_d_n3: f64 = (eq11_e250_d_n3 * p.p1);
        let eq11_e252_d_n4: f64 = (eq11_e250_d_n4 * p.p1);
        let eq11_e252_d_n5: f64 = (eq11_e250_d_n5 * p.p1);
        let eq11_e252_d_n6: f64 = (eq11_e250_d_n6 * p.p1);
        let eq11_e252_d_n7: f64 = (eq11_e250_d_n7 * p.p1);
        let eq11_e252_d_n8: f64 = (eq11_e250_d_n8 * p.p1);
        let eq11_e252_d_n9: f64 = (eq11_e250_d_n9 * p.p1);
        let eq11_e252_d_n10: f64 = (eq11_e250_d_n10 * p.p1);
        let eq11_e252_d_n11: f64 = (eq11_e250_d_n11 * p.p1);
        let eq11_value: f64 = eq11_e252;
        let eq11_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq11_node_derivatives: [f64; 11] = [eq11_e252_d_n0, eq11_e252_d_n1, eq11_e252_d_n3, eq11_e252_d_n4, eq11_e252_d_n5, eq11_e252_d_n6, eq11_e252_d_n7, eq11_e252_d_n8, eq11_e252_d_n9, eq11_e252_d_n10, eq11_e252_d_n11];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e255: f64 = (p.p3 * locals.var_vee1);
        let eq12_e255_d_n2: f64 = (p.p3 * locals.var_vee1_dn2);
        let eq12_e255_d_n5: f64 = (p.p3 * locals.var_vee1_dn5);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_re_t;
        let eq12_e257: f64 = (eq12_e255 * __rspice_inv_cse_0);
        let eq12_e257_d_n2: f64 = (eq12_e255_d_n2 * __rspice_inv_cse_0);
        let eq12_e257_d_n4: f64 = (-((eq12_e255 * locals.var_re_t_dn4) / (locals.var_re_t * locals.var_re_t)));
        let eq12_e257_d_n5: f64 = (eq12_e255_d_n5 / locals.var_re_t);
        let eq12_e259: f64 = (eq12_e257 * p.p1);
        let eq12_e259_d_n2: f64 = (eq12_e257_d_n2 * p.p1);
        let eq12_e259_d_n4: f64 = (eq12_e257_d_n4 * p.p1);
        let eq12_e259_d_n5: f64 = (eq12_e257_d_n5 * p.p1);
        let eq12_value: f64 = eq12_e259;
        stamper.stamp_current_node3_local(
            Some(2),
            Some(5),
            multiplicity * (eq12_value),
            2,
            multiplicity * (eq12_e259_d_n2),
            4,
            multiplicity * (eq12_e259_d_n4),
            5,
            multiplicity * (eq12_e259_d_n5),
        );
        let eq13_e262: f64 = (p.p3 * locals.var_vbb1);
        let eq13_e262_d_n1: f64 = (p.p3 * locals.var_vbb1_dn1);
        let eq13_e262_d_n6: f64 = (p.p3 * locals.var_vbb1_dn6);
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_rbc_t;
        let eq13_e264: f64 = (eq13_e262 * __rspice_inv_cse_1);
        let eq13_e264_d_n1: f64 = (eq13_e262_d_n1 * __rspice_inv_cse_1);
        let eq13_e264_d_n4: f64 = (-((eq13_e262 * locals.var_rbc_t_dn4) / (locals.var_rbc_t * locals.var_rbc_t)));
        let eq13_e264_d_n6: f64 = (eq13_e262_d_n6 / locals.var_rbc_t);
        let eq13_e266: f64 = (eq13_e264 * p.p1);
        let eq13_e266_d_n1: f64 = (eq13_e264_d_n1 * p.p1);
        let eq13_e266_d_n4: f64 = (eq13_e264_d_n4 * p.p1);
        let eq13_e266_d_n6: f64 = (eq13_e264_d_n6 * p.p1);
        let eq13_value: f64 = eq13_e266;
        stamper.stamp_current_node3_local(
            Some(1),
            Some(6),
            multiplicity * (eq13_value),
            1,
            multiplicity * (eq13_e266_d_n1),
            4,
            multiplicity * (eq13_e266_d_n4),
            6,
            multiplicity * (eq13_e266_d_n6),
        );
        let eq14_value: f64 = locals.var_p_rth;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq14_value),
            4,
            multiplicity * (locals.var_p_rth_dn4),
        );
        let eq15_value: f64 = locals.var_i_cth;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq15_value),
            4,
            multiplicity * (locals.var_i_cth_dn4),
        );
        let eq16_e270: f64 = (-1.0);
        let eq16_e272: f64 = (eq16_e270 * locals.var_power);
        let eq16_e272_d_n0: f64 = (eq16_e270 * locals.var_power_dn0);
        let eq16_e272_d_n1: f64 = (eq16_e270 * locals.var_power_dn1);
        let eq16_e272_d_n2: f64 = (eq16_e270 * locals.var_power_dn2);
        let eq16_e272_d_n3: f64 = (eq16_e270 * locals.var_power_dn3);
        let eq16_e272_d_n4: f64 = (eq16_e270 * locals.var_power_dn4);
        let eq16_e272_d_n5: f64 = (eq16_e270 * locals.var_power_dn5);
        let eq16_e272_d_n6: f64 = (eq16_e270 * locals.var_power_dn6);
        let eq16_e272_d_n7: f64 = (eq16_e270 * locals.var_power_dn7);
        let eq16_e272_d_n8: f64 = (eq16_e270 * locals.var_power_dn8);
        let eq16_e272_d_n9: f64 = (eq16_e270 * locals.var_power_dn9);
        let eq16_e272_d_n10: f64 = (eq16_e270 * locals.var_power_dn10);
        let eq16_e272_d_n11: f64 = (eq16_e270 * locals.var_power_dn11);
        let eq16_e274: f64 = (eq16_e272 * p.p1);
        let eq16_e274_d_n0: f64 = (eq16_e272_d_n0 * p.p1);
        let eq16_e274_d_n1: f64 = (eq16_e272_d_n1 * p.p1);
        let eq16_e274_d_n2: f64 = (eq16_e272_d_n2 * p.p1);
        let eq16_e274_d_n3: f64 = (eq16_e272_d_n3 * p.p1);
        let eq16_e274_d_n4: f64 = (eq16_e272_d_n4 * p.p1);
        let eq16_e274_d_n5: f64 = (eq16_e272_d_n5 * p.p1);
        let eq16_e274_d_n6: f64 = (eq16_e272_d_n6 * p.p1);
        let eq16_e274_d_n7: f64 = (eq16_e272_d_n7 * p.p1);
        let eq16_e274_d_n8: f64 = (eq16_e272_d_n8 * p.p1);
        let eq16_e274_d_n9: f64 = (eq16_e272_d_n9 * p.p1);
        let eq16_e274_d_n10: f64 = (eq16_e272_d_n10 * p.p1);
        let eq16_e274_d_n11: f64 = (eq16_e272_d_n11 * p.p1);
        let eq16_value: f64 = eq16_e274;
        let eq16_node_derivative_indices: [usize; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq16_node_derivatives: [f64; 12] = [eq16_e274_d_n0, eq16_e274_d_n1, eq16_e274_d_n2, eq16_e274_d_n3, eq16_e274_d_n4, eq16_e274_d_n5, eq16_e274_d_n6, eq16_e274_d_n7, eq16_e274_d_n8, eq16_e274_d_n9, eq16_e274_d_n10, eq16_e274_d_n11];
        let eq16_branch_derivative_indices: [usize; 0] = [];
        let eq16_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq16_value),
            &eq16_node_derivative_indices,
            &eq16_node_derivatives,
            &eq16_branch_derivative_indices,
            &eq16_branch_derivatives,
            multiplicity,
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
        let eq24_e331: f64 = (p.p3 * locals.var_xiex);
        let eq24_e331_d_n0: f64 = (p.p3 * locals.var_xiex_dn0);
        let eq24_e331_d_n1: f64 = (p.p3 * locals.var_xiex_dn1);
        let eq24_e331_d_n3: f64 = (p.p3 * locals.var_xiex_dn3);
        let eq24_e331_d_n4: f64 = (p.p3 * locals.var_xiex_dn4);
        let eq24_e331_d_n5: f64 = (p.p3 * locals.var_xiex_dn5);
        let eq24_e331_d_n6: f64 = (p.p3 * locals.var_xiex_dn6);
        let eq24_e331_d_n7: f64 = (p.p3 * locals.var_xiex_dn7);
        let eq24_e331_d_n8: f64 = (p.p3 * locals.var_xiex_dn8);
        let eq24_e331_d_n9: f64 = (p.p3 * locals.var_xiex_dn9);
        let eq24_e331_d_n10: f64 = (p.p3 * locals.var_xiex_dn10);
        let eq24_e331_d_n11: f64 = (p.p3 * locals.var_xiex_dn11);
        let eq24_e333: f64 = (eq24_e331 * p.p1);
        let eq24_e333_d_n0: f64 = (eq24_e331_d_n0 * p.p1);
        let eq24_e333_d_n1: f64 = (eq24_e331_d_n1 * p.p1);
        let eq24_e333_d_n3: f64 = (eq24_e331_d_n3 * p.p1);
        let eq24_e333_d_n4: f64 = (eq24_e331_d_n4 * p.p1);
        let eq24_e333_d_n5: f64 = (eq24_e331_d_n5 * p.p1);
        let eq24_e333_d_n6: f64 = (eq24_e331_d_n6 * p.p1);
        let eq24_e333_d_n7: f64 = (eq24_e331_d_n7 * p.p1);
        let eq24_e333_d_n8: f64 = (eq24_e331_d_n8 * p.p1);
        let eq24_e333_d_n9: f64 = (eq24_e331_d_n9 * p.p1);
        let eq24_e333_d_n10: f64 = (eq24_e331_d_n10 * p.p1);
        let eq24_e333_d_n11: f64 = (eq24_e331_d_n11 * p.p1);
        let eq24_value: f64 = eq24_e333;
        let eq24_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq24_node_derivatives: [f64; 11] = [eq24_e333_d_n0, eq24_e333_d_n1, eq24_e333_d_n3, eq24_e333_d_n4, eq24_e333_d_n5, eq24_e333_d_n6, eq24_e333_d_n7, eq24_e333_d_n8, eq24_e333_d_n9, eq24_e333_d_n10, eq24_e333_d_n11];
        let eq24_branch_derivative_indices: [usize; 0] = [];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(1),
            Some(10),
            multiplicity * (eq24_value),
            &eq24_node_derivative_indices,
            &eq24_node_derivatives,
            &eq24_branch_derivative_indices,
            &eq24_branch_derivatives,
            multiplicity,
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq25_e336: f64 = (p.p3 * locals.var_vcc3);
        let eq25_e336_d_n0: f64 = (p.p3 * locals.var_vcc3_dn0);
        let eq25_e336_d_n1: f64 = (p.p3 * locals.var_vcc3_dn1);
        let eq25_e336_d_n6: f64 = (p.p3 * locals.var_vcc3_dn6);
        let eq25_e336_d_n7: f64 = (p.p3 * locals.var_vcc3_dn7);
        let eq25_e336_d_n8: f64 = (p.p3 * locals.var_vcc3_dn8);
        let eq25_e336_d_n9: f64 = (p.p3 * locals.var_vcc3_dn9);
        let eq25_e336_d_n10: f64 = (p.p3 * locals.var_vcc3_dn10);
        let eq25_e336_d_n11: f64 = (p.p3 * locals.var_vcc3_dn11);
        let eq25_e338: f64 = (eq25_e336 * locals.var_gcc_xx_t);
        let eq25_e338_d_n0: f64 = (eq25_e336_d_n0 * locals.var_gcc_xx_t);
        let eq25_e338_d_n1: f64 = (eq25_e336_d_n1 * locals.var_gcc_xx_t);
        let eq25_e338_d_n4: f64 = (eq25_e336 * locals.var_gcc_xx_t_dn4);
        let eq25_e338_d_n6: f64 = (eq25_e336_d_n6 * locals.var_gcc_xx_t);
        let eq25_e338_d_n7: f64 = (eq25_e336_d_n7 * locals.var_gcc_xx_t);
        let eq25_e338_d_n8: f64 = (eq25_e336_d_n8 * locals.var_gcc_xx_t);
        let eq25_e338_d_n9: f64 = (eq25_e336_d_n9 * locals.var_gcc_xx_t);
        let eq25_e338_d_n10: f64 = (eq25_e336_d_n10 * locals.var_gcc_xx_t);
        let eq25_e338_d_n11: f64 = (eq25_e336_d_n11 * locals.var_gcc_xx_t);
        let eq25_e340: f64 = (eq25_e338 * p.p1);
        let eq25_e340_d_n0: f64 = (eq25_e338_d_n0 * p.p1);
        let eq25_e340_d_n1: f64 = (eq25_e338_d_n1 * p.p1);
        let eq25_e340_d_n4: f64 = (eq25_e338_d_n4 * p.p1);
        let eq25_e340_d_n6: f64 = (eq25_e338_d_n6 * p.p1);
        let eq25_e340_d_n7: f64 = (eq25_e338_d_n7 * p.p1);
        let eq25_e340_d_n8: f64 = (eq25_e338_d_n8 * p.p1);
        let eq25_e340_d_n9: f64 = (eq25_e338_d_n9 * p.p1);
        let eq25_e340_d_n10: f64 = (eq25_e338_d_n10 * p.p1);
        let eq25_e340_d_n11: f64 = (eq25_e338_d_n11 * p.p1);
        let eq25_value: f64 = eq25_e340;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(0),
            Some(10),
            multiplicity * (eq25_value),
            [0, 1, 4, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq25_e340_d_n0), multiplicity * (eq25_e340_d_n1), multiplicity * (eq25_e340_d_n4), multiplicity * (eq25_e340_d_n6), multiplicity * (eq25_e340_d_n7), multiplicity * (eq25_e340_d_n8), multiplicity * (eq25_e340_d_n9), multiplicity * (eq25_e340_d_n10), multiplicity * (eq25_e340_d_n11)],
            [],
            [],
            1.0,
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
        let eq27_e353: f64 = (locals.var_gmin * locals.var_vb1c4);
        let eq27_e353_d_n6: f64 = (locals.var_gmin * locals.var_vb1c4_dn6);
        let eq27_e353_d_n7: f64 = (locals.var_gmin * locals.var_vb1c4_dn7);
        let eq27_e353_d_n8: f64 = (locals.var_gmin * locals.var_vb1c4_dn8);
        let eq27_e353_d_n9: f64 = (locals.var_gmin * locals.var_vb1c4_dn9);
        let eq27_e353_d_n11: f64 = (locals.var_gmin * locals.var_vb1c4_dn11);
        let eq27_e354: f64 = (locals.var_ib3 + eq27_e353);
        let eq27_e354_d_n6: f64 = (locals.var_ib3_dn6 + eq27_e353_d_n6);
        let eq27_e354_d_n7: f64 = (locals.var_ib3_dn7 + eq27_e353_d_n7);
        let eq27_e354_d_n8: f64 = (locals.var_ib3_dn8 + eq27_e353_d_n8);
        let eq27_e354_d_n9: f64 = (locals.var_ib3_dn9 + eq27_e353_d_n9);
        let eq27_e354_d_n11: f64 = (locals.var_ib3_dn11 + eq27_e353_d_n11);
        let eq27_e356: f64 = (eq27_e354 + locals.var_iex);
        let eq27_e356_d_n0: f64 = (locals.var_ib3_dn0 + locals.var_iex_dn0);
        let eq27_e356_d_n1: f64 = (locals.var_ib3_dn1 + locals.var_iex_dn1);
        let eq27_e356_d_n3: f64 = (locals.var_ib3_dn3 + locals.var_iex_dn3);
        let eq27_e356_d_n4: f64 = (locals.var_ib3_dn4 + locals.var_iex_dn4);
        let eq27_e356_d_n5: f64 = (locals.var_ib3_dn5 + locals.var_iex_dn5);
        let eq27_e356_d_n6: f64 = (eq27_e354_d_n6 + locals.var_iex_dn6);
        let eq27_e356_d_n7: f64 = (eq27_e354_d_n7 + locals.var_iex_dn7);
        let eq27_e356_d_n8: f64 = (eq27_e354_d_n8 + locals.var_iex_dn8);
        let eq27_e356_d_n9: f64 = (eq27_e354_d_n9 + locals.var_iex_dn9);
        let eq27_e356_d_n10: f64 = (locals.var_ib3_dn10 + locals.var_iex_dn10);
        let eq27_e356_d_n11: f64 = (eq27_e354_d_n11 + locals.var_iex_dn11);
        let eq27_e357: f64 = (p.p3 * eq27_e356);
        let eq27_e357_d_n0: f64 = (p.p3 * eq27_e356_d_n0);
        let eq27_e357_d_n1: f64 = (p.p3 * eq27_e356_d_n1);
        let eq27_e357_d_n3: f64 = (p.p3 * eq27_e356_d_n3);
        let eq27_e357_d_n4: f64 = (p.p3 * eq27_e356_d_n4);
        let eq27_e357_d_n5: f64 = (p.p3 * eq27_e356_d_n5);
        let eq27_e357_d_n6: f64 = (p.p3 * eq27_e356_d_n6);
        let eq27_e357_d_n7: f64 = (p.p3 * eq27_e356_d_n7);
        let eq27_e357_d_n8: f64 = (p.p3 * eq27_e356_d_n8);
        let eq27_e357_d_n9: f64 = (p.p3 * eq27_e356_d_n9);
        let eq27_e357_d_n10: f64 = (p.p3 * eq27_e356_d_n10);
        let eq27_e357_d_n11: f64 = (p.p3 * eq27_e356_d_n11);
        let eq27_e359: f64 = (eq27_e357 * p.p1);
        let eq27_e359_d_n0: f64 = (eq27_e357_d_n0 * p.p1);
        let eq27_e359_d_n1: f64 = (eq27_e357_d_n1 * p.p1);
        let eq27_e359_d_n3: f64 = (eq27_e357_d_n3 * p.p1);
        let eq27_e359_d_n4: f64 = (eq27_e357_d_n4 * p.p1);
        let eq27_e359_d_n5: f64 = (eq27_e357_d_n5 * p.p1);
        let eq27_e359_d_n6: f64 = (eq27_e357_d_n6 * p.p1);
        let eq27_e359_d_n7: f64 = (eq27_e357_d_n7 * p.p1);
        let eq27_e359_d_n8: f64 = (eq27_e357_d_n8 * p.p1);
        let eq27_e359_d_n9: f64 = (eq27_e357_d_n9 * p.p1);
        let eq27_e359_d_n10: f64 = (eq27_e357_d_n10 * p.p1);
        let eq27_e359_d_n11: f64 = (eq27_e357_d_n11 * p.p1);
        let eq27_value: f64 = eq27_e359;
        let eq27_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq27_node_derivatives: [f64; 11] = [eq27_e359_d_n0, eq27_e359_d_n1, eq27_e359_d_n3, eq27_e359_d_n4, eq27_e359_d_n5, eq27_e359_d_n6, eq27_e359_d_n7, eq27_e359_d_n8, eq27_e359_d_n9, eq27_e359_d_n10, eq27_e359_d_n11];
        let eq27_branch_derivative_indices: [usize; 0] = [];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(11),
            multiplicity * (eq27_value),
            &eq27_node_derivative_indices,
            &eq27_node_derivatives,
            &eq27_branch_derivative_indices,
            &eq27_branch_derivatives,
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
        let (eq29_e377, eq29_e377_d_n4, eq29_e377_d_n10, eq29_e377_d_n11,) = {
    if (locals.var_guard129 != 0.0) {
        let eq29_e371: f64 = (p.p3 * locals.var_vc3c4);
        let eq29_e371_d_n10: f64 = (p.p3 * locals.var_vc3c4_dn10);
        let eq29_e371_d_n11: f64 = (p.p3 * locals.var_vc3c4_dn11);
        let eq29_e373: f64 = (eq29_e371 * locals.var_gcc_ex_t);
        let eq29_e373_d_n4: f64 = (eq29_e371 * locals.var_gcc_ex_t_dn4);
        let eq29_e373_d_n10: f64 = (eq29_e371_d_n10 * locals.var_gcc_ex_t);
        let eq29_e373_d_n11: f64 = (eq29_e371_d_n11 * locals.var_gcc_ex_t);
        let eq29_e375: f64 = (eq29_e373 * p.p1);
        let eq29_e375_d_n4: f64 = (eq29_e373_d_n4 * p.p1);
        let eq29_e375_d_n10: f64 = (eq29_e373_d_n10 * p.p1);
        let eq29_e375_d_n11: f64 = (eq29_e373_d_n11 * p.p1);
        (eq29_e375, eq29_e375_d_n4, eq29_e375_d_n10, eq29_e375_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e377;
        stamper.stamp_current_node3_local(
            Some(10),
            Some(11),
            multiplicity * (eq29_value),
            4,
            multiplicity * (eq29_e377_d_n4),
            10,
            multiplicity * (eq29_e377_d_n10),
            11,
            multiplicity * (eq29_e377_d_n11),
        );
        let (eq31_e392, eq31_e392_d_n4, eq31_e392_d_n8, eq31_e392_d_n11,) = {
    if (locals.var_guard130 != 0.0) {
        let eq31_e386: f64 = (p.p3 * locals.var_vc4c1);
        let eq31_e386_d_n8: f64 = (p.p3 * locals.var_vc4c1_dn8);
        let eq31_e386_d_n11: f64 = (p.p3 * locals.var_vc4c1_dn11);
        let eq31_e388: f64 = (eq31_e386 * locals.var_gcc_in_t);
        let eq31_e388_d_n4: f64 = (eq31_e386 * locals.var_gcc_in_t_dn4);
        let eq31_e388_d_n8: f64 = (eq31_e386_d_n8 * locals.var_gcc_in_t);
        let eq31_e388_d_n11: f64 = (eq31_e386_d_n11 * locals.var_gcc_in_t);
        let eq31_e390: f64 = (eq31_e388 * p.p1);
        let eq31_e390_d_n4: f64 = (eq31_e388_d_n4 * p.p1);
        let eq31_e390_d_n8: f64 = (eq31_e388_d_n8 * p.p1);
        let eq31_e390_d_n11: f64 = (eq31_e388_d_n11 * p.p1);
        (eq31_e390, eq31_e390_d_n4, eq31_e390_d_n8, eq31_e390_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e392;
        stamper.stamp_current_node3_local(
            Some(11),
            Some(8),
            multiplicity * (eq31_value),
            4,
            multiplicity * (eq31_e392_d_n4),
            8,
            multiplicity * (eq31_e392_d_n8),
            11,
            multiplicity * (eq31_e392_d_n11),
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
        let eq36_e410: f64 = (locals.var_gem_n * (nv12 - 0.0));
        let eq36_e410_d_n0: f64 = (locals.var_gem_n_dn0 * (nv12 - 0.0));
        let eq36_e410_d_n1: f64 = (locals.var_gem_n_dn1 * (nv12 - 0.0));
        let eq36_e410_d_n3: f64 = (locals.var_gem_n_dn3 * (nv12 - 0.0));
        let eq36_e410_d_n4: f64 = (locals.var_gem_n_dn4 * (nv12 - 0.0));
        let eq36_e410_d_n5: f64 = (locals.var_gem_n_dn5 * (nv12 - 0.0));
        let eq36_e410_d_n6: f64 = (locals.var_gem_n_dn6 * (nv12 - 0.0));
        let eq36_e410_d_n7: f64 = (locals.var_gem_n_dn7 * (nv12 - 0.0));
        let eq36_e410_d_n8: f64 = (locals.var_gem_n_dn8 * (nv12 - 0.0));
        let eq36_e410_d_n9: f64 = (locals.var_gem_n_dn9 * (nv12 - 0.0));
        let eq36_e410_d_n10: f64 = (locals.var_gem_n_dn10 * (nv12 - 0.0));
        let eq36_e410_d_n11: f64 = (locals.var_gem_n_dn11 * (nv12 - 0.0));
        let eq36_value: f64 = eq36_e410;
        let eq36_node_derivative_indices: [usize; 12] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let eq36_node_derivatives: [f64; 12] = [eq36_e410_d_n0, eq36_e410_d_n1, eq36_e410_d_n3, eq36_e410_d_n4, eq36_e410_d_n5, eq36_e410_d_n6, eq36_e410_d_n7, eq36_e410_d_n8, eq36_e410_d_n9, eq36_e410_d_n10, eq36_e410_d_n11, locals.var_gem_n];
        let eq36_branch_derivative_indices: [usize; 0] = [];
        let eq36_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq36_value),
            &eq36_node_derivative_indices,
            &eq36_node_derivatives,
            &eq36_branch_derivative_indices,
            &eq36_branch_derivatives,
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
