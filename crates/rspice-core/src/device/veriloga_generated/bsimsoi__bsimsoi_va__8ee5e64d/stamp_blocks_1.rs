#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign8970_e6513, assign8970_e6513_d_n3, assign8970_e6513_d_n4, assign8970_e6513_d_n5, assign8970_e6513_d_n6, assign8970_e6513_d_n7, assign8970_e6513_d_n8, assign8970_e6513_d_n9, assign8970_e6513_d_n10, assign8970_e6513_d_n11, assign8970_e6513_d_n12,) = {
    if ((locals.var_guard906 == 0.0) && (locals.var_guard914 != 0.0)) {
        let assign8970_e6506: f64 = (locals.var_t0 * locals.var_pparam_b4soinpeak);
        let assign8970_e6508: f64 = (assign8970_e6506 * locals.var_pparam_b4soixt);
        let assign8970_e6510: f64 = (assign8970_e6508 * locals.var_pparam_b4soixt);
        let assign8970_e6511: f64 = (locals.var_pparam_b4soiphi - assign8970_e6510);
        (assign8970_e6511, (locals.var_pparam_b4soiphi_dn3 - ((((locals.var_t0_dn3 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn3)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)), (locals.var_pparam_b4soiphi_dn4 - ((((locals.var_t0_dn4 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn4)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)), (locals.var_pparam_b4soiphi_dn5 - ((((locals.var_t0_dn5 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn5)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)), (locals.var_pparam_b4soiphi_dn6 - ((((locals.var_t0_dn6 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn6)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)), (locals.var_pparam_b4soiphi_dn7 - ((((locals.var_t0_dn7 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn7)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)), (locals.var_pparam_b4soiphi_dn8 - ((((locals.var_t0_dn8 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn8)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)), (locals.var_pparam_b4soiphi_dn9 - ((((locals.var_t0_dn9 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn9)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)), (locals.var_pparam_b4soiphi_dn10 - ((((locals.var_t0_dn10 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn10)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)), (locals.var_pparam_b4soiphi_dn11 - ((((locals.var_t0_dn11 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn11)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)), (locals.var_pparam_b4soiphi_dn12 - ((((locals.var_t0_dn12 * locals.var_pparam_b4soinpeak) + (locals.var_t0 * locals.var_pparam_b4soinpeak_dn12)) * locals.var_pparam_b4soixt) * locals.var_pparam_b4soixt)),)
    } else {
        (locals.var_pparam_b4soivbx, locals.var_pparam_b4soivbx_dn3, locals.var_pparam_b4soivbx_dn4, locals.var_pparam_b4soivbx_dn5, locals.var_pparam_b4soivbx_dn6, locals.var_pparam_b4soivbx_dn7, locals.var_pparam_b4soivbx_dn8, locals.var_pparam_b4soivbx_dn9, locals.var_pparam_b4soivbx_dn10, locals.var_pparam_b4soivbx_dn11, locals.var_pparam_b4soivbx_dn12,)
    }
};
        locals.var_pparam_b4soivbx = assign8970_e6513;
        locals.var_pparam_b4soivbx_dn3 = assign8970_e6513_d_n3;
        locals.var_pparam_b4soivbx_dn4 = assign8970_e6513_d_n4;
        locals.var_pparam_b4soivbx_dn5 = assign8970_e6513_d_n5;
        locals.var_pparam_b4soivbx_dn6 = assign8970_e6513_d_n6;
        locals.var_pparam_b4soivbx_dn7 = assign8970_e6513_d_n7;
        locals.var_pparam_b4soivbx_dn8 = assign8970_e6513_d_n8;
        locals.var_pparam_b4soivbx_dn9 = assign8970_e6513_d_n9;
        locals.var_pparam_b4soivbx_dn10 = assign8970_e6513_d_n10;
        locals.var_pparam_b4soivbx_dn11 = assign8970_e6513_d_n11;
        locals.var_pparam_b4soivbx_dn12 = assign8970_e6513_d_n12;

        let assign8980_e6516: f64 = if locals.var_pparam_b4soivbx > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard915 = assign8980_e6516;

        let (assign8990_e6524, assign8990_e6524_d_n3, assign8990_e6524_d_n4, assign8990_e6524_d_n5, assign8990_e6524_d_n6, assign8990_e6524_d_n7, assign8990_e6524_d_n8, assign8990_e6524_d_n9, assign8990_e6524_d_n10, assign8990_e6524_d_n11, assign8990_e6524_d_n12,) = {
    if ((locals.var_guard906 == 0.0) && (locals.var_guard915 != 0.0)) {
        let assign8990_e6522: f64 = (-locals.var_pparam_b4soivbx);
        (assign8990_e6522, (-locals.var_pparam_b4soivbx_dn3), (-locals.var_pparam_b4soivbx_dn4), (-locals.var_pparam_b4soivbx_dn5), (-locals.var_pparam_b4soivbx_dn6), (-locals.var_pparam_b4soivbx_dn7), (-locals.var_pparam_b4soivbx_dn8), (-locals.var_pparam_b4soivbx_dn9), (-locals.var_pparam_b4soivbx_dn10), (-locals.var_pparam_b4soivbx_dn11), (-locals.var_pparam_b4soivbx_dn12),)
    } else {
        (locals.var_pparam_b4soivbx, locals.var_pparam_b4soivbx_dn3, locals.var_pparam_b4soivbx_dn4, locals.var_pparam_b4soivbx_dn5, locals.var_pparam_b4soivbx_dn6, locals.var_pparam_b4soivbx_dn7, locals.var_pparam_b4soivbx_dn8, locals.var_pparam_b4soivbx_dn9, locals.var_pparam_b4soivbx_dn10, locals.var_pparam_b4soivbx_dn11, locals.var_pparam_b4soivbx_dn12,)
    }
};
        locals.var_pparam_b4soivbx = assign8990_e6524;
        locals.var_pparam_b4soivbx_dn3 = assign8990_e6524_d_n3;
        locals.var_pparam_b4soivbx_dn4 = assign8990_e6524_d_n4;
        locals.var_pparam_b4soivbx_dn5 = assign8990_e6524_d_n5;
        locals.var_pparam_b4soivbx_dn6 = assign8990_e6524_d_n6;
        locals.var_pparam_b4soivbx_dn7 = assign8990_e6524_d_n7;
        locals.var_pparam_b4soivbx_dn8 = assign8990_e6524_d_n8;
        locals.var_pparam_b4soivbx_dn9 = assign8990_e6524_d_n9;
        locals.var_pparam_b4soivbx_dn10 = assign8990_e6524_d_n10;
        locals.var_pparam_b4soivbx_dn11 = assign8990_e6524_d_n11;
        locals.var_pparam_b4soivbx_dn12 = assign8990_e6524_d_n12;

        let assign9000_e6527: f64 = if locals.var_pparam_b4soivbm > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard916 = assign9000_e6527;

        let (assign9010_e6535,) = {
    if ((locals.var_guard906 == 0.0) && (locals.var_guard916 != 0.0)) {
        let assign9010_e6533: f64 = (-locals.var_pparam_b4soivbm);
        (assign9010_e6533,)
    } else {
        (locals.var_pparam_b4soivbm,)
    }
};
        locals.var_pparam_b4soivbm = assign9010_e6535;

        let assign9020_e6538: f64 = if (!param_given[84]) { 1.0 } else { 0.0 };
        locals.var_guard917 = assign9020_e6538;

        let (assign9030_e6550, assign9030_e6550_d_n3, assign9030_e6550_d_n4, assign9030_e6550_d_n5, assign9030_e6550_d_n6, assign9030_e6550_d_n7, assign9030_e6550_d_n8, assign9030_e6550_d_n9, assign9030_e6550_d_n10, assign9030_e6550_d_n11, assign9030_e6550_d_n12,) = {
    if ((locals.var_guard906 == 0.0) && (locals.var_guard917 != 0.0)) {
        let assign9030_e6545: f64 = (locals.var_pparam_b4soinpeak).sqrt();
        let assign9030_e6546: f64 = (locals.var_sqrt2qeps * assign9030_e6545);
        let assign9030_e6548: f64 = (assign9030_e6546 / locals.var_b4soicox);
        (assign9030_e6548, ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn3 / (2.0 * assign9030_e6545))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn4 / (2.0 * assign9030_e6545))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn5 / (2.0 * assign9030_e6545))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn6 / (2.0 * assign9030_e6545))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn7 / (2.0 * assign9030_e6545))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn8 / (2.0 * assign9030_e6545))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn9 / (2.0 * assign9030_e6545))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn10 / (2.0 * assign9030_e6545))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn11 / (2.0 * assign9030_e6545))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinpeak_dn12 / (2.0 * assign9030_e6545))) / locals.var_b4soicox),)
    } else {
        (locals.var_pparam_b4soigamma1, locals.var_pparam_b4soigamma1_dn3, locals.var_pparam_b4soigamma1_dn4, locals.var_pparam_b4soigamma1_dn5, locals.var_pparam_b4soigamma1_dn6, locals.var_pparam_b4soigamma1_dn7, locals.var_pparam_b4soigamma1_dn8, locals.var_pparam_b4soigamma1_dn9, locals.var_pparam_b4soigamma1_dn10, locals.var_pparam_b4soigamma1_dn11, locals.var_pparam_b4soigamma1_dn12,)
    }
};
        locals.var_pparam_b4soigamma1 = assign9030_e6550;
        locals.var_pparam_b4soigamma1_dn3 = assign9030_e6550_d_n3;
        locals.var_pparam_b4soigamma1_dn4 = assign9030_e6550_d_n4;
        locals.var_pparam_b4soigamma1_dn5 = assign9030_e6550_d_n5;
        locals.var_pparam_b4soigamma1_dn6 = assign9030_e6550_d_n6;
        locals.var_pparam_b4soigamma1_dn7 = assign9030_e6550_d_n7;
        locals.var_pparam_b4soigamma1_dn8 = assign9030_e6550_d_n8;
        locals.var_pparam_b4soigamma1_dn9 = assign9030_e6550_d_n9;
        locals.var_pparam_b4soigamma1_dn10 = assign9030_e6550_d_n10;
        locals.var_pparam_b4soigamma1_dn11 = assign9030_e6550_d_n11;
        locals.var_pparam_b4soigamma1_dn12 = assign9030_e6550_d_n12;

        let assign9040_e6553: f64 = if (!param_given[85]) { 1.0 } else { 0.0 };
        locals.var_guard918 = assign9040_e6553;

        let (assign9050_e6565, assign9050_e6565_d_n3, assign9050_e6565_d_n4, assign9050_e6565_d_n5, assign9050_e6565_d_n6, assign9050_e6565_d_n7, assign9050_e6565_d_n8, assign9050_e6565_d_n9, assign9050_e6565_d_n10, assign9050_e6565_d_n11, assign9050_e6565_d_n12,) = {
    if ((locals.var_guard906 == 0.0) && (locals.var_guard918 != 0.0)) {
        let assign9050_e6560: f64 = (locals.var_pparam_b4soinsub).sqrt();
        let assign9050_e6561: f64 = (locals.var_sqrt2qeps * assign9050_e6560);
        let assign9050_e6563: f64 = (assign9050_e6561 / locals.var_b4soicox);
        (assign9050_e6563, ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn3 / (2.0 * assign9050_e6560))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn4 / (2.0 * assign9050_e6560))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn5 / (2.0 * assign9050_e6560))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn6 / (2.0 * assign9050_e6560))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn7 / (2.0 * assign9050_e6560))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn8 / (2.0 * assign9050_e6560))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn9 / (2.0 * assign9050_e6560))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn10 / (2.0 * assign9050_e6560))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn11 / (2.0 * assign9050_e6560))) / locals.var_b4soicox), ((locals.var_sqrt2qeps * (locals.var_pparam_b4soinsub_dn12 / (2.0 * assign9050_e6560))) / locals.var_b4soicox),)
    } else {
        (locals.var_pparam_b4soigamma2, locals.var_pparam_b4soigamma2_dn3, locals.var_pparam_b4soigamma2_dn4, locals.var_pparam_b4soigamma2_dn5, locals.var_pparam_b4soigamma2_dn6, locals.var_pparam_b4soigamma2_dn7, locals.var_pparam_b4soigamma2_dn8, locals.var_pparam_b4soigamma2_dn9, locals.var_pparam_b4soigamma2_dn10, locals.var_pparam_b4soigamma2_dn11, locals.var_pparam_b4soigamma2_dn12,)
    }
};
        locals.var_pparam_b4soigamma2 = assign9050_e6565;
        locals.var_pparam_b4soigamma2_dn3 = assign9050_e6565_d_n3;
        locals.var_pparam_b4soigamma2_dn4 = assign9050_e6565_d_n4;
        locals.var_pparam_b4soigamma2_dn5 = assign9050_e6565_d_n5;
        locals.var_pparam_b4soigamma2_dn6 = assign9050_e6565_d_n6;
        locals.var_pparam_b4soigamma2_dn7 = assign9050_e6565_d_n7;
        locals.var_pparam_b4soigamma2_dn8 = assign9050_e6565_d_n8;
        locals.var_pparam_b4soigamma2_dn9 = assign9050_e6565_d_n9;
        locals.var_pparam_b4soigamma2_dn10 = assign9050_e6565_d_n10;
        locals.var_pparam_b4soigamma2_dn11 = assign9050_e6565_d_n11;
        locals.var_pparam_b4soigamma2_dn12 = assign9050_e6565_d_n12;

        let (assign9060_e6572, assign9060_e6572_d_n3, assign9060_e6572_d_n4, assign9060_e6572_d_n5, assign9060_e6572_d_n6, assign9060_e6572_d_n7, assign9060_e6572_d_n8, assign9060_e6572_d_n9, assign9060_e6572_d_n10, assign9060_e6572_d_n11, assign9060_e6572_d_n12,) = {
    if (locals.var_guard906 == 0.0) {
        let assign9060_e6570: f64 = (locals.var_pparam_b4soigamma1 - locals.var_pparam_b4soigamma2);
        (assign9060_e6570, (locals.var_pparam_b4soigamma1_dn3 - locals.var_pparam_b4soigamma2_dn3), (locals.var_pparam_b4soigamma1_dn4 - locals.var_pparam_b4soigamma2_dn4), (locals.var_pparam_b4soigamma1_dn5 - locals.var_pparam_b4soigamma2_dn5), (locals.var_pparam_b4soigamma1_dn6 - locals.var_pparam_b4soigamma2_dn6), (locals.var_pparam_b4soigamma1_dn7 - locals.var_pparam_b4soigamma2_dn7), (locals.var_pparam_b4soigamma1_dn8 - locals.var_pparam_b4soigamma2_dn8), (locals.var_pparam_b4soigamma1_dn9 - locals.var_pparam_b4soigamma2_dn9), (locals.var_pparam_b4soigamma1_dn10 - locals.var_pparam_b4soigamma2_dn10), (locals.var_pparam_b4soigamma1_dn11 - locals.var_pparam_b4soigamma2_dn11), (locals.var_pparam_b4soigamma1_dn12 - locals.var_pparam_b4soigamma2_dn12),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign9060_e6572;
        locals.var_t0_dn3 = assign9060_e6572_d_n3;
        locals.var_t0_dn4 = assign9060_e6572_d_n4;
        locals.var_t0_dn5 = assign9060_e6572_d_n5;
        locals.var_t0_dn6 = assign9060_e6572_d_n6;
        locals.var_t0_dn7 = assign9060_e6572_d_n7;
        locals.var_t0_dn8 = assign9060_e6572_d_n8;
        locals.var_t0_dn9 = assign9060_e6572_d_n9;
        locals.var_t0_dn10 = assign9060_e6572_d_n10;
        locals.var_t0_dn11 = assign9060_e6572_d_n11;
        locals.var_t0_dn12 = assign9060_e6572_d_n12;

        let (assign9070_e6582, assign9070_e6582_d_n3, assign9070_e6582_d_n4, assign9070_e6582_d_n5, assign9070_e6582_d_n6, assign9070_e6582_d_n7, assign9070_e6582_d_n8, assign9070_e6582_d_n9, assign9070_e6582_d_n10, assign9070_e6582_d_n11, assign9070_e6582_d_n12,) = {
    if (locals.var_guard906 == 0.0) {
        let assign9070_e6577: f64 = (locals.var_pparam_b4soiphi - locals.var_pparam_b4soivbx);
        let assign9070_e6578: f64 = (assign9070_e6577).sqrt();
        let assign9070_e6580: f64 = (assign9070_e6578 - locals.var_pparam_b4soisqrtphi);
        (assign9070_e6580, (((locals.var_pparam_b4soiphi_dn3 - locals.var_pparam_b4soivbx_dn3) / (2.0 * assign9070_e6578)) - locals.var_pparam_b4soisqrtphi_dn3), (((locals.var_pparam_b4soiphi_dn4 - locals.var_pparam_b4soivbx_dn4) / (2.0 * assign9070_e6578)) - locals.var_pparam_b4soisqrtphi_dn4), (((locals.var_pparam_b4soiphi_dn5 - locals.var_pparam_b4soivbx_dn5) / (2.0 * assign9070_e6578)) - locals.var_pparam_b4soisqrtphi_dn5), (((locals.var_pparam_b4soiphi_dn6 - locals.var_pparam_b4soivbx_dn6) / (2.0 * assign9070_e6578)) - locals.var_pparam_b4soisqrtphi_dn6), (((locals.var_pparam_b4soiphi_dn7 - locals.var_pparam_b4soivbx_dn7) / (2.0 * assign9070_e6578)) - locals.var_pparam_b4soisqrtphi_dn7), (((locals.var_pparam_b4soiphi_dn8 - locals.var_pparam_b4soivbx_dn8) / (2.0 * assign9070_e6578)) - locals.var_pparam_b4soisqrtphi_dn8), (((locals.var_pparam_b4soiphi_dn9 - locals.var_pparam_b4soivbx_dn9) / (2.0 * assign9070_e6578)) - locals.var_pparam_b4soisqrtphi_dn9), (((locals.var_pparam_b4soiphi_dn10 - locals.var_pparam_b4soivbx_dn10) / (2.0 * assign9070_e6578)) - locals.var_pparam_b4soisqrtphi_dn10), (((locals.var_pparam_b4soiphi_dn11 - locals.var_pparam_b4soivbx_dn11) / (2.0 * assign9070_e6578)) - locals.var_pparam_b4soisqrtphi_dn11), (((locals.var_pparam_b4soiphi_dn12 - locals.var_pparam_b4soivbx_dn12) / (2.0 * assign9070_e6578)) - locals.var_pparam_b4soisqrtphi_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign9070_e6582;
        locals.var_t1_dn3 = assign9070_e6582_d_n3;
        locals.var_t1_dn4 = assign9070_e6582_d_n4;
        locals.var_t1_dn5 = assign9070_e6582_d_n5;
        locals.var_t1_dn6 = assign9070_e6582_d_n6;
        locals.var_t1_dn7 = assign9070_e6582_d_n7;
        locals.var_t1_dn8 = assign9070_e6582_d_n8;
        locals.var_t1_dn9 = assign9070_e6582_d_n9;
        locals.var_t1_dn10 = assign9070_e6582_d_n10;
        locals.var_t1_dn11 = assign9070_e6582_d_n11;
        locals.var_t1_dn12 = assign9070_e6582_d_n12;

        let (assign9080_e6594, assign9080_e6594_d_n3, assign9080_e6594_d_n4, assign9080_e6594_d_n5, assign9080_e6594_d_n6, assign9080_e6594_d_n7, assign9080_e6594_d_n8, assign9080_e6594_d_n9, assign9080_e6594_d_n10, assign9080_e6594_d_n11, assign9080_e6594_d_n12,) = {
    if (locals.var_guard906 == 0.0) {
        let assign9080_e6588: f64 = (locals.var_pparam_b4soiphi - locals.var_pparam_b4soivbm);
        let assign9080_e6589: f64 = (assign9080_e6588).sqrt();
        let assign9080_e6591: f64 = (assign9080_e6589 - locals.var_pparam_b4soisqrtphi);
        let assign9080_e6592: f64 = (locals.var_pparam_b4soisqrtphi * assign9080_e6591);
        (assign9080_e6592, ((locals.var_pparam_b4soisqrtphi_dn3 * assign9080_e6591) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn3 / (2.0 * assign9080_e6589)) - locals.var_pparam_b4soisqrtphi_dn3))), ((locals.var_pparam_b4soisqrtphi_dn4 * assign9080_e6591) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn4 / (2.0 * assign9080_e6589)) - locals.var_pparam_b4soisqrtphi_dn4))), ((locals.var_pparam_b4soisqrtphi_dn5 * assign9080_e6591) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn5 / (2.0 * assign9080_e6589)) - locals.var_pparam_b4soisqrtphi_dn5))), ((locals.var_pparam_b4soisqrtphi_dn6 * assign9080_e6591) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn6 / (2.0 * assign9080_e6589)) - locals.var_pparam_b4soisqrtphi_dn6))), ((locals.var_pparam_b4soisqrtphi_dn7 * assign9080_e6591) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn7 / (2.0 * assign9080_e6589)) - locals.var_pparam_b4soisqrtphi_dn7))), ((locals.var_pparam_b4soisqrtphi_dn8 * assign9080_e6591) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn8 / (2.0 * assign9080_e6589)) - locals.var_pparam_b4soisqrtphi_dn8))), ((locals.var_pparam_b4soisqrtphi_dn9 * assign9080_e6591) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn9 / (2.0 * assign9080_e6589)) - locals.var_pparam_b4soisqrtphi_dn9))), ((locals.var_pparam_b4soisqrtphi_dn10 * assign9080_e6591) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn10 / (2.0 * assign9080_e6589)) - locals.var_pparam_b4soisqrtphi_dn10))), ((locals.var_pparam_b4soisqrtphi_dn11 * assign9080_e6591) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn11 / (2.0 * assign9080_e6589)) - locals.var_pparam_b4soisqrtphi_dn11))), ((locals.var_pparam_b4soisqrtphi_dn12 * assign9080_e6591) + (locals.var_pparam_b4soisqrtphi * ((locals.var_pparam_b4soiphi_dn12 / (2.0 * assign9080_e6589)) - locals.var_pparam_b4soisqrtphi_dn12))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign9080_e6594;
        locals.var_t2_dn3 = assign9080_e6594_d_n3;
        locals.var_t2_dn4 = assign9080_e6594_d_n4;
        locals.var_t2_dn5 = assign9080_e6594_d_n5;
        locals.var_t2_dn6 = assign9080_e6594_d_n6;
        locals.var_t2_dn7 = assign9080_e6594_d_n7;
        locals.var_t2_dn8 = assign9080_e6594_d_n8;
        locals.var_t2_dn9 = assign9080_e6594_d_n9;
        locals.var_t2_dn10 = assign9080_e6594_d_n10;
        locals.var_t2_dn11 = assign9080_e6594_d_n11;
        locals.var_t2_dn12 = assign9080_e6594_d_n12;

        let (assign9090_e6607, assign9090_e6607_d_n3, assign9090_e6607_d_n4, assign9090_e6607_d_n5, assign9090_e6607_d_n6, assign9090_e6607_d_n7, assign9090_e6607_d_n8, assign9090_e6607_d_n9, assign9090_e6607_d_n10, assign9090_e6607_d_n11, assign9090_e6607_d_n12,) = {
    if (locals.var_guard906 == 0.0) {
        let assign9090_e6599: f64 = (locals.var_t0 * locals.var_t1);
        let assign9090_e6602: f64 = (2.0 * locals.var_t2);
        let assign9090_e6604: f64 = (assign9090_e6602 + locals.var_pparam_b4soivbm);
        let assign9090_e6605: f64 = (assign9090_e6599 / assign9090_e6604);
        (assign9090_e6605, (((((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)) * assign9090_e6604) - (assign9090_e6599 * (2.0 * locals.var_t2_dn3))) / (assign9090_e6604 * assign9090_e6604)), (((((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)) * assign9090_e6604) - (assign9090_e6599 * (2.0 * locals.var_t2_dn4))) / (assign9090_e6604 * assign9090_e6604)), (((((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)) * assign9090_e6604) - (assign9090_e6599 * (2.0 * locals.var_t2_dn5))) / (assign9090_e6604 * assign9090_e6604)), (((((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)) * assign9090_e6604) - (assign9090_e6599 * (2.0 * locals.var_t2_dn6))) / (assign9090_e6604 * assign9090_e6604)), (((((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)) * assign9090_e6604) - (assign9090_e6599 * (2.0 * locals.var_t2_dn7))) / (assign9090_e6604 * assign9090_e6604)), (((((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)) * assign9090_e6604) - (assign9090_e6599 * (2.0 * locals.var_t2_dn8))) / (assign9090_e6604 * assign9090_e6604)), (((((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)) * assign9090_e6604) - (assign9090_e6599 * (2.0 * locals.var_t2_dn9))) / (assign9090_e6604 * assign9090_e6604)), (((((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)) * assign9090_e6604) - (assign9090_e6599 * (2.0 * locals.var_t2_dn10))) / (assign9090_e6604 * assign9090_e6604)), (((((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)) * assign9090_e6604) - (assign9090_e6599 * (2.0 * locals.var_t2_dn11))) / (assign9090_e6604 * assign9090_e6604)), (((((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12)) * assign9090_e6604) - (assign9090_e6599 * (2.0 * locals.var_t2_dn12))) / (assign9090_e6604 * assign9090_e6604)),)
    } else {
        (locals.var_pparam_b4soik2, locals.var_pparam_b4soik2_dn3, locals.var_pparam_b4soik2_dn4, locals.var_pparam_b4soik2_dn5, locals.var_pparam_b4soik2_dn6, locals.var_pparam_b4soik2_dn7, locals.var_pparam_b4soik2_dn8, locals.var_pparam_b4soik2_dn9, locals.var_pparam_b4soik2_dn10, locals.var_pparam_b4soik2_dn11, locals.var_pparam_b4soik2_dn12,)
    }
};
        locals.var_pparam_b4soik2 = assign9090_e6607;
        locals.var_pparam_b4soik2_dn3 = assign9090_e6607_d_n3;
        locals.var_pparam_b4soik2_dn4 = assign9090_e6607_d_n4;
        locals.var_pparam_b4soik2_dn5 = assign9090_e6607_d_n5;
        locals.var_pparam_b4soik2_dn6 = assign9090_e6607_d_n6;
        locals.var_pparam_b4soik2_dn7 = assign9090_e6607_d_n7;
        locals.var_pparam_b4soik2_dn8 = assign9090_e6607_d_n8;
        locals.var_pparam_b4soik2_dn9 = assign9090_e6607_d_n9;
        locals.var_pparam_b4soik2_dn10 = assign9090_e6607_d_n10;
        locals.var_pparam_b4soik2_dn11 = assign9090_e6607_d_n11;
        locals.var_pparam_b4soik2_dn12 = assign9090_e6607_d_n12;

        let (assign9100_e6621, assign9100_e6621_d_n3, assign9100_e6621_d_n4, assign9100_e6621_d_n5, assign9100_e6621_d_n6, assign9100_e6621_d_n7, assign9100_e6621_d_n8, assign9100_e6621_d_n9, assign9100_e6621_d_n10, assign9100_e6621_d_n11, assign9100_e6621_d_n12,) = {
    if (locals.var_guard906 == 0.0) {
        let assign9100_e6613: f64 = (2.0 * locals.var_pparam_b4soik2);
        let assign9100_e6616: f64 = (locals.var_pparam_b4soiphi - locals.var_pparam_b4soivbm);
        let assign9100_e6617: f64 = (assign9100_e6616).sqrt();
        let assign9100_e6618: f64 = (assign9100_e6613 * assign9100_e6617);
        let assign9100_e6619: f64 = (locals.var_pparam_b4soigamma2 - assign9100_e6618);
        (assign9100_e6619, (locals.var_pparam_b4soigamma2_dn3 - (((2.0 * locals.var_pparam_b4soik2_dn3) * assign9100_e6617) + (assign9100_e6613 * (locals.var_pparam_b4soiphi_dn3 / (2.0 * assign9100_e6617))))), (locals.var_pparam_b4soigamma2_dn4 - (((2.0 * locals.var_pparam_b4soik2_dn4) * assign9100_e6617) + (assign9100_e6613 * (locals.var_pparam_b4soiphi_dn4 / (2.0 * assign9100_e6617))))), (locals.var_pparam_b4soigamma2_dn5 - (((2.0 * locals.var_pparam_b4soik2_dn5) * assign9100_e6617) + (assign9100_e6613 * (locals.var_pparam_b4soiphi_dn5 / (2.0 * assign9100_e6617))))), (locals.var_pparam_b4soigamma2_dn6 - (((2.0 * locals.var_pparam_b4soik2_dn6) * assign9100_e6617) + (assign9100_e6613 * (locals.var_pparam_b4soiphi_dn6 / (2.0 * assign9100_e6617))))), (locals.var_pparam_b4soigamma2_dn7 - (((2.0 * locals.var_pparam_b4soik2_dn7) * assign9100_e6617) + (assign9100_e6613 * (locals.var_pparam_b4soiphi_dn7 / (2.0 * assign9100_e6617))))), (locals.var_pparam_b4soigamma2_dn8 - (((2.0 * locals.var_pparam_b4soik2_dn8) * assign9100_e6617) + (assign9100_e6613 * (locals.var_pparam_b4soiphi_dn8 / (2.0 * assign9100_e6617))))), (locals.var_pparam_b4soigamma2_dn9 - (((2.0 * locals.var_pparam_b4soik2_dn9) * assign9100_e6617) + (assign9100_e6613 * (locals.var_pparam_b4soiphi_dn9 / (2.0 * assign9100_e6617))))), (locals.var_pparam_b4soigamma2_dn10 - (((2.0 * locals.var_pparam_b4soik2_dn10) * assign9100_e6617) + (assign9100_e6613 * (locals.var_pparam_b4soiphi_dn10 / (2.0 * assign9100_e6617))))), (locals.var_pparam_b4soigamma2_dn11 - (((2.0 * locals.var_pparam_b4soik2_dn11) * assign9100_e6617) + (assign9100_e6613 * (locals.var_pparam_b4soiphi_dn11 / (2.0 * assign9100_e6617))))), (locals.var_pparam_b4soigamma2_dn12 - (((2.0 * locals.var_pparam_b4soik2_dn12) * assign9100_e6617) + (assign9100_e6613 * (locals.var_pparam_b4soiphi_dn12 / (2.0 * assign9100_e6617))))),)
    } else {
        (locals.var_pparam_b4soik1, locals.var_pparam_b4soik1_dn3, locals.var_pparam_b4soik1_dn4, locals.var_pparam_b4soik1_dn5, locals.var_pparam_b4soik1_dn6, locals.var_pparam_b4soik1_dn7, locals.var_pparam_b4soik1_dn8, locals.var_pparam_b4soik1_dn9, locals.var_pparam_b4soik1_dn10, locals.var_pparam_b4soik1_dn11, locals.var_pparam_b4soik1_dn12,)
    }
};
        locals.var_pparam_b4soik1 = assign9100_e6621;
        locals.var_pparam_b4soik1_dn3 = assign9100_e6621_d_n3;
        locals.var_pparam_b4soik1_dn4 = assign9100_e6621_d_n4;
        locals.var_pparam_b4soik1_dn5 = assign9100_e6621_d_n5;
        locals.var_pparam_b4soik1_dn6 = assign9100_e6621_d_n6;
        locals.var_pparam_b4soik1_dn7 = assign9100_e6621_d_n7;
        locals.var_pparam_b4soik1_dn8 = assign9100_e6621_d_n8;
        locals.var_pparam_b4soik1_dn9 = assign9100_e6621_d_n9;
        locals.var_pparam_b4soik1_dn10 = assign9100_e6621_d_n10;
        locals.var_pparam_b4soik1_dn11 = assign9100_e6621_d_n11;
        locals.var_pparam_b4soik1_dn12 = assign9100_e6621_d_n12;

        let assign9110_e6624: f64 = (locals.var_pparam_b4soiweff + locals.var_pparam_b4soik1w2);
        locals.var_t0 = assign9110_e6624;
        locals.var_t0_dn3 = (locals.var_pparam_b4soiweff_dn3 + locals.var_pparam_b4soik1w2_dn3);
        locals.var_t0_dn4 = (locals.var_pparam_b4soiweff_dn4 + locals.var_pparam_b4soik1w2_dn4);
        locals.var_t0_dn5 = (locals.var_pparam_b4soiweff_dn5 + locals.var_pparam_b4soik1w2_dn5);
        locals.var_t0_dn6 = (locals.var_pparam_b4soiweff_dn6 + locals.var_pparam_b4soik1w2_dn6);
        locals.var_t0_dn7 = (locals.var_pparam_b4soiweff_dn7 + locals.var_pparam_b4soik1w2_dn7);
        locals.var_t0_dn8 = (locals.var_pparam_b4soiweff_dn8 + locals.var_pparam_b4soik1w2_dn8);
        locals.var_t0_dn9 = (locals.var_pparam_b4soiweff_dn9 + locals.var_pparam_b4soik1w2_dn9);
        locals.var_t0_dn10 = (locals.var_pparam_b4soiweff_dn10 + locals.var_pparam_b4soik1w2_dn10);
        locals.var_t0_dn11 = (locals.var_pparam_b4soiweff_dn11 + locals.var_pparam_b4soik1w2_dn11);
        locals.var_t0_dn12 = (locals.var_pparam_b4soiweff_dn12 + locals.var_pparam_b4soik1w2_dn12);

        let assign9120_e6627: f64 = if locals.var_t0 < 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard919 = assign9120_e6627;

        let (assign9130_e6631, assign9130_e6631_d_n3, assign9130_e6631_d_n4, assign9130_e6631_d_n5, assign9130_e6631_d_n6, assign9130_e6631_d_n7, assign9130_e6631_d_n8, assign9130_e6631_d_n9, assign9130_e6631_d_n10, assign9130_e6631_d_n11, assign9130_e6631_d_n12,) = {
    if (locals.var_guard919 != 0.0) {
        (1e-8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign9130_e6631;
        locals.var_t0_dn3 = assign9130_e6631_d_n3;
        locals.var_t0_dn4 = assign9130_e6631_d_n4;
        locals.var_t0_dn5 = assign9130_e6631_d_n5;
        locals.var_t0_dn6 = assign9130_e6631_d_n6;
        locals.var_t0_dn7 = assign9130_e6631_d_n7;
        locals.var_t0_dn8 = assign9130_e6631_d_n8;
        locals.var_t0_dn9 = assign9130_e6631_d_n9;
        locals.var_t0_dn10 = assign9130_e6631_d_n10;
        locals.var_t0_dn11 = assign9130_e6631_d_n11;
        locals.var_t0_dn12 = assign9130_e6631_d_n12;

        let assign9140_e6636: f64 = (locals.var_pparam_b4soik1w1 / locals.var_t0);
        let assign9140_e6637: f64 = (1.0 + assign9140_e6636);
        let assign9140_e6638: f64 = (locals.var_pparam_b4soik1 * assign9140_e6637);
        locals.var_pparam_b4soik1eff = assign9140_e6638;
        locals.var_pparam_b4soik1eff_dn3 = ((locals.var_pparam_b4soik1_dn3 * assign9140_e6637) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn3 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0))));
        locals.var_pparam_b4soik1eff_dn4 = ((locals.var_pparam_b4soik1_dn4 * assign9140_e6637) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn4 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0))));
        locals.var_pparam_b4soik1eff_dn5 = ((locals.var_pparam_b4soik1_dn5 * assign9140_e6637) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn5 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0))));
        locals.var_pparam_b4soik1eff_dn6 = ((locals.var_pparam_b4soik1_dn6 * assign9140_e6637) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn6 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0))));
        locals.var_pparam_b4soik1eff_dn7 = ((locals.var_pparam_b4soik1_dn7 * assign9140_e6637) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn7 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0))));
        locals.var_pparam_b4soik1eff_dn8 = ((locals.var_pparam_b4soik1_dn8 * assign9140_e6637) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn8 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0))));
        locals.var_pparam_b4soik1eff_dn9 = ((locals.var_pparam_b4soik1_dn9 * assign9140_e6637) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn9 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0))));
        locals.var_pparam_b4soik1eff_dn10 = ((locals.var_pparam_b4soik1_dn10 * assign9140_e6637) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn10 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0))));
        locals.var_pparam_b4soik1eff_dn11 = ((locals.var_pparam_b4soik1_dn11 * assign9140_e6637) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn11 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0))));
        locals.var_pparam_b4soik1eff_dn12 = ((locals.var_pparam_b4soik1_dn12 * assign9140_e6637) + (locals.var_pparam_b4soik1 * (((locals.var_pparam_b4soik1w1_dn12 * locals.var_t0) - (locals.var_pparam_b4soik1w1 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0))));

        let assign9150_e6641: f64 = if (!param_given[108]) { 1.0 } else { 0.0 };
        locals.var_guard920 = assign9150_e6641;

        let assign9160_e6646: f64 = if (param_given[107] || param_given[106]) { 1.0 } else { 0.0 };
        locals.var_guard921 = assign9160_e6646;

        let (assign9170_e6660, assign9170_e6660_d_n3, assign9170_e6660_d_n4, assign9170_e6660_d_n5, assign9170_e6660_d_n6, assign9170_e6660_d_n7, assign9170_e6660_d_n8, assign9170_e6660_d_n9, assign9170_e6660_d_n10, assign9170_e6660_d_n11, assign9170_e6660_d_n12,) = {
    if ((locals.var_guard920 != 0.0) && (locals.var_guard921 != 0.0)) {
        let assign9170_e6652: f64 = (locals.var_b4soitype * locals.var_pparam_b4soivth0);
        let assign9170_e6654: f64 = (assign9170_e6652 - locals.var_pparam_b4soiphi);
        let assign9170_e6657: f64 = (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi);
        let assign9170_e6658: f64 = (assign9170_e6654 - assign9170_e6657);
        (assign9170_e6658, (((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn3) - locals.var_pparam_b4soiphi_dn3) - ((locals.var_pparam_b4soik1eff_dn3 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn3))), (((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn4) - locals.var_pparam_b4soiphi_dn4) - ((locals.var_pparam_b4soik1eff_dn4 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn4))), (((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn5) - locals.var_pparam_b4soiphi_dn5) - ((locals.var_pparam_b4soik1eff_dn5 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn5))), (((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn6) - locals.var_pparam_b4soiphi_dn6) - ((locals.var_pparam_b4soik1eff_dn6 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn6))), (((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn7) - locals.var_pparam_b4soiphi_dn7) - ((locals.var_pparam_b4soik1eff_dn7 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn7))), (((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn8) - locals.var_pparam_b4soiphi_dn8) - ((locals.var_pparam_b4soik1eff_dn8 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn8))), (((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn9) - locals.var_pparam_b4soiphi_dn9) - ((locals.var_pparam_b4soik1eff_dn9 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn9))), (((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn10) - locals.var_pparam_b4soiphi_dn10) - ((locals.var_pparam_b4soik1eff_dn10 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn10))), (((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn11) - locals.var_pparam_b4soiphi_dn11) - ((locals.var_pparam_b4soik1eff_dn11 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn11))), (((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn12) - locals.var_pparam_b4soiphi_dn12) - ((locals.var_pparam_b4soik1eff_dn12 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn12))),)
    } else {
        (locals.var_pparam_b4soivfb, locals.var_pparam_b4soivfb_dn3, locals.var_pparam_b4soivfb_dn4, locals.var_pparam_b4soivfb_dn5, locals.var_pparam_b4soivfb_dn6, locals.var_pparam_b4soivfb_dn7, locals.var_pparam_b4soivfb_dn8, locals.var_pparam_b4soivfb_dn9, locals.var_pparam_b4soivfb_dn10, locals.var_pparam_b4soivfb_dn11, locals.var_pparam_b4soivfb_dn12,)
    }
};
        locals.var_pparam_b4soivfb = assign9170_e6660;
        locals.var_pparam_b4soivfb_dn3 = assign9170_e6660_d_n3;
        locals.var_pparam_b4soivfb_dn4 = assign9170_e6660_d_n4;
        locals.var_pparam_b4soivfb_dn5 = assign9170_e6660_d_n5;
        locals.var_pparam_b4soivfb_dn6 = assign9170_e6660_d_n6;
        locals.var_pparam_b4soivfb_dn7 = assign9170_e6660_d_n7;
        locals.var_pparam_b4soivfb_dn8 = assign9170_e6660_d_n8;
        locals.var_pparam_b4soivfb_dn9 = assign9170_e6660_d_n9;
        locals.var_pparam_b4soivfb_dn10 = assign9170_e6660_d_n10;
        locals.var_pparam_b4soivfb_dn11 = assign9170_e6660_d_n11;
        locals.var_pparam_b4soivfb_dn12 = assign9170_e6660_d_n12;

        let (assign9180_e6668, assign9180_e6668_d_n3, assign9180_e6668_d_n4, assign9180_e6668_d_n5, assign9180_e6668_d_n6, assign9180_e6668_d_n7, assign9180_e6668_d_n8, assign9180_e6668_d_n9, assign9180_e6668_d_n10, assign9180_e6668_d_n11, assign9180_e6668_d_n12,) = {
    if ((locals.var_guard920 != 0.0) && (locals.var_guard921 == 0.0)) {
        let assign9180_e6666: f64 = (-1.0);
        (assign9180_e6666, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soivfb, locals.var_pparam_b4soivfb_dn3, locals.var_pparam_b4soivfb_dn4, locals.var_pparam_b4soivfb_dn5, locals.var_pparam_b4soivfb_dn6, locals.var_pparam_b4soivfb_dn7, locals.var_pparam_b4soivfb_dn8, locals.var_pparam_b4soivfb_dn9, locals.var_pparam_b4soivfb_dn10, locals.var_pparam_b4soivfb_dn11, locals.var_pparam_b4soivfb_dn12,)
    }
};
        locals.var_pparam_b4soivfb = assign9180_e6668;
        locals.var_pparam_b4soivfb_dn3 = assign9180_e6668_d_n3;
        locals.var_pparam_b4soivfb_dn4 = assign9180_e6668_d_n4;
        locals.var_pparam_b4soivfb_dn5 = assign9180_e6668_d_n5;
        locals.var_pparam_b4soivfb_dn6 = assign9180_e6668_d_n6;
        locals.var_pparam_b4soivfb_dn7 = assign9180_e6668_d_n7;
        locals.var_pparam_b4soivfb_dn8 = assign9180_e6668_d_n8;
        locals.var_pparam_b4soivfb_dn9 = assign9180_e6668_d_n9;
        locals.var_pparam_b4soivfb_dn10 = assign9180_e6668_d_n10;
        locals.var_pparam_b4soivfb_dn11 = assign9180_e6668_d_n11;
        locals.var_pparam_b4soivfb_dn12 = assign9180_e6668_d_n12;

        let assign9190_e6671: f64 = if (!param_given[107]) { 1.0 } else { 0.0 };
        locals.var_guard922 = assign9190_e6671;

        let (assign9200_e6683, assign9200_e6683_d_n3, assign9200_e6683_d_n4, assign9200_e6683_d_n5, assign9200_e6683_d_n6, assign9200_e6683_d_n7, assign9200_e6683_d_n8, assign9200_e6683_d_n9, assign9200_e6683_d_n10, assign9200_e6683_d_n11, assign9200_e6683_d_n12,) = {
    if (locals.var_guard922 != 0.0) {
        let assign9200_e6676: f64 = (locals.var_pparam_b4soivfb + locals.var_pparam_b4soiphi);
        let assign9200_e6679: f64 = (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi);
        let assign9200_e6680: f64 = (assign9200_e6676 + assign9200_e6679);
        let assign9200_e6681: f64 = (locals.var_b4soitype * assign9200_e6680);
        (assign9200_e6681, (locals.var_b4soitype * ((locals.var_pparam_b4soivfb_dn3 + locals.var_pparam_b4soiphi_dn3) + ((locals.var_pparam_b4soik1eff_dn3 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn3)))), (locals.var_b4soitype * ((locals.var_pparam_b4soivfb_dn4 + locals.var_pparam_b4soiphi_dn4) + ((locals.var_pparam_b4soik1eff_dn4 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn4)))), (locals.var_b4soitype * ((locals.var_pparam_b4soivfb_dn5 + locals.var_pparam_b4soiphi_dn5) + ((locals.var_pparam_b4soik1eff_dn5 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn5)))), (locals.var_b4soitype * ((locals.var_pparam_b4soivfb_dn6 + locals.var_pparam_b4soiphi_dn6) + ((locals.var_pparam_b4soik1eff_dn6 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn6)))), (locals.var_b4soitype * ((locals.var_pparam_b4soivfb_dn7 + locals.var_pparam_b4soiphi_dn7) + ((locals.var_pparam_b4soik1eff_dn7 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn7)))), (locals.var_b4soitype * ((locals.var_pparam_b4soivfb_dn8 + locals.var_pparam_b4soiphi_dn8) + ((locals.var_pparam_b4soik1eff_dn8 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn8)))), (locals.var_b4soitype * ((locals.var_pparam_b4soivfb_dn9 + locals.var_pparam_b4soiphi_dn9) + ((locals.var_pparam_b4soik1eff_dn9 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn9)))), (locals.var_b4soitype * ((locals.var_pparam_b4soivfb_dn10 + locals.var_pparam_b4soiphi_dn10) + ((locals.var_pparam_b4soik1eff_dn10 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn10)))), (locals.var_b4soitype * ((locals.var_pparam_b4soivfb_dn11 + locals.var_pparam_b4soiphi_dn11) + ((locals.var_pparam_b4soik1eff_dn11 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn11)))), (locals.var_b4soitype * ((locals.var_pparam_b4soivfb_dn12 + locals.var_pparam_b4soiphi_dn12) + ((locals.var_pparam_b4soik1eff_dn12 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1eff * locals.var_pparam_b4soisqrtphi_dn12)))),)
    } else {
        (locals.var_pparam_b4soivth0, locals.var_pparam_b4soivth0_dn3, locals.var_pparam_b4soivth0_dn4, locals.var_pparam_b4soivth0_dn5, locals.var_pparam_b4soivth0_dn6, locals.var_pparam_b4soivth0_dn7, locals.var_pparam_b4soivth0_dn8, locals.var_pparam_b4soivth0_dn9, locals.var_pparam_b4soivth0_dn10, locals.var_pparam_b4soivth0_dn11, locals.var_pparam_b4soivth0_dn12,)
    }
};
        locals.var_pparam_b4soivth0 = assign9200_e6683;
        locals.var_pparam_b4soivth0_dn3 = assign9200_e6683_d_n3;
        locals.var_pparam_b4soivth0_dn4 = assign9200_e6683_d_n4;
        locals.var_pparam_b4soivth0_dn5 = assign9200_e6683_d_n5;
        locals.var_pparam_b4soivth0_dn6 = assign9200_e6683_d_n6;
        locals.var_pparam_b4soivth0_dn7 = assign9200_e6683_d_n7;
        locals.var_pparam_b4soivth0_dn8 = assign9200_e6683_d_n8;
        locals.var_pparam_b4soivth0_dn9 = assign9200_e6683_d_n9;
        locals.var_pparam_b4soivth0_dn10 = assign9200_e6683_d_n10;
        locals.var_pparam_b4soivth0_dn11 = assign9200_e6683_d_n11;
        locals.var_pparam_b4soivth0_dn12 = assign9200_e6683_d_n12;

        let assign9210_e6686: f64 = (locals.var_pparam_b4soik1eff * locals.var_b4soitox);
        let assign9210_e6688: f64 = (assign9210_e6686 / locals.var_b4soitoxm);
        locals.var_pparam_b4soik1ox = assign9210_e6688;
        locals.var_pparam_b4soik1ox_dn3 = ((locals.var_pparam_b4soik1eff_dn3 * locals.var_b4soitox) / locals.var_b4soitoxm);
        locals.var_pparam_b4soik1ox_dn4 = ((locals.var_pparam_b4soik1eff_dn4 * locals.var_b4soitox) / locals.var_b4soitoxm);
        locals.var_pparam_b4soik1ox_dn5 = ((locals.var_pparam_b4soik1eff_dn5 * locals.var_b4soitox) / locals.var_b4soitoxm);
        locals.var_pparam_b4soik1ox_dn6 = ((locals.var_pparam_b4soik1eff_dn6 * locals.var_b4soitox) / locals.var_b4soitoxm);
        locals.var_pparam_b4soik1ox_dn7 = ((locals.var_pparam_b4soik1eff_dn7 * locals.var_b4soitox) / locals.var_b4soitoxm);
        locals.var_pparam_b4soik1ox_dn8 = ((locals.var_pparam_b4soik1eff_dn8 * locals.var_b4soitox) / locals.var_b4soitoxm);
        locals.var_pparam_b4soik1ox_dn9 = ((locals.var_pparam_b4soik1eff_dn9 * locals.var_b4soitox) / locals.var_b4soitoxm);
        locals.var_pparam_b4soik1ox_dn10 = ((locals.var_pparam_b4soik1eff_dn10 * locals.var_b4soitox) / locals.var_b4soitoxm);
        locals.var_pparam_b4soik1ox_dn11 = ((locals.var_pparam_b4soik1eff_dn11 * locals.var_b4soitox) / locals.var_b4soitoxm);
        locals.var_pparam_b4soik1ox_dn12 = ((locals.var_pparam_b4soik1eff_dn12 * locals.var_b4soitox) / locals.var_b4soitoxm);

        let assign9220_e6691: f64 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0);
        locals.var_t1 = assign9220_e6691;
        locals.var_t1_dn3 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn3);
        locals.var_t1_dn4 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn4);
        locals.var_t1_dn5 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn5);
        locals.var_t1_dn6 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn6);
        locals.var_t1_dn7 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn7);
        locals.var_t1_dn8 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn8);
        locals.var_t1_dn9 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn9);
        locals.var_t1_dn10 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn10);
        locals.var_t1_dn11 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn11);
        locals.var_t1_dn12 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn12);

        let assign9230_e6693: f64 = (-0.5);
        let assign9230_e6695: f64 = (assign9230_e6693 * locals.var_pparam_b4soidsub);
        let assign9230_e6697: f64 = (assign9230_e6695 * locals.var_pparam_b4soileff);
        let assign9230_e6699: f64 = (assign9230_e6697 / locals.var_t1);
        let assign9230_e6700: f64 = (assign9230_e6699).exp();
        locals.var_t0 = assign9230_e6700;
        locals.var_t0_dn3 = (assign9230_e6700 * ((((((assign9230_e6693 * locals.var_pparam_b4soidsub_dn3) * locals.var_pparam_b4soileff) + (assign9230_e6695 * locals.var_pparam_b4soileff_dn3)) * locals.var_t1) - (assign9230_e6697 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn4 = (assign9230_e6700 * ((((((assign9230_e6693 * locals.var_pparam_b4soidsub_dn4) * locals.var_pparam_b4soileff) + (assign9230_e6695 * locals.var_pparam_b4soileff_dn4)) * locals.var_t1) - (assign9230_e6697 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn5 = (assign9230_e6700 * ((((((assign9230_e6693 * locals.var_pparam_b4soidsub_dn5) * locals.var_pparam_b4soileff) + (assign9230_e6695 * locals.var_pparam_b4soileff_dn5)) * locals.var_t1) - (assign9230_e6697 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn6 = (assign9230_e6700 * ((((((assign9230_e6693 * locals.var_pparam_b4soidsub_dn6) * locals.var_pparam_b4soileff) + (assign9230_e6695 * locals.var_pparam_b4soileff_dn6)) * locals.var_t1) - (assign9230_e6697 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn7 = (assign9230_e6700 * ((((((assign9230_e6693 * locals.var_pparam_b4soidsub_dn7) * locals.var_pparam_b4soileff) + (assign9230_e6695 * locals.var_pparam_b4soileff_dn7)) * locals.var_t1) - (assign9230_e6697 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn8 = (assign9230_e6700 * ((((((assign9230_e6693 * locals.var_pparam_b4soidsub_dn8) * locals.var_pparam_b4soileff) + (assign9230_e6695 * locals.var_pparam_b4soileff_dn8)) * locals.var_t1) - (assign9230_e6697 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn9 = (assign9230_e6700 * ((((((assign9230_e6693 * locals.var_pparam_b4soidsub_dn9) * locals.var_pparam_b4soileff) + (assign9230_e6695 * locals.var_pparam_b4soileff_dn9)) * locals.var_t1) - (assign9230_e6697 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn10 = (assign9230_e6700 * ((((((assign9230_e6693 * locals.var_pparam_b4soidsub_dn10) * locals.var_pparam_b4soileff) + (assign9230_e6695 * locals.var_pparam_b4soileff_dn10)) * locals.var_t1) - (assign9230_e6697 * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn11 = (assign9230_e6700 * ((((((assign9230_e6693 * locals.var_pparam_b4soidsub_dn11) * locals.var_pparam_b4soileff) + (assign9230_e6695 * locals.var_pparam_b4soileff_dn11)) * locals.var_t1) - (assign9230_e6697 * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn12 = (assign9230_e6700 * ((((((assign9230_e6693 * locals.var_pparam_b4soidsub_dn12) * locals.var_pparam_b4soileff) + (assign9230_e6695 * locals.var_pparam_b4soileff_dn12)) * locals.var_t1) - (assign9230_e6697 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1)));

        let assign9240_e6704: f64 = (2.0 * locals.var_t0);
        let assign9240_e6706: f64 = (assign9240_e6704 * locals.var_t0);
        let assign9240_e6707: f64 = (locals.var_t0 + assign9240_e6706);
        locals.var_pparam_b4soitheta0vb0 = assign9240_e6707;
        locals.var_pparam_b4soitheta0vb0_dn3 = (locals.var_t0_dn3 + (((2.0 * locals.var_t0_dn3) * locals.var_t0) + (assign9240_e6704 * locals.var_t0_dn3)));
        locals.var_pparam_b4soitheta0vb0_dn4 = (locals.var_t0_dn4 + (((2.0 * locals.var_t0_dn4) * locals.var_t0) + (assign9240_e6704 * locals.var_t0_dn4)));
        locals.var_pparam_b4soitheta0vb0_dn5 = (locals.var_t0_dn5 + (((2.0 * locals.var_t0_dn5) * locals.var_t0) + (assign9240_e6704 * locals.var_t0_dn5)));
        locals.var_pparam_b4soitheta0vb0_dn6 = (locals.var_t0_dn6 + (((2.0 * locals.var_t0_dn6) * locals.var_t0) + (assign9240_e6704 * locals.var_t0_dn6)));
        locals.var_pparam_b4soitheta0vb0_dn7 = (locals.var_t0_dn7 + (((2.0 * locals.var_t0_dn7) * locals.var_t0) + (assign9240_e6704 * locals.var_t0_dn7)));
        locals.var_pparam_b4soitheta0vb0_dn8 = (locals.var_t0_dn8 + (((2.0 * locals.var_t0_dn8) * locals.var_t0) + (assign9240_e6704 * locals.var_t0_dn8)));
        locals.var_pparam_b4soitheta0vb0_dn9 = (locals.var_t0_dn9 + (((2.0 * locals.var_t0_dn9) * locals.var_t0) + (assign9240_e6704 * locals.var_t0_dn9)));
        locals.var_pparam_b4soitheta0vb0_dn10 = (locals.var_t0_dn10 + (((2.0 * locals.var_t0_dn10) * locals.var_t0) + (assign9240_e6704 * locals.var_t0_dn10)));
        locals.var_pparam_b4soitheta0vb0_dn11 = (locals.var_t0_dn11 + (((2.0 * locals.var_t0_dn11) * locals.var_t0) + (assign9240_e6704 * locals.var_t0_dn11)));
        locals.var_pparam_b4soitheta0vb0_dn12 = (locals.var_t0_dn12 + (((2.0 * locals.var_t0_dn12) * locals.var_t0) + (assign9240_e6704 * locals.var_t0_dn12)));

        let assign9250_e6709: f64 = (-0.5);
        let assign9250_e6711: f64 = (assign9250_e6709 * locals.var_pparam_b4soidrout);
        let assign9250_e6713: f64 = (assign9250_e6711 * locals.var_pparam_b4soileff);
        let assign9250_e6715: f64 = (assign9250_e6713 / locals.var_t1);
        let assign9250_e6716: f64 = (assign9250_e6715).exp();
        locals.var_t0 = assign9250_e6716;
        locals.var_t0_dn3 = (assign9250_e6716 * ((((((assign9250_e6709 * locals.var_pparam_b4soidrout_dn3) * locals.var_pparam_b4soileff) + (assign9250_e6711 * locals.var_pparam_b4soileff_dn3)) * locals.var_t1) - (assign9250_e6713 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn4 = (assign9250_e6716 * ((((((assign9250_e6709 * locals.var_pparam_b4soidrout_dn4) * locals.var_pparam_b4soileff) + (assign9250_e6711 * locals.var_pparam_b4soileff_dn4)) * locals.var_t1) - (assign9250_e6713 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn5 = (assign9250_e6716 * ((((((assign9250_e6709 * locals.var_pparam_b4soidrout_dn5) * locals.var_pparam_b4soileff) + (assign9250_e6711 * locals.var_pparam_b4soileff_dn5)) * locals.var_t1) - (assign9250_e6713 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn6 = (assign9250_e6716 * ((((((assign9250_e6709 * locals.var_pparam_b4soidrout_dn6) * locals.var_pparam_b4soileff) + (assign9250_e6711 * locals.var_pparam_b4soileff_dn6)) * locals.var_t1) - (assign9250_e6713 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn7 = (assign9250_e6716 * ((((((assign9250_e6709 * locals.var_pparam_b4soidrout_dn7) * locals.var_pparam_b4soileff) + (assign9250_e6711 * locals.var_pparam_b4soileff_dn7)) * locals.var_t1) - (assign9250_e6713 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn8 = (assign9250_e6716 * ((((((assign9250_e6709 * locals.var_pparam_b4soidrout_dn8) * locals.var_pparam_b4soileff) + (assign9250_e6711 * locals.var_pparam_b4soileff_dn8)) * locals.var_t1) - (assign9250_e6713 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn9 = (assign9250_e6716 * ((((((assign9250_e6709 * locals.var_pparam_b4soidrout_dn9) * locals.var_pparam_b4soileff) + (assign9250_e6711 * locals.var_pparam_b4soileff_dn9)) * locals.var_t1) - (assign9250_e6713 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn10 = (assign9250_e6716 * ((((((assign9250_e6709 * locals.var_pparam_b4soidrout_dn10) * locals.var_pparam_b4soileff) + (assign9250_e6711 * locals.var_pparam_b4soileff_dn10)) * locals.var_t1) - (assign9250_e6713 * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn11 = (assign9250_e6716 * ((((((assign9250_e6709 * locals.var_pparam_b4soidrout_dn11) * locals.var_pparam_b4soileff) + (assign9250_e6711 * locals.var_pparam_b4soileff_dn11)) * locals.var_t1) - (assign9250_e6713 * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)));
        locals.var_t0_dn12 = (assign9250_e6716 * ((((((assign9250_e6709 * locals.var_pparam_b4soidrout_dn12) * locals.var_pparam_b4soileff) + (assign9250_e6711 * locals.var_pparam_b4soileff_dn12)) * locals.var_t1) - (assign9250_e6713 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1)));

        let assign9260_e6720: f64 = (2.0 * locals.var_t0);
        let assign9260_e6722: f64 = (assign9260_e6720 * locals.var_t0);
        let assign9260_e6723: f64 = (locals.var_t0 + assign9260_e6722);
        locals.var_t2 = assign9260_e6723;
        locals.var_t2_dn3 = (locals.var_t0_dn3 + (((2.0 * locals.var_t0_dn3) * locals.var_t0) + (assign9260_e6720 * locals.var_t0_dn3)));
        locals.var_t2_dn4 = (locals.var_t0_dn4 + (((2.0 * locals.var_t0_dn4) * locals.var_t0) + (assign9260_e6720 * locals.var_t0_dn4)));
        locals.var_t2_dn5 = (locals.var_t0_dn5 + (((2.0 * locals.var_t0_dn5) * locals.var_t0) + (assign9260_e6720 * locals.var_t0_dn5)));
        locals.var_t2_dn6 = (locals.var_t0_dn6 + (((2.0 * locals.var_t0_dn6) * locals.var_t0) + (assign9260_e6720 * locals.var_t0_dn6)));
        locals.var_t2_dn7 = (locals.var_t0_dn7 + (((2.0 * locals.var_t0_dn7) * locals.var_t0) + (assign9260_e6720 * locals.var_t0_dn7)));
        locals.var_t2_dn8 = (locals.var_t0_dn8 + (((2.0 * locals.var_t0_dn8) * locals.var_t0) + (assign9260_e6720 * locals.var_t0_dn8)));
        locals.var_t2_dn9 = (locals.var_t0_dn9 + (((2.0 * locals.var_t0_dn9) * locals.var_t0) + (assign9260_e6720 * locals.var_t0_dn9)));
        locals.var_t2_dn10 = (locals.var_t0_dn10 + (((2.0 * locals.var_t0_dn10) * locals.var_t0) + (assign9260_e6720 * locals.var_t0_dn10)));
        locals.var_t2_dn11 = (locals.var_t0_dn11 + (((2.0 * locals.var_t0_dn11) * locals.var_t0) + (assign9260_e6720 * locals.var_t0_dn11)));
        locals.var_t2_dn12 = (locals.var_t0_dn12 + (((2.0 * locals.var_t0_dn12) * locals.var_t0) + (assign9260_e6720 * locals.var_t0_dn12)));

        let assign9270_e6726: f64 = (locals.var_pparam_b4soipdibl1 * locals.var_t2);
        let assign9270_e6728: f64 = (assign9270_e6726 + locals.var_pparam_b4soipdibl2);
        locals.var_pparam_b4soithetarout = assign9270_e6728;
        locals.var_pparam_b4soithetarout_dn3 = (((locals.var_pparam_b4soipdibl1_dn3 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn3)) + locals.var_pparam_b4soipdibl2_dn3);
        locals.var_pparam_b4soithetarout_dn4 = (((locals.var_pparam_b4soipdibl1_dn4 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn4)) + locals.var_pparam_b4soipdibl2_dn4);
        locals.var_pparam_b4soithetarout_dn5 = (((locals.var_pparam_b4soipdibl1_dn5 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn5)) + locals.var_pparam_b4soipdibl2_dn5);
        locals.var_pparam_b4soithetarout_dn6 = (((locals.var_pparam_b4soipdibl1_dn6 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn6)) + locals.var_pparam_b4soipdibl2_dn6);
        locals.var_pparam_b4soithetarout_dn7 = (((locals.var_pparam_b4soipdibl1_dn7 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn7)) + locals.var_pparam_b4soipdibl2_dn7);
        locals.var_pparam_b4soithetarout_dn8 = (((locals.var_pparam_b4soipdibl1_dn8 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn8)) + locals.var_pparam_b4soipdibl2_dn8);
        locals.var_pparam_b4soithetarout_dn9 = (((locals.var_pparam_b4soipdibl1_dn9 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn9)) + locals.var_pparam_b4soipdibl2_dn9);
        locals.var_pparam_b4soithetarout_dn10 = (((locals.var_pparam_b4soipdibl1_dn10 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn10)) + locals.var_pparam_b4soipdibl2_dn10);
        locals.var_pparam_b4soithetarout_dn11 = (((locals.var_pparam_b4soipdibl1_dn11 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn11)) + locals.var_pparam_b4soipdibl2_dn11);
        locals.var_pparam_b4soithetarout_dn12 = (((locals.var_pparam_b4soipdibl1_dn12 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn12)) + locals.var_pparam_b4soipdibl2_dn12);

        let (assign9280_e6738, assign9280_e6738_d_n3, assign9280_e6738_d_n4, assign9280_e6738_d_n5, assign9280_e6738_d_n6, assign9280_e6738_d_n7, assign9280_e6738_d_n8, assign9280_e6738_d_n9, assign9280_e6738_d_n10, assign9280_e6738_d_n11, assign9280_e6738_d_n12,) = {
    if (locals.var_pparam_b4soileff > 1e-38) {
        let assign9280_e6735: f64 = (locals.var_pparam_b4soileff).ln();
        (assign9280_e6735, (locals.var_pparam_b4soileff_dn3 / locals.var_pparam_b4soileff), (locals.var_pparam_b4soileff_dn4 / locals.var_pparam_b4soileff), (locals.var_pparam_b4soileff_dn5 / locals.var_pparam_b4soileff), (locals.var_pparam_b4soileff_dn6 / locals.var_pparam_b4soileff), (locals.var_pparam_b4soileff_dn7 / locals.var_pparam_b4soileff), (locals.var_pparam_b4soileff_dn8 / locals.var_pparam_b4soileff), (locals.var_pparam_b4soileff_dn9 / locals.var_pparam_b4soileff), (locals.var_pparam_b4soileff_dn10 / locals.var_pparam_b4soileff), (locals.var_pparam_b4soileff_dn11 / locals.var_pparam_b4soileff), (locals.var_pparam_b4soileff_dn12 / locals.var_pparam_b4soileff),)
    } else {
        let assign9280_e6737: f64 = (-87.49823353377374);
        (assign9280_e6737, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let assign9280_e6739: f64 = (locals.var_pparam_b4soidvtp3 * assign9280_e6738);
        let assign9280_e6740: f64 = (assign9280_e6739).exp();
        let assign9280_e6741: f64 = (locals.var_pparam_b4soidvtp2 / assign9280_e6740);
        locals.var_pparam_b4soidvtp2factor = assign9280_e6741;
        locals.var_pparam_b4soidvtp2factor_dn3 = (((locals.var_pparam_b4soidvtp2_dn3 * assign9280_e6740) - (locals.var_pparam_b4soidvtp2 * (assign9280_e6740 * ((locals.var_pparam_b4soidvtp3_dn3 * assign9280_e6738) + (locals.var_pparam_b4soidvtp3 * assign9280_e6738_d_n3))))) / (assign9280_e6740 * assign9280_e6740));
        locals.var_pparam_b4soidvtp2factor_dn4 = (((locals.var_pparam_b4soidvtp2_dn4 * assign9280_e6740) - (locals.var_pparam_b4soidvtp2 * (assign9280_e6740 * ((locals.var_pparam_b4soidvtp3_dn4 * assign9280_e6738) + (locals.var_pparam_b4soidvtp3 * assign9280_e6738_d_n4))))) / (assign9280_e6740 * assign9280_e6740));
        locals.var_pparam_b4soidvtp2factor_dn5 = (((locals.var_pparam_b4soidvtp2_dn5 * assign9280_e6740) - (locals.var_pparam_b4soidvtp2 * (assign9280_e6740 * ((locals.var_pparam_b4soidvtp3_dn5 * assign9280_e6738) + (locals.var_pparam_b4soidvtp3 * assign9280_e6738_d_n5))))) / (assign9280_e6740 * assign9280_e6740));
        locals.var_pparam_b4soidvtp2factor_dn6 = (((locals.var_pparam_b4soidvtp2_dn6 * assign9280_e6740) - (locals.var_pparam_b4soidvtp2 * (assign9280_e6740 * ((locals.var_pparam_b4soidvtp3_dn6 * assign9280_e6738) + (locals.var_pparam_b4soidvtp3 * assign9280_e6738_d_n6))))) / (assign9280_e6740 * assign9280_e6740));
        locals.var_pparam_b4soidvtp2factor_dn7 = (((locals.var_pparam_b4soidvtp2_dn7 * assign9280_e6740) - (locals.var_pparam_b4soidvtp2 * (assign9280_e6740 * ((locals.var_pparam_b4soidvtp3_dn7 * assign9280_e6738) + (locals.var_pparam_b4soidvtp3 * assign9280_e6738_d_n7))))) / (assign9280_e6740 * assign9280_e6740));
        locals.var_pparam_b4soidvtp2factor_dn8 = (((locals.var_pparam_b4soidvtp2_dn8 * assign9280_e6740) - (locals.var_pparam_b4soidvtp2 * (assign9280_e6740 * ((locals.var_pparam_b4soidvtp3_dn8 * assign9280_e6738) + (locals.var_pparam_b4soidvtp3 * assign9280_e6738_d_n8))))) / (assign9280_e6740 * assign9280_e6740));
        locals.var_pparam_b4soidvtp2factor_dn9 = (((locals.var_pparam_b4soidvtp2_dn9 * assign9280_e6740) - (locals.var_pparam_b4soidvtp2 * (assign9280_e6740 * ((locals.var_pparam_b4soidvtp3_dn9 * assign9280_e6738) + (locals.var_pparam_b4soidvtp3 * assign9280_e6738_d_n9))))) / (assign9280_e6740 * assign9280_e6740));
        locals.var_pparam_b4soidvtp2factor_dn10 = (((locals.var_pparam_b4soidvtp2_dn10 * assign9280_e6740) - (locals.var_pparam_b4soidvtp2 * (assign9280_e6740 * ((locals.var_pparam_b4soidvtp3_dn10 * assign9280_e6738) + (locals.var_pparam_b4soidvtp3 * assign9280_e6738_d_n10))))) / (assign9280_e6740 * assign9280_e6740));
        locals.var_pparam_b4soidvtp2factor_dn11 = (((locals.var_pparam_b4soidvtp2_dn11 * assign9280_e6740) - (locals.var_pparam_b4soidvtp2 * (assign9280_e6740 * ((locals.var_pparam_b4soidvtp3_dn11 * assign9280_e6738) + (locals.var_pparam_b4soidvtp3 * assign9280_e6738_d_n11))))) / (assign9280_e6740 * assign9280_e6740));
        locals.var_pparam_b4soidvtp2factor_dn12 = (((locals.var_pparam_b4soidvtp2_dn12 * assign9280_e6740) - (locals.var_pparam_b4soidvtp2 * (assign9280_e6740 * ((locals.var_pparam_b4soidvtp3_dn12 * assign9280_e6738) + (locals.var_pparam_b4soidvtp3 * assign9280_e6738_d_n12))))) / (assign9280_e6740 * assign9280_e6740));

        let assign9290_e6744: f64 = if locals.var_b4soiwlod < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard923 = assign9290_e6744;

        let (assign9300_e6748,) = {
    if (locals.var_guard923 != 0.0) {
        (0.0,)
    } else {
        (locals.var_b4soiwlod,)
    }
};
        locals.var_b4soiwlod = assign9300_e6748;

        let assign9310_e6751: f64 = (locals.var_ldrn).powf(locals.var_b4soillodku0);
        locals.var_t0 = assign9310_e6751;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;

        let assign9320_e6754: f64 = (locals.var_wdrn + locals.var_b4soiwlod);
        locals.var_w_tmp = assign9320_e6754;

    }

    pub(super) fn stamp_transient_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign9330_e6757: f64 = (locals.var_w_tmp).powf(locals.var_b4soiwlodku0);
        locals.var_t1 = assign9330_e6757;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;

        let assign9340_e6760: f64 = (p.p230 / locals.var_t0);
        let assign9340_e6763: f64 = (p.p231 / locals.var_t1);
        let assign9340_e6764: f64 = (assign9340_e6760 + assign9340_e6763);
        let assign9340_e6768: f64 = (locals.var_t0 * locals.var_t1);
        let assign9340_e6769: f64 = (p.p232 / assign9340_e6768);
        let assign9340_e6770: f64 = (assign9340_e6764 + assign9340_e6769);
        locals.var_tmp1 = assign9340_e6770;
        locals.var_tmp1_dn3 = (((-((p.p230 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))) + (-((p.p231 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1)))) + (-((p.p232 * ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3))) / (assign9340_e6768 * assign9340_e6768))));
        locals.var_tmp1_dn4 = (((-((p.p230 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) + (-((p.p231 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)))) + (-((p.p232 * ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4))) / (assign9340_e6768 * assign9340_e6768))));
        locals.var_tmp1_dn5 = (((-((p.p230 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) + (-((p.p231 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)))) + (-((p.p232 * ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5))) / (assign9340_e6768 * assign9340_e6768))));
        locals.var_tmp1_dn6 = (((-((p.p230 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) + (-((p.p231 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)))) + (-((p.p232 * ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6))) / (assign9340_e6768 * assign9340_e6768))));
        locals.var_tmp1_dn7 = (((-((p.p230 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) + (-((p.p231 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)))) + (-((p.p232 * ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7))) / (assign9340_e6768 * assign9340_e6768))));
        locals.var_tmp1_dn8 = (((-((p.p230 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) + (-((p.p231 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)))) + (-((p.p232 * ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8))) / (assign9340_e6768 * assign9340_e6768))));
        locals.var_tmp1_dn9 = (((-((p.p230 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) + (-((p.p231 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)))) + (-((p.p232 * ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9))) / (assign9340_e6768 * assign9340_e6768))));
        locals.var_tmp1_dn10 = (((-((p.p230 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) + (-((p.p231 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)))) + (-((p.p232 * ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10))) / (assign9340_e6768 * assign9340_e6768))));
        locals.var_tmp1_dn11 = (((-((p.p230 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) + (-((p.p231 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)))) + (-((p.p232 * ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11))) / (assign9340_e6768 * assign9340_e6768))));
        locals.var_tmp1_dn12 = (((-((p.p230 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))) + (-((p.p231 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1)))) + (-((p.p232 * ((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12))) / (assign9340_e6768 * assign9340_e6768))));

        let assign9350_e6773: f64 = (1.0 + locals.var_tmp1);
        locals.var_pparam_b4soiku0 = assign9350_e6773;
        locals.var_pparam_b4soiku0_dn3 = locals.var_tmp1_dn3;
        locals.var_pparam_b4soiku0_dn4 = locals.var_tmp1_dn4;
        locals.var_pparam_b4soiku0_dn5 = locals.var_tmp1_dn5;
        locals.var_pparam_b4soiku0_dn6 = locals.var_tmp1_dn6;
        locals.var_pparam_b4soiku0_dn7 = locals.var_tmp1_dn7;
        locals.var_pparam_b4soiku0_dn8 = locals.var_tmp1_dn8;
        locals.var_pparam_b4soiku0_dn9 = locals.var_tmp1_dn9;
        locals.var_pparam_b4soiku0_dn10 = locals.var_tmp1_dn10;
        locals.var_pparam_b4soiku0_dn11 = locals.var_tmp1_dn11;
        locals.var_pparam_b4soiku0_dn12 = locals.var_tmp1_dn12;

        let assign9360_e6776: f64 = (locals.var_ldrn).powf(locals.var_b4soillodvth);
        locals.var_t0 = assign9360_e6776;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;

        let assign9370_e6779: f64 = (locals.var_w_tmp).powf(locals.var_b4soiwlodvth);
        locals.var_t1 = assign9370_e6779;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;

        let assign9380_e6782: f64 = (p.p233 / locals.var_t0);
        let assign9380_e6785: f64 = (p.p234 / locals.var_t1);
        let assign9380_e6786: f64 = (assign9380_e6782 + assign9380_e6785);
        let assign9380_e6790: f64 = (locals.var_t0 * locals.var_t1);
        let assign9380_e6791: f64 = (p.p235 / assign9380_e6790);
        let assign9380_e6792: f64 = (assign9380_e6786 + assign9380_e6791);
        locals.var_tmp1 = assign9380_e6792;
        locals.var_tmp1_dn3 = (((-((p.p233 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))) + (-((p.p234 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1)))) + (-((p.p235 * ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3))) / (assign9380_e6790 * assign9380_e6790))));
        locals.var_tmp1_dn4 = (((-((p.p233 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) + (-((p.p234 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)))) + (-((p.p235 * ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4))) / (assign9380_e6790 * assign9380_e6790))));
        locals.var_tmp1_dn5 = (((-((p.p233 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) + (-((p.p234 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)))) + (-((p.p235 * ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5))) / (assign9380_e6790 * assign9380_e6790))));
        locals.var_tmp1_dn6 = (((-((p.p233 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) + (-((p.p234 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)))) + (-((p.p235 * ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6))) / (assign9380_e6790 * assign9380_e6790))));
        locals.var_tmp1_dn7 = (((-((p.p233 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) + (-((p.p234 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)))) + (-((p.p235 * ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7))) / (assign9380_e6790 * assign9380_e6790))));
        locals.var_tmp1_dn8 = (((-((p.p233 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) + (-((p.p234 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)))) + (-((p.p235 * ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8))) / (assign9380_e6790 * assign9380_e6790))));
        locals.var_tmp1_dn9 = (((-((p.p233 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) + (-((p.p234 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)))) + (-((p.p235 * ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9))) / (assign9380_e6790 * assign9380_e6790))));
        locals.var_tmp1_dn10 = (((-((p.p233 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) + (-((p.p234 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)))) + (-((p.p235 * ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10))) / (assign9380_e6790 * assign9380_e6790))));
        locals.var_tmp1_dn11 = (((-((p.p233 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) + (-((p.p234 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)))) + (-((p.p235 * ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11))) / (assign9380_e6790 * assign9380_e6790))));
        locals.var_tmp1_dn12 = (((-((p.p233 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))) + (-((p.p234 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1)))) + (-((p.p235 * ((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12))) / (assign9380_e6790 * assign9380_e6790))));

        let assign9390_e6795: f64 = (1.0 + locals.var_tmp1);
        locals.var_pparam_b4soikvth0 = assign9390_e6795;
        locals.var_pparam_b4soikvth0_dn3 = locals.var_tmp1_dn3;
        locals.var_pparam_b4soikvth0_dn4 = locals.var_tmp1_dn4;
        locals.var_pparam_b4soikvth0_dn5 = locals.var_tmp1_dn5;
        locals.var_pparam_b4soikvth0_dn6 = locals.var_tmp1_dn6;
        locals.var_pparam_b4soikvth0_dn7 = locals.var_tmp1_dn7;
        locals.var_pparam_b4soikvth0_dn8 = locals.var_tmp1_dn8;
        locals.var_pparam_b4soikvth0_dn9 = locals.var_tmp1_dn9;
        locals.var_pparam_b4soikvth0_dn10 = locals.var_tmp1_dn10;
        locals.var_pparam_b4soikvth0_dn11 = locals.var_tmp1_dn11;
        locals.var_pparam_b4soikvth0_dn12 = locals.var_tmp1_dn12;

        let assign9400_e6798: f64 = (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0);
        let assign9400_e6800: f64 = (assign9400_e6798 + 1e-9);
        let assign9400_e6801: f64 = (assign9400_e6800).sqrt();
        locals.var_pparam_b4soikvth0 = assign9400_e6801;
        locals.var_pparam_b4soikvth0_dn3 = (((locals.var_pparam_b4soikvth0_dn3 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn3)) / (2.0 * assign9400_e6801));
        locals.var_pparam_b4soikvth0_dn4 = (((locals.var_pparam_b4soikvth0_dn4 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn4)) / (2.0 * assign9400_e6801));
        locals.var_pparam_b4soikvth0_dn5 = (((locals.var_pparam_b4soikvth0_dn5 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn5)) / (2.0 * assign9400_e6801));
        locals.var_pparam_b4soikvth0_dn6 = (((locals.var_pparam_b4soikvth0_dn6 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn6)) / (2.0 * assign9400_e6801));
        locals.var_pparam_b4soikvth0_dn7 = (((locals.var_pparam_b4soikvth0_dn7 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn7)) / (2.0 * assign9400_e6801));
        locals.var_pparam_b4soikvth0_dn8 = (((locals.var_pparam_b4soikvth0_dn8 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn8)) / (2.0 * assign9400_e6801));
        locals.var_pparam_b4soikvth0_dn9 = (((locals.var_pparam_b4soikvth0_dn9 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn9)) / (2.0 * assign9400_e6801));
        locals.var_pparam_b4soikvth0_dn10 = (((locals.var_pparam_b4soikvth0_dn10 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn10)) / (2.0 * assign9400_e6801));
        locals.var_pparam_b4soikvth0_dn11 = (((locals.var_pparam_b4soikvth0_dn11 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn11)) / (2.0 * assign9400_e6801));
        locals.var_pparam_b4soikvth0_dn12 = (((locals.var_pparam_b4soikvth0_dn12 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn12)) / (2.0 * assign9400_e6801));

        let assign9410_e6804: f64 = (locals.var_tempratio__blk792 - 1.0);
        locals.var_t0 = assign9410_e6804;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = locals.var_tempratio__blk792_dn6;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;

        let assign9420_e6809: f64 = (locals.var_b4soitku0 * locals.var_t0);
        let assign9420_e6810: f64 = (1.0 + assign9420_e6809);
        let assign9420_e6811: f64 = (locals.var_pparam_b4soiku0 * assign9420_e6810);
        let assign9420_e6813: f64 = (assign9420_e6811 + 1e-9);
        locals.var_pparam_b4soiku0temp = assign9420_e6813;
        locals.var_pparam_b4soiku0temp_dn3 = ((locals.var_pparam_b4soiku0_dn3 * assign9420_e6810) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t0_dn3)));
        locals.var_pparam_b4soiku0temp_dn4 = ((locals.var_pparam_b4soiku0_dn4 * assign9420_e6810) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t0_dn4)));
        locals.var_pparam_b4soiku0temp_dn5 = ((locals.var_pparam_b4soiku0_dn5 * assign9420_e6810) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t0_dn5)));
        locals.var_pparam_b4soiku0temp_dn6 = ((locals.var_pparam_b4soiku0_dn6 * assign9420_e6810) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t0_dn6)));
        locals.var_pparam_b4soiku0temp_dn7 = ((locals.var_pparam_b4soiku0_dn7 * assign9420_e6810) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t0_dn7)));
        locals.var_pparam_b4soiku0temp_dn8 = ((locals.var_pparam_b4soiku0_dn8 * assign9420_e6810) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t0_dn8)));
        locals.var_pparam_b4soiku0temp_dn9 = ((locals.var_pparam_b4soiku0_dn9 * assign9420_e6810) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t0_dn9)));
        locals.var_pparam_b4soiku0temp_dn10 = ((locals.var_pparam_b4soiku0_dn10 * assign9420_e6810) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t0_dn10)));
        locals.var_pparam_b4soiku0temp_dn11 = ((locals.var_pparam_b4soiku0_dn11 * assign9420_e6810) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t0_dn11)));
        locals.var_pparam_b4soiku0temp_dn12 = ((locals.var_pparam_b4soiku0_dn12 * assign9420_e6810) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t0_dn12)));

        let assign9430_e6818: f64 = (0.5 * locals.var_ldrn);
        let assign9430_e6819: f64 = (locals.var_b4soisaref + assign9430_e6818);
        let assign9430_e6820: f64 = (1.0 / assign9430_e6819);
        locals.var_inv_saref = assign9430_e6820;

        let assign9440_e6825: f64 = (0.5 * locals.var_ldrn);
        let assign9440_e6826: f64 = (locals.var_b4soisbref + assign9440_e6825);
        let assign9440_e6827: f64 = (1.0 / assign9440_e6826);
        locals.var_inv_sbref = assign9440_e6827;

        let assign9450_e6830: f64 = (locals.var_inv_saref + locals.var_inv_sbref);
        locals.var_pparam_b4soiinv_od_ref = assign9450_e6830;

        let assign9460_e6833: f64 = (locals.var_b4soiku0 / locals.var_pparam_b4soiku0temp);
        let assign9460_e6835: f64 = (assign9460_e6833 * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref = assign9460_e6835;
        locals.var_pparam_b4soirho_ref_dn3 = ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn3) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref_dn4 = ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn4) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref_dn5 = ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn5) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref_dn6 = ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn6) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref_dn7 = ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn7) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref_dn8 = ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn8) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref_dn9 = ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn9) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref_dn10 = ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn10) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref_dn11 = ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn11) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref_dn12 = ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn12) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);

        let assign9470_e6854: f64 = if (((locals.var_b4soisa > 0.0) && (locals.var_b4soisb > 0.0)) && ((locals.var_b4soinf == 1.0) || ((locals.var_b4soinf > 1.0) && (locals.var_b4soisd > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard924 = assign9470_e6854;

        let (assign9480_e6858,) = {
    if (locals.var_guard924 != 0.0) {
        (0.0,)
    } else {
        (locals.var_inv_sa,)
    }
};
        locals.var_inv_sa = assign9480_e6858;

        let (assign9490_e6862,) = {
    if (locals.var_guard924 != 0.0) {
        (0.0,)
    } else {
        (locals.var_inv_sb,)
    }
};
        locals.var_inv_sb = assign9490_e6862;

        let assign9500_e6865: f64 = (-1.0);
        let assign9500_e6866: f64 = if locals.var_b4soikvsat < assign9500_e6865 { 1.0 } else { 0.0 };
        locals.var_guard925 = assign9500_e6866;

        let (assign9510_e6873,) = {
    if ((locals.var_guard924 != 0.0) && (locals.var_guard925 != 0.0)) {
        let assign9510_e6871: f64 = (-1.0);
        (assign9510_e6871,)
    } else {
        (locals.var_b4soikvsat,)
    }
};
        locals.var_b4soikvsat = assign9510_e6873;

        let assign9520_e6876: f64 = if locals.var_b4soikvsat > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard926 = assign9520_e6876;

        let (assign9530_e6885,) = {
    if (((locals.var_guard924 != 0.0) && (locals.var_guard925 == 0.0)) && (locals.var_guard926 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_b4soikvsat,)
    }
};
        locals.var_b4soikvsat = assign9530_e6885;

        let (assign9540_e6895,) = {
    if (((locals.var_guard924 != 0.0) && (locals.var_guard925 == 0.0)) && (locals.var_guard926 == 0.0)) {
        (locals.var_b4soikvsat,)
    } else {
        (locals.var_b4soikvsat,)
    }
};
        locals.var_b4soikvsat = assign9540_e6895;

        let (assign9550_e6899,) = {
    if (locals.var_guard924 != 0.0) {
        (0.0,)
    } else {
        (locals.var_i,)
    }
};
        locals.var_i = assign9550_e6899;

        let mut assign9560_loop_guard: usize = 0;
        while {
            let assign9560_cond_e6904: f64 = if ((locals.var_guard924 != 0.0) && (locals.var_i < locals.var_b4soinf)) { 1.0 } else { 0.0 };
            assign9560_cond_e6904 != 0.0
        } {
            assign9560_loop_guard += 1;
            assert!(assign9560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign9560_body0_e6922,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9560_body0_e6908: f64 = (1.0 / locals.var_b4soinf);
        let assign9560_body0_e6912: f64 = (0.5 * locals.var_ldrn);
        let assign9560_body0_e6913: f64 = (locals.var_b4soisa + assign9560_body0_e6912);
        let assign9560_body0_e6917: f64 = (locals.var_b4soisd + locals.var_ldrn);
        let assign9560_body0_e6918: f64 = (locals.var_i * assign9560_body0_e6917);
        let assign9560_body0_e6919: f64 = (assign9560_body0_e6913 + assign9560_body0_e6918);
        let assign9560_body0_e6920: f64 = (assign9560_body0_e6908 / assign9560_body0_e6919);
        (assign9560_body0_e6920,)
    } else {
        (locals.var_t0__blk927,)
    }
};
            locals.var_t0__blk927 = assign9560_body0_e6922;
            let (assign9560_body1_e6940,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9560_body1_e6926: f64 = (1.0 / locals.var_b4soinf);
        let assign9560_body1_e6930: f64 = (0.5 * locals.var_ldrn);
        let assign9560_body1_e6931: f64 = (locals.var_b4soisb + assign9560_body1_e6930);
        let assign9560_body1_e6935: f64 = (locals.var_b4soisd + locals.var_ldrn);
        let assign9560_body1_e6936: f64 = (locals.var_i * assign9560_body1_e6935);
        let assign9560_body1_e6937: f64 = (assign9560_body1_e6931 + assign9560_body1_e6936);
        let assign9560_body1_e6938: f64 = (assign9560_body1_e6926 / assign9560_body1_e6937);
        (assign9560_body1_e6938,)
    } else {
        (locals.var_t1__blk928,)
    }
};
            locals.var_t1__blk928 = assign9560_body1_e6940;
            let (assign9560_body2_e6946,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9560_body2_e6944: f64 = (locals.var_inv_sa + locals.var_t0__blk927);
        (assign9560_body2_e6944,)
    } else {
        (locals.var_inv_sa,)
    }
};
            locals.var_inv_sa = assign9560_body2_e6946;
            let (assign9560_body3_e6952,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9560_body3_e6950: f64 = (locals.var_inv_sb + locals.var_t1__blk928);
        (assign9560_body3_e6950,)
    } else {
        (locals.var_inv_sb,)
    }
};
            locals.var_inv_sb = assign9560_body3_e6952;
            let (assign9560_body4_e6958,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9560_body4_e6956: f64 = (locals.var_i + 1.0);
        (assign9560_body4_e6956,)
    } else {
        (locals.var_i,)
    }
};
            locals.var_i = assign9560_body4_e6958;
        }

        let (assign9570_e6964,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9570_e6962: f64 = (locals.var_inv_sa + locals.var_inv_sb);
        (assign9570_e6962,)
    } else {
        (locals.var_inv_odeff,)
    }
};
        locals.var_inv_odeff = assign9570_e6964;

        let (assign9580_e6968,) = {
    if (locals.var_guard924 != 0.0) {
        (locals.var_inv_odeff,)
    } else {
        (locals.var_b4soiinv_odeff,)
    }
};
        locals.var_b4soiinv_odeff = assign9580_e6968;

        let (assign9590_e6976, assign9590_e6976_d_n3, assign9590_e6976_d_n4, assign9590_e6976_d_n5, assign9590_e6976_d_n6, assign9590_e6976_d_n7, assign9590_e6976_d_n8, assign9590_e6976_d_n9, assign9590_e6976_d_n10, assign9590_e6976_d_n11, assign9590_e6976_d_n12,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9590_e6972: f64 = (locals.var_b4soiku0 / locals.var_pparam_b4soiku0temp);
        let assign9590_e6974: f64 = (assign9590_e6972 * locals.var_inv_odeff);
        (assign9590_e6974, ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn3) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff), ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn4) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff), ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn5) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff), ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn6) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff), ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn7) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff), ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn8) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff), ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn9) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff), ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn10) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff), ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn11) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff), ((-((locals.var_b4soiku0 * locals.var_pparam_b4soiku0temp_dn12) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff),)
    } else {
        (locals.var_rho, locals.var_rho_dn3, locals.var_rho_dn4, locals.var_rho_dn5, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn8, locals.var_rho_dn9, locals.var_rho_dn10, locals.var_rho_dn11, locals.var_rho_dn12,)
    }
};
        locals.var_rho = assign9590_e6976;
        locals.var_rho_dn3 = assign9590_e6976_d_n3;
        locals.var_rho_dn4 = assign9590_e6976_d_n4;
        locals.var_rho_dn5 = assign9590_e6976_d_n5;
        locals.var_rho_dn6 = assign9590_e6976_d_n6;
        locals.var_rho_dn7 = assign9590_e6976_d_n7;
        locals.var_rho_dn8 = assign9590_e6976_d_n8;
        locals.var_rho_dn9 = assign9590_e6976_d_n9;
        locals.var_rho_dn10 = assign9590_e6976_d_n10;
        locals.var_rho_dn11 = assign9590_e6976_d_n11;
        locals.var_rho_dn12 = assign9590_e6976_d_n12;

        let (assign9600_e6986, assign9600_e6986_d_n3, assign9600_e6986_d_n4, assign9600_e6986_d_n5, assign9600_e6986_d_n6, assign9600_e6986_d_n7, assign9600_e6986_d_n8, assign9600_e6986_d_n9, assign9600_e6986_d_n10, assign9600_e6986_d_n11, assign9600_e6986_d_n12,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9600_e6980: f64 = (1.0 + locals.var_rho);
        let assign9600_e6983: f64 = (1.0 + locals.var_pparam_b4soirho_ref);
        let assign9600_e6984: f64 = (assign9600_e6980 / assign9600_e6983);
        (assign9600_e6984, (((locals.var_rho_dn3 * assign9600_e6983) - (assign9600_e6980 * locals.var_pparam_b4soirho_ref_dn3)) / (assign9600_e6983 * assign9600_e6983)), (((locals.var_rho_dn4 * assign9600_e6983) - (assign9600_e6980 * locals.var_pparam_b4soirho_ref_dn4)) / (assign9600_e6983 * assign9600_e6983)), (((locals.var_rho_dn5 * assign9600_e6983) - (assign9600_e6980 * locals.var_pparam_b4soirho_ref_dn5)) / (assign9600_e6983 * assign9600_e6983)), (((locals.var_rho_dn6 * assign9600_e6983) - (assign9600_e6980 * locals.var_pparam_b4soirho_ref_dn6)) / (assign9600_e6983 * assign9600_e6983)), (((locals.var_rho_dn7 * assign9600_e6983) - (assign9600_e6980 * locals.var_pparam_b4soirho_ref_dn7)) / (assign9600_e6983 * assign9600_e6983)), (((locals.var_rho_dn8 * assign9600_e6983) - (assign9600_e6980 * locals.var_pparam_b4soirho_ref_dn8)) / (assign9600_e6983 * assign9600_e6983)), (((locals.var_rho_dn9 * assign9600_e6983) - (assign9600_e6980 * locals.var_pparam_b4soirho_ref_dn9)) / (assign9600_e6983 * assign9600_e6983)), (((locals.var_rho_dn10 * assign9600_e6983) - (assign9600_e6980 * locals.var_pparam_b4soirho_ref_dn10)) / (assign9600_e6983 * assign9600_e6983)), (((locals.var_rho_dn11 * assign9600_e6983) - (assign9600_e6980 * locals.var_pparam_b4soirho_ref_dn11)) / (assign9600_e6983 * assign9600_e6983)), (((locals.var_rho_dn12 * assign9600_e6983) - (assign9600_e6980 * locals.var_pparam_b4soirho_ref_dn12)) / (assign9600_e6983 * assign9600_e6983)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign9600_e6986;
        locals.var_t0_dn3 = assign9600_e6986_d_n3;
        locals.var_t0_dn4 = assign9600_e6986_d_n4;
        locals.var_t0_dn5 = assign9600_e6986_d_n5;
        locals.var_t0_dn6 = assign9600_e6986_d_n6;
        locals.var_t0_dn7 = assign9600_e6986_d_n7;
        locals.var_t0_dn8 = assign9600_e6986_d_n8;
        locals.var_t0_dn9 = assign9600_e6986_d_n9;
        locals.var_t0_dn10 = assign9600_e6986_d_n10;
        locals.var_t0_dn11 = assign9600_e6986_d_n11;
        locals.var_t0_dn12 = assign9600_e6986_d_n12;

        let (assign9610_e6992, assign9610_e6992_d_n3, assign9610_e6992_d_n4, assign9610_e6992_d_n5, assign9610_e6992_d_n6, assign9610_e6992_d_n7, assign9610_e6992_d_n8, assign9610_e6992_d_n9, assign9610_e6992_d_n10, assign9610_e6992_d_n11, assign9610_e6992_d_n12,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9610_e6990: f64 = (locals.var_pparam_b4soiu0temp * locals.var_t0);
        (assign9610_e6990, ((locals.var_pparam_b4soiu0temp_dn3 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn3)), ((locals.var_pparam_b4soiu0temp_dn4 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn4)), ((locals.var_pparam_b4soiu0temp_dn5 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn5)), ((locals.var_pparam_b4soiu0temp_dn6 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn6)), ((locals.var_pparam_b4soiu0temp_dn7 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn7)), ((locals.var_pparam_b4soiu0temp_dn8 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn8)), ((locals.var_pparam_b4soiu0temp_dn9 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn9)), ((locals.var_pparam_b4soiu0temp_dn10 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn10)), ((locals.var_pparam_b4soiu0temp_dn11 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn11)), ((locals.var_pparam_b4soiu0temp_dn12 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn12)),)
    } else {
        (locals.var_here_b4soiu0temp, locals.var_here_b4soiu0temp_dn3, locals.var_here_b4soiu0temp_dn4, locals.var_here_b4soiu0temp_dn5, locals.var_here_b4soiu0temp_dn6, locals.var_here_b4soiu0temp_dn7, locals.var_here_b4soiu0temp_dn8, locals.var_here_b4soiu0temp_dn9, locals.var_here_b4soiu0temp_dn10, locals.var_here_b4soiu0temp_dn11, locals.var_here_b4soiu0temp_dn12,)
    }
};
        locals.var_here_b4soiu0temp = assign9610_e6992;
        locals.var_here_b4soiu0temp_dn3 = assign9610_e6992_d_n3;
        locals.var_here_b4soiu0temp_dn4 = assign9610_e6992_d_n4;
        locals.var_here_b4soiu0temp_dn5 = assign9610_e6992_d_n5;
        locals.var_here_b4soiu0temp_dn6 = assign9610_e6992_d_n6;
        locals.var_here_b4soiu0temp_dn7 = assign9610_e6992_d_n7;
        locals.var_here_b4soiu0temp_dn8 = assign9610_e6992_d_n8;
        locals.var_here_b4soiu0temp_dn9 = assign9610_e6992_d_n9;
        locals.var_here_b4soiu0temp_dn10 = assign9610_e6992_d_n10;
        locals.var_here_b4soiu0temp_dn11 = assign9610_e6992_d_n11;
        locals.var_here_b4soiu0temp_dn12 = assign9610_e6992_d_n12;

        let (assign9620_e7006, assign9620_e7006_d_n3, assign9620_e7006_d_n4, assign9620_e7006_d_n5, assign9620_e7006_d_n6, assign9620_e7006_d_n7, assign9620_e7006_d_n8, assign9620_e7006_d_n9, assign9620_e7006_d_n10, assign9620_e7006_d_n11, assign9620_e7006_d_n12,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9620_e6997: f64 = (locals.var_b4soikvsat * locals.var_rho);
        let assign9620_e6998: f64 = (1.0 + assign9620_e6997);
        let assign9620_e7002: f64 = (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref);
        let assign9620_e7003: f64 = (1.0 + assign9620_e7002);
        let assign9620_e7004: f64 = (assign9620_e6998 / assign9620_e7003);
        (assign9620_e7004, ((((locals.var_b4soikvsat * locals.var_rho_dn3) * assign9620_e7003) - (assign9620_e6998 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn3))) / (assign9620_e7003 * assign9620_e7003)), ((((locals.var_b4soikvsat * locals.var_rho_dn4) * assign9620_e7003) - (assign9620_e6998 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn4))) / (assign9620_e7003 * assign9620_e7003)), ((((locals.var_b4soikvsat * locals.var_rho_dn5) * assign9620_e7003) - (assign9620_e6998 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn5))) / (assign9620_e7003 * assign9620_e7003)), ((((locals.var_b4soikvsat * locals.var_rho_dn6) * assign9620_e7003) - (assign9620_e6998 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn6))) / (assign9620_e7003 * assign9620_e7003)), ((((locals.var_b4soikvsat * locals.var_rho_dn7) * assign9620_e7003) - (assign9620_e6998 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn7))) / (assign9620_e7003 * assign9620_e7003)), ((((locals.var_b4soikvsat * locals.var_rho_dn8) * assign9620_e7003) - (assign9620_e6998 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn8))) / (assign9620_e7003 * assign9620_e7003)), ((((locals.var_b4soikvsat * locals.var_rho_dn9) * assign9620_e7003) - (assign9620_e6998 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn9))) / (assign9620_e7003 * assign9620_e7003)), ((((locals.var_b4soikvsat * locals.var_rho_dn10) * assign9620_e7003) - (assign9620_e6998 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn10))) / (assign9620_e7003 * assign9620_e7003)), ((((locals.var_b4soikvsat * locals.var_rho_dn11) * assign9620_e7003) - (assign9620_e6998 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn11))) / (assign9620_e7003 * assign9620_e7003)), ((((locals.var_b4soikvsat * locals.var_rho_dn12) * assign9620_e7003) - (assign9620_e6998 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn12))) / (assign9620_e7003 * assign9620_e7003)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign9620_e7006;
        locals.var_t1_dn3 = assign9620_e7006_d_n3;
        locals.var_t1_dn4 = assign9620_e7006_d_n4;
        locals.var_t1_dn5 = assign9620_e7006_d_n5;
        locals.var_t1_dn6 = assign9620_e7006_d_n6;
        locals.var_t1_dn7 = assign9620_e7006_d_n7;
        locals.var_t1_dn8 = assign9620_e7006_d_n8;
        locals.var_t1_dn9 = assign9620_e7006_d_n9;
        locals.var_t1_dn10 = assign9620_e7006_d_n10;
        locals.var_t1_dn11 = assign9620_e7006_d_n11;
        locals.var_t1_dn12 = assign9620_e7006_d_n12;

        let (assign9630_e7012, assign9630_e7012_d_n3, assign9630_e7012_d_n4, assign9630_e7012_d_n5, assign9630_e7012_d_n6, assign9630_e7012_d_n7, assign9630_e7012_d_n8, assign9630_e7012_d_n9, assign9630_e7012_d_n10, assign9630_e7012_d_n11, assign9630_e7012_d_n12,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9630_e7010: f64 = (locals.var_pparam_b4soivsattemp * locals.var_t1);
        (assign9630_e7010, ((locals.var_pparam_b4soivsattemp_dn3 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn3)), ((locals.var_pparam_b4soivsattemp_dn4 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn4)), ((locals.var_pparam_b4soivsattemp_dn5 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn5)), ((locals.var_pparam_b4soivsattemp_dn6 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn6)), ((locals.var_pparam_b4soivsattemp_dn7 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn7)), ((locals.var_pparam_b4soivsattemp_dn8 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn8)), ((locals.var_pparam_b4soivsattemp_dn9 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn9)), ((locals.var_pparam_b4soivsattemp_dn10 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn10)), ((locals.var_pparam_b4soivsattemp_dn11 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn11)), ((locals.var_pparam_b4soivsattemp_dn12 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn12)),)
    } else {
        (locals.var_here_b4soivsattemp, locals.var_here_b4soivsattemp_dn3, locals.var_here_b4soivsattemp_dn4, locals.var_here_b4soivsattemp_dn5, locals.var_here_b4soivsattemp_dn6, locals.var_here_b4soivsattemp_dn7, locals.var_here_b4soivsattemp_dn8, locals.var_here_b4soivsattemp_dn9, locals.var_here_b4soivsattemp_dn10, locals.var_here_b4soivsattemp_dn11, locals.var_here_b4soivsattemp_dn12,)
    }
};
        locals.var_here_b4soivsattemp = assign9630_e7012;
        locals.var_here_b4soivsattemp_dn3 = assign9630_e7012_d_n3;
        locals.var_here_b4soivsattemp_dn4 = assign9630_e7012_d_n4;
        locals.var_here_b4soivsattemp_dn5 = assign9630_e7012_d_n5;
        locals.var_here_b4soivsattemp_dn6 = assign9630_e7012_d_n6;
        locals.var_here_b4soivsattemp_dn7 = assign9630_e7012_d_n7;
        locals.var_here_b4soivsattemp_dn8 = assign9630_e7012_d_n8;
        locals.var_here_b4soivsattemp_dn9 = assign9630_e7012_d_n9;
        locals.var_here_b4soivsattemp_dn10 = assign9630_e7012_d_n10;
        locals.var_here_b4soivsattemp_dn11 = assign9630_e7012_d_n11;
        locals.var_here_b4soivsattemp_dn12 = assign9630_e7012_d_n12;

        let (assign9640_e7018,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9640_e7016: f64 = (locals.var_inv_odeff - locals.var_pparam_b4soiinv_od_ref);
        (assign9640_e7016,)
    } else {
        (locals.var_od_offset,)
    }
};
        locals.var_od_offset = assign9640_e7018;

        let (assign9650_e7026, assign9650_e7026_d_n3, assign9650_e7026_d_n4, assign9650_e7026_d_n5, assign9650_e7026_d_n6, assign9650_e7026_d_n7, assign9650_e7026_d_n8, assign9650_e7026_d_n9, assign9650_e7026_d_n10, assign9650_e7026_d_n11, assign9650_e7026_d_n12,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9650_e7022: f64 = (locals.var_b4soikvth0 / locals.var_pparam_b4soikvth0);
        let assign9650_e7024: f64 = (assign9650_e7022 * locals.var_od_offset);
        (assign9650_e7024, ((-((locals.var_b4soikvth0 * locals.var_pparam_b4soikvth0_dn3) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset), ((-((locals.var_b4soikvth0 * locals.var_pparam_b4soikvth0_dn4) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset), ((-((locals.var_b4soikvth0 * locals.var_pparam_b4soikvth0_dn5) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset), ((-((locals.var_b4soikvth0 * locals.var_pparam_b4soikvth0_dn6) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset), ((-((locals.var_b4soikvth0 * locals.var_pparam_b4soikvth0_dn7) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset), ((-((locals.var_b4soikvth0 * locals.var_pparam_b4soikvth0_dn8) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset), ((-((locals.var_b4soikvth0 * locals.var_pparam_b4soikvth0_dn9) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset), ((-((locals.var_b4soikvth0 * locals.var_pparam_b4soikvth0_dn10) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset), ((-((locals.var_b4soikvth0 * locals.var_pparam_b4soikvth0_dn11) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset), ((-((locals.var_b4soikvth0 * locals.var_pparam_b4soikvth0_dn12) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset),)
    } else {
        (locals.var_dvth0_lod, locals.var_dvth0_lod_dn3, locals.var_dvth0_lod_dn4, locals.var_dvth0_lod_dn5, locals.var_dvth0_lod_dn6, locals.var_dvth0_lod_dn7, locals.var_dvth0_lod_dn8, locals.var_dvth0_lod_dn9, locals.var_dvth0_lod_dn10, locals.var_dvth0_lod_dn11, locals.var_dvth0_lod_dn12,)
    }
};
        locals.var_dvth0_lod = assign9650_e7026;
        locals.var_dvth0_lod_dn3 = assign9650_e7026_d_n3;
        locals.var_dvth0_lod_dn4 = assign9650_e7026_d_n4;
        locals.var_dvth0_lod_dn5 = assign9650_e7026_d_n5;
        locals.var_dvth0_lod_dn6 = assign9650_e7026_d_n6;
        locals.var_dvth0_lod_dn7 = assign9650_e7026_d_n7;
        locals.var_dvth0_lod_dn8 = assign9650_e7026_d_n8;
        locals.var_dvth0_lod_dn9 = assign9650_e7026_d_n9;
        locals.var_dvth0_lod_dn10 = assign9650_e7026_d_n10;
        locals.var_dvth0_lod_dn11 = assign9650_e7026_d_n11;
        locals.var_dvth0_lod_dn12 = assign9650_e7026_d_n12;

        let (assign9660_e7036, assign9660_e7036_d_n3, assign9660_e7036_d_n4, assign9660_e7036_d_n5, assign9660_e7036_d_n6, assign9660_e7036_d_n7, assign9660_e7036_d_n8, assign9660_e7036_d_n9, assign9660_e7036_d_n10, assign9660_e7036_d_n11, assign9660_e7036_d_n12,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9660_e7031: f64 = (locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodk2);
        let assign9660_e7032: f64 = (locals.var_b4soistk2 / assign9660_e7031);
        let assign9660_e7034: f64 = (assign9660_e7032 * locals.var_od_offset);
        (assign9660_e7034, ((-((locals.var_b4soistk2 * if 0.0 == 0.0 && ((locals.var_b4soilodk2) as f64).is_finite() && ((locals.var_b4soilodk2) as f64).fract() == 0.0 { if locals.var_b4soilodk2 == 0.0 { 0.0 } else { (locals.var_b4soilodk2 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodk2 - 1.0) * locals.var_pparam_b4soikvth0_dn3)) } } else { (assign9660_e7031 * (locals.var_b4soilodk2 * (locals.var_pparam_b4soikvth0_dn3 / locals.var_pparam_b4soikvth0))) }) / (assign9660_e7031 * assign9660_e7031))) * locals.var_od_offset), ((-((locals.var_b4soistk2 * if 0.0 == 0.0 && ((locals.var_b4soilodk2) as f64).is_finite() && ((locals.var_b4soilodk2) as f64).fract() == 0.0 { if locals.var_b4soilodk2 == 0.0 { 0.0 } else { (locals.var_b4soilodk2 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodk2 - 1.0) * locals.var_pparam_b4soikvth0_dn4)) } } else { (assign9660_e7031 * (locals.var_b4soilodk2 * (locals.var_pparam_b4soikvth0_dn4 / locals.var_pparam_b4soikvth0))) }) / (assign9660_e7031 * assign9660_e7031))) * locals.var_od_offset), ((-((locals.var_b4soistk2 * if 0.0 == 0.0 && ((locals.var_b4soilodk2) as f64).is_finite() && ((locals.var_b4soilodk2) as f64).fract() == 0.0 { if locals.var_b4soilodk2 == 0.0 { 0.0 } else { (locals.var_b4soilodk2 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodk2 - 1.0) * locals.var_pparam_b4soikvth0_dn5)) } } else { (assign9660_e7031 * (locals.var_b4soilodk2 * (locals.var_pparam_b4soikvth0_dn5 / locals.var_pparam_b4soikvth0))) }) / (assign9660_e7031 * assign9660_e7031))) * locals.var_od_offset), ((-((locals.var_b4soistk2 * if 0.0 == 0.0 && ((locals.var_b4soilodk2) as f64).is_finite() && ((locals.var_b4soilodk2) as f64).fract() == 0.0 { if locals.var_b4soilodk2 == 0.0 { 0.0 } else { (locals.var_b4soilodk2 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodk2 - 1.0) * locals.var_pparam_b4soikvth0_dn6)) } } else { (assign9660_e7031 * (locals.var_b4soilodk2 * (locals.var_pparam_b4soikvth0_dn6 / locals.var_pparam_b4soikvth0))) }) / (assign9660_e7031 * assign9660_e7031))) * locals.var_od_offset), ((-((locals.var_b4soistk2 * if 0.0 == 0.0 && ((locals.var_b4soilodk2) as f64).is_finite() && ((locals.var_b4soilodk2) as f64).fract() == 0.0 { if locals.var_b4soilodk2 == 0.0 { 0.0 } else { (locals.var_b4soilodk2 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodk2 - 1.0) * locals.var_pparam_b4soikvth0_dn7)) } } else { (assign9660_e7031 * (locals.var_b4soilodk2 * (locals.var_pparam_b4soikvth0_dn7 / locals.var_pparam_b4soikvth0))) }) / (assign9660_e7031 * assign9660_e7031))) * locals.var_od_offset), ((-((locals.var_b4soistk2 * if 0.0 == 0.0 && ((locals.var_b4soilodk2) as f64).is_finite() && ((locals.var_b4soilodk2) as f64).fract() == 0.0 { if locals.var_b4soilodk2 == 0.0 { 0.0 } else { (locals.var_b4soilodk2 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodk2 - 1.0) * locals.var_pparam_b4soikvth0_dn8)) } } else { (assign9660_e7031 * (locals.var_b4soilodk2 * (locals.var_pparam_b4soikvth0_dn8 / locals.var_pparam_b4soikvth0))) }) / (assign9660_e7031 * assign9660_e7031))) * locals.var_od_offset), ((-((locals.var_b4soistk2 * if 0.0 == 0.0 && ((locals.var_b4soilodk2) as f64).is_finite() && ((locals.var_b4soilodk2) as f64).fract() == 0.0 { if locals.var_b4soilodk2 == 0.0 { 0.0 } else { (locals.var_b4soilodk2 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodk2 - 1.0) * locals.var_pparam_b4soikvth0_dn9)) } } else { (assign9660_e7031 * (locals.var_b4soilodk2 * (locals.var_pparam_b4soikvth0_dn9 / locals.var_pparam_b4soikvth0))) }) / (assign9660_e7031 * assign9660_e7031))) * locals.var_od_offset), ((-((locals.var_b4soistk2 * if 0.0 == 0.0 && ((locals.var_b4soilodk2) as f64).is_finite() && ((locals.var_b4soilodk2) as f64).fract() == 0.0 { if locals.var_b4soilodk2 == 0.0 { 0.0 } else { (locals.var_b4soilodk2 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodk2 - 1.0) * locals.var_pparam_b4soikvth0_dn10)) } } else { (assign9660_e7031 * (locals.var_b4soilodk2 * (locals.var_pparam_b4soikvth0_dn10 / locals.var_pparam_b4soikvth0))) }) / (assign9660_e7031 * assign9660_e7031))) * locals.var_od_offset), ((-((locals.var_b4soistk2 * if 0.0 == 0.0 && ((locals.var_b4soilodk2) as f64).is_finite() && ((locals.var_b4soilodk2) as f64).fract() == 0.0 { if locals.var_b4soilodk2 == 0.0 { 0.0 } else { (locals.var_b4soilodk2 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodk2 - 1.0) * locals.var_pparam_b4soikvth0_dn11)) } } else { (assign9660_e7031 * (locals.var_b4soilodk2 * (locals.var_pparam_b4soikvth0_dn11 / locals.var_pparam_b4soikvth0))) }) / (assign9660_e7031 * assign9660_e7031))) * locals.var_od_offset), ((-((locals.var_b4soistk2 * if 0.0 == 0.0 && ((locals.var_b4soilodk2) as f64).is_finite() && ((locals.var_b4soilodk2) as f64).fract() == 0.0 { if locals.var_b4soilodk2 == 0.0 { 0.0 } else { (locals.var_b4soilodk2 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodk2 - 1.0) * locals.var_pparam_b4soikvth0_dn12)) } } else { (assign9660_e7031 * (locals.var_b4soilodk2 * (locals.var_pparam_b4soikvth0_dn12 / locals.var_pparam_b4soikvth0))) }) / (assign9660_e7031 * assign9660_e7031))) * locals.var_od_offset),)
    } else {
        (locals.var_dk2_lod, locals.var_dk2_lod_dn3, locals.var_dk2_lod_dn4, locals.var_dk2_lod_dn5, locals.var_dk2_lod_dn6, locals.var_dk2_lod_dn7, locals.var_dk2_lod_dn8, locals.var_dk2_lod_dn9, locals.var_dk2_lod_dn10, locals.var_dk2_lod_dn11, locals.var_dk2_lod_dn12,)
    }
};
        locals.var_dk2_lod = assign9660_e7036;
        locals.var_dk2_lod_dn3 = assign9660_e7036_d_n3;
        locals.var_dk2_lod_dn4 = assign9660_e7036_d_n4;
        locals.var_dk2_lod_dn5 = assign9660_e7036_d_n5;
        locals.var_dk2_lod_dn6 = assign9660_e7036_d_n6;
        locals.var_dk2_lod_dn7 = assign9660_e7036_d_n7;
        locals.var_dk2_lod_dn8 = assign9660_e7036_d_n8;
        locals.var_dk2_lod_dn9 = assign9660_e7036_d_n9;
        locals.var_dk2_lod_dn10 = assign9660_e7036_d_n10;
        locals.var_dk2_lod_dn11 = assign9660_e7036_d_n11;
        locals.var_dk2_lod_dn12 = assign9660_e7036_d_n12;

        let (assign9670_e7046, assign9670_e7046_d_n3, assign9670_e7046_d_n4, assign9670_e7046_d_n5, assign9670_e7046_d_n6, assign9670_e7046_d_n7, assign9670_e7046_d_n8, assign9670_e7046_d_n9, assign9670_e7046_d_n10, assign9670_e7046_d_n11, assign9670_e7046_d_n12,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9670_e7041: f64 = (locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0);
        let assign9670_e7042: f64 = (locals.var_b4soisteta0 / assign9670_e7041);
        let assign9670_e7044: f64 = (assign9670_e7042 * locals.var_od_offset);
        (assign9670_e7044, ((-((locals.var_b4soisteta0 * if 0.0 == 0.0 && ((locals.var_b4soilodeta0) as f64).is_finite() && ((locals.var_b4soilodeta0) as f64).fract() == 0.0 { if locals.var_b4soilodeta0 == 0.0 { 0.0 } else { (locals.var_b4soilodeta0 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0 - 1.0) * locals.var_pparam_b4soikvth0_dn3)) } } else { (assign9670_e7041 * (locals.var_b4soilodeta0 * (locals.var_pparam_b4soikvth0_dn3 / locals.var_pparam_b4soikvth0))) }) / (assign9670_e7041 * assign9670_e7041))) * locals.var_od_offset), ((-((locals.var_b4soisteta0 * if 0.0 == 0.0 && ((locals.var_b4soilodeta0) as f64).is_finite() && ((locals.var_b4soilodeta0) as f64).fract() == 0.0 { if locals.var_b4soilodeta0 == 0.0 { 0.0 } else { (locals.var_b4soilodeta0 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0 - 1.0) * locals.var_pparam_b4soikvth0_dn4)) } } else { (assign9670_e7041 * (locals.var_b4soilodeta0 * (locals.var_pparam_b4soikvth0_dn4 / locals.var_pparam_b4soikvth0))) }) / (assign9670_e7041 * assign9670_e7041))) * locals.var_od_offset), ((-((locals.var_b4soisteta0 * if 0.0 == 0.0 && ((locals.var_b4soilodeta0) as f64).is_finite() && ((locals.var_b4soilodeta0) as f64).fract() == 0.0 { if locals.var_b4soilodeta0 == 0.0 { 0.0 } else { (locals.var_b4soilodeta0 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0 - 1.0) * locals.var_pparam_b4soikvth0_dn5)) } } else { (assign9670_e7041 * (locals.var_b4soilodeta0 * (locals.var_pparam_b4soikvth0_dn5 / locals.var_pparam_b4soikvth0))) }) / (assign9670_e7041 * assign9670_e7041))) * locals.var_od_offset), ((-((locals.var_b4soisteta0 * if 0.0 == 0.0 && ((locals.var_b4soilodeta0) as f64).is_finite() && ((locals.var_b4soilodeta0) as f64).fract() == 0.0 { if locals.var_b4soilodeta0 == 0.0 { 0.0 } else { (locals.var_b4soilodeta0 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0 - 1.0) * locals.var_pparam_b4soikvth0_dn6)) } } else { (assign9670_e7041 * (locals.var_b4soilodeta0 * (locals.var_pparam_b4soikvth0_dn6 / locals.var_pparam_b4soikvth0))) }) / (assign9670_e7041 * assign9670_e7041))) * locals.var_od_offset), ((-((locals.var_b4soisteta0 * if 0.0 == 0.0 && ((locals.var_b4soilodeta0) as f64).is_finite() && ((locals.var_b4soilodeta0) as f64).fract() == 0.0 { if locals.var_b4soilodeta0 == 0.0 { 0.0 } else { (locals.var_b4soilodeta0 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0 - 1.0) * locals.var_pparam_b4soikvth0_dn7)) } } else { (assign9670_e7041 * (locals.var_b4soilodeta0 * (locals.var_pparam_b4soikvth0_dn7 / locals.var_pparam_b4soikvth0))) }) / (assign9670_e7041 * assign9670_e7041))) * locals.var_od_offset), ((-((locals.var_b4soisteta0 * if 0.0 == 0.0 && ((locals.var_b4soilodeta0) as f64).is_finite() && ((locals.var_b4soilodeta0) as f64).fract() == 0.0 { if locals.var_b4soilodeta0 == 0.0 { 0.0 } else { (locals.var_b4soilodeta0 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0 - 1.0) * locals.var_pparam_b4soikvth0_dn8)) } } else { (assign9670_e7041 * (locals.var_b4soilodeta0 * (locals.var_pparam_b4soikvth0_dn8 / locals.var_pparam_b4soikvth0))) }) / (assign9670_e7041 * assign9670_e7041))) * locals.var_od_offset), ((-((locals.var_b4soisteta0 * if 0.0 == 0.0 && ((locals.var_b4soilodeta0) as f64).is_finite() && ((locals.var_b4soilodeta0) as f64).fract() == 0.0 { if locals.var_b4soilodeta0 == 0.0 { 0.0 } else { (locals.var_b4soilodeta0 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0 - 1.0) * locals.var_pparam_b4soikvth0_dn9)) } } else { (assign9670_e7041 * (locals.var_b4soilodeta0 * (locals.var_pparam_b4soikvth0_dn9 / locals.var_pparam_b4soikvth0))) }) / (assign9670_e7041 * assign9670_e7041))) * locals.var_od_offset), ((-((locals.var_b4soisteta0 * if 0.0 == 0.0 && ((locals.var_b4soilodeta0) as f64).is_finite() && ((locals.var_b4soilodeta0) as f64).fract() == 0.0 { if locals.var_b4soilodeta0 == 0.0 { 0.0 } else { (locals.var_b4soilodeta0 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0 - 1.0) * locals.var_pparam_b4soikvth0_dn10)) } } else { (assign9670_e7041 * (locals.var_b4soilodeta0 * (locals.var_pparam_b4soikvth0_dn10 / locals.var_pparam_b4soikvth0))) }) / (assign9670_e7041 * assign9670_e7041))) * locals.var_od_offset), ((-((locals.var_b4soisteta0 * if 0.0 == 0.0 && ((locals.var_b4soilodeta0) as f64).is_finite() && ((locals.var_b4soilodeta0) as f64).fract() == 0.0 { if locals.var_b4soilodeta0 == 0.0 { 0.0 } else { (locals.var_b4soilodeta0 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0 - 1.0) * locals.var_pparam_b4soikvth0_dn11)) } } else { (assign9670_e7041 * (locals.var_b4soilodeta0 * (locals.var_pparam_b4soikvth0_dn11 / locals.var_pparam_b4soikvth0))) }) / (assign9670_e7041 * assign9670_e7041))) * locals.var_od_offset), ((-((locals.var_b4soisteta0 * if 0.0 == 0.0 && ((locals.var_b4soilodeta0) as f64).is_finite() && ((locals.var_b4soilodeta0) as f64).fract() == 0.0 { if locals.var_b4soilodeta0 == 0.0 { 0.0 } else { (locals.var_b4soilodeta0 * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0 - 1.0) * locals.var_pparam_b4soikvth0_dn12)) } } else { (assign9670_e7041 * (locals.var_b4soilodeta0 * (locals.var_pparam_b4soikvth0_dn12 / locals.var_pparam_b4soikvth0))) }) / (assign9670_e7041 * assign9670_e7041))) * locals.var_od_offset),)
    } else {
        (locals.var_deta0_lod, locals.var_deta0_lod_dn3, locals.var_deta0_lod_dn4, locals.var_deta0_lod_dn5, locals.var_deta0_lod_dn6, locals.var_deta0_lod_dn7, locals.var_deta0_lod_dn8, locals.var_deta0_lod_dn9, locals.var_deta0_lod_dn10, locals.var_deta0_lod_dn11, locals.var_deta0_lod_dn12,)
    }
};
        locals.var_deta0_lod = assign9670_e7046;
        locals.var_deta0_lod_dn3 = assign9670_e7046_d_n3;
        locals.var_deta0_lod_dn4 = assign9670_e7046_d_n4;
        locals.var_deta0_lod_dn5 = assign9670_e7046_d_n5;
        locals.var_deta0_lod_dn6 = assign9670_e7046_d_n6;
        locals.var_deta0_lod_dn7 = assign9670_e7046_d_n7;
        locals.var_deta0_lod_dn8 = assign9670_e7046_d_n8;
        locals.var_deta0_lod_dn9 = assign9670_e7046_d_n9;
        locals.var_deta0_lod_dn10 = assign9670_e7046_d_n10;
        locals.var_deta0_lod_dn11 = assign9670_e7046_d_n11;
        locals.var_deta0_lod_dn12 = assign9670_e7046_d_n12;

    }

    pub(super) fn stamp_transient_block_18(
        locals: &mut StampLocals,
    ) {
        let (assign9680_e7056, assign9680_e7056_d_n3, assign9680_e7056_d_n4, assign9680_e7056_d_n5, assign9680_e7056_d_n6, assign9680_e7056_d_n7, assign9680_e7056_d_n8, assign9680_e7056_d_n9, assign9680_e7056_d_n10, assign9680_e7056_d_n11, assign9680_e7056_d_n12,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9680_e7051: f64 = (locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0cv);
        let assign9680_e7052: f64 = (locals.var_b4soisteta0cv / assign9680_e7051);
        let assign9680_e7054: f64 = (assign9680_e7052 * locals.var_od_offset);
        (assign9680_e7054, ((-((locals.var_b4soisteta0cv * if 0.0 == 0.0 && ((locals.var_b4soilodeta0cv) as f64).is_finite() && ((locals.var_b4soilodeta0cv) as f64).fract() == 0.0 { if locals.var_b4soilodeta0cv == 0.0 { 0.0 } else { (locals.var_b4soilodeta0cv * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0cv - 1.0) * locals.var_pparam_b4soikvth0_dn3)) } } else { (assign9680_e7051 * (locals.var_b4soilodeta0cv * (locals.var_pparam_b4soikvth0_dn3 / locals.var_pparam_b4soikvth0))) }) / (assign9680_e7051 * assign9680_e7051))) * locals.var_od_offset), ((-((locals.var_b4soisteta0cv * if 0.0 == 0.0 && ((locals.var_b4soilodeta0cv) as f64).is_finite() && ((locals.var_b4soilodeta0cv) as f64).fract() == 0.0 { if locals.var_b4soilodeta0cv == 0.0 { 0.0 } else { (locals.var_b4soilodeta0cv * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0cv - 1.0) * locals.var_pparam_b4soikvth0_dn4)) } } else { (assign9680_e7051 * (locals.var_b4soilodeta0cv * (locals.var_pparam_b4soikvth0_dn4 / locals.var_pparam_b4soikvth0))) }) / (assign9680_e7051 * assign9680_e7051))) * locals.var_od_offset), ((-((locals.var_b4soisteta0cv * if 0.0 == 0.0 && ((locals.var_b4soilodeta0cv) as f64).is_finite() && ((locals.var_b4soilodeta0cv) as f64).fract() == 0.0 { if locals.var_b4soilodeta0cv == 0.0 { 0.0 } else { (locals.var_b4soilodeta0cv * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0cv - 1.0) * locals.var_pparam_b4soikvth0_dn5)) } } else { (assign9680_e7051 * (locals.var_b4soilodeta0cv * (locals.var_pparam_b4soikvth0_dn5 / locals.var_pparam_b4soikvth0))) }) / (assign9680_e7051 * assign9680_e7051))) * locals.var_od_offset), ((-((locals.var_b4soisteta0cv * if 0.0 == 0.0 && ((locals.var_b4soilodeta0cv) as f64).is_finite() && ((locals.var_b4soilodeta0cv) as f64).fract() == 0.0 { if locals.var_b4soilodeta0cv == 0.0 { 0.0 } else { (locals.var_b4soilodeta0cv * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0cv - 1.0) * locals.var_pparam_b4soikvth0_dn6)) } } else { (assign9680_e7051 * (locals.var_b4soilodeta0cv * (locals.var_pparam_b4soikvth0_dn6 / locals.var_pparam_b4soikvth0))) }) / (assign9680_e7051 * assign9680_e7051))) * locals.var_od_offset), ((-((locals.var_b4soisteta0cv * if 0.0 == 0.0 && ((locals.var_b4soilodeta0cv) as f64).is_finite() && ((locals.var_b4soilodeta0cv) as f64).fract() == 0.0 { if locals.var_b4soilodeta0cv == 0.0 { 0.0 } else { (locals.var_b4soilodeta0cv * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0cv - 1.0) * locals.var_pparam_b4soikvth0_dn7)) } } else { (assign9680_e7051 * (locals.var_b4soilodeta0cv * (locals.var_pparam_b4soikvth0_dn7 / locals.var_pparam_b4soikvth0))) }) / (assign9680_e7051 * assign9680_e7051))) * locals.var_od_offset), ((-((locals.var_b4soisteta0cv * if 0.0 == 0.0 && ((locals.var_b4soilodeta0cv) as f64).is_finite() && ((locals.var_b4soilodeta0cv) as f64).fract() == 0.0 { if locals.var_b4soilodeta0cv == 0.0 { 0.0 } else { (locals.var_b4soilodeta0cv * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0cv - 1.0) * locals.var_pparam_b4soikvth0_dn8)) } } else { (assign9680_e7051 * (locals.var_b4soilodeta0cv * (locals.var_pparam_b4soikvth0_dn8 / locals.var_pparam_b4soikvth0))) }) / (assign9680_e7051 * assign9680_e7051))) * locals.var_od_offset), ((-((locals.var_b4soisteta0cv * if 0.0 == 0.0 && ((locals.var_b4soilodeta0cv) as f64).is_finite() && ((locals.var_b4soilodeta0cv) as f64).fract() == 0.0 { if locals.var_b4soilodeta0cv == 0.0 { 0.0 } else { (locals.var_b4soilodeta0cv * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0cv - 1.0) * locals.var_pparam_b4soikvth0_dn9)) } } else { (assign9680_e7051 * (locals.var_b4soilodeta0cv * (locals.var_pparam_b4soikvth0_dn9 / locals.var_pparam_b4soikvth0))) }) / (assign9680_e7051 * assign9680_e7051))) * locals.var_od_offset), ((-((locals.var_b4soisteta0cv * if 0.0 == 0.0 && ((locals.var_b4soilodeta0cv) as f64).is_finite() && ((locals.var_b4soilodeta0cv) as f64).fract() == 0.0 { if locals.var_b4soilodeta0cv == 0.0 { 0.0 } else { (locals.var_b4soilodeta0cv * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0cv - 1.0) * locals.var_pparam_b4soikvth0_dn10)) } } else { (assign9680_e7051 * (locals.var_b4soilodeta0cv * (locals.var_pparam_b4soikvth0_dn10 / locals.var_pparam_b4soikvth0))) }) / (assign9680_e7051 * assign9680_e7051))) * locals.var_od_offset), ((-((locals.var_b4soisteta0cv * if 0.0 == 0.0 && ((locals.var_b4soilodeta0cv) as f64).is_finite() && ((locals.var_b4soilodeta0cv) as f64).fract() == 0.0 { if locals.var_b4soilodeta0cv == 0.0 { 0.0 } else { (locals.var_b4soilodeta0cv * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0cv - 1.0) * locals.var_pparam_b4soikvth0_dn11)) } } else { (assign9680_e7051 * (locals.var_b4soilodeta0cv * (locals.var_pparam_b4soikvth0_dn11 / locals.var_pparam_b4soikvth0))) }) / (assign9680_e7051 * assign9680_e7051))) * locals.var_od_offset), ((-((locals.var_b4soisteta0cv * if 0.0 == 0.0 && ((locals.var_b4soilodeta0cv) as f64).is_finite() && ((locals.var_b4soilodeta0cv) as f64).fract() == 0.0 { if locals.var_b4soilodeta0cv == 0.0 { 0.0 } else { (locals.var_b4soilodeta0cv * ((locals.var_pparam_b4soikvth0).powf(locals.var_b4soilodeta0cv - 1.0) * locals.var_pparam_b4soikvth0_dn12)) } } else { (assign9680_e7051 * (locals.var_b4soilodeta0cv * (locals.var_pparam_b4soikvth0_dn12 / locals.var_pparam_b4soikvth0))) }) / (assign9680_e7051 * assign9680_e7051))) * locals.var_od_offset),)
    } else {
        (locals.var_deta0cv_lod, locals.var_deta0cv_lod_dn3, locals.var_deta0cv_lod_dn4, locals.var_deta0cv_lod_dn5, locals.var_deta0cv_lod_dn6, locals.var_deta0cv_lod_dn7, locals.var_deta0cv_lod_dn8, locals.var_deta0cv_lod_dn9, locals.var_deta0cv_lod_dn10, locals.var_deta0cv_lod_dn11, locals.var_deta0cv_lod_dn12,)
    }
};
        locals.var_deta0cv_lod = assign9680_e7056;
        locals.var_deta0cv_lod_dn3 = assign9680_e7056_d_n3;
        locals.var_deta0cv_lod_dn4 = assign9680_e7056_d_n4;
        locals.var_deta0cv_lod_dn5 = assign9680_e7056_d_n5;
        locals.var_deta0cv_lod_dn6 = assign9680_e7056_d_n6;
        locals.var_deta0cv_lod_dn7 = assign9680_e7056_d_n7;
        locals.var_deta0cv_lod_dn8 = assign9680_e7056_d_n8;
        locals.var_deta0cv_lod_dn9 = assign9680_e7056_d_n9;
        locals.var_deta0cv_lod_dn10 = assign9680_e7056_d_n10;
        locals.var_deta0cv_lod_dn11 = assign9680_e7056_d_n11;
        locals.var_deta0cv_lod_dn12 = assign9680_e7056_d_n12;

        let (assign9690_e7062, assign9690_e7062_d_n3, assign9690_e7062_d_n4, assign9690_e7062_d_n5, assign9690_e7062_d_n6, assign9690_e7062_d_n7, assign9690_e7062_d_n8, assign9690_e7062_d_n9, assign9690_e7062_d_n10, assign9690_e7062_d_n11, assign9690_e7062_d_n12,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9690_e7060: f64 = (locals.var_pparam_b4soivth0 + locals.var_dvth0_lod);
        (assign9690_e7060, (locals.var_pparam_b4soivth0_dn3 + locals.var_dvth0_lod_dn3), (locals.var_pparam_b4soivth0_dn4 + locals.var_dvth0_lod_dn4), (locals.var_pparam_b4soivth0_dn5 + locals.var_dvth0_lod_dn5), (locals.var_pparam_b4soivth0_dn6 + locals.var_dvth0_lod_dn6), (locals.var_pparam_b4soivth0_dn7 + locals.var_dvth0_lod_dn7), (locals.var_pparam_b4soivth0_dn8 + locals.var_dvth0_lod_dn8), (locals.var_pparam_b4soivth0_dn9 + locals.var_dvth0_lod_dn9), (locals.var_pparam_b4soivth0_dn10 + locals.var_dvth0_lod_dn10), (locals.var_pparam_b4soivth0_dn11 + locals.var_dvth0_lod_dn11), (locals.var_pparam_b4soivth0_dn12 + locals.var_dvth0_lod_dn12),)
    } else {
        (locals.var_here_b4soivth0, locals.var_here_b4soivth0_dn3, locals.var_here_b4soivth0_dn4, locals.var_here_b4soivth0_dn5, locals.var_here_b4soivth0_dn6, locals.var_here_b4soivth0_dn7, locals.var_here_b4soivth0_dn8, locals.var_here_b4soivth0_dn9, locals.var_here_b4soivth0_dn10, locals.var_here_b4soivth0_dn11, locals.var_here_b4soivth0_dn12,)
    }
};
        locals.var_here_b4soivth0 = assign9690_e7062;
        locals.var_here_b4soivth0_dn3 = assign9690_e7062_d_n3;
        locals.var_here_b4soivth0_dn4 = assign9690_e7062_d_n4;
        locals.var_here_b4soivth0_dn5 = assign9690_e7062_d_n5;
        locals.var_here_b4soivth0_dn6 = assign9690_e7062_d_n6;
        locals.var_here_b4soivth0_dn7 = assign9690_e7062_d_n7;
        locals.var_here_b4soivth0_dn8 = assign9690_e7062_d_n8;
        locals.var_here_b4soivth0_dn9 = assign9690_e7062_d_n9;
        locals.var_here_b4soivth0_dn10 = assign9690_e7062_d_n10;
        locals.var_here_b4soivth0_dn11 = assign9690_e7062_d_n11;
        locals.var_here_b4soivth0_dn12 = assign9690_e7062_d_n12;

        let (assign9700_e7068, assign9700_e7068_d_n3, assign9700_e7068_d_n4, assign9700_e7068_d_n5, assign9700_e7068_d_n6, assign9700_e7068_d_n7, assign9700_e7068_d_n8, assign9700_e7068_d_n9, assign9700_e7068_d_n10, assign9700_e7068_d_n11, assign9700_e7068_d_n12,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9700_e7066: f64 = (locals.var_pparam_b4soik2 + locals.var_dk2_lod);
        (assign9700_e7066, (locals.var_pparam_b4soik2_dn3 + locals.var_dk2_lod_dn3), (locals.var_pparam_b4soik2_dn4 + locals.var_dk2_lod_dn4), (locals.var_pparam_b4soik2_dn5 + locals.var_dk2_lod_dn5), (locals.var_pparam_b4soik2_dn6 + locals.var_dk2_lod_dn6), (locals.var_pparam_b4soik2_dn7 + locals.var_dk2_lod_dn7), (locals.var_pparam_b4soik2_dn8 + locals.var_dk2_lod_dn8), (locals.var_pparam_b4soik2_dn9 + locals.var_dk2_lod_dn9), (locals.var_pparam_b4soik2_dn10 + locals.var_dk2_lod_dn10), (locals.var_pparam_b4soik2_dn11 + locals.var_dk2_lod_dn11), (locals.var_pparam_b4soik2_dn12 + locals.var_dk2_lod_dn12),)
    } else {
        (locals.var_here_b4soik2, locals.var_here_b4soik2_dn3, locals.var_here_b4soik2_dn4, locals.var_here_b4soik2_dn5, locals.var_here_b4soik2_dn6, locals.var_here_b4soik2_dn7, locals.var_here_b4soik2_dn8, locals.var_here_b4soik2_dn9, locals.var_here_b4soik2_dn10, locals.var_here_b4soik2_dn11, locals.var_here_b4soik2_dn12,)
    }
};
        locals.var_here_b4soik2 = assign9700_e7068;
        locals.var_here_b4soik2_dn3 = assign9700_e7068_d_n3;
        locals.var_here_b4soik2_dn4 = assign9700_e7068_d_n4;
        locals.var_here_b4soik2_dn5 = assign9700_e7068_d_n5;
        locals.var_here_b4soik2_dn6 = assign9700_e7068_d_n6;
        locals.var_here_b4soik2_dn7 = assign9700_e7068_d_n7;
        locals.var_here_b4soik2_dn8 = assign9700_e7068_d_n8;
        locals.var_here_b4soik2_dn9 = assign9700_e7068_d_n9;
        locals.var_here_b4soik2_dn10 = assign9700_e7068_d_n10;
        locals.var_here_b4soik2_dn11 = assign9700_e7068_d_n11;
        locals.var_here_b4soik2_dn12 = assign9700_e7068_d_n12;

        let (assign9710_e7074, assign9710_e7074_d_n3, assign9710_e7074_d_n4, assign9710_e7074_d_n5, assign9710_e7074_d_n6, assign9710_e7074_d_n7, assign9710_e7074_d_n8, assign9710_e7074_d_n9, assign9710_e7074_d_n10, assign9710_e7074_d_n11, assign9710_e7074_d_n12,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9710_e7072: f64 = (locals.var_pparam_b4soieta0 + locals.var_deta0_lod);
        (assign9710_e7072, (locals.var_pparam_b4soieta0_dn3 + locals.var_deta0_lod_dn3), (locals.var_pparam_b4soieta0_dn4 + locals.var_deta0_lod_dn4), (locals.var_pparam_b4soieta0_dn5 + locals.var_deta0_lod_dn5), (locals.var_pparam_b4soieta0_dn6 + locals.var_deta0_lod_dn6), (locals.var_pparam_b4soieta0_dn7 + locals.var_deta0_lod_dn7), (locals.var_pparam_b4soieta0_dn8 + locals.var_deta0_lod_dn8), (locals.var_pparam_b4soieta0_dn9 + locals.var_deta0_lod_dn9), (locals.var_pparam_b4soieta0_dn10 + locals.var_deta0_lod_dn10), (locals.var_pparam_b4soieta0_dn11 + locals.var_deta0_lod_dn11), (locals.var_pparam_b4soieta0_dn12 + locals.var_deta0_lod_dn12),)
    } else {
        (locals.var_here_b4soieta0, locals.var_here_b4soieta0_dn3, locals.var_here_b4soieta0_dn4, locals.var_here_b4soieta0_dn5, locals.var_here_b4soieta0_dn6, locals.var_here_b4soieta0_dn7, locals.var_here_b4soieta0_dn8, locals.var_here_b4soieta0_dn9, locals.var_here_b4soieta0_dn10, locals.var_here_b4soieta0_dn11, locals.var_here_b4soieta0_dn12,)
    }
};
        locals.var_here_b4soieta0 = assign9710_e7074;
        locals.var_here_b4soieta0_dn3 = assign9710_e7074_d_n3;
        locals.var_here_b4soieta0_dn4 = assign9710_e7074_d_n4;
        locals.var_here_b4soieta0_dn5 = assign9710_e7074_d_n5;
        locals.var_here_b4soieta0_dn6 = assign9710_e7074_d_n6;
        locals.var_here_b4soieta0_dn7 = assign9710_e7074_d_n7;
        locals.var_here_b4soieta0_dn8 = assign9710_e7074_d_n8;
        locals.var_here_b4soieta0_dn9 = assign9710_e7074_d_n9;
        locals.var_here_b4soieta0_dn10 = assign9710_e7074_d_n10;
        locals.var_here_b4soieta0_dn11 = assign9710_e7074_d_n11;
        locals.var_here_b4soieta0_dn12 = assign9710_e7074_d_n12;

        let (assign9720_e7080, assign9720_e7080_d_n3, assign9720_e7080_d_n4, assign9720_e7080_d_n5, assign9720_e7080_d_n6, assign9720_e7080_d_n7, assign9720_e7080_d_n8, assign9720_e7080_d_n9, assign9720_e7080_d_n10, assign9720_e7080_d_n11, assign9720_e7080_d_n12,) = {
    if (locals.var_guard924 != 0.0) {
        let assign9720_e7078: f64 = (locals.var_pparam_b4soieta0cv + locals.var_deta0cv_lod);
        (assign9720_e7078, (locals.var_pparam_b4soieta0cv_dn3 + locals.var_deta0cv_lod_dn3), (locals.var_pparam_b4soieta0cv_dn4 + locals.var_deta0cv_lod_dn4), (locals.var_pparam_b4soieta0cv_dn5 + locals.var_deta0cv_lod_dn5), (locals.var_pparam_b4soieta0cv_dn6 + locals.var_deta0cv_lod_dn6), (locals.var_pparam_b4soieta0cv_dn7 + locals.var_deta0cv_lod_dn7), (locals.var_pparam_b4soieta0cv_dn8 + locals.var_deta0cv_lod_dn8), (locals.var_pparam_b4soieta0cv_dn9 + locals.var_deta0cv_lod_dn9), (locals.var_pparam_b4soieta0cv_dn10 + locals.var_deta0cv_lod_dn10), (locals.var_pparam_b4soieta0cv_dn11 + locals.var_deta0cv_lod_dn11), (locals.var_pparam_b4soieta0cv_dn12 + locals.var_deta0cv_lod_dn12),)
    } else {
        (locals.var_here_b4soieta0cv, locals.var_here_b4soieta0cv_dn3, locals.var_here_b4soieta0cv_dn4, locals.var_here_b4soieta0cv_dn5, locals.var_here_b4soieta0cv_dn6, locals.var_here_b4soieta0cv_dn7, locals.var_here_b4soieta0cv_dn8, locals.var_here_b4soieta0cv_dn9, locals.var_here_b4soieta0cv_dn10, locals.var_here_b4soieta0cv_dn11, locals.var_here_b4soieta0cv_dn12,)
    }
};
        locals.var_here_b4soieta0cv = assign9720_e7080;
        locals.var_here_b4soieta0cv_dn3 = assign9720_e7080_d_n3;
        locals.var_here_b4soieta0cv_dn4 = assign9720_e7080_d_n4;
        locals.var_here_b4soieta0cv_dn5 = assign9720_e7080_d_n5;
        locals.var_here_b4soieta0cv_dn6 = assign9720_e7080_d_n6;
        locals.var_here_b4soieta0cv_dn7 = assign9720_e7080_d_n7;
        locals.var_here_b4soieta0cv_dn8 = assign9720_e7080_d_n8;
        locals.var_here_b4soieta0cv_dn9 = assign9720_e7080_d_n9;
        locals.var_here_b4soieta0cv_dn10 = assign9720_e7080_d_n10;
        locals.var_here_b4soieta0cv_dn11 = assign9720_e7080_d_n11;
        locals.var_here_b4soieta0cv_dn12 = assign9720_e7080_d_n12;

        let (assign9730_e7085, assign9730_e7085_d_n3, assign9730_e7085_d_n4, assign9730_e7085_d_n5, assign9730_e7085_d_n6, assign9730_e7085_d_n7, assign9730_e7085_d_n8, assign9730_e7085_d_n9, assign9730_e7085_d_n10, assign9730_e7085_d_n11, assign9730_e7085_d_n12,) = {
    if (locals.var_guard924 == 0.0) {
        (locals.var_pparam_b4soiu0temp, locals.var_pparam_b4soiu0temp_dn3, locals.var_pparam_b4soiu0temp_dn4, locals.var_pparam_b4soiu0temp_dn5, locals.var_pparam_b4soiu0temp_dn6, locals.var_pparam_b4soiu0temp_dn7, locals.var_pparam_b4soiu0temp_dn8, locals.var_pparam_b4soiu0temp_dn9, locals.var_pparam_b4soiu0temp_dn10, locals.var_pparam_b4soiu0temp_dn11, locals.var_pparam_b4soiu0temp_dn12,)
    } else {
        (locals.var_here_b4soiu0temp, locals.var_here_b4soiu0temp_dn3, locals.var_here_b4soiu0temp_dn4, locals.var_here_b4soiu0temp_dn5, locals.var_here_b4soiu0temp_dn6, locals.var_here_b4soiu0temp_dn7, locals.var_here_b4soiu0temp_dn8, locals.var_here_b4soiu0temp_dn9, locals.var_here_b4soiu0temp_dn10, locals.var_here_b4soiu0temp_dn11, locals.var_here_b4soiu0temp_dn12,)
    }
};
        locals.var_here_b4soiu0temp = assign9730_e7085;
        locals.var_here_b4soiu0temp_dn3 = assign9730_e7085_d_n3;
        locals.var_here_b4soiu0temp_dn4 = assign9730_e7085_d_n4;
        locals.var_here_b4soiu0temp_dn5 = assign9730_e7085_d_n5;
        locals.var_here_b4soiu0temp_dn6 = assign9730_e7085_d_n6;
        locals.var_here_b4soiu0temp_dn7 = assign9730_e7085_d_n7;
        locals.var_here_b4soiu0temp_dn8 = assign9730_e7085_d_n8;
        locals.var_here_b4soiu0temp_dn9 = assign9730_e7085_d_n9;
        locals.var_here_b4soiu0temp_dn10 = assign9730_e7085_d_n10;
        locals.var_here_b4soiu0temp_dn11 = assign9730_e7085_d_n11;
        locals.var_here_b4soiu0temp_dn12 = assign9730_e7085_d_n12;

        let (assign9740_e7090, assign9740_e7090_d_n3, assign9740_e7090_d_n4, assign9740_e7090_d_n5, assign9740_e7090_d_n6, assign9740_e7090_d_n7, assign9740_e7090_d_n8, assign9740_e7090_d_n9, assign9740_e7090_d_n10, assign9740_e7090_d_n11, assign9740_e7090_d_n12,) = {
    if (locals.var_guard924 == 0.0) {
        (locals.var_pparam_b4soivth0, locals.var_pparam_b4soivth0_dn3, locals.var_pparam_b4soivth0_dn4, locals.var_pparam_b4soivth0_dn5, locals.var_pparam_b4soivth0_dn6, locals.var_pparam_b4soivth0_dn7, locals.var_pparam_b4soivth0_dn8, locals.var_pparam_b4soivth0_dn9, locals.var_pparam_b4soivth0_dn10, locals.var_pparam_b4soivth0_dn11, locals.var_pparam_b4soivth0_dn12,)
    } else {
        (locals.var_here_b4soivth0, locals.var_here_b4soivth0_dn3, locals.var_here_b4soivth0_dn4, locals.var_here_b4soivth0_dn5, locals.var_here_b4soivth0_dn6, locals.var_here_b4soivth0_dn7, locals.var_here_b4soivth0_dn8, locals.var_here_b4soivth0_dn9, locals.var_here_b4soivth0_dn10, locals.var_here_b4soivth0_dn11, locals.var_here_b4soivth0_dn12,)
    }
};
        locals.var_here_b4soivth0 = assign9740_e7090;
        locals.var_here_b4soivth0_dn3 = assign9740_e7090_d_n3;
        locals.var_here_b4soivth0_dn4 = assign9740_e7090_d_n4;
        locals.var_here_b4soivth0_dn5 = assign9740_e7090_d_n5;
        locals.var_here_b4soivth0_dn6 = assign9740_e7090_d_n6;
        locals.var_here_b4soivth0_dn7 = assign9740_e7090_d_n7;
        locals.var_here_b4soivth0_dn8 = assign9740_e7090_d_n8;
        locals.var_here_b4soivth0_dn9 = assign9740_e7090_d_n9;
        locals.var_here_b4soivth0_dn10 = assign9740_e7090_d_n10;
        locals.var_here_b4soivth0_dn11 = assign9740_e7090_d_n11;
        locals.var_here_b4soivth0_dn12 = assign9740_e7090_d_n12;

        let (assign9750_e7095, assign9750_e7095_d_n3, assign9750_e7095_d_n4, assign9750_e7095_d_n5, assign9750_e7095_d_n6, assign9750_e7095_d_n7, assign9750_e7095_d_n8, assign9750_e7095_d_n9, assign9750_e7095_d_n10, assign9750_e7095_d_n11, assign9750_e7095_d_n12,) = {
    if (locals.var_guard924 == 0.0) {
        (locals.var_pparam_b4soivsattemp, locals.var_pparam_b4soivsattemp_dn3, locals.var_pparam_b4soivsattemp_dn4, locals.var_pparam_b4soivsattemp_dn5, locals.var_pparam_b4soivsattemp_dn6, locals.var_pparam_b4soivsattemp_dn7, locals.var_pparam_b4soivsattemp_dn8, locals.var_pparam_b4soivsattemp_dn9, locals.var_pparam_b4soivsattemp_dn10, locals.var_pparam_b4soivsattemp_dn11, locals.var_pparam_b4soivsattemp_dn12,)
    } else {
        (locals.var_here_b4soivsattemp, locals.var_here_b4soivsattemp_dn3, locals.var_here_b4soivsattemp_dn4, locals.var_here_b4soivsattemp_dn5, locals.var_here_b4soivsattemp_dn6, locals.var_here_b4soivsattemp_dn7, locals.var_here_b4soivsattemp_dn8, locals.var_here_b4soivsattemp_dn9, locals.var_here_b4soivsattemp_dn10, locals.var_here_b4soivsattemp_dn11, locals.var_here_b4soivsattemp_dn12,)
    }
};
        locals.var_here_b4soivsattemp = assign9750_e7095;
        locals.var_here_b4soivsattemp_dn3 = assign9750_e7095_d_n3;
        locals.var_here_b4soivsattemp_dn4 = assign9750_e7095_d_n4;
        locals.var_here_b4soivsattemp_dn5 = assign9750_e7095_d_n5;
        locals.var_here_b4soivsattemp_dn6 = assign9750_e7095_d_n6;
        locals.var_here_b4soivsattemp_dn7 = assign9750_e7095_d_n7;
        locals.var_here_b4soivsattemp_dn8 = assign9750_e7095_d_n8;
        locals.var_here_b4soivsattemp_dn9 = assign9750_e7095_d_n9;
        locals.var_here_b4soivsattemp_dn10 = assign9750_e7095_d_n10;
        locals.var_here_b4soivsattemp_dn11 = assign9750_e7095_d_n11;
        locals.var_here_b4soivsattemp_dn12 = assign9750_e7095_d_n12;

        let (assign9760_e7100, assign9760_e7100_d_n3, assign9760_e7100_d_n4, assign9760_e7100_d_n5, assign9760_e7100_d_n6, assign9760_e7100_d_n7, assign9760_e7100_d_n8, assign9760_e7100_d_n9, assign9760_e7100_d_n10, assign9760_e7100_d_n11, assign9760_e7100_d_n12,) = {
    if (locals.var_guard924 == 0.0) {
        (locals.var_pparam_b4soik2, locals.var_pparam_b4soik2_dn3, locals.var_pparam_b4soik2_dn4, locals.var_pparam_b4soik2_dn5, locals.var_pparam_b4soik2_dn6, locals.var_pparam_b4soik2_dn7, locals.var_pparam_b4soik2_dn8, locals.var_pparam_b4soik2_dn9, locals.var_pparam_b4soik2_dn10, locals.var_pparam_b4soik2_dn11, locals.var_pparam_b4soik2_dn12,)
    } else {
        (locals.var_here_b4soik2, locals.var_here_b4soik2_dn3, locals.var_here_b4soik2_dn4, locals.var_here_b4soik2_dn5, locals.var_here_b4soik2_dn6, locals.var_here_b4soik2_dn7, locals.var_here_b4soik2_dn8, locals.var_here_b4soik2_dn9, locals.var_here_b4soik2_dn10, locals.var_here_b4soik2_dn11, locals.var_here_b4soik2_dn12,)
    }
};
        locals.var_here_b4soik2 = assign9760_e7100;
        locals.var_here_b4soik2_dn3 = assign9760_e7100_d_n3;
        locals.var_here_b4soik2_dn4 = assign9760_e7100_d_n4;
        locals.var_here_b4soik2_dn5 = assign9760_e7100_d_n5;
        locals.var_here_b4soik2_dn6 = assign9760_e7100_d_n6;
        locals.var_here_b4soik2_dn7 = assign9760_e7100_d_n7;
        locals.var_here_b4soik2_dn8 = assign9760_e7100_d_n8;
        locals.var_here_b4soik2_dn9 = assign9760_e7100_d_n9;
        locals.var_here_b4soik2_dn10 = assign9760_e7100_d_n10;
        locals.var_here_b4soik2_dn11 = assign9760_e7100_d_n11;
        locals.var_here_b4soik2_dn12 = assign9760_e7100_d_n12;

        let (assign9770_e7105, assign9770_e7105_d_n3, assign9770_e7105_d_n4, assign9770_e7105_d_n5, assign9770_e7105_d_n6, assign9770_e7105_d_n7, assign9770_e7105_d_n8, assign9770_e7105_d_n9, assign9770_e7105_d_n10, assign9770_e7105_d_n11, assign9770_e7105_d_n12,) = {
    if (locals.var_guard924 == 0.0) {
        (locals.var_pparam_b4soieta0, locals.var_pparam_b4soieta0_dn3, locals.var_pparam_b4soieta0_dn4, locals.var_pparam_b4soieta0_dn5, locals.var_pparam_b4soieta0_dn6, locals.var_pparam_b4soieta0_dn7, locals.var_pparam_b4soieta0_dn8, locals.var_pparam_b4soieta0_dn9, locals.var_pparam_b4soieta0_dn10, locals.var_pparam_b4soieta0_dn11, locals.var_pparam_b4soieta0_dn12,)
    } else {
        (locals.var_here_b4soieta0, locals.var_here_b4soieta0_dn3, locals.var_here_b4soieta0_dn4, locals.var_here_b4soieta0_dn5, locals.var_here_b4soieta0_dn6, locals.var_here_b4soieta0_dn7, locals.var_here_b4soieta0_dn8, locals.var_here_b4soieta0_dn9, locals.var_here_b4soieta0_dn10, locals.var_here_b4soieta0_dn11, locals.var_here_b4soieta0_dn12,)
    }
};
        locals.var_here_b4soieta0 = assign9770_e7105;
        locals.var_here_b4soieta0_dn3 = assign9770_e7105_d_n3;
        locals.var_here_b4soieta0_dn4 = assign9770_e7105_d_n4;
        locals.var_here_b4soieta0_dn5 = assign9770_e7105_d_n5;
        locals.var_here_b4soieta0_dn6 = assign9770_e7105_d_n6;
        locals.var_here_b4soieta0_dn7 = assign9770_e7105_d_n7;
        locals.var_here_b4soieta0_dn8 = assign9770_e7105_d_n8;
        locals.var_here_b4soieta0_dn9 = assign9770_e7105_d_n9;
        locals.var_here_b4soieta0_dn10 = assign9770_e7105_d_n10;
        locals.var_here_b4soieta0_dn11 = assign9770_e7105_d_n11;
        locals.var_here_b4soieta0_dn12 = assign9770_e7105_d_n12;

        let (assign9780_e7110, assign9780_e7110_d_n3, assign9780_e7110_d_n4, assign9780_e7110_d_n5, assign9780_e7110_d_n6, assign9780_e7110_d_n7, assign9780_e7110_d_n8, assign9780_e7110_d_n9, assign9780_e7110_d_n10, assign9780_e7110_d_n11, assign9780_e7110_d_n12,) = {
    if (locals.var_guard924 == 0.0) {
        (locals.var_pparam_b4soieta0cv, locals.var_pparam_b4soieta0cv_dn3, locals.var_pparam_b4soieta0cv_dn4, locals.var_pparam_b4soieta0cv_dn5, locals.var_pparam_b4soieta0cv_dn6, locals.var_pparam_b4soieta0cv_dn7, locals.var_pparam_b4soieta0cv_dn8, locals.var_pparam_b4soieta0cv_dn9, locals.var_pparam_b4soieta0cv_dn10, locals.var_pparam_b4soieta0cv_dn11, locals.var_pparam_b4soieta0cv_dn12,)
    } else {
        (locals.var_here_b4soieta0cv, locals.var_here_b4soieta0cv_dn3, locals.var_here_b4soieta0cv_dn4, locals.var_here_b4soieta0cv_dn5, locals.var_here_b4soieta0cv_dn6, locals.var_here_b4soieta0cv_dn7, locals.var_here_b4soieta0cv_dn8, locals.var_here_b4soieta0cv_dn9, locals.var_here_b4soieta0cv_dn10, locals.var_here_b4soieta0cv_dn11, locals.var_here_b4soieta0cv_dn12,)
    }
};
        locals.var_here_b4soieta0cv = assign9780_e7110;
        locals.var_here_b4soieta0cv_dn3 = assign9780_e7110_d_n3;
        locals.var_here_b4soieta0cv_dn4 = assign9780_e7110_d_n4;
        locals.var_here_b4soieta0cv_dn5 = assign9780_e7110_d_n5;
        locals.var_here_b4soieta0cv_dn6 = assign9780_e7110_d_n6;
        locals.var_here_b4soieta0cv_dn7 = assign9780_e7110_d_n7;
        locals.var_here_b4soieta0cv_dn8 = assign9780_e7110_d_n8;
        locals.var_here_b4soieta0cv_dn9 = assign9780_e7110_d_n9;
        locals.var_here_b4soieta0cv_dn10 = assign9780_e7110_d_n10;
        locals.var_here_b4soieta0cv_dn11 = assign9780_e7110_d_n11;
        locals.var_here_b4soieta0cv_dn12 = assign9780_e7110_d_n12;

        let (assign9790_e7115,) = {
    if (locals.var_guard924 == 0.0) {
        (0.0,)
    } else {
        (locals.var_b4soiinv_odeff,)
    }
};
        locals.var_b4soiinv_odeff = assign9790_e7115;

        let (assign9800_e7120,) = {
    if (locals.var_guard924 == 0.0) {
        (0.0,)
    } else {
        (locals.var_pparam_b4soiinv_od_ref,)
    }
};
        locals.var_pparam_b4soiinv_od_ref = assign9800_e7120;

        let (assign9810_e7125,) = {
    if (locals.var_guard924 == 0.0) {
        (0.0,)
    } else {
        (locals.var_b4soikvsat,)
    }
};
        locals.var_b4soikvsat = assign9810_e7125;

        let assign9820_e7128: f64 = (locals.var_here_b4soik2 * locals.var_b4soitox);
        let assign9820_e7130: f64 = (assign9820_e7128 / locals.var_b4soitoxm);
        locals.var_here_b4soik2ox = assign9820_e7130;
        locals.var_here_b4soik2ox_dn3 = ((locals.var_here_b4soik2_dn3 * locals.var_b4soitox) / locals.var_b4soitoxm);
        locals.var_here_b4soik2ox_dn4 = ((locals.var_here_b4soik2_dn4 * locals.var_b4soitox) / locals.var_b4soitoxm);
        locals.var_here_b4soik2ox_dn5 = ((locals.var_here_b4soik2_dn5 * locals.var_b4soitox) / locals.var_b4soitoxm);
        locals.var_here_b4soik2ox_dn6 = ((locals.var_here_b4soik2_dn6 * locals.var_b4soitox) / locals.var_b4soitoxm);
        locals.var_here_b4soik2ox_dn7 = ((locals.var_here_b4soik2_dn7 * locals.var_b4soitox) / locals.var_b4soitoxm);
        locals.var_here_b4soik2ox_dn8 = ((locals.var_here_b4soik2_dn8 * locals.var_b4soitox) / locals.var_b4soitoxm);
        locals.var_here_b4soik2ox_dn9 = ((locals.var_here_b4soik2_dn9 * locals.var_b4soitox) / locals.var_b4soitoxm);
        locals.var_here_b4soik2ox_dn10 = ((locals.var_here_b4soik2_dn10 * locals.var_b4soitox) / locals.var_b4soitoxm);
        locals.var_here_b4soik2ox_dn11 = ((locals.var_here_b4soik2_dn11 * locals.var_b4soitox) / locals.var_b4soitoxm);
        locals.var_here_b4soik2ox_dn12 = ((locals.var_here_b4soik2_dn12 * locals.var_b4soitox) / locals.var_b4soitoxm);

        let assign9830_e7133: f64 = (locals.var_here_b4soivth0 + locals.var_b4soidelvto);
        locals.var_here_b4soivth0 = assign9830_e7133;
        locals.var_here_b4soivth0_dn3 = locals.var_here_b4soivth0_dn3;
        locals.var_here_b4soivth0_dn4 = locals.var_here_b4soivth0_dn4;
        locals.var_here_b4soivth0_dn5 = locals.var_here_b4soivth0_dn5;
        locals.var_here_b4soivth0_dn6 = locals.var_here_b4soivth0_dn6;
        locals.var_here_b4soivth0_dn7 = locals.var_here_b4soivth0_dn7;
        locals.var_here_b4soivth0_dn8 = locals.var_here_b4soivth0_dn8;
        locals.var_here_b4soivth0_dn9 = locals.var_here_b4soivth0_dn9;
        locals.var_here_b4soivth0_dn10 = locals.var_here_b4soivth0_dn10;
        locals.var_here_b4soivth0_dn11 = locals.var_here_b4soivth0_dn11;
        locals.var_here_b4soivth0_dn12 = locals.var_here_b4soivth0_dn12;

        let assign9840_e7137: f64 = (locals.var_b4soitype * locals.var_b4soidelvto);
        let assign9840_e7138: f64 = (locals.var_pparam_b4soivfb + assign9840_e7137);
        locals.var_here_b4soivfb = assign9840_e7138;
        locals.var_here_b4soivfb_dn3 = locals.var_pparam_b4soivfb_dn3;
        locals.var_here_b4soivfb_dn4 = locals.var_pparam_b4soivfb_dn4;
        locals.var_here_b4soivfb_dn5 = locals.var_pparam_b4soivfb_dn5;
        locals.var_here_b4soivfb_dn6 = locals.var_pparam_b4soivfb_dn6;
        locals.var_here_b4soivfb_dn7 = locals.var_pparam_b4soivfb_dn7;
        locals.var_here_b4soivfb_dn8 = locals.var_pparam_b4soivfb_dn8;
        locals.var_here_b4soivfb_dn9 = locals.var_pparam_b4soivfb_dn9;
        locals.var_here_b4soivfb_dn10 = locals.var_pparam_b4soivfb_dn10;
        locals.var_here_b4soivfb_dn11 = locals.var_pparam_b4soivfb_dn11;
        locals.var_here_b4soivfb_dn12 = locals.var_pparam_b4soivfb_dn12;

        let assign9850_e7141: f64 = (locals.var_b4soicbox * locals.var_b4soisourcearea);
        locals.var_b4soicsbox = assign9850_e7141;

        let assign9860_e7144: f64 = (locals.var_b4soicsdmin * locals.var_b4soisourcearea);
        locals.var_b4soicsmin = assign9860_e7144;
        locals.var_b4soicsmin_dn3 = (locals.var_b4soicsdmin_dn3 * locals.var_b4soisourcearea);
        locals.var_b4soicsmin_dn4 = (locals.var_b4soicsdmin_dn4 * locals.var_b4soisourcearea);
        locals.var_b4soicsmin_dn5 = (locals.var_b4soicsdmin_dn5 * locals.var_b4soisourcearea);
        locals.var_b4soicsmin_dn6 = (locals.var_b4soicsdmin_dn6 * locals.var_b4soisourcearea);
        locals.var_b4soicsmin_dn7 = (locals.var_b4soicsdmin_dn7 * locals.var_b4soisourcearea);
        locals.var_b4soicsmin_dn8 = (locals.var_b4soicsdmin_dn8 * locals.var_b4soisourcearea);
        locals.var_b4soicsmin_dn9 = (locals.var_b4soicsdmin_dn9 * locals.var_b4soisourcearea);
        locals.var_b4soicsmin_dn10 = (locals.var_b4soicsdmin_dn10 * locals.var_b4soisourcearea);
        locals.var_b4soicsmin_dn11 = (locals.var_b4soicsdmin_dn11 * locals.var_b4soisourcearea);
        locals.var_b4soicsmin_dn12 = (locals.var_b4soicsdmin_dn12 * locals.var_b4soisourcearea);

        let assign9870_e7147: f64 = (locals.var_b4soicbox * locals.var_b4soidrainarea);
        locals.var_b4soicdbox = assign9870_e7147;

        let assign9880_e7150: f64 = (locals.var_b4soicsdmin * locals.var_b4soidrainarea);
        locals.var_b4soicdmin = assign9880_e7150;
        locals.var_b4soicdmin_dn3 = (locals.var_b4soicsdmin_dn3 * locals.var_b4soidrainarea);
        locals.var_b4soicdmin_dn4 = (locals.var_b4soicsdmin_dn4 * locals.var_b4soidrainarea);
        locals.var_b4soicdmin_dn5 = (locals.var_b4soicsdmin_dn5 * locals.var_b4soidrainarea);
        locals.var_b4soicdmin_dn6 = (locals.var_b4soicsdmin_dn6 * locals.var_b4soidrainarea);
        locals.var_b4soicdmin_dn7 = (locals.var_b4soicsdmin_dn7 * locals.var_b4soidrainarea);
        locals.var_b4soicdmin_dn8 = (locals.var_b4soicsdmin_dn8 * locals.var_b4soidrainarea);
        locals.var_b4soicdmin_dn9 = (locals.var_b4soicsdmin_dn9 * locals.var_b4soidrainarea);
        locals.var_b4soicdmin_dn10 = (locals.var_b4soicsdmin_dn10 * locals.var_b4soidrainarea);
        locals.var_b4soicdmin_dn11 = (locals.var_b4soicsdmin_dn11 * locals.var_b4soidrainarea);
        locals.var_b4soicdmin_dn12 = (locals.var_b4soicsdmin_dn12 * locals.var_b4soidrainarea);

        let assign9890_e7153: f64 = if locals.var_b4soicsdmin > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard929 = assign9890_e7153;

        let assign9900_e7168: f64 = if (((locals.var_pparam_b4soinsub > 0.0) && (locals.var_b4soitype > 0.0)) || ((locals.var_pparam_b4soinsub < 0.0) && (locals.var_b4soitype < 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard930 = assign9900_e7168;

        let (assign9910_e7176, assign9910_e7176_d_n3, assign9910_e7176_d_n4, assign9910_e7176_d_n5, assign9910_e7176_d_n6, assign9910_e7176_d_n7, assign9910_e7176_d_n8, assign9910_e7176_d_n9, assign9910_e7176_d_n10, assign9910_e7176_d_n11, assign9910_e7176_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 != 0.0)) {
        let assign9910_e7174: f64 = (locals.var_pparam_b4soivsdth - locals.var_pparam_b4soivsdfb);
        (assign9910_e7174, (locals.var_pparam_b4soivsdth_dn3 - locals.var_pparam_b4soivsdfb_dn3), (locals.var_pparam_b4soivsdth_dn4 - locals.var_pparam_b4soivsdfb_dn4), (locals.var_pparam_b4soivsdth_dn5 - locals.var_pparam_b4soivsdfb_dn5), (locals.var_pparam_b4soivsdth_dn6 - locals.var_pparam_b4soivsdfb_dn6), (locals.var_pparam_b4soivsdth_dn7 - locals.var_pparam_b4soivsdfb_dn7), (locals.var_pparam_b4soivsdth_dn8 - locals.var_pparam_b4soivsdfb_dn8), (locals.var_pparam_b4soivsdth_dn9 - locals.var_pparam_b4soivsdfb_dn9), (locals.var_pparam_b4soivsdth_dn10 - locals.var_pparam_b4soivsdfb_dn10), (locals.var_pparam_b4soivsdth_dn11 - locals.var_pparam_b4soivsdfb_dn11), (locals.var_pparam_b4soivsdth_dn12 - locals.var_pparam_b4soivsdfb_dn12),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign9910_e7176;
        locals.var_t0_dn3 = assign9910_e7176_d_n3;
        locals.var_t0_dn4 = assign9910_e7176_d_n4;
        locals.var_t0_dn5 = assign9910_e7176_d_n5;
        locals.var_t0_dn6 = assign9910_e7176_d_n6;
        locals.var_t0_dn7 = assign9910_e7176_d_n7;
        locals.var_t0_dn8 = assign9910_e7176_d_n8;
        locals.var_t0_dn9 = assign9910_e7176_d_n9;
        locals.var_t0_dn10 = assign9910_e7176_d_n10;
        locals.var_t0_dn11 = assign9910_e7176_d_n11;
        locals.var_t0_dn12 = assign9910_e7176_d_n12;

        let (assign9920_e7186,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 != 0.0)) {
        let assign9920_e7183: f64 = (locals.var_b4soiasd * locals.var_t0);
        let assign9920_e7184: f64 = (locals.var_pparam_b4soivsdfb + assign9920_e7183);
        (assign9920_e7184,)
    } else {
        (locals.var_pparam_b4soisdt1,)
    }
};
        locals.var_pparam_b4soisdt1 = assign9920_e7186;

        let (assign9930_e7194, assign9930_e7194_d_n3, assign9930_e7194_d_n4, assign9930_e7194_d_n5, assign9930_e7194_d_n6, assign9930_e7194_d_n7, assign9930_e7194_d_n8, assign9930_e7194_d_n9, assign9930_e7194_d_n10, assign9930_e7194_d_n11, assign9930_e7194_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 != 0.0)) {
        let assign9930_e7192: f64 = (locals.var_b4soicsbox - locals.var_b4soicsmin);
        (assign9930_e7192, (-locals.var_b4soicsmin_dn3), (-locals.var_b4soicsmin_dn4), (-locals.var_b4soicsmin_dn5), (-locals.var_b4soicsmin_dn6), (-locals.var_b4soicsmin_dn7), (-locals.var_b4soicsmin_dn8), (-locals.var_b4soicsmin_dn9), (-locals.var_b4soicsmin_dn10), (-locals.var_b4soicsmin_dn11), (-locals.var_b4soicsmin_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign9930_e7194;
        locals.var_t1_dn3 = assign9930_e7194_d_n3;
        locals.var_t1_dn4 = assign9930_e7194_d_n4;
        locals.var_t1_dn5 = assign9930_e7194_d_n5;
        locals.var_t1_dn6 = assign9930_e7194_d_n6;
        locals.var_t1_dn7 = assign9930_e7194_d_n7;
        locals.var_t1_dn8 = assign9930_e7194_d_n8;
        locals.var_t1_dn9 = assign9930_e7194_d_n9;
        locals.var_t1_dn10 = assign9930_e7194_d_n10;
        locals.var_t1_dn11 = assign9930_e7194_d_n11;
        locals.var_t1_dn12 = assign9930_e7194_d_n12;

        let (assign9940_e7204, assign9940_e7204_d_n3, assign9940_e7204_d_n4, assign9940_e7204_d_n5, assign9940_e7204_d_n6, assign9940_e7204_d_n7, assign9940_e7204_d_n8, assign9940_e7204_d_n9, assign9940_e7204_d_n10, assign9940_e7204_d_n11, assign9940_e7204_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t0;
        let assign9940_e7200: f64 = (locals.var_t1 * __rspice_inv_cse_0);
        let assign9940_e7202: f64 = (assign9940_e7200 * __rspice_inv_cse_0);
        (assign9940_e7202, ((((((locals.var_t1_dn3 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9940_e7200 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn4 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9940_e7200 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn5 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9940_e7200 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn6 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9940_e7200 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn7 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9940_e7200 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn8 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9940_e7200 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn9 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9940_e7200 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn10 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9940_e7200 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn11 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9940_e7200 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn12 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9940_e7200 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign9940_e7204;
        locals.var_t2_dn3 = assign9940_e7204_d_n3;
        locals.var_t2_dn4 = assign9940_e7204_d_n4;
        locals.var_t2_dn5 = assign9940_e7204_d_n5;
        locals.var_t2_dn6 = assign9940_e7204_d_n6;
        locals.var_t2_dn7 = assign9940_e7204_d_n7;
        locals.var_t2_dn8 = assign9940_e7204_d_n8;
        locals.var_t2_dn9 = assign9940_e7204_d_n9;
        locals.var_t2_dn10 = assign9940_e7204_d_n10;
        locals.var_t2_dn11 = assign9940_e7204_d_n11;
        locals.var_t2_dn12 = assign9940_e7204_d_n12;

        let (assign9950_e7212, assign9950_e7212_d_n3, assign9950_e7212_d_n4, assign9950_e7212_d_n5, assign9950_e7212_d_n6, assign9950_e7212_d_n7, assign9950_e7212_d_n8, assign9950_e7212_d_n9, assign9950_e7212_d_n10, assign9950_e7212_d_n11, assign9950_e7212_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 != 0.0)) {
        let assign9950_e7210: f64 = (locals.var_t2 / locals.var_b4soiasd);
        (assign9950_e7210, (locals.var_t2_dn3 / locals.var_b4soiasd), (locals.var_t2_dn4 / locals.var_b4soiasd), (locals.var_t2_dn5 / locals.var_b4soiasd), (locals.var_t2_dn6 / locals.var_b4soiasd), (locals.var_t2_dn7 / locals.var_b4soiasd), (locals.var_t2_dn8 / locals.var_b4soiasd), (locals.var_t2_dn9 / locals.var_b4soiasd), (locals.var_t2_dn10 / locals.var_b4soiasd), (locals.var_t2_dn11 / locals.var_b4soiasd), (locals.var_t2_dn12 / locals.var_b4soiasd),)
    } else {
        (locals.var_pparam_b4soist2, locals.var_pparam_b4soist2_dn3, locals.var_pparam_b4soist2_dn4, locals.var_pparam_b4soist2_dn5, locals.var_pparam_b4soist2_dn6, locals.var_pparam_b4soist2_dn7, locals.var_pparam_b4soist2_dn8, locals.var_pparam_b4soist2_dn9, locals.var_pparam_b4soist2_dn10, locals.var_pparam_b4soist2_dn11, locals.var_pparam_b4soist2_dn12,)
    }
};
        locals.var_pparam_b4soist2 = assign9950_e7212;
        locals.var_pparam_b4soist2_dn3 = assign9950_e7212_d_n3;
        locals.var_pparam_b4soist2_dn4 = assign9950_e7212_d_n4;
        locals.var_pparam_b4soist2_dn5 = assign9950_e7212_d_n5;
        locals.var_pparam_b4soist2_dn6 = assign9950_e7212_d_n6;
        locals.var_pparam_b4soist2_dn7 = assign9950_e7212_d_n7;
        locals.var_pparam_b4soist2_dn8 = assign9950_e7212_d_n8;
        locals.var_pparam_b4soist2_dn9 = assign9950_e7212_d_n9;
        locals.var_pparam_b4soist2_dn10 = assign9950_e7212_d_n10;
        locals.var_pparam_b4soist2_dn11 = assign9950_e7212_d_n11;
        locals.var_pparam_b4soist2_dn12 = assign9950_e7212_d_n12;

        let (assign9960_e7222, assign9960_e7222_d_n3, assign9960_e7222_d_n4, assign9960_e7222_d_n5, assign9960_e7222_d_n6, assign9960_e7222_d_n7, assign9960_e7222_d_n8, assign9960_e7222_d_n9, assign9960_e7222_d_n10, assign9960_e7222_d_n11, assign9960_e7222_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 != 0.0)) {
        let assign9960_e7219: f64 = (1.0 - locals.var_b4soiasd);
        let assign9960_e7220: f64 = (locals.var_t2 / assign9960_e7219);
        (assign9960_e7220, (locals.var_t2_dn3 / assign9960_e7219), (locals.var_t2_dn4 / assign9960_e7219), (locals.var_t2_dn5 / assign9960_e7219), (locals.var_t2_dn6 / assign9960_e7219), (locals.var_t2_dn7 / assign9960_e7219), (locals.var_t2_dn8 / assign9960_e7219), (locals.var_t2_dn9 / assign9960_e7219), (locals.var_t2_dn10 / assign9960_e7219), (locals.var_t2_dn11 / assign9960_e7219), (locals.var_t2_dn12 / assign9960_e7219),)
    } else {
        (locals.var_pparam_b4soist3, locals.var_pparam_b4soist3_dn3, locals.var_pparam_b4soist3_dn4, locals.var_pparam_b4soist3_dn5, locals.var_pparam_b4soist3_dn6, locals.var_pparam_b4soist3_dn7, locals.var_pparam_b4soist3_dn8, locals.var_pparam_b4soist3_dn9, locals.var_pparam_b4soist3_dn10, locals.var_pparam_b4soist3_dn11, locals.var_pparam_b4soist3_dn12,)
    }
};
        locals.var_pparam_b4soist3 = assign9960_e7222;
        locals.var_pparam_b4soist3_dn3 = assign9960_e7222_d_n3;
        locals.var_pparam_b4soist3_dn4 = assign9960_e7222_d_n4;
        locals.var_pparam_b4soist3_dn5 = assign9960_e7222_d_n5;
        locals.var_pparam_b4soist3_dn6 = assign9960_e7222_d_n6;
        locals.var_pparam_b4soist3_dn7 = assign9960_e7222_d_n7;
        locals.var_pparam_b4soist3_dn8 = assign9960_e7222_d_n8;
        locals.var_pparam_b4soist3_dn9 = assign9960_e7222_d_n9;
        locals.var_pparam_b4soist3_dn10 = assign9960_e7222_d_n10;
        locals.var_pparam_b4soist3_dn11 = assign9960_e7222_d_n11;
        locals.var_pparam_b4soist3_dn12 = assign9960_e7222_d_n12;

        let (assign9970_e7240, assign9970_e7240_d_n3, assign9970_e7240_d_n4, assign9970_e7240_d_n5, assign9970_e7240_d_n6, assign9970_e7240_d_n7, assign9970_e7240_d_n8, assign9970_e7240_d_n9, assign9970_e7240_d_n10, assign9970_e7240_d_n11, assign9970_e7240_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 != 0.0)) {
        let assign9970_e7228: f64 = (locals.var_t0 * locals.var_t1);
        let assign9970_e7231: f64 = (1.0 + locals.var_b4soiasd);
        let assign9970_e7232: f64 = (assign9970_e7228 * assign9970_e7231);
        let assign9970_e7234: f64 = (assign9970_e7232 / 3.0);
        let assign9970_e7237: f64 = (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb);
        let assign9970_e7238: f64 = (assign9970_e7234 - assign9970_e7237);
        (assign9970_e7238, (((((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)) * assign9970_e7231) / 3.0) - ((locals.var_b4soicsmin_dn3 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn3))), (((((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)) * assign9970_e7231) / 3.0) - ((locals.var_b4soicsmin_dn4 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn4))), (((((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)) * assign9970_e7231) / 3.0) - ((locals.var_b4soicsmin_dn5 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn5))), (((((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)) * assign9970_e7231) / 3.0) - ((locals.var_b4soicsmin_dn6 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn6))), (((((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)) * assign9970_e7231) / 3.0) - ((locals.var_b4soicsmin_dn7 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn7))), (((((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)) * assign9970_e7231) / 3.0) - ((locals.var_b4soicsmin_dn8 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn8))), (((((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)) * assign9970_e7231) / 3.0) - ((locals.var_b4soicsmin_dn9 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn9))), (((((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)) * assign9970_e7231) / 3.0) - ((locals.var_b4soicsmin_dn10 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn10))), (((((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)) * assign9970_e7231) / 3.0) - ((locals.var_b4soicsmin_dn11 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn11))), (((((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12)) * assign9970_e7231) / 3.0) - ((locals.var_b4soicsmin_dn12 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn12))),)
    } else {
        (locals.var_b4soist4, locals.var_b4soist4_dn3, locals.var_b4soist4_dn4, locals.var_b4soist4_dn5, locals.var_b4soist4_dn6, locals.var_b4soist4_dn7, locals.var_b4soist4_dn8, locals.var_b4soist4_dn9, locals.var_b4soist4_dn10, locals.var_b4soist4_dn11, locals.var_b4soist4_dn12,)
    }
};
        locals.var_b4soist4 = assign9970_e7240;
        locals.var_b4soist4_dn3 = assign9970_e7240_d_n3;
        locals.var_b4soist4_dn4 = assign9970_e7240_d_n4;
        locals.var_b4soist4_dn5 = assign9970_e7240_d_n5;
        locals.var_b4soist4_dn6 = assign9970_e7240_d_n6;
        locals.var_b4soist4_dn7 = assign9970_e7240_d_n7;
        locals.var_b4soist4_dn8 = assign9970_e7240_d_n8;
        locals.var_b4soist4_dn9 = assign9970_e7240_d_n9;
        locals.var_b4soist4_dn10 = assign9970_e7240_d_n10;
        locals.var_b4soist4_dn11 = assign9970_e7240_d_n11;
        locals.var_b4soist4_dn12 = assign9970_e7240_d_n12;

        let (assign9980_e7248, assign9980_e7248_d_n3, assign9980_e7248_d_n4, assign9980_e7248_d_n5, assign9980_e7248_d_n6, assign9980_e7248_d_n7, assign9980_e7248_d_n8, assign9980_e7248_d_n9, assign9980_e7248_d_n10, assign9980_e7248_d_n11, assign9980_e7248_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 != 0.0)) {
        let assign9980_e7246: f64 = (locals.var_b4soicdbox - locals.var_b4soicdmin);
        (assign9980_e7246, (-locals.var_b4soicdmin_dn3), (-locals.var_b4soicdmin_dn4), (-locals.var_b4soicdmin_dn5), (-locals.var_b4soicdmin_dn6), (-locals.var_b4soicdmin_dn7), (-locals.var_b4soicdmin_dn8), (-locals.var_b4soicdmin_dn9), (-locals.var_b4soicdmin_dn10), (-locals.var_b4soicdmin_dn11), (-locals.var_b4soicdmin_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign9980_e7248;
        locals.var_t1_dn3 = assign9980_e7248_d_n3;
        locals.var_t1_dn4 = assign9980_e7248_d_n4;
        locals.var_t1_dn5 = assign9980_e7248_d_n5;
        locals.var_t1_dn6 = assign9980_e7248_d_n6;
        locals.var_t1_dn7 = assign9980_e7248_d_n7;
        locals.var_t1_dn8 = assign9980_e7248_d_n8;
        locals.var_t1_dn9 = assign9980_e7248_d_n9;
        locals.var_t1_dn10 = assign9980_e7248_d_n10;
        locals.var_t1_dn11 = assign9980_e7248_d_n11;
        locals.var_t1_dn12 = assign9980_e7248_d_n12;

        let (assign9990_e7258, assign9990_e7258_d_n3, assign9990_e7258_d_n4, assign9990_e7258_d_n5, assign9990_e7258_d_n6, assign9990_e7258_d_n7, assign9990_e7258_d_n8, assign9990_e7258_d_n9, assign9990_e7258_d_n10, assign9990_e7258_d_n11, assign9990_e7258_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 != 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_t0;
        let assign9990_e7254: f64 = (locals.var_t1 * __rspice_inv_cse_1);
        let assign9990_e7256: f64 = (assign9990_e7254 * __rspice_inv_cse_1);
        (assign9990_e7256, ((((((locals.var_t1_dn3 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9990_e7254 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn4 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9990_e7254 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn5 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9990_e7254 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn6 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9990_e7254 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn7 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9990_e7254 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn8 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9990_e7254 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn9 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9990_e7254 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn10 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9990_e7254 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn11 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9990_e7254 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn12 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign9990_e7254 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign9990_e7258;
        locals.var_t2_dn3 = assign9990_e7258_d_n3;
        locals.var_t2_dn4 = assign9990_e7258_d_n4;
        locals.var_t2_dn5 = assign9990_e7258_d_n5;
        locals.var_t2_dn6 = assign9990_e7258_d_n6;
        locals.var_t2_dn7 = assign9990_e7258_d_n7;
        locals.var_t2_dn8 = assign9990_e7258_d_n8;
        locals.var_t2_dn9 = assign9990_e7258_d_n9;
        locals.var_t2_dn10 = assign9990_e7258_d_n10;
        locals.var_t2_dn11 = assign9990_e7258_d_n11;
        locals.var_t2_dn12 = assign9990_e7258_d_n12;

    }

    pub(super) fn stamp_transient_block_19(
        locals: &mut StampLocals,
    ) {
        let (assign10000_e7266, assign10000_e7266_d_n3, assign10000_e7266_d_n4, assign10000_e7266_d_n5, assign10000_e7266_d_n6, assign10000_e7266_d_n7, assign10000_e7266_d_n8, assign10000_e7266_d_n9, assign10000_e7266_d_n10, assign10000_e7266_d_n11, assign10000_e7266_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 != 0.0)) {
        let assign10000_e7264: f64 = (locals.var_t2 / locals.var_b4soiasd);
        (assign10000_e7264, (locals.var_t2_dn3 / locals.var_b4soiasd), (locals.var_t2_dn4 / locals.var_b4soiasd), (locals.var_t2_dn5 / locals.var_b4soiasd), (locals.var_t2_dn6 / locals.var_b4soiasd), (locals.var_t2_dn7 / locals.var_b4soiasd), (locals.var_t2_dn8 / locals.var_b4soiasd), (locals.var_t2_dn9 / locals.var_b4soiasd), (locals.var_t2_dn10 / locals.var_b4soiasd), (locals.var_t2_dn11 / locals.var_b4soiasd), (locals.var_t2_dn12 / locals.var_b4soiasd),)
    } else {
        (locals.var_pparam_b4soidt2, locals.var_pparam_b4soidt2_dn3, locals.var_pparam_b4soidt2_dn4, locals.var_pparam_b4soidt2_dn5, locals.var_pparam_b4soidt2_dn6, locals.var_pparam_b4soidt2_dn7, locals.var_pparam_b4soidt2_dn8, locals.var_pparam_b4soidt2_dn9, locals.var_pparam_b4soidt2_dn10, locals.var_pparam_b4soidt2_dn11, locals.var_pparam_b4soidt2_dn12,)
    }
};
        locals.var_pparam_b4soidt2 = assign10000_e7266;
        locals.var_pparam_b4soidt2_dn3 = assign10000_e7266_d_n3;
        locals.var_pparam_b4soidt2_dn4 = assign10000_e7266_d_n4;
        locals.var_pparam_b4soidt2_dn5 = assign10000_e7266_d_n5;
        locals.var_pparam_b4soidt2_dn6 = assign10000_e7266_d_n6;
        locals.var_pparam_b4soidt2_dn7 = assign10000_e7266_d_n7;
        locals.var_pparam_b4soidt2_dn8 = assign10000_e7266_d_n8;
        locals.var_pparam_b4soidt2_dn9 = assign10000_e7266_d_n9;
        locals.var_pparam_b4soidt2_dn10 = assign10000_e7266_d_n10;
        locals.var_pparam_b4soidt2_dn11 = assign10000_e7266_d_n11;
        locals.var_pparam_b4soidt2_dn12 = assign10000_e7266_d_n12;

        let (assign10010_e7276, assign10010_e7276_d_n3, assign10010_e7276_d_n4, assign10010_e7276_d_n5, assign10010_e7276_d_n6, assign10010_e7276_d_n7, assign10010_e7276_d_n8, assign10010_e7276_d_n9, assign10010_e7276_d_n10, assign10010_e7276_d_n11, assign10010_e7276_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 != 0.0)) {
        let assign10010_e7273: f64 = (1.0 - locals.var_b4soiasd);
        let assign10010_e7274: f64 = (locals.var_t2 / assign10010_e7273);
        (assign10010_e7274, (locals.var_t2_dn3 / assign10010_e7273), (locals.var_t2_dn4 / assign10010_e7273), (locals.var_t2_dn5 / assign10010_e7273), (locals.var_t2_dn6 / assign10010_e7273), (locals.var_t2_dn7 / assign10010_e7273), (locals.var_t2_dn8 / assign10010_e7273), (locals.var_t2_dn9 / assign10010_e7273), (locals.var_t2_dn10 / assign10010_e7273), (locals.var_t2_dn11 / assign10010_e7273), (locals.var_t2_dn12 / assign10010_e7273),)
    } else {
        (locals.var_pparam_b4soidt3, locals.var_pparam_b4soidt3_dn3, locals.var_pparam_b4soidt3_dn4, locals.var_pparam_b4soidt3_dn5, locals.var_pparam_b4soidt3_dn6, locals.var_pparam_b4soidt3_dn7, locals.var_pparam_b4soidt3_dn8, locals.var_pparam_b4soidt3_dn9, locals.var_pparam_b4soidt3_dn10, locals.var_pparam_b4soidt3_dn11, locals.var_pparam_b4soidt3_dn12,)
    }
};
        locals.var_pparam_b4soidt3 = assign10010_e7276;
        locals.var_pparam_b4soidt3_dn3 = assign10010_e7276_d_n3;
        locals.var_pparam_b4soidt3_dn4 = assign10010_e7276_d_n4;
        locals.var_pparam_b4soidt3_dn5 = assign10010_e7276_d_n5;
        locals.var_pparam_b4soidt3_dn6 = assign10010_e7276_d_n6;
        locals.var_pparam_b4soidt3_dn7 = assign10010_e7276_d_n7;
        locals.var_pparam_b4soidt3_dn8 = assign10010_e7276_d_n8;
        locals.var_pparam_b4soidt3_dn9 = assign10010_e7276_d_n9;
        locals.var_pparam_b4soidt3_dn10 = assign10010_e7276_d_n10;
        locals.var_pparam_b4soidt3_dn11 = assign10010_e7276_d_n11;
        locals.var_pparam_b4soidt3_dn12 = assign10010_e7276_d_n12;

        let (assign10020_e7294, assign10020_e7294_d_n3, assign10020_e7294_d_n4, assign10020_e7294_d_n5, assign10020_e7294_d_n6, assign10020_e7294_d_n7, assign10020_e7294_d_n8, assign10020_e7294_d_n9, assign10020_e7294_d_n10, assign10020_e7294_d_n11, assign10020_e7294_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 != 0.0)) {
        let assign10020_e7282: f64 = (locals.var_t0 * locals.var_t1);
        let assign10020_e7285: f64 = (1.0 + locals.var_b4soiasd);
        let assign10020_e7286: f64 = (assign10020_e7282 * assign10020_e7285);
        let assign10020_e7288: f64 = (assign10020_e7286 / 3.0);
        let assign10020_e7291: f64 = (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb);
        let assign10020_e7292: f64 = (assign10020_e7288 - assign10020_e7291);
        (assign10020_e7292, (((((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)) * assign10020_e7285) / 3.0) - ((locals.var_b4soicdmin_dn3 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn3))), (((((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)) * assign10020_e7285) / 3.0) - ((locals.var_b4soicdmin_dn4 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn4))), (((((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)) * assign10020_e7285) / 3.0) - ((locals.var_b4soicdmin_dn5 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn5))), (((((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)) * assign10020_e7285) / 3.0) - ((locals.var_b4soicdmin_dn6 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn6))), (((((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)) * assign10020_e7285) / 3.0) - ((locals.var_b4soicdmin_dn7 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn7))), (((((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)) * assign10020_e7285) / 3.0) - ((locals.var_b4soicdmin_dn8 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn8))), (((((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)) * assign10020_e7285) / 3.0) - ((locals.var_b4soicdmin_dn9 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn9))), (((((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)) * assign10020_e7285) / 3.0) - ((locals.var_b4soicdmin_dn10 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn10))), (((((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)) * assign10020_e7285) / 3.0) - ((locals.var_b4soicdmin_dn11 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn11))), (((((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12)) * assign10020_e7285) / 3.0) - ((locals.var_b4soicdmin_dn12 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn12))),)
    } else {
        (locals.var_b4soidt4, locals.var_b4soidt4_dn3, locals.var_b4soidt4_dn4, locals.var_b4soidt4_dn5, locals.var_b4soidt4_dn6, locals.var_b4soidt4_dn7, locals.var_b4soidt4_dn8, locals.var_b4soidt4_dn9, locals.var_b4soidt4_dn10, locals.var_b4soidt4_dn11, locals.var_b4soidt4_dn12,)
    }
};
        locals.var_b4soidt4 = assign10020_e7294;
        locals.var_b4soidt4_dn3 = assign10020_e7294_d_n3;
        locals.var_b4soidt4_dn4 = assign10020_e7294_d_n4;
        locals.var_b4soidt4_dn5 = assign10020_e7294_d_n5;
        locals.var_b4soidt4_dn6 = assign10020_e7294_d_n6;
        locals.var_b4soidt4_dn7 = assign10020_e7294_d_n7;
        locals.var_b4soidt4_dn8 = assign10020_e7294_d_n8;
        locals.var_b4soidt4_dn9 = assign10020_e7294_d_n9;
        locals.var_b4soidt4_dn10 = assign10020_e7294_d_n10;
        locals.var_b4soidt4_dn11 = assign10020_e7294_d_n11;
        locals.var_b4soidt4_dn12 = assign10020_e7294_d_n12;

        let (assign10030_e7303, assign10030_e7303_d_n3, assign10030_e7303_d_n4, assign10030_e7303_d_n5, assign10030_e7303_d_n6, assign10030_e7303_d_n7, assign10030_e7303_d_n8, assign10030_e7303_d_n9, assign10030_e7303_d_n10, assign10030_e7303_d_n11, assign10030_e7303_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 == 0.0)) {
        let assign10030_e7301: f64 = (locals.var_pparam_b4soivsdfb - locals.var_pparam_b4soivsdth);
        (assign10030_e7301, (locals.var_pparam_b4soivsdfb_dn3 - locals.var_pparam_b4soivsdth_dn3), (locals.var_pparam_b4soivsdfb_dn4 - locals.var_pparam_b4soivsdth_dn4), (locals.var_pparam_b4soivsdfb_dn5 - locals.var_pparam_b4soivsdth_dn5), (locals.var_pparam_b4soivsdfb_dn6 - locals.var_pparam_b4soivsdth_dn6), (locals.var_pparam_b4soivsdfb_dn7 - locals.var_pparam_b4soivsdth_dn7), (locals.var_pparam_b4soivsdfb_dn8 - locals.var_pparam_b4soivsdth_dn8), (locals.var_pparam_b4soivsdfb_dn9 - locals.var_pparam_b4soivsdth_dn9), (locals.var_pparam_b4soivsdfb_dn10 - locals.var_pparam_b4soivsdth_dn10), (locals.var_pparam_b4soivsdfb_dn11 - locals.var_pparam_b4soivsdth_dn11), (locals.var_pparam_b4soivsdfb_dn12 - locals.var_pparam_b4soivsdth_dn12),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign10030_e7303;
        locals.var_t0_dn3 = assign10030_e7303_d_n3;
        locals.var_t0_dn4 = assign10030_e7303_d_n4;
        locals.var_t0_dn5 = assign10030_e7303_d_n5;
        locals.var_t0_dn6 = assign10030_e7303_d_n6;
        locals.var_t0_dn7 = assign10030_e7303_d_n7;
        locals.var_t0_dn8 = assign10030_e7303_d_n8;
        locals.var_t0_dn9 = assign10030_e7303_d_n9;
        locals.var_t0_dn10 = assign10030_e7303_d_n10;
        locals.var_t0_dn11 = assign10030_e7303_d_n11;
        locals.var_t0_dn12 = assign10030_e7303_d_n12;

        let (assign10040_e7314,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 == 0.0)) {
        let assign10040_e7311: f64 = (locals.var_b4soiasd * locals.var_t0);
        let assign10040_e7312: f64 = (locals.var_pparam_b4soivsdth + assign10040_e7311);
        (assign10040_e7312,)
    } else {
        (locals.var_pparam_b4soisdt1,)
    }
};
        locals.var_pparam_b4soisdt1 = assign10040_e7314;

        let (assign10050_e7323, assign10050_e7323_d_n3, assign10050_e7323_d_n4, assign10050_e7323_d_n5, assign10050_e7323_d_n6, assign10050_e7323_d_n7, assign10050_e7323_d_n8, assign10050_e7323_d_n9, assign10050_e7323_d_n10, assign10050_e7323_d_n11, assign10050_e7323_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 == 0.0)) {
        let assign10050_e7321: f64 = (locals.var_b4soicsmin - locals.var_b4soicsbox);
        (assign10050_e7321, locals.var_b4soicsmin_dn3, locals.var_b4soicsmin_dn4, locals.var_b4soicsmin_dn5, locals.var_b4soicsmin_dn6, locals.var_b4soicsmin_dn7, locals.var_b4soicsmin_dn8, locals.var_b4soicsmin_dn9, locals.var_b4soicsmin_dn10, locals.var_b4soicsmin_dn11, locals.var_b4soicsmin_dn12,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign10050_e7323;
        locals.var_t1_dn3 = assign10050_e7323_d_n3;
        locals.var_t1_dn4 = assign10050_e7323_d_n4;
        locals.var_t1_dn5 = assign10050_e7323_d_n5;
        locals.var_t1_dn6 = assign10050_e7323_d_n6;
        locals.var_t1_dn7 = assign10050_e7323_d_n7;
        locals.var_t1_dn8 = assign10050_e7323_d_n8;
        locals.var_t1_dn9 = assign10050_e7323_d_n9;
        locals.var_t1_dn10 = assign10050_e7323_d_n10;
        locals.var_t1_dn11 = assign10050_e7323_d_n11;
        locals.var_t1_dn12 = assign10050_e7323_d_n12;

        let (assign10060_e7334, assign10060_e7334_d_n3, assign10060_e7334_d_n4, assign10060_e7334_d_n5, assign10060_e7334_d_n6, assign10060_e7334_d_n7, assign10060_e7334_d_n8, assign10060_e7334_d_n9, assign10060_e7334_d_n10, assign10060_e7334_d_n11, assign10060_e7334_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t0;
        let assign10060_e7330: f64 = (locals.var_t1 * __rspice_inv_cse_0);
        let assign10060_e7332: f64 = (assign10060_e7330 * __rspice_inv_cse_0);
        (assign10060_e7332, ((((((locals.var_t1_dn3 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10060_e7330 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn4 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10060_e7330 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn5 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10060_e7330 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn6 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10060_e7330 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn7 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10060_e7330 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn8 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10060_e7330 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn9 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10060_e7330 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn10 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10060_e7330 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn11 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10060_e7330 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn12 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10060_e7330 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign10060_e7334;
        locals.var_t2_dn3 = assign10060_e7334_d_n3;
        locals.var_t2_dn4 = assign10060_e7334_d_n4;
        locals.var_t2_dn5 = assign10060_e7334_d_n5;
        locals.var_t2_dn6 = assign10060_e7334_d_n6;
        locals.var_t2_dn7 = assign10060_e7334_d_n7;
        locals.var_t2_dn8 = assign10060_e7334_d_n8;
        locals.var_t2_dn9 = assign10060_e7334_d_n9;
        locals.var_t2_dn10 = assign10060_e7334_d_n10;
        locals.var_t2_dn11 = assign10060_e7334_d_n11;
        locals.var_t2_dn12 = assign10060_e7334_d_n12;

        let (assign10070_e7343, assign10070_e7343_d_n3, assign10070_e7343_d_n4, assign10070_e7343_d_n5, assign10070_e7343_d_n6, assign10070_e7343_d_n7, assign10070_e7343_d_n8, assign10070_e7343_d_n9, assign10070_e7343_d_n10, assign10070_e7343_d_n11, assign10070_e7343_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 == 0.0)) {
        let assign10070_e7341: f64 = (locals.var_t2 / locals.var_b4soiasd);
        (assign10070_e7341, (locals.var_t2_dn3 / locals.var_b4soiasd), (locals.var_t2_dn4 / locals.var_b4soiasd), (locals.var_t2_dn5 / locals.var_b4soiasd), (locals.var_t2_dn6 / locals.var_b4soiasd), (locals.var_t2_dn7 / locals.var_b4soiasd), (locals.var_t2_dn8 / locals.var_b4soiasd), (locals.var_t2_dn9 / locals.var_b4soiasd), (locals.var_t2_dn10 / locals.var_b4soiasd), (locals.var_t2_dn11 / locals.var_b4soiasd), (locals.var_t2_dn12 / locals.var_b4soiasd),)
    } else {
        (locals.var_pparam_b4soist2, locals.var_pparam_b4soist2_dn3, locals.var_pparam_b4soist2_dn4, locals.var_pparam_b4soist2_dn5, locals.var_pparam_b4soist2_dn6, locals.var_pparam_b4soist2_dn7, locals.var_pparam_b4soist2_dn8, locals.var_pparam_b4soist2_dn9, locals.var_pparam_b4soist2_dn10, locals.var_pparam_b4soist2_dn11, locals.var_pparam_b4soist2_dn12,)
    }
};
        locals.var_pparam_b4soist2 = assign10070_e7343;
        locals.var_pparam_b4soist2_dn3 = assign10070_e7343_d_n3;
        locals.var_pparam_b4soist2_dn4 = assign10070_e7343_d_n4;
        locals.var_pparam_b4soist2_dn5 = assign10070_e7343_d_n5;
        locals.var_pparam_b4soist2_dn6 = assign10070_e7343_d_n6;
        locals.var_pparam_b4soist2_dn7 = assign10070_e7343_d_n7;
        locals.var_pparam_b4soist2_dn8 = assign10070_e7343_d_n8;
        locals.var_pparam_b4soist2_dn9 = assign10070_e7343_d_n9;
        locals.var_pparam_b4soist2_dn10 = assign10070_e7343_d_n10;
        locals.var_pparam_b4soist2_dn11 = assign10070_e7343_d_n11;
        locals.var_pparam_b4soist2_dn12 = assign10070_e7343_d_n12;

        let (assign10080_e7354, assign10080_e7354_d_n3, assign10080_e7354_d_n4, assign10080_e7354_d_n5, assign10080_e7354_d_n6, assign10080_e7354_d_n7, assign10080_e7354_d_n8, assign10080_e7354_d_n9, assign10080_e7354_d_n10, assign10080_e7354_d_n11, assign10080_e7354_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 == 0.0)) {
        let assign10080_e7351: f64 = (1.0 - locals.var_b4soiasd);
        let assign10080_e7352: f64 = (locals.var_t2 / assign10080_e7351);
        (assign10080_e7352, (locals.var_t2_dn3 / assign10080_e7351), (locals.var_t2_dn4 / assign10080_e7351), (locals.var_t2_dn5 / assign10080_e7351), (locals.var_t2_dn6 / assign10080_e7351), (locals.var_t2_dn7 / assign10080_e7351), (locals.var_t2_dn8 / assign10080_e7351), (locals.var_t2_dn9 / assign10080_e7351), (locals.var_t2_dn10 / assign10080_e7351), (locals.var_t2_dn11 / assign10080_e7351), (locals.var_t2_dn12 / assign10080_e7351),)
    } else {
        (locals.var_pparam_b4soist3, locals.var_pparam_b4soist3_dn3, locals.var_pparam_b4soist3_dn4, locals.var_pparam_b4soist3_dn5, locals.var_pparam_b4soist3_dn6, locals.var_pparam_b4soist3_dn7, locals.var_pparam_b4soist3_dn8, locals.var_pparam_b4soist3_dn9, locals.var_pparam_b4soist3_dn10, locals.var_pparam_b4soist3_dn11, locals.var_pparam_b4soist3_dn12,)
    }
};
        locals.var_pparam_b4soist3 = assign10080_e7354;
        locals.var_pparam_b4soist3_dn3 = assign10080_e7354_d_n3;
        locals.var_pparam_b4soist3_dn4 = assign10080_e7354_d_n4;
        locals.var_pparam_b4soist3_dn5 = assign10080_e7354_d_n5;
        locals.var_pparam_b4soist3_dn6 = assign10080_e7354_d_n6;
        locals.var_pparam_b4soist3_dn7 = assign10080_e7354_d_n7;
        locals.var_pparam_b4soist3_dn8 = assign10080_e7354_d_n8;
        locals.var_pparam_b4soist3_dn9 = assign10080_e7354_d_n9;
        locals.var_pparam_b4soist3_dn10 = assign10080_e7354_d_n10;
        locals.var_pparam_b4soist3_dn11 = assign10080_e7354_d_n11;
        locals.var_pparam_b4soist3_dn12 = assign10080_e7354_d_n12;

        let (assign10090_e7373, assign10090_e7373_d_n3, assign10090_e7373_d_n4, assign10090_e7373_d_n5, assign10090_e7373_d_n6, assign10090_e7373_d_n7, assign10090_e7373_d_n8, assign10090_e7373_d_n9, assign10090_e7373_d_n10, assign10090_e7373_d_n11, assign10090_e7373_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 == 0.0)) {
        let assign10090_e7361: f64 = (locals.var_t0 * locals.var_t1);
        let assign10090_e7364: f64 = (1.0 + locals.var_b4soiasd);
        let assign10090_e7365: f64 = (assign10090_e7361 * assign10090_e7364);
        let assign10090_e7367: f64 = (assign10090_e7365 / 3.0);
        let assign10090_e7370: f64 = (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth);
        let assign10090_e7371: f64 = (assign10090_e7367 - assign10090_e7370);
        (assign10090_e7371, (((((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)) * assign10090_e7364) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn3)), (((((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)) * assign10090_e7364) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn4)), (((((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)) * assign10090_e7364) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn5)), (((((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)) * assign10090_e7364) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn6)), (((((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)) * assign10090_e7364) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn7)), (((((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)) * assign10090_e7364) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn8)), (((((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)) * assign10090_e7364) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn9)), (((((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)) * assign10090_e7364) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn10)), (((((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)) * assign10090_e7364) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn11)), (((((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12)) * assign10090_e7364) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn12)),)
    } else {
        (locals.var_b4soist4, locals.var_b4soist4_dn3, locals.var_b4soist4_dn4, locals.var_b4soist4_dn5, locals.var_b4soist4_dn6, locals.var_b4soist4_dn7, locals.var_b4soist4_dn8, locals.var_b4soist4_dn9, locals.var_b4soist4_dn10, locals.var_b4soist4_dn11, locals.var_b4soist4_dn12,)
    }
};
        locals.var_b4soist4 = assign10090_e7373;
        locals.var_b4soist4_dn3 = assign10090_e7373_d_n3;
        locals.var_b4soist4_dn4 = assign10090_e7373_d_n4;
        locals.var_b4soist4_dn5 = assign10090_e7373_d_n5;
        locals.var_b4soist4_dn6 = assign10090_e7373_d_n6;
        locals.var_b4soist4_dn7 = assign10090_e7373_d_n7;
        locals.var_b4soist4_dn8 = assign10090_e7373_d_n8;
        locals.var_b4soist4_dn9 = assign10090_e7373_d_n9;
        locals.var_b4soist4_dn10 = assign10090_e7373_d_n10;
        locals.var_b4soist4_dn11 = assign10090_e7373_d_n11;
        locals.var_b4soist4_dn12 = assign10090_e7373_d_n12;

        let (assign10100_e7382, assign10100_e7382_d_n3, assign10100_e7382_d_n4, assign10100_e7382_d_n5, assign10100_e7382_d_n6, assign10100_e7382_d_n7, assign10100_e7382_d_n8, assign10100_e7382_d_n9, assign10100_e7382_d_n10, assign10100_e7382_d_n11, assign10100_e7382_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 == 0.0)) {
        let assign10100_e7380: f64 = (locals.var_b4soicdmin - locals.var_b4soicdbox);
        (assign10100_e7380, locals.var_b4soicdmin_dn3, locals.var_b4soicdmin_dn4, locals.var_b4soicdmin_dn5, locals.var_b4soicdmin_dn6, locals.var_b4soicdmin_dn7, locals.var_b4soicdmin_dn8, locals.var_b4soicdmin_dn9, locals.var_b4soicdmin_dn10, locals.var_b4soicdmin_dn11, locals.var_b4soicdmin_dn12,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign10100_e7382;
        locals.var_t1_dn3 = assign10100_e7382_d_n3;
        locals.var_t1_dn4 = assign10100_e7382_d_n4;
        locals.var_t1_dn5 = assign10100_e7382_d_n5;
        locals.var_t1_dn6 = assign10100_e7382_d_n6;
        locals.var_t1_dn7 = assign10100_e7382_d_n7;
        locals.var_t1_dn8 = assign10100_e7382_d_n8;
        locals.var_t1_dn9 = assign10100_e7382_d_n9;
        locals.var_t1_dn10 = assign10100_e7382_d_n10;
        locals.var_t1_dn11 = assign10100_e7382_d_n11;
        locals.var_t1_dn12 = assign10100_e7382_d_n12;

        let (assign10110_e7393, assign10110_e7393_d_n3, assign10110_e7393_d_n4, assign10110_e7393_d_n5, assign10110_e7393_d_n6, assign10110_e7393_d_n7, assign10110_e7393_d_n8, assign10110_e7393_d_n9, assign10110_e7393_d_n10, assign10110_e7393_d_n11, assign10110_e7393_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 == 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_t0;
        let assign10110_e7389: f64 = (locals.var_t1 * __rspice_inv_cse_1);
        let assign10110_e7391: f64 = (assign10110_e7389 * __rspice_inv_cse_1);
        (assign10110_e7391, ((((((locals.var_t1_dn3 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10110_e7389 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn4 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10110_e7389 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn5 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10110_e7389 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn6 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10110_e7389 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn7 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10110_e7389 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn8 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10110_e7389 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn9 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10110_e7389 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn10 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10110_e7389 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn11 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10110_e7389 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn12 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign10110_e7389 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign10110_e7393;
        locals.var_t2_dn3 = assign10110_e7393_d_n3;
        locals.var_t2_dn4 = assign10110_e7393_d_n4;
        locals.var_t2_dn5 = assign10110_e7393_d_n5;
        locals.var_t2_dn6 = assign10110_e7393_d_n6;
        locals.var_t2_dn7 = assign10110_e7393_d_n7;
        locals.var_t2_dn8 = assign10110_e7393_d_n8;
        locals.var_t2_dn9 = assign10110_e7393_d_n9;
        locals.var_t2_dn10 = assign10110_e7393_d_n10;
        locals.var_t2_dn11 = assign10110_e7393_d_n11;
        locals.var_t2_dn12 = assign10110_e7393_d_n12;

        let (assign10120_e7402, assign10120_e7402_d_n3, assign10120_e7402_d_n4, assign10120_e7402_d_n5, assign10120_e7402_d_n6, assign10120_e7402_d_n7, assign10120_e7402_d_n8, assign10120_e7402_d_n9, assign10120_e7402_d_n10, assign10120_e7402_d_n11, assign10120_e7402_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 == 0.0)) {
        let assign10120_e7400: f64 = (locals.var_t2 / locals.var_b4soiasd);
        (assign10120_e7400, (locals.var_t2_dn3 / locals.var_b4soiasd), (locals.var_t2_dn4 / locals.var_b4soiasd), (locals.var_t2_dn5 / locals.var_b4soiasd), (locals.var_t2_dn6 / locals.var_b4soiasd), (locals.var_t2_dn7 / locals.var_b4soiasd), (locals.var_t2_dn8 / locals.var_b4soiasd), (locals.var_t2_dn9 / locals.var_b4soiasd), (locals.var_t2_dn10 / locals.var_b4soiasd), (locals.var_t2_dn11 / locals.var_b4soiasd), (locals.var_t2_dn12 / locals.var_b4soiasd),)
    } else {
        (locals.var_pparam_b4soidt2, locals.var_pparam_b4soidt2_dn3, locals.var_pparam_b4soidt2_dn4, locals.var_pparam_b4soidt2_dn5, locals.var_pparam_b4soidt2_dn6, locals.var_pparam_b4soidt2_dn7, locals.var_pparam_b4soidt2_dn8, locals.var_pparam_b4soidt2_dn9, locals.var_pparam_b4soidt2_dn10, locals.var_pparam_b4soidt2_dn11, locals.var_pparam_b4soidt2_dn12,)
    }
};
        locals.var_pparam_b4soidt2 = assign10120_e7402;
        locals.var_pparam_b4soidt2_dn3 = assign10120_e7402_d_n3;
        locals.var_pparam_b4soidt2_dn4 = assign10120_e7402_d_n4;
        locals.var_pparam_b4soidt2_dn5 = assign10120_e7402_d_n5;
        locals.var_pparam_b4soidt2_dn6 = assign10120_e7402_d_n6;
        locals.var_pparam_b4soidt2_dn7 = assign10120_e7402_d_n7;
        locals.var_pparam_b4soidt2_dn8 = assign10120_e7402_d_n8;
        locals.var_pparam_b4soidt2_dn9 = assign10120_e7402_d_n9;
        locals.var_pparam_b4soidt2_dn10 = assign10120_e7402_d_n10;
        locals.var_pparam_b4soidt2_dn11 = assign10120_e7402_d_n11;
        locals.var_pparam_b4soidt2_dn12 = assign10120_e7402_d_n12;

        let (assign10130_e7413, assign10130_e7413_d_n3, assign10130_e7413_d_n4, assign10130_e7413_d_n5, assign10130_e7413_d_n6, assign10130_e7413_d_n7, assign10130_e7413_d_n8, assign10130_e7413_d_n9, assign10130_e7413_d_n10, assign10130_e7413_d_n11, assign10130_e7413_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 == 0.0)) {
        let assign10130_e7410: f64 = (1.0 - locals.var_b4soiasd);
        let assign10130_e7411: f64 = (locals.var_t2 / assign10130_e7410);
        (assign10130_e7411, (locals.var_t2_dn3 / assign10130_e7410), (locals.var_t2_dn4 / assign10130_e7410), (locals.var_t2_dn5 / assign10130_e7410), (locals.var_t2_dn6 / assign10130_e7410), (locals.var_t2_dn7 / assign10130_e7410), (locals.var_t2_dn8 / assign10130_e7410), (locals.var_t2_dn9 / assign10130_e7410), (locals.var_t2_dn10 / assign10130_e7410), (locals.var_t2_dn11 / assign10130_e7410), (locals.var_t2_dn12 / assign10130_e7410),)
    } else {
        (locals.var_pparam_b4soidt3, locals.var_pparam_b4soidt3_dn3, locals.var_pparam_b4soidt3_dn4, locals.var_pparam_b4soidt3_dn5, locals.var_pparam_b4soidt3_dn6, locals.var_pparam_b4soidt3_dn7, locals.var_pparam_b4soidt3_dn8, locals.var_pparam_b4soidt3_dn9, locals.var_pparam_b4soidt3_dn10, locals.var_pparam_b4soidt3_dn11, locals.var_pparam_b4soidt3_dn12,)
    }
};
        locals.var_pparam_b4soidt3 = assign10130_e7413;
        locals.var_pparam_b4soidt3_dn3 = assign10130_e7413_d_n3;
        locals.var_pparam_b4soidt3_dn4 = assign10130_e7413_d_n4;
        locals.var_pparam_b4soidt3_dn5 = assign10130_e7413_d_n5;
        locals.var_pparam_b4soidt3_dn6 = assign10130_e7413_d_n6;
        locals.var_pparam_b4soidt3_dn7 = assign10130_e7413_d_n7;
        locals.var_pparam_b4soidt3_dn8 = assign10130_e7413_d_n8;
        locals.var_pparam_b4soidt3_dn9 = assign10130_e7413_d_n9;
        locals.var_pparam_b4soidt3_dn10 = assign10130_e7413_d_n10;
        locals.var_pparam_b4soidt3_dn11 = assign10130_e7413_d_n11;
        locals.var_pparam_b4soidt3_dn12 = assign10130_e7413_d_n12;

        let (assign10140_e7432, assign10140_e7432_d_n3, assign10140_e7432_d_n4, assign10140_e7432_d_n5, assign10140_e7432_d_n6, assign10140_e7432_d_n7, assign10140_e7432_d_n8, assign10140_e7432_d_n9, assign10140_e7432_d_n10, assign10140_e7432_d_n11, assign10140_e7432_d_n12,) = {
    if ((locals.var_guard929 != 0.0) && (locals.var_guard930 == 0.0)) {
        let assign10140_e7420: f64 = (locals.var_t0 * locals.var_t1);
        let assign10140_e7423: f64 = (1.0 + locals.var_b4soiasd);
        let assign10140_e7424: f64 = (assign10140_e7420 * assign10140_e7423);
        let assign10140_e7426: f64 = (assign10140_e7424 / 3.0);
        let assign10140_e7429: f64 = (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth);
        let assign10140_e7430: f64 = (assign10140_e7426 - assign10140_e7429);
        (assign10140_e7430, (((((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)) * assign10140_e7423) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn3)), (((((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)) * assign10140_e7423) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn4)), (((((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)) * assign10140_e7423) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn5)), (((((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)) * assign10140_e7423) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn6)), (((((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)) * assign10140_e7423) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn7)), (((((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)) * assign10140_e7423) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn8)), (((((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)) * assign10140_e7423) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn9)), (((((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)) * assign10140_e7423) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn10)), (((((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)) * assign10140_e7423) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn11)), (((((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12)) * assign10140_e7423) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn12)),)
    } else {
        (locals.var_b4soidt4, locals.var_b4soidt4_dn3, locals.var_b4soidt4_dn4, locals.var_b4soidt4_dn5, locals.var_b4soidt4_dn6, locals.var_b4soidt4_dn7, locals.var_b4soidt4_dn8, locals.var_b4soidt4_dn9, locals.var_b4soidt4_dn10, locals.var_b4soidt4_dn11, locals.var_b4soidt4_dn12,)
    }
};
        locals.var_b4soidt4 = assign10140_e7432;
        locals.var_b4soidt4_dn3 = assign10140_e7432_d_n3;
        locals.var_b4soidt4_dn4 = assign10140_e7432_d_n4;
        locals.var_b4soidt4_dn5 = assign10140_e7432_d_n5;
        locals.var_b4soidt4_dn6 = assign10140_e7432_d_n6;
        locals.var_b4soidt4_dn7 = assign10140_e7432_d_n7;
        locals.var_b4soidt4_dn8 = assign10140_e7432_d_n8;
        locals.var_b4soidt4_dn9 = assign10140_e7432_d_n9;
        locals.var_b4soidt4_dn10 = assign10140_e7432_d_n10;
        locals.var_b4soidt4_dn11 = assign10140_e7432_d_n11;
        locals.var_b4soidt4_dn12 = assign10140_e7432_d_n12;

        let (assign10150_e7437,) = {
    if (locals.var_guard929 == 0.0) {
        (0.0,)
    } else {
        (locals.var_pparam_b4soisdt1,)
    }
};
        locals.var_pparam_b4soisdt1 = assign10150_e7437;

        let (assign10160_e7442, assign10160_e7442_d_n3, assign10160_e7442_d_n4, assign10160_e7442_d_n5, assign10160_e7442_d_n6, assign10160_e7442_d_n7, assign10160_e7442_d_n8, assign10160_e7442_d_n9, assign10160_e7442_d_n10, assign10160_e7442_d_n11, assign10160_e7442_d_n12,) = {
    if (locals.var_guard929 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soist2, locals.var_pparam_b4soist2_dn3, locals.var_pparam_b4soist2_dn4, locals.var_pparam_b4soist2_dn5, locals.var_pparam_b4soist2_dn6, locals.var_pparam_b4soist2_dn7, locals.var_pparam_b4soist2_dn8, locals.var_pparam_b4soist2_dn9, locals.var_pparam_b4soist2_dn10, locals.var_pparam_b4soist2_dn11, locals.var_pparam_b4soist2_dn12,)
    }
};
        locals.var_pparam_b4soist2 = assign10160_e7442;
        locals.var_pparam_b4soist2_dn3 = assign10160_e7442_d_n3;
        locals.var_pparam_b4soist2_dn4 = assign10160_e7442_d_n4;
        locals.var_pparam_b4soist2_dn5 = assign10160_e7442_d_n5;
        locals.var_pparam_b4soist2_dn6 = assign10160_e7442_d_n6;
        locals.var_pparam_b4soist2_dn7 = assign10160_e7442_d_n7;
        locals.var_pparam_b4soist2_dn8 = assign10160_e7442_d_n8;
        locals.var_pparam_b4soist2_dn9 = assign10160_e7442_d_n9;
        locals.var_pparam_b4soist2_dn10 = assign10160_e7442_d_n10;
        locals.var_pparam_b4soist2_dn11 = assign10160_e7442_d_n11;
        locals.var_pparam_b4soist2_dn12 = assign10160_e7442_d_n12;

        let (assign10170_e7447, assign10170_e7447_d_n3, assign10170_e7447_d_n4, assign10170_e7447_d_n5, assign10170_e7447_d_n6, assign10170_e7447_d_n7, assign10170_e7447_d_n8, assign10170_e7447_d_n9, assign10170_e7447_d_n10, assign10170_e7447_d_n11, assign10170_e7447_d_n12,) = {
    if (locals.var_guard929 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soist3, locals.var_pparam_b4soist3_dn3, locals.var_pparam_b4soist3_dn4, locals.var_pparam_b4soist3_dn5, locals.var_pparam_b4soist3_dn6, locals.var_pparam_b4soist3_dn7, locals.var_pparam_b4soist3_dn8, locals.var_pparam_b4soist3_dn9, locals.var_pparam_b4soist3_dn10, locals.var_pparam_b4soist3_dn11, locals.var_pparam_b4soist3_dn12,)
    }
};
        locals.var_pparam_b4soist3 = assign10170_e7447;
        locals.var_pparam_b4soist3_dn3 = assign10170_e7447_d_n3;
        locals.var_pparam_b4soist3_dn4 = assign10170_e7447_d_n4;
        locals.var_pparam_b4soist3_dn5 = assign10170_e7447_d_n5;
        locals.var_pparam_b4soist3_dn6 = assign10170_e7447_d_n6;
        locals.var_pparam_b4soist3_dn7 = assign10170_e7447_d_n7;
        locals.var_pparam_b4soist3_dn8 = assign10170_e7447_d_n8;
        locals.var_pparam_b4soist3_dn9 = assign10170_e7447_d_n9;
        locals.var_pparam_b4soist3_dn10 = assign10170_e7447_d_n10;
        locals.var_pparam_b4soist3_dn11 = assign10170_e7447_d_n11;
        locals.var_pparam_b4soist3_dn12 = assign10170_e7447_d_n12;

        let (assign10180_e7452, assign10180_e7452_d_n3, assign10180_e7452_d_n4, assign10180_e7452_d_n5, assign10180_e7452_d_n6, assign10180_e7452_d_n7, assign10180_e7452_d_n8, assign10180_e7452_d_n9, assign10180_e7452_d_n10, assign10180_e7452_d_n11, assign10180_e7452_d_n12,) = {
    if (locals.var_guard929 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soist4, locals.var_b4soist4_dn3, locals.var_b4soist4_dn4, locals.var_b4soist4_dn5, locals.var_b4soist4_dn6, locals.var_b4soist4_dn7, locals.var_b4soist4_dn8, locals.var_b4soist4_dn9, locals.var_b4soist4_dn10, locals.var_b4soist4_dn11, locals.var_b4soist4_dn12,)
    }
};
        locals.var_b4soist4 = assign10180_e7452;
        locals.var_b4soist4_dn3 = assign10180_e7452_d_n3;
        locals.var_b4soist4_dn4 = assign10180_e7452_d_n4;
        locals.var_b4soist4_dn5 = assign10180_e7452_d_n5;
        locals.var_b4soist4_dn6 = assign10180_e7452_d_n6;
        locals.var_b4soist4_dn7 = assign10180_e7452_d_n7;
        locals.var_b4soist4_dn8 = assign10180_e7452_d_n8;
        locals.var_b4soist4_dn9 = assign10180_e7452_d_n9;
        locals.var_b4soist4_dn10 = assign10180_e7452_d_n10;
        locals.var_b4soist4_dn11 = assign10180_e7452_d_n11;
        locals.var_b4soist4_dn12 = assign10180_e7452_d_n12;

        let (assign10190_e7457, assign10190_e7457_d_n3, assign10190_e7457_d_n4, assign10190_e7457_d_n5, assign10190_e7457_d_n6, assign10190_e7457_d_n7, assign10190_e7457_d_n8, assign10190_e7457_d_n9, assign10190_e7457_d_n10, assign10190_e7457_d_n11, assign10190_e7457_d_n12,) = {
    if (locals.var_guard929 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soidt2, locals.var_pparam_b4soidt2_dn3, locals.var_pparam_b4soidt2_dn4, locals.var_pparam_b4soidt2_dn5, locals.var_pparam_b4soidt2_dn6, locals.var_pparam_b4soidt2_dn7, locals.var_pparam_b4soidt2_dn8, locals.var_pparam_b4soidt2_dn9, locals.var_pparam_b4soidt2_dn10, locals.var_pparam_b4soidt2_dn11, locals.var_pparam_b4soidt2_dn12,)
    }
};
        locals.var_pparam_b4soidt2 = assign10190_e7457;
        locals.var_pparam_b4soidt2_dn3 = assign10190_e7457_d_n3;
        locals.var_pparam_b4soidt2_dn4 = assign10190_e7457_d_n4;
        locals.var_pparam_b4soidt2_dn5 = assign10190_e7457_d_n5;
        locals.var_pparam_b4soidt2_dn6 = assign10190_e7457_d_n6;
        locals.var_pparam_b4soidt2_dn7 = assign10190_e7457_d_n7;
        locals.var_pparam_b4soidt2_dn8 = assign10190_e7457_d_n8;
        locals.var_pparam_b4soidt2_dn9 = assign10190_e7457_d_n9;
        locals.var_pparam_b4soidt2_dn10 = assign10190_e7457_d_n10;
        locals.var_pparam_b4soidt2_dn11 = assign10190_e7457_d_n11;
        locals.var_pparam_b4soidt2_dn12 = assign10190_e7457_d_n12;

        let (assign10200_e7462, assign10200_e7462_d_n3, assign10200_e7462_d_n4, assign10200_e7462_d_n5, assign10200_e7462_d_n6, assign10200_e7462_d_n7, assign10200_e7462_d_n8, assign10200_e7462_d_n9, assign10200_e7462_d_n10, assign10200_e7462_d_n11, assign10200_e7462_d_n12,) = {
    if (locals.var_guard929 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soidt3, locals.var_pparam_b4soidt3_dn3, locals.var_pparam_b4soidt3_dn4, locals.var_pparam_b4soidt3_dn5, locals.var_pparam_b4soidt3_dn6, locals.var_pparam_b4soidt3_dn7, locals.var_pparam_b4soidt3_dn8, locals.var_pparam_b4soidt3_dn9, locals.var_pparam_b4soidt3_dn10, locals.var_pparam_b4soidt3_dn11, locals.var_pparam_b4soidt3_dn12,)
    }
};
        locals.var_pparam_b4soidt3 = assign10200_e7462;
        locals.var_pparam_b4soidt3_dn3 = assign10200_e7462_d_n3;
        locals.var_pparam_b4soidt3_dn4 = assign10200_e7462_d_n4;
        locals.var_pparam_b4soidt3_dn5 = assign10200_e7462_d_n5;
        locals.var_pparam_b4soidt3_dn6 = assign10200_e7462_d_n6;
        locals.var_pparam_b4soidt3_dn7 = assign10200_e7462_d_n7;
        locals.var_pparam_b4soidt3_dn8 = assign10200_e7462_d_n8;
        locals.var_pparam_b4soidt3_dn9 = assign10200_e7462_d_n9;
        locals.var_pparam_b4soidt3_dn10 = assign10200_e7462_d_n10;
        locals.var_pparam_b4soidt3_dn11 = assign10200_e7462_d_n11;
        locals.var_pparam_b4soidt3_dn12 = assign10200_e7462_d_n12;

        let (assign10210_e7467, assign10210_e7467_d_n3, assign10210_e7467_d_n4, assign10210_e7467_d_n5, assign10210_e7467_d_n6, assign10210_e7467_d_n7, assign10210_e7467_d_n8, assign10210_e7467_d_n9, assign10210_e7467_d_n10, assign10210_e7467_d_n11, assign10210_e7467_d_n12,) = {
    if (locals.var_guard929 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soidt4, locals.var_b4soidt4_dn3, locals.var_b4soidt4_dn4, locals.var_b4soidt4_dn5, locals.var_b4soidt4_dn6, locals.var_b4soidt4_dn7, locals.var_b4soidt4_dn8, locals.var_b4soidt4_dn9, locals.var_b4soidt4_dn10, locals.var_b4soidt4_dn11, locals.var_b4soidt4_dn12,)
    }
};
        locals.var_b4soidt4 = assign10210_e7467;
        locals.var_b4soidt4_dn3 = assign10210_e7467_d_n3;
        locals.var_b4soidt4_dn4 = assign10210_e7467_d_n4;
        locals.var_b4soidt4_dn5 = assign10210_e7467_d_n5;
        locals.var_b4soidt4_dn6 = assign10210_e7467_d_n6;
        locals.var_b4soidt4_dn7 = assign10210_e7467_d_n7;
        locals.var_b4soidt4_dn8 = assign10210_e7467_d_n8;
        locals.var_b4soidt4_dn9 = assign10210_e7467_d_n9;
        locals.var_b4soidt4_dn10 = assign10210_e7467_d_n10;
        locals.var_b4soidt4_dn11 = assign10210_e7467_d_n11;
        locals.var_b4soidt4_dn12 = assign10210_e7467_d_n12;

        let assign10220_e7474: f64 = if ((locals.var_b4soicfrcoeff < 1.0) || (locals.var_b4soicfrcoeff > 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard931 = assign10220_e7474;

        let (assign10230_e7478,) = {
    if (locals.var_guard931 != 0.0) {
        (1.0,)
    } else {
        (locals.var_b4soicfrcoeff,)
    }
};
        locals.var_b4soicfrcoeff = assign10230_e7478;

        let assign10240_e7484: f64 = (locals.var_b4soitsi / locals.var_b4soitbox);
        let assign10240_e7485: f64 = (1.0 + assign10240_e7484);
        let assign10240_e7486: f64 = (locals.var_b4soicfrcoeff * assign10240_e7485);
        let (assign10240_e7499,) = {
    if (assign10240_e7486 > 1e-38) {
        let assign10240_e7493: f64 = (locals.var_b4soitsi / locals.var_b4soitbox);
        let assign10240_e7494: f64 = (1.0 + assign10240_e7493);
        let assign10240_e7495: f64 = (locals.var_b4soicfrcoeff * assign10240_e7494);
        let assign10240_e7496: f64 = (assign10240_e7495).ln();
        (assign10240_e7496,)
    } else {
        let assign10240_e7498: f64 = (-87.49823353377374);
        (assign10240_e7498,)
    }
};
        let assign10240_e7500: f64 = (locals.var_b4soicsdesw * assign10240_e7499);
        locals.var_t0 = assign10240_e7500;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;

        let assign10250_e7503: f64 = (locals.var_b4soisourceperimeter - locals.var_b4soiw);
        locals.var_t1 = assign10250_e7503;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;

        let assign10260_e7506: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard932 = assign10260_e7506;

        let (assign10270_e7512, assign10270_e7512_d_n3, assign10270_e7512_d_n4, assign10270_e7512_d_n5, assign10270_e7512_d_n6, assign10270_e7512_d_n7, assign10270_e7512_d_n8, assign10270_e7512_d_n9, assign10270_e7512_d_n10, assign10270_e7512_d_n11, assign10270_e7512_d_n12,) = {
    if (locals.var_guard932 != 0.0) {
        let assign10270_e7510: f64 = (locals.var_t0 * locals.var_t1);
        (assign10270_e7510, ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)), ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)), ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)), ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)), ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)), ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)), ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)), ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)), ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)), ((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12)),)
    } else {
        (locals.var_b4soicsesw, locals.var_b4soicsesw_dn3, locals.var_b4soicsesw_dn4, locals.var_b4soicsesw_dn5, locals.var_b4soicsesw_dn6, locals.var_b4soicsesw_dn7, locals.var_b4soicsesw_dn8, locals.var_b4soicsesw_dn9, locals.var_b4soicsesw_dn10, locals.var_b4soicsesw_dn11, locals.var_b4soicsesw_dn12,)
    }
};
        locals.var_b4soicsesw = assign10270_e7512;
        locals.var_b4soicsesw_dn3 = assign10270_e7512_d_n3;
        locals.var_b4soicsesw_dn4 = assign10270_e7512_d_n4;
        locals.var_b4soicsesw_dn5 = assign10270_e7512_d_n5;
        locals.var_b4soicsesw_dn6 = assign10270_e7512_d_n6;
        locals.var_b4soicsesw_dn7 = assign10270_e7512_d_n7;
        locals.var_b4soicsesw_dn8 = assign10270_e7512_d_n8;
        locals.var_b4soicsesw_dn9 = assign10270_e7512_d_n9;
        locals.var_b4soicsesw_dn10 = assign10270_e7512_d_n10;
        locals.var_b4soicsesw_dn11 = assign10270_e7512_d_n11;
        locals.var_b4soicsesw_dn12 = assign10270_e7512_d_n12;

    }

    pub(super) fn stamp_transient_block_20(
        locals: &mut StampLocals,
    ) {
        let (assign10280_e7517, assign10280_e7517_d_n3, assign10280_e7517_d_n4, assign10280_e7517_d_n5, assign10280_e7517_d_n6, assign10280_e7517_d_n7, assign10280_e7517_d_n8, assign10280_e7517_d_n9, assign10280_e7517_d_n10, assign10280_e7517_d_n11, assign10280_e7517_d_n12,) = {
    if (locals.var_guard932 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soicsesw, locals.var_b4soicsesw_dn3, locals.var_b4soicsesw_dn4, locals.var_b4soicsesw_dn5, locals.var_b4soicsesw_dn6, locals.var_b4soicsesw_dn7, locals.var_b4soicsesw_dn8, locals.var_b4soicsesw_dn9, locals.var_b4soicsesw_dn10, locals.var_b4soicsesw_dn11, locals.var_b4soicsesw_dn12,)
    }
};
        locals.var_b4soicsesw = assign10280_e7517;
        locals.var_b4soicsesw_dn3 = assign10280_e7517_d_n3;
        locals.var_b4soicsesw_dn4 = assign10280_e7517_d_n4;
        locals.var_b4soicsesw_dn5 = assign10280_e7517_d_n5;
        locals.var_b4soicsesw_dn6 = assign10280_e7517_d_n6;
        locals.var_b4soicsesw_dn7 = assign10280_e7517_d_n7;
        locals.var_b4soicsesw_dn8 = assign10280_e7517_d_n8;
        locals.var_b4soicsesw_dn9 = assign10280_e7517_d_n9;
        locals.var_b4soicsesw_dn10 = assign10280_e7517_d_n10;
        locals.var_b4soicsesw_dn11 = assign10280_e7517_d_n11;
        locals.var_b4soicsesw_dn12 = assign10280_e7517_d_n12;

        let assign10290_e7520: f64 = (locals.var_b4soidrainperimeter - locals.var_b4soiw);
        locals.var_t1 = assign10290_e7520;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;

        let assign10300_e7523: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard933 = assign10300_e7523;

        let (assign10310_e7529, assign10310_e7529_d_n3, assign10310_e7529_d_n4, assign10310_e7529_d_n5, assign10310_e7529_d_n6, assign10310_e7529_d_n7, assign10310_e7529_d_n8, assign10310_e7529_d_n9, assign10310_e7529_d_n10, assign10310_e7529_d_n11, assign10310_e7529_d_n12,) = {
    if (locals.var_guard933 != 0.0) {
        let assign10310_e7527: f64 = (locals.var_t0 * locals.var_t1);
        (assign10310_e7527, ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)), ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)), ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)), ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)), ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)), ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)), ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)), ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)), ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)), ((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12)),)
    } else {
        (locals.var_b4soicdesw, locals.var_b4soicdesw_dn3, locals.var_b4soicdesw_dn4, locals.var_b4soicdesw_dn5, locals.var_b4soicdesw_dn6, locals.var_b4soicdesw_dn7, locals.var_b4soicdesw_dn8, locals.var_b4soicdesw_dn9, locals.var_b4soicdesw_dn10, locals.var_b4soicdesw_dn11, locals.var_b4soicdesw_dn12,)
    }
};
        locals.var_b4soicdesw = assign10310_e7529;
        locals.var_b4soicdesw_dn3 = assign10310_e7529_d_n3;
        locals.var_b4soicdesw_dn4 = assign10310_e7529_d_n4;
        locals.var_b4soicdesw_dn5 = assign10310_e7529_d_n5;
        locals.var_b4soicdesw_dn6 = assign10310_e7529_d_n6;
        locals.var_b4soicdesw_dn7 = assign10310_e7529_d_n7;
        locals.var_b4soicdesw_dn8 = assign10310_e7529_d_n8;
        locals.var_b4soicdesw_dn9 = assign10310_e7529_d_n9;
        locals.var_b4soicdesw_dn10 = assign10310_e7529_d_n10;
        locals.var_b4soicdesw_dn11 = assign10310_e7529_d_n11;
        locals.var_b4soicdesw_dn12 = assign10310_e7529_d_n12;

        let (assign10320_e7534, assign10320_e7534_d_n3, assign10320_e7534_d_n4, assign10320_e7534_d_n5, assign10320_e7534_d_n6, assign10320_e7534_d_n7, assign10320_e7534_d_n8, assign10320_e7534_d_n9, assign10320_e7534_d_n10, assign10320_e7534_d_n11, assign10320_e7534_d_n12,) = {
    if (locals.var_guard933 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soicdesw, locals.var_b4soicdesw_dn3, locals.var_b4soicdesw_dn4, locals.var_b4soicdesw_dn5, locals.var_b4soicdesw_dn6, locals.var_b4soicdesw_dn7, locals.var_b4soicdesw_dn8, locals.var_b4soicdesw_dn9, locals.var_b4soicdesw_dn10, locals.var_b4soicdesw_dn11, locals.var_b4soicdesw_dn12,)
    }
};
        locals.var_b4soicdesw = assign10320_e7534;
        locals.var_b4soicdesw_dn3 = assign10320_e7534_d_n3;
        locals.var_b4soicdesw_dn4 = assign10320_e7534_d_n4;
        locals.var_b4soicdesw_dn5 = assign10320_e7534_d_n5;
        locals.var_b4soicdesw_dn6 = assign10320_e7534_d_n6;
        locals.var_b4soicdesw_dn7 = assign10320_e7534_d_n7;
        locals.var_b4soicdesw_dn8 = assign10320_e7534_d_n8;
        locals.var_b4soicdesw_dn9 = assign10320_e7534_d_n9;
        locals.var_b4soicdesw_dn10 = assign10320_e7534_d_n10;
        locals.var_b4soicdesw_dn11 = assign10320_e7534_d_n11;
        locals.var_b4soicdesw_dn12 = assign10320_e7534_d_n12;

        let assign10330_e7537: f64 = (locals.var_b4soisheetresistance * locals.var_b4soidrainsquares);
        locals.var_b4soidrainresistance = assign10330_e7537;

        let assign10340_e7540: f64 = if locals.var_b4soidrainresistance <= 0.001 { 1.0 } else { 0.0 };
        locals.var_guard934 = assign10340_e7540;

        let (assign10350_e7544,) = {
    if (locals.var_guard934 != 0.0) {
        (0.001,)
    } else {
        (locals.var_b4soidrainresistance,)
    }
};
        locals.var_b4soidrainresistance = assign10350_e7544;

        let assign10360_e7547: f64 = (locals.var_b4soisheetresistance * locals.var_b4soisourcesquares);
        locals.var_b4soisourceresistance = assign10360_e7547;

        let assign10370_e7550: f64 = if locals.var_b4soisourceresistance <= 0.001 { 1.0 } else { 0.0 };
        locals.var_guard935 = assign10370_e7550;

        let (assign10380_e7554,) = {
    if (locals.var_guard935 != 0.0) {
        (0.001,)
    } else {
        (locals.var_b4soisourceresistance,)
    }
};
        locals.var_b4soisourceresistance = assign10380_e7554;

        let assign10390_e7557: f64 = if locals.var_b4soiln < 1e-15 { 1.0 } else { 0.0 };
        locals.var_guard936 = assign10390_e7557;

        let (assign10400_e7561,) = {
    if (locals.var_guard936 != 0.0) {
        (1e-15,)
    } else {
        (locals.var_b4soiln,)
    }
};
        locals.var_b4soiln = assign10400_e7561;

        let assign10410_e7563: f64 = (-0.5);
        let assign10410_e7565: f64 = (assign10410_e7563 * locals.var_pparam_b4soileff);
        let assign10410_e7567: f64 = (assign10410_e7565 * locals.var_pparam_b4soileff);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_b4soiln;
        let assign10410_e7569: f64 = (assign10410_e7567 * __rspice_inv_cse_0);
        let assign10410_e7571: f64 = (assign10410_e7569 * __rspice_inv_cse_0);
        locals.var_t0 = assign10410_e7571;
        locals.var_t0_dn3 = (((((assign10410_e7563 * locals.var_pparam_b4soileff_dn3) * locals.var_pparam_b4soileff) + (assign10410_e7565 * locals.var_pparam_b4soileff_dn3)) / locals.var_b4soiln) / locals.var_b4soiln);
        locals.var_t0_dn4 = (((((assign10410_e7563 * locals.var_pparam_b4soileff_dn4) * locals.var_pparam_b4soileff) + (assign10410_e7565 * locals.var_pparam_b4soileff_dn4)) / locals.var_b4soiln) / locals.var_b4soiln);
        locals.var_t0_dn5 = (((((assign10410_e7563 * locals.var_pparam_b4soileff_dn5) * locals.var_pparam_b4soileff) + (assign10410_e7565 * locals.var_pparam_b4soileff_dn5)) / locals.var_b4soiln) / locals.var_b4soiln);
        locals.var_t0_dn6 = (((((assign10410_e7563 * locals.var_pparam_b4soileff_dn6) * locals.var_pparam_b4soileff) + (assign10410_e7565 * locals.var_pparam_b4soileff_dn6)) / locals.var_b4soiln) / locals.var_b4soiln);
        locals.var_t0_dn7 = (((((assign10410_e7563 * locals.var_pparam_b4soileff_dn7) * locals.var_pparam_b4soileff) + (assign10410_e7565 * locals.var_pparam_b4soileff_dn7)) / locals.var_b4soiln) / locals.var_b4soiln);
        locals.var_t0_dn8 = (((((assign10410_e7563 * locals.var_pparam_b4soileff_dn8) * locals.var_pparam_b4soileff) + (assign10410_e7565 * locals.var_pparam_b4soileff_dn8)) / locals.var_b4soiln) / locals.var_b4soiln);
        locals.var_t0_dn9 = (((((assign10410_e7563 * locals.var_pparam_b4soileff_dn9) * locals.var_pparam_b4soileff) + (assign10410_e7565 * locals.var_pparam_b4soileff_dn9)) / locals.var_b4soiln) / locals.var_b4soiln);
        locals.var_t0_dn10 = (((((assign10410_e7563 * locals.var_pparam_b4soileff_dn10) * locals.var_pparam_b4soileff) + (assign10410_e7565 * locals.var_pparam_b4soileff_dn10)) / locals.var_b4soiln) / locals.var_b4soiln);
        locals.var_t0_dn11 = (((((assign10410_e7563 * locals.var_pparam_b4soileff_dn11) * locals.var_pparam_b4soileff) + (assign10410_e7565 * locals.var_pparam_b4soileff_dn11)) / locals.var_b4soiln) / locals.var_b4soiln);
        locals.var_t0_dn12 = (((((assign10410_e7563 * locals.var_pparam_b4soileff_dn12) * locals.var_pparam_b4soileff) + (assign10410_e7565 * locals.var_pparam_b4soileff_dn12)) / locals.var_b4soiln) / locals.var_b4soiln);

        let assign10420_e7574: f64 = if locals.var_t0 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard937 = assign10420_e7574;

        let (assign10430_e7584, assign10430_e7584_d_n3, assign10430_e7584_d_n4, assign10430_e7584_d_n5, assign10430_e7584_d_n6, assign10430_e7584_d_n7, assign10430_e7584_d_n8, assign10430_e7584_d_n9, assign10430_e7584_d_n10, assign10430_e7584_d_n11, assign10430_e7584_d_n12,) = {
    if (locals.var_guard937 != 0.0) {
        let assign10430_e7579: f64 = (1.0 + locals.var_t0);
        let assign10430_e7581: f64 = (assign10430_e7579 - 100.0);
        let assign10430_e7582: f64 = (2.688117142e43 * assign10430_e7581);
        (assign10430_e7582, (2.688117142e43 * locals.var_t0_dn3), (2.688117142e43 * locals.var_t0_dn4), (2.688117142e43 * locals.var_t0_dn5), (2.688117142e43 * locals.var_t0_dn6), (2.688117142e43 * locals.var_t0_dn7), (2.688117142e43 * locals.var_t0_dn8), (2.688117142e43 * locals.var_t0_dn9), (2.688117142e43 * locals.var_t0_dn10), (2.688117142e43 * locals.var_t0_dn11), (2.688117142e43 * locals.var_t0_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign10430_e7584;
        locals.var_t1_dn3 = assign10430_e7584_d_n3;
        locals.var_t1_dn4 = assign10430_e7584_d_n4;
        locals.var_t1_dn5 = assign10430_e7584_d_n5;
        locals.var_t1_dn6 = assign10430_e7584_d_n6;
        locals.var_t1_dn7 = assign10430_e7584_d_n7;
        locals.var_t1_dn8 = assign10430_e7584_d_n8;
        locals.var_t1_dn9 = assign10430_e7584_d_n9;
        locals.var_t1_dn10 = assign10430_e7584_d_n10;
        locals.var_t1_dn11 = assign10430_e7584_d_n11;
        locals.var_t1_dn12 = assign10430_e7584_d_n12;

        let assign10440_e7587: f64 = (-100.0);
        let assign10440_e7588: f64 = if locals.var_t0 < assign10440_e7587 { 1.0 } else { 0.0 };
        locals.var_guard938 = assign10440_e7588;

        let (assign10450_e7595, assign10450_e7595_d_n3, assign10450_e7595_d_n4, assign10450_e7595_d_n5, assign10450_e7595_d_n6, assign10450_e7595_d_n7, assign10450_e7595_d_n8, assign10450_e7595_d_n9, assign10450_e7595_d_n10, assign10450_e7595_d_n11, assign10450_e7595_d_n12,) = {
    if ((locals.var_guard937 == 0.0) && (locals.var_guard938 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign10450_e7595;
        locals.var_t1_dn3 = assign10450_e7595_d_n3;
        locals.var_t1_dn4 = assign10450_e7595_d_n4;
        locals.var_t1_dn5 = assign10450_e7595_d_n5;
        locals.var_t1_dn6 = assign10450_e7595_d_n6;
        locals.var_t1_dn7 = assign10450_e7595_d_n7;
        locals.var_t1_dn8 = assign10450_e7595_d_n8;
        locals.var_t1_dn9 = assign10450_e7595_d_n9;
        locals.var_t1_dn10 = assign10450_e7595_d_n10;
        locals.var_t1_dn11 = assign10450_e7595_d_n11;
        locals.var_t1_dn12 = assign10450_e7595_d_n12;

        let (assign10460_e7604, assign10460_e7604_d_n3, assign10460_e7604_d_n4, assign10460_e7604_d_n5, assign10460_e7604_d_n6, assign10460_e7604_d_n7, assign10460_e7604_d_n8, assign10460_e7604_d_n9, assign10460_e7604_d_n10, assign10460_e7604_d_n11, assign10460_e7604_d_n12,) = {
    if ((locals.var_guard937 == 0.0) && (locals.var_guard938 == 0.0)) {
        let assign10460_e7602: f64 = (locals.var_t0).exp();
        (assign10460_e7602, (assign10460_e7602 * locals.var_t0_dn3), (assign10460_e7602 * locals.var_t0_dn4), (assign10460_e7602 * locals.var_t0_dn5), (assign10460_e7602 * locals.var_t0_dn6), (assign10460_e7602 * locals.var_t0_dn7), (assign10460_e7602 * locals.var_t0_dn8), (assign10460_e7602 * locals.var_t0_dn9), (assign10460_e7602 * locals.var_t0_dn10), (assign10460_e7602 * locals.var_t0_dn11), (assign10460_e7602 * locals.var_t0_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign10460_e7604;
        locals.var_t1_dn3 = assign10460_e7604_d_n3;
        locals.var_t1_dn4 = assign10460_e7604_d_n4;
        locals.var_t1_dn5 = assign10460_e7604_d_n5;
        locals.var_t1_dn6 = assign10460_e7604_d_n6;
        locals.var_t1_dn7 = assign10460_e7604_d_n7;
        locals.var_t1_dn8 = assign10460_e7604_d_n8;
        locals.var_t1_dn9 = assign10460_e7604_d_n9;
        locals.var_t1_dn10 = assign10460_e7604_d_n10;
        locals.var_t1_dn11 = assign10460_e7604_d_n11;
        locals.var_t1_dn12 = assign10460_e7604_d_n12;

        locals.var_pparam_b4soiarfabjt = locals.var_t1;
        locals.var_pparam_b4soiarfabjt_dn3 = locals.var_t1_dn3;
        locals.var_pparam_b4soiarfabjt_dn4 = locals.var_t1_dn4;
        locals.var_pparam_b4soiarfabjt_dn5 = locals.var_t1_dn5;
        locals.var_pparam_b4soiarfabjt_dn6 = locals.var_t1_dn6;
        locals.var_pparam_b4soiarfabjt_dn7 = locals.var_t1_dn7;
        locals.var_pparam_b4soiarfabjt_dn8 = locals.var_t1_dn8;
        locals.var_pparam_b4soiarfabjt_dn9 = locals.var_t1_dn9;
        locals.var_pparam_b4soiarfabjt_dn10 = locals.var_t1_dn10;
        locals.var_pparam_b4soiarfabjt_dn11 = locals.var_t1_dn11;
        locals.var_pparam_b4soiarfabjt_dn12 = locals.var_t1_dn12;

        let assign10480_e7609: f64 = (1.0 / locals.var_pparam_b4soileff);
        let assign10480_e7612: f64 = (1.0 / locals.var_b4soiln);
        let assign10480_e7613: f64 = (assign10480_e7609 + assign10480_e7612);
        let assign10480_e7614: f64 = (locals.var_pparam_b4soilbjt0 * assign10480_e7613);
        locals.var_t0 = assign10480_e7614;
        locals.var_t0_dn3 = ((locals.var_pparam_b4soilbjt0_dn3 * assign10480_e7613) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn3 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));
        locals.var_t0_dn4 = ((locals.var_pparam_b4soilbjt0_dn4 * assign10480_e7613) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn4 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));
        locals.var_t0_dn5 = ((locals.var_pparam_b4soilbjt0_dn5 * assign10480_e7613) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn5 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));
        locals.var_t0_dn6 = ((locals.var_pparam_b4soilbjt0_dn6 * assign10480_e7613) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn6 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));
        locals.var_t0_dn7 = ((locals.var_pparam_b4soilbjt0_dn7 * assign10480_e7613) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn7 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));
        locals.var_t0_dn8 = ((locals.var_pparam_b4soilbjt0_dn8 * assign10480_e7613) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn8 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));
        locals.var_t0_dn9 = ((locals.var_pparam_b4soilbjt0_dn9 * assign10480_e7613) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn9 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));
        locals.var_t0_dn10 = ((locals.var_pparam_b4soilbjt0_dn10 * assign10480_e7613) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn10 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));
        locals.var_t0_dn11 = ((locals.var_pparam_b4soilbjt0_dn11 * assign10480_e7613) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn11 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));
        locals.var_t0_dn12 = ((locals.var_pparam_b4soilbjt0_dn12 * assign10480_e7613) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn12 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));

        let assign10490_e7617: f64 = (locals.var_t0).powf(locals.var_pparam_b4soinbjt);
        locals.var_pparam_b4soilratio = assign10490_e7617;
        locals.var_pparam_b4soilratio_dn3 = if locals.var_pparam_b4soinbjt_dn3 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn3)) } } else { (assign10490_e7617 * ((locals.var_pparam_b4soinbjt_dn3 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn3 / locals.var_t0)))) };
        locals.var_pparam_b4soilratio_dn4 = if locals.var_pparam_b4soinbjt_dn4 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn4)) } } else { (assign10490_e7617 * ((locals.var_pparam_b4soinbjt_dn4 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn4 / locals.var_t0)))) };
        locals.var_pparam_b4soilratio_dn5 = if locals.var_pparam_b4soinbjt_dn5 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn5)) } } else { (assign10490_e7617 * ((locals.var_pparam_b4soinbjt_dn5 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn5 / locals.var_t0)))) };
        locals.var_pparam_b4soilratio_dn6 = if locals.var_pparam_b4soinbjt_dn6 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn6)) } } else { (assign10490_e7617 * ((locals.var_pparam_b4soinbjt_dn6 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn6 / locals.var_t0)))) };
        locals.var_pparam_b4soilratio_dn7 = if locals.var_pparam_b4soinbjt_dn7 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn7)) } } else { (assign10490_e7617 * ((locals.var_pparam_b4soinbjt_dn7 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn7 / locals.var_t0)))) };
        locals.var_pparam_b4soilratio_dn8 = if locals.var_pparam_b4soinbjt_dn8 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn8)) } } else { (assign10490_e7617 * ((locals.var_pparam_b4soinbjt_dn8 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn8 / locals.var_t0)))) };
        locals.var_pparam_b4soilratio_dn9 = if locals.var_pparam_b4soinbjt_dn9 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn9)) } } else { (assign10490_e7617 * ((locals.var_pparam_b4soinbjt_dn9 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn9 / locals.var_t0)))) };
        locals.var_pparam_b4soilratio_dn10 = if locals.var_pparam_b4soinbjt_dn10 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn10)) } } else { (assign10490_e7617 * ((locals.var_pparam_b4soinbjt_dn10 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn10 / locals.var_t0)))) };
        locals.var_pparam_b4soilratio_dn11 = if locals.var_pparam_b4soinbjt_dn11 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn11)) } } else { (assign10490_e7617 * ((locals.var_pparam_b4soinbjt_dn11 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn11 / locals.var_t0)))) };
        locals.var_pparam_b4soilratio_dn12 = if locals.var_pparam_b4soinbjt_dn12 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn12)) } } else { (assign10490_e7617 * ((locals.var_pparam_b4soinbjt_dn12 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn12 / locals.var_t0)))) };

        let assign10500_e7622: f64 = (locals.var_t0).powf(locals.var_pparam_b4soindif);
        let assign10500_e7623: f64 = (locals.var_b4soildif0 * assign10500_e7622);
        let assign10500_e7624: f64 = (1.0 + assign10500_e7623);
        locals.var_pparam_b4soilratiodif = assign10500_e7624;
        locals.var_pparam_b4soilratiodif_dn3 = (locals.var_b4soildif0 * if locals.var_pparam_b4soindif_dn3 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn3)) } } else { (assign10500_e7622 * ((locals.var_pparam_b4soindif_dn3 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn3 / locals.var_t0)))) });
        locals.var_pparam_b4soilratiodif_dn4 = (locals.var_b4soildif0 * if locals.var_pparam_b4soindif_dn4 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn4)) } } else { (assign10500_e7622 * ((locals.var_pparam_b4soindif_dn4 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn4 / locals.var_t0)))) });
        locals.var_pparam_b4soilratiodif_dn5 = (locals.var_b4soildif0 * if locals.var_pparam_b4soindif_dn5 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn5)) } } else { (assign10500_e7622 * ((locals.var_pparam_b4soindif_dn5 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn5 / locals.var_t0)))) });
        locals.var_pparam_b4soilratiodif_dn6 = (locals.var_b4soildif0 * if locals.var_pparam_b4soindif_dn6 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn6)) } } else { (assign10500_e7622 * ((locals.var_pparam_b4soindif_dn6 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn6 / locals.var_t0)))) });
        locals.var_pparam_b4soilratiodif_dn7 = (locals.var_b4soildif0 * if locals.var_pparam_b4soindif_dn7 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn7)) } } else { (assign10500_e7622 * ((locals.var_pparam_b4soindif_dn7 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn7 / locals.var_t0)))) });
        locals.var_pparam_b4soilratiodif_dn8 = (locals.var_b4soildif0 * if locals.var_pparam_b4soindif_dn8 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn8)) } } else { (assign10500_e7622 * ((locals.var_pparam_b4soindif_dn8 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn8 / locals.var_t0)))) });
        locals.var_pparam_b4soilratiodif_dn9 = (locals.var_b4soildif0 * if locals.var_pparam_b4soindif_dn9 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn9)) } } else { (assign10500_e7622 * ((locals.var_pparam_b4soindif_dn9 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn9 / locals.var_t0)))) });
        locals.var_pparam_b4soilratiodif_dn10 = (locals.var_b4soildif0 * if locals.var_pparam_b4soindif_dn10 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn10)) } } else { (assign10500_e7622 * ((locals.var_pparam_b4soindif_dn10 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn10 / locals.var_t0)))) });
        locals.var_pparam_b4soilratiodif_dn11 = (locals.var_b4soildif0 * if locals.var_pparam_b4soindif_dn11 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn11)) } } else { (assign10500_e7622 * ((locals.var_pparam_b4soindif_dn11 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn11 / locals.var_t0)))) });
        locals.var_pparam_b4soilratiodif_dn12 = (locals.var_b4soildif0 * if locals.var_pparam_b4soindif_dn12 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn12)) } } else { (assign10500_e7622 * ((locals.var_pparam_b4soindif_dn12 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn12 / locals.var_t0)))) });

        let assign10510_e7628: f64 = (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff);
        let assign10510_e7629: f64 = (locals.var_pparam_b4soivabjt + assign10510_e7628);
        locals.var_pparam_b4soivearly = assign10510_e7629;
        locals.var_pparam_b4soivearly_dn3 = (locals.var_pparam_b4soivabjt_dn3 + ((locals.var_pparam_b4soiaely_dn3 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn3)));
        locals.var_pparam_b4soivearly_dn4 = (locals.var_pparam_b4soivabjt_dn4 + ((locals.var_pparam_b4soiaely_dn4 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn4)));
        locals.var_pparam_b4soivearly_dn5 = (locals.var_pparam_b4soivabjt_dn5 + ((locals.var_pparam_b4soiaely_dn5 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn5)));
        locals.var_pparam_b4soivearly_dn6 = (locals.var_pparam_b4soivabjt_dn6 + ((locals.var_pparam_b4soiaely_dn6 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn6)));
        locals.var_pparam_b4soivearly_dn7 = (locals.var_pparam_b4soivabjt_dn7 + ((locals.var_pparam_b4soiaely_dn7 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn7)));
        locals.var_pparam_b4soivearly_dn8 = (locals.var_pparam_b4soivabjt_dn8 + ((locals.var_pparam_b4soiaely_dn8 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn8)));
        locals.var_pparam_b4soivearly_dn9 = (locals.var_pparam_b4soivabjt_dn9 + ((locals.var_pparam_b4soiaely_dn9 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn9)));
        locals.var_pparam_b4soivearly_dn10 = (locals.var_pparam_b4soivabjt_dn10 + ((locals.var_pparam_b4soiaely_dn10 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn10)));
        locals.var_pparam_b4soivearly_dn11 = (locals.var_pparam_b4soivabjt_dn11 + ((locals.var_pparam_b4soiaely_dn11 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn11)));
        locals.var_pparam_b4soivearly_dn12 = (locals.var_pparam_b4soivabjt_dn12 + ((locals.var_pparam_b4soiaely_dn12 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn12)));

        let assign10520_e7632: f64 = if locals.var_pparam_b4soivearly < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard939 = assign10520_e7632;

        let (assign10530_e7636, assign10530_e7636_d_n3, assign10530_e7636_d_n4, assign10530_e7636_d_n5, assign10530_e7636_d_n6, assign10530_e7636_d_n7, assign10530_e7636_d_n8, assign10530_e7636_d_n9, assign10530_e7636_d_n10, assign10530_e7636_d_n11, assign10530_e7636_d_n12,) = {
    if (locals.var_guard939 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soivearly, locals.var_pparam_b4soivearly_dn3, locals.var_pparam_b4soivearly_dn4, locals.var_pparam_b4soivearly_dn5, locals.var_pparam_b4soivearly_dn6, locals.var_pparam_b4soivearly_dn7, locals.var_pparam_b4soivearly_dn8, locals.var_pparam_b4soivearly_dn9, locals.var_pparam_b4soivearly_dn10, locals.var_pparam_b4soivearly_dn11, locals.var_pparam_b4soivearly_dn12,)
    }
};
        locals.var_pparam_b4soivearly = assign10530_e7636;
        locals.var_pparam_b4soivearly_dn3 = assign10530_e7636_d_n3;
        locals.var_pparam_b4soivearly_dn4 = assign10530_e7636_d_n4;
        locals.var_pparam_b4soivearly_dn5 = assign10530_e7636_d_n5;
        locals.var_pparam_b4soivearly_dn6 = assign10530_e7636_d_n6;
        locals.var_pparam_b4soivearly_dn7 = assign10530_e7636_d_n7;
        locals.var_pparam_b4soivearly_dn8 = assign10530_e7636_d_n8;
        locals.var_pparam_b4soivearly_dn9 = assign10530_e7636_d_n9;
        locals.var_pparam_b4soivearly_dn10 = assign10530_e7636_d_n10;
        locals.var_pparam_b4soivearly_dn11 = assign10530_e7636_d_n11;
        locals.var_pparam_b4soivearly_dn12 = assign10530_e7636_d_n12;

        let assign10540_e7639: f64 = if locals.var_b4soimtrlmod == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard940 = assign10540_e7639;

        let (assign10550_e7645, assign10550_e7645_d_n3, assign10550_e7645_d_n4, assign10550_e7645_d_n5, assign10550_e7645_d_n6, assign10550_e7645_d_n7, assign10550_e7645_d_n8, assign10550_e7645_d_n9, assign10550_e7645_d_n10, assign10550_e7645_d_n11, assign10550_e7645_d_n12,) = {
    if (locals.var_guard940 != 0.0) {
        let assign10550_e7643: f64 = (locals.var_b4soitox - locals.var_b4soidtoxcv);
        (assign10550_e7643, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soitoxp, locals.var_b4soitoxp_dn3, locals.var_b4soitoxp_dn4, locals.var_b4soitoxp_dn5, locals.var_b4soitoxp_dn6, locals.var_b4soitoxp_dn7, locals.var_b4soitoxp_dn8, locals.var_b4soitoxp_dn9, locals.var_b4soitoxp_dn10, locals.var_b4soitoxp_dn11, locals.var_b4soitoxp_dn12,)
    }
};
        locals.var_b4soitoxp = assign10550_e7645;
        locals.var_b4soitoxp_dn3 = assign10550_e7645_d_n3;
        locals.var_b4soitoxp_dn4 = assign10550_e7645_d_n4;
        locals.var_b4soitoxp_dn5 = assign10550_e7645_d_n5;
        locals.var_b4soitoxp_dn6 = assign10550_e7645_d_n6;
        locals.var_b4soitoxp_dn7 = assign10550_e7645_d_n7;
        locals.var_b4soitoxp_dn8 = assign10550_e7645_d_n8;
        locals.var_b4soitoxp_dn9 = assign10550_e7645_d_n9;
        locals.var_b4soitoxp_dn10 = assign10550_e7645_d_n10;
        locals.var_b4soitoxp_dn11 = assign10550_e7645_d_n11;
        locals.var_b4soitoxp_dn12 = assign10550_e7645_d_n12;

        let (assign10560_e7652,) = {
    if (locals.var_guard940 == 0.0) {
        let assign10560_e7650: f64 = (8.617087e-5 * locals.var_b4soitempeot);
        (assign10560_e7650,)
    } else {
        (locals.var_vtm0eot,)
    }
};
        locals.var_vtm0eot = assign10560_e7652;

        let (assign10570_e7657,) = {
    if (locals.var_guard940 == 0.0) {
        (locals.var_vtm0eot,)
    } else {
        (locals.var_vtmeot,)
    }
};
        locals.var_vtmeot = assign10570_e7657;

        let (assign10580_e7683, assign10580_e7683_d_n3, assign10580_e7683_d_n4, assign10580_e7683_d_n5, assign10580_e7683_d_n6, assign10580_e7683_d_n7, assign10580_e7683_d_n8, assign10580_e7683_d_n9, assign10580_e7683_d_n10, assign10580_e7683_d_n11, assign10580_e7683_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign10580_e7663: f64 = (1e20 * locals.var_pparam_b4soinpeak);
        let assign10580_e7666: f64 = (locals.var_ni * locals.var_ni);
        let assign10580_e7667: f64 = (assign10580_e7663 / assign10580_e7666);
        let (assign10580_e7680, assign10580_e7680_d_n3, assign10580_e7680_d_n4, assign10580_e7680_d_n5, assign10580_e7680_d_n6, assign10580_e7680_d_n7, assign10580_e7680_d_n8, assign10580_e7680_d_n9, assign10580_e7680_d_n10, assign10580_e7680_d_n11, assign10580_e7680_d_n12,) = {
            if (assign10580_e7667 > 1e-38) {
                let assign10580_e7672: f64 = (1e20 * locals.var_pparam_b4soinpeak);
                let assign10580_e7675: f64 = (locals.var_ni * locals.var_ni);
                let assign10580_e7676: f64 = (assign10580_e7672 / assign10580_e7675);
                let assign10580_e7677: f64 = (assign10580_e7676).ln();
                (assign10580_e7677, (((1e20 * locals.var_pparam_b4soinpeak_dn3) / assign10580_e7675) / assign10580_e7676), (((1e20 * locals.var_pparam_b4soinpeak_dn4) / assign10580_e7675) / assign10580_e7676), (((1e20 * locals.var_pparam_b4soinpeak_dn5) / assign10580_e7675) / assign10580_e7676), (((((1e20 * locals.var_pparam_b4soinpeak_dn6) * assign10580_e7675) - (assign10580_e7672 * ((locals.var_ni_dn6 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn6)))) / (assign10580_e7675 * assign10580_e7675)) / assign10580_e7676), (((1e20 * locals.var_pparam_b4soinpeak_dn7) / assign10580_e7675) / assign10580_e7676), (((1e20 * locals.var_pparam_b4soinpeak_dn8) / assign10580_e7675) / assign10580_e7676), (((1e20 * locals.var_pparam_b4soinpeak_dn9) / assign10580_e7675) / assign10580_e7676), (((1e20 * locals.var_pparam_b4soinpeak_dn10) / assign10580_e7675) / assign10580_e7676), (((1e20 * locals.var_pparam_b4soinpeak_dn11) / assign10580_e7675) / assign10580_e7676), (((1e20 * locals.var_pparam_b4soinpeak_dn12) / assign10580_e7675) / assign10580_e7676),)
            } else {
                let assign10580_e7679: f64 = (-87.49823353377374);
                (assign10580_e7679, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign10580_e7681: f64 = (locals.var_vtm0eot * assign10580_e7680);
        (assign10580_e7681, (locals.var_vtm0eot * assign10580_e7680_d_n3), (locals.var_vtm0eot * assign10580_e7680_d_n4), (locals.var_vtm0eot * assign10580_e7680_d_n5), (locals.var_vtm0eot * assign10580_e7680_d_n6), (locals.var_vtm0eot * assign10580_e7680_d_n7), (locals.var_vtm0eot * assign10580_e7680_d_n8), (locals.var_vtm0eot * assign10580_e7680_d_n9), (locals.var_vtm0eot * assign10580_e7680_d_n10), (locals.var_vtm0eot * assign10580_e7680_d_n11), (locals.var_vtm0eot * assign10580_e7680_d_n12),)
    } else {
        (locals.var_vbieot, locals.var_vbieot_dn3, locals.var_vbieot_dn4, locals.var_vbieot_dn5, locals.var_vbieot_dn6, locals.var_vbieot_dn7, locals.var_vbieot_dn8, locals.var_vbieot_dn9, locals.var_vbieot_dn10, locals.var_vbieot_dn11, locals.var_vbieot_dn12,)
    }
};
        locals.var_vbieot = assign10580_e7683;
        locals.var_vbieot_dn3 = assign10580_e7683_d_n3;
        locals.var_vbieot_dn4 = assign10580_e7683_d_n4;
        locals.var_vbieot_dn5 = assign10580_e7683_d_n5;
        locals.var_vbieot_dn6 = assign10580_e7683_d_n6;
        locals.var_vbieot_dn7 = assign10580_e7683_d_n7;
        locals.var_vbieot_dn8 = assign10580_e7683_d_n8;
        locals.var_vbieot_dn9 = assign10580_e7683_d_n9;
        locals.var_vbieot_dn10 = assign10580_e7683_d_n10;
        locals.var_vbieot_dn11 = assign10580_e7683_d_n11;
        locals.var_vbieot_dn12 = assign10580_e7683_d_n12;

        let (assign10590_e7703, assign10590_e7703_d_n3, assign10590_e7703_d_n4, assign10590_e7703_d_n5, assign10590_e7703_d_n6, assign10590_e7703_d_n7, assign10590_e7703_d_n8, assign10590_e7703_d_n9, assign10590_e7703_d_n10, assign10590_e7703_d_n11, assign10590_e7703_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign10590_e7688: f64 = (2.0 * locals.var_vtm0eot);
        let assign10590_e7691: f64 = (locals.var_pparam_b4soinpeak / locals.var_ni);
        let (assign10590_e7700, assign10590_e7700_d_n3, assign10590_e7700_d_n4, assign10590_e7700_d_n5, assign10590_e7700_d_n6, assign10590_e7700_d_n7, assign10590_e7700_d_n8, assign10590_e7700_d_n9, assign10590_e7700_d_n10, assign10590_e7700_d_n11, assign10590_e7700_d_n12,) = {
            if (assign10590_e7691 > 1e-38) {
                let assign10590_e7696: f64 = (locals.var_pparam_b4soinpeak / locals.var_ni);
                let assign10590_e7697: f64 = (assign10590_e7696).ln();
                (assign10590_e7697, ((locals.var_pparam_b4soinpeak_dn3 / locals.var_ni) / assign10590_e7696), ((locals.var_pparam_b4soinpeak_dn4 / locals.var_ni) / assign10590_e7696), ((locals.var_pparam_b4soinpeak_dn5 / locals.var_ni) / assign10590_e7696), ((((locals.var_pparam_b4soinpeak_dn6 * locals.var_ni) - (locals.var_pparam_b4soinpeak * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) / assign10590_e7696), ((locals.var_pparam_b4soinpeak_dn7 / locals.var_ni) / assign10590_e7696), ((locals.var_pparam_b4soinpeak_dn8 / locals.var_ni) / assign10590_e7696), ((locals.var_pparam_b4soinpeak_dn9 / locals.var_ni) / assign10590_e7696), ((locals.var_pparam_b4soinpeak_dn10 / locals.var_ni) / assign10590_e7696), ((locals.var_pparam_b4soinpeak_dn11 / locals.var_ni) / assign10590_e7696), ((locals.var_pparam_b4soinpeak_dn12 / locals.var_ni) / assign10590_e7696),)
            } else {
                let assign10590_e7699: f64 = (-87.49823353377374);
                (assign10590_e7699, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign10590_e7701: f64 = (assign10590_e7688 * assign10590_e7700);
        (assign10590_e7701, (assign10590_e7688 * assign10590_e7700_d_n3), (assign10590_e7688 * assign10590_e7700_d_n4), (assign10590_e7688 * assign10590_e7700_d_n5), (assign10590_e7688 * assign10590_e7700_d_n6), (assign10590_e7688 * assign10590_e7700_d_n7), (assign10590_e7688 * assign10590_e7700_d_n8), (assign10590_e7688 * assign10590_e7700_d_n9), (assign10590_e7688 * assign10590_e7700_d_n10), (assign10590_e7688 * assign10590_e7700_d_n11), (assign10590_e7688 * assign10590_e7700_d_n12),)
    } else {
        (locals.var_phieot, locals.var_phieot_dn3, locals.var_phieot_dn4, locals.var_phieot_dn5, locals.var_phieot_dn6, locals.var_phieot_dn7, locals.var_phieot_dn8, locals.var_phieot_dn9, locals.var_phieot_dn10, locals.var_phieot_dn11, locals.var_phieot_dn12,)
    }
};
        locals.var_phieot = assign10590_e7703;
        locals.var_phieot_dn3 = assign10590_e7703_d_n3;
        locals.var_phieot_dn4 = assign10590_e7703_d_n4;
        locals.var_phieot_dn5 = assign10590_e7703_d_n5;
        locals.var_phieot_dn6 = assign10590_e7703_d_n6;
        locals.var_phieot_dn7 = assign10590_e7703_d_n7;
        locals.var_phieot_dn8 = assign10590_e7703_d_n8;
        locals.var_phieot_dn9 = assign10590_e7703_d_n9;
        locals.var_phieot_dn10 = assign10590_e7703_d_n10;
        locals.var_phieot_dn11 = assign10590_e7703_d_n11;
        locals.var_phieot_dn12 = assign10590_e7703_d_n12;

        let (assign10600_e7709, assign10600_e7709_d_n3, assign10600_e7709_d_n4, assign10600_e7709_d_n5, assign10600_e7709_d_n6, assign10600_e7709_d_n7, assign10600_e7709_d_n8, assign10600_e7709_d_n9, assign10600_e7709_d_n10, assign10600_e7709_d_n11, assign10600_e7709_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign10600_e7707: f64 = (locals.var_phieot).sqrt();
        (assign10600_e7707, (locals.var_phieot_dn3 / (2.0 * assign10600_e7707)), (locals.var_phieot_dn4 / (2.0 * assign10600_e7707)), (locals.var_phieot_dn5 / (2.0 * assign10600_e7707)), (locals.var_phieot_dn6 / (2.0 * assign10600_e7707)), (locals.var_phieot_dn7 / (2.0 * assign10600_e7707)), (locals.var_phieot_dn8 / (2.0 * assign10600_e7707)), (locals.var_phieot_dn9 / (2.0 * assign10600_e7707)), (locals.var_phieot_dn10 / (2.0 * assign10600_e7707)), (locals.var_phieot_dn11 / (2.0 * assign10600_e7707)), (locals.var_phieot_dn12 / (2.0 * assign10600_e7707)),)
    } else {
        (locals.var_sqrtphieot, locals.var_sqrtphieot_dn3, locals.var_sqrtphieot_dn4, locals.var_sqrtphieot_dn5, locals.var_sqrtphieot_dn6, locals.var_sqrtphieot_dn7, locals.var_sqrtphieot_dn8, locals.var_sqrtphieot_dn9, locals.var_sqrtphieot_dn10, locals.var_sqrtphieot_dn11, locals.var_sqrtphieot_dn12,)
    }
};
        locals.var_sqrtphieot = assign10600_e7709;
        locals.var_sqrtphieot_dn3 = assign10600_e7709_d_n3;
        locals.var_sqrtphieot_dn4 = assign10600_e7709_d_n4;
        locals.var_sqrtphieot_dn5 = assign10600_e7709_d_n5;
        locals.var_sqrtphieot_dn6 = assign10600_e7709_d_n6;
        locals.var_sqrtphieot_dn7 = assign10600_e7709_d_n7;
        locals.var_sqrtphieot_dn8 = assign10600_e7709_d_n8;
        locals.var_sqrtphieot_dn9 = assign10600_e7709_d_n9;
        locals.var_sqrtphieot_dn10 = assign10600_e7709_d_n10;
        locals.var_sqrtphieot_dn11 = assign10600_e7709_d_n11;
        locals.var_sqrtphieot_dn12 = assign10600_e7709_d_n12;

        let (assign10610_e7716, assign10610_e7716_d_n3, assign10610_e7716_d_n4, assign10610_e7716_d_n5, assign10610_e7716_d_n6, assign10610_e7716_d_n7, assign10610_e7716_d_n8, assign10610_e7716_d_n9, assign10610_e7716_d_n10, assign10610_e7716_d_n11, assign10610_e7716_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign10610_e7714: f64 = (locals.var_here_b4soivfb + locals.var_phieot);
        (assign10610_e7714, (locals.var_here_b4soivfb_dn3 + locals.var_phieot_dn3), (locals.var_here_b4soivfb_dn4 + locals.var_phieot_dn4), (locals.var_here_b4soivfb_dn5 + locals.var_phieot_dn5), (locals.var_here_b4soivfb_dn6 + locals.var_phieot_dn6), (locals.var_here_b4soivfb_dn7 + locals.var_phieot_dn7), (locals.var_here_b4soivfb_dn8 + locals.var_phieot_dn8), (locals.var_here_b4soivfb_dn9 + locals.var_phieot_dn9), (locals.var_here_b4soivfb_dn10 + locals.var_phieot_dn10), (locals.var_here_b4soivfb_dn11 + locals.var_phieot_dn11), (locals.var_here_b4soivfb_dn12 + locals.var_phieot_dn12),)
    } else {
        (locals.var_tmp2, locals.var_tmp2_dn3, locals.var_tmp2_dn4, locals.var_tmp2_dn5, locals.var_tmp2_dn6, locals.var_tmp2_dn7, locals.var_tmp2_dn8, locals.var_tmp2_dn9, locals.var_tmp2_dn10, locals.var_tmp2_dn11, locals.var_tmp2_dn12,)
    }
};
        locals.var_tmp2 = assign10610_e7716;
        locals.var_tmp2_dn3 = assign10610_e7716_d_n3;
        locals.var_tmp2_dn4 = assign10610_e7716_d_n4;
        locals.var_tmp2_dn5 = assign10610_e7716_d_n5;
        locals.var_tmp2_dn6 = assign10610_e7716_d_n6;
        locals.var_tmp2_dn7 = assign10610_e7716_d_n7;
        locals.var_tmp2_dn8 = assign10610_e7716_d_n8;
        locals.var_tmp2_dn9 = assign10610_e7716_d_n9;
        locals.var_tmp2_dn10 = assign10610_e7716_d_n10;
        locals.var_tmp2_dn11 = assign10610_e7716_d_n11;
        locals.var_tmp2_dn12 = assign10610_e7716_d_n12;

        let (assign10620_e7723,) = {
    if (locals.var_guard940 == 0.0) {
        let assign10620_e7721: f64 = (locals.var_b4soitype * locals.var_b4soivddeot);
        (assign10620_e7721,)
    } else {
        (locals.var_vddeot,)
    }
};
        locals.var_vddeot = assign10620_e7723;

        let (assign10630_e7730, assign10630_e7730_d_n3, assign10630_e7730_d_n4, assign10630_e7730_d_n5, assign10630_e7730_d_n6, assign10630_e7730_d_n7, assign10630_e7730_d_n8, assign10630_e7730_d_n9, assign10630_e7730_d_n10, assign10630_e7730_d_n11, assign10630_e7730_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign10630_e7728: f64 = (locals.var_b4soiepsrgate * 8.85418e-12);
        (assign10630_e7728, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign10630_e7730;
        locals.var_t0_dn3 = assign10630_e7730_d_n3;
        locals.var_t0_dn4 = assign10630_e7730_d_n4;
        locals.var_t0_dn5 = assign10630_e7730_d_n5;
        locals.var_t0_dn6 = assign10630_e7730_d_n6;
        locals.var_t0_dn7 = assign10630_e7730_d_n7;
        locals.var_t0_dn8 = assign10630_e7730_d_n8;
        locals.var_t0_dn9 = assign10630_e7730_d_n9;
        locals.var_t0_dn10 = assign10630_e7730_d_n10;
        locals.var_t0_dn11 = assign10630_e7730_d_n11;
        locals.var_t0_dn12 = assign10630_e7730_d_n12;

        let assign10640_e7745: f64 = if ((((locals.var_pparam_b4soingate > 1e18) && (locals.var_pparam_b4soingate < 1e25)) && (locals.var_vddeot > locals.var_tmp2)) && (locals.var_t0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard941 = assign10640_e7745;

        let (assign10650_e7762, assign10650_e7762_d_n3, assign10650_e7762_d_n4, assign10650_e7762_d_n5, assign10650_e7762_d_n6, assign10650_e7762_d_n7, assign10650_e7762_d_n8, assign10650_e7762_d_n9, assign10650_e7762_d_n10, assign10650_e7762_d_n11, assign10650_e7762_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard941 != 0.0)) {
        let assign10650_e7752: f64 = (1000000.0 * 1.60219e-19);
        let assign10650_e7754: f64 = (assign10650_e7752 * locals.var_epssub);
        let assign10650_e7756: f64 = (assign10650_e7754 * locals.var_pparam_b4soingate);
        let assign10650_e7759: f64 = (locals.var_b4soicox * locals.var_b4soicox);
        let assign10650_e7760: f64 = (assign10650_e7756 / assign10650_e7759);
        (assign10650_e7760, ((assign10650_e7754 * locals.var_pparam_b4soingate_dn3) / assign10650_e7759), ((assign10650_e7754 * locals.var_pparam_b4soingate_dn4) / assign10650_e7759), ((assign10650_e7754 * locals.var_pparam_b4soingate_dn5) / assign10650_e7759), ((assign10650_e7754 * locals.var_pparam_b4soingate_dn6) / assign10650_e7759), ((assign10650_e7754 * locals.var_pparam_b4soingate_dn7) / assign10650_e7759), ((assign10650_e7754 * locals.var_pparam_b4soingate_dn8) / assign10650_e7759), ((assign10650_e7754 * locals.var_pparam_b4soingate_dn9) / assign10650_e7759), ((assign10650_e7754 * locals.var_pparam_b4soingate_dn10) / assign10650_e7759), ((assign10650_e7754 * locals.var_pparam_b4soingate_dn11) / assign10650_e7759), ((assign10650_e7754 * locals.var_pparam_b4soingate_dn12) / assign10650_e7759),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign10650_e7762;
        locals.var_t1_dn3 = assign10650_e7762_d_n3;
        locals.var_t1_dn4 = assign10650_e7762_d_n4;
        locals.var_t1_dn5 = assign10650_e7762_d_n5;
        locals.var_t1_dn6 = assign10650_e7762_d_n6;
        locals.var_t1_dn7 = assign10650_e7762_d_n7;
        locals.var_t1_dn8 = assign10650_e7762_d_n8;
        locals.var_t1_dn9 = assign10650_e7762_d_n9;
        locals.var_t1_dn10 = assign10650_e7762_d_n10;
        locals.var_t1_dn11 = assign10650_e7762_d_n11;
        locals.var_t1_dn12 = assign10650_e7762_d_n12;

    }

    pub(super) fn stamp_transient_block_21(
        locals: &mut StampLocals,
    ) {
        let (assign10660_e7778, assign10660_e7778_d_n3, assign10660_e7778_d_n4, assign10660_e7778_d_n5, assign10660_e7778_d_n6, assign10660_e7778_d_n7, assign10660_e7778_d_n8, assign10660_e7778_d_n9, assign10660_e7778_d_n10, assign10660_e7778_d_n11, assign10660_e7778_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard941 != 0.0)) {
        let assign10660_e7771: f64 = (locals.var_vddeot - locals.var_t0);
        let assign10660_e7772: f64 = (2.0 * assign10660_e7771);
        let assign10660_e7774: f64 = (assign10660_e7772 / locals.var_t1);
        let assign10660_e7775: f64 = (1.0 + assign10660_e7774);
        let assign10660_e7776: f64 = (assign10660_e7775).sqrt();
        (assign10660_e7776, (((((2.0 * (-locals.var_t0_dn3)) * locals.var_t1) - (assign10660_e7772 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign10660_e7776)), (((((2.0 * (-locals.var_t0_dn4)) * locals.var_t1) - (assign10660_e7772 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign10660_e7776)), (((((2.0 * (-locals.var_t0_dn5)) * locals.var_t1) - (assign10660_e7772 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign10660_e7776)), (((((2.0 * (-locals.var_t0_dn6)) * locals.var_t1) - (assign10660_e7772 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign10660_e7776)), (((((2.0 * (-locals.var_t0_dn7)) * locals.var_t1) - (assign10660_e7772 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign10660_e7776)), (((((2.0 * (-locals.var_t0_dn8)) * locals.var_t1) - (assign10660_e7772 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign10660_e7776)), (((((2.0 * (-locals.var_t0_dn9)) * locals.var_t1) - (assign10660_e7772 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign10660_e7776)), (((((2.0 * (-locals.var_t0_dn10)) * locals.var_t1) - (assign10660_e7772 * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign10660_e7776)), (((((2.0 * (-locals.var_t0_dn11)) * locals.var_t1) - (assign10660_e7772 * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign10660_e7776)), (((((2.0 * (-locals.var_t0_dn12)) * locals.var_t1) - (assign10660_e7772 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign10660_e7776)),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign10660_e7778;
        locals.var_t4_dn3 = assign10660_e7778_d_n3;
        locals.var_t4_dn4 = assign10660_e7778_d_n4;
        locals.var_t4_dn5 = assign10660_e7778_d_n5;
        locals.var_t4_dn6 = assign10660_e7778_d_n6;
        locals.var_t4_dn7 = assign10660_e7778_d_n7;
        locals.var_t4_dn8 = assign10660_e7778_d_n8;
        locals.var_t4_dn9 = assign10660_e7778_d_n9;
        locals.var_t4_dn10 = assign10660_e7778_d_n10;
        locals.var_t4_dn11 = assign10660_e7778_d_n11;
        locals.var_t4_dn12 = assign10660_e7778_d_n12;

        let (assign10670_e7789, assign10670_e7789_d_n3, assign10670_e7789_d_n4, assign10670_e7789_d_n5, assign10670_e7789_d_n6, assign10670_e7789_d_n7, assign10670_e7789_d_n8, assign10670_e7789_d_n9, assign10670_e7789_d_n10, assign10670_e7789_d_n11, assign10670_e7789_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard941 != 0.0)) {
        let assign10670_e7786: f64 = (locals.var_t4 - 1.0);
        let assign10670_e7787: f64 = (locals.var_t1 * assign10670_e7786);
        (assign10670_e7787, ((locals.var_t1_dn3 * assign10670_e7786) + (locals.var_t1 * locals.var_t4_dn3)), ((locals.var_t1_dn4 * assign10670_e7786) + (locals.var_t1 * locals.var_t4_dn4)), ((locals.var_t1_dn5 * assign10670_e7786) + (locals.var_t1 * locals.var_t4_dn5)), ((locals.var_t1_dn6 * assign10670_e7786) + (locals.var_t1 * locals.var_t4_dn6)), ((locals.var_t1_dn7 * assign10670_e7786) + (locals.var_t1 * locals.var_t4_dn7)), ((locals.var_t1_dn8 * assign10670_e7786) + (locals.var_t1 * locals.var_t4_dn8)), ((locals.var_t1_dn9 * assign10670_e7786) + (locals.var_t1 * locals.var_t4_dn9)), ((locals.var_t1_dn10 * assign10670_e7786) + (locals.var_t1 * locals.var_t4_dn10)), ((locals.var_t1_dn11 * assign10670_e7786) + (locals.var_t1 * locals.var_t4_dn11)), ((locals.var_t1_dn12 * assign10670_e7786) + (locals.var_t1 * locals.var_t4_dn12)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign10670_e7789;
        locals.var_t2_dn3 = assign10670_e7789_d_n3;
        locals.var_t2_dn4 = assign10670_e7789_d_n4;
        locals.var_t2_dn5 = assign10670_e7789_d_n5;
        locals.var_t2_dn6 = assign10670_e7789_d_n6;
        locals.var_t2_dn7 = assign10670_e7789_d_n7;
        locals.var_t2_dn8 = assign10670_e7789_d_n8;
        locals.var_t2_dn9 = assign10670_e7789_d_n9;
        locals.var_t2_dn10 = assign10670_e7789_d_n10;
        locals.var_t2_dn11 = assign10670_e7789_d_n11;
        locals.var_t2_dn12 = assign10670_e7789_d_n12;

        let (assign10680_e7802, assign10680_e7802_d_n3, assign10680_e7802_d_n4, assign10680_e7802_d_n5, assign10680_e7802_d_n6, assign10680_e7802_d_n7, assign10680_e7802_d_n8, assign10680_e7802_d_n9, assign10680_e7802_d_n10, assign10680_e7802_d_n11, assign10680_e7802_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard941 != 0.0)) {
        let assign10680_e7796: f64 = (0.5 * locals.var_t2);
        let assign10680_e7798: f64 = (assign10680_e7796 * locals.var_t2);
        let assign10680_e7800: f64 = (assign10680_e7798 / locals.var_t1);
        (assign10680_e7800, ((((((0.5 * locals.var_t2_dn3) * locals.var_t2) + (assign10680_e7796 * locals.var_t2_dn3)) * locals.var_t1) - (assign10680_e7798 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), ((((((0.5 * locals.var_t2_dn4) * locals.var_t2) + (assign10680_e7796 * locals.var_t2_dn4)) * locals.var_t1) - (assign10680_e7798 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), ((((((0.5 * locals.var_t2_dn5) * locals.var_t2) + (assign10680_e7796 * locals.var_t2_dn5)) * locals.var_t1) - (assign10680_e7798 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), ((((((0.5 * locals.var_t2_dn6) * locals.var_t2) + (assign10680_e7796 * locals.var_t2_dn6)) * locals.var_t1) - (assign10680_e7798 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), ((((((0.5 * locals.var_t2_dn7) * locals.var_t2) + (assign10680_e7796 * locals.var_t2_dn7)) * locals.var_t1) - (assign10680_e7798 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), ((((((0.5 * locals.var_t2_dn8) * locals.var_t2) + (assign10680_e7796 * locals.var_t2_dn8)) * locals.var_t1) - (assign10680_e7798 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), ((((((0.5 * locals.var_t2_dn9) * locals.var_t2) + (assign10680_e7796 * locals.var_t2_dn9)) * locals.var_t1) - (assign10680_e7798 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), ((((((0.5 * locals.var_t2_dn10) * locals.var_t2) + (assign10680_e7796 * locals.var_t2_dn10)) * locals.var_t1) - (assign10680_e7798 * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), ((((((0.5 * locals.var_t2_dn11) * locals.var_t2) + (assign10680_e7796 * locals.var_t2_dn11)) * locals.var_t1) - (assign10680_e7798 * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)), ((((((0.5 * locals.var_t2_dn12) * locals.var_t2) + (assign10680_e7796 * locals.var_t2_dn12)) * locals.var_t1) - (assign10680_e7798 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign10680_e7802;
        locals.var_t3_dn3 = assign10680_e7802_d_n3;
        locals.var_t3_dn4 = assign10680_e7802_d_n4;
        locals.var_t3_dn5 = assign10680_e7802_d_n5;
        locals.var_t3_dn6 = assign10680_e7802_d_n6;
        locals.var_t3_dn7 = assign10680_e7802_d_n7;
        locals.var_t3_dn8 = assign10680_e7802_d_n8;
        locals.var_t3_dn9 = assign10680_e7802_d_n9;
        locals.var_t3_dn10 = assign10680_e7802_d_n10;
        locals.var_t3_dn11 = assign10680_e7802_d_n11;
        locals.var_t3_dn12 = assign10680_e7802_d_n12;

        let (assign10690_e7813, assign10690_e7813_d_n3, assign10690_e7813_d_n4, assign10690_e7813_d_n5, assign10690_e7813_d_n6, assign10690_e7813_d_n7, assign10690_e7813_d_n8, assign10690_e7813_d_n9, assign10690_e7813_d_n10, assign10690_e7813_d_n11, assign10690_e7813_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard941 != 0.0)) {
        let assign10690_e7809: f64 = (locals.var_eggdep - locals.var_t3);
        let assign10690_e7811: f64 = (assign10690_e7809 - 0.05);
        (assign10690_e7811, (-locals.var_t3_dn3), (-locals.var_t3_dn4), (-locals.var_t3_dn5), (-locals.var_t3_dn6), (-locals.var_t3_dn7), (-locals.var_t3_dn8), (-locals.var_t3_dn9), (-locals.var_t3_dn10), (-locals.var_t3_dn11), (-locals.var_t3_dn12),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12,)
    }
};
        locals.var_t7 = assign10690_e7813;
        locals.var_t7_dn3 = assign10690_e7813_d_n3;
        locals.var_t7_dn4 = assign10690_e7813_d_n4;
        locals.var_t7_dn5 = assign10690_e7813_d_n5;
        locals.var_t7_dn6 = assign10690_e7813_d_n6;
        locals.var_t7_dn7 = assign10690_e7813_d_n7;
        locals.var_t7_dn8 = assign10690_e7813_d_n8;
        locals.var_t7_dn9 = assign10690_e7813_d_n9;
        locals.var_t7_dn10 = assign10690_e7813_d_n10;
        locals.var_t7_dn11 = assign10690_e7813_d_n11;
        locals.var_t7_dn12 = assign10690_e7813_d_n12;

        let (assign10700_e7825, assign10700_e7825_d_n3, assign10700_e7825_d_n4, assign10700_e7825_d_n5, assign10700_e7825_d_n6, assign10700_e7825_d_n7, assign10700_e7825_d_n8, assign10700_e7825_d_n9, assign10700_e7825_d_n10, assign10700_e7825_d_n11, assign10700_e7825_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard941 != 0.0)) {
        let assign10700_e7820: f64 = (locals.var_t7 * locals.var_t7);
        let assign10700_e7822: f64 = (assign10700_e7820 + 0.224);
        let assign10700_e7823: f64 = (assign10700_e7822).sqrt();
        (assign10700_e7823, (((locals.var_t7_dn3 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn3)) / (2.0 * assign10700_e7823)), (((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)) / (2.0 * assign10700_e7823)), (((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)) / (2.0 * assign10700_e7823)), (((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)) / (2.0 * assign10700_e7823)), (((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)) / (2.0 * assign10700_e7823)), (((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)) / (2.0 * assign10700_e7823)), (((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)) / (2.0 * assign10700_e7823)), (((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)) / (2.0 * assign10700_e7823)), (((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)) / (2.0 * assign10700_e7823)), (((locals.var_t7_dn12 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn12)) / (2.0 * assign10700_e7823)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign10700_e7825;
        locals.var_t6_dn3 = assign10700_e7825_d_n3;
        locals.var_t6_dn4 = assign10700_e7825_d_n4;
        locals.var_t6_dn5 = assign10700_e7825_d_n5;
        locals.var_t6_dn6 = assign10700_e7825_d_n6;
        locals.var_t6_dn7 = assign10700_e7825_d_n7;
        locals.var_t6_dn8 = assign10700_e7825_d_n8;
        locals.var_t6_dn9 = assign10700_e7825_d_n9;
        locals.var_t6_dn10 = assign10700_e7825_d_n10;
        locals.var_t6_dn11 = assign10700_e7825_d_n11;
        locals.var_t6_dn12 = assign10700_e7825_d_n12;

        let (assign10710_e7838, assign10710_e7838_d_n3, assign10710_e7838_d_n4, assign10710_e7838_d_n5, assign10710_e7838_d_n6, assign10710_e7838_d_n7, assign10710_e7838_d_n8, assign10710_e7838_d_n9, assign10710_e7838_d_n10, assign10710_e7838_d_n11, assign10710_e7838_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard941 != 0.0)) {
        let assign10710_e7834: f64 = (locals.var_t7 + locals.var_t6);
        let assign10710_e7835: f64 = (0.5 * assign10710_e7834);
        let assign10710_e7836: f64 = (locals.var_eggdep - assign10710_e7835);
        (assign10710_e7836, (-(0.5 * (locals.var_t7_dn3 + locals.var_t6_dn3))), (-(0.5 * (locals.var_t7_dn4 + locals.var_t6_dn4))), (-(0.5 * (locals.var_t7_dn5 + locals.var_t6_dn5))), (-(0.5 * (locals.var_t7_dn6 + locals.var_t6_dn6))), (-(0.5 * (locals.var_t7_dn7 + locals.var_t6_dn7))), (-(0.5 * (locals.var_t7_dn8 + locals.var_t6_dn8))), (-(0.5 * (locals.var_t7_dn9 + locals.var_t6_dn9))), (-(0.5 * (locals.var_t7_dn10 + locals.var_t6_dn10))), (-(0.5 * (locals.var_t7_dn11 + locals.var_t6_dn11))), (-(0.5 * (locals.var_t7_dn12 + locals.var_t6_dn12))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign10710_e7838;
        locals.var_t5_dn3 = assign10710_e7838_d_n3;
        locals.var_t5_dn4 = assign10710_e7838_d_n4;
        locals.var_t5_dn5 = assign10710_e7838_d_n5;
        locals.var_t5_dn6 = assign10710_e7838_d_n6;
        locals.var_t5_dn7 = assign10710_e7838_d_n7;
        locals.var_t5_dn8 = assign10710_e7838_d_n8;
        locals.var_t5_dn9 = assign10710_e7838_d_n9;
        locals.var_t5_dn10 = assign10710_e7838_d_n10;
        locals.var_t5_dn11 = assign10710_e7838_d_n11;
        locals.var_t5_dn12 = assign10710_e7838_d_n12;

        let (assign10720_e7847, assign10720_e7847_d_n3, assign10720_e7847_d_n4, assign10720_e7847_d_n5, assign10720_e7847_d_n6, assign10720_e7847_d_n7, assign10720_e7847_d_n8, assign10720_e7847_d_n9, assign10720_e7847_d_n10, assign10720_e7847_d_n11, assign10720_e7847_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard941 != 0.0)) {
        let assign10720_e7845: f64 = (locals.var_vddeot - locals.var_t5);
        (assign10720_e7845, (-locals.var_t5_dn3), (-locals.var_t5_dn4), (-locals.var_t5_dn5), (-locals.var_t5_dn6), (-locals.var_t5_dn7), (-locals.var_t5_dn8), (-locals.var_t5_dn9), (-locals.var_t5_dn10), (-locals.var_t5_dn11), (-locals.var_t5_dn12),)
    } else {
        (locals.var_vgs_eff, locals.var_vgs_eff_dn3, locals.var_vgs_eff_dn4, locals.var_vgs_eff_dn5, locals.var_vgs_eff_dn6, locals.var_vgs_eff_dn7, locals.var_vgs_eff_dn8, locals.var_vgs_eff_dn9, locals.var_vgs_eff_dn10, locals.var_vgs_eff_dn11, locals.var_vgs_eff_dn12,)
    }
};
        locals.var_vgs_eff = assign10720_e7847;
        locals.var_vgs_eff_dn3 = assign10720_e7847_d_n3;
        locals.var_vgs_eff_dn4 = assign10720_e7847_d_n4;
        locals.var_vgs_eff_dn5 = assign10720_e7847_d_n5;
        locals.var_vgs_eff_dn6 = assign10720_e7847_d_n6;
        locals.var_vgs_eff_dn7 = assign10720_e7847_d_n7;
        locals.var_vgs_eff_dn8 = assign10720_e7847_d_n8;
        locals.var_vgs_eff_dn9 = assign10720_e7847_d_n9;
        locals.var_vgs_eff_dn10 = assign10720_e7847_d_n10;
        locals.var_vgs_eff_dn11 = assign10720_e7847_d_n11;
        locals.var_vgs_eff_dn12 = assign10720_e7847_d_n12;

        let (assign10730_e7855, assign10730_e7855_d_n3, assign10730_e7855_d_n4, assign10730_e7855_d_n5, assign10730_e7855_d_n6, assign10730_e7855_d_n7, assign10730_e7855_d_n8, assign10730_e7855_d_n9, assign10730_e7855_d_n10, assign10730_e7855_d_n11, assign10730_e7855_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard941 == 0.0)) {
        (locals.var_vddeot, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgs_eff, locals.var_vgs_eff_dn3, locals.var_vgs_eff_dn4, locals.var_vgs_eff_dn5, locals.var_vgs_eff_dn6, locals.var_vgs_eff_dn7, locals.var_vgs_eff_dn8, locals.var_vgs_eff_dn9, locals.var_vgs_eff_dn10, locals.var_vgs_eff_dn11, locals.var_vgs_eff_dn12,)
    }
};
        locals.var_vgs_eff = assign10730_e7855;
        locals.var_vgs_eff_dn3 = assign10730_e7855_d_n3;
        locals.var_vgs_eff_dn4 = assign10730_e7855_d_n4;
        locals.var_vgs_eff_dn5 = assign10730_e7855_d_n5;
        locals.var_vgs_eff_dn6 = assign10730_e7855_d_n6;
        locals.var_vgs_eff_dn7 = assign10730_e7855_d_n7;
        locals.var_vgs_eff_dn8 = assign10730_e7855_d_n8;
        locals.var_vgs_eff_dn9 = assign10730_e7855_d_n9;
        locals.var_vgs_eff_dn10 = assign10730_e7855_d_n10;
        locals.var_vgs_eff_dn11 = assign10730_e7855_d_n11;
        locals.var_vgs_eff_dn12 = assign10730_e7855_d_n12;

        let (assign10740_e7862, assign10740_e7862_d_n3, assign10740_e7862_d_n4, assign10740_e7862_d_n5, assign10740_e7862_d_n6, assign10740_e7862_d_n7, assign10740_e7862_d_n8, assign10740_e7862_d_n9, assign10740_e7862_d_n10, assign10740_e7862_d_n11, assign10740_e7862_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign10740_e7860: f64 = (locals.var_vbieot - locals.var_phieot);
        (assign10740_e7860, (locals.var_vbieot_dn3 - locals.var_phieot_dn3), (locals.var_vbieot_dn4 - locals.var_phieot_dn4), (locals.var_vbieot_dn5 - locals.var_phieot_dn5), (locals.var_vbieot_dn6 - locals.var_phieot_dn6), (locals.var_vbieot_dn7 - locals.var_phieot_dn7), (locals.var_vbieot_dn8 - locals.var_phieot_dn8), (locals.var_vbieot_dn9 - locals.var_phieot_dn9), (locals.var_vbieot_dn10 - locals.var_phieot_dn10), (locals.var_vbieot_dn11 - locals.var_phieot_dn11), (locals.var_vbieot_dn12 - locals.var_phieot_dn12),)
    } else {
        (locals.var_v0, locals.var_v0_dn3, locals.var_v0_dn4, locals.var_v0_dn5, locals.var_v0_dn6, locals.var_v0_dn7, locals.var_v0_dn8, locals.var_v0_dn9, locals.var_v0_dn10, locals.var_v0_dn11, locals.var_v0_dn12,)
    }
};
        locals.var_v0 = assign10740_e7862;
        locals.var_v0_dn3 = assign10740_e7862_d_n3;
        locals.var_v0_dn4 = assign10740_e7862_d_n4;
        locals.var_v0_dn5 = assign10740_e7862_d_n5;
        locals.var_v0_dn6 = assign10740_e7862_d_n6;
        locals.var_v0_dn7 = assign10740_e7862_d_n7;
        locals.var_v0_dn8 = assign10740_e7862_d_n8;
        locals.var_v0_dn9 = assign10740_e7862_d_n9;
        locals.var_v0_dn10 = assign10740_e7862_d_n10;
        locals.var_v0_dn11 = assign10740_e7862_d_n11;
        locals.var_v0_dn12 = assign10740_e7862_d_n12;

        let (assign10750_e7867, assign10750_e7867_d_n3, assign10750_e7867_d_n4, assign10750_e7867_d_n5, assign10750_e7867_d_n6, assign10750_e7867_d_n7, assign10750_e7867_d_n8, assign10750_e7867_d_n9, assign10750_e7867_d_n10, assign10750_e7867_d_n11, assign10750_e7867_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        (locals.var_pparam_b4soisqrtxdep0, locals.var_pparam_b4soisqrtxdep0_dn3, locals.var_pparam_b4soisqrtxdep0_dn4, locals.var_pparam_b4soisqrtxdep0_dn5, locals.var_pparam_b4soisqrtxdep0_dn6, locals.var_pparam_b4soisqrtxdep0_dn7, locals.var_pparam_b4soisqrtxdep0_dn8, locals.var_pparam_b4soisqrtxdep0_dn9, locals.var_pparam_b4soisqrtxdep0_dn10, locals.var_pparam_b4soisqrtxdep0_dn11, locals.var_pparam_b4soisqrtxdep0_dn12,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign10750_e7867;
        locals.var_t3_dn3 = assign10750_e7867_d_n3;
        locals.var_t3_dn4 = assign10750_e7867_d_n4;
        locals.var_t3_dn5 = assign10750_e7867_d_n5;
        locals.var_t3_dn6 = assign10750_e7867_d_n6;
        locals.var_t3_dn7 = assign10750_e7867_d_n7;
        locals.var_t3_dn8 = assign10750_e7867_d_n8;
        locals.var_t3_dn9 = assign10750_e7867_d_n9;
        locals.var_t3_dn10 = assign10750_e7867_d_n10;
        locals.var_t3_dn11 = assign10750_e7867_d_n11;
        locals.var_t3_dn12 = assign10750_e7867_d_n12;

        let (assign10760_e7874, assign10760_e7874_d_n3, assign10760_e7874_d_n4, assign10760_e7874_d_n5, assign10760_e7874_d_n6, assign10760_e7874_d_n7, assign10760_e7874_d_n8, assign10760_e7874_d_n9, assign10760_e7874_d_n10, assign10760_e7874_d_n11, assign10760_e7874_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign10760_e7872: f64 = (locals.var_b4soifactor1 * locals.var_t3);
        (assign10760_e7872, (locals.var_b4soifactor1 * locals.var_t3_dn3), (locals.var_b4soifactor1 * locals.var_t3_dn4), (locals.var_b4soifactor1 * locals.var_t3_dn5), (locals.var_b4soifactor1 * locals.var_t3_dn6), (locals.var_b4soifactor1 * locals.var_t3_dn7), (locals.var_b4soifactor1 * locals.var_t3_dn8), (locals.var_b4soifactor1 * locals.var_t3_dn9), (locals.var_b4soifactor1 * locals.var_t3_dn10), (locals.var_b4soifactor1 * locals.var_t3_dn11), (locals.var_b4soifactor1 * locals.var_t3_dn12),)
    } else {
        (locals.var_lt1, locals.var_lt1_dn3, locals.var_lt1_dn4, locals.var_lt1_dn5, locals.var_lt1_dn6, locals.var_lt1_dn7, locals.var_lt1_dn8, locals.var_lt1_dn9, locals.var_lt1_dn10, locals.var_lt1_dn11, locals.var_lt1_dn12,)
    }
};
        locals.var_lt1 = assign10760_e7874;
        locals.var_lt1_dn3 = assign10760_e7874_d_n3;
        locals.var_lt1_dn4 = assign10760_e7874_d_n4;
        locals.var_lt1_dn5 = assign10760_e7874_d_n5;
        locals.var_lt1_dn6 = assign10760_e7874_d_n6;
        locals.var_lt1_dn7 = assign10760_e7874_d_n7;
        locals.var_lt1_dn8 = assign10760_e7874_d_n8;
        locals.var_lt1_dn9 = assign10760_e7874_d_n9;
        locals.var_lt1_dn10 = assign10760_e7874_d_n10;
        locals.var_lt1_dn11 = assign10760_e7874_d_n11;
        locals.var_lt1_dn12 = assign10760_e7874_d_n12;

        let (assign10770_e7881, assign10770_e7881_d_n3, assign10770_e7881_d_n4, assign10770_e7881_d_n5, assign10770_e7881_d_n6, assign10770_e7881_d_n7, assign10770_e7881_d_n8, assign10770_e7881_d_n9, assign10770_e7881_d_n10, assign10770_e7881_d_n11, assign10770_e7881_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign10770_e7879: f64 = (locals.var_b4soifactor1 * locals.var_t3);
        (assign10770_e7879, (locals.var_b4soifactor1 * locals.var_t3_dn3), (locals.var_b4soifactor1 * locals.var_t3_dn4), (locals.var_b4soifactor1 * locals.var_t3_dn5), (locals.var_b4soifactor1 * locals.var_t3_dn6), (locals.var_b4soifactor1 * locals.var_t3_dn7), (locals.var_b4soifactor1 * locals.var_t3_dn8), (locals.var_b4soifactor1 * locals.var_t3_dn9), (locals.var_b4soifactor1 * locals.var_t3_dn10), (locals.var_b4soifactor1 * locals.var_t3_dn11), (locals.var_b4soifactor1 * locals.var_t3_dn12),)
    } else {
        (locals.var_ltw, locals.var_ltw_dn3, locals.var_ltw_dn4, locals.var_ltw_dn5, locals.var_ltw_dn6, locals.var_ltw_dn7, locals.var_ltw_dn8, locals.var_ltw_dn9, locals.var_ltw_dn10, locals.var_ltw_dn11, locals.var_ltw_dn12,)
    }
};
        locals.var_ltw = assign10770_e7881;
        locals.var_ltw_dn3 = assign10770_e7881_d_n3;
        locals.var_ltw_dn4 = assign10770_e7881_d_n4;
        locals.var_ltw_dn5 = assign10770_e7881_d_n5;
        locals.var_ltw_dn6 = assign10770_e7881_d_n6;
        locals.var_ltw_dn7 = assign10770_e7881_d_n7;
        locals.var_ltw_dn8 = assign10770_e7881_d_n8;
        locals.var_ltw_dn9 = assign10770_e7881_d_n9;
        locals.var_ltw_dn10 = assign10770_e7881_d_n10;
        locals.var_ltw_dn11 = assign10770_e7881_d_n11;
        locals.var_ltw_dn12 = assign10770_e7881_d_n12;

        let (assign10780_e7893, assign10780_e7893_d_n3, assign10780_e7893_d_n4, assign10780_e7893_d_n5, assign10780_e7893_d_n6, assign10780_e7893_d_n7, assign10780_e7893_d_n8, assign10780_e7893_d_n9, assign10780_e7893_d_n10, assign10780_e7893_d_n11, assign10780_e7893_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign10780_e7885: f64 = (-0.5);
        let assign10780_e7887: f64 = (assign10780_e7885 * locals.var_pparam_b4soidvt1);
        let assign10780_e7889: f64 = (assign10780_e7887 * locals.var_b4soileffeot);
        let assign10780_e7891: f64 = (assign10780_e7889 / locals.var_lt1);
        (assign10780_e7891, (((((assign10780_e7885 * locals.var_pparam_b4soidvt1_dn3) * locals.var_b4soileffeot) * locals.var_lt1) - (assign10780_e7889 * locals.var_lt1_dn3)) / (locals.var_lt1 * locals.var_lt1)), (((((assign10780_e7885 * locals.var_pparam_b4soidvt1_dn4) * locals.var_b4soileffeot) * locals.var_lt1) - (assign10780_e7889 * locals.var_lt1_dn4)) / (locals.var_lt1 * locals.var_lt1)), (((((assign10780_e7885 * locals.var_pparam_b4soidvt1_dn5) * locals.var_b4soileffeot) * locals.var_lt1) - (assign10780_e7889 * locals.var_lt1_dn5)) / (locals.var_lt1 * locals.var_lt1)), (((((assign10780_e7885 * locals.var_pparam_b4soidvt1_dn6) * locals.var_b4soileffeot) * locals.var_lt1) - (assign10780_e7889 * locals.var_lt1_dn6)) / (locals.var_lt1 * locals.var_lt1)), (((((assign10780_e7885 * locals.var_pparam_b4soidvt1_dn7) * locals.var_b4soileffeot) * locals.var_lt1) - (assign10780_e7889 * locals.var_lt1_dn7)) / (locals.var_lt1 * locals.var_lt1)), (((((assign10780_e7885 * locals.var_pparam_b4soidvt1_dn8) * locals.var_b4soileffeot) * locals.var_lt1) - (assign10780_e7889 * locals.var_lt1_dn8)) / (locals.var_lt1 * locals.var_lt1)), (((((assign10780_e7885 * locals.var_pparam_b4soidvt1_dn9) * locals.var_b4soileffeot) * locals.var_lt1) - (assign10780_e7889 * locals.var_lt1_dn9)) / (locals.var_lt1 * locals.var_lt1)), (((((assign10780_e7885 * locals.var_pparam_b4soidvt1_dn10) * locals.var_b4soileffeot) * locals.var_lt1) - (assign10780_e7889 * locals.var_lt1_dn10)) / (locals.var_lt1 * locals.var_lt1)), (((((assign10780_e7885 * locals.var_pparam_b4soidvt1_dn11) * locals.var_b4soileffeot) * locals.var_lt1) - (assign10780_e7889 * locals.var_lt1_dn11)) / (locals.var_lt1 * locals.var_lt1)), (((((assign10780_e7885 * locals.var_pparam_b4soidvt1_dn12) * locals.var_b4soileffeot) * locals.var_lt1) - (assign10780_e7889 * locals.var_lt1_dn12)) / (locals.var_lt1 * locals.var_lt1)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign10780_e7893;
        locals.var_t0_dn3 = assign10780_e7893_d_n3;
        locals.var_t0_dn4 = assign10780_e7893_d_n4;
        locals.var_t0_dn5 = assign10780_e7893_d_n5;
        locals.var_t0_dn6 = assign10780_e7893_d_n6;
        locals.var_t0_dn7 = assign10780_e7893_d_n7;
        locals.var_t0_dn8 = assign10780_e7893_d_n8;
        locals.var_t0_dn9 = assign10780_e7893_d_n9;
        locals.var_t0_dn10 = assign10780_e7893_d_n10;
        locals.var_t0_dn11 = assign10780_e7893_d_n11;
        locals.var_t0_dn12 = assign10780_e7893_d_n12;

        let assign10790_e7896: f64 = (-100.0);
        let assign10790_e7897: f64 = if locals.var_t0 > assign10790_e7896 { 1.0 } else { 0.0 };
        locals.var_guard942 = assign10790_e7897;

        let (assign10800_e7905, assign10800_e7905_d_n3, assign10800_e7905_d_n4, assign10800_e7905_d_n5, assign10800_e7905_d_n6, assign10800_e7905_d_n7, assign10800_e7905_d_n8, assign10800_e7905_d_n9, assign10800_e7905_d_n10, assign10800_e7905_d_n11, assign10800_e7905_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard942 != 0.0)) {
        let assign10800_e7903: f64 = (locals.var_t0).exp();
        (assign10800_e7903, (assign10800_e7903 * locals.var_t0_dn3), (assign10800_e7903 * locals.var_t0_dn4), (assign10800_e7903 * locals.var_t0_dn5), (assign10800_e7903 * locals.var_t0_dn6), (assign10800_e7903 * locals.var_t0_dn7), (assign10800_e7903 * locals.var_t0_dn8), (assign10800_e7903 * locals.var_t0_dn9), (assign10800_e7903 * locals.var_t0_dn10), (assign10800_e7903 * locals.var_t0_dn11), (assign10800_e7903 * locals.var_t0_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign10800_e7905;
        locals.var_t1_dn3 = assign10800_e7905_d_n3;
        locals.var_t1_dn4 = assign10800_e7905_d_n4;
        locals.var_t1_dn5 = assign10800_e7905_d_n5;
        locals.var_t1_dn6 = assign10800_e7905_d_n6;
        locals.var_t1_dn7 = assign10800_e7905_d_n7;
        locals.var_t1_dn8 = assign10800_e7905_d_n8;
        locals.var_t1_dn9 = assign10800_e7905_d_n9;
        locals.var_t1_dn10 = assign10800_e7905_d_n10;
        locals.var_t1_dn11 = assign10800_e7905_d_n11;
        locals.var_t1_dn12 = assign10800_e7905_d_n12;

        let (assign10810_e7918, assign10810_e7918_d_n3, assign10810_e7918_d_n4, assign10810_e7918_d_n5, assign10810_e7918_d_n6, assign10810_e7918_d_n7, assign10810_e7918_d_n8, assign10810_e7918_d_n9, assign10810_e7918_d_n10, assign10810_e7918_d_n11, assign10810_e7918_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard942 != 0.0)) {
        let assign10810_e7914: f64 = (2.0 * locals.var_t1);
        let assign10810_e7915: f64 = (1.0 + assign10810_e7914);
        let assign10810_e7916: f64 = (locals.var_t1 * assign10810_e7915);
        (assign10810_e7916, ((locals.var_t1_dn3 * assign10810_e7915) + (locals.var_t1 * (2.0 * locals.var_t1_dn3))), ((locals.var_t1_dn4 * assign10810_e7915) + (locals.var_t1 * (2.0 * locals.var_t1_dn4))), ((locals.var_t1_dn5 * assign10810_e7915) + (locals.var_t1 * (2.0 * locals.var_t1_dn5))), ((locals.var_t1_dn6 * assign10810_e7915) + (locals.var_t1 * (2.0 * locals.var_t1_dn6))), ((locals.var_t1_dn7 * assign10810_e7915) + (locals.var_t1 * (2.0 * locals.var_t1_dn7))), ((locals.var_t1_dn8 * assign10810_e7915) + (locals.var_t1 * (2.0 * locals.var_t1_dn8))), ((locals.var_t1_dn9 * assign10810_e7915) + (locals.var_t1 * (2.0 * locals.var_t1_dn9))), ((locals.var_t1_dn10 * assign10810_e7915) + (locals.var_t1 * (2.0 * locals.var_t1_dn10))), ((locals.var_t1_dn11 * assign10810_e7915) + (locals.var_t1 * (2.0 * locals.var_t1_dn11))), ((locals.var_t1_dn12 * assign10810_e7915) + (locals.var_t1 * (2.0 * locals.var_t1_dn12))),)
    } else {
        (locals.var_theta0, locals.var_theta0_dn3, locals.var_theta0_dn4, locals.var_theta0_dn5, locals.var_theta0_dn6, locals.var_theta0_dn7, locals.var_theta0_dn8, locals.var_theta0_dn9, locals.var_theta0_dn10, locals.var_theta0_dn11, locals.var_theta0_dn12,)
    }
};
        locals.var_theta0 = assign10810_e7918;
        locals.var_theta0_dn3 = assign10810_e7918_d_n3;
        locals.var_theta0_dn4 = assign10810_e7918_d_n4;
        locals.var_theta0_dn5 = assign10810_e7918_d_n5;
        locals.var_theta0_dn6 = assign10810_e7918_d_n6;
        locals.var_theta0_dn7 = assign10810_e7918_d_n7;
        locals.var_theta0_dn8 = assign10810_e7918_d_n8;
        locals.var_theta0_dn9 = assign10810_e7918_d_n9;
        locals.var_theta0_dn10 = assign10810_e7918_d_n10;
        locals.var_theta0_dn11 = assign10810_e7918_d_n11;
        locals.var_theta0_dn12 = assign10810_e7918_d_n12;

        let (assign10820_e7926, assign10820_e7926_d_n3, assign10820_e7926_d_n4, assign10820_e7926_d_n5, assign10820_e7926_d_n6, assign10820_e7926_d_n7, assign10820_e7926_d_n8, assign10820_e7926_d_n9, assign10820_e7926_d_n10, assign10820_e7926_d_n11, assign10820_e7926_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard942 == 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign10820_e7926;
        locals.var_t1_dn3 = assign10820_e7926_d_n3;
        locals.var_t1_dn4 = assign10820_e7926_d_n4;
        locals.var_t1_dn5 = assign10820_e7926_d_n5;
        locals.var_t1_dn6 = assign10820_e7926_d_n6;
        locals.var_t1_dn7 = assign10820_e7926_d_n7;
        locals.var_t1_dn8 = assign10820_e7926_d_n8;
        locals.var_t1_dn9 = assign10820_e7926_d_n9;
        locals.var_t1_dn10 = assign10820_e7926_d_n10;
        locals.var_t1_dn11 = assign10820_e7926_d_n11;
        locals.var_t1_dn12 = assign10820_e7926_d_n12;

        let (assign10830_e7940, assign10830_e7940_d_n3, assign10830_e7940_d_n4, assign10830_e7940_d_n5, assign10830_e7940_d_n6, assign10830_e7940_d_n7, assign10830_e7940_d_n8, assign10830_e7940_d_n9, assign10830_e7940_d_n10, assign10830_e7940_d_n11, assign10830_e7940_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard942 == 0.0)) {
        let assign10830_e7936: f64 = (2.0 * locals.var_t1);
        let assign10830_e7937: f64 = (1.0 + assign10830_e7936);
        let assign10830_e7938: f64 = (locals.var_t1 * assign10830_e7937);
        (assign10830_e7938, ((locals.var_t1_dn3 * assign10830_e7937) + (locals.var_t1 * (2.0 * locals.var_t1_dn3))), ((locals.var_t1_dn4 * assign10830_e7937) + (locals.var_t1 * (2.0 * locals.var_t1_dn4))), ((locals.var_t1_dn5 * assign10830_e7937) + (locals.var_t1 * (2.0 * locals.var_t1_dn5))), ((locals.var_t1_dn6 * assign10830_e7937) + (locals.var_t1 * (2.0 * locals.var_t1_dn6))), ((locals.var_t1_dn7 * assign10830_e7937) + (locals.var_t1 * (2.0 * locals.var_t1_dn7))), ((locals.var_t1_dn8 * assign10830_e7937) + (locals.var_t1 * (2.0 * locals.var_t1_dn8))), ((locals.var_t1_dn9 * assign10830_e7937) + (locals.var_t1 * (2.0 * locals.var_t1_dn9))), ((locals.var_t1_dn10 * assign10830_e7937) + (locals.var_t1 * (2.0 * locals.var_t1_dn10))), ((locals.var_t1_dn11 * assign10830_e7937) + (locals.var_t1 * (2.0 * locals.var_t1_dn11))), ((locals.var_t1_dn12 * assign10830_e7937) + (locals.var_t1 * (2.0 * locals.var_t1_dn12))),)
    } else {
        (locals.var_theta0, locals.var_theta0_dn3, locals.var_theta0_dn4, locals.var_theta0_dn5, locals.var_theta0_dn6, locals.var_theta0_dn7, locals.var_theta0_dn8, locals.var_theta0_dn9, locals.var_theta0_dn10, locals.var_theta0_dn11, locals.var_theta0_dn12,)
    }
};
        locals.var_theta0 = assign10830_e7940;
        locals.var_theta0_dn3 = assign10830_e7940_d_n3;
        locals.var_theta0_dn4 = assign10830_e7940_d_n4;
        locals.var_theta0_dn5 = assign10830_e7940_d_n5;
        locals.var_theta0_dn6 = assign10830_e7940_d_n6;
        locals.var_theta0_dn7 = assign10830_e7940_d_n7;
        locals.var_theta0_dn8 = assign10830_e7940_d_n8;
        locals.var_theta0_dn9 = assign10830_e7940_d_n9;
        locals.var_theta0_dn10 = assign10830_e7940_d_n10;
        locals.var_theta0_dn11 = assign10830_e7940_d_n11;
        locals.var_theta0_dn12 = assign10830_e7940_d_n12;

        let (assign10840_e7949, assign10840_e7949_d_n3, assign10840_e7949_d_n4, assign10840_e7949_d_n5, assign10840_e7949_d_n6, assign10840_e7949_d_n7, assign10840_e7949_d_n8, assign10840_e7949_d_n9, assign10840_e7949_d_n10, assign10840_e7949_d_n11, assign10840_e7949_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign10840_e7945: f64 = (locals.var_pparam_b4soinfactor * locals.var_epssub);
        let assign10840_e7947: f64 = (assign10840_e7945 / locals.var_pparam_b4soixdep0);
        (assign10840_e7947, ((((locals.var_pparam_b4soinfactor_dn3 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign10840_e7945 * locals.var_pparam_b4soixdep0_dn3)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)), ((((locals.var_pparam_b4soinfactor_dn4 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign10840_e7945 * locals.var_pparam_b4soixdep0_dn4)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)), ((((locals.var_pparam_b4soinfactor_dn5 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign10840_e7945 * locals.var_pparam_b4soixdep0_dn5)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)), ((((locals.var_pparam_b4soinfactor_dn6 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign10840_e7945 * locals.var_pparam_b4soixdep0_dn6)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)), ((((locals.var_pparam_b4soinfactor_dn7 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign10840_e7945 * locals.var_pparam_b4soixdep0_dn7)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)), ((((locals.var_pparam_b4soinfactor_dn8 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign10840_e7945 * locals.var_pparam_b4soixdep0_dn8)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)), ((((locals.var_pparam_b4soinfactor_dn9 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign10840_e7945 * locals.var_pparam_b4soixdep0_dn9)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)), ((((locals.var_pparam_b4soinfactor_dn10 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign10840_e7945 * locals.var_pparam_b4soixdep0_dn10)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)), ((((locals.var_pparam_b4soinfactor_dn11 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign10840_e7945 * locals.var_pparam_b4soixdep0_dn11)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)), ((((locals.var_pparam_b4soinfactor_dn12 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign10840_e7945 * locals.var_pparam_b4soixdep0_dn12)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign10840_e7949;
        locals.var_t2_dn3 = assign10840_e7949_d_n3;
        locals.var_t2_dn4 = assign10840_e7949_d_n4;
        locals.var_t2_dn5 = assign10840_e7949_d_n5;
        locals.var_t2_dn6 = assign10840_e7949_d_n6;
        locals.var_t2_dn7 = assign10840_e7949_d_n7;
        locals.var_t2_dn8 = assign10840_e7949_d_n8;
        locals.var_t2_dn9 = assign10840_e7949_d_n9;
        locals.var_t2_dn10 = assign10840_e7949_d_n10;
        locals.var_t2_dn11 = assign10840_e7949_d_n11;
        locals.var_t2_dn12 = assign10840_e7949_d_n12;

        let (assign10850_e7954, assign10850_e7954_d_n3, assign10850_e7954_d_n4, assign10850_e7954_d_n5, assign10850_e7954_d_n6, assign10850_e7954_d_n7, assign10850_e7954_d_n8, assign10850_e7954_d_n9, assign10850_e7954_d_n10, assign10850_e7954_d_n11, assign10850_e7954_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        (locals.var_pparam_b4soicdsc, locals.var_pparam_b4soicdsc_dn3, locals.var_pparam_b4soicdsc_dn4, locals.var_pparam_b4soicdsc_dn5, locals.var_pparam_b4soicdsc_dn6, locals.var_pparam_b4soicdsc_dn7, locals.var_pparam_b4soicdsc_dn8, locals.var_pparam_b4soicdsc_dn9, locals.var_pparam_b4soicdsc_dn10, locals.var_pparam_b4soicdsc_dn11, locals.var_pparam_b4soicdsc_dn12,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign10850_e7954;
        locals.var_t3_dn3 = assign10850_e7954_d_n3;
        locals.var_t3_dn4 = assign10850_e7954_d_n4;
        locals.var_t3_dn5 = assign10850_e7954_d_n5;
        locals.var_t3_dn6 = assign10850_e7954_d_n6;
        locals.var_t3_dn7 = assign10850_e7954_d_n7;
        locals.var_t3_dn8 = assign10850_e7954_d_n8;
        locals.var_t3_dn9 = assign10850_e7954_d_n9;
        locals.var_t3_dn10 = assign10850_e7954_d_n10;
        locals.var_t3_dn11 = assign10850_e7954_d_n11;
        locals.var_t3_dn12 = assign10850_e7954_d_n12;

        let (assign10860_e7967, assign10860_e7967_d_n3, assign10860_e7967_d_n4, assign10860_e7967_d_n5, assign10860_e7967_d_n6, assign10860_e7967_d_n7, assign10860_e7967_d_n8, assign10860_e7967_d_n9, assign10860_e7967_d_n10, assign10860_e7967_d_n11, assign10860_e7967_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign10860_e7960: f64 = (locals.var_t3 * locals.var_theta0);
        let assign10860_e7961: f64 = (locals.var_t2 + assign10860_e7960);
        let assign10860_e7963: f64 = (assign10860_e7961 + locals.var_pparam_b4soicit);
        let assign10860_e7965: f64 = (assign10860_e7963 / locals.var_b4soicox);
        (assign10860_e7965, (((locals.var_t2_dn3 + ((locals.var_t3_dn3 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn3))) + locals.var_pparam_b4soicit_dn3) / locals.var_b4soicox), (((locals.var_t2_dn4 + ((locals.var_t3_dn4 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn4))) + locals.var_pparam_b4soicit_dn4) / locals.var_b4soicox), (((locals.var_t2_dn5 + ((locals.var_t3_dn5 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn5))) + locals.var_pparam_b4soicit_dn5) / locals.var_b4soicox), (((locals.var_t2_dn6 + ((locals.var_t3_dn6 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn6))) + locals.var_pparam_b4soicit_dn6) / locals.var_b4soicox), (((locals.var_t2_dn7 + ((locals.var_t3_dn7 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn7))) + locals.var_pparam_b4soicit_dn7) / locals.var_b4soicox), (((locals.var_t2_dn8 + ((locals.var_t3_dn8 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn8))) + locals.var_pparam_b4soicit_dn8) / locals.var_b4soicox), (((locals.var_t2_dn9 + ((locals.var_t3_dn9 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn9))) + locals.var_pparam_b4soicit_dn9) / locals.var_b4soicox), (((locals.var_t2_dn10 + ((locals.var_t3_dn10 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn10))) + locals.var_pparam_b4soicit_dn10) / locals.var_b4soicox), (((locals.var_t2_dn11 + ((locals.var_t3_dn11 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn11))) + locals.var_pparam_b4soicit_dn11) / locals.var_b4soicox), (((locals.var_t2_dn12 + ((locals.var_t3_dn12 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn12))) + locals.var_pparam_b4soicit_dn12) / locals.var_b4soicox),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign10860_e7967;
        locals.var_t4_dn3 = assign10860_e7967_d_n3;
        locals.var_t4_dn4 = assign10860_e7967_d_n4;
        locals.var_t4_dn5 = assign10860_e7967_d_n5;
        locals.var_t4_dn6 = assign10860_e7967_d_n6;
        locals.var_t4_dn7 = assign10860_e7967_d_n7;
        locals.var_t4_dn8 = assign10860_e7967_d_n8;
        locals.var_t4_dn9 = assign10860_e7967_d_n9;
        locals.var_t4_dn10 = assign10860_e7967_d_n10;
        locals.var_t4_dn11 = assign10860_e7967_d_n11;
        locals.var_t4_dn12 = assign10860_e7967_d_n12;

        let assign10870_e7970: f64 = (-0.5);
        let assign10870_e7971: f64 = if locals.var_t4 >= assign10870_e7970 { 1.0 } else { 0.0 };
        locals.var_guard943 = assign10870_e7971;

        let (assign10880_e7980, assign10880_e7980_d_n3, assign10880_e7980_d_n4, assign10880_e7980_d_n5, assign10880_e7980_d_n6, assign10880_e7980_d_n7, assign10880_e7980_d_n8, assign10880_e7980_d_n9, assign10880_e7980_d_n10, assign10880_e7980_d_n11, assign10880_e7980_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard943 != 0.0)) {
        let assign10880_e7978: f64 = (1.0 + locals.var_t4);
        (assign10880_e7978, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    } else {
        (locals.var_n, locals.var_n_dn3, locals.var_n_dn4, locals.var_n_dn5, locals.var_n_dn6, locals.var_n_dn7, locals.var_n_dn8, locals.var_n_dn9, locals.var_n_dn10, locals.var_n_dn11, locals.var_n_dn12,)
    }
};
        locals.var_n = assign10880_e7980;
        locals.var_n_dn3 = assign10880_e7980_d_n3;
        locals.var_n_dn4 = assign10880_e7980_d_n4;
        locals.var_n_dn5 = assign10880_e7980_d_n5;
        locals.var_n_dn6 = assign10880_e7980_d_n6;
        locals.var_n_dn7 = assign10880_e7980_d_n7;
        locals.var_n_dn8 = assign10880_e7980_d_n8;
        locals.var_n_dn9 = assign10880_e7980_d_n9;
        locals.var_n_dn10 = assign10880_e7980_d_n10;
        locals.var_n_dn11 = assign10880_e7980_d_n11;
        locals.var_n_dn12 = assign10880_e7980_d_n12;

        let (assign10890_e7994, assign10890_e7994_d_n3, assign10890_e7994_d_n4, assign10890_e7994_d_n5, assign10890_e7994_d_n6, assign10890_e7994_d_n7, assign10890_e7994_d_n8, assign10890_e7994_d_n9, assign10890_e7994_d_n10, assign10890_e7994_d_n11, assign10890_e7994_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard943 == 0.0)) {
        let assign10890_e7990: f64 = (8.0 * locals.var_t4);
        let assign10890_e7991: f64 = (3.0 + assign10890_e7990);
        let assign10890_e7992: f64 = (1.0 / assign10890_e7991);
        (assign10890_e7992, (-((8.0 * locals.var_t4_dn3) / (assign10890_e7991 * assign10890_e7991))), (-((8.0 * locals.var_t4_dn4) / (assign10890_e7991 * assign10890_e7991))), (-((8.0 * locals.var_t4_dn5) / (assign10890_e7991 * assign10890_e7991))), (-((8.0 * locals.var_t4_dn6) / (assign10890_e7991 * assign10890_e7991))), (-((8.0 * locals.var_t4_dn7) / (assign10890_e7991 * assign10890_e7991))), (-((8.0 * locals.var_t4_dn8) / (assign10890_e7991 * assign10890_e7991))), (-((8.0 * locals.var_t4_dn9) / (assign10890_e7991 * assign10890_e7991))), (-((8.0 * locals.var_t4_dn10) / (assign10890_e7991 * assign10890_e7991))), (-((8.0 * locals.var_t4_dn11) / (assign10890_e7991 * assign10890_e7991))), (-((8.0 * locals.var_t4_dn12) / (assign10890_e7991 * assign10890_e7991))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign10890_e7994;
        locals.var_t0_dn3 = assign10890_e7994_d_n3;
        locals.var_t0_dn4 = assign10890_e7994_d_n4;
        locals.var_t0_dn5 = assign10890_e7994_d_n5;
        locals.var_t0_dn6 = assign10890_e7994_d_n6;
        locals.var_t0_dn7 = assign10890_e7994_d_n7;
        locals.var_t0_dn8 = assign10890_e7994_d_n8;
        locals.var_t0_dn9 = assign10890_e7994_d_n9;
        locals.var_t0_dn10 = assign10890_e7994_d_n10;
        locals.var_t0_dn11 = assign10890_e7994_d_n11;
        locals.var_t0_dn12 = assign10890_e7994_d_n12;

        let (assign10900_e8008, assign10900_e8008_d_n3, assign10900_e8008_d_n4, assign10900_e8008_d_n5, assign10900_e8008_d_n6, assign10900_e8008_d_n7, assign10900_e8008_d_n8, assign10900_e8008_d_n9, assign10900_e8008_d_n10, assign10900_e8008_d_n11, assign10900_e8008_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard943 == 0.0)) {
        let assign10900_e8003: f64 = (3.0 * locals.var_t4);
        let assign10900_e8004: f64 = (1.0 + assign10900_e8003);
        let assign10900_e8006: f64 = (assign10900_e8004 * locals.var_t0);
        (assign10900_e8006, (((3.0 * locals.var_t4_dn3) * locals.var_t0) + (assign10900_e8004 * locals.var_t0_dn3)), (((3.0 * locals.var_t4_dn4) * locals.var_t0) + (assign10900_e8004 * locals.var_t0_dn4)), (((3.0 * locals.var_t4_dn5) * locals.var_t0) + (assign10900_e8004 * locals.var_t0_dn5)), (((3.0 * locals.var_t4_dn6) * locals.var_t0) + (assign10900_e8004 * locals.var_t0_dn6)), (((3.0 * locals.var_t4_dn7) * locals.var_t0) + (assign10900_e8004 * locals.var_t0_dn7)), (((3.0 * locals.var_t4_dn8) * locals.var_t0) + (assign10900_e8004 * locals.var_t0_dn8)), (((3.0 * locals.var_t4_dn9) * locals.var_t0) + (assign10900_e8004 * locals.var_t0_dn9)), (((3.0 * locals.var_t4_dn10) * locals.var_t0) + (assign10900_e8004 * locals.var_t0_dn10)), (((3.0 * locals.var_t4_dn11) * locals.var_t0) + (assign10900_e8004 * locals.var_t0_dn11)), (((3.0 * locals.var_t4_dn12) * locals.var_t0) + (assign10900_e8004 * locals.var_t0_dn12)),)
    } else {
        (locals.var_n, locals.var_n_dn3, locals.var_n_dn4, locals.var_n_dn5, locals.var_n_dn6, locals.var_n_dn7, locals.var_n_dn8, locals.var_n_dn9, locals.var_n_dn10, locals.var_n_dn11, locals.var_n_dn12,)
    }
};
        locals.var_n = assign10900_e8008;
        locals.var_n_dn3 = assign10900_e8008_d_n3;
        locals.var_n_dn4 = assign10900_e8008_d_n4;
        locals.var_n_dn5 = assign10900_e8008_d_n5;
        locals.var_n_dn6 = assign10900_e8008_d_n6;
        locals.var_n_dn7 = assign10900_e8008_d_n7;
        locals.var_n_dn8 = assign10900_e8008_d_n8;
        locals.var_n_dn9 = assign10900_e8008_d_n9;
        locals.var_n_dn10 = assign10900_e8008_d_n10;
        locals.var_n_dn11 = assign10900_e8008_d_n11;
        locals.var_n_dn12 = assign10900_e8008_d_n12;

        let assign10910_e8011: f64 = if locals.var_pparam_b4soidvtp0 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard944 = assign10910_e8011;

    }

    pub(super) fn stamp_transient_block_22(
        locals: &mut StampLocals,
    ) {
        let (assign10920_e8022, assign10920_e8022_d_n3, assign10920_e8022_d_n4, assign10920_e8022_d_n5, assign10920_e8022_d_n6, assign10920_e8022_d_n7, assign10920_e8022_d_n8, assign10920_e8022_d_n9, assign10920_e8022_d_n10, assign10920_e8022_d_n11, assign10920_e8022_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard944 != 0.0)) {
        let assign10920_e8019: f64 = (2.0 * locals.var_pparam_b4soidvtp0);
        let assign10920_e8020: f64 = (locals.var_b4soileffeot + assign10920_e8019);
        (assign10920_e8020, (2.0 * locals.var_pparam_b4soidvtp0_dn3), (2.0 * locals.var_pparam_b4soidvtp0_dn4), (2.0 * locals.var_pparam_b4soidvtp0_dn5), (2.0 * locals.var_pparam_b4soidvtp0_dn6), (2.0 * locals.var_pparam_b4soidvtp0_dn7), (2.0 * locals.var_pparam_b4soidvtp0_dn8), (2.0 * locals.var_pparam_b4soidvtp0_dn9), (2.0 * locals.var_pparam_b4soidvtp0_dn10), (2.0 * locals.var_pparam_b4soidvtp0_dn11), (2.0 * locals.var_pparam_b4soidvtp0_dn12),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign10920_e8022;
        locals.var_t3_dn3 = assign10920_e8022_d_n3;
        locals.var_t3_dn4 = assign10920_e8022_d_n4;
        locals.var_t3_dn5 = assign10920_e8022_d_n5;
        locals.var_t3_dn6 = assign10920_e8022_d_n6;
        locals.var_t3_dn7 = assign10920_e8022_d_n7;
        locals.var_t3_dn8 = assign10920_e8022_d_n8;
        locals.var_t3_dn9 = assign10920_e8022_d_n9;
        locals.var_t3_dn10 = assign10920_e8022_d_n10;
        locals.var_t3_dn11 = assign10920_e8022_d_n11;
        locals.var_t3_dn12 = assign10920_e8022_d_n12;

        let (assign10930_e8042, assign10930_e8042_d_n3, assign10930_e8042_d_n4, assign10930_e8042_d_n5, assign10930_e8042_d_n6, assign10930_e8042_d_n7, assign10930_e8042_d_n8, assign10930_e8042_d_n9, assign10930_e8042_d_n10, assign10930_e8042_d_n11, assign10930_e8042_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard944 != 0.0)) {
        let assign10930_e8030: f64 = (locals.var_b4soileffeot / locals.var_t3);
        let (assign10930_e8039, assign10930_e8039_d_n3, assign10930_e8039_d_n4, assign10930_e8039_d_n5, assign10930_e8039_d_n6, assign10930_e8039_d_n7, assign10930_e8039_d_n8, assign10930_e8039_d_n9, assign10930_e8039_d_n10, assign10930_e8039_d_n11, assign10930_e8039_d_n12,) = {
            if (assign10930_e8030 > 1e-38) {
                let assign10930_e8035: f64 = (locals.var_b4soileffeot / locals.var_t3);
                let assign10930_e8036: f64 = (assign10930_e8035).ln();
                (assign10930_e8036, ((-((locals.var_b4soileffeot * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3))) / assign10930_e8035), ((-((locals.var_b4soileffeot * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))) / assign10930_e8035), ((-((locals.var_b4soileffeot * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))) / assign10930_e8035), ((-((locals.var_b4soileffeot * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))) / assign10930_e8035), ((-((locals.var_b4soileffeot * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))) / assign10930_e8035), ((-((locals.var_b4soileffeot * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))) / assign10930_e8035), ((-((locals.var_b4soileffeot * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))) / assign10930_e8035), ((-((locals.var_b4soileffeot * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))) / assign10930_e8035), ((-((locals.var_b4soileffeot * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))) / assign10930_e8035), ((-((locals.var_b4soileffeot * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3))) / assign10930_e8035),)
            } else {
                let assign10930_e8038: f64 = (-87.49823353377374);
                (assign10930_e8038, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign10930_e8040: f64 = (locals.var_vtmeot * assign10930_e8039);
        (assign10930_e8040, (locals.var_vtmeot * assign10930_e8039_d_n3), (locals.var_vtmeot * assign10930_e8039_d_n4), (locals.var_vtmeot * assign10930_e8039_d_n5), (locals.var_vtmeot * assign10930_e8039_d_n6), (locals.var_vtmeot * assign10930_e8039_d_n7), (locals.var_vtmeot * assign10930_e8039_d_n8), (locals.var_vtmeot * assign10930_e8039_d_n9), (locals.var_vtmeot * assign10930_e8039_d_n10), (locals.var_vtmeot * assign10930_e8039_d_n11), (locals.var_vtmeot * assign10930_e8039_d_n12),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign10930_e8042;
        locals.var_t4_dn3 = assign10930_e8042_d_n3;
        locals.var_t4_dn4 = assign10930_e8042_d_n4;
        locals.var_t4_dn5 = assign10930_e8042_d_n5;
        locals.var_t4_dn6 = assign10930_e8042_d_n6;
        locals.var_t4_dn7 = assign10930_e8042_d_n7;
        locals.var_t4_dn8 = assign10930_e8042_d_n8;
        locals.var_t4_dn9 = assign10930_e8042_d_n9;
        locals.var_t4_dn10 = assign10930_e8042_d_n10;
        locals.var_t4_dn11 = assign10930_e8042_d_n11;
        locals.var_t4_dn12 = assign10930_e8042_d_n12;

        let (assign10940_e8051, assign10940_e8051_d_n3, assign10940_e8051_d_n4, assign10940_e8051_d_n5, assign10940_e8051_d_n6, assign10940_e8051_d_n7, assign10940_e8051_d_n8, assign10940_e8051_d_n9, assign10940_e8051_d_n10, assign10940_e8051_d_n11, assign10940_e8051_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard944 != 0.0)) {
        let assign10940_e8049: f64 = (locals.var_n * locals.var_t4);
        (assign10940_e8049, ((locals.var_n_dn3 * locals.var_t4) + (locals.var_n * locals.var_t4_dn3)), ((locals.var_n_dn4 * locals.var_t4) + (locals.var_n * locals.var_t4_dn4)), ((locals.var_n_dn5 * locals.var_t4) + (locals.var_n * locals.var_t4_dn5)), ((locals.var_n_dn6 * locals.var_t4) + (locals.var_n * locals.var_t4_dn6)), ((locals.var_n_dn7 * locals.var_t4) + (locals.var_n * locals.var_t4_dn7)), ((locals.var_n_dn8 * locals.var_t4) + (locals.var_n * locals.var_t4_dn8)), ((locals.var_n_dn9 * locals.var_t4) + (locals.var_n * locals.var_t4_dn9)), ((locals.var_n_dn10 * locals.var_t4) + (locals.var_n * locals.var_t4_dn10)), ((locals.var_n_dn11 * locals.var_t4) + (locals.var_n * locals.var_t4_dn11)), ((locals.var_n_dn12 * locals.var_t4) + (locals.var_n * locals.var_t4_dn12)),)
    } else {
        (locals.var_dits_sft, locals.var_dits_sft_dn3, locals.var_dits_sft_dn4, locals.var_dits_sft_dn5, locals.var_dits_sft_dn6, locals.var_dits_sft_dn7, locals.var_dits_sft_dn8, locals.var_dits_sft_dn9, locals.var_dits_sft_dn10, locals.var_dits_sft_dn11, locals.var_dits_sft_dn12,)
    }
};
        locals.var_dits_sft = assign10940_e8051;
        locals.var_dits_sft_dn3 = assign10940_e8051_d_n3;
        locals.var_dits_sft_dn4 = assign10940_e8051_d_n4;
        locals.var_dits_sft_dn5 = assign10940_e8051_d_n5;
        locals.var_dits_sft_dn6 = assign10940_e8051_d_n6;
        locals.var_dits_sft_dn7 = assign10940_e8051_d_n7;
        locals.var_dits_sft_dn8 = assign10940_e8051_d_n8;
        locals.var_dits_sft_dn9 = assign10940_e8051_d_n9;
        locals.var_dits_sft_dn10 = assign10940_e8051_d_n10;
        locals.var_dits_sft_dn11 = assign10940_e8051_d_n11;
        locals.var_dits_sft_dn12 = assign10940_e8051_d_n12;

        let (assign10950_e8059, assign10950_e8059_d_n3, assign10950_e8059_d_n4, assign10950_e8059_d_n5, assign10950_e8059_d_n6, assign10950_e8059_d_n7, assign10950_e8059_d_n8, assign10950_e8059_d_n9, assign10950_e8059_d_n10, assign10950_e8059_d_n11, assign10950_e8059_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard944 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dits_sft, locals.var_dits_sft_dn3, locals.var_dits_sft_dn4, locals.var_dits_sft_dn5, locals.var_dits_sft_dn6, locals.var_dits_sft_dn7, locals.var_dits_sft_dn8, locals.var_dits_sft_dn9, locals.var_dits_sft_dn10, locals.var_dits_sft_dn11, locals.var_dits_sft_dn12,)
    }
};
        locals.var_dits_sft = assign10950_e8059;
        locals.var_dits_sft_dn3 = assign10950_e8059_d_n3;
        locals.var_dits_sft_dn4 = assign10950_e8059_d_n4;
        locals.var_dits_sft_dn5 = assign10950_e8059_d_n5;
        locals.var_dits_sft_dn6 = assign10950_e8059_d_n6;
        locals.var_dits_sft_dn7 = assign10950_e8059_d_n7;
        locals.var_dits_sft_dn8 = assign10950_e8059_d_n8;
        locals.var_dits_sft_dn9 = assign10950_e8059_d_n9;
        locals.var_dits_sft_dn10 = assign10950_e8059_d_n10;
        locals.var_dits_sft_dn11 = assign10950_e8059_d_n11;
        locals.var_dits_sft_dn12 = assign10950_e8059_d_n12;

        let (assign10960_e8066, assign10960_e8066_d_n3, assign10960_e8066_d_n4, assign10960_e8066_d_n5, assign10960_e8066_d_n6, assign10960_e8066_d_n7, assign10960_e8066_d_n8, assign10960_e8066_d_n9, assign10960_e8066_d_n10, assign10960_e8066_d_n11, assign10960_e8066_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign10960_e8064: f64 = (locals.var_pparam_b4soidvt0 * locals.var_theta0);
        (assign10960_e8064, ((locals.var_pparam_b4soidvt0_dn3 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn3)), ((locals.var_pparam_b4soidvt0_dn4 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn4)), ((locals.var_pparam_b4soidvt0_dn5 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn5)), ((locals.var_pparam_b4soidvt0_dn6 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn6)), ((locals.var_pparam_b4soidvt0_dn7 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn7)), ((locals.var_pparam_b4soidvt0_dn8 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn8)), ((locals.var_pparam_b4soidvt0_dn9 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn9)), ((locals.var_pparam_b4soidvt0_dn10 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn10)), ((locals.var_pparam_b4soidvt0_dn11 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn11)), ((locals.var_pparam_b4soidvt0_dn12 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn12)),)
    } else {
        (locals.var_b4soithetavth, locals.var_b4soithetavth_dn3, locals.var_b4soithetavth_dn4, locals.var_b4soithetavth_dn5, locals.var_b4soithetavth_dn6, locals.var_b4soithetavth_dn7, locals.var_b4soithetavth_dn8, locals.var_b4soithetavth_dn9, locals.var_b4soithetavth_dn10, locals.var_b4soithetavth_dn11, locals.var_b4soithetavth_dn12,)
    }
};
        locals.var_b4soithetavth = assign10960_e8066;
        locals.var_b4soithetavth_dn3 = assign10960_e8066_d_n3;
        locals.var_b4soithetavth_dn4 = assign10960_e8066_d_n4;
        locals.var_b4soithetavth_dn5 = assign10960_e8066_d_n5;
        locals.var_b4soithetavth_dn6 = assign10960_e8066_d_n6;
        locals.var_b4soithetavth_dn7 = assign10960_e8066_d_n7;
        locals.var_b4soithetavth_dn8 = assign10960_e8066_d_n8;
        locals.var_b4soithetavth_dn9 = assign10960_e8066_d_n9;
        locals.var_b4soithetavth_dn10 = assign10960_e8066_d_n10;
        locals.var_b4soithetavth_dn11 = assign10960_e8066_d_n11;
        locals.var_b4soithetavth_dn12 = assign10960_e8066_d_n12;

        let (assign10970_e8073, assign10970_e8073_d_n3, assign10970_e8073_d_n4, assign10970_e8073_d_n5, assign10970_e8073_d_n6, assign10970_e8073_d_n7, assign10970_e8073_d_n8, assign10970_e8073_d_n9, assign10970_e8073_d_n10, assign10970_e8073_d_n11, assign10970_e8073_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign10970_e8071: f64 = (locals.var_b4soithetavth * locals.var_v0);
        (assign10970_e8071, ((locals.var_b4soithetavth_dn3 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn3)), ((locals.var_b4soithetavth_dn4 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn4)), ((locals.var_b4soithetavth_dn5 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn5)), ((locals.var_b4soithetavth_dn6 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn6)), ((locals.var_b4soithetavth_dn7 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn7)), ((locals.var_b4soithetavth_dn8 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn8)), ((locals.var_b4soithetavth_dn9 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn9)), ((locals.var_b4soithetavth_dn10 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn10)), ((locals.var_b4soithetavth_dn11 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn11)), ((locals.var_b4soithetavth_dn12 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn12)),)
    } else {
        (locals.var_delt_vth, locals.var_delt_vth_dn3, locals.var_delt_vth_dn4, locals.var_delt_vth_dn5, locals.var_delt_vth_dn6, locals.var_delt_vth_dn7, locals.var_delt_vth_dn8, locals.var_delt_vth_dn9, locals.var_delt_vth_dn10, locals.var_delt_vth_dn11, locals.var_delt_vth_dn12,)
    }
};
        locals.var_delt_vth = assign10970_e8073;
        locals.var_delt_vth_dn3 = assign10970_e8073_d_n3;
        locals.var_delt_vth_dn4 = assign10970_e8073_d_n4;
        locals.var_delt_vth_dn5 = assign10970_e8073_d_n5;
        locals.var_delt_vth_dn6 = assign10970_e8073_d_n6;
        locals.var_delt_vth_dn7 = assign10970_e8073_d_n7;
        locals.var_delt_vth_dn8 = assign10970_e8073_d_n8;
        locals.var_delt_vth_dn9 = assign10970_e8073_d_n9;
        locals.var_delt_vth_dn10 = assign10970_e8073_d_n10;
        locals.var_delt_vth_dn11 = assign10970_e8073_d_n11;
        locals.var_delt_vth_dn12 = assign10970_e8073_d_n12;

        let (assign10980_e8087, assign10980_e8087_d_n3, assign10980_e8087_d_n4, assign10980_e8087_d_n5, assign10980_e8087_d_n6, assign10980_e8087_d_n7, assign10980_e8087_d_n8, assign10980_e8087_d_n9, assign10980_e8087_d_n10, assign10980_e8087_d_n11, assign10980_e8087_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign10980_e8077: f64 = (-0.5);
        let assign10980_e8079: f64 = (assign10980_e8077 * locals.var_pparam_b4soidvt1w);
        let assign10980_e8081: f64 = (assign10980_e8079 * locals.var_b4soiweffeot);
        let assign10980_e8083: f64 = (assign10980_e8081 * locals.var_b4soileffeot);
        let assign10980_e8085: f64 = (assign10980_e8083 / locals.var_ltw);
        (assign10980_e8085, ((((((assign10980_e8077 * locals.var_pparam_b4soidvt1w_dn3) * locals.var_b4soiweffeot) * locals.var_b4soileffeot) * locals.var_ltw) - (assign10980_e8083 * locals.var_ltw_dn3)) / (locals.var_ltw * locals.var_ltw)), ((((((assign10980_e8077 * locals.var_pparam_b4soidvt1w_dn4) * locals.var_b4soiweffeot) * locals.var_b4soileffeot) * locals.var_ltw) - (assign10980_e8083 * locals.var_ltw_dn4)) / (locals.var_ltw * locals.var_ltw)), ((((((assign10980_e8077 * locals.var_pparam_b4soidvt1w_dn5) * locals.var_b4soiweffeot) * locals.var_b4soileffeot) * locals.var_ltw) - (assign10980_e8083 * locals.var_ltw_dn5)) / (locals.var_ltw * locals.var_ltw)), ((((((assign10980_e8077 * locals.var_pparam_b4soidvt1w_dn6) * locals.var_b4soiweffeot) * locals.var_b4soileffeot) * locals.var_ltw) - (assign10980_e8083 * locals.var_ltw_dn6)) / (locals.var_ltw * locals.var_ltw)), ((((((assign10980_e8077 * locals.var_pparam_b4soidvt1w_dn7) * locals.var_b4soiweffeot) * locals.var_b4soileffeot) * locals.var_ltw) - (assign10980_e8083 * locals.var_ltw_dn7)) / (locals.var_ltw * locals.var_ltw)), ((((((assign10980_e8077 * locals.var_pparam_b4soidvt1w_dn8) * locals.var_b4soiweffeot) * locals.var_b4soileffeot) * locals.var_ltw) - (assign10980_e8083 * locals.var_ltw_dn8)) / (locals.var_ltw * locals.var_ltw)), ((((((assign10980_e8077 * locals.var_pparam_b4soidvt1w_dn9) * locals.var_b4soiweffeot) * locals.var_b4soileffeot) * locals.var_ltw) - (assign10980_e8083 * locals.var_ltw_dn9)) / (locals.var_ltw * locals.var_ltw)), ((((((assign10980_e8077 * locals.var_pparam_b4soidvt1w_dn10) * locals.var_b4soiweffeot) * locals.var_b4soileffeot) * locals.var_ltw) - (assign10980_e8083 * locals.var_ltw_dn10)) / (locals.var_ltw * locals.var_ltw)), ((((((assign10980_e8077 * locals.var_pparam_b4soidvt1w_dn11) * locals.var_b4soiweffeot) * locals.var_b4soileffeot) * locals.var_ltw) - (assign10980_e8083 * locals.var_ltw_dn11)) / (locals.var_ltw * locals.var_ltw)), ((((((assign10980_e8077 * locals.var_pparam_b4soidvt1w_dn12) * locals.var_b4soiweffeot) * locals.var_b4soileffeot) * locals.var_ltw) - (assign10980_e8083 * locals.var_ltw_dn12)) / (locals.var_ltw * locals.var_ltw)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign10980_e8087;
        locals.var_t0_dn3 = assign10980_e8087_d_n3;
        locals.var_t0_dn4 = assign10980_e8087_d_n4;
        locals.var_t0_dn5 = assign10980_e8087_d_n5;
        locals.var_t0_dn6 = assign10980_e8087_d_n6;
        locals.var_t0_dn7 = assign10980_e8087_d_n7;
        locals.var_t0_dn8 = assign10980_e8087_d_n8;
        locals.var_t0_dn9 = assign10980_e8087_d_n9;
        locals.var_t0_dn10 = assign10980_e8087_d_n10;
        locals.var_t0_dn11 = assign10980_e8087_d_n11;
        locals.var_t0_dn12 = assign10980_e8087_d_n12;

        let assign10990_e8090: f64 = (-100.0);
        let assign10990_e8091: f64 = if locals.var_t0 > assign10990_e8090 { 1.0 } else { 0.0 };
        locals.var_guard945 = assign10990_e8091;

        let (assign11000_e8099, assign11000_e8099_d_n3, assign11000_e8099_d_n4, assign11000_e8099_d_n5, assign11000_e8099_d_n6, assign11000_e8099_d_n7, assign11000_e8099_d_n8, assign11000_e8099_d_n9, assign11000_e8099_d_n10, assign11000_e8099_d_n11, assign11000_e8099_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard945 != 0.0)) {
        let assign11000_e8097: f64 = (locals.var_t0).exp();
        (assign11000_e8097, (assign11000_e8097 * locals.var_t0_dn3), (assign11000_e8097 * locals.var_t0_dn4), (assign11000_e8097 * locals.var_t0_dn5), (assign11000_e8097 * locals.var_t0_dn6), (assign11000_e8097 * locals.var_t0_dn7), (assign11000_e8097 * locals.var_t0_dn8), (assign11000_e8097 * locals.var_t0_dn9), (assign11000_e8097 * locals.var_t0_dn10), (assign11000_e8097 * locals.var_t0_dn11), (assign11000_e8097 * locals.var_t0_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign11000_e8099;
        locals.var_t1_dn3 = assign11000_e8099_d_n3;
        locals.var_t1_dn4 = assign11000_e8099_d_n4;
        locals.var_t1_dn5 = assign11000_e8099_d_n5;
        locals.var_t1_dn6 = assign11000_e8099_d_n6;
        locals.var_t1_dn7 = assign11000_e8099_d_n7;
        locals.var_t1_dn8 = assign11000_e8099_d_n8;
        locals.var_t1_dn9 = assign11000_e8099_d_n9;
        locals.var_t1_dn10 = assign11000_e8099_d_n10;
        locals.var_t1_dn11 = assign11000_e8099_d_n11;
        locals.var_t1_dn12 = assign11000_e8099_d_n12;

        let (assign11010_e8112, assign11010_e8112_d_n3, assign11010_e8112_d_n4, assign11010_e8112_d_n5, assign11010_e8112_d_n6, assign11010_e8112_d_n7, assign11010_e8112_d_n8, assign11010_e8112_d_n9, assign11010_e8112_d_n10, assign11010_e8112_d_n11, assign11010_e8112_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard945 != 0.0)) {
        let assign11010_e8108: f64 = (2.0 * locals.var_t1);
        let assign11010_e8109: f64 = (1.0 + assign11010_e8108);
        let assign11010_e8110: f64 = (locals.var_t1 * assign11010_e8109);
        (assign11010_e8110, ((locals.var_t1_dn3 * assign11010_e8109) + (locals.var_t1 * (2.0 * locals.var_t1_dn3))), ((locals.var_t1_dn4 * assign11010_e8109) + (locals.var_t1 * (2.0 * locals.var_t1_dn4))), ((locals.var_t1_dn5 * assign11010_e8109) + (locals.var_t1 * (2.0 * locals.var_t1_dn5))), ((locals.var_t1_dn6 * assign11010_e8109) + (locals.var_t1 * (2.0 * locals.var_t1_dn6))), ((locals.var_t1_dn7 * assign11010_e8109) + (locals.var_t1 * (2.0 * locals.var_t1_dn7))), ((locals.var_t1_dn8 * assign11010_e8109) + (locals.var_t1 * (2.0 * locals.var_t1_dn8))), ((locals.var_t1_dn9 * assign11010_e8109) + (locals.var_t1 * (2.0 * locals.var_t1_dn9))), ((locals.var_t1_dn10 * assign11010_e8109) + (locals.var_t1 * (2.0 * locals.var_t1_dn10))), ((locals.var_t1_dn11 * assign11010_e8109) + (locals.var_t1 * (2.0 * locals.var_t1_dn11))), ((locals.var_t1_dn12 * assign11010_e8109) + (locals.var_t1 * (2.0 * locals.var_t1_dn12))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign11010_e8112;
        locals.var_t2_dn3 = assign11010_e8112_d_n3;
        locals.var_t2_dn4 = assign11010_e8112_d_n4;
        locals.var_t2_dn5 = assign11010_e8112_d_n5;
        locals.var_t2_dn6 = assign11010_e8112_d_n6;
        locals.var_t2_dn7 = assign11010_e8112_d_n7;
        locals.var_t2_dn8 = assign11010_e8112_d_n8;
        locals.var_t2_dn9 = assign11010_e8112_d_n9;
        locals.var_t2_dn10 = assign11010_e8112_d_n10;
        locals.var_t2_dn11 = assign11010_e8112_d_n11;
        locals.var_t2_dn12 = assign11010_e8112_d_n12;

        let (assign11020_e8120, assign11020_e8120_d_n3, assign11020_e8120_d_n4, assign11020_e8120_d_n5, assign11020_e8120_d_n6, assign11020_e8120_d_n7, assign11020_e8120_d_n8, assign11020_e8120_d_n9, assign11020_e8120_d_n10, assign11020_e8120_d_n11, assign11020_e8120_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard945 == 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign11020_e8120;
        locals.var_t1_dn3 = assign11020_e8120_d_n3;
        locals.var_t1_dn4 = assign11020_e8120_d_n4;
        locals.var_t1_dn5 = assign11020_e8120_d_n5;
        locals.var_t1_dn6 = assign11020_e8120_d_n6;
        locals.var_t1_dn7 = assign11020_e8120_d_n7;
        locals.var_t1_dn8 = assign11020_e8120_d_n8;
        locals.var_t1_dn9 = assign11020_e8120_d_n9;
        locals.var_t1_dn10 = assign11020_e8120_d_n10;
        locals.var_t1_dn11 = assign11020_e8120_d_n11;
        locals.var_t1_dn12 = assign11020_e8120_d_n12;

        let (assign11030_e8134, assign11030_e8134_d_n3, assign11030_e8134_d_n4, assign11030_e8134_d_n5, assign11030_e8134_d_n6, assign11030_e8134_d_n7, assign11030_e8134_d_n8, assign11030_e8134_d_n9, assign11030_e8134_d_n10, assign11030_e8134_d_n11, assign11030_e8134_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard945 == 0.0)) {
        let assign11030_e8130: f64 = (2.0 * locals.var_t1);
        let assign11030_e8131: f64 = (1.0 + assign11030_e8130);
        let assign11030_e8132: f64 = (locals.var_t1 * assign11030_e8131);
        (assign11030_e8132, ((locals.var_t1_dn3 * assign11030_e8131) + (locals.var_t1 * (2.0 * locals.var_t1_dn3))), ((locals.var_t1_dn4 * assign11030_e8131) + (locals.var_t1 * (2.0 * locals.var_t1_dn4))), ((locals.var_t1_dn5 * assign11030_e8131) + (locals.var_t1 * (2.0 * locals.var_t1_dn5))), ((locals.var_t1_dn6 * assign11030_e8131) + (locals.var_t1 * (2.0 * locals.var_t1_dn6))), ((locals.var_t1_dn7 * assign11030_e8131) + (locals.var_t1 * (2.0 * locals.var_t1_dn7))), ((locals.var_t1_dn8 * assign11030_e8131) + (locals.var_t1 * (2.0 * locals.var_t1_dn8))), ((locals.var_t1_dn9 * assign11030_e8131) + (locals.var_t1 * (2.0 * locals.var_t1_dn9))), ((locals.var_t1_dn10 * assign11030_e8131) + (locals.var_t1 * (2.0 * locals.var_t1_dn10))), ((locals.var_t1_dn11 * assign11030_e8131) + (locals.var_t1 * (2.0 * locals.var_t1_dn11))), ((locals.var_t1_dn12 * assign11030_e8131) + (locals.var_t1 * (2.0 * locals.var_t1_dn12))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign11030_e8134;
        locals.var_t2_dn3 = assign11030_e8134_d_n3;
        locals.var_t2_dn4 = assign11030_e8134_d_n4;
        locals.var_t2_dn5 = assign11030_e8134_d_n5;
        locals.var_t2_dn6 = assign11030_e8134_d_n6;
        locals.var_t2_dn7 = assign11030_e8134_d_n7;
        locals.var_t2_dn8 = assign11030_e8134_d_n8;
        locals.var_t2_dn9 = assign11030_e8134_d_n9;
        locals.var_t2_dn10 = assign11030_e8134_d_n10;
        locals.var_t2_dn11 = assign11030_e8134_d_n11;
        locals.var_t2_dn12 = assign11030_e8134_d_n12;

        let (assign11040_e8141, assign11040_e8141_d_n3, assign11040_e8141_d_n4, assign11040_e8141_d_n5, assign11040_e8141_d_n6, assign11040_e8141_d_n7, assign11040_e8141_d_n8, assign11040_e8141_d_n9, assign11040_e8141_d_n10, assign11040_e8141_d_n11, assign11040_e8141_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11040_e8139: f64 = (locals.var_pparam_b4soidvt0w * locals.var_t2);
        (assign11040_e8139, ((locals.var_pparam_b4soidvt0w_dn3 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn3)), ((locals.var_pparam_b4soidvt0w_dn4 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn4)), ((locals.var_pparam_b4soidvt0w_dn5 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn5)), ((locals.var_pparam_b4soidvt0w_dn6 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn6)), ((locals.var_pparam_b4soidvt0w_dn7 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn7)), ((locals.var_pparam_b4soidvt0w_dn8 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn8)), ((locals.var_pparam_b4soidvt0w_dn9 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn9)), ((locals.var_pparam_b4soidvt0w_dn10 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn10)), ((locals.var_pparam_b4soidvt0w_dn11 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn11)), ((locals.var_pparam_b4soidvt0w_dn12 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn12)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign11040_e8141;
        locals.var_t0_dn3 = assign11040_e8141_d_n3;
        locals.var_t0_dn4 = assign11040_e8141_d_n4;
        locals.var_t0_dn5 = assign11040_e8141_d_n5;
        locals.var_t0_dn6 = assign11040_e8141_d_n6;
        locals.var_t0_dn7 = assign11040_e8141_d_n7;
        locals.var_t0_dn8 = assign11040_e8141_d_n8;
        locals.var_t0_dn9 = assign11040_e8141_d_n9;
        locals.var_t0_dn10 = assign11040_e8141_d_n10;
        locals.var_t0_dn11 = assign11040_e8141_d_n11;
        locals.var_t0_dn12 = assign11040_e8141_d_n12;

        let (assign11050_e8148, assign11050_e8148_d_n3, assign11050_e8148_d_n4, assign11050_e8148_d_n5, assign11050_e8148_d_n6, assign11050_e8148_d_n7, assign11050_e8148_d_n8, assign11050_e8148_d_n9, assign11050_e8148_d_n10, assign11050_e8148_d_n11, assign11050_e8148_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11050_e8146: f64 = (locals.var_t0 * locals.var_v0);
        (assign11050_e8146, ((locals.var_t0_dn3 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn3)), ((locals.var_t0_dn4 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn4)), ((locals.var_t0_dn5 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn5)), ((locals.var_t0_dn6 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn6)), ((locals.var_t0_dn7 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn7)), ((locals.var_t0_dn8 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn8)), ((locals.var_t0_dn9 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn9)), ((locals.var_t0_dn10 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn10)), ((locals.var_t0_dn11 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn11)), ((locals.var_t0_dn12 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn12)),)
    } else {
        (locals.var_deltvthw, locals.var_deltvthw_dn3, locals.var_deltvthw_dn4, locals.var_deltvthw_dn5, locals.var_deltvthw_dn6, locals.var_deltvthw_dn7, locals.var_deltvthw_dn8, locals.var_deltvthw_dn9, locals.var_deltvthw_dn10, locals.var_deltvthw_dn11, locals.var_deltvthw_dn12,)
    }
};
        locals.var_deltvthw = assign11050_e8148;
        locals.var_deltvthw_dn3 = assign11050_e8148_d_n3;
        locals.var_deltvthw_dn4 = assign11050_e8148_d_n4;
        locals.var_deltvthw_dn5 = assign11050_e8148_d_n5;
        locals.var_deltvthw_dn6 = assign11050_e8148_d_n6;
        locals.var_deltvthw_dn7 = assign11050_e8148_d_n7;
        locals.var_deltvthw_dn8 = assign11050_e8148_d_n8;
        locals.var_deltvthw_dn9 = assign11050_e8148_d_n9;
        locals.var_deltvthw_dn10 = assign11050_e8148_d_n10;
        locals.var_deltvthw_dn11 = assign11050_e8148_d_n11;
        locals.var_deltvthw_dn12 = assign11050_e8148_d_n12;

        let (assign11060_e8157,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11060_e8153: f64 = (locals.var_b4soitempeot / locals.var_b4soitnom);
        let assign11060_e8155: f64 = (assign11060_e8153 - 1.0);
        (assign11060_e8155,)
    } else {
        (locals.var_tempratiominus1__blk828,)
    }
};
        locals.var_tempratiominus1__blk828 = assign11060_e8157;

        let (assign11070_e8167, assign11070_e8167_d_n3, assign11070_e8167_d_n4, assign11070_e8167_d_n5, assign11070_e8167_d_n6, assign11070_e8167_d_n7, assign11070_e8167_d_n8, assign11070_e8167_d_n9, assign11070_e8167_d_n10, assign11070_e8167_d_n11, assign11070_e8167_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11070_e8163: f64 = (locals.var_pparam_b4soilpe0 / locals.var_b4soileffeot);
        let assign11070_e8164: f64 = (1.0 + assign11070_e8163);
        let assign11070_e8165: f64 = (assign11070_e8164).sqrt();
        (assign11070_e8165, ((locals.var_pparam_b4soilpe0_dn3 / locals.var_b4soileffeot) / (2.0 * assign11070_e8165)), ((locals.var_pparam_b4soilpe0_dn4 / locals.var_b4soileffeot) / (2.0 * assign11070_e8165)), ((locals.var_pparam_b4soilpe0_dn5 / locals.var_b4soileffeot) / (2.0 * assign11070_e8165)), ((locals.var_pparam_b4soilpe0_dn6 / locals.var_b4soileffeot) / (2.0 * assign11070_e8165)), ((locals.var_pparam_b4soilpe0_dn7 / locals.var_b4soileffeot) / (2.0 * assign11070_e8165)), ((locals.var_pparam_b4soilpe0_dn8 / locals.var_b4soileffeot) / (2.0 * assign11070_e8165)), ((locals.var_pparam_b4soilpe0_dn9 / locals.var_b4soileffeot) / (2.0 * assign11070_e8165)), ((locals.var_pparam_b4soilpe0_dn10 / locals.var_b4soileffeot) / (2.0 * assign11070_e8165)), ((locals.var_pparam_b4soilpe0_dn11 / locals.var_b4soileffeot) / (2.0 * assign11070_e8165)), ((locals.var_pparam_b4soilpe0_dn12 / locals.var_b4soileffeot) / (2.0 * assign11070_e8165)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign11070_e8167;
        locals.var_t0_dn3 = assign11070_e8167_d_n3;
        locals.var_t0_dn4 = assign11070_e8167_d_n4;
        locals.var_t0_dn5 = assign11070_e8167_d_n5;
        locals.var_t0_dn6 = assign11070_e8167_d_n6;
        locals.var_t0_dn7 = assign11070_e8167_d_n7;
        locals.var_t0_dn8 = assign11070_e8167_d_n8;
        locals.var_t0_dn9 = assign11070_e8167_d_n9;
        locals.var_t0_dn10 = assign11070_e8167_d_n10;
        locals.var_t0_dn11 = assign11070_e8167_d_n11;
        locals.var_t0_dn12 = assign11070_e8167_d_n12;

        let (assign11080_e8176, assign11080_e8176_d_n3, assign11080_e8176_d_n4, assign11080_e8176_d_n5, assign11080_e8176_d_n6, assign11080_e8176_d_n7, assign11080_e8176_d_n8, assign11080_e8176_d_n9, assign11080_e8176_d_n10, assign11080_e8176_d_n11, assign11080_e8176_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11080_e8173: f64 = (locals.var_pparam_b4soikt1l / locals.var_b4soileffeot);
        let assign11080_e8174: f64 = (locals.var_pparam_b4soikt1 + assign11080_e8173);
        (assign11080_e8174, (locals.var_pparam_b4soikt1_dn3 + (locals.var_pparam_b4soikt1l_dn3 / locals.var_b4soileffeot)), (locals.var_pparam_b4soikt1_dn4 + (locals.var_pparam_b4soikt1l_dn4 / locals.var_b4soileffeot)), (locals.var_pparam_b4soikt1_dn5 + (locals.var_pparam_b4soikt1l_dn5 / locals.var_b4soileffeot)), (locals.var_pparam_b4soikt1_dn6 + (locals.var_pparam_b4soikt1l_dn6 / locals.var_b4soileffeot)), (locals.var_pparam_b4soikt1_dn7 + (locals.var_pparam_b4soikt1l_dn7 / locals.var_b4soileffeot)), (locals.var_pparam_b4soikt1_dn8 + (locals.var_pparam_b4soikt1l_dn8 / locals.var_b4soileffeot)), (locals.var_pparam_b4soikt1_dn9 + (locals.var_pparam_b4soikt1l_dn9 / locals.var_b4soileffeot)), (locals.var_pparam_b4soikt1_dn10 + (locals.var_pparam_b4soikt1l_dn10 / locals.var_b4soileffeot)), (locals.var_pparam_b4soikt1_dn11 + (locals.var_pparam_b4soikt1l_dn11 / locals.var_b4soileffeot)), (locals.var_pparam_b4soikt1_dn12 + (locals.var_pparam_b4soikt1l_dn12 / locals.var_b4soileffeot)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign11080_e8176;
        locals.var_t1_dn3 = assign11080_e8176_d_n3;
        locals.var_t1_dn4 = assign11080_e8176_d_n4;
        locals.var_t1_dn5 = assign11080_e8176_d_n5;
        locals.var_t1_dn6 = assign11080_e8176_d_n6;
        locals.var_t1_dn7 = assign11080_e8176_d_n7;
        locals.var_t1_dn8 = assign11080_e8176_d_n8;
        locals.var_t1_dn9 = assign11080_e8176_d_n9;
        locals.var_t1_dn10 = assign11080_e8176_d_n10;
        locals.var_t1_dn11 = assign11080_e8176_d_n11;
        locals.var_t1_dn12 = assign11080_e8176_d_n12;

        let (assign11090_e8191, assign11090_e8191_d_n3, assign11090_e8191_d_n4, assign11090_e8191_d_n5, assign11090_e8191_d_n6, assign11090_e8191_d_n7, assign11090_e8191_d_n8, assign11090_e8191_d_n9, assign11090_e8191_d_n10, assign11090_e8191_d_n11, assign11090_e8191_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11090_e8182: f64 = (locals.var_t0 - 1.0);
        let assign11090_e8183: f64 = (locals.var_pparam_b4soik1ox * assign11090_e8182);
        let assign11090_e8185: f64 = (assign11090_e8183 * locals.var_sqrtphieot);
        let assign11090_e8188: f64 = (locals.var_t1 * locals.var_tempratiominus1__blk828);
        let assign11090_e8189: f64 = (assign11090_e8185 + assign11090_e8188);
        (assign11090_e8189, (((((locals.var_pparam_b4soik1ox_dn3 * assign11090_e8182) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn3)) * locals.var_sqrtphieot) + (assign11090_e8183 * locals.var_sqrtphieot_dn3)) + (locals.var_t1_dn3 * locals.var_tempratiominus1__blk828)), (((((locals.var_pparam_b4soik1ox_dn4 * assign11090_e8182) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn4)) * locals.var_sqrtphieot) + (assign11090_e8183 * locals.var_sqrtphieot_dn4)) + (locals.var_t1_dn4 * locals.var_tempratiominus1__blk828)), (((((locals.var_pparam_b4soik1ox_dn5 * assign11090_e8182) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn5)) * locals.var_sqrtphieot) + (assign11090_e8183 * locals.var_sqrtphieot_dn5)) + (locals.var_t1_dn5 * locals.var_tempratiominus1__blk828)), (((((locals.var_pparam_b4soik1ox_dn6 * assign11090_e8182) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn6)) * locals.var_sqrtphieot) + (assign11090_e8183 * locals.var_sqrtphieot_dn6)) + (locals.var_t1_dn6 * locals.var_tempratiominus1__blk828)), (((((locals.var_pparam_b4soik1ox_dn7 * assign11090_e8182) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn7)) * locals.var_sqrtphieot) + (assign11090_e8183 * locals.var_sqrtphieot_dn7)) + (locals.var_t1_dn7 * locals.var_tempratiominus1__blk828)), (((((locals.var_pparam_b4soik1ox_dn8 * assign11090_e8182) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn8)) * locals.var_sqrtphieot) + (assign11090_e8183 * locals.var_sqrtphieot_dn8)) + (locals.var_t1_dn8 * locals.var_tempratiominus1__blk828)), (((((locals.var_pparam_b4soik1ox_dn9 * assign11090_e8182) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn9)) * locals.var_sqrtphieot) + (assign11090_e8183 * locals.var_sqrtphieot_dn9)) + (locals.var_t1_dn9 * locals.var_tempratiominus1__blk828)), (((((locals.var_pparam_b4soik1ox_dn10 * assign11090_e8182) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn10)) * locals.var_sqrtphieot) + (assign11090_e8183 * locals.var_sqrtphieot_dn10)) + (locals.var_t1_dn10 * locals.var_tempratiominus1__blk828)), (((((locals.var_pparam_b4soik1ox_dn11 * assign11090_e8182) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn11)) * locals.var_sqrtphieot) + (assign11090_e8183 * locals.var_sqrtphieot_dn11)) + (locals.var_t1_dn11 * locals.var_tempratiominus1__blk828)), (((((locals.var_pparam_b4soik1ox_dn12 * assign11090_e8182) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn12)) * locals.var_sqrtphieot) + (assign11090_e8183 * locals.var_sqrtphieot_dn12)) + (locals.var_t1_dn12 * locals.var_tempratiominus1__blk828)),)
    } else {
        (locals.var_deltvthtemp, locals.var_deltvthtemp_dn3, locals.var_deltvthtemp_dn4, locals.var_deltvthtemp_dn5, locals.var_deltvthtemp_dn6, locals.var_deltvthtemp_dn7, locals.var_deltvthtemp_dn8, locals.var_deltvthtemp_dn9, locals.var_deltvthtemp_dn10, locals.var_deltvthtemp_dn11, locals.var_deltvthtemp_dn12,)
    }
};
        locals.var_deltvthtemp = assign11090_e8191;
        locals.var_deltvthtemp_dn3 = assign11090_e8191_d_n3;
        locals.var_deltvthtemp_dn4 = assign11090_e8191_d_n4;
        locals.var_deltvthtemp_dn5 = assign11090_e8191_d_n5;
        locals.var_deltvthtemp_dn6 = assign11090_e8191_d_n6;
        locals.var_deltvthtemp_dn7 = assign11090_e8191_d_n7;
        locals.var_deltvthtemp_dn8 = assign11090_e8191_d_n8;
        locals.var_deltvthtemp_dn9 = assign11090_e8191_d_n9;
        locals.var_deltvthtemp_dn10 = assign11090_e8191_d_n10;
        locals.var_deltvthtemp_dn11 = assign11090_e8191_d_n11;
        locals.var_deltvthtemp_dn12 = assign11090_e8191_d_n12;

        let (assign11100_e8202, assign11100_e8202_d_n3, assign11100_e8202_d_n4, assign11100_e8202_d_n5, assign11100_e8202_d_n6, assign11100_e8202_d_n7, assign11100_e8202_d_n8, assign11100_e8202_d_n9, assign11100_e8202_d_n10, assign11100_e8202_d_n11, assign11100_e8202_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11100_e8196: f64 = (locals.var_toxe * locals.var_phieot);
        let assign11100_e8199: f64 = (locals.var_b4soiweffeot + locals.var_pparam_b4soiw0);
        let assign11100_e8200: f64 = (assign11100_e8196 / assign11100_e8199);
        (assign11100_e8200, ((((locals.var_toxe * locals.var_phieot_dn3) * assign11100_e8199) - (assign11100_e8196 * locals.var_pparam_b4soiw0_dn3)) / (assign11100_e8199 * assign11100_e8199)), ((((locals.var_toxe * locals.var_phieot_dn4) * assign11100_e8199) - (assign11100_e8196 * locals.var_pparam_b4soiw0_dn4)) / (assign11100_e8199 * assign11100_e8199)), ((((locals.var_toxe * locals.var_phieot_dn5) * assign11100_e8199) - (assign11100_e8196 * locals.var_pparam_b4soiw0_dn5)) / (assign11100_e8199 * assign11100_e8199)), ((((locals.var_toxe * locals.var_phieot_dn6) * assign11100_e8199) - (assign11100_e8196 * locals.var_pparam_b4soiw0_dn6)) / (assign11100_e8199 * assign11100_e8199)), ((((locals.var_toxe * locals.var_phieot_dn7) * assign11100_e8199) - (assign11100_e8196 * locals.var_pparam_b4soiw0_dn7)) / (assign11100_e8199 * assign11100_e8199)), ((((locals.var_toxe * locals.var_phieot_dn8) * assign11100_e8199) - (assign11100_e8196 * locals.var_pparam_b4soiw0_dn8)) / (assign11100_e8199 * assign11100_e8199)), ((((locals.var_toxe * locals.var_phieot_dn9) * assign11100_e8199) - (assign11100_e8196 * locals.var_pparam_b4soiw0_dn9)) / (assign11100_e8199 * assign11100_e8199)), ((((locals.var_toxe * locals.var_phieot_dn10) * assign11100_e8199) - (assign11100_e8196 * locals.var_pparam_b4soiw0_dn10)) / (assign11100_e8199 * assign11100_e8199)), ((((locals.var_toxe * locals.var_phieot_dn11) * assign11100_e8199) - (assign11100_e8196 * locals.var_pparam_b4soiw0_dn11)) / (assign11100_e8199 * assign11100_e8199)), ((((locals.var_toxe * locals.var_phieot_dn12) * assign11100_e8199) - (assign11100_e8196 * locals.var_pparam_b4soiw0_dn12)) / (assign11100_e8199 * assign11100_e8199)),)
    } else {
        (locals.var_tmp2, locals.var_tmp2_dn3, locals.var_tmp2_dn4, locals.var_tmp2_dn5, locals.var_tmp2_dn6, locals.var_tmp2_dn7, locals.var_tmp2_dn8, locals.var_tmp2_dn9, locals.var_tmp2_dn10, locals.var_tmp2_dn11, locals.var_tmp2_dn12,)
    }
};
        locals.var_tmp2 = assign11100_e8202;
        locals.var_tmp2_dn3 = assign11100_e8202_d_n3;
        locals.var_tmp2_dn4 = assign11100_e8202_d_n4;
        locals.var_tmp2_dn5 = assign11100_e8202_d_n5;
        locals.var_tmp2_dn6 = assign11100_e8202_d_n6;
        locals.var_tmp2_dn7 = assign11100_e8202_d_n7;
        locals.var_tmp2_dn8 = assign11100_e8202_d_n8;
        locals.var_tmp2_dn9 = assign11100_e8202_d_n9;
        locals.var_tmp2_dn10 = assign11100_e8202_d_n10;
        locals.var_tmp2_dn11 = assign11100_e8202_d_n11;
        locals.var_tmp2_dn12 = assign11100_e8202_d_n12;

        let (assign11110_e8207,) = {
    if (locals.var_guard940 == 0.0) {
        (0.0,)
    } else {
        (locals.var_dibl_sft,)
    }
};
        locals.var_dibl_sft = assign11110_e8207;

        let (assign11120_e8212,) = {
    if (locals.var_guard940 == 0.0) {
        (0.0,)
    } else {
        (locals.var_dits_sft2,)
    }
};
        locals.var_dits_sft2 = assign11120_e8212;

        let (assign11130_e8222, assign11130_e8222_d_n3, assign11130_e8222_d_n4, assign11130_e8222_d_n5, assign11130_e8222_d_n6, assign11130_e8222_d_n7, assign11130_e8222_d_n8, assign11130_e8222_d_n9, assign11130_e8222_d_n10, assign11130_e8222_d_n11, assign11130_e8222_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11130_e8218: f64 = (locals.var_pparam_b4soilpeb / locals.var_b4soileffeot);
        let assign11130_e8219: f64 = (1.0 + assign11130_e8218);
        let assign11130_e8220: f64 = (assign11130_e8219).sqrt();
        (assign11130_e8220, ((locals.var_pparam_b4soilpeb_dn3 / locals.var_b4soileffeot) / (2.0 * assign11130_e8220)), ((locals.var_pparam_b4soilpeb_dn4 / locals.var_b4soileffeot) / (2.0 * assign11130_e8220)), ((locals.var_pparam_b4soilpeb_dn5 / locals.var_b4soileffeot) / (2.0 * assign11130_e8220)), ((locals.var_pparam_b4soilpeb_dn6 / locals.var_b4soileffeot) / (2.0 * assign11130_e8220)), ((locals.var_pparam_b4soilpeb_dn7 / locals.var_b4soileffeot) / (2.0 * assign11130_e8220)), ((locals.var_pparam_b4soilpeb_dn8 / locals.var_b4soileffeot) / (2.0 * assign11130_e8220)), ((locals.var_pparam_b4soilpeb_dn9 / locals.var_b4soileffeot) / (2.0 * assign11130_e8220)), ((locals.var_pparam_b4soilpeb_dn10 / locals.var_b4soileffeot) / (2.0 * assign11130_e8220)), ((locals.var_pparam_b4soilpeb_dn11 / locals.var_b4soileffeot) / (2.0 * assign11130_e8220)), ((locals.var_pparam_b4soilpeb_dn12 / locals.var_b4soileffeot) / (2.0 * assign11130_e8220)),)
    } else {
        (locals.var_lpe_vb, locals.var_lpe_vb_dn3, locals.var_lpe_vb_dn4, locals.var_lpe_vb_dn5, locals.var_lpe_vb_dn6, locals.var_lpe_vb_dn7, locals.var_lpe_vb_dn8, locals.var_lpe_vb_dn9, locals.var_lpe_vb_dn10, locals.var_lpe_vb_dn11, locals.var_lpe_vb_dn12,)
    }
};
        locals.var_lpe_vb = assign11130_e8222;
        locals.var_lpe_vb_dn3 = assign11130_e8222_d_n3;
        locals.var_lpe_vb_dn4 = assign11130_e8222_d_n4;
        locals.var_lpe_vb_dn5 = assign11130_e8222_d_n5;
        locals.var_lpe_vb_dn6 = assign11130_e8222_d_n6;
        locals.var_lpe_vb_dn7 = assign11130_e8222_d_n7;
        locals.var_lpe_vb_dn8 = assign11130_e8222_d_n8;
        locals.var_lpe_vb_dn9 = assign11130_e8222_d_n9;
        locals.var_lpe_vb_dn10 = assign11130_e8222_d_n10;
        locals.var_lpe_vb_dn11 = assign11130_e8222_d_n11;
        locals.var_lpe_vb_dn12 = assign11130_e8222_d_n12;

        let (assign11140_e8227, assign11140_e8227_d_n3, assign11140_e8227_d_n4, assign11140_e8227_d_n5, assign11140_e8227_d_n6, assign11140_e8227_d_n7, assign11140_e8227_d_n8, assign11140_e8227_d_n9, assign11140_e8227_d_n10, assign11140_e8227_d_n11, assign11140_e8227_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        (locals.var_sqrtphieot, locals.var_sqrtphieot_dn3, locals.var_sqrtphieot_dn4, locals.var_sqrtphieot_dn5, locals.var_sqrtphieot_dn6, locals.var_sqrtphieot_dn7, locals.var_sqrtphieot_dn8, locals.var_sqrtphieot_dn9, locals.var_sqrtphieot_dn10, locals.var_sqrtphieot_dn11, locals.var_sqrtphieot_dn12,)
    } else {
        (locals.var_sqrtphisext, locals.var_sqrtphisext_dn3, locals.var_sqrtphisext_dn4, locals.var_sqrtphisext_dn5, locals.var_sqrtphisext_dn6, locals.var_sqrtphisext_dn7, locals.var_sqrtphisext_dn8, locals.var_sqrtphisext_dn9, locals.var_sqrtphisext_dn10, locals.var_sqrtphisext_dn11, locals.var_sqrtphisext_dn12,)
    }
};
        locals.var_sqrtphisext = assign11140_e8227;
        locals.var_sqrtphisext_dn3 = assign11140_e8227_d_n3;
        locals.var_sqrtphisext_dn4 = assign11140_e8227_d_n4;
        locals.var_sqrtphisext_dn5 = assign11140_e8227_d_n5;
        locals.var_sqrtphisext_dn6 = assign11140_e8227_d_n6;
        locals.var_sqrtphisext_dn7 = assign11140_e8227_d_n7;
        locals.var_sqrtphisext_dn8 = assign11140_e8227_d_n8;
        locals.var_sqrtphisext_dn9 = assign11140_e8227_d_n9;
        locals.var_sqrtphisext_dn10 = assign11140_e8227_d_n10;
        locals.var_sqrtphisext_dn11 = assign11140_e8227_d_n11;
        locals.var_sqrtphisext_dn12 = assign11140_e8227_d_n12;

        let (assign11150_e8260, assign11150_e8260_d_n3, assign11150_e8260_d_n4, assign11150_e8260_d_n5, assign11150_e8260_d_n6, assign11150_e8260_d_n7, assign11150_e8260_d_n8, assign11150_e8260_d_n9, assign11150_e8260_d_n10, assign11150_e8260_d_n11, assign11150_e8260_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11150_e8232: f64 = (locals.var_b4soitype * locals.var_here_b4soivth0);
        let assign11150_e8235: f64 = (locals.var_pparam_b4soik1ox * locals.var_sqrtphisext);
        let assign11150_e8238: f64 = (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot);
        let assign11150_e8239: f64 = (assign11150_e8235 - assign11150_e8238);
        let assign11150_e8241: f64 = (assign11150_e8239 * locals.var_lpe_vb);
        let assign11150_e8242: f64 = (assign11150_e8232 + assign11150_e8241);
        let assign11150_e8244: f64 = (assign11150_e8242 - locals.var_delt_vth);
        let assign11150_e8246: f64 = (assign11150_e8244 - locals.var_deltvthw);
        let assign11150_e8249: f64 = (locals.var_pparam_b4soik3 * locals.var_tmp2);
        let assign11150_e8250: f64 = (assign11150_e8246 + assign11150_e8249);
        let assign11150_e8252: f64 = (assign11150_e8250 + locals.var_deltvthtemp);
        let assign11150_e8254: f64 = (assign11150_e8252 - locals.var_dibl_sft);
        let assign11150_e8256: f64 = (assign11150_e8254 - locals.var_dits_sft);
        let assign11150_e8258: f64 = (assign11150_e8256 - locals.var_dits_sft2);
        (assign11150_e8258, (((((((locals.var_b4soitype * locals.var_here_b4soivth0_dn3) + (((((locals.var_pparam_b4soik1ox_dn3 * locals.var_sqrtphisext) + (locals.var_pparam_b4soik1ox * locals.var_sqrtphisext_dn3)) - ((locals.var_pparam_b4soik1eff_dn3 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn3))) * locals.var_lpe_vb) + (assign11150_e8239 * locals.var_lpe_vb_dn3))) - locals.var_delt_vth_dn3) - locals.var_deltvthw_dn3) + ((locals.var_pparam_b4soik3_dn3 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn3))) + locals.var_deltvthtemp_dn3) - locals.var_dits_sft_dn3), (((((((locals.var_b4soitype * locals.var_here_b4soivth0_dn4) + (((((locals.var_pparam_b4soik1ox_dn4 * locals.var_sqrtphisext) + (locals.var_pparam_b4soik1ox * locals.var_sqrtphisext_dn4)) - ((locals.var_pparam_b4soik1eff_dn4 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn4))) * locals.var_lpe_vb) + (assign11150_e8239 * locals.var_lpe_vb_dn4))) - locals.var_delt_vth_dn4) - locals.var_deltvthw_dn4) + ((locals.var_pparam_b4soik3_dn4 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn4))) + locals.var_deltvthtemp_dn4) - locals.var_dits_sft_dn4), (((((((locals.var_b4soitype * locals.var_here_b4soivth0_dn5) + (((((locals.var_pparam_b4soik1ox_dn5 * locals.var_sqrtphisext) + (locals.var_pparam_b4soik1ox * locals.var_sqrtphisext_dn5)) - ((locals.var_pparam_b4soik1eff_dn5 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn5))) * locals.var_lpe_vb) + (assign11150_e8239 * locals.var_lpe_vb_dn5))) - locals.var_delt_vth_dn5) - locals.var_deltvthw_dn5) + ((locals.var_pparam_b4soik3_dn5 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn5))) + locals.var_deltvthtemp_dn5) - locals.var_dits_sft_dn5), (((((((locals.var_b4soitype * locals.var_here_b4soivth0_dn6) + (((((locals.var_pparam_b4soik1ox_dn6 * locals.var_sqrtphisext) + (locals.var_pparam_b4soik1ox * locals.var_sqrtphisext_dn6)) - ((locals.var_pparam_b4soik1eff_dn6 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn6))) * locals.var_lpe_vb) + (assign11150_e8239 * locals.var_lpe_vb_dn6))) - locals.var_delt_vth_dn6) - locals.var_deltvthw_dn6) + ((locals.var_pparam_b4soik3_dn6 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn6))) + locals.var_deltvthtemp_dn6) - locals.var_dits_sft_dn6), (((((((locals.var_b4soitype * locals.var_here_b4soivth0_dn7) + (((((locals.var_pparam_b4soik1ox_dn7 * locals.var_sqrtphisext) + (locals.var_pparam_b4soik1ox * locals.var_sqrtphisext_dn7)) - ((locals.var_pparam_b4soik1eff_dn7 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn7))) * locals.var_lpe_vb) + (assign11150_e8239 * locals.var_lpe_vb_dn7))) - locals.var_delt_vth_dn7) - locals.var_deltvthw_dn7) + ((locals.var_pparam_b4soik3_dn7 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn7))) + locals.var_deltvthtemp_dn7) - locals.var_dits_sft_dn7), (((((((locals.var_b4soitype * locals.var_here_b4soivth0_dn8) + (((((locals.var_pparam_b4soik1ox_dn8 * locals.var_sqrtphisext) + (locals.var_pparam_b4soik1ox * locals.var_sqrtphisext_dn8)) - ((locals.var_pparam_b4soik1eff_dn8 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn8))) * locals.var_lpe_vb) + (assign11150_e8239 * locals.var_lpe_vb_dn8))) - locals.var_delt_vth_dn8) - locals.var_deltvthw_dn8) + ((locals.var_pparam_b4soik3_dn8 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn8))) + locals.var_deltvthtemp_dn8) - locals.var_dits_sft_dn8), (((((((locals.var_b4soitype * locals.var_here_b4soivth0_dn9) + (((((locals.var_pparam_b4soik1ox_dn9 * locals.var_sqrtphisext) + (locals.var_pparam_b4soik1ox * locals.var_sqrtphisext_dn9)) - ((locals.var_pparam_b4soik1eff_dn9 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn9))) * locals.var_lpe_vb) + (assign11150_e8239 * locals.var_lpe_vb_dn9))) - locals.var_delt_vth_dn9) - locals.var_deltvthw_dn9) + ((locals.var_pparam_b4soik3_dn9 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn9))) + locals.var_deltvthtemp_dn9) - locals.var_dits_sft_dn9), (((((((locals.var_b4soitype * locals.var_here_b4soivth0_dn10) + (((((locals.var_pparam_b4soik1ox_dn10 * locals.var_sqrtphisext) + (locals.var_pparam_b4soik1ox * locals.var_sqrtphisext_dn10)) - ((locals.var_pparam_b4soik1eff_dn10 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn10))) * locals.var_lpe_vb) + (assign11150_e8239 * locals.var_lpe_vb_dn10))) - locals.var_delt_vth_dn10) - locals.var_deltvthw_dn10) + ((locals.var_pparam_b4soik3_dn10 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn10))) + locals.var_deltvthtemp_dn10) - locals.var_dits_sft_dn10), (((((((locals.var_b4soitype * locals.var_here_b4soivth0_dn11) + (((((locals.var_pparam_b4soik1ox_dn11 * locals.var_sqrtphisext) + (locals.var_pparam_b4soik1ox * locals.var_sqrtphisext_dn11)) - ((locals.var_pparam_b4soik1eff_dn11 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn11))) * locals.var_lpe_vb) + (assign11150_e8239 * locals.var_lpe_vb_dn11))) - locals.var_delt_vth_dn11) - locals.var_deltvthw_dn11) + ((locals.var_pparam_b4soik3_dn11 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn11))) + locals.var_deltvthtemp_dn11) - locals.var_dits_sft_dn11), (((((((locals.var_b4soitype * locals.var_here_b4soivth0_dn12) + (((((locals.var_pparam_b4soik1ox_dn12 * locals.var_sqrtphisext) + (locals.var_pparam_b4soik1ox * locals.var_sqrtphisext_dn12)) - ((locals.var_pparam_b4soik1eff_dn12 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn12))) * locals.var_lpe_vb) + (assign11150_e8239 * locals.var_lpe_vb_dn12))) - locals.var_delt_vth_dn12) - locals.var_deltvthw_dn12) + ((locals.var_pparam_b4soik3_dn12 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn12))) + locals.var_deltvthtemp_dn12) - locals.var_dits_sft_dn12),)
    } else {
        (locals.var_vth_1, locals.var_vth_1_dn3, locals.var_vth_1_dn4, locals.var_vth_1_dn5, locals.var_vth_1_dn6, locals.var_vth_1_dn7, locals.var_vth_1_dn8, locals.var_vth_1_dn9, locals.var_vth_1_dn10, locals.var_vth_1_dn11, locals.var_vth_1_dn12,)
    }
};
        locals.var_vth_1 = assign11150_e8260;
        locals.var_vth_1_dn3 = assign11150_e8260_d_n3;
        locals.var_vth_1_dn4 = assign11150_e8260_d_n4;
        locals.var_vth_1_dn5 = assign11150_e8260_d_n5;
        locals.var_vth_1_dn6 = assign11150_e8260_d_n6;
        locals.var_vth_1_dn7 = assign11150_e8260_d_n7;
        locals.var_vth_1_dn8 = assign11150_e8260_d_n8;
        locals.var_vth_1_dn9 = assign11150_e8260_d_n9;
        locals.var_vth_1_dn10 = assign11150_e8260_d_n10;
        locals.var_vth_1_dn11 = assign11150_e8260_d_n11;
        locals.var_vth_1_dn12 = assign11150_e8260_d_n12;

        let (assign11160_e8267, assign11160_e8267_d_n3, assign11160_e8267_d_n4, assign11160_e8267_d_n5, assign11160_e8267_d_n6, assign11160_e8267_d_n7, assign11160_e8267_d_n8, assign11160_e8267_d_n9, assign11160_e8267_d_n10, assign11160_e8267_d_n11, assign11160_e8267_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11160_e8265: f64 = (locals.var_vgs_eff - locals.var_vth_1);
        (assign11160_e8265, (locals.var_vgs_eff_dn3 - locals.var_vth_1_dn3), (locals.var_vgs_eff_dn4 - locals.var_vth_1_dn4), (locals.var_vgs_eff_dn5 - locals.var_vth_1_dn5), (locals.var_vgs_eff_dn6 - locals.var_vth_1_dn6), (locals.var_vgs_eff_dn7 - locals.var_vth_1_dn7), (locals.var_vgs_eff_dn8 - locals.var_vth_1_dn8), (locals.var_vgs_eff_dn9 - locals.var_vth_1_dn9), (locals.var_vgs_eff_dn10 - locals.var_vth_1_dn10), (locals.var_vgs_eff_dn11 - locals.var_vth_1_dn11), (locals.var_vgs_eff_dn12 - locals.var_vth_1_dn12),)
    } else {
        (locals.var_vgst, locals.var_vgst_dn3, locals.var_vgst_dn4, locals.var_vgst_dn5, locals.var_vgst_dn6, locals.var_vgst_dn7, locals.var_vgst_dn8, locals.var_vgst_dn9, locals.var_vgst_dn10, locals.var_vgst_dn11, locals.var_vgst_dn12,)
    }
};
        locals.var_vgst = assign11160_e8267;
        locals.var_vgst_dn3 = assign11160_e8267_d_n3;
        locals.var_vgst_dn4 = assign11160_e8267_d_n4;
        locals.var_vgst_dn5 = assign11160_e8267_d_n5;
        locals.var_vgst_dn6 = assign11160_e8267_d_n6;
        locals.var_vgst_dn7 = assign11160_e8267_d_n7;
        locals.var_vgst_dn8 = assign11160_e8267_d_n8;
        locals.var_vgst_dn9 = assign11160_e8267_d_n9;
        locals.var_vgst_dn10 = assign11160_e8267_d_n10;
        locals.var_vgst_dn11 = assign11160_e8267_d_n11;
        locals.var_vgst_dn12 = assign11160_e8267_d_n12;

    }

    pub(super) fn stamp_transient_block_23(
        locals: &mut StampLocals,
    ) {
        let (assign11170_e8274, assign11170_e8274_d_n3, assign11170_e8274_d_n4, assign11170_e8274_d_n5, assign11170_e8274_d_n6, assign11170_e8274_d_n7, assign11170_e8274_d_n8, assign11170_e8274_d_n9, assign11170_e8274_d_n10, assign11170_e8274_d_n11, assign11170_e8274_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11170_e8272: f64 = (locals.var_n * locals.var_vtmeot);
        (assign11170_e8272, (locals.var_n_dn3 * locals.var_vtmeot), (locals.var_n_dn4 * locals.var_vtmeot), (locals.var_n_dn5 * locals.var_vtmeot), (locals.var_n_dn6 * locals.var_vtmeot), (locals.var_n_dn7 * locals.var_vtmeot), (locals.var_n_dn8 * locals.var_vtmeot), (locals.var_n_dn9 * locals.var_vtmeot), (locals.var_n_dn10 * locals.var_vtmeot), (locals.var_n_dn11 * locals.var_vtmeot), (locals.var_n_dn12 * locals.var_vtmeot),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12,)
    }
};
        locals.var_t10 = assign11170_e8274;
        locals.var_t10_dn3 = assign11170_e8274_d_n3;
        locals.var_t10_dn4 = assign11170_e8274_d_n4;
        locals.var_t10_dn5 = assign11170_e8274_d_n5;
        locals.var_t10_dn6 = assign11170_e8274_d_n6;
        locals.var_t10_dn7 = assign11170_e8274_d_n7;
        locals.var_t10_dn8 = assign11170_e8274_d_n8;
        locals.var_t10_dn9 = assign11170_e8274_d_n9;
        locals.var_t10_dn10 = assign11170_e8274_d_n10;
        locals.var_t10_dn11 = assign11170_e8274_d_n11;
        locals.var_t10_dn12 = assign11170_e8274_d_n12;

        let (assign11180_e8283, assign11180_e8283_d_n3, assign11180_e8283_d_n4, assign11180_e8283_d_n5, assign11180_e8283_d_n6, assign11180_e8283_d_n7, assign11180_e8283_d_n8, assign11180_e8283_d_n9, assign11180_e8283_d_n10, assign11180_e8283_d_n11, assign11180_e8283_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11180_e8279: f64 = (locals.var_pparam_b4soimstar * locals.var_vgst);
        let assign11180_e8281: f64 = (assign11180_e8279 / locals.var_t10);
        (assign11180_e8281, (((((locals.var_pparam_b4soimstar_dn3 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn3)) * locals.var_t10) - (assign11180_e8279 * locals.var_t10_dn3)) / (locals.var_t10 * locals.var_t10)), (((((locals.var_pparam_b4soimstar_dn4 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn4)) * locals.var_t10) - (assign11180_e8279 * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)), (((((locals.var_pparam_b4soimstar_dn5 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn5)) * locals.var_t10) - (assign11180_e8279 * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)), (((((locals.var_pparam_b4soimstar_dn6 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn6)) * locals.var_t10) - (assign11180_e8279 * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)), (((((locals.var_pparam_b4soimstar_dn7 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn7)) * locals.var_t10) - (assign11180_e8279 * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)), (((((locals.var_pparam_b4soimstar_dn8 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn8)) * locals.var_t10) - (assign11180_e8279 * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)), (((((locals.var_pparam_b4soimstar_dn9 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn9)) * locals.var_t10) - (assign11180_e8279 * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)), (((((locals.var_pparam_b4soimstar_dn10 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn10)) * locals.var_t10) - (assign11180_e8279 * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)), (((((locals.var_pparam_b4soimstar_dn11 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn11)) * locals.var_t10) - (assign11180_e8279 * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)), (((((locals.var_pparam_b4soimstar_dn12 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn12)) * locals.var_t10) - (assign11180_e8279 * locals.var_t10_dn12)) / (locals.var_t10 * locals.var_t10)),)
    } else {
        (locals.var_vgstnvt, locals.var_vgstnvt_dn3, locals.var_vgstnvt_dn4, locals.var_vgstnvt_dn5, locals.var_vgstnvt_dn6, locals.var_vgstnvt_dn7, locals.var_vgstnvt_dn8, locals.var_vgstnvt_dn9, locals.var_vgstnvt_dn10, locals.var_vgstnvt_dn11, locals.var_vgstnvt_dn12,)
    }
};
        locals.var_vgstnvt = assign11180_e8283;
        locals.var_vgstnvt_dn3 = assign11180_e8283_d_n3;
        locals.var_vgstnvt_dn4 = assign11180_e8283_d_n4;
        locals.var_vgstnvt_dn5 = assign11180_e8283_d_n5;
        locals.var_vgstnvt_dn6 = assign11180_e8283_d_n6;
        locals.var_vgstnvt_dn7 = assign11180_e8283_d_n7;
        locals.var_vgstnvt_dn8 = assign11180_e8283_d_n8;
        locals.var_vgstnvt_dn9 = assign11180_e8283_d_n9;
        locals.var_vgstnvt_dn10 = assign11180_e8283_d_n10;
        locals.var_vgstnvt_dn11 = assign11180_e8283_d_n11;
        locals.var_vgstnvt_dn12 = assign11180_e8283_d_n12;

        let (assign11190_e8296, assign11190_e8296_d_n3, assign11190_e8296_d_n4, assign11190_e8296_d_n5, assign11190_e8296_d_n6, assign11190_e8296_d_n7, assign11190_e8296_d_n8, assign11190_e8296_d_n9, assign11190_e8296_d_n10, assign11190_e8296_d_n11, assign11190_e8296_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11190_e8289: f64 = (1.0 - locals.var_pparam_b4soimstar);
        let assign11190_e8291: f64 = (assign11190_e8289 * locals.var_vgst);
        let assign11190_e8292: f64 = (locals.var_pparam_b4soivoff - assign11190_e8291);
        let assign11190_e8294: f64 = (assign11190_e8292 / locals.var_t10);
        (assign11190_e8294, ((((locals.var_pparam_b4soivoff_dn3 - (((-locals.var_pparam_b4soimstar_dn3) * locals.var_vgst) + (assign11190_e8289 * locals.var_vgst_dn3))) * locals.var_t10) - (assign11190_e8292 * locals.var_t10_dn3)) / (locals.var_t10 * locals.var_t10)), ((((locals.var_pparam_b4soivoff_dn4 - (((-locals.var_pparam_b4soimstar_dn4) * locals.var_vgst) + (assign11190_e8289 * locals.var_vgst_dn4))) * locals.var_t10) - (assign11190_e8292 * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)), ((((locals.var_pparam_b4soivoff_dn5 - (((-locals.var_pparam_b4soimstar_dn5) * locals.var_vgst) + (assign11190_e8289 * locals.var_vgst_dn5))) * locals.var_t10) - (assign11190_e8292 * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)), ((((locals.var_pparam_b4soivoff_dn6 - (((-locals.var_pparam_b4soimstar_dn6) * locals.var_vgst) + (assign11190_e8289 * locals.var_vgst_dn6))) * locals.var_t10) - (assign11190_e8292 * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)), ((((locals.var_pparam_b4soivoff_dn7 - (((-locals.var_pparam_b4soimstar_dn7) * locals.var_vgst) + (assign11190_e8289 * locals.var_vgst_dn7))) * locals.var_t10) - (assign11190_e8292 * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)), ((((locals.var_pparam_b4soivoff_dn8 - (((-locals.var_pparam_b4soimstar_dn8) * locals.var_vgst) + (assign11190_e8289 * locals.var_vgst_dn8))) * locals.var_t10) - (assign11190_e8292 * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)), ((((locals.var_pparam_b4soivoff_dn9 - (((-locals.var_pparam_b4soimstar_dn9) * locals.var_vgst) + (assign11190_e8289 * locals.var_vgst_dn9))) * locals.var_t10) - (assign11190_e8292 * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)), ((((locals.var_pparam_b4soivoff_dn10 - (((-locals.var_pparam_b4soimstar_dn10) * locals.var_vgst) + (assign11190_e8289 * locals.var_vgst_dn10))) * locals.var_t10) - (assign11190_e8292 * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)), ((((locals.var_pparam_b4soivoff_dn11 - (((-locals.var_pparam_b4soimstar_dn11) * locals.var_vgst) + (assign11190_e8289 * locals.var_vgst_dn11))) * locals.var_t10) - (assign11190_e8292 * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)), ((((locals.var_pparam_b4soivoff_dn12 - (((-locals.var_pparam_b4soimstar_dn12) * locals.var_vgst) + (assign11190_e8289 * locals.var_vgst_dn12))) * locals.var_t10) - (assign11190_e8292 * locals.var_t10_dn12)) / (locals.var_t10 * locals.var_t10)),)
    } else {
        (locals.var_exparg, locals.var_exparg_dn3, locals.var_exparg_dn4, locals.var_exparg_dn5, locals.var_exparg_dn6, locals.var_exparg_dn7, locals.var_exparg_dn8, locals.var_exparg_dn9, locals.var_exparg_dn10, locals.var_exparg_dn11, locals.var_exparg_dn12,)
    }
};
        locals.var_exparg = assign11190_e8296;
        locals.var_exparg_dn3 = assign11190_e8296_d_n3;
        locals.var_exparg_dn4 = assign11190_e8296_d_n4;
        locals.var_exparg_dn5 = assign11190_e8296_d_n5;
        locals.var_exparg_dn6 = assign11190_e8296_d_n6;
        locals.var_exparg_dn7 = assign11190_e8296_d_n7;
        locals.var_exparg_dn8 = assign11190_e8296_d_n8;
        locals.var_exparg_dn9 = assign11190_e8296_d_n9;
        locals.var_exparg_dn10 = assign11190_e8296_d_n10;
        locals.var_exparg_dn11 = assign11190_e8296_d_n11;
        locals.var_exparg_dn12 = assign11190_e8296_d_n12;

        let assign11200_e8299: f64 = if locals.var_vgstnvt > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard946 = assign11200_e8299;

        let (assign11210_e8306, assign11210_e8306_d_n3, assign11210_e8306_d_n4, assign11210_e8306_d_n5, assign11210_e8306_d_n6, assign11210_e8306_d_n7, assign11210_e8306_d_n8, assign11210_e8306_d_n9, assign11210_e8306_d_n10, assign11210_e8306_d_n11, assign11210_e8306_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard946 != 0.0)) {
        (locals.var_vgst, locals.var_vgst_dn3, locals.var_vgst_dn4, locals.var_vgst_dn5, locals.var_vgst_dn6, locals.var_vgst_dn7, locals.var_vgst_dn8, locals.var_vgst_dn9, locals.var_vgst_dn10, locals.var_vgst_dn11, locals.var_vgst_dn12,)
    } else {
        (locals.var_vgsteff, locals.var_vgsteff_dn3, locals.var_vgsteff_dn4, locals.var_vgsteff_dn5, locals.var_vgsteff_dn6, locals.var_vgsteff_dn7, locals.var_vgsteff_dn8, locals.var_vgsteff_dn9, locals.var_vgsteff_dn10, locals.var_vgsteff_dn11, locals.var_vgsteff_dn12,)
    }
};
        locals.var_vgsteff = assign11210_e8306;
        locals.var_vgsteff_dn3 = assign11210_e8306_d_n3;
        locals.var_vgsteff_dn4 = assign11210_e8306_d_n4;
        locals.var_vgsteff_dn5 = assign11210_e8306_d_n5;
        locals.var_vgsteff_dn6 = assign11210_e8306_d_n6;
        locals.var_vgsteff_dn7 = assign11210_e8306_d_n7;
        locals.var_vgsteff_dn8 = assign11210_e8306_d_n8;
        locals.var_vgsteff_dn9 = assign11210_e8306_d_n9;
        locals.var_vgsteff_dn10 = assign11210_e8306_d_n10;
        locals.var_vgsteff_dn11 = assign11210_e8306_d_n11;
        locals.var_vgsteff_dn12 = assign11210_e8306_d_n12;

        let assign11220_e8309: f64 = if locals.var_exparg > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard947 = assign11220_e8309;

        let (assign11230_e8325, assign11230_e8325_d_n3, assign11230_e8325_d_n4, assign11230_e8325_d_n5, assign11230_e8325_d_n6, assign11230_e8325_d_n7, assign11230_e8325_d_n8, assign11230_e8325_d_n9, assign11230_e8325_d_n10, assign11230_e8325_d_n11, assign11230_e8325_d_n12,) = {
    if (((locals.var_guard940 == 0.0) && (locals.var_guard946 == 0.0)) && (locals.var_guard947 != 0.0)) {
        let assign11230_e8319: f64 = (locals.var_vgst - locals.var_pparam_b4soivoff);
        let assign11230_e8322: f64 = (locals.var_n * locals.var_vtmeot);
        let assign11230_e8323: f64 = (assign11230_e8319 / assign11230_e8322);
        (assign11230_e8323, ((((locals.var_vgst_dn3 - locals.var_pparam_b4soivoff_dn3) * assign11230_e8322) - (assign11230_e8319 * (locals.var_n_dn3 * locals.var_vtmeot))) / (assign11230_e8322 * assign11230_e8322)), ((((locals.var_vgst_dn4 - locals.var_pparam_b4soivoff_dn4) * assign11230_e8322) - (assign11230_e8319 * (locals.var_n_dn4 * locals.var_vtmeot))) / (assign11230_e8322 * assign11230_e8322)), ((((locals.var_vgst_dn5 - locals.var_pparam_b4soivoff_dn5) * assign11230_e8322) - (assign11230_e8319 * (locals.var_n_dn5 * locals.var_vtmeot))) / (assign11230_e8322 * assign11230_e8322)), ((((locals.var_vgst_dn6 - locals.var_pparam_b4soivoff_dn6) * assign11230_e8322) - (assign11230_e8319 * (locals.var_n_dn6 * locals.var_vtmeot))) / (assign11230_e8322 * assign11230_e8322)), ((((locals.var_vgst_dn7 - locals.var_pparam_b4soivoff_dn7) * assign11230_e8322) - (assign11230_e8319 * (locals.var_n_dn7 * locals.var_vtmeot))) / (assign11230_e8322 * assign11230_e8322)), ((((locals.var_vgst_dn8 - locals.var_pparam_b4soivoff_dn8) * assign11230_e8322) - (assign11230_e8319 * (locals.var_n_dn8 * locals.var_vtmeot))) / (assign11230_e8322 * assign11230_e8322)), ((((locals.var_vgst_dn9 - locals.var_pparam_b4soivoff_dn9) * assign11230_e8322) - (assign11230_e8319 * (locals.var_n_dn9 * locals.var_vtmeot))) / (assign11230_e8322 * assign11230_e8322)), ((((locals.var_vgst_dn10 - locals.var_pparam_b4soivoff_dn10) * assign11230_e8322) - (assign11230_e8319 * (locals.var_n_dn10 * locals.var_vtmeot))) / (assign11230_e8322 * assign11230_e8322)), ((((locals.var_vgst_dn11 - locals.var_pparam_b4soivoff_dn11) * assign11230_e8322) - (assign11230_e8319 * (locals.var_n_dn11 * locals.var_vtmeot))) / (assign11230_e8322 * assign11230_e8322)), ((((locals.var_vgst_dn12 - locals.var_pparam_b4soivoff_dn12) * assign11230_e8322) - (assign11230_e8319 * (locals.var_n_dn12 * locals.var_vtmeot))) / (assign11230_e8322 * assign11230_e8322)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign11230_e8325;
        locals.var_t0_dn3 = assign11230_e8325_d_n3;
        locals.var_t0_dn4 = assign11230_e8325_d_n4;
        locals.var_t0_dn5 = assign11230_e8325_d_n5;
        locals.var_t0_dn6 = assign11230_e8325_d_n6;
        locals.var_t0_dn7 = assign11230_e8325_d_n7;
        locals.var_t0_dn8 = assign11230_e8325_d_n8;
        locals.var_t0_dn9 = assign11230_e8325_d_n9;
        locals.var_t0_dn10 = assign11230_e8325_d_n10;
        locals.var_t0_dn11 = assign11230_e8325_d_n11;
        locals.var_t0_dn12 = assign11230_e8325_d_n12;

        let (assign11240_e8336, assign11240_e8336_d_n3, assign11240_e8336_d_n4, assign11240_e8336_d_n5, assign11240_e8336_d_n6, assign11240_e8336_d_n7, assign11240_e8336_d_n8, assign11240_e8336_d_n9, assign11240_e8336_d_n10, assign11240_e8336_d_n11, assign11240_e8336_d_n12,) = {
    if (((locals.var_guard940 == 0.0) && (locals.var_guard946 == 0.0)) && (locals.var_guard947 != 0.0)) {
        let assign11240_e8334: f64 = (locals.var_t0).exp();
        (assign11240_e8334, (assign11240_e8334 * locals.var_t0_dn3), (assign11240_e8334 * locals.var_t0_dn4), (assign11240_e8334 * locals.var_t0_dn5), (assign11240_e8334 * locals.var_t0_dn6), (assign11240_e8334 * locals.var_t0_dn7), (assign11240_e8334 * locals.var_t0_dn8), (assign11240_e8334 * locals.var_t0_dn9), (assign11240_e8334 * locals.var_t0_dn10), (assign11240_e8334 * locals.var_t0_dn11), (assign11240_e8334 * locals.var_t0_dn12),)
    } else {
        (locals.var_expvgst, locals.var_expvgst_dn3, locals.var_expvgst_dn4, locals.var_expvgst_dn5, locals.var_expvgst_dn6, locals.var_expvgst_dn7, locals.var_expvgst_dn8, locals.var_expvgst_dn9, locals.var_expvgst_dn10, locals.var_expvgst_dn11, locals.var_expvgst_dn12,)
    }
};
        locals.var_expvgst = assign11240_e8336;
        locals.var_expvgst_dn3 = assign11240_e8336_d_n3;
        locals.var_expvgst_dn4 = assign11240_e8336_d_n4;
        locals.var_expvgst_dn5 = assign11240_e8336_d_n5;
        locals.var_expvgst_dn6 = assign11240_e8336_d_n6;
        locals.var_expvgst_dn7 = assign11240_e8336_d_n7;
        locals.var_expvgst_dn8 = assign11240_e8336_d_n8;
        locals.var_expvgst_dn9 = assign11240_e8336_d_n9;
        locals.var_expvgst_dn10 = assign11240_e8336_d_n10;
        locals.var_expvgst_dn11 = assign11240_e8336_d_n11;
        locals.var_expvgst_dn12 = assign11240_e8336_d_n12;

        let (assign11250_e8352, assign11250_e8352_d_n3, assign11250_e8352_d_n4, assign11250_e8352_d_n5, assign11250_e8352_d_n6, assign11250_e8352_d_n7, assign11250_e8352_d_n8, assign11250_e8352_d_n9, assign11250_e8352_d_n10, assign11250_e8352_d_n11, assign11250_e8352_d_n12,) = {
    if (((locals.var_guard940 == 0.0) && (locals.var_guard946 == 0.0)) && (locals.var_guard947 != 0.0)) {
        let assign11250_e8346: f64 = (locals.var_vtmeot * locals.var_pparam_b4soicdep0);
        let assign11250_e8348: f64 = (assign11250_e8346 / locals.var_b4soicox);
        let assign11250_e8350: f64 = (assign11250_e8348 * locals.var_expvgst);
        (assign11250_e8350, ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn3) / locals.var_b4soicox) * locals.var_expvgst) + (assign11250_e8348 * locals.var_expvgst_dn3)), ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn4) / locals.var_b4soicox) * locals.var_expvgst) + (assign11250_e8348 * locals.var_expvgst_dn4)), ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn5) / locals.var_b4soicox) * locals.var_expvgst) + (assign11250_e8348 * locals.var_expvgst_dn5)), ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn6) / locals.var_b4soicox) * locals.var_expvgst) + (assign11250_e8348 * locals.var_expvgst_dn6)), ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn7) / locals.var_b4soicox) * locals.var_expvgst) + (assign11250_e8348 * locals.var_expvgst_dn7)), ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn8) / locals.var_b4soicox) * locals.var_expvgst) + (assign11250_e8348 * locals.var_expvgst_dn8)), ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn9) / locals.var_b4soicox) * locals.var_expvgst) + (assign11250_e8348 * locals.var_expvgst_dn9)), ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn10) / locals.var_b4soicox) * locals.var_expvgst) + (assign11250_e8348 * locals.var_expvgst_dn10)), ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn11) / locals.var_b4soicox) * locals.var_expvgst) + (assign11250_e8348 * locals.var_expvgst_dn11)), ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn12) / locals.var_b4soicox) * locals.var_expvgst) + (assign11250_e8348 * locals.var_expvgst_dn12)),)
    } else {
        (locals.var_vgsteff, locals.var_vgsteff_dn3, locals.var_vgsteff_dn4, locals.var_vgsteff_dn5, locals.var_vgsteff_dn6, locals.var_vgsteff_dn7, locals.var_vgsteff_dn8, locals.var_vgsteff_dn9, locals.var_vgsteff_dn10, locals.var_vgsteff_dn11, locals.var_vgsteff_dn12,)
    }
};
        locals.var_vgsteff = assign11250_e8352;
        locals.var_vgsteff_dn3 = assign11250_e8352_d_n3;
        locals.var_vgsteff_dn4 = assign11250_e8352_d_n4;
        locals.var_vgsteff_dn5 = assign11250_e8352_d_n5;
        locals.var_vgsteff_dn6 = assign11250_e8352_d_n6;
        locals.var_vgsteff_dn7 = assign11250_e8352_d_n7;
        locals.var_vgsteff_dn8 = assign11250_e8352_d_n8;
        locals.var_vgsteff_dn9 = assign11250_e8352_d_n9;
        locals.var_vgsteff_dn10 = assign11250_e8352_d_n10;
        locals.var_vgsteff_dn11 = assign11250_e8352_d_n11;
        locals.var_vgsteff_dn12 = assign11250_e8352_d_n12;

        let (assign11260_e8364, assign11260_e8364_d_n3, assign11260_e8364_d_n4, assign11260_e8364_d_n5, assign11260_e8364_d_n6, assign11260_e8364_d_n7, assign11260_e8364_d_n8, assign11260_e8364_d_n9, assign11260_e8364_d_n10, assign11260_e8364_d_n11, assign11260_e8364_d_n12,) = {
    if (((locals.var_guard940 == 0.0) && (locals.var_guard946 == 0.0)) && (locals.var_guard947 == 0.0)) {
        let assign11260_e8362: f64 = (locals.var_vgstnvt).exp();
        (assign11260_e8362, (assign11260_e8362 * locals.var_vgstnvt_dn3), (assign11260_e8362 * locals.var_vgstnvt_dn4), (assign11260_e8362 * locals.var_vgstnvt_dn5), (assign11260_e8362 * locals.var_vgstnvt_dn6), (assign11260_e8362 * locals.var_vgstnvt_dn7), (assign11260_e8362 * locals.var_vgstnvt_dn8), (assign11260_e8362 * locals.var_vgstnvt_dn9), (assign11260_e8362 * locals.var_vgstnvt_dn10), (assign11260_e8362 * locals.var_vgstnvt_dn11), (assign11260_e8362 * locals.var_vgstnvt_dn12),)
    } else {
        (locals.var_expvgst, locals.var_expvgst_dn3, locals.var_expvgst_dn4, locals.var_expvgst_dn5, locals.var_expvgst_dn6, locals.var_expvgst_dn7, locals.var_expvgst_dn8, locals.var_expvgst_dn9, locals.var_expvgst_dn10, locals.var_expvgst_dn11, locals.var_expvgst_dn12,)
    }
};
        locals.var_expvgst = assign11260_e8364;
        locals.var_expvgst_dn3 = assign11260_e8364_d_n3;
        locals.var_expvgst_dn4 = assign11260_e8364_d_n4;
        locals.var_expvgst_dn5 = assign11260_e8364_d_n5;
        locals.var_expvgst_dn6 = assign11260_e8364_d_n6;
        locals.var_expvgst_dn7 = assign11260_e8364_d_n7;
        locals.var_expvgst_dn8 = assign11260_e8364_d_n8;
        locals.var_expvgst_dn9 = assign11260_e8364_d_n9;
        locals.var_expvgst_dn10 = assign11260_e8364_d_n10;
        locals.var_expvgst_dn11 = assign11260_e8364_d_n11;
        locals.var_expvgst_dn12 = assign11260_e8364_d_n12;

        let (assign11270_e8388, assign11270_e8388_d_n3, assign11270_e8388_d_n4, assign11270_e8388_d_n5, assign11270_e8388_d_n6, assign11270_e8388_d_n7, assign11270_e8388_d_n8, assign11270_e8388_d_n9, assign11270_e8388_d_n10, assign11270_e8388_d_n11, assign11270_e8388_d_n12,) = {
    if (((locals.var_guard940 == 0.0) && (locals.var_guard946 == 0.0)) && (locals.var_guard947 == 0.0)) {
        let assign11270_e8376: f64 = (1.0 + locals.var_expvgst);
        let (assign11270_e8385, assign11270_e8385_d_n3, assign11270_e8385_d_n4, assign11270_e8385_d_n5, assign11270_e8385_d_n6, assign11270_e8385_d_n7, assign11270_e8385_d_n8, assign11270_e8385_d_n9, assign11270_e8385_d_n10, assign11270_e8385_d_n11, assign11270_e8385_d_n12,) = {
            if (assign11270_e8376 > 1e-38) {
                let assign11270_e8381: f64 = (1.0 + locals.var_expvgst);
                let assign11270_e8382: f64 = (assign11270_e8381).ln();
                (assign11270_e8382, (locals.var_expvgst_dn3 / assign11270_e8381), (locals.var_expvgst_dn4 / assign11270_e8381), (locals.var_expvgst_dn5 / assign11270_e8381), (locals.var_expvgst_dn6 / assign11270_e8381), (locals.var_expvgst_dn7 / assign11270_e8381), (locals.var_expvgst_dn8 / assign11270_e8381), (locals.var_expvgst_dn9 / assign11270_e8381), (locals.var_expvgst_dn10 / assign11270_e8381), (locals.var_expvgst_dn11 / assign11270_e8381), (locals.var_expvgst_dn12 / assign11270_e8381),)
            } else {
                let assign11270_e8384: f64 = (-87.49823353377374);
                (assign11270_e8384, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign11270_e8386: f64 = (locals.var_t10 * assign11270_e8385);
        (assign11270_e8386, ((locals.var_t10_dn3 * assign11270_e8385) + (locals.var_t10 * assign11270_e8385_d_n3)), ((locals.var_t10_dn4 * assign11270_e8385) + (locals.var_t10 * assign11270_e8385_d_n4)), ((locals.var_t10_dn5 * assign11270_e8385) + (locals.var_t10 * assign11270_e8385_d_n5)), ((locals.var_t10_dn6 * assign11270_e8385) + (locals.var_t10 * assign11270_e8385_d_n6)), ((locals.var_t10_dn7 * assign11270_e8385) + (locals.var_t10 * assign11270_e8385_d_n7)), ((locals.var_t10_dn8 * assign11270_e8385) + (locals.var_t10 * assign11270_e8385_d_n8)), ((locals.var_t10_dn9 * assign11270_e8385) + (locals.var_t10 * assign11270_e8385_d_n9)), ((locals.var_t10_dn10 * assign11270_e8385) + (locals.var_t10 * assign11270_e8385_d_n10)), ((locals.var_t10_dn11 * assign11270_e8385) + (locals.var_t10 * assign11270_e8385_d_n11)), ((locals.var_t10_dn12 * assign11270_e8385) + (locals.var_t10 * assign11270_e8385_d_n12)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign11270_e8388;
        locals.var_t1_dn3 = assign11270_e8388_d_n3;
        locals.var_t1_dn4 = assign11270_e8388_d_n4;
        locals.var_t1_dn5 = assign11270_e8388_d_n5;
        locals.var_t1_dn6 = assign11270_e8388_d_n6;
        locals.var_t1_dn7 = assign11270_e8388_d_n7;
        locals.var_t1_dn8 = assign11270_e8388_d_n8;
        locals.var_t1_dn9 = assign11270_e8388_d_n9;
        locals.var_t1_dn10 = assign11270_e8388_d_n10;
        locals.var_t1_dn11 = assign11270_e8388_d_n11;
        locals.var_t1_dn12 = assign11270_e8388_d_n12;

        let (assign11280_e8411, assign11280_e8411_d_n3, assign11280_e8411_d_n4, assign11280_e8411_d_n5, assign11280_e8411_d_n6, assign11280_e8411_d_n7, assign11280_e8411_d_n8, assign11280_e8411_d_n9, assign11280_e8411_d_n10, assign11280_e8411_d_n11, assign11280_e8411_d_n12,) = {
    if (((locals.var_guard940 == 0.0) && (locals.var_guard946 == 0.0)) && (locals.var_guard947 == 0.0)) {
        let assign11280_e8398: f64 = (-locals.var_b4soicox);
        let assign11280_e8401: f64 = (locals.var_vtm0eot * locals.var_pparam_b4soicdep0);
        let assign11280_e8402: f64 = (assign11280_e8398 / assign11280_e8401);
        let assign11280_e8404: f64 = (locals.var_exparg).exp();
        let assign11280_e8405: f64 = (assign11280_e8402 * assign11280_e8404);
        let assign11280_e8408: f64 = (1.0 - locals.var_pparam_b4soimstar);
        let assign11280_e8409: f64 = (assign11280_e8405 * assign11280_e8408);
        (assign11280_e8409, (((((-((assign11280_e8398 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn3)) / (assign11280_e8401 * assign11280_e8401))) * assign11280_e8404) + (assign11280_e8402 * (assign11280_e8404 * locals.var_exparg_dn3))) * assign11280_e8408) + (assign11280_e8405 * (-locals.var_pparam_b4soimstar_dn3))), (((((-((assign11280_e8398 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn4)) / (assign11280_e8401 * assign11280_e8401))) * assign11280_e8404) + (assign11280_e8402 * (assign11280_e8404 * locals.var_exparg_dn4))) * assign11280_e8408) + (assign11280_e8405 * (-locals.var_pparam_b4soimstar_dn4))), (((((-((assign11280_e8398 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn5)) / (assign11280_e8401 * assign11280_e8401))) * assign11280_e8404) + (assign11280_e8402 * (assign11280_e8404 * locals.var_exparg_dn5))) * assign11280_e8408) + (assign11280_e8405 * (-locals.var_pparam_b4soimstar_dn5))), (((((-((assign11280_e8398 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn6)) / (assign11280_e8401 * assign11280_e8401))) * assign11280_e8404) + (assign11280_e8402 * (assign11280_e8404 * locals.var_exparg_dn6))) * assign11280_e8408) + (assign11280_e8405 * (-locals.var_pparam_b4soimstar_dn6))), (((((-((assign11280_e8398 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn7)) / (assign11280_e8401 * assign11280_e8401))) * assign11280_e8404) + (assign11280_e8402 * (assign11280_e8404 * locals.var_exparg_dn7))) * assign11280_e8408) + (assign11280_e8405 * (-locals.var_pparam_b4soimstar_dn7))), (((((-((assign11280_e8398 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn8)) / (assign11280_e8401 * assign11280_e8401))) * assign11280_e8404) + (assign11280_e8402 * (assign11280_e8404 * locals.var_exparg_dn8))) * assign11280_e8408) + (assign11280_e8405 * (-locals.var_pparam_b4soimstar_dn8))), (((((-((assign11280_e8398 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn9)) / (assign11280_e8401 * assign11280_e8401))) * assign11280_e8404) + (assign11280_e8402 * (assign11280_e8404 * locals.var_exparg_dn9))) * assign11280_e8408) + (assign11280_e8405 * (-locals.var_pparam_b4soimstar_dn9))), (((((-((assign11280_e8398 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn10)) / (assign11280_e8401 * assign11280_e8401))) * assign11280_e8404) + (assign11280_e8402 * (assign11280_e8404 * locals.var_exparg_dn10))) * assign11280_e8408) + (assign11280_e8405 * (-locals.var_pparam_b4soimstar_dn10))), (((((-((assign11280_e8398 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn11)) / (assign11280_e8401 * assign11280_e8401))) * assign11280_e8404) + (assign11280_e8402 * (assign11280_e8404 * locals.var_exparg_dn11))) * assign11280_e8408) + (assign11280_e8405 * (-locals.var_pparam_b4soimstar_dn11))), (((((-((assign11280_e8398 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn12)) / (assign11280_e8401 * assign11280_e8401))) * assign11280_e8404) + (assign11280_e8402 * (assign11280_e8404 * locals.var_exparg_dn12))) * assign11280_e8408) + (assign11280_e8405 * (-locals.var_pparam_b4soimstar_dn12))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign11280_e8411;
        locals.var_t4_dn3 = assign11280_e8411_d_n3;
        locals.var_t4_dn4 = assign11280_e8411_d_n4;
        locals.var_t4_dn5 = assign11280_e8411_d_n5;
        locals.var_t4_dn6 = assign11280_e8411_d_n6;
        locals.var_t4_dn7 = assign11280_e8411_d_n7;
        locals.var_t4_dn8 = assign11280_e8411_d_n8;
        locals.var_t4_dn9 = assign11280_e8411_d_n9;
        locals.var_t4_dn10 = assign11280_e8411_d_n10;
        locals.var_t4_dn11 = assign11280_e8411_d_n11;
        locals.var_t4_dn12 = assign11280_e8411_d_n12;

        let (assign11290_e8430, assign11290_e8430_d_n3, assign11290_e8430_d_n4, assign11290_e8430_d_n5, assign11290_e8430_d_n6, assign11290_e8430_d_n7, assign11290_e8430_d_n8, assign11290_e8430_d_n9, assign11290_e8430_d_n10, assign11290_e8430_d_n11, assign11290_e8430_d_n12,) = {
    if (((locals.var_guard940 == 0.0) && (locals.var_guard946 == 0.0)) && (locals.var_guard947 == 0.0)) {
        let assign11290_e8423: f64 = (locals.var_t10 * locals.var_t4);
        let assign11290_e8426: f64 = (1.0 - locals.var_pparam_b4soimstar);
        let assign11290_e8427: f64 = (assign11290_e8423 / assign11290_e8426);
        let assign11290_e8428: f64 = (locals.var_pparam_b4soimstar - assign11290_e8427);
        (assign11290_e8428, (locals.var_pparam_b4soimstar_dn3 - (((((locals.var_t10_dn3 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn3)) * assign11290_e8426) - (assign11290_e8423 * (-locals.var_pparam_b4soimstar_dn3))) / (assign11290_e8426 * assign11290_e8426))), (locals.var_pparam_b4soimstar_dn4 - (((((locals.var_t10_dn4 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn4)) * assign11290_e8426) - (assign11290_e8423 * (-locals.var_pparam_b4soimstar_dn4))) / (assign11290_e8426 * assign11290_e8426))), (locals.var_pparam_b4soimstar_dn5 - (((((locals.var_t10_dn5 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn5)) * assign11290_e8426) - (assign11290_e8423 * (-locals.var_pparam_b4soimstar_dn5))) / (assign11290_e8426 * assign11290_e8426))), (locals.var_pparam_b4soimstar_dn6 - (((((locals.var_t10_dn6 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn6)) * assign11290_e8426) - (assign11290_e8423 * (-locals.var_pparam_b4soimstar_dn6))) / (assign11290_e8426 * assign11290_e8426))), (locals.var_pparam_b4soimstar_dn7 - (((((locals.var_t10_dn7 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn7)) * assign11290_e8426) - (assign11290_e8423 * (-locals.var_pparam_b4soimstar_dn7))) / (assign11290_e8426 * assign11290_e8426))), (locals.var_pparam_b4soimstar_dn8 - (((((locals.var_t10_dn8 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn8)) * assign11290_e8426) - (assign11290_e8423 * (-locals.var_pparam_b4soimstar_dn8))) / (assign11290_e8426 * assign11290_e8426))), (locals.var_pparam_b4soimstar_dn9 - (((((locals.var_t10_dn9 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn9)) * assign11290_e8426) - (assign11290_e8423 * (-locals.var_pparam_b4soimstar_dn9))) / (assign11290_e8426 * assign11290_e8426))), (locals.var_pparam_b4soimstar_dn10 - (((((locals.var_t10_dn10 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn10)) * assign11290_e8426) - (assign11290_e8423 * (-locals.var_pparam_b4soimstar_dn10))) / (assign11290_e8426 * assign11290_e8426))), (locals.var_pparam_b4soimstar_dn11 - (((((locals.var_t10_dn11 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn11)) * assign11290_e8426) - (assign11290_e8423 * (-locals.var_pparam_b4soimstar_dn11))) / (assign11290_e8426 * assign11290_e8426))), (locals.var_pparam_b4soimstar_dn12 - (((((locals.var_t10_dn12 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn12)) * assign11290_e8426) - (assign11290_e8423 * (-locals.var_pparam_b4soimstar_dn12))) / (assign11290_e8426 * assign11290_e8426))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign11290_e8430;
        locals.var_t2_dn3 = assign11290_e8430_d_n3;
        locals.var_t2_dn4 = assign11290_e8430_d_n4;
        locals.var_t2_dn5 = assign11290_e8430_d_n5;
        locals.var_t2_dn6 = assign11290_e8430_d_n6;
        locals.var_t2_dn7 = assign11290_e8430_d_n7;
        locals.var_t2_dn8 = assign11290_e8430_d_n8;
        locals.var_t2_dn9 = assign11290_e8430_d_n9;
        locals.var_t2_dn10 = assign11290_e8430_d_n10;
        locals.var_t2_dn11 = assign11290_e8430_d_n11;
        locals.var_t2_dn12 = assign11290_e8430_d_n12;

        let (assign11300_e8443, assign11300_e8443_d_n3, assign11300_e8443_d_n4, assign11300_e8443_d_n5, assign11300_e8443_d_n6, assign11300_e8443_d_n7, assign11300_e8443_d_n8, assign11300_e8443_d_n9, assign11300_e8443_d_n10, assign11300_e8443_d_n11, assign11300_e8443_d_n12,) = {
    if (((locals.var_guard940 == 0.0) && (locals.var_guard946 == 0.0)) && (locals.var_guard947 == 0.0)) {
        let assign11300_e8441: f64 = (locals.var_t1 / locals.var_t2);
        (assign11300_e8441, (((locals.var_t1_dn3 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn3)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn12 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn12)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_vgsteff, locals.var_vgsteff_dn3, locals.var_vgsteff_dn4, locals.var_vgsteff_dn5, locals.var_vgsteff_dn6, locals.var_vgsteff_dn7, locals.var_vgsteff_dn8, locals.var_vgsteff_dn9, locals.var_vgsteff_dn10, locals.var_vgsteff_dn11, locals.var_vgsteff_dn12,)
    }
};
        locals.var_vgsteff = assign11300_e8443;
        locals.var_vgsteff_dn3 = assign11300_e8443_d_n3;
        locals.var_vgsteff_dn4 = assign11300_e8443_d_n4;
        locals.var_vgsteff_dn5 = assign11300_e8443_d_n5;
        locals.var_vgsteff_dn6 = assign11300_e8443_d_n6;
        locals.var_vgsteff_dn7 = assign11300_e8443_d_n7;
        locals.var_vgsteff_dn8 = assign11300_e8443_d_n8;
        locals.var_vgsteff_dn9 = assign11300_e8443_d_n9;
        locals.var_vgsteff_dn10 = assign11300_e8443_d_n10;
        locals.var_vgsteff_dn11 = assign11300_e8443_d_n11;
        locals.var_vgsteff_dn12 = assign11300_e8443_d_n12;

        let (assign11310_e8454, assign11310_e8454_d_n3, assign11310_e8454_d_n4, assign11310_e8454_d_n5, assign11310_e8454_d_n6, assign11310_e8454_d_n7, assign11310_e8454_d_n8, assign11310_e8454_d_n9, assign11310_e8454_d_n10, assign11310_e8454_d_n11, assign11310_e8454_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11310_e8448: f64 = (locals.var_b4soitype * locals.var_here_b4soivth0);
        let assign11310_e8450: f64 = (assign11310_e8448 - locals.var_here_b4soivfb);
        let assign11310_e8452: f64 = (assign11310_e8450 - locals.var_phieot);
        (assign11310_e8452, (((locals.var_b4soitype * locals.var_here_b4soivth0_dn3) - locals.var_here_b4soivfb_dn3) - locals.var_phieot_dn3), (((locals.var_b4soitype * locals.var_here_b4soivth0_dn4) - locals.var_here_b4soivfb_dn4) - locals.var_phieot_dn4), (((locals.var_b4soitype * locals.var_here_b4soivth0_dn5) - locals.var_here_b4soivfb_dn5) - locals.var_phieot_dn5), (((locals.var_b4soitype * locals.var_here_b4soivth0_dn6) - locals.var_here_b4soivfb_dn6) - locals.var_phieot_dn6), (((locals.var_b4soitype * locals.var_here_b4soivth0_dn7) - locals.var_here_b4soivfb_dn7) - locals.var_phieot_dn7), (((locals.var_b4soitype * locals.var_here_b4soivth0_dn8) - locals.var_here_b4soivfb_dn8) - locals.var_phieot_dn8), (((locals.var_b4soitype * locals.var_here_b4soivth0_dn9) - locals.var_here_b4soivfb_dn9) - locals.var_phieot_dn9), (((locals.var_b4soitype * locals.var_here_b4soivth0_dn10) - locals.var_here_b4soivfb_dn10) - locals.var_phieot_dn10), (((locals.var_b4soitype * locals.var_here_b4soivth0_dn11) - locals.var_here_b4soivfb_dn11) - locals.var_phieot_dn11), (((locals.var_b4soitype * locals.var_here_b4soivth0_dn12) - locals.var_here_b4soivfb_dn12) - locals.var_phieot_dn12),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign11310_e8454;
        locals.var_t3_dn3 = assign11310_e8454_d_n3;
        locals.var_t3_dn4 = assign11310_e8454_d_n4;
        locals.var_t3_dn5 = assign11310_e8454_d_n5;
        locals.var_t3_dn6 = assign11310_e8454_d_n6;
        locals.var_t3_dn7 = assign11310_e8454_d_n7;
        locals.var_t3_dn8 = assign11310_e8454_d_n8;
        locals.var_t3_dn9 = assign11310_e8454_d_n9;
        locals.var_t3_dn10 = assign11310_e8454_d_n10;
        locals.var_t3_dn11 = assign11310_e8454_d_n11;
        locals.var_t3_dn12 = assign11310_e8454_d_n12;

        let (assign11320_e8461, assign11320_e8461_d_n3, assign11320_e8461_d_n4, assign11320_e8461_d_n5, assign11320_e8461_d_n6, assign11320_e8461_d_n7, assign11320_e8461_d_n8, assign11320_e8461_d_n9, assign11320_e8461_d_n10, assign11320_e8461_d_n11, assign11320_e8461_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11320_e8459: f64 = (4.0 * locals.var_t3);
        (assign11320_e8459, (4.0 * locals.var_t3_dn3), (4.0 * locals.var_t3_dn4), (4.0 * locals.var_t3_dn5), (4.0 * locals.var_t3_dn6), (4.0 * locals.var_t3_dn7), (4.0 * locals.var_t3_dn8), (4.0 * locals.var_t3_dn9), (4.0 * locals.var_t3_dn10), (4.0 * locals.var_t3_dn11), (4.0 * locals.var_t3_dn12),)
    } else {
        (locals.var_vtfbphi2eot, locals.var_vtfbphi2eot_dn3, locals.var_vtfbphi2eot_dn4, locals.var_vtfbphi2eot_dn5, locals.var_vtfbphi2eot_dn6, locals.var_vtfbphi2eot_dn7, locals.var_vtfbphi2eot_dn8, locals.var_vtfbphi2eot_dn9, locals.var_vtfbphi2eot_dn10, locals.var_vtfbphi2eot_dn11, locals.var_vtfbphi2eot_dn12,)
    }
};
        locals.var_vtfbphi2eot = assign11320_e8461;
        locals.var_vtfbphi2eot_dn3 = assign11320_e8461_d_n3;
        locals.var_vtfbphi2eot_dn4 = assign11320_e8461_d_n4;
        locals.var_vtfbphi2eot_dn5 = assign11320_e8461_d_n5;
        locals.var_vtfbphi2eot_dn6 = assign11320_e8461_d_n6;
        locals.var_vtfbphi2eot_dn7 = assign11320_e8461_d_n7;
        locals.var_vtfbphi2eot_dn8 = assign11320_e8461_d_n8;
        locals.var_vtfbphi2eot_dn9 = assign11320_e8461_d_n9;
        locals.var_vtfbphi2eot_dn10 = assign11320_e8461_d_n10;
        locals.var_vtfbphi2eot_dn11 = assign11320_e8461_d_n11;
        locals.var_vtfbphi2eot_dn12 = assign11320_e8461_d_n12;

        let assign11330_e8464: f64 = if locals.var_vtfbphi2eot < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard948 = assign11330_e8464;

        let (assign11340_e8471, assign11340_e8471_d_n3, assign11340_e8471_d_n4, assign11340_e8471_d_n5, assign11340_e8471_d_n6, assign11340_e8471_d_n7, assign11340_e8471_d_n8, assign11340_e8471_d_n9, assign11340_e8471_d_n10, assign11340_e8471_d_n11, assign11340_e8471_d_n12,) = {
    if ((locals.var_guard940 == 0.0) && (locals.var_guard948 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vtfbphi2eot, locals.var_vtfbphi2eot_dn3, locals.var_vtfbphi2eot_dn4, locals.var_vtfbphi2eot_dn5, locals.var_vtfbphi2eot_dn6, locals.var_vtfbphi2eot_dn7, locals.var_vtfbphi2eot_dn8, locals.var_vtfbphi2eot_dn9, locals.var_vtfbphi2eot_dn10, locals.var_vtfbphi2eot_dn11, locals.var_vtfbphi2eot_dn12,)
    }
};
        locals.var_vtfbphi2eot = assign11340_e8471;
        locals.var_vtfbphi2eot_dn3 = assign11340_e8471_d_n3;
        locals.var_vtfbphi2eot_dn4 = assign11340_e8471_d_n4;
        locals.var_vtfbphi2eot_dn5 = assign11340_e8471_d_n5;
        locals.var_vtfbphi2eot_dn6 = assign11340_e8471_d_n6;
        locals.var_vtfbphi2eot_dn7 = assign11340_e8471_d_n7;
        locals.var_vtfbphi2eot_dn8 = assign11340_e8471_d_n8;
        locals.var_vtfbphi2eot_dn9 = assign11340_e8471_d_n9;
        locals.var_vtfbphi2eot_dn10 = assign11340_e8471_d_n10;
        locals.var_vtfbphi2eot_dn11 = assign11340_e8471_d_n11;
        locals.var_vtfbphi2eot_dn12 = assign11340_e8471_d_n12;

        let (assign11350_e8476,) = {
    if (locals.var_guard940 == 0.0) {
        (0.0,)
    } else {
        (locals.var_niter,)
    }
};
        locals.var_niter = assign11350_e8476;

        let (assign11360_e8481, assign11360_e8481_d_n3, assign11360_e8481_d_n4, assign11360_e8481_d_n5, assign11360_e8481_d_n6, assign11360_e8481_d_n7, assign11360_e8481_d_n8, assign11360_e8481_d_n9, assign11360_e8481_d_n10, assign11360_e8481_d_n11, assign11360_e8481_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        (locals.var_toxe, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_toxpf, locals.var_toxpf_dn3, locals.var_toxpf_dn4, locals.var_toxpf_dn5, locals.var_toxpf_dn6, locals.var_toxpf_dn7, locals.var_toxpf_dn8, locals.var_toxpf_dn9, locals.var_toxpf_dn10, locals.var_toxpf_dn11, locals.var_toxpf_dn12,)
    }
};
        locals.var_toxpf = assign11360_e8481;
        locals.var_toxpf_dn3 = assign11360_e8481_d_n3;
        locals.var_toxpf_dn4 = assign11360_e8481_d_n4;
        locals.var_toxpf_dn5 = assign11360_e8481_d_n5;
        locals.var_toxpf_dn6 = assign11360_e8481_d_n6;
        locals.var_toxpf_dn7 = assign11360_e8481_d_n7;
        locals.var_toxpf_dn8 = assign11360_e8481_d_n8;
        locals.var_toxpf_dn9 = assign11360_e8481_d_n9;
        locals.var_toxpf_dn10 = assign11360_e8481_d_n10;
        locals.var_toxpf_dn11 = assign11360_e8481_d_n11;
        locals.var_toxpf_dn12 = assign11360_e8481_d_n12;

        let (assign11370_e8486,) = {
    if (locals.var_guard940 == 0.0) {
        (1000000.0,)
    } else {
        (locals.var_toxpi,)
    }
};
        locals.var_toxpi = assign11370_e8486;

    }

    pub(super) fn stamp_transient_block_24(
        locals: &mut StampLocals,
    ) {
        let mut assign11380_loop_guard: usize = 0;
        while {
            let assign11380_cond_e8494: f64 = (locals.var_toxpf - locals.var_toxpi);
            let assign11380_cond_e8495: f64 = (assign11380_cond_e8494).abs();
            let assign11380_cond_e8495_d_n3: f64 = if assign11380_cond_e8494 >= 0.0 { locals.var_toxpf_dn3 } else { (-locals.var_toxpf_dn3) };
            let assign11380_cond_e8495_d_n4: f64 = if assign11380_cond_e8494 >= 0.0 { locals.var_toxpf_dn4 } else { (-locals.var_toxpf_dn4) };
            let assign11380_cond_e8495_d_n5: f64 = if assign11380_cond_e8494 >= 0.0 { locals.var_toxpf_dn5 } else { (-locals.var_toxpf_dn5) };
            let assign11380_cond_e8495_d_n6: f64 = if assign11380_cond_e8494 >= 0.0 { locals.var_toxpf_dn6 } else { (-locals.var_toxpf_dn6) };
            let assign11380_cond_e8495_d_n7: f64 = if assign11380_cond_e8494 >= 0.0 { locals.var_toxpf_dn7 } else { (-locals.var_toxpf_dn7) };
            let assign11380_cond_e8495_d_n8: f64 = if assign11380_cond_e8494 >= 0.0 { locals.var_toxpf_dn8 } else { (-locals.var_toxpf_dn8) };
            let assign11380_cond_e8495_d_n9: f64 = if assign11380_cond_e8494 >= 0.0 { locals.var_toxpf_dn9 } else { (-locals.var_toxpf_dn9) };
            let assign11380_cond_e8495_d_n10: f64 = if assign11380_cond_e8494 >= 0.0 { locals.var_toxpf_dn10 } else { (-locals.var_toxpf_dn10) };
            let assign11380_cond_e8495_d_n11: f64 = if assign11380_cond_e8494 >= 0.0 { locals.var_toxpf_dn11 } else { (-locals.var_toxpf_dn11) };
            let assign11380_cond_e8495_d_n12: f64 = if assign11380_cond_e8494 >= 0.0 { locals.var_toxpf_dn12 } else { (-locals.var_toxpf_dn12) };
            let assign11380_cond_e8499: f64 = if ((locals.var_guard940 == 0.0) && ((locals.var_niter <= 4.0) && (assign11380_cond_e8495 > 1e-12))) { 1.0 } else { 0.0 };
            assign11380_cond_e8499 != 0.0
        } {
            assign11380_loop_guard += 1;
            assert!(assign11380_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign11380_body0_e8504,) = {
    if (locals.var_guard940 == 0.0) {
        (locals.var_toxpf,)
    } else {
        (locals.var_toxpi,)
    }
};
            locals.var_toxpi = assign11380_body0_e8504;
            let (assign11380_body1_e8511, assign11380_body1_e8511_d_n3, assign11380_body1_e8511_d_n4, assign11380_body1_e8511_d_n5, assign11380_body1_e8511_d_n6, assign11380_body1_e8511_d_n7, assign11380_body1_e8511_d_n8, assign11380_body1_e8511_d_n9, assign11380_body1_e8511_d_n10, assign11380_body1_e8511_d_n11, assign11380_body1_e8511_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11380_body1_e8509: f64 = (200000000.0 * locals.var_toxpf);
        (assign11380_body1_e8509, (200000000.0 * locals.var_toxpf_dn3), (200000000.0 * locals.var_toxpf_dn4), (200000000.0 * locals.var_toxpf_dn5), (200000000.0 * locals.var_toxpf_dn6), (200000000.0 * locals.var_toxpf_dn7), (200000000.0 * locals.var_toxpf_dn8), (200000000.0 * locals.var_toxpf_dn9), (200000000.0 * locals.var_toxpf_dn10), (200000000.0 * locals.var_toxpf_dn11), (200000000.0 * locals.var_toxpf_dn12),)
    } else {
        (locals.var_tmp2, locals.var_tmp2_dn3, locals.var_tmp2_dn4, locals.var_tmp2_dn5, locals.var_tmp2_dn6, locals.var_tmp2_dn7, locals.var_tmp2_dn8, locals.var_tmp2_dn9, locals.var_tmp2_dn10, locals.var_tmp2_dn11, locals.var_tmp2_dn12,)
    }
};
            locals.var_tmp2 = assign11380_body1_e8511;
            locals.var_tmp2_dn3 = assign11380_body1_e8511_d_n3;
            locals.var_tmp2_dn4 = assign11380_body1_e8511_d_n4;
            locals.var_tmp2_dn5 = assign11380_body1_e8511_d_n5;
            locals.var_tmp2_dn6 = assign11380_body1_e8511_d_n6;
            locals.var_tmp2_dn7 = assign11380_body1_e8511_d_n7;
            locals.var_tmp2_dn8 = assign11380_body1_e8511_d_n8;
            locals.var_tmp2_dn9 = assign11380_body1_e8511_d_n9;
            locals.var_tmp2_dn10 = assign11380_body1_e8511_d_n10;
            locals.var_tmp2_dn11 = assign11380_body1_e8511_d_n11;
            locals.var_tmp2_dn12 = assign11380_body1_e8511_d_n12;
            let (assign11380_body2_e8520, assign11380_body2_e8520_d_n3, assign11380_body2_e8520_d_n4, assign11380_body2_e8520_d_n5, assign11380_body2_e8520_d_n6, assign11380_body2_e8520_d_n7, assign11380_body2_e8520_d_n8, assign11380_body2_e8520_d_n9, assign11380_body2_e8520_d_n10, assign11380_body2_e8520_d_n11, assign11380_body2_e8520_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11380_body2_e8516: f64 = (locals.var_vgsteff + locals.var_vtfbphi2eot);
        let assign11380_body2_e8518: f64 = (assign11380_body2_e8516 / locals.var_tmp2);
        (assign11380_body2_e8518, ((((locals.var_vgsteff_dn3 + locals.var_vtfbphi2eot_dn3) * locals.var_tmp2) - (assign11380_body2_e8516 * locals.var_tmp2_dn3)) / (locals.var_tmp2 * locals.var_tmp2)), ((((locals.var_vgsteff_dn4 + locals.var_vtfbphi2eot_dn4) * locals.var_tmp2) - (assign11380_body2_e8516 * locals.var_tmp2_dn4)) / (locals.var_tmp2 * locals.var_tmp2)), ((((locals.var_vgsteff_dn5 + locals.var_vtfbphi2eot_dn5) * locals.var_tmp2) - (assign11380_body2_e8516 * locals.var_tmp2_dn5)) / (locals.var_tmp2 * locals.var_tmp2)), ((((locals.var_vgsteff_dn6 + locals.var_vtfbphi2eot_dn6) * locals.var_tmp2) - (assign11380_body2_e8516 * locals.var_tmp2_dn6)) / (locals.var_tmp2 * locals.var_tmp2)), ((((locals.var_vgsteff_dn7 + locals.var_vtfbphi2eot_dn7) * locals.var_tmp2) - (assign11380_body2_e8516 * locals.var_tmp2_dn7)) / (locals.var_tmp2 * locals.var_tmp2)), ((((locals.var_vgsteff_dn8 + locals.var_vtfbphi2eot_dn8) * locals.var_tmp2) - (assign11380_body2_e8516 * locals.var_tmp2_dn8)) / (locals.var_tmp2 * locals.var_tmp2)), ((((locals.var_vgsteff_dn9 + locals.var_vtfbphi2eot_dn9) * locals.var_tmp2) - (assign11380_body2_e8516 * locals.var_tmp2_dn9)) / (locals.var_tmp2 * locals.var_tmp2)), ((((locals.var_vgsteff_dn10 + locals.var_vtfbphi2eot_dn10) * locals.var_tmp2) - (assign11380_body2_e8516 * locals.var_tmp2_dn10)) / (locals.var_tmp2 * locals.var_tmp2)), ((((locals.var_vgsteff_dn11 + locals.var_vtfbphi2eot_dn11) * locals.var_tmp2) - (assign11380_body2_e8516 * locals.var_tmp2_dn11)) / (locals.var_tmp2 * locals.var_tmp2)), ((((locals.var_vgsteff_dn12 + locals.var_vtfbphi2eot_dn12) * locals.var_tmp2) - (assign11380_body2_e8516 * locals.var_tmp2_dn12)) / (locals.var_tmp2 * locals.var_tmp2)),)
    } else {
        (locals.var_t0__blk949, locals.var_t0__blk949_dn3, locals.var_t0__blk949_dn4, locals.var_t0__blk949_dn5, locals.var_t0__blk949_dn6, locals.var_t0__blk949_dn7, locals.var_t0__blk949_dn8, locals.var_t0__blk949_dn9, locals.var_t0__blk949_dn10, locals.var_t0__blk949_dn11, locals.var_t0__blk949_dn12,)
    }
};
            locals.var_t0__blk949 = assign11380_body2_e8520;
            locals.var_t0__blk949_dn3 = assign11380_body2_e8520_d_n3;
            locals.var_t0__blk949_dn4 = assign11380_body2_e8520_d_n4;
            locals.var_t0__blk949_dn5 = assign11380_body2_e8520_d_n5;
            locals.var_t0__blk949_dn6 = assign11380_body2_e8520_d_n6;
            locals.var_t0__blk949_dn7 = assign11380_body2_e8520_d_n7;
            locals.var_t0__blk949_dn8 = assign11380_body2_e8520_d_n8;
            locals.var_t0__blk949_dn9 = assign11380_body2_e8520_d_n9;
            locals.var_t0__blk949_dn10 = assign11380_body2_e8520_d_n10;
            locals.var_t0__blk949_dn11 = assign11380_body2_e8520_d_n11;
            locals.var_t0__blk949_dn12 = assign11380_body2_e8520_d_n12;
            let (assign11380_body3_e8539, assign11380_body3_e8539_d_n3, assign11380_body3_e8539_d_n4, assign11380_body3_e8539_d_n5, assign11380_body3_e8539_d_n6, assign11380_body3_e8539_d_n7, assign11380_body3_e8539_d_n8, assign11380_body3_e8539_d_n9, assign11380_body3_e8539_d_n10, assign11380_body3_e8539_d_n11, assign11380_body3_e8539_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11380_body3_e8526: f64 = (locals.var_b4soibdos * 0.7);
        let (assign11380_body3_e8534, assign11380_body3_e8534_d_n3, assign11380_body3_e8534_d_n4, assign11380_body3_e8534_d_n5, assign11380_body3_e8534_d_n6, assign11380_body3_e8534_d_n7, assign11380_body3_e8534_d_n8, assign11380_body3_e8534_d_n9, assign11380_body3_e8534_d_n10, assign11380_body3_e8534_d_n11, assign11380_body3_e8534_d_n12,) = {
            if (locals.var_t0__blk949 > 1e-38) {
                let assign11380_body3_e8531: f64 = (locals.var_t0__blk949).ln();
                (assign11380_body3_e8531, (locals.var_t0__blk949_dn3 / locals.var_t0__blk949), (locals.var_t0__blk949_dn4 / locals.var_t0__blk949), (locals.var_t0__blk949_dn5 / locals.var_t0__blk949), (locals.var_t0__blk949_dn6 / locals.var_t0__blk949), (locals.var_t0__blk949_dn7 / locals.var_t0__blk949), (locals.var_t0__blk949_dn8 / locals.var_t0__blk949), (locals.var_t0__blk949_dn9 / locals.var_t0__blk949), (locals.var_t0__blk949_dn10 / locals.var_t0__blk949), (locals.var_t0__blk949_dn11 / locals.var_t0__blk949), (locals.var_t0__blk949_dn12 / locals.var_t0__blk949),)
            } else {
                let assign11380_body3_e8533: f64 = (-87.49823353377374);
                (assign11380_body3_e8533, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign11380_body3_e8535: f64 = (assign11380_body3_e8526 * assign11380_body3_e8534);
        let assign11380_body3_e8536: f64 = (assign11380_body3_e8535).exp();
        let assign11380_body3_e8537: f64 = (1.0 + assign11380_body3_e8536);
        (assign11380_body3_e8537, (assign11380_body3_e8536 * (assign11380_body3_e8526 * assign11380_body3_e8534_d_n3)), (assign11380_body3_e8536 * (assign11380_body3_e8526 * assign11380_body3_e8534_d_n4)), (assign11380_body3_e8536 * (assign11380_body3_e8526 * assign11380_body3_e8534_d_n5)), (assign11380_body3_e8536 * (assign11380_body3_e8526 * assign11380_body3_e8534_d_n6)), (assign11380_body3_e8536 * (assign11380_body3_e8526 * assign11380_body3_e8534_d_n7)), (assign11380_body3_e8536 * (assign11380_body3_e8526 * assign11380_body3_e8534_d_n8)), (assign11380_body3_e8536 * (assign11380_body3_e8526 * assign11380_body3_e8534_d_n9)), (assign11380_body3_e8536 * (assign11380_body3_e8526 * assign11380_body3_e8534_d_n10)), (assign11380_body3_e8536 * (assign11380_body3_e8526 * assign11380_body3_e8534_d_n11)), (assign11380_body3_e8536 * (assign11380_body3_e8526 * assign11380_body3_e8534_d_n12)),)
    } else {
        (locals.var_t1__blk950, locals.var_t1__blk950_dn3, locals.var_t1__blk950_dn4, locals.var_t1__blk950_dn5, locals.var_t1__blk950_dn6, locals.var_t1__blk950_dn7, locals.var_t1__blk950_dn8, locals.var_t1__blk950_dn9, locals.var_t1__blk950_dn10, locals.var_t1__blk950_dn11, locals.var_t1__blk950_dn12,)
    }
};
            locals.var_t1__blk950 = assign11380_body3_e8539;
            locals.var_t1__blk950_dn3 = assign11380_body3_e8539_d_n3;
            locals.var_t1__blk950_dn4 = assign11380_body3_e8539_d_n4;
            locals.var_t1__blk950_dn5 = assign11380_body3_e8539_d_n5;
            locals.var_t1__blk950_dn6 = assign11380_body3_e8539_d_n6;
            locals.var_t1__blk950_dn7 = assign11380_body3_e8539_d_n7;
            locals.var_t1__blk950_dn8 = assign11380_body3_e8539_d_n8;
            locals.var_t1__blk950_dn9 = assign11380_body3_e8539_d_n9;
            locals.var_t1__blk950_dn10 = assign11380_body3_e8539_d_n10;
            locals.var_t1__blk950_dn11 = assign11380_body3_e8539_d_n11;
            locals.var_t1__blk950_dn12 = assign11380_body3_e8539_d_n12;
            let (assign11380_body4_e8548, assign11380_body4_e8548_d_n3, assign11380_body4_e8548_d_n4, assign11380_body4_e8548_d_n5, assign11380_body4_e8548_d_n6, assign11380_body4_e8548_d_n7, assign11380_body4_e8548_d_n8, assign11380_body4_e8548_d_n9, assign11380_body4_e8548_d_n10, assign11380_body4_e8548_d_n11, assign11380_body4_e8548_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11380_body4_e8544: f64 = (locals.var_b4soiados * 1.9e-9);
        let assign11380_body4_e8546: f64 = (assign11380_body4_e8544 / locals.var_t1__blk950);
        (assign11380_body4_e8546, (-((assign11380_body4_e8544 * locals.var_t1__blk950_dn3) / (locals.var_t1__blk950 * locals.var_t1__blk950))), (-((assign11380_body4_e8544 * locals.var_t1__blk950_dn4) / (locals.var_t1__blk950 * locals.var_t1__blk950))), (-((assign11380_body4_e8544 * locals.var_t1__blk950_dn5) / (locals.var_t1__blk950 * locals.var_t1__blk950))), (-((assign11380_body4_e8544 * locals.var_t1__blk950_dn6) / (locals.var_t1__blk950 * locals.var_t1__blk950))), (-((assign11380_body4_e8544 * locals.var_t1__blk950_dn7) / (locals.var_t1__blk950 * locals.var_t1__blk950))), (-((assign11380_body4_e8544 * locals.var_t1__blk950_dn8) / (locals.var_t1__blk950 * locals.var_t1__blk950))), (-((assign11380_body4_e8544 * locals.var_t1__blk950_dn9) / (locals.var_t1__blk950 * locals.var_t1__blk950))), (-((assign11380_body4_e8544 * locals.var_t1__blk950_dn10) / (locals.var_t1__blk950 * locals.var_t1__blk950))), (-((assign11380_body4_e8544 * locals.var_t1__blk950_dn11) / (locals.var_t1__blk950 * locals.var_t1__blk950))), (-((assign11380_body4_e8544 * locals.var_t1__blk950_dn12) / (locals.var_t1__blk950 * locals.var_t1__blk950))),)
    } else {
        (locals.var_tcen, locals.var_tcen_dn3, locals.var_tcen_dn4, locals.var_tcen_dn5, locals.var_tcen_dn6, locals.var_tcen_dn7, locals.var_tcen_dn8, locals.var_tcen_dn9, locals.var_tcen_dn10, locals.var_tcen_dn11, locals.var_tcen_dn12,)
    }
};
            locals.var_tcen = assign11380_body4_e8548;
            locals.var_tcen_dn3 = assign11380_body4_e8548_d_n3;
            locals.var_tcen_dn4 = assign11380_body4_e8548_d_n4;
            locals.var_tcen_dn5 = assign11380_body4_e8548_d_n5;
            locals.var_tcen_dn6 = assign11380_body4_e8548_d_n6;
            locals.var_tcen_dn7 = assign11380_body4_e8548_d_n7;
            locals.var_tcen_dn8 = assign11380_body4_e8548_d_n8;
            locals.var_tcen_dn9 = assign11380_body4_e8548_d_n9;
            locals.var_tcen_dn10 = assign11380_body4_e8548_d_n10;
            locals.var_tcen_dn11 = assign11380_body4_e8548_d_n11;
            locals.var_tcen_dn12 = assign11380_body4_e8548_d_n12;
            let (assign11380_body5_e8559, assign11380_body5_e8559_d_n3, assign11380_body5_e8559_d_n4, assign11380_body5_e8559_d_n5, assign11380_body5_e8559_d_n6, assign11380_body5_e8559_d_n7, assign11380_body5_e8559_d_n8, assign11380_body5_e8559_d_n9, assign11380_body5_e8559_d_n10, assign11380_body5_e8559_d_n11, assign11380_body5_e8559_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11380_body5_e8554: f64 = (locals.var_epsrox / locals.var_b4soiepsrsub);
        let assign11380_body5_e8556: f64 = (assign11380_body5_e8554 * locals.var_tcen);
        let assign11380_body5_e8557: f64 = (locals.var_toxe - assign11380_body5_e8556);
        (assign11380_body5_e8557, (-(assign11380_body5_e8554 * locals.var_tcen_dn3)), (-(assign11380_body5_e8554 * locals.var_tcen_dn4)), (-(assign11380_body5_e8554 * locals.var_tcen_dn5)), (-(assign11380_body5_e8554 * locals.var_tcen_dn6)), (-(assign11380_body5_e8554 * locals.var_tcen_dn7)), (-(assign11380_body5_e8554 * locals.var_tcen_dn8)), (-(assign11380_body5_e8554 * locals.var_tcen_dn9)), (-(assign11380_body5_e8554 * locals.var_tcen_dn10)), (-(assign11380_body5_e8554 * locals.var_tcen_dn11)), (-(assign11380_body5_e8554 * locals.var_tcen_dn12)),)
    } else {
        (locals.var_toxpf, locals.var_toxpf_dn3, locals.var_toxpf_dn4, locals.var_toxpf_dn5, locals.var_toxpf_dn6, locals.var_toxpf_dn7, locals.var_toxpf_dn8, locals.var_toxpf_dn9, locals.var_toxpf_dn10, locals.var_toxpf_dn11, locals.var_toxpf_dn12,)
    }
};
            locals.var_toxpf = assign11380_body5_e8559;
            locals.var_toxpf_dn3 = assign11380_body5_e8559_d_n3;
            locals.var_toxpf_dn4 = assign11380_body5_e8559_d_n4;
            locals.var_toxpf_dn5 = assign11380_body5_e8559_d_n5;
            locals.var_toxpf_dn6 = assign11380_body5_e8559_d_n6;
            locals.var_toxpf_dn7 = assign11380_body5_e8559_d_n7;
            locals.var_toxpf_dn8 = assign11380_body5_e8559_d_n8;
            locals.var_toxpf_dn9 = assign11380_body5_e8559_d_n9;
            locals.var_toxpf_dn10 = assign11380_body5_e8559_d_n10;
            locals.var_toxpf_dn11 = assign11380_body5_e8559_d_n11;
            locals.var_toxpf_dn12 = assign11380_body5_e8559_d_n12;
            let (assign11380_body6_e8566,) = {
    if (locals.var_guard940 == 0.0) {
        let assign11380_body6_e8564: f64 = (locals.var_niter + 1.0);
        (assign11380_body6_e8564,)
    } else {
        (locals.var_niter,)
    }
};
            locals.var_niter = assign11380_body6_e8566;
        }

        let (assign11390_e8571, assign11390_e8571_d_n3, assign11390_e8571_d_n4, assign11390_e8571_d_n5, assign11390_e8571_d_n6, assign11390_e8571_d_n7, assign11390_e8571_d_n8, assign11390_e8571_d_n9, assign11390_e8571_d_n10, assign11390_e8571_d_n11, assign11390_e8571_d_n12,) = {
    if (locals.var_guard940 == 0.0) {
        (locals.var_toxpf, locals.var_toxpf_dn3, locals.var_toxpf_dn4, locals.var_toxpf_dn5, locals.var_toxpf_dn6, locals.var_toxpf_dn7, locals.var_toxpf_dn8, locals.var_toxpf_dn9, locals.var_toxpf_dn10, locals.var_toxpf_dn11, locals.var_toxpf_dn12,)
    } else {
        (locals.var_b4soitoxp, locals.var_b4soitoxp_dn3, locals.var_b4soitoxp_dn4, locals.var_b4soitoxp_dn5, locals.var_b4soitoxp_dn6, locals.var_b4soitoxp_dn7, locals.var_b4soitoxp_dn8, locals.var_b4soitoxp_dn9, locals.var_b4soitoxp_dn10, locals.var_b4soitoxp_dn11, locals.var_b4soitoxp_dn12,)
    }
};
        locals.var_b4soitoxp = assign11390_e8571;
        locals.var_b4soitoxp_dn3 = assign11390_e8571_d_n3;
        locals.var_b4soitoxp_dn4 = assign11390_e8571_d_n4;
        locals.var_b4soitoxp_dn5 = assign11390_e8571_d_n5;
        locals.var_b4soitoxp_dn6 = assign11390_e8571_d_n6;
        locals.var_b4soitoxp_dn7 = assign11390_e8571_d_n7;
        locals.var_b4soitoxp_dn8 = assign11390_e8571_d_n8;
        locals.var_b4soitoxp_dn9 = assign11390_e8571_d_n9;
        locals.var_b4soitoxp_dn10 = assign11390_e8571_d_n10;
        locals.var_b4soitoxp_dn11 = assign11390_e8571_d_n11;
        locals.var_b4soitoxp_dn12 = assign11390_e8571_d_n12;

        locals.var_tmp = locals.var_pparam_b4soisqrtxdep0;
        locals.var_tmp_dn3 = locals.var_pparam_b4soisqrtxdep0_dn3;
        locals.var_tmp_dn4 = locals.var_pparam_b4soisqrtxdep0_dn4;
        locals.var_tmp_dn5 = locals.var_pparam_b4soisqrtxdep0_dn5;
        locals.var_tmp_dn6 = locals.var_pparam_b4soisqrtxdep0_dn6;
        locals.var_tmp_dn7 = locals.var_pparam_b4soisqrtxdep0_dn7;
        locals.var_tmp_dn8 = locals.var_pparam_b4soisqrtxdep0_dn8;
        locals.var_tmp_dn9 = locals.var_pparam_b4soisqrtxdep0_dn9;
        locals.var_tmp_dn10 = locals.var_pparam_b4soisqrtxdep0_dn10;
        locals.var_tmp_dn11 = locals.var_pparam_b4soisqrtxdep0_dn11;
        locals.var_tmp_dn12 = locals.var_pparam_b4soisqrtxdep0_dn12;

        let assign11410_e8575: f64 = (locals.var_pparam_b4soivbi - locals.var_pparam_b4soiphi);
        locals.var_tmp1 = assign11410_e8575;
        locals.var_tmp1_dn3 = (locals.var_pparam_b4soivbi_dn3 - locals.var_pparam_b4soiphi_dn3);
        locals.var_tmp1_dn4 = (locals.var_pparam_b4soivbi_dn4 - locals.var_pparam_b4soiphi_dn4);
        locals.var_tmp1_dn5 = (locals.var_pparam_b4soivbi_dn5 - locals.var_pparam_b4soiphi_dn5);
        locals.var_tmp1_dn6 = (locals.var_pparam_b4soivbi_dn6 - locals.var_pparam_b4soiphi_dn6);
        locals.var_tmp1_dn7 = (locals.var_pparam_b4soivbi_dn7 - locals.var_pparam_b4soiphi_dn7);
        locals.var_tmp1_dn8 = (locals.var_pparam_b4soivbi_dn8 - locals.var_pparam_b4soiphi_dn8);
        locals.var_tmp1_dn9 = (locals.var_pparam_b4soivbi_dn9 - locals.var_pparam_b4soiphi_dn9);
        locals.var_tmp1_dn10 = (locals.var_pparam_b4soivbi_dn10 - locals.var_pparam_b4soiphi_dn10);
        locals.var_tmp1_dn11 = (locals.var_pparam_b4soivbi_dn11 - locals.var_pparam_b4soiphi_dn11);
        locals.var_tmp1_dn12 = (locals.var_pparam_b4soivbi_dn12 - locals.var_pparam_b4soiphi_dn12);

        let assign11420_e8578: f64 = (locals.var_b4soifactor1 * locals.var_tmp);
        locals.var_tmp2 = assign11420_e8578;
        locals.var_tmp2_dn3 = (locals.var_b4soifactor1 * locals.var_tmp_dn3);
        locals.var_tmp2_dn4 = (locals.var_b4soifactor1 * locals.var_tmp_dn4);
        locals.var_tmp2_dn5 = (locals.var_b4soifactor1 * locals.var_tmp_dn5);
        locals.var_tmp2_dn6 = (locals.var_b4soifactor1 * locals.var_tmp_dn6);
        locals.var_tmp2_dn7 = (locals.var_b4soifactor1 * locals.var_tmp_dn7);
        locals.var_tmp2_dn8 = (locals.var_b4soifactor1 * locals.var_tmp_dn8);
        locals.var_tmp2_dn9 = (locals.var_b4soifactor1 * locals.var_tmp_dn9);
        locals.var_tmp2_dn10 = (locals.var_b4soifactor1 * locals.var_tmp_dn10);
        locals.var_tmp2_dn11 = (locals.var_b4soifactor1 * locals.var_tmp_dn11);
        locals.var_tmp2_dn12 = (locals.var_b4soifactor1 * locals.var_tmp_dn12);

        let assign11430_e8580: f64 = (-0.5);
        let assign11430_e8582: f64 = (assign11430_e8580 * locals.var_pparam_b4soidvt1w);
        let assign11430_e8584: f64 = (assign11430_e8582 * locals.var_pparam_b4soiweff);
        let assign11430_e8586: f64 = (assign11430_e8584 * locals.var_pparam_b4soileff);
        let assign11430_e8588: f64 = (assign11430_e8586 / locals.var_tmp2);
        locals.var_t0 = assign11430_e8588;
        locals.var_t0_dn3 = ((((((((assign11430_e8580 * locals.var_pparam_b4soidvt1w_dn3) * locals.var_pparam_b4soiweff) + (assign11430_e8582 * locals.var_pparam_b4soiweff_dn3)) * locals.var_pparam_b4soileff) + (assign11430_e8584 * locals.var_pparam_b4soileff_dn3)) * locals.var_tmp2) - (assign11430_e8586 * locals.var_tmp2_dn3)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn4 = ((((((((assign11430_e8580 * locals.var_pparam_b4soidvt1w_dn4) * locals.var_pparam_b4soiweff) + (assign11430_e8582 * locals.var_pparam_b4soiweff_dn4)) * locals.var_pparam_b4soileff) + (assign11430_e8584 * locals.var_pparam_b4soileff_dn4)) * locals.var_tmp2) - (assign11430_e8586 * locals.var_tmp2_dn4)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn5 = ((((((((assign11430_e8580 * locals.var_pparam_b4soidvt1w_dn5) * locals.var_pparam_b4soiweff) + (assign11430_e8582 * locals.var_pparam_b4soiweff_dn5)) * locals.var_pparam_b4soileff) + (assign11430_e8584 * locals.var_pparam_b4soileff_dn5)) * locals.var_tmp2) - (assign11430_e8586 * locals.var_tmp2_dn5)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn6 = ((((((((assign11430_e8580 * locals.var_pparam_b4soidvt1w_dn6) * locals.var_pparam_b4soiweff) + (assign11430_e8582 * locals.var_pparam_b4soiweff_dn6)) * locals.var_pparam_b4soileff) + (assign11430_e8584 * locals.var_pparam_b4soileff_dn6)) * locals.var_tmp2) - (assign11430_e8586 * locals.var_tmp2_dn6)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn7 = ((((((((assign11430_e8580 * locals.var_pparam_b4soidvt1w_dn7) * locals.var_pparam_b4soiweff) + (assign11430_e8582 * locals.var_pparam_b4soiweff_dn7)) * locals.var_pparam_b4soileff) + (assign11430_e8584 * locals.var_pparam_b4soileff_dn7)) * locals.var_tmp2) - (assign11430_e8586 * locals.var_tmp2_dn7)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn8 = ((((((((assign11430_e8580 * locals.var_pparam_b4soidvt1w_dn8) * locals.var_pparam_b4soiweff) + (assign11430_e8582 * locals.var_pparam_b4soiweff_dn8)) * locals.var_pparam_b4soileff) + (assign11430_e8584 * locals.var_pparam_b4soileff_dn8)) * locals.var_tmp2) - (assign11430_e8586 * locals.var_tmp2_dn8)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn9 = ((((((((assign11430_e8580 * locals.var_pparam_b4soidvt1w_dn9) * locals.var_pparam_b4soiweff) + (assign11430_e8582 * locals.var_pparam_b4soiweff_dn9)) * locals.var_pparam_b4soileff) + (assign11430_e8584 * locals.var_pparam_b4soileff_dn9)) * locals.var_tmp2) - (assign11430_e8586 * locals.var_tmp2_dn9)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn10 = ((((((((assign11430_e8580 * locals.var_pparam_b4soidvt1w_dn10) * locals.var_pparam_b4soiweff) + (assign11430_e8582 * locals.var_pparam_b4soiweff_dn10)) * locals.var_pparam_b4soileff) + (assign11430_e8584 * locals.var_pparam_b4soileff_dn10)) * locals.var_tmp2) - (assign11430_e8586 * locals.var_tmp2_dn10)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn11 = ((((((((assign11430_e8580 * locals.var_pparam_b4soidvt1w_dn11) * locals.var_pparam_b4soiweff) + (assign11430_e8582 * locals.var_pparam_b4soiweff_dn11)) * locals.var_pparam_b4soileff) + (assign11430_e8584 * locals.var_pparam_b4soileff_dn11)) * locals.var_tmp2) - (assign11430_e8586 * locals.var_tmp2_dn11)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn12 = ((((((((assign11430_e8580 * locals.var_pparam_b4soidvt1w_dn12) * locals.var_pparam_b4soiweff) + (assign11430_e8582 * locals.var_pparam_b4soiweff_dn12)) * locals.var_pparam_b4soileff) + (assign11430_e8584 * locals.var_pparam_b4soileff_dn12)) * locals.var_tmp2) - (assign11430_e8586 * locals.var_tmp2_dn12)) / (locals.var_tmp2 * locals.var_tmp2));

        let assign11440_e8591: f64 = (-100.0);
        let assign11440_e8592: f64 = if locals.var_t0 > assign11440_e8591 { 1.0 } else { 0.0 };
        locals.var_guard951 = assign11440_e8592;

        let (assign11450_e8597, assign11450_e8597_d_n3, assign11450_e8597_d_n4, assign11450_e8597_d_n5, assign11450_e8597_d_n6, assign11450_e8597_d_n7, assign11450_e8597_d_n8, assign11450_e8597_d_n9, assign11450_e8597_d_n10, assign11450_e8597_d_n11, assign11450_e8597_d_n12,) = {
    if (locals.var_guard951 != 0.0) {
        let assign11450_e8595: f64 = (locals.var_t0).exp();
        (assign11450_e8595, (assign11450_e8595 * locals.var_t0_dn3), (assign11450_e8595 * locals.var_t0_dn4), (assign11450_e8595 * locals.var_t0_dn5), (assign11450_e8595 * locals.var_t0_dn6), (assign11450_e8595 * locals.var_t0_dn7), (assign11450_e8595 * locals.var_t0_dn8), (assign11450_e8595 * locals.var_t0_dn9), (assign11450_e8595 * locals.var_t0_dn10), (assign11450_e8595 * locals.var_t0_dn11), (assign11450_e8595 * locals.var_t0_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign11450_e8597;
        locals.var_t1_dn3 = assign11450_e8597_d_n3;
        locals.var_t1_dn4 = assign11450_e8597_d_n4;
        locals.var_t1_dn5 = assign11450_e8597_d_n5;
        locals.var_t1_dn6 = assign11450_e8597_d_n6;
        locals.var_t1_dn7 = assign11450_e8597_d_n7;
        locals.var_t1_dn8 = assign11450_e8597_d_n8;
        locals.var_t1_dn9 = assign11450_e8597_d_n9;
        locals.var_t1_dn10 = assign11450_e8597_d_n10;
        locals.var_t1_dn11 = assign11450_e8597_d_n11;
        locals.var_t1_dn12 = assign11450_e8597_d_n12;

        let (assign11460_e8607, assign11460_e8607_d_n3, assign11460_e8607_d_n4, assign11460_e8607_d_n5, assign11460_e8607_d_n6, assign11460_e8607_d_n7, assign11460_e8607_d_n8, assign11460_e8607_d_n9, assign11460_e8607_d_n10, assign11460_e8607_d_n11, assign11460_e8607_d_n12,) = {
    if (locals.var_guard951 != 0.0) {
        let assign11460_e8603: f64 = (2.0 * locals.var_t1);
        let assign11460_e8604: f64 = (1.0 + assign11460_e8603);
        let assign11460_e8605: f64 = (locals.var_t1 * assign11460_e8604);
        (assign11460_e8605, ((locals.var_t1_dn3 * assign11460_e8604) + (locals.var_t1 * (2.0 * locals.var_t1_dn3))), ((locals.var_t1_dn4 * assign11460_e8604) + (locals.var_t1 * (2.0 * locals.var_t1_dn4))), ((locals.var_t1_dn5 * assign11460_e8604) + (locals.var_t1 * (2.0 * locals.var_t1_dn5))), ((locals.var_t1_dn6 * assign11460_e8604) + (locals.var_t1 * (2.0 * locals.var_t1_dn6))), ((locals.var_t1_dn7 * assign11460_e8604) + (locals.var_t1 * (2.0 * locals.var_t1_dn7))), ((locals.var_t1_dn8 * assign11460_e8604) + (locals.var_t1 * (2.0 * locals.var_t1_dn8))), ((locals.var_t1_dn9 * assign11460_e8604) + (locals.var_t1 * (2.0 * locals.var_t1_dn9))), ((locals.var_t1_dn10 * assign11460_e8604) + (locals.var_t1 * (2.0 * locals.var_t1_dn10))), ((locals.var_t1_dn11 * assign11460_e8604) + (locals.var_t1 * (2.0 * locals.var_t1_dn11))), ((locals.var_t1_dn12 * assign11460_e8604) + (locals.var_t1 * (2.0 * locals.var_t1_dn12))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign11460_e8607;
        locals.var_t2_dn3 = assign11460_e8607_d_n3;
        locals.var_t2_dn4 = assign11460_e8607_d_n4;
        locals.var_t2_dn5 = assign11460_e8607_d_n5;
        locals.var_t2_dn6 = assign11460_e8607_d_n6;
        locals.var_t2_dn7 = assign11460_e8607_d_n7;
        locals.var_t2_dn8 = assign11460_e8607_d_n8;
        locals.var_t2_dn9 = assign11460_e8607_d_n9;
        locals.var_t2_dn10 = assign11460_e8607_d_n10;
        locals.var_t2_dn11 = assign11460_e8607_d_n11;
        locals.var_t2_dn12 = assign11460_e8607_d_n12;

        let (assign11470_e8612, assign11470_e8612_d_n3, assign11470_e8612_d_n4, assign11470_e8612_d_n5, assign11470_e8612_d_n6, assign11470_e8612_d_n7, assign11470_e8612_d_n8, assign11470_e8612_d_n9, assign11470_e8612_d_n10, assign11470_e8612_d_n11, assign11470_e8612_d_n12,) = {
    if (locals.var_guard951 == 0.0) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign11470_e8612;
        locals.var_t1_dn3 = assign11470_e8612_d_n3;
        locals.var_t1_dn4 = assign11470_e8612_d_n4;
        locals.var_t1_dn5 = assign11470_e8612_d_n5;
        locals.var_t1_dn6 = assign11470_e8612_d_n6;
        locals.var_t1_dn7 = assign11470_e8612_d_n7;
        locals.var_t1_dn8 = assign11470_e8612_d_n8;
        locals.var_t1_dn9 = assign11470_e8612_d_n9;
        locals.var_t1_dn10 = assign11470_e8612_d_n10;
        locals.var_t1_dn11 = assign11470_e8612_d_n11;
        locals.var_t1_dn12 = assign11470_e8612_d_n12;

        let (assign11480_e8623, assign11480_e8623_d_n3, assign11480_e8623_d_n4, assign11480_e8623_d_n5, assign11480_e8623_d_n6, assign11480_e8623_d_n7, assign11480_e8623_d_n8, assign11480_e8623_d_n9, assign11480_e8623_d_n10, assign11480_e8623_d_n11, assign11480_e8623_d_n12,) = {
    if (locals.var_guard951 == 0.0) {
        let assign11480_e8619: f64 = (2.0 * locals.var_t1);
        let assign11480_e8620: f64 = (1.0 + assign11480_e8619);
        let assign11480_e8621: f64 = (locals.var_t1 * assign11480_e8620);
        (assign11480_e8621, ((locals.var_t1_dn3 * assign11480_e8620) + (locals.var_t1 * (2.0 * locals.var_t1_dn3))), ((locals.var_t1_dn4 * assign11480_e8620) + (locals.var_t1 * (2.0 * locals.var_t1_dn4))), ((locals.var_t1_dn5 * assign11480_e8620) + (locals.var_t1 * (2.0 * locals.var_t1_dn5))), ((locals.var_t1_dn6 * assign11480_e8620) + (locals.var_t1 * (2.0 * locals.var_t1_dn6))), ((locals.var_t1_dn7 * assign11480_e8620) + (locals.var_t1 * (2.0 * locals.var_t1_dn7))), ((locals.var_t1_dn8 * assign11480_e8620) + (locals.var_t1 * (2.0 * locals.var_t1_dn8))), ((locals.var_t1_dn9 * assign11480_e8620) + (locals.var_t1 * (2.0 * locals.var_t1_dn9))), ((locals.var_t1_dn10 * assign11480_e8620) + (locals.var_t1 * (2.0 * locals.var_t1_dn10))), ((locals.var_t1_dn11 * assign11480_e8620) + (locals.var_t1 * (2.0 * locals.var_t1_dn11))), ((locals.var_t1_dn12 * assign11480_e8620) + (locals.var_t1 * (2.0 * locals.var_t1_dn12))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign11480_e8623;
        locals.var_t2_dn3 = assign11480_e8623_d_n3;
        locals.var_t2_dn4 = assign11480_e8623_d_n4;
        locals.var_t2_dn5 = assign11480_e8623_d_n5;
        locals.var_t2_dn6 = assign11480_e8623_d_n6;
        locals.var_t2_dn7 = assign11480_e8623_d_n7;
        locals.var_t2_dn8 = assign11480_e8623_d_n8;
        locals.var_t2_dn9 = assign11480_e8623_d_n9;
        locals.var_t2_dn10 = assign11480_e8623_d_n10;
        locals.var_t2_dn11 = assign11480_e8623_d_n11;
        locals.var_t2_dn12 = assign11480_e8623_d_n12;

        let assign11490_e8626: f64 = (locals.var_pparam_b4soidvt0w * locals.var_t2);
        locals.var_t0 = assign11490_e8626;
        locals.var_t0_dn3 = ((locals.var_pparam_b4soidvt0w_dn3 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn3));
        locals.var_t0_dn4 = ((locals.var_pparam_b4soidvt0w_dn4 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn4));
        locals.var_t0_dn5 = ((locals.var_pparam_b4soidvt0w_dn5 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn5));
        locals.var_t0_dn6 = ((locals.var_pparam_b4soidvt0w_dn6 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn6));
        locals.var_t0_dn7 = ((locals.var_pparam_b4soidvt0w_dn7 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn7));
        locals.var_t0_dn8 = ((locals.var_pparam_b4soidvt0w_dn8 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn8));
        locals.var_t0_dn9 = ((locals.var_pparam_b4soidvt0w_dn9 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn9));
        locals.var_t0_dn10 = ((locals.var_pparam_b4soidvt0w_dn10 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn10));
        locals.var_t0_dn11 = ((locals.var_pparam_b4soidvt0w_dn11 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn11));
        locals.var_t0_dn12 = ((locals.var_pparam_b4soidvt0w_dn12 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn12));

        let assign11500_e8629: f64 = (locals.var_t0 * locals.var_tmp1);
        locals.var_t2 = assign11500_e8629;
        locals.var_t2_dn3 = ((locals.var_t0_dn3 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn3));
        locals.var_t2_dn4 = ((locals.var_t0_dn4 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn4));
        locals.var_t2_dn5 = ((locals.var_t0_dn5 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn5));
        locals.var_t2_dn6 = ((locals.var_t0_dn6 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn6));
        locals.var_t2_dn7 = ((locals.var_t0_dn7 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn7));
        locals.var_t2_dn8 = ((locals.var_t0_dn8 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn8));
        locals.var_t2_dn9 = ((locals.var_t0_dn9 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn9));
        locals.var_t2_dn10 = ((locals.var_t0_dn10 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn10));
        locals.var_t2_dn11 = ((locals.var_t0_dn11 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn11));
        locals.var_t2_dn12 = ((locals.var_t0_dn12 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn12));

        let assign11510_e8631: f64 = (-0.5);
        let assign11510_e8633: f64 = (assign11510_e8631 * locals.var_pparam_b4soidvt1);
        let assign11510_e8635: f64 = (assign11510_e8633 * locals.var_pparam_b4soileff);
        let assign11510_e8637: f64 = (assign11510_e8635 / locals.var_tmp2);
        locals.var_t0 = assign11510_e8637;
        locals.var_t0_dn3 = ((((((assign11510_e8631 * locals.var_pparam_b4soidvt1_dn3) * locals.var_pparam_b4soileff) + (assign11510_e8633 * locals.var_pparam_b4soileff_dn3)) * locals.var_tmp2) - (assign11510_e8635 * locals.var_tmp2_dn3)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn4 = ((((((assign11510_e8631 * locals.var_pparam_b4soidvt1_dn4) * locals.var_pparam_b4soileff) + (assign11510_e8633 * locals.var_pparam_b4soileff_dn4)) * locals.var_tmp2) - (assign11510_e8635 * locals.var_tmp2_dn4)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn5 = ((((((assign11510_e8631 * locals.var_pparam_b4soidvt1_dn5) * locals.var_pparam_b4soileff) + (assign11510_e8633 * locals.var_pparam_b4soileff_dn5)) * locals.var_tmp2) - (assign11510_e8635 * locals.var_tmp2_dn5)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn6 = ((((((assign11510_e8631 * locals.var_pparam_b4soidvt1_dn6) * locals.var_pparam_b4soileff) + (assign11510_e8633 * locals.var_pparam_b4soileff_dn6)) * locals.var_tmp2) - (assign11510_e8635 * locals.var_tmp2_dn6)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn7 = ((((((assign11510_e8631 * locals.var_pparam_b4soidvt1_dn7) * locals.var_pparam_b4soileff) + (assign11510_e8633 * locals.var_pparam_b4soileff_dn7)) * locals.var_tmp2) - (assign11510_e8635 * locals.var_tmp2_dn7)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn8 = ((((((assign11510_e8631 * locals.var_pparam_b4soidvt1_dn8) * locals.var_pparam_b4soileff) + (assign11510_e8633 * locals.var_pparam_b4soileff_dn8)) * locals.var_tmp2) - (assign11510_e8635 * locals.var_tmp2_dn8)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn9 = ((((((assign11510_e8631 * locals.var_pparam_b4soidvt1_dn9) * locals.var_pparam_b4soileff) + (assign11510_e8633 * locals.var_pparam_b4soileff_dn9)) * locals.var_tmp2) - (assign11510_e8635 * locals.var_tmp2_dn9)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn10 = ((((((assign11510_e8631 * locals.var_pparam_b4soidvt1_dn10) * locals.var_pparam_b4soileff) + (assign11510_e8633 * locals.var_pparam_b4soileff_dn10)) * locals.var_tmp2) - (assign11510_e8635 * locals.var_tmp2_dn10)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn11 = ((((((assign11510_e8631 * locals.var_pparam_b4soidvt1_dn11) * locals.var_pparam_b4soileff) + (assign11510_e8633 * locals.var_pparam_b4soileff_dn11)) * locals.var_tmp2) - (assign11510_e8635 * locals.var_tmp2_dn11)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn12 = ((((((assign11510_e8631 * locals.var_pparam_b4soidvt1_dn12) * locals.var_pparam_b4soileff) + (assign11510_e8633 * locals.var_pparam_b4soileff_dn12)) * locals.var_tmp2) - (assign11510_e8635 * locals.var_tmp2_dn12)) / (locals.var_tmp2 * locals.var_tmp2));

        let assign11520_e8640: f64 = (-100.0);
        let assign11520_e8641: f64 = if locals.var_t0 > assign11520_e8640 { 1.0 } else { 0.0 };
        locals.var_guard952 = assign11520_e8641;

        let (assign11530_e8646, assign11530_e8646_d_n3, assign11530_e8646_d_n4, assign11530_e8646_d_n5, assign11530_e8646_d_n6, assign11530_e8646_d_n7, assign11530_e8646_d_n8, assign11530_e8646_d_n9, assign11530_e8646_d_n10, assign11530_e8646_d_n11, assign11530_e8646_d_n12,) = {
    if (locals.var_guard952 != 0.0) {
        let assign11530_e8644: f64 = (locals.var_t0).exp();
        (assign11530_e8644, (assign11530_e8644 * locals.var_t0_dn3), (assign11530_e8644 * locals.var_t0_dn4), (assign11530_e8644 * locals.var_t0_dn5), (assign11530_e8644 * locals.var_t0_dn6), (assign11530_e8644 * locals.var_t0_dn7), (assign11530_e8644 * locals.var_t0_dn8), (assign11530_e8644 * locals.var_t0_dn9), (assign11530_e8644 * locals.var_t0_dn10), (assign11530_e8644 * locals.var_t0_dn11), (assign11530_e8644 * locals.var_t0_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign11530_e8646;
        locals.var_t1_dn3 = assign11530_e8646_d_n3;
        locals.var_t1_dn4 = assign11530_e8646_d_n4;
        locals.var_t1_dn5 = assign11530_e8646_d_n5;
        locals.var_t1_dn6 = assign11530_e8646_d_n6;
        locals.var_t1_dn7 = assign11530_e8646_d_n7;
        locals.var_t1_dn8 = assign11530_e8646_d_n8;
        locals.var_t1_dn9 = assign11530_e8646_d_n9;
        locals.var_t1_dn10 = assign11530_e8646_d_n10;
        locals.var_t1_dn11 = assign11530_e8646_d_n11;
        locals.var_t1_dn12 = assign11530_e8646_d_n12;

        let (assign11540_e8656, assign11540_e8656_d_n3, assign11540_e8656_d_n4, assign11540_e8656_d_n5, assign11540_e8656_d_n6, assign11540_e8656_d_n7, assign11540_e8656_d_n8, assign11540_e8656_d_n9, assign11540_e8656_d_n10, assign11540_e8656_d_n11, assign11540_e8656_d_n12,) = {
    if (locals.var_guard952 != 0.0) {
        let assign11540_e8652: f64 = (2.0 * locals.var_t1);
        let assign11540_e8653: f64 = (1.0 + assign11540_e8652);
        let assign11540_e8654: f64 = (locals.var_t1 * assign11540_e8653);
        (assign11540_e8654, ((locals.var_t1_dn3 * assign11540_e8653) + (locals.var_t1 * (2.0 * locals.var_t1_dn3))), ((locals.var_t1_dn4 * assign11540_e8653) + (locals.var_t1 * (2.0 * locals.var_t1_dn4))), ((locals.var_t1_dn5 * assign11540_e8653) + (locals.var_t1 * (2.0 * locals.var_t1_dn5))), ((locals.var_t1_dn6 * assign11540_e8653) + (locals.var_t1 * (2.0 * locals.var_t1_dn6))), ((locals.var_t1_dn7 * assign11540_e8653) + (locals.var_t1 * (2.0 * locals.var_t1_dn7))), ((locals.var_t1_dn8 * assign11540_e8653) + (locals.var_t1 * (2.0 * locals.var_t1_dn8))), ((locals.var_t1_dn9 * assign11540_e8653) + (locals.var_t1 * (2.0 * locals.var_t1_dn9))), ((locals.var_t1_dn10 * assign11540_e8653) + (locals.var_t1 * (2.0 * locals.var_t1_dn10))), ((locals.var_t1_dn11 * assign11540_e8653) + (locals.var_t1 * (2.0 * locals.var_t1_dn11))), ((locals.var_t1_dn12 * assign11540_e8653) + (locals.var_t1 * (2.0 * locals.var_t1_dn12))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign11540_e8656;
        locals.var_t3_dn3 = assign11540_e8656_d_n3;
        locals.var_t3_dn4 = assign11540_e8656_d_n4;
        locals.var_t3_dn5 = assign11540_e8656_d_n5;
        locals.var_t3_dn6 = assign11540_e8656_d_n6;
        locals.var_t3_dn7 = assign11540_e8656_d_n7;
        locals.var_t3_dn8 = assign11540_e8656_d_n8;
        locals.var_t3_dn9 = assign11540_e8656_d_n9;
        locals.var_t3_dn10 = assign11540_e8656_d_n10;
        locals.var_t3_dn11 = assign11540_e8656_d_n11;
        locals.var_t3_dn12 = assign11540_e8656_d_n12;

        let (assign11550_e8661, assign11550_e8661_d_n3, assign11550_e8661_d_n4, assign11550_e8661_d_n5, assign11550_e8661_d_n6, assign11550_e8661_d_n7, assign11550_e8661_d_n8, assign11550_e8661_d_n9, assign11550_e8661_d_n10, assign11550_e8661_d_n11, assign11550_e8661_d_n12,) = {
    if (locals.var_guard952 == 0.0) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign11550_e8661;
        locals.var_t1_dn3 = assign11550_e8661_d_n3;
        locals.var_t1_dn4 = assign11550_e8661_d_n4;
        locals.var_t1_dn5 = assign11550_e8661_d_n5;
        locals.var_t1_dn6 = assign11550_e8661_d_n6;
        locals.var_t1_dn7 = assign11550_e8661_d_n7;
        locals.var_t1_dn8 = assign11550_e8661_d_n8;
        locals.var_t1_dn9 = assign11550_e8661_d_n9;
        locals.var_t1_dn10 = assign11550_e8661_d_n10;
        locals.var_t1_dn11 = assign11550_e8661_d_n11;
        locals.var_t1_dn12 = assign11550_e8661_d_n12;

        let (assign11560_e8672, assign11560_e8672_d_n3, assign11560_e8672_d_n4, assign11560_e8672_d_n5, assign11560_e8672_d_n6, assign11560_e8672_d_n7, assign11560_e8672_d_n8, assign11560_e8672_d_n9, assign11560_e8672_d_n10, assign11560_e8672_d_n11, assign11560_e8672_d_n12,) = {
    if (locals.var_guard952 == 0.0) {
        let assign11560_e8668: f64 = (2.0 * locals.var_t1);
        let assign11560_e8669: f64 = (1.0 + assign11560_e8668);
        let assign11560_e8670: f64 = (locals.var_t1 * assign11560_e8669);
        (assign11560_e8670, ((locals.var_t1_dn3 * assign11560_e8669) + (locals.var_t1 * (2.0 * locals.var_t1_dn3))), ((locals.var_t1_dn4 * assign11560_e8669) + (locals.var_t1 * (2.0 * locals.var_t1_dn4))), ((locals.var_t1_dn5 * assign11560_e8669) + (locals.var_t1 * (2.0 * locals.var_t1_dn5))), ((locals.var_t1_dn6 * assign11560_e8669) + (locals.var_t1 * (2.0 * locals.var_t1_dn6))), ((locals.var_t1_dn7 * assign11560_e8669) + (locals.var_t1 * (2.0 * locals.var_t1_dn7))), ((locals.var_t1_dn8 * assign11560_e8669) + (locals.var_t1 * (2.0 * locals.var_t1_dn8))), ((locals.var_t1_dn9 * assign11560_e8669) + (locals.var_t1 * (2.0 * locals.var_t1_dn9))), ((locals.var_t1_dn10 * assign11560_e8669) + (locals.var_t1 * (2.0 * locals.var_t1_dn10))), ((locals.var_t1_dn11 * assign11560_e8669) + (locals.var_t1 * (2.0 * locals.var_t1_dn11))), ((locals.var_t1_dn12 * assign11560_e8669) + (locals.var_t1 * (2.0 * locals.var_t1_dn12))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign11560_e8672;
        locals.var_t3_dn3 = assign11560_e8672_d_n3;
        locals.var_t3_dn4 = assign11560_e8672_d_n4;
        locals.var_t3_dn5 = assign11560_e8672_d_n5;
        locals.var_t3_dn6 = assign11560_e8672_d_n6;
        locals.var_t3_dn7 = assign11560_e8672_d_n7;
        locals.var_t3_dn8 = assign11560_e8672_d_n8;
        locals.var_t3_dn9 = assign11560_e8672_d_n9;
        locals.var_t3_dn10 = assign11560_e8672_d_n10;
        locals.var_t3_dn11 = assign11560_e8672_d_n11;
        locals.var_t3_dn12 = assign11560_e8672_d_n12;

        let assign11570_e8675: f64 = (locals.var_pparam_b4soidvt0 * locals.var_t3);
        let assign11570_e8677: f64 = (assign11570_e8675 * locals.var_tmp1);
        locals.var_t3 = assign11570_e8677;
        locals.var_t3_dn3 = ((((locals.var_pparam_b4soidvt0_dn3 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn3)) * locals.var_tmp1) + (assign11570_e8675 * locals.var_tmp1_dn3));
        locals.var_t3_dn4 = ((((locals.var_pparam_b4soidvt0_dn4 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn4)) * locals.var_tmp1) + (assign11570_e8675 * locals.var_tmp1_dn4));
        locals.var_t3_dn5 = ((((locals.var_pparam_b4soidvt0_dn5 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn5)) * locals.var_tmp1) + (assign11570_e8675 * locals.var_tmp1_dn5));
        locals.var_t3_dn6 = ((((locals.var_pparam_b4soidvt0_dn6 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn6)) * locals.var_tmp1) + (assign11570_e8675 * locals.var_tmp1_dn6));
        locals.var_t3_dn7 = ((((locals.var_pparam_b4soidvt0_dn7 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn7)) * locals.var_tmp1) + (assign11570_e8675 * locals.var_tmp1_dn7));
        locals.var_t3_dn8 = ((((locals.var_pparam_b4soidvt0_dn8 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn8)) * locals.var_tmp1) + (assign11570_e8675 * locals.var_tmp1_dn8));
        locals.var_t3_dn9 = ((((locals.var_pparam_b4soidvt0_dn9 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn9)) * locals.var_tmp1) + (assign11570_e8675 * locals.var_tmp1_dn9));
        locals.var_t3_dn10 = ((((locals.var_pparam_b4soidvt0_dn10 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn10)) * locals.var_tmp1) + (assign11570_e8675 * locals.var_tmp1_dn10));
        locals.var_t3_dn11 = ((((locals.var_pparam_b4soidvt0_dn11 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn11)) * locals.var_tmp1) + (assign11570_e8675 * locals.var_tmp1_dn11));
        locals.var_t3_dn12 = ((((locals.var_pparam_b4soidvt0_dn12 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn12)) * locals.var_tmp1) + (assign11570_e8675 * locals.var_tmp1_dn12));

        let assign11580_e8680: f64 = (locals.var_b4soitoxp * locals.var_pparam_b4soiphi);
        let assign11580_e8683: f64 = (locals.var_pparam_b4soiweff + locals.var_pparam_b4soiw0);
        let assign11580_e8684: f64 = (assign11580_e8680 / assign11580_e8683);
        locals.var_t4 = assign11580_e8684;
        locals.var_t4_dn3 = (((((locals.var_b4soitoxp_dn3 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn3)) * assign11580_e8683) - (assign11580_e8680 * (locals.var_pparam_b4soiweff_dn3 + locals.var_pparam_b4soiw0_dn3))) / (assign11580_e8683 * assign11580_e8683));
        locals.var_t4_dn4 = (((((locals.var_b4soitoxp_dn4 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn4)) * assign11580_e8683) - (assign11580_e8680 * (locals.var_pparam_b4soiweff_dn4 + locals.var_pparam_b4soiw0_dn4))) / (assign11580_e8683 * assign11580_e8683));
        locals.var_t4_dn5 = (((((locals.var_b4soitoxp_dn5 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn5)) * assign11580_e8683) - (assign11580_e8680 * (locals.var_pparam_b4soiweff_dn5 + locals.var_pparam_b4soiw0_dn5))) / (assign11580_e8683 * assign11580_e8683));
        locals.var_t4_dn6 = (((((locals.var_b4soitoxp_dn6 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn6)) * assign11580_e8683) - (assign11580_e8680 * (locals.var_pparam_b4soiweff_dn6 + locals.var_pparam_b4soiw0_dn6))) / (assign11580_e8683 * assign11580_e8683));
        locals.var_t4_dn7 = (((((locals.var_b4soitoxp_dn7 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn7)) * assign11580_e8683) - (assign11580_e8680 * (locals.var_pparam_b4soiweff_dn7 + locals.var_pparam_b4soiw0_dn7))) / (assign11580_e8683 * assign11580_e8683));
        locals.var_t4_dn8 = (((((locals.var_b4soitoxp_dn8 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn8)) * assign11580_e8683) - (assign11580_e8680 * (locals.var_pparam_b4soiweff_dn8 + locals.var_pparam_b4soiw0_dn8))) / (assign11580_e8683 * assign11580_e8683));
        locals.var_t4_dn9 = (((((locals.var_b4soitoxp_dn9 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn9)) * assign11580_e8683) - (assign11580_e8680 * (locals.var_pparam_b4soiweff_dn9 + locals.var_pparam_b4soiw0_dn9))) / (assign11580_e8683 * assign11580_e8683));
        locals.var_t4_dn10 = (((((locals.var_b4soitoxp_dn10 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn10)) * assign11580_e8683) - (assign11580_e8680 * (locals.var_pparam_b4soiweff_dn10 + locals.var_pparam_b4soiw0_dn10))) / (assign11580_e8683 * assign11580_e8683));
        locals.var_t4_dn11 = (((((locals.var_b4soitoxp_dn11 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn11)) * assign11580_e8683) - (assign11580_e8680 * (locals.var_pparam_b4soiweff_dn11 + locals.var_pparam_b4soiw0_dn11))) / (assign11580_e8683 * assign11580_e8683));
        locals.var_t4_dn12 = (((((locals.var_b4soitoxp_dn12 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn12)) * assign11580_e8683) - (assign11580_e8680 * (locals.var_pparam_b4soiweff_dn12 + locals.var_pparam_b4soiw0_dn12))) / (assign11580_e8683 * assign11580_e8683));

        let assign11590_e8688: f64 = (locals.var_pparam_b4soilpe0 / locals.var_pparam_b4soileff);
        let assign11590_e8689: f64 = (1.0 + assign11590_e8688);
        let assign11590_e8690: f64 = (assign11590_e8689).sqrt();
        locals.var_t0 = assign11590_e8690;
        locals.var_t0_dn3 = ((((locals.var_pparam_b4soilpe0_dn3 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn3)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign11590_e8690));
        locals.var_t0_dn4 = ((((locals.var_pparam_b4soilpe0_dn4 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn4)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign11590_e8690));
        locals.var_t0_dn5 = ((((locals.var_pparam_b4soilpe0_dn5 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn5)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign11590_e8690));
        locals.var_t0_dn6 = ((((locals.var_pparam_b4soilpe0_dn6 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn6)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign11590_e8690));
        locals.var_t0_dn7 = ((((locals.var_pparam_b4soilpe0_dn7 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn7)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign11590_e8690));
        locals.var_t0_dn8 = ((((locals.var_pparam_b4soilpe0_dn8 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn8)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign11590_e8690));
        locals.var_t0_dn9 = ((((locals.var_pparam_b4soilpe0_dn9 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn9)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign11590_e8690));
        locals.var_t0_dn10 = ((((locals.var_pparam_b4soilpe0_dn10 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn10)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign11590_e8690));
        locals.var_t0_dn11 = ((((locals.var_pparam_b4soilpe0_dn11 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn11)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign11590_e8690));
        locals.var_t0_dn12 = ((((locals.var_pparam_b4soilpe0_dn12 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn12)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign11590_e8690));

        let assign11600_e8694: f64 = (locals.var_t0 - 1.0);
        let assign11600_e8695: f64 = (locals.var_pparam_b4soik1ox * assign11600_e8694);
        let assign11600_e8697: f64 = (assign11600_e8695 * locals.var_pparam_b4soisqrtphi);
        let assign11600_e8701: f64 = (locals.var_pparam_b4soikt1l / locals.var_pparam_b4soileff);
        let assign11600_e8702: f64 = (locals.var_pparam_b4soikt1 + assign11600_e8701);
        let assign11600_e8705: f64 = (locals.var_tempratio__blk792 - 1.0);
        let assign11600_e8706: f64 = (assign11600_e8702 * assign11600_e8705);
        let assign11600_e8707: f64 = (assign11600_e8697 + assign11600_e8706);
        locals.var_t5 = assign11600_e8707;
        locals.var_t5_dn3 = (((((locals.var_pparam_b4soik1ox_dn3 * assign11600_e8694) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn3)) * locals.var_pparam_b4soisqrtphi) + (assign11600_e8695 * locals.var_pparam_b4soisqrtphi_dn3)) + ((locals.var_pparam_b4soikt1_dn3 + (((locals.var_pparam_b4soikt1l_dn3 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn3)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * assign11600_e8705));
        locals.var_t5_dn4 = (((((locals.var_pparam_b4soik1ox_dn4 * assign11600_e8694) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn4)) * locals.var_pparam_b4soisqrtphi) + (assign11600_e8695 * locals.var_pparam_b4soisqrtphi_dn4)) + ((locals.var_pparam_b4soikt1_dn4 + (((locals.var_pparam_b4soikt1l_dn4 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn4)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * assign11600_e8705));
        locals.var_t5_dn5 = (((((locals.var_pparam_b4soik1ox_dn5 * assign11600_e8694) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn5)) * locals.var_pparam_b4soisqrtphi) + (assign11600_e8695 * locals.var_pparam_b4soisqrtphi_dn5)) + ((locals.var_pparam_b4soikt1_dn5 + (((locals.var_pparam_b4soikt1l_dn5 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn5)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * assign11600_e8705));
        locals.var_t5_dn6 = (((((locals.var_pparam_b4soik1ox_dn6 * assign11600_e8694) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn6)) * locals.var_pparam_b4soisqrtphi) + (assign11600_e8695 * locals.var_pparam_b4soisqrtphi_dn6)) + (((locals.var_pparam_b4soikt1_dn6 + (((locals.var_pparam_b4soikt1l_dn6 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn6)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * assign11600_e8705) + (assign11600_e8702 * locals.var_tempratio__blk792_dn6)));
        locals.var_t5_dn7 = (((((locals.var_pparam_b4soik1ox_dn7 * assign11600_e8694) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn7)) * locals.var_pparam_b4soisqrtphi) + (assign11600_e8695 * locals.var_pparam_b4soisqrtphi_dn7)) + ((locals.var_pparam_b4soikt1_dn7 + (((locals.var_pparam_b4soikt1l_dn7 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn7)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * assign11600_e8705));
        locals.var_t5_dn8 = (((((locals.var_pparam_b4soik1ox_dn8 * assign11600_e8694) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn8)) * locals.var_pparam_b4soisqrtphi) + (assign11600_e8695 * locals.var_pparam_b4soisqrtphi_dn8)) + ((locals.var_pparam_b4soikt1_dn8 + (((locals.var_pparam_b4soikt1l_dn8 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn8)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * assign11600_e8705));
        locals.var_t5_dn9 = (((((locals.var_pparam_b4soik1ox_dn9 * assign11600_e8694) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn9)) * locals.var_pparam_b4soisqrtphi) + (assign11600_e8695 * locals.var_pparam_b4soisqrtphi_dn9)) + ((locals.var_pparam_b4soikt1_dn9 + (((locals.var_pparam_b4soikt1l_dn9 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn9)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * assign11600_e8705));
        locals.var_t5_dn10 = (((((locals.var_pparam_b4soik1ox_dn10 * assign11600_e8694) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn10)) * locals.var_pparam_b4soisqrtphi) + (assign11600_e8695 * locals.var_pparam_b4soisqrtphi_dn10)) + ((locals.var_pparam_b4soikt1_dn10 + (((locals.var_pparam_b4soikt1l_dn10 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn10)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * assign11600_e8705));
        locals.var_t5_dn11 = (((((locals.var_pparam_b4soik1ox_dn11 * assign11600_e8694) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn11)) * locals.var_pparam_b4soisqrtphi) + (assign11600_e8695 * locals.var_pparam_b4soisqrtphi_dn11)) + ((locals.var_pparam_b4soikt1_dn11 + (((locals.var_pparam_b4soikt1l_dn11 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn11)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * assign11600_e8705));
        locals.var_t5_dn12 = (((((locals.var_pparam_b4soik1ox_dn12 * assign11600_e8694) + (locals.var_pparam_b4soik1ox * locals.var_t0_dn12)) * locals.var_pparam_b4soisqrtphi) + (assign11600_e8695 * locals.var_pparam_b4soisqrtphi_dn12)) + ((locals.var_pparam_b4soikt1_dn12 + (((locals.var_pparam_b4soikt1l_dn12 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn12)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * assign11600_e8705));

    }

    pub(super) fn stamp_transient_block_25(
        locals: &mut StampLocals,
    ) {
        let assign11610_e8710: f64 = (locals.var_b4soitype * locals.var_pparam_b4soivth0);
        let assign11610_e8712: f64 = (assign11610_e8710 - locals.var_t2);
        let assign11610_e8714: f64 = (assign11610_e8712 - locals.var_t3);
        let assign11610_e8717: f64 = (locals.var_pparam_b4soik3 * locals.var_t4);
        let assign11610_e8718: f64 = (assign11610_e8714 + assign11610_e8717);
        let assign11610_e8720: f64 = (assign11610_e8718 + locals.var_t5);
        locals.var_tmp3 = assign11610_e8720;
        locals.var_tmp3_dn3 = (((((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn3) - locals.var_t2_dn3) - locals.var_t3_dn3) + ((locals.var_pparam_b4soik3_dn3 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn3))) + locals.var_t5_dn3);
        locals.var_tmp3_dn4 = (((((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn4) - locals.var_t2_dn4) - locals.var_t3_dn4) + ((locals.var_pparam_b4soik3_dn4 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn4))) + locals.var_t5_dn4);
        locals.var_tmp3_dn5 = (((((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn5) - locals.var_t2_dn5) - locals.var_t3_dn5) + ((locals.var_pparam_b4soik3_dn5 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn5))) + locals.var_t5_dn5);
        locals.var_tmp3_dn6 = (((((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn6) - locals.var_t2_dn6) - locals.var_t3_dn6) + ((locals.var_pparam_b4soik3_dn6 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn6))) + locals.var_t5_dn6);
        locals.var_tmp3_dn7 = (((((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn7) - locals.var_t2_dn7) - locals.var_t3_dn7) + ((locals.var_pparam_b4soik3_dn7 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn7))) + locals.var_t5_dn7);
        locals.var_tmp3_dn8 = (((((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn8) - locals.var_t2_dn8) - locals.var_t3_dn8) + ((locals.var_pparam_b4soik3_dn8 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn8))) + locals.var_t5_dn8);
        locals.var_tmp3_dn9 = (((((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn9) - locals.var_t2_dn9) - locals.var_t3_dn9) + ((locals.var_pparam_b4soik3_dn9 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn9))) + locals.var_t5_dn9);
        locals.var_tmp3_dn10 = (((((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn10) - locals.var_t2_dn10) - locals.var_t3_dn10) + ((locals.var_pparam_b4soik3_dn10 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn10))) + locals.var_t5_dn10);
        locals.var_tmp3_dn11 = (((((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn11) - locals.var_t2_dn11) - locals.var_t3_dn11) + ((locals.var_pparam_b4soik3_dn11 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn11))) + locals.var_t5_dn11);
        locals.var_tmp3_dn12 = (((((locals.var_b4soitype * locals.var_pparam_b4soivth0_dn12) - locals.var_t2_dn12) - locals.var_t3_dn12) + ((locals.var_pparam_b4soik3_dn12 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn12))) + locals.var_t5_dn12);

        let assign11620_e8723: f64 = (locals.var_tmp3 - locals.var_pparam_b4soiphi);
        let assign11620_e8726: f64 = (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi);
        let assign11620_e8727: f64 = (assign11620_e8723 - assign11620_e8726);
        locals.var_pparam_b4soivfbzb = assign11620_e8727;
        locals.var_pparam_b4soivfbzb_dn3 = ((locals.var_tmp3_dn3 - locals.var_pparam_b4soiphi_dn3) - ((locals.var_pparam_b4soik1_dn3 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn3)));
        locals.var_pparam_b4soivfbzb_dn4 = ((locals.var_tmp3_dn4 - locals.var_pparam_b4soiphi_dn4) - ((locals.var_pparam_b4soik1_dn4 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn4)));
        locals.var_pparam_b4soivfbzb_dn5 = ((locals.var_tmp3_dn5 - locals.var_pparam_b4soiphi_dn5) - ((locals.var_pparam_b4soik1_dn5 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn5)));
        locals.var_pparam_b4soivfbzb_dn6 = ((locals.var_tmp3_dn6 - locals.var_pparam_b4soiphi_dn6) - ((locals.var_pparam_b4soik1_dn6 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn6)));
        locals.var_pparam_b4soivfbzb_dn7 = ((locals.var_tmp3_dn7 - locals.var_pparam_b4soiphi_dn7) - ((locals.var_pparam_b4soik1_dn7 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn7)));
        locals.var_pparam_b4soivfbzb_dn8 = ((locals.var_tmp3_dn8 - locals.var_pparam_b4soiphi_dn8) - ((locals.var_pparam_b4soik1_dn8 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn8)));
        locals.var_pparam_b4soivfbzb_dn9 = ((locals.var_tmp3_dn9 - locals.var_pparam_b4soiphi_dn9) - ((locals.var_pparam_b4soik1_dn9 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn9)));
        locals.var_pparam_b4soivfbzb_dn10 = ((locals.var_tmp3_dn10 - locals.var_pparam_b4soiphi_dn10) - ((locals.var_pparam_b4soik1_dn10 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn10)));
        locals.var_pparam_b4soivfbzb_dn11 = ((locals.var_tmp3_dn11 - locals.var_pparam_b4soiphi_dn11) - ((locals.var_pparam_b4soik1_dn11 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn11)));
        locals.var_pparam_b4soivfbzb_dn12 = ((locals.var_tmp3_dn12 - locals.var_pparam_b4soiphi_dn12) - ((locals.var_pparam_b4soik1_dn12 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn12)));

        let assign11630_e8730: f64 = (1.60219e-19 * locals.var_pparam_b4soinpeak);
        let assign11630_e8734: f64 = (locals.var_pparam_b4soilpe0 / locals.var_pparam_b4soileff);
        let assign11630_e8735: f64 = (1.0 + assign11630_e8734);
        let assign11630_e8736: f64 = (assign11630_e8730 * assign11630_e8735);
        let assign11630_e8738: f64 = (assign11630_e8736 * 1000000.0);
        let assign11630_e8740: f64 = (assign11630_e8738 * locals.var_b4soitsi);
        locals.var_pparam_b4soiqsi = assign11630_e8740;
        locals.var_pparam_b4soiqsi_dn3 = (((((1.60219e-19 * locals.var_pparam_b4soinpeak_dn3) * assign11630_e8735) + (assign11630_e8730 * (((locals.var_pparam_b4soilpe0_dn3 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn3)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * locals.var_b4soitsi);
        locals.var_pparam_b4soiqsi_dn4 = (((((1.60219e-19 * locals.var_pparam_b4soinpeak_dn4) * assign11630_e8735) + (assign11630_e8730 * (((locals.var_pparam_b4soilpe0_dn4 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn4)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * locals.var_b4soitsi);
        locals.var_pparam_b4soiqsi_dn5 = (((((1.60219e-19 * locals.var_pparam_b4soinpeak_dn5) * assign11630_e8735) + (assign11630_e8730 * (((locals.var_pparam_b4soilpe0_dn5 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn5)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * locals.var_b4soitsi);
        locals.var_pparam_b4soiqsi_dn6 = (((((1.60219e-19 * locals.var_pparam_b4soinpeak_dn6) * assign11630_e8735) + (assign11630_e8730 * (((locals.var_pparam_b4soilpe0_dn6 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn6)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * locals.var_b4soitsi);
        locals.var_pparam_b4soiqsi_dn7 = (((((1.60219e-19 * locals.var_pparam_b4soinpeak_dn7) * assign11630_e8735) + (assign11630_e8730 * (((locals.var_pparam_b4soilpe0_dn7 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn7)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * locals.var_b4soitsi);
        locals.var_pparam_b4soiqsi_dn8 = (((((1.60219e-19 * locals.var_pparam_b4soinpeak_dn8) * assign11630_e8735) + (assign11630_e8730 * (((locals.var_pparam_b4soilpe0_dn8 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn8)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * locals.var_b4soitsi);
        locals.var_pparam_b4soiqsi_dn9 = (((((1.60219e-19 * locals.var_pparam_b4soinpeak_dn9) * assign11630_e8735) + (assign11630_e8730 * (((locals.var_pparam_b4soilpe0_dn9 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn9)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * locals.var_b4soitsi);
        locals.var_pparam_b4soiqsi_dn10 = (((((1.60219e-19 * locals.var_pparam_b4soinpeak_dn10) * assign11630_e8735) + (assign11630_e8730 * (((locals.var_pparam_b4soilpe0_dn10 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn10)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * locals.var_b4soitsi);
        locals.var_pparam_b4soiqsi_dn11 = (((((1.60219e-19 * locals.var_pparam_b4soinpeak_dn11) * assign11630_e8735) + (assign11630_e8730 * (((locals.var_pparam_b4soilpe0_dn11 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn11)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * locals.var_b4soitsi);
        locals.var_pparam_b4soiqsi_dn12 = (((((1.60219e-19 * locals.var_pparam_b4soinpeak_dn12) * assign11630_e8735) + (assign11630_e8730 * (((locals.var_pparam_b4soilpe0_dn12 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn12)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * locals.var_b4soitsi);

        let assign11640_e8745: f64 = (locals.var_pparam_b4soiweff / locals.var_b4soinseg);
        let assign11640_e8747: f64 = (assign11640_e8745 / 3.0);
        let assign11640_e8749: f64 = (assign11640_e8747 / locals.var_b4soingcon);
        let assign11640_e8750: f64 = (locals.var_b4soixgw + assign11640_e8749);
        let assign11640_e8751: f64 = (locals.var_b4soirshg * assign11640_e8750);
        let assign11640_e8754: f64 = (locals.var_b4soingcon * locals.var_b4soinf);
        let assign11640_e8757: f64 = (locals.var_b4soil - locals.var_b4soixgl);
        let assign11640_e8758: f64 = (assign11640_e8754 * assign11640_e8757);
        let assign11640_e8759: f64 = (assign11640_e8751 / assign11640_e8758);
        locals.var_b4soigrgeltd = assign11640_e8759;
        locals.var_b4soigrgeltd_dn3 = ((locals.var_b4soirshg * (((locals.var_pparam_b4soiweff_dn3 / locals.var_b4soinseg) / 3.0) / locals.var_b4soingcon)) / assign11640_e8758);
        locals.var_b4soigrgeltd_dn4 = ((locals.var_b4soirshg * (((locals.var_pparam_b4soiweff_dn4 / locals.var_b4soinseg) / 3.0) / locals.var_b4soingcon)) / assign11640_e8758);
        locals.var_b4soigrgeltd_dn5 = ((locals.var_b4soirshg * (((locals.var_pparam_b4soiweff_dn5 / locals.var_b4soinseg) / 3.0) / locals.var_b4soingcon)) / assign11640_e8758);
        locals.var_b4soigrgeltd_dn6 = ((locals.var_b4soirshg * (((locals.var_pparam_b4soiweff_dn6 / locals.var_b4soinseg) / 3.0) / locals.var_b4soingcon)) / assign11640_e8758);
        locals.var_b4soigrgeltd_dn7 = ((locals.var_b4soirshg * (((locals.var_pparam_b4soiweff_dn7 / locals.var_b4soinseg) / 3.0) / locals.var_b4soingcon)) / assign11640_e8758);
        locals.var_b4soigrgeltd_dn8 = ((locals.var_b4soirshg * (((locals.var_pparam_b4soiweff_dn8 / locals.var_b4soinseg) / 3.0) / locals.var_b4soingcon)) / assign11640_e8758);
        locals.var_b4soigrgeltd_dn9 = ((locals.var_b4soirshg * (((locals.var_pparam_b4soiweff_dn9 / locals.var_b4soinseg) / 3.0) / locals.var_b4soingcon)) / assign11640_e8758);
        locals.var_b4soigrgeltd_dn10 = ((locals.var_b4soirshg * (((locals.var_pparam_b4soiweff_dn10 / locals.var_b4soinseg) / 3.0) / locals.var_b4soingcon)) / assign11640_e8758);
        locals.var_b4soigrgeltd_dn11 = ((locals.var_b4soirshg * (((locals.var_pparam_b4soiweff_dn11 / locals.var_b4soinseg) / 3.0) / locals.var_b4soingcon)) / assign11640_e8758);
        locals.var_b4soigrgeltd_dn12 = ((locals.var_b4soirshg * (((locals.var_pparam_b4soiweff_dn12 / locals.var_b4soinseg) / 3.0) / locals.var_b4soingcon)) / assign11640_e8758);

        let assign11650_e8762: f64 = if locals.var_b4soigrgeltd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard953 = assign11650_e8762;

        let (assign11660_e8768, assign11660_e8768_d_n3, assign11660_e8768_d_n4, assign11660_e8768_d_n5, assign11660_e8768_d_n6, assign11660_e8768_d_n7, assign11660_e8768_d_n8, assign11660_e8768_d_n9, assign11660_e8768_d_n10, assign11660_e8768_d_n11, assign11660_e8768_d_n12,) = {
    if (locals.var_guard953 != 0.0) {
        let assign11660_e8766: f64 = (1.0 / locals.var_b4soigrgeltd);
        (assign11660_e8766, (-(locals.var_b4soigrgeltd_dn3 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))), (-(locals.var_b4soigrgeltd_dn4 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))), (-(locals.var_b4soigrgeltd_dn5 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))), (-(locals.var_b4soigrgeltd_dn6 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))), (-(locals.var_b4soigrgeltd_dn7 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))), (-(locals.var_b4soigrgeltd_dn8 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))), (-(locals.var_b4soigrgeltd_dn9 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))), (-(locals.var_b4soigrgeltd_dn10 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))), (-(locals.var_b4soigrgeltd_dn11 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))), (-(locals.var_b4soigrgeltd_dn12 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))),)
    } else {
        (locals.var_b4soigrgeltd, locals.var_b4soigrgeltd_dn3, locals.var_b4soigrgeltd_dn4, locals.var_b4soigrgeltd_dn5, locals.var_b4soigrgeltd_dn6, locals.var_b4soigrgeltd_dn7, locals.var_b4soigrgeltd_dn8, locals.var_b4soigrgeltd_dn9, locals.var_b4soigrgeltd_dn10, locals.var_b4soigrgeltd_dn11, locals.var_b4soigrgeltd_dn12,)
    }
};
        locals.var_b4soigrgeltd = assign11660_e8768;
        locals.var_b4soigrgeltd_dn3 = assign11660_e8768_d_n3;
        locals.var_b4soigrgeltd_dn4 = assign11660_e8768_d_n4;
        locals.var_b4soigrgeltd_dn5 = assign11660_e8768_d_n5;
        locals.var_b4soigrgeltd_dn6 = assign11660_e8768_d_n6;
        locals.var_b4soigrgeltd_dn7 = assign11660_e8768_d_n7;
        locals.var_b4soigrgeltd_dn8 = assign11660_e8768_d_n8;
        locals.var_b4soigrgeltd_dn9 = assign11660_e8768_d_n9;
        locals.var_b4soigrgeltd_dn10 = assign11660_e8768_d_n10;
        locals.var_b4soigrgeltd_dn11 = assign11660_e8768_d_n11;
        locals.var_b4soigrgeltd_dn12 = assign11660_e8768_d_n12;

        let (assign11670_e8773, assign11670_e8773_d_n3, assign11670_e8773_d_n4, assign11670_e8773_d_n5, assign11670_e8773_d_n6, assign11670_e8773_d_n7, assign11670_e8773_d_n8, assign11670_e8773_d_n9, assign11670_e8773_d_n10, assign11670_e8773_d_n11, assign11670_e8773_d_n12,) = {
    if (locals.var_guard953 == 0.0) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soigrgeltd, locals.var_b4soigrgeltd_dn3, locals.var_b4soigrgeltd_dn4, locals.var_b4soigrgeltd_dn5, locals.var_b4soigrgeltd_dn6, locals.var_b4soigrgeltd_dn7, locals.var_b4soigrgeltd_dn8, locals.var_b4soigrgeltd_dn9, locals.var_b4soigrgeltd_dn10, locals.var_b4soigrgeltd_dn11, locals.var_b4soigrgeltd_dn12,)
    }
};
        locals.var_b4soigrgeltd = assign11670_e8773;
        locals.var_b4soigrgeltd_dn3 = assign11670_e8773_d_n3;
        locals.var_b4soigrgeltd_dn4 = assign11670_e8773_d_n4;
        locals.var_b4soigrgeltd_dn5 = assign11670_e8773_d_n5;
        locals.var_b4soigrgeltd_dn6 = assign11670_e8773_d_n6;
        locals.var_b4soigrgeltd_dn7 = assign11670_e8773_d_n7;
        locals.var_b4soigrgeltd_dn8 = assign11670_e8773_d_n8;
        locals.var_b4soigrgeltd_dn9 = assign11670_e8773_d_n9;
        locals.var_b4soigrgeltd_dn10 = assign11670_e8773_d_n10;
        locals.var_b4soigrgeltd_dn11 = assign11670_e8773_d_n11;
        locals.var_b4soigrgeltd_dn12 = assign11670_e8773_d_n12;

        let assign11770_e8830: f64 = (locals.var_b4soitype * locals.var_b4soidelvto);
        let assign11770_e8831: f64 = (locals.var_pparam_b4soivfbzb + assign11770_e8830);
        locals.var_b4soivfbzb = assign11770_e8831;
        locals.var_b4soivfbzb_dn3 = locals.var_pparam_b4soivfbzb_dn3;
        locals.var_b4soivfbzb_dn4 = locals.var_pparam_b4soivfbzb_dn4;
        locals.var_b4soivfbzb_dn5 = locals.var_pparam_b4soivfbzb_dn5;
        locals.var_b4soivfbzb_dn6 = locals.var_pparam_b4soivfbzb_dn6;
        locals.var_b4soivfbzb_dn7 = locals.var_pparam_b4soivfbzb_dn7;
        locals.var_b4soivfbzb_dn8 = locals.var_pparam_b4soivfbzb_dn8;
        locals.var_b4soivfbzb_dn9 = locals.var_pparam_b4soivfbzb_dn9;
        locals.var_b4soivfbzb_dn10 = locals.var_pparam_b4soivfbzb_dn10;
        locals.var_b4soivfbzb_dn11 = locals.var_pparam_b4soivfbzb_dn11;
        locals.var_b4soivfbzb_dn12 = locals.var_pparam_b4soivfbzb_dn12;

        let assign11780_e8834: f64 = (locals.var_epssub * locals.var_vtm0);
        let assign11780_e8837: f64 = (1.60219e-19 * locals.var_pparam_b4soinpeak);
        let assign11780_e8839: f64 = (assign11780_e8837 * 1000000.0);
        let assign11780_e8840: f64 = (assign11780_e8834 / assign11780_e8839);
        let assign11780_e8841: f64 = (assign11780_e8840).sqrt();
        let assign11780_e8843: f64 = (assign11780_e8841 / 3.0);
        locals.var_pparam_b4soildeb = assign11780_e8843;
        locals.var_pparam_b4soildeb_dn3 = (((-((assign11780_e8834 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn3) * 1000000.0)) / (assign11780_e8839 * assign11780_e8839))) / (2.0 * assign11780_e8841)) / 3.0);
        locals.var_pparam_b4soildeb_dn4 = (((-((assign11780_e8834 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn4) * 1000000.0)) / (assign11780_e8839 * assign11780_e8839))) / (2.0 * assign11780_e8841)) / 3.0);
        locals.var_pparam_b4soildeb_dn5 = (((-((assign11780_e8834 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn5) * 1000000.0)) / (assign11780_e8839 * assign11780_e8839))) / (2.0 * assign11780_e8841)) / 3.0);
        locals.var_pparam_b4soildeb_dn6 = (((-((assign11780_e8834 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn6) * 1000000.0)) / (assign11780_e8839 * assign11780_e8839))) / (2.0 * assign11780_e8841)) / 3.0);
        locals.var_pparam_b4soildeb_dn7 = (((-((assign11780_e8834 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn7) * 1000000.0)) / (assign11780_e8839 * assign11780_e8839))) / (2.0 * assign11780_e8841)) / 3.0);
        locals.var_pparam_b4soildeb_dn8 = (((-((assign11780_e8834 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn8) * 1000000.0)) / (assign11780_e8839 * assign11780_e8839))) / (2.0 * assign11780_e8841)) / 3.0);
        locals.var_pparam_b4soildeb_dn9 = (((-((assign11780_e8834 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn9) * 1000000.0)) / (assign11780_e8839 * assign11780_e8839))) / (2.0 * assign11780_e8841)) / 3.0);
        locals.var_pparam_b4soildeb_dn10 = (((-((assign11780_e8834 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn10) * 1000000.0)) / (assign11780_e8839 * assign11780_e8839))) / (2.0 * assign11780_e8841)) / 3.0);
        locals.var_pparam_b4soildeb_dn11 = (((-((assign11780_e8834 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn11) * 1000000.0)) / (assign11780_e8839 * assign11780_e8839))) / (2.0 * assign11780_e8841)) / 3.0);
        locals.var_pparam_b4soildeb_dn12 = (((-((assign11780_e8834 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn12) * 1000000.0)) / (assign11780_e8839 * assign11780_e8839))) / (2.0 * assign11780_e8841)) / 3.0);

        let assign11790_e8846: f64 = (locals.var_b4soitype * locals.var_here_b4soivth0);
        let assign11790_e8848: f64 = (assign11790_e8846 - locals.var_here_b4soivfb);
        let assign11790_e8850: f64 = (assign11790_e8848 - locals.var_pparam_b4soiphi);
        locals.var_t1 = assign11790_e8850;
        locals.var_t1_dn3 = (((locals.var_b4soitype * locals.var_here_b4soivth0_dn3) - locals.var_here_b4soivfb_dn3) - locals.var_pparam_b4soiphi_dn3);
        locals.var_t1_dn4 = (((locals.var_b4soitype * locals.var_here_b4soivth0_dn4) - locals.var_here_b4soivfb_dn4) - locals.var_pparam_b4soiphi_dn4);
        locals.var_t1_dn5 = (((locals.var_b4soitype * locals.var_here_b4soivth0_dn5) - locals.var_here_b4soivfb_dn5) - locals.var_pparam_b4soiphi_dn5);
        locals.var_t1_dn6 = (((locals.var_b4soitype * locals.var_here_b4soivth0_dn6) - locals.var_here_b4soivfb_dn6) - locals.var_pparam_b4soiphi_dn6);
        locals.var_t1_dn7 = (((locals.var_b4soitype * locals.var_here_b4soivth0_dn7) - locals.var_here_b4soivfb_dn7) - locals.var_pparam_b4soiphi_dn7);
        locals.var_t1_dn8 = (((locals.var_b4soitype * locals.var_here_b4soivth0_dn8) - locals.var_here_b4soivfb_dn8) - locals.var_pparam_b4soiphi_dn8);
        locals.var_t1_dn9 = (((locals.var_b4soitype * locals.var_here_b4soivth0_dn9) - locals.var_here_b4soivfb_dn9) - locals.var_pparam_b4soiphi_dn9);
        locals.var_t1_dn10 = (((locals.var_b4soitype * locals.var_here_b4soivth0_dn10) - locals.var_here_b4soivfb_dn10) - locals.var_pparam_b4soiphi_dn10);
        locals.var_t1_dn11 = (((locals.var_b4soitype * locals.var_here_b4soivth0_dn11) - locals.var_here_b4soivfb_dn11) - locals.var_pparam_b4soiphi_dn11);
        locals.var_t1_dn12 = (((locals.var_b4soitype * locals.var_here_b4soivth0_dn12) - locals.var_here_b4soivfb_dn12) - locals.var_pparam_b4soiphi_dn12);

        let assign11800_e8853: f64 = (locals.var_t1 + locals.var_t1);
        locals.var_t2 = assign11800_e8853;
        locals.var_t2_dn3 = (locals.var_t1_dn3 + locals.var_t1_dn3);
        locals.var_t2_dn4 = (locals.var_t1_dn4 + locals.var_t1_dn4);
        locals.var_t2_dn5 = (locals.var_t1_dn5 + locals.var_t1_dn5);
        locals.var_t2_dn6 = (locals.var_t1_dn6 + locals.var_t1_dn6);
        locals.var_t2_dn7 = (locals.var_t1_dn7 + locals.var_t1_dn7);
        locals.var_t2_dn8 = (locals.var_t1_dn8 + locals.var_t1_dn8);
        locals.var_t2_dn9 = (locals.var_t1_dn9 + locals.var_t1_dn9);
        locals.var_t2_dn10 = (locals.var_t1_dn10 + locals.var_t1_dn10);
        locals.var_t2_dn11 = (locals.var_t1_dn11 + locals.var_t1_dn11);
        locals.var_t2_dn12 = (locals.var_t1_dn12 + locals.var_t1_dn12);

        let assign11810_e8856: f64 = (2.5 * locals.var_t1);
        locals.var_t3 = assign11810_e8856;
        locals.var_t3_dn3 = (2.5 * locals.var_t1_dn3);
        locals.var_t3_dn4 = (2.5 * locals.var_t1_dn4);
        locals.var_t3_dn5 = (2.5 * locals.var_t1_dn5);
        locals.var_t3_dn6 = (2.5 * locals.var_t1_dn6);
        locals.var_t3_dn7 = (2.5 * locals.var_t1_dn7);
        locals.var_t3_dn8 = (2.5 * locals.var_t1_dn8);
        locals.var_t3_dn9 = (2.5 * locals.var_t1_dn9);
        locals.var_t3_dn10 = (2.5 * locals.var_t1_dn10);
        locals.var_t3_dn11 = (2.5 * locals.var_t1_dn11);
        locals.var_t3_dn12 = (2.5 * locals.var_t1_dn12);

        let (assign11820_e8862, assign11820_e8862_d_n3, assign11820_e8862_d_n4, assign11820_e8862_d_n5, assign11820_e8862_d_n6, assign11820_e8862_d_n7, assign11820_e8862_d_n8, assign11820_e8862_d_n9, assign11820_e8862_d_n10, assign11820_e8862_d_n11, assign11820_e8862_d_n12,) = {
    if (locals.var_b4soitype == 1.0) {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_b4soivtfbphi1 = assign11820_e8862;
        locals.var_b4soivtfbphi1_dn3 = assign11820_e8862_d_n3;
        locals.var_b4soivtfbphi1_dn4 = assign11820_e8862_d_n4;
        locals.var_b4soivtfbphi1_dn5 = assign11820_e8862_d_n5;
        locals.var_b4soivtfbphi1_dn6 = assign11820_e8862_d_n6;
        locals.var_b4soivtfbphi1_dn7 = assign11820_e8862_d_n7;
        locals.var_b4soivtfbphi1_dn8 = assign11820_e8862_d_n8;
        locals.var_b4soivtfbphi1_dn9 = assign11820_e8862_d_n9;
        locals.var_b4soivtfbphi1_dn10 = assign11820_e8862_d_n10;
        locals.var_b4soivtfbphi1_dn11 = assign11820_e8862_d_n11;
        locals.var_b4soivtfbphi1_dn12 = assign11820_e8862_d_n12;

        let assign11830_e8865: f64 = if locals.var_b4soivtfbphi1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard957 = assign11830_e8865;

        let (assign11840_e8869, assign11840_e8869_d_n3, assign11840_e8869_d_n4, assign11840_e8869_d_n5, assign11840_e8869_d_n6, assign11840_e8869_d_n7, assign11840_e8869_d_n8, assign11840_e8869_d_n9, assign11840_e8869_d_n10, assign11840_e8869_d_n11, assign11840_e8869_d_n12,) = {
    if (locals.var_guard957 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soivtfbphi1, locals.var_b4soivtfbphi1_dn3, locals.var_b4soivtfbphi1_dn4, locals.var_b4soivtfbphi1_dn5, locals.var_b4soivtfbphi1_dn6, locals.var_b4soivtfbphi1_dn7, locals.var_b4soivtfbphi1_dn8, locals.var_b4soivtfbphi1_dn9, locals.var_b4soivtfbphi1_dn10, locals.var_b4soivtfbphi1_dn11, locals.var_b4soivtfbphi1_dn12,)
    }
};
        locals.var_b4soivtfbphi1 = assign11840_e8869;
        locals.var_b4soivtfbphi1_dn3 = assign11840_e8869_d_n3;
        locals.var_b4soivtfbphi1_dn4 = assign11840_e8869_d_n4;
        locals.var_b4soivtfbphi1_dn5 = assign11840_e8869_d_n5;
        locals.var_b4soivtfbphi1_dn6 = assign11840_e8869_d_n6;
        locals.var_b4soivtfbphi1_dn7 = assign11840_e8869_d_n7;
        locals.var_b4soivtfbphi1_dn8 = assign11840_e8869_d_n8;
        locals.var_b4soivtfbphi1_dn9 = assign11840_e8869_d_n9;
        locals.var_b4soivtfbphi1_dn10 = assign11840_e8869_d_n10;
        locals.var_b4soivtfbphi1_dn11 = assign11840_e8869_d_n11;
        locals.var_b4soivtfbphi1_dn12 = assign11840_e8869_d_n12;

        let assign11850_e8872: f64 = if locals.var_b4soimobmod == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard958 = assign11850_e8872;

        let (assign11860_e8878, assign11860_e8878_d_n3, assign11860_e8878_d_n4, assign11860_e8878_d_n5, assign11860_e8878_d_n6, assign11860_e8878_d_n7, assign11860_e8878_d_n8, assign11860_e8878_d_n9, assign11860_e8878_d_n10, assign11860_e8878_d_n11, assign11860_e8878_d_n12,) = {
    if (locals.var_guard958 != 0.0) {
        let assign11860_e8876: f64 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0);
        (assign11860_e8876, (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn3), (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn4), (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn5), (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn6), (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn7), (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn8), (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn9), (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn10), (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn11), (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn12),)
    } else {
        (locals.var_lt1, locals.var_lt1_dn3, locals.var_lt1_dn4, locals.var_lt1_dn5, locals.var_lt1_dn6, locals.var_lt1_dn7, locals.var_lt1_dn8, locals.var_lt1_dn9, locals.var_lt1_dn10, locals.var_lt1_dn11, locals.var_lt1_dn12,)
    }
};
        locals.var_lt1 = assign11860_e8878;
        locals.var_lt1_dn3 = assign11860_e8878_d_n3;
        locals.var_lt1_dn4 = assign11860_e8878_d_n4;
        locals.var_lt1_dn5 = assign11860_e8878_d_n5;
        locals.var_lt1_dn6 = assign11860_e8878_d_n6;
        locals.var_lt1_dn7 = assign11860_e8878_d_n7;
        locals.var_lt1_dn8 = assign11860_e8878_d_n8;
        locals.var_lt1_dn9 = assign11860_e8878_d_n9;
        locals.var_lt1_dn10 = assign11860_e8878_d_n10;
        locals.var_lt1_dn11 = assign11860_e8878_d_n11;
        locals.var_lt1_dn12 = assign11860_e8878_d_n12;

        let (assign11870_e8886, assign11870_e8886_d_n3, assign11870_e8886_d_n4, assign11870_e8886_d_n5, assign11870_e8886_d_n6, assign11870_e8886_d_n7, assign11870_e8886_d_n8, assign11870_e8886_d_n9, assign11870_e8886_d_n10, assign11870_e8886_d_n11, assign11870_e8886_d_n12,) = {
    if (locals.var_guard958 != 0.0) {
        let assign11870_e8882: f64 = (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff);
        let assign11870_e8884: f64 = (assign11870_e8882 / locals.var_lt1);
        (assign11870_e8884, (((((locals.var_pparam_b4soidvt1_dn3 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn3)) * locals.var_lt1) - (assign11870_e8882 * locals.var_lt1_dn3)) / (locals.var_lt1 * locals.var_lt1)), (((((locals.var_pparam_b4soidvt1_dn4 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn4)) * locals.var_lt1) - (assign11870_e8882 * locals.var_lt1_dn4)) / (locals.var_lt1 * locals.var_lt1)), (((((locals.var_pparam_b4soidvt1_dn5 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn5)) * locals.var_lt1) - (assign11870_e8882 * locals.var_lt1_dn5)) / (locals.var_lt1 * locals.var_lt1)), (((((locals.var_pparam_b4soidvt1_dn6 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn6)) * locals.var_lt1) - (assign11870_e8882 * locals.var_lt1_dn6)) / (locals.var_lt1 * locals.var_lt1)), (((((locals.var_pparam_b4soidvt1_dn7 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn7)) * locals.var_lt1) - (assign11870_e8882 * locals.var_lt1_dn7)) / (locals.var_lt1 * locals.var_lt1)), (((((locals.var_pparam_b4soidvt1_dn8 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn8)) * locals.var_lt1) - (assign11870_e8882 * locals.var_lt1_dn8)) / (locals.var_lt1 * locals.var_lt1)), (((((locals.var_pparam_b4soidvt1_dn9 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn9)) * locals.var_lt1) - (assign11870_e8882 * locals.var_lt1_dn9)) / (locals.var_lt1 * locals.var_lt1)), (((((locals.var_pparam_b4soidvt1_dn10 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn10)) * locals.var_lt1) - (assign11870_e8882 * locals.var_lt1_dn10)) / (locals.var_lt1 * locals.var_lt1)), (((((locals.var_pparam_b4soidvt1_dn11 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn11)) * locals.var_lt1) - (assign11870_e8882 * locals.var_lt1_dn11)) / (locals.var_lt1 * locals.var_lt1)), (((((locals.var_pparam_b4soidvt1_dn12 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn12)) * locals.var_lt1) - (assign11870_e8882 * locals.var_lt1_dn12)) / (locals.var_lt1 * locals.var_lt1)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign11870_e8886;
        locals.var_t0_dn3 = assign11870_e8886_d_n3;
        locals.var_t0_dn4 = assign11870_e8886_d_n4;
        locals.var_t0_dn5 = assign11870_e8886_d_n5;
        locals.var_t0_dn6 = assign11870_e8886_d_n6;
        locals.var_t0_dn7 = assign11870_e8886_d_n7;
        locals.var_t0_dn8 = assign11870_e8886_d_n8;
        locals.var_t0_dn9 = assign11870_e8886_d_n9;
        locals.var_t0_dn10 = assign11870_e8886_d_n10;
        locals.var_t0_dn11 = assign11870_e8886_d_n11;
        locals.var_t0_dn12 = assign11870_e8886_d_n12;

        let assign11880_e8889: f64 = if locals.var_t0 < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard959 = assign11880_e8889;

        let (assign11890_e8896, assign11890_e8896_d_n3, assign11890_e8896_d_n4, assign11890_e8896_d_n5, assign11890_e8896_d_n6, assign11890_e8896_d_n7, assign11890_e8896_d_n8, assign11890_e8896_d_n9, assign11890_e8896_d_n10, assign11890_e8896_d_n11, assign11890_e8896_d_n12,) = {
    if ((locals.var_guard958 != 0.0) && (locals.var_guard959 != 0.0)) {
        let assign11890_e8894: f64 = (locals.var_t0).exp();
        (assign11890_e8894, (assign11890_e8894 * locals.var_t0_dn3), (assign11890_e8894 * locals.var_t0_dn4), (assign11890_e8894 * locals.var_t0_dn5), (assign11890_e8894 * locals.var_t0_dn6), (assign11890_e8894 * locals.var_t0_dn7), (assign11890_e8894 * locals.var_t0_dn8), (assign11890_e8894 * locals.var_t0_dn9), (assign11890_e8894 * locals.var_t0_dn10), (assign11890_e8894 * locals.var_t0_dn11), (assign11890_e8894 * locals.var_t0_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign11890_e8896;
        locals.var_t1_dn3 = assign11890_e8896_d_n3;
        locals.var_t1_dn4 = assign11890_e8896_d_n4;
        locals.var_t1_dn5 = assign11890_e8896_d_n5;
        locals.var_t1_dn6 = assign11890_e8896_d_n6;
        locals.var_t1_dn7 = assign11890_e8896_d_n7;
        locals.var_t1_dn8 = assign11890_e8896_d_n8;
        locals.var_t1_dn9 = assign11890_e8896_d_n9;
        locals.var_t1_dn10 = assign11890_e8896_d_n10;
        locals.var_t1_dn11 = assign11890_e8896_d_n11;
        locals.var_t1_dn12 = assign11890_e8896_d_n12;

        let (assign11900_e8904, assign11900_e8904_d_n3, assign11900_e8904_d_n4, assign11900_e8904_d_n5, assign11900_e8904_d_n6, assign11900_e8904_d_n7, assign11900_e8904_d_n8, assign11900_e8904_d_n9, assign11900_e8904_d_n10, assign11900_e8904_d_n11, assign11900_e8904_d_n12,) = {
    if ((locals.var_guard958 != 0.0) && (locals.var_guard959 != 0.0)) {
        let assign11900_e8902: f64 = (locals.var_t1 - 1.0);
        (assign11900_e8902, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign11900_e8904;
        locals.var_t2_dn3 = assign11900_e8904_d_n3;
        locals.var_t2_dn4 = assign11900_e8904_d_n4;
        locals.var_t2_dn5 = assign11900_e8904_d_n5;
        locals.var_t2_dn6 = assign11900_e8904_d_n6;
        locals.var_t2_dn7 = assign11900_e8904_d_n7;
        locals.var_t2_dn8 = assign11900_e8904_d_n8;
        locals.var_t2_dn9 = assign11900_e8904_d_n9;
        locals.var_t2_dn10 = assign11900_e8904_d_n10;
        locals.var_t2_dn11 = assign11900_e8904_d_n11;
        locals.var_t2_dn12 = assign11900_e8904_d_n12;

        let (assign11910_e8912, assign11910_e8912_d_n3, assign11910_e8912_d_n4, assign11910_e8912_d_n5, assign11910_e8912_d_n6, assign11910_e8912_d_n7, assign11910_e8912_d_n8, assign11910_e8912_d_n9, assign11910_e8912_d_n10, assign11910_e8912_d_n11, assign11910_e8912_d_n12,) = {
    if ((locals.var_guard958 != 0.0) && (locals.var_guard959 != 0.0)) {
        let assign11910_e8910: f64 = (locals.var_t2 * locals.var_t2);
        (assign11910_e8910, ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign11910_e8912;
        locals.var_t3_dn3 = assign11910_e8912_d_n3;
        locals.var_t3_dn4 = assign11910_e8912_d_n4;
        locals.var_t3_dn5 = assign11910_e8912_d_n5;
        locals.var_t3_dn6 = assign11910_e8912_d_n6;
        locals.var_t3_dn7 = assign11910_e8912_d_n7;
        locals.var_t3_dn8 = assign11910_e8912_d_n8;
        locals.var_t3_dn9 = assign11910_e8912_d_n9;
        locals.var_t3_dn10 = assign11910_e8912_d_n10;
        locals.var_t3_dn11 = assign11910_e8912_d_n11;
        locals.var_t3_dn12 = assign11910_e8912_d_n12;

        let (assign11920_e8924, assign11920_e8924_d_n3, assign11920_e8924_d_n4, assign11920_e8924_d_n5, assign11920_e8924_d_n6, assign11920_e8924_d_n7, assign11920_e8924_d_n8, assign11920_e8924_d_n9, assign11920_e8924_d_n10, assign11920_e8924_d_n11, assign11920_e8924_d_n12,) = {
    if ((locals.var_guard958 != 0.0) && (locals.var_guard959 != 0.0)) {
        let assign11920_e8919: f64 = (2.0 * locals.var_t1);
        let assign11920_e8921: f64 = (assign11920_e8919 * 3.720075976e-44);
        let assign11920_e8922: f64 = (locals.var_t3 + assign11920_e8921);
        (assign11920_e8922, (locals.var_t3_dn3 + ((2.0 * locals.var_t1_dn3) * 3.720075976e-44)), (locals.var_t3_dn4 + ((2.0 * locals.var_t1_dn4) * 3.720075976e-44)), (locals.var_t3_dn5 + ((2.0 * locals.var_t1_dn5) * 3.720075976e-44)), (locals.var_t3_dn6 + ((2.0 * locals.var_t1_dn6) * 3.720075976e-44)), (locals.var_t3_dn7 + ((2.0 * locals.var_t1_dn7) * 3.720075976e-44)), (locals.var_t3_dn8 + ((2.0 * locals.var_t1_dn8) * 3.720075976e-44)), (locals.var_t3_dn9 + ((2.0 * locals.var_t1_dn9) * 3.720075976e-44)), (locals.var_t3_dn10 + ((2.0 * locals.var_t1_dn10) * 3.720075976e-44)), (locals.var_t3_dn11 + ((2.0 * locals.var_t1_dn11) * 3.720075976e-44)), (locals.var_t3_dn12 + ((2.0 * locals.var_t1_dn12) * 3.720075976e-44)),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign11920_e8924;
        locals.var_t4_dn3 = assign11920_e8924_d_n3;
        locals.var_t4_dn4 = assign11920_e8924_d_n4;
        locals.var_t4_dn5 = assign11920_e8924_d_n5;
        locals.var_t4_dn6 = assign11920_e8924_d_n6;
        locals.var_t4_dn7 = assign11920_e8924_d_n7;
        locals.var_t4_dn8 = assign11920_e8924_d_n8;
        locals.var_t4_dn9 = assign11920_e8924_d_n9;
        locals.var_t4_dn10 = assign11920_e8924_d_n10;
        locals.var_t4_dn11 = assign11920_e8924_d_n11;
        locals.var_t4_dn12 = assign11920_e8924_d_n12;

        let (assign11930_e8932, assign11930_e8932_d_n3, assign11930_e8932_d_n4, assign11930_e8932_d_n5, assign11930_e8932_d_n6, assign11930_e8932_d_n7, assign11930_e8932_d_n8, assign11930_e8932_d_n9, assign11930_e8932_d_n10, assign11930_e8932_d_n11, assign11930_e8932_d_n12,) = {
    if ((locals.var_guard958 != 0.0) && (locals.var_guard959 != 0.0)) {
        let assign11930_e8930: f64 = (locals.var_t1 / locals.var_t4);
        (assign11930_e8930, (((locals.var_t1_dn3 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t1_dn4 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t1_dn5 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t1_dn6 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t1_dn7 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t1_dn8 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t1_dn9 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t1_dn10 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t1_dn11 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t1_dn12 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_theta0, locals.var_theta0_dn3, locals.var_theta0_dn4, locals.var_theta0_dn5, locals.var_theta0_dn6, locals.var_theta0_dn7, locals.var_theta0_dn8, locals.var_theta0_dn9, locals.var_theta0_dn10, locals.var_theta0_dn11, locals.var_theta0_dn12,)
    }
};
        locals.var_theta0 = assign11930_e8932;
        locals.var_theta0_dn3 = assign11930_e8932_d_n3;
        locals.var_theta0_dn4 = assign11930_e8932_d_n4;
        locals.var_theta0_dn5 = assign11930_e8932_d_n5;
        locals.var_theta0_dn6 = assign11930_e8932_d_n6;
        locals.var_theta0_dn7 = assign11930_e8932_d_n7;
        locals.var_theta0_dn8 = assign11930_e8932_d_n8;
        locals.var_theta0_dn9 = assign11930_e8932_d_n9;
        locals.var_theta0_dn10 = assign11930_e8932_d_n10;
        locals.var_theta0_dn11 = assign11930_e8932_d_n11;
        locals.var_theta0_dn12 = assign11930_e8932_d_n12;

        let (assign11940_e8943, assign11940_e8943_d_n3, assign11940_e8943_d_n4, assign11940_e8943_d_n5, assign11940_e8943_d_n6, assign11940_e8943_d_n7, assign11940_e8943_d_n8, assign11940_e8943_d_n9, assign11940_e8943_d_n10, assign11940_e8943_d_n11, assign11940_e8943_d_n12,) = {
    if ((locals.var_guard958 != 0.0) && (locals.var_guard959 == 0.0)) {
        let assign11940_e8940: f64 = (2.688117142e43 - 2.0);
        let assign11940_e8941: f64 = (1.0 / assign11940_e8940);
        (assign11940_e8941, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_theta0, locals.var_theta0_dn3, locals.var_theta0_dn4, locals.var_theta0_dn5, locals.var_theta0_dn6, locals.var_theta0_dn7, locals.var_theta0_dn8, locals.var_theta0_dn9, locals.var_theta0_dn10, locals.var_theta0_dn11, locals.var_theta0_dn12,)
    }
};
        locals.var_theta0 = assign11940_e8943;
        locals.var_theta0_dn3 = assign11940_e8943_d_n3;
        locals.var_theta0_dn4 = assign11940_e8943_d_n4;
        locals.var_theta0_dn5 = assign11940_e8943_d_n5;
        locals.var_theta0_dn6 = assign11940_e8943_d_n6;
        locals.var_theta0_dn7 = assign11940_e8943_d_n7;
        locals.var_theta0_dn8 = assign11940_e8943_d_n8;
        locals.var_theta0_dn9 = assign11940_e8943_d_n9;
        locals.var_theta0_dn10 = assign11940_e8943_d_n10;
        locals.var_theta0_dn11 = assign11940_e8943_d_n11;
        locals.var_theta0_dn12 = assign11940_e8943_d_n12;

        let (assign11950_e8949, assign11950_e8949_d_n3, assign11950_e8949_d_n4, assign11950_e8949_d_n5, assign11950_e8949_d_n6, assign11950_e8949_d_n7, assign11950_e8949_d_n8, assign11950_e8949_d_n9, assign11950_e8949_d_n10, assign11950_e8949_d_n11, assign11950_e8949_d_n12,) = {
    if (locals.var_guard958 != 0.0) {
        let assign11950_e8947: f64 = (locals.var_epssub / locals.var_pparam_b4soixdep0);
        (assign11950_e8947, (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn3) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))), (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn4) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))), (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn5) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))), (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn6) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))), (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn7) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))), (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn8) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))), (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn9) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))), (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn10) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))), (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn11) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))), (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn12) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))),)
    } else {
        (locals.var_tmp1, locals.var_tmp1_dn3, locals.var_tmp1_dn4, locals.var_tmp1_dn5, locals.var_tmp1_dn6, locals.var_tmp1_dn7, locals.var_tmp1_dn8, locals.var_tmp1_dn9, locals.var_tmp1_dn10, locals.var_tmp1_dn11, locals.var_tmp1_dn12,)
    }
};
        locals.var_tmp1 = assign11950_e8949;
        locals.var_tmp1_dn3 = assign11950_e8949_d_n3;
        locals.var_tmp1_dn4 = assign11950_e8949_d_n4;
        locals.var_tmp1_dn5 = assign11950_e8949_d_n5;
        locals.var_tmp1_dn6 = assign11950_e8949_d_n6;
        locals.var_tmp1_dn7 = assign11950_e8949_d_n7;
        locals.var_tmp1_dn8 = assign11950_e8949_d_n8;
        locals.var_tmp1_dn9 = assign11950_e8949_d_n9;
        locals.var_tmp1_dn10 = assign11950_e8949_d_n10;
        locals.var_tmp1_dn11 = assign11950_e8949_d_n11;
        locals.var_tmp1_dn12 = assign11950_e8949_d_n12;

        let (assign11960_e8955, assign11960_e8955_d_n3, assign11960_e8955_d_n4, assign11960_e8955_d_n5, assign11960_e8955_d_n6, assign11960_e8955_d_n7, assign11960_e8955_d_n8, assign11960_e8955_d_n9, assign11960_e8955_d_n10, assign11960_e8955_d_n11, assign11960_e8955_d_n12,) = {
    if (locals.var_guard958 != 0.0) {
        let assign11960_e8953: f64 = (locals.var_pparam_b4soinfactor * locals.var_tmp1);
        (assign11960_e8953, ((locals.var_pparam_b4soinfactor_dn3 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn3)), ((locals.var_pparam_b4soinfactor_dn4 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn4)), ((locals.var_pparam_b4soinfactor_dn5 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn5)), ((locals.var_pparam_b4soinfactor_dn6 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn6)), ((locals.var_pparam_b4soinfactor_dn7 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn7)), ((locals.var_pparam_b4soinfactor_dn8 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn8)), ((locals.var_pparam_b4soinfactor_dn9 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn9)), ((locals.var_pparam_b4soinfactor_dn10 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn10)), ((locals.var_pparam_b4soinfactor_dn11 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn11)), ((locals.var_pparam_b4soinfactor_dn12 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn12)),)
    } else {
        (locals.var_tmp2, locals.var_tmp2_dn3, locals.var_tmp2_dn4, locals.var_tmp2_dn5, locals.var_tmp2_dn6, locals.var_tmp2_dn7, locals.var_tmp2_dn8, locals.var_tmp2_dn9, locals.var_tmp2_dn10, locals.var_tmp2_dn11, locals.var_tmp2_dn12,)
    }
};
        locals.var_tmp2 = assign11960_e8955;
        locals.var_tmp2_dn3 = assign11960_e8955_d_n3;
        locals.var_tmp2_dn4 = assign11960_e8955_d_n4;
        locals.var_tmp2_dn5 = assign11960_e8955_d_n5;
        locals.var_tmp2_dn6 = assign11960_e8955_d_n6;
        locals.var_tmp2_dn7 = assign11960_e8955_d_n7;
        locals.var_tmp2_dn8 = assign11960_e8955_d_n8;
        locals.var_tmp2_dn9 = assign11960_e8955_d_n9;
        locals.var_tmp2_dn10 = assign11960_e8955_d_n10;
        locals.var_tmp2_dn11 = assign11960_e8955_d_n11;
        locals.var_tmp2_dn12 = assign11960_e8955_d_n12;

        let (assign11970_e8967, assign11970_e8967_d_n3, assign11970_e8967_d_n4, assign11970_e8967_d_n5, assign11970_e8967_d_n6, assign11970_e8967_d_n7, assign11970_e8967_d_n8, assign11970_e8967_d_n9, assign11970_e8967_d_n10, assign11970_e8967_d_n11, assign11970_e8967_d_n12,) = {
    if (locals.var_guard958 != 0.0) {
        let assign11970_e8960: f64 = (locals.var_pparam_b4soicdsc * locals.var_theta0);
        let assign11970_e8961: f64 = (locals.var_tmp2 + assign11970_e8960);
        let assign11970_e8963: f64 = (assign11970_e8961 + locals.var_pparam_b4soicit);
        let assign11970_e8965: f64 = (assign11970_e8963 / locals.var_b4soicox);
        (assign11970_e8965, (((locals.var_tmp2_dn3 + ((locals.var_pparam_b4soicdsc_dn3 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn3))) + locals.var_pparam_b4soicit_dn3) / locals.var_b4soicox), (((locals.var_tmp2_dn4 + ((locals.var_pparam_b4soicdsc_dn4 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn4))) + locals.var_pparam_b4soicit_dn4) / locals.var_b4soicox), (((locals.var_tmp2_dn5 + ((locals.var_pparam_b4soicdsc_dn5 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn5))) + locals.var_pparam_b4soicit_dn5) / locals.var_b4soicox), (((locals.var_tmp2_dn6 + ((locals.var_pparam_b4soicdsc_dn6 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn6))) + locals.var_pparam_b4soicit_dn6) / locals.var_b4soicox), (((locals.var_tmp2_dn7 + ((locals.var_pparam_b4soicdsc_dn7 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn7))) + locals.var_pparam_b4soicit_dn7) / locals.var_b4soicox), (((locals.var_tmp2_dn8 + ((locals.var_pparam_b4soicdsc_dn8 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn8))) + locals.var_pparam_b4soicit_dn8) / locals.var_b4soicox), (((locals.var_tmp2_dn9 + ((locals.var_pparam_b4soicdsc_dn9 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn9))) + locals.var_pparam_b4soicit_dn9) / locals.var_b4soicox), (((locals.var_tmp2_dn10 + ((locals.var_pparam_b4soicdsc_dn10 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn10))) + locals.var_pparam_b4soicit_dn10) / locals.var_b4soicox), (((locals.var_tmp2_dn11 + ((locals.var_pparam_b4soicdsc_dn11 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn11))) + locals.var_pparam_b4soicit_dn11) / locals.var_b4soicox), (((locals.var_tmp2_dn12 + ((locals.var_pparam_b4soicdsc_dn12 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn12))) + locals.var_pparam_b4soicit_dn12) / locals.var_b4soicox),)
    } else {
        (locals.var_tmp3, locals.var_tmp3_dn3, locals.var_tmp3_dn4, locals.var_tmp3_dn5, locals.var_tmp3_dn6, locals.var_tmp3_dn7, locals.var_tmp3_dn8, locals.var_tmp3_dn9, locals.var_tmp3_dn10, locals.var_tmp3_dn11, locals.var_tmp3_dn12,)
    }
};
        locals.var_tmp3 = assign11970_e8967;
        locals.var_tmp3_dn3 = assign11970_e8967_d_n3;
        locals.var_tmp3_dn4 = assign11970_e8967_d_n4;
        locals.var_tmp3_dn5 = assign11970_e8967_d_n5;
        locals.var_tmp3_dn6 = assign11970_e8967_d_n6;
        locals.var_tmp3_dn7 = assign11970_e8967_d_n7;
        locals.var_tmp3_dn8 = assign11970_e8967_d_n8;
        locals.var_tmp3_dn9 = assign11970_e8967_d_n9;
        locals.var_tmp3_dn10 = assign11970_e8967_d_n10;
        locals.var_tmp3_dn11 = assign11970_e8967_d_n11;
        locals.var_tmp3_dn12 = assign11970_e8967_d_n12;

        let assign11980_e8970: f64 = (-0.5);
        let assign11980_e8971: f64 = if locals.var_tmp3 >= assign11980_e8970 { 1.0 } else { 0.0 };
        locals.var_guard960 = assign11980_e8971;

        let (assign11990_e8979, assign11990_e8979_d_n3, assign11990_e8979_d_n4, assign11990_e8979_d_n5, assign11990_e8979_d_n6, assign11990_e8979_d_n7, assign11990_e8979_d_n8, assign11990_e8979_d_n9, assign11990_e8979_d_n10, assign11990_e8979_d_n11, assign11990_e8979_d_n12,) = {
    if ((locals.var_guard958 != 0.0) && (locals.var_guard960 != 0.0)) {
        let assign11990_e8977: f64 = (1.0 + locals.var_tmp3);
        (assign11990_e8977, locals.var_tmp3_dn3, locals.var_tmp3_dn4, locals.var_tmp3_dn5, locals.var_tmp3_dn6, locals.var_tmp3_dn7, locals.var_tmp3_dn8, locals.var_tmp3_dn9, locals.var_tmp3_dn10, locals.var_tmp3_dn11, locals.var_tmp3_dn12,)
    } else {
        (locals.var_n0, locals.var_n0_dn3, locals.var_n0_dn4, locals.var_n0_dn5, locals.var_n0_dn6, locals.var_n0_dn7, locals.var_n0_dn8, locals.var_n0_dn9, locals.var_n0_dn10, locals.var_n0_dn11, locals.var_n0_dn12,)
    }
};
        locals.var_n0 = assign11990_e8979;
        locals.var_n0_dn3 = assign11990_e8979_d_n3;
        locals.var_n0_dn4 = assign11990_e8979_d_n4;
        locals.var_n0_dn5 = assign11990_e8979_d_n5;
        locals.var_n0_dn6 = assign11990_e8979_d_n6;
        locals.var_n0_dn7 = assign11990_e8979_d_n7;
        locals.var_n0_dn8 = assign11990_e8979_d_n8;
        locals.var_n0_dn9 = assign11990_e8979_d_n9;
        locals.var_n0_dn10 = assign11990_e8979_d_n10;
        locals.var_n0_dn11 = assign11990_e8979_d_n11;
        locals.var_n0_dn12 = assign11990_e8979_d_n12;

        let (assign12000_e8992, assign12000_e8992_d_n3, assign12000_e8992_d_n4, assign12000_e8992_d_n5, assign12000_e8992_d_n6, assign12000_e8992_d_n7, assign12000_e8992_d_n8, assign12000_e8992_d_n9, assign12000_e8992_d_n10, assign12000_e8992_d_n11, assign12000_e8992_d_n12,) = {
    if ((locals.var_guard958 != 0.0) && (locals.var_guard960 == 0.0)) {
        let assign12000_e8988: f64 = (8.0 * locals.var_tmp3);
        let assign12000_e8989: f64 = (3.0 + assign12000_e8988);
        let assign12000_e8990: f64 = (1.0 / assign12000_e8989);
        (assign12000_e8990, (-((8.0 * locals.var_tmp3_dn3) / (assign12000_e8989 * assign12000_e8989))), (-((8.0 * locals.var_tmp3_dn4) / (assign12000_e8989 * assign12000_e8989))), (-((8.0 * locals.var_tmp3_dn5) / (assign12000_e8989 * assign12000_e8989))), (-((8.0 * locals.var_tmp3_dn6) / (assign12000_e8989 * assign12000_e8989))), (-((8.0 * locals.var_tmp3_dn7) / (assign12000_e8989 * assign12000_e8989))), (-((8.0 * locals.var_tmp3_dn8) / (assign12000_e8989 * assign12000_e8989))), (-((8.0 * locals.var_tmp3_dn9) / (assign12000_e8989 * assign12000_e8989))), (-((8.0 * locals.var_tmp3_dn10) / (assign12000_e8989 * assign12000_e8989))), (-((8.0 * locals.var_tmp3_dn11) / (assign12000_e8989 * assign12000_e8989))), (-((8.0 * locals.var_tmp3_dn12) / (assign12000_e8989 * assign12000_e8989))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign12000_e8992;
        locals.var_t0_dn3 = assign12000_e8992_d_n3;
        locals.var_t0_dn4 = assign12000_e8992_d_n4;
        locals.var_t0_dn5 = assign12000_e8992_d_n5;
        locals.var_t0_dn6 = assign12000_e8992_d_n6;
        locals.var_t0_dn7 = assign12000_e8992_d_n7;
        locals.var_t0_dn8 = assign12000_e8992_d_n8;
        locals.var_t0_dn9 = assign12000_e8992_d_n9;
        locals.var_t0_dn10 = assign12000_e8992_d_n10;
        locals.var_t0_dn11 = assign12000_e8992_d_n11;
        locals.var_t0_dn12 = assign12000_e8992_d_n12;

    }

    pub(super) fn stamp_transient_block_26(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let (assign12010_e9005, assign12010_e9005_d_n3, assign12010_e9005_d_n4, assign12010_e9005_d_n5, assign12010_e9005_d_n6, assign12010_e9005_d_n7, assign12010_e9005_d_n8, assign12010_e9005_d_n9, assign12010_e9005_d_n10, assign12010_e9005_d_n11, assign12010_e9005_d_n12,) = {
    if ((locals.var_guard958 != 0.0) && (locals.var_guard960 == 0.0)) {
        let assign12010_e9000: f64 = (3.0 * locals.var_tmp3);
        let assign12010_e9001: f64 = (1.0 + assign12010_e9000);
        let assign12010_e9003: f64 = (assign12010_e9001 * locals.var_t0);
        (assign12010_e9003, (((3.0 * locals.var_tmp3_dn3) * locals.var_t0) + (assign12010_e9001 * locals.var_t0_dn3)), (((3.0 * locals.var_tmp3_dn4) * locals.var_t0) + (assign12010_e9001 * locals.var_t0_dn4)), (((3.0 * locals.var_tmp3_dn5) * locals.var_t0) + (assign12010_e9001 * locals.var_t0_dn5)), (((3.0 * locals.var_tmp3_dn6) * locals.var_t0) + (assign12010_e9001 * locals.var_t0_dn6)), (((3.0 * locals.var_tmp3_dn7) * locals.var_t0) + (assign12010_e9001 * locals.var_t0_dn7)), (((3.0 * locals.var_tmp3_dn8) * locals.var_t0) + (assign12010_e9001 * locals.var_t0_dn8)), (((3.0 * locals.var_tmp3_dn9) * locals.var_t0) + (assign12010_e9001 * locals.var_t0_dn9)), (((3.0 * locals.var_tmp3_dn10) * locals.var_t0) + (assign12010_e9001 * locals.var_t0_dn10)), (((3.0 * locals.var_tmp3_dn11) * locals.var_t0) + (assign12010_e9001 * locals.var_t0_dn11)), (((3.0 * locals.var_tmp3_dn12) * locals.var_t0) + (assign12010_e9001 * locals.var_t0_dn12)),)
    } else {
        (locals.var_n0, locals.var_n0_dn3, locals.var_n0_dn4, locals.var_n0_dn5, locals.var_n0_dn6, locals.var_n0_dn7, locals.var_n0_dn8, locals.var_n0_dn9, locals.var_n0_dn10, locals.var_n0_dn11, locals.var_n0_dn12,)
    }
};
        locals.var_n0 = assign12010_e9005;
        locals.var_n0_dn3 = assign12010_e9005_d_n3;
        locals.var_n0_dn4 = assign12010_e9005_d_n4;
        locals.var_n0_dn5 = assign12010_e9005_d_n5;
        locals.var_n0_dn6 = assign12010_e9005_d_n6;
        locals.var_n0_dn7 = assign12010_e9005_d_n7;
        locals.var_n0_dn8 = assign12010_e9005_d_n8;
        locals.var_n0_dn9 = assign12010_e9005_d_n9;
        locals.var_n0_dn10 = assign12010_e9005_d_n10;
        locals.var_n0_dn11 = assign12010_e9005_d_n11;
        locals.var_n0_dn12 = assign12010_e9005_d_n12;

        let (assign12020_e9011, assign12020_e9011_d_n3, assign12020_e9011_d_n4, assign12020_e9011_d_n5, assign12020_e9011_d_n6, assign12020_e9011_d_n7, assign12020_e9011_d_n8, assign12020_e9011_d_n9, assign12020_e9011_d_n10, assign12020_e9011_d_n11, assign12020_e9011_d_n12,) = {
    if (locals.var_guard958 != 0.0) {
        let assign12020_e9009: f64 = (locals.var_n0 * locals.var_vtm0);
        (assign12020_e9009, (locals.var_n0_dn3 * locals.var_vtm0), (locals.var_n0_dn4 * locals.var_vtm0), (locals.var_n0_dn5 * locals.var_vtm0), (locals.var_n0_dn6 * locals.var_vtm0), (locals.var_n0_dn7 * locals.var_vtm0), (locals.var_n0_dn8 * locals.var_vtm0), (locals.var_n0_dn9 * locals.var_vtm0), (locals.var_n0_dn10 * locals.var_vtm0), (locals.var_n0_dn11 * locals.var_vtm0), (locals.var_n0_dn12 * locals.var_vtm0),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign12020_e9011;
        locals.var_t0_dn3 = assign12020_e9011_d_n3;
        locals.var_t0_dn4 = assign12020_e9011_d_n4;
        locals.var_t0_dn5 = assign12020_e9011_d_n5;
        locals.var_t0_dn6 = assign12020_e9011_d_n6;
        locals.var_t0_dn7 = assign12020_e9011_d_n7;
        locals.var_t0_dn8 = assign12020_e9011_d_n8;
        locals.var_t0_dn9 = assign12020_e9011_d_n9;
        locals.var_t0_dn10 = assign12020_e9011_d_n10;
        locals.var_t0_dn11 = assign12020_e9011_d_n11;
        locals.var_t0_dn12 = assign12020_e9011_d_n12;

        let (assign12030_e9015, assign12030_e9015_d_n3, assign12030_e9015_d_n4, assign12030_e9015_d_n5, assign12030_e9015_d_n6, assign12030_e9015_d_n7, assign12030_e9015_d_n8, assign12030_e9015_d_n9, assign12030_e9015_d_n10, assign12030_e9015_d_n11, assign12030_e9015_d_n12,) = {
    if (locals.var_guard958 != 0.0) {
        (locals.var_pparam_b4soivoff, locals.var_pparam_b4soivoff_dn3, locals.var_pparam_b4soivoff_dn4, locals.var_pparam_b4soivoff_dn5, locals.var_pparam_b4soivoff_dn6, locals.var_pparam_b4soivoff_dn7, locals.var_pparam_b4soivoff_dn8, locals.var_pparam_b4soivoff_dn9, locals.var_pparam_b4soivoff_dn10, locals.var_pparam_b4soivoff_dn11, locals.var_pparam_b4soivoff_dn12,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign12030_e9015;
        locals.var_t1_dn3 = assign12030_e9015_d_n3;
        locals.var_t1_dn4 = assign12030_e9015_d_n4;
        locals.var_t1_dn5 = assign12030_e9015_d_n5;
        locals.var_t1_dn6 = assign12030_e9015_d_n6;
        locals.var_t1_dn7 = assign12030_e9015_d_n7;
        locals.var_t1_dn8 = assign12030_e9015_d_n8;
        locals.var_t1_dn9 = assign12030_e9015_d_n9;
        locals.var_t1_dn10 = assign12030_e9015_d_n10;
        locals.var_t1_dn11 = assign12030_e9015_d_n11;
        locals.var_t1_dn12 = assign12030_e9015_d_n12;

        let (assign12040_e9021, assign12040_e9021_d_n3, assign12040_e9021_d_n4, assign12040_e9021_d_n5, assign12040_e9021_d_n6, assign12040_e9021_d_n7, assign12040_e9021_d_n8, assign12040_e9021_d_n9, assign12040_e9021_d_n10, assign12040_e9021_d_n11, assign12040_e9021_d_n12,) = {
    if (locals.var_guard958 != 0.0) {
        let assign12040_e9019: f64 = (locals.var_t1 / locals.var_t0);
        (assign12040_e9019, (((locals.var_t1_dn3 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t1_dn4 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t1_dn5 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t1_dn6 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t1_dn7 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t1_dn8 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t1_dn9 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t1_dn10 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t1_dn11 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t1_dn12 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign12040_e9021;
        locals.var_t2_dn3 = assign12040_e9021_d_n3;
        locals.var_t2_dn4 = assign12040_e9021_d_n4;
        locals.var_t2_dn5 = assign12040_e9021_d_n5;
        locals.var_t2_dn6 = assign12040_e9021_d_n6;
        locals.var_t2_dn7 = assign12040_e9021_d_n7;
        locals.var_t2_dn8 = assign12040_e9021_d_n8;
        locals.var_t2_dn9 = assign12040_e9021_d_n9;
        locals.var_t2_dn10 = assign12040_e9021_d_n10;
        locals.var_t2_dn11 = assign12040_e9021_d_n11;
        locals.var_t2_dn12 = assign12040_e9021_d_n12;

        let assign12050_e9024: f64 = (-100.0);
        let assign12050_e9025: f64 = if locals.var_t2 < assign12050_e9024 { 1.0 } else { 0.0 };
        locals.var_guard961 = assign12050_e9025;

        let (assign12060_e9035, assign12060_e9035_d_n3, assign12060_e9035_d_n4, assign12060_e9035_d_n5, assign12060_e9035_d_n6, assign12060_e9035_d_n7, assign12060_e9035_d_n8, assign12060_e9035_d_n9, assign12060_e9035_d_n10, assign12060_e9035_d_n11, assign12060_e9035_d_n12,) = {
    if ((locals.var_guard958 != 0.0) && (locals.var_guard961 != 0.0)) {
        let assign12060_e9031: f64 = (locals.var_b4soicox * 3.720075976e-44);
        let assign12060_e9033: f64 = (assign12060_e9031 / locals.var_pparam_b4soicdep0);
        (assign12060_e9033, (-((assign12060_e9031 * locals.var_pparam_b4soicdep0_dn3) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign12060_e9031 * locals.var_pparam_b4soicdep0_dn4) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign12060_e9031 * locals.var_pparam_b4soicdep0_dn5) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign12060_e9031 * locals.var_pparam_b4soicdep0_dn6) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign12060_e9031 * locals.var_pparam_b4soicdep0_dn7) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign12060_e9031 * locals.var_pparam_b4soicdep0_dn8) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign12060_e9031 * locals.var_pparam_b4soicdep0_dn9) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign12060_e9031 * locals.var_pparam_b4soicdep0_dn10) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign12060_e9031 * locals.var_pparam_b4soicdep0_dn11) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign12060_e9031 * locals.var_pparam_b4soicdep0_dn12) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign12060_e9035;
        locals.var_t3_dn3 = assign12060_e9035_d_n3;
        locals.var_t3_dn4 = assign12060_e9035_d_n4;
        locals.var_t3_dn5 = assign12060_e9035_d_n5;
        locals.var_t3_dn6 = assign12060_e9035_d_n6;
        locals.var_t3_dn7 = assign12060_e9035_d_n7;
        locals.var_t3_dn8 = assign12060_e9035_d_n8;
        locals.var_t3_dn9 = assign12060_e9035_d_n9;
        locals.var_t3_dn10 = assign12060_e9035_d_n10;
        locals.var_t3_dn11 = assign12060_e9035_d_n11;
        locals.var_t3_dn12 = assign12060_e9035_d_n12;

        let (assign12070_e9045, assign12070_e9045_d_n3, assign12070_e9045_d_n4, assign12070_e9045_d_n5, assign12070_e9045_d_n6, assign12070_e9045_d_n7, assign12070_e9045_d_n8, assign12070_e9045_d_n9, assign12070_e9045_d_n10, assign12070_e9045_d_n11, assign12070_e9045_d_n12,) = {
    if ((locals.var_guard958 != 0.0) && (locals.var_guard961 != 0.0)) {
        let assign12070_e9042: f64 = (locals.var_t3 * locals.var_n0);
        let assign12070_e9043: f64 = (locals.var_pparam_b4soimstar + assign12070_e9042);
        (assign12070_e9043, (locals.var_pparam_b4soimstar_dn3 + ((locals.var_t3_dn3 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn3))), (locals.var_pparam_b4soimstar_dn4 + ((locals.var_t3_dn4 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn4))), (locals.var_pparam_b4soimstar_dn5 + ((locals.var_t3_dn5 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn5))), (locals.var_pparam_b4soimstar_dn6 + ((locals.var_t3_dn6 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn6))), (locals.var_pparam_b4soimstar_dn7 + ((locals.var_t3_dn7 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn7))), (locals.var_pparam_b4soimstar_dn8 + ((locals.var_t3_dn8 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn8))), (locals.var_pparam_b4soimstar_dn9 + ((locals.var_t3_dn9 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn9))), (locals.var_pparam_b4soimstar_dn10 + ((locals.var_t3_dn10 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn10))), (locals.var_pparam_b4soimstar_dn11 + ((locals.var_t3_dn11 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn11))), (locals.var_pparam_b4soimstar_dn12 + ((locals.var_t3_dn12 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn12))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign12070_e9045;
        locals.var_t4_dn3 = assign12070_e9045_d_n3;
        locals.var_t4_dn4 = assign12070_e9045_d_n4;
        locals.var_t4_dn5 = assign12070_e9045_d_n5;
        locals.var_t4_dn6 = assign12070_e9045_d_n6;
        locals.var_t4_dn7 = assign12070_e9045_d_n7;
        locals.var_t4_dn8 = assign12070_e9045_d_n8;
        locals.var_t4_dn9 = assign12070_e9045_d_n9;
        locals.var_t4_dn10 = assign12070_e9045_d_n10;
        locals.var_t4_dn11 = assign12070_e9045_d_n11;
        locals.var_t4_dn12 = assign12070_e9045_d_n12;

        let assign12080_e9048: f64 = if locals.var_t2 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard962 = assign12080_e9048;

        let (assign12090_e9061, assign12090_e9061_d_n3, assign12090_e9061_d_n4, assign12090_e9061_d_n5, assign12090_e9061_d_n6, assign12090_e9061_d_n7, assign12090_e9061_d_n8, assign12090_e9061_d_n9, assign12090_e9061_d_n10, assign12090_e9061_d_n11, assign12090_e9061_d_n12,) = {
    if (((locals.var_guard958 != 0.0) && (locals.var_guard961 == 0.0)) && (locals.var_guard962 != 0.0)) {
        let assign12090_e9057: f64 = (locals.var_b4soicox * 2.688117142e43);
        let assign12090_e9059: f64 = (assign12090_e9057 / locals.var_pparam_b4soicdep0);
        (assign12090_e9059, (-((assign12090_e9057 * locals.var_pparam_b4soicdep0_dn3) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign12090_e9057 * locals.var_pparam_b4soicdep0_dn4) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign12090_e9057 * locals.var_pparam_b4soicdep0_dn5) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign12090_e9057 * locals.var_pparam_b4soicdep0_dn6) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign12090_e9057 * locals.var_pparam_b4soicdep0_dn7) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign12090_e9057 * locals.var_pparam_b4soicdep0_dn8) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign12090_e9057 * locals.var_pparam_b4soicdep0_dn9) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign12090_e9057 * locals.var_pparam_b4soicdep0_dn10) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign12090_e9057 * locals.var_pparam_b4soicdep0_dn11) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign12090_e9057 * locals.var_pparam_b4soicdep0_dn12) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign12090_e9061;
        locals.var_t3_dn3 = assign12090_e9061_d_n3;
        locals.var_t3_dn4 = assign12090_e9061_d_n4;
        locals.var_t3_dn5 = assign12090_e9061_d_n5;
        locals.var_t3_dn6 = assign12090_e9061_d_n6;
        locals.var_t3_dn7 = assign12090_e9061_d_n7;
        locals.var_t3_dn8 = assign12090_e9061_d_n8;
        locals.var_t3_dn9 = assign12090_e9061_d_n9;
        locals.var_t3_dn10 = assign12090_e9061_d_n10;
        locals.var_t3_dn11 = assign12090_e9061_d_n11;
        locals.var_t3_dn12 = assign12090_e9061_d_n12;

        let (assign12100_e9074, assign12100_e9074_d_n3, assign12100_e9074_d_n4, assign12100_e9074_d_n5, assign12100_e9074_d_n6, assign12100_e9074_d_n7, assign12100_e9074_d_n8, assign12100_e9074_d_n9, assign12100_e9074_d_n10, assign12100_e9074_d_n11, assign12100_e9074_d_n12,) = {
    if (((locals.var_guard958 != 0.0) && (locals.var_guard961 == 0.0)) && (locals.var_guard962 != 0.0)) {
        let assign12100_e9071: f64 = (locals.var_t3 * locals.var_n0);
        let assign12100_e9072: f64 = (locals.var_pparam_b4soimstar + assign12100_e9071);
        (assign12100_e9072, (locals.var_pparam_b4soimstar_dn3 + ((locals.var_t3_dn3 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn3))), (locals.var_pparam_b4soimstar_dn4 + ((locals.var_t3_dn4 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn4))), (locals.var_pparam_b4soimstar_dn5 + ((locals.var_t3_dn5 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn5))), (locals.var_pparam_b4soimstar_dn6 + ((locals.var_t3_dn6 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn6))), (locals.var_pparam_b4soimstar_dn7 + ((locals.var_t3_dn7 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn7))), (locals.var_pparam_b4soimstar_dn8 + ((locals.var_t3_dn8 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn8))), (locals.var_pparam_b4soimstar_dn9 + ((locals.var_t3_dn9 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn9))), (locals.var_pparam_b4soimstar_dn10 + ((locals.var_t3_dn10 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn10))), (locals.var_pparam_b4soimstar_dn11 + ((locals.var_t3_dn11 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn11))), (locals.var_pparam_b4soimstar_dn12 + ((locals.var_t3_dn12 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn12))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign12100_e9074;
        locals.var_t4_dn3 = assign12100_e9074_d_n3;
        locals.var_t4_dn4 = assign12100_e9074_d_n4;
        locals.var_t4_dn5 = assign12100_e9074_d_n5;
        locals.var_t4_dn6 = assign12100_e9074_d_n6;
        locals.var_t4_dn7 = assign12100_e9074_d_n7;
        locals.var_t4_dn8 = assign12100_e9074_d_n8;
        locals.var_t4_dn9 = assign12100_e9074_d_n9;
        locals.var_t4_dn10 = assign12100_e9074_d_n10;
        locals.var_t4_dn11 = assign12100_e9074_d_n11;
        locals.var_t4_dn12 = assign12100_e9074_d_n12;

        let (assign12110_e9089, assign12110_e9089_d_n3, assign12110_e9089_d_n4, assign12110_e9089_d_n5, assign12110_e9089_d_n6, assign12110_e9089_d_n7, assign12110_e9089_d_n8, assign12110_e9089_d_n9, assign12110_e9089_d_n10, assign12110_e9089_d_n11, assign12110_e9089_d_n12,) = {
    if (((locals.var_guard958 != 0.0) && (locals.var_guard961 == 0.0)) && (locals.var_guard962 == 0.0)) {
        let assign12110_e9083: f64 = (locals.var_t2).exp();
        let assign12110_e9085: f64 = (assign12110_e9083 * locals.var_b4soicox);
        let assign12110_e9087: f64 = (assign12110_e9085 / locals.var_pparam_b4soicdep0);
        (assign12110_e9087, (((((assign12110_e9083 * locals.var_t2_dn3) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign12110_e9085 * locals.var_pparam_b4soicdep0_dn3)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)), (((((assign12110_e9083 * locals.var_t2_dn4) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign12110_e9085 * locals.var_pparam_b4soicdep0_dn4)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)), (((((assign12110_e9083 * locals.var_t2_dn5) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign12110_e9085 * locals.var_pparam_b4soicdep0_dn5)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)), (((((assign12110_e9083 * locals.var_t2_dn6) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign12110_e9085 * locals.var_pparam_b4soicdep0_dn6)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)), (((((assign12110_e9083 * locals.var_t2_dn7) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign12110_e9085 * locals.var_pparam_b4soicdep0_dn7)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)), (((((assign12110_e9083 * locals.var_t2_dn8) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign12110_e9085 * locals.var_pparam_b4soicdep0_dn8)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)), (((((assign12110_e9083 * locals.var_t2_dn9) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign12110_e9085 * locals.var_pparam_b4soicdep0_dn9)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)), (((((assign12110_e9083 * locals.var_t2_dn10) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign12110_e9085 * locals.var_pparam_b4soicdep0_dn10)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)), (((((assign12110_e9083 * locals.var_t2_dn11) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign12110_e9085 * locals.var_pparam_b4soicdep0_dn11)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)), (((((assign12110_e9083 * locals.var_t2_dn12) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign12110_e9085 * locals.var_pparam_b4soicdep0_dn12)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign12110_e9089;
        locals.var_t3_dn3 = assign12110_e9089_d_n3;
        locals.var_t3_dn4 = assign12110_e9089_d_n4;
        locals.var_t3_dn5 = assign12110_e9089_d_n5;
        locals.var_t3_dn6 = assign12110_e9089_d_n6;
        locals.var_t3_dn7 = assign12110_e9089_d_n7;
        locals.var_t3_dn8 = assign12110_e9089_d_n8;
        locals.var_t3_dn9 = assign12110_e9089_d_n9;
        locals.var_t3_dn10 = assign12110_e9089_d_n10;
        locals.var_t3_dn11 = assign12110_e9089_d_n11;
        locals.var_t3_dn12 = assign12110_e9089_d_n12;

        let (assign12120_e9103, assign12120_e9103_d_n3, assign12120_e9103_d_n4, assign12120_e9103_d_n5, assign12120_e9103_d_n6, assign12120_e9103_d_n7, assign12120_e9103_d_n8, assign12120_e9103_d_n9, assign12120_e9103_d_n10, assign12120_e9103_d_n11, assign12120_e9103_d_n12,) = {
    if (((locals.var_guard958 != 0.0) && (locals.var_guard961 == 0.0)) && (locals.var_guard962 == 0.0)) {
        let assign12120_e9100: f64 = (locals.var_t3 * locals.var_n0);
        let assign12120_e9101: f64 = (locals.var_pparam_b4soimstar + assign12120_e9100);
        (assign12120_e9101, (locals.var_pparam_b4soimstar_dn3 + ((locals.var_t3_dn3 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn3))), (locals.var_pparam_b4soimstar_dn4 + ((locals.var_t3_dn4 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn4))), (locals.var_pparam_b4soimstar_dn5 + ((locals.var_t3_dn5 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn5))), (locals.var_pparam_b4soimstar_dn6 + ((locals.var_t3_dn6 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn6))), (locals.var_pparam_b4soimstar_dn7 + ((locals.var_t3_dn7 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn7))), (locals.var_pparam_b4soimstar_dn8 + ((locals.var_t3_dn8 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn8))), (locals.var_pparam_b4soimstar_dn9 + ((locals.var_t3_dn9 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn9))), (locals.var_pparam_b4soimstar_dn10 + ((locals.var_t3_dn10 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn10))), (locals.var_pparam_b4soimstar_dn11 + ((locals.var_t3_dn11 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn11))), (locals.var_pparam_b4soimstar_dn12 + ((locals.var_t3_dn12 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn12))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign12120_e9103;
        locals.var_t4_dn3 = assign12120_e9103_d_n3;
        locals.var_t4_dn4 = assign12120_e9103_d_n4;
        locals.var_t4_dn5 = assign12120_e9103_d_n5;
        locals.var_t4_dn6 = assign12120_e9103_d_n6;
        locals.var_t4_dn7 = assign12120_e9103_d_n7;
        locals.var_t4_dn8 = assign12120_e9103_d_n8;
        locals.var_t4_dn9 = assign12120_e9103_d_n9;
        locals.var_t4_dn10 = assign12120_e9103_d_n10;
        locals.var_t4_dn11 = assign12120_e9103_d_n11;
        locals.var_t4_dn12 = assign12120_e9103_d_n12;

        let (assign12130_e9111, assign12130_e9111_d_n3, assign12130_e9111_d_n4, assign12130_e9111_d_n5, assign12130_e9111_d_n6, assign12130_e9111_d_n7, assign12130_e9111_d_n8, assign12130_e9111_d_n9, assign12130_e9111_d_n10, assign12130_e9111_d_n11, assign12130_e9111_d_n12,) = {
    if (locals.var_guard958 != 0.0) {
        let assign12130_e9107: f64 = (locals.var_t0 * 0.6931471805599453);
        let assign12130_e9109: f64 = (assign12130_e9107 / locals.var_t4);
        (assign12130_e9109, ((((locals.var_t0_dn3 * 0.6931471805599453) * locals.var_t4) - (assign12130_e9107 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)), ((((locals.var_t0_dn4 * 0.6931471805599453) * locals.var_t4) - (assign12130_e9107 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), ((((locals.var_t0_dn5 * 0.6931471805599453) * locals.var_t4) - (assign12130_e9107 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), ((((locals.var_t0_dn6 * 0.6931471805599453) * locals.var_t4) - (assign12130_e9107 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), ((((locals.var_t0_dn7 * 0.6931471805599453) * locals.var_t4) - (assign12130_e9107 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), ((((locals.var_t0_dn8 * 0.6931471805599453) * locals.var_t4) - (assign12130_e9107 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), ((((locals.var_t0_dn9 * 0.6931471805599453) * locals.var_t4) - (assign12130_e9107 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), ((((locals.var_t0_dn10 * 0.6931471805599453) * locals.var_t4) - (assign12130_e9107 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), ((((locals.var_t0_dn11 * 0.6931471805599453) * locals.var_t4) - (assign12130_e9107 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), ((((locals.var_t0_dn12 * 0.6931471805599453) * locals.var_t4) - (assign12130_e9107 * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_b4soivgsteffvth, locals.var_b4soivgsteffvth_dn3, locals.var_b4soivgsteffvth_dn4, locals.var_b4soivgsteffvth_dn5, locals.var_b4soivgsteffvth_dn6, locals.var_b4soivgsteffvth_dn7, locals.var_b4soivgsteffvth_dn8, locals.var_b4soivgsteffvth_dn9, locals.var_b4soivgsteffvth_dn10, locals.var_b4soivgsteffvth_dn11, locals.var_b4soivgsteffvth_dn12,)
    }
};
        locals.var_b4soivgsteffvth = assign12130_e9111;
        locals.var_b4soivgsteffvth_dn3 = assign12130_e9111_d_n3;
        locals.var_b4soivgsteffvth_dn4 = assign12130_e9111_d_n4;
        locals.var_b4soivgsteffvth_dn5 = assign12130_e9111_d_n5;
        locals.var_b4soivgsteffvth_dn6 = assign12130_e9111_d_n6;
        locals.var_b4soivgsteffvth_dn7 = assign12130_e9111_d_n7;
        locals.var_b4soivgsteffvth_dn8 = assign12130_e9111_d_n8;
        locals.var_b4soivgsteffvth_dn9 = assign12130_e9111_d_n9;
        locals.var_b4soivgsteffvth_dn10 = assign12130_e9111_d_n10;
        locals.var_b4soivgsteffvth_dn11 = assign12130_e9111_d_n11;
        locals.var_b4soivgsteffvth_dn12 = assign12130_e9111_d_n12;

        let (assign12140_e9116, assign12140_e9116_d_n3, assign12140_e9116_d_n4, assign12140_e9116_d_n5, assign12140_e9116_d_n6, assign12140_e9116_d_n7, assign12140_e9116_d_n8, assign12140_e9116_d_n9, assign12140_e9116_d_n10, assign12140_e9116_d_n11, assign12140_e9116_d_n12,) = {
    if (locals.var_guard958 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soivgsteffvth, locals.var_b4soivgsteffvth_dn3, locals.var_b4soivgsteffvth_dn4, locals.var_b4soivgsteffvth_dn5, locals.var_b4soivgsteffvth_dn6, locals.var_b4soivgsteffvth_dn7, locals.var_b4soivgsteffvth_dn8, locals.var_b4soivgsteffvth_dn9, locals.var_b4soivgsteffvth_dn10, locals.var_b4soivgsteffvth_dn11, locals.var_b4soivgsteffvth_dn12,)
    }
};
        locals.var_b4soivgsteffvth = assign12140_e9116;
        locals.var_b4soivgsteffvth_dn3 = assign12140_e9116_d_n3;
        locals.var_b4soivgsteffvth_dn4 = assign12140_e9116_d_n4;
        locals.var_b4soivgsteffvth_dn5 = assign12140_e9116_d_n5;
        locals.var_b4soivgsteffvth_dn6 = assign12140_e9116_d_n6;
        locals.var_b4soivgsteffvth_dn7 = assign12140_e9116_d_n7;
        locals.var_b4soivgsteffvth_dn8 = assign12140_e9116_d_n8;
        locals.var_b4soivgsteffvth_dn9 = assign12140_e9116_d_n9;
        locals.var_b4soivgsteffvth_dn10 = assign12140_e9116_d_n10;
        locals.var_b4soivgsteffvth_dn11 = assign12140_e9116_d_n11;
        locals.var_b4soivgsteffvth_dn12 = assign12140_e9116_d_n12;

        let assign13030_e9469: f64 = if ((p.p35 >= 4.4) || (p.p61 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1015 = assign13030_e9469;

        let assign13040_e9472: f64 = if locals.var_pparam_b4soia2 < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard1016 = assign13040_e9472;

        let (assign13050_e9478, assign13050_e9478_d_n3, assign13050_e9478_d_n4, assign13050_e9478_d_n5, assign13050_e9478_d_n6, assign13050_e9478_d_n7, assign13050_e9478_d_n8, assign13050_e9478_d_n9, assign13050_e9478_d_n10, assign13050_e9478_d_n11, assign13050_e9478_d_n12,) = {
    if ((locals.var_guard1015 != 0.0) && (locals.var_guard1016 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soia2, locals.var_pparam_b4soia2_dn3, locals.var_pparam_b4soia2_dn4, locals.var_pparam_b4soia2_dn5, locals.var_pparam_b4soia2_dn6, locals.var_pparam_b4soia2_dn7, locals.var_pparam_b4soia2_dn8, locals.var_pparam_b4soia2_dn9, locals.var_pparam_b4soia2_dn10, locals.var_pparam_b4soia2_dn11, locals.var_pparam_b4soia2_dn12,)
    }
};
        locals.var_pparam_b4soia2 = assign13050_e9478;
        locals.var_pparam_b4soia2_dn3 = assign13050_e9478_d_n3;
        locals.var_pparam_b4soia2_dn4 = assign13050_e9478_d_n4;
        locals.var_pparam_b4soia2_dn5 = assign13050_e9478_d_n5;
        locals.var_pparam_b4soia2_dn6 = assign13050_e9478_d_n6;
        locals.var_pparam_b4soia2_dn7 = assign13050_e9478_d_n7;
        locals.var_pparam_b4soia2_dn8 = assign13050_e9478_d_n8;
        locals.var_pparam_b4soia2_dn9 = assign13050_e9478_d_n9;
        locals.var_pparam_b4soia2_dn10 = assign13050_e9478_d_n10;
        locals.var_pparam_b4soia2_dn11 = assign13050_e9478_d_n11;
        locals.var_pparam_b4soia2_dn12 = assign13050_e9478_d_n12;

        let assign13060_e9481: f64 = if locals.var_pparam_b4soia2 > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1017 = assign13060_e9481;

        let (assign13070_e9490, assign13070_e9490_d_n3, assign13070_e9490_d_n4, assign13070_e9490_d_n5, assign13070_e9490_d_n6, assign13070_e9490_d_n7, assign13070_e9490_d_n8, assign13070_e9490_d_n9, assign13070_e9490_d_n10, assign13070_e9490_d_n11, assign13070_e9490_d_n12,) = {
    if (((locals.var_guard1015 != 0.0) && (locals.var_guard1016 == 0.0)) && (locals.var_guard1017 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soia2, locals.var_pparam_b4soia2_dn3, locals.var_pparam_b4soia2_dn4, locals.var_pparam_b4soia2_dn5, locals.var_pparam_b4soia2_dn6, locals.var_pparam_b4soia2_dn7, locals.var_pparam_b4soia2_dn8, locals.var_pparam_b4soia2_dn9, locals.var_pparam_b4soia2_dn10, locals.var_pparam_b4soia2_dn11, locals.var_pparam_b4soia2_dn12,)
    }
};
        locals.var_pparam_b4soia2 = assign13070_e9490;
        locals.var_pparam_b4soia2_dn3 = assign13070_e9490_d_n3;
        locals.var_pparam_b4soia2_dn4 = assign13070_e9490_d_n4;
        locals.var_pparam_b4soia2_dn5 = assign13070_e9490_d_n5;
        locals.var_pparam_b4soia2_dn6 = assign13070_e9490_d_n6;
        locals.var_pparam_b4soia2_dn7 = assign13070_e9490_d_n7;
        locals.var_pparam_b4soia2_dn8 = assign13070_e9490_d_n8;
        locals.var_pparam_b4soia2_dn9 = assign13070_e9490_d_n9;
        locals.var_pparam_b4soia2_dn10 = assign13070_e9490_d_n10;
        locals.var_pparam_b4soia2_dn11 = assign13070_e9490_d_n11;
        locals.var_pparam_b4soia2_dn12 = assign13070_e9490_d_n12;

        let (assign13080_e9499, assign13080_e9499_d_n3, assign13080_e9499_d_n4, assign13080_e9499_d_n5, assign13080_e9499_d_n6, assign13080_e9499_d_n7, assign13080_e9499_d_n8, assign13080_e9499_d_n9, assign13080_e9499_d_n10, assign13080_e9499_d_n11, assign13080_e9499_d_n12,) = {
    if (((locals.var_guard1015 != 0.0) && (locals.var_guard1016 == 0.0)) && (locals.var_guard1017 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soia1, locals.var_pparam_b4soia1_dn3, locals.var_pparam_b4soia1_dn4, locals.var_pparam_b4soia1_dn5, locals.var_pparam_b4soia1_dn6, locals.var_pparam_b4soia1_dn7, locals.var_pparam_b4soia1_dn8, locals.var_pparam_b4soia1_dn9, locals.var_pparam_b4soia1_dn10, locals.var_pparam_b4soia1_dn11, locals.var_pparam_b4soia1_dn12,)
    }
};
        locals.var_pparam_b4soia1 = assign13080_e9499;
        locals.var_pparam_b4soia1_dn3 = assign13080_e9499_d_n3;
        locals.var_pparam_b4soia1_dn4 = assign13080_e9499_d_n4;
        locals.var_pparam_b4soia1_dn5 = assign13080_e9499_d_n5;
        locals.var_pparam_b4soia1_dn6 = assign13080_e9499_d_n6;
        locals.var_pparam_b4soia1_dn7 = assign13080_e9499_d_n7;
        locals.var_pparam_b4soia1_dn8 = assign13080_e9499_d_n8;
        locals.var_pparam_b4soia1_dn9 = assign13080_e9499_d_n9;
        locals.var_pparam_b4soia1_dn10 = assign13080_e9499_d_n10;
        locals.var_pparam_b4soia1_dn11 = assign13080_e9499_d_n11;
        locals.var_pparam_b4soia1_dn12 = assign13080_e9499_d_n12;

        let assign13090_e9502: f64 = if locals.var_pparam_b4soirdsw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1018 = assign13090_e9502;

        let (assign13100_e9506, assign13100_e9506_d_n3, assign13100_e9506_d_n4, assign13100_e9506_d_n5, assign13100_e9506_d_n6, assign13100_e9506_d_n7, assign13100_e9506_d_n8, assign13100_e9506_d_n9, assign13100_e9506_d_n10, assign13100_e9506_d_n11, assign13100_e9506_d_n12,) = {
    if (locals.var_guard1018 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soirdsw, locals.var_pparam_b4soirdsw_dn3, locals.var_pparam_b4soirdsw_dn4, locals.var_pparam_b4soirdsw_dn5, locals.var_pparam_b4soirdsw_dn6, locals.var_pparam_b4soirdsw_dn7, locals.var_pparam_b4soirdsw_dn8, locals.var_pparam_b4soirdsw_dn9, locals.var_pparam_b4soirdsw_dn10, locals.var_pparam_b4soirdsw_dn11, locals.var_pparam_b4soirdsw_dn12,)
    }
};
        locals.var_pparam_b4soirdsw = assign13100_e9506;
        locals.var_pparam_b4soirdsw_dn3 = assign13100_e9506_d_n3;
        locals.var_pparam_b4soirdsw_dn4 = assign13100_e9506_d_n4;
        locals.var_pparam_b4soirdsw_dn5 = assign13100_e9506_d_n5;
        locals.var_pparam_b4soirdsw_dn6 = assign13100_e9506_d_n6;
        locals.var_pparam_b4soirdsw_dn7 = assign13100_e9506_d_n7;
        locals.var_pparam_b4soirdsw_dn8 = assign13100_e9506_d_n8;
        locals.var_pparam_b4soirdsw_dn9 = assign13100_e9506_d_n9;
        locals.var_pparam_b4soirdsw_dn10 = assign13100_e9506_d_n10;
        locals.var_pparam_b4soirdsw_dn11 = assign13100_e9506_d_n11;
        locals.var_pparam_b4soirdsw_dn12 = assign13100_e9506_d_n12;

        let (assign13110_e9510, assign13110_e9510_d_n3, assign13110_e9510_d_n4, assign13110_e9510_d_n5, assign13110_e9510_d_n6, assign13110_e9510_d_n7, assign13110_e9510_d_n8, assign13110_e9510_d_n9, assign13110_e9510_d_n10, assign13110_e9510_d_n11, assign13110_e9510_d_n12,) = {
    if (locals.var_guard1018 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soirds0, locals.var_pparam_b4soirds0_dn3, locals.var_pparam_b4soirds0_dn4, locals.var_pparam_b4soirds0_dn5, locals.var_pparam_b4soirds0_dn6, locals.var_pparam_b4soirds0_dn7, locals.var_pparam_b4soirds0_dn8, locals.var_pparam_b4soirds0_dn9, locals.var_pparam_b4soirds0_dn10, locals.var_pparam_b4soirds0_dn11, locals.var_pparam_b4soirds0_dn12,)
    }
};
        locals.var_pparam_b4soirds0 = assign13110_e9510;
        locals.var_pparam_b4soirds0_dn3 = assign13110_e9510_d_n3;
        locals.var_pparam_b4soirds0_dn4 = assign13110_e9510_d_n4;
        locals.var_pparam_b4soirds0_dn5 = assign13110_e9510_d_n5;
        locals.var_pparam_b4soirds0_dn6 = assign13110_e9510_d_n6;
        locals.var_pparam_b4soirds0_dn7 = assign13110_e9510_d_n7;
        locals.var_pparam_b4soirds0_dn8 = assign13110_e9510_d_n8;
        locals.var_pparam_b4soirds0_dn9 = assign13110_e9510_d_n9;
        locals.var_pparam_b4soirds0_dn10 = assign13110_e9510_d_n10;
        locals.var_pparam_b4soirds0_dn11 = assign13110_e9510_d_n11;
        locals.var_pparam_b4soirds0_dn12 = assign13110_e9510_d_n12;

        let assign13120_e9517: f64 = if ((locals.var_pparam_b4soirds0 < 0.001) && (locals.var_pparam_b4soirds0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1019 = assign13120_e9517;

        let (assign13130_e9524, assign13130_e9524_d_n3, assign13130_e9524_d_n4, assign13130_e9524_d_n5, assign13130_e9524_d_n6, assign13130_e9524_d_n7, assign13130_e9524_d_n8, assign13130_e9524_d_n9, assign13130_e9524_d_n10, assign13130_e9524_d_n11, assign13130_e9524_d_n12,) = {
    if ((locals.var_guard1018 == 0.0) && (locals.var_guard1019 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soirds0, locals.var_pparam_b4soirds0_dn3, locals.var_pparam_b4soirds0_dn4, locals.var_pparam_b4soirds0_dn5, locals.var_pparam_b4soirds0_dn6, locals.var_pparam_b4soirds0_dn7, locals.var_pparam_b4soirds0_dn8, locals.var_pparam_b4soirds0_dn9, locals.var_pparam_b4soirds0_dn10, locals.var_pparam_b4soirds0_dn11, locals.var_pparam_b4soirds0_dn12,)
    }
};
        locals.var_pparam_b4soirds0 = assign13130_e9524;
        locals.var_pparam_b4soirds0_dn3 = assign13130_e9524_d_n3;
        locals.var_pparam_b4soirds0_dn4 = assign13130_e9524_d_n4;
        locals.var_pparam_b4soirds0_dn5 = assign13130_e9524_d_n5;
        locals.var_pparam_b4soirds0_dn6 = assign13130_e9524_d_n6;
        locals.var_pparam_b4soirds0_dn7 = assign13130_e9524_d_n7;
        locals.var_pparam_b4soirds0_dn8 = assign13130_e9524_d_n8;
        locals.var_pparam_b4soirds0_dn9 = assign13130_e9524_d_n9;
        locals.var_pparam_b4soirds0_dn10 = assign13130_e9524_d_n10;
        locals.var_pparam_b4soirds0_dn11 = assign13130_e9524_d_n11;
        locals.var_pparam_b4soirds0_dn12 = assign13130_e9524_d_n12;

        locals.var_deltemp = 0.0;
        locals.var_deltemp_dn6 = 0.0;

        let assign14040_e9824: f64 = if ((p.p33 == 1.0) && (p.p16 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1109 = assign14040_e9824;

        let (assign14050_e9828, assign14050_e9828_d_n6,) = {
    if (locals.var_guard1109 != 0.0) {
        ((nv6 - 0.0), 1.0,)
    } else {
        (locals.var_deltemp, locals.var_deltemp_dn6,)
    }
};
        locals.var_deltemp = assign14050_e9828;
        locals.var_deltemp_dn6 = assign14050_e9828_d_n6;

        let (assign14060_e9833, assign14060_e9833_d_n6,) = {
    if (locals.var_guard1109 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_deltemp, locals.var_deltemp_dn6,)
    }
};
        locals.var_deltemp = assign14060_e9833;
        locals.var_deltemp_dn6 = assign14060_e9833_d_n6;

        let assign14070_e9836: f64 = (locals.var_deltemp + locals.var_devtemp);
        locals.var_devtemp = assign14070_e9836;
        locals.var_devtemp_dn6 = (locals.var_deltemp_dn6 + locals.var_devtemp_dn6);

        let assign14080_e9839: f64 = (locals.var_devtemp / locals.var_b4soitnom);
        locals.var_tempratio = assign14080_e9839;
        locals.var_tempratio_dn6 = (locals.var_devtemp_dn6 / locals.var_b4soitnom);

        let assign14090_e9842: f64 = (locals.var_devtemp / locals.var_b4soitnom);
        let assign14090_e9844: f64 = (assign14090_e9842 - 1.0);
        locals.var_tempratiominus1 = assign14090_e9844;
        locals.var_tempratiominus1_dn6 = (locals.var_devtemp_dn6 / locals.var_b4soitnom);

        locals.var_coxeff2 = 0.0;
        locals.var_coxeff2_dn3 = 0.0;
        locals.var_coxeff2_dn4 = 0.0;
        locals.var_coxeff2_dn5 = 0.0;
        locals.var_coxeff2_dn6 = 0.0;
        locals.var_coxeff2_dn7 = 0.0;
        locals.var_coxeff2_dn8 = 0.0;
        locals.var_coxeff2_dn9 = 0.0;
        locals.var_coxeff2_dn10 = 0.0;
        locals.var_coxeff2_dn11 = 0.0;
        locals.var_coxeff2_dn12 = 0.0;

        locals.var_coxwlcen2 = 0.0;
        locals.var_coxwlcen2_dn3 = 0.0;
        locals.var_coxwlcen2_dn4 = 0.0;
        locals.var_coxwlcen2_dn5 = 0.0;
        locals.var_coxwlcen2_dn6 = 0.0;
        locals.var_coxwlcen2_dn7 = 0.0;
        locals.var_coxwlcen2_dn8 = 0.0;
        locals.var_coxwlcen2_dn9 = 0.0;
        locals.var_coxwlcen2_dn10 = 0.0;
        locals.var_coxwlcen2_dn11 = 0.0;
        locals.var_coxwlcen2_dn12 = 0.0;

        locals.var_coxwlcenb2 = 0.0;
        locals.var_coxwlcenb2_dn3 = 0.0;
        locals.var_coxwlcenb2_dn4 = 0.0;
        locals.var_coxwlcenb2_dn5 = 0.0;
        locals.var_coxwlcenb2_dn6 = 0.0;
        locals.var_coxwlcenb2_dn7 = 0.0;
        locals.var_coxwlcenb2_dn8 = 0.0;
        locals.var_coxwlcenb2_dn9 = 0.0;
        locals.var_coxwlcenb2_dn10 = 0.0;
        locals.var_coxwlcenb2_dn11 = 0.0;
        locals.var_coxwlcenb2_dn12 = 0.0;

        locals.var_deltaphi2 = 0.0;
        locals.var_deltaphi2_dn3 = 0.0;
        locals.var_deltaphi2_dn4 = 0.0;
        locals.var_deltaphi2_dn5 = 0.0;
        locals.var_deltaphi2_dn6 = 0.0;
        locals.var_deltaphi2_dn7 = 0.0;
        locals.var_deltaphi2_dn8 = 0.0;
        locals.var_deltaphi2_dn9 = 0.0;
        locals.var_deltaphi2_dn10 = 0.0;
        locals.var_deltaphi2_dn11 = 0.0;
        locals.var_deltaphi2_dn12 = 0.0;

        locals.var_tcen2 = 0.0;
        locals.var_tcen2_dn3 = 0.0;
        locals.var_tcen2_dn4 = 0.0;
        locals.var_tcen2_dn5 = 0.0;
        locals.var_tcen2_dn6 = 0.0;
        locals.var_tcen2_dn7 = 0.0;
        locals.var_tcen2_dn8 = 0.0;
        locals.var_tcen2_dn9 = 0.0;
        locals.var_tcen2_dn10 = 0.0;
        locals.var_tcen2_dn11 = 0.0;
        locals.var_tcen2_dn12 = 0.0;

        locals.var_t02 = 0.0;
        locals.var_t02_dn3 = 0.0;
        locals.var_t02_dn4 = 0.0;
        locals.var_t02_dn5 = 0.0;
        locals.var_t02_dn6 = 0.0;
        locals.var_t02_dn7 = 0.0;
        locals.var_t02_dn8 = 0.0;
        locals.var_t02_dn9 = 0.0;
        locals.var_t02_dn10 = 0.0;
        locals.var_t02_dn11 = 0.0;
        locals.var_t02_dn12 = 0.0;

        locals.var_t12 = 0.0;
        locals.var_t12_dn3 = 0.0;
        locals.var_t12_dn4 = 0.0;
        locals.var_t12_dn5 = 0.0;
        locals.var_t12_dn6 = 0.0;
        locals.var_t12_dn7 = 0.0;
        locals.var_t12_dn8 = 0.0;
        locals.var_t12_dn9 = 0.0;
        locals.var_t12_dn10 = 0.0;
        locals.var_t12_dn11 = 0.0;
        locals.var_t12_dn12 = 0.0;

    }

    pub(super) fn stamp_transient_block_27(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_t22 = 0.0;
        locals.var_t22_dn3 = 0.0;
        locals.var_t22_dn4 = 0.0;
        locals.var_t22_dn5 = 0.0;
        locals.var_t22_dn6 = 0.0;
        locals.var_t22_dn7 = 0.0;
        locals.var_t22_dn8 = 0.0;
        locals.var_t22_dn9 = 0.0;
        locals.var_t22_dn10 = 0.0;
        locals.var_t22_dn11 = 0.0;
        locals.var_t22_dn12 = 0.0;

        locals.var_vdseffcv2 = 0.0;
        locals.var_vdseffcv2_dn3 = 0.0;
        locals.var_vdseffcv2_dn4 = 0.0;
        locals.var_vdseffcv2_dn5 = 0.0;
        locals.var_vdseffcv2_dn6 = 0.0;
        locals.var_vdseffcv2_dn7 = 0.0;
        locals.var_vdseffcv2_dn8 = 0.0;
        locals.var_vdseffcv2_dn9 = 0.0;
        locals.var_vdseffcv2_dn10 = 0.0;
        locals.var_vdseffcv2_dn11 = 0.0;
        locals.var_vdseffcv2_dn12 = 0.0;

        locals.var_vfb2 = 0.0;
        locals.var_vfb2_dn3 = 0.0;
        locals.var_vfb2_dn4 = 0.0;
        locals.var_vfb2_dn5 = 0.0;
        locals.var_vfb2_dn6 = 0.0;
        locals.var_vfb2_dn7 = 0.0;
        locals.var_vfb2_dn8 = 0.0;
        locals.var_vfb2_dn9 = 0.0;
        locals.var_vfb2_dn10 = 0.0;
        locals.var_vfb2_dn11 = 0.0;
        locals.var_vfb2_dn12 = 0.0;

        locals.var_vfbeff2 = 0.0;
        locals.var_vfbeff2_dn3 = 0.0;
        locals.var_vfbeff2_dn4 = 0.0;
        locals.var_vfbeff2_dn5 = 0.0;
        locals.var_vfbeff2_dn6 = 0.0;
        locals.var_vfbeff2_dn7 = 0.0;
        locals.var_vfbeff2_dn8 = 0.0;
        locals.var_vfbeff2_dn9 = 0.0;
        locals.var_vfbeff2_dn10 = 0.0;
        locals.var_vfbeff2_dn11 = 0.0;
        locals.var_vfbeff2_dn12 = 0.0;

        locals.var_vfbzb2 = 0.0;
        locals.var_vfbzb2_dn3 = 0.0;
        locals.var_vfbzb2_dn4 = 0.0;
        locals.var_vfbzb2_dn5 = 0.0;
        locals.var_vfbzb2_dn6 = 0.0;
        locals.var_vfbzb2_dn7 = 0.0;
        locals.var_vfbzb2_dn8 = 0.0;
        locals.var_vfbzb2_dn9 = 0.0;
        locals.var_vfbzb2_dn10 = 0.0;
        locals.var_vfbzb2_dn11 = 0.0;
        locals.var_vfbzb2_dn12 = 0.0;

        locals.var_vgsteff2 = 0.0;
        locals.var_vgsteff2_dn3 = 0.0;
        locals.var_vgsteff2_dn4 = 0.0;
        locals.var_vgsteff2_dn5 = 0.0;
        locals.var_vgsteff2_dn6 = 0.0;
        locals.var_vgsteff2_dn7 = 0.0;
        locals.var_vgsteff2_dn8 = 0.0;
        locals.var_vgsteff2_dn9 = 0.0;
        locals.var_vgsteff2_dn10 = 0.0;
        locals.var_vgsteff2_dn11 = 0.0;
        locals.var_vgsteff2_dn12 = 0.0;

        locals.var_rds0 = locals.var_pparam_b4soirds0;
        locals.var_rds0_dn3 = locals.var_pparam_b4soirds0_dn3;
        locals.var_rds0_dn4 = locals.var_pparam_b4soirds0_dn4;
        locals.var_rds0_dn5 = locals.var_pparam_b4soirds0_dn5;
        locals.var_rds0_dn6 = locals.var_pparam_b4soirds0_dn6;
        locals.var_rds0_dn7 = locals.var_pparam_b4soirds0_dn7;
        locals.var_rds0_dn8 = locals.var_pparam_b4soirds0_dn8;
        locals.var_rds0_dn9 = locals.var_pparam_b4soirds0_dn9;
        locals.var_rds0_dn10 = locals.var_pparam_b4soirds0_dn10;
        locals.var_rds0_dn11 = locals.var_pparam_b4soirds0_dn11;
        locals.var_rds0_dn12 = locals.var_pparam_b4soirds0_dn12;

        locals.var_rd0 = locals.var_pparam_b4soird0;
        locals.var_rd0_dn3 = locals.var_pparam_b4soird0_dn3;
        locals.var_rd0_dn4 = locals.var_pparam_b4soird0_dn4;
        locals.var_rd0_dn5 = locals.var_pparam_b4soird0_dn5;
        locals.var_rd0_dn6 = locals.var_pparam_b4soird0_dn6;
        locals.var_rd0_dn7 = locals.var_pparam_b4soird0_dn7;
        locals.var_rd0_dn8 = locals.var_pparam_b4soird0_dn8;
        locals.var_rd0_dn9 = locals.var_pparam_b4soird0_dn9;
        locals.var_rd0_dn10 = locals.var_pparam_b4soird0_dn10;
        locals.var_rd0_dn11 = locals.var_pparam_b4soird0_dn11;
        locals.var_rd0_dn12 = locals.var_pparam_b4soird0_dn12;

        locals.var_rs0 = locals.var_pparam_b4soirs0;
        locals.var_rs0_dn3 = locals.var_pparam_b4soirs0_dn3;
        locals.var_rs0_dn4 = locals.var_pparam_b4soirs0_dn4;
        locals.var_rs0_dn5 = locals.var_pparam_b4soirs0_dn5;
        locals.var_rs0_dn6 = locals.var_pparam_b4soirs0_dn6;
        locals.var_rs0_dn7 = locals.var_pparam_b4soirs0_dn7;
        locals.var_rs0_dn8 = locals.var_pparam_b4soirs0_dn8;
        locals.var_rs0_dn9 = locals.var_pparam_b4soirs0_dn9;
        locals.var_rs0_dn10 = locals.var_pparam_b4soirs0_dn10;
        locals.var_rs0_dn11 = locals.var_pparam_b4soirs0_dn11;
        locals.var_rs0_dn12 = locals.var_pparam_b4soirs0_dn12;

        locals.var_rdwmin = locals.var_pparam_b4soirdwmin;
        locals.var_rdwmin_dn3 = locals.var_pparam_b4soirdwmin_dn3;
        locals.var_rdwmin_dn4 = locals.var_pparam_b4soirdwmin_dn4;
        locals.var_rdwmin_dn5 = locals.var_pparam_b4soirdwmin_dn5;
        locals.var_rdwmin_dn6 = locals.var_pparam_b4soirdwmin_dn6;
        locals.var_rdwmin_dn7 = locals.var_pparam_b4soirdwmin_dn7;
        locals.var_rdwmin_dn8 = locals.var_pparam_b4soirdwmin_dn8;
        locals.var_rdwmin_dn9 = locals.var_pparam_b4soirdwmin_dn9;
        locals.var_rdwmin_dn10 = locals.var_pparam_b4soirdwmin_dn10;
        locals.var_rdwmin_dn11 = locals.var_pparam_b4soirdwmin_dn11;
        locals.var_rdwmin_dn12 = locals.var_pparam_b4soirdwmin_dn12;

        locals.var_rswmin = locals.var_pparam_b4soirswmin;
        locals.var_rswmin_dn3 = locals.var_pparam_b4soirswmin_dn3;
        locals.var_rswmin_dn4 = locals.var_pparam_b4soirswmin_dn4;
        locals.var_rswmin_dn5 = locals.var_pparam_b4soirswmin_dn5;
        locals.var_rswmin_dn6 = locals.var_pparam_b4soirswmin_dn6;
        locals.var_rswmin_dn7 = locals.var_pparam_b4soirswmin_dn7;
        locals.var_rswmin_dn8 = locals.var_pparam_b4soirswmin_dn8;
        locals.var_rswmin_dn9 = locals.var_pparam_b4soirswmin_dn9;
        locals.var_rswmin_dn10 = locals.var_pparam_b4soirswmin_dn10;
        locals.var_rswmin_dn11 = locals.var_pparam_b4soirswmin_dn11;
        locals.var_rswmin_dn12 = locals.var_pparam_b4soirswmin_dn12;

        let assign14320_e9875: f64 = if ((p.p33 == 1.0) && (p.p16 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1457 = assign14320_e9875;

        let assign14330_e9878: f64 = if locals.var_b4soimtrlmod == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1458 = assign14330_e9878;

        let (assign14340_e9886, assign14340_e9886_d_n6,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 != 0.0)) {
        let assign14340_e9884: f64 = (8.617087e-5 * locals.var_devtemp);
        (assign14340_e9884, (8.617087e-5 * locals.var_devtemp_dn6),)
    } else {
        (locals.var_vtm, locals.var_vtm_dn6,)
    }
};
        locals.var_vtm = assign14340_e9886;
        locals.var_vtm_dn6 = assign14340_e9886_d_n6;

        let (assign14350_e9894, assign14350_e9894_d_n3, assign14350_e9894_d_n4, assign14350_e9894_d_n5, assign14350_e9894_d_n6, assign14350_e9894_d_n7, assign14350_e9894_d_n8, assign14350_e9894_d_n9, assign14350_e9894_d_n10, assign14350_e9894_d_n11, assign14350_e9894_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 != 0.0)) {
        let assign14350_e9892: f64 = (1108.0 + locals.var_devtemp);
        (assign14350_e9892, 0.0, 0.0, 0.0, locals.var_devtemp_dn6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign14350_e9894;
        locals.var_t0__blk1144_dn3 = assign14350_e9894_d_n3;
        locals.var_t0__blk1144_dn4 = assign14350_e9894_d_n4;
        locals.var_t0__blk1144_dn5 = assign14350_e9894_d_n5;
        locals.var_t0__blk1144_dn6 = assign14350_e9894_d_n6;
        locals.var_t0__blk1144_dn7 = assign14350_e9894_d_n7;
        locals.var_t0__blk1144_dn8 = assign14350_e9894_d_n8;
        locals.var_t0__blk1144_dn9 = assign14350_e9894_d_n9;
        locals.var_t0__blk1144_dn10 = assign14350_e9894_d_n10;
        locals.var_t0__blk1144_dn11 = assign14350_e9894_d_n11;
        locals.var_t0__blk1144_dn12 = assign14350_e9894_d_n12;

        let (assign14360_e9902, assign14360_e9902_d_n3, assign14360_e9902_d_n4, assign14360_e9902_d_n5, assign14360_e9902_d_n6, assign14360_e9902_d_n7, assign14360_e9902_d_n8, assign14360_e9902_d_n9, assign14360_e9902_d_n10, assign14360_e9902_d_n11, assign14360_e9902_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 != 0.0)) {
        let assign14360_e9900: f64 = (locals.var_devtemp * locals.var_devtemp);
        (assign14360_e9900, 0.0, 0.0, 0.0, ((locals.var_devtemp_dn6 * locals.var_devtemp) + (locals.var_devtemp * locals.var_devtemp_dn6)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk1149, locals.var_t5__blk1149_dn3, locals.var_t5__blk1149_dn4, locals.var_t5__blk1149_dn5, locals.var_t5__blk1149_dn6, locals.var_t5__blk1149_dn7, locals.var_t5__blk1149_dn8, locals.var_t5__blk1149_dn9, locals.var_t5__blk1149_dn10, locals.var_t5__blk1149_dn11, locals.var_t5__blk1149_dn12,)
    }
};
        locals.var_t5__blk1149 = assign14360_e9902;
        locals.var_t5__blk1149_dn3 = assign14360_e9902_d_n3;
        locals.var_t5__blk1149_dn4 = assign14360_e9902_d_n4;
        locals.var_t5__blk1149_dn5 = assign14360_e9902_d_n5;
        locals.var_t5__blk1149_dn6 = assign14360_e9902_d_n6;
        locals.var_t5__blk1149_dn7 = assign14360_e9902_d_n7;
        locals.var_t5__blk1149_dn8 = assign14360_e9902_d_n8;
        locals.var_t5__blk1149_dn9 = assign14360_e9902_d_n9;
        locals.var_t5__blk1149_dn10 = assign14360_e9902_d_n10;
        locals.var_t5__blk1149_dn11 = assign14360_e9902_d_n11;
        locals.var_t5__blk1149_dn12 = assign14360_e9902_d_n12;

        let (assign14370_e9914, assign14370_e9914_d_n3, assign14370_e9914_d_n4, assign14370_e9914_d_n5, assign14370_e9914_d_n6, assign14370_e9914_d_n7, assign14370_e9914_d_n8, assign14370_e9914_d_n9, assign14370_e9914_d_n10, assign14370_e9914_d_n11, assign14370_e9914_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 != 0.0)) {
        let assign14370_e9909: f64 = (0.000702 * locals.var_t5__blk1149);
        let assign14370_e9911: f64 = (assign14370_e9909 / locals.var_t0__blk1144);
        let assign14370_e9912: f64 = (1.16 - assign14370_e9911);
        (assign14370_e9912, (-((((0.000702 * locals.var_t5__blk1149_dn3) * locals.var_t0__blk1144) - (assign14370_e9909 * locals.var_t0__blk1144_dn3)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))), (-((((0.000702 * locals.var_t5__blk1149_dn4) * locals.var_t0__blk1144) - (assign14370_e9909 * locals.var_t0__blk1144_dn4)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))), (-((((0.000702 * locals.var_t5__blk1149_dn5) * locals.var_t0__blk1144) - (assign14370_e9909 * locals.var_t0__blk1144_dn5)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))), (-((((0.000702 * locals.var_t5__blk1149_dn6) * locals.var_t0__blk1144) - (assign14370_e9909 * locals.var_t0__blk1144_dn6)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))), (-((((0.000702 * locals.var_t5__blk1149_dn7) * locals.var_t0__blk1144) - (assign14370_e9909 * locals.var_t0__blk1144_dn7)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))), (-((((0.000702 * locals.var_t5__blk1149_dn8) * locals.var_t0__blk1144) - (assign14370_e9909 * locals.var_t0__blk1144_dn8)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))), (-((((0.000702 * locals.var_t5__blk1149_dn9) * locals.var_t0__blk1144) - (assign14370_e9909 * locals.var_t0__blk1144_dn9)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))), (-((((0.000702 * locals.var_t5__blk1149_dn10) * locals.var_t0__blk1144) - (assign14370_e9909 * locals.var_t0__blk1144_dn10)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))), (-((((0.000702 * locals.var_t5__blk1149_dn11) * locals.var_t0__blk1144) - (assign14370_e9909 * locals.var_t0__blk1144_dn11)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))), (-((((0.000702 * locals.var_t5__blk1149_dn12) * locals.var_t0__blk1144) - (assign14370_e9909 * locals.var_t0__blk1144_dn12)) / (locals.var_t0__blk1144 * locals.var_t0__blk1144))),)
    } else {
        (locals.var_eg__blk1212, locals.var_eg__blk1212_dn3, locals.var_eg__blk1212_dn4, locals.var_eg__blk1212_dn5, locals.var_eg__blk1212_dn6, locals.var_eg__blk1212_dn7, locals.var_eg__blk1212_dn8, locals.var_eg__blk1212_dn9, locals.var_eg__blk1212_dn10, locals.var_eg__blk1212_dn11, locals.var_eg__blk1212_dn12,)
    }
};
        locals.var_eg__blk1212 = assign14370_e9914;
        locals.var_eg__blk1212_dn3 = assign14370_e9914_d_n3;
        locals.var_eg__blk1212_dn4 = assign14370_e9914_d_n4;
        locals.var_eg__blk1212_dn5 = assign14370_e9914_d_n5;
        locals.var_eg__blk1212_dn6 = assign14370_e9914_d_n6;
        locals.var_eg__blk1212_dn7 = assign14370_e9914_d_n7;
        locals.var_eg__blk1212_dn8 = assign14370_e9914_d_n8;
        locals.var_eg__blk1212_dn9 = assign14370_e9914_d_n9;
        locals.var_eg__blk1212_dn10 = assign14370_e9914_d_n10;
        locals.var_eg__blk1212_dn11 = assign14370_e9914_d_n11;
        locals.var_eg__blk1212_dn12 = assign14370_e9914_d_n12;

        let (assign14380_e9920, assign14380_e9920_d_n3, assign14380_e9920_d_n4, assign14380_e9920_d_n5, assign14380_e9920_d_n6, assign14380_e9920_d_n7, assign14380_e9920_d_n8, assign14380_e9920_d_n9, assign14380_e9920_d_n10, assign14380_e9920_d_n11, assign14380_e9920_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 != 0.0)) {
        (0.00019230584, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign14380_e9920;
        locals.var_t2__blk1146_dn3 = assign14380_e9920_d_n3;
        locals.var_t2__blk1146_dn4 = assign14380_e9920_d_n4;
        locals.var_t2__blk1146_dn5 = assign14380_e9920_d_n5;
        locals.var_t2__blk1146_dn6 = assign14380_e9920_d_n6;
        locals.var_t2__blk1146_dn7 = assign14380_e9920_d_n7;
        locals.var_t2__blk1146_dn8 = assign14380_e9920_d_n8;
        locals.var_t2__blk1146_dn9 = assign14380_e9920_d_n9;
        locals.var_t2__blk1146_dn10 = assign14380_e9920_d_n10;
        locals.var_t2__blk1146_dn11 = assign14380_e9920_d_n11;
        locals.var_t2__blk1146_dn12 = assign14380_e9920_d_n12;

        let (assign14390_e9927, assign14390_e9927_d_n3, assign14390_e9927_d_n4, assign14390_e9927_d_n5, assign14390_e9927_d_n6, assign14390_e9927_d_n7, assign14390_e9927_d_n8, assign14390_e9927_d_n9, assign14390_e9927_d_n10, assign14390_e9927_d_n11, assign14390_e9927_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 != 0.0)) {
        let assign14390_e9925: f64 = (locals.var_devtemp).sqrt();
        (assign14390_e9925, 0.0, 0.0, 0.0, (locals.var_devtemp_dn6 / (2.0 * assign14390_e9925)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk1149, locals.var_t5__blk1149_dn3, locals.var_t5__blk1149_dn4, locals.var_t5__blk1149_dn5, locals.var_t5__blk1149_dn6, locals.var_t5__blk1149_dn7, locals.var_t5__blk1149_dn8, locals.var_t5__blk1149_dn9, locals.var_t5__blk1149_dn10, locals.var_t5__blk1149_dn11, locals.var_t5__blk1149_dn12,)
    }
};
        locals.var_t5__blk1149 = assign14390_e9927;
        locals.var_t5__blk1149_dn3 = assign14390_e9927_d_n3;
        locals.var_t5__blk1149_dn4 = assign14390_e9927_d_n4;
        locals.var_t5__blk1149_dn5 = assign14390_e9927_d_n5;
        locals.var_t5__blk1149_dn6 = assign14390_e9927_d_n6;
        locals.var_t5__blk1149_dn7 = assign14390_e9927_d_n7;
        locals.var_t5__blk1149_dn8 = assign14390_e9927_d_n8;
        locals.var_t5__blk1149_dn9 = assign14390_e9927_d_n9;
        locals.var_t5__blk1149_dn10 = assign14390_e9927_d_n10;
        locals.var_t5__blk1149_dn11 = assign14390_e9927_d_n11;
        locals.var_t5__blk1149_dn12 = assign14390_e9927_d_n12;

        let (assign14400_e9939, assign14400_e9939_d_n3, assign14400_e9939_d_n4, assign14400_e9939_d_n5, assign14400_e9939_d_n6, assign14400_e9939_d_n7, assign14400_e9939_d_n8, assign14400_e9939_d_n9, assign14400_e9939_d_n10, assign14400_e9939_d_n11, assign14400_e9939_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 != 0.0)) {
        let assign14400_e9933: f64 = (14500000000.0 * locals.var_devtemp);
        let assign14400_e9935: f64 = (assign14400_e9933 * locals.var_t5__blk1149);
        let assign14400_e9937: f64 = (assign14400_e9935 * locals.var_t2__blk1146);
        (assign14400_e9937, (((assign14400_e9933 * locals.var_t5__blk1149_dn3) * locals.var_t2__blk1146) + (assign14400_e9935 * locals.var_t2__blk1146_dn3)), (((assign14400_e9933 * locals.var_t5__blk1149_dn4) * locals.var_t2__blk1146) + (assign14400_e9935 * locals.var_t2__blk1146_dn4)), (((assign14400_e9933 * locals.var_t5__blk1149_dn5) * locals.var_t2__blk1146) + (assign14400_e9935 * locals.var_t2__blk1146_dn5)), (((((14500000000.0 * locals.var_devtemp_dn6) * locals.var_t5__blk1149) + (assign14400_e9933 * locals.var_t5__blk1149_dn6)) * locals.var_t2__blk1146) + (assign14400_e9935 * locals.var_t2__blk1146_dn6)), (((assign14400_e9933 * locals.var_t5__blk1149_dn7) * locals.var_t2__blk1146) + (assign14400_e9935 * locals.var_t2__blk1146_dn7)), (((assign14400_e9933 * locals.var_t5__blk1149_dn8) * locals.var_t2__blk1146) + (assign14400_e9935 * locals.var_t2__blk1146_dn8)), (((assign14400_e9933 * locals.var_t5__blk1149_dn9) * locals.var_t2__blk1146) + (assign14400_e9935 * locals.var_t2__blk1146_dn9)), (((assign14400_e9933 * locals.var_t5__blk1149_dn10) * locals.var_t2__blk1146) + (assign14400_e9935 * locals.var_t2__blk1146_dn10)), (((assign14400_e9933 * locals.var_t5__blk1149_dn11) * locals.var_t2__blk1146) + (assign14400_e9935 * locals.var_t2__blk1146_dn11)), (((assign14400_e9933 * locals.var_t5__blk1149_dn12) * locals.var_t2__blk1146) + (assign14400_e9935 * locals.var_t2__blk1146_dn12)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign14400_e9939;
        locals.var_t3__blk1147_dn3 = assign14400_e9939_d_n3;
        locals.var_t3__blk1147_dn4 = assign14400_e9939_d_n4;
        locals.var_t3__blk1147_dn5 = assign14400_e9939_d_n5;
        locals.var_t3__blk1147_dn6 = assign14400_e9939_d_n6;
        locals.var_t3__blk1147_dn7 = assign14400_e9939_d_n7;
        locals.var_t3__blk1147_dn8 = assign14400_e9939_d_n8;
        locals.var_t3__blk1147_dn9 = assign14400_e9939_d_n9;
        locals.var_t3__blk1147_dn10 = assign14400_e9939_d_n10;
        locals.var_t3__blk1147_dn11 = assign14400_e9939_d_n11;
        locals.var_t3__blk1147_dn12 = assign14400_e9939_d_n12;

        let (assign14410_e9951, assign14410_e9951_d_n3, assign14410_e9951_d_n4, assign14410_e9951_d_n5, assign14410_e9951_d_n6, assign14410_e9951_d_n7, assign14410_e9951_d_n8, assign14410_e9951_d_n9, assign14410_e9951_d_n10, assign14410_e9951_d_n11, assign14410_e9951_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 != 0.0)) {
        let assign14410_e9947: f64 = (2.0 * locals.var_vtm);
        let assign14410_e9948: f64 = (locals.var_eg__blk1212 / assign14410_e9947);
        let assign14410_e9949: f64 = (21.5565981 - assign14410_e9948);
        (assign14410_e9949, (-(locals.var_eg__blk1212_dn3 / assign14410_e9947)), (-(locals.var_eg__blk1212_dn4 / assign14410_e9947)), (-(locals.var_eg__blk1212_dn5 / assign14410_e9947)), (-(((locals.var_eg__blk1212_dn6 * assign14410_e9947) - (locals.var_eg__blk1212 * (2.0 * locals.var_vtm_dn6))) / (assign14410_e9947 * assign14410_e9947))), (-(locals.var_eg__blk1212_dn7 / assign14410_e9947)), (-(locals.var_eg__blk1212_dn8 / assign14410_e9947)), (-(locals.var_eg__blk1212_dn9 / assign14410_e9947)), (-(locals.var_eg__blk1212_dn10 / assign14410_e9947)), (-(locals.var_eg__blk1212_dn11 / assign14410_e9947)), (-(locals.var_eg__blk1212_dn12 / assign14410_e9947)),)
    } else {
        (locals.var_t6__blk1150, locals.var_t6__blk1150_dn3, locals.var_t6__blk1150_dn4, locals.var_t6__blk1150_dn5, locals.var_t6__blk1150_dn6, locals.var_t6__blk1150_dn7, locals.var_t6__blk1150_dn8, locals.var_t6__blk1150_dn9, locals.var_t6__blk1150_dn10, locals.var_t6__blk1150_dn11, locals.var_t6__blk1150_dn12,)
    }
};
        locals.var_t6__blk1150 = assign14410_e9951;
        locals.var_t6__blk1150_dn3 = assign14410_e9951_d_n3;
        locals.var_t6__blk1150_dn4 = assign14410_e9951_d_n4;
        locals.var_t6__blk1150_dn5 = assign14410_e9951_d_n5;
        locals.var_t6__blk1150_dn6 = assign14410_e9951_d_n6;
        locals.var_t6__blk1150_dn7 = assign14410_e9951_d_n7;
        locals.var_t6__blk1150_dn8 = assign14410_e9951_d_n8;
        locals.var_t6__blk1150_dn9 = assign14410_e9951_d_n9;
        locals.var_t6__blk1150_dn10 = assign14410_e9951_d_n10;
        locals.var_t6__blk1150_dn11 = assign14410_e9951_d_n11;
        locals.var_t6__blk1150_dn12 = assign14410_e9951_d_n12;

        let assign14420_e9954: f64 = (-100.0);
        let assign14420_e9955: f64 = if locals.var_t6__blk1150 > assign14420_e9954 { 1.0 } else { 0.0 };
        locals.var_guard1459 = assign14420_e9955;

        let (assign14430_e9964, assign14430_e9964_d_n3, assign14430_e9964_d_n4, assign14430_e9964_d_n5, assign14430_e9964_d_n6, assign14430_e9964_d_n7, assign14430_e9964_d_n8, assign14430_e9964_d_n9, assign14430_e9964_d_n10, assign14430_e9964_d_n11, assign14430_e9964_d_n12,) = {
    if (((locals.var_guard1457 != 0.0) && (locals.var_guard1458 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign14430_e9962: f64 = (locals.var_t6__blk1150).exp();
        (assign14430_e9962, (assign14430_e9962 * locals.var_t6__blk1150_dn3), (assign14430_e9962 * locals.var_t6__blk1150_dn4), (assign14430_e9962 * locals.var_t6__blk1150_dn5), (assign14430_e9962 * locals.var_t6__blk1150_dn6), (assign14430_e9962 * locals.var_t6__blk1150_dn7), (assign14430_e9962 * locals.var_t6__blk1150_dn8), (assign14430_e9962 * locals.var_t6__blk1150_dn9), (assign14430_e9962 * locals.var_t6__blk1150_dn10), (assign14430_e9962 * locals.var_t6__blk1150_dn11), (assign14430_e9962 * locals.var_t6__blk1150_dn12),)
    } else {
        (locals.var_t4__blk1148, locals.var_t4__blk1148_dn3, locals.var_t4__blk1148_dn4, locals.var_t4__blk1148_dn5, locals.var_t4__blk1148_dn6, locals.var_t4__blk1148_dn7, locals.var_t4__blk1148_dn8, locals.var_t4__blk1148_dn9, locals.var_t4__blk1148_dn10, locals.var_t4__blk1148_dn11, locals.var_t4__blk1148_dn12,)
    }
};
        locals.var_t4__blk1148 = assign14430_e9964;
        locals.var_t4__blk1148_dn3 = assign14430_e9964_d_n3;
        locals.var_t4__blk1148_dn4 = assign14430_e9964_d_n4;
        locals.var_t4__blk1148_dn5 = assign14430_e9964_d_n5;
        locals.var_t4__blk1148_dn6 = assign14430_e9964_d_n6;
        locals.var_t4__blk1148_dn7 = assign14430_e9964_d_n7;
        locals.var_t4__blk1148_dn8 = assign14430_e9964_d_n8;
        locals.var_t4__blk1148_dn9 = assign14430_e9964_d_n9;
        locals.var_t4__blk1148_dn10 = assign14430_e9964_d_n10;
        locals.var_t4__blk1148_dn11 = assign14430_e9964_d_n11;
        locals.var_t4__blk1148_dn12 = assign14430_e9964_d_n12;

        let (assign14440_e9975, assign14440_e9975_d_n3, assign14440_e9975_d_n4, assign14440_e9975_d_n5, assign14440_e9975_d_n6, assign14440_e9975_d_n7, assign14440_e9975_d_n8, assign14440_e9975_d_n9, assign14440_e9975_d_n10, assign14440_e9975_d_n11, assign14440_e9975_d_n12,) = {
    if (((locals.var_guard1457 != 0.0) && (locals.var_guard1458 != 0.0)) && (locals.var_guard1459 == 0.0)) {
        let assign14440_e9972: f64 = (-100.0);
        let assign14440_e9973: f64 = (assign14440_e9972).exp();
        (assign14440_e9973, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk1148, locals.var_t4__blk1148_dn3, locals.var_t4__blk1148_dn4, locals.var_t4__blk1148_dn5, locals.var_t4__blk1148_dn6, locals.var_t4__blk1148_dn7, locals.var_t4__blk1148_dn8, locals.var_t4__blk1148_dn9, locals.var_t4__blk1148_dn10, locals.var_t4__blk1148_dn11, locals.var_t4__blk1148_dn12,)
    }
};
        locals.var_t4__blk1148 = assign14440_e9975;
        locals.var_t4__blk1148_dn3 = assign14440_e9975_d_n3;
        locals.var_t4__blk1148_dn4 = assign14440_e9975_d_n4;
        locals.var_t4__blk1148_dn5 = assign14440_e9975_d_n5;
        locals.var_t4__blk1148_dn6 = assign14440_e9975_d_n6;
        locals.var_t4__blk1148_dn7 = assign14440_e9975_d_n7;
        locals.var_t4__blk1148_dn8 = assign14440_e9975_d_n8;
        locals.var_t4__blk1148_dn9 = assign14440_e9975_d_n9;
        locals.var_t4__blk1148_dn10 = assign14440_e9975_d_n10;
        locals.var_t4__blk1148_dn11 = assign14440_e9975_d_n11;
        locals.var_t4__blk1148_dn12 = assign14440_e9975_d_n12;

        let (assign14450_e9983, assign14450_e9983_d_n3, assign14450_e9983_d_n4, assign14450_e9983_d_n5, assign14450_e9983_d_n6, assign14450_e9983_d_n7, assign14450_e9983_d_n8, assign14450_e9983_d_n9, assign14450_e9983_d_n10, assign14450_e9983_d_n11, assign14450_e9983_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 != 0.0)) {
        let assign14450_e9981: f64 = (locals.var_t3__blk1147 * locals.var_t4__blk1148);
        (assign14450_e9981, ((locals.var_t3__blk1147_dn3 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn3)), ((locals.var_t3__blk1147_dn4 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn4)), ((locals.var_t3__blk1147_dn5 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn5)), ((locals.var_t3__blk1147_dn6 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn6)), ((locals.var_t3__blk1147_dn7 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn7)), ((locals.var_t3__blk1147_dn8 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn8)), ((locals.var_t3__blk1147_dn9 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn9)), ((locals.var_t3__blk1147_dn10 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn10)), ((locals.var_t3__blk1147_dn11 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn11)), ((locals.var_t3__blk1147_dn12 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn12)),)
    } else {
        (locals.var_ni__blk1211, locals.var_ni__blk1211_dn3, locals.var_ni__blk1211_dn4, locals.var_ni__blk1211_dn5, locals.var_ni__blk1211_dn6, locals.var_ni__blk1211_dn7, locals.var_ni__blk1211_dn8, locals.var_ni__blk1211_dn9, locals.var_ni__blk1211_dn10, locals.var_ni__blk1211_dn11, locals.var_ni__blk1211_dn12,)
    }
};
        locals.var_ni__blk1211 = assign14450_e9983;
        locals.var_ni__blk1211_dn3 = assign14450_e9983_d_n3;
        locals.var_ni__blk1211_dn4 = assign14450_e9983_d_n4;
        locals.var_ni__blk1211_dn5 = assign14450_e9983_d_n5;
        locals.var_ni__blk1211_dn6 = assign14450_e9983_d_n6;
        locals.var_ni__blk1211_dn7 = assign14450_e9983_d_n7;
        locals.var_ni__blk1211_dn8 = assign14450_e9983_d_n8;
        locals.var_ni__blk1211_dn9 = assign14450_e9983_d_n9;
        locals.var_ni__blk1211_dn10 = assign14450_e9983_d_n10;
        locals.var_ni__blk1211_dn11 = assign14450_e9983_d_n11;
        locals.var_ni__blk1211_dn12 = assign14450_e9983_d_n12;

        let (assign14460_e10008, assign14460_e10008_d_n3, assign14460_e10008_d_n4, assign14460_e10008_d_n5, assign14460_e10008_d_n6, assign14460_e10008_d_n7, assign14460_e10008_d_n8, assign14460_e10008_d_n9, assign14460_e10008_d_n10, assign14460_e10008_d_n11, assign14460_e10008_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 != 0.0)) {
        let assign14460_e9989: f64 = (1e20 * locals.var_pparam_b4soinpeak);
        let assign14460_e9992: f64 = (locals.var_ni__blk1211 * locals.var_ni__blk1211);
        let assign14460_e9993: f64 = (assign14460_e9989 / assign14460_e9992);
        let (assign14460_e10006, assign14460_e10006_d_n3, assign14460_e10006_d_n4, assign14460_e10006_d_n5, assign14460_e10006_d_n6, assign14460_e10006_d_n7, assign14460_e10006_d_n8, assign14460_e10006_d_n9, assign14460_e10006_d_n10, assign14460_e10006_d_n11, assign14460_e10006_d_n12,) = {
            if (assign14460_e9993 > 1e-38) {
                let assign14460_e9998: f64 = (1e20 * locals.var_pparam_b4soinpeak);
                let assign14460_e10001: f64 = (locals.var_ni__blk1211 * locals.var_ni__blk1211);
                let assign14460_e10002: f64 = (assign14460_e9998 / assign14460_e10001);
                let assign14460_e10003: f64 = (assign14460_e10002).ln();
                (assign14460_e10003, (((((1e20 * locals.var_pparam_b4soinpeak_dn3) * assign14460_e10001) - (assign14460_e9998 * ((locals.var_ni__blk1211_dn3 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn3)))) / (assign14460_e10001 * assign14460_e10001)) / assign14460_e10002), (((((1e20 * locals.var_pparam_b4soinpeak_dn4) * assign14460_e10001) - (assign14460_e9998 * ((locals.var_ni__blk1211_dn4 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn4)))) / (assign14460_e10001 * assign14460_e10001)) / assign14460_e10002), (((((1e20 * locals.var_pparam_b4soinpeak_dn5) * assign14460_e10001) - (assign14460_e9998 * ((locals.var_ni__blk1211_dn5 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn5)))) / (assign14460_e10001 * assign14460_e10001)) / assign14460_e10002), (((((1e20 * locals.var_pparam_b4soinpeak_dn6) * assign14460_e10001) - (assign14460_e9998 * ((locals.var_ni__blk1211_dn6 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn6)))) / (assign14460_e10001 * assign14460_e10001)) / assign14460_e10002), (((((1e20 * locals.var_pparam_b4soinpeak_dn7) * assign14460_e10001) - (assign14460_e9998 * ((locals.var_ni__blk1211_dn7 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn7)))) / (assign14460_e10001 * assign14460_e10001)) / assign14460_e10002), (((((1e20 * locals.var_pparam_b4soinpeak_dn8) * assign14460_e10001) - (assign14460_e9998 * ((locals.var_ni__blk1211_dn8 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn8)))) / (assign14460_e10001 * assign14460_e10001)) / assign14460_e10002), (((((1e20 * locals.var_pparam_b4soinpeak_dn9) * assign14460_e10001) - (assign14460_e9998 * ((locals.var_ni__blk1211_dn9 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn9)))) / (assign14460_e10001 * assign14460_e10001)) / assign14460_e10002), (((((1e20 * locals.var_pparam_b4soinpeak_dn10) * assign14460_e10001) - (assign14460_e9998 * ((locals.var_ni__blk1211_dn10 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn10)))) / (assign14460_e10001 * assign14460_e10001)) / assign14460_e10002), (((((1e20 * locals.var_pparam_b4soinpeak_dn11) * assign14460_e10001) - (assign14460_e9998 * ((locals.var_ni__blk1211_dn11 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn11)))) / (assign14460_e10001 * assign14460_e10001)) / assign14460_e10002), (((((1e20 * locals.var_pparam_b4soinpeak_dn12) * assign14460_e10001) - (assign14460_e9998 * ((locals.var_ni__blk1211_dn12 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn12)))) / (assign14460_e10001 * assign14460_e10001)) / assign14460_e10002),)
            } else {
                let assign14460_e10005: f64 = (-87.49823353377374);
                (assign14460_e10005, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign14460_e10006, assign14460_e10006_d_n3, assign14460_e10006_d_n4, assign14460_e10006_d_n5, assign14460_e10006_d_n6, assign14460_e10006_d_n7, assign14460_e10006_d_n8, assign14460_e10006_d_n9, assign14460_e10006_d_n10, assign14460_e10006_d_n11, assign14460_e10006_d_n12,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign14460_e10008;
        locals.var_t0__blk1144_dn3 = assign14460_e10008_d_n3;
        locals.var_t0__blk1144_dn4 = assign14460_e10008_d_n4;
        locals.var_t0__blk1144_dn5 = assign14460_e10008_d_n5;
        locals.var_t0__blk1144_dn6 = assign14460_e10008_d_n6;
        locals.var_t0__blk1144_dn7 = assign14460_e10008_d_n7;
        locals.var_t0__blk1144_dn8 = assign14460_e10008_d_n8;
        locals.var_t0__blk1144_dn9 = assign14460_e10008_d_n9;
        locals.var_t0__blk1144_dn10 = assign14460_e10008_d_n10;
        locals.var_t0__blk1144_dn11 = assign14460_e10008_d_n11;
        locals.var_t0__blk1144_dn12 = assign14460_e10008_d_n12;

        let (assign14470_e10016, assign14470_e10016_d_n3, assign14470_e10016_d_n4, assign14470_e10016_d_n5, assign14470_e10016_d_n6, assign14470_e10016_d_n7, assign14470_e10016_d_n8, assign14470_e10016_d_n9, assign14470_e10016_d_n10, assign14470_e10016_d_n11, assign14470_e10016_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 != 0.0)) {
        let assign14470_e10014: f64 = (locals.var_vtm * locals.var_t0__blk1144);
        (assign14470_e10014, (locals.var_vtm * locals.var_t0__blk1144_dn3), (locals.var_vtm * locals.var_t0__blk1144_dn4), (locals.var_vtm * locals.var_t0__blk1144_dn5), ((locals.var_vtm_dn6 * locals.var_t0__blk1144) + (locals.var_vtm * locals.var_t0__blk1144_dn6)), (locals.var_vtm * locals.var_t0__blk1144_dn7), (locals.var_vtm * locals.var_t0__blk1144_dn8), (locals.var_vtm * locals.var_t0__blk1144_dn9), (locals.var_vtm * locals.var_t0__blk1144_dn10), (locals.var_vtm * locals.var_t0__blk1144_dn11), (locals.var_vtm * locals.var_t0__blk1144_dn12),)
    } else {
        (locals.var_vbi, locals.var_vbi_dn3, locals.var_vbi_dn4, locals.var_vbi_dn5, locals.var_vbi_dn6, locals.var_vbi_dn7, locals.var_vbi_dn8, locals.var_vbi_dn9, locals.var_vbi_dn10, locals.var_vbi_dn11, locals.var_vbi_dn12,)
    }
};
        locals.var_vbi = assign14470_e10016;
        locals.var_vbi_dn3 = assign14470_e10016_d_n3;
        locals.var_vbi_dn4 = assign14470_e10016_d_n4;
        locals.var_vbi_dn5 = assign14470_e10016_d_n5;
        locals.var_vbi_dn6 = assign14470_e10016_d_n6;
        locals.var_vbi_dn7 = assign14470_e10016_d_n7;
        locals.var_vbi_dn8 = assign14470_e10016_d_n8;
        locals.var_vbi_dn9 = assign14470_e10016_d_n9;
        locals.var_vbi_dn10 = assign14470_e10016_d_n10;
        locals.var_vbi_dn11 = assign14470_e10016_d_n11;
        locals.var_vbi_dn12 = assign14470_e10016_d_n12;

        let (assign14480_e10023,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 == 0.0)) {
        (locals.var_b4soitnom,)
    } else {
        (locals.var_tnom__blk1400,)
    }
};
        locals.var_tnom__blk1400 = assign14480_e10023;

        let (assign14490_e10032, assign14490_e10032_d_n6,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 == 0.0)) {
        let assign14490_e10030: f64 = (8.617087e-5 * locals.var_devtemp);
        (assign14490_e10030, (8.617087e-5 * locals.var_devtemp_dn6),)
    } else {
        (locals.var_vtm, locals.var_vtm_dn6,)
    }
};
        locals.var_vtm = assign14490_e10032;
        locals.var_vtm_dn6 = assign14490_e10032_d_n6;

        let (assign14500_e10041,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 == 0.0)) {
        let assign14500_e10039: f64 = (8.617087e-5 * locals.var_tnom__blk1400);
        (assign14500_e10039,)
    } else {
        (locals.var_vtm0__blk1402,)
    }
};
        locals.var_vtm0__blk1402 = assign14500_e10041;

        let (assign14510_e10048,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 == 0.0)) {
        (locals.var_b4soieg0,)
    } else {
        (locals.var_eg0__blk1401,)
    }
};
        locals.var_eg0__blk1401 = assign14510_e10048;

        let (assign14520_e10065, assign14520_e10065_d_n3, assign14520_e10065_d_n4, assign14520_e10065_d_n5, assign14520_e10065_d_n6, assign14520_e10065_d_n7, assign14520_e10065_d_n8, assign14520_e10065_d_n9, assign14520_e10065_d_n10, assign14520_e10065_d_n11, assign14520_e10065_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 == 0.0)) {
        let assign14520_e10056: f64 = (locals.var_b4soitbgasub * locals.var_devtemp);
        let assign14520_e10058: f64 = (assign14520_e10056 * locals.var_devtemp);
        let assign14520_e10061: f64 = (locals.var_devtemp + locals.var_b4soitbgbsub);
        let assign14520_e10062: f64 = (assign14520_e10058 / assign14520_e10061);
        let assign14520_e10063: f64 = (locals.var_b4soibg0sub - assign14520_e10062);
        (assign14520_e10063, 0.0, 0.0, 0.0, (-((((((locals.var_b4soitbgasub * locals.var_devtemp_dn6) * locals.var_devtemp) + (assign14520_e10056 * locals.var_devtemp_dn6)) * assign14520_e10061) - (assign14520_e10058 * locals.var_devtemp_dn6)) / (assign14520_e10061 * assign14520_e10061))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eg__blk1212, locals.var_eg__blk1212_dn3, locals.var_eg__blk1212_dn4, locals.var_eg__blk1212_dn5, locals.var_eg__blk1212_dn6, locals.var_eg__blk1212_dn7, locals.var_eg__blk1212_dn8, locals.var_eg__blk1212_dn9, locals.var_eg__blk1212_dn10, locals.var_eg__blk1212_dn11, locals.var_eg__blk1212_dn12,)
    }
};
        locals.var_eg__blk1212 = assign14520_e10065;
        locals.var_eg__blk1212_dn3 = assign14520_e10065_d_n3;
        locals.var_eg__blk1212_dn4 = assign14520_e10065_d_n4;
        locals.var_eg__blk1212_dn5 = assign14520_e10065_d_n5;
        locals.var_eg__blk1212_dn6 = assign14520_e10065_d_n6;
        locals.var_eg__blk1212_dn7 = assign14520_e10065_d_n7;
        locals.var_eg__blk1212_dn8 = assign14520_e10065_d_n8;
        locals.var_eg__blk1212_dn9 = assign14520_e10065_d_n9;
        locals.var_eg__blk1212_dn10 = assign14520_e10065_d_n10;
        locals.var_eg__blk1212_dn11 = assign14520_e10065_d_n11;
        locals.var_eg__blk1212_dn12 = assign14520_e10065_d_n12;

        let (assign14530_e10079, assign14530_e10079_d_n3, assign14530_e10079_d_n4, assign14530_e10079_d_n5, assign14530_e10079_d_n6, assign14530_e10079_d_n7, assign14530_e10079_d_n8, assign14530_e10079_d_n9, assign14530_e10079_d_n10, assign14530_e10079_d_n11, assign14530_e10079_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 == 0.0)) {
        let assign14530_e10073: f64 = (locals.var_tnom__blk1400 * locals.var_tnom__blk1400);
        let assign14530_e10075: f64 = (assign14530_e10073 * locals.var_tnom__blk1400);
        let assign14530_e10076: f64 = (assign14530_e10075).sqrt();
        let assign14530_e10077: f64 = (1.0 / assign14530_e10076);
        (assign14530_e10077, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign14530_e10079;
        locals.var_t2__blk1146_dn3 = assign14530_e10079_d_n3;
        locals.var_t2__blk1146_dn4 = assign14530_e10079_d_n4;
        locals.var_t2__blk1146_dn5 = assign14530_e10079_d_n5;
        locals.var_t2__blk1146_dn6 = assign14530_e10079_d_n6;
        locals.var_t2__blk1146_dn7 = assign14530_e10079_d_n7;
        locals.var_t2__blk1146_dn8 = assign14530_e10079_d_n8;
        locals.var_t2__blk1146_dn9 = assign14530_e10079_d_n9;
        locals.var_t2__blk1146_dn10 = assign14530_e10079_d_n10;
        locals.var_t2__blk1146_dn11 = assign14530_e10079_d_n11;
        locals.var_t2__blk1146_dn12 = assign14530_e10079_d_n12;

    }

    pub(super) fn stamp_transient_block_28(
        locals: &mut StampLocals,
    ) {
        let (assign14540_e10087, assign14540_e10087_d_n3, assign14540_e10087_d_n4, assign14540_e10087_d_n5, assign14540_e10087_d_n6, assign14540_e10087_d_n7, assign14540_e10087_d_n8, assign14540_e10087_d_n9, assign14540_e10087_d_n10, assign14540_e10087_d_n11, assign14540_e10087_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 == 0.0)) {
        let assign14540_e10085: f64 = (locals.var_devtemp).sqrt();
        (assign14540_e10085, 0.0, 0.0, 0.0, (locals.var_devtemp_dn6 / (2.0 * assign14540_e10085)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk1149, locals.var_t5__blk1149_dn3, locals.var_t5__blk1149_dn4, locals.var_t5__blk1149_dn5, locals.var_t5__blk1149_dn6, locals.var_t5__blk1149_dn7, locals.var_t5__blk1149_dn8, locals.var_t5__blk1149_dn9, locals.var_t5__blk1149_dn10, locals.var_t5__blk1149_dn11, locals.var_t5__blk1149_dn12,)
    }
};
        locals.var_t5__blk1149 = assign14540_e10087;
        locals.var_t5__blk1149_dn3 = assign14540_e10087_d_n3;
        locals.var_t5__blk1149_dn4 = assign14540_e10087_d_n4;
        locals.var_t5__blk1149_dn5 = assign14540_e10087_d_n5;
        locals.var_t5__blk1149_dn6 = assign14540_e10087_d_n6;
        locals.var_t5__blk1149_dn7 = assign14540_e10087_d_n7;
        locals.var_t5__blk1149_dn8 = assign14540_e10087_d_n8;
        locals.var_t5__blk1149_dn9 = assign14540_e10087_d_n9;
        locals.var_t5__blk1149_dn10 = assign14540_e10087_d_n10;
        locals.var_t5__blk1149_dn11 = assign14540_e10087_d_n11;
        locals.var_t5__blk1149_dn12 = assign14540_e10087_d_n12;

        let (assign14550_e10100, assign14550_e10100_d_n3, assign14550_e10100_d_n4, assign14550_e10100_d_n5, assign14550_e10100_d_n6, assign14550_e10100_d_n7, assign14550_e10100_d_n8, assign14550_e10100_d_n9, assign14550_e10100_d_n10, assign14550_e10100_d_n11, assign14550_e10100_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 == 0.0)) {
        let assign14550_e10094: f64 = (locals.var_b4soini0sub * locals.var_devtemp);
        let assign14550_e10096: f64 = (assign14550_e10094 * locals.var_t5__blk1149);
        let assign14550_e10098: f64 = (assign14550_e10096 * locals.var_t2__blk1146);
        (assign14550_e10098, (((assign14550_e10094 * locals.var_t5__blk1149_dn3) * locals.var_t2__blk1146) + (assign14550_e10096 * locals.var_t2__blk1146_dn3)), (((assign14550_e10094 * locals.var_t5__blk1149_dn4) * locals.var_t2__blk1146) + (assign14550_e10096 * locals.var_t2__blk1146_dn4)), (((assign14550_e10094 * locals.var_t5__blk1149_dn5) * locals.var_t2__blk1146) + (assign14550_e10096 * locals.var_t2__blk1146_dn5)), (((((locals.var_b4soini0sub * locals.var_devtemp_dn6) * locals.var_t5__blk1149) + (assign14550_e10094 * locals.var_t5__blk1149_dn6)) * locals.var_t2__blk1146) + (assign14550_e10096 * locals.var_t2__blk1146_dn6)), (((assign14550_e10094 * locals.var_t5__blk1149_dn7) * locals.var_t2__blk1146) + (assign14550_e10096 * locals.var_t2__blk1146_dn7)), (((assign14550_e10094 * locals.var_t5__blk1149_dn8) * locals.var_t2__blk1146) + (assign14550_e10096 * locals.var_t2__blk1146_dn8)), (((assign14550_e10094 * locals.var_t5__blk1149_dn9) * locals.var_t2__blk1146) + (assign14550_e10096 * locals.var_t2__blk1146_dn9)), (((assign14550_e10094 * locals.var_t5__blk1149_dn10) * locals.var_t2__blk1146) + (assign14550_e10096 * locals.var_t2__blk1146_dn10)), (((assign14550_e10094 * locals.var_t5__blk1149_dn11) * locals.var_t2__blk1146) + (assign14550_e10096 * locals.var_t2__blk1146_dn11)), (((assign14550_e10094 * locals.var_t5__blk1149_dn12) * locals.var_t2__blk1146) + (assign14550_e10096 * locals.var_t2__blk1146_dn12)),)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign14550_e10100;
        locals.var_t3__blk1147_dn3 = assign14550_e10100_d_n3;
        locals.var_t3__blk1147_dn4 = assign14550_e10100_d_n4;
        locals.var_t3__blk1147_dn5 = assign14550_e10100_d_n5;
        locals.var_t3__blk1147_dn6 = assign14550_e10100_d_n6;
        locals.var_t3__blk1147_dn7 = assign14550_e10100_d_n7;
        locals.var_t3__blk1147_dn8 = assign14550_e10100_d_n8;
        locals.var_t3__blk1147_dn9 = assign14550_e10100_d_n9;
        locals.var_t3__blk1147_dn10 = assign14550_e10100_d_n10;
        locals.var_t3__blk1147_dn11 = assign14550_e10100_d_n11;
        locals.var_t3__blk1147_dn12 = assign14550_e10100_d_n12;

        let (assign14560_e10118, assign14560_e10118_d_n3, assign14560_e10118_d_n4, assign14560_e10118_d_n5, assign14560_e10118_d_n6, assign14560_e10118_d_n7, assign14560_e10118_d_n8, assign14560_e10118_d_n9, assign14560_e10118_d_n10, assign14560_e10118_d_n11, assign14560_e10118_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 == 0.0)) {
        let assign14560_e10108: f64 = (2.0 * locals.var_vtm0__blk1402);
        let assign14560_e10109: f64 = (locals.var_eg0__blk1401 / assign14560_e10108);
        let assign14560_e10113: f64 = (2.0 * locals.var_vtm);
        let assign14560_e10114: f64 = (locals.var_eg__blk1212 / assign14560_e10113);
        let assign14560_e10115: f64 = (assign14560_e10109 - assign14560_e10114);
        let assign14560_e10116: f64 = (assign14560_e10115).exp();
        (assign14560_e10116, (assign14560_e10116 * (-(locals.var_eg__blk1212_dn3 / assign14560_e10113))), (assign14560_e10116 * (-(locals.var_eg__blk1212_dn4 / assign14560_e10113))), (assign14560_e10116 * (-(locals.var_eg__blk1212_dn5 / assign14560_e10113))), (assign14560_e10116 * (-(((locals.var_eg__blk1212_dn6 * assign14560_e10113) - (locals.var_eg__blk1212 * (2.0 * locals.var_vtm_dn6))) / (assign14560_e10113 * assign14560_e10113)))), (assign14560_e10116 * (-(locals.var_eg__blk1212_dn7 / assign14560_e10113))), (assign14560_e10116 * (-(locals.var_eg__blk1212_dn8 / assign14560_e10113))), (assign14560_e10116 * (-(locals.var_eg__blk1212_dn9 / assign14560_e10113))), (assign14560_e10116 * (-(locals.var_eg__blk1212_dn10 / assign14560_e10113))), (assign14560_e10116 * (-(locals.var_eg__blk1212_dn11 / assign14560_e10113))), (assign14560_e10116 * (-(locals.var_eg__blk1212_dn12 / assign14560_e10113))),)
    } else {
        (locals.var_t4__blk1148, locals.var_t4__blk1148_dn3, locals.var_t4__blk1148_dn4, locals.var_t4__blk1148_dn5, locals.var_t4__blk1148_dn6, locals.var_t4__blk1148_dn7, locals.var_t4__blk1148_dn8, locals.var_t4__blk1148_dn9, locals.var_t4__blk1148_dn10, locals.var_t4__blk1148_dn11, locals.var_t4__blk1148_dn12,)
    }
};
        locals.var_t4__blk1148 = assign14560_e10118;
        locals.var_t4__blk1148_dn3 = assign14560_e10118_d_n3;
        locals.var_t4__blk1148_dn4 = assign14560_e10118_d_n4;
        locals.var_t4__blk1148_dn5 = assign14560_e10118_d_n5;
        locals.var_t4__blk1148_dn6 = assign14560_e10118_d_n6;
        locals.var_t4__blk1148_dn7 = assign14560_e10118_d_n7;
        locals.var_t4__blk1148_dn8 = assign14560_e10118_d_n8;
        locals.var_t4__blk1148_dn9 = assign14560_e10118_d_n9;
        locals.var_t4__blk1148_dn10 = assign14560_e10118_d_n10;
        locals.var_t4__blk1148_dn11 = assign14560_e10118_d_n11;
        locals.var_t4__blk1148_dn12 = assign14560_e10118_d_n12;

        let (assign14570_e10127, assign14570_e10127_d_n3, assign14570_e10127_d_n4, assign14570_e10127_d_n5, assign14570_e10127_d_n6, assign14570_e10127_d_n7, assign14570_e10127_d_n8, assign14570_e10127_d_n9, assign14570_e10127_d_n10, assign14570_e10127_d_n11, assign14570_e10127_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 == 0.0)) {
        let assign14570_e10125: f64 = (locals.var_t3__blk1147 * locals.var_t4__blk1148);
        (assign14570_e10125, ((locals.var_t3__blk1147_dn3 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn3)), ((locals.var_t3__blk1147_dn4 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn4)), ((locals.var_t3__blk1147_dn5 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn5)), ((locals.var_t3__blk1147_dn6 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn6)), ((locals.var_t3__blk1147_dn7 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn7)), ((locals.var_t3__blk1147_dn8 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn8)), ((locals.var_t3__blk1147_dn9 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn9)), ((locals.var_t3__blk1147_dn10 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn10)), ((locals.var_t3__blk1147_dn11 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn11)), ((locals.var_t3__blk1147_dn12 * locals.var_t4__blk1148) + (locals.var_t3__blk1147 * locals.var_t4__blk1148_dn12)),)
    } else {
        (locals.var_ni__blk1211, locals.var_ni__blk1211_dn3, locals.var_ni__blk1211_dn4, locals.var_ni__blk1211_dn5, locals.var_ni__blk1211_dn6, locals.var_ni__blk1211_dn7, locals.var_ni__blk1211_dn8, locals.var_ni__blk1211_dn9, locals.var_ni__blk1211_dn10, locals.var_ni__blk1211_dn11, locals.var_ni__blk1211_dn12,)
    }
};
        locals.var_ni__blk1211 = assign14570_e10127;
        locals.var_ni__blk1211_dn3 = assign14570_e10127_d_n3;
        locals.var_ni__blk1211_dn4 = assign14570_e10127_d_n4;
        locals.var_ni__blk1211_dn5 = assign14570_e10127_d_n5;
        locals.var_ni__blk1211_dn6 = assign14570_e10127_d_n6;
        locals.var_ni__blk1211_dn7 = assign14570_e10127_d_n7;
        locals.var_ni__blk1211_dn8 = assign14570_e10127_d_n8;
        locals.var_ni__blk1211_dn9 = assign14570_e10127_d_n9;
        locals.var_ni__blk1211_dn10 = assign14570_e10127_d_n10;
        locals.var_ni__blk1211_dn11 = assign14570_e10127_d_n11;
        locals.var_ni__blk1211_dn12 = assign14570_e10127_d_n12;

        let (assign14580_e10153, assign14580_e10153_d_n3, assign14580_e10153_d_n4, assign14580_e10153_d_n5, assign14580_e10153_d_n6, assign14580_e10153_d_n7, assign14580_e10153_d_n8, assign14580_e10153_d_n9, assign14580_e10153_d_n10, assign14580_e10153_d_n11, assign14580_e10153_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 == 0.0)) {
        let assign14580_e10134: f64 = (1e20 * locals.var_pparam_b4soinpeak);
        let assign14580_e10137: f64 = (locals.var_ni__blk1211 * locals.var_ni__blk1211);
        let assign14580_e10138: f64 = (assign14580_e10134 / assign14580_e10137);
        let (assign14580_e10151, assign14580_e10151_d_n3, assign14580_e10151_d_n4, assign14580_e10151_d_n5, assign14580_e10151_d_n6, assign14580_e10151_d_n7, assign14580_e10151_d_n8, assign14580_e10151_d_n9, assign14580_e10151_d_n10, assign14580_e10151_d_n11, assign14580_e10151_d_n12,) = {
            if (assign14580_e10138 > 1e-38) {
                let assign14580_e10143: f64 = (1e20 * locals.var_pparam_b4soinpeak);
                let assign14580_e10146: f64 = (locals.var_ni__blk1211 * locals.var_ni__blk1211);
                let assign14580_e10147: f64 = (assign14580_e10143 / assign14580_e10146);
                let assign14580_e10148: f64 = (assign14580_e10147).ln();
                (assign14580_e10148, (((((1e20 * locals.var_pparam_b4soinpeak_dn3) * assign14580_e10146) - (assign14580_e10143 * ((locals.var_ni__blk1211_dn3 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn3)))) / (assign14580_e10146 * assign14580_e10146)) / assign14580_e10147), (((((1e20 * locals.var_pparam_b4soinpeak_dn4) * assign14580_e10146) - (assign14580_e10143 * ((locals.var_ni__blk1211_dn4 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn4)))) / (assign14580_e10146 * assign14580_e10146)) / assign14580_e10147), (((((1e20 * locals.var_pparam_b4soinpeak_dn5) * assign14580_e10146) - (assign14580_e10143 * ((locals.var_ni__blk1211_dn5 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn5)))) / (assign14580_e10146 * assign14580_e10146)) / assign14580_e10147), (((((1e20 * locals.var_pparam_b4soinpeak_dn6) * assign14580_e10146) - (assign14580_e10143 * ((locals.var_ni__blk1211_dn6 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn6)))) / (assign14580_e10146 * assign14580_e10146)) / assign14580_e10147), (((((1e20 * locals.var_pparam_b4soinpeak_dn7) * assign14580_e10146) - (assign14580_e10143 * ((locals.var_ni__blk1211_dn7 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn7)))) / (assign14580_e10146 * assign14580_e10146)) / assign14580_e10147), (((((1e20 * locals.var_pparam_b4soinpeak_dn8) * assign14580_e10146) - (assign14580_e10143 * ((locals.var_ni__blk1211_dn8 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn8)))) / (assign14580_e10146 * assign14580_e10146)) / assign14580_e10147), (((((1e20 * locals.var_pparam_b4soinpeak_dn9) * assign14580_e10146) - (assign14580_e10143 * ((locals.var_ni__blk1211_dn9 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn9)))) / (assign14580_e10146 * assign14580_e10146)) / assign14580_e10147), (((((1e20 * locals.var_pparam_b4soinpeak_dn10) * assign14580_e10146) - (assign14580_e10143 * ((locals.var_ni__blk1211_dn10 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn10)))) / (assign14580_e10146 * assign14580_e10146)) / assign14580_e10147), (((((1e20 * locals.var_pparam_b4soinpeak_dn11) * assign14580_e10146) - (assign14580_e10143 * ((locals.var_ni__blk1211_dn11 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn11)))) / (assign14580_e10146 * assign14580_e10146)) / assign14580_e10147), (((((1e20 * locals.var_pparam_b4soinpeak_dn12) * assign14580_e10146) - (assign14580_e10143 * ((locals.var_ni__blk1211_dn12 * locals.var_ni__blk1211) + (locals.var_ni__blk1211 * locals.var_ni__blk1211_dn12)))) / (assign14580_e10146 * assign14580_e10146)) / assign14580_e10147),)
            } else {
                let assign14580_e10150: f64 = (-87.49823353377374);
                (assign14580_e10150, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign14580_e10151, assign14580_e10151_d_n3, assign14580_e10151_d_n4, assign14580_e10151_d_n5, assign14580_e10151_d_n6, assign14580_e10151_d_n7, assign14580_e10151_d_n8, assign14580_e10151_d_n9, assign14580_e10151_d_n10, assign14580_e10151_d_n11, assign14580_e10151_d_n12,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign14580_e10153;
        locals.var_t0__blk1144_dn3 = assign14580_e10153_d_n3;
        locals.var_t0__blk1144_dn4 = assign14580_e10153_d_n4;
        locals.var_t0__blk1144_dn5 = assign14580_e10153_d_n5;
        locals.var_t0__blk1144_dn6 = assign14580_e10153_d_n6;
        locals.var_t0__blk1144_dn7 = assign14580_e10153_d_n7;
        locals.var_t0__blk1144_dn8 = assign14580_e10153_d_n8;
        locals.var_t0__blk1144_dn9 = assign14580_e10153_d_n9;
        locals.var_t0__blk1144_dn10 = assign14580_e10153_d_n10;
        locals.var_t0__blk1144_dn11 = assign14580_e10153_d_n11;
        locals.var_t0__blk1144_dn12 = assign14580_e10153_d_n12;

        let (assign14590_e10162, assign14590_e10162_d_n3, assign14590_e10162_d_n4, assign14590_e10162_d_n5, assign14590_e10162_d_n6, assign14590_e10162_d_n7, assign14590_e10162_d_n8, assign14590_e10162_d_n9, assign14590_e10162_d_n10, assign14590_e10162_d_n11, assign14590_e10162_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1458 == 0.0)) {
        let assign14590_e10160: f64 = (locals.var_vtm * locals.var_t0__blk1144);
        (assign14590_e10160, (locals.var_vtm * locals.var_t0__blk1144_dn3), (locals.var_vtm * locals.var_t0__blk1144_dn4), (locals.var_vtm * locals.var_t0__blk1144_dn5), ((locals.var_vtm_dn6 * locals.var_t0__blk1144) + (locals.var_vtm * locals.var_t0__blk1144_dn6)), (locals.var_vtm * locals.var_t0__blk1144_dn7), (locals.var_vtm * locals.var_t0__blk1144_dn8), (locals.var_vtm * locals.var_t0__blk1144_dn9), (locals.var_vtm * locals.var_t0__blk1144_dn10), (locals.var_vtm * locals.var_t0__blk1144_dn11), (locals.var_vtm * locals.var_t0__blk1144_dn12),)
    } else {
        (locals.var_vbi, locals.var_vbi_dn3, locals.var_vbi_dn4, locals.var_vbi_dn5, locals.var_vbi_dn6, locals.var_vbi_dn7, locals.var_vbi_dn8, locals.var_vbi_dn9, locals.var_vbi_dn10, locals.var_vbi_dn11, locals.var_vbi_dn12,)
    }
};
        locals.var_vbi = assign14590_e10162;
        locals.var_vbi_dn3 = assign14590_e10162_d_n3;
        locals.var_vbi_dn4 = assign14590_e10162_d_n4;
        locals.var_vbi_dn5 = assign14590_e10162_d_n5;
        locals.var_vbi_dn6 = assign14590_e10162_d_n6;
        locals.var_vbi_dn7 = assign14590_e10162_d_n7;
        locals.var_vbi_dn8 = assign14590_e10162_d_n8;
        locals.var_vbi_dn9 = assign14590_e10162_d_n9;
        locals.var_vbi_dn10 = assign14590_e10162_d_n10;
        locals.var_vbi_dn11 = assign14590_e10162_d_n11;
        locals.var_vbi_dn12 = assign14590_e10162_d_n12;

        let assign14600_e10165: f64 = if locals.var_pparam_b4soinsub > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1460 = assign14600_e10165;

        let (assign14610_e10182, assign14610_e10182_d_n3, assign14610_e10182_d_n4, assign14610_e10182_d_n5, assign14610_e10182_d_n6, assign14610_e10182_d_n7, assign14610_e10182_d_n8, assign14610_e10182_d_n9, assign14610_e10182_d_n10, assign14610_e10182_d_n11, assign14610_e10182_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1460 != 0.0)) {
        let assign14610_e10171: f64 = (locals.var_pparam_b4soinpeak / locals.var_pparam_b4soinsub);
        let (assign14610_e10180, assign14610_e10180_d_n3, assign14610_e10180_d_n4, assign14610_e10180_d_n5, assign14610_e10180_d_n6, assign14610_e10180_d_n7, assign14610_e10180_d_n8, assign14610_e10180_d_n9, assign14610_e10180_d_n10, assign14610_e10180_d_n11, assign14610_e10180_d_n12,) = {
            if (assign14610_e10171 > 1e-38) {
                let assign14610_e10176: f64 = (locals.var_pparam_b4soinpeak / locals.var_pparam_b4soinsub);
                let assign14610_e10177: f64 = (assign14610_e10176).ln();
                (assign14610_e10177, ((((locals.var_pparam_b4soinpeak_dn3 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn3)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign14610_e10176), ((((locals.var_pparam_b4soinpeak_dn4 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn4)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign14610_e10176), ((((locals.var_pparam_b4soinpeak_dn5 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn5)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign14610_e10176), ((((locals.var_pparam_b4soinpeak_dn6 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn6)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign14610_e10176), ((((locals.var_pparam_b4soinpeak_dn7 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn7)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign14610_e10176), ((((locals.var_pparam_b4soinpeak_dn8 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn8)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign14610_e10176), ((((locals.var_pparam_b4soinpeak_dn9 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn9)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign14610_e10176), ((((locals.var_pparam_b4soinpeak_dn10 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn10)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign14610_e10176), ((((locals.var_pparam_b4soinpeak_dn11 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn11)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign14610_e10176), ((((locals.var_pparam_b4soinpeak_dn12 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn12)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign14610_e10176),)
            } else {
                let assign14610_e10179: f64 = (-87.49823353377374);
                (assign14610_e10179, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign14610_e10180, assign14610_e10180_d_n3, assign14610_e10180_d_n4, assign14610_e10180_d_n5, assign14610_e10180_d_n6, assign14610_e10180_d_n7, assign14610_e10180_d_n8, assign14610_e10180_d_n9, assign14610_e10180_d_n10, assign14610_e10180_d_n11, assign14610_e10180_d_n12,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign14610_e10182;
        locals.var_t0__blk1144_dn3 = assign14610_e10182_d_n3;
        locals.var_t0__blk1144_dn4 = assign14610_e10182_d_n4;
        locals.var_t0__blk1144_dn5 = assign14610_e10182_d_n5;
        locals.var_t0__blk1144_dn6 = assign14610_e10182_d_n6;
        locals.var_t0__blk1144_dn7 = assign14610_e10182_d_n7;
        locals.var_t0__blk1144_dn8 = assign14610_e10182_d_n8;
        locals.var_t0__blk1144_dn9 = assign14610_e10182_d_n9;
        locals.var_t0__blk1144_dn10 = assign14610_e10182_d_n10;
        locals.var_t0__blk1144_dn11 = assign14610_e10182_d_n11;
        locals.var_t0__blk1144_dn12 = assign14610_e10182_d_n12;

        let (assign14620_e10193, assign14620_e10193_d_n3, assign14620_e10193_d_n4, assign14620_e10193_d_n5, assign14620_e10193_d_n6, assign14620_e10193_d_n7, assign14620_e10193_d_n8, assign14620_e10193_d_n9, assign14620_e10193_d_n10, assign14620_e10193_d_n11, assign14620_e10193_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1460 != 0.0)) {
        let assign14620_e10187: f64 = (-locals.var_b4soitype);
        let assign14620_e10189: f64 = (assign14620_e10187 * locals.var_vtm);
        let assign14620_e10191: f64 = (assign14620_e10189 * locals.var_t0__blk1144);
        (assign14620_e10191, (assign14620_e10189 * locals.var_t0__blk1144_dn3), (assign14620_e10189 * locals.var_t0__blk1144_dn4), (assign14620_e10189 * locals.var_t0__blk1144_dn5), (((assign14620_e10187 * locals.var_vtm_dn6) * locals.var_t0__blk1144) + (assign14620_e10189 * locals.var_t0__blk1144_dn6)), (assign14620_e10189 * locals.var_t0__blk1144_dn7), (assign14620_e10189 * locals.var_t0__blk1144_dn8), (assign14620_e10189 * locals.var_t0__blk1144_dn9), (assign14620_e10189 * locals.var_t0__blk1144_dn10), (assign14620_e10189 * locals.var_t0__blk1144_dn11), (assign14620_e10189 * locals.var_t0__blk1144_dn12),)
    } else {
        (locals.var_vfbb, locals.var_vfbb_dn3, locals.var_vfbb_dn4, locals.var_vfbb_dn5, locals.var_vfbb_dn6, locals.var_vfbb_dn7, locals.var_vfbb_dn8, locals.var_vfbb_dn9, locals.var_vfbb_dn10, locals.var_vfbb_dn11, locals.var_vfbb_dn12,)
    }
};
        locals.var_vfbb = assign14620_e10193;
        locals.var_vfbb_dn3 = assign14620_e10193_d_n3;
        locals.var_vfbb_dn4 = assign14620_e10193_d_n4;
        locals.var_vfbb_dn5 = assign14620_e10193_d_n5;
        locals.var_vfbb_dn6 = assign14620_e10193_d_n6;
        locals.var_vfbb_dn7 = assign14620_e10193_d_n7;
        locals.var_vfbb_dn8 = assign14620_e10193_d_n8;
        locals.var_vfbb_dn9 = assign14620_e10193_d_n9;
        locals.var_vfbb_dn10 = assign14620_e10193_d_n10;
        locals.var_vfbb_dn11 = assign14620_e10193_d_n11;
        locals.var_vfbb_dn12 = assign14620_e10193_d_n12;

        let (assign14630_e10221, assign14630_e10221_d_n3, assign14630_e10221_d_n4, assign14630_e10221_d_n5, assign14630_e10221_d_n6, assign14630_e10221_d_n7, assign14630_e10221_d_n8, assign14630_e10221_d_n9, assign14630_e10221_d_n10, assign14630_e10221_d_n11, assign14630_e10221_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1460 == 0.0)) {
        let assign14630_e10199: f64 = (-locals.var_pparam_b4soinpeak);
        let assign14630_e10201: f64 = (assign14630_e10199 * locals.var_pparam_b4soinsub);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_ni__blk1211;
        let assign14630_e10203: f64 = (assign14630_e10201 * __rspice_inv_cse_0);
        let assign14630_e10205: f64 = (assign14630_e10203 * __rspice_inv_cse_0);
        let (assign14630_e10219, assign14630_e10219_d_n3, assign14630_e10219_d_n4, assign14630_e10219_d_n5, assign14630_e10219_d_n6, assign14630_e10219_d_n7, assign14630_e10219_d_n8, assign14630_e10219_d_n9, assign14630_e10219_d_n10, assign14630_e10219_d_n11, assign14630_e10219_d_n12,) = {
            if (assign14630_e10205 > 1e-38) {
                let assign14630_e10209: f64 = (-locals.var_pparam_b4soinpeak);
                let assign14630_e10211: f64 = (assign14630_e10209 * locals.var_pparam_b4soinsub);
                let __rspice_inv_cse_1: f64 = 1.0 / locals.var_ni__blk1211;
                let assign14630_e10213: f64 = (assign14630_e10211 * __rspice_inv_cse_1);
                let assign14630_e10215: f64 = (assign14630_e10213 * __rspice_inv_cse_1);
                let assign14630_e10216: f64 = (assign14630_e10215).ln();
                (assign14630_e10216, ((((((((((-locals.var_pparam_b4soinpeak_dn3) * locals.var_pparam_b4soinsub) + (assign14630_e10209 * locals.var_pparam_b4soinsub_dn3)) * locals.var_ni__blk1211) - (assign14630_e10211 * locals.var_ni__blk1211_dn3)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) * locals.var_ni__blk1211) - (assign14630_e10213 * locals.var_ni__blk1211_dn3)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14630_e10215), ((((((((((-locals.var_pparam_b4soinpeak_dn4) * locals.var_pparam_b4soinsub) + (assign14630_e10209 * locals.var_pparam_b4soinsub_dn4)) * locals.var_ni__blk1211) - (assign14630_e10211 * locals.var_ni__blk1211_dn4)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) * locals.var_ni__blk1211) - (assign14630_e10213 * locals.var_ni__blk1211_dn4)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14630_e10215), ((((((((((-locals.var_pparam_b4soinpeak_dn5) * locals.var_pparam_b4soinsub) + (assign14630_e10209 * locals.var_pparam_b4soinsub_dn5)) * locals.var_ni__blk1211) - (assign14630_e10211 * locals.var_ni__blk1211_dn5)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) * locals.var_ni__blk1211) - (assign14630_e10213 * locals.var_ni__blk1211_dn5)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14630_e10215), ((((((((((-locals.var_pparam_b4soinpeak_dn6) * locals.var_pparam_b4soinsub) + (assign14630_e10209 * locals.var_pparam_b4soinsub_dn6)) * locals.var_ni__blk1211) - (assign14630_e10211 * locals.var_ni__blk1211_dn6)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) * locals.var_ni__blk1211) - (assign14630_e10213 * locals.var_ni__blk1211_dn6)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14630_e10215), ((((((((((-locals.var_pparam_b4soinpeak_dn7) * locals.var_pparam_b4soinsub) + (assign14630_e10209 * locals.var_pparam_b4soinsub_dn7)) * locals.var_ni__blk1211) - (assign14630_e10211 * locals.var_ni__blk1211_dn7)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) * locals.var_ni__blk1211) - (assign14630_e10213 * locals.var_ni__blk1211_dn7)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14630_e10215), ((((((((((-locals.var_pparam_b4soinpeak_dn8) * locals.var_pparam_b4soinsub) + (assign14630_e10209 * locals.var_pparam_b4soinsub_dn8)) * locals.var_ni__blk1211) - (assign14630_e10211 * locals.var_ni__blk1211_dn8)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) * locals.var_ni__blk1211) - (assign14630_e10213 * locals.var_ni__blk1211_dn8)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14630_e10215), ((((((((((-locals.var_pparam_b4soinpeak_dn9) * locals.var_pparam_b4soinsub) + (assign14630_e10209 * locals.var_pparam_b4soinsub_dn9)) * locals.var_ni__blk1211) - (assign14630_e10211 * locals.var_ni__blk1211_dn9)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) * locals.var_ni__blk1211) - (assign14630_e10213 * locals.var_ni__blk1211_dn9)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14630_e10215), ((((((((((-locals.var_pparam_b4soinpeak_dn10) * locals.var_pparam_b4soinsub) + (assign14630_e10209 * locals.var_pparam_b4soinsub_dn10)) * locals.var_ni__blk1211) - (assign14630_e10211 * locals.var_ni__blk1211_dn10)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) * locals.var_ni__blk1211) - (assign14630_e10213 * locals.var_ni__blk1211_dn10)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14630_e10215), ((((((((((-locals.var_pparam_b4soinpeak_dn11) * locals.var_pparam_b4soinsub) + (assign14630_e10209 * locals.var_pparam_b4soinsub_dn11)) * locals.var_ni__blk1211) - (assign14630_e10211 * locals.var_ni__blk1211_dn11)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) * locals.var_ni__blk1211) - (assign14630_e10213 * locals.var_ni__blk1211_dn11)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14630_e10215), ((((((((((-locals.var_pparam_b4soinpeak_dn12) * locals.var_pparam_b4soinsub) + (assign14630_e10209 * locals.var_pparam_b4soinsub_dn12)) * locals.var_ni__blk1211) - (assign14630_e10211 * locals.var_ni__blk1211_dn12)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) * locals.var_ni__blk1211) - (assign14630_e10213 * locals.var_ni__blk1211_dn12)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14630_e10215),)
            } else {
                let assign14630_e10218: f64 = (-87.49823353377374);
                (assign14630_e10218, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign14630_e10219, assign14630_e10219_d_n3, assign14630_e10219_d_n4, assign14630_e10219_d_n5, assign14630_e10219_d_n6, assign14630_e10219_d_n7, assign14630_e10219_d_n8, assign14630_e10219_d_n9, assign14630_e10219_d_n10, assign14630_e10219_d_n11, assign14630_e10219_d_n12,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign14630_e10221;
        locals.var_t0__blk1144_dn3 = assign14630_e10221_d_n3;
        locals.var_t0__blk1144_dn4 = assign14630_e10221_d_n4;
        locals.var_t0__blk1144_dn5 = assign14630_e10221_d_n5;
        locals.var_t0__blk1144_dn6 = assign14630_e10221_d_n6;
        locals.var_t0__blk1144_dn7 = assign14630_e10221_d_n7;
        locals.var_t0__blk1144_dn8 = assign14630_e10221_d_n8;
        locals.var_t0__blk1144_dn9 = assign14630_e10221_d_n9;
        locals.var_t0__blk1144_dn10 = assign14630_e10221_d_n10;
        locals.var_t0__blk1144_dn11 = assign14630_e10221_d_n11;
        locals.var_t0__blk1144_dn12 = assign14630_e10221_d_n12;

        let (assign14640_e10233, assign14640_e10233_d_n3, assign14640_e10233_d_n4, assign14640_e10233_d_n5, assign14640_e10233_d_n6, assign14640_e10233_d_n7, assign14640_e10233_d_n8, assign14640_e10233_d_n9, assign14640_e10233_d_n10, assign14640_e10233_d_n11, assign14640_e10233_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1460 == 0.0)) {
        let assign14640_e10227: f64 = (-locals.var_b4soitype);
        let assign14640_e10229: f64 = (assign14640_e10227 * locals.var_vtm);
        let assign14640_e10231: f64 = (assign14640_e10229 * locals.var_t0__blk1144);
        (assign14640_e10231, (assign14640_e10229 * locals.var_t0__blk1144_dn3), (assign14640_e10229 * locals.var_t0__blk1144_dn4), (assign14640_e10229 * locals.var_t0__blk1144_dn5), (((assign14640_e10227 * locals.var_vtm_dn6) * locals.var_t0__blk1144) + (assign14640_e10229 * locals.var_t0__blk1144_dn6)), (assign14640_e10229 * locals.var_t0__blk1144_dn7), (assign14640_e10229 * locals.var_t0__blk1144_dn8), (assign14640_e10229 * locals.var_t0__blk1144_dn9), (assign14640_e10229 * locals.var_t0__blk1144_dn10), (assign14640_e10229 * locals.var_t0__blk1144_dn11), (assign14640_e10229 * locals.var_t0__blk1144_dn12),)
    } else {
        (locals.var_vfbb, locals.var_vfbb_dn3, locals.var_vfbb_dn4, locals.var_vfbb_dn5, locals.var_vfbb_dn6, locals.var_vfbb_dn7, locals.var_vfbb_dn8, locals.var_vfbb_dn9, locals.var_vfbb_dn10, locals.var_vfbb_dn11, locals.var_vfbb_dn12,)
    }
};
        locals.var_vfbb = assign14640_e10233;
        locals.var_vfbb_dn3 = assign14640_e10233_d_n3;
        locals.var_vfbb_dn4 = assign14640_e10233_d_n4;
        locals.var_vfbb_dn5 = assign14640_e10233_d_n5;
        locals.var_vfbb_dn6 = assign14640_e10233_d_n6;
        locals.var_vfbb_dn7 = assign14640_e10233_d_n7;
        locals.var_vfbb_dn8 = assign14640_e10233_d_n8;
        locals.var_vfbb_dn9 = assign14640_e10233_d_n9;
        locals.var_vfbb_dn10 = assign14640_e10233_d_n10;
        locals.var_vfbb_dn11 = assign14640_e10233_d_n11;
        locals.var_vfbb_dn12 = assign14640_e10233_d_n12;

        let (assign14650_e10252, assign14650_e10252_d_n3, assign14650_e10252_d_n4, assign14650_e10252_d_n5, assign14650_e10252_d_n6, assign14650_e10252_d_n7, assign14650_e10252_d_n8, assign14650_e10252_d_n9, assign14650_e10252_d_n10, assign14650_e10252_d_n11, assign14650_e10252_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign14650_e10237: f64 = (2.0 * locals.var_vtm);
        let assign14650_e10240: f64 = (locals.var_pparam_b4soinpeak / locals.var_ni__blk1211);
        let (assign14650_e10249, assign14650_e10249_d_n3, assign14650_e10249_d_n4, assign14650_e10249_d_n5, assign14650_e10249_d_n6, assign14650_e10249_d_n7, assign14650_e10249_d_n8, assign14650_e10249_d_n9, assign14650_e10249_d_n10, assign14650_e10249_d_n11, assign14650_e10249_d_n12,) = {
            if (assign14650_e10240 > 1e-38) {
                let assign14650_e10245: f64 = (locals.var_pparam_b4soinpeak / locals.var_ni__blk1211);
                let assign14650_e10246: f64 = (assign14650_e10245).ln();
                (assign14650_e10246, ((((locals.var_pparam_b4soinpeak_dn3 * locals.var_ni__blk1211) - (locals.var_pparam_b4soinpeak * locals.var_ni__blk1211_dn3)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14650_e10245), ((((locals.var_pparam_b4soinpeak_dn4 * locals.var_ni__blk1211) - (locals.var_pparam_b4soinpeak * locals.var_ni__blk1211_dn4)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14650_e10245), ((((locals.var_pparam_b4soinpeak_dn5 * locals.var_ni__blk1211) - (locals.var_pparam_b4soinpeak * locals.var_ni__blk1211_dn5)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14650_e10245), ((((locals.var_pparam_b4soinpeak_dn6 * locals.var_ni__blk1211) - (locals.var_pparam_b4soinpeak * locals.var_ni__blk1211_dn6)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14650_e10245), ((((locals.var_pparam_b4soinpeak_dn7 * locals.var_ni__blk1211) - (locals.var_pparam_b4soinpeak * locals.var_ni__blk1211_dn7)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14650_e10245), ((((locals.var_pparam_b4soinpeak_dn8 * locals.var_ni__blk1211) - (locals.var_pparam_b4soinpeak * locals.var_ni__blk1211_dn8)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14650_e10245), ((((locals.var_pparam_b4soinpeak_dn9 * locals.var_ni__blk1211) - (locals.var_pparam_b4soinpeak * locals.var_ni__blk1211_dn9)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14650_e10245), ((((locals.var_pparam_b4soinpeak_dn10 * locals.var_ni__blk1211) - (locals.var_pparam_b4soinpeak * locals.var_ni__blk1211_dn10)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14650_e10245), ((((locals.var_pparam_b4soinpeak_dn11 * locals.var_ni__blk1211) - (locals.var_pparam_b4soinpeak * locals.var_ni__blk1211_dn11)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14650_e10245), ((((locals.var_pparam_b4soinpeak_dn12 * locals.var_ni__blk1211) - (locals.var_pparam_b4soinpeak * locals.var_ni__blk1211_dn12)) / (locals.var_ni__blk1211 * locals.var_ni__blk1211)) / assign14650_e10245),)
            } else {
                let assign14650_e10248: f64 = (-87.49823353377374);
                (assign14650_e10248, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign14650_e10250: f64 = (assign14650_e10237 * assign14650_e10249);
        (assign14650_e10250, (assign14650_e10237 * assign14650_e10249_d_n3), (assign14650_e10237 * assign14650_e10249_d_n4), (assign14650_e10237 * assign14650_e10249_d_n5), (((2.0 * locals.var_vtm_dn6) * assign14650_e10249) + (assign14650_e10237 * assign14650_e10249_d_n6)), (assign14650_e10237 * assign14650_e10249_d_n7), (assign14650_e10237 * assign14650_e10249_d_n8), (assign14650_e10237 * assign14650_e10249_d_n9), (assign14650_e10237 * assign14650_e10249_d_n10), (assign14650_e10237 * assign14650_e10249_d_n11), (assign14650_e10237 * assign14650_e10249_d_n12),)
    } else {
        (locals.var_phi, locals.var_phi_dn3, locals.var_phi_dn4, locals.var_phi_dn5, locals.var_phi_dn6, locals.var_phi_dn7, locals.var_phi_dn8, locals.var_phi_dn9, locals.var_phi_dn10, locals.var_phi_dn11, locals.var_phi_dn12,)
    }
};
        locals.var_phi = assign14650_e10252;
        locals.var_phi_dn3 = assign14650_e10252_d_n3;
        locals.var_phi_dn4 = assign14650_e10252_d_n4;
        locals.var_phi_dn5 = assign14650_e10252_d_n5;
        locals.var_phi_dn6 = assign14650_e10252_d_n6;
        locals.var_phi_dn7 = assign14650_e10252_d_n7;
        locals.var_phi_dn8 = assign14650_e10252_d_n8;
        locals.var_phi_dn9 = assign14650_e10252_d_n9;
        locals.var_phi_dn10 = assign14650_e10252_d_n10;
        locals.var_phi_dn11 = assign14650_e10252_d_n11;
        locals.var_phi_dn12 = assign14650_e10252_d_n12;

        let (assign14660_e10257, assign14660_e10257_d_n3, assign14660_e10257_d_n4, assign14660_e10257_d_n5, assign14660_e10257_d_n6, assign14660_e10257_d_n7, assign14660_e10257_d_n8, assign14660_e10257_d_n9, assign14660_e10257_d_n10, assign14660_e10257_d_n11, assign14660_e10257_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign14660_e10255: f64 = (locals.var_phi).sqrt();
        (assign14660_e10255, (locals.var_phi_dn3 / (2.0 * assign14660_e10255)), (locals.var_phi_dn4 / (2.0 * assign14660_e10255)), (locals.var_phi_dn5 / (2.0 * assign14660_e10255)), (locals.var_phi_dn6 / (2.0 * assign14660_e10255)), (locals.var_phi_dn7 / (2.0 * assign14660_e10255)), (locals.var_phi_dn8 / (2.0 * assign14660_e10255)), (locals.var_phi_dn9 / (2.0 * assign14660_e10255)), (locals.var_phi_dn10 / (2.0 * assign14660_e10255)), (locals.var_phi_dn11 / (2.0 * assign14660_e10255)), (locals.var_phi_dn12 / (2.0 * assign14660_e10255)),)
    } else {
        (locals.var_sqrtphi, locals.var_sqrtphi_dn3, locals.var_sqrtphi_dn4, locals.var_sqrtphi_dn5, locals.var_sqrtphi_dn6, locals.var_sqrtphi_dn7, locals.var_sqrtphi_dn8, locals.var_sqrtphi_dn9, locals.var_sqrtphi_dn10, locals.var_sqrtphi_dn11, locals.var_sqrtphi_dn12,)
    }
};
        locals.var_sqrtphi = assign14660_e10257;
        locals.var_sqrtphi_dn3 = assign14660_e10257_d_n3;
        locals.var_sqrtphi_dn4 = assign14660_e10257_d_n4;
        locals.var_sqrtphi_dn5 = assign14660_e10257_d_n5;
        locals.var_sqrtphi_dn6 = assign14660_e10257_d_n6;
        locals.var_sqrtphi_dn7 = assign14660_e10257_d_n7;
        locals.var_sqrtphi_dn8 = assign14660_e10257_d_n8;
        locals.var_sqrtphi_dn9 = assign14660_e10257_d_n9;
        locals.var_sqrtphi_dn10 = assign14660_e10257_d_n10;
        locals.var_sqrtphi_dn11 = assign14660_e10257_d_n11;
        locals.var_sqrtphi_dn12 = assign14660_e10257_d_n12;

        let (assign14670_e10272, assign14670_e10272_d_n3, assign14670_e10272_d_n4, assign14670_e10272_d_n5, assign14670_e10272_d_n6, assign14670_e10272_d_n7, assign14670_e10272_d_n8, assign14670_e10272_d_n9, assign14670_e10272_d_n10, assign14670_e10272_d_n11, assign14670_e10272_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign14670_e10261: f64 = (2.0 * locals.var_epssub);
        let assign14670_e10264: f64 = (1.60219e-19 * locals.var_pparam_b4soinpeak);
        let assign14670_e10266: f64 = (assign14670_e10264 * 1000000.0);
        let assign14670_e10267: f64 = (assign14670_e10261 / assign14670_e10266);
        let assign14670_e10268: f64 = (assign14670_e10267).sqrt();
        let assign14670_e10270: f64 = (assign14670_e10268 * locals.var_sqrtphi);
        (assign14670_e10270, ((((-((assign14670_e10261 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn3) * 1000000.0)) / (assign14670_e10266 * assign14670_e10266))) / (2.0 * assign14670_e10268)) * locals.var_sqrtphi) + (assign14670_e10268 * locals.var_sqrtphi_dn3)), ((((-((assign14670_e10261 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn4) * 1000000.0)) / (assign14670_e10266 * assign14670_e10266))) / (2.0 * assign14670_e10268)) * locals.var_sqrtphi) + (assign14670_e10268 * locals.var_sqrtphi_dn4)), ((((-((assign14670_e10261 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn5) * 1000000.0)) / (assign14670_e10266 * assign14670_e10266))) / (2.0 * assign14670_e10268)) * locals.var_sqrtphi) + (assign14670_e10268 * locals.var_sqrtphi_dn5)), ((((-((assign14670_e10261 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn6) * 1000000.0)) / (assign14670_e10266 * assign14670_e10266))) / (2.0 * assign14670_e10268)) * locals.var_sqrtphi) + (assign14670_e10268 * locals.var_sqrtphi_dn6)), ((((-((assign14670_e10261 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn7) * 1000000.0)) / (assign14670_e10266 * assign14670_e10266))) / (2.0 * assign14670_e10268)) * locals.var_sqrtphi) + (assign14670_e10268 * locals.var_sqrtphi_dn7)), ((((-((assign14670_e10261 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn8) * 1000000.0)) / (assign14670_e10266 * assign14670_e10266))) / (2.0 * assign14670_e10268)) * locals.var_sqrtphi) + (assign14670_e10268 * locals.var_sqrtphi_dn8)), ((((-((assign14670_e10261 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn9) * 1000000.0)) / (assign14670_e10266 * assign14670_e10266))) / (2.0 * assign14670_e10268)) * locals.var_sqrtphi) + (assign14670_e10268 * locals.var_sqrtphi_dn9)), ((((-((assign14670_e10261 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn10) * 1000000.0)) / (assign14670_e10266 * assign14670_e10266))) / (2.0 * assign14670_e10268)) * locals.var_sqrtphi) + (assign14670_e10268 * locals.var_sqrtphi_dn10)), ((((-((assign14670_e10261 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn11) * 1000000.0)) / (assign14670_e10266 * assign14670_e10266))) / (2.0 * assign14670_e10268)) * locals.var_sqrtphi) + (assign14670_e10268 * locals.var_sqrtphi_dn11)), ((((-((assign14670_e10261 * ((1.60219e-19 * locals.var_pparam_b4soinpeak_dn12) * 1000000.0)) / (assign14670_e10266 * assign14670_e10266))) / (2.0 * assign14670_e10268)) * locals.var_sqrtphi) + (assign14670_e10268 * locals.var_sqrtphi_dn12)),)
    } else {
        (locals.var_xdep0, locals.var_xdep0_dn3, locals.var_xdep0_dn4, locals.var_xdep0_dn5, locals.var_xdep0_dn6, locals.var_xdep0_dn7, locals.var_xdep0_dn8, locals.var_xdep0_dn9, locals.var_xdep0_dn10, locals.var_xdep0_dn11, locals.var_xdep0_dn12,)
    }
};
        locals.var_xdep0 = assign14670_e10272;
        locals.var_xdep0_dn3 = assign14670_e10272_d_n3;
        locals.var_xdep0_dn4 = assign14670_e10272_d_n4;
        locals.var_xdep0_dn5 = assign14670_e10272_d_n5;
        locals.var_xdep0_dn6 = assign14670_e10272_d_n6;
        locals.var_xdep0_dn7 = assign14670_e10272_d_n7;
        locals.var_xdep0_dn8 = assign14670_e10272_d_n8;
        locals.var_xdep0_dn9 = assign14670_e10272_d_n9;
        locals.var_xdep0_dn10 = assign14670_e10272_d_n10;
        locals.var_xdep0_dn11 = assign14670_e10272_d_n11;
        locals.var_xdep0_dn12 = assign14670_e10272_d_n12;

        let (assign14680_e10287, assign14680_e10287_d_n3, assign14680_e10287_d_n4, assign14680_e10287_d_n5, assign14680_e10287_d_n6, assign14680_e10287_d_n7, assign14680_e10287_d_n8, assign14680_e10287_d_n9, assign14680_e10287_d_n10, assign14680_e10287_d_n11, assign14680_e10287_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign14680_e10276: f64 = (1.60219e-19 * locals.var_epssub);
        let assign14680_e10278: f64 = (assign14680_e10276 * locals.var_pparam_b4soinpeak);
        let assign14680_e10280: f64 = (assign14680_e10278 * 1000000.0);
        let assign14680_e10282: f64 = (assign14680_e10280 / 2.0);
        let assign14680_e10283: f64 = (assign14680_e10282).sqrt();
        let assign14680_e10285: f64 = (assign14680_e10283 / locals.var_sqrtphi);
        (assign14680_e10285, (((((((assign14680_e10276 * locals.var_pparam_b4soinpeak_dn3) * 1000000.0) / 2.0) / (2.0 * assign14680_e10283)) * locals.var_sqrtphi) - (assign14680_e10283 * locals.var_sqrtphi_dn3)) / (locals.var_sqrtphi * locals.var_sqrtphi)), (((((((assign14680_e10276 * locals.var_pparam_b4soinpeak_dn4) * 1000000.0) / 2.0) / (2.0 * assign14680_e10283)) * locals.var_sqrtphi) - (assign14680_e10283 * locals.var_sqrtphi_dn4)) / (locals.var_sqrtphi * locals.var_sqrtphi)), (((((((assign14680_e10276 * locals.var_pparam_b4soinpeak_dn5) * 1000000.0) / 2.0) / (2.0 * assign14680_e10283)) * locals.var_sqrtphi) - (assign14680_e10283 * locals.var_sqrtphi_dn5)) / (locals.var_sqrtphi * locals.var_sqrtphi)), (((((((assign14680_e10276 * locals.var_pparam_b4soinpeak_dn6) * 1000000.0) / 2.0) / (2.0 * assign14680_e10283)) * locals.var_sqrtphi) - (assign14680_e10283 * locals.var_sqrtphi_dn6)) / (locals.var_sqrtphi * locals.var_sqrtphi)), (((((((assign14680_e10276 * locals.var_pparam_b4soinpeak_dn7) * 1000000.0) / 2.0) / (2.0 * assign14680_e10283)) * locals.var_sqrtphi) - (assign14680_e10283 * locals.var_sqrtphi_dn7)) / (locals.var_sqrtphi * locals.var_sqrtphi)), (((((((assign14680_e10276 * locals.var_pparam_b4soinpeak_dn8) * 1000000.0) / 2.0) / (2.0 * assign14680_e10283)) * locals.var_sqrtphi) - (assign14680_e10283 * locals.var_sqrtphi_dn8)) / (locals.var_sqrtphi * locals.var_sqrtphi)), (((((((assign14680_e10276 * locals.var_pparam_b4soinpeak_dn9) * 1000000.0) / 2.0) / (2.0 * assign14680_e10283)) * locals.var_sqrtphi) - (assign14680_e10283 * locals.var_sqrtphi_dn9)) / (locals.var_sqrtphi * locals.var_sqrtphi)), (((((((assign14680_e10276 * locals.var_pparam_b4soinpeak_dn10) * 1000000.0) / 2.0) / (2.0 * assign14680_e10283)) * locals.var_sqrtphi) - (assign14680_e10283 * locals.var_sqrtphi_dn10)) / (locals.var_sqrtphi * locals.var_sqrtphi)), (((((((assign14680_e10276 * locals.var_pparam_b4soinpeak_dn11) * 1000000.0) / 2.0) / (2.0 * assign14680_e10283)) * locals.var_sqrtphi) - (assign14680_e10283 * locals.var_sqrtphi_dn11)) / (locals.var_sqrtphi * locals.var_sqrtphi)), (((((((assign14680_e10276 * locals.var_pparam_b4soinpeak_dn12) * 1000000.0) / 2.0) / (2.0 * assign14680_e10283)) * locals.var_sqrtphi) - (assign14680_e10283 * locals.var_sqrtphi_dn12)) / (locals.var_sqrtphi * locals.var_sqrtphi)),)
    } else {
        (locals.var_cdep0, locals.var_cdep0_dn3, locals.var_cdep0_dn4, locals.var_cdep0_dn5, locals.var_cdep0_dn6, locals.var_cdep0_dn7, locals.var_cdep0_dn8, locals.var_cdep0_dn9, locals.var_cdep0_dn10, locals.var_cdep0_dn11, locals.var_cdep0_dn12,)
    }
};
        locals.var_cdep0 = assign14680_e10287;
        locals.var_cdep0_dn3 = assign14680_e10287_d_n3;
        locals.var_cdep0_dn4 = assign14680_e10287_d_n4;
        locals.var_cdep0_dn5 = assign14680_e10287_d_n5;
        locals.var_cdep0_dn6 = assign14680_e10287_d_n6;
        locals.var_cdep0_dn7 = assign14680_e10287_d_n7;
        locals.var_cdep0_dn8 = assign14680_e10287_d_n8;
        locals.var_cdep0_dn9 = assign14680_e10287_d_n9;
        locals.var_cdep0_dn10 = assign14680_e10287_d_n10;
        locals.var_cdep0_dn11 = assign14680_e10287_d_n11;
        locals.var_cdep0_dn12 = assign14680_e10287_d_n12;

        let (assign14690_e10300, assign14690_e10300_d_n3, assign14690_e10300_d_n4, assign14690_e10300_d_n5, assign14690_e10300_d_n6, assign14690_e10300_d_n7, assign14690_e10300_d_n8, assign14690_e10300_d_n9, assign14690_e10300_d_n10, assign14690_e10300_d_n11, assign14690_e10300_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign14690_e10292: f64 = (locals.var_epsrox * 8.85418e-12);
        let assign14690_e10293: f64 = (locals.var_epssub / assign14690_e10292);
        let assign14690_e10295: f64 = (assign14690_e10293 * locals.var_toxe);
        let assign14690_e10297: f64 = (assign14690_e10295 * locals.var_xdep0);
        let assign14690_e10298: f64 = (assign14690_e10297).sqrt();
        (assign14690_e10298, ((assign14690_e10295 * locals.var_xdep0_dn3) / (2.0 * assign14690_e10298)), ((assign14690_e10295 * locals.var_xdep0_dn4) / (2.0 * assign14690_e10298)), ((assign14690_e10295 * locals.var_xdep0_dn5) / (2.0 * assign14690_e10298)), ((assign14690_e10295 * locals.var_xdep0_dn6) / (2.0 * assign14690_e10298)), ((assign14690_e10295 * locals.var_xdep0_dn7) / (2.0 * assign14690_e10298)), ((assign14690_e10295 * locals.var_xdep0_dn8) / (2.0 * assign14690_e10298)), ((assign14690_e10295 * locals.var_xdep0_dn9) / (2.0 * assign14690_e10298)), ((assign14690_e10295 * locals.var_xdep0_dn10) / (2.0 * assign14690_e10298)), ((assign14690_e10295 * locals.var_xdep0_dn11) / (2.0 * assign14690_e10298)), ((assign14690_e10295 * locals.var_xdep0_dn12) / (2.0 * assign14690_e10298)),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign14690_e10300;
        locals.var_t1__blk1145_dn3 = assign14690_e10300_d_n3;
        locals.var_t1__blk1145_dn4 = assign14690_e10300_d_n4;
        locals.var_t1__blk1145_dn5 = assign14690_e10300_d_n5;
        locals.var_t1__blk1145_dn6 = assign14690_e10300_d_n6;
        locals.var_t1__blk1145_dn7 = assign14690_e10300_d_n7;
        locals.var_t1__blk1145_dn8 = assign14690_e10300_d_n8;
        locals.var_t1__blk1145_dn9 = assign14690_e10300_d_n9;
        locals.var_t1__blk1145_dn10 = assign14690_e10300_d_n10;
        locals.var_t1__blk1145_dn11 = assign14690_e10300_d_n11;
        locals.var_t1__blk1145_dn12 = assign14690_e10300_d_n12;

        let (assign14700_e10312, assign14700_e10312_d_n3, assign14700_e10312_d_n4, assign14700_e10312_d_n5, assign14700_e10312_d_n6, assign14700_e10312_d_n7, assign14700_e10312_d_n8, assign14700_e10312_d_n9, assign14700_e10312_d_n10, assign14700_e10312_d_n11, assign14700_e10312_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign14700_e10303: f64 = (-0.5);
        let assign14700_e10305: f64 = (assign14700_e10303 * locals.var_pparam_b4soidsub);
        let assign14700_e10307: f64 = (assign14700_e10305 * locals.var_pparam_b4soileff);
        let assign14700_e10309: f64 = (assign14700_e10307 / locals.var_t1__blk1145);
        let assign14700_e10310: f64 = (assign14700_e10309).exp();
        (assign14700_e10310, (assign14700_e10310 * ((((((assign14700_e10303 * locals.var_pparam_b4soidsub_dn3) * locals.var_pparam_b4soileff) + (assign14700_e10305 * locals.var_pparam_b4soileff_dn3)) * locals.var_t1__blk1145) - (assign14700_e10307 * locals.var_t1__blk1145_dn3)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (assign14700_e10310 * ((((((assign14700_e10303 * locals.var_pparam_b4soidsub_dn4) * locals.var_pparam_b4soileff) + (assign14700_e10305 * locals.var_pparam_b4soileff_dn4)) * locals.var_t1__blk1145) - (assign14700_e10307 * locals.var_t1__blk1145_dn4)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (assign14700_e10310 * ((((((assign14700_e10303 * locals.var_pparam_b4soidsub_dn5) * locals.var_pparam_b4soileff) + (assign14700_e10305 * locals.var_pparam_b4soileff_dn5)) * locals.var_t1__blk1145) - (assign14700_e10307 * locals.var_t1__blk1145_dn5)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (assign14700_e10310 * ((((((assign14700_e10303 * locals.var_pparam_b4soidsub_dn6) * locals.var_pparam_b4soileff) + (assign14700_e10305 * locals.var_pparam_b4soileff_dn6)) * locals.var_t1__blk1145) - (assign14700_e10307 * locals.var_t1__blk1145_dn6)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (assign14700_e10310 * ((((((assign14700_e10303 * locals.var_pparam_b4soidsub_dn7) * locals.var_pparam_b4soileff) + (assign14700_e10305 * locals.var_pparam_b4soileff_dn7)) * locals.var_t1__blk1145) - (assign14700_e10307 * locals.var_t1__blk1145_dn7)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (assign14700_e10310 * ((((((assign14700_e10303 * locals.var_pparam_b4soidsub_dn8) * locals.var_pparam_b4soileff) + (assign14700_e10305 * locals.var_pparam_b4soileff_dn8)) * locals.var_t1__blk1145) - (assign14700_e10307 * locals.var_t1__blk1145_dn8)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (assign14700_e10310 * ((((((assign14700_e10303 * locals.var_pparam_b4soidsub_dn9) * locals.var_pparam_b4soileff) + (assign14700_e10305 * locals.var_pparam_b4soileff_dn9)) * locals.var_t1__blk1145) - (assign14700_e10307 * locals.var_t1__blk1145_dn9)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (assign14700_e10310 * ((((((assign14700_e10303 * locals.var_pparam_b4soidsub_dn10) * locals.var_pparam_b4soileff) + (assign14700_e10305 * locals.var_pparam_b4soileff_dn10)) * locals.var_t1__blk1145) - (assign14700_e10307 * locals.var_t1__blk1145_dn10)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (assign14700_e10310 * ((((((assign14700_e10303 * locals.var_pparam_b4soidsub_dn11) * locals.var_pparam_b4soileff) + (assign14700_e10305 * locals.var_pparam_b4soileff_dn11)) * locals.var_t1__blk1145) - (assign14700_e10307 * locals.var_t1__blk1145_dn11)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (assign14700_e10310 * ((((((assign14700_e10303 * locals.var_pparam_b4soidsub_dn12) * locals.var_pparam_b4soileff) + (assign14700_e10305 * locals.var_pparam_b4soileff_dn12)) * locals.var_t1__blk1145) - (assign14700_e10307 * locals.var_t1__blk1145_dn12)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign14700_e10312;
        locals.var_t0__blk1144_dn3 = assign14700_e10312_d_n3;
        locals.var_t0__blk1144_dn4 = assign14700_e10312_d_n4;
        locals.var_t0__blk1144_dn5 = assign14700_e10312_d_n5;
        locals.var_t0__blk1144_dn6 = assign14700_e10312_d_n6;
        locals.var_t0__blk1144_dn7 = assign14700_e10312_d_n7;
        locals.var_t0__blk1144_dn8 = assign14700_e10312_d_n8;
        locals.var_t0__blk1144_dn9 = assign14700_e10312_d_n9;
        locals.var_t0__blk1144_dn10 = assign14700_e10312_d_n10;
        locals.var_t0__blk1144_dn11 = assign14700_e10312_d_n11;
        locals.var_t0__blk1144_dn12 = assign14700_e10312_d_n12;

        let (assign14710_e10322, assign14710_e10322_d_n3, assign14710_e10322_d_n4, assign14710_e10322_d_n5, assign14710_e10322_d_n6, assign14710_e10322_d_n7, assign14710_e10322_d_n8, assign14710_e10322_d_n9, assign14710_e10322_d_n10, assign14710_e10322_d_n11, assign14710_e10322_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign14710_e10317: f64 = (2.0 * locals.var_t0__blk1144);
        let assign14710_e10319: f64 = (assign14710_e10317 * locals.var_t0__blk1144);
        let assign14710_e10320: f64 = (locals.var_t0__blk1144 + assign14710_e10319);
        (assign14710_e10320, (locals.var_t0__blk1144_dn3 + (((2.0 * locals.var_t0__blk1144_dn3) * locals.var_t0__blk1144) + (assign14710_e10317 * locals.var_t0__blk1144_dn3))), (locals.var_t0__blk1144_dn4 + (((2.0 * locals.var_t0__blk1144_dn4) * locals.var_t0__blk1144) + (assign14710_e10317 * locals.var_t0__blk1144_dn4))), (locals.var_t0__blk1144_dn5 + (((2.0 * locals.var_t0__blk1144_dn5) * locals.var_t0__blk1144) + (assign14710_e10317 * locals.var_t0__blk1144_dn5))), (locals.var_t0__blk1144_dn6 + (((2.0 * locals.var_t0__blk1144_dn6) * locals.var_t0__blk1144) + (assign14710_e10317 * locals.var_t0__blk1144_dn6))), (locals.var_t0__blk1144_dn7 + (((2.0 * locals.var_t0__blk1144_dn7) * locals.var_t0__blk1144) + (assign14710_e10317 * locals.var_t0__blk1144_dn7))), (locals.var_t0__blk1144_dn8 + (((2.0 * locals.var_t0__blk1144_dn8) * locals.var_t0__blk1144) + (assign14710_e10317 * locals.var_t0__blk1144_dn8))), (locals.var_t0__blk1144_dn9 + (((2.0 * locals.var_t0__blk1144_dn9) * locals.var_t0__blk1144) + (assign14710_e10317 * locals.var_t0__blk1144_dn9))), (locals.var_t0__blk1144_dn10 + (((2.0 * locals.var_t0__blk1144_dn10) * locals.var_t0__blk1144) + (assign14710_e10317 * locals.var_t0__blk1144_dn10))), (locals.var_t0__blk1144_dn11 + (((2.0 * locals.var_t0__blk1144_dn11) * locals.var_t0__blk1144) + (assign14710_e10317 * locals.var_t0__blk1144_dn11))), (locals.var_t0__blk1144_dn12 + (((2.0 * locals.var_t0__blk1144_dn12) * locals.var_t0__blk1144) + (assign14710_e10317 * locals.var_t0__blk1144_dn12))),)
    } else {
        (locals.var_theta0vb0, locals.var_theta0vb0_dn3, locals.var_theta0vb0_dn4, locals.var_theta0vb0_dn5, locals.var_theta0vb0_dn6, locals.var_theta0vb0_dn7, locals.var_theta0vb0_dn8, locals.var_theta0vb0_dn9, locals.var_theta0vb0_dn10, locals.var_theta0vb0_dn11, locals.var_theta0vb0_dn12,)
    }
};
        locals.var_theta0vb0 = assign14710_e10322;
        locals.var_theta0vb0_dn3 = assign14710_e10322_d_n3;
        locals.var_theta0vb0_dn4 = assign14710_e10322_d_n4;
        locals.var_theta0vb0_dn5 = assign14710_e10322_d_n5;
        locals.var_theta0vb0_dn6 = assign14710_e10322_d_n6;
        locals.var_theta0vb0_dn7 = assign14710_e10322_d_n7;
        locals.var_theta0vb0_dn8 = assign14710_e10322_d_n8;
        locals.var_theta0vb0_dn9 = assign14710_e10322_d_n9;
        locals.var_theta0vb0_dn10 = assign14710_e10322_d_n10;
        locals.var_theta0vb0_dn11 = assign14710_e10322_d_n11;
        locals.var_theta0vb0_dn12 = assign14710_e10322_d_n12;

        let (assign14720_e10334, assign14720_e10334_d_n3, assign14720_e10334_d_n4, assign14720_e10334_d_n5, assign14720_e10334_d_n6, assign14720_e10334_d_n7, assign14720_e10334_d_n8, assign14720_e10334_d_n9, assign14720_e10334_d_n10, assign14720_e10334_d_n11, assign14720_e10334_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign14720_e10325: f64 = (-0.5);
        let assign14720_e10327: f64 = (assign14720_e10325 * locals.var_pparam_b4soidrout);
        let assign14720_e10329: f64 = (assign14720_e10327 * locals.var_pparam_b4soileff);
        let assign14720_e10331: f64 = (assign14720_e10329 / locals.var_t1__blk1145);
        let assign14720_e10332: f64 = (assign14720_e10331).exp();
        (assign14720_e10332, (assign14720_e10332 * ((((((assign14720_e10325 * locals.var_pparam_b4soidrout_dn3) * locals.var_pparam_b4soileff) + (assign14720_e10327 * locals.var_pparam_b4soileff_dn3)) * locals.var_t1__blk1145) - (assign14720_e10329 * locals.var_t1__blk1145_dn3)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (assign14720_e10332 * ((((((assign14720_e10325 * locals.var_pparam_b4soidrout_dn4) * locals.var_pparam_b4soileff) + (assign14720_e10327 * locals.var_pparam_b4soileff_dn4)) * locals.var_t1__blk1145) - (assign14720_e10329 * locals.var_t1__blk1145_dn4)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (assign14720_e10332 * ((((((assign14720_e10325 * locals.var_pparam_b4soidrout_dn5) * locals.var_pparam_b4soileff) + (assign14720_e10327 * locals.var_pparam_b4soileff_dn5)) * locals.var_t1__blk1145) - (assign14720_e10329 * locals.var_t1__blk1145_dn5)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (assign14720_e10332 * ((((((assign14720_e10325 * locals.var_pparam_b4soidrout_dn6) * locals.var_pparam_b4soileff) + (assign14720_e10327 * locals.var_pparam_b4soileff_dn6)) * locals.var_t1__blk1145) - (assign14720_e10329 * locals.var_t1__blk1145_dn6)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (assign14720_e10332 * ((((((assign14720_e10325 * locals.var_pparam_b4soidrout_dn7) * locals.var_pparam_b4soileff) + (assign14720_e10327 * locals.var_pparam_b4soileff_dn7)) * locals.var_t1__blk1145) - (assign14720_e10329 * locals.var_t1__blk1145_dn7)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (assign14720_e10332 * ((((((assign14720_e10325 * locals.var_pparam_b4soidrout_dn8) * locals.var_pparam_b4soileff) + (assign14720_e10327 * locals.var_pparam_b4soileff_dn8)) * locals.var_t1__blk1145) - (assign14720_e10329 * locals.var_t1__blk1145_dn8)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (assign14720_e10332 * ((((((assign14720_e10325 * locals.var_pparam_b4soidrout_dn9) * locals.var_pparam_b4soileff) + (assign14720_e10327 * locals.var_pparam_b4soileff_dn9)) * locals.var_t1__blk1145) - (assign14720_e10329 * locals.var_t1__blk1145_dn9)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (assign14720_e10332 * ((((((assign14720_e10325 * locals.var_pparam_b4soidrout_dn10) * locals.var_pparam_b4soileff) + (assign14720_e10327 * locals.var_pparam_b4soileff_dn10)) * locals.var_t1__blk1145) - (assign14720_e10329 * locals.var_t1__blk1145_dn10)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (assign14720_e10332 * ((((((assign14720_e10325 * locals.var_pparam_b4soidrout_dn11) * locals.var_pparam_b4soileff) + (assign14720_e10327 * locals.var_pparam_b4soileff_dn11)) * locals.var_t1__blk1145) - (assign14720_e10329 * locals.var_t1__blk1145_dn11)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))), (assign14720_e10332 * ((((((assign14720_e10325 * locals.var_pparam_b4soidrout_dn12) * locals.var_pparam_b4soileff) + (assign14720_e10327 * locals.var_pparam_b4soileff_dn12)) * locals.var_t1__blk1145) - (assign14720_e10329 * locals.var_t1__blk1145_dn12)) / (locals.var_t1__blk1145 * locals.var_t1__blk1145))),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign14720_e10334;
        locals.var_t0__blk1144_dn3 = assign14720_e10334_d_n3;
        locals.var_t0__blk1144_dn4 = assign14720_e10334_d_n4;
        locals.var_t0__blk1144_dn5 = assign14720_e10334_d_n5;
        locals.var_t0__blk1144_dn6 = assign14720_e10334_d_n6;
        locals.var_t0__blk1144_dn7 = assign14720_e10334_d_n7;
        locals.var_t0__blk1144_dn8 = assign14720_e10334_d_n8;
        locals.var_t0__blk1144_dn9 = assign14720_e10334_d_n9;
        locals.var_t0__blk1144_dn10 = assign14720_e10334_d_n10;
        locals.var_t0__blk1144_dn11 = assign14720_e10334_d_n11;
        locals.var_t0__blk1144_dn12 = assign14720_e10334_d_n12;

        let (assign14730_e10344, assign14730_e10344_d_n3, assign14730_e10344_d_n4, assign14730_e10344_d_n5, assign14730_e10344_d_n6, assign14730_e10344_d_n7, assign14730_e10344_d_n8, assign14730_e10344_d_n9, assign14730_e10344_d_n10, assign14730_e10344_d_n11, assign14730_e10344_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign14730_e10339: f64 = (2.0 * locals.var_t0__blk1144);
        let assign14730_e10341: f64 = (assign14730_e10339 * locals.var_t0__blk1144);
        let assign14730_e10342: f64 = (locals.var_t0__blk1144 + assign14730_e10341);
        (assign14730_e10342, (locals.var_t0__blk1144_dn3 + (((2.0 * locals.var_t0__blk1144_dn3) * locals.var_t0__blk1144) + (assign14730_e10339 * locals.var_t0__blk1144_dn3))), (locals.var_t0__blk1144_dn4 + (((2.0 * locals.var_t0__blk1144_dn4) * locals.var_t0__blk1144) + (assign14730_e10339 * locals.var_t0__blk1144_dn4))), (locals.var_t0__blk1144_dn5 + (((2.0 * locals.var_t0__blk1144_dn5) * locals.var_t0__blk1144) + (assign14730_e10339 * locals.var_t0__blk1144_dn5))), (locals.var_t0__blk1144_dn6 + (((2.0 * locals.var_t0__blk1144_dn6) * locals.var_t0__blk1144) + (assign14730_e10339 * locals.var_t0__blk1144_dn6))), (locals.var_t0__blk1144_dn7 + (((2.0 * locals.var_t0__blk1144_dn7) * locals.var_t0__blk1144) + (assign14730_e10339 * locals.var_t0__blk1144_dn7))), (locals.var_t0__blk1144_dn8 + (((2.0 * locals.var_t0__blk1144_dn8) * locals.var_t0__blk1144) + (assign14730_e10339 * locals.var_t0__blk1144_dn8))), (locals.var_t0__blk1144_dn9 + (((2.0 * locals.var_t0__blk1144_dn9) * locals.var_t0__blk1144) + (assign14730_e10339 * locals.var_t0__blk1144_dn9))), (locals.var_t0__blk1144_dn10 + (((2.0 * locals.var_t0__blk1144_dn10) * locals.var_t0__blk1144) + (assign14730_e10339 * locals.var_t0__blk1144_dn10))), (locals.var_t0__blk1144_dn11 + (((2.0 * locals.var_t0__blk1144_dn11) * locals.var_t0__blk1144) + (assign14730_e10339 * locals.var_t0__blk1144_dn11))), (locals.var_t0__blk1144_dn12 + (((2.0 * locals.var_t0__blk1144_dn12) * locals.var_t0__blk1144) + (assign14730_e10339 * locals.var_t0__blk1144_dn12))),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign14730_e10344;
        locals.var_t2__blk1146_dn3 = assign14730_e10344_d_n3;
        locals.var_t2__blk1146_dn4 = assign14730_e10344_d_n4;
        locals.var_t2__blk1146_dn5 = assign14730_e10344_d_n5;
        locals.var_t2__blk1146_dn6 = assign14730_e10344_d_n6;
        locals.var_t2__blk1146_dn7 = assign14730_e10344_d_n7;
        locals.var_t2__blk1146_dn8 = assign14730_e10344_d_n8;
        locals.var_t2__blk1146_dn9 = assign14730_e10344_d_n9;
        locals.var_t2__blk1146_dn10 = assign14730_e10344_d_n10;
        locals.var_t2__blk1146_dn11 = assign14730_e10344_d_n11;
        locals.var_t2__blk1146_dn12 = assign14730_e10344_d_n12;

        let (assign14740_e10352, assign14740_e10352_d_n3, assign14740_e10352_d_n4, assign14740_e10352_d_n5, assign14740_e10352_d_n6, assign14740_e10352_d_n7, assign14740_e10352_d_n8, assign14740_e10352_d_n9, assign14740_e10352_d_n10, assign14740_e10352_d_n11, assign14740_e10352_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign14740_e10348: f64 = (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk1146);
        let assign14740_e10350: f64 = (assign14740_e10348 + locals.var_pparam_b4soipdibl2);
        (assign14740_e10350, (((locals.var_pparam_b4soipdibl1_dn3 * locals.var_t2__blk1146) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk1146_dn3)) + locals.var_pparam_b4soipdibl2_dn3), (((locals.var_pparam_b4soipdibl1_dn4 * locals.var_t2__blk1146) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk1146_dn4)) + locals.var_pparam_b4soipdibl2_dn4), (((locals.var_pparam_b4soipdibl1_dn5 * locals.var_t2__blk1146) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk1146_dn5)) + locals.var_pparam_b4soipdibl2_dn5), (((locals.var_pparam_b4soipdibl1_dn6 * locals.var_t2__blk1146) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk1146_dn6)) + locals.var_pparam_b4soipdibl2_dn6), (((locals.var_pparam_b4soipdibl1_dn7 * locals.var_t2__blk1146) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk1146_dn7)) + locals.var_pparam_b4soipdibl2_dn7), (((locals.var_pparam_b4soipdibl1_dn8 * locals.var_t2__blk1146) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk1146_dn8)) + locals.var_pparam_b4soipdibl2_dn8), (((locals.var_pparam_b4soipdibl1_dn9 * locals.var_t2__blk1146) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk1146_dn9)) + locals.var_pparam_b4soipdibl2_dn9), (((locals.var_pparam_b4soipdibl1_dn10 * locals.var_t2__blk1146) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk1146_dn10)) + locals.var_pparam_b4soipdibl2_dn10), (((locals.var_pparam_b4soipdibl1_dn11 * locals.var_t2__blk1146) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk1146_dn11)) + locals.var_pparam_b4soipdibl2_dn11), (((locals.var_pparam_b4soipdibl1_dn12 * locals.var_t2__blk1146) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk1146_dn12)) + locals.var_pparam_b4soipdibl2_dn12),)
    } else {
        (locals.var_thetarout, locals.var_thetarout_dn3, locals.var_thetarout_dn4, locals.var_thetarout_dn5, locals.var_thetarout_dn6, locals.var_thetarout_dn7, locals.var_thetarout_dn8, locals.var_thetarout_dn9, locals.var_thetarout_dn10, locals.var_thetarout_dn11, locals.var_thetarout_dn12,)
    }
};
        locals.var_thetarout = assign14740_e10352;
        locals.var_thetarout_dn3 = assign14740_e10352_d_n3;
        locals.var_thetarout_dn4 = assign14740_e10352_d_n4;
        locals.var_thetarout_dn5 = assign14740_e10352_d_n5;
        locals.var_thetarout_dn6 = assign14740_e10352_d_n6;
        locals.var_thetarout_dn7 = assign14740_e10352_d_n7;
        locals.var_thetarout_dn8 = assign14740_e10352_d_n8;
        locals.var_thetarout_dn9 = assign14740_e10352_d_n9;
        locals.var_thetarout_dn10 = assign14740_e10352_d_n10;
        locals.var_thetarout_dn11 = assign14740_e10352_d_n11;
        locals.var_thetarout_dn12 = assign14740_e10352_d_n12;

        let (assign14750_e10356, assign14750_e10356_d_n6,) = {
    if (locals.var_guard1457 != 0.0) {
        (locals.var_vtm, locals.var_vtm_dn6,)
    } else {
        (locals.var_b4soivtm, locals.var_b4soivtm_dn6,)
    }
};
        locals.var_b4soivtm = assign14750_e10356;
        locals.var_b4soivtm_dn6 = assign14750_e10356_d_n6;

    }

    pub(super) fn stamp_transient_block_29(
        locals: &mut StampLocals,
    ) {
        let (assign14760_e10362, assign14760_e10362_d_n3, assign14760_e10362_d_n4, assign14760_e10362_d_n5, assign14760_e10362_d_n6, assign14760_e10362_d_n7, assign14760_e10362_d_n8, assign14760_e10362_d_n9, assign14760_e10362_d_n10, assign14760_e10362_d_n11, assign14760_e10362_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign14760_e10360: f64 = (locals.var_tempratio - 1.0);
        (assign14760_e10360, 0.0, 0.0, 0.0, locals.var_tempratio_dn6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk1147, locals.var_t3__blk1147_dn3, locals.var_t3__blk1147_dn4, locals.var_t3__blk1147_dn5, locals.var_t3__blk1147_dn6, locals.var_t3__blk1147_dn7, locals.var_t3__blk1147_dn8, locals.var_t3__blk1147_dn9, locals.var_t3__blk1147_dn10, locals.var_t3__blk1147_dn11, locals.var_t3__blk1147_dn12,)
    }
};
        locals.var_t3__blk1147 = assign14760_e10362;
        locals.var_t3__blk1147_dn3 = assign14760_e10362_d_n3;
        locals.var_t3__blk1147_dn4 = assign14760_e10362_d_n4;
        locals.var_t3__blk1147_dn5 = assign14760_e10362_d_n5;
        locals.var_t3__blk1147_dn6 = assign14760_e10362_d_n6;
        locals.var_t3__blk1147_dn7 = assign14760_e10362_d_n7;
        locals.var_t3__blk1147_dn8 = assign14760_e10362_d_n8;
        locals.var_t3__blk1147_dn9 = assign14760_e10362_d_n9;
        locals.var_t3__blk1147_dn10 = assign14760_e10362_d_n10;
        locals.var_t3__blk1147_dn11 = assign14760_e10362_d_n11;
        locals.var_t3__blk1147_dn12 = assign14760_e10362_d_n12;

        let (assign14770_e10370, assign14770_e10370_d_n3, assign14770_e10370_d_n4, assign14770_e10370_d_n5, assign14770_e10370_d_n6, assign14770_e10370_d_n7, assign14770_e10370_d_n8, assign14770_e10370_d_n9, assign14770_e10370_d_n10, assign14770_e10370_d_n11, assign14770_e10370_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign14770_e10366: f64 = (1.115 / locals.var_vtm);
        let assign14770_e10368: f64 = (assign14770_e10366 * locals.var_t3__blk1147);
        (assign14770_e10368, (assign14770_e10366 * locals.var_t3__blk1147_dn3), (assign14770_e10366 * locals.var_t3__blk1147_dn4), (assign14770_e10366 * locals.var_t3__blk1147_dn5), (((-((1.115 * locals.var_vtm_dn6) / (locals.var_vtm * locals.var_vtm))) * locals.var_t3__blk1147) + (assign14770_e10366 * locals.var_t3__blk1147_dn6)), (assign14770_e10366 * locals.var_t3__blk1147_dn7), (assign14770_e10366 * locals.var_t3__blk1147_dn8), (assign14770_e10366 * locals.var_t3__blk1147_dn9), (assign14770_e10366 * locals.var_t3__blk1147_dn10), (assign14770_e10366 * locals.var_t3__blk1147_dn11), (assign14770_e10366 * locals.var_t3__blk1147_dn12),)
    } else {
        (locals.var_t4__blk1148, locals.var_t4__blk1148_dn3, locals.var_t4__blk1148_dn4, locals.var_t4__blk1148_dn5, locals.var_t4__blk1148_dn6, locals.var_t4__blk1148_dn7, locals.var_t4__blk1148_dn8, locals.var_t4__blk1148_dn9, locals.var_t4__blk1148_dn10, locals.var_t4__blk1148_dn11, locals.var_t4__blk1148_dn12,)
    }
};
        locals.var_t4__blk1148 = assign14770_e10370;
        locals.var_t4__blk1148_dn3 = assign14770_e10370_d_n3;
        locals.var_t4__blk1148_dn4 = assign14770_e10370_d_n4;
        locals.var_t4__blk1148_dn5 = assign14770_e10370_d_n5;
        locals.var_t4__blk1148_dn6 = assign14770_e10370_d_n6;
        locals.var_t4__blk1148_dn7 = assign14770_e10370_d_n7;
        locals.var_t4__blk1148_dn8 = assign14770_e10370_d_n8;
        locals.var_t4__blk1148_dn9 = assign14770_e10370_d_n9;
        locals.var_t4__blk1148_dn10 = assign14770_e10370_d_n10;
        locals.var_t4__blk1148_dn11 = assign14770_e10370_d_n11;
        locals.var_t4__blk1148_dn12 = assign14770_e10370_d_n12;

        let (assign14780_e10378, assign14780_e10378_d_n3, assign14780_e10378_d_n4, assign14780_e10378_d_n5, assign14780_e10378_d_n6, assign14780_e10378_d_n7, assign14780_e10378_d_n8, assign14780_e10378_d_n9, assign14780_e10378_d_n10, assign14780_e10378_d_n11, assign14780_e10378_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign14780_e10374: f64 = (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148);
        let assign14780_e10376: f64 = (assign14780_e10374 / locals.var_pparam_b4soindiode);
        (assign14780_e10376, (((((locals.var_pparam_b4soixbjt_dn3 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn3)) * locals.var_pparam_b4soindiode) - (assign14780_e10374 * locals.var_pparam_b4soindiode_dn3)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixbjt_dn4 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn4)) * locals.var_pparam_b4soindiode) - (assign14780_e10374 * locals.var_pparam_b4soindiode_dn4)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixbjt_dn5 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn5)) * locals.var_pparam_b4soindiode) - (assign14780_e10374 * locals.var_pparam_b4soindiode_dn5)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixbjt_dn6 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn6)) * locals.var_pparam_b4soindiode) - (assign14780_e10374 * locals.var_pparam_b4soindiode_dn6)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixbjt_dn7 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn7)) * locals.var_pparam_b4soindiode) - (assign14780_e10374 * locals.var_pparam_b4soindiode_dn7)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixbjt_dn8 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn8)) * locals.var_pparam_b4soindiode) - (assign14780_e10374 * locals.var_pparam_b4soindiode_dn8)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixbjt_dn9 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn9)) * locals.var_pparam_b4soindiode) - (assign14780_e10374 * locals.var_pparam_b4soindiode_dn9)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixbjt_dn10 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn10)) * locals.var_pparam_b4soindiode) - (assign14780_e10374 * locals.var_pparam_b4soindiode_dn10)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixbjt_dn11 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn11)) * locals.var_pparam_b4soindiode) - (assign14780_e10374 * locals.var_pparam_b4soindiode_dn11)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixbjt_dn12 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn12)) * locals.var_pparam_b4soindiode) - (assign14780_e10374 * locals.var_pparam_b4soindiode_dn12)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)),)
    } else {
        (locals.var_t7__blk1151, locals.var_t7__blk1151_dn3, locals.var_t7__blk1151_dn4, locals.var_t7__blk1151_dn5, locals.var_t7__blk1151_dn6, locals.var_t7__blk1151_dn7, locals.var_t7__blk1151_dn8, locals.var_t7__blk1151_dn9, locals.var_t7__blk1151_dn10, locals.var_t7__blk1151_dn11, locals.var_t7__blk1151_dn12,)
    }
};
        locals.var_t7__blk1151 = assign14780_e10378;
        locals.var_t7__blk1151_dn3 = assign14780_e10378_d_n3;
        locals.var_t7__blk1151_dn4 = assign14780_e10378_d_n4;
        locals.var_t7__blk1151_dn5 = assign14780_e10378_d_n5;
        locals.var_t7__blk1151_dn6 = assign14780_e10378_d_n6;
        locals.var_t7__blk1151_dn7 = assign14780_e10378_d_n7;
        locals.var_t7__blk1151_dn8 = assign14780_e10378_d_n8;
        locals.var_t7__blk1151_dn9 = assign14780_e10378_d_n9;
        locals.var_t7__blk1151_dn10 = assign14780_e10378_d_n10;
        locals.var_t7__blk1151_dn11 = assign14780_e10378_d_n11;
        locals.var_t7__blk1151_dn12 = assign14780_e10378_d_n12;

        let assign14790_e10381: f64 = if locals.var_t7__blk1151 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1461 = assign14790_e10381;

        let (assign14800_e10393, assign14800_e10393_d_n3, assign14800_e10393_d_n4, assign14800_e10393_d_n5, assign14800_e10393_d_n6, assign14800_e10393_d_n7, assign14800_e10393_d_n8, assign14800_e10393_d_n9, assign14800_e10393_d_n10, assign14800_e10393_d_n11, assign14800_e10393_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1461 != 0.0)) {
        let assign14800_e10388: f64 = (1.0 + locals.var_t7__blk1151);
        let assign14800_e10390: f64 = (assign14800_e10388 - 100.0);
        let assign14800_e10391: f64 = (2.688117142e43 * assign14800_e10390);
        (assign14800_e10391, (2.688117142e43 * locals.var_t7__blk1151_dn3), (2.688117142e43 * locals.var_t7__blk1151_dn4), (2.688117142e43 * locals.var_t7__blk1151_dn5), (2.688117142e43 * locals.var_t7__blk1151_dn6), (2.688117142e43 * locals.var_t7__blk1151_dn7), (2.688117142e43 * locals.var_t7__blk1151_dn8), (2.688117142e43 * locals.var_t7__blk1151_dn9), (2.688117142e43 * locals.var_t7__blk1151_dn10), (2.688117142e43 * locals.var_t7__blk1151_dn11), (2.688117142e43 * locals.var_t7__blk1151_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign14800_e10393;
        locals.var_t0__blk1144_dn3 = assign14800_e10393_d_n3;
        locals.var_t0__blk1144_dn4 = assign14800_e10393_d_n4;
        locals.var_t0__blk1144_dn5 = assign14800_e10393_d_n5;
        locals.var_t0__blk1144_dn6 = assign14800_e10393_d_n6;
        locals.var_t0__blk1144_dn7 = assign14800_e10393_d_n7;
        locals.var_t0__blk1144_dn8 = assign14800_e10393_d_n8;
        locals.var_t0__blk1144_dn9 = assign14800_e10393_d_n9;
        locals.var_t0__blk1144_dn10 = assign14800_e10393_d_n10;
        locals.var_t0__blk1144_dn11 = assign14800_e10393_d_n11;
        locals.var_t0__blk1144_dn12 = assign14800_e10393_d_n12;

        let assign14810_e10396: f64 = (-100.0);
        let assign14810_e10397: f64 = if locals.var_t7__blk1151 < assign14810_e10396 { 1.0 } else { 0.0 };
        locals.var_guard1462 = assign14810_e10397;

        let (assign14820_e10406, assign14820_e10406_d_n3, assign14820_e10406_d_n4, assign14820_e10406_d_n5, assign14820_e10406_d_n6, assign14820_e10406_d_n7, assign14820_e10406_d_n8, assign14820_e10406_d_n9, assign14820_e10406_d_n10, assign14820_e10406_d_n11, assign14820_e10406_d_n12,) = {
    if (((locals.var_guard1457 != 0.0) && (locals.var_guard1461 == 0.0)) && (locals.var_guard1462 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign14820_e10406;
        locals.var_t0__blk1144_dn3 = assign14820_e10406_d_n3;
        locals.var_t0__blk1144_dn4 = assign14820_e10406_d_n4;
        locals.var_t0__blk1144_dn5 = assign14820_e10406_d_n5;
        locals.var_t0__blk1144_dn6 = assign14820_e10406_d_n6;
        locals.var_t0__blk1144_dn7 = assign14820_e10406_d_n7;
        locals.var_t0__blk1144_dn8 = assign14820_e10406_d_n8;
        locals.var_t0__blk1144_dn9 = assign14820_e10406_d_n9;
        locals.var_t0__blk1144_dn10 = assign14820_e10406_d_n10;
        locals.var_t0__blk1144_dn11 = assign14820_e10406_d_n11;
        locals.var_t0__blk1144_dn12 = assign14820_e10406_d_n12;

        let (assign14830_e10417, assign14830_e10417_d_n3, assign14830_e10417_d_n4, assign14830_e10417_d_n5, assign14830_e10417_d_n6, assign14830_e10417_d_n7, assign14830_e10417_d_n8, assign14830_e10417_d_n9, assign14830_e10417_d_n10, assign14830_e10417_d_n11, assign14830_e10417_d_n12,) = {
    if (((locals.var_guard1457 != 0.0) && (locals.var_guard1461 == 0.0)) && (locals.var_guard1462 == 0.0)) {
        let assign14830_e10415: f64 = (locals.var_t7__blk1151).exp();
        (assign14830_e10415, (assign14830_e10415 * locals.var_t7__blk1151_dn3), (assign14830_e10415 * locals.var_t7__blk1151_dn4), (assign14830_e10415 * locals.var_t7__blk1151_dn5), (assign14830_e10415 * locals.var_t7__blk1151_dn6), (assign14830_e10415 * locals.var_t7__blk1151_dn7), (assign14830_e10415 * locals.var_t7__blk1151_dn8), (assign14830_e10415 * locals.var_t7__blk1151_dn9), (assign14830_e10415 * locals.var_t7__blk1151_dn10), (assign14830_e10415 * locals.var_t7__blk1151_dn11), (assign14830_e10415 * locals.var_t7__blk1151_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign14830_e10417;
        locals.var_t0__blk1144_dn3 = assign14830_e10417_d_n3;
        locals.var_t0__blk1144_dn4 = assign14830_e10417_d_n4;
        locals.var_t0__blk1144_dn5 = assign14830_e10417_d_n5;
        locals.var_t0__blk1144_dn6 = assign14830_e10417_d_n6;
        locals.var_t0__blk1144_dn7 = assign14830_e10417_d_n7;
        locals.var_t0__blk1144_dn8 = assign14830_e10417_d_n8;
        locals.var_t0__blk1144_dn9 = assign14830_e10417_d_n9;
        locals.var_t0__blk1144_dn10 = assign14830_e10417_d_n10;
        locals.var_t0__blk1144_dn11 = assign14830_e10417_d_n11;
        locals.var_t0__blk1144_dn12 = assign14830_e10417_d_n12;

        let assign14840_e10420: f64 = if locals.var_pparam_b4soixbjt == locals.var_pparam_b4soixdif { 1.0 } else { 0.0 };
        locals.var_guard1463 = assign14840_e10420;

        let (assign14850_e10426, assign14850_e10426_d_n3, assign14850_e10426_d_n4, assign14850_e10426_d_n5, assign14850_e10426_d_n6, assign14850_e10426_d_n7, assign14850_e10426_d_n8, assign14850_e10426_d_n9, assign14850_e10426_d_n10, assign14850_e10426_d_n11, assign14850_e10426_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1463 != 0.0)) {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign14850_e10426;
        locals.var_t1__blk1145_dn3 = assign14850_e10426_d_n3;
        locals.var_t1__blk1145_dn4 = assign14850_e10426_d_n4;
        locals.var_t1__blk1145_dn5 = assign14850_e10426_d_n5;
        locals.var_t1__blk1145_dn6 = assign14850_e10426_d_n6;
        locals.var_t1__blk1145_dn7 = assign14850_e10426_d_n7;
        locals.var_t1__blk1145_dn8 = assign14850_e10426_d_n8;
        locals.var_t1__blk1145_dn9 = assign14850_e10426_d_n9;
        locals.var_t1__blk1145_dn10 = assign14850_e10426_d_n10;
        locals.var_t1__blk1145_dn11 = assign14850_e10426_d_n11;
        locals.var_t1__blk1145_dn12 = assign14850_e10426_d_n12;

        let (assign14860_e10437, assign14860_e10437_d_n3, assign14860_e10437_d_n4, assign14860_e10437_d_n5, assign14860_e10437_d_n6, assign14860_e10437_d_n7, assign14860_e10437_d_n8, assign14860_e10437_d_n9, assign14860_e10437_d_n10, assign14860_e10437_d_n11, assign14860_e10437_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1463 == 0.0)) {
        let assign14860_e10433: f64 = (locals.var_pparam_b4soixdif * locals.var_t4__blk1148);
        let assign14860_e10435: f64 = (assign14860_e10433 / locals.var_pparam_b4soindiode);
        (assign14860_e10435, (((((locals.var_pparam_b4soixdif_dn3 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdif * locals.var_t4__blk1148_dn3)) * locals.var_pparam_b4soindiode) - (assign14860_e10433 * locals.var_pparam_b4soindiode_dn3)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixdif_dn4 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdif * locals.var_t4__blk1148_dn4)) * locals.var_pparam_b4soindiode) - (assign14860_e10433 * locals.var_pparam_b4soindiode_dn4)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixdif_dn5 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdif * locals.var_t4__blk1148_dn5)) * locals.var_pparam_b4soindiode) - (assign14860_e10433 * locals.var_pparam_b4soindiode_dn5)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixdif_dn6 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdif * locals.var_t4__blk1148_dn6)) * locals.var_pparam_b4soindiode) - (assign14860_e10433 * locals.var_pparam_b4soindiode_dn6)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixdif_dn7 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdif * locals.var_t4__blk1148_dn7)) * locals.var_pparam_b4soindiode) - (assign14860_e10433 * locals.var_pparam_b4soindiode_dn7)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixdif_dn8 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdif * locals.var_t4__blk1148_dn8)) * locals.var_pparam_b4soindiode) - (assign14860_e10433 * locals.var_pparam_b4soindiode_dn8)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixdif_dn9 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdif * locals.var_t4__blk1148_dn9)) * locals.var_pparam_b4soindiode) - (assign14860_e10433 * locals.var_pparam_b4soindiode_dn9)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixdif_dn10 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdif * locals.var_t4__blk1148_dn10)) * locals.var_pparam_b4soindiode) - (assign14860_e10433 * locals.var_pparam_b4soindiode_dn10)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixdif_dn11 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdif * locals.var_t4__blk1148_dn11)) * locals.var_pparam_b4soindiode) - (assign14860_e10433 * locals.var_pparam_b4soindiode_dn11)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixdif_dn12 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdif * locals.var_t4__blk1148_dn12)) * locals.var_pparam_b4soindiode) - (assign14860_e10433 * locals.var_pparam_b4soindiode_dn12)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)),)
    } else {
        (locals.var_t7__blk1151, locals.var_t7__blk1151_dn3, locals.var_t7__blk1151_dn4, locals.var_t7__blk1151_dn5, locals.var_t7__blk1151_dn6, locals.var_t7__blk1151_dn7, locals.var_t7__blk1151_dn8, locals.var_t7__blk1151_dn9, locals.var_t7__blk1151_dn10, locals.var_t7__blk1151_dn11, locals.var_t7__blk1151_dn12,)
    }
};
        locals.var_t7__blk1151 = assign14860_e10437;
        locals.var_t7__blk1151_dn3 = assign14860_e10437_d_n3;
        locals.var_t7__blk1151_dn4 = assign14860_e10437_d_n4;
        locals.var_t7__blk1151_dn5 = assign14860_e10437_d_n5;
        locals.var_t7__blk1151_dn6 = assign14860_e10437_d_n6;
        locals.var_t7__blk1151_dn7 = assign14860_e10437_d_n7;
        locals.var_t7__blk1151_dn8 = assign14860_e10437_d_n8;
        locals.var_t7__blk1151_dn9 = assign14860_e10437_d_n9;
        locals.var_t7__blk1151_dn10 = assign14860_e10437_d_n10;
        locals.var_t7__blk1151_dn11 = assign14860_e10437_d_n11;
        locals.var_t7__blk1151_dn12 = assign14860_e10437_d_n12;

        let assign14870_e10440: f64 = if locals.var_t7__blk1151 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1464 = assign14870_e10440;

        let (assign14880_e10455, assign14880_e10455_d_n3, assign14880_e10455_d_n4, assign14880_e10455_d_n5, assign14880_e10455_d_n6, assign14880_e10455_d_n7, assign14880_e10455_d_n8, assign14880_e10455_d_n9, assign14880_e10455_d_n10, assign14880_e10455_d_n11, assign14880_e10455_d_n12,) = {
    if (((locals.var_guard1457 != 0.0) && (locals.var_guard1463 == 0.0)) && (locals.var_guard1464 != 0.0)) {
        let assign14880_e10450: f64 = (1.0 + locals.var_t7__blk1151);
        let assign14880_e10452: f64 = (assign14880_e10450 - 100.0);
        let assign14880_e10453: f64 = (2.688117142e43 * assign14880_e10452);
        (assign14880_e10453, (2.688117142e43 * locals.var_t7__blk1151_dn3), (2.688117142e43 * locals.var_t7__blk1151_dn4), (2.688117142e43 * locals.var_t7__blk1151_dn5), (2.688117142e43 * locals.var_t7__blk1151_dn6), (2.688117142e43 * locals.var_t7__blk1151_dn7), (2.688117142e43 * locals.var_t7__blk1151_dn8), (2.688117142e43 * locals.var_t7__blk1151_dn9), (2.688117142e43 * locals.var_t7__blk1151_dn10), (2.688117142e43 * locals.var_t7__blk1151_dn11), (2.688117142e43 * locals.var_t7__blk1151_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign14880_e10455;
        locals.var_t1__blk1145_dn3 = assign14880_e10455_d_n3;
        locals.var_t1__blk1145_dn4 = assign14880_e10455_d_n4;
        locals.var_t1__blk1145_dn5 = assign14880_e10455_d_n5;
        locals.var_t1__blk1145_dn6 = assign14880_e10455_d_n6;
        locals.var_t1__blk1145_dn7 = assign14880_e10455_d_n7;
        locals.var_t1__blk1145_dn8 = assign14880_e10455_d_n8;
        locals.var_t1__blk1145_dn9 = assign14880_e10455_d_n9;
        locals.var_t1__blk1145_dn10 = assign14880_e10455_d_n10;
        locals.var_t1__blk1145_dn11 = assign14880_e10455_d_n11;
        locals.var_t1__blk1145_dn12 = assign14880_e10455_d_n12;

        let assign14890_e10458: f64 = (-100.0);
        let assign14890_e10459: f64 = if locals.var_t7__blk1151 < assign14890_e10458 { 1.0 } else { 0.0 };
        locals.var_guard1465 = assign14890_e10459;

        let (assign14900_e10471, assign14900_e10471_d_n3, assign14900_e10471_d_n4, assign14900_e10471_d_n5, assign14900_e10471_d_n6, assign14900_e10471_d_n7, assign14900_e10471_d_n8, assign14900_e10471_d_n9, assign14900_e10471_d_n10, assign14900_e10471_d_n11, assign14900_e10471_d_n12,) = {
    if ((((locals.var_guard1457 != 0.0) && (locals.var_guard1463 == 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1465 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign14900_e10471;
        locals.var_t1__blk1145_dn3 = assign14900_e10471_d_n3;
        locals.var_t1__blk1145_dn4 = assign14900_e10471_d_n4;
        locals.var_t1__blk1145_dn5 = assign14900_e10471_d_n5;
        locals.var_t1__blk1145_dn6 = assign14900_e10471_d_n6;
        locals.var_t1__blk1145_dn7 = assign14900_e10471_d_n7;
        locals.var_t1__blk1145_dn8 = assign14900_e10471_d_n8;
        locals.var_t1__blk1145_dn9 = assign14900_e10471_d_n9;
        locals.var_t1__blk1145_dn10 = assign14900_e10471_d_n10;
        locals.var_t1__blk1145_dn11 = assign14900_e10471_d_n11;
        locals.var_t1__blk1145_dn12 = assign14900_e10471_d_n12;

        let (assign14910_e10485, assign14910_e10485_d_n3, assign14910_e10485_d_n4, assign14910_e10485_d_n5, assign14910_e10485_d_n6, assign14910_e10485_d_n7, assign14910_e10485_d_n8, assign14910_e10485_d_n9, assign14910_e10485_d_n10, assign14910_e10485_d_n11, assign14910_e10485_d_n12,) = {
    if ((((locals.var_guard1457 != 0.0) && (locals.var_guard1463 == 0.0)) && (locals.var_guard1464 == 0.0)) && (locals.var_guard1465 == 0.0)) {
        let assign14910_e10483: f64 = (locals.var_t7__blk1151).exp();
        (assign14910_e10483, (assign14910_e10483 * locals.var_t7__blk1151_dn3), (assign14910_e10483 * locals.var_t7__blk1151_dn4), (assign14910_e10483 * locals.var_t7__blk1151_dn5), (assign14910_e10483 * locals.var_t7__blk1151_dn6), (assign14910_e10483 * locals.var_t7__blk1151_dn7), (assign14910_e10483 * locals.var_t7__blk1151_dn8), (assign14910_e10483 * locals.var_t7__blk1151_dn9), (assign14910_e10483 * locals.var_t7__blk1151_dn10), (assign14910_e10483 * locals.var_t7__blk1151_dn11), (assign14910_e10483 * locals.var_t7__blk1151_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign14910_e10485;
        locals.var_t1__blk1145_dn3 = assign14910_e10485_d_n3;
        locals.var_t1__blk1145_dn4 = assign14910_e10485_d_n4;
        locals.var_t1__blk1145_dn5 = assign14910_e10485_d_n5;
        locals.var_t1__blk1145_dn6 = assign14910_e10485_d_n6;
        locals.var_t1__blk1145_dn7 = assign14910_e10485_d_n7;
        locals.var_t1__blk1145_dn8 = assign14910_e10485_d_n8;
        locals.var_t1__blk1145_dn9 = assign14910_e10485_d_n9;
        locals.var_t1__blk1145_dn10 = assign14910_e10485_d_n10;
        locals.var_t1__blk1145_dn11 = assign14910_e10485_d_n11;
        locals.var_t1__blk1145_dn12 = assign14910_e10485_d_n12;

        let (assign14920_e10493, assign14920_e10493_d_n3, assign14920_e10493_d_n4, assign14920_e10493_d_n5, assign14920_e10493_d_n6, assign14920_e10493_d_n7, assign14920_e10493_d_n8, assign14920_e10493_d_n9, assign14920_e10493_d_n10, assign14920_e10493_d_n11, assign14920_e10493_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign14920_e10489: f64 = (locals.var_pparam_b4soixrec * locals.var_t4__blk1148);
        let assign14920_e10491: f64 = (assign14920_e10489 / locals.var_pparam_b4soinrecf0);
        (assign14920_e10491, (((((locals.var_pparam_b4soixrec_dn3 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrec * locals.var_t4__blk1148_dn3)) * locals.var_pparam_b4soinrecf0) - (assign14920_e10489 * locals.var_pparam_b4soinrecf0_dn3)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)), (((((locals.var_pparam_b4soixrec_dn4 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrec * locals.var_t4__blk1148_dn4)) * locals.var_pparam_b4soinrecf0) - (assign14920_e10489 * locals.var_pparam_b4soinrecf0_dn4)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)), (((((locals.var_pparam_b4soixrec_dn5 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrec * locals.var_t4__blk1148_dn5)) * locals.var_pparam_b4soinrecf0) - (assign14920_e10489 * locals.var_pparam_b4soinrecf0_dn5)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)), (((((locals.var_pparam_b4soixrec_dn6 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrec * locals.var_t4__blk1148_dn6)) * locals.var_pparam_b4soinrecf0) - (assign14920_e10489 * locals.var_pparam_b4soinrecf0_dn6)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)), (((((locals.var_pparam_b4soixrec_dn7 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrec * locals.var_t4__blk1148_dn7)) * locals.var_pparam_b4soinrecf0) - (assign14920_e10489 * locals.var_pparam_b4soinrecf0_dn7)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)), (((((locals.var_pparam_b4soixrec_dn8 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrec * locals.var_t4__blk1148_dn8)) * locals.var_pparam_b4soinrecf0) - (assign14920_e10489 * locals.var_pparam_b4soinrecf0_dn8)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)), (((((locals.var_pparam_b4soixrec_dn9 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrec * locals.var_t4__blk1148_dn9)) * locals.var_pparam_b4soinrecf0) - (assign14920_e10489 * locals.var_pparam_b4soinrecf0_dn9)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)), (((((locals.var_pparam_b4soixrec_dn10 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrec * locals.var_t4__blk1148_dn10)) * locals.var_pparam_b4soinrecf0) - (assign14920_e10489 * locals.var_pparam_b4soinrecf0_dn10)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)), (((((locals.var_pparam_b4soixrec_dn11 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrec * locals.var_t4__blk1148_dn11)) * locals.var_pparam_b4soinrecf0) - (assign14920_e10489 * locals.var_pparam_b4soinrecf0_dn11)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)), (((((locals.var_pparam_b4soixrec_dn12 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrec * locals.var_t4__blk1148_dn12)) * locals.var_pparam_b4soinrecf0) - (assign14920_e10489 * locals.var_pparam_b4soinrecf0_dn12)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)),)
    } else {
        (locals.var_t7__blk1151, locals.var_t7__blk1151_dn3, locals.var_t7__blk1151_dn4, locals.var_t7__blk1151_dn5, locals.var_t7__blk1151_dn6, locals.var_t7__blk1151_dn7, locals.var_t7__blk1151_dn8, locals.var_t7__blk1151_dn9, locals.var_t7__blk1151_dn10, locals.var_t7__blk1151_dn11, locals.var_t7__blk1151_dn12,)
    }
};
        locals.var_t7__blk1151 = assign14920_e10493;
        locals.var_t7__blk1151_dn3 = assign14920_e10493_d_n3;
        locals.var_t7__blk1151_dn4 = assign14920_e10493_d_n4;
        locals.var_t7__blk1151_dn5 = assign14920_e10493_d_n5;
        locals.var_t7__blk1151_dn6 = assign14920_e10493_d_n6;
        locals.var_t7__blk1151_dn7 = assign14920_e10493_d_n7;
        locals.var_t7__blk1151_dn8 = assign14920_e10493_d_n8;
        locals.var_t7__blk1151_dn9 = assign14920_e10493_d_n9;
        locals.var_t7__blk1151_dn10 = assign14920_e10493_d_n10;
        locals.var_t7__blk1151_dn11 = assign14920_e10493_d_n11;
        locals.var_t7__blk1151_dn12 = assign14920_e10493_d_n12;

        let assign14930_e10496: f64 = if locals.var_t7__blk1151 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1466 = assign14930_e10496;

        let (assign14940_e10508, assign14940_e10508_d_n3, assign14940_e10508_d_n4, assign14940_e10508_d_n5, assign14940_e10508_d_n6, assign14940_e10508_d_n7, assign14940_e10508_d_n8, assign14940_e10508_d_n9, assign14940_e10508_d_n10, assign14940_e10508_d_n11, assign14940_e10508_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1466 != 0.0)) {
        let assign14940_e10503: f64 = (1.0 + locals.var_t7__blk1151);
        let assign14940_e10505: f64 = (assign14940_e10503 - 100.0);
        let assign14940_e10506: f64 = (2.688117142e43 * assign14940_e10505);
        (assign14940_e10506, (2.688117142e43 * locals.var_t7__blk1151_dn3), (2.688117142e43 * locals.var_t7__blk1151_dn4), (2.688117142e43 * locals.var_t7__blk1151_dn5), (2.688117142e43 * locals.var_t7__blk1151_dn6), (2.688117142e43 * locals.var_t7__blk1151_dn7), (2.688117142e43 * locals.var_t7__blk1151_dn8), (2.688117142e43 * locals.var_t7__blk1151_dn9), (2.688117142e43 * locals.var_t7__blk1151_dn10), (2.688117142e43 * locals.var_t7__blk1151_dn11), (2.688117142e43 * locals.var_t7__blk1151_dn12),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign14940_e10508;
        locals.var_t2__blk1146_dn3 = assign14940_e10508_d_n3;
        locals.var_t2__blk1146_dn4 = assign14940_e10508_d_n4;
        locals.var_t2__blk1146_dn5 = assign14940_e10508_d_n5;
        locals.var_t2__blk1146_dn6 = assign14940_e10508_d_n6;
        locals.var_t2__blk1146_dn7 = assign14940_e10508_d_n7;
        locals.var_t2__blk1146_dn8 = assign14940_e10508_d_n8;
        locals.var_t2__blk1146_dn9 = assign14940_e10508_d_n9;
        locals.var_t2__blk1146_dn10 = assign14940_e10508_d_n10;
        locals.var_t2__blk1146_dn11 = assign14940_e10508_d_n11;
        locals.var_t2__blk1146_dn12 = assign14940_e10508_d_n12;

        let assign14950_e10511: f64 = (-100.0);
        let assign14950_e10512: f64 = if locals.var_t7__blk1151 < assign14950_e10511 { 1.0 } else { 0.0 };
        locals.var_guard1467 = assign14950_e10512;

        let (assign14960_e10521, assign14960_e10521_d_n3, assign14960_e10521_d_n4, assign14960_e10521_d_n5, assign14960_e10521_d_n6, assign14960_e10521_d_n7, assign14960_e10521_d_n8, assign14960_e10521_d_n9, assign14960_e10521_d_n10, assign14960_e10521_d_n11, assign14960_e10521_d_n12,) = {
    if (((locals.var_guard1457 != 0.0) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1467 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign14960_e10521;
        locals.var_t2__blk1146_dn3 = assign14960_e10521_d_n3;
        locals.var_t2__blk1146_dn4 = assign14960_e10521_d_n4;
        locals.var_t2__blk1146_dn5 = assign14960_e10521_d_n5;
        locals.var_t2__blk1146_dn6 = assign14960_e10521_d_n6;
        locals.var_t2__blk1146_dn7 = assign14960_e10521_d_n7;
        locals.var_t2__blk1146_dn8 = assign14960_e10521_d_n8;
        locals.var_t2__blk1146_dn9 = assign14960_e10521_d_n9;
        locals.var_t2__blk1146_dn10 = assign14960_e10521_d_n10;
        locals.var_t2__blk1146_dn11 = assign14960_e10521_d_n11;
        locals.var_t2__blk1146_dn12 = assign14960_e10521_d_n12;

        let (assign14970_e10532, assign14970_e10532_d_n3, assign14970_e10532_d_n4, assign14970_e10532_d_n5, assign14970_e10532_d_n6, assign14970_e10532_d_n7, assign14970_e10532_d_n8, assign14970_e10532_d_n9, assign14970_e10532_d_n10, assign14970_e10532_d_n11, assign14970_e10532_d_n12,) = {
    if (((locals.var_guard1457 != 0.0) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1467 == 0.0)) {
        let assign14970_e10530: f64 = (locals.var_t7__blk1151).exp();
        (assign14970_e10530, (assign14970_e10530 * locals.var_t7__blk1151_dn3), (assign14970_e10530 * locals.var_t7__blk1151_dn4), (assign14970_e10530 * locals.var_t7__blk1151_dn5), (assign14970_e10530 * locals.var_t7__blk1151_dn6), (assign14970_e10530 * locals.var_t7__blk1151_dn7), (assign14970_e10530 * locals.var_t7__blk1151_dn8), (assign14970_e10530 * locals.var_t7__blk1151_dn9), (assign14970_e10530 * locals.var_t7__blk1151_dn10), (assign14970_e10530 * locals.var_t7__blk1151_dn11), (assign14970_e10530 * locals.var_t7__blk1151_dn12),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign14970_e10532;
        locals.var_t2__blk1146_dn3 = assign14970_e10532_d_n3;
        locals.var_t2__blk1146_dn4 = assign14970_e10532_d_n4;
        locals.var_t2__blk1146_dn5 = assign14970_e10532_d_n5;
        locals.var_t2__blk1146_dn6 = assign14970_e10532_d_n6;
        locals.var_t2__blk1146_dn7 = assign14970_e10532_d_n7;
        locals.var_t2__blk1146_dn8 = assign14970_e10532_d_n8;
        locals.var_t2__blk1146_dn9 = assign14970_e10532_d_n9;
        locals.var_t2__blk1146_dn10 = assign14970_e10532_d_n10;
        locals.var_t2__blk1146_dn11 = assign14970_e10532_d_n11;
        locals.var_t2__blk1146_dn12 = assign14970_e10532_d_n12;

        let (assign14980_e10538, assign14980_e10538_d_n3, assign14980_e10538_d_n4, assign14980_e10538_d_n5, assign14980_e10538_d_n6, assign14980_e10538_d_n7, assign14980_e10538_d_n8, assign14980_e10538_d_n9, assign14980_e10538_d_n10, assign14980_e10538_d_n11, assign14980_e10538_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign14980_e10536: f64 = (locals.var_pparam_b4soiahli * locals.var_t0__blk1144);
        (assign14980_e10536, ((locals.var_pparam_b4soiahli_dn3 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahli * locals.var_t0__blk1144_dn3)), ((locals.var_pparam_b4soiahli_dn4 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahli * locals.var_t0__blk1144_dn4)), ((locals.var_pparam_b4soiahli_dn5 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahli * locals.var_t0__blk1144_dn5)), ((locals.var_pparam_b4soiahli_dn6 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahli * locals.var_t0__blk1144_dn6)), ((locals.var_pparam_b4soiahli_dn7 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahli * locals.var_t0__blk1144_dn7)), ((locals.var_pparam_b4soiahli_dn8 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahli * locals.var_t0__blk1144_dn8)), ((locals.var_pparam_b4soiahli_dn9 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahli * locals.var_t0__blk1144_dn9)), ((locals.var_pparam_b4soiahli_dn10 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahli * locals.var_t0__blk1144_dn10)), ((locals.var_pparam_b4soiahli_dn11 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahli * locals.var_t0__blk1144_dn11)), ((locals.var_pparam_b4soiahli_dn12 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahli * locals.var_t0__blk1144_dn12)),)
    } else {
        (locals.var_ahlis, locals.var_ahlis_dn3, locals.var_ahlis_dn4, locals.var_ahlis_dn5, locals.var_ahlis_dn6, locals.var_ahlis_dn7, locals.var_ahlis_dn8, locals.var_ahlis_dn9, locals.var_ahlis_dn10, locals.var_ahlis_dn11, locals.var_ahlis_dn12,)
    }
};
        locals.var_ahlis = assign14980_e10538;
        locals.var_ahlis_dn3 = assign14980_e10538_d_n3;
        locals.var_ahlis_dn4 = assign14980_e10538_d_n4;
        locals.var_ahlis_dn5 = assign14980_e10538_d_n5;
        locals.var_ahlis_dn6 = assign14980_e10538_d_n6;
        locals.var_ahlis_dn7 = assign14980_e10538_d_n7;
        locals.var_ahlis_dn8 = assign14980_e10538_d_n8;
        locals.var_ahlis_dn9 = assign14980_e10538_d_n9;
        locals.var_ahlis_dn10 = assign14980_e10538_d_n10;
        locals.var_ahlis_dn11 = assign14980_e10538_d_n11;
        locals.var_ahlis_dn12 = assign14980_e10538_d_n12;

        let (assign14990_e10544, assign14990_e10544_d_n3, assign14990_e10544_d_n4, assign14990_e10544_d_n5, assign14990_e10544_d_n6, assign14990_e10544_d_n7, assign14990_e10544_d_n8, assign14990_e10544_d_n9, assign14990_e10544_d_n10, assign14990_e10544_d_n11, assign14990_e10544_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign14990_e10542: f64 = (locals.var_pparam_b4soiisbjt * locals.var_t0__blk1144);
        (assign14990_e10542, ((locals.var_pparam_b4soiisbjt_dn3 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk1144_dn3)), ((locals.var_pparam_b4soiisbjt_dn4 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk1144_dn4)), ((locals.var_pparam_b4soiisbjt_dn5 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk1144_dn5)), ((locals.var_pparam_b4soiisbjt_dn6 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk1144_dn6)), ((locals.var_pparam_b4soiisbjt_dn7 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk1144_dn7)), ((locals.var_pparam_b4soiisbjt_dn8 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk1144_dn8)), ((locals.var_pparam_b4soiisbjt_dn9 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk1144_dn9)), ((locals.var_pparam_b4soiisbjt_dn10 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk1144_dn10)), ((locals.var_pparam_b4soiisbjt_dn11 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk1144_dn11)), ((locals.var_pparam_b4soiisbjt_dn12 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk1144_dn12)),)
    } else {
        (locals.var_jbjts, locals.var_jbjts_dn3, locals.var_jbjts_dn4, locals.var_jbjts_dn5, locals.var_jbjts_dn6, locals.var_jbjts_dn7, locals.var_jbjts_dn8, locals.var_jbjts_dn9, locals.var_jbjts_dn10, locals.var_jbjts_dn11, locals.var_jbjts_dn12,)
    }
};
        locals.var_jbjts = assign14990_e10544;
        locals.var_jbjts_dn3 = assign14990_e10544_d_n3;
        locals.var_jbjts_dn4 = assign14990_e10544_d_n4;
        locals.var_jbjts_dn5 = assign14990_e10544_d_n5;
        locals.var_jbjts_dn6 = assign14990_e10544_d_n6;
        locals.var_jbjts_dn7 = assign14990_e10544_d_n7;
        locals.var_jbjts_dn8 = assign14990_e10544_d_n8;
        locals.var_jbjts_dn9 = assign14990_e10544_d_n9;
        locals.var_jbjts_dn10 = assign14990_e10544_d_n10;
        locals.var_jbjts_dn11 = assign14990_e10544_d_n11;
        locals.var_jbjts_dn12 = assign14990_e10544_d_n12;

        let (assign15000_e10550, assign15000_e10550_d_n3, assign15000_e10550_d_n4, assign15000_e10550_d_n5, assign15000_e10550_d_n6, assign15000_e10550_d_n7, assign15000_e10550_d_n8, assign15000_e10550_d_n9, assign15000_e10550_d_n10, assign15000_e10550_d_n11, assign15000_e10550_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15000_e10548: f64 = (locals.var_pparam_b4soiisdif * locals.var_t1__blk1145);
        (assign15000_e10548, ((locals.var_pparam_b4soiisdif_dn3 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk1145_dn3)), ((locals.var_pparam_b4soiisdif_dn4 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk1145_dn4)), ((locals.var_pparam_b4soiisdif_dn5 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk1145_dn5)), ((locals.var_pparam_b4soiisdif_dn6 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk1145_dn6)), ((locals.var_pparam_b4soiisdif_dn7 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk1145_dn7)), ((locals.var_pparam_b4soiisdif_dn8 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk1145_dn8)), ((locals.var_pparam_b4soiisdif_dn9 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk1145_dn9)), ((locals.var_pparam_b4soiisdif_dn10 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk1145_dn10)), ((locals.var_pparam_b4soiisdif_dn11 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk1145_dn11)), ((locals.var_pparam_b4soiisdif_dn12 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk1145_dn12)),)
    } else {
        (locals.var_jdifs, locals.var_jdifs_dn3, locals.var_jdifs_dn4, locals.var_jdifs_dn5, locals.var_jdifs_dn6, locals.var_jdifs_dn7, locals.var_jdifs_dn8, locals.var_jdifs_dn9, locals.var_jdifs_dn10, locals.var_jdifs_dn11, locals.var_jdifs_dn12,)
    }
};
        locals.var_jdifs = assign15000_e10550;
        locals.var_jdifs_dn3 = assign15000_e10550_d_n3;
        locals.var_jdifs_dn4 = assign15000_e10550_d_n4;
        locals.var_jdifs_dn5 = assign15000_e10550_d_n5;
        locals.var_jdifs_dn6 = assign15000_e10550_d_n6;
        locals.var_jdifs_dn7 = assign15000_e10550_d_n7;
        locals.var_jdifs_dn8 = assign15000_e10550_d_n8;
        locals.var_jdifs_dn9 = assign15000_e10550_d_n9;
        locals.var_jdifs_dn10 = assign15000_e10550_d_n10;
        locals.var_jdifs_dn11 = assign15000_e10550_d_n11;
        locals.var_jdifs_dn12 = assign15000_e10550_d_n12;

        let (assign15010_e10556, assign15010_e10556_d_n3, assign15010_e10556_d_n4, assign15010_e10556_d_n5, assign15010_e10556_d_n6, assign15010_e10556_d_n7, assign15010_e10556_d_n8, assign15010_e10556_d_n9, assign15010_e10556_d_n10, assign15010_e10556_d_n11, assign15010_e10556_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15010_e10554: f64 = (locals.var_pparam_b4soiisrec * locals.var_t2__blk1146);
        (assign15010_e10554, ((locals.var_pparam_b4soiisrec_dn3 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk1146_dn3)), ((locals.var_pparam_b4soiisrec_dn4 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk1146_dn4)), ((locals.var_pparam_b4soiisrec_dn5 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk1146_dn5)), ((locals.var_pparam_b4soiisrec_dn6 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk1146_dn6)), ((locals.var_pparam_b4soiisrec_dn7 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk1146_dn7)), ((locals.var_pparam_b4soiisrec_dn8 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk1146_dn8)), ((locals.var_pparam_b4soiisrec_dn9 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk1146_dn9)), ((locals.var_pparam_b4soiisrec_dn10 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk1146_dn10)), ((locals.var_pparam_b4soiisrec_dn11 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk1146_dn11)), ((locals.var_pparam_b4soiisrec_dn12 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk1146_dn12)),)
    } else {
        (locals.var_jrecs, locals.var_jrecs_dn3, locals.var_jrecs_dn4, locals.var_jrecs_dn5, locals.var_jrecs_dn6, locals.var_jrecs_dn7, locals.var_jrecs_dn8, locals.var_jrecs_dn9, locals.var_jrecs_dn10, locals.var_jrecs_dn11, locals.var_jrecs_dn12,)
    }
};
        locals.var_jrecs = assign15010_e10556;
        locals.var_jrecs_dn3 = assign15010_e10556_d_n3;
        locals.var_jrecs_dn4 = assign15010_e10556_d_n4;
        locals.var_jrecs_dn5 = assign15010_e10556_d_n5;
        locals.var_jrecs_dn6 = assign15010_e10556_d_n6;
        locals.var_jrecs_dn7 = assign15010_e10556_d_n7;
        locals.var_jrecs_dn8 = assign15010_e10556_d_n8;
        locals.var_jrecs_dn9 = assign15010_e10556_d_n9;
        locals.var_jrecs_dn10 = assign15010_e10556_d_n10;
        locals.var_jrecs_dn11 = assign15010_e10556_d_n11;
        locals.var_jrecs_dn12 = assign15010_e10556_d_n12;

        let (assign15020_e10562, assign15020_e10562_d_n3, assign15020_e10562_d_n4, assign15020_e10562_d_n5, assign15020_e10562_d_n6, assign15020_e10562_d_n7, assign15020_e10562_d_n8, assign15020_e10562_d_n9, assign15020_e10562_d_n10, assign15020_e10562_d_n11, assign15020_e10562_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15020_e10560: f64 = (locals.var_pparam_b4soixtun * locals.var_t3__blk1147);
        (assign15020_e10560, ((locals.var_pparam_b4soixtun_dn3 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtun * locals.var_t3__blk1147_dn3)), ((locals.var_pparam_b4soixtun_dn4 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtun * locals.var_t3__blk1147_dn4)), ((locals.var_pparam_b4soixtun_dn5 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtun * locals.var_t3__blk1147_dn5)), ((locals.var_pparam_b4soixtun_dn6 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtun * locals.var_t3__blk1147_dn6)), ((locals.var_pparam_b4soixtun_dn7 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtun * locals.var_t3__blk1147_dn7)), ((locals.var_pparam_b4soixtun_dn8 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtun * locals.var_t3__blk1147_dn8)), ((locals.var_pparam_b4soixtun_dn9 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtun * locals.var_t3__blk1147_dn9)), ((locals.var_pparam_b4soixtun_dn10 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtun * locals.var_t3__blk1147_dn10)), ((locals.var_pparam_b4soixtun_dn11 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtun * locals.var_t3__blk1147_dn11)), ((locals.var_pparam_b4soixtun_dn12 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtun * locals.var_t3__blk1147_dn12)),)
    } else {
        (locals.var_t7__blk1151, locals.var_t7__blk1151_dn3, locals.var_t7__blk1151_dn4, locals.var_t7__blk1151_dn5, locals.var_t7__blk1151_dn6, locals.var_t7__blk1151_dn7, locals.var_t7__blk1151_dn8, locals.var_t7__blk1151_dn9, locals.var_t7__blk1151_dn10, locals.var_t7__blk1151_dn11, locals.var_t7__blk1151_dn12,)
    }
};
        locals.var_t7__blk1151 = assign15020_e10562;
        locals.var_t7__blk1151_dn3 = assign15020_e10562_d_n3;
        locals.var_t7__blk1151_dn4 = assign15020_e10562_d_n4;
        locals.var_t7__blk1151_dn5 = assign15020_e10562_d_n5;
        locals.var_t7__blk1151_dn6 = assign15020_e10562_d_n6;
        locals.var_t7__blk1151_dn7 = assign15020_e10562_d_n7;
        locals.var_t7__blk1151_dn8 = assign15020_e10562_d_n8;
        locals.var_t7__blk1151_dn9 = assign15020_e10562_d_n9;
        locals.var_t7__blk1151_dn10 = assign15020_e10562_d_n10;
        locals.var_t7__blk1151_dn11 = assign15020_e10562_d_n11;
        locals.var_t7__blk1151_dn12 = assign15020_e10562_d_n12;

        let assign15030_e10565: f64 = if locals.var_t7__blk1151 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1468 = assign15030_e10565;

        let (assign15040_e10577, assign15040_e10577_d_n3, assign15040_e10577_d_n4, assign15040_e10577_d_n5, assign15040_e10577_d_n6, assign15040_e10577_d_n7, assign15040_e10577_d_n8, assign15040_e10577_d_n9, assign15040_e10577_d_n10, assign15040_e10577_d_n11, assign15040_e10577_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1468 != 0.0)) {
        let assign15040_e10572: f64 = (1.0 + locals.var_t7__blk1151);
        let assign15040_e10574: f64 = (assign15040_e10572 - 100.0);
        let assign15040_e10575: f64 = (2.688117142e43 * assign15040_e10574);
        (assign15040_e10575, (2.688117142e43 * locals.var_t7__blk1151_dn3), (2.688117142e43 * locals.var_t7__blk1151_dn4), (2.688117142e43 * locals.var_t7__blk1151_dn5), (2.688117142e43 * locals.var_t7__blk1151_dn6), (2.688117142e43 * locals.var_t7__blk1151_dn7), (2.688117142e43 * locals.var_t7__blk1151_dn8), (2.688117142e43 * locals.var_t7__blk1151_dn9), (2.688117142e43 * locals.var_t7__blk1151_dn10), (2.688117142e43 * locals.var_t7__blk1151_dn11), (2.688117142e43 * locals.var_t7__blk1151_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign15040_e10577;
        locals.var_t0__blk1144_dn3 = assign15040_e10577_d_n3;
        locals.var_t0__blk1144_dn4 = assign15040_e10577_d_n4;
        locals.var_t0__blk1144_dn5 = assign15040_e10577_d_n5;
        locals.var_t0__blk1144_dn6 = assign15040_e10577_d_n6;
        locals.var_t0__blk1144_dn7 = assign15040_e10577_d_n7;
        locals.var_t0__blk1144_dn8 = assign15040_e10577_d_n8;
        locals.var_t0__blk1144_dn9 = assign15040_e10577_d_n9;
        locals.var_t0__blk1144_dn10 = assign15040_e10577_d_n10;
        locals.var_t0__blk1144_dn11 = assign15040_e10577_d_n11;
        locals.var_t0__blk1144_dn12 = assign15040_e10577_d_n12;

        let assign15050_e10580: f64 = (-100.0);
        let assign15050_e10581: f64 = if locals.var_t7__blk1151 < assign15050_e10580 { 1.0 } else { 0.0 };
        locals.var_guard1469 = assign15050_e10581;

        let (assign15060_e10590, assign15060_e10590_d_n3, assign15060_e10590_d_n4, assign15060_e10590_d_n5, assign15060_e10590_d_n6, assign15060_e10590_d_n7, assign15060_e10590_d_n8, assign15060_e10590_d_n9, assign15060_e10590_d_n10, assign15060_e10590_d_n11, assign15060_e10590_d_n12,) = {
    if (((locals.var_guard1457 != 0.0) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign15060_e10590;
        locals.var_t0__blk1144_dn3 = assign15060_e10590_d_n3;
        locals.var_t0__blk1144_dn4 = assign15060_e10590_d_n4;
        locals.var_t0__blk1144_dn5 = assign15060_e10590_d_n5;
        locals.var_t0__blk1144_dn6 = assign15060_e10590_d_n6;
        locals.var_t0__blk1144_dn7 = assign15060_e10590_d_n7;
        locals.var_t0__blk1144_dn8 = assign15060_e10590_d_n8;
        locals.var_t0__blk1144_dn9 = assign15060_e10590_d_n9;
        locals.var_t0__blk1144_dn10 = assign15060_e10590_d_n10;
        locals.var_t0__blk1144_dn11 = assign15060_e10590_d_n11;
        locals.var_t0__blk1144_dn12 = assign15060_e10590_d_n12;

        let (assign15070_e10601, assign15070_e10601_d_n3, assign15070_e10601_d_n4, assign15070_e10601_d_n5, assign15070_e10601_d_n6, assign15070_e10601_d_n7, assign15070_e10601_d_n8, assign15070_e10601_d_n9, assign15070_e10601_d_n10, assign15070_e10601_d_n11, assign15070_e10601_d_n12,) = {
    if (((locals.var_guard1457 != 0.0) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
        let assign15070_e10599: f64 = (locals.var_t7__blk1151).exp();
        (assign15070_e10599, (assign15070_e10599 * locals.var_t7__blk1151_dn3), (assign15070_e10599 * locals.var_t7__blk1151_dn4), (assign15070_e10599 * locals.var_t7__blk1151_dn5), (assign15070_e10599 * locals.var_t7__blk1151_dn6), (assign15070_e10599 * locals.var_t7__blk1151_dn7), (assign15070_e10599 * locals.var_t7__blk1151_dn8), (assign15070_e10599 * locals.var_t7__blk1151_dn9), (assign15070_e10599 * locals.var_t7__blk1151_dn10), (assign15070_e10599 * locals.var_t7__blk1151_dn11), (assign15070_e10599 * locals.var_t7__blk1151_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign15070_e10601;
        locals.var_t0__blk1144_dn3 = assign15070_e10601_d_n3;
        locals.var_t0__blk1144_dn4 = assign15070_e10601_d_n4;
        locals.var_t0__blk1144_dn5 = assign15070_e10601_d_n5;
        locals.var_t0__blk1144_dn6 = assign15070_e10601_d_n6;
        locals.var_t0__blk1144_dn7 = assign15070_e10601_d_n7;
        locals.var_t0__blk1144_dn8 = assign15070_e10601_d_n8;
        locals.var_t0__blk1144_dn9 = assign15070_e10601_d_n9;
        locals.var_t0__blk1144_dn10 = assign15070_e10601_d_n10;
        locals.var_t0__blk1144_dn11 = assign15070_e10601_d_n11;
        locals.var_t0__blk1144_dn12 = assign15070_e10601_d_n12;

    }

    pub(super) fn stamp_transient_block_30(
        locals: &mut StampLocals,
    ) {
        let (assign15080_e10607, assign15080_e10607_d_n3, assign15080_e10607_d_n4, assign15080_e10607_d_n5, assign15080_e10607_d_n6, assign15080_e10607_d_n7, assign15080_e10607_d_n8, assign15080_e10607_d_n9, assign15080_e10607_d_n10, assign15080_e10607_d_n11, assign15080_e10607_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15080_e10605: f64 = (locals.var_pparam_b4soiistun * locals.var_t0__blk1144);
        (assign15080_e10605, ((locals.var_pparam_b4soiistun_dn3 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiistun * locals.var_t0__blk1144_dn3)), ((locals.var_pparam_b4soiistun_dn4 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiistun * locals.var_t0__blk1144_dn4)), ((locals.var_pparam_b4soiistun_dn5 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiistun * locals.var_t0__blk1144_dn5)), ((locals.var_pparam_b4soiistun_dn6 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiistun * locals.var_t0__blk1144_dn6)), ((locals.var_pparam_b4soiistun_dn7 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiistun * locals.var_t0__blk1144_dn7)), ((locals.var_pparam_b4soiistun_dn8 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiistun * locals.var_t0__blk1144_dn8)), ((locals.var_pparam_b4soiistun_dn9 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiistun * locals.var_t0__blk1144_dn9)), ((locals.var_pparam_b4soiistun_dn10 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiistun * locals.var_t0__blk1144_dn10)), ((locals.var_pparam_b4soiistun_dn11 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiistun * locals.var_t0__blk1144_dn11)), ((locals.var_pparam_b4soiistun_dn12 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiistun * locals.var_t0__blk1144_dn12)),)
    } else {
        (locals.var_jtuns, locals.var_jtuns_dn3, locals.var_jtuns_dn4, locals.var_jtuns_dn5, locals.var_jtuns_dn6, locals.var_jtuns_dn7, locals.var_jtuns_dn8, locals.var_jtuns_dn9, locals.var_jtuns_dn10, locals.var_jtuns_dn11, locals.var_jtuns_dn12,)
    }
};
        locals.var_jtuns = assign15080_e10607;
        locals.var_jtuns_dn3 = assign15080_e10607_d_n3;
        locals.var_jtuns_dn4 = assign15080_e10607_d_n4;
        locals.var_jtuns_dn5 = assign15080_e10607_d_n5;
        locals.var_jtuns_dn6 = assign15080_e10607_d_n6;
        locals.var_jtuns_dn7 = assign15080_e10607_d_n7;
        locals.var_jtuns_dn8 = assign15080_e10607_d_n8;
        locals.var_jtuns_dn9 = assign15080_e10607_d_n9;
        locals.var_jtuns_dn10 = assign15080_e10607_d_n10;
        locals.var_jtuns_dn11 = assign15080_e10607_d_n11;
        locals.var_jtuns_dn12 = assign15080_e10607_d_n12;

        let (assign15090_e10615, assign15090_e10615_d_n3, assign15090_e10615_d_n4, assign15090_e10615_d_n5, assign15090_e10615_d_n6, assign15090_e10615_d_n7, assign15090_e10615_d_n8, assign15090_e10615_d_n9, assign15090_e10615_d_n10, assign15090_e10615_d_n11, assign15090_e10615_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15090_e10611: f64 = (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148);
        let assign15090_e10613: f64 = (assign15090_e10611 / locals.var_pparam_b4soindioded);
        (assign15090_e10613, (((((locals.var_pparam_b4soixbjt_dn3 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn3)) * locals.var_pparam_b4soindioded) - (assign15090_e10611 * locals.var_pparam_b4soindioded_dn3)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixbjt_dn4 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn4)) * locals.var_pparam_b4soindioded) - (assign15090_e10611 * locals.var_pparam_b4soindioded_dn4)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixbjt_dn5 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn5)) * locals.var_pparam_b4soindioded) - (assign15090_e10611 * locals.var_pparam_b4soindioded_dn5)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixbjt_dn6 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn6)) * locals.var_pparam_b4soindioded) - (assign15090_e10611 * locals.var_pparam_b4soindioded_dn6)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixbjt_dn7 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn7)) * locals.var_pparam_b4soindioded) - (assign15090_e10611 * locals.var_pparam_b4soindioded_dn7)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixbjt_dn8 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn8)) * locals.var_pparam_b4soindioded) - (assign15090_e10611 * locals.var_pparam_b4soindioded_dn8)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixbjt_dn9 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn9)) * locals.var_pparam_b4soindioded) - (assign15090_e10611 * locals.var_pparam_b4soindioded_dn9)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixbjt_dn10 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn10)) * locals.var_pparam_b4soindioded) - (assign15090_e10611 * locals.var_pparam_b4soindioded_dn10)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixbjt_dn11 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn11)) * locals.var_pparam_b4soindioded) - (assign15090_e10611 * locals.var_pparam_b4soindioded_dn11)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixbjt_dn12 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk1148_dn12)) * locals.var_pparam_b4soindioded) - (assign15090_e10611 * locals.var_pparam_b4soindioded_dn12)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)),)
    } else {
        (locals.var_t7__blk1151, locals.var_t7__blk1151_dn3, locals.var_t7__blk1151_dn4, locals.var_t7__blk1151_dn5, locals.var_t7__blk1151_dn6, locals.var_t7__blk1151_dn7, locals.var_t7__blk1151_dn8, locals.var_t7__blk1151_dn9, locals.var_t7__blk1151_dn10, locals.var_t7__blk1151_dn11, locals.var_t7__blk1151_dn12,)
    }
};
        locals.var_t7__blk1151 = assign15090_e10615;
        locals.var_t7__blk1151_dn3 = assign15090_e10615_d_n3;
        locals.var_t7__blk1151_dn4 = assign15090_e10615_d_n4;
        locals.var_t7__blk1151_dn5 = assign15090_e10615_d_n5;
        locals.var_t7__blk1151_dn6 = assign15090_e10615_d_n6;
        locals.var_t7__blk1151_dn7 = assign15090_e10615_d_n7;
        locals.var_t7__blk1151_dn8 = assign15090_e10615_d_n8;
        locals.var_t7__blk1151_dn9 = assign15090_e10615_d_n9;
        locals.var_t7__blk1151_dn10 = assign15090_e10615_d_n10;
        locals.var_t7__blk1151_dn11 = assign15090_e10615_d_n11;
        locals.var_t7__blk1151_dn12 = assign15090_e10615_d_n12;

        let assign15100_e10618: f64 = if locals.var_t7__blk1151 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1470 = assign15100_e10618;

        let (assign15110_e10630, assign15110_e10630_d_n3, assign15110_e10630_d_n4, assign15110_e10630_d_n5, assign15110_e10630_d_n6, assign15110_e10630_d_n7, assign15110_e10630_d_n8, assign15110_e10630_d_n9, assign15110_e10630_d_n10, assign15110_e10630_d_n11, assign15110_e10630_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1470 != 0.0)) {
        let assign15110_e10625: f64 = (1.0 + locals.var_t7__blk1151);
        let assign15110_e10627: f64 = (assign15110_e10625 - 100.0);
        let assign15110_e10628: f64 = (2.688117142e43 * assign15110_e10627);
        (assign15110_e10628, (2.688117142e43 * locals.var_t7__blk1151_dn3), (2.688117142e43 * locals.var_t7__blk1151_dn4), (2.688117142e43 * locals.var_t7__blk1151_dn5), (2.688117142e43 * locals.var_t7__blk1151_dn6), (2.688117142e43 * locals.var_t7__blk1151_dn7), (2.688117142e43 * locals.var_t7__blk1151_dn8), (2.688117142e43 * locals.var_t7__blk1151_dn9), (2.688117142e43 * locals.var_t7__blk1151_dn10), (2.688117142e43 * locals.var_t7__blk1151_dn11), (2.688117142e43 * locals.var_t7__blk1151_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign15110_e10630;
        locals.var_t0__blk1144_dn3 = assign15110_e10630_d_n3;
        locals.var_t0__blk1144_dn4 = assign15110_e10630_d_n4;
        locals.var_t0__blk1144_dn5 = assign15110_e10630_d_n5;
        locals.var_t0__blk1144_dn6 = assign15110_e10630_d_n6;
        locals.var_t0__blk1144_dn7 = assign15110_e10630_d_n7;
        locals.var_t0__blk1144_dn8 = assign15110_e10630_d_n8;
        locals.var_t0__blk1144_dn9 = assign15110_e10630_d_n9;
        locals.var_t0__blk1144_dn10 = assign15110_e10630_d_n10;
        locals.var_t0__blk1144_dn11 = assign15110_e10630_d_n11;
        locals.var_t0__blk1144_dn12 = assign15110_e10630_d_n12;

        let assign15120_e10633: f64 = (-100.0);
        let assign15120_e10634: f64 = if locals.var_t7__blk1151 < assign15120_e10633 { 1.0 } else { 0.0 };
        locals.var_guard1471 = assign15120_e10634;

        let (assign15130_e10643, assign15130_e10643_d_n3, assign15130_e10643_d_n4, assign15130_e10643_d_n5, assign15130_e10643_d_n6, assign15130_e10643_d_n7, assign15130_e10643_d_n8, assign15130_e10643_d_n9, assign15130_e10643_d_n10, assign15130_e10643_d_n11, assign15130_e10643_d_n12,) = {
    if (((locals.var_guard1457 != 0.0) && (locals.var_guard1470 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign15130_e10643;
        locals.var_t0__blk1144_dn3 = assign15130_e10643_d_n3;
        locals.var_t0__blk1144_dn4 = assign15130_e10643_d_n4;
        locals.var_t0__blk1144_dn5 = assign15130_e10643_d_n5;
        locals.var_t0__blk1144_dn6 = assign15130_e10643_d_n6;
        locals.var_t0__blk1144_dn7 = assign15130_e10643_d_n7;
        locals.var_t0__blk1144_dn8 = assign15130_e10643_d_n8;
        locals.var_t0__blk1144_dn9 = assign15130_e10643_d_n9;
        locals.var_t0__blk1144_dn10 = assign15130_e10643_d_n10;
        locals.var_t0__blk1144_dn11 = assign15130_e10643_d_n11;
        locals.var_t0__blk1144_dn12 = assign15130_e10643_d_n12;

        let (assign15140_e10654, assign15140_e10654_d_n3, assign15140_e10654_d_n4, assign15140_e10654_d_n5, assign15140_e10654_d_n6, assign15140_e10654_d_n7, assign15140_e10654_d_n8, assign15140_e10654_d_n9, assign15140_e10654_d_n10, assign15140_e10654_d_n11, assign15140_e10654_d_n12,) = {
    if (((locals.var_guard1457 != 0.0) && (locals.var_guard1470 == 0.0)) && (locals.var_guard1471 == 0.0)) {
        let assign15140_e10652: f64 = (locals.var_t7__blk1151).exp();
        (assign15140_e10652, (assign15140_e10652 * locals.var_t7__blk1151_dn3), (assign15140_e10652 * locals.var_t7__blk1151_dn4), (assign15140_e10652 * locals.var_t7__blk1151_dn5), (assign15140_e10652 * locals.var_t7__blk1151_dn6), (assign15140_e10652 * locals.var_t7__blk1151_dn7), (assign15140_e10652 * locals.var_t7__blk1151_dn8), (assign15140_e10652 * locals.var_t7__blk1151_dn9), (assign15140_e10652 * locals.var_t7__blk1151_dn10), (assign15140_e10652 * locals.var_t7__blk1151_dn11), (assign15140_e10652 * locals.var_t7__blk1151_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign15140_e10654;
        locals.var_t0__blk1144_dn3 = assign15140_e10654_d_n3;
        locals.var_t0__blk1144_dn4 = assign15140_e10654_d_n4;
        locals.var_t0__blk1144_dn5 = assign15140_e10654_d_n5;
        locals.var_t0__blk1144_dn6 = assign15140_e10654_d_n6;
        locals.var_t0__blk1144_dn7 = assign15140_e10654_d_n7;
        locals.var_t0__blk1144_dn8 = assign15140_e10654_d_n8;
        locals.var_t0__blk1144_dn9 = assign15140_e10654_d_n9;
        locals.var_t0__blk1144_dn10 = assign15140_e10654_d_n10;
        locals.var_t0__blk1144_dn11 = assign15140_e10654_d_n11;
        locals.var_t0__blk1144_dn12 = assign15140_e10654_d_n12;

        let assign15150_e10657: f64 = if locals.var_pparam_b4soixbjt == locals.var_pparam_b4soixdifd { 1.0 } else { 0.0 };
        locals.var_guard1472 = assign15150_e10657;

        let (assign15160_e10663, assign15160_e10663_d_n3, assign15160_e10663_d_n4, assign15160_e10663_d_n5, assign15160_e10663_d_n6, assign15160_e10663_d_n7, assign15160_e10663_d_n8, assign15160_e10663_d_n9, assign15160_e10663_d_n10, assign15160_e10663_d_n11, assign15160_e10663_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1472 != 0.0)) {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign15160_e10663;
        locals.var_t1__blk1145_dn3 = assign15160_e10663_d_n3;
        locals.var_t1__blk1145_dn4 = assign15160_e10663_d_n4;
        locals.var_t1__blk1145_dn5 = assign15160_e10663_d_n5;
        locals.var_t1__blk1145_dn6 = assign15160_e10663_d_n6;
        locals.var_t1__blk1145_dn7 = assign15160_e10663_d_n7;
        locals.var_t1__blk1145_dn8 = assign15160_e10663_d_n8;
        locals.var_t1__blk1145_dn9 = assign15160_e10663_d_n9;
        locals.var_t1__blk1145_dn10 = assign15160_e10663_d_n10;
        locals.var_t1__blk1145_dn11 = assign15160_e10663_d_n11;
        locals.var_t1__blk1145_dn12 = assign15160_e10663_d_n12;

        let (assign15170_e10674, assign15170_e10674_d_n3, assign15170_e10674_d_n4, assign15170_e10674_d_n5, assign15170_e10674_d_n6, assign15170_e10674_d_n7, assign15170_e10674_d_n8, assign15170_e10674_d_n9, assign15170_e10674_d_n10, assign15170_e10674_d_n11, assign15170_e10674_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1472 == 0.0)) {
        let assign15170_e10670: f64 = (locals.var_pparam_b4soixdifd * locals.var_t4__blk1148);
        let assign15170_e10672: f64 = (assign15170_e10670 / locals.var_pparam_b4soindioded);
        (assign15170_e10672, (((((locals.var_pparam_b4soixdifd_dn3 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk1148_dn3)) * locals.var_pparam_b4soindioded) - (assign15170_e10670 * locals.var_pparam_b4soindioded_dn3)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixdifd_dn4 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk1148_dn4)) * locals.var_pparam_b4soindioded) - (assign15170_e10670 * locals.var_pparam_b4soindioded_dn4)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixdifd_dn5 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk1148_dn5)) * locals.var_pparam_b4soindioded) - (assign15170_e10670 * locals.var_pparam_b4soindioded_dn5)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixdifd_dn6 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk1148_dn6)) * locals.var_pparam_b4soindioded) - (assign15170_e10670 * locals.var_pparam_b4soindioded_dn6)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixdifd_dn7 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk1148_dn7)) * locals.var_pparam_b4soindioded) - (assign15170_e10670 * locals.var_pparam_b4soindioded_dn7)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixdifd_dn8 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk1148_dn8)) * locals.var_pparam_b4soindioded) - (assign15170_e10670 * locals.var_pparam_b4soindioded_dn8)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixdifd_dn9 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk1148_dn9)) * locals.var_pparam_b4soindioded) - (assign15170_e10670 * locals.var_pparam_b4soindioded_dn9)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixdifd_dn10 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk1148_dn10)) * locals.var_pparam_b4soindioded) - (assign15170_e10670 * locals.var_pparam_b4soindioded_dn10)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixdifd_dn11 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk1148_dn11)) * locals.var_pparam_b4soindioded) - (assign15170_e10670 * locals.var_pparam_b4soindioded_dn11)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixdifd_dn12 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk1148_dn12)) * locals.var_pparam_b4soindioded) - (assign15170_e10670 * locals.var_pparam_b4soindioded_dn12)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)),)
    } else {
        (locals.var_t7__blk1151, locals.var_t7__blk1151_dn3, locals.var_t7__blk1151_dn4, locals.var_t7__blk1151_dn5, locals.var_t7__blk1151_dn6, locals.var_t7__blk1151_dn7, locals.var_t7__blk1151_dn8, locals.var_t7__blk1151_dn9, locals.var_t7__blk1151_dn10, locals.var_t7__blk1151_dn11, locals.var_t7__blk1151_dn12,)
    }
};
        locals.var_t7__blk1151 = assign15170_e10674;
        locals.var_t7__blk1151_dn3 = assign15170_e10674_d_n3;
        locals.var_t7__blk1151_dn4 = assign15170_e10674_d_n4;
        locals.var_t7__blk1151_dn5 = assign15170_e10674_d_n5;
        locals.var_t7__blk1151_dn6 = assign15170_e10674_d_n6;
        locals.var_t7__blk1151_dn7 = assign15170_e10674_d_n7;
        locals.var_t7__blk1151_dn8 = assign15170_e10674_d_n8;
        locals.var_t7__blk1151_dn9 = assign15170_e10674_d_n9;
        locals.var_t7__blk1151_dn10 = assign15170_e10674_d_n10;
        locals.var_t7__blk1151_dn11 = assign15170_e10674_d_n11;
        locals.var_t7__blk1151_dn12 = assign15170_e10674_d_n12;

        let assign15180_e10677: f64 = if locals.var_t7__blk1151 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1473 = assign15180_e10677;

        let (assign15190_e10692, assign15190_e10692_d_n3, assign15190_e10692_d_n4, assign15190_e10692_d_n5, assign15190_e10692_d_n6, assign15190_e10692_d_n7, assign15190_e10692_d_n8, assign15190_e10692_d_n9, assign15190_e10692_d_n10, assign15190_e10692_d_n11, assign15190_e10692_d_n12,) = {
    if (((locals.var_guard1457 != 0.0) && (locals.var_guard1472 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        let assign15190_e10687: f64 = (1.0 + locals.var_t7__blk1151);
        let assign15190_e10689: f64 = (assign15190_e10687 - 100.0);
        let assign15190_e10690: f64 = (2.688117142e43 * assign15190_e10689);
        (assign15190_e10690, (2.688117142e43 * locals.var_t7__blk1151_dn3), (2.688117142e43 * locals.var_t7__blk1151_dn4), (2.688117142e43 * locals.var_t7__blk1151_dn5), (2.688117142e43 * locals.var_t7__blk1151_dn6), (2.688117142e43 * locals.var_t7__blk1151_dn7), (2.688117142e43 * locals.var_t7__blk1151_dn8), (2.688117142e43 * locals.var_t7__blk1151_dn9), (2.688117142e43 * locals.var_t7__blk1151_dn10), (2.688117142e43 * locals.var_t7__blk1151_dn11), (2.688117142e43 * locals.var_t7__blk1151_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign15190_e10692;
        locals.var_t1__blk1145_dn3 = assign15190_e10692_d_n3;
        locals.var_t1__blk1145_dn4 = assign15190_e10692_d_n4;
        locals.var_t1__blk1145_dn5 = assign15190_e10692_d_n5;
        locals.var_t1__blk1145_dn6 = assign15190_e10692_d_n6;
        locals.var_t1__blk1145_dn7 = assign15190_e10692_d_n7;
        locals.var_t1__blk1145_dn8 = assign15190_e10692_d_n8;
        locals.var_t1__blk1145_dn9 = assign15190_e10692_d_n9;
        locals.var_t1__blk1145_dn10 = assign15190_e10692_d_n10;
        locals.var_t1__blk1145_dn11 = assign15190_e10692_d_n11;
        locals.var_t1__blk1145_dn12 = assign15190_e10692_d_n12;

        let assign15200_e10695: f64 = (-100.0);
        let assign15200_e10696: f64 = if locals.var_t7__blk1151 < assign15200_e10695 { 1.0 } else { 0.0 };
        locals.var_guard1474 = assign15200_e10696;

        let (assign15210_e10708, assign15210_e10708_d_n3, assign15210_e10708_d_n4, assign15210_e10708_d_n5, assign15210_e10708_d_n6, assign15210_e10708_d_n7, assign15210_e10708_d_n8, assign15210_e10708_d_n9, assign15210_e10708_d_n10, assign15210_e10708_d_n11, assign15210_e10708_d_n12,) = {
    if ((((locals.var_guard1457 != 0.0) && (locals.var_guard1472 == 0.0)) && (locals.var_guard1473 == 0.0)) && (locals.var_guard1474 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign15210_e10708;
        locals.var_t1__blk1145_dn3 = assign15210_e10708_d_n3;
        locals.var_t1__blk1145_dn4 = assign15210_e10708_d_n4;
        locals.var_t1__blk1145_dn5 = assign15210_e10708_d_n5;
        locals.var_t1__blk1145_dn6 = assign15210_e10708_d_n6;
        locals.var_t1__blk1145_dn7 = assign15210_e10708_d_n7;
        locals.var_t1__blk1145_dn8 = assign15210_e10708_d_n8;
        locals.var_t1__blk1145_dn9 = assign15210_e10708_d_n9;
        locals.var_t1__blk1145_dn10 = assign15210_e10708_d_n10;
        locals.var_t1__blk1145_dn11 = assign15210_e10708_d_n11;
        locals.var_t1__blk1145_dn12 = assign15210_e10708_d_n12;

        let (assign15220_e10722, assign15220_e10722_d_n3, assign15220_e10722_d_n4, assign15220_e10722_d_n5, assign15220_e10722_d_n6, assign15220_e10722_d_n7, assign15220_e10722_d_n8, assign15220_e10722_d_n9, assign15220_e10722_d_n10, assign15220_e10722_d_n11, assign15220_e10722_d_n12,) = {
    if ((((locals.var_guard1457 != 0.0) && (locals.var_guard1472 == 0.0)) && (locals.var_guard1473 == 0.0)) && (locals.var_guard1474 == 0.0)) {
        let assign15220_e10720: f64 = (locals.var_t7__blk1151).exp();
        (assign15220_e10720, (assign15220_e10720 * locals.var_t7__blk1151_dn3), (assign15220_e10720 * locals.var_t7__blk1151_dn4), (assign15220_e10720 * locals.var_t7__blk1151_dn5), (assign15220_e10720 * locals.var_t7__blk1151_dn6), (assign15220_e10720 * locals.var_t7__blk1151_dn7), (assign15220_e10720 * locals.var_t7__blk1151_dn8), (assign15220_e10720 * locals.var_t7__blk1151_dn9), (assign15220_e10720 * locals.var_t7__blk1151_dn10), (assign15220_e10720 * locals.var_t7__blk1151_dn11), (assign15220_e10720 * locals.var_t7__blk1151_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign15220_e10722;
        locals.var_t1__blk1145_dn3 = assign15220_e10722_d_n3;
        locals.var_t1__blk1145_dn4 = assign15220_e10722_d_n4;
        locals.var_t1__blk1145_dn5 = assign15220_e10722_d_n5;
        locals.var_t1__blk1145_dn6 = assign15220_e10722_d_n6;
        locals.var_t1__blk1145_dn7 = assign15220_e10722_d_n7;
        locals.var_t1__blk1145_dn8 = assign15220_e10722_d_n8;
        locals.var_t1__blk1145_dn9 = assign15220_e10722_d_n9;
        locals.var_t1__blk1145_dn10 = assign15220_e10722_d_n10;
        locals.var_t1__blk1145_dn11 = assign15220_e10722_d_n11;
        locals.var_t1__blk1145_dn12 = assign15220_e10722_d_n12;

        let (assign15230_e10730, assign15230_e10730_d_n3, assign15230_e10730_d_n4, assign15230_e10730_d_n5, assign15230_e10730_d_n6, assign15230_e10730_d_n7, assign15230_e10730_d_n8, assign15230_e10730_d_n9, assign15230_e10730_d_n10, assign15230_e10730_d_n11, assign15230_e10730_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15230_e10726: f64 = (locals.var_pparam_b4soixrecd * locals.var_t4__blk1148);
        let assign15230_e10728: f64 = (assign15230_e10726 / locals.var_pparam_b4soinrecf0d);
        (assign15230_e10728, (((((locals.var_pparam_b4soixrecd_dn3 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk1148_dn3)) * locals.var_pparam_b4soinrecf0d) - (assign15230_e10726 * locals.var_pparam_b4soinrecf0d_dn3)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)), (((((locals.var_pparam_b4soixrecd_dn4 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk1148_dn4)) * locals.var_pparam_b4soinrecf0d) - (assign15230_e10726 * locals.var_pparam_b4soinrecf0d_dn4)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)), (((((locals.var_pparam_b4soixrecd_dn5 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk1148_dn5)) * locals.var_pparam_b4soinrecf0d) - (assign15230_e10726 * locals.var_pparam_b4soinrecf0d_dn5)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)), (((((locals.var_pparam_b4soixrecd_dn6 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk1148_dn6)) * locals.var_pparam_b4soinrecf0d) - (assign15230_e10726 * locals.var_pparam_b4soinrecf0d_dn6)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)), (((((locals.var_pparam_b4soixrecd_dn7 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk1148_dn7)) * locals.var_pparam_b4soinrecf0d) - (assign15230_e10726 * locals.var_pparam_b4soinrecf0d_dn7)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)), (((((locals.var_pparam_b4soixrecd_dn8 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk1148_dn8)) * locals.var_pparam_b4soinrecf0d) - (assign15230_e10726 * locals.var_pparam_b4soinrecf0d_dn8)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)), (((((locals.var_pparam_b4soixrecd_dn9 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk1148_dn9)) * locals.var_pparam_b4soinrecf0d) - (assign15230_e10726 * locals.var_pparam_b4soinrecf0d_dn9)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)), (((((locals.var_pparam_b4soixrecd_dn10 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk1148_dn10)) * locals.var_pparam_b4soinrecf0d) - (assign15230_e10726 * locals.var_pparam_b4soinrecf0d_dn10)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)), (((((locals.var_pparam_b4soixrecd_dn11 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk1148_dn11)) * locals.var_pparam_b4soinrecf0d) - (assign15230_e10726 * locals.var_pparam_b4soinrecf0d_dn11)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)), (((((locals.var_pparam_b4soixrecd_dn12 * locals.var_t4__blk1148) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk1148_dn12)) * locals.var_pparam_b4soinrecf0d) - (assign15230_e10726 * locals.var_pparam_b4soinrecf0d_dn12)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)),)
    } else {
        (locals.var_t7__blk1151, locals.var_t7__blk1151_dn3, locals.var_t7__blk1151_dn4, locals.var_t7__blk1151_dn5, locals.var_t7__blk1151_dn6, locals.var_t7__blk1151_dn7, locals.var_t7__blk1151_dn8, locals.var_t7__blk1151_dn9, locals.var_t7__blk1151_dn10, locals.var_t7__blk1151_dn11, locals.var_t7__blk1151_dn12,)
    }
};
        locals.var_t7__blk1151 = assign15230_e10730;
        locals.var_t7__blk1151_dn3 = assign15230_e10730_d_n3;
        locals.var_t7__blk1151_dn4 = assign15230_e10730_d_n4;
        locals.var_t7__blk1151_dn5 = assign15230_e10730_d_n5;
        locals.var_t7__blk1151_dn6 = assign15230_e10730_d_n6;
        locals.var_t7__blk1151_dn7 = assign15230_e10730_d_n7;
        locals.var_t7__blk1151_dn8 = assign15230_e10730_d_n8;
        locals.var_t7__blk1151_dn9 = assign15230_e10730_d_n9;
        locals.var_t7__blk1151_dn10 = assign15230_e10730_d_n10;
        locals.var_t7__blk1151_dn11 = assign15230_e10730_d_n11;
        locals.var_t7__blk1151_dn12 = assign15230_e10730_d_n12;

        let assign15240_e10733: f64 = if locals.var_t7__blk1151 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1475 = assign15240_e10733;

        let (assign15250_e10745, assign15250_e10745_d_n3, assign15250_e10745_d_n4, assign15250_e10745_d_n5, assign15250_e10745_d_n6, assign15250_e10745_d_n7, assign15250_e10745_d_n8, assign15250_e10745_d_n9, assign15250_e10745_d_n10, assign15250_e10745_d_n11, assign15250_e10745_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1475 != 0.0)) {
        let assign15250_e10740: f64 = (1.0 + locals.var_t7__blk1151);
        let assign15250_e10742: f64 = (assign15250_e10740 - 100.0);
        let assign15250_e10743: f64 = (2.688117142e43 * assign15250_e10742);
        (assign15250_e10743, (2.688117142e43 * locals.var_t7__blk1151_dn3), (2.688117142e43 * locals.var_t7__blk1151_dn4), (2.688117142e43 * locals.var_t7__blk1151_dn5), (2.688117142e43 * locals.var_t7__blk1151_dn6), (2.688117142e43 * locals.var_t7__blk1151_dn7), (2.688117142e43 * locals.var_t7__blk1151_dn8), (2.688117142e43 * locals.var_t7__blk1151_dn9), (2.688117142e43 * locals.var_t7__blk1151_dn10), (2.688117142e43 * locals.var_t7__blk1151_dn11), (2.688117142e43 * locals.var_t7__blk1151_dn12),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign15250_e10745;
        locals.var_t2__blk1146_dn3 = assign15250_e10745_d_n3;
        locals.var_t2__blk1146_dn4 = assign15250_e10745_d_n4;
        locals.var_t2__blk1146_dn5 = assign15250_e10745_d_n5;
        locals.var_t2__blk1146_dn6 = assign15250_e10745_d_n6;
        locals.var_t2__blk1146_dn7 = assign15250_e10745_d_n7;
        locals.var_t2__blk1146_dn8 = assign15250_e10745_d_n8;
        locals.var_t2__blk1146_dn9 = assign15250_e10745_d_n9;
        locals.var_t2__blk1146_dn10 = assign15250_e10745_d_n10;
        locals.var_t2__blk1146_dn11 = assign15250_e10745_d_n11;
        locals.var_t2__blk1146_dn12 = assign15250_e10745_d_n12;

        let assign15260_e10748: f64 = (-100.0);
        let assign15260_e10749: f64 = if locals.var_t7__blk1151 < assign15260_e10748 { 1.0 } else { 0.0 };
        locals.var_guard1476 = assign15260_e10749;

        let (assign15270_e10758, assign15270_e10758_d_n3, assign15270_e10758_d_n4, assign15270_e10758_d_n5, assign15270_e10758_d_n6, assign15270_e10758_d_n7, assign15270_e10758_d_n8, assign15270_e10758_d_n9, assign15270_e10758_d_n10, assign15270_e10758_d_n11, assign15270_e10758_d_n12,) = {
    if (((locals.var_guard1457 != 0.0) && (locals.var_guard1475 == 0.0)) && (locals.var_guard1476 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign15270_e10758;
        locals.var_t2__blk1146_dn3 = assign15270_e10758_d_n3;
        locals.var_t2__blk1146_dn4 = assign15270_e10758_d_n4;
        locals.var_t2__blk1146_dn5 = assign15270_e10758_d_n5;
        locals.var_t2__blk1146_dn6 = assign15270_e10758_d_n6;
        locals.var_t2__blk1146_dn7 = assign15270_e10758_d_n7;
        locals.var_t2__blk1146_dn8 = assign15270_e10758_d_n8;
        locals.var_t2__blk1146_dn9 = assign15270_e10758_d_n9;
        locals.var_t2__blk1146_dn10 = assign15270_e10758_d_n10;
        locals.var_t2__blk1146_dn11 = assign15270_e10758_d_n11;
        locals.var_t2__blk1146_dn12 = assign15270_e10758_d_n12;

        let (assign15280_e10769, assign15280_e10769_d_n3, assign15280_e10769_d_n4, assign15280_e10769_d_n5, assign15280_e10769_d_n6, assign15280_e10769_d_n7, assign15280_e10769_d_n8, assign15280_e10769_d_n9, assign15280_e10769_d_n10, assign15280_e10769_d_n11, assign15280_e10769_d_n12,) = {
    if (((locals.var_guard1457 != 0.0) && (locals.var_guard1475 == 0.0)) && (locals.var_guard1476 == 0.0)) {
        let assign15280_e10767: f64 = (locals.var_t7__blk1151).exp();
        (assign15280_e10767, (assign15280_e10767 * locals.var_t7__blk1151_dn3), (assign15280_e10767 * locals.var_t7__blk1151_dn4), (assign15280_e10767 * locals.var_t7__blk1151_dn5), (assign15280_e10767 * locals.var_t7__blk1151_dn6), (assign15280_e10767 * locals.var_t7__blk1151_dn7), (assign15280_e10767 * locals.var_t7__blk1151_dn8), (assign15280_e10767 * locals.var_t7__blk1151_dn9), (assign15280_e10767 * locals.var_t7__blk1151_dn10), (assign15280_e10767 * locals.var_t7__blk1151_dn11), (assign15280_e10767 * locals.var_t7__blk1151_dn12),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign15280_e10769;
        locals.var_t2__blk1146_dn3 = assign15280_e10769_d_n3;
        locals.var_t2__blk1146_dn4 = assign15280_e10769_d_n4;
        locals.var_t2__blk1146_dn5 = assign15280_e10769_d_n5;
        locals.var_t2__blk1146_dn6 = assign15280_e10769_d_n6;
        locals.var_t2__blk1146_dn7 = assign15280_e10769_d_n7;
        locals.var_t2__blk1146_dn8 = assign15280_e10769_d_n8;
        locals.var_t2__blk1146_dn9 = assign15280_e10769_d_n9;
        locals.var_t2__blk1146_dn10 = assign15280_e10769_d_n10;
        locals.var_t2__blk1146_dn11 = assign15280_e10769_d_n11;
        locals.var_t2__blk1146_dn12 = assign15280_e10769_d_n12;

        let (assign15290_e10775, assign15290_e10775_d_n3, assign15290_e10775_d_n4, assign15290_e10775_d_n5, assign15290_e10775_d_n6, assign15290_e10775_d_n7, assign15290_e10775_d_n8, assign15290_e10775_d_n9, assign15290_e10775_d_n10, assign15290_e10775_d_n11, assign15290_e10775_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15290_e10773: f64 = (locals.var_pparam_b4soiahlid * locals.var_t0__blk1144);
        (assign15290_e10773, ((locals.var_pparam_b4soiahlid_dn3 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk1144_dn3)), ((locals.var_pparam_b4soiahlid_dn4 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk1144_dn4)), ((locals.var_pparam_b4soiahlid_dn5 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk1144_dn5)), ((locals.var_pparam_b4soiahlid_dn6 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk1144_dn6)), ((locals.var_pparam_b4soiahlid_dn7 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk1144_dn7)), ((locals.var_pparam_b4soiahlid_dn8 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk1144_dn8)), ((locals.var_pparam_b4soiahlid_dn9 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk1144_dn9)), ((locals.var_pparam_b4soiahlid_dn10 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk1144_dn10)), ((locals.var_pparam_b4soiahlid_dn11 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk1144_dn11)), ((locals.var_pparam_b4soiahlid_dn12 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk1144_dn12)),)
    } else {
        (locals.var_ahlid, locals.var_ahlid_dn3, locals.var_ahlid_dn4, locals.var_ahlid_dn5, locals.var_ahlid_dn6, locals.var_ahlid_dn7, locals.var_ahlid_dn8, locals.var_ahlid_dn9, locals.var_ahlid_dn10, locals.var_ahlid_dn11, locals.var_ahlid_dn12,)
    }
};
        locals.var_ahlid = assign15290_e10775;
        locals.var_ahlid_dn3 = assign15290_e10775_d_n3;
        locals.var_ahlid_dn4 = assign15290_e10775_d_n4;
        locals.var_ahlid_dn5 = assign15290_e10775_d_n5;
        locals.var_ahlid_dn6 = assign15290_e10775_d_n6;
        locals.var_ahlid_dn7 = assign15290_e10775_d_n7;
        locals.var_ahlid_dn8 = assign15290_e10775_d_n8;
        locals.var_ahlid_dn9 = assign15290_e10775_d_n9;
        locals.var_ahlid_dn10 = assign15290_e10775_d_n10;
        locals.var_ahlid_dn11 = assign15290_e10775_d_n11;
        locals.var_ahlid_dn12 = assign15290_e10775_d_n12;

        let (assign15300_e10781, assign15300_e10781_d_n3, assign15300_e10781_d_n4, assign15300_e10781_d_n5, assign15300_e10781_d_n6, assign15300_e10781_d_n7, assign15300_e10781_d_n8, assign15300_e10781_d_n9, assign15300_e10781_d_n10, assign15300_e10781_d_n11, assign15300_e10781_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15300_e10779: f64 = (locals.var_pparam_b4soiidbjt * locals.var_t0__blk1144);
        (assign15300_e10779, ((locals.var_pparam_b4soiidbjt_dn3 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk1144_dn3)), ((locals.var_pparam_b4soiidbjt_dn4 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk1144_dn4)), ((locals.var_pparam_b4soiidbjt_dn5 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk1144_dn5)), ((locals.var_pparam_b4soiidbjt_dn6 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk1144_dn6)), ((locals.var_pparam_b4soiidbjt_dn7 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk1144_dn7)), ((locals.var_pparam_b4soiidbjt_dn8 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk1144_dn8)), ((locals.var_pparam_b4soiidbjt_dn9 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk1144_dn9)), ((locals.var_pparam_b4soiidbjt_dn10 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk1144_dn10)), ((locals.var_pparam_b4soiidbjt_dn11 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk1144_dn11)), ((locals.var_pparam_b4soiidbjt_dn12 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk1144_dn12)),)
    } else {
        (locals.var_jbjtd, locals.var_jbjtd_dn3, locals.var_jbjtd_dn4, locals.var_jbjtd_dn5, locals.var_jbjtd_dn6, locals.var_jbjtd_dn7, locals.var_jbjtd_dn8, locals.var_jbjtd_dn9, locals.var_jbjtd_dn10, locals.var_jbjtd_dn11, locals.var_jbjtd_dn12,)
    }
};
        locals.var_jbjtd = assign15300_e10781;
        locals.var_jbjtd_dn3 = assign15300_e10781_d_n3;
        locals.var_jbjtd_dn4 = assign15300_e10781_d_n4;
        locals.var_jbjtd_dn5 = assign15300_e10781_d_n5;
        locals.var_jbjtd_dn6 = assign15300_e10781_d_n6;
        locals.var_jbjtd_dn7 = assign15300_e10781_d_n7;
        locals.var_jbjtd_dn8 = assign15300_e10781_d_n8;
        locals.var_jbjtd_dn9 = assign15300_e10781_d_n9;
        locals.var_jbjtd_dn10 = assign15300_e10781_d_n10;
        locals.var_jbjtd_dn11 = assign15300_e10781_d_n11;
        locals.var_jbjtd_dn12 = assign15300_e10781_d_n12;

        let (assign15310_e10787, assign15310_e10787_d_n3, assign15310_e10787_d_n4, assign15310_e10787_d_n5, assign15310_e10787_d_n6, assign15310_e10787_d_n7, assign15310_e10787_d_n8, assign15310_e10787_d_n9, assign15310_e10787_d_n10, assign15310_e10787_d_n11, assign15310_e10787_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15310_e10785: f64 = (locals.var_pparam_b4soiiddif * locals.var_t1__blk1145);
        (assign15310_e10785, ((locals.var_pparam_b4soiiddif_dn3 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk1145_dn3)), ((locals.var_pparam_b4soiiddif_dn4 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk1145_dn4)), ((locals.var_pparam_b4soiiddif_dn5 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk1145_dn5)), ((locals.var_pparam_b4soiiddif_dn6 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk1145_dn6)), ((locals.var_pparam_b4soiiddif_dn7 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk1145_dn7)), ((locals.var_pparam_b4soiiddif_dn8 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk1145_dn8)), ((locals.var_pparam_b4soiiddif_dn9 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk1145_dn9)), ((locals.var_pparam_b4soiiddif_dn10 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk1145_dn10)), ((locals.var_pparam_b4soiiddif_dn11 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk1145_dn11)), ((locals.var_pparam_b4soiiddif_dn12 * locals.var_t1__blk1145) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk1145_dn12)),)
    } else {
        (locals.var_jdifd, locals.var_jdifd_dn3, locals.var_jdifd_dn4, locals.var_jdifd_dn5, locals.var_jdifd_dn6, locals.var_jdifd_dn7, locals.var_jdifd_dn8, locals.var_jdifd_dn9, locals.var_jdifd_dn10, locals.var_jdifd_dn11, locals.var_jdifd_dn12,)
    }
};
        locals.var_jdifd = assign15310_e10787;
        locals.var_jdifd_dn3 = assign15310_e10787_d_n3;
        locals.var_jdifd_dn4 = assign15310_e10787_d_n4;
        locals.var_jdifd_dn5 = assign15310_e10787_d_n5;
        locals.var_jdifd_dn6 = assign15310_e10787_d_n6;
        locals.var_jdifd_dn7 = assign15310_e10787_d_n7;
        locals.var_jdifd_dn8 = assign15310_e10787_d_n8;
        locals.var_jdifd_dn9 = assign15310_e10787_d_n9;
        locals.var_jdifd_dn10 = assign15310_e10787_d_n10;
        locals.var_jdifd_dn11 = assign15310_e10787_d_n11;
        locals.var_jdifd_dn12 = assign15310_e10787_d_n12;

        let (assign15320_e10793, assign15320_e10793_d_n3, assign15320_e10793_d_n4, assign15320_e10793_d_n5, assign15320_e10793_d_n6, assign15320_e10793_d_n7, assign15320_e10793_d_n8, assign15320_e10793_d_n9, assign15320_e10793_d_n10, assign15320_e10793_d_n11, assign15320_e10793_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15320_e10791: f64 = (locals.var_pparam_b4soiidrec * locals.var_t2__blk1146);
        (assign15320_e10791, ((locals.var_pparam_b4soiidrec_dn3 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk1146_dn3)), ((locals.var_pparam_b4soiidrec_dn4 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk1146_dn4)), ((locals.var_pparam_b4soiidrec_dn5 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk1146_dn5)), ((locals.var_pparam_b4soiidrec_dn6 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk1146_dn6)), ((locals.var_pparam_b4soiidrec_dn7 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk1146_dn7)), ((locals.var_pparam_b4soiidrec_dn8 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk1146_dn8)), ((locals.var_pparam_b4soiidrec_dn9 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk1146_dn9)), ((locals.var_pparam_b4soiidrec_dn10 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk1146_dn10)), ((locals.var_pparam_b4soiidrec_dn11 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk1146_dn11)), ((locals.var_pparam_b4soiidrec_dn12 * locals.var_t2__blk1146) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk1146_dn12)),)
    } else {
        (locals.var_jrecd, locals.var_jrecd_dn3, locals.var_jrecd_dn4, locals.var_jrecd_dn5, locals.var_jrecd_dn6, locals.var_jrecd_dn7, locals.var_jrecd_dn8, locals.var_jrecd_dn9, locals.var_jrecd_dn10, locals.var_jrecd_dn11, locals.var_jrecd_dn12,)
    }
};
        locals.var_jrecd = assign15320_e10793;
        locals.var_jrecd_dn3 = assign15320_e10793_d_n3;
        locals.var_jrecd_dn4 = assign15320_e10793_d_n4;
        locals.var_jrecd_dn5 = assign15320_e10793_d_n5;
        locals.var_jrecd_dn6 = assign15320_e10793_d_n6;
        locals.var_jrecd_dn7 = assign15320_e10793_d_n7;
        locals.var_jrecd_dn8 = assign15320_e10793_d_n8;
        locals.var_jrecd_dn9 = assign15320_e10793_d_n9;
        locals.var_jrecd_dn10 = assign15320_e10793_d_n10;
        locals.var_jrecd_dn11 = assign15320_e10793_d_n11;
        locals.var_jrecd_dn12 = assign15320_e10793_d_n12;

        let (assign15330_e10799, assign15330_e10799_d_n3, assign15330_e10799_d_n4, assign15330_e10799_d_n5, assign15330_e10799_d_n6, assign15330_e10799_d_n7, assign15330_e10799_d_n8, assign15330_e10799_d_n9, assign15330_e10799_d_n10, assign15330_e10799_d_n11, assign15330_e10799_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15330_e10797: f64 = (locals.var_pparam_b4soixtund * locals.var_t3__blk1147);
        (assign15330_e10797, ((locals.var_pparam_b4soixtund_dn3 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtund * locals.var_t3__blk1147_dn3)), ((locals.var_pparam_b4soixtund_dn4 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtund * locals.var_t3__blk1147_dn4)), ((locals.var_pparam_b4soixtund_dn5 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtund * locals.var_t3__blk1147_dn5)), ((locals.var_pparam_b4soixtund_dn6 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtund * locals.var_t3__blk1147_dn6)), ((locals.var_pparam_b4soixtund_dn7 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtund * locals.var_t3__blk1147_dn7)), ((locals.var_pparam_b4soixtund_dn8 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtund * locals.var_t3__blk1147_dn8)), ((locals.var_pparam_b4soixtund_dn9 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtund * locals.var_t3__blk1147_dn9)), ((locals.var_pparam_b4soixtund_dn10 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtund * locals.var_t3__blk1147_dn10)), ((locals.var_pparam_b4soixtund_dn11 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtund * locals.var_t3__blk1147_dn11)), ((locals.var_pparam_b4soixtund_dn12 * locals.var_t3__blk1147) + (locals.var_pparam_b4soixtund * locals.var_t3__blk1147_dn12)),)
    } else {
        (locals.var_t7__blk1151, locals.var_t7__blk1151_dn3, locals.var_t7__blk1151_dn4, locals.var_t7__blk1151_dn5, locals.var_t7__blk1151_dn6, locals.var_t7__blk1151_dn7, locals.var_t7__blk1151_dn8, locals.var_t7__blk1151_dn9, locals.var_t7__blk1151_dn10, locals.var_t7__blk1151_dn11, locals.var_t7__blk1151_dn12,)
    }
};
        locals.var_t7__blk1151 = assign15330_e10799;
        locals.var_t7__blk1151_dn3 = assign15330_e10799_d_n3;
        locals.var_t7__blk1151_dn4 = assign15330_e10799_d_n4;
        locals.var_t7__blk1151_dn5 = assign15330_e10799_d_n5;
        locals.var_t7__blk1151_dn6 = assign15330_e10799_d_n6;
        locals.var_t7__blk1151_dn7 = assign15330_e10799_d_n7;
        locals.var_t7__blk1151_dn8 = assign15330_e10799_d_n8;
        locals.var_t7__blk1151_dn9 = assign15330_e10799_d_n9;
        locals.var_t7__blk1151_dn10 = assign15330_e10799_d_n10;
        locals.var_t7__blk1151_dn11 = assign15330_e10799_d_n11;
        locals.var_t7__blk1151_dn12 = assign15330_e10799_d_n12;

        let assign15340_e10802: f64 = if locals.var_t7__blk1151 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1477 = assign15340_e10802;

        let (assign15350_e10814, assign15350_e10814_d_n3, assign15350_e10814_d_n4, assign15350_e10814_d_n5, assign15350_e10814_d_n6, assign15350_e10814_d_n7, assign15350_e10814_d_n8, assign15350_e10814_d_n9, assign15350_e10814_d_n10, assign15350_e10814_d_n11, assign15350_e10814_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1477 != 0.0)) {
        let assign15350_e10809: f64 = (1.0 + locals.var_t7__blk1151);
        let assign15350_e10811: f64 = (assign15350_e10809 - 100.0);
        let assign15350_e10812: f64 = (2.688117142e43 * assign15350_e10811);
        (assign15350_e10812, (2.688117142e43 * locals.var_t7__blk1151_dn3), (2.688117142e43 * locals.var_t7__blk1151_dn4), (2.688117142e43 * locals.var_t7__blk1151_dn5), (2.688117142e43 * locals.var_t7__blk1151_dn6), (2.688117142e43 * locals.var_t7__blk1151_dn7), (2.688117142e43 * locals.var_t7__blk1151_dn8), (2.688117142e43 * locals.var_t7__blk1151_dn9), (2.688117142e43 * locals.var_t7__blk1151_dn10), (2.688117142e43 * locals.var_t7__blk1151_dn11), (2.688117142e43 * locals.var_t7__blk1151_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign15350_e10814;
        locals.var_t0__blk1144_dn3 = assign15350_e10814_d_n3;
        locals.var_t0__blk1144_dn4 = assign15350_e10814_d_n4;
        locals.var_t0__blk1144_dn5 = assign15350_e10814_d_n5;
        locals.var_t0__blk1144_dn6 = assign15350_e10814_d_n6;
        locals.var_t0__blk1144_dn7 = assign15350_e10814_d_n7;
        locals.var_t0__blk1144_dn8 = assign15350_e10814_d_n8;
        locals.var_t0__blk1144_dn9 = assign15350_e10814_d_n9;
        locals.var_t0__blk1144_dn10 = assign15350_e10814_d_n10;
        locals.var_t0__blk1144_dn11 = assign15350_e10814_d_n11;
        locals.var_t0__blk1144_dn12 = assign15350_e10814_d_n12;

        let assign15360_e10817: f64 = (-100.0);
        let assign15360_e10818: f64 = if locals.var_t7__blk1151 < assign15360_e10817 { 1.0 } else { 0.0 };
        locals.var_guard1478 = assign15360_e10818;

        let (assign15370_e10827, assign15370_e10827_d_n3, assign15370_e10827_d_n4, assign15370_e10827_d_n5, assign15370_e10827_d_n6, assign15370_e10827_d_n7, assign15370_e10827_d_n8, assign15370_e10827_d_n9, assign15370_e10827_d_n10, assign15370_e10827_d_n11, assign15370_e10827_d_n12,) = {
    if (((locals.var_guard1457 != 0.0) && (locals.var_guard1477 == 0.0)) && (locals.var_guard1478 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign15370_e10827;
        locals.var_t0__blk1144_dn3 = assign15370_e10827_d_n3;
        locals.var_t0__blk1144_dn4 = assign15370_e10827_d_n4;
        locals.var_t0__blk1144_dn5 = assign15370_e10827_d_n5;
        locals.var_t0__blk1144_dn6 = assign15370_e10827_d_n6;
        locals.var_t0__blk1144_dn7 = assign15370_e10827_d_n7;
        locals.var_t0__blk1144_dn8 = assign15370_e10827_d_n8;
        locals.var_t0__blk1144_dn9 = assign15370_e10827_d_n9;
        locals.var_t0__blk1144_dn10 = assign15370_e10827_d_n10;
        locals.var_t0__blk1144_dn11 = assign15370_e10827_d_n11;
        locals.var_t0__blk1144_dn12 = assign15370_e10827_d_n12;

        let (assign15380_e10838, assign15380_e10838_d_n3, assign15380_e10838_d_n4, assign15380_e10838_d_n5, assign15380_e10838_d_n6, assign15380_e10838_d_n7, assign15380_e10838_d_n8, assign15380_e10838_d_n9, assign15380_e10838_d_n10, assign15380_e10838_d_n11, assign15380_e10838_d_n12,) = {
    if (((locals.var_guard1457 != 0.0) && (locals.var_guard1477 == 0.0)) && (locals.var_guard1478 == 0.0)) {
        let assign15380_e10836: f64 = (locals.var_t7__blk1151).exp();
        (assign15380_e10836, (assign15380_e10836 * locals.var_t7__blk1151_dn3), (assign15380_e10836 * locals.var_t7__blk1151_dn4), (assign15380_e10836 * locals.var_t7__blk1151_dn5), (assign15380_e10836 * locals.var_t7__blk1151_dn6), (assign15380_e10836 * locals.var_t7__blk1151_dn7), (assign15380_e10836 * locals.var_t7__blk1151_dn8), (assign15380_e10836 * locals.var_t7__blk1151_dn9), (assign15380_e10836 * locals.var_t7__blk1151_dn10), (assign15380_e10836 * locals.var_t7__blk1151_dn11), (assign15380_e10836 * locals.var_t7__blk1151_dn12),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign15380_e10838;
        locals.var_t0__blk1144_dn3 = assign15380_e10838_d_n3;
        locals.var_t0__blk1144_dn4 = assign15380_e10838_d_n4;
        locals.var_t0__blk1144_dn5 = assign15380_e10838_d_n5;
        locals.var_t0__blk1144_dn6 = assign15380_e10838_d_n6;
        locals.var_t0__blk1144_dn7 = assign15380_e10838_d_n7;
        locals.var_t0__blk1144_dn8 = assign15380_e10838_d_n8;
        locals.var_t0__blk1144_dn9 = assign15380_e10838_d_n9;
        locals.var_t0__blk1144_dn10 = assign15380_e10838_d_n10;
        locals.var_t0__blk1144_dn11 = assign15380_e10838_d_n11;
        locals.var_t0__blk1144_dn12 = assign15380_e10838_d_n12;

        let (assign15390_e10844, assign15390_e10844_d_n3, assign15390_e10844_d_n4, assign15390_e10844_d_n5, assign15390_e10844_d_n6, assign15390_e10844_d_n7, assign15390_e10844_d_n8, assign15390_e10844_d_n9, assign15390_e10844_d_n10, assign15390_e10844_d_n11, assign15390_e10844_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15390_e10842: f64 = (locals.var_pparam_b4soiidtun * locals.var_t0__blk1144);
        (assign15390_e10842, ((locals.var_pparam_b4soiidtun_dn3 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk1144_dn3)), ((locals.var_pparam_b4soiidtun_dn4 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk1144_dn4)), ((locals.var_pparam_b4soiidtun_dn5 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk1144_dn5)), ((locals.var_pparam_b4soiidtun_dn6 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk1144_dn6)), ((locals.var_pparam_b4soiidtun_dn7 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk1144_dn7)), ((locals.var_pparam_b4soiidtun_dn8 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk1144_dn8)), ((locals.var_pparam_b4soiidtun_dn9 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk1144_dn9)), ((locals.var_pparam_b4soiidtun_dn10 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk1144_dn10)), ((locals.var_pparam_b4soiidtun_dn11 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk1144_dn11)), ((locals.var_pparam_b4soiidtun_dn12 * locals.var_t0__blk1144) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk1144_dn12)),)
    } else {
        (locals.var_jtund, locals.var_jtund_dn3, locals.var_jtund_dn4, locals.var_jtund_dn5, locals.var_jtund_dn6, locals.var_jtund_dn7, locals.var_jtund_dn8, locals.var_jtund_dn9, locals.var_jtund_dn10, locals.var_jtund_dn11, locals.var_jtund_dn12,)
    }
};
        locals.var_jtund = assign15390_e10844;
        locals.var_jtund_dn3 = assign15390_e10844_d_n3;
        locals.var_jtund_dn4 = assign15390_e10844_d_n4;
        locals.var_jtund_dn5 = assign15390_e10844_d_n5;
        locals.var_jtund_dn6 = assign15390_e10844_d_n6;
        locals.var_jtund_dn7 = assign15390_e10844_d_n7;
        locals.var_jtund_dn8 = assign15390_e10844_d_n8;
        locals.var_jtund_dn9 = assign15390_e10844_d_n9;
        locals.var_jtund_dn10 = assign15390_e10844_d_n10;
        locals.var_jtund_dn11 = assign15390_e10844_d_n11;
        locals.var_jtund_dn12 = assign15390_e10844_d_n12;

    }

    pub(super) fn stamp_transient_block_31(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15400_e10852, assign15400_e10852_d_n3, assign15400_e10852_d_n4, assign15400_e10852_d_n5, assign15400_e10852_d_n6, assign15400_e10852_d_n7, assign15400_e10852_d_n8, assign15400_e10852_d_n9, assign15400_e10852_d_n10, assign15400_e10852_d_n11, assign15400_e10852_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15400_e10849: f64 = (locals.var_tempratio).powf(locals.var_pparam_b4soiute);
        let assign15400_e10850: f64 = (locals.var_pparam_b4soiu0 * assign15400_e10849);
        (assign15400_e10850, ((locals.var_pparam_b4soiu0_dn3 * assign15400_e10849) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn3 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign15400_e10849 * (locals.var_pparam_b4soiute_dn3 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiu0_dn4 * assign15400_e10849) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn4 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign15400_e10849 * (locals.var_pparam_b4soiute_dn4 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiu0_dn5 * assign15400_e10849) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn5 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign15400_e10849 * (locals.var_pparam_b4soiute_dn5 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiu0_dn6 * assign15400_e10849) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn6 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { if locals.var_pparam_b4soiute == 0.0 { 0.0 } else { (locals.var_pparam_b4soiute * ((locals.var_tempratio).powf(locals.var_pparam_b4soiute - 1.0) * locals.var_tempratio_dn6)) } } else { (assign15400_e10849 * ((locals.var_pparam_b4soiute_dn6 * (locals.var_tempratio).ln()) + (locals.var_pparam_b4soiute * (locals.var_tempratio_dn6 / locals.var_tempratio)))) })), ((locals.var_pparam_b4soiu0_dn7 * assign15400_e10849) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn7 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign15400_e10849 * (locals.var_pparam_b4soiute_dn7 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiu0_dn8 * assign15400_e10849) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn8 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign15400_e10849 * (locals.var_pparam_b4soiute_dn8 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiu0_dn9 * assign15400_e10849) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn9 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign15400_e10849 * (locals.var_pparam_b4soiute_dn9 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiu0_dn10 * assign15400_e10849) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn10 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign15400_e10849 * (locals.var_pparam_b4soiute_dn10 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiu0_dn11 * assign15400_e10849) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn11 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign15400_e10849 * (locals.var_pparam_b4soiute_dn11 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiu0_dn12 * assign15400_e10849) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn12 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign15400_e10849 * (locals.var_pparam_b4soiute_dn12 * (locals.var_tempratio).ln())) })),)
    } else {
        (locals.var_u0temp, locals.var_u0temp_dn3, locals.var_u0temp_dn4, locals.var_u0temp_dn5, locals.var_u0temp_dn6, locals.var_u0temp_dn7, locals.var_u0temp_dn8, locals.var_u0temp_dn9, locals.var_u0temp_dn10, locals.var_u0temp_dn11, locals.var_u0temp_dn12,)
    }
};
        locals.var_u0temp = assign15400_e10852;
        locals.var_u0temp_dn3 = assign15400_e10852_d_n3;
        locals.var_u0temp_dn4 = assign15400_e10852_d_n4;
        locals.var_u0temp_dn5 = assign15400_e10852_d_n5;
        locals.var_u0temp_dn6 = assign15400_e10852_d_n6;
        locals.var_u0temp_dn7 = assign15400_e10852_d_n7;
        locals.var_u0temp_dn8 = assign15400_e10852_d_n8;
        locals.var_u0temp_dn9 = assign15400_e10852_d_n9;
        locals.var_u0temp_dn10 = assign15400_e10852_d_n10;
        locals.var_u0temp_dn11 = assign15400_e10852_d_n11;
        locals.var_u0temp_dn12 = assign15400_e10852_d_n12;

        let assign15410_e10855: f64 = if p.p35 < 4.2 { 1.0 } else { 0.0 };
        locals.var_guard1479 = assign15410_e10855;

        let (assign15420_e10869, assign15420_e10869_d_n3, assign15420_e10869_d_n4, assign15420_e10869_d_n5, assign15420_e10869_d_n6, assign15420_e10869_d_n7, assign15420_e10869_d_n8, assign15420_e10869_d_n9, assign15420_e10869_d_n10, assign15420_e10869_d_n11, assign15420_e10869_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1479 != 0.0)) {
        let assign15420_e10863: f64 = (locals.var_b4soitku0 * locals.var_tempratio);
        let assign15420_e10864: f64 = (1.0 + assign15420_e10863);
        let assign15420_e10865: f64 = (locals.var_pparam_b4soiku0 * assign15420_e10864);
        let assign15420_e10867: f64 = (assign15420_e10865 + 1e-9);
        (assign15420_e10867, (locals.var_pparam_b4soiku0_dn3 * assign15420_e10864), (locals.var_pparam_b4soiku0_dn4 * assign15420_e10864), (locals.var_pparam_b4soiku0_dn5 * assign15420_e10864), ((locals.var_pparam_b4soiku0_dn6 * assign15420_e10864) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_tempratio_dn6))), (locals.var_pparam_b4soiku0_dn7 * assign15420_e10864), (locals.var_pparam_b4soiku0_dn8 * assign15420_e10864), (locals.var_pparam_b4soiku0_dn9 * assign15420_e10864), (locals.var_pparam_b4soiku0_dn10 * assign15420_e10864), (locals.var_pparam_b4soiku0_dn11 * assign15420_e10864), (locals.var_pparam_b4soiku0_dn12 * assign15420_e10864),)
    } else {
        (locals.var_ku0temp, locals.var_ku0temp_dn3, locals.var_ku0temp_dn4, locals.var_ku0temp_dn5, locals.var_ku0temp_dn6, locals.var_ku0temp_dn7, locals.var_ku0temp_dn8, locals.var_ku0temp_dn9, locals.var_ku0temp_dn10, locals.var_ku0temp_dn11, locals.var_ku0temp_dn12,)
    }
};
        locals.var_ku0temp = assign15420_e10869;
        locals.var_ku0temp_dn3 = assign15420_e10869_d_n3;
        locals.var_ku0temp_dn4 = assign15420_e10869_d_n4;
        locals.var_ku0temp_dn5 = assign15420_e10869_d_n5;
        locals.var_ku0temp_dn6 = assign15420_e10869_d_n6;
        locals.var_ku0temp_dn7 = assign15420_e10869_d_n7;
        locals.var_ku0temp_dn8 = assign15420_e10869_d_n8;
        locals.var_ku0temp_dn9 = assign15420_e10869_d_n9;
        locals.var_ku0temp_dn10 = assign15420_e10869_d_n10;
        locals.var_ku0temp_dn11 = assign15420_e10869_d_n11;
        locals.var_ku0temp_dn12 = assign15420_e10869_d_n12;

        let (assign15430_e10884, assign15430_e10884_d_n3, assign15430_e10884_d_n4, assign15430_e10884_d_n5, assign15430_e10884_d_n6, assign15430_e10884_d_n7, assign15430_e10884_d_n8, assign15430_e10884_d_n9, assign15430_e10884_d_n10, assign15430_e10884_d_n11, assign15430_e10884_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1479 == 0.0)) {
        let assign15430_e10878: f64 = (locals.var_b4soitku0 * locals.var_t3__blk1147);
        let assign15430_e10879: f64 = (1.0 + assign15430_e10878);
        let assign15430_e10880: f64 = (locals.var_pparam_b4soiku0 * assign15430_e10879);
        let assign15430_e10882: f64 = (assign15430_e10880 + 1e-9);
        (assign15430_e10882, ((locals.var_pparam_b4soiku0_dn3 * assign15430_e10879) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t3__blk1147_dn3))), ((locals.var_pparam_b4soiku0_dn4 * assign15430_e10879) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t3__blk1147_dn4))), ((locals.var_pparam_b4soiku0_dn5 * assign15430_e10879) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t3__blk1147_dn5))), ((locals.var_pparam_b4soiku0_dn6 * assign15430_e10879) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t3__blk1147_dn6))), ((locals.var_pparam_b4soiku0_dn7 * assign15430_e10879) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t3__blk1147_dn7))), ((locals.var_pparam_b4soiku0_dn8 * assign15430_e10879) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t3__blk1147_dn8))), ((locals.var_pparam_b4soiku0_dn9 * assign15430_e10879) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t3__blk1147_dn9))), ((locals.var_pparam_b4soiku0_dn10 * assign15430_e10879) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t3__blk1147_dn10))), ((locals.var_pparam_b4soiku0_dn11 * assign15430_e10879) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t3__blk1147_dn11))), ((locals.var_pparam_b4soiku0_dn12 * assign15430_e10879) + (locals.var_pparam_b4soiku0 * (locals.var_b4soitku0 * locals.var_t3__blk1147_dn12))),)
    } else {
        (locals.var_ku0temp, locals.var_ku0temp_dn3, locals.var_ku0temp_dn4, locals.var_ku0temp_dn5, locals.var_ku0temp_dn6, locals.var_ku0temp_dn7, locals.var_ku0temp_dn8, locals.var_ku0temp_dn9, locals.var_ku0temp_dn10, locals.var_ku0temp_dn11, locals.var_ku0temp_dn12,)
    }
};
        locals.var_ku0temp = assign15430_e10884;
        locals.var_ku0temp_dn3 = assign15430_e10884_d_n3;
        locals.var_ku0temp_dn4 = assign15430_e10884_d_n4;
        locals.var_ku0temp_dn5 = assign15430_e10884_d_n5;
        locals.var_ku0temp_dn6 = assign15430_e10884_d_n6;
        locals.var_ku0temp_dn7 = assign15430_e10884_d_n7;
        locals.var_ku0temp_dn8 = assign15430_e10884_d_n8;
        locals.var_ku0temp_dn9 = assign15430_e10884_d_n9;
        locals.var_ku0temp_dn10 = assign15430_e10884_d_n10;
        locals.var_ku0temp_dn11 = assign15430_e10884_d_n11;
        locals.var_ku0temp_dn12 = assign15430_e10884_d_n12;

        let (assign15440_e10890, assign15440_e10890_d_n3, assign15440_e10890_d_n4, assign15440_e10890_d_n5, assign15440_e10890_d_n6, assign15440_e10890_d_n7, assign15440_e10890_d_n8, assign15440_e10890_d_n9, assign15440_e10890_d_n10, assign15440_e10890_d_n11, assign15440_e10890_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15440_e10888: f64 = (locals.var_b4soiku0 * locals.var_pparam_b4soiinv_od_ref);
        (assign15440_e10888, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7__blk1151, locals.var_t7__blk1151_dn3, locals.var_t7__blk1151_dn4, locals.var_t7__blk1151_dn5, locals.var_t7__blk1151_dn6, locals.var_t7__blk1151_dn7, locals.var_t7__blk1151_dn8, locals.var_t7__blk1151_dn9, locals.var_t7__blk1151_dn10, locals.var_t7__blk1151_dn11, locals.var_t7__blk1151_dn12,)
    }
};
        locals.var_t7__blk1151 = assign15440_e10890;
        locals.var_t7__blk1151_dn3 = assign15440_e10890_d_n3;
        locals.var_t7__blk1151_dn4 = assign15440_e10890_d_n4;
        locals.var_t7__blk1151_dn5 = assign15440_e10890_d_n5;
        locals.var_t7__blk1151_dn6 = assign15440_e10890_d_n6;
        locals.var_t7__blk1151_dn7 = assign15440_e10890_d_n7;
        locals.var_t7__blk1151_dn8 = assign15440_e10890_d_n8;
        locals.var_t7__blk1151_dn9 = assign15440_e10890_d_n9;
        locals.var_t7__blk1151_dn10 = assign15440_e10890_d_n10;
        locals.var_t7__blk1151_dn11 = assign15440_e10890_d_n11;
        locals.var_t7__blk1151_dn12 = assign15440_e10890_d_n12;

        let (assign15450_e10896, assign15450_e10896_d_n3, assign15450_e10896_d_n4, assign15450_e10896_d_n5, assign15450_e10896_d_n6, assign15450_e10896_d_n7, assign15450_e10896_d_n8, assign15450_e10896_d_n9, assign15450_e10896_d_n10, assign15450_e10896_d_n11, assign15450_e10896_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15450_e10894: f64 = (locals.var_t7__blk1151 / locals.var_ku0temp);
        (assign15450_e10894, (((locals.var_t7__blk1151_dn3 * locals.var_ku0temp) - (locals.var_t7__blk1151 * locals.var_ku0temp_dn3)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t7__blk1151_dn4 * locals.var_ku0temp) - (locals.var_t7__blk1151 * locals.var_ku0temp_dn4)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t7__blk1151_dn5 * locals.var_ku0temp) - (locals.var_t7__blk1151 * locals.var_ku0temp_dn5)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t7__blk1151_dn6 * locals.var_ku0temp) - (locals.var_t7__blk1151 * locals.var_ku0temp_dn6)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t7__blk1151_dn7 * locals.var_ku0temp) - (locals.var_t7__blk1151 * locals.var_ku0temp_dn7)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t7__blk1151_dn8 * locals.var_ku0temp) - (locals.var_t7__blk1151 * locals.var_ku0temp_dn8)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t7__blk1151_dn9 * locals.var_ku0temp) - (locals.var_t7__blk1151 * locals.var_ku0temp_dn9)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t7__blk1151_dn10 * locals.var_ku0temp) - (locals.var_t7__blk1151 * locals.var_ku0temp_dn10)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t7__blk1151_dn11 * locals.var_ku0temp) - (locals.var_t7__blk1151 * locals.var_ku0temp_dn11)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t7__blk1151_dn12 * locals.var_ku0temp) - (locals.var_t7__blk1151 * locals.var_ku0temp_dn12)) / (locals.var_ku0temp * locals.var_ku0temp)),)
    } else {
        (locals.var_rho_ref, locals.var_rho_ref_dn3, locals.var_rho_ref_dn4, locals.var_rho_ref_dn5, locals.var_rho_ref_dn6, locals.var_rho_ref_dn7, locals.var_rho_ref_dn8, locals.var_rho_ref_dn9, locals.var_rho_ref_dn10, locals.var_rho_ref_dn11, locals.var_rho_ref_dn12,)
    }
};
        locals.var_rho_ref = assign15450_e10896;
        locals.var_rho_ref_dn3 = assign15450_e10896_d_n3;
        locals.var_rho_ref_dn4 = assign15450_e10896_d_n4;
        locals.var_rho_ref_dn5 = assign15450_e10896_d_n5;
        locals.var_rho_ref_dn6 = assign15450_e10896_d_n6;
        locals.var_rho_ref_dn7 = assign15450_e10896_d_n7;
        locals.var_rho_ref_dn8 = assign15450_e10896_d_n8;
        locals.var_rho_ref_dn9 = assign15450_e10896_d_n9;
        locals.var_rho_ref_dn10 = assign15450_e10896_d_n10;
        locals.var_rho_ref_dn11 = assign15450_e10896_d_n11;
        locals.var_rho_ref_dn12 = assign15450_e10896_d_n12;

        let (assign15460_e10902, assign15460_e10902_d_n3, assign15460_e10902_d_n4, assign15460_e10902_d_n5, assign15460_e10902_d_n6, assign15460_e10902_d_n7, assign15460_e10902_d_n8, assign15460_e10902_d_n9, assign15460_e10902_d_n10, assign15460_e10902_d_n11, assign15460_e10902_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15460_e10900: f64 = (locals.var_b4soiku0 * locals.var_b4soiinv_odeff);
        (assign15460_e10900, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk1148, locals.var_t4__blk1148_dn3, locals.var_t4__blk1148_dn4, locals.var_t4__blk1148_dn5, locals.var_t4__blk1148_dn6, locals.var_t4__blk1148_dn7, locals.var_t4__blk1148_dn8, locals.var_t4__blk1148_dn9, locals.var_t4__blk1148_dn10, locals.var_t4__blk1148_dn11, locals.var_t4__blk1148_dn12,)
    }
};
        locals.var_t4__blk1148 = assign15460_e10902;
        locals.var_t4__blk1148_dn3 = assign15460_e10902_d_n3;
        locals.var_t4__blk1148_dn4 = assign15460_e10902_d_n4;
        locals.var_t4__blk1148_dn5 = assign15460_e10902_d_n5;
        locals.var_t4__blk1148_dn6 = assign15460_e10902_d_n6;
        locals.var_t4__blk1148_dn7 = assign15460_e10902_d_n7;
        locals.var_t4__blk1148_dn8 = assign15460_e10902_d_n8;
        locals.var_t4__blk1148_dn9 = assign15460_e10902_d_n9;
        locals.var_t4__blk1148_dn10 = assign15460_e10902_d_n10;
        locals.var_t4__blk1148_dn11 = assign15460_e10902_d_n11;
        locals.var_t4__blk1148_dn12 = assign15460_e10902_d_n12;

        let (assign15470_e10908, assign15470_e10908_d_n3, assign15470_e10908_d_n4, assign15470_e10908_d_n5, assign15470_e10908_d_n6, assign15470_e10908_d_n7, assign15470_e10908_d_n8, assign15470_e10908_d_n9, assign15470_e10908_d_n10, assign15470_e10908_d_n11, assign15470_e10908_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15470_e10906: f64 = (locals.var_t4__blk1148 / locals.var_ku0temp);
        (assign15470_e10906, (((locals.var_t4__blk1148_dn3 * locals.var_ku0temp) - (locals.var_t4__blk1148 * locals.var_ku0temp_dn3)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t4__blk1148_dn4 * locals.var_ku0temp) - (locals.var_t4__blk1148 * locals.var_ku0temp_dn4)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t4__blk1148_dn5 * locals.var_ku0temp) - (locals.var_t4__blk1148 * locals.var_ku0temp_dn5)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t4__blk1148_dn6 * locals.var_ku0temp) - (locals.var_t4__blk1148 * locals.var_ku0temp_dn6)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t4__blk1148_dn7 * locals.var_ku0temp) - (locals.var_t4__blk1148 * locals.var_ku0temp_dn7)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t4__blk1148_dn8 * locals.var_ku0temp) - (locals.var_t4__blk1148 * locals.var_ku0temp_dn8)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t4__blk1148_dn9 * locals.var_ku0temp) - (locals.var_t4__blk1148 * locals.var_ku0temp_dn9)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t4__blk1148_dn10 * locals.var_ku0temp) - (locals.var_t4__blk1148 * locals.var_ku0temp_dn10)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t4__blk1148_dn11 * locals.var_ku0temp) - (locals.var_t4__blk1148 * locals.var_ku0temp_dn11)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t4__blk1148_dn12 * locals.var_ku0temp) - (locals.var_t4__blk1148 * locals.var_ku0temp_dn12)) / (locals.var_ku0temp * locals.var_ku0temp)),)
    } else {
        (locals.var_rho__blk1259, locals.var_rho__blk1259_dn3, locals.var_rho__blk1259_dn4, locals.var_rho__blk1259_dn5, locals.var_rho__blk1259_dn6, locals.var_rho__blk1259_dn7, locals.var_rho__blk1259_dn8, locals.var_rho__blk1259_dn9, locals.var_rho__blk1259_dn10, locals.var_rho__blk1259_dn11, locals.var_rho__blk1259_dn12,)
    }
};
        locals.var_rho__blk1259 = assign15470_e10908;
        locals.var_rho__blk1259_dn3 = assign15470_e10908_d_n3;
        locals.var_rho__blk1259_dn4 = assign15470_e10908_d_n4;
        locals.var_rho__blk1259_dn5 = assign15470_e10908_d_n5;
        locals.var_rho__blk1259_dn6 = assign15470_e10908_d_n6;
        locals.var_rho__blk1259_dn7 = assign15470_e10908_d_n7;
        locals.var_rho__blk1259_dn8 = assign15470_e10908_d_n8;
        locals.var_rho__blk1259_dn9 = assign15470_e10908_d_n9;
        locals.var_rho__blk1259_dn10 = assign15470_e10908_d_n10;
        locals.var_rho__blk1259_dn11 = assign15470_e10908_d_n11;
        locals.var_rho__blk1259_dn12 = assign15470_e10908_d_n12;

        let (assign15480_e10914, assign15480_e10914_d_n3, assign15480_e10914_d_n4, assign15480_e10914_d_n5, assign15480_e10914_d_n6, assign15480_e10914_d_n7, assign15480_e10914_d_n8, assign15480_e10914_d_n9, assign15480_e10914_d_n10, assign15480_e10914_d_n11, assign15480_e10914_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15480_e10912: f64 = (1.0 + locals.var_rho__blk1259);
        (assign15480_e10912, locals.var_rho__blk1259_dn3, locals.var_rho__blk1259_dn4, locals.var_rho__blk1259_dn5, locals.var_rho__blk1259_dn6, locals.var_rho__blk1259_dn7, locals.var_rho__blk1259_dn8, locals.var_rho__blk1259_dn9, locals.var_rho__blk1259_dn10, locals.var_rho__blk1259_dn11, locals.var_rho__blk1259_dn12,)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign15480_e10914;
        locals.var_t2__blk1146_dn3 = assign15480_e10914_d_n3;
        locals.var_t2__blk1146_dn4 = assign15480_e10914_d_n4;
        locals.var_t2__blk1146_dn5 = assign15480_e10914_d_n5;
        locals.var_t2__blk1146_dn6 = assign15480_e10914_d_n6;
        locals.var_t2__blk1146_dn7 = assign15480_e10914_d_n7;
        locals.var_t2__blk1146_dn8 = assign15480_e10914_d_n8;
        locals.var_t2__blk1146_dn9 = assign15480_e10914_d_n9;
        locals.var_t2__blk1146_dn10 = assign15480_e10914_d_n10;
        locals.var_t2__blk1146_dn11 = assign15480_e10914_d_n11;
        locals.var_t2__blk1146_dn12 = assign15480_e10914_d_n12;

        let (assign15490_e10920, assign15490_e10920_d_n3, assign15490_e10920_d_n4, assign15490_e10920_d_n5, assign15490_e10920_d_n6, assign15490_e10920_d_n7, assign15490_e10920_d_n8, assign15490_e10920_d_n9, assign15490_e10920_d_n10, assign15490_e10920_d_n11, assign15490_e10920_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15490_e10918: f64 = (1.0 + locals.var_rho_ref);
        (assign15490_e10918, locals.var_rho_ref_dn3, locals.var_rho_ref_dn4, locals.var_rho_ref_dn5, locals.var_rho_ref_dn6, locals.var_rho_ref_dn7, locals.var_rho_ref_dn8, locals.var_rho_ref_dn9, locals.var_rho_ref_dn10, locals.var_rho_ref_dn11, locals.var_rho_ref_dn12,)
    } else {
        (locals.var_t7__blk1151, locals.var_t7__blk1151_dn3, locals.var_t7__blk1151_dn4, locals.var_t7__blk1151_dn5, locals.var_t7__blk1151_dn6, locals.var_t7__blk1151_dn7, locals.var_t7__blk1151_dn8, locals.var_t7__blk1151_dn9, locals.var_t7__blk1151_dn10, locals.var_t7__blk1151_dn11, locals.var_t7__blk1151_dn12,)
    }
};
        locals.var_t7__blk1151 = assign15490_e10920;
        locals.var_t7__blk1151_dn3 = assign15490_e10920_d_n3;
        locals.var_t7__blk1151_dn4 = assign15490_e10920_d_n4;
        locals.var_t7__blk1151_dn5 = assign15490_e10920_d_n5;
        locals.var_t7__blk1151_dn6 = assign15490_e10920_d_n6;
        locals.var_t7__blk1151_dn7 = assign15490_e10920_d_n7;
        locals.var_t7__blk1151_dn8 = assign15490_e10920_d_n8;
        locals.var_t7__blk1151_dn9 = assign15490_e10920_d_n9;
        locals.var_t7__blk1151_dn10 = assign15490_e10920_d_n10;
        locals.var_t7__blk1151_dn11 = assign15490_e10920_d_n11;
        locals.var_t7__blk1151_dn12 = assign15490_e10920_d_n12;

        let (assign15500_e10926, assign15500_e10926_d_n3, assign15500_e10926_d_n4, assign15500_e10926_d_n5, assign15500_e10926_d_n6, assign15500_e10926_d_n7, assign15500_e10926_d_n8, assign15500_e10926_d_n9, assign15500_e10926_d_n10, assign15500_e10926_d_n11, assign15500_e10926_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15500_e10924: f64 = (locals.var_t2__blk1146 / locals.var_t7__blk1151);
        (assign15500_e10924, (((locals.var_t2__blk1146_dn3 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn3)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)), (((locals.var_t2__blk1146_dn4 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn4)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)), (((locals.var_t2__blk1146_dn5 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn5)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)), (((locals.var_t2__blk1146_dn6 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn6)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)), (((locals.var_t2__blk1146_dn7 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn7)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)), (((locals.var_t2__blk1146_dn8 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn8)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)), (((locals.var_t2__blk1146_dn9 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn9)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)), (((locals.var_t2__blk1146_dn10 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn10)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)), (((locals.var_t2__blk1146_dn11 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn11)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)), (((locals.var_t2__blk1146_dn12 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn12)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign15500_e10926;
        locals.var_t0__blk1144_dn3 = assign15500_e10926_d_n3;
        locals.var_t0__blk1144_dn4 = assign15500_e10926_d_n4;
        locals.var_t0__blk1144_dn5 = assign15500_e10926_d_n5;
        locals.var_t0__blk1144_dn6 = assign15500_e10926_d_n6;
        locals.var_t0__blk1144_dn7 = assign15500_e10926_d_n7;
        locals.var_t0__blk1144_dn8 = assign15500_e10926_d_n8;
        locals.var_t0__blk1144_dn9 = assign15500_e10926_d_n9;
        locals.var_t0__blk1144_dn10 = assign15500_e10926_d_n10;
        locals.var_t0__blk1144_dn11 = assign15500_e10926_d_n11;
        locals.var_t0__blk1144_dn12 = assign15500_e10926_d_n12;

        let (assign15510_e10932, assign15510_e10932_d_n3, assign15510_e10932_d_n4, assign15510_e10932_d_n5, assign15510_e10932_d_n6, assign15510_e10932_d_n7, assign15510_e10932_d_n8, assign15510_e10932_d_n9, assign15510_e10932_d_n10, assign15510_e10932_d_n11, assign15510_e10932_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15510_e10930: f64 = (locals.var_u0temp * locals.var_t0__blk1144);
        (assign15510_e10930, ((locals.var_u0temp_dn3 * locals.var_t0__blk1144) + (locals.var_u0temp * locals.var_t0__blk1144_dn3)), ((locals.var_u0temp_dn4 * locals.var_t0__blk1144) + (locals.var_u0temp * locals.var_t0__blk1144_dn4)), ((locals.var_u0temp_dn5 * locals.var_t0__blk1144) + (locals.var_u0temp * locals.var_t0__blk1144_dn5)), ((locals.var_u0temp_dn6 * locals.var_t0__blk1144) + (locals.var_u0temp * locals.var_t0__blk1144_dn6)), ((locals.var_u0temp_dn7 * locals.var_t0__blk1144) + (locals.var_u0temp * locals.var_t0__blk1144_dn7)), ((locals.var_u0temp_dn8 * locals.var_t0__blk1144) + (locals.var_u0temp * locals.var_t0__blk1144_dn8)), ((locals.var_u0temp_dn9 * locals.var_t0__blk1144) + (locals.var_u0temp * locals.var_t0__blk1144_dn9)), ((locals.var_u0temp_dn10 * locals.var_t0__blk1144) + (locals.var_u0temp * locals.var_t0__blk1144_dn10)), ((locals.var_u0temp_dn11 * locals.var_t0__blk1144) + (locals.var_u0temp * locals.var_t0__blk1144_dn11)), ((locals.var_u0temp_dn12 * locals.var_t0__blk1144) + (locals.var_u0temp * locals.var_t0__blk1144_dn12)),)
    } else {
        (locals.var_u0temp, locals.var_u0temp_dn3, locals.var_u0temp_dn4, locals.var_u0temp_dn5, locals.var_u0temp_dn6, locals.var_u0temp_dn7, locals.var_u0temp_dn8, locals.var_u0temp_dn9, locals.var_u0temp_dn10, locals.var_u0temp_dn11, locals.var_u0temp_dn12,)
    }
};
        locals.var_u0temp = assign15510_e10932;
        locals.var_u0temp_dn3 = assign15510_e10932_d_n3;
        locals.var_u0temp_dn4 = assign15510_e10932_d_n4;
        locals.var_u0temp_dn5 = assign15510_e10932_d_n5;
        locals.var_u0temp_dn6 = assign15510_e10932_d_n6;
        locals.var_u0temp_dn7 = assign15510_e10932_d_n7;
        locals.var_u0temp_dn8 = assign15510_e10932_d_n8;
        locals.var_u0temp_dn9 = assign15510_e10932_d_n9;
        locals.var_u0temp_dn10 = assign15510_e10932_d_n10;
        locals.var_u0temp_dn11 = assign15510_e10932_d_n11;
        locals.var_u0temp_dn12 = assign15510_e10932_d_n12;

        let (assign15520_e10940, assign15520_e10940_d_n3, assign15520_e10940_d_n4, assign15520_e10940_d_n5, assign15520_e10940_d_n6, assign15520_e10940_d_n7, assign15520_e10940_d_n8, assign15520_e10940_d_n9, assign15520_e10940_d_n10, assign15520_e10940_d_n11, assign15520_e10940_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15520_e10937: f64 = (locals.var_pparam_b4soiat * locals.var_t3__blk1147);
        let assign15520_e10938: f64 = (locals.var_pparam_b4soivsat - assign15520_e10937);
        (assign15520_e10938, (locals.var_pparam_b4soivsat_dn3 - ((locals.var_pparam_b4soiat_dn3 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiat * locals.var_t3__blk1147_dn3))), (locals.var_pparam_b4soivsat_dn4 - ((locals.var_pparam_b4soiat_dn4 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiat * locals.var_t3__blk1147_dn4))), (locals.var_pparam_b4soivsat_dn5 - ((locals.var_pparam_b4soiat_dn5 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiat * locals.var_t3__blk1147_dn5))), (locals.var_pparam_b4soivsat_dn6 - ((locals.var_pparam_b4soiat_dn6 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiat * locals.var_t3__blk1147_dn6))), (locals.var_pparam_b4soivsat_dn7 - ((locals.var_pparam_b4soiat_dn7 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiat * locals.var_t3__blk1147_dn7))), (locals.var_pparam_b4soivsat_dn8 - ((locals.var_pparam_b4soiat_dn8 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiat * locals.var_t3__blk1147_dn8))), (locals.var_pparam_b4soivsat_dn9 - ((locals.var_pparam_b4soiat_dn9 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiat * locals.var_t3__blk1147_dn9))), (locals.var_pparam_b4soivsat_dn10 - ((locals.var_pparam_b4soiat_dn10 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiat * locals.var_t3__blk1147_dn10))), (locals.var_pparam_b4soivsat_dn11 - ((locals.var_pparam_b4soiat_dn11 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiat * locals.var_t3__blk1147_dn11))), (locals.var_pparam_b4soivsat_dn12 - ((locals.var_pparam_b4soiat_dn12 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiat * locals.var_t3__blk1147_dn12))),)
    } else {
        (locals.var_vsattemp, locals.var_vsattemp_dn3, locals.var_vsattemp_dn4, locals.var_vsattemp_dn5, locals.var_vsattemp_dn6, locals.var_vsattemp_dn7, locals.var_vsattemp_dn8, locals.var_vsattemp_dn9, locals.var_vsattemp_dn10, locals.var_vsattemp_dn11, locals.var_vsattemp_dn12,)
    }
};
        locals.var_vsattemp = assign15520_e10940;
        locals.var_vsattemp_dn3 = assign15520_e10940_d_n3;
        locals.var_vsattemp_dn4 = assign15520_e10940_d_n4;
        locals.var_vsattemp_dn5 = assign15520_e10940_d_n5;
        locals.var_vsattemp_dn6 = assign15520_e10940_d_n6;
        locals.var_vsattemp_dn7 = assign15520_e10940_d_n7;
        locals.var_vsattemp_dn8 = assign15520_e10940_d_n8;
        locals.var_vsattemp_dn9 = assign15520_e10940_d_n9;
        locals.var_vsattemp_dn10 = assign15520_e10940_d_n10;
        locals.var_vsattemp_dn11 = assign15520_e10940_d_n11;
        locals.var_vsattemp_dn12 = assign15520_e10940_d_n12;

        let (assign15530_e10948, assign15530_e10948_d_n3, assign15530_e10948_d_n4, assign15530_e10948_d_n5, assign15530_e10948_d_n6, assign15530_e10948_d_n7, assign15530_e10948_d_n8, assign15530_e10948_d_n9, assign15530_e10948_d_n10, assign15530_e10948_d_n11, assign15530_e10948_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15530_e10945: f64 = (locals.var_b4soikvsat * locals.var_rho__blk1259);
        let assign15530_e10946: f64 = (1.0 + assign15530_e10945);
        (assign15530_e10946, (locals.var_b4soikvsat * locals.var_rho__blk1259_dn3), (locals.var_b4soikvsat * locals.var_rho__blk1259_dn4), (locals.var_b4soikvsat * locals.var_rho__blk1259_dn5), (locals.var_b4soikvsat * locals.var_rho__blk1259_dn6), (locals.var_b4soikvsat * locals.var_rho__blk1259_dn7), (locals.var_b4soikvsat * locals.var_rho__blk1259_dn8), (locals.var_b4soikvsat * locals.var_rho__blk1259_dn9), (locals.var_b4soikvsat * locals.var_rho__blk1259_dn10), (locals.var_b4soikvsat * locals.var_rho__blk1259_dn11), (locals.var_b4soikvsat * locals.var_rho__blk1259_dn12),)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign15530_e10948;
        locals.var_t2__blk1146_dn3 = assign15530_e10948_d_n3;
        locals.var_t2__blk1146_dn4 = assign15530_e10948_d_n4;
        locals.var_t2__blk1146_dn5 = assign15530_e10948_d_n5;
        locals.var_t2__blk1146_dn6 = assign15530_e10948_d_n6;
        locals.var_t2__blk1146_dn7 = assign15530_e10948_d_n7;
        locals.var_t2__blk1146_dn8 = assign15530_e10948_d_n8;
        locals.var_t2__blk1146_dn9 = assign15530_e10948_d_n9;
        locals.var_t2__blk1146_dn10 = assign15530_e10948_d_n10;
        locals.var_t2__blk1146_dn11 = assign15530_e10948_d_n11;
        locals.var_t2__blk1146_dn12 = assign15530_e10948_d_n12;

        let (assign15540_e10956, assign15540_e10956_d_n3, assign15540_e10956_d_n4, assign15540_e10956_d_n5, assign15540_e10956_d_n6, assign15540_e10956_d_n7, assign15540_e10956_d_n8, assign15540_e10956_d_n9, assign15540_e10956_d_n10, assign15540_e10956_d_n11, assign15540_e10956_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15540_e10953: f64 = (locals.var_b4soikvsat * locals.var_rho_ref);
        let assign15540_e10954: f64 = (1.0 + assign15540_e10953);
        (assign15540_e10954, (locals.var_b4soikvsat * locals.var_rho_ref_dn3), (locals.var_b4soikvsat * locals.var_rho_ref_dn4), (locals.var_b4soikvsat * locals.var_rho_ref_dn5), (locals.var_b4soikvsat * locals.var_rho_ref_dn6), (locals.var_b4soikvsat * locals.var_rho_ref_dn7), (locals.var_b4soikvsat * locals.var_rho_ref_dn8), (locals.var_b4soikvsat * locals.var_rho_ref_dn9), (locals.var_b4soikvsat * locals.var_rho_ref_dn10), (locals.var_b4soikvsat * locals.var_rho_ref_dn11), (locals.var_b4soikvsat * locals.var_rho_ref_dn12),)
    } else {
        (locals.var_t7__blk1151, locals.var_t7__blk1151_dn3, locals.var_t7__blk1151_dn4, locals.var_t7__blk1151_dn5, locals.var_t7__blk1151_dn6, locals.var_t7__blk1151_dn7, locals.var_t7__blk1151_dn8, locals.var_t7__blk1151_dn9, locals.var_t7__blk1151_dn10, locals.var_t7__blk1151_dn11, locals.var_t7__blk1151_dn12,)
    }
};
        locals.var_t7__blk1151 = assign15540_e10956;
        locals.var_t7__blk1151_dn3 = assign15540_e10956_d_n3;
        locals.var_t7__blk1151_dn4 = assign15540_e10956_d_n4;
        locals.var_t7__blk1151_dn5 = assign15540_e10956_d_n5;
        locals.var_t7__blk1151_dn6 = assign15540_e10956_d_n6;
        locals.var_t7__blk1151_dn7 = assign15540_e10956_d_n7;
        locals.var_t7__blk1151_dn8 = assign15540_e10956_d_n8;
        locals.var_t7__blk1151_dn9 = assign15540_e10956_d_n9;
        locals.var_t7__blk1151_dn10 = assign15540_e10956_d_n10;
        locals.var_t7__blk1151_dn11 = assign15540_e10956_d_n11;
        locals.var_t7__blk1151_dn12 = assign15540_e10956_d_n12;

        let (assign15550_e10962, assign15550_e10962_d_n3, assign15550_e10962_d_n4, assign15550_e10962_d_n5, assign15550_e10962_d_n6, assign15550_e10962_d_n7, assign15550_e10962_d_n8, assign15550_e10962_d_n9, assign15550_e10962_d_n10, assign15550_e10962_d_n11, assign15550_e10962_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15550_e10960: f64 = (locals.var_t2__blk1146 / locals.var_t7__blk1151);
        (assign15550_e10960, (((locals.var_t2__blk1146_dn3 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn3)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)), (((locals.var_t2__blk1146_dn4 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn4)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)), (((locals.var_t2__blk1146_dn5 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn5)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)), (((locals.var_t2__blk1146_dn6 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn6)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)), (((locals.var_t2__blk1146_dn7 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn7)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)), (((locals.var_t2__blk1146_dn8 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn8)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)), (((locals.var_t2__blk1146_dn9 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn9)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)), (((locals.var_t2__blk1146_dn10 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn10)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)), (((locals.var_t2__blk1146_dn11 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn11)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)), (((locals.var_t2__blk1146_dn12 * locals.var_t7__blk1151) - (locals.var_t2__blk1146 * locals.var_t7__blk1151_dn12)) / (locals.var_t7__blk1151 * locals.var_t7__blk1151)),)
    } else {
        (locals.var_t0__blk1144, locals.var_t0__blk1144_dn3, locals.var_t0__blk1144_dn4, locals.var_t0__blk1144_dn5, locals.var_t0__blk1144_dn6, locals.var_t0__blk1144_dn7, locals.var_t0__blk1144_dn8, locals.var_t0__blk1144_dn9, locals.var_t0__blk1144_dn10, locals.var_t0__blk1144_dn11, locals.var_t0__blk1144_dn12,)
    }
};
        locals.var_t0__blk1144 = assign15550_e10962;
        locals.var_t0__blk1144_dn3 = assign15550_e10962_d_n3;
        locals.var_t0__blk1144_dn4 = assign15550_e10962_d_n4;
        locals.var_t0__blk1144_dn5 = assign15550_e10962_d_n5;
        locals.var_t0__blk1144_dn6 = assign15550_e10962_d_n6;
        locals.var_t0__blk1144_dn7 = assign15550_e10962_d_n7;
        locals.var_t0__blk1144_dn8 = assign15550_e10962_d_n8;
        locals.var_t0__blk1144_dn9 = assign15550_e10962_d_n9;
        locals.var_t0__blk1144_dn10 = assign15550_e10962_d_n10;
        locals.var_t0__blk1144_dn11 = assign15550_e10962_d_n11;
        locals.var_t0__blk1144_dn12 = assign15550_e10962_d_n12;

        let (assign15560_e10968, assign15560_e10968_d_n3, assign15560_e10968_d_n4, assign15560_e10968_d_n5, assign15560_e10968_d_n6, assign15560_e10968_d_n7, assign15560_e10968_d_n8, assign15560_e10968_d_n9, assign15560_e10968_d_n10, assign15560_e10968_d_n11, assign15560_e10968_d_n12,) = {
    if (locals.var_guard1457 != 0.0) {
        let assign15560_e10966: f64 = (locals.var_vsattemp * locals.var_t0__blk1144);
        (assign15560_e10966, ((locals.var_vsattemp_dn3 * locals.var_t0__blk1144) + (locals.var_vsattemp * locals.var_t0__blk1144_dn3)), ((locals.var_vsattemp_dn4 * locals.var_t0__blk1144) + (locals.var_vsattemp * locals.var_t0__blk1144_dn4)), ((locals.var_vsattemp_dn5 * locals.var_t0__blk1144) + (locals.var_vsattemp * locals.var_t0__blk1144_dn5)), ((locals.var_vsattemp_dn6 * locals.var_t0__blk1144) + (locals.var_vsattemp * locals.var_t0__blk1144_dn6)), ((locals.var_vsattemp_dn7 * locals.var_t0__blk1144) + (locals.var_vsattemp * locals.var_t0__blk1144_dn7)), ((locals.var_vsattemp_dn8 * locals.var_t0__blk1144) + (locals.var_vsattemp * locals.var_t0__blk1144_dn8)), ((locals.var_vsattemp_dn9 * locals.var_t0__blk1144) + (locals.var_vsattemp * locals.var_t0__blk1144_dn9)), ((locals.var_vsattemp_dn10 * locals.var_t0__blk1144) + (locals.var_vsattemp * locals.var_t0__blk1144_dn10)), ((locals.var_vsattemp_dn11 * locals.var_t0__blk1144) + (locals.var_vsattemp * locals.var_t0__blk1144_dn11)), ((locals.var_vsattemp_dn12 * locals.var_t0__blk1144) + (locals.var_vsattemp * locals.var_t0__blk1144_dn12)),)
    } else {
        (locals.var_vsattemp, locals.var_vsattemp_dn3, locals.var_vsattemp_dn4, locals.var_vsattemp_dn5, locals.var_vsattemp_dn6, locals.var_vsattemp_dn7, locals.var_vsattemp_dn8, locals.var_vsattemp_dn9, locals.var_vsattemp_dn10, locals.var_vsattemp_dn11, locals.var_vsattemp_dn12,)
    }
};
        locals.var_vsattemp = assign15560_e10968;
        locals.var_vsattemp_dn3 = assign15560_e10968_d_n3;
        locals.var_vsattemp_dn4 = assign15560_e10968_d_n4;
        locals.var_vsattemp_dn5 = assign15560_e10968_d_n5;
        locals.var_vsattemp_dn6 = assign15560_e10968_d_n6;
        locals.var_vsattemp_dn7 = assign15560_e10968_d_n7;
        locals.var_vsattemp_dn8 = assign15560_e10968_d_n8;
        locals.var_vsattemp_dn9 = assign15560_e10968_d_n9;
        locals.var_vsattemp_dn10 = assign15560_e10968_d_n10;
        locals.var_vsattemp_dn11 = assign15560_e10968_d_n11;
        locals.var_vsattemp_dn12 = assign15560_e10968_d_n12;

        let assign15570_e10971: f64 = if locals.var_b4soirdsmod != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1480 = assign15570_e10971;

        let (assign15580_e10983, assign15580_e10983_d_n3, assign15580_e10983_d_n4, assign15580_e10983_d_n5, assign15580_e10983_d_n6, assign15580_e10983_d_n7, assign15580_e10983_d_n8, assign15580_e10983_d_n9, assign15580_e10983_d_n10, assign15580_e10983_d_n11, assign15580_e10983_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1480 != 0.0)) {
        let assign15580_e10978: f64 = (locals.var_pparam_b4soiprt * locals.var_t3__blk1147);
        let assign15580_e10979: f64 = (locals.var_pparam_b4soirdsw + assign15580_e10978);
        let assign15580_e10981: f64 = (assign15580_e10979 / locals.var_pparam_b4soirds0denom);
        (assign15580_e10981, ((((locals.var_pparam_b4soirdsw_dn3 + ((locals.var_pparam_b4soiprt_dn3 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn3))) * locals.var_pparam_b4soirds0denom) - (assign15580_e10979 * locals.var_pparam_b4soirds0denom_dn3)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)), ((((locals.var_pparam_b4soirdsw_dn4 + ((locals.var_pparam_b4soiprt_dn4 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn4))) * locals.var_pparam_b4soirds0denom) - (assign15580_e10979 * locals.var_pparam_b4soirds0denom_dn4)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)), ((((locals.var_pparam_b4soirdsw_dn5 + ((locals.var_pparam_b4soiprt_dn5 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn5))) * locals.var_pparam_b4soirds0denom) - (assign15580_e10979 * locals.var_pparam_b4soirds0denom_dn5)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)), ((((locals.var_pparam_b4soirdsw_dn6 + ((locals.var_pparam_b4soiprt_dn6 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn6))) * locals.var_pparam_b4soirds0denom) - (assign15580_e10979 * locals.var_pparam_b4soirds0denom_dn6)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)), ((((locals.var_pparam_b4soirdsw_dn7 + ((locals.var_pparam_b4soiprt_dn7 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn7))) * locals.var_pparam_b4soirds0denom) - (assign15580_e10979 * locals.var_pparam_b4soirds0denom_dn7)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)), ((((locals.var_pparam_b4soirdsw_dn8 + ((locals.var_pparam_b4soiprt_dn8 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn8))) * locals.var_pparam_b4soirds0denom) - (assign15580_e10979 * locals.var_pparam_b4soirds0denom_dn8)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)), ((((locals.var_pparam_b4soirdsw_dn9 + ((locals.var_pparam_b4soiprt_dn9 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn9))) * locals.var_pparam_b4soirds0denom) - (assign15580_e10979 * locals.var_pparam_b4soirds0denom_dn9)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)), ((((locals.var_pparam_b4soirdsw_dn10 + ((locals.var_pparam_b4soiprt_dn10 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn10))) * locals.var_pparam_b4soirds0denom) - (assign15580_e10979 * locals.var_pparam_b4soirds0denom_dn10)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)), ((((locals.var_pparam_b4soirdsw_dn11 + ((locals.var_pparam_b4soiprt_dn11 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn11))) * locals.var_pparam_b4soirds0denom) - (assign15580_e10979 * locals.var_pparam_b4soirds0denom_dn11)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)), ((((locals.var_pparam_b4soirdsw_dn12 + ((locals.var_pparam_b4soiprt_dn12 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn12))) * locals.var_pparam_b4soirds0denom) - (assign15580_e10979 * locals.var_pparam_b4soirds0denom_dn12)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)),)
    } else {
        (locals.var_rds0, locals.var_rds0_dn3, locals.var_rds0_dn4, locals.var_rds0_dn5, locals.var_rds0_dn6, locals.var_rds0_dn7, locals.var_rds0_dn8, locals.var_rds0_dn9, locals.var_rds0_dn10, locals.var_rds0_dn11, locals.var_rds0_dn12,)
    }
};
        locals.var_rds0 = assign15580_e10983;
        locals.var_rds0_dn3 = assign15580_e10983_d_n3;
        locals.var_rds0_dn4 = assign15580_e10983_d_n4;
        locals.var_rds0_dn5 = assign15580_e10983_d_n5;
        locals.var_rds0_dn6 = assign15580_e10983_d_n6;
        locals.var_rds0_dn7 = assign15580_e10983_d_n7;
        locals.var_rds0_dn8 = assign15580_e10983_d_n8;
        locals.var_rds0_dn9 = assign15580_e10983_d_n9;
        locals.var_rds0_dn10 = assign15580_e10983_d_n10;
        locals.var_rds0_dn11 = assign15580_e10983_d_n11;
        locals.var_rds0_dn12 = assign15580_e10983_d_n12;

        let (assign15590_e10989, assign15590_e10989_d_n3, assign15590_e10989_d_n4, assign15590_e10989_d_n5, assign15590_e10989_d_n6, assign15590_e10989_d_n7, assign15590_e10989_d_n8, assign15590_e10989_d_n9, assign15590_e10989_d_n10, assign15590_e10989_d_n11, assign15590_e10989_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1480 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rd0, locals.var_rd0_dn3, locals.var_rd0_dn4, locals.var_rd0_dn5, locals.var_rd0_dn6, locals.var_rd0_dn7, locals.var_rd0_dn8, locals.var_rd0_dn9, locals.var_rd0_dn10, locals.var_rd0_dn11, locals.var_rd0_dn12,)
    }
};
        locals.var_rd0 = assign15590_e10989;
        locals.var_rd0_dn3 = assign15590_e10989_d_n3;
        locals.var_rd0_dn4 = assign15590_e10989_d_n4;
        locals.var_rd0_dn5 = assign15590_e10989_d_n5;
        locals.var_rd0_dn6 = assign15590_e10989_d_n6;
        locals.var_rd0_dn7 = assign15590_e10989_d_n7;
        locals.var_rd0_dn8 = assign15590_e10989_d_n8;
        locals.var_rd0_dn9 = assign15590_e10989_d_n9;
        locals.var_rd0_dn10 = assign15590_e10989_d_n10;
        locals.var_rd0_dn11 = assign15590_e10989_d_n11;
        locals.var_rd0_dn12 = assign15590_e10989_d_n12;

        let (assign15600_e10995, assign15600_e10995_d_n3, assign15600_e10995_d_n4, assign15600_e10995_d_n5, assign15600_e10995_d_n6, assign15600_e10995_d_n7, assign15600_e10995_d_n8, assign15600_e10995_d_n9, assign15600_e10995_d_n10, assign15600_e10995_d_n11, assign15600_e10995_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1480 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rs0, locals.var_rs0_dn3, locals.var_rs0_dn4, locals.var_rs0_dn5, locals.var_rs0_dn6, locals.var_rs0_dn7, locals.var_rs0_dn8, locals.var_rs0_dn9, locals.var_rs0_dn10, locals.var_rs0_dn11, locals.var_rs0_dn12,)
    }
};
        locals.var_rs0 = assign15600_e10995;
        locals.var_rs0_dn3 = assign15600_e10995_d_n3;
        locals.var_rs0_dn4 = assign15600_e10995_d_n4;
        locals.var_rs0_dn5 = assign15600_e10995_d_n5;
        locals.var_rs0_dn6 = assign15600_e10995_d_n6;
        locals.var_rs0_dn7 = assign15600_e10995_d_n7;
        locals.var_rs0_dn8 = assign15600_e10995_d_n8;
        locals.var_rs0_dn9 = assign15600_e10995_d_n9;
        locals.var_rs0_dn10 = assign15600_e10995_d_n10;
        locals.var_rs0_dn11 = assign15600_e10995_d_n11;
        locals.var_rs0_dn12 = assign15600_e10995_d_n12;

        let (assign15610_e11002, assign15610_e11002_d_n3, assign15610_e11002_d_n4, assign15610_e11002_d_n5, assign15610_e11002_d_n6, assign15610_e11002_d_n7, assign15610_e11002_d_n8, assign15610_e11002_d_n9, assign15610_e11002_d_n10, assign15610_e11002_d_n11, assign15610_e11002_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1480 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rds0, locals.var_rds0_dn3, locals.var_rds0_dn4, locals.var_rds0_dn5, locals.var_rds0_dn6, locals.var_rds0_dn7, locals.var_rds0_dn8, locals.var_rds0_dn9, locals.var_rds0_dn10, locals.var_rds0_dn11, locals.var_rds0_dn12,)
    }
};
        locals.var_rds0 = assign15610_e11002;
        locals.var_rds0_dn3 = assign15610_e11002_d_n3;
        locals.var_rds0_dn4 = assign15610_e11002_d_n4;
        locals.var_rds0_dn5 = assign15610_e11002_d_n5;
        locals.var_rds0_dn6 = assign15610_e11002_d_n6;
        locals.var_rds0_dn7 = assign15610_e11002_d_n7;
        locals.var_rds0_dn8 = assign15610_e11002_d_n8;
        locals.var_rds0_dn9 = assign15610_e11002_d_n9;
        locals.var_rds0_dn10 = assign15610_e11002_d_n10;
        locals.var_rds0_dn11 = assign15610_e11002_d_n11;
        locals.var_rds0_dn12 = assign15610_e11002_d_n12;

        let (assign15620_e11011, assign15620_e11011_d_n3, assign15620_e11011_d_n4, assign15620_e11011_d_n5, assign15620_e11011_d_n6, assign15620_e11011_d_n7, assign15620_e11011_d_n8, assign15620_e11011_d_n9, assign15620_e11011_d_n10, assign15620_e11011_d_n11, assign15620_e11011_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1480 == 0.0)) {
        let assign15620_e11009: f64 = (locals.var_pparam_b4soirds0denom * locals.var_b4soinf);
        (assign15620_e11009, (locals.var_pparam_b4soirds0denom_dn3 * locals.var_b4soinf), (locals.var_pparam_b4soirds0denom_dn4 * locals.var_b4soinf), (locals.var_pparam_b4soirds0denom_dn5 * locals.var_b4soinf), (locals.var_pparam_b4soirds0denom_dn6 * locals.var_b4soinf), (locals.var_pparam_b4soirds0denom_dn7 * locals.var_b4soinf), (locals.var_pparam_b4soirds0denom_dn8 * locals.var_b4soinf), (locals.var_pparam_b4soirds0denom_dn9 * locals.var_b4soinf), (locals.var_pparam_b4soirds0denom_dn10 * locals.var_b4soinf), (locals.var_pparam_b4soirds0denom_dn11 * locals.var_b4soinf), (locals.var_pparam_b4soirds0denom_dn12 * locals.var_b4soinf),)
    } else {
        (locals.var_powweffwr__blk1393, locals.var_powweffwr__blk1393_dn3, locals.var_powweffwr__blk1393_dn4, locals.var_powweffwr__blk1393_dn5, locals.var_powweffwr__blk1393_dn6, locals.var_powweffwr__blk1393_dn7, locals.var_powweffwr__blk1393_dn8, locals.var_powweffwr__blk1393_dn9, locals.var_powweffwr__blk1393_dn10, locals.var_powweffwr__blk1393_dn11, locals.var_powweffwr__blk1393_dn12,)
    }
};
        locals.var_powweffwr__blk1393 = assign15620_e11011;
        locals.var_powweffwr__blk1393_dn3 = assign15620_e11011_d_n3;
        locals.var_powweffwr__blk1393_dn4 = assign15620_e11011_d_n4;
        locals.var_powweffwr__blk1393_dn5 = assign15620_e11011_d_n5;
        locals.var_powweffwr__blk1393_dn6 = assign15620_e11011_d_n6;
        locals.var_powweffwr__blk1393_dn7 = assign15620_e11011_d_n7;
        locals.var_powweffwr__blk1393_dn8 = assign15620_e11011_d_n8;
        locals.var_powweffwr__blk1393_dn9 = assign15620_e11011_d_n9;
        locals.var_powweffwr__blk1393_dn10 = assign15620_e11011_d_n10;
        locals.var_powweffwr__blk1393_dn11 = assign15620_e11011_d_n11;
        locals.var_powweffwr__blk1393_dn12 = assign15620_e11011_d_n12;

        let (assign15630_e11020, assign15630_e11020_d_n3, assign15630_e11020_d_n4, assign15630_e11020_d_n5, assign15630_e11020_d_n6, assign15630_e11020_d_n7, assign15630_e11020_d_n8, assign15630_e11020_d_n9, assign15630_e11020_d_n10, assign15630_e11020_d_n11, assign15630_e11020_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1480 == 0.0)) {
        let assign15630_e11018: f64 = (locals.var_pparam_b4soiprt * locals.var_t3__blk1147);
        (assign15630_e11018, ((locals.var_pparam_b4soiprt_dn3 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn3)), ((locals.var_pparam_b4soiprt_dn4 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn4)), ((locals.var_pparam_b4soiprt_dn5 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn5)), ((locals.var_pparam_b4soiprt_dn6 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn6)), ((locals.var_pparam_b4soiprt_dn7 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn7)), ((locals.var_pparam_b4soiprt_dn8 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn8)), ((locals.var_pparam_b4soiprt_dn9 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn9)), ((locals.var_pparam_b4soiprt_dn10 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn10)), ((locals.var_pparam_b4soiprt_dn11 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn11)), ((locals.var_pparam_b4soiprt_dn12 * locals.var_t3__blk1147) + (locals.var_pparam_b4soiprt * locals.var_t3__blk1147_dn12)),)
    } else {
        (locals.var_t10__blk1154, locals.var_t10__blk1154_dn3, locals.var_t10__blk1154_dn4, locals.var_t10__blk1154_dn5, locals.var_t10__blk1154_dn6, locals.var_t10__blk1154_dn7, locals.var_t10__blk1154_dn8, locals.var_t10__blk1154_dn9, locals.var_t10__blk1154_dn10, locals.var_t10__blk1154_dn11, locals.var_t10__blk1154_dn12,)
    }
};
        locals.var_t10__blk1154 = assign15630_e11020;
        locals.var_t10__blk1154_dn3 = assign15630_e11020_d_n3;
        locals.var_t10__blk1154_dn4 = assign15630_e11020_d_n4;
        locals.var_t10__blk1154_dn5 = assign15630_e11020_d_n5;
        locals.var_t10__blk1154_dn6 = assign15630_e11020_d_n6;
        locals.var_t10__blk1154_dn7 = assign15630_e11020_d_n7;
        locals.var_t10__blk1154_dn8 = assign15630_e11020_d_n8;
        locals.var_t10__blk1154_dn9 = assign15630_e11020_d_n9;
        locals.var_t10__blk1154_dn10 = assign15630_e11020_d_n10;
        locals.var_t10__blk1154_dn11 = assign15630_e11020_d_n11;
        locals.var_t10__blk1154_dn12 = assign15630_e11020_d_n12;

        let (assign15640_e11029, assign15640_e11029_d_n3, assign15640_e11029_d_n4, assign15640_e11029_d_n5, assign15640_e11029_d_n6, assign15640_e11029_d_n7, assign15640_e11029_d_n8, assign15640_e11029_d_n9, assign15640_e11029_d_n10, assign15640_e11029_d_n11, assign15640_e11029_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1480 == 0.0)) {
        let assign15640_e11027: f64 = (locals.var_pparam_b4soirdw + locals.var_t10__blk1154);
        (assign15640_e11027, (locals.var_pparam_b4soirdw_dn3 + locals.var_t10__blk1154_dn3), (locals.var_pparam_b4soirdw_dn4 + locals.var_t10__blk1154_dn4), (locals.var_pparam_b4soirdw_dn5 + locals.var_t10__blk1154_dn5), (locals.var_pparam_b4soirdw_dn6 + locals.var_t10__blk1154_dn6), (locals.var_pparam_b4soirdw_dn7 + locals.var_t10__blk1154_dn7), (locals.var_pparam_b4soirdw_dn8 + locals.var_t10__blk1154_dn8), (locals.var_pparam_b4soirdw_dn9 + locals.var_t10__blk1154_dn9), (locals.var_pparam_b4soirdw_dn10 + locals.var_t10__blk1154_dn10), (locals.var_pparam_b4soirdw_dn11 + locals.var_t10__blk1154_dn11), (locals.var_pparam_b4soirdw_dn12 + locals.var_t10__blk1154_dn12),)
    } else {
        (locals.var_t1__blk1145, locals.var_t1__blk1145_dn3, locals.var_t1__blk1145_dn4, locals.var_t1__blk1145_dn5, locals.var_t1__blk1145_dn6, locals.var_t1__blk1145_dn7, locals.var_t1__blk1145_dn8, locals.var_t1__blk1145_dn9, locals.var_t1__blk1145_dn10, locals.var_t1__blk1145_dn11, locals.var_t1__blk1145_dn12,)
    }
};
        locals.var_t1__blk1145 = assign15640_e11029;
        locals.var_t1__blk1145_dn3 = assign15640_e11029_d_n3;
        locals.var_t1__blk1145_dn4 = assign15640_e11029_d_n4;
        locals.var_t1__blk1145_dn5 = assign15640_e11029_d_n5;
        locals.var_t1__blk1145_dn6 = assign15640_e11029_d_n6;
        locals.var_t1__blk1145_dn7 = assign15640_e11029_d_n7;
        locals.var_t1__blk1145_dn8 = assign15640_e11029_d_n8;
        locals.var_t1__blk1145_dn9 = assign15640_e11029_d_n9;
        locals.var_t1__blk1145_dn10 = assign15640_e11029_d_n10;
        locals.var_t1__blk1145_dn11 = assign15640_e11029_d_n11;
        locals.var_t1__blk1145_dn12 = assign15640_e11029_d_n12;

        let (assign15650_e11038, assign15650_e11038_d_n3, assign15650_e11038_d_n4, assign15650_e11038_d_n5, assign15650_e11038_d_n6, assign15650_e11038_d_n7, assign15650_e11038_d_n8, assign15650_e11038_d_n9, assign15650_e11038_d_n10, assign15650_e11038_d_n11, assign15650_e11038_d_n12,) = {
    if ((locals.var_guard1457 != 0.0) && (locals.var_guard1480 == 0.0)) {
        let assign15650_e11036: f64 = (locals.var_b4soirdwmin + locals.var_t10__blk1154);
        (assign15650_e11036, locals.var_t10__blk1154_dn3, locals.var_t10__blk1154_dn4, locals.var_t10__blk1154_dn5, locals.var_t10__blk1154_dn6, locals.var_t10__blk1154_dn7, locals.var_t10__blk1154_dn8, locals.var_t10__blk1154_dn9, locals.var_t10__blk1154_dn10, locals.var_t10__blk1154_dn11, locals.var_t10__blk1154_dn12,)
    } else {
        (locals.var_t2__blk1146, locals.var_t2__blk1146_dn3, locals.var_t2__blk1146_dn4, locals.var_t2__blk1146_dn5, locals.var_t2__blk1146_dn6, locals.var_t2__blk1146_dn7, locals.var_t2__blk1146_dn8, locals.var_t2__blk1146_dn9, locals.var_t2__blk1146_dn10, locals.var_t2__blk1146_dn11, locals.var_t2__blk1146_dn12,)
    }
};
        locals.var_t2__blk1146 = assign15650_e11038;
        locals.var_t2__blk1146_dn3 = assign15650_e11038_d_n3;
        locals.var_t2__blk1146_dn4 = assign15650_e11038_d_n4;
        locals.var_t2__blk1146_dn5 = assign15650_e11038_d_n5;
        locals.var_t2__blk1146_dn6 = assign15650_e11038_d_n6;
        locals.var_t2__blk1146_dn7 = assign15650_e11038_d_n7;
        locals.var_t2__blk1146_dn8 = assign15650_e11038_d_n8;
        locals.var_t2__blk1146_dn9 = assign15650_e11038_d_n9;
        locals.var_t2__blk1146_dn10 = assign15650_e11038_d_n10;
        locals.var_t2__blk1146_dn11 = assign15650_e11038_d_n11;
        locals.var_t2__blk1146_dn12 = assign15650_e11038_d_n12;

    }
}
