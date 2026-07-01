#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_54(
        locals: &mut StampLocals,
    ) {
        let (assign19930_e19851, assign19930_e19851_d_n4, assign19930_e19851_d_n6, assign19930_e19851_d_n7, assign19930_e19851_d_n8, assign19930_e19851_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 != 0.0)) {
        let assign19930_e19839: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign19930_e19844: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign19930_e19845: f64 = (locals.var_q_d1_ln + assign19930_e19844);
        let assign19930_e19846: f64 = (locals.var_q_d1_qsq * assign19930_e19845);
        let assign19930_e19847: f64 = (assign19930_e19839 - assign19930_e19846);
        let assign19930_e19849: f64 = (assign19930_e19847 / locals.var_q_qsq);
        (assign19930_e19849, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign19930_e19845) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign19930_e19847 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign19930_e19845) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign19930_e19847 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign19930_e19845) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign19930_e19847 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign19930_e19845) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign19930_e19847 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign19930_e19845) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign19930_e19847 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign19930_e19851;
        locals.var_q_d2_ln_dn4 = assign19930_e19851_d_n4;
        locals.var_q_d2_ln_dn6 = assign19930_e19851_d_n6;
        locals.var_q_d2_ln_dn7 = assign19930_e19851_d_n7;
        locals.var_q_d2_ln_dn8 = assign19930_e19851_d_n8;
        locals.var_q_d2_ln_dn9 = assign19930_e19851_d_n9;
        locals.var_q_d2_ln_rv = 0.0;

        let assign19940_e19854: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard650 = assign19940_e19854;
        locals.var_guard650_rv = 0.0;

        let (assign19950_e19867, assign19950_e19867_d_n4, assign19950_e19867_d_n6, assign19950_e19867_d_n7, assign19950_e19867_d_n8, assign19950_e19867_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign19950_e19864: f64 = (locals.var_q_qsq).abs();
        let assign19950_e19865: f64 = (assign19950_e19864).sqrt();
        (assign19950_e19865, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign19950_e19865)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign19950_e19865)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign19950_e19865)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign19950_e19865)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign19950_e19865)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign19950_e19867;
        locals.var_q_rac_qsq_dn4 = assign19950_e19867_d_n4;
        locals.var_q_rac_qsq_dn6 = assign19950_e19867_d_n6;
        locals.var_q_rac_qsq_dn7 = assign19950_e19867_d_n7;
        locals.var_q_rac_qsq_dn8 = assign19950_e19867_d_n8;
        locals.var_q_rac_qsq_dn9 = assign19950_e19867_d_n9;
        locals.var_q_rac_qsq_rv = 0.0;

        let (assign19960_e19880, assign19960_e19880_d_n4, assign19960_e19880_d_n6, assign19960_e19880_d_n7, assign19960_e19880_d_n8, assign19960_e19880_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign19960_e19877: f64 = (-locals.var_q_rac_qsq);
        let assign19960_e19878: f64 = (assign19960_e19877).exp();
        (assign19960_e19878, (assign19960_e19878 * (-locals.var_q_rac_qsq_dn4)), (assign19960_e19878 * (-locals.var_q_rac_qsq_dn6)), (assign19960_e19878 * (-locals.var_q_rac_qsq_dn7)), (assign19960_e19878 * (-locals.var_q_rac_qsq_dn8)), (assign19960_e19878 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign19960_e19880;
        locals.var_q_invexpq_dn4 = assign19960_e19880_d_n4;
        locals.var_q_invexpq_dn6 = assign19960_e19880_d_n6;
        locals.var_q_invexpq_dn7 = assign19960_e19880_d_n7;
        locals.var_q_invexpq_dn8 = assign19960_e19880_d_n8;
        locals.var_q_invexpq_dn9 = assign19960_e19880_d_n9;
        locals.var_q_invexpq_rv = 0.0;

        let (assign19970_e19899, assign19970_e19899_d_n4, assign19970_e19899_d_n6, assign19970_e19899_d_n7, assign19970_e19899_d_n8, assign19970_e19899_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign19970_e19892: f64 = (1.0 + locals.var_q_invexpq);
        let assign19970_e19893: f64 = (locals.var_q_rac_qsq * assign19970_e19892);
        let assign19970_e19896: f64 = (1.0 - locals.var_q_invexpq);
        let assign19970_e19897: f64 = (assign19970_e19893 / assign19970_e19896);
        (assign19970_e19897, (((((locals.var_q_rac_qsq_dn4 * assign19970_e19892) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign19970_e19896) - (assign19970_e19893 * (-locals.var_q_invexpq_dn4))) / (assign19970_e19896 * assign19970_e19896)), (((((locals.var_q_rac_qsq_dn6 * assign19970_e19892) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign19970_e19896) - (assign19970_e19893 * (-locals.var_q_invexpq_dn6))) / (assign19970_e19896 * assign19970_e19896)), (((((locals.var_q_rac_qsq_dn7 * assign19970_e19892) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign19970_e19896) - (assign19970_e19893 * (-locals.var_q_invexpq_dn7))) / (assign19970_e19896 * assign19970_e19896)), (((((locals.var_q_rac_qsq_dn8 * assign19970_e19892) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign19970_e19896) - (assign19970_e19893 * (-locals.var_q_invexpq_dn8))) / (assign19970_e19896 * assign19970_e19896)), (((((locals.var_q_rac_qsq_dn9 * assign19970_e19892) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign19970_e19896) - (assign19970_e19893 * (-locals.var_q_invexpq_dn9))) / (assign19970_e19896 * assign19970_e19896)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign19970_e19899;
        locals.var_q_qcoth_dn4 = assign19970_e19899_d_n4;
        locals.var_q_qcoth_dn6 = assign19970_e19899_d_n6;
        locals.var_q_qcoth_dn7 = assign19970_e19899_d_n7;
        locals.var_q_qcoth_dn8 = assign19970_e19899_d_n8;
        locals.var_q_qcoth_dn9 = assign19970_e19899_d_n9;
        locals.var_q_qcoth_rv = 0.0;

        let (assign19980_e19914, assign19980_e19914_d_n4, assign19980_e19914_d_n6, assign19980_e19914_d_n7, assign19980_e19914_d_n8, assign19980_e19914_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign19980_e19910: f64 = (0.25 * locals.var_q_d1_qsq);
        let assign19980_e19912: f64 = (assign19980_e19910 / locals.var_q_qsq);
        (assign19980_e19912, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign19980_e19910 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign19980_e19910 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign19980_e19910 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign19980_e19910 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign19980_e19910 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign19980_e19914;
        locals.var_q_temp1_dn4 = assign19980_e19914_d_n4;
        locals.var_q_temp1_dn6 = assign19980_e19914_d_n6;
        locals.var_q_temp1_dn7 = assign19980_e19914_d_n7;
        locals.var_q_temp1_dn8 = assign19980_e19914_d_n8;
        locals.var_q_temp1_dn9 = assign19980_e19914_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign19990_e19933, assign19990_e19933_d_n4, assign19990_e19933_d_n6, assign19990_e19933_d_n7, assign19990_e19933_d_n8, assign19990_e19933_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign19990_e19927: f64 = (2.0 - locals.var_q_qcoth);
        let assign19990_e19928: f64 = (locals.var_q_qcoth * assign19990_e19927);
        let assign19990_e19929: f64 = (locals.var_q_qsq + assign19990_e19928);
        let assign19990_e19931: f64 = (assign19990_e19929 * locals.var_q_temp1);
        (assign19990_e19931, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign19990_e19927) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign19990_e19929 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign19990_e19927) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign19990_e19929 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign19990_e19927) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign19990_e19929 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign19990_e19927) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign19990_e19929 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign19990_e19927) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign19990_e19929 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign19990_e19933;
        locals.var_q_d1_qcoth_dn4 = assign19990_e19933_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign19990_e19933_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign19990_e19933_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign19990_e19933_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign19990_e19933_d_n9;
        locals.var_q_d1_qcoth_rv = 0.0;

        let (assign20000_e19960, assign20000_e19960_d_n4, assign20000_e19960_d_n6, assign20000_e19960_d_n7, assign20000_e19960_d_n8, assign20000_e19960_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign20000_e19945: f64 = (2.0 * locals.var_q_d1_qcoth);
        let assign20000_e19948: f64 = (1.0 + locals.var_q_qcoth);
        let assign20000_e19949: f64 = (assign20000_e19945 * assign20000_e19948);
        let assign20000_e19950: f64 = (locals.var_q_d1_qsq - assign20000_e19949);
        let assign20000_e19952: f64 = (assign20000_e19950 * locals.var_q_temp1);
        let assign20000_e19955: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
        let assign20000_e19957: f64 = (assign20000_e19955 / locals.var_q_d1_qsq);
        let assign20000_e19958: f64 = (assign20000_e19952 + assign20000_e19957);
        (assign20000_e19958, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign20000_e19948) + (assign20000_e19945 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign20000_e19950 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign20000_e19955 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign20000_e19948) + (assign20000_e19945 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign20000_e19950 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign20000_e19955 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign20000_e19948) + (assign20000_e19945 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign20000_e19950 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign20000_e19955 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign20000_e19948) + (assign20000_e19945 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign20000_e19950 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign20000_e19955 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign20000_e19948) + (assign20000_e19945 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign20000_e19950 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign20000_e19955 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign20000_e19960;
        locals.var_q_d2_qcoth_dn4 = assign20000_e19960_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign20000_e19960_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign20000_e19960_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign20000_e19960_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign20000_e19960_d_n9;
        locals.var_q_d2_qcoth_rv = 0.0;

        let (assign20010_e19975, assign20010_e19975_d_n4, assign20010_e19975_d_n6, assign20010_e19975_d_n7, assign20010_e19975_d_n8, assign20010_e19975_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign20010_e19972: f64 = (0.5 * locals.var_q_qcoth);
        let assign20010_e19973: f64 = (1.0 - assign20010_e19972);
        (assign20010_e19973, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20010_e19975;
        locals.var_q_temp2_dn4 = assign20010_e19975_d_n4;
        locals.var_q_temp2_dn6 = assign20010_e19975_d_n6;
        locals.var_q_temp2_dn7 = assign20010_e19975_d_n7;
        locals.var_q_temp2_dn8 = assign20010_e19975_d_n8;
        locals.var_q_temp2_dn9 = assign20010_e19975_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign20020_e19990, assign20020_e19990_d_n4, assign20020_e19990_d_n6, assign20020_e19990_d_n7, assign20020_e19990_d_n8, assign20020_e19990_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign20020_e19986: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
        let assign20020_e19988: f64 = (assign20020_e19986 * locals.var_q_temp2);
        (assign20020_e19988, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign20020_e19986 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign20020_e19986 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign20020_e19986 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign20020_e19986 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign20020_e19986 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign20020_e19990;
        locals.var_q_d1_ln_dn4 = assign20020_e19990_d_n4;
        locals.var_q_d1_ln_dn6 = assign20020_e19990_d_n6;
        locals.var_q_d1_ln_dn7 = assign20020_e19990_d_n7;
        locals.var_q_d1_ln_dn8 = assign20020_e19990_d_n8;
        locals.var_q_d1_ln_dn9 = assign20020_e19990_d_n9;
        locals.var_q_d1_ln_rv = 0.0;

        let (assign20030_e20013, assign20030_e20013_d_n4, assign20030_e20013_d_n6, assign20030_e20013_d_n7, assign20030_e20013_d_n8, assign20030_e20013_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 != 0.0)) {
        let assign20030_e20001: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
        let assign20030_e20006: f64 = (0.5 * locals.var_q_d1_qcoth);
        let assign20030_e20007: f64 = (locals.var_q_d1_ln + assign20030_e20006);
        let assign20030_e20008: f64 = (locals.var_q_d1_qsq * assign20030_e20007);
        let assign20030_e20009: f64 = (assign20030_e20001 - assign20030_e20008);
        let assign20030_e20011: f64 = (assign20030_e20009 / locals.var_q_qsq);
        (assign20030_e20011, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign20030_e20007) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign20030_e20009 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign20030_e20007) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign20030_e20009 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign20030_e20007) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign20030_e20009 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign20030_e20007) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign20030_e20009 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign20030_e20007) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign20030_e20009 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign20030_e20013;
        locals.var_q_d2_ln_dn4 = assign20030_e20013_d_n4;
        locals.var_q_d2_ln_dn6 = assign20030_e20013_d_n6;
        locals.var_q_d2_ln_dn7 = assign20030_e20013_d_n7;
        locals.var_q_d2_ln_dn8 = assign20030_e20013_d_n8;
        locals.var_q_d2_ln_dn9 = assign20030_e20013_d_n9;
        locals.var_q_d2_ln_rv = 0.0;

        let (assign20040_e20043, assign20040_e20043_d_n4, assign20040_e20043_d_n6, assign20040_e20043_d_n7, assign20040_e20043_d_n8, assign20040_e20043_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign20040_e20027: f64 = (locals.var_q_qsq * 0.0166666666667);
        let assign20040_e20031: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign20040_e20035: f64 = (locals.var_q_qsq * 0.025);
        let assign20040_e20036: f64 = (1.0 - assign20040_e20035);
        let assign20040_e20037: f64 = (assign20040_e20031 * assign20040_e20036);
        let assign20040_e20038: f64 = (1.0 - assign20040_e20037);
        let assign20040_e20039: f64 = (assign20040_e20027 * assign20040_e20038);
        let assign20040_e20040: f64 = (1.0 - assign20040_e20039);
        let assign20040_e20041: f64 = (0.1666666666667 * assign20040_e20040);
        (assign20040_e20041, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign20040_e20038) + (assign20040_e20027 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign20040_e20036) + (assign20040_e20031 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign20040_e20038) + (assign20040_e20027 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign20040_e20036) + (assign20040_e20031 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign20040_e20038) + (assign20040_e20027 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign20040_e20036) + (assign20040_e20031 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign20040_e20038) + (assign20040_e20027 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign20040_e20036) + (assign20040_e20031 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign20040_e20038) + (assign20040_e20027 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign20040_e20036) + (assign20040_e20031 * (-(locals.var_q_qsq_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign20040_e20043;
        locals.var_q_temp3_dn4 = assign20040_e20043_d_n4;
        locals.var_q_temp3_dn6 = assign20040_e20043_d_n6;
        locals.var_q_temp3_dn7 = assign20040_e20043_d_n7;
        locals.var_q_temp3_dn8 = assign20040_e20043_d_n8;
        locals.var_q_temp3_dn9 = assign20040_e20043_d_n9;
        locals.var_q_temp3_rv = 0.0;

        let (assign20050_e20059, assign20050_e20059_d_n4, assign20050_e20059_d_n6, assign20050_e20059_d_n7, assign20050_e20059_d_n8, assign20050_e20059_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign20050_e20056: f64 = (locals.var_q_qsq * locals.var_q_temp3);
        let assign20050_e20057: f64 = (2.0 + assign20050_e20056);
        (assign20050_e20057, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign20050_e20059;
        locals.var_q_qcoth_dn4 = assign20050_e20059_d_n4;
        locals.var_q_qcoth_dn6 = assign20050_e20059_d_n6;
        locals.var_q_qcoth_dn7 = assign20050_e20059_d_n7;
        locals.var_q_qcoth_dn8 = assign20050_e20059_d_n8;
        locals.var_q_qcoth_dn9 = assign20050_e20059_d_n9;
        locals.var_q_qcoth_rv = 0.0;

        let (assign20060_e20089, assign20060_e20089_d_n4, assign20060_e20089_d_n6, assign20060_e20089_d_n7, assign20060_e20089_d_n8, assign20060_e20089_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign20060_e20073: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign20060_e20077: f64 = (locals.var_q_qsq * 0.0357142857143);
        let assign20060_e20081: f64 = (locals.var_q_qsq * 0.0333333333333);
        let assign20060_e20082: f64 = (1.0 - assign20060_e20081);
        let assign20060_e20083: f64 = (assign20060_e20077 * assign20060_e20082);
        let assign20060_e20084: f64 = (1.0 - assign20060_e20083);
        let assign20060_e20085: f64 = (assign20060_e20073 * assign20060_e20084);
        let assign20060_e20086: f64 = (1.0 - assign20060_e20085);
        let assign20060_e20087: f64 = (0.1666666666667 * assign20060_e20086);
        (assign20060_e20087, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign20060_e20084) + (assign20060_e20073 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign20060_e20082) + (assign20060_e20077 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign20060_e20084) + (assign20060_e20073 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign20060_e20082) + (assign20060_e20077 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign20060_e20084) + (assign20060_e20073 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign20060_e20082) + (assign20060_e20077 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign20060_e20084) + (assign20060_e20073 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign20060_e20082) + (assign20060_e20077 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign20060_e20084) + (assign20060_e20073 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign20060_e20082) + (assign20060_e20077 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign20060_e20089;
        locals.var_q_temp1_dn4 = assign20060_e20089_d_n4;
        locals.var_q_temp1_dn6 = assign20060_e20089_d_n6;
        locals.var_q_temp1_dn7 = assign20060_e20089_d_n7;
        locals.var_q_temp1_dn8 = assign20060_e20089_d_n8;
        locals.var_q_temp1_dn9 = assign20060_e20089_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign20070_e20103, assign20070_e20103_d_n4, assign20070_e20103_d_n6, assign20070_e20103_d_n7, assign20070_e20103_d_n8, assign20070_e20103_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign20070_e20101: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
        (assign20070_e20101, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9,)
    }
};
        locals.var_q_d1_qcoth = assign20070_e20103;
        locals.var_q_d1_qcoth_dn4 = assign20070_e20103_d_n4;
        locals.var_q_d1_qcoth_dn6 = assign20070_e20103_d_n6;
        locals.var_q_d1_qcoth_dn7 = assign20070_e20103_d_n7;
        locals.var_q_d1_qcoth_dn8 = assign20070_e20103_d_n8;
        locals.var_q_d1_qcoth_dn9 = assign20070_e20103_d_n9;
        locals.var_q_d1_qcoth_rv = 0.0;

        let (assign20080_e20133, assign20080_e20133_d_n4, assign20080_e20133_d_n6, assign20080_e20133_d_n7, assign20080_e20133_d_n8, assign20080_e20133_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign20080_e20117: f64 = (locals.var_q_qsq * 0.0714285714286);
        let assign20080_e20121: f64 = (0.05 * locals.var_q_qsq);
        let assign20080_e20125: f64 = (0.0420875420875421 * locals.var_q_qsq);
        let assign20080_e20126: f64 = (1.0 - assign20080_e20125);
        let assign20080_e20127: f64 = (assign20080_e20121 * assign20080_e20126);
        let assign20080_e20128: f64 = (1.0 - assign20080_e20127);
        let assign20080_e20129: f64 = (assign20080_e20117 * assign20080_e20128);
        let assign20080_e20130: f64 = (1.0 - assign20080_e20129);
        let assign20080_e20131: f64 = (0.0055555555556 * assign20080_e20130);
        (assign20080_e20131, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign20080_e20128) + (assign20080_e20117 * (-(((0.05 * locals.var_q_qsq_dn4) * assign20080_e20126) + (assign20080_e20121 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign20080_e20128) + (assign20080_e20117 * (-(((0.05 * locals.var_q_qsq_dn6) * assign20080_e20126) + (assign20080_e20121 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign20080_e20128) + (assign20080_e20117 * (-(((0.05 * locals.var_q_qsq_dn7) * assign20080_e20126) + (assign20080_e20121 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign20080_e20128) + (assign20080_e20117 * (-(((0.05 * locals.var_q_qsq_dn8) * assign20080_e20126) + (assign20080_e20121 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign20080_e20128) + (assign20080_e20117 * (-(((0.05 * locals.var_q_qsq_dn9) * assign20080_e20126) + (assign20080_e20121 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20080_e20133;
        locals.var_q_temp2_dn4 = assign20080_e20133_d_n4;
        locals.var_q_temp2_dn6 = assign20080_e20133_d_n6;
        locals.var_q_temp2_dn7 = assign20080_e20133_d_n7;
        locals.var_q_temp2_dn8 = assign20080_e20133_d_n8;
        locals.var_q_temp2_dn9 = assign20080_e20133_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign20090_e20153, assign20090_e20153_d_n4, assign20090_e20153_d_n6, assign20090_e20153_d_n7, assign20090_e20153_d_n8, assign20090_e20153_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign20090_e20145: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
        let assign20090_e20148: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
        let assign20090_e20150: f64 = (assign20090_e20148 * locals.var_q_temp2);
        let assign20090_e20151: f64 = (assign20090_e20145 - assign20090_e20150);
        (assign20090_e20151, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign20090_e20148 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign20090_e20148 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign20090_e20148 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign20090_e20148 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign20090_e20148 * locals.var_q_temp2_dn9))),)
    } else {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    }
};
        locals.var_q_d2_qcoth = assign20090_e20153;
        locals.var_q_d2_qcoth_dn4 = assign20090_e20153_d_n4;
        locals.var_q_d2_qcoth_dn6 = assign20090_e20153_d_n6;
        locals.var_q_d2_qcoth_dn7 = assign20090_e20153_d_n7;
        locals.var_q_d2_qcoth_dn8 = assign20090_e20153_d_n8;
        locals.var_q_d2_qcoth_dn9 = assign20090_e20153_d_n9;
        locals.var_q_d2_qcoth_rv = 0.0;

        let (assign20100_e20170, assign20100_e20170_d_n4, assign20100_e20170_d_n6, assign20100_e20170_d_n7, assign20100_e20170_d_n8, assign20100_e20170_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign20100_e20164: f64 = (-0.5);
        let assign20100_e20166: f64 = (assign20100_e20164 * locals.var_q_d1_qsq);
        let assign20100_e20168: f64 = (assign20100_e20166 * locals.var_q_temp3);
        (assign20100_e20168, (((assign20100_e20164 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign20100_e20166 * locals.var_q_temp3_dn4)), (((assign20100_e20164 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign20100_e20166 * locals.var_q_temp3_dn6)), (((assign20100_e20164 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign20100_e20166 * locals.var_q_temp3_dn7)), (((assign20100_e20164 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign20100_e20166 * locals.var_q_temp3_dn8)), (((assign20100_e20164 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign20100_e20166 * locals.var_q_temp3_dn9)),)
    } else {
        (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9,)
    }
};
        locals.var_q_d1_ln = assign20100_e20170;
        locals.var_q_d1_ln_dn4 = assign20100_e20170_d_n4;
        locals.var_q_d1_ln_dn6 = assign20100_e20170_d_n6;
        locals.var_q_d1_ln_dn7 = assign20100_e20170_d_n7;
        locals.var_q_d1_ln_dn8 = assign20100_e20170_d_n8;
        locals.var_q_d1_ln_dn9 = assign20100_e20170_d_n9;
        locals.var_q_d1_ln_rv = 0.0;

        let (assign20110_e20207, assign20110_e20207_d_n4, assign20110_e20207_d_n6, assign20110_e20207_d_n7, assign20110_e20207_d_n8, assign20110_e20207_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard649 == 0.0)) && (locals.var_guard650 == 0.0)) {
        let assign20110_e20181: f64 = (-0.5);
        let assign20110_e20183: f64 = (assign20110_e20181 * locals.var_q_d2_qsq);
        let assign20110_e20185: f64 = (assign20110_e20183 * locals.var_q_temp3);
        let assign20110_e20188: f64 = (0.25 * 0.0055555555556);
        let assign20110_e20190: f64 = (assign20110_e20188 * locals.var_q_d1_qsq);
        let assign20110_e20192: f64 = (assign20110_e20190 * locals.var_q_d1_qsq);
        let assign20110_e20196: f64 = (locals.var_q_qsq * 0.0238095238095);
        let assign20110_e20200: f64 = (0.075 * locals.var_q_qsq);
        let assign20110_e20201: f64 = (2.0 - assign20110_e20200);
        let assign20110_e20202: f64 = (assign20110_e20196 * assign20110_e20201);
        let assign20110_e20203: f64 = (1.0 - assign20110_e20202);
        let assign20110_e20204: f64 = (assign20110_e20192 * assign20110_e20203);
        let assign20110_e20205: f64 = (assign20110_e20185 + assign20110_e20204);
        (assign20110_e20205, ((((assign20110_e20181 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign20110_e20183 * locals.var_q_temp3_dn4)) + (((((assign20110_e20188 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign20110_e20190 * locals.var_q_d1_qsq_dn4)) * assign20110_e20203) + (assign20110_e20192 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign20110_e20201) + (assign20110_e20196 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign20110_e20181 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign20110_e20183 * locals.var_q_temp3_dn6)) + (((((assign20110_e20188 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign20110_e20190 * locals.var_q_d1_qsq_dn6)) * assign20110_e20203) + (assign20110_e20192 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign20110_e20201) + (assign20110_e20196 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign20110_e20181 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign20110_e20183 * locals.var_q_temp3_dn7)) + (((((assign20110_e20188 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign20110_e20190 * locals.var_q_d1_qsq_dn7)) * assign20110_e20203) + (assign20110_e20192 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign20110_e20201) + (assign20110_e20196 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign20110_e20181 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign20110_e20183 * locals.var_q_temp3_dn8)) + (((((assign20110_e20188 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign20110_e20190 * locals.var_q_d1_qsq_dn8)) * assign20110_e20203) + (assign20110_e20192 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign20110_e20201) + (assign20110_e20196 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign20110_e20181 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign20110_e20183 * locals.var_q_temp3_dn9)) + (((((assign20110_e20188 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign20110_e20190 * locals.var_q_d1_qsq_dn9)) * assign20110_e20203) + (assign20110_e20192 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign20110_e20201) + (assign20110_e20196 * (-(0.075 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9,)
    }
};
        locals.var_q_d2_ln = assign20110_e20207;
        locals.var_q_d2_ln_dn4 = assign20110_e20207_d_n4;
        locals.var_q_d2_ln_dn6 = assign20110_e20207_d_n6;
        locals.var_q_d2_ln_dn7 = assign20110_e20207_d_n7;
        locals.var_q_d2_ln_dn8 = assign20110_e20207_d_n8;
        locals.var_q_d2_ln_dn9 = assign20110_e20207_d_n9;
        locals.var_q_d2_ln_rv = 0.0;

        let assign20120_e20210: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard651 = assign20120_e20210;
        locals.var_guard651_rv = 0.0;

        let (assign20130_e20228, assign20130_e20228_d_n4, assign20130_e20228_d_n6, assign20130_e20228_d_n7, assign20130_e20228_d_n8, assign20130_e20228_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign20130_e20218: f64 = (4.0 * locals.var_q_qsq);
        let assign20130_e20223: f64 = (2.0 - locals.var_q_invexpq);
        let assign20130_e20224: f64 = (locals.var_q_invexpq * assign20130_e20223);
        let assign20130_e20225: f64 = (1.0 - assign20130_e20224);
        let assign20130_e20226: f64 = (assign20130_e20218 / assign20130_e20225);
        (assign20130_e20226, ((((4.0 * locals.var_q_qsq_dn4) * assign20130_e20225) - (assign20130_e20218 * (-((locals.var_q_invexpq_dn4 * assign20130_e20223) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign20130_e20225 * assign20130_e20225)), ((((4.0 * locals.var_q_qsq_dn6) * assign20130_e20225) - (assign20130_e20218 * (-((locals.var_q_invexpq_dn6 * assign20130_e20223) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign20130_e20225 * assign20130_e20225)), ((((4.0 * locals.var_q_qsq_dn7) * assign20130_e20225) - (assign20130_e20218 * (-((locals.var_q_invexpq_dn7 * assign20130_e20223) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign20130_e20225 * assign20130_e20225)), ((((4.0 * locals.var_q_qsq_dn8) * assign20130_e20225) - (assign20130_e20218 * (-((locals.var_q_invexpq_dn8 * assign20130_e20223) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign20130_e20225 * assign20130_e20225)), ((((4.0 * locals.var_q_qsq_dn9) * assign20130_e20225) - (assign20130_e20218 * (-((locals.var_q_invexpq_dn9 * assign20130_e20223) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign20130_e20225 * assign20130_e20225)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20130_e20228;
        locals.var_q_temp2_dn4 = assign20130_e20228_d_n4;
        locals.var_q_temp2_dn6 = assign20130_e20228_d_n6;
        locals.var_q_temp2_dn7 = assign20130_e20228_d_n7;
        locals.var_q_temp2_dn8 = assign20130_e20228_d_n8;
        locals.var_q_temp2_dn9 = assign20130_e20228_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign20140_e20238, assign20140_e20238_d_n4, assign20140_e20238_d_n6, assign20140_e20238_d_n7, assign20140_e20238_d_n8, assign20140_e20238_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign20140_e20236: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
        (assign20140_e20236, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign20140_e20238;
        locals.var_q_sh_term_dn4 = assign20140_e20238_d_n4;
        locals.var_q_sh_term_dn6 = assign20140_e20238_d_n6;
        locals.var_q_sh_term_dn7 = assign20140_e20238_d_n7;
        locals.var_q_sh_term_dn8 = assign20140_e20238_d_n8;
        locals.var_q_sh_term_dn9 = assign20140_e20238_d_n9;
        locals.var_q_sh_term_rv = 0.0;

        let (assign20150_e20249, assign20150_e20249_d_n4, assign20150_e20249_d_n6, assign20150_e20249_d_n7, assign20150_e20249_d_n8, assign20150_e20249_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign20150_e20245: f64 = (locals.var_q_temp2).ln();
        let assign20150_e20247: f64 = (assign20150_e20245 - locals.var_q_rac_qsq);
        (assign20150_e20247, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign20150_e20249;
        locals.var_q_ln_term_dn4 = assign20150_e20249_d_n4;
        locals.var_q_ln_term_dn6 = assign20150_e20249_d_n6;
        locals.var_q_ln_term_dn7 = assign20150_e20249_d_n7;
        locals.var_q_ln_term_dn8 = assign20150_e20249_d_n8;
        locals.var_q_ln_term_dn9 = assign20150_e20249_d_n9;
        locals.var_q_ln_term_rv = 0.0;

        let assign20160_e20252: f64 = (-0.005);
        let assign20160_e20253: f64 = if locals.var_q_qsq < assign20160_e20252 { 1.0 } else { 0.0 };
        locals.var_guard652 = assign20160_e20253;
        locals.var_guard652_rv = 0.0;

        let (assign20170_e20267, assign20170_e20267_d_n4, assign20170_e20267_d_n6, assign20170_e20267_d_n7, assign20170_e20267_d_n8, assign20170_e20267_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard651 == 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign20170_e20264: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign20170_e20265: f64 = (assign20170_e20264).sin();
        (assign20170_e20265, ((assign20170_e20264).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign20170_e20264).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign20170_e20264).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign20170_e20264).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign20170_e20264).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20170_e20267;
        locals.var_q_temp2_dn4 = assign20170_e20267_d_n4;
        locals.var_q_temp2_dn6 = assign20170_e20267_d_n6;
        locals.var_q_temp2_dn7 = assign20170_e20267_d_n7;
        locals.var_q_temp2_dn8 = assign20170_e20267_d_n8;
        locals.var_q_temp2_dn9 = assign20170_e20267_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign20180_e20283, assign20180_e20283_d_n4, assign20180_e20283_d_n6, assign20180_e20283_d_n7, assign20180_e20283_d_n8, assign20180_e20283_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard651 == 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign20180_e20277: f64 = (-locals.var_q_qsq);
        let assign20180_e20280: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
        let assign20180_e20281: f64 = (assign20180_e20277 / assign20180_e20280);
        (assign20180_e20281, ((((-locals.var_q_qsq_dn4) * assign20180_e20280) - (assign20180_e20277 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign20180_e20280 * assign20180_e20280)), ((((-locals.var_q_qsq_dn6) * assign20180_e20280) - (assign20180_e20277 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign20180_e20280 * assign20180_e20280)), ((((-locals.var_q_qsq_dn7) * assign20180_e20280) - (assign20180_e20277 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign20180_e20280 * assign20180_e20280)), ((((-locals.var_q_qsq_dn8) * assign20180_e20280) - (assign20180_e20277 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign20180_e20280 * assign20180_e20280)), ((((-locals.var_q_qsq_dn9) * assign20180_e20280) - (assign20180_e20277 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign20180_e20280 * assign20180_e20280)),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign20180_e20283;
        locals.var_q_sh_term_dn4 = assign20180_e20283_d_n4;
        locals.var_q_sh_term_dn6 = assign20180_e20283_d_n6;
        locals.var_q_sh_term_dn7 = assign20180_e20283_d_n7;
        locals.var_q_sh_term_dn8 = assign20180_e20283_d_n8;
        locals.var_q_sh_term_dn9 = assign20180_e20283_d_n9;
        locals.var_q_sh_term_rv = 0.0;

        let (assign20190_e20295, assign20190_e20295_d_n4, assign20190_e20295_d_n6, assign20190_e20295_d_n7, assign20190_e20295_d_n8, assign20190_e20295_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard651 == 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign20190_e20293: f64 = (locals.var_q_sh_term).ln();
        (assign20190_e20293, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign20190_e20295;
        locals.var_q_ln_term_dn4 = assign20190_e20295_d_n4;
        locals.var_q_ln_term_dn6 = assign20190_e20295_d_n6;
        locals.var_q_ln_term_dn7 = assign20190_e20295_d_n7;
        locals.var_q_ln_term_dn8 = assign20190_e20295_d_n8;
        locals.var_q_ln_term_dn9 = assign20190_e20295_d_n9;
        locals.var_q_ln_term_rv = 0.0;

        let (assign20200_e20323, assign20200_e20323_d_n4, assign20200_e20323_d_n6, assign20200_e20323_d_n7, assign20200_e20323_d_n8, assign20200_e20323_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard651 == 0.0)) && (locals.var_guard652 == 0.0)) {
        let assign20200_e20308: f64 = (locals.var_q_qsq * 0.3333333333333);
        let assign20200_e20312: f64 = (0.05 * locals.var_q_qsq);
        let assign20200_e20316: f64 = (0.0396825396825397 * locals.var_q_qsq);
        let assign20200_e20317: f64 = (1.0 - assign20200_e20316);
        let assign20200_e20318: f64 = (assign20200_e20312 * assign20200_e20317);
        let assign20200_e20319: f64 = (1.0 - assign20200_e20318);
        let assign20200_e20320: f64 = (assign20200_e20308 * assign20200_e20319);
        let assign20200_e20321: f64 = (4.0 - assign20200_e20320);
        (assign20200_e20321, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign20200_e20319) + (assign20200_e20308 * (-(((0.05 * locals.var_q_qsq_dn4) * assign20200_e20317) + (assign20200_e20312 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign20200_e20319) + (assign20200_e20308 * (-(((0.05 * locals.var_q_qsq_dn6) * assign20200_e20317) + (assign20200_e20312 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign20200_e20319) + (assign20200_e20308 * (-(((0.05 * locals.var_q_qsq_dn7) * assign20200_e20317) + (assign20200_e20312 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign20200_e20319) + (assign20200_e20308 * (-(((0.05 * locals.var_q_qsq_dn8) * assign20200_e20317) + (assign20200_e20312 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign20200_e20319) + (assign20200_e20308 * (-(((0.05 * locals.var_q_qsq_dn9) * assign20200_e20317) + (assign20200_e20312 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))),)
    } else {
        (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9,)
    }
};
        locals.var_q_sh_term = assign20200_e20323;
        locals.var_q_sh_term_dn4 = assign20200_e20323_d_n4;
        locals.var_q_sh_term_dn6 = assign20200_e20323_d_n6;
        locals.var_q_sh_term_dn7 = assign20200_e20323_d_n7;
        locals.var_q_sh_term_dn8 = assign20200_e20323_d_n8;
        locals.var_q_sh_term_dn9 = assign20200_e20323_d_n9;
        locals.var_q_sh_term_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_55(
        locals: &mut StampLocals,
    ) {
        let (assign20210_e20336, assign20210_e20336_d_n4, assign20210_e20336_d_n6, assign20210_e20336_d_n7, assign20210_e20336_d_n8, assign20210_e20336_d_n9,) = {
    if ((((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard651 == 0.0)) && (locals.var_guard652 == 0.0)) {
        let assign20210_e20334: f64 = (locals.var_q_sh_term).ln();
        (assign20210_e20334, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term),)
    } else {
        (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9,)
    }
};
        locals.var_q_ln_term = assign20210_e20336;
        locals.var_q_ln_term_dn4 = assign20210_e20336_d_n4;
        locals.var_q_ln_term_dn6 = assign20210_e20336_d_n6;
        locals.var_q_ln_term_dn7 = assign20210_e20336_d_n7;
        locals.var_q_ln_term_dn8 = assign20210_e20336_d_n8;
        locals.var_q_ln_term_dn9 = assign20210_e20336_d_n9;
        locals.var_q_ln_term_rv = 0.0;

        let assign20220_e20339: f64 = (1.01 * locals.var_q_k1q1);
        let assign20220_e20341: f64 = (assign20220_e20339 + locals.var_q_qcoth);
        let assign20220_e20343: f64 = if assign20220_e20341 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard653 = assign20220_e20343;
        locals.var_guard653_rv = 0.0;

        let (assign20230_e20353, assign20230_e20353_d_n4, assign20230_e20353_d_n6, assign20230_e20353_d_n7, assign20230_e20353_d_n8, assign20230_e20353_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard653 != 0.0)) {
        let assign20230_e20351: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
        (assign20230_e20351, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign20230_e20353;
        locals.var_q_expnum_dn4 = assign20230_e20353_d_n4;
        locals.var_q_expnum_dn6 = assign20230_e20353_d_n6;
        locals.var_q_expnum_dn7 = assign20230_e20353_d_n7;
        locals.var_q_expnum_dn8 = assign20230_e20353_d_n8;
        locals.var_q_expnum_dn9 = assign20230_e20353_d_n9;
        locals.var_q_expnum_rv = 0.0;

        let (assign20240_e20363, assign20240_e20363_d_n4, assign20240_e20363_d_n6, assign20240_e20363_d_n7, assign20240_e20363_d_n8, assign20240_e20363_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard653 != 0.0)) {
        let assign20240_e20361: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
        (assign20240_e20361, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign20240_e20363;
        locals.var_q_d1_expnum_dn4 = assign20240_e20363_d_n4;
        locals.var_q_d1_expnum_dn6 = assign20240_e20363_d_n6;
        locals.var_q_d1_expnum_dn7 = assign20240_e20363_d_n7;
        locals.var_q_d1_expnum_dn8 = assign20240_e20363_d_n8;
        locals.var_q_d1_expnum_dn9 = assign20240_e20363_d_n9;
        locals.var_q_d1_expnum_rv = 0.0;

        let (assign20250_e20371, assign20250_e20371_d_n4, assign20250_e20371_d_n6, assign20250_e20371_d_n7, assign20250_e20371_d_n8, assign20250_e20371_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard653 != 0.0)) {
        (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9,)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign20250_e20371;
        locals.var_q_d2_expnum_dn4 = assign20250_e20371_d_n4;
        locals.var_q_d2_expnum_dn6 = assign20250_e20371_d_n6;
        locals.var_q_d2_expnum_dn7 = assign20250_e20371_d_n7;
        locals.var_q_d2_expnum_dn8 = assign20250_e20371_d_n8;
        locals.var_q_d2_expnum_dn9 = assign20250_e20371_d_n9;
        locals.var_q_d2_expnum_rv = 0.0;

        let (assign20260_e20384, assign20260_e20384_d_n4, assign20260_e20384_d_n6, assign20260_e20384_d_n7, assign20260_e20384_d_n8, assign20260_e20384_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard653 == 0.0)) {
        let assign20260_e20381: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
        let assign20260_e20382: f64 = (1.0 / assign20260_e20381);
        (assign20260_e20382, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign20260_e20381 * assign20260_e20381))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign20260_e20381 * assign20260_e20381))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign20260_e20381 * assign20260_e20381))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign20260_e20381 * assign20260_e20381))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign20260_e20381 * assign20260_e20381))),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20260_e20384;
        locals.var_q_temp2_dn4 = assign20260_e20384_d_n4;
        locals.var_q_temp2_dn6 = assign20260_e20384_d_n6;
        locals.var_q_temp2_dn7 = assign20260_e20384_d_n7;
        locals.var_q_temp2_dn8 = assign20260_e20384_d_n8;
        locals.var_q_temp2_dn9 = assign20260_e20384_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign20270_e20395, assign20270_e20395_d_n4, assign20270_e20395_d_n6, assign20270_e20395_d_n7, assign20270_e20395_d_n8, assign20270_e20395_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard653 == 0.0)) {
        let assign20270_e20393: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
        (assign20270_e20393, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign20270_e20395;
        locals.var_q_temp3_dn4 = assign20270_e20395_d_n4;
        locals.var_q_temp3_dn6 = assign20270_e20395_d_n6;
        locals.var_q_temp3_dn7 = assign20270_e20395_d_n7;
        locals.var_q_temp3_dn8 = assign20270_e20395_d_n8;
        locals.var_q_temp3_dn9 = assign20270_e20395_d_n9;
        locals.var_q_temp3_rv = 0.0;

        let (assign20280_e20408, assign20280_e20408_d_n4, assign20280_e20408_d_n6, assign20280_e20408_d_n7, assign20280_e20408_d_n8, assign20280_e20408_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard653 == 0.0)) {
        let assign20280_e20404: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
        let assign20280_e20406: f64 = (assign20280_e20404 * locals.var_q_temp2);
        (assign20280_e20406, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign20280_e20404 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign20280_e20404 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign20280_e20404 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign20280_e20404 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign20280_e20404 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9,)
    }
};
        locals.var_q_expnum = assign20280_e20408;
        locals.var_q_expnum_dn4 = assign20280_e20408_d_n4;
        locals.var_q_expnum_dn6 = assign20280_e20408_d_n6;
        locals.var_q_expnum_dn7 = assign20280_e20408_d_n7;
        locals.var_q_expnum_dn8 = assign20280_e20408_d_n8;
        locals.var_q_expnum_dn9 = assign20280_e20408_d_n9;
        locals.var_q_expnum_rv = 0.0;

        let (assign20290_e20427, assign20290_e20427_d_n4, assign20290_e20427_d_n6, assign20290_e20427_d_n7, assign20290_e20427_d_n8, assign20290_e20427_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard653 == 0.0)) {
        let assign20290_e20417: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
        let assign20290_e20419: f64 = (assign20290_e20417 - locals.var_q_aexp);
        let assign20290_e20422: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
        let assign20290_e20423: f64 = (assign20290_e20419 - assign20290_e20422);
        let assign20290_e20425: f64 = (assign20290_e20423 * locals.var_q_temp2);
        (assign20290_e20425, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign20290_e20423 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign20290_e20423 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign20290_e20423 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign20290_e20423 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign20290_e20423 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9,)
    }
};
        locals.var_q_d1_expnum = assign20290_e20427;
        locals.var_q_d1_expnum_dn4 = assign20290_e20427_d_n4;
        locals.var_q_d1_expnum_dn6 = assign20290_e20427_d_n6;
        locals.var_q_d1_expnum_dn7 = assign20290_e20427_d_n7;
        locals.var_q_d1_expnum_dn8 = assign20290_e20427_d_n8;
        locals.var_q_d1_expnum_dn9 = assign20290_e20427_d_n9;
        locals.var_q_d1_expnum_rv = 0.0;

        let (assign20300_e20456, assign20300_e20456_d_n4, assign20300_e20456_d_n6, assign20300_e20456_d_n7, assign20300_e20456_d_n8, assign20300_e20456_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard653 == 0.0)) {
        let assign20300_e20436: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
        let assign20300_e20439: f64 = (2.0 * locals.var_q_temp3);
        let assign20300_e20441: f64 = (assign20300_e20439 * locals.var_q_d1_expnum);
        let assign20300_e20442: f64 = (assign20300_e20436 + assign20300_e20441);
        let assign20300_e20444: f64 = (assign20300_e20442 + locals.var_q_aexp);
        let assign20300_e20448: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
        let assign20300_e20449: f64 = (locals.var_q_d2_ln + assign20300_e20448);
        let assign20300_e20451: f64 = (assign20300_e20449 * locals.var_q_sh_term);
        let assign20300_e20452: f64 = (assign20300_e20444 - assign20300_e20451);
        let assign20300_e20454: f64 = (assign20300_e20452 * locals.var_q_temp2);
        (assign20300_e20454, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign20300_e20439 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign20300_e20449 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign20300_e20452 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign20300_e20439 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign20300_e20449 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign20300_e20452 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign20300_e20439 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign20300_e20449 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign20300_e20452 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign20300_e20439 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign20300_e20449 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign20300_e20452 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign20300_e20439 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign20300_e20449 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign20300_e20452 * locals.var_q_temp2_dn9)),)
    } else {
        (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9,)
    }
};
        locals.var_q_d2_expnum = assign20300_e20456;
        locals.var_q_d2_expnum_dn4 = assign20300_e20456_d_n4;
        locals.var_q_d2_expnum_dn6 = assign20300_e20456_d_n6;
        locals.var_q_d2_expnum_dn7 = assign20300_e20456_d_n7;
        locals.var_q_d2_expnum_dn8 = assign20300_e20456_d_n8;
        locals.var_q_d2_expnum_dn9 = assign20300_e20456_d_n9;
        locals.var_q_d2_expnum_rv = 0.0;

        let assign20310_e20459: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard654 = assign20310_e20459;
        locals.var_guard654_rv = 0.0;

        let (assign20320_e20468, assign20320_e20468_d_n4, assign20320_e20468_d_n6, assign20320_e20468_d_n7, assign20320_e20468_d_n8, assign20320_e20468_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard654 != 0.0)) {
        let assign20320_e20466: f64 = (locals.var_q_expnum).ln();
        (assign20320_e20466, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign20320_e20468;
        locals.var_q_lnexpnum_dn4 = assign20320_e20468_d_n4;
        locals.var_q_lnexpnum_dn6 = assign20320_e20468_d_n6;
        locals.var_q_lnexpnum_dn7 = assign20320_e20468_d_n7;
        locals.var_q_lnexpnum_dn8 = assign20320_e20468_d_n8;
        locals.var_q_lnexpnum_dn9 = assign20320_e20468_d_n9;
        locals.var_q_lnexpnum_rv = 0.0;

        let (assign20330_e20478, assign20330_e20478_d_n4, assign20330_e20478_d_n6, assign20330_e20478_d_n7, assign20330_e20478_d_n8, assign20330_e20478_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard654 != 0.0)) {
        let assign20330_e20476: f64 = (1.0 / locals.var_q_expnum);
        (assign20330_e20476, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign20330_e20478;
        locals.var_q_temp1_dn4 = assign20330_e20478_d_n4;
        locals.var_q_temp1_dn6 = assign20330_e20478_d_n6;
        locals.var_q_temp1_dn7 = assign20330_e20478_d_n7;
        locals.var_q_temp1_dn8 = assign20330_e20478_d_n8;
        locals.var_q_temp1_dn9 = assign20330_e20478_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign20340_e20488, assign20340_e20488_d_n4, assign20340_e20488_d_n6, assign20340_e20488_d_n7, assign20340_e20488_d_n8, assign20340_e20488_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard654 != 0.0)) {
        let assign20340_e20486: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
        (assign20340_e20486, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign20340_e20488;
        locals.var_q_d1_lnexpnum_dn4 = assign20340_e20488_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign20340_e20488_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign20340_e20488_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign20340_e20488_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign20340_e20488_d_n9;
        locals.var_q_d1_lnexpnum_rv = 0.0;

        let (assign20350_e20502, assign20350_e20502_d_n4, assign20350_e20502_d_n6, assign20350_e20502_d_n7, assign20350_e20502_d_n8, assign20350_e20502_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard654 != 0.0)) {
        let assign20350_e20496: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
        let assign20350_e20499: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
        let assign20350_e20500: f64 = (assign20350_e20496 - assign20350_e20499);
        (assign20350_e20500, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign20350_e20502;
        locals.var_q_d2_lnexpnum_dn4 = assign20350_e20502_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign20350_e20502_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign20350_e20502_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign20350_e20502_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign20350_e20502_d_n9;
        locals.var_q_d2_lnexpnum_rv = 0.0;

        let (assign20360_e20517, assign20360_e20517_d_n4, assign20360_e20517_d_n6, assign20360_e20517_d_n7, assign20360_e20517_d_n8, assign20360_e20517_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard654 == 0.0)) {
        let assign20360_e20511: f64 = (locals.var_q_k1q1 + 0.6931471805599);
        let assign20360_e20513: f64 = (-locals.var_q_k1q1);
        let assign20360_e20514: f64 = (assign20360_e20513).ln();
        let assign20360_e20515: f64 = (assign20360_e20511 + assign20360_e20514);
        (assign20360_e20515, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign20360_e20513)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign20360_e20513)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign20360_e20513)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign20360_e20513)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign20360_e20513)),)
    } else {
        (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9,)
    }
};
        locals.var_q_lnexpnum = assign20360_e20517;
        locals.var_q_lnexpnum_dn4 = assign20360_e20517_d_n4;
        locals.var_q_lnexpnum_dn6 = assign20360_e20517_d_n6;
        locals.var_q_lnexpnum_dn7 = assign20360_e20517_d_n7;
        locals.var_q_lnexpnum_dn8 = assign20360_e20517_d_n8;
        locals.var_q_lnexpnum_dn9 = assign20360_e20517_d_n9;
        locals.var_q_lnexpnum_rv = 0.0;

        let (assign20370_e20528, assign20370_e20528_d_n4, assign20370_e20528_d_n6, assign20370_e20528_d_n7, assign20370_e20528_d_n8, assign20370_e20528_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard654 == 0.0)) {
        let assign20370_e20526: f64 = (1.0 / locals.var_q1d);
        (assign20370_e20526, (-(locals.var_q1d_dn4 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn6 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn7 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn8 / (locals.var_q1d * locals.var_q1d))), (-(locals.var_q1d_dn9 / (locals.var_q1d * locals.var_q1d))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign20370_e20528;
        locals.var_q_temp1_dn4 = assign20370_e20528_d_n4;
        locals.var_q_temp1_dn6 = assign20370_e20528_d_n6;
        locals.var_q_temp1_dn7 = assign20370_e20528_d_n7;
        locals.var_q_temp1_dn8 = assign20370_e20528_d_n8;
        locals.var_q_temp1_dn9 = assign20370_e20528_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign20380_e20539, assign20380_e20539_d_n4, assign20380_e20539_d_n6, assign20380_e20539_d_n7, assign20380_e20539_d_n8, assign20380_e20539_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard654 == 0.0)) {
        let assign20380_e20537: f64 = (locals.var_k1 + locals.var_q_temp1);
        (assign20380_e20537, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9,)
    }
};
        locals.var_q_d1_lnexpnum = assign20380_e20539;
        locals.var_q_d1_lnexpnum_dn4 = assign20380_e20539_d_n4;
        locals.var_q_d1_lnexpnum_dn6 = assign20380_e20539_d_n6;
        locals.var_q_d1_lnexpnum_dn7 = assign20380_e20539_d_n7;
        locals.var_q_d1_lnexpnum_dn8 = assign20380_e20539_d_n8;
        locals.var_q_d1_lnexpnum_dn9 = assign20380_e20539_d_n9;
        locals.var_q_d1_lnexpnum_rv = 0.0;

        let (assign20390_e20551, assign20390_e20551_d_n4, assign20390_e20551_d_n6, assign20390_e20551_d_n7, assign20390_e20551_d_n8, assign20390_e20551_d_n9,) = {
    if (((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) && (locals.var_guard654 == 0.0)) {
        let assign20390_e20547: f64 = (-locals.var_q_temp1);
        let assign20390_e20549: f64 = (assign20390_e20547 * locals.var_q_temp1);
        (assign20390_e20549, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign20390_e20547 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign20390_e20547 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign20390_e20547 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign20390_e20547 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign20390_e20547 * locals.var_q_temp1_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9,)
    }
};
        locals.var_q_d2_lnexpnum = assign20390_e20551;
        locals.var_q_d2_lnexpnum_dn4 = assign20390_e20551_d_n4;
        locals.var_q_d2_lnexpnum_dn6 = assign20390_e20551_d_n6;
        locals.var_q_d2_lnexpnum_dn7 = assign20390_e20551_d_n7;
        locals.var_q_d2_lnexpnum_dn8 = assign20390_e20551_d_n8;
        locals.var_q_d2_lnexpnum_dn9 = assign20390_e20551_d_n9;
        locals.var_q_d2_lnexpnum_rv = 0.0;

        let (assign20400_e20567, assign20400_e20567_d_n4, assign20400_e20567_d_n6, assign20400_e20567_d_n7, assign20400_e20567_d_n8, assign20400_e20567_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20400_e20557: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign20400_e20559: f64 = (assign20400_e20557 + locals.var_q1d);
        let assign20400_e20562: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign20400_e20563: f64 = (assign20400_e20559 + assign20400_e20562);
        let assign20400_e20565: f64 = (assign20400_e20563 - locals.var_q_ln_term);
        (assign20400_e20565, ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1d_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4), ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1d_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6), ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1d_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7), ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1d_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8), ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1d_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9),)
    } else {
        (locals.var_q_q2_int, locals.var_q_q2_int_dn4, locals.var_q_q2_int_dn6, locals.var_q_q2_int_dn7, locals.var_q_q2_int_dn8, locals.var_q_q2_int_dn9,)
    }
};
        locals.var_q_q2_int = assign20400_e20567;
        locals.var_q_q2_int_dn4 = assign20400_e20567_d_n4;
        locals.var_q_q2_int_dn6 = assign20400_e20567_d_n6;
        locals.var_q_q2_int_dn7 = assign20400_e20567_d_n7;
        locals.var_q_q2_int_dn8 = assign20400_e20567_d_n8;
        locals.var_q_q2_int_dn9 = assign20400_e20567_d_n9;
        locals.var_q_q2_int_rv = 0.0;

        let (assign20410_e20579, assign20410_e20579_d_n4, assign20410_e20579_d_n6, assign20410_e20579_d_n7, assign20410_e20579_d_n8, assign20410_e20579_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20410_e20574: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign20410_e20575: f64 = (1.0 + assign20410_e20574);
        let assign20410_e20577: f64 = (assign20410_e20575 - locals.var_q_d1_ln);
        (assign20410_e20577, ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4), ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6), ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7), ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8), ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9),)
    } else {
        (locals.var_q_d1_q2, locals.var_q_d1_q2_dn4, locals.var_q_d1_q2_dn6, locals.var_q_d1_q2_dn7, locals.var_q_d1_q2_dn8, locals.var_q_d1_q2_dn9,)
    }
};
        locals.var_q_d1_q2 = assign20410_e20579;
        locals.var_q_d1_q2_dn4 = assign20410_e20579_d_n4;
        locals.var_q_d1_q2_dn6 = assign20410_e20579_d_n6;
        locals.var_q_d1_q2_dn7 = assign20410_e20579_d_n7;
        locals.var_q_d1_q2_dn8 = assign20410_e20579_d_n8;
        locals.var_q_d1_q2_dn9 = assign20410_e20579_d_n9;
        locals.var_q_d1_q2_rv = 0.0;

        let (assign20420_e20589, assign20420_e20589_d_n4, assign20420_e20589_d_n6, assign20420_e20589_d_n7, assign20420_e20589_d_n8, assign20420_e20589_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20420_e20585: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign20420_e20587: f64 = (assign20420_e20585 - locals.var_q_d2_ln);
        (assign20420_e20587, ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4), ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6), ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7), ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8), ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9),)
    } else {
        (locals.var_q_d2_q2, locals.var_q_d2_q2_dn4, locals.var_q_d2_q2_dn6, locals.var_q_d2_q2_dn7, locals.var_q_d2_q2_dn8, locals.var_q_d2_q2_dn9,)
    }
};
        locals.var_q_d2_q2 = assign20420_e20589;
        locals.var_q_d2_q2_dn4 = assign20420_e20589_d_n4;
        locals.var_q_d2_q2_dn6 = assign20420_e20589_d_n6;
        locals.var_q_d2_q2_dn7 = assign20420_e20589_d_n7;
        locals.var_q_d2_q2_dn8 = assign20420_e20589_d_n8;
        locals.var_q_d2_q2_dn9 = assign20420_e20589_d_n9;
        locals.var_q_d2_q2_rv = 0.0;

        let (assign20430_e20599, assign20430_e20599_d_n4, assign20430_e20599_d_n6, assign20430_e20599_d_n7, assign20430_e20599_d_n8, assign20430_e20599_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20430_e20596: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign20430_e20597: f64 = (locals.var_q_k1q1 + assign20430_e20596);
        (assign20430_e20597, (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4))), (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6))), (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7))), (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8))), (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9))),)
    } else {
        (locals.var_q_qi_int, locals.var_q_qi_int_dn4, locals.var_q_qi_int_dn6, locals.var_q_qi_int_dn7, locals.var_q_qi_int_dn8, locals.var_q_qi_int_dn9,)
    }
};
        locals.var_q_qi_int = assign20430_e20599;
        locals.var_q_qi_int_dn4 = assign20430_e20599_d_n4;
        locals.var_q_qi_int_dn6 = assign20430_e20599_d_n6;
        locals.var_q_qi_int_dn7 = assign20430_e20599_d_n7;
        locals.var_q_qi_int_dn8 = assign20430_e20599_d_n8;
        locals.var_q_qi_int_dn9 = assign20430_e20599_d_n9;
        locals.var_q_qi_int_rv = 0.0;

        let (assign20440_e20609, assign20440_e20609_d_n4, assign20440_e20609_d_n6, assign20440_e20609_d_n7, assign20440_e20609_d_n8, assign20440_e20609_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20440_e20606: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign20440_e20607: f64 = (locals.var_k1 + assign20440_e20606);
        (assign20440_e20607, (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4))), (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6))), (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7))), (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8))), (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9))),)
    } else {
        (locals.var_q_d1_qi, locals.var_q_d1_qi_dn4, locals.var_q_d1_qi_dn6, locals.var_q_d1_qi_dn7, locals.var_q_d1_qi_dn8, locals.var_q_d1_qi_dn9,)
    }
};
        locals.var_q_d1_qi = assign20440_e20609;
        locals.var_q_d1_qi_dn4 = assign20440_e20609_d_n4;
        locals.var_q_d1_qi_dn6 = assign20440_e20609_d_n6;
        locals.var_q_d1_qi_dn7 = assign20440_e20609_d_n7;
        locals.var_q_d1_qi_dn8 = assign20440_e20609_d_n8;
        locals.var_q_d1_qi_dn9 = assign20440_e20609_d_n9;
        locals.var_q_d1_qi_rv = 0.0;

        let (assign20450_e20617, assign20450_e20617_d_n4, assign20450_e20617_d_n6, assign20450_e20617_d_n7, assign20450_e20617_d_n8, assign20450_e20617_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20450_e20615: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        (assign20450_e20615, ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4)), ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6)), ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7)), ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8)), ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9)),)
    } else {
        (locals.var_q_d2_qi, locals.var_q_d2_qi_dn4, locals.var_q_d2_qi_dn6, locals.var_q_d2_qi_dn7, locals.var_q_d2_qi_dn8, locals.var_q_d2_qi_dn9,)
    }
};
        locals.var_q_d2_qi = assign20450_e20617;
        locals.var_q_d2_qi_dn4 = assign20450_e20617_d_n4;
        locals.var_q_d2_qi_dn6 = assign20450_e20617_d_n6;
        locals.var_q_d2_qi_dn7 = assign20450_e20617_d_n7;
        locals.var_q_d2_qi_dn8 = assign20450_e20617_d_n8;
        locals.var_q_d2_qi_dn9 = assign20450_e20617_d_n9;
        locals.var_q_d2_qi_rv = 0.0;

        let (assign20460_e20627, assign20460_e20627_d_n4, assign20460_e20627_d_n6, assign20460_e20627_d_n7, assign20460_e20627_d_n8, assign20460_e20627_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20460_e20623: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign20460_e20625: f64 = (assign20460_e20623 - locals.var_q_aexp);
        (assign20460_e20625, (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4), (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6), (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7), (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8), (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9,)
    }
};
        locals.var_q_zero = assign20460_e20627;
        locals.var_q_zero_dn4 = assign20460_e20627_d_n4;
        locals.var_q_zero_dn6 = assign20460_e20627_d_n6;
        locals.var_q_zero_dn7 = assign20460_e20627_d_n7;
        locals.var_q_zero_dn8 = assign20460_e20627_d_n8;
        locals.var_q_zero_dn9 = assign20460_e20627_d_n9;
        locals.var_q_zero_rv = 0.0;

        let (assign20470_e20641, assign20470_e20641_d_n4, assign20470_e20641_d_n6, assign20470_e20641_d_n7, assign20470_e20641_d_n8, assign20470_e20641_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20470_e20633: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign20470_e20636: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign20470_e20637: f64 = (assign20470_e20633 + assign20470_e20636);
        let assign20470_e20639: f64 = (assign20470_e20637 + locals.var_q_aexp);
        (assign20470_e20639, ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4), ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6), ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7), ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8), ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9,)
    }
};
        locals.var_q_d1_zero = assign20470_e20641;
        locals.var_q_d1_zero_dn4 = assign20470_e20641_d_n4;
        locals.var_q_d1_zero_dn6 = assign20470_e20641_d_n6;
        locals.var_q_d1_zero_dn7 = assign20470_e20641_d_n7;
        locals.var_q_d1_zero_dn8 = assign20470_e20641_d_n8;
        locals.var_q_d1_zero_dn9 = assign20470_e20641_d_n9;
        locals.var_q_d1_zero_rv = 0.0;

        let (assign20480_e20661, assign20480_e20661_d_n4, assign20480_e20661_d_n6, assign20480_e20661_d_n7, assign20480_e20661_d_n8, assign20480_e20661_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20480_e20647: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign20480_e20650: f64 = (2.0 * locals.var_q_d1_qi);
        let assign20480_e20652: f64 = (assign20480_e20650 * locals.var_q_d1_expnum);
        let assign20480_e20653: f64 = (assign20480_e20647 + assign20480_e20652);
        let assign20480_e20656: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign20480_e20657: f64 = (assign20480_e20653 + assign20480_e20656);
        let assign20480_e20659: f64 = (assign20480_e20657 - locals.var_q_aexp);
        (assign20480_e20659, (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign20480_e20650 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4), (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign20480_e20650 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6), (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign20480_e20650 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7), (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign20480_e20650 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8), (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign20480_e20650 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9),)
    } else {
        (locals.var_q_d2_zero, locals.var_q_d2_zero_dn4, locals.var_q_d2_zero_dn6, locals.var_q_d2_zero_dn7, locals.var_q_d2_zero_dn8, locals.var_q_d2_zero_dn9,)
    }
};
        locals.var_q_d2_zero = assign20480_e20661;
        locals.var_q_d2_zero_dn4 = assign20480_e20661_d_n4;
        locals.var_q_d2_zero_dn6 = assign20480_e20661_d_n6;
        locals.var_q_d2_zero_dn7 = assign20480_e20661_d_n7;
        locals.var_q_d2_zero_dn8 = assign20480_e20661_d_n8;
        locals.var_q_d2_zero_dn9 = assign20480_e20661_d_n9;
        locals.var_q_d2_zero_rv = 0.0;

        let (assign20490_e20675, assign20490_e20675_d_n4, assign20490_e20675_d_n6, assign20490_e20675_d_n7, assign20490_e20675_d_n8, assign20490_e20675_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20490_e20667: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign20490_e20670: f64 = (0.5 * locals.var_q_zero);
        let assign20490_e20672: f64 = (assign20490_e20670 * locals.var_q_d2_zero);
        let assign20490_e20673: f64 = (assign20490_e20667 - assign20490_e20672);
        (assign20490_e20673, (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign20490_e20670 * locals.var_q_d2_zero_dn4))), (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign20490_e20670 * locals.var_q_d2_zero_dn6))), (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign20490_e20670 * locals.var_q_d2_zero_dn7))), (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign20490_e20670 * locals.var_q_d2_zero_dn8))), (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign20490_e20670 * locals.var_q_d2_zero_dn9))),)
    } else {
        (locals.var_q_temp, locals.var_q_temp_dn4, locals.var_q_temp_dn6, locals.var_q_temp_dn7, locals.var_q_temp_dn8, locals.var_q_temp_dn9,)
    }
};
        locals.var_q_temp = assign20490_e20675;
        locals.var_q_temp_dn4 = assign20490_e20675_d_n4;
        locals.var_q_temp_dn6 = assign20490_e20675_d_n6;
        locals.var_q_temp_dn7 = assign20490_e20675_d_n7;
        locals.var_q_temp_dn8 = assign20490_e20675_d_n8;
        locals.var_q_temp_dn9 = assign20490_e20675_d_n9;
        locals.var_q_temp_rv = 0.0;

        let (assign20500_e20692, assign20500_e20692_d_n4, assign20500_e20692_d_n6, assign20500_e20692_d_n7, assign20500_e20692_d_n8, assign20500_e20692_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20500_e20680: f64 = (-locals.var_q_zero);
        let assign20500_e20682: f64 = (assign20500_e20680 * locals.var_q_d1_zero);
        let assign20500_e20684: f64 = (assign20500_e20682 * locals.var_q_temp);
        let assign20500_e20687: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign20500_e20689: f64 = (assign20500_e20687 + 1e-200);
        let assign20500_e20690: f64 = (assign20500_e20684 / assign20500_e20689);
        (assign20500_e20690, ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign20500_e20680 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign20500_e20682 * locals.var_q_temp_dn4)) * assign20500_e20689) - (assign20500_e20684 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign20500_e20689 * assign20500_e20689)), ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign20500_e20680 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign20500_e20682 * locals.var_q_temp_dn6)) * assign20500_e20689) - (assign20500_e20684 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign20500_e20689 * assign20500_e20689)), ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign20500_e20680 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign20500_e20682 * locals.var_q_temp_dn7)) * assign20500_e20689) - (assign20500_e20684 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign20500_e20689 * assign20500_e20689)), ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign20500_e20680 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign20500_e20682 * locals.var_q_temp_dn8)) * assign20500_e20689) - (assign20500_e20684 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign20500_e20689 * assign20500_e20689)), ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign20500_e20680 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign20500_e20682 * locals.var_q_temp_dn9)) * assign20500_e20689) - (assign20500_e20684 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign20500_e20689 * assign20500_e20689)),)
    } else {
        (locals.var_q_eps2, locals.var_q_eps2_dn4, locals.var_q_eps2_dn6, locals.var_q_eps2_dn7, locals.var_q_eps2_dn8, locals.var_q_eps2_dn9,)
    }
};
        locals.var_q_eps2 = assign20500_e20692;
        locals.var_q_eps2_dn4 = assign20500_e20692_d_n4;
        locals.var_q_eps2_dn6 = assign20500_e20692_d_n6;
        locals.var_q_eps2_dn7 = assign20500_e20692_d_n7;
        locals.var_q_eps2_dn8 = assign20500_e20692_d_n8;
        locals.var_q_eps2_dn9 = assign20500_e20692_d_n9;
        locals.var_q_eps2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_56(
        locals: &mut StampLocals,
    ) {
        let (assign20510_e20700, assign20510_e20700_d_n4, assign20510_e20700_d_n6, assign20510_e20700_d_n7, assign20510_e20700_d_n8, assign20510_e20700_d_n9,) = {
    if ((locals.var_guard646 != 0.0) && (locals.var_guard647 != 0.0)) {
        let assign20510_e20698: f64 = (locals.var_q1d + locals.var_q_eps2);
        (assign20510_e20698, (locals.var_q1d_dn4 + locals.var_q_eps2_dn4), (locals.var_q1d_dn6 + locals.var_q_eps2_dn6), (locals.var_q1d_dn7 + locals.var_q_eps2_dn7), (locals.var_q1d_dn8 + locals.var_q_eps2_dn8), (locals.var_q1d_dn9 + locals.var_q_eps2_dn9),)
    } else {
        (locals.var_q1d, locals.var_q1d_dn4, locals.var_q1d_dn6, locals.var_q1d_dn7, locals.var_q1d_dn8, locals.var_q1d_dn9,)
    }
};
        locals.var_q1d = assign20510_e20700;
        locals.var_q1d_dn4 = assign20510_e20700_d_n4;
        locals.var_q1d_dn6 = assign20510_e20700_d_n6;
        locals.var_q1d_dn7 = assign20510_e20700_d_n7;
        locals.var_q1d_dn8 = assign20510_e20700_d_n8;
        locals.var_q1d_dn9 = assign20510_e20700_d_n9;
        locals.var_q1d_rv = 0.0;

        let assign20520_e20703: f64 = (locals.var_k1 * locals.var_q1d);
        locals.var_k1q1d = assign20520_e20703;
        locals.var_k1q1d_dn4 = ((locals.var_k1_dn4 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn4));
        locals.var_k1q1d_dn6 = ((locals.var_k1_dn6 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn6));
        locals.var_k1q1d_dn7 = ((locals.var_k1_dn7 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn7));
        locals.var_k1q1d_dn8 = ((locals.var_k1_dn8 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn8));
        locals.var_k1q1d_dn9 = ((locals.var_k1_dn9 * locals.var_q1d) + (locals.var_k1 * locals.var_q1d_dn9));
        locals.var_k1q1d_rv = 0.0;

        let assign20530_e20706: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign20530_e20708: f64 = (assign20530_e20706 - locals.var_xdeff);
        let assign20530_e20710: f64 = if assign20530_e20708 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard655 = assign20530_e20710;
        locals.var_guard655_rv = 0.0;

        let (assign20540_e20719, assign20540_e20719_d_n4, assign20540_e20719_d_n6, assign20540_e20719_d_n7, assign20540_e20719_d_n8, assign20540_e20719_d_n9,) = {
    if (locals.var_guard655 != 0.0) {
        let assign20540_e20714: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign20540_e20716: f64 = (assign20540_e20714 - locals.var_xdeff);
        let assign20540_e20717: f64 = (assign20540_e20716).exp();
        (assign20540_e20717, (assign20540_e20717 * ((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4)), (assign20540_e20717 * ((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6)), (assign20540_e20717 * ((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7)), (assign20540_e20717 * ((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8)), (assign20540_e20717 * ((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign20540_e20719;
        locals.var_q_temp1_dn4 = assign20540_e20719_d_n4;
        locals.var_q_temp1_dn6 = assign20540_e20719_d_n6;
        locals.var_q_temp1_dn7 = assign20540_e20719_d_n7;
        locals.var_q_temp1_dn8 = assign20540_e20719_d_n8;
        locals.var_q_temp1_dn9 = assign20540_e20719_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign20550_e20758, assign20550_e20758_d_n4, assign20550_e20758_d_n6, assign20550_e20758_d_n7, assign20550_e20758_d_n8, assign20550_e20758_d_n9,) = {
    if (locals.var_guard655 == 0.0) {
        let assign20550_e20726: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign20550_e20728: f64 = (assign20550_e20726 - locals.var_xdeff);
        let assign20550_e20730: f64 = (assign20550_e20728 - 80.0);
        let assign20550_e20735: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign20550_e20737: f64 = (assign20550_e20735 - locals.var_xdeff);
        let assign20550_e20739: f64 = (assign20550_e20737 - 80.0);
        let assign20550_e20740: f64 = (0.5 * assign20550_e20739);
        let assign20550_e20744: f64 = (locals.var_xg1x - locals.var_q1d);
        let assign20550_e20746: f64 = (assign20550_e20744 - locals.var_xdeff);
        let assign20550_e20748: f64 = (assign20550_e20746 - 80.0);
        let assign20550_e20750: f64 = (assign20550_e20748 * 0.3333333333333);
        let assign20550_e20751: f64 = (1.0 + assign20550_e20750);
        let assign20550_e20752: f64 = (assign20550_e20740 * assign20550_e20751);
        let assign20550_e20753: f64 = (1.0 + assign20550_e20752);
        let assign20550_e20754: f64 = (assign20550_e20730 * assign20550_e20753);
        let assign20550_e20755: f64 = (1.0 + assign20550_e20754);
        let assign20550_e20756: f64 = (5.54062e34 * assign20550_e20755);
        (assign20550_e20756, (5.54062e34 * ((((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4) * assign20550_e20753) + (assign20550_e20730 * (((0.5 * ((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4)) * assign20550_e20751) + (assign20550_e20740 * (((locals.var_xg1x_dn4 - locals.var_q1d_dn4) - locals.var_xdeff_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6) * assign20550_e20753) + (assign20550_e20730 * (((0.5 * ((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6)) * assign20550_e20751) + (assign20550_e20740 * (((locals.var_xg1x_dn6 - locals.var_q1d_dn6) - locals.var_xdeff_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7) * assign20550_e20753) + (assign20550_e20730 * (((0.5 * ((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7)) * assign20550_e20751) + (assign20550_e20740 * (((locals.var_xg1x_dn7 - locals.var_q1d_dn7) - locals.var_xdeff_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8) * assign20550_e20753) + (assign20550_e20730 * (((0.5 * ((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8)) * assign20550_e20751) + (assign20550_e20740 * (((locals.var_xg1x_dn8 - locals.var_q1d_dn8) - locals.var_xdeff_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9) * assign20550_e20753) + (assign20550_e20730 * (((0.5 * ((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9)) * assign20550_e20751) + (assign20550_e20740 * (((locals.var_xg1x_dn9 - locals.var_q1d_dn9) - locals.var_xdeff_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign20550_e20758;
        locals.var_q_temp1_dn4 = assign20550_e20758_d_n4;
        locals.var_q_temp1_dn6 = assign20550_e20758_d_n6;
        locals.var_q_temp1_dn7 = assign20550_e20758_d_n7;
        locals.var_q_temp1_dn8 = assign20550_e20758_d_n8;
        locals.var_q_temp1_dn9 = assign20550_e20758_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let assign20560_e20761: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_aexp1d = assign20560_e20761;
        locals.var_aexp1d_dn4 = ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4));
        locals.var_aexp1d_dn6 = ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6));
        locals.var_aexp1d_dn7 = ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7));
        locals.var_aexp1d_dn8 = ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8));
        locals.var_aexp1d_dn9 = ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9));
        locals.var_aexp1d_rv = 0.0;

        let assign20570_e20764: f64 = (locals.var_k1q1d * locals.var_k1q1d);
        let assign20570_e20766: f64 = (assign20570_e20764 - locals.var_aexp1d);
        locals.var_qsqd = assign20570_e20766;
        locals.var_qsqd_dn4 = (((locals.var_k1q1d_dn4 * locals.var_k1q1d) + (locals.var_k1q1d * locals.var_k1q1d_dn4)) - locals.var_aexp1d_dn4);
        locals.var_qsqd_dn6 = (((locals.var_k1q1d_dn6 * locals.var_k1q1d) + (locals.var_k1q1d * locals.var_k1q1d_dn6)) - locals.var_aexp1d_dn6);
        locals.var_qsqd_dn7 = (((locals.var_k1q1d_dn7 * locals.var_k1q1d) + (locals.var_k1q1d * locals.var_k1q1d_dn7)) - locals.var_aexp1d_dn7);
        locals.var_qsqd_dn8 = (((locals.var_k1q1d_dn8 * locals.var_k1q1d) + (locals.var_k1q1d * locals.var_k1q1d_dn8)) - locals.var_aexp1d_dn8);
        locals.var_qsqd_dn9 = (((locals.var_k1q1d_dn9 * locals.var_k1q1d) + (locals.var_k1q1d * locals.var_k1q1d_dn9)) - locals.var_aexp1d_dn9);
        locals.var_qsqd_rv = 0.0;

        let assign20580_e20769: f64 = if locals.var_aexp1d <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard656 = assign20580_e20769;
        locals.var_guard656_rv = 0.0;

        let (assign20590_e20773, assign20590_e20773_d_n4, assign20590_e20773_d_n6, assign20590_e20773_d_n7, assign20590_e20773_d_n8, assign20590_e20773_d_n9,) = {
    if (locals.var_guard656 != 0.0) {
        (1e-80, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qid, locals.var_qid_dn4, locals.var_qid_dn6, locals.var_qid_dn7, locals.var_qid_dn8, locals.var_qid_dn9,)
    }
};
        locals.var_qid = assign20590_e20773;
        locals.var_qid_dn4 = assign20590_e20773_d_n4;
        locals.var_qid_dn6 = assign20590_e20773_d_n6;
        locals.var_qid_dn7 = assign20590_e20773_d_n7;
        locals.var_qid_dn8 = assign20590_e20773_d_n8;
        locals.var_qid_dn9 = assign20590_e20773_d_n9;
        locals.var_qid_rv = 0.0;

        let (assign20600_e20779, assign20600_e20779_d_n4, assign20600_e20779_d_n6, assign20600_e20779_d_n7, assign20600_e20779_d_n8, assign20600_e20779_d_n9,) = {
    if (locals.var_guard656 != 0.0) {
        let assign20600_e20777: f64 = (locals.var_qid - locals.var_k1q1d);
        (assign20600_e20777, (locals.var_qid_dn4 - locals.var_k1q1d_dn4), (locals.var_qid_dn6 - locals.var_k1q1d_dn6), (locals.var_qid_dn7 - locals.var_k1q1d_dn7), (locals.var_qid_dn8 - locals.var_k1q1d_dn8), (locals.var_qid_dn9 - locals.var_k1q1d_dn9),)
    } else {
        (locals.var_k2q2d, locals.var_k2q2d_dn4, locals.var_k2q2d_dn6, locals.var_k2q2d_dn7, locals.var_k2q2d_dn8, locals.var_k2q2d_dn9,)
    }
};
        locals.var_k2q2d = assign20600_e20779;
        locals.var_k2q2d_dn4 = assign20600_e20779_d_n4;
        locals.var_k2q2d_dn6 = assign20600_e20779_d_n6;
        locals.var_k2q2d_dn7 = assign20600_e20779_d_n7;
        locals.var_k2q2d_dn8 = assign20600_e20779_d_n8;
        locals.var_k2q2d_dn9 = assign20600_e20779_d_n9;
        locals.var_k2q2d_rv = 0.0;

        let (assign20610_e20785, assign20610_e20785_d_n4, assign20610_e20785_d_n6, assign20610_e20785_d_n7, assign20610_e20785_d_n8, assign20610_e20785_d_n9,) = {
    if (locals.var_guard656 != 0.0) {
        let assign20610_e20783: f64 = (locals.var_k2q2d / locals.var_k2);
        (assign20610_e20783, (((locals.var_k2q2d_dn4 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn4)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn6 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn6)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn7 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn7)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn8 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn8)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn9 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn9)) / (locals.var_k2 * locals.var_k2)),)
    } else {
        (locals.var_q2d, locals.var_q2d_dn4, locals.var_q2d_dn6, locals.var_q2d_dn7, locals.var_q2d_dn8, locals.var_q2d_dn9,)
    }
};
        locals.var_q2d = assign20610_e20785;
        locals.var_q2d_dn4 = assign20610_e20785_d_n4;
        locals.var_q2d_dn6 = assign20610_e20785_d_n6;
        locals.var_q2d_dn7 = assign20610_e20785_d_n7;
        locals.var_q2d_dn8 = assign20610_e20785_d_n8;
        locals.var_q2d_dn9 = assign20610_e20785_d_n9;
        locals.var_q2d_rv = 0.0;

        let assign20620_e20788: f64 = (-0.005);
        let assign20620_e20789: f64 = if locals.var_qsqd < assign20620_e20788 { 1.0 } else { 0.0 };
        locals.var_guard657 = assign20620_e20789;
        locals.var_guard657_rv = 0.0;

        let (assign20630_e20798, assign20630_e20798_d_n4, assign20630_e20798_d_n6, assign20630_e20798_d_n7, assign20630_e20798_d_n8, assign20630_e20798_d_n9,) = {
    if ((locals.var_guard656 == 0.0) && (locals.var_guard657 != 0.0)) {
        let assign20630_e20795: f64 = (locals.var_qsqd).abs();
        let assign20630_e20796: f64 = (assign20630_e20795).sqrt();
        (assign20630_e20796, (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn4 } else { (-locals.var_qsqd_dn4) } / (2.0 * assign20630_e20796)), (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn6 } else { (-locals.var_qsqd_dn6) } / (2.0 * assign20630_e20796)), (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn7 } else { (-locals.var_qsqd_dn7) } / (2.0 * assign20630_e20796)), (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn8 } else { (-locals.var_qsqd_dn8) } / (2.0 * assign20630_e20796)), (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn9 } else { (-locals.var_qsqd_dn9) } / (2.0 * assign20630_e20796)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign20630_e20798;
        locals.var_q_rac_qsq_dn4 = assign20630_e20798_d_n4;
        locals.var_q_rac_qsq_dn6 = assign20630_e20798_d_n6;
        locals.var_q_rac_qsq_dn7 = assign20630_e20798_d_n7;
        locals.var_q_rac_qsq_dn8 = assign20630_e20798_d_n8;
        locals.var_q_rac_qsq_dn9 = assign20630_e20798_d_n9;
        locals.var_q_rac_qsq_rv = 0.0;

        let (assign20640_e20810, assign20640_e20810_d_n4, assign20640_e20810_d_n6, assign20640_e20810_d_n7, assign20640_e20810_d_n8, assign20640_e20810_d_n9,) = {
    if ((locals.var_guard656 == 0.0) && (locals.var_guard657 != 0.0)) {
        let assign20640_e20806: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign20640_e20807: f64 = (assign20640_e20806).tan();
        let assign20640_e20808: f64 = (locals.var_q_rac_qsq / assign20640_e20807);
        (assign20640_e20808, (((locals.var_q_rac_qsq_dn4 * assign20640_e20807) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign20640_e20806).cos() * (assign20640_e20806).cos())))) / (assign20640_e20807 * assign20640_e20807)), (((locals.var_q_rac_qsq_dn6 * assign20640_e20807) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign20640_e20806).cos() * (assign20640_e20806).cos())))) / (assign20640_e20807 * assign20640_e20807)), (((locals.var_q_rac_qsq_dn7 * assign20640_e20807) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign20640_e20806).cos() * (assign20640_e20806).cos())))) / (assign20640_e20807 * assign20640_e20807)), (((locals.var_q_rac_qsq_dn8 * assign20640_e20807) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign20640_e20806).cos() * (assign20640_e20806).cos())))) / (assign20640_e20807 * assign20640_e20807)), (((locals.var_q_rac_qsq_dn9 * assign20640_e20807) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign20640_e20806).cos() * (assign20640_e20806).cos())))) / (assign20640_e20807 * assign20640_e20807)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign20640_e20810;
        locals.var_q_qcoth_dn4 = assign20640_e20810_d_n4;
        locals.var_q_qcoth_dn6 = assign20640_e20810_d_n6;
        locals.var_q_qcoth_dn7 = assign20640_e20810_d_n7;
        locals.var_q_qcoth_dn8 = assign20640_e20810_d_n8;
        locals.var_q_qcoth_dn9 = assign20640_e20810_d_n9;
        locals.var_q_qcoth_rv = 0.0;

        let assign20650_e20813: f64 = if locals.var_qsqd > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard658 = assign20650_e20813;
        locals.var_guard658_rv = 0.0;

        let (assign20660_e20825, assign20660_e20825_d_n4, assign20660_e20825_d_n6, assign20660_e20825_d_n7, assign20660_e20825_d_n8, assign20660_e20825_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard657 == 0.0)) && (locals.var_guard658 != 0.0)) {
        let assign20660_e20822: f64 = (locals.var_qsqd).abs();
        let assign20660_e20823: f64 = (assign20660_e20822).sqrt();
        (assign20660_e20823, (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn4 } else { (-locals.var_qsqd_dn4) } / (2.0 * assign20660_e20823)), (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn6 } else { (-locals.var_qsqd_dn6) } / (2.0 * assign20660_e20823)), (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn7 } else { (-locals.var_qsqd_dn7) } / (2.0 * assign20660_e20823)), (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn8 } else { (-locals.var_qsqd_dn8) } / (2.0 * assign20660_e20823)), (if locals.var_qsqd >= 0.0 { locals.var_qsqd_dn9 } else { (-locals.var_qsqd_dn9) } / (2.0 * assign20660_e20823)),)
    } else {
        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9,)
    }
};
        locals.var_q_rac_qsq = assign20660_e20825;
        locals.var_q_rac_qsq_dn4 = assign20660_e20825_d_n4;
        locals.var_q_rac_qsq_dn6 = assign20660_e20825_d_n6;
        locals.var_q_rac_qsq_dn7 = assign20660_e20825_d_n7;
        locals.var_q_rac_qsq_dn8 = assign20660_e20825_d_n8;
        locals.var_q_rac_qsq_dn9 = assign20660_e20825_d_n9;
        locals.var_q_rac_qsq_rv = 0.0;

        let (assign20670_e20837, assign20670_e20837_d_n4, assign20670_e20837_d_n6, assign20670_e20837_d_n7, assign20670_e20837_d_n8, assign20670_e20837_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard657 == 0.0)) && (locals.var_guard658 != 0.0)) {
        let assign20670_e20834: f64 = (-locals.var_q_rac_qsq);
        let assign20670_e20835: f64 = (assign20670_e20834).exp();
        (assign20670_e20835, (assign20670_e20835 * (-locals.var_q_rac_qsq_dn4)), (assign20670_e20835 * (-locals.var_q_rac_qsq_dn6)), (assign20670_e20835 * (-locals.var_q_rac_qsq_dn7)), (assign20670_e20835 * (-locals.var_q_rac_qsq_dn8)), (assign20670_e20835 * (-locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9,)
    }
};
        locals.var_q_invexpq = assign20670_e20837;
        locals.var_q_invexpq_dn4 = assign20670_e20837_d_n4;
        locals.var_q_invexpq_dn6 = assign20670_e20837_d_n6;
        locals.var_q_invexpq_dn7 = assign20670_e20837_d_n7;
        locals.var_q_invexpq_dn8 = assign20670_e20837_d_n8;
        locals.var_q_invexpq_dn9 = assign20670_e20837_d_n9;
        locals.var_q_invexpq_rv = 0.0;

        let (assign20680_e20855, assign20680_e20855_d_n4, assign20680_e20855_d_n6, assign20680_e20855_d_n7, assign20680_e20855_d_n8, assign20680_e20855_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard657 == 0.0)) && (locals.var_guard658 != 0.0)) {
        let assign20680_e20848: f64 = (1.0 + locals.var_q_invexpq);
        let assign20680_e20849: f64 = (locals.var_q_rac_qsq * assign20680_e20848);
        let assign20680_e20852: f64 = (1.0 - locals.var_q_invexpq);
        let assign20680_e20853: f64 = (assign20680_e20849 / assign20680_e20852);
        (assign20680_e20853, (((((locals.var_q_rac_qsq_dn4 * assign20680_e20848) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign20680_e20852) - (assign20680_e20849 * (-locals.var_q_invexpq_dn4))) / (assign20680_e20852 * assign20680_e20852)), (((((locals.var_q_rac_qsq_dn6 * assign20680_e20848) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign20680_e20852) - (assign20680_e20849 * (-locals.var_q_invexpq_dn6))) / (assign20680_e20852 * assign20680_e20852)), (((((locals.var_q_rac_qsq_dn7 * assign20680_e20848) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign20680_e20852) - (assign20680_e20849 * (-locals.var_q_invexpq_dn7))) / (assign20680_e20852 * assign20680_e20852)), (((((locals.var_q_rac_qsq_dn8 * assign20680_e20848) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign20680_e20852) - (assign20680_e20849 * (-locals.var_q_invexpq_dn8))) / (assign20680_e20852 * assign20680_e20852)), (((((locals.var_q_rac_qsq_dn9 * assign20680_e20848) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign20680_e20852) - (assign20680_e20849 * (-locals.var_q_invexpq_dn9))) / (assign20680_e20852 * assign20680_e20852)),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign20680_e20855;
        locals.var_q_qcoth_dn4 = assign20680_e20855_d_n4;
        locals.var_q_qcoth_dn6 = assign20680_e20855_d_n6;
        locals.var_q_qcoth_dn7 = assign20680_e20855_d_n7;
        locals.var_q_qcoth_dn8 = assign20680_e20855_d_n8;
        locals.var_q_qcoth_dn9 = assign20680_e20855_d_n9;
        locals.var_q_qcoth_rv = 0.0;

        let (assign20690_e20882, assign20690_e20882_d_n4, assign20690_e20882_d_n6, assign20690_e20882_d_n7, assign20690_e20882_d_n8, assign20690_e20882_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard657 == 0.0)) && (locals.var_guard658 == 0.0)) {
        let assign20690_e20867: f64 = (locals.var_qsqd * 0.1666666666667);
        let assign20690_e20871: f64 = (locals.var_qsqd * 0.0166666666667);
        let assign20690_e20875: f64 = (locals.var_qsqd * 0.0238095238095);
        let assign20690_e20876: f64 = (1.0 - assign20690_e20875);
        let assign20690_e20877: f64 = (assign20690_e20871 * assign20690_e20876);
        let assign20690_e20878: f64 = (1.0 - assign20690_e20877);
        let assign20690_e20879: f64 = (assign20690_e20867 * assign20690_e20878);
        let assign20690_e20880: f64 = (2.0 + assign20690_e20879);
        (assign20690_e20880, (((locals.var_qsqd_dn4 * 0.1666666666667) * assign20690_e20878) + (assign20690_e20867 * (-(((locals.var_qsqd_dn4 * 0.0166666666667) * assign20690_e20876) + (assign20690_e20871 * (-(locals.var_qsqd_dn4 * 0.0238095238095))))))), (((locals.var_qsqd_dn6 * 0.1666666666667) * assign20690_e20878) + (assign20690_e20867 * (-(((locals.var_qsqd_dn6 * 0.0166666666667) * assign20690_e20876) + (assign20690_e20871 * (-(locals.var_qsqd_dn6 * 0.0238095238095))))))), (((locals.var_qsqd_dn7 * 0.1666666666667) * assign20690_e20878) + (assign20690_e20867 * (-(((locals.var_qsqd_dn7 * 0.0166666666667) * assign20690_e20876) + (assign20690_e20871 * (-(locals.var_qsqd_dn7 * 0.0238095238095))))))), (((locals.var_qsqd_dn8 * 0.1666666666667) * assign20690_e20878) + (assign20690_e20867 * (-(((locals.var_qsqd_dn8 * 0.0166666666667) * assign20690_e20876) + (assign20690_e20871 * (-(locals.var_qsqd_dn8 * 0.0238095238095))))))), (((locals.var_qsqd_dn9 * 0.1666666666667) * assign20690_e20878) + (assign20690_e20867 * (-(((locals.var_qsqd_dn9 * 0.0166666666667) * assign20690_e20876) + (assign20690_e20871 * (-(locals.var_qsqd_dn9 * 0.0238095238095))))))),)
    } else {
        (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9,)
    }
};
        locals.var_q_qcoth = assign20690_e20882;
        locals.var_q_qcoth_dn4 = assign20690_e20882_d_n4;
        locals.var_q_qcoth_dn6 = assign20690_e20882_d_n6;
        locals.var_q_qcoth_dn7 = assign20690_e20882_d_n7;
        locals.var_q_qcoth_dn8 = assign20690_e20882_d_n8;
        locals.var_q_qcoth_dn9 = assign20690_e20882_d_n9;
        locals.var_q_qcoth_rv = 0.0;

        let assign20700_e20885: f64 = (1.01 * locals.var_k1q1d);
        let assign20700_e20887: f64 = (assign20700_e20885 + locals.var_q_qcoth);
        let assign20700_e20889: f64 = if assign20700_e20887 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard659 = assign20700_e20889;
        locals.var_guard659_rv = 0.0;

        let (assign20710_e20898, assign20710_e20898_d_n4, assign20710_e20898_d_n6, assign20710_e20898_d_n7, assign20710_e20898_d_n8, assign20710_e20898_d_n9,) = {
    if ((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) {
        let assign20710_e20896: f64 = (locals.var_k1q1d + locals.var_q_qcoth);
        (assign20710_e20896, (locals.var_k1q1d_dn4 + locals.var_q_qcoth_dn4), (locals.var_k1q1d_dn6 + locals.var_q_qcoth_dn6), (locals.var_k1q1d_dn7 + locals.var_q_qcoth_dn7), (locals.var_k1q1d_dn8 + locals.var_q_qcoth_dn8), (locals.var_k1q1d_dn9 + locals.var_q_qcoth_dn9),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign20710_e20898;
        locals.var_q_temp1_dn4 = assign20710_e20898_d_n4;
        locals.var_q_temp1_dn6 = assign20710_e20898_d_n6;
        locals.var_q_temp1_dn7 = assign20710_e20898_d_n7;
        locals.var_q_temp1_dn8 = assign20710_e20898_d_n8;
        locals.var_q_temp1_dn9 = assign20710_e20898_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let assign20720_e20901: f64 = (locals.var_aexp1d * locals.var_k1q1d);
        let assign20720_e20904: f64 = (0.9 * locals.var_k1q1d);
        let assign20720_e20906: f64 = (assign20720_e20904 * locals.var_k1q1d);
        let assign20720_e20908: f64 = (assign20720_e20906 * locals.var_q_temp1);
        let assign20720_e20909: f64 = if assign20720_e20901 < assign20720_e20908 { 1.0 } else { 0.0 };
        locals.var_guard660 = assign20720_e20909;
        locals.var_guard660_rv = 0.0;

        let (assign20730_e20922, assign20730_e20922_d_n4, assign20730_e20922_d_n6, assign20730_e20922_d_n7, assign20730_e20922_d_n8, assign20730_e20922_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 != 0.0)) {
        let assign20730_e20918: f64 = (locals.var_aexp1d / locals.var_q_temp1);
        let assign20730_e20920: f64 = (assign20730_e20918 + 1e-80);
        (assign20730_e20920, (((locals.var_aexp1d_dn4 * locals.var_q_temp1) - (locals.var_aexp1d * locals.var_q_temp1_dn4)) / (locals.var_q_temp1 * locals.var_q_temp1)), (((locals.var_aexp1d_dn6 * locals.var_q_temp1) - (locals.var_aexp1d * locals.var_q_temp1_dn6)) / (locals.var_q_temp1 * locals.var_q_temp1)), (((locals.var_aexp1d_dn7 * locals.var_q_temp1) - (locals.var_aexp1d * locals.var_q_temp1_dn7)) / (locals.var_q_temp1 * locals.var_q_temp1)), (((locals.var_aexp1d_dn8 * locals.var_q_temp1) - (locals.var_aexp1d * locals.var_q_temp1_dn8)) / (locals.var_q_temp1 * locals.var_q_temp1)), (((locals.var_aexp1d_dn9 * locals.var_q_temp1) - (locals.var_aexp1d * locals.var_q_temp1_dn9)) / (locals.var_q_temp1 * locals.var_q_temp1)),)
    } else {
        (locals.var_qid, locals.var_qid_dn4, locals.var_qid_dn6, locals.var_qid_dn7, locals.var_qid_dn8, locals.var_qid_dn9,)
    }
};
        locals.var_qid = assign20730_e20922;
        locals.var_qid_dn4 = assign20730_e20922_d_n4;
        locals.var_qid_dn6 = assign20730_e20922_d_n6;
        locals.var_qid_dn7 = assign20730_e20922_d_n7;
        locals.var_qid_dn8 = assign20730_e20922_d_n8;
        locals.var_qid_dn9 = assign20730_e20922_d_n9;
        locals.var_qid_rv = 0.0;

        let (assign20740_e20933, assign20740_e20933_d_n4, assign20740_e20933_d_n6, assign20740_e20933_d_n7, assign20740_e20933_d_n8, assign20740_e20933_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 != 0.0)) {
        let assign20740_e20931: f64 = (locals.var_qid - locals.var_k1q1d);
        (assign20740_e20931, (locals.var_qid_dn4 - locals.var_k1q1d_dn4), (locals.var_qid_dn6 - locals.var_k1q1d_dn6), (locals.var_qid_dn7 - locals.var_k1q1d_dn7), (locals.var_qid_dn8 - locals.var_k1q1d_dn8), (locals.var_qid_dn9 - locals.var_k1q1d_dn9),)
    } else {
        (locals.var_k2q2d, locals.var_k2q2d_dn4, locals.var_k2q2d_dn6, locals.var_k2q2d_dn7, locals.var_k2q2d_dn8, locals.var_k2q2d_dn9,)
    }
};
        locals.var_k2q2d = assign20740_e20933;
        locals.var_k2q2d_dn4 = assign20740_e20933_d_n4;
        locals.var_k2q2d_dn6 = assign20740_e20933_d_n6;
        locals.var_k2q2d_dn7 = assign20740_e20933_d_n7;
        locals.var_k2q2d_dn8 = assign20740_e20933_d_n8;
        locals.var_k2q2d_dn9 = assign20740_e20933_d_n9;
        locals.var_k2q2d_rv = 0.0;

        let (assign20750_e20944, assign20750_e20944_d_n4, assign20750_e20944_d_n6, assign20750_e20944_d_n7, assign20750_e20944_d_n8, assign20750_e20944_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 != 0.0)) {
        let assign20750_e20942: f64 = (locals.var_k2q2d / locals.var_k2);
        (assign20750_e20942, (((locals.var_k2q2d_dn4 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn4)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn6 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn6)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn7 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn7)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn8 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn8)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn9 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn9)) / (locals.var_k2 * locals.var_k2)),)
    } else {
        (locals.var_q2d, locals.var_q2d_dn4, locals.var_q2d_dn6, locals.var_q2d_dn7, locals.var_q2d_dn8, locals.var_q2d_dn9,)
    }
};
        locals.var_q2d = assign20750_e20944;
        locals.var_q2d_dn4 = assign20750_e20944_d_n4;
        locals.var_q2d_dn6 = assign20750_e20944_d_n6;
        locals.var_q2d_dn7 = assign20750_e20944_d_n7;
        locals.var_q2d_dn8 = assign20750_e20944_d_n8;
        locals.var_q2d_dn9 = assign20750_e20944_d_n9;
        locals.var_q2d_rv = 0.0;

        let assign20760_e20947: f64 = if locals.var_qsqd > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard661 = assign20760_e20947;
        locals.var_guard661_rv = 0.0;

        let (assign20770_e20972, assign20770_e20972_d_n4, assign20770_e20972_d_n6, assign20770_e20972_d_n7, assign20770_e20972_d_n8, assign20770_e20972_d_n9,) = {
    if ((((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 == 0.0)) && (locals.var_guard661 != 0.0)) {
        let assign20770_e20959: f64 = (4.0 * locals.var_qsqd);
        let assign20770_e20964: f64 = (2.0 - locals.var_q_invexpq);
        let assign20770_e20965: f64 = (locals.var_q_invexpq * assign20770_e20964);
        let assign20770_e20966: f64 = (1.0 - assign20770_e20965);
        let assign20770_e20967: f64 = (assign20770_e20959 / assign20770_e20966);
        let assign20770_e20968: f64 = (assign20770_e20967).ln();
        let assign20770_e20970: f64 = (assign20770_e20968 - locals.var_q_rac_qsq);
        (assign20770_e20970, ((((((4.0 * locals.var_qsqd_dn4) * assign20770_e20966) - (assign20770_e20959 * (-((locals.var_q_invexpq_dn4 * assign20770_e20964) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign20770_e20966 * assign20770_e20966)) / assign20770_e20967) - locals.var_q_rac_qsq_dn4), ((((((4.0 * locals.var_qsqd_dn6) * assign20770_e20966) - (assign20770_e20959 * (-((locals.var_q_invexpq_dn6 * assign20770_e20964) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign20770_e20966 * assign20770_e20966)) / assign20770_e20967) - locals.var_q_rac_qsq_dn6), ((((((4.0 * locals.var_qsqd_dn7) * assign20770_e20966) - (assign20770_e20959 * (-((locals.var_q_invexpq_dn7 * assign20770_e20964) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign20770_e20966 * assign20770_e20966)) / assign20770_e20967) - locals.var_q_rac_qsq_dn7), ((((((4.0 * locals.var_qsqd_dn8) * assign20770_e20966) - (assign20770_e20959 * (-((locals.var_q_invexpq_dn8 * assign20770_e20964) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign20770_e20966 * assign20770_e20966)) / assign20770_e20967) - locals.var_q_rac_qsq_dn8), ((((((4.0 * locals.var_qsqd_dn9) * assign20770_e20966) - (assign20770_e20959 * (-((locals.var_q_invexpq_dn9 * assign20770_e20964) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign20770_e20966 * assign20770_e20966)) / assign20770_e20967) - locals.var_q_rac_qsq_dn9),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20770_e20972;
        locals.var_q_temp2_dn4 = assign20770_e20972_d_n4;
        locals.var_q_temp2_dn6 = assign20770_e20972_d_n6;
        locals.var_q_temp2_dn7 = assign20770_e20972_d_n7;
        locals.var_q_temp2_dn8 = assign20770_e20972_d_n8;
        locals.var_q_temp2_dn9 = assign20770_e20972_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let assign20780_e20975: f64 = (-0.005);
        let assign20780_e20976: f64 = if locals.var_qsqd < assign20780_e20975 { 1.0 } else { 0.0 };
        locals.var_guard662 = assign20780_e20976;
        locals.var_guard662_rv = 0.0;

        let (assign20790_e20994, assign20790_e20994_d_n4, assign20790_e20994_d_n6, assign20790_e20994_d_n7, assign20790_e20994_d_n8, assign20790_e20994_d_n9,) = {
    if (((((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 == 0.0)) && (locals.var_guard661 == 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign20790_e20991: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign20790_e20992: f64 = (assign20790_e20991).sin();
        (assign20790_e20992, ((assign20790_e20991).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign20790_e20991).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign20790_e20991).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign20790_e20991).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign20790_e20991).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign20790_e20994;
        locals.var_q_temp3_dn4 = assign20790_e20994_d_n4;
        locals.var_q_temp3_dn6 = assign20790_e20994_d_n6;
        locals.var_q_temp3_dn7 = assign20790_e20994_d_n7;
        locals.var_q_temp3_dn8 = assign20790_e20994_d_n8;
        locals.var_q_temp3_dn9 = assign20790_e20994_d_n9;
        locals.var_q_temp3_rv = 0.0;

        let (assign20800_e21015, assign20800_e21015_d_n4, assign20800_e21015_d_n6, assign20800_e21015_d_n7, assign20800_e21015_d_n8, assign20800_e21015_d_n9,) = {
    if (((((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 == 0.0)) && (locals.var_guard661 == 0.0)) && (locals.var_guard662 != 0.0)) {
        let assign20800_e21008: f64 = (-locals.var_qsqd);
        let assign20800_e21011: f64 = (locals.var_q_temp3 * locals.var_q_temp3);
        let assign20800_e21012: f64 = (assign20800_e21008 / assign20800_e21011);
        let assign20800_e21013: f64 = (assign20800_e21012).ln();
        (assign20800_e21013, (((((-locals.var_qsqd_dn4) * assign20800_e21011) - (assign20800_e21008 * ((locals.var_q_temp3_dn4 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn4)))) / (assign20800_e21011 * assign20800_e21011)) / assign20800_e21012), (((((-locals.var_qsqd_dn6) * assign20800_e21011) - (assign20800_e21008 * ((locals.var_q_temp3_dn6 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn6)))) / (assign20800_e21011 * assign20800_e21011)) / assign20800_e21012), (((((-locals.var_qsqd_dn7) * assign20800_e21011) - (assign20800_e21008 * ((locals.var_q_temp3_dn7 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn7)))) / (assign20800_e21011 * assign20800_e21011)) / assign20800_e21012), (((((-locals.var_qsqd_dn8) * assign20800_e21011) - (assign20800_e21008 * ((locals.var_q_temp3_dn8 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn8)))) / (assign20800_e21011 * assign20800_e21011)) / assign20800_e21012), (((((-locals.var_qsqd_dn9) * assign20800_e21011) - (assign20800_e21008 * ((locals.var_q_temp3_dn9 * locals.var_q_temp3) + (locals.var_q_temp3 * locals.var_q_temp3_dn9)))) / (assign20800_e21011 * assign20800_e21011)) / assign20800_e21012),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20800_e21015;
        locals.var_q_temp2_dn4 = assign20800_e21015_d_n4;
        locals.var_q_temp2_dn6 = assign20800_e21015_d_n6;
        locals.var_q_temp2_dn7 = assign20800_e21015_d_n7;
        locals.var_q_temp2_dn8 = assign20800_e21015_d_n8;
        locals.var_q_temp2_dn9 = assign20800_e21015_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign20810_e21048, assign20810_e21048_d_n4, assign20810_e21048_d_n6, assign20810_e21048_d_n7, assign20810_e21048_d_n8, assign20810_e21048_d_n9,) = {
    if (((((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 == 0.0)) && (locals.var_guard661 == 0.0)) && (locals.var_guard662 == 0.0)) {
        let assign20810_e21032: f64 = (locals.var_qsqd * 0.3333333333333);
        let assign20810_e21036: f64 = (0.05 * locals.var_qsqd);
        let assign20810_e21040: f64 = (0.0396825396825397 * locals.var_qsqd);
        let assign20810_e21041: f64 = (1.0 - assign20810_e21040);
        let assign20810_e21042: f64 = (assign20810_e21036 * assign20810_e21041);
        let assign20810_e21043: f64 = (1.0 - assign20810_e21042);
        let assign20810_e21044: f64 = (assign20810_e21032 * assign20810_e21043);
        let assign20810_e21045: f64 = (4.0 - assign20810_e21044);
        let assign20810_e21046: f64 = (assign20810_e21045).ln();
        (assign20810_e21046, ((-(((locals.var_qsqd_dn4 * 0.3333333333333) * assign20810_e21043) + (assign20810_e21032 * (-(((0.05 * locals.var_qsqd_dn4) * assign20810_e21041) + (assign20810_e21036 * (-(0.0396825396825397 * locals.var_qsqd_dn4)))))))) / assign20810_e21045), ((-(((locals.var_qsqd_dn6 * 0.3333333333333) * assign20810_e21043) + (assign20810_e21032 * (-(((0.05 * locals.var_qsqd_dn6) * assign20810_e21041) + (assign20810_e21036 * (-(0.0396825396825397 * locals.var_qsqd_dn6)))))))) / assign20810_e21045), ((-(((locals.var_qsqd_dn7 * 0.3333333333333) * assign20810_e21043) + (assign20810_e21032 * (-(((0.05 * locals.var_qsqd_dn7) * assign20810_e21041) + (assign20810_e21036 * (-(0.0396825396825397 * locals.var_qsqd_dn7)))))))) / assign20810_e21045), ((-(((locals.var_qsqd_dn8 * 0.3333333333333) * assign20810_e21043) + (assign20810_e21032 * (-(((0.05 * locals.var_qsqd_dn8) * assign20810_e21041) + (assign20810_e21036 * (-(0.0396825396825397 * locals.var_qsqd_dn8)))))))) / assign20810_e21045), ((-(((locals.var_qsqd_dn9 * 0.3333333333333) * assign20810_e21043) + (assign20810_e21032 * (-(((0.05 * locals.var_qsqd_dn9) * assign20810_e21041) + (assign20810_e21036 * (-(0.0396825396825397 * locals.var_qsqd_dn9)))))))) / assign20810_e21045),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20810_e21048;
        locals.var_q_temp2_dn4 = assign20810_e21048_d_n4;
        locals.var_q_temp2_dn6 = assign20810_e21048_d_n6;
        locals.var_q_temp2_dn7 = assign20810_e21048_d_n7;
        locals.var_q_temp2_dn8 = assign20810_e21048_d_n8;
        locals.var_q_temp2_dn9 = assign20810_e21048_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign20820_e21069, assign20820_e21069_d_n4, assign20820_e21069_d_n6, assign20820_e21069_d_n7, assign20820_e21069_d_n8, assign20820_e21069_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 == 0.0)) {
        let assign20820_e21058: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign20820_e21060: f64 = (assign20820_e21058 + locals.var_q1d);
        let assign20820_e21063: f64 = (locals.var_q_temp1).ln();
        let assign20820_e21064: f64 = (2.0 * assign20820_e21063);
        let assign20820_e21065: f64 = (assign20820_e21060 + assign20820_e21064);
        let assign20820_e21067: f64 = (assign20820_e21065 - locals.var_q_temp2);
        (assign20820_e21067, ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1d_dn4) + (2.0 * (locals.var_q_temp1_dn4 / locals.var_q_temp1))) - locals.var_q_temp2_dn4), ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1d_dn6) + (2.0 * (locals.var_q_temp1_dn6 / locals.var_q_temp1))) - locals.var_q_temp2_dn6), ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1d_dn7) + (2.0 * (locals.var_q_temp1_dn7 / locals.var_q_temp1))) - locals.var_q_temp2_dn7), ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1d_dn8) + (2.0 * (locals.var_q_temp1_dn8 / locals.var_q_temp1))) - locals.var_q_temp2_dn8), ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1d_dn9) + (2.0 * (locals.var_q_temp1_dn9 / locals.var_q_temp1))) - locals.var_q_temp2_dn9),)
    } else {
        (locals.var_q2d, locals.var_q2d_dn4, locals.var_q2d_dn6, locals.var_q2d_dn7, locals.var_q2d_dn8, locals.var_q2d_dn9,)
    }
};
        locals.var_q2d = assign20820_e21069;
        locals.var_q2d_dn4 = assign20820_e21069_d_n4;
        locals.var_q2d_dn6 = assign20820_e21069_d_n6;
        locals.var_q2d_dn7 = assign20820_e21069_d_n7;
        locals.var_q2d_dn8 = assign20820_e21069_d_n8;
        locals.var_q2d_dn9 = assign20820_e21069_d_n9;
        locals.var_q2d_rv = 0.0;

        let (assign20830_e21081, assign20830_e21081_d_n4, assign20830_e21081_d_n6, assign20830_e21081_d_n7, assign20830_e21081_d_n8, assign20830_e21081_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 == 0.0)) {
        let assign20830_e21079: f64 = (locals.var_k2 * locals.var_q2d);
        (assign20830_e21079, ((locals.var_k2_dn4 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn4)), ((locals.var_k2_dn6 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn6)), ((locals.var_k2_dn7 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn7)), ((locals.var_k2_dn8 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn8)), ((locals.var_k2_dn9 * locals.var_q2d) + (locals.var_k2 * locals.var_q2d_dn9)),)
    } else {
        (locals.var_k2q2d, locals.var_k2q2d_dn4, locals.var_k2q2d_dn6, locals.var_k2q2d_dn7, locals.var_k2q2d_dn8, locals.var_k2q2d_dn9,)
    }
};
        locals.var_k2q2d = assign20830_e21081;
        locals.var_k2q2d_dn4 = assign20830_e21081_d_n4;
        locals.var_k2q2d_dn6 = assign20830_e21081_d_n6;
        locals.var_k2q2d_dn7 = assign20830_e21081_d_n7;
        locals.var_k2q2d_dn8 = assign20830_e21081_d_n8;
        locals.var_k2q2d_dn9 = assign20830_e21081_d_n9;
        locals.var_k2q2d_rv = 0.0;

        let (assign20840_e21093, assign20840_e21093_d_n4, assign20840_e21093_d_n6, assign20840_e21093_d_n7, assign20840_e21093_d_n8, assign20840_e21093_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 == 0.0)) {
        let assign20840_e21091: f64 = (locals.var_k1q1d + locals.var_k2q2d);
        (assign20840_e21091, (locals.var_k1q1d_dn4 + locals.var_k2q2d_dn4), (locals.var_k1q1d_dn6 + locals.var_k2q2d_dn6), (locals.var_k1q1d_dn7 + locals.var_k2q2d_dn7), (locals.var_k1q1d_dn8 + locals.var_k2q2d_dn8), (locals.var_k1q1d_dn9 + locals.var_k2q2d_dn9),)
    } else {
        (locals.var_qid, locals.var_qid_dn4, locals.var_qid_dn6, locals.var_qid_dn7, locals.var_qid_dn8, locals.var_qid_dn9,)
    }
};
        locals.var_qid = assign20840_e21093;
        locals.var_qid_dn4 = assign20840_e21093_d_n4;
        locals.var_qid_dn6 = assign20840_e21093_d_n6;
        locals.var_qid_dn7 = assign20840_e21093_d_n7;
        locals.var_qid_dn8 = assign20840_e21093_d_n8;
        locals.var_qid_dn9 = assign20840_e21093_d_n9;
        locals.var_qid_rv = 0.0;

        let assign20850_e21096: f64 = if locals.var_qsqd > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard663 = assign20850_e21096;
        locals.var_guard663_rv = 0.0;

        let assign20860_e21099: f64 = (locals.var_q1d + locals.var_xdeff);
        let assign20860_e21101: f64 = (assign20860_e21099 - locals.var_xg1x);
        let assign20860_e21103: f64 = (assign20860_e21101 - locals.var_q_rac_qsq);
        let assign20860_e21105: f64 = if assign20860_e21103 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard664 = assign20860_e21105;
        locals.var_guard664_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_57(
        locals: &mut StampLocals,
    ) {
        let (assign20870_e21124, assign20870_e21124_d_n4, assign20870_e21124_d_n6, assign20870_e21124_d_n7, assign20870_e21124_d_n8, assign20870_e21124_d_n9,) = {
    if ((((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) {
        let assign20870_e21117: f64 = (locals.var_q1d + locals.var_xdeff);
        let assign20870_e21119: f64 = (assign20870_e21117 - locals.var_xg1x);
        let assign20870_e21121: f64 = (assign20870_e21119 - locals.var_q_rac_qsq);
        let assign20870_e21122: f64 = (assign20870_e21121).exp();
        (assign20870_e21122, (assign20870_e21122 * (((locals.var_q1d_dn4 + locals.var_xdeff_dn4) - locals.var_xg1x_dn4) - locals.var_q_rac_qsq_dn4)), (assign20870_e21122 * (((locals.var_q1d_dn6 + locals.var_xdeff_dn6) - locals.var_xg1x_dn6) - locals.var_q_rac_qsq_dn6)), (assign20870_e21122 * (((locals.var_q1d_dn7 + locals.var_xdeff_dn7) - locals.var_xg1x_dn7) - locals.var_q_rac_qsq_dn7)), (assign20870_e21122 * (((locals.var_q1d_dn8 + locals.var_xdeff_dn8) - locals.var_xg1x_dn8) - locals.var_q_rac_qsq_dn8)), (assign20870_e21122 * (((locals.var_q1d_dn9 + locals.var_xdeff_dn9) - locals.var_xg1x_dn9) - locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign20870_e21124;
        locals.var_q_temp3_dn4 = assign20870_e21124_d_n4;
        locals.var_q_temp3_dn6 = assign20870_e21124_d_n6;
        locals.var_q_temp3_dn7 = assign20870_e21124_d_n7;
        locals.var_q_temp3_dn8 = assign20870_e21124_d_n8;
        locals.var_q_temp3_dn9 = assign20870_e21124_d_n9;
        locals.var_q_temp3_rv = 0.0;

        let (assign20880_e21177, assign20880_e21177_d_n4, assign20880_e21177_d_n6, assign20880_e21177_d_n7, assign20880_e21177_d_n8, assign20880_e21177_d_n9,) = {
    if ((((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 == 0.0)) {
        let assign20880_e21139: f64 = (locals.var_q1d + locals.var_xdeff);
        let assign20880_e21141: f64 = (assign20880_e21139 - locals.var_xg1x);
        let assign20880_e21143: f64 = (assign20880_e21141 - locals.var_q_rac_qsq);
        let assign20880_e21145: f64 = (assign20880_e21143 - 80.0);
        let assign20880_e21150: f64 = (locals.var_q1d + locals.var_xdeff);
        let assign20880_e21152: f64 = (assign20880_e21150 - locals.var_xg1x);
        let assign20880_e21154: f64 = (assign20880_e21152 - locals.var_q_rac_qsq);
        let assign20880_e21156: f64 = (assign20880_e21154 - 80.0);
        let assign20880_e21157: f64 = (0.5 * assign20880_e21156);
        let assign20880_e21161: f64 = (locals.var_q1d + locals.var_xdeff);
        let assign20880_e21163: f64 = (assign20880_e21161 - locals.var_xg1x);
        let assign20880_e21165: f64 = (assign20880_e21163 - locals.var_q_rac_qsq);
        let assign20880_e21167: f64 = (assign20880_e21165 - 80.0);
        let assign20880_e21169: f64 = (assign20880_e21167 * 0.3333333333333);
        let assign20880_e21170: f64 = (1.0 + assign20880_e21169);
        let assign20880_e21171: f64 = (assign20880_e21157 * assign20880_e21170);
        let assign20880_e21172: f64 = (1.0 + assign20880_e21171);
        let assign20880_e21173: f64 = (assign20880_e21145 * assign20880_e21172);
        let assign20880_e21174: f64 = (1.0 + assign20880_e21173);
        let assign20880_e21175: f64 = (5.54062e34 * assign20880_e21174);
        (assign20880_e21175, (5.54062e34 * (((((locals.var_q1d_dn4 + locals.var_xdeff_dn4) - locals.var_xg1x_dn4) - locals.var_q_rac_qsq_dn4) * assign20880_e21172) + (assign20880_e21145 * (((0.5 * (((locals.var_q1d_dn4 + locals.var_xdeff_dn4) - locals.var_xg1x_dn4) - locals.var_q_rac_qsq_dn4)) * assign20880_e21170) + (assign20880_e21157 * ((((locals.var_q1d_dn4 + locals.var_xdeff_dn4) - locals.var_xg1x_dn4) - locals.var_q_rac_qsq_dn4) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d_dn6 + locals.var_xdeff_dn6) - locals.var_xg1x_dn6) - locals.var_q_rac_qsq_dn6) * assign20880_e21172) + (assign20880_e21145 * (((0.5 * (((locals.var_q1d_dn6 + locals.var_xdeff_dn6) - locals.var_xg1x_dn6) - locals.var_q_rac_qsq_dn6)) * assign20880_e21170) + (assign20880_e21157 * ((((locals.var_q1d_dn6 + locals.var_xdeff_dn6) - locals.var_xg1x_dn6) - locals.var_q_rac_qsq_dn6) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d_dn7 + locals.var_xdeff_dn7) - locals.var_xg1x_dn7) - locals.var_q_rac_qsq_dn7) * assign20880_e21172) + (assign20880_e21145 * (((0.5 * (((locals.var_q1d_dn7 + locals.var_xdeff_dn7) - locals.var_xg1x_dn7) - locals.var_q_rac_qsq_dn7)) * assign20880_e21170) + (assign20880_e21157 * ((((locals.var_q1d_dn7 + locals.var_xdeff_dn7) - locals.var_xg1x_dn7) - locals.var_q_rac_qsq_dn7) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d_dn8 + locals.var_xdeff_dn8) - locals.var_xg1x_dn8) - locals.var_q_rac_qsq_dn8) * assign20880_e21172) + (assign20880_e21145 * (((0.5 * (((locals.var_q1d_dn8 + locals.var_xdeff_dn8) - locals.var_xg1x_dn8) - locals.var_q_rac_qsq_dn8)) * assign20880_e21170) + (assign20880_e21157 * ((((locals.var_q1d_dn8 + locals.var_xdeff_dn8) - locals.var_xg1x_dn8) - locals.var_q_rac_qsq_dn8) * 0.3333333333333)))))), (5.54062e34 * (((((locals.var_q1d_dn9 + locals.var_xdeff_dn9) - locals.var_xg1x_dn9) - locals.var_q_rac_qsq_dn9) * assign20880_e21172) + (assign20880_e21145 * (((0.5 * (((locals.var_q1d_dn9 + locals.var_xdeff_dn9) - locals.var_xg1x_dn9) - locals.var_q_rac_qsq_dn9)) * assign20880_e21170) + (assign20880_e21157 * ((((locals.var_q1d_dn9 + locals.var_xdeff_dn9) - locals.var_xg1x_dn9) - locals.var_q_rac_qsq_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9,)
    }
};
        locals.var_q_temp3 = assign20880_e21177;
        locals.var_q_temp3_dn4 = assign20880_e21177_d_n4;
        locals.var_q_temp3_dn6 = assign20880_e21177_d_n6;
        locals.var_q_temp3_dn7 = assign20880_e21177_d_n7;
        locals.var_q_temp3_dn8 = assign20880_e21177_d_n8;
        locals.var_q_temp3_dn9 = assign20880_e21177_d_n9;
        locals.var_q_temp3_rv = 0.0;

        let (assign20890_e21189, assign20890_e21189_d_n4, assign20890_e21189_d_n6, assign20890_e21189_d_n7, assign20890_e21189_d_n8, assign20890_e21189_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) && (locals.var_guard663 != 0.0)) {
        let assign20890_e21187: f64 = (locals.var_q_temp3 / locals.var_a0);
        (assign20890_e21187, (((locals.var_q_temp3_dn4 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)), (((locals.var_q_temp3_dn6 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)), (((locals.var_q_temp3_dn7 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)), (((locals.var_q_temp3_dn8 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)), (((locals.var_q_temp3_dn9 * locals.var_a0) - (locals.var_q_temp3 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20890_e21189;
        locals.var_q_temp2_dn4 = assign20890_e21189_d_n4;
        locals.var_q_temp2_dn6 = assign20890_e21189_d_n6;
        locals.var_q_temp2_dn7 = assign20890_e21189_d_n7;
        locals.var_q_temp2_dn8 = assign20890_e21189_d_n8;
        locals.var_q_temp2_dn9 = assign20890_e21189_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign20900_e21211, assign20900_e21211_d_n4, assign20900_e21211_d_n6, assign20900_e21211_d_n7, assign20900_e21211_d_n8, assign20900_e21211_d_n9,) = {
    if (((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) && (locals.var_guard663 != 0.0)) {
        let assign20900_e21199: f64 = (4.0 * locals.var_qsqd);
        let assign20900_e21201: f64 = (assign20900_e21199 * locals.var_q_temp2);
        let assign20900_e21206: f64 = (2.0 - locals.var_q_invexpq);
        let assign20900_e21207: f64 = (locals.var_q_invexpq * assign20900_e21206);
        let assign20900_e21208: f64 = (1.0 - assign20900_e21207);
        let assign20900_e21209: f64 = (assign20900_e21201 / assign20900_e21208);
        (assign20900_e21209, ((((((4.0 * locals.var_qsqd_dn4) * locals.var_q_temp2) + (assign20900_e21199 * locals.var_q_temp2_dn4)) * assign20900_e21208) - (assign20900_e21201 * (-((locals.var_q_invexpq_dn4 * assign20900_e21206) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign20900_e21208 * assign20900_e21208)), ((((((4.0 * locals.var_qsqd_dn6) * locals.var_q_temp2) + (assign20900_e21199 * locals.var_q_temp2_dn6)) * assign20900_e21208) - (assign20900_e21201 * (-((locals.var_q_invexpq_dn6 * assign20900_e21206) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign20900_e21208 * assign20900_e21208)), ((((((4.0 * locals.var_qsqd_dn7) * locals.var_q_temp2) + (assign20900_e21199 * locals.var_q_temp2_dn7)) * assign20900_e21208) - (assign20900_e21201 * (-((locals.var_q_invexpq_dn7 * assign20900_e21206) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign20900_e21208 * assign20900_e21208)), ((((((4.0 * locals.var_qsqd_dn8) * locals.var_q_temp2) + (assign20900_e21199 * locals.var_q_temp2_dn8)) * assign20900_e21208) - (assign20900_e21201 * (-((locals.var_q_invexpq_dn8 * assign20900_e21206) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign20900_e21208 * assign20900_e21208)), ((((((4.0 * locals.var_qsqd_dn9) * locals.var_q_temp2) + (assign20900_e21199 * locals.var_q_temp2_dn9)) * assign20900_e21208) - (assign20900_e21201 * (-((locals.var_q_invexpq_dn9 * assign20900_e21206) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign20900_e21208 * assign20900_e21208)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign20900_e21211;
        locals.var_q_temp1_dn4 = assign20900_e21211_d_n4;
        locals.var_q_temp1_dn6 = assign20900_e21211_d_n6;
        locals.var_q_temp1_dn7 = assign20900_e21211_d_n7;
        locals.var_q_temp1_dn8 = assign20900_e21211_d_n8;
        locals.var_q_temp1_dn9 = assign20900_e21211_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let assign20910_e21214: f64 = (-0.005);
        let assign20910_e21215: f64 = if locals.var_qsqd < assign20910_e21214 { 1.0 } else { 0.0 };
        locals.var_guard665 = assign20910_e21215;
        locals.var_guard665_rv = 0.0;

        let (assign20920_e21231, assign20920_e21231_d_n4, assign20920_e21231_d_n6, assign20920_e21231_d_n7, assign20920_e21231_d_n8, assign20920_e21231_d_n9,) = {
    if ((((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) && (locals.var_guard663 == 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign20920_e21228: f64 = (0.5 * locals.var_q_rac_qsq);
        let assign20920_e21229: f64 = (assign20920_e21228).sin();
        (assign20920_e21229, ((assign20920_e21228).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign20920_e21228).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign20920_e21228).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign20920_e21228).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign20920_e21228).cos() * (0.5 * locals.var_q_rac_qsq_dn9)),)
    } else {
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9,)
    }
};
        locals.var_q_temp2 = assign20920_e21231;
        locals.var_q_temp2_dn4 = assign20920_e21231_d_n4;
        locals.var_q_temp2_dn6 = assign20920_e21231_d_n6;
        locals.var_q_temp2_dn7 = assign20920_e21231_d_n7;
        locals.var_q_temp2_dn8 = assign20920_e21231_d_n8;
        locals.var_q_temp2_dn9 = assign20920_e21231_d_n9;
        locals.var_q_temp2_rv = 0.0;

        let (assign20930_e21251, assign20930_e21251_d_n4, assign20930_e21251_d_n6, assign20930_e21251_d_n7, assign20930_e21251_d_n8, assign20930_e21251_d_n9,) = {
    if ((((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) && (locals.var_guard663 == 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign20930_e21243: f64 = (-locals.var_qsqd);
        let assign20930_e21246: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
        let assign20930_e21247: f64 = (assign20930_e21243 / assign20930_e21246);
        let assign20930_e21249: f64 = (assign20930_e21247 / locals.var_aexp1d);
        (assign20930_e21249, (((((((-locals.var_qsqd_dn4) * assign20930_e21246) - (assign20930_e21243 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign20930_e21246 * assign20930_e21246)) * locals.var_aexp1d) - (assign20930_e21247 * locals.var_aexp1d_dn4)) / (locals.var_aexp1d * locals.var_aexp1d)), (((((((-locals.var_qsqd_dn6) * assign20930_e21246) - (assign20930_e21243 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign20930_e21246 * assign20930_e21246)) * locals.var_aexp1d) - (assign20930_e21247 * locals.var_aexp1d_dn6)) / (locals.var_aexp1d * locals.var_aexp1d)), (((((((-locals.var_qsqd_dn7) * assign20930_e21246) - (assign20930_e21243 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign20930_e21246 * assign20930_e21246)) * locals.var_aexp1d) - (assign20930_e21247 * locals.var_aexp1d_dn7)) / (locals.var_aexp1d * locals.var_aexp1d)), (((((((-locals.var_qsqd_dn8) * assign20930_e21246) - (assign20930_e21243 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign20930_e21246 * assign20930_e21246)) * locals.var_aexp1d) - (assign20930_e21247 * locals.var_aexp1d_dn8)) / (locals.var_aexp1d * locals.var_aexp1d)), (((((((-locals.var_qsqd_dn9) * assign20930_e21246) - (assign20930_e21243 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign20930_e21246 * assign20930_e21246)) * locals.var_aexp1d) - (assign20930_e21247 * locals.var_aexp1d_dn9)) / (locals.var_aexp1d * locals.var_aexp1d)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign20930_e21251;
        locals.var_q_temp1_dn4 = assign20930_e21251_d_n4;
        locals.var_q_temp1_dn6 = assign20930_e21251_d_n6;
        locals.var_q_temp1_dn7 = assign20930_e21251_d_n7;
        locals.var_q_temp1_dn8 = assign20930_e21251_d_n8;
        locals.var_q_temp1_dn9 = assign20930_e21251_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign20940_e21283, assign20940_e21283_d_n4, assign20940_e21283_d_n6, assign20940_e21283_d_n7, assign20940_e21283_d_n8, assign20940_e21283_d_n9,) = {
    if ((((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) && (locals.var_guard663 == 0.0)) && (locals.var_guard665 == 0.0)) {
        let assign20940_e21266: f64 = (locals.var_qsqd * 0.3333333333333);
        let assign20940_e21270: f64 = (0.05 * locals.var_qsqd);
        let assign20940_e21274: f64 = (0.0396825396825397 * locals.var_qsqd);
        let assign20940_e21275: f64 = (1.0 - assign20940_e21274);
        let assign20940_e21276: f64 = (assign20940_e21270 * assign20940_e21275);
        let assign20940_e21277: f64 = (1.0 - assign20940_e21276);
        let assign20940_e21278: f64 = (assign20940_e21266 * assign20940_e21277);
        let assign20940_e21279: f64 = (4.0 - assign20940_e21278);
        let assign20940_e21281: f64 = (assign20940_e21279 / locals.var_aexp1d);
        (assign20940_e21281, ((((-(((locals.var_qsqd_dn4 * 0.3333333333333) * assign20940_e21277) + (assign20940_e21266 * (-(((0.05 * locals.var_qsqd_dn4) * assign20940_e21275) + (assign20940_e21270 * (-(0.0396825396825397 * locals.var_qsqd_dn4)))))))) * locals.var_aexp1d) - (assign20940_e21279 * locals.var_aexp1d_dn4)) / (locals.var_aexp1d * locals.var_aexp1d)), ((((-(((locals.var_qsqd_dn6 * 0.3333333333333) * assign20940_e21277) + (assign20940_e21266 * (-(((0.05 * locals.var_qsqd_dn6) * assign20940_e21275) + (assign20940_e21270 * (-(0.0396825396825397 * locals.var_qsqd_dn6)))))))) * locals.var_aexp1d) - (assign20940_e21279 * locals.var_aexp1d_dn6)) / (locals.var_aexp1d * locals.var_aexp1d)), ((((-(((locals.var_qsqd_dn7 * 0.3333333333333) * assign20940_e21277) + (assign20940_e21266 * (-(((0.05 * locals.var_qsqd_dn7) * assign20940_e21275) + (assign20940_e21270 * (-(0.0396825396825397 * locals.var_qsqd_dn7)))))))) * locals.var_aexp1d) - (assign20940_e21279 * locals.var_aexp1d_dn7)) / (locals.var_aexp1d * locals.var_aexp1d)), ((((-(((locals.var_qsqd_dn8 * 0.3333333333333) * assign20940_e21277) + (assign20940_e21266 * (-(((0.05 * locals.var_qsqd_dn8) * assign20940_e21275) + (assign20940_e21270 * (-(0.0396825396825397 * locals.var_qsqd_dn8)))))))) * locals.var_aexp1d) - (assign20940_e21279 * locals.var_aexp1d_dn8)) / (locals.var_aexp1d * locals.var_aexp1d)), ((((-(((locals.var_qsqd_dn9 * 0.3333333333333) * assign20940_e21277) + (assign20940_e21266 * (-(((0.05 * locals.var_qsqd_dn9) * assign20940_e21275) + (assign20940_e21270 * (-(0.0396825396825397 * locals.var_qsqd_dn9)))))))) * locals.var_aexp1d) - (assign20940_e21279 * locals.var_aexp1d_dn9)) / (locals.var_aexp1d * locals.var_aexp1d)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign20940_e21283;
        locals.var_q_temp1_dn4 = assign20940_e21283_d_n4;
        locals.var_q_temp1_dn6 = assign20940_e21283_d_n6;
        locals.var_q_temp1_dn7 = assign20940_e21283_d_n7;
        locals.var_q_temp1_dn8 = assign20940_e21283_d_n8;
        locals.var_q_temp1_dn9 = assign20940_e21283_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign20950_e21299, assign20950_e21299_d_n4, assign20950_e21299_d_n6, assign20950_e21299_d_n7, assign20950_e21299_d_n8, assign20950_e21299_d_n9,) = {
    if ((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) {
        let assign20950_e21291: f64 = (locals.var_k1q1d - locals.var_q_qcoth);
        let assign20950_e21294: f64 = (1.0 - locals.var_q_temp1);
        let assign20950_e21295: f64 = (assign20950_e21291 / assign20950_e21294);
        let assign20950_e21297: f64 = (assign20950_e21295 + 1e-80);
        (assign20950_e21297, ((((locals.var_k1q1d_dn4 - locals.var_q_qcoth_dn4) * assign20950_e21294) - (assign20950_e21291 * (-locals.var_q_temp1_dn4))) / (assign20950_e21294 * assign20950_e21294)), ((((locals.var_k1q1d_dn6 - locals.var_q_qcoth_dn6) * assign20950_e21294) - (assign20950_e21291 * (-locals.var_q_temp1_dn6))) / (assign20950_e21294 * assign20950_e21294)), ((((locals.var_k1q1d_dn7 - locals.var_q_qcoth_dn7) * assign20950_e21294) - (assign20950_e21291 * (-locals.var_q_temp1_dn7))) / (assign20950_e21294 * assign20950_e21294)), ((((locals.var_k1q1d_dn8 - locals.var_q_qcoth_dn8) * assign20950_e21294) - (assign20950_e21291 * (-locals.var_q_temp1_dn8))) / (assign20950_e21294 * assign20950_e21294)), ((((locals.var_k1q1d_dn9 - locals.var_q_qcoth_dn9) * assign20950_e21294) - (assign20950_e21291 * (-locals.var_q_temp1_dn9))) / (assign20950_e21294 * assign20950_e21294)),)
    } else {
        (locals.var_qid, locals.var_qid_dn4, locals.var_qid_dn6, locals.var_qid_dn7, locals.var_qid_dn8, locals.var_qid_dn9,)
    }
};
        locals.var_qid = assign20950_e21299;
        locals.var_qid_dn4 = assign20950_e21299_d_n4;
        locals.var_qid_dn6 = assign20950_e21299_d_n6;
        locals.var_qid_dn7 = assign20950_e21299_d_n7;
        locals.var_qid_dn8 = assign20950_e21299_d_n8;
        locals.var_qid_dn9 = assign20950_e21299_d_n9;
        locals.var_qid_rv = 0.0;

        let (assign20960_e21309, assign20960_e21309_d_n4, assign20960_e21309_d_n6, assign20960_e21309_d_n7, assign20960_e21309_d_n8, assign20960_e21309_d_n9,) = {
    if ((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) {
        let assign20960_e21307: f64 = (locals.var_qid - locals.var_k1q1d);
        (assign20960_e21307, (locals.var_qid_dn4 - locals.var_k1q1d_dn4), (locals.var_qid_dn6 - locals.var_k1q1d_dn6), (locals.var_qid_dn7 - locals.var_k1q1d_dn7), (locals.var_qid_dn8 - locals.var_k1q1d_dn8), (locals.var_qid_dn9 - locals.var_k1q1d_dn9),)
    } else {
        (locals.var_k2q2d, locals.var_k2q2d_dn4, locals.var_k2q2d_dn6, locals.var_k2q2d_dn7, locals.var_k2q2d_dn8, locals.var_k2q2d_dn9,)
    }
};
        locals.var_k2q2d = assign20960_e21309;
        locals.var_k2q2d_dn4 = assign20960_e21309_d_n4;
        locals.var_k2q2d_dn6 = assign20960_e21309_d_n6;
        locals.var_k2q2d_dn7 = assign20960_e21309_d_n7;
        locals.var_k2q2d_dn8 = assign20960_e21309_d_n8;
        locals.var_k2q2d_dn9 = assign20960_e21309_d_n9;
        locals.var_k2q2d_rv = 0.0;

        let (assign20970_e21319, assign20970_e21319_d_n4, assign20970_e21319_d_n6, assign20970_e21319_d_n7, assign20970_e21319_d_n8, assign20970_e21319_d_n9,) = {
    if ((locals.var_guard656 == 0.0) && (locals.var_guard659 == 0.0)) {
        let assign20970_e21317: f64 = (locals.var_k2q2d / locals.var_k2);
        (assign20970_e21317, (((locals.var_k2q2d_dn4 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn4)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn6 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn6)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn7 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn7)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn8 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn8)) / (locals.var_k2 * locals.var_k2)), (((locals.var_k2q2d_dn9 * locals.var_k2) - (locals.var_k2q2d * locals.var_k2_dn9)) / (locals.var_k2 * locals.var_k2)),)
    } else {
        (locals.var_q2d, locals.var_q2d_dn4, locals.var_q2d_dn6, locals.var_q2d_dn7, locals.var_q2d_dn8, locals.var_q2d_dn9,)
    }
};
        locals.var_q2d = assign20970_e21319;
        locals.var_q2d_dn4 = assign20970_e21319_d_n4;
        locals.var_q2d_dn6 = assign20970_e21319_d_n6;
        locals.var_q2d_dn7 = assign20970_e21319_d_n7;
        locals.var_q2d_dn8 = assign20970_e21319_d_n8;
        locals.var_q2d_dn9 = assign20970_e21319_d_n9;
        locals.var_q2d_rv = 0.0;

        let assign20980_e21322: f64 = (locals.var_xg2x - locals.var_q2d);
        let assign20980_e21324: f64 = (assign20980_e21322 - locals.var_xdeff);
        let assign20980_e21326: f64 = if assign20980_e21324 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard666 = assign20980_e21326;
        locals.var_guard666_rv = 0.0;

        let (assign20990_e21335, assign20990_e21335_d_n4, assign20990_e21335_d_n6, assign20990_e21335_d_n7, assign20990_e21335_d_n8, assign20990_e21335_d_n9,) = {
    if (locals.var_guard666 != 0.0) {
        let assign20990_e21330: f64 = (locals.var_xg2x - locals.var_q2d);
        let assign20990_e21332: f64 = (assign20990_e21330 - locals.var_xdeff);
        let assign20990_e21333: f64 = (assign20990_e21332).exp();
        (assign20990_e21333, (assign20990_e21333 * ((locals.var_xg2x_dn4 - locals.var_q2d_dn4) - locals.var_xdeff_dn4)), (assign20990_e21333 * ((locals.var_xg2x_dn6 - locals.var_q2d_dn6) - locals.var_xdeff_dn6)), (assign20990_e21333 * ((locals.var_xg2x_dn7 - locals.var_q2d_dn7) - locals.var_xdeff_dn7)), (assign20990_e21333 * ((locals.var_xg2x_dn8 - locals.var_q2d_dn8) - locals.var_xdeff_dn8)), (assign20990_e21333 * ((locals.var_xg2x_dn9 - locals.var_q2d_dn9) - locals.var_xdeff_dn9)),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign20990_e21335;
        locals.var_q_temp1_dn4 = assign20990_e21335_d_n4;
        locals.var_q_temp1_dn6 = assign20990_e21335_d_n6;
        locals.var_q_temp1_dn7 = assign20990_e21335_d_n7;
        locals.var_q_temp1_dn8 = assign20990_e21335_d_n8;
        locals.var_q_temp1_dn9 = assign20990_e21335_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let (assign21000_e21374, assign21000_e21374_d_n4, assign21000_e21374_d_n6, assign21000_e21374_d_n7, assign21000_e21374_d_n8, assign21000_e21374_d_n9,) = {
    if (locals.var_guard666 == 0.0) {
        let assign21000_e21342: f64 = (locals.var_xg2x - locals.var_q2d);
        let assign21000_e21344: f64 = (assign21000_e21342 - locals.var_xdeff);
        let assign21000_e21346: f64 = (assign21000_e21344 - 80.0);
        let assign21000_e21351: f64 = (locals.var_xg2x - locals.var_q2d);
        let assign21000_e21353: f64 = (assign21000_e21351 - locals.var_xdeff);
        let assign21000_e21355: f64 = (assign21000_e21353 - 80.0);
        let assign21000_e21356: f64 = (0.5 * assign21000_e21355);
        let assign21000_e21360: f64 = (locals.var_xg2x - locals.var_q2d);
        let assign21000_e21362: f64 = (assign21000_e21360 - locals.var_xdeff);
        let assign21000_e21364: f64 = (assign21000_e21362 - 80.0);
        let assign21000_e21366: f64 = (assign21000_e21364 * 0.3333333333333);
        let assign21000_e21367: f64 = (1.0 + assign21000_e21366);
        let assign21000_e21368: f64 = (assign21000_e21356 * assign21000_e21367);
        let assign21000_e21369: f64 = (1.0 + assign21000_e21368);
        let assign21000_e21370: f64 = (assign21000_e21346 * assign21000_e21369);
        let assign21000_e21371: f64 = (1.0 + assign21000_e21370);
        let assign21000_e21372: f64 = (5.54062e34 * assign21000_e21371);
        (assign21000_e21372, (5.54062e34 * ((((locals.var_xg2x_dn4 - locals.var_q2d_dn4) - locals.var_xdeff_dn4) * assign21000_e21369) + (assign21000_e21346 * (((0.5 * ((locals.var_xg2x_dn4 - locals.var_q2d_dn4) - locals.var_xdeff_dn4)) * assign21000_e21367) + (assign21000_e21356 * (((locals.var_xg2x_dn4 - locals.var_q2d_dn4) - locals.var_xdeff_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x_dn6 - locals.var_q2d_dn6) - locals.var_xdeff_dn6) * assign21000_e21369) + (assign21000_e21346 * (((0.5 * ((locals.var_xg2x_dn6 - locals.var_q2d_dn6) - locals.var_xdeff_dn6)) * assign21000_e21367) + (assign21000_e21356 * (((locals.var_xg2x_dn6 - locals.var_q2d_dn6) - locals.var_xdeff_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x_dn7 - locals.var_q2d_dn7) - locals.var_xdeff_dn7) * assign21000_e21369) + (assign21000_e21346 * (((0.5 * ((locals.var_xg2x_dn7 - locals.var_q2d_dn7) - locals.var_xdeff_dn7)) * assign21000_e21367) + (assign21000_e21356 * (((locals.var_xg2x_dn7 - locals.var_q2d_dn7) - locals.var_xdeff_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x_dn8 - locals.var_q2d_dn8) - locals.var_xdeff_dn8) * assign21000_e21369) + (assign21000_e21346 * (((0.5 * ((locals.var_xg2x_dn8 - locals.var_q2d_dn8) - locals.var_xdeff_dn8)) * assign21000_e21367) + (assign21000_e21356 * (((locals.var_xg2x_dn8 - locals.var_q2d_dn8) - locals.var_xdeff_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_xg2x_dn9 - locals.var_q2d_dn9) - locals.var_xdeff_dn9) * assign21000_e21369) + (assign21000_e21346 * (((0.5 * ((locals.var_xg2x_dn9 - locals.var_q2d_dn9) - locals.var_xdeff_dn9)) * assign21000_e21367) + (assign21000_e21356 * (((locals.var_xg2x_dn9 - locals.var_q2d_dn9) - locals.var_xdeff_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9,)
    }
};
        locals.var_q_temp1 = assign21000_e21374;
        locals.var_q_temp1_dn4 = assign21000_e21374_d_n4;
        locals.var_q_temp1_dn6 = assign21000_e21374_d_n6;
        locals.var_q_temp1_dn7 = assign21000_e21374_d_n7;
        locals.var_q_temp1_dn8 = assign21000_e21374_d_n8;
        locals.var_q_temp1_dn9 = assign21000_e21374_d_n9;
        locals.var_q_temp1_rv = 0.0;

        let assign21010_e21377: f64 = (locals.var_a0 * locals.var_q_temp1);
        locals.var_aexp2d = assign21010_e21377;
        locals.var_aexp2d_dn4 = ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4));
        locals.var_aexp2d_dn6 = ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6));
        locals.var_aexp2d_dn7 = ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7));
        locals.var_aexp2d_dn8 = ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8));
        locals.var_aexp2d_dn9 = ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9));
        locals.var_aexp2d_rv = 0.0;

        locals.var_a1d = 0.0;
        locals.var_a1d_dn4 = 0.0;
        locals.var_a1d_dn6 = 0.0;
        locals.var_a1d_dn7 = 0.0;
        locals.var_a1d_dn8 = 0.0;
        locals.var_a1d_dn9 = 0.0;
        locals.var_a1d_rv = 0.0;

        locals.var_a2d = 0.0;
        locals.var_a2d_dn4 = 0.0;
        locals.var_a2d_dn6 = 0.0;
        locals.var_a2d_dn7 = 0.0;
        locals.var_a2d_dn8 = 0.0;
        locals.var_a2d_dn9 = 0.0;
        locals.var_a2d_rv = 0.0;

        locals.var_b1d = 0.0;
        locals.var_b1d_dn4 = 0.0;
        locals.var_b1d_dn6 = 0.0;
        locals.var_b1d_dn7 = 0.0;
        locals.var_b1d_dn8 = 0.0;
        locals.var_b1d_dn9 = 0.0;
        locals.var_b1d_rv = 0.0;

        locals.var_b2d = 0.0;
        locals.var_b2d_dn4 = 0.0;
        locals.var_b2d_dn6 = 0.0;
        locals.var_b2d_dn7 = 0.0;
        locals.var_b2d_dn8 = 0.0;
        locals.var_b2d_dn9 = 0.0;
        locals.var_b2d_rv = 0.0;

        locals.var_sumd = 0.0;
        locals.var_sumd_dn4 = 0.0;
        locals.var_sumd_dn6 = 0.0;
        locals.var_sumd_dn7 = 0.0;
        locals.var_sumd_dn8 = 0.0;
        locals.var_sumd_dn9 = 0.0;
        locals.var_sumd_rv = 0.0;

        locals.var_dqsqd_dxn_qi = 0.0;
        locals.var_dqsqd_dxn_qi_dn4 = 0.0;
        locals.var_dqsqd_dxn_qi_dn6 = 0.0;
        locals.var_dqsqd_dxn_qi_dn7 = 0.0;
        locals.var_dqsqd_dxn_qi_dn8 = 0.0;
        locals.var_dqsqd_dxn_qi_dn9 = 0.0;
        locals.var_dqsqd_dxn_qi_rv = 0.0;

        let assign21080_e21386: f64 = if locals.var_qis > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard667 = assign21080_e21386;
        locals.var_guard667_rv = 0.0;

        let (assign21090_e21392, assign21090_e21392_d_n4, assign21090_e21392_d_n6, assign21090_e21392_d_n7, assign21090_e21392_d_n8, assign21090_e21392_d_n9,) = {
    if (locals.var_guard667 != 0.0) {
        let assign21090_e21390: f64 = (locals.var_aexp1d * locals.var_inv_k1);
        (assign21090_e21390, ((locals.var_aexp1d_dn4 * locals.var_inv_k1) + (locals.var_aexp1d * locals.var_inv_k1_dn4)), ((locals.var_aexp1d_dn6 * locals.var_inv_k1) + (locals.var_aexp1d * locals.var_inv_k1_dn6)), ((locals.var_aexp1d_dn7 * locals.var_inv_k1) + (locals.var_aexp1d * locals.var_inv_k1_dn7)), ((locals.var_aexp1d_dn8 * locals.var_inv_k1) + (locals.var_aexp1d * locals.var_inv_k1_dn8)), ((locals.var_aexp1d_dn9 * locals.var_inv_k1) + (locals.var_aexp1d * locals.var_inv_k1_dn9)),)
    } else {
        (locals.var_b1d, locals.var_b1d_dn4, locals.var_b1d_dn6, locals.var_b1d_dn7, locals.var_b1d_dn8, locals.var_b1d_dn9,)
    }
};
        locals.var_b1d = assign21090_e21392;
        locals.var_b1d_dn4 = assign21090_e21392_d_n4;
        locals.var_b1d_dn6 = assign21090_e21392_d_n6;
        locals.var_b1d_dn7 = assign21090_e21392_d_n7;
        locals.var_b1d_dn8 = assign21090_e21392_d_n8;
        locals.var_b1d_dn9 = assign21090_e21392_d_n9;
        locals.var_b1d_rv = 0.0;

        let (assign21100_e21398, assign21100_e21398_d_n4, assign21100_e21398_d_n6, assign21100_e21398_d_n7, assign21100_e21398_d_n8, assign21100_e21398_d_n9,) = {
    if (locals.var_guard667 != 0.0) {
        let assign21100_e21396: f64 = (locals.var_aexp2d * locals.var_inv_k2);
        (assign21100_e21396, ((locals.var_aexp2d_dn4 * locals.var_inv_k2) + (locals.var_aexp2d * locals.var_inv_k2_dn4)), ((locals.var_aexp2d_dn6 * locals.var_inv_k2) + (locals.var_aexp2d * locals.var_inv_k2_dn6)), ((locals.var_aexp2d_dn7 * locals.var_inv_k2) + (locals.var_aexp2d * locals.var_inv_k2_dn7)), ((locals.var_aexp2d_dn8 * locals.var_inv_k2) + (locals.var_aexp2d * locals.var_inv_k2_dn8)), ((locals.var_aexp2d_dn9 * locals.var_inv_k2) + (locals.var_aexp2d * locals.var_inv_k2_dn9)),)
    } else {
        (locals.var_b2d, locals.var_b2d_dn4, locals.var_b2d_dn6, locals.var_b2d_dn7, locals.var_b2d_dn8, locals.var_b2d_dn9,)
    }
};
        locals.var_b2d = assign21100_e21398;
        locals.var_b2d_dn4 = assign21100_e21398_d_n4;
        locals.var_b2d_dn6 = assign21100_e21398_d_n6;
        locals.var_b2d_dn7 = assign21100_e21398_d_n7;
        locals.var_b2d_dn8 = assign21100_e21398_d_n8;
        locals.var_b2d_dn9 = assign21100_e21398_d_n9;
        locals.var_b2d_rv = 0.0;

        let (assign21110_e21406, assign21110_e21406_d_n4, assign21110_e21406_d_n6, assign21110_e21406_d_n7, assign21110_e21406_d_n8, assign21110_e21406_d_n9,) = {
    if (locals.var_guard667 != 0.0) {
        let assign21110_e21403: f64 = (2.0 * locals.var_k1q1d);
        let assign21110_e21404: f64 = (locals.var_b1d + assign21110_e21403);
        (assign21110_e21404, (locals.var_b1d_dn4 + (2.0 * locals.var_k1q1d_dn4)), (locals.var_b1d_dn6 + (2.0 * locals.var_k1q1d_dn6)), (locals.var_b1d_dn7 + (2.0 * locals.var_k1q1d_dn7)), (locals.var_b1d_dn8 + (2.0 * locals.var_k1q1d_dn8)), (locals.var_b1d_dn9 + (2.0 * locals.var_k1q1d_dn9)),)
    } else {
        (locals.var_a1d, locals.var_a1d_dn4, locals.var_a1d_dn6, locals.var_a1d_dn7, locals.var_a1d_dn8, locals.var_a1d_dn9,)
    }
};
        locals.var_a1d = assign21110_e21406;
        locals.var_a1d_dn4 = assign21110_e21406_d_n4;
        locals.var_a1d_dn6 = assign21110_e21406_d_n6;
        locals.var_a1d_dn7 = assign21110_e21406_d_n7;
        locals.var_a1d_dn8 = assign21110_e21406_d_n8;
        locals.var_a1d_dn9 = assign21110_e21406_d_n9;
        locals.var_a1d_rv = 0.0;

        let (assign21120_e21414, assign21120_e21414_d_n4, assign21120_e21414_d_n6, assign21120_e21414_d_n7, assign21120_e21414_d_n8, assign21120_e21414_d_n9,) = {
    if (locals.var_guard667 != 0.0) {
        let assign21120_e21411: f64 = (2.0 * locals.var_k2q2d);
        let assign21120_e21412: f64 = (locals.var_b2d + assign21120_e21411);
        (assign21120_e21412, (locals.var_b2d_dn4 + (2.0 * locals.var_k2q2d_dn4)), (locals.var_b2d_dn6 + (2.0 * locals.var_k2q2d_dn6)), (locals.var_b2d_dn7 + (2.0 * locals.var_k2q2d_dn7)), (locals.var_b2d_dn8 + (2.0 * locals.var_k2q2d_dn8)), (locals.var_b2d_dn9 + (2.0 * locals.var_k2q2d_dn9)),)
    } else {
        (locals.var_a2d, locals.var_a2d_dn4, locals.var_a2d_dn6, locals.var_a2d_dn7, locals.var_a2d_dn8, locals.var_a2d_dn9,)
    }
};
        locals.var_a2d = assign21120_e21414;
        locals.var_a2d_dn4 = assign21120_e21414_d_n4;
        locals.var_a2d_dn6 = assign21120_e21414_d_n6;
        locals.var_a2d_dn7 = assign21120_e21414_d_n7;
        locals.var_a2d_dn8 = assign21120_e21414_d_n8;
        locals.var_a2d_dn9 = assign21120_e21414_d_n9;
        locals.var_a2d_rv = 0.0;

        let (assign21130_e21424, assign21130_e21424_d_n4, assign21130_e21424_d_n6, assign21130_e21424_d_n7, assign21130_e21424_d_n8, assign21130_e21424_d_n9,) = {
    if (locals.var_guard667 != 0.0) {
        let assign21130_e21418: f64 = (2.0 * locals.var_qid);
        let assign21130_e21420: f64 = (assign21130_e21418 + locals.var_b1d);
        let assign21130_e21422: f64 = (assign21130_e21420 + locals.var_b2d);
        (assign21130_e21422, (((2.0 * locals.var_qid_dn4) + locals.var_b1d_dn4) + locals.var_b2d_dn4), (((2.0 * locals.var_qid_dn6) + locals.var_b1d_dn6) + locals.var_b2d_dn6), (((2.0 * locals.var_qid_dn7) + locals.var_b1d_dn7) + locals.var_b2d_dn7), (((2.0 * locals.var_qid_dn8) + locals.var_b1d_dn8) + locals.var_b2d_dn8), (((2.0 * locals.var_qid_dn9) + locals.var_b1d_dn9) + locals.var_b2d_dn9),)
    } else {
        (locals.var_sumd, locals.var_sumd_dn4, locals.var_sumd_dn6, locals.var_sumd_dn7, locals.var_sumd_dn8, locals.var_sumd_dn9,)
    }
};
        locals.var_sumd = assign21130_e21424;
        locals.var_sumd_dn4 = assign21130_e21424_d_n4;
        locals.var_sumd_dn6 = assign21130_e21424_d_n6;
        locals.var_sumd_dn7 = assign21130_e21424_d_n7;
        locals.var_sumd_dn8 = assign21130_e21424_d_n8;
        locals.var_sumd_dn9 = assign21130_e21424_d_n9;
        locals.var_sumd_rv = 0.0;

        let assign21140_e21426: f64 = (locals.var_qsqd).abs();
        let assign21140_e21428: f64 = if assign21140_e21426 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard668 = assign21140_e21428;
        locals.var_guard668_rv = 0.0;

        let (assign21150_e21452, assign21150_e21452_d_n4, assign21150_e21452_d_n6, assign21150_e21452_d_n7, assign21150_e21452_d_n8, assign21150_e21452_d_n9,) = {
    if ((locals.var_guard667 != 0.0) && (locals.var_guard668 != 0.0)) {
        let assign21150_e21434: f64 = (locals.var_a1d * locals.var_a2d);
        let assign21150_e21438: f64 = (locals.var_q1d + 2.0);
        let assign21150_e21439: f64 = (2.0 * assign21150_e21438);
        let assign21150_e21441: f64 = (assign21150_e21439 * locals.var_a2d);
        let assign21150_e21442: f64 = (assign21150_e21434 + assign21150_e21441);
        let assign21150_e21446: f64 = (locals.var_q2d + 2.0);
        let assign21150_e21447: f64 = (2.0 * assign21150_e21446);
        let assign21150_e21449: f64 = (assign21150_e21447 * locals.var_a1d);
        let assign21150_e21450: f64 = (assign21150_e21442 + assign21150_e21449);
        (assign21150_e21450, ((((locals.var_a1d_dn4 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn4)) + (((2.0 * locals.var_q1d_dn4) * locals.var_a2d) + (assign21150_e21439 * locals.var_a2d_dn4))) + (((2.0 * locals.var_q2d_dn4) * locals.var_a1d) + (assign21150_e21447 * locals.var_a1d_dn4))), ((((locals.var_a1d_dn6 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn6)) + (((2.0 * locals.var_q1d_dn6) * locals.var_a2d) + (assign21150_e21439 * locals.var_a2d_dn6))) + (((2.0 * locals.var_q2d_dn6) * locals.var_a1d) + (assign21150_e21447 * locals.var_a1d_dn6))), ((((locals.var_a1d_dn7 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn7)) + (((2.0 * locals.var_q1d_dn7) * locals.var_a2d) + (assign21150_e21439 * locals.var_a2d_dn7))) + (((2.0 * locals.var_q2d_dn7) * locals.var_a1d) + (assign21150_e21447 * locals.var_a1d_dn7))), ((((locals.var_a1d_dn8 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn8)) + (((2.0 * locals.var_q1d_dn8) * locals.var_a2d) + (assign21150_e21439 * locals.var_a2d_dn8))) + (((2.0 * locals.var_q2d_dn8) * locals.var_a1d) + (assign21150_e21447 * locals.var_a1d_dn8))), ((((locals.var_a1d_dn9 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn9)) + (((2.0 * locals.var_q1d_dn9) * locals.var_a2d) + (assign21150_e21439 * locals.var_a2d_dn9))) + (((2.0 * locals.var_q2d_dn9) * locals.var_a1d) + (assign21150_e21447 * locals.var_a1d_dn9))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign21150_e21452;
        locals.var_temp1_dn4 = assign21150_e21452_d_n4;
        locals.var_temp1_dn6 = assign21150_e21452_d_n6;
        locals.var_temp1_dn7 = assign21150_e21452_d_n7;
        locals.var_temp1_dn8 = assign21150_e21452_d_n8;
        locals.var_temp1_dn9 = assign21150_e21452_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign21160_e21467, assign21160_e21467_d_n4, assign21160_e21467_d_n6, assign21160_e21467_d_n7, assign21160_e21467_d_n8, assign21160_e21467_d_n9,) = {
    if ((locals.var_guard667 != 0.0) && (locals.var_guard668 != 0.0)) {
        let assign21160_e21457: f64 = (-4.0);
        let assign21160_e21459: f64 = (assign21160_e21457 * locals.var_qsqd);
        let assign21160_e21461: f64 = (assign21160_e21459 * locals.var_sumd);
        let assign21160_e21464: f64 = (locals.var_qid * locals.var_temp1);
        let assign21160_e21465: f64 = (assign21160_e21461 / assign21160_e21464);
        (assign21160_e21465, ((((((assign21160_e21457 * locals.var_qsqd_dn4) * locals.var_sumd) + (assign21160_e21459 * locals.var_sumd_dn4)) * assign21160_e21464) - (assign21160_e21461 * ((locals.var_qid_dn4 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn4)))) / (assign21160_e21464 * assign21160_e21464)), ((((((assign21160_e21457 * locals.var_qsqd_dn6) * locals.var_sumd) + (assign21160_e21459 * locals.var_sumd_dn6)) * assign21160_e21464) - (assign21160_e21461 * ((locals.var_qid_dn6 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn6)))) / (assign21160_e21464 * assign21160_e21464)), ((((((assign21160_e21457 * locals.var_qsqd_dn7) * locals.var_sumd) + (assign21160_e21459 * locals.var_sumd_dn7)) * assign21160_e21464) - (assign21160_e21461 * ((locals.var_qid_dn7 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn7)))) / (assign21160_e21464 * assign21160_e21464)), ((((((assign21160_e21457 * locals.var_qsqd_dn8) * locals.var_sumd) + (assign21160_e21459 * locals.var_sumd_dn8)) * assign21160_e21464) - (assign21160_e21461 * ((locals.var_qid_dn8 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn8)))) / (assign21160_e21464 * assign21160_e21464)), ((((((assign21160_e21457 * locals.var_qsqd_dn9) * locals.var_sumd) + (assign21160_e21459 * locals.var_sumd_dn9)) * assign21160_e21464) - (assign21160_e21461 * ((locals.var_qid_dn9 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn9)))) / (assign21160_e21464 * assign21160_e21464)),)
    } else {
        (locals.var_dqsqd_dxn_qi, locals.var_dqsqd_dxn_qi_dn4, locals.var_dqsqd_dxn_qi_dn6, locals.var_dqsqd_dxn_qi_dn7, locals.var_dqsqd_dxn_qi_dn8, locals.var_dqsqd_dxn_qi_dn9,)
    }
};
        locals.var_dqsqd_dxn_qi = assign21160_e21467;
        locals.var_dqsqd_dxn_qi_dn4 = assign21160_e21467_d_n4;
        locals.var_dqsqd_dxn_qi_dn6 = assign21160_e21467_d_n6;
        locals.var_dqsqd_dxn_qi_dn7 = assign21160_e21467_d_n7;
        locals.var_dqsqd_dxn_qi_dn8 = assign21160_e21467_d_n8;
        locals.var_dqsqd_dxn_qi_dn9 = assign21160_e21467_d_n9;
        locals.var_dqsqd_dxn_qi_rv = 0.0;

        let (assign21170_e21492, assign21170_e21492_d_n4, assign21170_e21492_d_n6, assign21170_e21492_d_n7, assign21170_e21492_d_n8, assign21170_e21492_d_n9,) = {
    if ((locals.var_guard667 != 0.0) && (locals.var_guard668 == 0.0)) {
        let assign21170_e21476: f64 = (locals.var_qsqd * 0.0333333333333);
        let assign21170_e21480: f64 = (locals.var_qsqd * 0.0357142857143);
        let assign21170_e21484: f64 = (locals.var_qsqd * 0.0333333333333);
        let assign21170_e21485: f64 = (1.0 - assign21170_e21484);
        let assign21170_e21486: f64 = (assign21170_e21480 * assign21170_e21485);
        let assign21170_e21487: f64 = (1.0 - assign21170_e21486);
        let assign21170_e21488: f64 = (assign21170_e21476 * assign21170_e21487);
        let assign21170_e21489: f64 = (1.0 - assign21170_e21488);
        let assign21170_e21490: f64 = (0.1666666666667 * assign21170_e21489);
        (assign21170_e21490, (0.1666666666667 * (-(((locals.var_qsqd_dn4 * 0.0333333333333) * assign21170_e21487) + (assign21170_e21476 * (-(((locals.var_qsqd_dn4 * 0.0357142857143) * assign21170_e21485) + (assign21170_e21480 * (-(locals.var_qsqd_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd_dn6 * 0.0333333333333) * assign21170_e21487) + (assign21170_e21476 * (-(((locals.var_qsqd_dn6 * 0.0357142857143) * assign21170_e21485) + (assign21170_e21480 * (-(locals.var_qsqd_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd_dn7 * 0.0333333333333) * assign21170_e21487) + (assign21170_e21476 * (-(((locals.var_qsqd_dn7 * 0.0357142857143) * assign21170_e21485) + (assign21170_e21480 * (-(locals.var_qsqd_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd_dn8 * 0.0333333333333) * assign21170_e21487) + (assign21170_e21476 * (-(((locals.var_qsqd_dn8 * 0.0357142857143) * assign21170_e21485) + (assign21170_e21480 * (-(locals.var_qsqd_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqd_dn9 * 0.0333333333333) * assign21170_e21487) + (assign21170_e21476 * (-(((locals.var_qsqd_dn9 * 0.0357142857143) * assign21170_e21485) + (assign21170_e21480 * (-(locals.var_qsqd_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign21170_e21492;
        locals.var_temp1_dn4 = assign21170_e21492_d_n4;
        locals.var_temp1_dn6 = assign21170_e21492_d_n6;
        locals.var_temp1_dn7 = assign21170_e21492_d_n7;
        locals.var_temp1_dn8 = assign21170_e21492_d_n8;
        locals.var_temp1_dn9 = assign21170_e21492_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign21180_e21517, assign21180_e21517_d_n4, assign21180_e21517_d_n6, assign21180_e21517_d_n7, assign21180_e21517_d_n8, assign21180_e21517_d_n9,) = {
    if ((locals.var_guard667 != 0.0) && (locals.var_guard668 == 0.0)) {
        let assign21180_e21499: f64 = (locals.var_a1d * locals.var_aexp1d);
        let assign21180_e21502: f64 = (locals.var_a2d * locals.var_aexp2d);
        let assign21180_e21503: f64 = (assign21180_e21499 + assign21180_e21502);
        let assign21180_e21506: f64 = (locals.var_a1d * locals.var_a2d);
        let assign21180_e21508: f64 = (assign21180_e21506 * locals.var_qid);
        let assign21180_e21512: f64 = (locals.var_qid * locals.var_temp1);
        let assign21180_e21513: f64 = (1.0 + assign21180_e21512);
        let assign21180_e21514: f64 = (assign21180_e21508 * assign21180_e21513);
        let assign21180_e21515: f64 = (assign21180_e21503 + assign21180_e21514);
        (assign21180_e21515, ((((locals.var_a1d_dn4 * locals.var_aexp1d) + (locals.var_a1d * locals.var_aexp1d_dn4)) + ((locals.var_a2d_dn4 * locals.var_aexp2d) + (locals.var_a2d * locals.var_aexp2d_dn4))) + ((((((locals.var_a1d_dn4 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn4)) * locals.var_qid) + (assign21180_e21506 * locals.var_qid_dn4)) * assign21180_e21513) + (assign21180_e21508 * ((locals.var_qid_dn4 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn4))))), ((((locals.var_a1d_dn6 * locals.var_aexp1d) + (locals.var_a1d * locals.var_aexp1d_dn6)) + ((locals.var_a2d_dn6 * locals.var_aexp2d) + (locals.var_a2d * locals.var_aexp2d_dn6))) + ((((((locals.var_a1d_dn6 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn6)) * locals.var_qid) + (assign21180_e21506 * locals.var_qid_dn6)) * assign21180_e21513) + (assign21180_e21508 * ((locals.var_qid_dn6 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn6))))), ((((locals.var_a1d_dn7 * locals.var_aexp1d) + (locals.var_a1d * locals.var_aexp1d_dn7)) + ((locals.var_a2d_dn7 * locals.var_aexp2d) + (locals.var_a2d * locals.var_aexp2d_dn7))) + ((((((locals.var_a1d_dn7 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn7)) * locals.var_qid) + (assign21180_e21506 * locals.var_qid_dn7)) * assign21180_e21513) + (assign21180_e21508 * ((locals.var_qid_dn7 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn7))))), ((((locals.var_a1d_dn8 * locals.var_aexp1d) + (locals.var_a1d * locals.var_aexp1d_dn8)) + ((locals.var_a2d_dn8 * locals.var_aexp2d) + (locals.var_a2d * locals.var_aexp2d_dn8))) + ((((((locals.var_a1d_dn8 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn8)) * locals.var_qid) + (assign21180_e21506 * locals.var_qid_dn8)) * assign21180_e21513) + (assign21180_e21508 * ((locals.var_qid_dn8 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn8))))), ((((locals.var_a1d_dn9 * locals.var_aexp1d) + (locals.var_a1d * locals.var_aexp1d_dn9)) + ((locals.var_a2d_dn9 * locals.var_aexp2d) + (locals.var_a2d * locals.var_aexp2d_dn9))) + ((((((locals.var_a1d_dn9 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn9)) * locals.var_qid) + (assign21180_e21506 * locals.var_qid_dn9)) * assign21180_e21513) + (assign21180_e21508 * ((locals.var_qid_dn9 * locals.var_temp1) + (locals.var_qid * locals.var_temp1_dn9))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign21180_e21517;
        locals.var_temp2_dn4 = assign21180_e21517_d_n4;
        locals.var_temp2_dn6 = assign21180_e21517_d_n6;
        locals.var_temp2_dn7 = assign21180_e21517_d_n7;
        locals.var_temp2_dn8 = assign21180_e21517_d_n8;
        locals.var_temp2_dn9 = assign21180_e21517_d_n9;
        locals.var_temp2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_58(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21190_e21532, assign21190_e21532_d_n4, assign21190_e21532_d_n6, assign21190_e21532_d_n7, assign21190_e21532_d_n8, assign21190_e21532_d_n9,) = {
    if ((locals.var_guard667 != 0.0) && (locals.var_guard668 == 0.0)) {
        let assign21190_e21524: f64 = (locals.var_aexp1d * locals.var_aexp2d);
        let assign21190_e21526: f64 = (assign21190_e21524 * locals.var_sumd);
        let assign21190_e21529: f64 = (locals.var_qid * locals.var_temp2);
        let assign21190_e21530: f64 = (assign21190_e21526 / assign21190_e21529);
        (assign21190_e21530, (((((((locals.var_aexp1d_dn4 * locals.var_aexp2d) + (locals.var_aexp1d * locals.var_aexp2d_dn4)) * locals.var_sumd) + (assign21190_e21524 * locals.var_sumd_dn4)) * assign21190_e21529) - (assign21190_e21526 * ((locals.var_qid_dn4 * locals.var_temp2) + (locals.var_qid * locals.var_temp2_dn4)))) / (assign21190_e21529 * assign21190_e21529)), (((((((locals.var_aexp1d_dn6 * locals.var_aexp2d) + (locals.var_aexp1d * locals.var_aexp2d_dn6)) * locals.var_sumd) + (assign21190_e21524 * locals.var_sumd_dn6)) * assign21190_e21529) - (assign21190_e21526 * ((locals.var_qid_dn6 * locals.var_temp2) + (locals.var_qid * locals.var_temp2_dn6)))) / (assign21190_e21529 * assign21190_e21529)), (((((((locals.var_aexp1d_dn7 * locals.var_aexp2d) + (locals.var_aexp1d * locals.var_aexp2d_dn7)) * locals.var_sumd) + (assign21190_e21524 * locals.var_sumd_dn7)) * assign21190_e21529) - (assign21190_e21526 * ((locals.var_qid_dn7 * locals.var_temp2) + (locals.var_qid * locals.var_temp2_dn7)))) / (assign21190_e21529 * assign21190_e21529)), (((((((locals.var_aexp1d_dn8 * locals.var_aexp2d) + (locals.var_aexp1d * locals.var_aexp2d_dn8)) * locals.var_sumd) + (assign21190_e21524 * locals.var_sumd_dn8)) * assign21190_e21529) - (assign21190_e21526 * ((locals.var_qid_dn8 * locals.var_temp2) + (locals.var_qid * locals.var_temp2_dn8)))) / (assign21190_e21529 * assign21190_e21529)), (((((((locals.var_aexp1d_dn9 * locals.var_aexp2d) + (locals.var_aexp1d * locals.var_aexp2d_dn9)) * locals.var_sumd) + (assign21190_e21524 * locals.var_sumd_dn9)) * assign21190_e21529) - (assign21190_e21526 * ((locals.var_qid_dn9 * locals.var_temp2) + (locals.var_qid * locals.var_temp2_dn9)))) / (assign21190_e21529 * assign21190_e21529)),)
    } else {
        (locals.var_dqsqd_dxn_qi, locals.var_dqsqd_dxn_qi_dn4, locals.var_dqsqd_dxn_qi_dn6, locals.var_dqsqd_dxn_qi_dn7, locals.var_dqsqd_dxn_qi_dn8, locals.var_dqsqd_dxn_qi_dn9,)
    }
};
        locals.var_dqsqd_dxn_qi = assign21190_e21532;
        locals.var_dqsqd_dxn_qi_dn4 = assign21190_e21532_d_n4;
        locals.var_dqsqd_dxn_qi_dn6 = assign21190_e21532_d_n6;
        locals.var_dqsqd_dxn_qi_dn7 = assign21190_e21532_d_n7;
        locals.var_dqsqd_dxn_qi_dn8 = assign21190_e21532_d_n8;
        locals.var_dqsqd_dxn_qi_dn9 = assign21190_e21532_d_n9;
        locals.var_dqsqd_dxn_qi_rv = 0.0;

        let assign21200_e21535: f64 = (locals.var_qid).ln();
        let assign21200_e21536: f64 = (locals.var_xdeff + assign21200_e21535);
        locals.var_xdriftd = assign21200_e21536;
        locals.var_xdriftd_dn4 = (locals.var_xdeff_dn4 + (locals.var_qid_dn4 / locals.var_qid));
        locals.var_xdriftd_dn6 = (locals.var_xdeff_dn6 + (locals.var_qid_dn6 / locals.var_qid));
        locals.var_xdriftd_dn7 = (locals.var_xdeff_dn7 + (locals.var_qid_dn7 / locals.var_qid));
        locals.var_xdriftd_dn8 = (locals.var_xdeff_dn8 + (locals.var_qid_dn8 / locals.var_qid));
        locals.var_xdriftd_dn9 = (locals.var_xdeff_dn9 + (locals.var_qid_dn9 / locals.var_qid));
        locals.var_xdriftd_rv = 0.0;

        let assign21210_e21540: f64 = (locals.var_qis + locals.var_qid);
        let assign21210_e21541: f64 = (0.5 * assign21210_e21540);
        locals.var_qim = assign21210_e21541;
        locals.var_qim_dn4 = (0.5 * (locals.var_qis_dn4 + locals.var_qid_dn4));
        locals.var_qim_dn6 = (0.5 * (locals.var_qis_dn6 + locals.var_qid_dn6));
        locals.var_qim_dn7 = (0.5 * (locals.var_qis_dn7 + locals.var_qid_dn7));
        locals.var_qim_dn8 = (0.5 * (locals.var_qis_dn8 + locals.var_qid_dn8));
        locals.var_qim_dn9 = (0.5 * (locals.var_qis_dn9 + locals.var_qid_dn9));
        locals.var_qim_rv = 0.0;

        let assign21220_e21544: f64 = (locals.var_xdriftd - locals.var_xdrifts);
        locals.var_dxdrift = assign21220_e21544;
        locals.var_dxdrift_dn4 = (locals.var_xdriftd_dn4 - locals.var_xdrifts_dn4);
        locals.var_dxdrift_dn6 = (locals.var_xdriftd_dn6 - locals.var_xdrifts_dn6);
        locals.var_dxdrift_dn7 = (locals.var_xdriftd_dn7 - locals.var_xdrifts_dn7);
        locals.var_dxdrift_dn8 = (locals.var_xdriftd_dn8 - locals.var_xdrifts_dn8);
        locals.var_dxdrift_dn9 = (locals.var_xdriftd_dn9 - locals.var_xdrifts_dn9);
        locals.var_dxdrift_rv = 0.0;

        locals.var_ratio_pd = 1.0;
        locals.var_ratio_pd_dn4 = 0.0;
        locals.var_ratio_pd_dn6 = 0.0;
        locals.var_ratio_pd_dn7 = 0.0;
        locals.var_ratio_pd_dn8 = 0.0;
        locals.var_ratio_pd_dn9 = 0.0;
        locals.var_ratio_pd_rv = 0.0;

        let assign21240_e21548: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard669 = assign21240_e21548;
        locals.var_guard669_rv = 0.0;

        let (assign21250_e21558, assign21250_e21558_d_n4, assign21250_e21558_d_n6, assign21250_e21558_d_n7, assign21250_e21558_d_n8, assign21250_e21558_d_n9,) = {
    if (locals.var_guard669 != 0.0) {
        let assign21250_e21553: f64 = (locals.var_k1q1s + locals.var_k1q1d);
        let assign21250_e21554: f64 = (0.5 * assign21250_e21553);
        let assign21250_e21556: f64 = (assign21250_e21554 / locals.var_k1);
        (assign21250_e21556, ((((0.5 * (locals.var_k1q1s_dn4 + locals.var_k1q1d_dn4)) * locals.var_k1) - (assign21250_e21554 * locals.var_k1_dn4)) / (locals.var_k1 * locals.var_k1)), ((((0.5 * (locals.var_k1q1s_dn6 + locals.var_k1q1d_dn6)) * locals.var_k1) - (assign21250_e21554 * locals.var_k1_dn6)) / (locals.var_k1 * locals.var_k1)), ((((0.5 * (locals.var_k1q1s_dn7 + locals.var_k1q1d_dn7)) * locals.var_k1) - (assign21250_e21554 * locals.var_k1_dn7)) / (locals.var_k1 * locals.var_k1)), ((((0.5 * (locals.var_k1q1s_dn8 + locals.var_k1q1d_dn8)) * locals.var_k1) - (assign21250_e21554 * locals.var_k1_dn8)) / (locals.var_k1 * locals.var_k1)), ((((0.5 * (locals.var_k1q1s_dn9 + locals.var_k1q1d_dn9)) * locals.var_k1) - (assign21250_e21554 * locals.var_k1_dn9)) / (locals.var_k1 * locals.var_k1)),)
    } else {
        (locals.var_qim_pd, locals.var_qim_pd_dn4, locals.var_qim_pd_dn6, locals.var_qim_pd_dn7, locals.var_qim_pd_dn8, locals.var_qim_pd_dn9,)
    }
};
        locals.var_qim_pd = assign21250_e21558;
        locals.var_qim_pd_dn4 = assign21250_e21558_d_n4;
        locals.var_qim_pd_dn6 = assign21250_e21558_d_n6;
        locals.var_qim_pd_dn7 = assign21250_e21558_d_n7;
        locals.var_qim_pd_dn8 = assign21250_e21558_d_n8;
        locals.var_qim_pd_dn9 = assign21250_e21558_d_n9;
        locals.var_qim_pd_rv = 0.0;

        let (assign21260_e21577, assign21260_e21577_d_n4, assign21260_e21577_d_n6, assign21260_e21577_d_n7, assign21260_e21577_d_n8, assign21260_e21577_d_n9,) = {
    if (locals.var_guard669 != 0.0) {
        let assign21260_e21563: f64 = (locals.var_qim_pd + 1e-5);
        let assign21260_e21566: f64 = (locals.var_qim_pd - 1e-5);
        let assign21260_e21569: f64 = (locals.var_qim_pd - 1e-5);
        let assign21260_e21570: f64 = (assign21260_e21566 * assign21260_e21569);
        let assign21260_e21572: f64 = (assign21260_e21570 + 1.0);
        let assign21260_e21573: f64 = (assign21260_e21572).sqrt();
        let assign21260_e21574: f64 = (assign21260_e21563 + assign21260_e21573);
        let assign21260_e21575: f64 = (0.5 * assign21260_e21574);
        (assign21260_e21575, (0.5 * (locals.var_qim_pd_dn4 + (((locals.var_qim_pd_dn4 * assign21260_e21569) + (assign21260_e21566 * locals.var_qim_pd_dn4)) / (2.0 * assign21260_e21573)))), (0.5 * (locals.var_qim_pd_dn6 + (((locals.var_qim_pd_dn6 * assign21260_e21569) + (assign21260_e21566 * locals.var_qim_pd_dn6)) / (2.0 * assign21260_e21573)))), (0.5 * (locals.var_qim_pd_dn7 + (((locals.var_qim_pd_dn7 * assign21260_e21569) + (assign21260_e21566 * locals.var_qim_pd_dn7)) / (2.0 * assign21260_e21573)))), (0.5 * (locals.var_qim_pd_dn8 + (((locals.var_qim_pd_dn8 * assign21260_e21569) + (assign21260_e21566 * locals.var_qim_pd_dn8)) / (2.0 * assign21260_e21573)))), (0.5 * (locals.var_qim_pd_dn9 + (((locals.var_qim_pd_dn9 * assign21260_e21569) + (assign21260_e21566 * locals.var_qim_pd_dn9)) / (2.0 * assign21260_e21573)))),)
    } else {
        (locals.var_qim_pd, locals.var_qim_pd_dn4, locals.var_qim_pd_dn6, locals.var_qim_pd_dn7, locals.var_qim_pd_dn8, locals.var_qim_pd_dn9,)
    }
};
        locals.var_qim_pd = assign21260_e21577;
        locals.var_qim_pd_dn4 = assign21260_e21577_d_n4;
        locals.var_qim_pd_dn6 = assign21260_e21577_d_n6;
        locals.var_qim_pd_dn7 = assign21260_e21577_d_n7;
        locals.var_qim_pd_dn8 = assign21260_e21577_d_n8;
        locals.var_qim_pd_dn9 = assign21260_e21577_d_n9;
        locals.var_qim_pd_rv = 0.0;

        let (assign21270_e21594, assign21270_e21594_d_n4, assign21270_e21594_d_n6, assign21270_e21594_d_n7, assign21270_e21594_d_n8, assign21270_e21594_d_n9,) = {
    if (locals.var_guard669 != 0.0) {
        let assign21270_e21581: f64 = (locals.var_qim_pd / locals.var_inv_phit);
        let assign21270_e21584: f64 = (0.25 * locals.var_kp);
        let assign21270_e21586: f64 = (assign21270_e21584 * locals.var_kp);
        let assign21270_e21587: f64 = (assign21270_e21581 + assign21270_e21586);
        let assign21270_e21588: f64 = (assign21270_e21587).sqrt();
        let assign21270_e21591: f64 = (0.5 * locals.var_kp);
        let assign21270_e21592: f64 = (assign21270_e21588 - assign21270_e21591);
        (assign21270_e21592, ((((((locals.var_qim_pd_dn4 * locals.var_inv_phit) - (locals.var_qim_pd * locals.var_inv_phit_dn4)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn4) * locals.var_kp) + (assign21270_e21584 * locals.var_kp_dn4))) / (2.0 * assign21270_e21588)) - (0.5 * locals.var_kp_dn4)), ((((((locals.var_qim_pd_dn6 * locals.var_inv_phit) - (locals.var_qim_pd * locals.var_inv_phit_dn6)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn6) * locals.var_kp) + (assign21270_e21584 * locals.var_kp_dn6))) / (2.0 * assign21270_e21588)) - (0.5 * locals.var_kp_dn6)), ((((((locals.var_qim_pd_dn7 * locals.var_inv_phit) - (locals.var_qim_pd * locals.var_inv_phit_dn7)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn7) * locals.var_kp) + (assign21270_e21584 * locals.var_kp_dn7))) / (2.0 * assign21270_e21588)) - (0.5 * locals.var_kp_dn7)), ((((((locals.var_qim_pd_dn8 * locals.var_inv_phit) - (locals.var_qim_pd * locals.var_inv_phit_dn8)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn8) * locals.var_kp) + (assign21270_e21584 * locals.var_kp_dn8))) / (2.0 * assign21270_e21588)) - (0.5 * locals.var_kp_dn8)), ((((((locals.var_qim_pd_dn9 * locals.var_inv_phit) - (locals.var_qim_pd * locals.var_inv_phit_dn9)) / (locals.var_inv_phit * locals.var_inv_phit)) + (((0.25 * locals.var_kp_dn9) * locals.var_kp) + (assign21270_e21584 * locals.var_kp_dn9))) / (2.0 * assign21270_e21588)) - (0.5 * locals.var_kp_dn9)),)
    } else {
        (locals.var_temp0, locals.var_temp0_dn4, locals.var_temp0_dn6, locals.var_temp0_dn7, locals.var_temp0_dn8, locals.var_temp0_dn9,)
    }
};
        locals.var_temp0 = assign21270_e21594;
        locals.var_temp0_dn4 = assign21270_e21594_d_n4;
        locals.var_temp0_dn6 = assign21270_e21594_d_n6;
        locals.var_temp0_dn7 = assign21270_e21594_d_n7;
        locals.var_temp0_dn8 = assign21270_e21594_d_n8;
        locals.var_temp0_dn9 = assign21270_e21594_d_n9;
        locals.var_temp0_rv = 0.0;

        let (assign21280_e21602, assign21280_e21602_d_n4, assign21280_e21602_d_n6, assign21280_e21602_d_n7, assign21280_e21602_d_n8, assign21280_e21602_d_n9,) = {
    if (locals.var_guard669 != 0.0) {
        let assign21280_e21598: f64 = (locals.var_temp0).powf(2.0);
        let assign21280_e21600: f64 = (assign21280_e21598 * locals.var_inv_phit);
        (assign21280_e21600, ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn4)) } } else { (assign21280_e21598 * (2.0 * (locals.var_temp0_dn4 / locals.var_temp0))) } * locals.var_inv_phit) + (assign21280_e21598 * locals.var_inv_phit_dn4)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn6)) } } else { (assign21280_e21598 * (2.0 * (locals.var_temp0_dn6 / locals.var_temp0))) } * locals.var_inv_phit) + (assign21280_e21598 * locals.var_inv_phit_dn6)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn7)) } } else { (assign21280_e21598 * (2.0 * (locals.var_temp0_dn7 / locals.var_temp0))) } * locals.var_inv_phit) + (assign21280_e21598 * locals.var_inv_phit_dn7)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn8)) } } else { (assign21280_e21598 * (2.0 * (locals.var_temp0_dn8 / locals.var_temp0))) } * locals.var_inv_phit) + (assign21280_e21598 * locals.var_inv_phit_dn8)), ((if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_temp0).powf(2.0 - 1.0) * locals.var_temp0_dn9)) } } else { (assign21280_e21598 * (2.0 * (locals.var_temp0_dn9 / locals.var_temp0))) } * locals.var_inv_phit) + (assign21280_e21598 * locals.var_inv_phit_dn9)),)
    } else {
        (locals.var_xp_pd, locals.var_xp_pd_dn4, locals.var_xp_pd_dn6, locals.var_xp_pd_dn7, locals.var_xp_pd_dn8, locals.var_xp_pd_dn9,)
    }
};
        locals.var_xp_pd = assign21280_e21602;
        locals.var_xp_pd_dn4 = assign21280_e21602_d_n4;
        locals.var_xp_pd_dn6 = assign21280_e21602_d_n6;
        locals.var_xp_pd_dn7 = assign21280_e21602_d_n7;
        locals.var_xp_pd_dn8 = assign21280_e21602_d_n8;
        locals.var_xp_pd_dn9 = assign21280_e21602_d_n9;
        locals.var_xp_pd_rv = 0.0;

        let (assign21290_e21610, assign21290_e21610_d_n4, assign21290_e21610_d_n6, assign21290_e21610_d_n7, assign21290_e21610_d_n8, assign21290_e21610_d_n9,) = {
    if (locals.var_guard669 != 0.0) {
        let assign21290_e21607: f64 = (locals.var_xp_pd / locals.var_qim_pd);
        let assign21290_e21608: f64 = (1.0 - assign21290_e21607);
        (assign21290_e21608, (-(((locals.var_xp_pd_dn4 * locals.var_qim_pd) - (locals.var_xp_pd * locals.var_qim_pd_dn4)) / (locals.var_qim_pd * locals.var_qim_pd))), (-(((locals.var_xp_pd_dn6 * locals.var_qim_pd) - (locals.var_xp_pd * locals.var_qim_pd_dn6)) / (locals.var_qim_pd * locals.var_qim_pd))), (-(((locals.var_xp_pd_dn7 * locals.var_qim_pd) - (locals.var_xp_pd * locals.var_qim_pd_dn7)) / (locals.var_qim_pd * locals.var_qim_pd))), (-(((locals.var_xp_pd_dn8 * locals.var_qim_pd) - (locals.var_xp_pd * locals.var_qim_pd_dn8)) / (locals.var_qim_pd * locals.var_qim_pd))), (-(((locals.var_xp_pd_dn9 * locals.var_qim_pd) - (locals.var_xp_pd * locals.var_qim_pd_dn9)) / (locals.var_qim_pd * locals.var_qim_pd))),)
    } else {
        (locals.var_ratio_pd, locals.var_ratio_pd_dn4, locals.var_ratio_pd_dn6, locals.var_ratio_pd_dn7, locals.var_ratio_pd_dn8, locals.var_ratio_pd_dn9,)
    }
};
        locals.var_ratio_pd = assign21290_e21610;
        locals.var_ratio_pd_dn4 = assign21290_e21610_d_n4;
        locals.var_ratio_pd_dn6 = assign21290_e21610_d_n6;
        locals.var_ratio_pd_dn7 = assign21290_e21610_d_n7;
        locals.var_ratio_pd_dn8 = assign21290_e21610_d_n8;
        locals.var_ratio_pd_dn9 = assign21290_e21610_d_n9;
        locals.var_ratio_pd_rv = 0.0;

        let assign21300_e21613: f64 = (locals.var_k1q1d / 2.0);
        let assign21300_e21615: f64 = if assign21300_e21613 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard670 = assign21300_e21615;
        locals.var_guard670_rv = 0.0;

        let (assign21310_e21625, assign21310_e21625_d_n4, assign21310_e21625_d_n6, assign21310_e21625_d_n7, assign21310_e21625_d_n8, assign21310_e21625_d_n9,) = {
    if (locals.var_guard670 != 0.0) {
        let assign21310_e21620: f64 = (locals.var_k1q1d / 2.0);
        let assign21310_e21621: f64 = (assign21310_e21620).exp();
        let assign21310_e21622: f64 = (1.0 + assign21310_e21621);
        let assign21310_e21623: f64 = (assign21310_e21622).ln();
        (assign21310_e21623, ((assign21310_e21621 * (locals.var_k1q1d_dn4 / 2.0)) / assign21310_e21622), ((assign21310_e21621 * (locals.var_k1q1d_dn6 / 2.0)) / assign21310_e21622), ((assign21310_e21621 * (locals.var_k1q1d_dn7 / 2.0)) / assign21310_e21622), ((assign21310_e21621 * (locals.var_k1q1d_dn8 / 2.0)) / assign21310_e21622), ((assign21310_e21621 * (locals.var_k1q1d_dn9 / 2.0)) / assign21310_e21622),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign21310_e21625;
        locals.var_temp1_dn4 = assign21310_e21625_d_n4;
        locals.var_temp1_dn6 = assign21310_e21625_d_n6;
        locals.var_temp1_dn7 = assign21310_e21625_d_n7;
        locals.var_temp1_dn8 = assign21310_e21625_d_n8;
        locals.var_temp1_dn9 = assign21310_e21625_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign21320_e21632, assign21320_e21632_d_n4, assign21320_e21632_d_n6, assign21320_e21632_d_n7, assign21320_e21632_d_n8, assign21320_e21632_d_n9,) = {
    if (locals.var_guard670 == 0.0) {
        let assign21320_e21630: f64 = (locals.var_k1q1d / 2.0);
        (assign21320_e21630, (locals.var_k1q1d_dn4 / 2.0), (locals.var_k1q1d_dn6 / 2.0), (locals.var_k1q1d_dn7 / 2.0), (locals.var_k1q1d_dn8 / 2.0), (locals.var_k1q1d_dn9 / 2.0),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign21320_e21632;
        locals.var_temp1_dn4 = assign21320_e21632_d_n4;
        locals.var_temp1_dn6 = assign21320_e21632_d_n6;
        locals.var_temp1_dn7 = assign21320_e21632_d_n7;
        locals.var_temp1_dn8 = assign21320_e21632_d_n8;
        locals.var_temp1_dn9 = assign21320_e21632_d_n9;
        locals.var_temp1_rv = 0.0;

        let assign21330_e21635: f64 = (2.0 * locals.var_temp1);
        locals.var_esurf1d = assign21330_e21635;
        locals.var_esurf1d_dn4 = (2.0 * locals.var_temp1_dn4);
        locals.var_esurf1d_dn6 = (2.0 * locals.var_temp1_dn6);
        locals.var_esurf1d_dn7 = (2.0 * locals.var_temp1_dn7);
        locals.var_esurf1d_dn8 = (2.0 * locals.var_temp1_dn8);
        locals.var_esurf1d_dn9 = (2.0 * locals.var_temp1_dn9);
        locals.var_esurf1d_rv = 0.0;

        let assign21340_e21638: f64 = (locals.var_k2q2d / 2.0);
        let assign21340_e21640: f64 = if assign21340_e21638 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard671 = assign21340_e21640;
        locals.var_guard671_rv = 0.0;

        let (assign21350_e21650, assign21350_e21650_d_n4, assign21350_e21650_d_n6, assign21350_e21650_d_n7, assign21350_e21650_d_n8, assign21350_e21650_d_n9,) = {
    if (locals.var_guard671 != 0.0) {
        let assign21350_e21645: f64 = (locals.var_k2q2d / 2.0);
        let assign21350_e21646: f64 = (assign21350_e21645).exp();
        let assign21350_e21647: f64 = (1.0 + assign21350_e21646);
        let assign21350_e21648: f64 = (assign21350_e21647).ln();
        (assign21350_e21648, ((assign21350_e21646 * (locals.var_k2q2d_dn4 / 2.0)) / assign21350_e21647), ((assign21350_e21646 * (locals.var_k2q2d_dn6 / 2.0)) / assign21350_e21647), ((assign21350_e21646 * (locals.var_k2q2d_dn7 / 2.0)) / assign21350_e21647), ((assign21350_e21646 * (locals.var_k2q2d_dn8 / 2.0)) / assign21350_e21647), ((assign21350_e21646 * (locals.var_k2q2d_dn9 / 2.0)) / assign21350_e21647),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign21350_e21650;
        locals.var_temp2_dn4 = assign21350_e21650_d_n4;
        locals.var_temp2_dn6 = assign21350_e21650_d_n6;
        locals.var_temp2_dn7 = assign21350_e21650_d_n7;
        locals.var_temp2_dn8 = assign21350_e21650_d_n8;
        locals.var_temp2_dn9 = assign21350_e21650_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign21360_e21657, assign21360_e21657_d_n4, assign21360_e21657_d_n6, assign21360_e21657_d_n7, assign21360_e21657_d_n8, assign21360_e21657_d_n9,) = {
    if (locals.var_guard671 == 0.0) {
        let assign21360_e21655: f64 = (locals.var_k2q2d / 2.0);
        (assign21360_e21655, (locals.var_k2q2d_dn4 / 2.0), (locals.var_k2q2d_dn6 / 2.0), (locals.var_k2q2d_dn7 / 2.0), (locals.var_k2q2d_dn8 / 2.0), (locals.var_k2q2d_dn9 / 2.0),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign21360_e21657;
        locals.var_temp2_dn4 = assign21360_e21657_d_n4;
        locals.var_temp2_dn6 = assign21360_e21657_d_n6;
        locals.var_temp2_dn7 = assign21360_e21657_d_n7;
        locals.var_temp2_dn8 = assign21360_e21657_d_n8;
        locals.var_temp2_dn9 = assign21360_e21657_d_n9;
        locals.var_temp2_rv = 0.0;

        let assign21370_e21660: f64 = (2.0 * locals.var_temp2);
        locals.var_esurf2d = assign21370_e21660;
        locals.var_esurf2d_dn4 = (2.0 * locals.var_temp2_dn4);
        locals.var_esurf2d_dn6 = (2.0 * locals.var_temp2_dn6);
        locals.var_esurf2d_dn7 = (2.0 * locals.var_temp2_dn7);
        locals.var_esurf2d_dn8 = (2.0 * locals.var_temp2_dn8);
        locals.var_esurf2d_dn9 = (2.0 * locals.var_temp2_dn9);
        locals.var_esurf2d_rv = 0.0;

        let assign21380_e21663: f64 = (locals.var_esurf2d - locals.var_k2q2d);
        locals.var_ecpl1d = assign21380_e21663;
        locals.var_ecpl1d_dn4 = (locals.var_esurf2d_dn4 - locals.var_k2q2d_dn4);
        locals.var_ecpl1d_dn6 = (locals.var_esurf2d_dn6 - locals.var_k2q2d_dn6);
        locals.var_ecpl1d_dn7 = (locals.var_esurf2d_dn7 - locals.var_k2q2d_dn7);
        locals.var_ecpl1d_dn8 = (locals.var_esurf2d_dn8 - locals.var_k2q2d_dn8);
        locals.var_ecpl1d_dn9 = (locals.var_esurf2d_dn9 - locals.var_k2q2d_dn9);
        locals.var_ecpl1d_rv = 0.0;

        let assign21390_e21666: f64 = (locals.var_esurf1d - locals.var_k1q1d);
        locals.var_ecpl2d = assign21390_e21666;
        locals.var_ecpl2d_dn4 = (locals.var_esurf1d_dn4 - locals.var_k1q1d_dn4);
        locals.var_ecpl2d_dn6 = (locals.var_esurf1d_dn6 - locals.var_k1q1d_dn6);
        locals.var_ecpl2d_dn7 = (locals.var_esurf1d_dn7 - locals.var_k1q1d_dn7);
        locals.var_ecpl2d_dn8 = (locals.var_esurf1d_dn8 - locals.var_k1q1d_dn8);
        locals.var_ecpl2d_dn9 = (locals.var_esurf1d_dn9 - locals.var_k1q1d_dn9);
        locals.var_ecpl2d_rv = 0.0;

        let assign21400_e21669: f64 = (locals.var_eta_mu * locals.var_esurf1d);
        let assign21400_e21672: f64 = (locals.var_one_m_eta * locals.var_ecpl1d);
        let assign21400_e21673: f64 = (assign21400_e21669 + assign21400_e21672);
        locals.var_eeff1d = assign21400_e21673;
        locals.var_eeff1d_dn4 = ((locals.var_eta_mu * locals.var_esurf1d_dn4) + (locals.var_one_m_eta * locals.var_ecpl1d_dn4));
        locals.var_eeff1d_dn6 = ((locals.var_eta_mu * locals.var_esurf1d_dn6) + (locals.var_one_m_eta * locals.var_ecpl1d_dn6));
        locals.var_eeff1d_dn7 = ((locals.var_eta_mu * locals.var_esurf1d_dn7) + (locals.var_one_m_eta * locals.var_ecpl1d_dn7));
        locals.var_eeff1d_dn8 = ((locals.var_eta_mu * locals.var_esurf1d_dn8) + (locals.var_one_m_eta * locals.var_ecpl1d_dn8));
        locals.var_eeff1d_dn9 = ((locals.var_eta_mu * locals.var_esurf1d_dn9) + (locals.var_one_m_eta * locals.var_ecpl1d_dn9));
        locals.var_eeff1d_rv = 0.0;

        let assign21410_e21676: f64 = (locals.var_eta_mu * locals.var_esurf2d);
        let assign21410_e21679: f64 = (locals.var_one_m_eta * locals.var_ecpl2d);
        let assign21410_e21680: f64 = (assign21410_e21676 + assign21410_e21679);
        locals.var_eeff2d = assign21410_e21680;
        locals.var_eeff2d_dn4 = ((locals.var_eta_mu * locals.var_esurf2d_dn4) + (locals.var_one_m_eta * locals.var_ecpl2d_dn4));
        locals.var_eeff2d_dn6 = ((locals.var_eta_mu * locals.var_esurf2d_dn6) + (locals.var_one_m_eta * locals.var_ecpl2d_dn6));
        locals.var_eeff2d_dn7 = ((locals.var_eta_mu * locals.var_esurf2d_dn7) + (locals.var_one_m_eta * locals.var_ecpl2d_dn7));
        locals.var_eeff2d_dn8 = ((locals.var_eta_mu * locals.var_esurf2d_dn8) + (locals.var_one_m_eta * locals.var_ecpl2d_dn8));
        locals.var_eeff2d_dn9 = ((locals.var_eta_mu * locals.var_esurf2d_dn9) + (locals.var_one_m_eta * locals.var_ecpl2d_dn9));
        locals.var_eeff2d_rv = 0.0;

        let assign21420_e21684: f64 = (locals.var_esurf1s + locals.var_esurf1d);
        let assign21420_e21685: f64 = (0.5 * assign21420_e21684);
        locals.var_esurf1 = assign21420_e21685;
        locals.var_esurf1_dn4 = (0.5 * (locals.var_esurf1s_dn4 + locals.var_esurf1d_dn4));
        locals.var_esurf1_dn6 = (0.5 * (locals.var_esurf1s_dn6 + locals.var_esurf1d_dn6));
        locals.var_esurf1_dn7 = (0.5 * (locals.var_esurf1s_dn7 + locals.var_esurf1d_dn7));
        locals.var_esurf1_dn8 = (0.5 * (locals.var_esurf1s_dn8 + locals.var_esurf1d_dn8));
        locals.var_esurf1_dn9 = (0.5 * (locals.var_esurf1s_dn9 + locals.var_esurf1d_dn9));
        locals.var_esurf1_rv = 0.0;

        let assign21430_e21689: f64 = (locals.var_esurf2s + locals.var_esurf2d);
        let assign21430_e21690: f64 = (0.5 * assign21430_e21689);
        locals.var_esurf2 = assign21430_e21690;
        locals.var_esurf2_dn4 = (0.5 * (locals.var_esurf2s_dn4 + locals.var_esurf2d_dn4));
        locals.var_esurf2_dn6 = (0.5 * (locals.var_esurf2s_dn6 + locals.var_esurf2d_dn6));
        locals.var_esurf2_dn7 = (0.5 * (locals.var_esurf2s_dn7 + locals.var_esurf2d_dn7));
        locals.var_esurf2_dn8 = (0.5 * (locals.var_esurf2s_dn8 + locals.var_esurf2d_dn8));
        locals.var_esurf2_dn9 = (0.5 * (locals.var_esurf2s_dn9 + locals.var_esurf2d_dn9));
        locals.var_esurf2_rv = 0.0;

        let assign21440_e21694: f64 = (locals.var_esurf1 + locals.var_esurf2);
        let assign21440_e21695: f64 = (1.0 / assign21440_e21694);
        locals.var_temp = assign21440_e21695;
        locals.var_temp_dn4 = (-((locals.var_esurf1_dn4 + locals.var_esurf2_dn4) / (assign21440_e21694 * assign21440_e21694)));
        locals.var_temp_dn6 = (-((locals.var_esurf1_dn6 + locals.var_esurf2_dn6) / (assign21440_e21694 * assign21440_e21694)));
        locals.var_temp_dn7 = (-((locals.var_esurf1_dn7 + locals.var_esurf2_dn7) / (assign21440_e21694 * assign21440_e21694)));
        locals.var_temp_dn8 = (-((locals.var_esurf1_dn8 + locals.var_esurf2_dn8) / (assign21440_e21694 * assign21440_e21694)));
        locals.var_temp_dn9 = (-((locals.var_esurf1_dn9 + locals.var_esurf2_dn9) / (assign21440_e21694 * assign21440_e21694)));
        locals.var_temp_rv = 0.0;

        let assign21450_e21698: f64 = (locals.var_qim * locals.var_esurf1);
        let assign21450_e21700: f64 = (assign21450_e21698 * locals.var_temp);
        locals.var_qi1m = assign21450_e21700;
        locals.var_qi1m_dn4 = ((((locals.var_qim_dn4 * locals.var_esurf1) + (locals.var_qim * locals.var_esurf1_dn4)) * locals.var_temp) + (assign21450_e21698 * locals.var_temp_dn4));
        locals.var_qi1m_dn6 = ((((locals.var_qim_dn6 * locals.var_esurf1) + (locals.var_qim * locals.var_esurf1_dn6)) * locals.var_temp) + (assign21450_e21698 * locals.var_temp_dn6));
        locals.var_qi1m_dn7 = ((((locals.var_qim_dn7 * locals.var_esurf1) + (locals.var_qim * locals.var_esurf1_dn7)) * locals.var_temp) + (assign21450_e21698 * locals.var_temp_dn7));
        locals.var_qi1m_dn8 = ((((locals.var_qim_dn8 * locals.var_esurf1) + (locals.var_qim * locals.var_esurf1_dn8)) * locals.var_temp) + (assign21450_e21698 * locals.var_temp_dn8));
        locals.var_qi1m_dn9 = ((((locals.var_qim_dn9 * locals.var_esurf1) + (locals.var_qim * locals.var_esurf1_dn9)) * locals.var_temp) + (assign21450_e21698 * locals.var_temp_dn9));
        locals.var_qi1m_rv = 0.0;

        let assign21460_e21703: f64 = (locals.var_qim * locals.var_esurf2);
        let assign21460_e21705: f64 = (assign21460_e21703 * locals.var_temp);
        locals.var_qi2m = assign21460_e21705;
        locals.var_qi2m_dn4 = ((((locals.var_qim_dn4 * locals.var_esurf2) + (locals.var_qim * locals.var_esurf2_dn4)) * locals.var_temp) + (assign21460_e21703 * locals.var_temp_dn4));
        locals.var_qi2m_dn6 = ((((locals.var_qim_dn6 * locals.var_esurf2) + (locals.var_qim * locals.var_esurf2_dn6)) * locals.var_temp) + (assign21460_e21703 * locals.var_temp_dn6));
        locals.var_qi2m_dn7 = ((((locals.var_qim_dn7 * locals.var_esurf2) + (locals.var_qim * locals.var_esurf2_dn7)) * locals.var_temp) + (assign21460_e21703 * locals.var_temp_dn7));
        locals.var_qi2m_dn8 = ((((locals.var_qim_dn8 * locals.var_esurf2) + (locals.var_qim * locals.var_esurf2_dn8)) * locals.var_temp) + (assign21460_e21703 * locals.var_temp_dn8));
        locals.var_qi2m_dn9 = ((((locals.var_qim_dn9 * locals.var_esurf2) + (locals.var_qim * locals.var_esurf2_dn9)) * locals.var_temp) + (assign21460_e21703 * locals.var_temp_dn9));
        locals.var_qi2m_rv = 0.0;

        let assign21470_e21709: f64 = (locals.var_ecpl1s + locals.var_ecpl1d);
        let assign21470_e21710: f64 = (0.5 * assign21470_e21709);
        locals.var_ecpl1 = assign21470_e21710;
        locals.var_ecpl1_dn4 = (0.5 * (locals.var_ecpl1s_dn4 + locals.var_ecpl1d_dn4));
        locals.var_ecpl1_dn6 = (0.5 * (locals.var_ecpl1s_dn6 + locals.var_ecpl1d_dn6));
        locals.var_ecpl1_dn7 = (0.5 * (locals.var_ecpl1s_dn7 + locals.var_ecpl1d_dn7));
        locals.var_ecpl1_dn8 = (0.5 * (locals.var_ecpl1s_dn8 + locals.var_ecpl1d_dn8));
        locals.var_ecpl1_dn9 = (0.5 * (locals.var_ecpl1s_dn9 + locals.var_ecpl1d_dn9));
        locals.var_ecpl1_rv = 0.0;

        let assign21480_e21714: f64 = (locals.var_ecpl2s + locals.var_ecpl2d);
        let assign21480_e21715: f64 = (0.5 * assign21480_e21714);
        locals.var_ecpl2 = assign21480_e21715;
        locals.var_ecpl2_dn4 = (0.5 * (locals.var_ecpl2s_dn4 + locals.var_ecpl2d_dn4));
        locals.var_ecpl2_dn6 = (0.5 * (locals.var_ecpl2s_dn6 + locals.var_ecpl2d_dn6));
        locals.var_ecpl2_dn7 = (0.5 * (locals.var_ecpl2s_dn7 + locals.var_ecpl2d_dn7));
        locals.var_ecpl2_dn8 = (0.5 * (locals.var_ecpl2s_dn8 + locals.var_ecpl2d_dn8));
        locals.var_ecpl2_dn9 = (0.5 * (locals.var_ecpl2s_dn9 + locals.var_ecpl2d_dn9));
        locals.var_ecpl2_rv = 0.0;

        let assign21490_e21719: f64 = (locals.var_eeff1s + locals.var_eeff1d);
        let assign21490_e21720: f64 = (0.5 * assign21490_e21719);
        locals.var_eeff1 = assign21490_e21720;
        locals.var_eeff1_dn4 = (0.5 * (locals.var_eeff1s_dn4 + locals.var_eeff1d_dn4));
        locals.var_eeff1_dn6 = (0.5 * (locals.var_eeff1s_dn6 + locals.var_eeff1d_dn6));
        locals.var_eeff1_dn7 = (0.5 * (locals.var_eeff1s_dn7 + locals.var_eeff1d_dn7));
        locals.var_eeff1_dn8 = (0.5 * (locals.var_eeff1s_dn8 + locals.var_eeff1d_dn8));
        locals.var_eeff1_dn9 = (0.5 * (locals.var_eeff1s_dn9 + locals.var_eeff1d_dn9));
        locals.var_eeff1_rv = 0.0;

        let assign21500_e21724: f64 = (locals.var_eeff2s + locals.var_eeff2d);
        let assign21500_e21725: f64 = (0.5 * assign21500_e21724);
        locals.var_eeff2 = assign21500_e21725;
        locals.var_eeff2_dn4 = (0.5 * (locals.var_eeff2s_dn4 + locals.var_eeff2d_dn4));
        locals.var_eeff2_dn6 = (0.5 * (locals.var_eeff2s_dn6 + locals.var_eeff2d_dn6));
        locals.var_eeff2_dn7 = (0.5 * (locals.var_eeff2s_dn7 + locals.var_eeff2d_dn7));
        locals.var_eeff2_dn8 = (0.5 * (locals.var_eeff2s_dn8 + locals.var_eeff2d_dn8));
        locals.var_eeff2_dn9 = (0.5 * (locals.var_eeff2s_dn9 + locals.var_eeff2d_dn9));
        locals.var_eeff2_rv = 0.0;

        let assign21510_e21728: f64 = (locals.var_esurf1 * locals.var_betn1_t);
        let assign21510_e21731: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign21510_e21732: f64 = (assign21510_e21731).exp();
        let assign21510_e21733: f64 = (assign21510_e21728 * assign21510_e21732);
        let assign21510_e21735: f64 = (assign21510_e21733 * locals.var_ratio_pd);
        locals.var_c1 = assign21510_e21735;
        locals.var_c1_dn4 = ((((((locals.var_esurf1_dn4 * locals.var_betn1_t) + (locals.var_esurf1 * locals.var_betn1_t_dn4)) * assign21510_e21732) + (assign21510_e21728 * (assign21510_e21732 * (locals.var_stbet_i * locals.var_lnrtn_dn4)))) * locals.var_ratio_pd) + (assign21510_e21733 * locals.var_ratio_pd_dn4));
        locals.var_c1_dn6 = ((((((locals.var_esurf1_dn6 * locals.var_betn1_t) + (locals.var_esurf1 * locals.var_betn1_t_dn6)) * assign21510_e21732) + (assign21510_e21728 * (assign21510_e21732 * (locals.var_stbet_i * locals.var_lnrtn_dn6)))) * locals.var_ratio_pd) + (assign21510_e21733 * locals.var_ratio_pd_dn6));
        locals.var_c1_dn7 = ((((((locals.var_esurf1_dn7 * locals.var_betn1_t) + (locals.var_esurf1 * locals.var_betn1_t_dn7)) * assign21510_e21732) + (assign21510_e21728 * (assign21510_e21732 * (locals.var_stbet_i * locals.var_lnrtn_dn7)))) * locals.var_ratio_pd) + (assign21510_e21733 * locals.var_ratio_pd_dn7));
        locals.var_c1_dn8 = ((((((locals.var_esurf1_dn8 * locals.var_betn1_t) + (locals.var_esurf1 * locals.var_betn1_t_dn8)) * assign21510_e21732) + (assign21510_e21728 * (assign21510_e21732 * (locals.var_stbet_i * locals.var_lnrtn_dn8)))) * locals.var_ratio_pd) + (assign21510_e21733 * locals.var_ratio_pd_dn8));
        locals.var_c1_dn9 = ((((((locals.var_esurf1_dn9 * locals.var_betn1_t) + (locals.var_esurf1 * locals.var_betn1_t_dn9)) * assign21510_e21732) + (assign21510_e21728 * (assign21510_e21732 * (locals.var_stbet_i * locals.var_lnrtn_dn9)))) * locals.var_ratio_pd) + (assign21510_e21733 * locals.var_ratio_pd_dn9));
        locals.var_c1_rv = 0.0;

        let assign21520_e21738: f64 = (locals.var_esurf2 * locals.var_betn2_t);
        let assign21520_e21741: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign21520_e21742: f64 = (assign21520_e21741).exp();
        let assign21520_e21743: f64 = (assign21520_e21738 * assign21520_e21742);
        locals.var_c2 = assign21520_e21743;
        locals.var_c2_dn4 = ((((locals.var_esurf2_dn4 * locals.var_betn2_t) + (locals.var_esurf2 * locals.var_betn2_t_dn4)) * assign21520_e21742) + (assign21520_e21738 * (assign21520_e21742 * (locals.var_stbet_i * locals.var_lnrtn_dn4))));
        locals.var_c2_dn6 = ((((locals.var_esurf2_dn6 * locals.var_betn2_t) + (locals.var_esurf2 * locals.var_betn2_t_dn6)) * assign21520_e21742) + (assign21520_e21738 * (assign21520_e21742 * (locals.var_stbet_i * locals.var_lnrtn_dn6))));
        locals.var_c2_dn7 = ((((locals.var_esurf2_dn7 * locals.var_betn2_t) + (locals.var_esurf2 * locals.var_betn2_t_dn7)) * assign21520_e21742) + (assign21520_e21738 * (assign21520_e21742 * (locals.var_stbet_i * locals.var_lnrtn_dn7))));
        locals.var_c2_dn8 = ((((locals.var_esurf2_dn8 * locals.var_betn2_t) + (locals.var_esurf2 * locals.var_betn2_t_dn8)) * assign21520_e21742) + (assign21520_e21738 * (assign21520_e21742 * (locals.var_stbet_i * locals.var_lnrtn_dn8))));
        locals.var_c2_dn9 = ((((locals.var_esurf2_dn9 * locals.var_betn2_t) + (locals.var_esurf2 * locals.var_betn2_t_dn9)) * assign21520_e21742) + (assign21520_e21738 * (assign21520_e21742 * (locals.var_stbet_i * locals.var_lnrtn_dn9))));
        locals.var_c2_rv = 0.0;

        let assign21530_e21746: f64 = (locals.var_c1 + locals.var_c2);
        locals.var_csum = assign21530_e21746;
        locals.var_csum_dn4 = (locals.var_c1_dn4 + locals.var_c2_dn4);
        locals.var_csum_dn6 = (locals.var_c1_dn6 + locals.var_c2_dn6);
        locals.var_csum_dn7 = (locals.var_c1_dn7 + locals.var_c2_dn7);
        locals.var_csum_dn8 = (locals.var_c1_dn8 + locals.var_c2_dn8);
        locals.var_csum_dn9 = (locals.var_c1_dn9 + locals.var_c2_dn9);
        locals.var_csum_rv = 0.0;

        let assign21540_e21751: f64 = (locals.var_xcorb_i * locals.var_ecpl2);
        let assign21540_e21752: f64 = (locals.var_ecpl1 + assign21540_e21751);
        let assign21540_e21753: f64 = (locals.var_xcor_i * assign21540_e21752);
        locals.var_temp1 = assign21540_e21753;
        locals.var_temp1_dn4 = ((locals.var_xcor_i_dn4 * assign21540_e21752) + (locals.var_xcor_i * (locals.var_ecpl1_dn4 + (locals.var_xcorb_i * locals.var_ecpl2_dn4))));
        locals.var_temp1_dn6 = ((locals.var_xcor_i_dn6 * assign21540_e21752) + (locals.var_xcor_i * (locals.var_ecpl1_dn6 + (locals.var_xcorb_i * locals.var_ecpl2_dn6))));
        locals.var_temp1_dn7 = ((locals.var_xcor_i_dn7 * assign21540_e21752) + (locals.var_xcor_i * (locals.var_ecpl1_dn7 + (locals.var_xcorb_i * locals.var_ecpl2_dn7))));
        locals.var_temp1_dn8 = ((locals.var_xcor_i_dn8 * assign21540_e21752) + (locals.var_xcor_i * (locals.var_ecpl1_dn8 + (locals.var_xcorb_i * locals.var_ecpl2_dn8))));
        locals.var_temp1_dn9 = ((locals.var_xcor_i_dn9 * assign21540_e21752) + (locals.var_xcor_i * (locals.var_ecpl1_dn9 + (locals.var_xcorb_i * locals.var_ecpl2_dn9))));
        locals.var_temp1_rv = 0.0;

        let assign21550_e21757: f64 = (1.0 + locals.var_temp1);
        let assign21550_e21759: f64 = assign21550_e21757;
        let assign21550_e21762: f64 = (1.0 + locals.var_temp1);
        let assign21550_e21764: f64 = assign21550_e21762;
        let assign21550_e21767: f64 = (1.0 + locals.var_temp1);
        let assign21550_e21769: f64 = assign21550_e21767;
        let assign21550_e21770: f64 = (assign21550_e21764 * assign21550_e21769);
        let assign21550_e21772: f64 = (assign21550_e21770 + 0.01);
        let assign21550_e21773: f64 = (assign21550_e21772).sqrt();
        let assign21550_e21774: f64 = (assign21550_e21759 + assign21550_e21773);
        let assign21550_e21775: f64 = (0.5 * assign21550_e21774);
        locals.var_temp2 = assign21550_e21775;
        locals.var_temp2_dn4 = (0.5 * (locals.var_temp1_dn4 + (((locals.var_temp1_dn4 * assign21550_e21769) + (assign21550_e21764 * locals.var_temp1_dn4)) / (2.0 * assign21550_e21773))));
        locals.var_temp2_dn6 = (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign21550_e21769) + (assign21550_e21764 * locals.var_temp1_dn6)) / (2.0 * assign21550_e21773))));
        locals.var_temp2_dn7 = (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign21550_e21769) + (assign21550_e21764 * locals.var_temp1_dn7)) / (2.0 * assign21550_e21773))));
        locals.var_temp2_dn8 = (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign21550_e21769) + (assign21550_e21764 * locals.var_temp1_dn8)) / (2.0 * assign21550_e21773))));
        locals.var_temp2_dn9 = (0.5 * (locals.var_temp1_dn9 + (((locals.var_temp1_dn9 * assign21550_e21769) + (assign21550_e21764 * locals.var_temp1_dn9)) / (2.0 * assign21550_e21773))));
        locals.var_temp2_rv = 0.0;

        let assign21560_e21780: f64 = (0.2 * locals.var_temp1);
        let assign21560_e21781: f64 = (1.0 + assign21560_e21780);
        let assign21560_e21783: f64 = assign21560_e21781;
        let assign21560_e21787: f64 = (0.2 * locals.var_temp1);
        let assign21560_e21788: f64 = (1.0 + assign21560_e21787);
        let assign21560_e21790: f64 = assign21560_e21788;
        let assign21560_e21794: f64 = (0.2 * locals.var_temp1);
        let assign21560_e21795: f64 = (1.0 + assign21560_e21794);
        let assign21560_e21797: f64 = assign21560_e21795;
        let assign21560_e21798: f64 = (assign21560_e21790 * assign21560_e21797);
        let assign21560_e21800: f64 = (assign21560_e21798 + 0.01);
        let assign21560_e21801: f64 = (assign21560_e21800).sqrt();
        let assign21560_e21802: f64 = (assign21560_e21783 + assign21560_e21801);
        let assign21560_e21803: f64 = (0.5 * assign21560_e21802);
        locals.var_temp3 = assign21560_e21803;
        locals.var_temp3_dn4 = (0.5 * ((0.2 * locals.var_temp1_dn4) + ((((0.2 * locals.var_temp1_dn4) * assign21560_e21797) + (assign21560_e21790 * (0.2 * locals.var_temp1_dn4))) / (2.0 * assign21560_e21801))));
        locals.var_temp3_dn6 = (0.5 * ((0.2 * locals.var_temp1_dn6) + ((((0.2 * locals.var_temp1_dn6) * assign21560_e21797) + (assign21560_e21790 * (0.2 * locals.var_temp1_dn6))) / (2.0 * assign21560_e21801))));
        locals.var_temp3_dn7 = (0.5 * ((0.2 * locals.var_temp1_dn7) + ((((0.2 * locals.var_temp1_dn7) * assign21560_e21797) + (assign21560_e21790 * (0.2 * locals.var_temp1_dn7))) / (2.0 * assign21560_e21801))));
        locals.var_temp3_dn8 = (0.5 * ((0.2 * locals.var_temp1_dn8) + ((((0.2 * locals.var_temp1_dn8) * assign21560_e21797) + (assign21560_e21790 * (0.2 * locals.var_temp1_dn8))) / (2.0 * assign21560_e21801))));
        locals.var_temp3_dn9 = (0.5 * ((0.2 * locals.var_temp1_dn9) + ((((0.2 * locals.var_temp1_dn9) * assign21560_e21797) + (assign21560_e21790 * (0.2 * locals.var_temp1_dn9))) / (2.0 * assign21560_e21801))));
        locals.var_temp3_rv = 0.0;

        let assign21570_e21806: f64 = (locals.var_temp2 / locals.var_temp3);
        locals.var_fcor = assign21570_e21806;
        locals.var_fcor_dn4 = (((locals.var_temp2_dn4 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn4)) / (locals.var_temp3 * locals.var_temp3));
        locals.var_fcor_dn6 = (((locals.var_temp2_dn6 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn6)) / (locals.var_temp3 * locals.var_temp3));
        locals.var_fcor_dn7 = (((locals.var_temp2_dn7 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn7)) / (locals.var_temp3 * locals.var_temp3));
        locals.var_fcor_dn8 = (((locals.var_temp2_dn8 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn8)) / (locals.var_temp3 * locals.var_temp3));
        locals.var_fcor_dn9 = (((locals.var_temp2_dn9 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn9)) / (locals.var_temp3 * locals.var_temp3));
        locals.var_fcor_rv = 0.0;

        let assign21580_e21811: f64 = (locals.var_csfi_i * locals.var_ecpl1);
        let assign21580_e21812: f64 = (1.0 + assign21580_e21811);
        let assign21580_e21815: f64 = (locals.var_csbi_i * locals.var_ecpl2);
        let assign21580_e21816: f64 = (assign21580_e21812 + assign21580_e21815);
        let assign21580_e21817: f64 = (locals.var_cs_i * assign21580_e21816);
        let assign21580_e21819: f64 = (-locals.var_thecs_i);
        let assign21580_e21823: f64 = (locals.var_qi1m * locals.var_inv_qi1cs);
        let assign21580_e21824: f64 = (1.0 + assign21580_e21823);
        let assign21580_e21827: f64 = (locals.var_qi2m * locals.var_inv_qi2cs);
        let assign21580_e21828: f64 = (assign21580_e21824 + assign21580_e21827);
        let assign21580_e21829: f64 = (assign21580_e21828).ln();
        let assign21580_e21830: f64 = (assign21580_e21819 * assign21580_e21829);
        let assign21580_e21831: f64 = (assign21580_e21830).exp();
        let assign21580_e21832: f64 = (assign21580_e21817 * assign21580_e21831);
        locals.var_gcs = assign21580_e21832;
        locals.var_gcs_dn4 = ((((locals.var_cs_i_dn4 * assign21580_e21816) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1_dn4) + (locals.var_csbi_i * locals.var_ecpl2_dn4)))) * assign21580_e21831) + (assign21580_e21817 * (assign21580_e21831 * (((-locals.var_thecs_i_dn4) * assign21580_e21829) + (assign21580_e21819 * (((locals.var_qi1m_dn4 * locals.var_inv_qi1cs) + (locals.var_qi2m_dn4 * locals.var_inv_qi2cs)) / assign21580_e21828))))));
        locals.var_gcs_dn6 = ((((locals.var_cs_i_dn6 * assign21580_e21816) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1_dn6) + (locals.var_csbi_i * locals.var_ecpl2_dn6)))) * assign21580_e21831) + (assign21580_e21817 * (assign21580_e21831 * (((-locals.var_thecs_i_dn6) * assign21580_e21829) + (assign21580_e21819 * (((locals.var_qi1m_dn6 * locals.var_inv_qi1cs) + (locals.var_qi2m_dn6 * locals.var_inv_qi2cs)) / assign21580_e21828))))));
        locals.var_gcs_dn7 = ((((locals.var_cs_i_dn7 * assign21580_e21816) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1_dn7) + (locals.var_csbi_i * locals.var_ecpl2_dn7)))) * assign21580_e21831) + (assign21580_e21817 * (assign21580_e21831 * (((-locals.var_thecs_i_dn7) * assign21580_e21829) + (assign21580_e21819 * (((locals.var_qi1m_dn7 * locals.var_inv_qi1cs) + (locals.var_qi2m_dn7 * locals.var_inv_qi2cs)) / assign21580_e21828))))));
        locals.var_gcs_dn8 = ((((locals.var_cs_i_dn8 * assign21580_e21816) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1_dn8) + (locals.var_csbi_i * locals.var_ecpl2_dn8)))) * assign21580_e21831) + (assign21580_e21817 * (assign21580_e21831 * (((-locals.var_thecs_i_dn8) * assign21580_e21829) + (assign21580_e21819 * (((locals.var_qi1m_dn8 * locals.var_inv_qi1cs) + (locals.var_qi2m_dn8 * locals.var_inv_qi2cs)) / assign21580_e21828))))));
        locals.var_gcs_dn9 = ((((locals.var_cs_i_dn9 * assign21580_e21816) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1_dn9) + (locals.var_csbi_i * locals.var_ecpl2_dn9)))) * assign21580_e21831) + (assign21580_e21817 * (assign21580_e21831 * (((-locals.var_thecs_i_dn9) * assign21580_e21829) + (assign21580_e21819 * (((locals.var_qi1m_dn9 * locals.var_inv_qi1cs) + (locals.var_qi2m_dn9 * locals.var_inv_qi2cs)) / assign21580_e21828))))));
        locals.var_gcs_rv = 0.0;

        let assign21590_e21835: f64 = if locals.var_rsg_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard672 = assign21590_e21835;
        locals.var_guard672_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_59(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21600_e21839, assign21600_e21839_d_n4, assign21600_e21839_d_n6, assign21600_e21839_d_n7, assign21600_e21839_d_n8, assign21600_e21839_d_n9,) = {
    if (locals.var_guard672 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign21600_e21839;
        locals.var_temp3_dn4 = assign21600_e21839_d_n4;
        locals.var_temp3_dn6 = assign21600_e21839_d_n6;
        locals.var_temp3_dn7 = assign21600_e21839_d_n7;
        locals.var_temp3_dn8 = assign21600_e21839_d_n8;
        locals.var_temp3_dn9 = assign21600_e21839_d_n9;
        locals.var_temp3_rv = 0.0;

        let assign21610_e21842: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard673 = assign21610_e21842;
        locals.var_guard673_rv = 0.0;

        let (assign21620_e21857, assign21620_e21857_d_n4, assign21620_e21857_d_n6, assign21620_e21857_d_n7, assign21620_e21857_d_n8, assign21620_e21857_d_n9,) = {
    if ((locals.var_guard672 == 0.0) && (locals.var_guard673 != 0.0)) {
        let assign21620_e21851: f64 = (locals.var_qim + 1e-12);
        let assign21620_e21852: f64 = (assign21620_e21851).ln();
        let assign21620_e21853: f64 = (locals.var_thersg_i * assign21620_e21852);
        let assign21620_e21854: f64 = (assign21620_e21853).exp();
        let assign21620_e21855: f64 = (locals.var_rsg_i * assign21620_e21854);
        (assign21620_e21855, (locals.var_rsg_i * (assign21620_e21854 * (locals.var_thersg_i * (locals.var_qim_dn4 / assign21620_e21851)))), (locals.var_rsg_i * (assign21620_e21854 * (locals.var_thersg_i * (locals.var_qim_dn6 / assign21620_e21851)))), (locals.var_rsg_i * (assign21620_e21854 * (locals.var_thersg_i * (locals.var_qim_dn7 / assign21620_e21851)))), (locals.var_rsg_i * (assign21620_e21854 * (locals.var_thersg_i * (locals.var_qim_dn8 / assign21620_e21851)))), (locals.var_rsg_i * (assign21620_e21854 * (locals.var_thersg_i * (locals.var_qim_dn9 / assign21620_e21851)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign21620_e21857;
        locals.var_temp1_dn4 = assign21620_e21857_d_n4;
        locals.var_temp1_dn6 = assign21620_e21857_d_n6;
        locals.var_temp1_dn7 = assign21620_e21857_d_n7;
        locals.var_temp1_dn8 = assign21620_e21857_d_n8;
        locals.var_temp1_dn9 = assign21620_e21857_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign21630_e21866, assign21630_e21866_d_n4, assign21630_e21866_d_n6, assign21630_e21866_d_n7, assign21630_e21866_d_n8, assign21630_e21866_d_n9,) = {
    if ((locals.var_guard672 == 0.0) && (locals.var_guard673 != 0.0)) {
        let assign21630_e21864: f64 = (1.0 - locals.var_temp1);
        (assign21630_e21864, (-locals.var_temp1_dn4), (-locals.var_temp1_dn6), (-locals.var_temp1_dn7), (-locals.var_temp1_dn8), (-locals.var_temp1_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign21630_e21866;
        locals.var_temp3_dn4 = assign21630_e21866_d_n4;
        locals.var_temp3_dn6 = assign21630_e21866_d_n6;
        locals.var_temp3_dn7 = assign21630_e21866_d_n7;
        locals.var_temp3_dn8 = assign21630_e21866_d_n8;
        locals.var_temp3_dn9 = assign21630_e21866_d_n9;
        locals.var_temp3_rv = 0.0;

        let (assign21640_e21882, assign21640_e21882_d_n4, assign21640_e21882_d_n6, assign21640_e21882_d_n7, assign21640_e21882_d_n8, assign21640_e21882_d_n9,) = {
    if ((locals.var_guard672 == 0.0) && (locals.var_guard673 == 0.0)) {
        let assign21640_e21876: f64 = (locals.var_qim + 1e-12);
        let assign21640_e21877: f64 = (assign21640_e21876).ln();
        let assign21640_e21878: f64 = (locals.var_thersg_i * assign21640_e21877);
        let assign21640_e21879: f64 = (assign21640_e21878).exp();
        let assign21640_e21880: f64 = (locals.var_rsg_i * assign21640_e21879);
        (assign21640_e21880, (locals.var_rsg_i * (assign21640_e21879 * (locals.var_thersg_i * (locals.var_qim_dn4 / assign21640_e21876)))), (locals.var_rsg_i * (assign21640_e21879 * (locals.var_thersg_i * (locals.var_qim_dn6 / assign21640_e21876)))), (locals.var_rsg_i * (assign21640_e21879 * (locals.var_thersg_i * (locals.var_qim_dn7 / assign21640_e21876)))), (locals.var_rsg_i * (assign21640_e21879 * (locals.var_thersg_i * (locals.var_qim_dn8 / assign21640_e21876)))), (locals.var_rsg_i * (assign21640_e21879 * (locals.var_thersg_i * (locals.var_qim_dn9 / assign21640_e21876)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign21640_e21882;
        locals.var_temp1_dn4 = assign21640_e21882_d_n4;
        locals.var_temp1_dn6 = assign21640_e21882_d_n6;
        locals.var_temp1_dn7 = assign21640_e21882_d_n7;
        locals.var_temp1_dn8 = assign21640_e21882_d_n8;
        locals.var_temp1_dn9 = assign21640_e21882_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign21650_e21894, assign21650_e21894_d_n4, assign21650_e21894_d_n6, assign21650_e21894_d_n7, assign21650_e21894_d_n8, assign21650_e21894_d_n9,) = {
    if ((locals.var_guard672 == 0.0) && (locals.var_guard673 == 0.0)) {
        let assign21650_e21891: f64 = (1.0 + locals.var_temp1);
        let assign21650_e21892: f64 = (1.0 / assign21650_e21891);
        (assign21650_e21892, (-(locals.var_temp1_dn4 / (assign21650_e21891 * assign21650_e21891))), (-(locals.var_temp1_dn6 / (assign21650_e21891 * assign21650_e21891))), (-(locals.var_temp1_dn7 / (assign21650_e21891 * assign21650_e21891))), (-(locals.var_temp1_dn8 / (assign21650_e21891 * assign21650_e21891))), (-(locals.var_temp1_dn9 / (assign21650_e21891 * assign21650_e21891))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign21650_e21894;
        locals.var_temp3_dn4 = assign21650_e21894_d_n4;
        locals.var_temp3_dn6 = assign21650_e21894_d_n6;
        locals.var_temp3_dn7 = assign21650_e21894_d_n7;
        locals.var_temp3_dn8 = assign21650_e21894_d_n8;
        locals.var_temp3_dn9 = assign21650_e21894_d_n9;
        locals.var_temp3_rv = 0.0;

        let assign21660_e21898: f64 = (locals.var_qim * locals.var_temp3);
        let assign21660_e21900: f64 = (assign21660_e21898 + locals.var_rsig_i);
        let assign21660_e21901: f64 = (locals.var_frscsi * assign21660_e21900);
        locals.var_grs = assign21660_e21901;
        locals.var_grs_dn4 = ((locals.var_frscsi_dn4 * assign21660_e21900) + (locals.var_frscsi * ((locals.var_qim_dn4 * locals.var_temp3) + (locals.var_qim * locals.var_temp3_dn4))));
        locals.var_grs_dn6 = ((locals.var_frscsi_dn6 * assign21660_e21900) + (locals.var_frscsi * ((locals.var_qim_dn6 * locals.var_temp3) + (locals.var_qim * locals.var_temp3_dn6))));
        locals.var_grs_dn7 = ((locals.var_frscsi_dn7 * assign21660_e21900) + (locals.var_frscsi * ((locals.var_qim_dn7 * locals.var_temp3) + (locals.var_qim * locals.var_temp3_dn7))));
        locals.var_grs_dn8 = ((locals.var_frscsi_dn8 * assign21660_e21900) + (locals.var_frscsi * ((locals.var_qim_dn8 * locals.var_temp3) + (locals.var_qim * locals.var_temp3_dn8))));
        locals.var_grs_dn9 = ((locals.var_frscsi_dn9 * assign21660_e21900) + (locals.var_frscsi * ((locals.var_qim_dn9 * locals.var_temp3) + (locals.var_qim * locals.var_temp3_dn9))));
        locals.var_grs_rv = 0.0;

        let assign21670_e21906: f64 = (locals.var_fmue * locals.var_eeff1);
        let assign21670_e21908: f64 = (assign21670_e21906 + 1e-6);
        let assign21670_e21909: f64 = (assign21670_e21908).ln();
        let assign21670_e21910: f64 = (locals.var_themu_i * assign21670_e21909);
        let assign21670_e21911: f64 = (assign21670_e21910).exp();
        let assign21670_e21912: f64 = (1.0 + assign21670_e21911);
        let assign21670_e21914: f64 = (assign21670_e21912 + locals.var_gcs);
        let assign21670_e21917: f64 = (locals.var_betn1_i * locals.var_grs);
        let assign21670_e21918: f64 = (assign21670_e21914 + assign21670_e21917);
        locals.var_gmob1 = assign21670_e21918;
        locals.var_gmob1_dn4 = (((assign21670_e21911 * ((locals.var_themu_i_dn4 * assign21670_e21909) + (locals.var_themu_i * (((locals.var_fmue_dn4 * locals.var_eeff1) + (locals.var_fmue * locals.var_eeff1_dn4)) / assign21670_e21908)))) + locals.var_gcs_dn4) + ((locals.var_betn1_i_dn4 * locals.var_grs) + (locals.var_betn1_i * locals.var_grs_dn4)));
        locals.var_gmob1_dn6 = (((assign21670_e21911 * ((locals.var_themu_i_dn6 * assign21670_e21909) + (locals.var_themu_i * (((locals.var_fmue_dn6 * locals.var_eeff1) + (locals.var_fmue * locals.var_eeff1_dn6)) / assign21670_e21908)))) + locals.var_gcs_dn6) + ((locals.var_betn1_i_dn6 * locals.var_grs) + (locals.var_betn1_i * locals.var_grs_dn6)));
        locals.var_gmob1_dn7 = (((assign21670_e21911 * ((locals.var_themu_i_dn7 * assign21670_e21909) + (locals.var_themu_i * (((locals.var_fmue_dn7 * locals.var_eeff1) + (locals.var_fmue * locals.var_eeff1_dn7)) / assign21670_e21908)))) + locals.var_gcs_dn7) + ((locals.var_betn1_i_dn7 * locals.var_grs) + (locals.var_betn1_i * locals.var_grs_dn7)));
        locals.var_gmob1_dn8 = (((assign21670_e21911 * ((locals.var_themu_i_dn8 * assign21670_e21909) + (locals.var_themu_i * (((locals.var_fmue_dn8 * locals.var_eeff1) + (locals.var_fmue * locals.var_eeff1_dn8)) / assign21670_e21908)))) + locals.var_gcs_dn8) + ((locals.var_betn1_i_dn8 * locals.var_grs) + (locals.var_betn1_i * locals.var_grs_dn8)));
        locals.var_gmob1_dn9 = (((assign21670_e21911 * ((locals.var_themu_i_dn9 * assign21670_e21909) + (locals.var_themu_i * (((locals.var_fmue_dn9 * locals.var_eeff1) + (locals.var_fmue * locals.var_eeff1_dn9)) / assign21670_e21908)))) + locals.var_gcs_dn9) + ((locals.var_betn1_i_dn9 * locals.var_grs) + (locals.var_betn1_i * locals.var_grs_dn9)));
        locals.var_gmob1_rv = 0.0;

        let assign21680_e21923: f64 = (locals.var_fmue * locals.var_eeff2);
        let assign21680_e21925: f64 = (assign21680_e21923 + 1e-6);
        let assign21680_e21926: f64 = (assign21680_e21925).ln();
        let assign21680_e21927: f64 = (locals.var_themu_i * assign21680_e21926);
        let assign21680_e21928: f64 = (assign21680_e21927).exp();
        let assign21680_e21929: f64 = (1.0 + assign21680_e21928);
        let assign21680_e21931: f64 = (assign21680_e21929 + locals.var_gcs);
        let assign21680_e21934: f64 = (locals.var_betn2_i * locals.var_grs);
        let assign21680_e21935: f64 = (assign21680_e21931 + assign21680_e21934);
        locals.var_gmob2 = assign21680_e21935;
        locals.var_gmob2_dn4 = (((assign21680_e21928 * ((locals.var_themu_i_dn4 * assign21680_e21926) + (locals.var_themu_i * (((locals.var_fmue_dn4 * locals.var_eeff2) + (locals.var_fmue * locals.var_eeff2_dn4)) / assign21680_e21925)))) + locals.var_gcs_dn4) + ((locals.var_betn2_i_dn4 * locals.var_grs) + (locals.var_betn2_i * locals.var_grs_dn4)));
        locals.var_gmob2_dn6 = (((assign21680_e21928 * ((locals.var_themu_i_dn6 * assign21680_e21926) + (locals.var_themu_i * (((locals.var_fmue_dn6 * locals.var_eeff2) + (locals.var_fmue * locals.var_eeff2_dn6)) / assign21680_e21925)))) + locals.var_gcs_dn6) + ((locals.var_betn2_i_dn6 * locals.var_grs) + (locals.var_betn2_i * locals.var_grs_dn6)));
        locals.var_gmob2_dn7 = (((assign21680_e21928 * ((locals.var_themu_i_dn7 * assign21680_e21926) + (locals.var_themu_i * (((locals.var_fmue_dn7 * locals.var_eeff2) + (locals.var_fmue * locals.var_eeff2_dn7)) / assign21680_e21925)))) + locals.var_gcs_dn7) + ((locals.var_betn2_i_dn7 * locals.var_grs) + (locals.var_betn2_i * locals.var_grs_dn7)));
        locals.var_gmob2_dn8 = (((assign21680_e21928 * ((locals.var_themu_i_dn8 * assign21680_e21926) + (locals.var_themu_i * (((locals.var_fmue_dn8 * locals.var_eeff2) + (locals.var_fmue * locals.var_eeff2_dn8)) / assign21680_e21925)))) + locals.var_gcs_dn8) + ((locals.var_betn2_i_dn8 * locals.var_grs) + (locals.var_betn2_i * locals.var_grs_dn8)));
        locals.var_gmob2_dn9 = (((assign21680_e21928 * ((locals.var_themu_i_dn9 * assign21680_e21926) + (locals.var_themu_i * (((locals.var_fmue_dn9 * locals.var_eeff2) + (locals.var_fmue * locals.var_eeff2_dn9)) / assign21680_e21925)))) + locals.var_gcs_dn9) + ((locals.var_betn2_i_dn9 * locals.var_grs) + (locals.var_betn2_i * locals.var_grs_dn9)));
        locals.var_gmob2_rv = 0.0;

        let assign21690_e21938: f64 = (locals.var_fcor * locals.var_csum);
        let assign21690_e21941: f64 = (locals.var_c1 / locals.var_gmob1);
        let assign21690_e21944: f64 = (locals.var_c2 / locals.var_gmob2);
        let assign21690_e21945: f64 = (assign21690_e21941 + assign21690_e21944);
        let assign21690_e21946: f64 = (assign21690_e21938 / assign21690_e21945);
        locals.var_gmob = assign21690_e21946;
        locals.var_gmob_dn4 = (((((locals.var_fcor_dn4 * locals.var_csum) + (locals.var_fcor * locals.var_csum_dn4)) * assign21690_e21945) - (assign21690_e21938 * ((((locals.var_c1_dn4 * locals.var_gmob1) - (locals.var_c1 * locals.var_gmob1_dn4)) / (locals.var_gmob1 * locals.var_gmob1)) + (((locals.var_c2_dn4 * locals.var_gmob2) - (locals.var_c2 * locals.var_gmob2_dn4)) / (locals.var_gmob2 * locals.var_gmob2))))) / (assign21690_e21945 * assign21690_e21945));
        locals.var_gmob_dn6 = (((((locals.var_fcor_dn6 * locals.var_csum) + (locals.var_fcor * locals.var_csum_dn6)) * assign21690_e21945) - (assign21690_e21938 * ((((locals.var_c1_dn6 * locals.var_gmob1) - (locals.var_c1 * locals.var_gmob1_dn6)) / (locals.var_gmob1 * locals.var_gmob1)) + (((locals.var_c2_dn6 * locals.var_gmob2) - (locals.var_c2 * locals.var_gmob2_dn6)) / (locals.var_gmob2 * locals.var_gmob2))))) / (assign21690_e21945 * assign21690_e21945));
        locals.var_gmob_dn7 = (((((locals.var_fcor_dn7 * locals.var_csum) + (locals.var_fcor * locals.var_csum_dn7)) * assign21690_e21945) - (assign21690_e21938 * ((((locals.var_c1_dn7 * locals.var_gmob1) - (locals.var_c1 * locals.var_gmob1_dn7)) / (locals.var_gmob1 * locals.var_gmob1)) + (((locals.var_c2_dn7 * locals.var_gmob2) - (locals.var_c2 * locals.var_gmob2_dn7)) / (locals.var_gmob2 * locals.var_gmob2))))) / (assign21690_e21945 * assign21690_e21945));
        locals.var_gmob_dn8 = (((((locals.var_fcor_dn8 * locals.var_csum) + (locals.var_fcor * locals.var_csum_dn8)) * assign21690_e21945) - (assign21690_e21938 * ((((locals.var_c1_dn8 * locals.var_gmob1) - (locals.var_c1 * locals.var_gmob1_dn8)) / (locals.var_gmob1 * locals.var_gmob1)) + (((locals.var_c2_dn8 * locals.var_gmob2) - (locals.var_c2 * locals.var_gmob2_dn8)) / (locals.var_gmob2 * locals.var_gmob2))))) / (assign21690_e21945 * assign21690_e21945));
        locals.var_gmob_dn9 = (((((locals.var_fcor_dn9 * locals.var_csum) + (locals.var_fcor * locals.var_csum_dn9)) * assign21690_e21945) - (assign21690_e21938 * ((((locals.var_c1_dn9 * locals.var_gmob1) - (locals.var_c1 * locals.var_gmob1_dn9)) / (locals.var_gmob1 * locals.var_gmob1)) + (((locals.var_c2_dn9 * locals.var_gmob2) - (locals.var_c2 * locals.var_gmob2_dn9)) / (locals.var_gmob2 * locals.var_gmob2))))) / (assign21690_e21945 * assign21690_e21945));
        locals.var_gmob_rv = 0.0;

        let assign21700_e21950: f64 = (4.0 + locals.var_qim);
        let assign21700_e21951: f64 = (1.0 / assign21700_e21950);
        locals.var_inv_qimstar1 = assign21700_e21951;
        locals.var_inv_qimstar1_dn4 = (-(locals.var_qim_dn4 / (assign21700_e21950 * assign21700_e21950)));
        locals.var_inv_qimstar1_dn6 = (-(locals.var_qim_dn6 / (assign21700_e21950 * assign21700_e21950)));
        locals.var_inv_qimstar1_dn7 = (-(locals.var_qim_dn7 / (assign21700_e21950 * assign21700_e21950)));
        locals.var_inv_qimstar1_dn8 = (-(locals.var_qim_dn8 / (assign21700_e21950 * assign21700_e21950)));
        locals.var_inv_qimstar1_dn9 = (-(locals.var_qim_dn9 / (assign21700_e21950 * assign21700_e21950)));
        locals.var_inv_qimstar1_rv = 0.0;

        let assign21710_e21954: f64 = if locals.var_alpb_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard674 = assign21710_e21954;
        locals.var_guard674_rv = 0.0;

        let (assign21720_e21964, assign21720_e21964_d_n4, assign21720_e21964_d_n6, assign21720_e21964_d_n7, assign21720_e21964_d_n8, assign21720_e21964_d_n9,) = {
    if (locals.var_guard674 != 0.0) {
        let assign21720_e21960: f64 = (locals.var_alpb_i * locals.var_qi2m);
        let assign21720_e21961: f64 = (1.0 + assign21720_e21960);
        let assign21720_e21962: f64 = (1.0 / assign21720_e21961);
        (assign21720_e21962, (-((locals.var_alpb_i * locals.var_qi2m_dn4) / (assign21720_e21961 * assign21720_e21961))), (-((locals.var_alpb_i * locals.var_qi2m_dn6) / (assign21720_e21961 * assign21720_e21961))), (-((locals.var_alpb_i * locals.var_qi2m_dn7) / (assign21720_e21961 * assign21720_e21961))), (-((locals.var_alpb_i * locals.var_qi2m_dn8) / (assign21720_e21961 * assign21720_e21961))), (-((locals.var_alpb_i * locals.var_qi2m_dn9) / (assign21720_e21961 * assign21720_e21961))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign21720_e21964;
        locals.var_temp_dn4 = assign21720_e21964_d_n4;
        locals.var_temp_dn6 = assign21720_e21964_d_n6;
        locals.var_temp_dn7 = assign21720_e21964_d_n7;
        locals.var_temp_dn8 = assign21720_e21964_d_n8;
        locals.var_temp_dn9 = assign21720_e21964_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign21730_e21973, assign21730_e21973_d_n4, assign21730_e21973_d_n6, assign21730_e21973_d_n7, assign21730_e21973_d_n8, assign21730_e21973_d_n9,) = {
    if (locals.var_guard674 == 0.0) {
        let assign21730_e21970: f64 = (locals.var_alpb_i * locals.var_qi2m);
        let assign21730_e21971: f64 = (1.0 - assign21730_e21970);
        (assign21730_e21971, (-(locals.var_alpb_i * locals.var_qi2m_dn4)), (-(locals.var_alpb_i * locals.var_qi2m_dn6)), (-(locals.var_alpb_i * locals.var_qi2m_dn7)), (-(locals.var_alpb_i * locals.var_qi2m_dn8)), (-(locals.var_alpb_i * locals.var_qi2m_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign21730_e21973;
        locals.var_temp_dn4 = assign21730_e21973_d_n4;
        locals.var_temp_dn6 = assign21730_e21973_d_n6;
        locals.var_temp_dn7 = assign21730_e21973_d_n7;
        locals.var_temp_dn8 = assign21730_e21973_d_n8;
        locals.var_temp_dn9 = assign21730_e21973_d_n9;
        locals.var_temp_rv = 0.0;

        let assign21740_e21976: f64 = (locals.var_qim * locals.var_inv_qimstar1);
        let assign21740_e21978: f64 = (assign21740_e21976 * locals.var_temp);
        locals.var_r1 = assign21740_e21978;
        locals.var_r1_dn4 = ((((locals.var_qim_dn4 * locals.var_inv_qimstar1) + (locals.var_qim * locals.var_inv_qimstar1_dn4)) * locals.var_temp) + (assign21740_e21976 * locals.var_temp_dn4));
        locals.var_r1_dn6 = ((((locals.var_qim_dn6 * locals.var_inv_qimstar1) + (locals.var_qim * locals.var_inv_qimstar1_dn6)) * locals.var_temp) + (assign21740_e21976 * locals.var_temp_dn6));
        locals.var_r1_dn7 = ((((locals.var_qim_dn7 * locals.var_inv_qimstar1) + (locals.var_qim * locals.var_inv_qimstar1_dn7)) * locals.var_temp) + (assign21740_e21976 * locals.var_temp_dn7));
        locals.var_r1_dn8 = ((((locals.var_qim_dn8 * locals.var_inv_qimstar1) + (locals.var_qim * locals.var_inv_qimstar1_dn8)) * locals.var_temp) + (assign21740_e21976 * locals.var_temp_dn8));
        locals.var_r1_dn9 = ((((locals.var_qim_dn9 * locals.var_inv_qimstar1) + (locals.var_qim * locals.var_inv_qimstar1_dn9)) * locals.var_temp) + (assign21740_e21976 * locals.var_temp_dn9));
        locals.var_r1_rv = 0.0;

        let assign21750_e21982: f64 = (locals.var_xd - locals.var_xdeff);
        let assign21750_e21985: f64 = (locals.var_vp_i * locals.var_inv_phit);
        let assign21750_e21988: f64 = (locals.var_vpg_i * locals.var_qim);
        let assign21750_e21990: f64 = (assign21750_e21988 * locals.var_qim);
        let assign21750_e21991: f64 = (assign21750_e21985 + assign21750_e21990);
        let assign21750_e21992: f64 = (assign21750_e21982 / assign21750_e21991);
        let assign21750_e21993: f64 = (1.0 + assign21750_e21992);
        let assign21750_e21994: f64 = (assign21750_e21993).ln();
        let assign21750_e21996: f64 = (assign21750_e21994 * locals.var_r1);
        locals.var_dl_l_fact = assign21750_e21996;
        locals.var_dl_l_fact_dn4 = (((((((locals.var_xd_dn4 - locals.var_xdeff_dn4) * assign21750_e21991) - (assign21750_e21982 * ((locals.var_vp_i * locals.var_inv_phit_dn4) + (((locals.var_vpg_i * locals.var_qim_dn4) * locals.var_qim) + (assign21750_e21988 * locals.var_qim_dn4))))) / (assign21750_e21991 * assign21750_e21991)) / assign21750_e21993) * locals.var_r1) + (assign21750_e21994 * locals.var_r1_dn4));
        locals.var_dl_l_fact_dn6 = (((((((locals.var_xd_dn6 - locals.var_xdeff_dn6) * assign21750_e21991) - (assign21750_e21982 * ((locals.var_vp_i * locals.var_inv_phit_dn6) + (((locals.var_vpg_i * locals.var_qim_dn6) * locals.var_qim) + (assign21750_e21988 * locals.var_qim_dn6))))) / (assign21750_e21991 * assign21750_e21991)) / assign21750_e21993) * locals.var_r1) + (assign21750_e21994 * locals.var_r1_dn6));
        locals.var_dl_l_fact_dn7 = (((((((locals.var_xd_dn7 - locals.var_xdeff_dn7) * assign21750_e21991) - (assign21750_e21982 * ((locals.var_vp_i * locals.var_inv_phit_dn7) + (((locals.var_vpg_i * locals.var_qim_dn7) * locals.var_qim) + (assign21750_e21988 * locals.var_qim_dn7))))) / (assign21750_e21991 * assign21750_e21991)) / assign21750_e21993) * locals.var_r1) + (assign21750_e21994 * locals.var_r1_dn7));
        locals.var_dl_l_fact_dn8 = (((((((locals.var_xd_dn8 - locals.var_xdeff_dn8) * assign21750_e21991) - (assign21750_e21982 * ((locals.var_vp_i * locals.var_inv_phit_dn8) + (((locals.var_vpg_i * locals.var_qim_dn8) * locals.var_qim) + (assign21750_e21988 * locals.var_qim_dn8))))) / (assign21750_e21991 * assign21750_e21991)) / assign21750_e21993) * locals.var_r1) + (assign21750_e21994 * locals.var_r1_dn8));
        locals.var_dl_l_fact_dn9 = (((((((locals.var_xd_dn9 - locals.var_xdeff_dn9) * assign21750_e21991) - (assign21750_e21982 * ((locals.var_vp_i * locals.var_inv_phit_dn9) + (((locals.var_vpg_i * locals.var_qim_dn9) * locals.var_qim) + (assign21750_e21988 * locals.var_qim_dn9))))) / (assign21750_e21991 * assign21750_e21991)) / assign21750_e21993) * locals.var_r1) + (assign21750_e21994 * locals.var_r1_dn9));
        locals.var_dl_l_fact_rv = 0.0;

        let assign21760_e21999: f64 = (locals.var_alp_loc * locals.var_dl_l_fact);
        locals.var_dl_l = assign21760_e21999;
        locals.var_dl_l_dn4 = (locals.var_alp_loc * locals.var_dl_l_fact_dn4);
        locals.var_dl_l_dn6 = (locals.var_alp_loc * locals.var_dl_l_fact_dn6);
        locals.var_dl_l_dn7 = (locals.var_alp_loc * locals.var_dl_l_fact_dn7);
        locals.var_dl_l_dn8 = (locals.var_alp_loc * locals.var_dl_l_fact_dn8);
        locals.var_dl_l_dn9 = (locals.var_alp_loc * locals.var_dl_l_fact_dn9);
        locals.var_dl_l_rv = 0.0;

        let assign21770_e22005: f64 = (1.0 + locals.var_dl_l);
        let assign21770_e22006: f64 = (locals.var_dl_l * assign21770_e22005);
        let assign21770_e22007: f64 = (1.0 + assign21770_e22006);
        let assign21770_e22008: f64 = (1.0 / assign21770_e22007);
        locals.var_gdl = assign21770_e22008;
        locals.var_gdl_dn4 = (-(((locals.var_dl_l_dn4 * assign21770_e22005) + (locals.var_dl_l * locals.var_dl_l_dn4)) / (assign21770_e22007 * assign21770_e22007)));
        locals.var_gdl_dn6 = (-(((locals.var_dl_l_dn6 * assign21770_e22005) + (locals.var_dl_l * locals.var_dl_l_dn6)) / (assign21770_e22007 * assign21770_e22007)));
        locals.var_gdl_dn7 = (-(((locals.var_dl_l_dn7 * assign21770_e22005) + (locals.var_dl_l * locals.var_dl_l_dn7)) / (assign21770_e22007 * assign21770_e22007)));
        locals.var_gdl_dn8 = (-(((locals.var_dl_l_dn8 * assign21770_e22005) + (locals.var_dl_l * locals.var_dl_l_dn8)) / (assign21770_e22007 * assign21770_e22007)));
        locals.var_gdl_dn9 = (-(((locals.var_dl_l_dn9 * assign21770_e22005) + (locals.var_dl_l * locals.var_dl_l_dn9)) / (assign21770_e22007 * assign21770_e22007)));
        locals.var_gdl_rv = 0.0;

        let assign21780_e22011: f64 = (100.0 * locals.var_esurf1);
        let assign21780_e22014: f64 = (100.0 + locals.var_esurf1);
        let assign21780_e22015: f64 = (assign21780_e22011 / assign21780_e22014);
        locals.var_wsat1 = assign21780_e22015;
        locals.var_wsat1_dn4 = ((((100.0 * locals.var_esurf1_dn4) * assign21780_e22014) - (assign21780_e22011 * locals.var_esurf1_dn4)) / (assign21780_e22014 * assign21780_e22014));
        locals.var_wsat1_dn6 = ((((100.0 * locals.var_esurf1_dn6) * assign21780_e22014) - (assign21780_e22011 * locals.var_esurf1_dn6)) / (assign21780_e22014 * assign21780_e22014));
        locals.var_wsat1_dn7 = ((((100.0 * locals.var_esurf1_dn7) * assign21780_e22014) - (assign21780_e22011 * locals.var_esurf1_dn7)) / (assign21780_e22014 * assign21780_e22014));
        locals.var_wsat1_dn8 = ((((100.0 * locals.var_esurf1_dn8) * assign21780_e22014) - (assign21780_e22011 * locals.var_esurf1_dn8)) / (assign21780_e22014 * assign21780_e22014));
        locals.var_wsat1_dn9 = ((((100.0 * locals.var_esurf1_dn9) * assign21780_e22014) - (assign21780_e22011 * locals.var_esurf1_dn9)) / (assign21780_e22014 * assign21780_e22014));
        locals.var_wsat1_rv = 0.0;

        let assign21790_e22018: f64 = if locals.var_thesat1_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard675 = assign21790_e22018;
        locals.var_guard675_rv = 0.0;

        let (assign21800_e22028, assign21800_e22028_d_n4, assign21800_e22028_d_n6, assign21800_e22028_d_n7, assign21800_e22028_d_n8, assign21800_e22028_d_n9,) = {
    if (locals.var_guard675 != 0.0) {
        let assign21800_e22024: f64 = (locals.var_thesat1_i * locals.var_wsat1);
        let assign21800_e22025: f64 = (1.0 - assign21800_e22024);
        let assign21800_e22026: f64 = (1.0 / assign21800_e22025);
        (assign21800_e22026, (-((-(locals.var_thesat1_i * locals.var_wsat1_dn4)) / (assign21800_e22025 * assign21800_e22025))), (-((-(locals.var_thesat1_i * locals.var_wsat1_dn6)) / (assign21800_e22025 * assign21800_e22025))), (-((-(locals.var_thesat1_i * locals.var_wsat1_dn7)) / (assign21800_e22025 * assign21800_e22025))), (-((-(locals.var_thesat1_i * locals.var_wsat1_dn8)) / (assign21800_e22025 * assign21800_e22025))), (-((-(locals.var_thesat1_i * locals.var_wsat1_dn9)) / (assign21800_e22025 * assign21800_e22025))),)
    } else {
        (locals.var_sat_fact1, locals.var_sat_fact1_dn4, locals.var_sat_fact1_dn6, locals.var_sat_fact1_dn7, locals.var_sat_fact1_dn8, locals.var_sat_fact1_dn9,)
    }
};
        locals.var_sat_fact1 = assign21800_e22028;
        locals.var_sat_fact1_dn4 = assign21800_e22028_d_n4;
        locals.var_sat_fact1_dn6 = assign21800_e22028_d_n6;
        locals.var_sat_fact1_dn7 = assign21800_e22028_d_n7;
        locals.var_sat_fact1_dn8 = assign21800_e22028_d_n8;
        locals.var_sat_fact1_dn9 = assign21800_e22028_d_n9;
        locals.var_sat_fact1_rv = 0.0;

        let (assign21810_e22037, assign21810_e22037_d_n4, assign21810_e22037_d_n6, assign21810_e22037_d_n7, assign21810_e22037_d_n8, assign21810_e22037_d_n9,) = {
    if (locals.var_guard675 == 0.0) {
        let assign21810_e22034: f64 = (locals.var_thesat1_i * locals.var_wsat1);
        let assign21810_e22035: f64 = (1.0 + assign21810_e22034);
        (assign21810_e22035, (locals.var_thesat1_i * locals.var_wsat1_dn4), (locals.var_thesat1_i * locals.var_wsat1_dn6), (locals.var_thesat1_i * locals.var_wsat1_dn7), (locals.var_thesat1_i * locals.var_wsat1_dn8), (locals.var_thesat1_i * locals.var_wsat1_dn9),)
    } else {
        (locals.var_sat_fact1, locals.var_sat_fact1_dn4, locals.var_sat_fact1_dn6, locals.var_sat_fact1_dn7, locals.var_sat_fact1_dn8, locals.var_sat_fact1_dn9,)
    }
};
        locals.var_sat_fact1 = assign21810_e22037;
        locals.var_sat_fact1_dn4 = assign21810_e22037_d_n4;
        locals.var_sat_fact1_dn6 = assign21810_e22037_d_n6;
        locals.var_sat_fact1_dn7 = assign21810_e22037_d_n7;
        locals.var_sat_fact1_dn8 = assign21810_e22037_d_n8;
        locals.var_sat_fact1_dn9 = assign21810_e22037_d_n9;
        locals.var_sat_fact1_rv = 0.0;

        let assign21820_e22040: f64 = (100.0 * locals.var_esurf2);
        let assign21820_e22043: f64 = (100.0 + locals.var_esurf2);
        let assign21820_e22044: f64 = (assign21820_e22040 / assign21820_e22043);
        locals.var_wsat2 = assign21820_e22044;
        locals.var_wsat2_dn4 = ((((100.0 * locals.var_esurf2_dn4) * assign21820_e22043) - (assign21820_e22040 * locals.var_esurf2_dn4)) / (assign21820_e22043 * assign21820_e22043));
        locals.var_wsat2_dn6 = ((((100.0 * locals.var_esurf2_dn6) * assign21820_e22043) - (assign21820_e22040 * locals.var_esurf2_dn6)) / (assign21820_e22043 * assign21820_e22043));
        locals.var_wsat2_dn7 = ((((100.0 * locals.var_esurf2_dn7) * assign21820_e22043) - (assign21820_e22040 * locals.var_esurf2_dn7)) / (assign21820_e22043 * assign21820_e22043));
        locals.var_wsat2_dn8 = ((((100.0 * locals.var_esurf2_dn8) * assign21820_e22043) - (assign21820_e22040 * locals.var_esurf2_dn8)) / (assign21820_e22043 * assign21820_e22043));
        locals.var_wsat2_dn9 = ((((100.0 * locals.var_esurf2_dn9) * assign21820_e22043) - (assign21820_e22040 * locals.var_esurf2_dn9)) / (assign21820_e22043 * assign21820_e22043));
        locals.var_wsat2_rv = 0.0;

        let assign21830_e22047: f64 = if locals.var_thesat2_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard676 = assign21830_e22047;
        locals.var_guard676_rv = 0.0;

        let (assign21840_e22057, assign21840_e22057_d_n4, assign21840_e22057_d_n6, assign21840_e22057_d_n7, assign21840_e22057_d_n8, assign21840_e22057_d_n9,) = {
    if (locals.var_guard676 != 0.0) {
        let assign21840_e22053: f64 = (locals.var_thesat2_i * locals.var_wsat2);
        let assign21840_e22054: f64 = (1.0 - assign21840_e22053);
        let assign21840_e22055: f64 = (1.0 / assign21840_e22054);
        (assign21840_e22055, (-((-(locals.var_thesat2_i * locals.var_wsat2_dn4)) / (assign21840_e22054 * assign21840_e22054))), (-((-(locals.var_thesat2_i * locals.var_wsat2_dn6)) / (assign21840_e22054 * assign21840_e22054))), (-((-(locals.var_thesat2_i * locals.var_wsat2_dn7)) / (assign21840_e22054 * assign21840_e22054))), (-((-(locals.var_thesat2_i * locals.var_wsat2_dn8)) / (assign21840_e22054 * assign21840_e22054))), (-((-(locals.var_thesat2_i * locals.var_wsat2_dn9)) / (assign21840_e22054 * assign21840_e22054))),)
    } else {
        (locals.var_sat_fact2, locals.var_sat_fact2_dn4, locals.var_sat_fact2_dn6, locals.var_sat_fact2_dn7, locals.var_sat_fact2_dn8, locals.var_sat_fact2_dn9,)
    }
};
        locals.var_sat_fact2 = assign21840_e22057;
        locals.var_sat_fact2_dn4 = assign21840_e22057_d_n4;
        locals.var_sat_fact2_dn6 = assign21840_e22057_d_n6;
        locals.var_sat_fact2_dn7 = assign21840_e22057_d_n7;
        locals.var_sat_fact2_dn8 = assign21840_e22057_d_n8;
        locals.var_sat_fact2_dn9 = assign21840_e22057_d_n9;
        locals.var_sat_fact2_rv = 0.0;

        let (assign21850_e22066, assign21850_e22066_d_n4, assign21850_e22066_d_n6, assign21850_e22066_d_n7, assign21850_e22066_d_n8, assign21850_e22066_d_n9,) = {
    if (locals.var_guard676 == 0.0) {
        let assign21850_e22063: f64 = (locals.var_thesat2_i * locals.var_wsat2);
        let assign21850_e22064: f64 = (1.0 + assign21850_e22063);
        (assign21850_e22064, (locals.var_thesat2_i * locals.var_wsat2_dn4), (locals.var_thesat2_i * locals.var_wsat2_dn6), (locals.var_thesat2_i * locals.var_wsat2_dn7), (locals.var_thesat2_i * locals.var_wsat2_dn8), (locals.var_thesat2_i * locals.var_wsat2_dn9),)
    } else {
        (locals.var_sat_fact2, locals.var_sat_fact2_dn4, locals.var_sat_fact2_dn6, locals.var_sat_fact2_dn7, locals.var_sat_fact2_dn8, locals.var_sat_fact2_dn9,)
    }
};
        locals.var_sat_fact2 = assign21850_e22066;
        locals.var_sat_fact2_dn4 = assign21850_e22066_d_n4;
        locals.var_sat_fact2_dn6 = assign21850_e22066_d_n6;
        locals.var_sat_fact2_dn7 = assign21850_e22066_d_n7;
        locals.var_sat_fact2_dn8 = assign21850_e22066_d_n8;
        locals.var_sat_fact2_dn9 = assign21850_e22066_d_n9;
        locals.var_sat_fact2_rv = 0.0;

        let assign21860_e22069: f64 = (locals.var_sat_phit_loc * locals.var_dxdrift);
        let assign21860_e22071: f64 = (assign21860_e22069 * 0.5);
        let assign21860_e22074: f64 = (locals.var_sat_fact1 + locals.var_sat_fact2);
        let assign21860_e22075: f64 = (assign21860_e22071 * assign21860_e22074);
        locals.var_ggamma = assign21860_e22075;
        locals.var_ggamma_dn4 = (((((locals.var_sat_phit_loc_dn4 * locals.var_dxdrift) + (locals.var_sat_phit_loc * locals.var_dxdrift_dn4)) * 0.5) * assign21860_e22074) + (assign21860_e22071 * (locals.var_sat_fact1_dn4 + locals.var_sat_fact2_dn4)));
        locals.var_ggamma_dn6 = (((((locals.var_sat_phit_loc_dn6 * locals.var_dxdrift) + (locals.var_sat_phit_loc * locals.var_dxdrift_dn6)) * 0.5) * assign21860_e22074) + (assign21860_e22071 * (locals.var_sat_fact1_dn6 + locals.var_sat_fact2_dn6)));
        locals.var_ggamma_dn7 = (((((locals.var_sat_phit_loc_dn7 * locals.var_dxdrift) + (locals.var_sat_phit_loc * locals.var_dxdrift_dn7)) * 0.5) * assign21860_e22074) + (assign21860_e22071 * (locals.var_sat_fact1_dn7 + locals.var_sat_fact2_dn7)));
        locals.var_ggamma_dn8 = (((((locals.var_sat_phit_loc_dn8 * locals.var_dxdrift) + (locals.var_sat_phit_loc * locals.var_dxdrift_dn8)) * 0.5) * assign21860_e22074) + (assign21860_e22071 * (locals.var_sat_fact1_dn8 + locals.var_sat_fact2_dn8)));
        locals.var_ggamma_dn9 = (((((locals.var_sat_phit_loc_dn9 * locals.var_dxdrift) + (locals.var_sat_phit_loc * locals.var_dxdrift_dn9)) * 0.5) * assign21860_e22074) + (assign21860_e22071 * (locals.var_sat_fact1_dn9 + locals.var_sat_fact2_dn9)));
        locals.var_ggamma_rv = 0.0;

        let assign21870_e22079: f64 = (locals.var_gmob * locals.var_gdl);
        let assign21870_e22080: f64 = (locals.var_ggamma / assign21870_e22079);
        locals.var_sqrt_zsat = assign21870_e22080;
        locals.var_sqrt_zsat_dn4 = (((locals.var_ggamma_dn4 * assign21870_e22079) - (locals.var_ggamma * ((locals.var_gmob_dn4 * locals.var_gdl) + (locals.var_gmob * locals.var_gdl_dn4)))) / (assign21870_e22079 * assign21870_e22079));
        locals.var_sqrt_zsat_dn6 = (((locals.var_ggamma_dn6 * assign21870_e22079) - (locals.var_ggamma * ((locals.var_gmob_dn6 * locals.var_gdl) + (locals.var_gmob * locals.var_gdl_dn6)))) / (assign21870_e22079 * assign21870_e22079));
        locals.var_sqrt_zsat_dn7 = (((locals.var_ggamma_dn7 * assign21870_e22079) - (locals.var_ggamma * ((locals.var_gmob_dn7 * locals.var_gdl) + (locals.var_gmob * locals.var_gdl_dn7)))) / (assign21870_e22079 * assign21870_e22079));
        locals.var_sqrt_zsat_dn8 = (((locals.var_ggamma_dn8 * assign21870_e22079) - (locals.var_ggamma * ((locals.var_gmob_dn8 * locals.var_gdl) + (locals.var_gmob * locals.var_gdl_dn8)))) / (assign21870_e22079 * assign21870_e22079));
        locals.var_sqrt_zsat_dn9 = (((locals.var_ggamma_dn9 * assign21870_e22079) - (locals.var_ggamma * ((locals.var_gmob_dn9 * locals.var_gdl) + (locals.var_gmob * locals.var_gdl_dn9)))) / (assign21870_e22079 * assign21870_e22079));
        locals.var_sqrt_zsat_rv = 0.0;

        let assign21880_e22083: f64 = (locals.var_sqrt_zsat * locals.var_sqrt_zsat);
        locals.var_zsat = assign21880_e22083;
        locals.var_zsat_dn4 = ((locals.var_sqrt_zsat_dn4 * locals.var_sqrt_zsat) + (locals.var_sqrt_zsat * locals.var_sqrt_zsat_dn4));
        locals.var_zsat_dn6 = ((locals.var_sqrt_zsat_dn6 * locals.var_sqrt_zsat) + (locals.var_sqrt_zsat * locals.var_sqrt_zsat_dn6));
        locals.var_zsat_dn7 = ((locals.var_sqrt_zsat_dn7 * locals.var_sqrt_zsat) + (locals.var_sqrt_zsat * locals.var_sqrt_zsat_dn7));
        locals.var_zsat_dn8 = ((locals.var_sqrt_zsat_dn8 * locals.var_sqrt_zsat) + (locals.var_sqrt_zsat * locals.var_sqrt_zsat_dn8));
        locals.var_zsat_dn9 = ((locals.var_sqrt_zsat_dn9 * locals.var_sqrt_zsat) + (locals.var_sqrt_zsat * locals.var_sqrt_zsat_dn9));
        locals.var_zsat_rv = 0.0;

        let assign21890_e22086: f64 = (1.0 + locals.var_zsat);
        let assign21890_e22087: f64 = (assign21890_e22086).sqrt();
        locals.var_vsat_fact = assign21890_e22087;
        locals.var_vsat_fact_dn4 = (locals.var_zsat_dn4 / (2.0 * assign21890_e22087));
        locals.var_vsat_fact_dn6 = (locals.var_zsat_dn6 / (2.0 * assign21890_e22087));
        locals.var_vsat_fact_dn7 = (locals.var_zsat_dn7 / (2.0 * assign21890_e22087));
        locals.var_vsat_fact_dn8 = (locals.var_zsat_dn8 / (2.0 * assign21890_e22087));
        locals.var_vsat_fact_dn9 = (locals.var_zsat_dn9 / (2.0 * assign21890_e22087));
        locals.var_vsat_fact_rv = 0.0;

        let assign21900_e22091: f64 = (1.5 * locals.var_zsat);
        let assign21900_e22092: f64 = (1.0 + assign21900_e22091);
        let assign21900_e22094: f64 = (assign21900_e22092 / locals.var_vsat_fact);
        locals.var_hsat = assign21900_e22094;
        locals.var_hsat_dn4 = ((((1.5 * locals.var_zsat_dn4) * locals.var_vsat_fact) - (assign21900_e22092 * locals.var_vsat_fact_dn4)) / (locals.var_vsat_fact * locals.var_vsat_fact));
        locals.var_hsat_dn6 = ((((1.5 * locals.var_zsat_dn6) * locals.var_vsat_fact) - (assign21900_e22092 * locals.var_vsat_fact_dn6)) / (locals.var_vsat_fact * locals.var_vsat_fact));
        locals.var_hsat_dn7 = ((((1.5 * locals.var_zsat_dn7) * locals.var_vsat_fact) - (assign21900_e22092 * locals.var_vsat_fact_dn7)) / (locals.var_vsat_fact * locals.var_vsat_fact));
        locals.var_hsat_dn8 = ((((1.5 * locals.var_zsat_dn8) * locals.var_vsat_fact) - (assign21900_e22092 * locals.var_vsat_fact_dn8)) / (locals.var_vsat_fact * locals.var_vsat_fact));
        locals.var_hsat_dn9 = ((((1.5 * locals.var_zsat_dn9) * locals.var_vsat_fact) - (assign21900_e22092 * locals.var_vsat_fact_dn9)) / (locals.var_vsat_fact * locals.var_vsat_fact));
        locals.var_hsat_rv = 0.0;

        let assign21910_e22097: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard677 = assign21910_e22097;
        locals.var_guard677_rv = 0.0;

        let (assign21920_e22114, assign21920_e22114_d_n4, assign21920_e22114_d_n6, assign21920_e22114_d_n7, assign21920_e22114_d_n8, assign21920_e22114_d_n9,) = {
    if (locals.var_guard677 != 0.0) {
        let assign21920_e22101: f64 = (0.6 * locals.var_qq);
        let assign21920_e22103: f64 = (-0.1666666666667);
        let assign21920_e22106: f64 = (locals.var_esurf1 * locals.var_esurf1);
        let assign21920_e22108: f64 = (assign21920_e22106 + 60.0);
        let assign21920_e22109: f64 = (assign21920_e22108).ln();
        let assign21920_e22110: f64 = (assign21920_e22103 * assign21920_e22109);
        let assign21920_e22111: f64 = (assign21920_e22110).exp();
        let assign21920_e22112: f64 = (assign21920_e22101 * assign21920_e22111);
        (assign21920_e22112, (((0.6 * locals.var_qq_dn4) * assign21920_e22111) + (assign21920_e22101 * (assign21920_e22111 * (assign21920_e22103 * (((locals.var_esurf1_dn4 * locals.var_esurf1) + (locals.var_esurf1 * locals.var_esurf1_dn4)) / assign21920_e22108))))), (((0.6 * locals.var_qq_dn6) * assign21920_e22111) + (assign21920_e22101 * (assign21920_e22111 * (assign21920_e22103 * (((locals.var_esurf1_dn6 * locals.var_esurf1) + (locals.var_esurf1 * locals.var_esurf1_dn6)) / assign21920_e22108))))), (((0.6 * locals.var_qq_dn7) * assign21920_e22111) + (assign21920_e22101 * (assign21920_e22111 * (assign21920_e22103 * (((locals.var_esurf1_dn7 * locals.var_esurf1) + (locals.var_esurf1 * locals.var_esurf1_dn7)) / assign21920_e22108))))), (((0.6 * locals.var_qq_dn8) * assign21920_e22111) + (assign21920_e22101 * (assign21920_e22111 * (assign21920_e22103 * (((locals.var_esurf1_dn8 * locals.var_esurf1) + (locals.var_esurf1 * locals.var_esurf1_dn8)) / assign21920_e22108))))), (((0.6 * locals.var_qq_dn9) * assign21920_e22111) + (assign21920_e22101 * (assign21920_e22111 * (assign21920_e22103 * (((locals.var_esurf1_dn9 * locals.var_esurf1) + (locals.var_esurf1 * locals.var_esurf1_dn9)) / assign21920_e22108))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign21920_e22114;
        locals.var_temp1_dn4 = assign21920_e22114_d_n4;
        locals.var_temp1_dn6 = assign21920_e22114_d_n6;
        locals.var_temp1_dn7 = assign21920_e22114_d_n7;
        locals.var_temp1_dn8 = assign21920_e22114_d_n8;
        locals.var_temp1_dn9 = assign21920_e22114_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign21930_e22131, assign21930_e22131_d_n4, assign21930_e22131_d_n6, assign21930_e22131_d_n7, assign21930_e22131_d_n8, assign21930_e22131_d_n9,) = {
    if (locals.var_guard677 != 0.0) {
        let assign21930_e22118: f64 = (0.6 * locals.var_qq);
        let assign21930_e22120: f64 = (-0.1666666666667);
        let assign21930_e22123: f64 = (locals.var_esurf2 * locals.var_esurf2);
        let assign21930_e22125: f64 = (assign21930_e22123 + 60.0);
        let assign21930_e22126: f64 = (assign21930_e22125).ln();
        let assign21930_e22127: f64 = (assign21930_e22120 * assign21930_e22126);
        let assign21930_e22128: f64 = (assign21930_e22127).exp();
        let assign21930_e22129: f64 = (assign21930_e22118 * assign21930_e22128);
        (assign21930_e22129, (((0.6 * locals.var_qq_dn4) * assign21930_e22128) + (assign21930_e22118 * (assign21930_e22128 * (assign21930_e22120 * (((locals.var_esurf2_dn4 * locals.var_esurf2) + (locals.var_esurf2 * locals.var_esurf2_dn4)) / assign21930_e22125))))), (((0.6 * locals.var_qq_dn6) * assign21930_e22128) + (assign21930_e22118 * (assign21930_e22128 * (assign21930_e22120 * (((locals.var_esurf2_dn6 * locals.var_esurf2) + (locals.var_esurf2 * locals.var_esurf2_dn6)) / assign21930_e22125))))), (((0.6 * locals.var_qq_dn7) * assign21930_e22128) + (assign21930_e22118 * (assign21930_e22128 * (assign21930_e22120 * (((locals.var_esurf2_dn7 * locals.var_esurf2) + (locals.var_esurf2 * locals.var_esurf2_dn7)) / assign21930_e22125))))), (((0.6 * locals.var_qq_dn8) * assign21930_e22128) + (assign21930_e22118 * (assign21930_e22128 * (assign21930_e22120 * (((locals.var_esurf2_dn8 * locals.var_esurf2) + (locals.var_esurf2 * locals.var_esurf2_dn8)) / assign21930_e22125))))), (((0.6 * locals.var_qq_dn9) * assign21930_e22128) + (assign21930_e22118 * (assign21930_e22128 * (assign21930_e22120 * (((locals.var_esurf2_dn9 * locals.var_esurf2) + (locals.var_esurf2 * locals.var_esurf2_dn9)) / assign21930_e22125))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign21930_e22131;
        locals.var_temp2_dn4 = assign21930_e22131_d_n4;
        locals.var_temp2_dn6 = assign21930_e22131_d_n6;
        locals.var_temp2_dn7 = assign21930_e22131_d_n7;
        locals.var_temp2_dn8 = assign21930_e22131_d_n8;
        locals.var_temp2_dn9 = assign21930_e22131_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign21940_e22141, assign21940_e22141_d_n4, assign21940_e22141_d_n6, assign21940_e22141_d_n7, assign21940_e22141_d_n8, assign21940_e22141_d_n9,) = {
    if (locals.var_guard677 != 0.0) {
        let assign21940_e22136: f64 = (locals.var_k1 * locals.var_temp1);
        let assign21940_e22137: f64 = (1.0 + assign21940_e22136);
        let assign21940_e22139: f64 = (assign21940_e22137 / locals.var_tox1fact);
        (assign21940_e22139, (((((locals.var_k1_dn4 * locals.var_temp1) + (locals.var_k1 * locals.var_temp1_dn4)) * locals.var_tox1fact) - (assign21940_e22137 * locals.var_tox1fact_dn4)) / (locals.var_tox1fact * locals.var_tox1fact)), (((((locals.var_k1_dn6 * locals.var_temp1) + (locals.var_k1 * locals.var_temp1_dn6)) * locals.var_tox1fact) - (assign21940_e22137 * locals.var_tox1fact_dn6)) / (locals.var_tox1fact * locals.var_tox1fact)), (((((locals.var_k1_dn7 * locals.var_temp1) + (locals.var_k1 * locals.var_temp1_dn7)) * locals.var_tox1fact) - (assign21940_e22137 * locals.var_tox1fact_dn7)) / (locals.var_tox1fact * locals.var_tox1fact)), (((((locals.var_k1_dn8 * locals.var_temp1) + (locals.var_k1 * locals.var_temp1_dn8)) * locals.var_tox1fact) - (assign21940_e22137 * locals.var_tox1fact_dn8)) / (locals.var_tox1fact * locals.var_tox1fact)), (((((locals.var_k1_dn9 * locals.var_temp1) + (locals.var_k1 * locals.var_temp1_dn9)) * locals.var_tox1fact) - (assign21940_e22137 * locals.var_tox1fact_dn9)) / (locals.var_tox1fact * locals.var_tox1fact)),)
    } else {
        (locals.var_qmfact1, locals.var_qmfact1_dn4, locals.var_qmfact1_dn6, locals.var_qmfact1_dn7, locals.var_qmfact1_dn8, locals.var_qmfact1_dn9,)
    }
};
        locals.var_qmfact1 = assign21940_e22141;
        locals.var_qmfact1_dn4 = assign21940_e22141_d_n4;
        locals.var_qmfact1_dn6 = assign21940_e22141_d_n6;
        locals.var_qmfact1_dn7 = assign21940_e22141_d_n7;
        locals.var_qmfact1_dn8 = assign21940_e22141_d_n8;
        locals.var_qmfact1_dn9 = assign21940_e22141_d_n9;
        locals.var_qmfact1_rv = 0.0;

        let (assign21950_e22151, assign21950_e22151_d_n4, assign21950_e22151_d_n6, assign21950_e22151_d_n7, assign21950_e22151_d_n8, assign21950_e22151_d_n9,) = {
    if (locals.var_guard677 != 0.0) {
        let assign21950_e22146: f64 = (locals.var_k2 * locals.var_temp2);
        let assign21950_e22147: f64 = (1.0 + assign21950_e22146);
        let assign21950_e22149: f64 = (assign21950_e22147 / locals.var_tox2fact);
        (assign21950_e22149, (((((locals.var_k2_dn4 * locals.var_temp2) + (locals.var_k2 * locals.var_temp2_dn4)) * locals.var_tox2fact) - (assign21950_e22147 * locals.var_tox2fact_dn4)) / (locals.var_tox2fact * locals.var_tox2fact)), (((((locals.var_k2_dn6 * locals.var_temp2) + (locals.var_k2 * locals.var_temp2_dn6)) * locals.var_tox2fact) - (assign21950_e22147 * locals.var_tox2fact_dn6)) / (locals.var_tox2fact * locals.var_tox2fact)), (((((locals.var_k2_dn7 * locals.var_temp2) + (locals.var_k2 * locals.var_temp2_dn7)) * locals.var_tox2fact) - (assign21950_e22147 * locals.var_tox2fact_dn7)) / (locals.var_tox2fact * locals.var_tox2fact)), (((((locals.var_k2_dn8 * locals.var_temp2) + (locals.var_k2 * locals.var_temp2_dn8)) * locals.var_tox2fact) - (assign21950_e22147 * locals.var_tox2fact_dn8)) / (locals.var_tox2fact * locals.var_tox2fact)), (((((locals.var_k2_dn9 * locals.var_temp2) + (locals.var_k2 * locals.var_temp2_dn9)) * locals.var_tox2fact) - (assign21950_e22147 * locals.var_tox2fact_dn9)) / (locals.var_tox2fact * locals.var_tox2fact)),)
    } else {
        (locals.var_qmfact2, locals.var_qmfact2_dn4, locals.var_qmfact2_dn6, locals.var_qmfact2_dn7, locals.var_qmfact2_dn8, locals.var_qmfact2_dn9,)
    }
};
        locals.var_qmfact2 = assign21950_e22151;
        locals.var_qmfact2_dn4 = assign21950_e22151_d_n4;
        locals.var_qmfact2_dn6 = assign21950_e22151_d_n6;
        locals.var_qmfact2_dn7 = assign21950_e22151_d_n7;
        locals.var_qmfact2_dn8 = assign21950_e22151_d_n8;
        locals.var_qmfact2_dn9 = assign21950_e22151_d_n9;
        locals.var_qmfact2_rv = 0.0;

        let (assign21960_e22156, assign21960_e22156_d_n4, assign21960_e22156_d_n6, assign21960_e22156_d_n7, assign21960_e22156_d_n8, assign21960_e22156_d_n9,) = {
    if (locals.var_guard677 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qmfact1, locals.var_qmfact1_dn4, locals.var_qmfact1_dn6, locals.var_qmfact1_dn7, locals.var_qmfact1_dn8, locals.var_qmfact1_dn9,)
    }
};
        locals.var_qmfact1 = assign21960_e22156;
        locals.var_qmfact1_dn4 = assign21960_e22156_d_n4;
        locals.var_qmfact1_dn6 = assign21960_e22156_d_n6;
        locals.var_qmfact1_dn7 = assign21960_e22156_d_n7;
        locals.var_qmfact1_dn8 = assign21960_e22156_d_n8;
        locals.var_qmfact1_dn9 = assign21960_e22156_d_n9;
        locals.var_qmfact1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_60(
        locals: &mut StampLocals,
    ) {
        let (assign21970_e22161, assign21970_e22161_d_n4, assign21970_e22161_d_n6, assign21970_e22161_d_n7, assign21970_e22161_d_n8, assign21970_e22161_d_n9,) = {
    if (locals.var_guard677 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qmfact2, locals.var_qmfact2_dn4, locals.var_qmfact2_dn6, locals.var_qmfact2_dn7, locals.var_qmfact2_dn8, locals.var_qmfact2_dn9,)
    }
};
        locals.var_qmfact2 = assign21970_e22161;
        locals.var_qmfact2_dn4 = assign21970_e22161_d_n4;
        locals.var_qmfact2_dn6 = assign21970_e22161_d_n6;
        locals.var_qmfact2_dn7 = assign21970_e22161_d_n7;
        locals.var_qmfact2_dn8 = assign21970_e22161_d_n8;
        locals.var_qmfact2_dn9 = assign21970_e22161_d_n9;
        locals.var_qmfact2_rv = 0.0;

        let assign21980_e22164: f64 = if locals.var_qis > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard678 = assign21980_e22164;
        locals.var_guard678_rv = 0.0;

        let assign21990_e22167: f64 = if locals.var_qid > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard679 = assign21990_e22167;
        locals.var_guard679_rv = 0.0;

        let assign22000_e22169: f64 = (locals.var_a2d).abs();
        let assign22000_e22171: f64 = if assign22000_e22169 < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard680 = assign22000_e22171;
        locals.var_guard680_rv = 0.0;

        let (assign22010_e22191, assign22010_e22191_d_n4, assign22010_e22191_d_n6, assign22010_e22191_d_n7, assign22010_e22191_d_n8, assign22010_e22191_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign22010_e22179: f64 = (2.0 + locals.var_q1d);
        let assign22010_e22182: f64 = (0.5 * locals.var_a1d);
        let assign22010_e22183: f64 = (assign22010_e22179 + assign22010_e22182);
        let assign22010_e22186: f64 = (2.0 + locals.var_q2d);
        let assign22010_e22188: f64 = (assign22010_e22186 * locals.var_a1d);
        let assign22010_e22189: f64 = (assign22010_e22183 / assign22010_e22188);
        (assign22010_e22189, ((((locals.var_q1d_dn4 + (0.5 * locals.var_a1d_dn4)) * assign22010_e22188) - (assign22010_e22183 * ((locals.var_q2d_dn4 * locals.var_a1d) + (assign22010_e22186 * locals.var_a1d_dn4)))) / (assign22010_e22188 * assign22010_e22188)), ((((locals.var_q1d_dn6 + (0.5 * locals.var_a1d_dn6)) * assign22010_e22188) - (assign22010_e22183 * ((locals.var_q2d_dn6 * locals.var_a1d) + (assign22010_e22186 * locals.var_a1d_dn6)))) / (assign22010_e22188 * assign22010_e22188)), ((((locals.var_q1d_dn7 + (0.5 * locals.var_a1d_dn7)) * assign22010_e22188) - (assign22010_e22183 * ((locals.var_q2d_dn7 * locals.var_a1d) + (assign22010_e22186 * locals.var_a1d_dn7)))) / (assign22010_e22188 * assign22010_e22188)), ((((locals.var_q1d_dn8 + (0.5 * locals.var_a1d_dn8)) * assign22010_e22188) - (assign22010_e22183 * ((locals.var_q2d_dn8 * locals.var_a1d) + (assign22010_e22186 * locals.var_a1d_dn8)))) / (assign22010_e22188 * assign22010_e22188)), ((((locals.var_q1d_dn9 + (0.5 * locals.var_a1d_dn9)) * assign22010_e22188) - (assign22010_e22183 * ((locals.var_q2d_dn9 * locals.var_a1d) + (assign22010_e22186 * locals.var_a1d_dn9)))) / (assign22010_e22188 * assign22010_e22188)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign22010_e22191;
        locals.var_temp_dn4 = assign22010_e22191_d_n4;
        locals.var_temp_dn6 = assign22010_e22191_d_n6;
        locals.var_temp_dn7 = assign22010_e22191_d_n7;
        locals.var_temp_dn8 = assign22010_e22191_d_n8;
        locals.var_temp_dn9 = assign22010_e22191_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign22020_e22201, assign22020_e22201_d_n4, assign22020_e22201_d_n6, assign22020_e22201_d_n7, assign22020_e22201_d_n8, assign22020_e22201_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign22020_e22199: f64 = (locals.var_temp * locals.var_a2d);
        (assign22020_e22199, ((locals.var_temp_dn4 * locals.var_a2d) + (locals.var_temp * locals.var_a2d_dn4)), ((locals.var_temp_dn6 * locals.var_a2d) + (locals.var_temp * locals.var_a2d_dn6)), ((locals.var_temp_dn7 * locals.var_a2d) + (locals.var_temp * locals.var_a2d_dn7)), ((locals.var_temp_dn8 * locals.var_a2d) + (locals.var_temp * locals.var_a2d_dn8)), ((locals.var_temp_dn9 * locals.var_a2d) + (locals.var_temp * locals.var_a2d_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign22020_e22201;
        locals.var_temp1_dn4 = assign22020_e22201_d_n4;
        locals.var_temp1_dn6 = assign22020_e22201_d_n6;
        locals.var_temp1_dn7 = assign22020_e22201_d_n7;
        locals.var_temp1_dn8 = assign22020_e22201_d_n8;
        locals.var_temp1_dn9 = assign22020_e22201_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign22030_e22211, assign22030_e22211_d_n4, assign22030_e22211_d_n6, assign22030_e22211_d_n7, assign22030_e22211_d_n8, assign22030_e22211_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign22030_e22209: f64 = (locals.var_temp1 * locals.var_temp1);
        (assign22030_e22209, ((locals.var_temp1_dn4 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn4)), ((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)), ((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)), ((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)), ((locals.var_temp1_dn9 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign22030_e22211;
        locals.var_temp2_dn4 = assign22030_e22211_d_n4;
        locals.var_temp2_dn6 = assign22030_e22211_d_n6;
        locals.var_temp2_dn7 = assign22030_e22211_d_n7;
        locals.var_temp2_dn8 = assign22030_e22211_d_n8;
        locals.var_temp2_dn9 = assign22030_e22211_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign22040_e22223, assign22040_e22223_d_n4, assign22040_e22223_d_n6, assign22040_e22223_d_n7, assign22040_e22223_d_n8, assign22040_e22223_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign22040_e22219: f64 = (1.0 - locals.var_temp1);
        let assign22040_e22221: f64 = (assign22040_e22219 + locals.var_temp2);
        (assign22040_e22221, ((-locals.var_temp1_dn4) + locals.var_temp2_dn4), ((-locals.var_temp1_dn6) + locals.var_temp2_dn6), ((-locals.var_temp1_dn7) + locals.var_temp2_dn7), ((-locals.var_temp1_dn8) + locals.var_temp2_dn8), ((-locals.var_temp1_dn9) + locals.var_temp2_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign22040_e22223;
        locals.var_temp3_dn4 = assign22040_e22223_d_n4;
        locals.var_temp3_dn6 = assign22040_e22223_d_n6;
        locals.var_temp3_dn7 = assign22040_e22223_d_n7;
        locals.var_temp3_dn8 = assign22040_e22223_d_n8;
        locals.var_temp3_dn9 = assign22040_e22223_d_n9;
        locals.var_temp3_rv = 0.0;

        let (assign22050_e22235, assign22050_e22235_d_n4, assign22050_e22235_d_n6, assign22050_e22235_d_n7, assign22050_e22235_d_n8, assign22050_e22235_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign22050_e22232: f64 = (locals.var_temp1 * locals.var_temp2);
        let assign22050_e22233: f64 = (locals.var_temp3 - assign22050_e22232);
        (assign22050_e22233, (locals.var_temp3_dn4 - ((locals.var_temp1_dn4 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn4))), (locals.var_temp3_dn6 - ((locals.var_temp1_dn6 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn6))), (locals.var_temp3_dn7 - ((locals.var_temp1_dn7 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn7))), (locals.var_temp3_dn8 - ((locals.var_temp1_dn8 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn8))), (locals.var_temp3_dn9 - ((locals.var_temp1_dn9 * locals.var_temp2) + (locals.var_temp1 * locals.var_temp2_dn9))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign22050_e22235;
        locals.var_temp4_dn4 = assign22050_e22235_d_n4;
        locals.var_temp4_dn6 = assign22050_e22235_d_n6;
        locals.var_temp4_dn7 = assign22050_e22235_d_n7;
        locals.var_temp4_dn8 = assign22050_e22235_d_n8;
        locals.var_temp4_dn9 = assign22050_e22235_d_n9;
        locals.var_temp4_rv = 0.0;

        let (assign22060_e22259, assign22060_e22259_d_n4, assign22060_e22259_d_n6, assign22060_e22259_d_n7, assign22060_e22259_d_n8, assign22060_e22259_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign22060_e22244: f64 = (2.0 * locals.var_qsqd);
        let assign22060_e22248: f64 = (1.0 / locals.var_a1d);
        let assign22060_e22249: f64 = (locals.var_temp - assign22060_e22248);
        let assign22060_e22250: f64 = (assign22060_e22244 * assign22060_e22249);
        let assign22060_e22252: f64 = (assign22060_e22250 * locals.var_temp4);
        let assign22060_e22253: f64 = (locals.var_k2q2d - assign22060_e22252);
        let assign22060_e22256: f64 = (2.0 + locals.var_q2d);
        let assign22060_e22257: f64 = (assign22060_e22253 / assign22060_e22256);
        (assign22060_e22257, ((((locals.var_k2q2d_dn4 - (((((2.0 * locals.var_qsqd_dn4) * assign22060_e22249) + (assign22060_e22244 * (locals.var_temp_dn4 - (-(locals.var_a1d_dn4 / (locals.var_a1d * locals.var_a1d)))))) * locals.var_temp4) + (assign22060_e22250 * locals.var_temp4_dn4))) * assign22060_e22256) - (assign22060_e22253 * locals.var_q2d_dn4)) / (assign22060_e22256 * assign22060_e22256)), ((((locals.var_k2q2d_dn6 - (((((2.0 * locals.var_qsqd_dn6) * assign22060_e22249) + (assign22060_e22244 * (locals.var_temp_dn6 - (-(locals.var_a1d_dn6 / (locals.var_a1d * locals.var_a1d)))))) * locals.var_temp4) + (assign22060_e22250 * locals.var_temp4_dn6))) * assign22060_e22256) - (assign22060_e22253 * locals.var_q2d_dn6)) / (assign22060_e22256 * assign22060_e22256)), ((((locals.var_k2q2d_dn7 - (((((2.0 * locals.var_qsqd_dn7) * assign22060_e22249) + (assign22060_e22244 * (locals.var_temp_dn7 - (-(locals.var_a1d_dn7 / (locals.var_a1d * locals.var_a1d)))))) * locals.var_temp4) + (assign22060_e22250 * locals.var_temp4_dn7))) * assign22060_e22256) - (assign22060_e22253 * locals.var_q2d_dn7)) / (assign22060_e22256 * assign22060_e22256)), ((((locals.var_k2q2d_dn8 - (((((2.0 * locals.var_qsqd_dn8) * assign22060_e22249) + (assign22060_e22244 * (locals.var_temp_dn8 - (-(locals.var_a1d_dn8 / (locals.var_a1d * locals.var_a1d)))))) * locals.var_temp4) + (assign22060_e22250 * locals.var_temp4_dn8))) * assign22060_e22256) - (assign22060_e22253 * locals.var_q2d_dn8)) / (assign22060_e22256 * assign22060_e22256)), ((((locals.var_k2q2d_dn9 - (((((2.0 * locals.var_qsqd_dn9) * assign22060_e22249) + (assign22060_e22244 * (locals.var_temp_dn9 - (-(locals.var_a1d_dn9 / (locals.var_a1d * locals.var_a1d)))))) * locals.var_temp4) + (assign22060_e22250 * locals.var_temp4_dn9))) * assign22060_e22256) - (assign22060_e22253 * locals.var_q2d_dn9)) / (assign22060_e22256 * assign22060_e22256)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign22060_e22259;
        locals.var_temp1_dn4 = assign22060_e22259_d_n4;
        locals.var_temp1_dn6 = assign22060_e22259_d_n6;
        locals.var_temp1_dn7 = assign22060_e22259_d_n7;
        locals.var_temp1_dn8 = assign22060_e22259_d_n8;
        locals.var_temp1_dn9 = assign22060_e22259_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign22070_e22277, assign22070_e22277_d_n4, assign22070_e22277_d_n6, assign22070_e22277_d_n7, assign22070_e22277_d_n8, assign22070_e22277_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign22070_e22267: f64 = (locals.var_dqsqd_dxn_qi * locals.var_qid);
        let assign22070_e22269: f64 = (assign22070_e22267 - locals.var_aexp1d);
        let assign22070_e22271: f64 = (assign22070_e22269 / locals.var_a1d);
        let assign22070_e22273: f64 = (assign22070_e22271 - locals.var_temp1);
        let assign22070_e22275: f64 = (assign22070_e22273 / locals.var_qid);
        (assign22070_e22275, ((((((((((locals.var_dqsqd_dxn_qi_dn4 * locals.var_qid) + (locals.var_dqsqd_dxn_qi * locals.var_qid_dn4)) - locals.var_aexp1d_dn4) * locals.var_a1d) - (assign22070_e22269 * locals.var_a1d_dn4)) / (locals.var_a1d * locals.var_a1d)) - locals.var_temp1_dn4) * locals.var_qid) - (assign22070_e22273 * locals.var_qid_dn4)) / (locals.var_qid * locals.var_qid)), ((((((((((locals.var_dqsqd_dxn_qi_dn6 * locals.var_qid) + (locals.var_dqsqd_dxn_qi * locals.var_qid_dn6)) - locals.var_aexp1d_dn6) * locals.var_a1d) - (assign22070_e22269 * locals.var_a1d_dn6)) / (locals.var_a1d * locals.var_a1d)) - locals.var_temp1_dn6) * locals.var_qid) - (assign22070_e22273 * locals.var_qid_dn6)) / (locals.var_qid * locals.var_qid)), ((((((((((locals.var_dqsqd_dxn_qi_dn7 * locals.var_qid) + (locals.var_dqsqd_dxn_qi * locals.var_qid_dn7)) - locals.var_aexp1d_dn7) * locals.var_a1d) - (assign22070_e22269 * locals.var_a1d_dn7)) / (locals.var_a1d * locals.var_a1d)) - locals.var_temp1_dn7) * locals.var_qid) - (assign22070_e22273 * locals.var_qid_dn7)) / (locals.var_qid * locals.var_qid)), ((((((((((locals.var_dqsqd_dxn_qi_dn8 * locals.var_qid) + (locals.var_dqsqd_dxn_qi * locals.var_qid_dn8)) - locals.var_aexp1d_dn8) * locals.var_a1d) - (assign22070_e22269 * locals.var_a1d_dn8)) / (locals.var_a1d * locals.var_a1d)) - locals.var_temp1_dn8) * locals.var_qid) - (assign22070_e22273 * locals.var_qid_dn8)) / (locals.var_qid * locals.var_qid)), ((((((((((locals.var_dqsqd_dxn_qi_dn9 * locals.var_qid) + (locals.var_dqsqd_dxn_qi * locals.var_qid_dn9)) - locals.var_aexp1d_dn9) * locals.var_a1d) - (assign22070_e22269 * locals.var_a1d_dn9)) / (locals.var_a1d * locals.var_a1d)) - locals.var_temp1_dn9) * locals.var_qid) - (assign22070_e22273 * locals.var_qid_dn9)) / (locals.var_qid * locals.var_qid)),)
    } else {
        (locals.var_dqid_dxn_qi, locals.var_dqid_dxn_qi_dn4, locals.var_dqid_dxn_qi_dn6, locals.var_dqid_dxn_qi_dn7, locals.var_dqid_dxn_qi_dn8, locals.var_dqid_dxn_qi_dn9,)
    }
};
        locals.var_dqid_dxn_qi = assign22070_e22277;
        locals.var_dqid_dxn_qi_dn4 = assign22070_e22277_d_n4;
        locals.var_dqid_dxn_qi_dn6 = assign22070_e22277_d_n6;
        locals.var_dqid_dxn_qi_dn7 = assign22070_e22277_d_n7;
        locals.var_dqid_dxn_qi_dn8 = assign22070_e22277_d_n8;
        locals.var_dqid_dxn_qi_dn9 = assign22070_e22277_d_n9;
        locals.var_dqid_dxn_qi_rv = 0.0;

        let (assign22080_e22291, assign22080_e22291_d_n4, assign22080_e22291_d_n6, assign22080_e22291_d_n7, assign22080_e22291_d_n8, assign22080_e22291_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 != 0.0)) {
        let assign22080_e22285: f64 = (locals.var_dqid_dxn_qi * locals.var_qid);
        let assign22080_e22288: f64 = (locals.var_dqid_dxn_qi + 1.0);
        let assign22080_e22289: f64 = (assign22080_e22285 / assign22080_e22288);
        (assign22080_e22289, (((((locals.var_dqid_dxn_qi_dn4 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn4)) * assign22080_e22288) - (assign22080_e22285 * locals.var_dqid_dxn_qi_dn4)) / (assign22080_e22288 * assign22080_e22288)), (((((locals.var_dqid_dxn_qi_dn6 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn6)) * assign22080_e22288) - (assign22080_e22285 * locals.var_dqid_dxn_qi_dn6)) / (assign22080_e22288 * assign22080_e22288)), (((((locals.var_dqid_dxn_qi_dn7 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn7)) * assign22080_e22288) - (assign22080_e22285 * locals.var_dqid_dxn_qi_dn7)) / (assign22080_e22288 * assign22080_e22288)), (((((locals.var_dqid_dxn_qi_dn8 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn8)) * assign22080_e22288) - (assign22080_e22285 * locals.var_dqid_dxn_qi_dn8)) / (assign22080_e22288 * assign22080_e22288)), (((((locals.var_dqid_dxn_qi_dn9 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn9)) * assign22080_e22288) - (assign22080_e22285 * locals.var_dqid_dxn_qi_dn9)) / (assign22080_e22288 * assign22080_e22288)),)
    } else {
        (locals.var_dd, locals.var_dd_dn4, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, locals.var_dd_dn9,)
    }
};
        locals.var_dd = assign22080_e22291;
        locals.var_dd_dn4 = assign22080_e22291_d_n4;
        locals.var_dd_dn6 = assign22080_e22291_d_n6;
        locals.var_dd_dn7 = assign22080_e22291_d_n7;
        locals.var_dd_dn8 = assign22080_e22291_d_n8;
        locals.var_dd_dn9 = assign22080_e22291_d_n9;
        locals.var_dd_rv = 0.0;

        let (assign22090_e22316, assign22090_e22316_d_n4, assign22090_e22316_d_n6, assign22090_e22316_d_n7, assign22090_e22316_d_n8, assign22090_e22316_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 == 0.0)) {
        let assign22090_e22300: f64 = (locals.var_dqsqd_dxn_qi * locals.var_sumd);
        let assign22090_e22303: f64 = (locals.var_a1d * locals.var_a2d);
        let assign22090_e22304: f64 = (assign22090_e22300 / assign22090_e22303);
        let assign22090_e22307: f64 = (locals.var_aexp1d / locals.var_a1d);
        let assign22090_e22310: f64 = (locals.var_aexp2d / locals.var_a2d);
        let assign22090_e22311: f64 = (assign22090_e22307 + assign22090_e22310);
        let assign22090_e22313: f64 = (assign22090_e22311 / locals.var_qid);
        let assign22090_e22314: f64 = (assign22090_e22304 - assign22090_e22313);
        (assign22090_e22314, ((((((locals.var_dqsqd_dxn_qi_dn4 * locals.var_sumd) + (locals.var_dqsqd_dxn_qi * locals.var_sumd_dn4)) * assign22090_e22303) - (assign22090_e22300 * ((locals.var_a1d_dn4 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn4)))) / (assign22090_e22303 * assign22090_e22303)) - (((((((locals.var_aexp1d_dn4 * locals.var_a1d) - (locals.var_aexp1d * locals.var_a1d_dn4)) / (locals.var_a1d * locals.var_a1d)) + (((locals.var_aexp2d_dn4 * locals.var_a2d) - (locals.var_aexp2d * locals.var_a2d_dn4)) / (locals.var_a2d * locals.var_a2d))) * locals.var_qid) - (assign22090_e22311 * locals.var_qid_dn4)) / (locals.var_qid * locals.var_qid))), ((((((locals.var_dqsqd_dxn_qi_dn6 * locals.var_sumd) + (locals.var_dqsqd_dxn_qi * locals.var_sumd_dn6)) * assign22090_e22303) - (assign22090_e22300 * ((locals.var_a1d_dn6 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn6)))) / (assign22090_e22303 * assign22090_e22303)) - (((((((locals.var_aexp1d_dn6 * locals.var_a1d) - (locals.var_aexp1d * locals.var_a1d_dn6)) / (locals.var_a1d * locals.var_a1d)) + (((locals.var_aexp2d_dn6 * locals.var_a2d) - (locals.var_aexp2d * locals.var_a2d_dn6)) / (locals.var_a2d * locals.var_a2d))) * locals.var_qid) - (assign22090_e22311 * locals.var_qid_dn6)) / (locals.var_qid * locals.var_qid))), ((((((locals.var_dqsqd_dxn_qi_dn7 * locals.var_sumd) + (locals.var_dqsqd_dxn_qi * locals.var_sumd_dn7)) * assign22090_e22303) - (assign22090_e22300 * ((locals.var_a1d_dn7 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn7)))) / (assign22090_e22303 * assign22090_e22303)) - (((((((locals.var_aexp1d_dn7 * locals.var_a1d) - (locals.var_aexp1d * locals.var_a1d_dn7)) / (locals.var_a1d * locals.var_a1d)) + (((locals.var_aexp2d_dn7 * locals.var_a2d) - (locals.var_aexp2d * locals.var_a2d_dn7)) / (locals.var_a2d * locals.var_a2d))) * locals.var_qid) - (assign22090_e22311 * locals.var_qid_dn7)) / (locals.var_qid * locals.var_qid))), ((((((locals.var_dqsqd_dxn_qi_dn8 * locals.var_sumd) + (locals.var_dqsqd_dxn_qi * locals.var_sumd_dn8)) * assign22090_e22303) - (assign22090_e22300 * ((locals.var_a1d_dn8 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn8)))) / (assign22090_e22303 * assign22090_e22303)) - (((((((locals.var_aexp1d_dn8 * locals.var_a1d) - (locals.var_aexp1d * locals.var_a1d_dn8)) / (locals.var_a1d * locals.var_a1d)) + (((locals.var_aexp2d_dn8 * locals.var_a2d) - (locals.var_aexp2d * locals.var_a2d_dn8)) / (locals.var_a2d * locals.var_a2d))) * locals.var_qid) - (assign22090_e22311 * locals.var_qid_dn8)) / (locals.var_qid * locals.var_qid))), ((((((locals.var_dqsqd_dxn_qi_dn9 * locals.var_sumd) + (locals.var_dqsqd_dxn_qi * locals.var_sumd_dn9)) * assign22090_e22303) - (assign22090_e22300 * ((locals.var_a1d_dn9 * locals.var_a2d) + (locals.var_a1d * locals.var_a2d_dn9)))) / (assign22090_e22303 * assign22090_e22303)) - (((((((locals.var_aexp1d_dn9 * locals.var_a1d) - (locals.var_aexp1d * locals.var_a1d_dn9)) / (locals.var_a1d * locals.var_a1d)) + (((locals.var_aexp2d_dn9 * locals.var_a2d) - (locals.var_aexp2d * locals.var_a2d_dn9)) / (locals.var_a2d * locals.var_a2d))) * locals.var_qid) - (assign22090_e22311 * locals.var_qid_dn9)) / (locals.var_qid * locals.var_qid))),)
    } else {
        (locals.var_dqid_dxn_qi, locals.var_dqid_dxn_qi_dn4, locals.var_dqid_dxn_qi_dn6, locals.var_dqid_dxn_qi_dn7, locals.var_dqid_dxn_qi_dn8, locals.var_dqid_dxn_qi_dn9,)
    }
};
        locals.var_dqid_dxn_qi = assign22090_e22316;
        locals.var_dqid_dxn_qi_dn4 = assign22090_e22316_d_n4;
        locals.var_dqid_dxn_qi_dn6 = assign22090_e22316_d_n6;
        locals.var_dqid_dxn_qi_dn7 = assign22090_e22316_d_n7;
        locals.var_dqid_dxn_qi_dn8 = assign22090_e22316_d_n8;
        locals.var_dqid_dxn_qi_dn9 = assign22090_e22316_d_n9;
        locals.var_dqid_dxn_qi_rv = 0.0;

        let (assign22100_e22331, assign22100_e22331_d_n4, assign22100_e22331_d_n6, assign22100_e22331_d_n7, assign22100_e22331_d_n8, assign22100_e22331_d_n9,) = {
    if (((locals.var_guard678 != 0.0) && (locals.var_guard679 != 0.0)) && (locals.var_guard680 == 0.0)) {
        let assign22100_e22325: f64 = (locals.var_dqid_dxn_qi * locals.var_qid);
        let assign22100_e22328: f64 = (locals.var_dqid_dxn_qi + 1.0);
        let assign22100_e22329: f64 = (assign22100_e22325 / assign22100_e22328);
        (assign22100_e22329, (((((locals.var_dqid_dxn_qi_dn4 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn4)) * assign22100_e22328) - (assign22100_e22325 * locals.var_dqid_dxn_qi_dn4)) / (assign22100_e22328 * assign22100_e22328)), (((((locals.var_dqid_dxn_qi_dn6 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn6)) * assign22100_e22328) - (assign22100_e22325 * locals.var_dqid_dxn_qi_dn6)) / (assign22100_e22328 * assign22100_e22328)), (((((locals.var_dqid_dxn_qi_dn7 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn7)) * assign22100_e22328) - (assign22100_e22325 * locals.var_dqid_dxn_qi_dn7)) / (assign22100_e22328 * assign22100_e22328)), (((((locals.var_dqid_dxn_qi_dn8 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn8)) * assign22100_e22328) - (assign22100_e22325 * locals.var_dqid_dxn_qi_dn8)) / (assign22100_e22328 * assign22100_e22328)), (((((locals.var_dqid_dxn_qi_dn9 * locals.var_qid) + (locals.var_dqid_dxn_qi * locals.var_qid_dn9)) * assign22100_e22328) - (assign22100_e22325 * locals.var_dqid_dxn_qi_dn9)) / (assign22100_e22328 * assign22100_e22328)),)
    } else {
        (locals.var_dd, locals.var_dd_dn4, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, locals.var_dd_dn9,)
    }
};
        locals.var_dd = assign22100_e22331;
        locals.var_dd_dn4 = assign22100_e22331_d_n4;
        locals.var_dd_dn6 = assign22100_e22331_d_n6;
        locals.var_dd_dn7 = assign22100_e22331_d_n7;
        locals.var_dd_dn8 = assign22100_e22331_d_n8;
        locals.var_dd_dn9 = assign22100_e22331_d_n9;
        locals.var_dd_rv = 0.0;

        let (assign22110_e22338, assign22110_e22338_d_n4, assign22110_e22338_d_n6, assign22110_e22338_d_n7, assign22110_e22338_d_n8, assign22110_e22338_d_n9,) = {
    if ((locals.var_guard678 != 0.0) && (locals.var_guard679 == 0.0)) {
        (locals.var_dinf, locals.var_dinf_dn4, locals.var_dinf_dn6, locals.var_dinf_dn7, locals.var_dinf_dn8, locals.var_dinf_dn9,)
    } else {
        (locals.var_dd, locals.var_dd_dn4, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, locals.var_dd_dn9,)
    }
};
        locals.var_dd = assign22110_e22338;
        locals.var_dd_dn4 = assign22110_e22338_d_n4;
        locals.var_dd_dn6 = assign22110_e22338_d_n6;
        locals.var_dd_dn7 = assign22110_e22338_d_n7;
        locals.var_dd_dn8 = assign22110_e22338_d_n8;
        locals.var_dd_dn9 = assign22110_e22338_d_n9;
        locals.var_dd_rv = 0.0;

        let (assign22120_e22344, assign22120_e22344_d_n4, assign22120_e22344_d_n6, assign22120_e22344_d_n7, assign22120_e22344_d_n8, assign22120_e22344_d_n9,) = {
    if (locals.var_guard678 != 0.0) {
        let assign22120_e22342: f64 = (locals.var_dd - locals.var_ds);
        (assign22120_e22342, (locals.var_dd_dn4 - locals.var_ds_dn4), (locals.var_dd_dn6 - locals.var_ds_dn6), (locals.var_dd_dn7 - locals.var_ds_dn7), (locals.var_dd_dn8 - locals.var_ds_dn8), (locals.var_dd_dn9 - locals.var_ds_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign22120_e22344;
        locals.var_temp1_dn4 = assign22120_e22344_d_n4;
        locals.var_temp1_dn6 = assign22120_e22344_d_n6;
        locals.var_temp1_dn7 = assign22120_e22344_d_n7;
        locals.var_temp1_dn8 = assign22120_e22344_d_n8;
        locals.var_temp1_dn9 = assign22120_e22344_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign22130_e22354, assign22130_e22354_d_n4, assign22130_e22354_d_n6, assign22130_e22354_d_n7, assign22130_e22354_d_n8, assign22130_e22354_d_n9,) = {
    if (locals.var_guard678 != 0.0) {
        let assign22130_e22349: f64 = (36.0 * locals.var_temp1);
        let assign22130_e22351: f64 = (assign22130_e22349 * locals.var_temp1);
        let assign22130_e22352: f64 = (1.0 + assign22130_e22351);
        (assign22130_e22352, (((36.0 * locals.var_temp1_dn4) * locals.var_temp1) + (assign22130_e22349 * locals.var_temp1_dn4)), (((36.0 * locals.var_temp1_dn6) * locals.var_temp1) + (assign22130_e22349 * locals.var_temp1_dn6)), (((36.0 * locals.var_temp1_dn7) * locals.var_temp1) + (assign22130_e22349 * locals.var_temp1_dn7)), (((36.0 * locals.var_temp1_dn8) * locals.var_temp1) + (assign22130_e22349 * locals.var_temp1_dn8)), (((36.0 * locals.var_temp1_dn9) * locals.var_temp1) + (assign22130_e22349 * locals.var_temp1_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign22130_e22354;
        locals.var_temp2_dn4 = assign22130_e22354_d_n4;
        locals.var_temp2_dn6 = assign22130_e22354_d_n6;
        locals.var_temp2_dn7 = assign22130_e22354_d_n7;
        locals.var_temp2_dn8 = assign22130_e22354_d_n8;
        locals.var_temp2_dn9 = assign22130_e22354_d_n9;
        locals.var_temp2_rv = 0.0;

        let assign22140_e22356: f64 = (locals.var_temp1).abs();
        let assign22140_e22358: f64 = if assign22140_e22356 > 0.001 { 1.0 } else { 0.0 };
        locals.var_guard681 = assign22140_e22358;
        locals.var_guard681_rv = 0.0;

        let (assign22150_e22366, assign22150_e22366_d_n4, assign22150_e22366_d_n6, assign22150_e22366_d_n7, assign22150_e22366_d_n8, assign22150_e22366_d_n9,) = {
    if ((locals.var_guard678 != 0.0) && (locals.var_guard681 != 0.0)) {
        let assign22150_e22364: f64 = (locals.var_qid - locals.var_qis);
        (assign22150_e22364, (locals.var_qid_dn4 - locals.var_qis_dn4), (locals.var_qid_dn6 - locals.var_qis_dn6), (locals.var_qid_dn7 - locals.var_qis_dn7), (locals.var_qid_dn8 - locals.var_qis_dn8), (locals.var_qid_dn9 - locals.var_qis_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign22150_e22366;
        locals.var_temp3_dn4 = assign22150_e22366_d_n4;
        locals.var_temp3_dn6 = assign22150_e22366_d_n6;
        locals.var_temp3_dn7 = assign22150_e22366_d_n7;
        locals.var_temp3_dn8 = assign22150_e22366_d_n8;
        locals.var_temp3_dn9 = assign22150_e22366_d_n9;
        locals.var_temp3_rv = 0.0;

        let (assign22160_e22376, assign22160_e22376_d_n4, assign22160_e22376_d_n6, assign22160_e22376_d_n7, assign22160_e22376_d_n8, assign22160_e22376_d_n9,) = {
    if ((locals.var_guard678 != 0.0) && (locals.var_guard681 != 0.0)) {
        let assign22160_e22373: f64 = (locals.var_dd * locals.var_dxdrift);
        let assign22160_e22374: f64 = (locals.var_temp3 - assign22160_e22373);
        (assign22160_e22374, (locals.var_temp3_dn4 - ((locals.var_dd_dn4 * locals.var_dxdrift) + (locals.var_dd * locals.var_dxdrift_dn4))), (locals.var_temp3_dn6 - ((locals.var_dd_dn6 * locals.var_dxdrift) + (locals.var_dd * locals.var_dxdrift_dn6))), (locals.var_temp3_dn7 - ((locals.var_dd_dn7 * locals.var_dxdrift) + (locals.var_dd * locals.var_dxdrift_dn7))), (locals.var_temp3_dn8 - ((locals.var_dd_dn8 * locals.var_dxdrift) + (locals.var_dd * locals.var_dxdrift_dn8))), (locals.var_temp3_dn9 - ((locals.var_dd_dn9 * locals.var_dxdrift) + (locals.var_dd * locals.var_dxdrift_dn9))),)
    } else {
        (locals.var_ls, locals.var_ls_dn4, locals.var_ls_dn6, locals.var_ls_dn7, locals.var_ls_dn8, locals.var_ls_dn9,)
    }
};
        locals.var_ls = assign22160_e22376;
        locals.var_ls_dn4 = assign22160_e22376_d_n4;
        locals.var_ls_dn6 = assign22160_e22376_d_n6;
        locals.var_ls_dn7 = assign22160_e22376_d_n7;
        locals.var_ls_dn8 = assign22160_e22376_d_n8;
        locals.var_ls_dn9 = assign22160_e22376_d_n9;
        locals.var_ls_rv = 0.0;

        let (assign22170_e22386, assign22170_e22386_d_n4, assign22170_e22386_d_n6, assign22170_e22386_d_n7, assign22170_e22386_d_n8, assign22170_e22386_d_n9,) = {
    if ((locals.var_guard678 != 0.0) && (locals.var_guard681 != 0.0)) {
        let assign22170_e22383: f64 = (locals.var_ds * locals.var_dxdrift);
        let assign22170_e22384: f64 = (locals.var_temp3 - assign22170_e22383);
        (assign22170_e22384, (locals.var_temp3_dn4 - ((locals.var_ds_dn4 * locals.var_dxdrift) + (locals.var_ds * locals.var_dxdrift_dn4))), (locals.var_temp3_dn6 - ((locals.var_ds_dn6 * locals.var_dxdrift) + (locals.var_ds * locals.var_dxdrift_dn6))), (locals.var_temp3_dn7 - ((locals.var_ds_dn7 * locals.var_dxdrift) + (locals.var_ds * locals.var_dxdrift_dn7))), (locals.var_temp3_dn8 - ((locals.var_ds_dn8 * locals.var_dxdrift) + (locals.var_ds * locals.var_dxdrift_dn8))), (locals.var_temp3_dn9 - ((locals.var_ds_dn9 * locals.var_dxdrift) + (locals.var_ds * locals.var_dxdrift_dn9))),)
    } else {
        (locals.var_ld, locals.var_ld_dn4, locals.var_ld_dn6, locals.var_ld_dn7, locals.var_ld_dn8, locals.var_ld_dn9,)
    }
};
        locals.var_ld = assign22170_e22386;
        locals.var_ld_dn4 = assign22170_e22386_d_n4;
        locals.var_ld_dn6 = assign22170_e22386_d_n6;
        locals.var_ld_dn7 = assign22170_e22386_d_n7;
        locals.var_ld_dn8 = assign22170_e22386_d_n8;
        locals.var_ld_dn9 = assign22170_e22386_d_n9;
        locals.var_ld_rv = 0.0;

        let (assign22180_e22397, assign22180_e22397_d_n4, assign22180_e22397_d_n6, assign22180_e22397_d_n7, assign22180_e22397_d_n8, assign22180_e22397_d_n9,) = {
    if ((locals.var_guard678 != 0.0) && (locals.var_guard681 != 0.0)) {
        let assign22180_e22392: f64 = (locals.var_ls * locals.var_ls);
        let assign22180_e22394: f64 = (assign22180_e22392 + locals.var_temp2);
        let assign22180_e22395: f64 = (assign22180_e22394).sqrt();
        (assign22180_e22395, ((((locals.var_ls_dn4 * locals.var_ls) + (locals.var_ls * locals.var_ls_dn4)) + locals.var_temp2_dn4) / (2.0 * assign22180_e22395)), ((((locals.var_ls_dn6 * locals.var_ls) + (locals.var_ls * locals.var_ls_dn6)) + locals.var_temp2_dn6) / (2.0 * assign22180_e22395)), ((((locals.var_ls_dn7 * locals.var_ls) + (locals.var_ls * locals.var_ls_dn7)) + locals.var_temp2_dn7) / (2.0 * assign22180_e22395)), ((((locals.var_ls_dn8 * locals.var_ls) + (locals.var_ls * locals.var_ls_dn8)) + locals.var_temp2_dn8) / (2.0 * assign22180_e22395)), ((((locals.var_ls_dn9 * locals.var_ls) + (locals.var_ls * locals.var_ls_dn9)) + locals.var_temp2_dn9) / (2.0 * assign22180_e22395)),)
    } else {
        (locals.var_us, locals.var_us_dn4, locals.var_us_dn6, locals.var_us_dn7, locals.var_us_dn8, locals.var_us_dn9,)
    }
};
        locals.var_us = assign22180_e22397;
        locals.var_us_dn4 = assign22180_e22397_d_n4;
        locals.var_us_dn6 = assign22180_e22397_d_n6;
        locals.var_us_dn7 = assign22180_e22397_d_n7;
        locals.var_us_dn8 = assign22180_e22397_d_n8;
        locals.var_us_dn9 = assign22180_e22397_d_n9;
        locals.var_us_rv = 0.0;

        let (assign22190_e22408, assign22190_e22408_d_n4, assign22190_e22408_d_n6, assign22190_e22408_d_n7, assign22190_e22408_d_n8, assign22190_e22408_d_n9,) = {
    if ((locals.var_guard678 != 0.0) && (locals.var_guard681 != 0.0)) {
        let assign22190_e22403: f64 = (locals.var_ld * locals.var_ld);
        let assign22190_e22405: f64 = (assign22190_e22403 + locals.var_temp2);
        let assign22190_e22406: f64 = (assign22190_e22405).sqrt();
        (assign22190_e22406, ((((locals.var_ld_dn4 * locals.var_ld) + (locals.var_ld * locals.var_ld_dn4)) + locals.var_temp2_dn4) / (2.0 * assign22190_e22406)), ((((locals.var_ld_dn6 * locals.var_ld) + (locals.var_ld * locals.var_ld_dn6)) + locals.var_temp2_dn6) / (2.0 * assign22190_e22406)), ((((locals.var_ld_dn7 * locals.var_ld) + (locals.var_ld * locals.var_ld_dn7)) + locals.var_temp2_dn7) / (2.0 * assign22190_e22406)), ((((locals.var_ld_dn8 * locals.var_ld) + (locals.var_ld * locals.var_ld_dn8)) + locals.var_temp2_dn8) / (2.0 * assign22190_e22406)), ((((locals.var_ld_dn9 * locals.var_ld) + (locals.var_ld * locals.var_ld_dn9)) + locals.var_temp2_dn9) / (2.0 * assign22190_e22406)),)
    } else {
        (locals.var_ud, locals.var_ud_dn4, locals.var_ud_dn6, locals.var_ud_dn7, locals.var_ud_dn8, locals.var_ud_dn9,)
    }
};
        locals.var_ud = assign22190_e22408;
        locals.var_ud_dn4 = assign22190_e22408_d_n4;
        locals.var_ud_dn6 = assign22190_e22408_d_n6;
        locals.var_ud_dn7 = assign22190_e22408_d_n7;
        locals.var_ud_dn8 = assign22190_e22408_d_n8;
        locals.var_ud_dn9 = assign22190_e22408_d_n9;
        locals.var_ud_rv = 0.0;

        let (assign22200_e22435, assign22200_e22435_d_n4, assign22200_e22435_d_n6, assign22200_e22435_d_n7, assign22200_e22435_d_n8, assign22200_e22435_d_n9,) = {
    if ((locals.var_guard678 != 0.0) && (locals.var_guard681 != 0.0)) {
        let assign22200_e22414: f64 = (0.25 / locals.var_temp1);
        let assign22200_e22417: f64 = (locals.var_ud * locals.var_ls);
        let assign22200_e22420: f64 = (locals.var_us * locals.var_ld);
        let assign22200_e22421: f64 = (assign22200_e22417 - assign22200_e22420);
        let assign22200_e22425: f64 = (locals.var_ld + locals.var_ud);
        let assign22200_e22428: f64 = (locals.var_ls + locals.var_us);
        let assign22200_e22429: f64 = (assign22200_e22425 / assign22200_e22428);
        let assign22200_e22430: f64 = (assign22200_e22429).ln();
        let assign22200_e22431: f64 = (locals.var_temp2 * assign22200_e22430);
        let assign22200_e22432: f64 = (assign22200_e22421 + assign22200_e22431);
        let assign22200_e22433: f64 = (assign22200_e22414 * assign22200_e22432);
        (assign22200_e22433, (((-((0.25 * locals.var_temp1_dn4) / (locals.var_temp1 * locals.var_temp1))) * assign22200_e22432) + (assign22200_e22414 * ((((locals.var_ud_dn4 * locals.var_ls) + (locals.var_ud * locals.var_ls_dn4)) - ((locals.var_us_dn4 * locals.var_ld) + (locals.var_us * locals.var_ld_dn4))) + ((locals.var_temp2_dn4 * assign22200_e22430) + (locals.var_temp2 * (((((locals.var_ld_dn4 + locals.var_ud_dn4) * assign22200_e22428) - (assign22200_e22425 * (locals.var_ls_dn4 + locals.var_us_dn4))) / (assign22200_e22428 * assign22200_e22428)) / assign22200_e22429)))))), (((-((0.25 * locals.var_temp1_dn6) / (locals.var_temp1 * locals.var_temp1))) * assign22200_e22432) + (assign22200_e22414 * ((((locals.var_ud_dn6 * locals.var_ls) + (locals.var_ud * locals.var_ls_dn6)) - ((locals.var_us_dn6 * locals.var_ld) + (locals.var_us * locals.var_ld_dn6))) + ((locals.var_temp2_dn6 * assign22200_e22430) + (locals.var_temp2 * (((((locals.var_ld_dn6 + locals.var_ud_dn6) * assign22200_e22428) - (assign22200_e22425 * (locals.var_ls_dn6 + locals.var_us_dn6))) / (assign22200_e22428 * assign22200_e22428)) / assign22200_e22429)))))), (((-((0.25 * locals.var_temp1_dn7) / (locals.var_temp1 * locals.var_temp1))) * assign22200_e22432) + (assign22200_e22414 * ((((locals.var_ud_dn7 * locals.var_ls) + (locals.var_ud * locals.var_ls_dn7)) - ((locals.var_us_dn7 * locals.var_ld) + (locals.var_us * locals.var_ld_dn7))) + ((locals.var_temp2_dn7 * assign22200_e22430) + (locals.var_temp2 * (((((locals.var_ld_dn7 + locals.var_ud_dn7) * assign22200_e22428) - (assign22200_e22425 * (locals.var_ls_dn7 + locals.var_us_dn7))) / (assign22200_e22428 * assign22200_e22428)) / assign22200_e22429)))))), (((-((0.25 * locals.var_temp1_dn8) / (locals.var_temp1 * locals.var_temp1))) * assign22200_e22432) + (assign22200_e22414 * ((((locals.var_ud_dn8 * locals.var_ls) + (locals.var_ud * locals.var_ls_dn8)) - ((locals.var_us_dn8 * locals.var_ld) + (locals.var_us * locals.var_ld_dn8))) + ((locals.var_temp2_dn8 * assign22200_e22430) + (locals.var_temp2 * (((((locals.var_ld_dn8 + locals.var_ud_dn8) * assign22200_e22428) - (assign22200_e22425 * (locals.var_ls_dn8 + locals.var_us_dn8))) / (assign22200_e22428 * assign22200_e22428)) / assign22200_e22429)))))), (((-((0.25 * locals.var_temp1_dn9) / (locals.var_temp1 * locals.var_temp1))) * assign22200_e22432) + (assign22200_e22414 * ((((locals.var_ud_dn9 * locals.var_ls) + (locals.var_ud * locals.var_ls_dn9)) - ((locals.var_us_dn9 * locals.var_ld) + (locals.var_us * locals.var_ld_dn9))) + ((locals.var_temp2_dn9 * assign22200_e22430) + (locals.var_temp2 * (((((locals.var_ld_dn9 + locals.var_ud_dn9) * assign22200_e22428) - (assign22200_e22425 * (locals.var_ls_dn9 + locals.var_us_dn9))) / (assign22200_e22428 * assign22200_e22428)) / assign22200_e22429)))))),)
    } else {
        (locals.var_idrift2, locals.var_idrift2_dn4, locals.var_idrift2_dn6, locals.var_idrift2_dn7, locals.var_idrift2_dn8, locals.var_idrift2_dn9,)
    }
};
        locals.var_idrift2 = assign22200_e22435;
        locals.var_idrift2_dn4 = assign22200_e22435_d_n4;
        locals.var_idrift2_dn6 = assign22200_e22435_d_n6;
        locals.var_idrift2_dn7 = assign22200_e22435_d_n7;
        locals.var_idrift2_dn8 = assign22200_e22435_d_n8;
        locals.var_idrift2_dn9 = assign22200_e22435_d_n9;
        locals.var_idrift2_rv = 0.0;

        let (assign22210_e22444, assign22210_e22444_d_n4, assign22210_e22444_d_n6, assign22210_e22444_d_n7, assign22210_e22444_d_n8, assign22210_e22444_d_n9,) = {
    if ((locals.var_guard678 != 0.0) && (locals.var_guard681 == 0.0)) {
        let assign22210_e22442: f64 = (locals.var_dxdrift * locals.var_temp1);
        (assign22210_e22442, ((locals.var_dxdrift_dn4 * locals.var_temp1) + (locals.var_dxdrift * locals.var_temp1_dn4)), ((locals.var_dxdrift_dn6 * locals.var_temp1) + (locals.var_dxdrift * locals.var_temp1_dn6)), ((locals.var_dxdrift_dn7 * locals.var_temp1) + (locals.var_dxdrift * locals.var_temp1_dn7)), ((locals.var_dxdrift_dn8 * locals.var_temp1) + (locals.var_dxdrift * locals.var_temp1_dn8)), ((locals.var_dxdrift_dn9 * locals.var_temp1) + (locals.var_dxdrift * locals.var_temp1_dn9)),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign22210_e22444;
        locals.var_temp3_dn4 = assign22210_e22444_d_n4;
        locals.var_temp3_dn6 = assign22210_e22444_d_n6;
        locals.var_temp3_dn7 = assign22210_e22444_d_n7;
        locals.var_temp3_dn8 = assign22210_e22444_d_n8;
        locals.var_temp3_dn9 = assign22210_e22444_d_n9;
        locals.var_temp3_rv = 0.0;

        let (assign22220_e22463, assign22220_e22463_d_n4, assign22220_e22463_d_n6, assign22220_e22463_d_n7, assign22220_e22463_d_n8, assign22220_e22463_d_n9,) = {
    if ((locals.var_guard678 != 0.0) && (locals.var_guard681 == 0.0)) {
        let assign22220_e22450: f64 = (-0.25);
        let assign22220_e22452: f64 = (assign22220_e22450 * 0.1666666666667);
        let assign22220_e22454: f64 = (assign22220_e22452 * locals.var_dxdrift);
        let assign22220_e22456: f64 = (assign22220_e22454 * locals.var_temp3);
        let assign22220_e22458: f64 = (assign22220_e22456 * locals.var_temp3);
        let assign22220_e22460: f64 = (locals.var_temp2).sqrt();
        let assign22220_e22461: f64 = (assign22220_e22458 / assign22220_e22460);
        (assign22220_e22461, ((((((((assign22220_e22452 * locals.var_dxdrift_dn4) * locals.var_temp3) + (assign22220_e22454 * locals.var_temp3_dn4)) * locals.var_temp3) + (assign22220_e22456 * locals.var_temp3_dn4)) * assign22220_e22460) - (assign22220_e22458 * (locals.var_temp2_dn4 / (2.0 * assign22220_e22460)))) / (assign22220_e22460 * assign22220_e22460)), ((((((((assign22220_e22452 * locals.var_dxdrift_dn6) * locals.var_temp3) + (assign22220_e22454 * locals.var_temp3_dn6)) * locals.var_temp3) + (assign22220_e22456 * locals.var_temp3_dn6)) * assign22220_e22460) - (assign22220_e22458 * (locals.var_temp2_dn6 / (2.0 * assign22220_e22460)))) / (assign22220_e22460 * assign22220_e22460)), ((((((((assign22220_e22452 * locals.var_dxdrift_dn7) * locals.var_temp3) + (assign22220_e22454 * locals.var_temp3_dn7)) * locals.var_temp3) + (assign22220_e22456 * locals.var_temp3_dn7)) * assign22220_e22460) - (assign22220_e22458 * (locals.var_temp2_dn7 / (2.0 * assign22220_e22460)))) / (assign22220_e22460 * assign22220_e22460)), ((((((((assign22220_e22452 * locals.var_dxdrift_dn8) * locals.var_temp3) + (assign22220_e22454 * locals.var_temp3_dn8)) * locals.var_temp3) + (assign22220_e22456 * locals.var_temp3_dn8)) * assign22220_e22460) - (assign22220_e22458 * (locals.var_temp2_dn8 / (2.0 * assign22220_e22460)))) / (assign22220_e22460 * assign22220_e22460)), ((((((((assign22220_e22452 * locals.var_dxdrift_dn9) * locals.var_temp3) + (assign22220_e22454 * locals.var_temp3_dn9)) * locals.var_temp3) + (assign22220_e22456 * locals.var_temp3_dn9)) * assign22220_e22460) - (assign22220_e22458 * (locals.var_temp2_dn9 / (2.0 * assign22220_e22460)))) / (assign22220_e22460 * assign22220_e22460)),)
    } else {
        (locals.var_idrift2, locals.var_idrift2_dn4, locals.var_idrift2_dn6, locals.var_idrift2_dn7, locals.var_idrift2_dn8, locals.var_idrift2_dn9,)
    }
};
        locals.var_idrift2 = assign22220_e22463;
        locals.var_idrift2_dn4 = assign22220_e22463_d_n4;
        locals.var_idrift2_dn6 = assign22220_e22463_d_n6;
        locals.var_idrift2_dn7 = assign22220_e22463_d_n7;
        locals.var_idrift2_dn8 = assign22220_e22463_d_n8;
        locals.var_idrift2_dn9 = assign22220_e22463_d_n9;
        locals.var_idrift2_rv = 0.0;

        let (assign22230_e22468, assign22230_e22468_d_n4, assign22230_e22468_d_n6, assign22230_e22468_d_n7, assign22230_e22468_d_n8, assign22230_e22468_d_n9,) = {
    if (locals.var_guard678 == 0.0) {
        (locals.var_dinf, locals.var_dinf_dn4, locals.var_dinf_dn6, locals.var_dinf_dn7, locals.var_dinf_dn8, locals.var_dinf_dn9,)
    } else {
        (locals.var_dd, locals.var_dd_dn4, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, locals.var_dd_dn9,)
    }
};
        locals.var_dd = assign22230_e22468;
        locals.var_dd_dn4 = assign22230_e22468_d_n4;
        locals.var_dd_dn6 = assign22230_e22468_d_n6;
        locals.var_dd_dn7 = assign22230_e22468_d_n7;
        locals.var_dd_dn8 = assign22230_e22468_d_n8;
        locals.var_dd_dn9 = assign22230_e22468_d_n9;
        locals.var_dd_rv = 0.0;

        let (assign22240_e22473, assign22240_e22473_d_n4, assign22240_e22473_d_n6, assign22240_e22473_d_n7, assign22240_e22473_d_n8, assign22240_e22473_d_n9,) = {
    if (locals.var_guard678 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idrift2, locals.var_idrift2_dn4, locals.var_idrift2_dn6, locals.var_idrift2_dn7, locals.var_idrift2_dn8, locals.var_idrift2_dn9,)
    }
};
        locals.var_idrift2 = assign22240_e22473;
        locals.var_idrift2_dn4 = assign22240_e22473_d_n4;
        locals.var_idrift2_dn6 = assign22240_e22473_d_n6;
        locals.var_idrift2_dn7 = assign22240_e22473_d_n7;
        locals.var_idrift2_dn8 = assign22240_e22473_d_n8;
        locals.var_idrift2_dn9 = assign22240_e22473_d_n9;
        locals.var_idrift2_rv = 0.0;

        let assign22250_e22476: f64 = (locals.var_qim * locals.var_dxdrift);
        let assign22250_e22478: f64 = (assign22250_e22476 + locals.var_idrift2);
        let assign22250_e22480: f64 = (assign22250_e22478 + locals.var_qis);
        let assign22250_e22482: f64 = (assign22250_e22480 - locals.var_qid);
        locals.var_norm_ids = assign22250_e22482;
        locals.var_norm_ids_dn4 = (((((locals.var_qim_dn4 * locals.var_dxdrift) + (locals.var_qim * locals.var_dxdrift_dn4)) + locals.var_idrift2_dn4) + locals.var_qis_dn4) - locals.var_qid_dn4);
        locals.var_norm_ids_dn6 = (((((locals.var_qim_dn6 * locals.var_dxdrift) + (locals.var_qim * locals.var_dxdrift_dn6)) + locals.var_idrift2_dn6) + locals.var_qis_dn6) - locals.var_qid_dn6);
        locals.var_norm_ids_dn7 = (((((locals.var_qim_dn7 * locals.var_dxdrift) + (locals.var_qim * locals.var_dxdrift_dn7)) + locals.var_idrift2_dn7) + locals.var_qis_dn7) - locals.var_qid_dn7);
        locals.var_norm_ids_dn8 = (((((locals.var_qim_dn8 * locals.var_dxdrift) + (locals.var_qim * locals.var_dxdrift_dn8)) + locals.var_idrift2_dn8) + locals.var_qis_dn8) - locals.var_qid_dn8);
        locals.var_norm_ids_dn9 = (((((locals.var_qim_dn9 * locals.var_dxdrift) + (locals.var_qim * locals.var_dxdrift_dn9)) + locals.var_idrift2_dn9) + locals.var_qis_dn9) - locals.var_qid_dn9);
        locals.var_norm_ids_rv = 0.0;

        let assign22260_e22485: f64 = if locals.var_qis > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard682 = assign22260_e22485;
        locals.var_guard682_rv = 0.0;

        let assign22270_e22488: f64 = if locals.var_norm_ids > 1e-30 { 1.0 } else { 0.0 };
        locals.var_guard683 = assign22270_e22488;
        locals.var_guard683_rv = 0.0;

        let (assign22280_e22500, assign22280_e22500_d_n4, assign22280_e22500_d_n6, assign22280_e22500_d_n7, assign22280_e22500_d_n8, assign22280_e22500_d_n9,) = {
    if ((locals.var_guard682 != 0.0) && (locals.var_guard683 != 0.0)) {
        let assign22280_e22495: f64 = (locals.var_aexp1s / locals.var_qis);
        let assign22280_e22497: f64 = (assign22280_e22495 - locals.var_dqsqs_dxn_qi);
        let assign22280_e22498: f64 = (locals.var_a1s / assign22280_e22497);
        (assign22280_e22498, (((locals.var_a1s_dn4 * assign22280_e22497) - (locals.var_a1s * ((((locals.var_aexp1s_dn4 * locals.var_qis) - (locals.var_aexp1s * locals.var_qis_dn4)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn4))) / (assign22280_e22497 * assign22280_e22497)), (((locals.var_a1s_dn6 * assign22280_e22497) - (locals.var_a1s * ((((locals.var_aexp1s_dn6 * locals.var_qis) - (locals.var_aexp1s * locals.var_qis_dn6)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn6))) / (assign22280_e22497 * assign22280_e22497)), (((locals.var_a1s_dn7 * assign22280_e22497) - (locals.var_a1s * ((((locals.var_aexp1s_dn7 * locals.var_qis) - (locals.var_aexp1s * locals.var_qis_dn7)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn7))) / (assign22280_e22497 * assign22280_e22497)), (((locals.var_a1s_dn8 * assign22280_e22497) - (locals.var_a1s * ((((locals.var_aexp1s_dn8 * locals.var_qis) - (locals.var_aexp1s * locals.var_qis_dn8)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn8))) / (assign22280_e22497 * assign22280_e22497)), (((locals.var_a1s_dn9 * assign22280_e22497) - (locals.var_a1s * ((((locals.var_aexp1s_dn9 * locals.var_qis) - (locals.var_aexp1s * locals.var_qis_dn9)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn9))) / (assign22280_e22497 * assign22280_e22497)),)
    } else {
        (locals.var_q1s_chap, locals.var_q1s_chap_dn4, locals.var_q1s_chap_dn6, locals.var_q1s_chap_dn7, locals.var_q1s_chap_dn8, locals.var_q1s_chap_dn9,)
    }
};
        locals.var_q1s_chap = assign22280_e22500;
        locals.var_q1s_chap_dn4 = assign22280_e22500_d_n4;
        locals.var_q1s_chap_dn6 = assign22280_e22500_d_n6;
        locals.var_q1s_chap_dn7 = assign22280_e22500_d_n7;
        locals.var_q1s_chap_dn8 = assign22280_e22500_d_n8;
        locals.var_q1s_chap_dn9 = assign22280_e22500_d_n9;
        locals.var_q1s_chap_rv = 0.0;

        let (assign22290_e22512, assign22290_e22512_d_n4, assign22290_e22512_d_n6, assign22290_e22512_d_n7, assign22290_e22512_d_n8, assign22290_e22512_d_n9,) = {
    if ((locals.var_guard682 != 0.0) && (locals.var_guard683 != 0.0)) {
        let assign22290_e22507: f64 = (locals.var_aexp1d / locals.var_qid);
        let assign22290_e22509: f64 = (assign22290_e22507 - locals.var_dqsqd_dxn_qi);
        let assign22290_e22510: f64 = (locals.var_a1d / assign22290_e22509);
        (assign22290_e22510, (((locals.var_a1d_dn4 * assign22290_e22509) - (locals.var_a1d * ((((locals.var_aexp1d_dn4 * locals.var_qid) - (locals.var_aexp1d * locals.var_qid_dn4)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn4))) / (assign22290_e22509 * assign22290_e22509)), (((locals.var_a1d_dn6 * assign22290_e22509) - (locals.var_a1d * ((((locals.var_aexp1d_dn6 * locals.var_qid) - (locals.var_aexp1d * locals.var_qid_dn6)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn6))) / (assign22290_e22509 * assign22290_e22509)), (((locals.var_a1d_dn7 * assign22290_e22509) - (locals.var_a1d * ((((locals.var_aexp1d_dn7 * locals.var_qid) - (locals.var_aexp1d * locals.var_qid_dn7)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn7))) / (assign22290_e22509 * assign22290_e22509)), (((locals.var_a1d_dn8 * assign22290_e22509) - (locals.var_a1d * ((((locals.var_aexp1d_dn8 * locals.var_qid) - (locals.var_aexp1d * locals.var_qid_dn8)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn8))) / (assign22290_e22509 * assign22290_e22509)), (((locals.var_a1d_dn9 * assign22290_e22509) - (locals.var_a1d * ((((locals.var_aexp1d_dn9 * locals.var_qid) - (locals.var_aexp1d * locals.var_qid_dn9)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn9))) / (assign22290_e22509 * assign22290_e22509)),)
    } else {
        (locals.var_q1d_chap, locals.var_q1d_chap_dn4, locals.var_q1d_chap_dn6, locals.var_q1d_chap_dn7, locals.var_q1d_chap_dn8, locals.var_q1d_chap_dn9,)
    }
};
        locals.var_q1d_chap = assign22290_e22512;
        locals.var_q1d_chap_dn4 = assign22290_e22512_d_n4;
        locals.var_q1d_chap_dn6 = assign22290_e22512_d_n6;
        locals.var_q1d_chap_dn7 = assign22290_e22512_d_n7;
        locals.var_q1d_chap_dn8 = assign22290_e22512_d_n8;
        locals.var_q1d_chap_dn9 = assign22290_e22512_d_n9;
        locals.var_q1d_chap_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_61(
        locals: &mut StampLocals,
    ) {
        let (assign22300_e22522, assign22300_e22522_d_n4, assign22300_e22522_d_n6, assign22300_e22522_d_n7, assign22300_e22522_d_n8, assign22300_e22522_d_n9,) = {
    if ((locals.var_guard682 != 0.0) && (locals.var_guard683 != 0.0)) {
        let assign22300_e22518: f64 = (locals.var_q1s_chap - locals.var_q1d_chap);
        let assign22300_e22520: f64 = (assign22300_e22518 / locals.var_norm_ids);
        (assign22300_e22520, ((((locals.var_q1s_chap_dn4 - locals.var_q1d_chap_dn4) * locals.var_norm_ids) - (assign22300_e22518 * locals.var_norm_ids_dn4)) / (locals.var_norm_ids * locals.var_norm_ids)), ((((locals.var_q1s_chap_dn6 - locals.var_q1d_chap_dn6) * locals.var_norm_ids) - (assign22300_e22518 * locals.var_norm_ids_dn6)) / (locals.var_norm_ids * locals.var_norm_ids)), ((((locals.var_q1s_chap_dn7 - locals.var_q1d_chap_dn7) * locals.var_norm_ids) - (assign22300_e22518 * locals.var_norm_ids_dn7)) / (locals.var_norm_ids * locals.var_norm_ids)), ((((locals.var_q1s_chap_dn8 - locals.var_q1d_chap_dn8) * locals.var_norm_ids) - (assign22300_e22518 * locals.var_norm_ids_dn8)) / (locals.var_norm_ids * locals.var_norm_ids)), ((((locals.var_q1s_chap_dn9 - locals.var_q1d_chap_dn9) * locals.var_norm_ids) - (assign22300_e22518 * locals.var_norm_ids_dn9)) / (locals.var_norm_ids * locals.var_norm_ids)),)
    } else {
        (locals.var_inv_k1h1_0, locals.var_inv_k1h1_0_dn4, locals.var_inv_k1h1_0_dn6, locals.var_inv_k1h1_0_dn7, locals.var_inv_k1h1_0_dn8, locals.var_inv_k1h1_0_dn9,)
    }
};
        locals.var_inv_k1h1_0 = assign22300_e22522;
        locals.var_inv_k1h1_0_dn4 = assign22300_e22522_d_n4;
        locals.var_inv_k1h1_0_dn6 = assign22300_e22522_d_n6;
        locals.var_inv_k1h1_0_dn7 = assign22300_e22522_d_n7;
        locals.var_inv_k1h1_0_dn8 = assign22300_e22522_d_n8;
        locals.var_inv_k1h1_0_dn9 = assign22300_e22522_d_n9;
        locals.var_inv_k1h1_0_rv = 0.0;

        let (assign22310_e22534, assign22310_e22534_d_n4, assign22310_e22534_d_n6, assign22310_e22534_d_n7, assign22310_e22534_d_n8, assign22310_e22534_d_n9,) = {
    if ((locals.var_guard682 != 0.0) && (locals.var_guard683 != 0.0)) {
        let assign22310_e22529: f64 = (locals.var_aexp2s / locals.var_qis);
        let assign22310_e22531: f64 = (assign22310_e22529 - locals.var_dqsqs_dxn_qi);
        let assign22310_e22532: f64 = (locals.var_a2s / assign22310_e22531);
        (assign22310_e22532, (((locals.var_a2s_dn4 * assign22310_e22531) - (locals.var_a2s * ((((locals.var_aexp2s_dn4 * locals.var_qis) - (locals.var_aexp2s * locals.var_qis_dn4)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn4))) / (assign22310_e22531 * assign22310_e22531)), (((locals.var_a2s_dn6 * assign22310_e22531) - (locals.var_a2s * ((((locals.var_aexp2s_dn6 * locals.var_qis) - (locals.var_aexp2s * locals.var_qis_dn6)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn6))) / (assign22310_e22531 * assign22310_e22531)), (((locals.var_a2s_dn7 * assign22310_e22531) - (locals.var_a2s * ((((locals.var_aexp2s_dn7 * locals.var_qis) - (locals.var_aexp2s * locals.var_qis_dn7)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn7))) / (assign22310_e22531 * assign22310_e22531)), (((locals.var_a2s_dn8 * assign22310_e22531) - (locals.var_a2s * ((((locals.var_aexp2s_dn8 * locals.var_qis) - (locals.var_aexp2s * locals.var_qis_dn8)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn8))) / (assign22310_e22531 * assign22310_e22531)), (((locals.var_a2s_dn9 * assign22310_e22531) - (locals.var_a2s * ((((locals.var_aexp2s_dn9 * locals.var_qis) - (locals.var_aexp2s * locals.var_qis_dn9)) / (locals.var_qis * locals.var_qis)) - locals.var_dqsqs_dxn_qi_dn9))) / (assign22310_e22531 * assign22310_e22531)),)
    } else {
        (locals.var_q2s_chap, locals.var_q2s_chap_dn4, locals.var_q2s_chap_dn6, locals.var_q2s_chap_dn7, locals.var_q2s_chap_dn8, locals.var_q2s_chap_dn9,)
    }
};
        locals.var_q2s_chap = assign22310_e22534;
        locals.var_q2s_chap_dn4 = assign22310_e22534_d_n4;
        locals.var_q2s_chap_dn6 = assign22310_e22534_d_n6;
        locals.var_q2s_chap_dn7 = assign22310_e22534_d_n7;
        locals.var_q2s_chap_dn8 = assign22310_e22534_d_n8;
        locals.var_q2s_chap_dn9 = assign22310_e22534_d_n9;
        locals.var_q2s_chap_rv = 0.0;

        let (assign22320_e22546, assign22320_e22546_d_n4, assign22320_e22546_d_n6, assign22320_e22546_d_n7, assign22320_e22546_d_n8, assign22320_e22546_d_n9,) = {
    if ((locals.var_guard682 != 0.0) && (locals.var_guard683 != 0.0)) {
        let assign22320_e22541: f64 = (locals.var_aexp2d / locals.var_qid);
        let assign22320_e22543: f64 = (assign22320_e22541 - locals.var_dqsqd_dxn_qi);
        let assign22320_e22544: f64 = (locals.var_a2d / assign22320_e22543);
        (assign22320_e22544, (((locals.var_a2d_dn4 * assign22320_e22543) - (locals.var_a2d * ((((locals.var_aexp2d_dn4 * locals.var_qid) - (locals.var_aexp2d * locals.var_qid_dn4)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn4))) / (assign22320_e22543 * assign22320_e22543)), (((locals.var_a2d_dn6 * assign22320_e22543) - (locals.var_a2d * ((((locals.var_aexp2d_dn6 * locals.var_qid) - (locals.var_aexp2d * locals.var_qid_dn6)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn6))) / (assign22320_e22543 * assign22320_e22543)), (((locals.var_a2d_dn7 * assign22320_e22543) - (locals.var_a2d * ((((locals.var_aexp2d_dn7 * locals.var_qid) - (locals.var_aexp2d * locals.var_qid_dn7)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn7))) / (assign22320_e22543 * assign22320_e22543)), (((locals.var_a2d_dn8 * assign22320_e22543) - (locals.var_a2d * ((((locals.var_aexp2d_dn8 * locals.var_qid) - (locals.var_aexp2d * locals.var_qid_dn8)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn8))) / (assign22320_e22543 * assign22320_e22543)), (((locals.var_a2d_dn9 * assign22320_e22543) - (locals.var_a2d * ((((locals.var_aexp2d_dn9 * locals.var_qid) - (locals.var_aexp2d * locals.var_qid_dn9)) / (locals.var_qid * locals.var_qid)) - locals.var_dqsqd_dxn_qi_dn9))) / (assign22320_e22543 * assign22320_e22543)),)
    } else {
        (locals.var_q2d_chap, locals.var_q2d_chap_dn4, locals.var_q2d_chap_dn6, locals.var_q2d_chap_dn7, locals.var_q2d_chap_dn8, locals.var_q2d_chap_dn9,)
    }
};
        locals.var_q2d_chap = assign22320_e22546;
        locals.var_q2d_chap_dn4 = assign22320_e22546_d_n4;
        locals.var_q2d_chap_dn6 = assign22320_e22546_d_n6;
        locals.var_q2d_chap_dn7 = assign22320_e22546_d_n7;
        locals.var_q2d_chap_dn8 = assign22320_e22546_d_n8;
        locals.var_q2d_chap_dn9 = assign22320_e22546_d_n9;
        locals.var_q2d_chap_rv = 0.0;

        let (assign22330_e22556, assign22330_e22556_d_n4, assign22330_e22556_d_n6, assign22330_e22556_d_n7, assign22330_e22556_d_n8, assign22330_e22556_d_n9,) = {
    if ((locals.var_guard682 != 0.0) && (locals.var_guard683 != 0.0)) {
        let assign22330_e22552: f64 = (locals.var_q2s_chap - locals.var_q2d_chap);
        let assign22330_e22554: f64 = (assign22330_e22552 / locals.var_norm_ids);
        (assign22330_e22554, ((((locals.var_q2s_chap_dn4 - locals.var_q2d_chap_dn4) * locals.var_norm_ids) - (assign22330_e22552 * locals.var_norm_ids_dn4)) / (locals.var_norm_ids * locals.var_norm_ids)), ((((locals.var_q2s_chap_dn6 - locals.var_q2d_chap_dn6) * locals.var_norm_ids) - (assign22330_e22552 * locals.var_norm_ids_dn6)) / (locals.var_norm_ids * locals.var_norm_ids)), ((((locals.var_q2s_chap_dn7 - locals.var_q2d_chap_dn7) * locals.var_norm_ids) - (assign22330_e22552 * locals.var_norm_ids_dn7)) / (locals.var_norm_ids * locals.var_norm_ids)), ((((locals.var_q2s_chap_dn8 - locals.var_q2d_chap_dn8) * locals.var_norm_ids) - (assign22330_e22552 * locals.var_norm_ids_dn8)) / (locals.var_norm_ids * locals.var_norm_ids)), ((((locals.var_q2s_chap_dn9 - locals.var_q2d_chap_dn9) * locals.var_norm_ids) - (assign22330_e22552 * locals.var_norm_ids_dn9)) / (locals.var_norm_ids * locals.var_norm_ids)),)
    } else {
        (locals.var_inv_k2h2_0, locals.var_inv_k2h2_0_dn4, locals.var_inv_k2h2_0_dn6, locals.var_inv_k2h2_0_dn7, locals.var_inv_k2h2_0_dn8, locals.var_inv_k2h2_0_dn9,)
    }
};
        locals.var_inv_k2h2_0 = assign22330_e22556;
        locals.var_inv_k2h2_0_dn4 = assign22330_e22556_d_n4;
        locals.var_inv_k2h2_0_dn6 = assign22330_e22556_d_n6;
        locals.var_inv_k2h2_0_dn7 = assign22330_e22556_d_n7;
        locals.var_inv_k2h2_0_dn8 = assign22330_e22556_d_n8;
        locals.var_inv_k2h2_0_dn9 = assign22330_e22556_d_n9;
        locals.var_inv_k2h2_0_rv = 0.0;

        let (assign22340_e22563, assign22340_e22563_d_n4, assign22340_e22563_d_n6, assign22340_e22563_d_n7, assign22340_e22563_d_n8, assign22340_e22563_d_n9,) = {
    if ((locals.var_guard682 != 0.0) && (locals.var_guard683 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_inv_k1h1_0, locals.var_inv_k1h1_0_dn4, locals.var_inv_k1h1_0_dn6, locals.var_inv_k1h1_0_dn7, locals.var_inv_k1h1_0_dn8, locals.var_inv_k1h1_0_dn9,)
    }
};
        locals.var_inv_k1h1_0 = assign22340_e22563;
        locals.var_inv_k1h1_0_dn4 = assign22340_e22563_d_n4;
        locals.var_inv_k1h1_0_dn6 = assign22340_e22563_d_n6;
        locals.var_inv_k1h1_0_dn7 = assign22340_e22563_d_n7;
        locals.var_inv_k1h1_0_dn8 = assign22340_e22563_d_n8;
        locals.var_inv_k1h1_0_dn9 = assign22340_e22563_d_n9;
        locals.var_inv_k1h1_0_rv = 0.0;

        let (assign22350_e22570, assign22350_e22570_d_n4, assign22350_e22570_d_n6, assign22350_e22570_d_n7, assign22350_e22570_d_n8, assign22350_e22570_d_n9,) = {
    if ((locals.var_guard682 != 0.0) && (locals.var_guard683 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_inv_k2h2_0, locals.var_inv_k2h2_0_dn4, locals.var_inv_k2h2_0_dn6, locals.var_inv_k2h2_0_dn7, locals.var_inv_k2h2_0_dn8, locals.var_inv_k2h2_0_dn9,)
    }
};
        locals.var_inv_k2h2_0 = assign22350_e22570;
        locals.var_inv_k2h2_0_dn4 = assign22350_e22570_d_n4;
        locals.var_inv_k2h2_0_dn6 = assign22350_e22570_d_n6;
        locals.var_inv_k2h2_0_dn7 = assign22350_e22570_d_n7;
        locals.var_inv_k2h2_0_dn8 = assign22350_e22570_d_n8;
        locals.var_inv_k2h2_0_dn9 = assign22350_e22570_d_n9;
        locals.var_inv_k2h2_0_rv = 0.0;

        let (assign22360_e22584, assign22360_e22584_d_n4, assign22360_e22584_d_n6, assign22360_e22584_d_n7, assign22360_e22584_d_n8, assign22360_e22584_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22360_e22574: f64 = (-2.0);
        let assign22360_e22576: f64 = (assign22360_e22574 * locals.var_s1);
        let assign22360_e22579: f64 = (locals.var_inv_k1 / locals.var_q1chapinf);
        let assign22360_e22581: f64 = (assign22360_e22579 + locals.var_inv_dinf);
        let assign22360_e22582: f64 = (assign22360_e22576 * assign22360_e22581);
        (assign22360_e22582, (((assign22360_e22574 * locals.var_s1_dn4) * assign22360_e22581) + (assign22360_e22576 * ((((locals.var_inv_k1_dn4 * locals.var_q1chapinf) - (locals.var_inv_k1 * locals.var_q1chapinf_dn4)) / (locals.var_q1chapinf * locals.var_q1chapinf)) + locals.var_inv_dinf_dn4))), (((assign22360_e22574 * locals.var_s1_dn6) * assign22360_e22581) + (assign22360_e22576 * ((((locals.var_inv_k1_dn6 * locals.var_q1chapinf) - (locals.var_inv_k1 * locals.var_q1chapinf_dn6)) / (locals.var_q1chapinf * locals.var_q1chapinf)) + locals.var_inv_dinf_dn6))), (((assign22360_e22574 * locals.var_s1_dn7) * assign22360_e22581) + (assign22360_e22576 * ((((locals.var_inv_k1_dn7 * locals.var_q1chapinf) - (locals.var_inv_k1 * locals.var_q1chapinf_dn7)) / (locals.var_q1chapinf * locals.var_q1chapinf)) + locals.var_inv_dinf_dn7))), (((assign22360_e22574 * locals.var_s1_dn8) * assign22360_e22581) + (assign22360_e22576 * ((((locals.var_inv_k1_dn8 * locals.var_q1chapinf) - (locals.var_inv_k1 * locals.var_q1chapinf_dn8)) / (locals.var_q1chapinf * locals.var_q1chapinf)) + locals.var_inv_dinf_dn8))), (((assign22360_e22574 * locals.var_s1_dn9) * assign22360_e22581) + (assign22360_e22576 * ((((locals.var_inv_k1_dn9 * locals.var_q1chapinf) - (locals.var_inv_k1 * locals.var_q1chapinf_dn9)) / (locals.var_q1chapinf * locals.var_q1chapinf)) + locals.var_inv_dinf_dn9))),)
    } else {
        (locals.var_zeta1, locals.var_zeta1_dn4, locals.var_zeta1_dn6, locals.var_zeta1_dn7, locals.var_zeta1_dn8, locals.var_zeta1_dn9,)
    }
};
        locals.var_zeta1 = assign22360_e22584;
        locals.var_zeta1_dn4 = assign22360_e22584_d_n4;
        locals.var_zeta1_dn6 = assign22360_e22584_d_n6;
        locals.var_zeta1_dn7 = assign22360_e22584_d_n7;
        locals.var_zeta1_dn8 = assign22360_e22584_d_n8;
        locals.var_zeta1_dn9 = assign22360_e22584_d_n9;
        locals.var_zeta1_rv = 0.0;

        let (assign22370_e22598, assign22370_e22598_d_n4, assign22370_e22598_d_n6, assign22370_e22598_d_n7, assign22370_e22598_d_n8, assign22370_e22598_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22370_e22588: f64 = (-2.0);
        let assign22370_e22590: f64 = (assign22370_e22588 * locals.var_s2);
        let assign22370_e22593: f64 = (locals.var_inv_k2 / locals.var_q2chapinf);
        let assign22370_e22595: f64 = (assign22370_e22593 + locals.var_inv_dinf);
        let assign22370_e22596: f64 = (assign22370_e22590 * assign22370_e22595);
        (assign22370_e22596, (((assign22370_e22588 * locals.var_s2_dn4) * assign22370_e22595) + (assign22370_e22590 * ((((locals.var_inv_k2_dn4 * locals.var_q2chapinf) - (locals.var_inv_k2 * locals.var_q2chapinf_dn4)) / (locals.var_q2chapinf * locals.var_q2chapinf)) + locals.var_inv_dinf_dn4))), (((assign22370_e22588 * locals.var_s2_dn6) * assign22370_e22595) + (assign22370_e22590 * ((((locals.var_inv_k2_dn6 * locals.var_q2chapinf) - (locals.var_inv_k2 * locals.var_q2chapinf_dn6)) / (locals.var_q2chapinf * locals.var_q2chapinf)) + locals.var_inv_dinf_dn6))), (((assign22370_e22588 * locals.var_s2_dn7) * assign22370_e22595) + (assign22370_e22590 * ((((locals.var_inv_k2_dn7 * locals.var_q2chapinf) - (locals.var_inv_k2 * locals.var_q2chapinf_dn7)) / (locals.var_q2chapinf * locals.var_q2chapinf)) + locals.var_inv_dinf_dn7))), (((assign22370_e22588 * locals.var_s2_dn8) * assign22370_e22595) + (assign22370_e22590 * ((((locals.var_inv_k2_dn8 * locals.var_q2chapinf) - (locals.var_inv_k2 * locals.var_q2chapinf_dn8)) / (locals.var_q2chapinf * locals.var_q2chapinf)) + locals.var_inv_dinf_dn8))), (((assign22370_e22588 * locals.var_s2_dn9) * assign22370_e22595) + (assign22370_e22590 * ((((locals.var_inv_k2_dn9 * locals.var_q2chapinf) - (locals.var_inv_k2 * locals.var_q2chapinf_dn9)) / (locals.var_q2chapinf * locals.var_q2chapinf)) + locals.var_inv_dinf_dn9))),)
    } else {
        (locals.var_zeta2, locals.var_zeta2_dn4, locals.var_zeta2_dn6, locals.var_zeta2_dn7, locals.var_zeta2_dn8, locals.var_zeta2_dn9,)
    }
};
        locals.var_zeta2 = assign22370_e22598;
        locals.var_zeta2_dn4 = assign22370_e22598_d_n4;
        locals.var_zeta2_dn6 = assign22370_e22598_d_n6;
        locals.var_zeta2_dn7 = assign22370_e22598_d_n7;
        locals.var_zeta2_dn8 = assign22370_e22598_d_n8;
        locals.var_zeta2_dn9 = assign22370_e22598_d_n9;
        locals.var_zeta2_rv = 0.0;

        let (assign22380_e22607, assign22380_e22607_d_n4, assign22380_e22607_d_n6, assign22380_e22607_d_n7, assign22380_e22607_d_n8, assign22380_e22607_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22380_e22603: f64 = (locals.var_zeta2 - locals.var_zeta1);
        let assign22380_e22605: f64 = (assign22380_e22603 * locals.var_inv_dinf);
        (assign22380_e22605, (((locals.var_zeta2_dn4 - locals.var_zeta1_dn4) * locals.var_inv_dinf) + (assign22380_e22603 * locals.var_inv_dinf_dn4)), (((locals.var_zeta2_dn6 - locals.var_zeta1_dn6) * locals.var_inv_dinf) + (assign22380_e22603 * locals.var_inv_dinf_dn6)), (((locals.var_zeta2_dn7 - locals.var_zeta1_dn7) * locals.var_inv_dinf) + (assign22380_e22603 * locals.var_inv_dinf_dn7)), (((locals.var_zeta2_dn8 - locals.var_zeta1_dn8) * locals.var_inv_dinf) + (assign22380_e22603 * locals.var_inv_dinf_dn8)), (((locals.var_zeta2_dn9 - locals.var_zeta1_dn9) * locals.var_inv_dinf) + (assign22380_e22603 * locals.var_inv_dinf_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign22380_e22607;
        locals.var_temp_dn4 = assign22380_e22607_d_n4;
        locals.var_temp_dn6 = assign22380_e22607_d_n6;
        locals.var_temp_dn7 = assign22380_e22607_d_n7;
        locals.var_temp_dn8 = assign22380_e22607_d_n8;
        locals.var_temp_dn9 = assign22380_e22607_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign22390_e22614, assign22390_e22614_d_n4, assign22390_e22614_d_n6, assign22390_e22614_d_n7, assign22390_e22614_d_n8, assign22390_e22614_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22390_e22612: f64 = (locals.var_zeta1 * locals.var_inv_k1);
        (assign22390_e22612, ((locals.var_zeta1_dn4 * locals.var_inv_k1) + (locals.var_zeta1 * locals.var_inv_k1_dn4)), ((locals.var_zeta1_dn6 * locals.var_inv_k1) + (locals.var_zeta1 * locals.var_inv_k1_dn6)), ((locals.var_zeta1_dn7 * locals.var_inv_k1) + (locals.var_zeta1 * locals.var_inv_k1_dn7)), ((locals.var_zeta1_dn8 * locals.var_inv_k1) + (locals.var_zeta1 * locals.var_inv_k1_dn8)), ((locals.var_zeta1_dn9 * locals.var_inv_k1) + (locals.var_zeta1 * locals.var_inv_k1_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign22390_e22614;
        locals.var_temp1_dn4 = assign22390_e22614_d_n4;
        locals.var_temp1_dn6 = assign22390_e22614_d_n6;
        locals.var_temp1_dn7 = assign22390_e22614_d_n7;
        locals.var_temp1_dn8 = assign22390_e22614_d_n8;
        locals.var_temp1_dn9 = assign22390_e22614_d_n9;
        locals.var_temp1_rv = 0.0;

        let (assign22400_e22621, assign22400_e22621_d_n4, assign22400_e22621_d_n6, assign22400_e22621_d_n7, assign22400_e22621_d_n8, assign22400_e22621_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22400_e22619: f64 = (locals.var_zeta2 * locals.var_inv_k2);
        (assign22400_e22619, ((locals.var_zeta2_dn4 * locals.var_inv_k2) + (locals.var_zeta2 * locals.var_inv_k2_dn4)), ((locals.var_zeta2_dn6 * locals.var_inv_k2) + (locals.var_zeta2 * locals.var_inv_k2_dn6)), ((locals.var_zeta2_dn7 * locals.var_inv_k2) + (locals.var_zeta2 * locals.var_inv_k2_dn7)), ((locals.var_zeta2_dn8 * locals.var_inv_k2) + (locals.var_zeta2 * locals.var_inv_k2_dn8)), ((locals.var_zeta2_dn9 * locals.var_inv_k2) + (locals.var_zeta2 * locals.var_inv_k2_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign22400_e22621;
        locals.var_temp2_dn4 = assign22400_e22621_d_n4;
        locals.var_temp2_dn6 = assign22400_e22621_d_n6;
        locals.var_temp2_dn7 = assign22400_e22621_d_n7;
        locals.var_temp2_dn8 = assign22400_e22621_d_n8;
        locals.var_temp2_dn9 = assign22400_e22621_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign22410_e22628, assign22410_e22628_d_n4, assign22410_e22628_d_n6, assign22410_e22628_d_n7, assign22410_e22628_d_n8, assign22410_e22628_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22410_e22626: f64 = (locals.var_temp1 + locals.var_temp2);
        (assign22410_e22626, (locals.var_temp1_dn4 + locals.var_temp2_dn4), (locals.var_temp1_dn6 + locals.var_temp2_dn6), (locals.var_temp1_dn7 + locals.var_temp2_dn7), (locals.var_temp1_dn8 + locals.var_temp2_dn8), (locals.var_temp1_dn9 + locals.var_temp2_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign22410_e22628;
        locals.var_temp3_dn4 = assign22410_e22628_d_n4;
        locals.var_temp3_dn6 = assign22410_e22628_d_n6;
        locals.var_temp3_dn7 = assign22410_e22628_d_n7;
        locals.var_temp3_dn8 = assign22410_e22628_d_n8;
        locals.var_temp3_dn9 = assign22410_e22628_d_n9;
        locals.var_temp3_rv = 0.0;

        let (assign22420_e22643, assign22420_e22643_d_n4, assign22420_e22643_d_n6, assign22420_e22643_d_n7, assign22420_e22643_d_n8, assign22420_e22643_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22420_e22635: f64 = (locals.var_s1 * locals.var_inv_k1);
        let assign22420_e22638: f64 = (locals.var_s2 * locals.var_inv_k2);
        let assign22420_e22639: f64 = (assign22420_e22635 + assign22420_e22638);
        let assign22420_e22640: f64 = (2.0 * assign22420_e22639);
        let assign22420_e22641: f64 = (3.0 + assign22420_e22640);
        (assign22420_e22641, (2.0 * (((locals.var_s1_dn4 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn4)) + ((locals.var_s2_dn4 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn4)))), (2.0 * (((locals.var_s1_dn6 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn6)) + ((locals.var_s2_dn6 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn6)))), (2.0 * (((locals.var_s1_dn7 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn7)) + ((locals.var_s2_dn7 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn7)))), (2.0 * (((locals.var_s1_dn8 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn8)) + ((locals.var_s2_dn8 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn8)))), (2.0 * (((locals.var_s1_dn9 * locals.var_inv_k1) + (locals.var_s1 * locals.var_inv_k1_dn9)) + ((locals.var_s2_dn9 * locals.var_inv_k2) + (locals.var_s2 * locals.var_inv_k2_dn9)))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign22420_e22643;
        locals.var_temp4_dn4 = assign22420_e22643_d_n4;
        locals.var_temp4_dn6 = assign22420_e22643_d_n6;
        locals.var_temp4_dn7 = assign22420_e22643_d_n7;
        locals.var_temp4_dn8 = assign22420_e22643_d_n8;
        locals.var_temp4_dn9 = assign22420_e22643_d_n9;
        locals.var_temp4_rv = 0.0;

        let (assign22430_e22656, assign22430_e22656_d_n4, assign22430_e22656_d_n6, assign22430_e22656_d_n7, assign22430_e22656_d_n8, assign22430_e22656_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22430_e22648: f64 = (locals.var_temp2 + locals.var_temp);
        let assign22430_e22651: f64 = (locals.var_temp3 / locals.var_q1chapinf);
        let assign22430_e22652: f64 = (assign22430_e22648 - assign22430_e22651);
        let assign22430_e22654: f64 = (assign22430_e22652 / locals.var_temp4);
        (assign22430_e22654, (((((locals.var_temp2_dn4 + locals.var_temp_dn4) - (((locals.var_temp3_dn4 * locals.var_q1chapinf) - (locals.var_temp3 * locals.var_q1chapinf_dn4)) / (locals.var_q1chapinf * locals.var_q1chapinf))) * locals.var_temp4) - (assign22430_e22652 * locals.var_temp4_dn4)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn6 + locals.var_temp_dn6) - (((locals.var_temp3_dn6 * locals.var_q1chapinf) - (locals.var_temp3 * locals.var_q1chapinf_dn6)) / (locals.var_q1chapinf * locals.var_q1chapinf))) * locals.var_temp4) - (assign22430_e22652 * locals.var_temp4_dn6)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn7 + locals.var_temp_dn7) - (((locals.var_temp3_dn7 * locals.var_q1chapinf) - (locals.var_temp3 * locals.var_q1chapinf_dn7)) / (locals.var_q1chapinf * locals.var_q1chapinf))) * locals.var_temp4) - (assign22430_e22652 * locals.var_temp4_dn7)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn8 + locals.var_temp_dn8) - (((locals.var_temp3_dn8 * locals.var_q1chapinf) - (locals.var_temp3 * locals.var_q1chapinf_dn8)) / (locals.var_q1chapinf * locals.var_q1chapinf))) * locals.var_temp4) - (assign22430_e22652 * locals.var_temp4_dn8)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn9 + locals.var_temp_dn9) - (((locals.var_temp3_dn9 * locals.var_q1chapinf) - (locals.var_temp3 * locals.var_q1chapinf_dn9)) / (locals.var_q1chapinf * locals.var_q1chapinf))) * locals.var_temp4) - (assign22430_e22652 * locals.var_temp4_dn9)) / (locals.var_temp4 * locals.var_temp4)),)
    } else {
        (locals.var_ksi1, locals.var_ksi1_dn4, locals.var_ksi1_dn6, locals.var_ksi1_dn7, locals.var_ksi1_dn8, locals.var_ksi1_dn9,)
    }
};
        locals.var_ksi1 = assign22430_e22656;
        locals.var_ksi1_dn4 = assign22430_e22656_d_n4;
        locals.var_ksi1_dn6 = assign22430_e22656_d_n6;
        locals.var_ksi1_dn7 = assign22430_e22656_d_n7;
        locals.var_ksi1_dn8 = assign22430_e22656_d_n8;
        locals.var_ksi1_dn9 = assign22430_e22656_d_n9;
        locals.var_ksi1_rv = 0.0;

        let (assign22440_e22669, assign22440_e22669_d_n4, assign22440_e22669_d_n6, assign22440_e22669_d_n7, assign22440_e22669_d_n8, assign22440_e22669_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22440_e22661: f64 = (locals.var_temp1 - locals.var_temp);
        let assign22440_e22664: f64 = (locals.var_temp3 / locals.var_q2chapinf);
        let assign22440_e22665: f64 = (assign22440_e22661 - assign22440_e22664);
        let assign22440_e22667: f64 = (assign22440_e22665 / locals.var_temp4);
        (assign22440_e22667, (((((locals.var_temp1_dn4 - locals.var_temp_dn4) - (((locals.var_temp3_dn4 * locals.var_q2chapinf) - (locals.var_temp3 * locals.var_q2chapinf_dn4)) / (locals.var_q2chapinf * locals.var_q2chapinf))) * locals.var_temp4) - (assign22440_e22665 * locals.var_temp4_dn4)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn6 - locals.var_temp_dn6) - (((locals.var_temp3_dn6 * locals.var_q2chapinf) - (locals.var_temp3 * locals.var_q2chapinf_dn6)) / (locals.var_q2chapinf * locals.var_q2chapinf))) * locals.var_temp4) - (assign22440_e22665 * locals.var_temp4_dn6)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn7 - locals.var_temp_dn7) - (((locals.var_temp3_dn7 * locals.var_q2chapinf) - (locals.var_temp3 * locals.var_q2chapinf_dn7)) / (locals.var_q2chapinf * locals.var_q2chapinf))) * locals.var_temp4) - (assign22440_e22665 * locals.var_temp4_dn7)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn8 - locals.var_temp_dn8) - (((locals.var_temp3_dn8 * locals.var_q2chapinf) - (locals.var_temp3 * locals.var_q2chapinf_dn8)) / (locals.var_q2chapinf * locals.var_q2chapinf))) * locals.var_temp4) - (assign22440_e22665 * locals.var_temp4_dn8)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn9 - locals.var_temp_dn9) - (((locals.var_temp3_dn9 * locals.var_q2chapinf) - (locals.var_temp3 * locals.var_q2chapinf_dn9)) / (locals.var_q2chapinf * locals.var_q2chapinf))) * locals.var_temp4) - (assign22440_e22665 * locals.var_temp4_dn9)) / (locals.var_temp4 * locals.var_temp4)),)
    } else {
        (locals.var_ksi2, locals.var_ksi2_dn4, locals.var_ksi2_dn6, locals.var_ksi2_dn7, locals.var_ksi2_dn8, locals.var_ksi2_dn9,)
    }
};
        locals.var_ksi2 = assign22440_e22669;
        locals.var_ksi2_dn4 = assign22440_e22669_d_n4;
        locals.var_ksi2_dn6 = assign22440_e22669_d_n6;
        locals.var_ksi2_dn7 = assign22440_e22669_d_n7;
        locals.var_ksi2_dn8 = assign22440_e22669_d_n8;
        locals.var_ksi2_dn9 = assign22440_e22669_d_n9;
        locals.var_ksi2_rv = 0.0;

        let (assign22450_e22681, assign22450_e22681_d_n4, assign22450_e22681_d_n6, assign22450_e22681_d_n7, assign22450_e22681_d_n8, assign22450_e22681_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22450_e22673: f64 = (-locals.var_q1chapinf);
        let assign22450_e22676: f64 = (locals.var_ksi1 * locals.var_q1chapinf);
        let assign22450_e22678: f64 = (assign22450_e22676 + locals.var_inv_dinf);
        let assign22450_e22679: f64 = (assign22450_e22673 * assign22450_e22678);
        (assign22450_e22679, (((-locals.var_q1chapinf_dn4) * assign22450_e22678) + (assign22450_e22673 * (((locals.var_ksi1_dn4 * locals.var_q1chapinf) + (locals.var_ksi1 * locals.var_q1chapinf_dn4)) + locals.var_inv_dinf_dn4))), (((-locals.var_q1chapinf_dn6) * assign22450_e22678) + (assign22450_e22673 * (((locals.var_ksi1_dn6 * locals.var_q1chapinf) + (locals.var_ksi1 * locals.var_q1chapinf_dn6)) + locals.var_inv_dinf_dn6))), (((-locals.var_q1chapinf_dn7) * assign22450_e22678) + (assign22450_e22673 * (((locals.var_ksi1_dn7 * locals.var_q1chapinf) + (locals.var_ksi1 * locals.var_q1chapinf_dn7)) + locals.var_inv_dinf_dn7))), (((-locals.var_q1chapinf_dn8) * assign22450_e22678) + (assign22450_e22673 * (((locals.var_ksi1_dn8 * locals.var_q1chapinf) + (locals.var_ksi1 * locals.var_q1chapinf_dn8)) + locals.var_inv_dinf_dn8))), (((-locals.var_q1chapinf_dn9) * assign22450_e22678) + (assign22450_e22673 * (((locals.var_ksi1_dn9 * locals.var_q1chapinf) + (locals.var_ksi1 * locals.var_q1chapinf_dn9)) + locals.var_inv_dinf_dn9))),)
    } else {
        (locals.var_inv_k1h1_0, locals.var_inv_k1h1_0_dn4, locals.var_inv_k1h1_0_dn6, locals.var_inv_k1h1_0_dn7, locals.var_inv_k1h1_0_dn8, locals.var_inv_k1h1_0_dn9,)
    }
};
        locals.var_inv_k1h1_0 = assign22450_e22681;
        locals.var_inv_k1h1_0_dn4 = assign22450_e22681_d_n4;
        locals.var_inv_k1h1_0_dn6 = assign22450_e22681_d_n6;
        locals.var_inv_k1h1_0_dn7 = assign22450_e22681_d_n7;
        locals.var_inv_k1h1_0_dn8 = assign22450_e22681_d_n8;
        locals.var_inv_k1h1_0_dn9 = assign22450_e22681_d_n9;
        locals.var_inv_k1h1_0_rv = 0.0;

        let (assign22460_e22693, assign22460_e22693_d_n4, assign22460_e22693_d_n6, assign22460_e22693_d_n7, assign22460_e22693_d_n8, assign22460_e22693_d_n9,) = {
    if (locals.var_guard682 == 0.0) {
        let assign22460_e22685: f64 = (-locals.var_q2chapinf);
        let assign22460_e22688: f64 = (locals.var_ksi2 * locals.var_q2chapinf);
        let assign22460_e22690: f64 = (assign22460_e22688 + locals.var_inv_dinf);
        let assign22460_e22691: f64 = (assign22460_e22685 * assign22460_e22690);
        (assign22460_e22691, (((-locals.var_q2chapinf_dn4) * assign22460_e22690) + (assign22460_e22685 * (((locals.var_ksi2_dn4 * locals.var_q2chapinf) + (locals.var_ksi2 * locals.var_q2chapinf_dn4)) + locals.var_inv_dinf_dn4))), (((-locals.var_q2chapinf_dn6) * assign22460_e22690) + (assign22460_e22685 * (((locals.var_ksi2_dn6 * locals.var_q2chapinf) + (locals.var_ksi2 * locals.var_q2chapinf_dn6)) + locals.var_inv_dinf_dn6))), (((-locals.var_q2chapinf_dn7) * assign22460_e22690) + (assign22460_e22685 * (((locals.var_ksi2_dn7 * locals.var_q2chapinf) + (locals.var_ksi2 * locals.var_q2chapinf_dn7)) + locals.var_inv_dinf_dn7))), (((-locals.var_q2chapinf_dn8) * assign22460_e22690) + (assign22460_e22685 * (((locals.var_ksi2_dn8 * locals.var_q2chapinf) + (locals.var_ksi2 * locals.var_q2chapinf_dn8)) + locals.var_inv_dinf_dn8))), (((-locals.var_q2chapinf_dn9) * assign22460_e22690) + (assign22460_e22685 * (((locals.var_ksi2_dn9 * locals.var_q2chapinf) + (locals.var_ksi2 * locals.var_q2chapinf_dn9)) + locals.var_inv_dinf_dn9))),)
    } else {
        (locals.var_inv_k2h2_0, locals.var_inv_k2h2_0_dn4, locals.var_inv_k2h2_0_dn6, locals.var_inv_k2h2_0_dn7, locals.var_inv_k2h2_0_dn8, locals.var_inv_k2h2_0_dn9,)
    }
};
        locals.var_inv_k2h2_0 = assign22460_e22693;
        locals.var_inv_k2h2_0_dn4 = assign22460_e22693_d_n4;
        locals.var_inv_k2h2_0_dn6 = assign22460_e22693_d_n6;
        locals.var_inv_k2h2_0_dn7 = assign22460_e22693_d_n7;
        locals.var_inv_k2h2_0_dn8 = assign22460_e22693_d_n8;
        locals.var_inv_k2h2_0_dn9 = assign22460_e22693_d_n9;
        locals.var_inv_k2h2_0_rv = 0.0;

        let assign22470_e22696: f64 = (locals.var_inv_k1h1_0 * locals.var_hsat);
        locals.var_inv_k1h1 = assign22470_e22696;
        locals.var_inv_k1h1_dn4 = ((locals.var_inv_k1h1_0_dn4 * locals.var_hsat) + (locals.var_inv_k1h1_0 * locals.var_hsat_dn4));
        locals.var_inv_k1h1_dn6 = ((locals.var_inv_k1h1_0_dn6 * locals.var_hsat) + (locals.var_inv_k1h1_0 * locals.var_hsat_dn6));
        locals.var_inv_k1h1_dn7 = ((locals.var_inv_k1h1_0_dn7 * locals.var_hsat) + (locals.var_inv_k1h1_0 * locals.var_hsat_dn7));
        locals.var_inv_k1h1_dn8 = ((locals.var_inv_k1h1_0_dn8 * locals.var_hsat) + (locals.var_inv_k1h1_0 * locals.var_hsat_dn8));
        locals.var_inv_k1h1_dn9 = ((locals.var_inv_k1h1_0_dn9 * locals.var_hsat) + (locals.var_inv_k1h1_0 * locals.var_hsat_dn9));
        locals.var_inv_k1h1_rv = 0.0;

        let assign22480_e22699: f64 = (locals.var_inv_k2h2_0 * locals.var_hsat);
        locals.var_inv_k2h2 = assign22480_e22699;
        locals.var_inv_k2h2_dn4 = ((locals.var_inv_k2h2_0_dn4 * locals.var_hsat) + (locals.var_inv_k2h2_0 * locals.var_hsat_dn4));
        locals.var_inv_k2h2_dn6 = ((locals.var_inv_k2h2_0_dn6 * locals.var_hsat) + (locals.var_inv_k2h2_0 * locals.var_hsat_dn6));
        locals.var_inv_k2h2_dn7 = ((locals.var_inv_k2h2_0_dn7 * locals.var_hsat) + (locals.var_inv_k2h2_0 * locals.var_hsat_dn7));
        locals.var_inv_k2h2_dn8 = ((locals.var_inv_k2h2_0_dn8 * locals.var_hsat) + (locals.var_inv_k2h2_0 * locals.var_hsat_dn8));
        locals.var_inv_k2h2_dn9 = ((locals.var_inv_k2h2_0_dn9 * locals.var_hsat) + (locals.var_inv_k2h2_0 * locals.var_hsat_dn9));
        locals.var_inv_k2h2_rv = 0.0;

        let assign22490_e22703: f64 = (locals.var_k1q1d - locals.var_k1q1s);
        let assign22490_e22704: f64 = (0.5 * assign22490_e22703);
        locals.var_delta_k1q1 = assign22490_e22704;
        locals.var_delta_k1q1_dn4 = (0.5 * (locals.var_k1q1d_dn4 - locals.var_k1q1s_dn4));
        locals.var_delta_k1q1_dn6 = (0.5 * (locals.var_k1q1d_dn6 - locals.var_k1q1s_dn6));
        locals.var_delta_k1q1_dn7 = (0.5 * (locals.var_k1q1d_dn7 - locals.var_k1q1s_dn7));
        locals.var_delta_k1q1_dn8 = (0.5 * (locals.var_k1q1d_dn8 - locals.var_k1q1s_dn8));
        locals.var_delta_k1q1_dn9 = (0.5 * (locals.var_k1q1d_dn9 - locals.var_k1q1s_dn9));
        locals.var_delta_k1q1_rv = 0.0;

        let assign22500_e22708: f64 = (locals.var_k2q2d - locals.var_k2q2s);
        let assign22500_e22709: f64 = (0.5 * assign22500_e22708);
        locals.var_delta_k2q2 = assign22500_e22709;
        locals.var_delta_k2q2_dn4 = (0.5 * (locals.var_k2q2d_dn4 - locals.var_k2q2s_dn4));
        locals.var_delta_k2q2_dn6 = (0.5 * (locals.var_k2q2d_dn6 - locals.var_k2q2s_dn6));
        locals.var_delta_k2q2_dn7 = (0.5 * (locals.var_k2q2d_dn7 - locals.var_k2q2s_dn7));
        locals.var_delta_k2q2_dn8 = (0.5 * (locals.var_k2q2d_dn8 - locals.var_k2q2s_dn8));
        locals.var_delta_k2q2_dn9 = (0.5 * (locals.var_k2q2d_dn9 - locals.var_k2q2s_dn9));
        locals.var_delta_k2q2_rv = 0.0;

        let assign22510_e22712: f64 = (locals.var_delta_k1q1 * locals.var_inv_k1h1);
        locals.var_prod1 = assign22510_e22712;
        locals.var_prod1_dn4 = ((locals.var_delta_k1q1_dn4 * locals.var_inv_k1h1) + (locals.var_delta_k1q1 * locals.var_inv_k1h1_dn4));
        locals.var_prod1_dn6 = ((locals.var_delta_k1q1_dn6 * locals.var_inv_k1h1) + (locals.var_delta_k1q1 * locals.var_inv_k1h1_dn6));
        locals.var_prod1_dn7 = ((locals.var_delta_k1q1_dn7 * locals.var_inv_k1h1) + (locals.var_delta_k1q1 * locals.var_inv_k1h1_dn7));
        locals.var_prod1_dn8 = ((locals.var_delta_k1q1_dn8 * locals.var_inv_k1h1) + (locals.var_delta_k1q1 * locals.var_inv_k1h1_dn8));
        locals.var_prod1_dn9 = ((locals.var_delta_k1q1_dn9 * locals.var_inv_k1h1) + (locals.var_delta_k1q1 * locals.var_inv_k1h1_dn9));
        locals.var_prod1_rv = 0.0;

        let assign22520_e22715: f64 = (locals.var_delta_k2q2 * locals.var_inv_k2h2);
        locals.var_prod2 = assign22520_e22715;
        locals.var_prod2_dn4 = ((locals.var_delta_k2q2_dn4 * locals.var_inv_k2h2) + (locals.var_delta_k2q2 * locals.var_inv_k2h2_dn4));
        locals.var_prod2_dn6 = ((locals.var_delta_k2q2_dn6 * locals.var_inv_k2h2) + (locals.var_delta_k2q2 * locals.var_inv_k2h2_dn6));
        locals.var_prod2_dn7 = ((locals.var_delta_k2q2_dn7 * locals.var_inv_k2h2) + (locals.var_delta_k2q2 * locals.var_inv_k2h2_dn7));
        locals.var_prod2_dn8 = ((locals.var_delta_k2q2_dn8 * locals.var_inv_k2h2) + (locals.var_delta_k2q2 * locals.var_inv_k2h2_dn8));
        locals.var_prod2_dn9 = ((locals.var_delta_k2q2_dn9 * locals.var_inv_k2h2) + (locals.var_delta_k2q2 * locals.var_inv_k2h2_dn9));
        locals.var_prod2_rv = 0.0;

        locals.var_xg20shift_dc = locals.var_xg20shift;
        locals.var_xg20shift_dc_dn4 = locals.var_xg20shift_dn4;
        locals.var_xg20shift_dc_dn6 = locals.var_xg20shift_dn6;
        locals.var_xg20shift_dc_dn7 = locals.var_xg20shift_dn7;
        locals.var_xg20shift_dc_dn8 = locals.var_xg20shift_dn8;
        locals.var_xg20shift_dc_dn9 = locals.var_xg20shift_dn9;
        locals.var_xg20shift_dc_rv = 0.0;

        locals.var_diff_min_dc = locals.var_diff_min;
        locals.var_diff_min_dc_dn4 = locals.var_diff_min_dn4;
        locals.var_diff_min_dc_dn6 = locals.var_diff_min_dn6;
        locals.var_diff_min_dc_dn7 = locals.var_diff_min_dn7;
        locals.var_diff_min_dc_dn8 = locals.var_diff_min_dn8;
        locals.var_diff_min_dc_dn9 = locals.var_diff_min_dn9;
        locals.var_diff_min_dc_rv = 0.0;

        locals.var_a0_dc = locals.var_a0;
        locals.var_a0_dc_dn4 = locals.var_a0_dn4;
        locals.var_a0_dc_dn6 = locals.var_a0_dn6;
        locals.var_a0_dc_dn7 = locals.var_a0_dn7;
        locals.var_a0_dc_dn8 = locals.var_a0_dn8;
        locals.var_a0_dc_dn9 = locals.var_a0_dn9;
        locals.var_a0_dc_rv = 0.0;

        locals.var_inv_k1_dc = locals.var_inv_k1;
        locals.var_inv_k1_dc_dn4 = locals.var_inv_k1_dn4;
        locals.var_inv_k1_dc_dn6 = locals.var_inv_k1_dn6;
        locals.var_inv_k1_dc_dn7 = locals.var_inv_k1_dn7;
        locals.var_inv_k1_dc_dn8 = locals.var_inv_k1_dn8;
        locals.var_inv_k1_dc_dn9 = locals.var_inv_k1_dn9;
        locals.var_inv_k1_dc_rv = 0.0;

        locals.var_inv_k2_dc = locals.var_inv_k2;
        locals.var_inv_k2_dc_dn4 = locals.var_inv_k2_dn4;
        locals.var_inv_k2_dc_dn6 = locals.var_inv_k2_dn6;
        locals.var_inv_k2_dc_dn7 = locals.var_inv_k2_dn7;
        locals.var_inv_k2_dc_dn8 = locals.var_inv_k2_dn8;
        locals.var_inv_k2_dc_dn9 = locals.var_inv_k2_dn9;
        locals.var_inv_k2_dc_rv = 0.0;

        locals.var_keq_dc = locals.var_keq;
        locals.var_keq_dc_dn4 = locals.var_keq_dn4;
        locals.var_keq_dc_dn6 = locals.var_keq_dn6;
        locals.var_keq_dc_dn7 = locals.var_keq_dn7;
        locals.var_keq_dc_dn8 = locals.var_keq_dn8;
        locals.var_keq_dc_dn9 = locals.var_keq_dn9;
        locals.var_keq_dc_rv = 0.0;

        locals.var_dx_wi_dc = locals.var_dx_wi;
        locals.var_dx_wi_dc_dn4 = locals.var_dx_wi_dn4;
        locals.var_dx_wi_dc_dn6 = locals.var_dx_wi_dn6;
        locals.var_dx_wi_dc_dn7 = locals.var_dx_wi_dn7;
        locals.var_dx_wi_dc_dn8 = locals.var_dx_wi_dn8;
        locals.var_dx_wi_dc_dn9 = locals.var_dx_wi_dn9;
        locals.var_dx_wi_dc_rv = 0.0;

        locals.var_csiprime_dc = locals.var_csiprime;
        locals.var_csiprime_dc_dn4 = locals.var_csiprime_dn4;
        locals.var_csiprime_dc_dn6 = locals.var_csiprime_dn6;
        locals.var_csiprime_dc_dn7 = locals.var_csiprime_dn7;
        locals.var_csiprime_dc_dn8 = locals.var_csiprime_dn8;
        locals.var_csiprime_dc_dn9 = locals.var_csiprime_dn9;
        locals.var_csiprime_dc_rv = 0.0;

        locals.var_dx_wi_1d_dc = locals.var_dx_wi_1d;
        locals.var_dx_wi_1d_dc_dn4 = locals.var_dx_wi_1d_dn4;
        locals.var_dx_wi_1d_dc_dn6 = locals.var_dx_wi_1d_dn6;
        locals.var_dx_wi_1d_dc_dn7 = locals.var_dx_wi_1d_dn7;
        locals.var_dx_wi_1d_dc_dn8 = locals.var_dx_wi_1d_dn8;
        locals.var_dx_wi_1d_dc_dn9 = locals.var_dx_wi_1d_dn9;
        locals.var_dx_wi_1d_dc_rv = 0.0;

        locals.var_q1s_dc = locals.var_q1s;
        locals.var_q1s_dc_dn4 = locals.var_q1s_dn4;
        locals.var_q1s_dc_dn6 = locals.var_q1s_dn6;
        locals.var_q1s_dc_dn7 = locals.var_q1s_dn7;
        locals.var_q1s_dc_dn8 = locals.var_q1s_dn8;
        locals.var_q1s_dc_dn9 = locals.var_q1s_dn9;
        locals.var_q1s_dc_rv = 0.0;

        locals.var_dleff_dc = locals.var_dleff;
        locals.var_dleff_dc_dn4 = locals.var_dleff_dn4;
        locals.var_dleff_dc_dn6 = locals.var_dleff_dn6;
        locals.var_dleff_dc_dn7 = locals.var_dleff_dn7;
        locals.var_dleff_dc_dn8 = locals.var_dleff_dn8;
        locals.var_dleff_dc_dn9 = locals.var_dleff_dn9;
        locals.var_dleff_dc_rv = 0.0;

        locals.var_xedge_dc = locals.var_xedge;
        locals.var_xedge_dc_dn4 = locals.var_xedge_dn4;
        locals.var_xedge_dc_dn6 = locals.var_xedge_dn6;
        locals.var_xedge_dc_dn7 = locals.var_xedge_dn7;
        locals.var_xedge_dc_dn8 = locals.var_xedge_dn8;
        locals.var_xedge_dc_dn9 = locals.var_xedge_dn9;
        locals.var_xedge_dc_rv = 0.0;

        locals.var_sce1_dc = locals.var_sce1;
        locals.var_sce1_dc_dn4 = locals.var_sce1_dn4;
        locals.var_sce1_dc_dn6 = locals.var_sce1_dn6;
        locals.var_sce1_dc_dn7 = locals.var_sce1_dn7;
        locals.var_sce1_dc_dn8 = locals.var_sce1_dn8;
        locals.var_sce1_dc_dn9 = locals.var_sce1_dn9;
        locals.var_sce1_dc_rv = 0.0;

        locals.var_sce2_dc = locals.var_sce2;
        locals.var_sce2_dc_dn4 = locals.var_sce2_dn4;
        locals.var_sce2_dc_dn6 = locals.var_sce2_dn6;
        locals.var_sce2_dc_dn7 = locals.var_sce2_dn7;
        locals.var_sce2_dc_dn8 = locals.var_sce2_dn8;
        locals.var_sce2_dc_dn9 = locals.var_sce2_dn9;
        locals.var_sce2_dc_rv = 0.0;

        locals.var_dxg1_dibl_dc = locals.var_dxg1_dibl;
        locals.var_dxg1_dibl_dc_dn4 = locals.var_dxg1_dibl_dn4;
        locals.var_dxg1_dibl_dc_dn6 = locals.var_dxg1_dibl_dn6;
        locals.var_dxg1_dibl_dc_dn7 = locals.var_dxg1_dibl_dn7;
        locals.var_dxg1_dibl_dc_dn8 = locals.var_dxg1_dibl_dn8;
        locals.var_dxg1_dibl_dc_dn9 = locals.var_dxg1_dibl_dn9;
        locals.var_dxg1_dibl_dc_rv = 0.0;

        locals.var_xg2_dc = locals.var_xg2;
        locals.var_xg2_dc_dn4 = locals.var_xg2_dn4;
        locals.var_xg2_dc_dn6 = locals.var_xg2_dn6;
        locals.var_xg2_dc_dn7 = locals.var_xg2_dn7;
        locals.var_xg2_dc_dn8 = locals.var_xg2_dn8;
        locals.var_xg2_dc_dn9 = locals.var_xg2_dn9;
        locals.var_xg2_dc_rv = 0.0;

        locals.var_xg2x_dc = locals.var_xg2x;
        locals.var_xg2x_dc_dn4 = locals.var_xg2x_dn4;
        locals.var_xg2x_dc_dn6 = locals.var_xg2x_dn6;
        locals.var_xg2x_dc_dn7 = locals.var_xg2x_dn7;
        locals.var_xg2x_dc_dn8 = locals.var_xg2x_dn8;
        locals.var_xg2x_dc_dn9 = locals.var_xg2x_dn9;
        locals.var_xg2x_dc_rv = 0.0;

        locals.var_xg1x_dc = locals.var_xg1x;
        locals.var_xg1x_dc_dn4 = locals.var_xg1x_dn4;
        locals.var_xg1x_dc_dn6 = locals.var_xg1x_dn6;
        locals.var_xg1x_dc_dn7 = locals.var_xg1x_dn7;
        locals.var_xg1x_dc_dn8 = locals.var_xg1x_dn8;
        locals.var_xg1x_dc_dn9 = locals.var_xg1x_dn9;
        locals.var_xg1x_dc_rv = 0.0;

        locals.var_k1_dc = locals.var_k1;
        locals.var_k1_dc_dn4 = locals.var_k1_dn4;
        locals.var_k1_dc_dn6 = locals.var_k1_dn6;
        locals.var_k1_dc_dn7 = locals.var_k1_dn7;
        locals.var_k1_dc_dn8 = locals.var_k1_dn8;
        locals.var_k1_dc_dn9 = locals.var_k1_dn9;
        locals.var_k1_dc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_62(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_k2_dc = locals.var_k2;
        locals.var_k2_dc_dn4 = locals.var_k2_dn4;
        locals.var_k2_dc_dn6 = locals.var_k2_dn6;
        locals.var_k2_dc_dn7 = locals.var_k2_dn7;
        locals.var_k2_dc_dn8 = locals.var_k2_dn8;
        locals.var_k2_dc_dn9 = locals.var_k2_dn9;
        locals.var_k2_dc_rv = 0.0;

        locals.var_qis_dc = locals.var_qis;
        locals.var_qis_dc_dn4 = locals.var_qis_dn4;
        locals.var_qis_dc_dn6 = locals.var_qis_dn6;
        locals.var_qis_dc_dn7 = locals.var_qis_dn7;
        locals.var_qis_dc_dn8 = locals.var_qis_dn8;
        locals.var_qis_dc_dn9 = locals.var_qis_dn9;
        locals.var_qis_dc_rv = 0.0;

        locals.var_k1q1s_dc = locals.var_k1q1s;
        locals.var_k1q1s_dc_dn4 = locals.var_k1q1s_dn4;
        locals.var_k1q1s_dc_dn6 = locals.var_k1q1s_dn6;
        locals.var_k1q1s_dc_dn7 = locals.var_k1q1s_dn7;
        locals.var_k1q1s_dc_dn8 = locals.var_k1q1s_dn8;
        locals.var_k1q1s_dc_dn9 = locals.var_k1q1s_dn9;
        locals.var_k1q1s_dc_rv = 0.0;

        locals.var_k2q2s_dc = locals.var_k2q2s;
        locals.var_k2q2s_dc_dn4 = locals.var_k2q2s_dn4;
        locals.var_k2q2s_dc_dn6 = locals.var_k2q2s_dn6;
        locals.var_k2q2s_dc_dn7 = locals.var_k2q2s_dn7;
        locals.var_k2q2s_dc_dn8 = locals.var_k2q2s_dn8;
        locals.var_k2q2s_dc_dn9 = locals.var_k2q2s_dn9;
        locals.var_k2q2s_dc_rv = 0.0;

        locals.var_xdrifts_dc = locals.var_xdrifts;
        locals.var_xdrifts_dc_dn4 = locals.var_xdrifts_dn4;
        locals.var_xdrifts_dc_dn6 = locals.var_xdrifts_dn6;
        locals.var_xdrifts_dc_dn7 = locals.var_xdrifts_dn7;
        locals.var_xdrifts_dc_dn8 = locals.var_xdrifts_dn8;
        locals.var_xdrifts_dc_dn9 = locals.var_xdrifts_dn9;
        locals.var_xdrifts_dc_rv = 0.0;

        locals.var_ds_dc = locals.var_ds;
        locals.var_ds_dc_dn4 = locals.var_ds_dn4;
        locals.var_ds_dc_dn6 = locals.var_ds_dn6;
        locals.var_ds_dc_dn7 = locals.var_ds_dn7;
        locals.var_ds_dc_dn8 = locals.var_ds_dn8;
        locals.var_ds_dc_dn9 = locals.var_ds_dn9;
        locals.var_ds_dc_rv = 0.0;

        locals.var_k1q1d_dc = locals.var_k1q1d;
        locals.var_k1q1d_dc_dn4 = locals.var_k1q1d_dn4;
        locals.var_k1q1d_dc_dn6 = locals.var_k1q1d_dn6;
        locals.var_k1q1d_dc_dn7 = locals.var_k1q1d_dn7;
        locals.var_k1q1d_dc_dn8 = locals.var_k1q1d_dn8;
        locals.var_k1q1d_dc_dn9 = locals.var_k1q1d_dn9;
        locals.var_k1q1d_dc_rv = 0.0;

        locals.var_k2q2d_dc = locals.var_k2q2d;
        locals.var_k2q2d_dc_dn4 = locals.var_k2q2d_dn4;
        locals.var_k2q2d_dc_dn6 = locals.var_k2q2d_dn6;
        locals.var_k2q2d_dc_dn7 = locals.var_k2q2d_dn7;
        locals.var_k2q2d_dc_dn8 = locals.var_k2q2d_dn8;
        locals.var_k2q2d_dc_dn9 = locals.var_k2q2d_dn9;
        locals.var_k2q2d_dc_rv = 0.0;

        locals.var_xdeff_dc = locals.var_xdeff;
        locals.var_xdeff_dc_dn4 = locals.var_xdeff_dn4;
        locals.var_xdeff_dc_dn6 = locals.var_xdeff_dn6;
        locals.var_xdeff_dc_dn7 = locals.var_xdeff_dn7;
        locals.var_xdeff_dc_dn8 = locals.var_xdeff_dn8;
        locals.var_xdeff_dc_dn9 = locals.var_xdeff_dn9;
        locals.var_xdeff_dc_rv = 0.0;

        locals.var_q1d_dc = locals.var_q1d;
        locals.var_q1d_dc_dn4 = locals.var_q1d_dn4;
        locals.var_q1d_dc_dn6 = locals.var_q1d_dn6;
        locals.var_q1d_dc_dn7 = locals.var_q1d_dn7;
        locals.var_q1d_dc_dn8 = locals.var_q1d_dn8;
        locals.var_q1d_dc_dn9 = locals.var_q1d_dn9;
        locals.var_q1d_dc_rv = 0.0;

        locals.var_qid_dc = locals.var_qid;
        locals.var_qid_dc_dn4 = locals.var_qid_dn4;
        locals.var_qid_dc_dn6 = locals.var_qid_dn6;
        locals.var_qid_dc_dn7 = locals.var_qid_dn7;
        locals.var_qid_dc_dn8 = locals.var_qid_dn8;
        locals.var_qid_dc_dn9 = locals.var_qid_dn9;
        locals.var_qid_dc_rv = 0.0;

        locals.var_xdriftd_dc = locals.var_xdriftd;
        locals.var_xdriftd_dc_dn4 = locals.var_xdriftd_dn4;
        locals.var_xdriftd_dc_dn6 = locals.var_xdriftd_dn6;
        locals.var_xdriftd_dc_dn7 = locals.var_xdriftd_dn7;
        locals.var_xdriftd_dc_dn8 = locals.var_xdriftd_dn8;
        locals.var_xdriftd_dc_dn9 = locals.var_xdriftd_dn9;
        locals.var_xdriftd_dc_rv = 0.0;

        locals.var_qim_dc = locals.var_qim;
        locals.var_qim_dc_dn4 = locals.var_qim_dn4;
        locals.var_qim_dc_dn6 = locals.var_qim_dn6;
        locals.var_qim_dc_dn7 = locals.var_qim_dn7;
        locals.var_qim_dc_dn8 = locals.var_qim_dn8;
        locals.var_qim_dc_dn9 = locals.var_qim_dn9;
        locals.var_qim_dc_rv = 0.0;

        locals.var_ratio_pd_dc = locals.var_ratio_pd;
        locals.var_ratio_pd_dc_dn4 = locals.var_ratio_pd_dn4;
        locals.var_ratio_pd_dc_dn6 = locals.var_ratio_pd_dn6;
        locals.var_ratio_pd_dc_dn7 = locals.var_ratio_pd_dn7;
        locals.var_ratio_pd_dc_dn8 = locals.var_ratio_pd_dn8;
        locals.var_ratio_pd_dc_dn9 = locals.var_ratio_pd_dn9;
        locals.var_ratio_pd_dc_rv = 0.0;

        locals.var_esurf1_dc = locals.var_esurf1;
        locals.var_esurf1_dc_dn4 = locals.var_esurf1_dn4;
        locals.var_esurf1_dc_dn6 = locals.var_esurf1_dn6;
        locals.var_esurf1_dc_dn7 = locals.var_esurf1_dn7;
        locals.var_esurf1_dc_dn8 = locals.var_esurf1_dn8;
        locals.var_esurf1_dc_dn9 = locals.var_esurf1_dn9;
        locals.var_esurf1_dc_rv = 0.0;

        locals.var_esurf2_dc = locals.var_esurf2;
        locals.var_esurf2_dc_dn4 = locals.var_esurf2_dn4;
        locals.var_esurf2_dc_dn6 = locals.var_esurf2_dn6;
        locals.var_esurf2_dc_dn7 = locals.var_esurf2_dn7;
        locals.var_esurf2_dc_dn8 = locals.var_esurf2_dn8;
        locals.var_esurf2_dc_dn9 = locals.var_esurf2_dn9;
        locals.var_esurf2_dc_rv = 0.0;

        locals.var_qi1m_dc = locals.var_qi1m;
        locals.var_qi1m_dc_dn4 = locals.var_qi1m_dn4;
        locals.var_qi1m_dc_dn6 = locals.var_qi1m_dn6;
        locals.var_qi1m_dc_dn7 = locals.var_qi1m_dn7;
        locals.var_qi1m_dc_dn8 = locals.var_qi1m_dn8;
        locals.var_qi1m_dc_dn9 = locals.var_qi1m_dn9;
        locals.var_qi1m_dc_rv = 0.0;

        locals.var_qi2m_dc = locals.var_qi2m;
        locals.var_qi2m_dc_dn4 = locals.var_qi2m_dn4;
        locals.var_qi2m_dc_dn6 = locals.var_qi2m_dn6;
        locals.var_qi2m_dc_dn7 = locals.var_qi2m_dn7;
        locals.var_qi2m_dc_dn8 = locals.var_qi2m_dn8;
        locals.var_qi2m_dc_dn9 = locals.var_qi2m_dn9;
        locals.var_qi2m_dc_rv = 0.0;

        locals.var_csum_dc = locals.var_csum;
        locals.var_csum_dc_dn4 = locals.var_csum_dn4;
        locals.var_csum_dc_dn6 = locals.var_csum_dn6;
        locals.var_csum_dc_dn7 = locals.var_csum_dn7;
        locals.var_csum_dc_dn8 = locals.var_csum_dn8;
        locals.var_csum_dc_dn9 = locals.var_csum_dn9;
        locals.var_csum_dc_rv = 0.0;

        locals.var_gmob_dc = locals.var_gmob;
        locals.var_gmob_dc_dn4 = locals.var_gmob_dn4;
        locals.var_gmob_dc_dn6 = locals.var_gmob_dn6;
        locals.var_gmob_dc_dn7 = locals.var_gmob_dn7;
        locals.var_gmob_dc_dn8 = locals.var_gmob_dn8;
        locals.var_gmob_dc_dn9 = locals.var_gmob_dn9;
        locals.var_gmob_dc_rv = 0.0;

        locals.var_inv_qimstar1_dc = locals.var_inv_qimstar1;
        locals.var_inv_qimstar1_dc_dn4 = locals.var_inv_qimstar1_dn4;
        locals.var_inv_qimstar1_dc_dn6 = locals.var_inv_qimstar1_dn6;
        locals.var_inv_qimstar1_dc_dn7 = locals.var_inv_qimstar1_dn7;
        locals.var_inv_qimstar1_dc_dn8 = locals.var_inv_qimstar1_dn8;
        locals.var_inv_qimstar1_dc_dn9 = locals.var_inv_qimstar1_dn9;
        locals.var_inv_qimstar1_dc_rv = 0.0;

        locals.var_dl_l_fact_dc = locals.var_dl_l_fact;
        locals.var_dl_l_fact_dc_dn4 = locals.var_dl_l_fact_dn4;
        locals.var_dl_l_fact_dc_dn6 = locals.var_dl_l_fact_dn6;
        locals.var_dl_l_fact_dc_dn7 = locals.var_dl_l_fact_dn7;
        locals.var_dl_l_fact_dc_dn8 = locals.var_dl_l_fact_dn8;
        locals.var_dl_l_fact_dc_dn9 = locals.var_dl_l_fact_dn9;
        locals.var_dl_l_fact_dc_rv = 0.0;

        locals.var_gdl_dc = locals.var_gdl;
        locals.var_gdl_dc_dn4 = locals.var_gdl_dn4;
        locals.var_gdl_dc_dn6 = locals.var_gdl_dn6;
        locals.var_gdl_dc_dn7 = locals.var_gdl_dn7;
        locals.var_gdl_dc_dn8 = locals.var_gdl_dn8;
        locals.var_gdl_dc_dn9 = locals.var_gdl_dn9;
        locals.var_gdl_dc_rv = 0.0;

        locals.var_vsat_fact_dc = locals.var_vsat_fact;
        locals.var_vsat_fact_dc_dn4 = locals.var_vsat_fact_dn4;
        locals.var_vsat_fact_dc_dn6 = locals.var_vsat_fact_dn6;
        locals.var_vsat_fact_dc_dn7 = locals.var_vsat_fact_dn7;
        locals.var_vsat_fact_dc_dn8 = locals.var_vsat_fact_dn8;
        locals.var_vsat_fact_dc_dn9 = locals.var_vsat_fact_dn9;
        locals.var_vsat_fact_dc_rv = 0.0;

        locals.var_zsat_dc = locals.var_zsat;
        locals.var_zsat_dc_dn4 = locals.var_zsat_dn4;
        locals.var_zsat_dc_dn6 = locals.var_zsat_dn6;
        locals.var_zsat_dc_dn7 = locals.var_zsat_dn7;
        locals.var_zsat_dc_dn8 = locals.var_zsat_dn8;
        locals.var_zsat_dc_dn9 = locals.var_zsat_dn9;
        locals.var_zsat_dc_rv = 0.0;

        locals.var_hsat_dc = locals.var_hsat;
        locals.var_hsat_dc_dn4 = locals.var_hsat_dn4;
        locals.var_hsat_dc_dn6 = locals.var_hsat_dn6;
        locals.var_hsat_dc_dn7 = locals.var_hsat_dn7;
        locals.var_hsat_dc_dn8 = locals.var_hsat_dn8;
        locals.var_hsat_dc_dn9 = locals.var_hsat_dn9;
        locals.var_hsat_dc_rv = 0.0;

        locals.var_qmfact1_dc = locals.var_qmfact1;
        locals.var_qmfact1_dc_dn4 = locals.var_qmfact1_dn4;
        locals.var_qmfact1_dc_dn6 = locals.var_qmfact1_dn6;
        locals.var_qmfact1_dc_dn7 = locals.var_qmfact1_dn7;
        locals.var_qmfact1_dc_dn8 = locals.var_qmfact1_dn8;
        locals.var_qmfact1_dc_dn9 = locals.var_qmfact1_dn9;
        locals.var_qmfact1_dc_rv = 0.0;

        locals.var_qmfact2_dc = locals.var_qmfact2;
        locals.var_qmfact2_dc_dn4 = locals.var_qmfact2_dn4;
        locals.var_qmfact2_dc_dn6 = locals.var_qmfact2_dn6;
        locals.var_qmfact2_dc_dn7 = locals.var_qmfact2_dn7;
        locals.var_qmfact2_dc_dn8 = locals.var_qmfact2_dn8;
        locals.var_qmfact2_dc_dn9 = locals.var_qmfact2_dn9;
        locals.var_qmfact2_dc_rv = 0.0;

        locals.var_dd_dc = locals.var_dd;
        locals.var_dd_dc_dn4 = locals.var_dd_dn4;
        locals.var_dd_dc_dn6 = locals.var_dd_dn6;
        locals.var_dd_dc_dn7 = locals.var_dd_dn7;
        locals.var_dd_dc_dn8 = locals.var_dd_dn8;
        locals.var_dd_dc_dn9 = locals.var_dd_dn9;
        locals.var_dd_dc_rv = 0.0;

        locals.var_norm_ids_dc = locals.var_norm_ids;
        locals.var_norm_ids_dc_dn4 = locals.var_norm_ids_dn4;
        locals.var_norm_ids_dc_dn6 = locals.var_norm_ids_dn6;
        locals.var_norm_ids_dc_dn7 = locals.var_norm_ids_dn7;
        locals.var_norm_ids_dc_dn8 = locals.var_norm_ids_dn8;
        locals.var_norm_ids_dc_dn9 = locals.var_norm_ids_dn9;
        locals.var_norm_ids_dc_rv = 0.0;

        locals.var_inv_k1h1_0_dc = locals.var_inv_k1h1_0;
        locals.var_inv_k1h1_0_dc_dn4 = locals.var_inv_k1h1_0_dn4;
        locals.var_inv_k1h1_0_dc_dn6 = locals.var_inv_k1h1_0_dn6;
        locals.var_inv_k1h1_0_dc_dn7 = locals.var_inv_k1h1_0_dn7;
        locals.var_inv_k1h1_0_dc_dn8 = locals.var_inv_k1h1_0_dn8;
        locals.var_inv_k1h1_0_dc_dn9 = locals.var_inv_k1h1_0_dn9;
        locals.var_inv_k1h1_0_dc_rv = 0.0;

        locals.var_delta_k1q1_dc = locals.var_delta_k1q1;
        locals.var_delta_k1q1_dc_dn4 = locals.var_delta_k1q1_dn4;
        locals.var_delta_k1q1_dc_dn6 = locals.var_delta_k1q1_dn6;
        locals.var_delta_k1q1_dc_dn7 = locals.var_delta_k1q1_dn7;
        locals.var_delta_k1q1_dc_dn8 = locals.var_delta_k1q1_dn8;
        locals.var_delta_k1q1_dc_dn9 = locals.var_delta_k1q1_dn9;
        locals.var_delta_k1q1_dc_rv = 0.0;

        locals.var_delta_k2q2_dc = locals.var_delta_k2q2;
        locals.var_delta_k2q2_dc_dn4 = locals.var_delta_k2q2_dn4;
        locals.var_delta_k2q2_dc_dn6 = locals.var_delta_k2q2_dn6;
        locals.var_delta_k2q2_dc_dn7 = locals.var_delta_k2q2_dn7;
        locals.var_delta_k2q2_dc_dn8 = locals.var_delta_k2q2_dn8;
        locals.var_delta_k2q2_dc_dn9 = locals.var_delta_k2q2_dn9;
        locals.var_delta_k2q2_dc_rv = 0.0;

        locals.var_prod1_dc = locals.var_prod1;
        locals.var_prod1_dc_dn4 = locals.var_prod1_dn4;
        locals.var_prod1_dc_dn6 = locals.var_prod1_dn6;
        locals.var_prod1_dc_dn7 = locals.var_prod1_dn7;
        locals.var_prod1_dc_dn8 = locals.var_prod1_dn8;
        locals.var_prod1_dc_dn9 = locals.var_prod1_dn9;
        locals.var_prod1_dc_rv = 0.0;

        locals.var_prod2_dc = locals.var_prod2;
        locals.var_prod2_dc_dn4 = locals.var_prod2_dn4;
        locals.var_prod2_dc_dn6 = locals.var_prod2_dn6;
        locals.var_prod2_dc_dn7 = locals.var_prod2_dn7;
        locals.var_prod2_dc_dn8 = locals.var_prod2_dn8;
        locals.var_prod2_dc_dn9 = locals.var_prod2_dn9;
        locals.var_prod2_dc_rv = 0.0;

        let assign23100_e22775: f64 = (locals.var_csum_dc * p.p35);
        let assign23100_e22778: f64 = (locals.var_esurf1_dc + locals.var_esurf2_dc);
        let assign23100_e22779: f64 = (assign23100_e22775 / assign23100_e22778);
        locals.var_betneff = assign23100_e22779;
        locals.var_betneff_dn4 = ((((locals.var_csum_dc_dn4 * p.p35) * assign23100_e22778) - (assign23100_e22775 * (locals.var_esurf1_dc_dn4 + locals.var_esurf2_dc_dn4))) / (assign23100_e22778 * assign23100_e22778));
        locals.var_betneff_dn6 = ((((locals.var_csum_dc_dn6 * p.p35) * assign23100_e22778) - (assign23100_e22775 * (locals.var_esurf1_dc_dn6 + locals.var_esurf2_dc_dn6))) / (assign23100_e22778 * assign23100_e22778));
        locals.var_betneff_dn7 = ((((locals.var_csum_dc_dn7 * p.p35) * assign23100_e22778) - (assign23100_e22775 * (locals.var_esurf1_dc_dn7 + locals.var_esurf2_dc_dn7))) / (assign23100_e22778 * assign23100_e22778));
        locals.var_betneff_dn8 = ((((locals.var_csum_dc_dn8 * p.p35) * assign23100_e22778) - (assign23100_e22775 * (locals.var_esurf1_dc_dn8 + locals.var_esurf2_dc_dn8))) / (assign23100_e22778 * assign23100_e22778));
        locals.var_betneff_dn9 = ((((locals.var_csum_dc_dn9 * p.p35) * assign23100_e22778) - (assign23100_e22775 * (locals.var_esurf1_dc_dn9 + locals.var_esurf2_dc_dn9))) / (assign23100_e22778 * assign23100_e22778));
        locals.var_betneff_rv = 0.0;

        let assign23110_e22783: f64 = (locals.var_alp1_phit * locals.var_inv_qimstar1_dc);
        let assign23110_e22784: f64 = (locals.var_alp_i + assign23110_e22783);
        let assign23110_e22786: f64 = (assign23110_e22784 * locals.var_dl_l_fact_dc);
        locals.var_dl1_l = assign23110_e22786;
        locals.var_dl1_l_dn4 = ((((locals.var_alp1_phit_dn4 * locals.var_inv_qimstar1_dc) + (locals.var_alp1_phit * locals.var_inv_qimstar1_dc_dn4)) * locals.var_dl_l_fact_dc) + (assign23110_e22784 * locals.var_dl_l_fact_dc_dn4));
        locals.var_dl1_l_dn6 = ((((locals.var_alp1_phit_dn6 * locals.var_inv_qimstar1_dc) + (locals.var_alp1_phit * locals.var_inv_qimstar1_dc_dn6)) * locals.var_dl_l_fact_dc) + (assign23110_e22784 * locals.var_dl_l_fact_dc_dn6));
        locals.var_dl1_l_dn7 = ((((locals.var_alp1_phit_dn7 * locals.var_inv_qimstar1_dc) + (locals.var_alp1_phit * locals.var_inv_qimstar1_dc_dn7)) * locals.var_dl_l_fact_dc) + (assign23110_e22784 * locals.var_dl_l_fact_dc_dn7));
        locals.var_dl1_l_dn8 = ((((locals.var_alp1_phit_dn8 * locals.var_inv_qimstar1_dc) + (locals.var_alp1_phit * locals.var_inv_qimstar1_dc_dn8)) * locals.var_dl_l_fact_dc) + (assign23110_e22784 * locals.var_dl_l_fact_dc_dn8));
        locals.var_dl1_l_dn9 = ((((locals.var_alp1_phit_dn9 * locals.var_inv_qimstar1_dc) + (locals.var_alp1_phit * locals.var_inv_qimstar1_dc_dn9)) * locals.var_dl_l_fact_dc) + (assign23110_e22784 * locals.var_dl_l_fact_dc_dn9));
        locals.var_dl1_l_rv = 0.0;

        let assign23120_e22791: f64 = (1.0 + locals.var_dl1_l);
        let assign23120_e22792: f64 = (locals.var_dl1_l * assign23120_e22791);
        let assign23120_e22793: f64 = (1.0 + assign23120_e22792);
        let assign23120_e22795: f64 = (assign23120_e22793 * locals.var_gdl_dc);
        locals.var_fdl = assign23120_e22795;
        locals.var_fdl_dn4 = ((((locals.var_dl1_l_dn4 * assign23120_e22791) + (locals.var_dl1_l * locals.var_dl1_l_dn4)) * locals.var_gdl_dc) + (assign23120_e22793 * locals.var_gdl_dc_dn4));
        locals.var_fdl_dn6 = ((((locals.var_dl1_l_dn6 * assign23120_e22791) + (locals.var_dl1_l * locals.var_dl1_l_dn6)) * locals.var_gdl_dc) + (assign23120_e22793 * locals.var_gdl_dc_dn6));
        locals.var_fdl_dn7 = ((((locals.var_dl1_l_dn7 * assign23120_e22791) + (locals.var_dl1_l * locals.var_dl1_l_dn7)) * locals.var_gdl_dc) + (assign23120_e22793 * locals.var_gdl_dc_dn7));
        locals.var_fdl_dn8 = ((((locals.var_dl1_l_dn8 * assign23120_e22791) + (locals.var_dl1_l * locals.var_dl1_l_dn8)) * locals.var_gdl_dc) + (assign23120_e22793 * locals.var_gdl_dc_dn8));
        locals.var_fdl_dn9 = ((((locals.var_dl1_l_dn9 * assign23120_e22791) + (locals.var_dl1_l * locals.var_dl1_l_dn9)) * locals.var_gdl_dc) + (assign23120_e22793 * locals.var_gdl_dc_dn9));
        locals.var_fdl_rv = 0.0;

        let assign23130_e22798: f64 = (locals.var_gmob_dc * locals.var_gdl_dc);
        let assign23130_e22800: f64 = (assign23130_e22798 * locals.var_vsat_fact_dc);
        locals.var_gvsat = assign23130_e22800;
        locals.var_gvsat_dn4 = ((((locals.var_gmob_dc_dn4 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn4)) * locals.var_vsat_fact_dc) + (assign23130_e22798 * locals.var_vsat_fact_dc_dn4));
        locals.var_gvsat_dn6 = ((((locals.var_gmob_dc_dn6 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn6)) * locals.var_vsat_fact_dc) + (assign23130_e22798 * locals.var_vsat_fact_dc_dn6));
        locals.var_gvsat_dn7 = ((((locals.var_gmob_dc_dn7 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn7)) * locals.var_vsat_fact_dc) + (assign23130_e22798 * locals.var_vsat_fact_dc_dn7));
        locals.var_gvsat_dn8 = ((((locals.var_gmob_dc_dn8 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn8)) * locals.var_vsat_fact_dc) + (assign23130_e22798 * locals.var_vsat_fact_dc_dn8));
        locals.var_gvsat_dn9 = ((((locals.var_gmob_dc_dn9 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn9)) * locals.var_vsat_fact_dc) + (assign23130_e22798 * locals.var_vsat_fact_dc_dn9));
        locals.var_gvsat_rv = 0.0;

        let assign23140_e22803: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard684 = assign23140_e22803;
        locals.var_guard684_rv = 0.0;

        let (assign23150_e22817, assign23150_e22817_d_n4, assign23150_e22817_d_n6, assign23150_e22817_d_n7, assign23150_e22817_d_n8, assign23150_e22817_d_n9,) = {
    if (locals.var_guard684 != 0.0) {
        let assign23150_e22807: f64 = (locals.var_esurf1_dc + locals.var_esurf2_dc);
        let assign23150_e22810: f64 = (locals.var_esurf1_dc / locals.var_qmfact1_dc);
        let assign23150_e22813: f64 = (locals.var_esurf2_dc / locals.var_qmfact2_dc);
        let assign23150_e22814: f64 = (assign23150_e22810 + assign23150_e22813);
        let assign23150_e22815: f64 = (assign23150_e22807 / assign23150_e22814);
        (assign23150_e22815, ((((locals.var_esurf1_dc_dn4 + locals.var_esurf2_dc_dn4) * assign23150_e22814) - (assign23150_e22807 * ((((locals.var_esurf1_dc_dn4 * locals.var_qmfact1_dc) - (locals.var_esurf1_dc * locals.var_qmfact1_dc_dn4)) / (locals.var_qmfact1_dc * locals.var_qmfact1_dc)) + (((locals.var_esurf2_dc_dn4 * locals.var_qmfact2_dc) - (locals.var_esurf2_dc * locals.var_qmfact2_dc_dn4)) / (locals.var_qmfact2_dc * locals.var_qmfact2_dc))))) / (assign23150_e22814 * assign23150_e22814)), ((((locals.var_esurf1_dc_dn6 + locals.var_esurf2_dc_dn6) * assign23150_e22814) - (assign23150_e22807 * ((((locals.var_esurf1_dc_dn6 * locals.var_qmfact1_dc) - (locals.var_esurf1_dc * locals.var_qmfact1_dc_dn6)) / (locals.var_qmfact1_dc * locals.var_qmfact1_dc)) + (((locals.var_esurf2_dc_dn6 * locals.var_qmfact2_dc) - (locals.var_esurf2_dc * locals.var_qmfact2_dc_dn6)) / (locals.var_qmfact2_dc * locals.var_qmfact2_dc))))) / (assign23150_e22814 * assign23150_e22814)), ((((locals.var_esurf1_dc_dn7 + locals.var_esurf2_dc_dn7) * assign23150_e22814) - (assign23150_e22807 * ((((locals.var_esurf1_dc_dn7 * locals.var_qmfact1_dc) - (locals.var_esurf1_dc * locals.var_qmfact1_dc_dn7)) / (locals.var_qmfact1_dc * locals.var_qmfact1_dc)) + (((locals.var_esurf2_dc_dn7 * locals.var_qmfact2_dc) - (locals.var_esurf2_dc * locals.var_qmfact2_dc_dn7)) / (locals.var_qmfact2_dc * locals.var_qmfact2_dc))))) / (assign23150_e22814 * assign23150_e22814)), ((((locals.var_esurf1_dc_dn8 + locals.var_esurf2_dc_dn8) * assign23150_e22814) - (assign23150_e22807 * ((((locals.var_esurf1_dc_dn8 * locals.var_qmfact1_dc) - (locals.var_esurf1_dc * locals.var_qmfact1_dc_dn8)) / (locals.var_qmfact1_dc * locals.var_qmfact1_dc)) + (((locals.var_esurf2_dc_dn8 * locals.var_qmfact2_dc) - (locals.var_esurf2_dc * locals.var_qmfact2_dc_dn8)) / (locals.var_qmfact2_dc * locals.var_qmfact2_dc))))) / (assign23150_e22814 * assign23150_e22814)), ((((locals.var_esurf1_dc_dn9 + locals.var_esurf2_dc_dn9) * assign23150_e22814) - (assign23150_e22807 * ((((locals.var_esurf1_dc_dn9 * locals.var_qmfact1_dc) - (locals.var_esurf1_dc * locals.var_qmfact1_dc_dn9)) / (locals.var_qmfact1_dc * locals.var_qmfact1_dc)) + (((locals.var_esurf2_dc_dn9 * locals.var_qmfact2_dc) - (locals.var_esurf2_dc * locals.var_qmfact2_dc_dn9)) / (locals.var_qmfact2_dc * locals.var_qmfact2_dc))))) / (assign23150_e22814 * assign23150_e22814)),)
    } else {
        (locals.var_qmfact, locals.var_qmfact_dn4, locals.var_qmfact_dn6, locals.var_qmfact_dn7, locals.var_qmfact_dn8, locals.var_qmfact_dn9,)
    }
};
        locals.var_qmfact = assign23150_e22817;
        locals.var_qmfact_dn4 = assign23150_e22817_d_n4;
        locals.var_qmfact_dn6 = assign23150_e22817_d_n6;
        locals.var_qmfact_dn7 = assign23150_e22817_d_n7;
        locals.var_qmfact_dn8 = assign23150_e22817_d_n8;
        locals.var_qmfact_dn9 = assign23150_e22817_d_n9;
        locals.var_qmfact_rv = 0.0;

        let (assign23160_e22822, assign23160_e22822_d_n4, assign23160_e22822_d_n6, assign23160_e22822_d_n7, assign23160_e22822_d_n8, assign23160_e22822_d_n9,) = {
    if (locals.var_guard684 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qmfact, locals.var_qmfact_dn4, locals.var_qmfact_dn6, locals.var_qmfact_dn7, locals.var_qmfact_dn8, locals.var_qmfact_dn9,)
    }
};
        locals.var_qmfact = assign23160_e22822;
        locals.var_qmfact_dn4 = assign23160_e22822_d_n4;
        locals.var_qmfact_dn6 = assign23160_e22822_d_n6;
        locals.var_qmfact_dn7 = assign23160_e22822_d_n7;
        locals.var_qmfact_dn8 = assign23160_e22822_d_n8;
        locals.var_qmfact_dn9 = assign23160_e22822_d_n9;
        locals.var_qmfact_rv = 0.0;

        let assign23170_e22825: f64 = (locals.var_phit * locals.var_phit);
        let assign23170_e22827: f64 = (assign23170_e22825 * locals.var_betneff);
        locals.var_fact_ids = assign23170_e22827;
        locals.var_fact_ids_dn4 = ((((locals.var_phit_dn4 * locals.var_phit) + (locals.var_phit * locals.var_phit_dn4)) * locals.var_betneff) + (assign23170_e22825 * locals.var_betneff_dn4));
        locals.var_fact_ids_dn6 = ((((locals.var_phit_dn6 * locals.var_phit) + (locals.var_phit * locals.var_phit_dn6)) * locals.var_betneff) + (assign23170_e22825 * locals.var_betneff_dn6));
        locals.var_fact_ids_dn7 = ((((locals.var_phit_dn7 * locals.var_phit) + (locals.var_phit * locals.var_phit_dn7)) * locals.var_betneff) + (assign23170_e22825 * locals.var_betneff_dn7));
        locals.var_fact_ids_dn8 = ((((locals.var_phit_dn8 * locals.var_phit) + (locals.var_phit * locals.var_phit_dn8)) * locals.var_betneff) + (assign23170_e22825 * locals.var_betneff_dn8));
        locals.var_fact_ids_dn9 = ((((locals.var_phit_dn9 * locals.var_phit) + (locals.var_phit * locals.var_phit_dn9)) * locals.var_betneff) + (assign23170_e22825 * locals.var_betneff_dn9));
        locals.var_fact_ids_rv = 0.0;

        let assign23180_e22830: f64 = (locals.var_fact_ids * locals.var_csiprime_dc);
        let assign23180_e22832: f64 = (assign23180_e22830 * locals.var_norm_ids_dc);
        let assign23180_e22834: f64 = (assign23180_e22832 * locals.var_fdl);
        let assign23180_e22836: f64 = (assign23180_e22834 / locals.var_gvsat);
        let assign23180_e22838: f64 = (assign23180_e22836 / locals.var_qmfact);
        locals.var_ids = assign23180_e22838;
        locals.var_ids_dn4 = ((((((((((((locals.var_fact_ids_dn4 * locals.var_csiprime_dc) + (locals.var_fact_ids * locals.var_csiprime_dc_dn4)) * locals.var_norm_ids_dc) + (assign23180_e22830 * locals.var_norm_ids_dc_dn4)) * locals.var_fdl) + (assign23180_e22832 * locals.var_fdl_dn4)) * locals.var_gvsat) - (assign23180_e22834 * locals.var_gvsat_dn4)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign23180_e22836 * locals.var_qmfact_dn4)) / (locals.var_qmfact * locals.var_qmfact));
        locals.var_ids_dn6 = ((((((((((((locals.var_fact_ids_dn6 * locals.var_csiprime_dc) + (locals.var_fact_ids * locals.var_csiprime_dc_dn6)) * locals.var_norm_ids_dc) + (assign23180_e22830 * locals.var_norm_ids_dc_dn6)) * locals.var_fdl) + (assign23180_e22832 * locals.var_fdl_dn6)) * locals.var_gvsat) - (assign23180_e22834 * locals.var_gvsat_dn6)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign23180_e22836 * locals.var_qmfact_dn6)) / (locals.var_qmfact * locals.var_qmfact));
        locals.var_ids_dn7 = ((((((((((((locals.var_fact_ids_dn7 * locals.var_csiprime_dc) + (locals.var_fact_ids * locals.var_csiprime_dc_dn7)) * locals.var_norm_ids_dc) + (assign23180_e22830 * locals.var_norm_ids_dc_dn7)) * locals.var_fdl) + (assign23180_e22832 * locals.var_fdl_dn7)) * locals.var_gvsat) - (assign23180_e22834 * locals.var_gvsat_dn7)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign23180_e22836 * locals.var_qmfact_dn7)) / (locals.var_qmfact * locals.var_qmfact));
        locals.var_ids_dn8 = ((((((((((((locals.var_fact_ids_dn8 * locals.var_csiprime_dc) + (locals.var_fact_ids * locals.var_csiprime_dc_dn8)) * locals.var_norm_ids_dc) + (assign23180_e22830 * locals.var_norm_ids_dc_dn8)) * locals.var_fdl) + (assign23180_e22832 * locals.var_fdl_dn8)) * locals.var_gvsat) - (assign23180_e22834 * locals.var_gvsat_dn8)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign23180_e22836 * locals.var_qmfact_dn8)) / (locals.var_qmfact * locals.var_qmfact));
        locals.var_ids_dn9 = ((((((((((((locals.var_fact_ids_dn9 * locals.var_csiprime_dc) + (locals.var_fact_ids * locals.var_csiprime_dc_dn9)) * locals.var_norm_ids_dc) + (assign23180_e22830 * locals.var_norm_ids_dc_dn9)) * locals.var_fdl) + (assign23180_e22832 * locals.var_fdl_dn9)) * locals.var_gvsat) - (assign23180_e22834 * locals.var_gvsat_dn9)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign23180_e22836 * locals.var_qmfact_dn9)) / (locals.var_qmfact * locals.var_qmfact));
        locals.var_ids_rv = 0.0;

        let assign23190_e22840: f64 = (-locals.var_vgsu);
        let assign23190_e22842: f64 = (assign23190_e22840 * locals.var_inv_phit0);
        locals.var_xgs_ov = assign23190_e22842;
        locals.var_xgs_ov_dn4 = (assign23190_e22840 * locals.var_inv_phit0_dn4);
        locals.var_xgs_ov_dn6 = (((-locals.var_vgsu_dn6) * locals.var_inv_phit0) + (assign23190_e22840 * locals.var_inv_phit0_dn6));
        locals.var_xgs_ov_dn7 = (assign23190_e22840 * locals.var_inv_phit0_dn7);
        locals.var_xgs_ov_dn8 = (assign23190_e22840 * locals.var_inv_phit0_dn8);
        locals.var_xgs_ov_dn9 = (((-locals.var_vgsu_dn9) * locals.var_inv_phit0) + (assign23190_e22840 * locals.var_inv_phit0_dn9));
        locals.var_xgs_ov_rv = 0.0;

        let assign23200_e22844: f64 = (-locals.var_vgdu);
        let assign23200_e22846: f64 = (assign23200_e22844 * locals.var_inv_phit0);
        locals.var_xgd_ov = assign23200_e22846;
        locals.var_xgd_ov_dn4 = (assign23200_e22844 * locals.var_inv_phit0_dn4);
        locals.var_xgd_ov_dn6 = (((-locals.var_vgdu_dn6) * locals.var_inv_phit0) + (assign23200_e22844 * locals.var_inv_phit0_dn6));
        locals.var_xgd_ov_dn7 = (((-locals.var_vgdu_dn7) * locals.var_inv_phit0) + (assign23200_e22844 * locals.var_inv_phit0_dn7));
        locals.var_xgd_ov_dn8 = (assign23200_e22844 * locals.var_inv_phit0_dn8);
        locals.var_xgd_ov_dn9 = (((-locals.var_vgdu_dn9) * locals.var_inv_phit0) + (assign23200_e22844 * locals.var_inv_phit0_dn9));
        locals.var_xgd_ov_rv = 0.0;

        let assign23210_e22849: f64 = (p.p14 * locals.var_dvfbov_i);
        let assign23210_e22851: f64 = (assign23210_e22849 * locals.var_inv_phit0);
        let assign23210_e22853: f64 = (assign23210_e22851 + locals.var_eg_2phit0);
        locals.var_temp = assign23210_e22853;
        locals.var_temp_dn4 = ((assign23210_e22849 * locals.var_inv_phit0_dn4) + locals.var_eg_2phit0_dn4);
        locals.var_temp_dn6 = ((assign23210_e22849 * locals.var_inv_phit0_dn6) + locals.var_eg_2phit0_dn6);
        locals.var_temp_dn7 = ((assign23210_e22849 * locals.var_inv_phit0_dn7) + locals.var_eg_2phit0_dn7);
        locals.var_temp_dn8 = ((assign23210_e22849 * locals.var_inv_phit0_dn8) + locals.var_eg_2phit0_dn8);
        locals.var_temp_dn9 = ((assign23210_e22849 * locals.var_inv_phit0_dn9) + locals.var_eg_2phit0_dn9);
        locals.var_temp_rv = 0.0;

        let assign23220_e22856: f64 = (locals.var_xgs_ov + locals.var_temp);
        locals.var_xgs_ovcv = assign23220_e22856;
        locals.var_xgs_ovcv_dn4 = (locals.var_xgs_ov_dn4 + locals.var_temp_dn4);
        locals.var_xgs_ovcv_dn6 = (locals.var_xgs_ov_dn6 + locals.var_temp_dn6);
        locals.var_xgs_ovcv_dn7 = (locals.var_xgs_ov_dn7 + locals.var_temp_dn7);
        locals.var_xgs_ovcv_dn8 = (locals.var_xgs_ov_dn8 + locals.var_temp_dn8);
        locals.var_xgs_ovcv_dn9 = (locals.var_xgs_ov_dn9 + locals.var_temp_dn9);
        locals.var_xgs_ovcv_rv = 0.0;

        let assign23230_e22859: f64 = (locals.var_xgd_ov + locals.var_temp);
        locals.var_xgd_ovcv = assign23230_e22859;
        locals.var_xgd_ovcv_dn4 = (locals.var_xgd_ov_dn4 + locals.var_temp_dn4);
        locals.var_xgd_ovcv_dn6 = (locals.var_xgd_ov_dn6 + locals.var_temp_dn6);
        locals.var_xgd_ovcv_dn7 = (locals.var_xgd_ov_dn7 + locals.var_temp_dn7);
        locals.var_xgd_ovcv_dn8 = (locals.var_xgd_ov_dn8 + locals.var_temp_dn8);
        locals.var_xgd_ovcv_dn9 = (locals.var_xgd_ov_dn9 + locals.var_temp_dn9);
        locals.var_xgd_ovcv_rv = 0.0;

        locals.var_xs_ov = 0.0;
        locals.var_xs_ov_dn4 = 0.0;
        locals.var_xs_ov_dn6 = 0.0;
        locals.var_xs_ov_dn7 = 0.0;
        locals.var_xs_ov_dn8 = 0.0;
        locals.var_xs_ov_dn9 = 0.0;
        locals.var_xs_ov_rv = 0.0;

        locals.var_xd_ov = 0.0;
        locals.var_xd_ov_dn4 = 0.0;
        locals.var_xd_ov_dn6 = 0.0;
        locals.var_xd_ov_dn7 = 0.0;
        locals.var_xd_ov_dn8 = 0.0;
        locals.var_xd_ov_dn9 = 0.0;
        locals.var_xd_ov_rv = 0.0;

        locals.var_xs_ovcv = 0.0;
        locals.var_xs_ovcv_dn4 = 0.0;
        locals.var_xs_ovcv_dn6 = 0.0;
        locals.var_xs_ovcv_dn7 = 0.0;
        locals.var_xs_ovcv_dn8 = 0.0;
        locals.var_xs_ovcv_dn9 = 0.0;
        locals.var_xs_ovcv_rv = 0.0;

        locals.var_xd_ovcv = 0.0;
        locals.var_xd_ovcv_dn4 = 0.0;
        locals.var_xd_ovcv_dn6 = 0.0;
        locals.var_xd_ovcv_dn7 = 0.0;
        locals.var_xd_ovcv_dn8 = 0.0;
        locals.var_xd_ovcv_dn9 = 0.0;
        locals.var_xd_ovcv_rv = 0.0;

        let assign23280_e22866: f64 = (2.0 * 1.602176565e-19);
        let assign23280_e22868: f64 = (assign23280_e22866 * locals.var_nov_i);
        let assign23280_e22870: f64 = (assign23280_e22868 * locals.var_epsch);
        let assign23280_e22872: f64 = (assign23280_e22870 * locals.var_inv_phit0);
        let assign23280_e22873: f64 = (assign23280_e22872).sqrt();
        let assign23280_e22875: f64 = (assign23280_e22873 / locals.var_cox1prime);
        locals.var_gov = assign23280_e22875;
        locals.var_gov_dn4 = (((assign23280_e22870 * locals.var_inv_phit0_dn4) / (2.0 * assign23280_e22873)) / locals.var_cox1prime);
        locals.var_gov_dn6 = (((assign23280_e22870 * locals.var_inv_phit0_dn6) / (2.0 * assign23280_e22873)) / locals.var_cox1prime);
        locals.var_gov_dn7 = (((assign23280_e22870 * locals.var_inv_phit0_dn7) / (2.0 * assign23280_e22873)) / locals.var_cox1prime);
        locals.var_gov_dn8 = (((assign23280_e22870 * locals.var_inv_phit0_dn8) / (2.0 * assign23280_e22873)) / locals.var_cox1prime);
        locals.var_gov_dn9 = (((assign23280_e22870 * locals.var_inv_phit0_dn9) / (2.0 * assign23280_e22873)) / locals.var_cox1prime);
        locals.var_gov_rv = 0.0;

        let assign23290_e22878: f64 = (locals.var_gov * locals.var_gov);
        locals.var_gov2 = assign23290_e22878;
        locals.var_gov2_dn4 = ((locals.var_gov_dn4 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn4));
        locals.var_gov2_dn6 = ((locals.var_gov_dn6 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn6));
        locals.var_gov2_dn7 = ((locals.var_gov_dn7 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn7));
        locals.var_gov2_dn8 = ((locals.var_gov_dn8 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn8));
        locals.var_gov2_dn9 = ((locals.var_gov_dn9 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn9));
        locals.var_gov2_rv = 0.0;

        let assign23300_e22882: f64 = (locals.var_gov / 1.4142135623731);
        let assign23300_e22883: f64 = (1.0 + assign23300_e22882);
        locals.var_xi_ov = assign23300_e22883;
        locals.var_xi_ov_dn4 = (locals.var_gov_dn4 / 1.4142135623731);
        locals.var_xi_ov_dn6 = (locals.var_gov_dn6 / 1.4142135623731);
        locals.var_xi_ov_dn7 = (locals.var_gov_dn7 / 1.4142135623731);
        locals.var_xi_ov_dn8 = (locals.var_gov_dn8 / 1.4142135623731);
        locals.var_xi_ov_dn9 = (locals.var_gov_dn9 / 1.4142135623731);
        locals.var_xi_ov_rv = 0.0;

        let assign23310_e22886: f64 = (1e-5 * locals.var_xi_ov);
        locals.var_x_mrg_ov = assign23310_e22886;
        locals.var_x_mrg_ov_dn4 = (1e-5 * locals.var_xi_ov_dn4);
        locals.var_x_mrg_ov_dn6 = (1e-5 * locals.var_xi_ov_dn6);
        locals.var_x_mrg_ov_dn7 = (1e-5 * locals.var_xi_ov_dn7);
        locals.var_x_mrg_ov_dn8 = (1e-5 * locals.var_xi_ov_dn8);
        locals.var_x_mrg_ov_dn9 = (1e-5 * locals.var_xi_ov_dn9);
        locals.var_x_mrg_ov_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_63(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign23320_e22889: f64 = (1.0 / locals.var_xi_ov);
        locals.var_inv_xi_ov = assign23320_e22889;
        locals.var_inv_xi_ov_dn4 = (-(locals.var_xi_ov_dn4 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_dn6 = (-(locals.var_xi_ov_dn6 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_dn7 = (-(locals.var_xi_ov_dn7 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_dn8 = (-(locals.var_xi_ov_dn8 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_dn9 = (-(locals.var_xi_ov_dn9 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_rv = 0.0;

        let assign23330_e22894: f64 = (locals.var_gov * 0.7324648775608221);
        let assign23330_e22895: f64 = (1.25 + assign23330_e22894);
        let assign23330_e22896: f64 = (1.0 / assign23330_e22895);
        locals.var_inv_xg1 = assign23330_e22896;
        locals.var_inv_xg1_dn4 = (-((locals.var_gov_dn4 * 0.7324648775608221) / (assign23330_e22895 * assign23330_e22895)));
        locals.var_inv_xg1_dn6 = (-((locals.var_gov_dn6 * 0.7324648775608221) / (assign23330_e22895 * assign23330_e22895)));
        locals.var_inv_xg1_dn7 = (-((locals.var_gov_dn7 * 0.7324648775608221) / (assign23330_e22895 * assign23330_e22895)));
        locals.var_inv_xg1_dn8 = (-((locals.var_gov_dn8 * 0.7324648775608221) / (assign23330_e22895 * assign23330_e22895)));
        locals.var_inv_xg1_dn9 = (-((locals.var_gov_dn9 * 0.7324648775608221) / (assign23330_e22895 * assign23330_e22895)));
        locals.var_inv_xg1_rv = 0.0;

        let assign23340_e22915: f64 = if (((p.p3 > 0.0) && ((locals.var_igovinv_i > 0.0) || (locals.var_igovacc_i > 0.0))) || ((p.p4 > 0.0) && (locals.var_agidl_i > 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard685 = assign23340_e22915;
        locals.var_guard685_rv = 0.0;

        let assign23350_e22917: f64 = (locals.var_xgs_ov).abs();
        let assign23350_e22919: f64 = if assign23350_e22917 <= locals.var_x_mrg_ov { 1.0 } else { 0.0 };
        locals.var_guard686 = assign23350_e22919;
        locals.var_guard686_rv = 0.0;

        let (assign23360_e22928, assign23360_e22928_d_n4, assign23360_e22928_d_n6, assign23360_e22928_d_n7, assign23360_e22928_d_n8, assign23360_e22928_d_n9,) = {
    if ((locals.var_guard685 != 0.0) && (locals.var_guard686 != 0.0)) {
        let assign23360_e22924: f64 = (-locals.var_xgs_ov);
        let assign23360_e22926: f64 = (assign23360_e22924 * locals.var_inv_xi_ov);
        (assign23360_e22926, (((-locals.var_xgs_ov_dn4) * locals.var_inv_xi_ov) + (assign23360_e22924 * locals.var_inv_xi_ov_dn4)), (((-locals.var_xgs_ov_dn6) * locals.var_inv_xi_ov) + (assign23360_e22924 * locals.var_inv_xi_ov_dn6)), (((-locals.var_xgs_ov_dn7) * locals.var_inv_xi_ov) + (assign23360_e22924 * locals.var_inv_xi_ov_dn7)), (((-locals.var_xgs_ov_dn8) * locals.var_inv_xi_ov) + (assign23360_e22924 * locals.var_inv_xi_ov_dn8)), (((-locals.var_xgs_ov_dn9) * locals.var_inv_xi_ov) + (assign23360_e22924 * locals.var_inv_xi_ov_dn9)),)
    } else {
        (locals.var_xs_ov, locals.var_xs_ov_dn4, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7, locals.var_xs_ov_dn8, locals.var_xs_ov_dn9,)
    }
};
        locals.var_xs_ov = assign23360_e22928;
        locals.var_xs_ov_dn4 = assign23360_e22928_d_n4;
        locals.var_xs_ov_dn6 = assign23360_e22928_d_n6;
        locals.var_xs_ov_dn7 = assign23360_e22928_d_n7;
        locals.var_xs_ov_dn8 = assign23360_e22928_d_n8;
        locals.var_xs_ov_dn9 = assign23360_e22928_d_n9;
        locals.var_xs_ov_rv = 0.0;

        let assign23370_e22931: f64 = (-locals.var_x_mrg_ov);
        let assign23370_e22932: f64 = if locals.var_xgs_ov < assign23370_e22931 { 1.0 } else { 0.0 };
        locals.var_guard687 = assign23370_e22932;
        locals.var_guard687_rv = 0.0;

        let (assign23380_e22942, assign23380_e22942_d_n4, assign23380_e22942_d_n6, assign23380_e22942_d_n7, assign23380_e22942_d_n8, assign23380_e22942_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23380_e22940: f64 = (-locals.var_xgs_ov);
        (assign23380_e22940, (-locals.var_xgs_ov_dn4), (-locals.var_xgs_ov_dn6), (-locals.var_xgs_ov_dn7), (-locals.var_xgs_ov_dn8), (-locals.var_xgs_ov_dn9),)
    } else {
        (locals.var_sp_ov_ygf, locals.var_sp_ov_ygf_dn4, locals.var_sp_ov_ygf_dn6, locals.var_sp_ov_ygf_dn7, locals.var_sp_ov_ygf_dn8, locals.var_sp_ov_ygf_dn9,)
    }
};
        locals.var_sp_ov_ygf = assign23380_e22942;
        locals.var_sp_ov_ygf_dn4 = assign23380_e22942_d_n4;
        locals.var_sp_ov_ygf_dn6 = assign23380_e22942_d_n6;
        locals.var_sp_ov_ygf_dn7 = assign23380_e22942_d_n7;
        locals.var_sp_ov_ygf_dn8 = assign23380_e22942_d_n8;
        locals.var_sp_ov_ygf_dn9 = assign23380_e22942_d_n9;
        locals.var_sp_ov_ygf_rv = 0.0;

        let (assign23390_e22955, assign23390_e22955_d_n4, assign23390_e22955_d_n6, assign23390_e22955_d_n7, assign23390_e22955_d_n8, assign23390_e22955_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23390_e22951: f64 = (1.25 * locals.var_sp_ov_ygf);
        let assign23390_e22953: f64 = (assign23390_e22951 * locals.var_inv_xi_ov);
        (assign23390_e22953, (((1.25 * locals.var_sp_ov_ygf_dn4) * locals.var_inv_xi_ov) + (assign23390_e22951 * locals.var_inv_xi_ov_dn4)), (((1.25 * locals.var_sp_ov_ygf_dn6) * locals.var_inv_xi_ov) + (assign23390_e22951 * locals.var_inv_xi_ov_dn6)), (((1.25 * locals.var_sp_ov_ygf_dn7) * locals.var_inv_xi_ov) + (assign23390_e22951 * locals.var_inv_xi_ov_dn7)), (((1.25 * locals.var_sp_ov_ygf_dn8) * locals.var_inv_xi_ov) + (assign23390_e22951 * locals.var_inv_xi_ov_dn8)), (((1.25 * locals.var_sp_ov_ygf_dn9) * locals.var_inv_xi_ov) + (assign23390_e22951 * locals.var_inv_xi_ov_dn9)),)
    } else {
        (locals.var_sp_ov_z, locals.var_sp_ov_z_dn4, locals.var_sp_ov_z_dn6, locals.var_sp_ov_z_dn7, locals.var_sp_ov_z_dn8, locals.var_sp_ov_z_dn9,)
    }
};
        locals.var_sp_ov_z = assign23390_e22955;
        locals.var_sp_ov_z_dn4 = assign23390_e22955_d_n4;
        locals.var_sp_ov_z_dn6 = assign23390_e22955_d_n6;
        locals.var_sp_ov_z_dn7 = assign23390_e22955_d_n7;
        locals.var_sp_ov_z_dn8 = assign23390_e22955_d_n8;
        locals.var_sp_ov_z_dn9 = assign23390_e22955_d_n9;
        locals.var_sp_ov_z_rv = 0.0;

        let (assign23400_e22979, assign23400_e22979_d_n4, assign23400_e22979_d_n6, assign23400_e22979_d_n7, assign23400_e22979_d_n8, assign23400_e22979_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23400_e22965: f64 = (locals.var_sp_ov_z + 10.0);
        let assign23400_e22968: f64 = (locals.var_sp_ov_z - 6.0);
        let assign23400_e22971: f64 = (locals.var_sp_ov_z - 6.0);
        let assign23400_e22972: f64 = (assign23400_e22968 * assign23400_e22971);
        let assign23400_e22974: f64 = (assign23400_e22972 + 64.0);
        let assign23400_e22975: f64 = (assign23400_e22974).sqrt();
        let assign23400_e22976: f64 = (assign23400_e22965 - assign23400_e22975);
        let assign23400_e22977: f64 = (0.5 * assign23400_e22976);
        (assign23400_e22977, (0.5 * (locals.var_sp_ov_z_dn4 - (((locals.var_sp_ov_z_dn4 * assign23400_e22971) + (assign23400_e22968 * locals.var_sp_ov_z_dn4)) / (2.0 * assign23400_e22975)))), (0.5 * (locals.var_sp_ov_z_dn6 - (((locals.var_sp_ov_z_dn6 * assign23400_e22971) + (assign23400_e22968 * locals.var_sp_ov_z_dn6)) / (2.0 * assign23400_e22975)))), (0.5 * (locals.var_sp_ov_z_dn7 - (((locals.var_sp_ov_z_dn7 * assign23400_e22971) + (assign23400_e22968 * locals.var_sp_ov_z_dn7)) / (2.0 * assign23400_e22975)))), (0.5 * (locals.var_sp_ov_z_dn8 - (((locals.var_sp_ov_z_dn8 * assign23400_e22971) + (assign23400_e22968 * locals.var_sp_ov_z_dn8)) / (2.0 * assign23400_e22975)))), (0.5 * (locals.var_sp_ov_z_dn9 - (((locals.var_sp_ov_z_dn9 * assign23400_e22971) + (assign23400_e22968 * locals.var_sp_ov_z_dn9)) / (2.0 * assign23400_e22975)))),)
    } else {
        (locals.var_sp_ov_eta, locals.var_sp_ov_eta_dn4, locals.var_sp_ov_eta_dn6, locals.var_sp_ov_eta_dn7, locals.var_sp_ov_eta_dn8, locals.var_sp_ov_eta_dn9,)
    }
};
        locals.var_sp_ov_eta = assign23400_e22979;
        locals.var_sp_ov_eta_dn4 = assign23400_e22979_d_n4;
        locals.var_sp_ov_eta_dn6 = assign23400_e22979_d_n6;
        locals.var_sp_ov_eta_dn7 = assign23400_e22979_d_n7;
        locals.var_sp_ov_eta_dn8 = assign23400_e22979_d_n8;
        locals.var_sp_ov_eta_dn9 = assign23400_e22979_d_n9;
        locals.var_sp_ov_eta_rv = 0.0;

        let (assign23410_e23000, assign23410_e23000_d_n4, assign23410_e23000_d_n6, assign23410_e23000_d_n7, assign23410_e23000_d_n8, assign23410_e23000_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23410_e22988: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign23410_e22991: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign23410_e22992: f64 = (assign23410_e22988 * assign23410_e22991);
        let assign23410_e22996: f64 = (locals.var_sp_ov_eta + 1.0);
        let assign23410_e22997: f64 = (locals.var_gov2 * assign23410_e22996);
        let assign23410_e22998: f64 = (assign23410_e22992 + assign23410_e22997);
        (assign23410_e22998, ((((locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4) * assign23410_e22991) + (assign23410_e22988 * (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4))) + ((locals.var_gov2_dn4 * assign23410_e22996) + (locals.var_gov2 * locals.var_sp_ov_eta_dn4))), ((((locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6) * assign23410_e22991) + (assign23410_e22988 * (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6))) + ((locals.var_gov2_dn6 * assign23410_e22996) + (locals.var_gov2 * locals.var_sp_ov_eta_dn6))), ((((locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7) * assign23410_e22991) + (assign23410_e22988 * (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7))) + ((locals.var_gov2_dn7 * assign23410_e22996) + (locals.var_gov2 * locals.var_sp_ov_eta_dn7))), ((((locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8) * assign23410_e22991) + (assign23410_e22988 * (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8))) + ((locals.var_gov2_dn8 * assign23410_e22996) + (locals.var_gov2 * locals.var_sp_ov_eta_dn8))), ((((locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9) * assign23410_e22991) + (assign23410_e22988 * (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9))) + ((locals.var_gov2_dn9 * assign23410_e22996) + (locals.var_gov2 * locals.var_sp_ov_eta_dn9))),)
    } else {
        (locals.var_sp_ov_a, locals.var_sp_ov_a_dn4, locals.var_sp_ov_a_dn6, locals.var_sp_ov_a_dn7, locals.var_sp_ov_a_dn8, locals.var_sp_ov_a_dn9,)
    }
};
        locals.var_sp_ov_a = assign23410_e23000;
        locals.var_sp_ov_a_dn4 = assign23410_e23000_d_n4;
        locals.var_sp_ov_a_dn6 = assign23410_e23000_d_n6;
        locals.var_sp_ov_a_dn7 = assign23410_e23000_d_n7;
        locals.var_sp_ov_a_dn8 = assign23410_e23000_d_n8;
        locals.var_sp_ov_a_dn9 = assign23410_e23000_d_n9;
        locals.var_sp_ov_a_rv = 0.0;

        let (assign23420_e23015, assign23420_e23015_d_n4, assign23420_e23015_d_n6, assign23420_e23015_d_n7, assign23420_e23015_d_n8, assign23420_e23015_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23420_e23010: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign23420_e23011: f64 = (2.0 * assign23420_e23010);
        let assign23420_e23013: f64 = (assign23420_e23011 - locals.var_gov2);
        (assign23420_e23013, ((2.0 * (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4)) - locals.var_gov2_dn4), ((2.0 * (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6)) - locals.var_gov2_dn6), ((2.0 * (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7)) - locals.var_gov2_dn7), ((2.0 * (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8)) - locals.var_gov2_dn8), ((2.0 * (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9)) - locals.var_gov2_dn9),)
    } else {
        (locals.var_sp_ov_c, locals.var_sp_ov_c_dn4, locals.var_sp_ov_c_dn6, locals.var_sp_ov_c_dn7, locals.var_sp_ov_c_dn8, locals.var_sp_ov_c_dn9,)
    }
};
        locals.var_sp_ov_c = assign23420_e23015;
        locals.var_sp_ov_c_dn4 = assign23420_e23015_d_n4;
        locals.var_sp_ov_c_dn6 = assign23420_e23015_d_n6;
        locals.var_sp_ov_c_dn7 = assign23420_e23015_d_n7;
        locals.var_sp_ov_c_dn8 = assign23420_e23015_d_n8;
        locals.var_sp_ov_c_dn9 = assign23420_e23015_d_n9;
        locals.var_sp_ov_c_rv = 0.0;

        let (assign23430_e23029, assign23430_e23029_d_n4, assign23430_e23029_d_n6, assign23430_e23029_d_n7, assign23430_e23029_d_n8, assign23430_e23029_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23430_e23024: f64 = (locals.var_sp_ov_a / locals.var_gov2);
        let assign23430_e23025: f64 = (assign23430_e23024).ln();
        let assign23430_e23027: f64 = (assign23430_e23025 - locals.var_sp_ov_eta);
        (assign23430_e23027, (((((locals.var_sp_ov_a_dn4 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn4)) / (locals.var_gov2 * locals.var_gov2)) / assign23430_e23024) - locals.var_sp_ov_eta_dn4), (((((locals.var_sp_ov_a_dn6 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn6)) / (locals.var_gov2 * locals.var_gov2)) / assign23430_e23024) - locals.var_sp_ov_eta_dn6), (((((locals.var_sp_ov_a_dn7 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn7)) / (locals.var_gov2 * locals.var_gov2)) / assign23430_e23024) - locals.var_sp_ov_eta_dn7), (((((locals.var_sp_ov_a_dn8 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn8)) / (locals.var_gov2 * locals.var_gov2)) / assign23430_e23024) - locals.var_sp_ov_eta_dn8), (((((locals.var_sp_ov_a_dn9 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn9)) / (locals.var_gov2 * locals.var_gov2)) / assign23430_e23024) - locals.var_sp_ov_eta_dn9),)
    } else {
        (locals.var_sp_ov_tau, locals.var_sp_ov_tau_dn4, locals.var_sp_ov_tau_dn6, locals.var_sp_ov_tau_dn7, locals.var_sp_ov_tau_dn8, locals.var_sp_ov_tau_dn9,)
    }
};
        locals.var_sp_ov_tau = assign23430_e23029;
        locals.var_sp_ov_tau_dn4 = assign23430_e23029_d_n4;
        locals.var_sp_ov_tau_dn6 = assign23430_e23029_d_n6;
        locals.var_sp_ov_tau_dn7 = assign23430_e23029_d_n7;
        locals.var_sp_ov_tau_dn8 = assign23430_e23029_d_n8;
        locals.var_sp_ov_tau_dn9 = assign23430_e23029_d_n9;
        locals.var_sp_ov_tau_rv = 0.0;

        let (assign23440_e23040, assign23440_e23040_d_n4, assign23440_e23040_d_n6, assign23440_e23040_d_n7, assign23440_e23040_d_n8, assign23440_e23040_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23440_e23038: f64 = (locals.var_sp_ov_a + locals.var_sp_ov_c);
        (assign23440_e23038, (locals.var_sp_ov_a_dn4 + locals.var_sp_ov_c_dn4), (locals.var_sp_ov_a_dn6 + locals.var_sp_ov_c_dn6), (locals.var_sp_ov_a_dn7 + locals.var_sp_ov_c_dn7), (locals.var_sp_ov_a_dn8 + locals.var_sp_ov_c_dn8), (locals.var_sp_ov_a_dn9 + locals.var_sp_ov_c_dn9),)
    } else {
        (locals.var_sp_ov_nu, locals.var_sp_ov_nu_dn4, locals.var_sp_ov_nu_dn6, locals.var_sp_ov_nu_dn7, locals.var_sp_ov_nu_dn8, locals.var_sp_ov_nu_dn9,)
    }
};
        locals.var_sp_ov_nu = assign23440_e23040;
        locals.var_sp_ov_nu_dn4 = assign23440_e23040_d_n4;
        locals.var_sp_ov_nu_dn6 = assign23440_e23040_d_n6;
        locals.var_sp_ov_nu_dn7 = assign23440_e23040_d_n7;
        locals.var_sp_ov_nu_dn8 = assign23440_e23040_d_n8;
        locals.var_sp_ov_nu_dn9 = assign23440_e23040_d_n9;
        locals.var_sp_ov_nu_rv = 0.0;

        let (assign23450_e23061, assign23450_e23061_d_n4, assign23450_e23061_d_n6, assign23450_e23061_d_n7, assign23450_e23061_d_n8, assign23450_e23061_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23450_e23049: f64 = (locals.var_sp_ov_nu * locals.var_sp_ov_nu);
        let assign23450_e23053: f64 = (0.5 * locals.var_sp_ov_c);
        let assign23450_e23055: f64 = (assign23450_e23053 * locals.var_sp_ov_c);
        let assign23450_e23057: f64 = (assign23450_e23055 - locals.var_sp_ov_a);
        let assign23450_e23058: f64 = (locals.var_sp_ov_tau * assign23450_e23057);
        let assign23450_e23059: f64 = (assign23450_e23049 + assign23450_e23058);
        (assign23450_e23059, (((locals.var_sp_ov_nu_dn4 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn4)) + ((locals.var_sp_ov_tau_dn4 * assign23450_e23057) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn4) * locals.var_sp_ov_c) + (assign23450_e23053 * locals.var_sp_ov_c_dn4)) - locals.var_sp_ov_a_dn4)))), (((locals.var_sp_ov_nu_dn6 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn6)) + ((locals.var_sp_ov_tau_dn6 * assign23450_e23057) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn6) * locals.var_sp_ov_c) + (assign23450_e23053 * locals.var_sp_ov_c_dn6)) - locals.var_sp_ov_a_dn6)))), (((locals.var_sp_ov_nu_dn7 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn7)) + ((locals.var_sp_ov_tau_dn7 * assign23450_e23057) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn7) * locals.var_sp_ov_c) + (assign23450_e23053 * locals.var_sp_ov_c_dn7)) - locals.var_sp_ov_a_dn7)))), (((locals.var_sp_ov_nu_dn8 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn8)) + ((locals.var_sp_ov_tau_dn8 * assign23450_e23057) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn8) * locals.var_sp_ov_c) + (assign23450_e23053 * locals.var_sp_ov_c_dn8)) - locals.var_sp_ov_a_dn8)))), (((locals.var_sp_ov_nu_dn9 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn9)) + ((locals.var_sp_ov_tau_dn9 * assign23450_e23057) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn9) * locals.var_sp_ov_c) + (assign23450_e23053 * locals.var_sp_ov_c_dn9)) - locals.var_sp_ov_a_dn9)))),)
    } else {
        (locals.var_sp_ov_mutau, locals.var_sp_ov_mutau_dn4, locals.var_sp_ov_mutau_dn6, locals.var_sp_ov_mutau_dn7, locals.var_sp_ov_mutau_dn8, locals.var_sp_ov_mutau_dn9,)
    }
};
        locals.var_sp_ov_mutau = assign23450_e23061;
        locals.var_sp_ov_mutau_dn4 = assign23450_e23061_d_n4;
        locals.var_sp_ov_mutau_dn6 = assign23450_e23061_d_n6;
        locals.var_sp_ov_mutau_dn7 = assign23450_e23061_d_n7;
        locals.var_sp_ov_mutau_dn8 = assign23450_e23061_d_n8;
        locals.var_sp_ov_mutau_dn9 = assign23450_e23061_d_n9;
        locals.var_sp_ov_mutau_rv = 0.0;

        let (assign23460_e23088, assign23460_e23088_d_n4, assign23460_e23088_d_n6, assign23460_e23088_d_n7, assign23460_e23088_d_n8, assign23460_e23088_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23460_e23071: f64 = (locals.var_sp_ov_nu / locals.var_sp_ov_mutau);
        let assign23460_e23073: f64 = (assign23460_e23071 * locals.var_sp_ov_tau);
        let assign23460_e23075: f64 = (assign23460_e23073 * locals.var_sp_ov_tau);
        let assign23460_e23077: f64 = (assign23460_e23075 * locals.var_sp_ov_c);
        let assign23460_e23080: f64 = (locals.var_sp_ov_c * locals.var_sp_ov_c);
        let assign23460_e23082: f64 = (assign23460_e23080 * 0.3333333333333);
        let assign23460_e23084: f64 = (assign23460_e23082 - locals.var_sp_ov_a);
        let assign23460_e23085: f64 = (assign23460_e23077 * assign23460_e23084);
        let assign23460_e23086: f64 = (locals.var_sp_ov_mutau + assign23460_e23085);
        (assign23460_e23086, (locals.var_sp_ov_mutau_dn4 + (((((((((((locals.var_sp_ov_nu_dn4 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn4)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign23460_e23071 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_tau) + (assign23460_e23073 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_c) + (assign23460_e23075 * locals.var_sp_ov_c_dn4)) * assign23460_e23084) + (assign23460_e23077 * ((((locals.var_sp_ov_c_dn4 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn4)) * 0.3333333333333) - locals.var_sp_ov_a_dn4)))), (locals.var_sp_ov_mutau_dn6 + (((((((((((locals.var_sp_ov_nu_dn6 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn6)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign23460_e23071 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_tau) + (assign23460_e23073 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_c) + (assign23460_e23075 * locals.var_sp_ov_c_dn6)) * assign23460_e23084) + (assign23460_e23077 * ((((locals.var_sp_ov_c_dn6 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn6)) * 0.3333333333333) - locals.var_sp_ov_a_dn6)))), (locals.var_sp_ov_mutau_dn7 + (((((((((((locals.var_sp_ov_nu_dn7 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn7)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign23460_e23071 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_tau) + (assign23460_e23073 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_c) + (assign23460_e23075 * locals.var_sp_ov_c_dn7)) * assign23460_e23084) + (assign23460_e23077 * ((((locals.var_sp_ov_c_dn7 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn7)) * 0.3333333333333) - locals.var_sp_ov_a_dn7)))), (locals.var_sp_ov_mutau_dn8 + (((((((((((locals.var_sp_ov_nu_dn8 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn8)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign23460_e23071 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_tau) + (assign23460_e23073 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_c) + (assign23460_e23075 * locals.var_sp_ov_c_dn8)) * assign23460_e23084) + (assign23460_e23077 * ((((locals.var_sp_ov_c_dn8 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn8)) * 0.3333333333333) - locals.var_sp_ov_a_dn8)))), (locals.var_sp_ov_mutau_dn9 + (((((((((((locals.var_sp_ov_nu_dn9 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn9)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign23460_e23071 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_tau) + (assign23460_e23073 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_c) + (assign23460_e23075 * locals.var_sp_ov_c_dn9)) * assign23460_e23084) + (assign23460_e23077 * ((((locals.var_sp_ov_c_dn9 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn9)) * 0.3333333333333) - locals.var_sp_ov_a_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign23460_e23088;
        locals.var_sp_ov_temp_dn4 = assign23460_e23088_d_n4;
        locals.var_sp_ov_temp_dn6 = assign23460_e23088_d_n6;
        locals.var_sp_ov_temp_dn7 = assign23460_e23088_d_n7;
        locals.var_sp_ov_temp_dn8 = assign23460_e23088_d_n8;
        locals.var_sp_ov_temp_dn9 = assign23460_e23088_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign23470_e23105, assign23470_e23105_d_n4, assign23470_e23105_d_n6, assign23470_e23105_d_n7, assign23470_e23105_d_n8, assign23470_e23105_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23470_e23098: f64 = (locals.var_sp_ov_a * locals.var_sp_ov_nu);
        let assign23470_e23100: f64 = (assign23470_e23098 * locals.var_sp_ov_tau);
        let assign23470_e23102: f64 = (assign23470_e23100 / locals.var_sp_ov_temp);
        let assign23470_e23103: f64 = (locals.var_sp_ov_eta + assign23470_e23102);
        (assign23470_e23103, (locals.var_sp_ov_eta_dn4 + (((((((locals.var_sp_ov_a_dn4 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn4)) * locals.var_sp_ov_tau) + (assign23470_e23098 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_temp) - (assign23470_e23100 * locals.var_sp_ov_temp_dn4)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn6 + (((((((locals.var_sp_ov_a_dn6 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn6)) * locals.var_sp_ov_tau) + (assign23470_e23098 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_temp) - (assign23470_e23100 * locals.var_sp_ov_temp_dn6)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn7 + (((((((locals.var_sp_ov_a_dn7 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn7)) * locals.var_sp_ov_tau) + (assign23470_e23098 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_temp) - (assign23470_e23100 * locals.var_sp_ov_temp_dn7)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn8 + (((((((locals.var_sp_ov_a_dn8 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn8)) * locals.var_sp_ov_tau) + (assign23470_e23098 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_temp) - (assign23470_e23100 * locals.var_sp_ov_temp_dn8)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn9 + (((((((locals.var_sp_ov_a_dn9 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn9)) * locals.var_sp_ov_tau) + (assign23470_e23098 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_temp) - (assign23470_e23100 * locals.var_sp_ov_temp_dn9)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))),)
    } else {
        (locals.var_sp_ov_y0, locals.var_sp_ov_y0_dn4, locals.var_sp_ov_y0_dn6, locals.var_sp_ov_y0_dn7, locals.var_sp_ov_y0_dn8, locals.var_sp_ov_y0_dn9,)
    }
};
        locals.var_sp_ov_y0 = assign23470_e23105;
        locals.var_sp_ov_y0_dn4 = assign23470_e23105_d_n4;
        locals.var_sp_ov_y0_dn6 = assign23470_e23105_d_n6;
        locals.var_sp_ov_y0_dn7 = assign23470_e23105_d_n7;
        locals.var_sp_ov_y0_dn8 = assign23470_e23105_d_n8;
        locals.var_sp_ov_y0_dn9 = assign23470_e23105_d_n9;
        locals.var_sp_ov_y0_rv = 0.0;

        let assign23480_e23107: f64 = (locals.var_sp_ov_y0).abs();
        let assign23480_e23109: f64 = if assign23480_e23107 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard688 = assign23480_e23109;
        locals.var_guard688_rv = 0.0;

        let (assign23490_e23121, assign23490_e23121_d_n4, assign23490_e23121_d_n6, assign23490_e23121_d_n7, assign23490_e23121_d_n8, assign23490_e23121_d_n9,) = {
    if ((((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 != 0.0)) {
        let assign23490_e23119: f64 = (locals.var_sp_ov_y0).exp();
        (assign23490_e23119, (assign23490_e23119 * locals.var_sp_ov_y0_dn4), (assign23490_e23119 * locals.var_sp_ov_y0_dn6), (assign23490_e23119 * locals.var_sp_ov_y0_dn7), (assign23490_e23119 * locals.var_sp_ov_y0_dn8), (assign23490_e23119 * locals.var_sp_ov_y0_dn9),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign23490_e23121;
        locals.var_sp_ov_d0_dn4 = assign23490_e23121_d_n4;
        locals.var_sp_ov_d0_dn6 = assign23490_e23121_d_n6;
        locals.var_sp_ov_d0_dn7 = assign23490_e23121_d_n7;
        locals.var_sp_ov_d0_dn8 = assign23490_e23121_d_n8;
        locals.var_sp_ov_d0_dn9 = assign23490_e23121_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let assign23500_e23124: f64 = (-80.0);
        let assign23500_e23125: f64 = if locals.var_sp_ov_y0 < assign23500_e23124 { 1.0 } else { 0.0 };
        locals.var_guard689 = assign23500_e23125;
        locals.var_guard689_rv = 0.0;

        let (assign23510_e23164, assign23510_e23164_d_n4, assign23510_e23164_d_n6, assign23510_e23164_d_n7, assign23510_e23164_d_n8, assign23510_e23164_d_n9,) = {
    if (((((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 == 0.0)) && (locals.var_guard689 != 0.0)) {
        let assign23510_e23140: f64 = (-locals.var_sp_ov_y0);
        let assign23510_e23142: f64 = (assign23510_e23140 - 80.0);
        let assign23510_e23146: f64 = (-locals.var_sp_ov_y0);
        let assign23510_e23148: f64 = (assign23510_e23146 - 80.0);
        let assign23510_e23149: f64 = (0.5 * assign23510_e23148);
        let assign23510_e23152: f64 = (-locals.var_sp_ov_y0);
        let assign23510_e23154: f64 = (assign23510_e23152 - 80.0);
        let assign23510_e23156: f64 = (assign23510_e23154 * 0.3333333333333);
        let assign23510_e23157: f64 = (1.0 + assign23510_e23156);
        let assign23510_e23158: f64 = (assign23510_e23149 * assign23510_e23157);
        let assign23510_e23159: f64 = (1.0 + assign23510_e23158);
        let assign23510_e23160: f64 = (assign23510_e23142 * assign23510_e23159);
        let assign23510_e23161: f64 = (1.0 + assign23510_e23160);
        let assign23510_e23162: f64 = (1.80485e-35 / assign23510_e23161);
        (assign23510_e23162, (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn4) * assign23510_e23159) + (assign23510_e23142 * (((0.5 * (-locals.var_sp_ov_y0_dn4)) * assign23510_e23157) + (assign23510_e23149 * ((-locals.var_sp_ov_y0_dn4) * 0.3333333333333)))))) / (assign23510_e23161 * assign23510_e23161))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn6) * assign23510_e23159) + (assign23510_e23142 * (((0.5 * (-locals.var_sp_ov_y0_dn6)) * assign23510_e23157) + (assign23510_e23149 * ((-locals.var_sp_ov_y0_dn6) * 0.3333333333333)))))) / (assign23510_e23161 * assign23510_e23161))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn7) * assign23510_e23159) + (assign23510_e23142 * (((0.5 * (-locals.var_sp_ov_y0_dn7)) * assign23510_e23157) + (assign23510_e23149 * ((-locals.var_sp_ov_y0_dn7) * 0.3333333333333)))))) / (assign23510_e23161 * assign23510_e23161))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn8) * assign23510_e23159) + (assign23510_e23142 * (((0.5 * (-locals.var_sp_ov_y0_dn8)) * assign23510_e23157) + (assign23510_e23149 * ((-locals.var_sp_ov_y0_dn8) * 0.3333333333333)))))) / (assign23510_e23161 * assign23510_e23161))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn9) * assign23510_e23159) + (assign23510_e23142 * (((0.5 * (-locals.var_sp_ov_y0_dn9)) * assign23510_e23157) + (assign23510_e23149 * ((-locals.var_sp_ov_y0_dn9) * 0.3333333333333)))))) / (assign23510_e23161 * assign23510_e23161))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign23510_e23164;
        locals.var_sp_ov_d0_dn4 = assign23510_e23164_d_n4;
        locals.var_sp_ov_d0_dn6 = assign23510_e23164_d_n6;
        locals.var_sp_ov_d0_dn7 = assign23510_e23164_d_n7;
        locals.var_sp_ov_d0_dn8 = assign23510_e23164_d_n8;
        locals.var_sp_ov_d0_dn9 = assign23510_e23164_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign23520_e23201, assign23520_e23201_d_n4, assign23520_e23201_d_n6, assign23520_e23201_d_n7, assign23520_e23201_d_n8, assign23520_e23201_d_n9,) = {
    if (((((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) && (locals.var_guard688 == 0.0)) && (locals.var_guard689 == 0.0)) {
        let assign23520_e23181: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign23520_e23186: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign23520_e23187: f64 = (0.5 * assign23520_e23186);
        let assign23520_e23191: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign23520_e23193: f64 = (assign23520_e23191 * 0.3333333333333);
        let assign23520_e23194: f64 = (1.0 + assign23520_e23193);
        let assign23520_e23195: f64 = (assign23520_e23187 * assign23520_e23194);
        let assign23520_e23196: f64 = (1.0 + assign23520_e23195);
        let assign23520_e23197: f64 = (assign23520_e23181 * assign23520_e23196);
        let assign23520_e23198: f64 = (1.0 + assign23520_e23197);
        let assign23520_e23199: f64 = (5.54062e34 * assign23520_e23198);
        (assign23520_e23199, (5.54062e34 * ((locals.var_sp_ov_y0_dn4 * assign23520_e23196) + (assign23520_e23181 * (((0.5 * locals.var_sp_ov_y0_dn4) * assign23520_e23194) + (assign23520_e23187 * (locals.var_sp_ov_y0_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn6 * assign23520_e23196) + (assign23520_e23181 * (((0.5 * locals.var_sp_ov_y0_dn6) * assign23520_e23194) + (assign23520_e23187 * (locals.var_sp_ov_y0_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn7 * assign23520_e23196) + (assign23520_e23181 * (((0.5 * locals.var_sp_ov_y0_dn7) * assign23520_e23194) + (assign23520_e23187 * (locals.var_sp_ov_y0_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn8 * assign23520_e23196) + (assign23520_e23181 * (((0.5 * locals.var_sp_ov_y0_dn8) * assign23520_e23194) + (assign23520_e23187 * (locals.var_sp_ov_y0_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn9 * assign23520_e23196) + (assign23520_e23181 * (((0.5 * locals.var_sp_ov_y0_dn9) * assign23520_e23194) + (assign23520_e23187 * (locals.var_sp_ov_y0_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign23520_e23201;
        locals.var_sp_ov_d0_dn4 = assign23520_e23201_d_n4;
        locals.var_sp_ov_d0_dn6 = assign23520_e23201_d_n6;
        locals.var_sp_ov_d0_dn7 = assign23520_e23201_d_n7;
        locals.var_sp_ov_d0_dn8 = assign23520_e23201_d_n8;
        locals.var_sp_ov_d0_dn9 = assign23520_e23201_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign23530_e23212, assign23530_e23212_d_n4, assign23530_e23212_d_n6, assign23530_e23212_d_n7, assign23530_e23212_d_n8, assign23530_e23212_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23530_e23210: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_y0);
        (assign23530_e23210, (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_y0_dn4), (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_y0_dn6), (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_y0_dn7), (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_y0_dn8), (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_y0_dn9),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign23530_e23212;
        locals.var_sp_ov_temp_dn4 = assign23530_e23212_d_n4;
        locals.var_sp_ov_temp_dn6 = assign23530_e23212_d_n6;
        locals.var_sp_ov_temp_dn7 = assign23530_e23212_d_n7;
        locals.var_sp_ov_temp_dn8 = assign23530_e23212_d_n8;
        locals.var_sp_ov_temp_dn9 = assign23530_e23212_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign23540_e23229, assign23540_e23229_d_n4, assign23540_e23229_d_n6, assign23540_e23229_d_n7, assign23540_e23229_d_n8, assign23540_e23229_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23540_e23221: f64 = (2.0 * locals.var_sp_ov_temp);
        let assign23540_e23225: f64 = (locals.var_sp_ov_d0 - 1.0);
        let assign23540_e23226: f64 = (locals.var_gov2 * assign23540_e23225);
        let assign23540_e23227: f64 = (assign23540_e23221 + assign23540_e23226);
        (assign23540_e23227, ((2.0 * locals.var_sp_ov_temp_dn4) + ((locals.var_gov2_dn4 * assign23540_e23225) + (locals.var_gov2 * locals.var_sp_ov_d0_dn4))), ((2.0 * locals.var_sp_ov_temp_dn6) + ((locals.var_gov2_dn6 * assign23540_e23225) + (locals.var_gov2 * locals.var_sp_ov_d0_dn6))), ((2.0 * locals.var_sp_ov_temp_dn7) + ((locals.var_gov2_dn7 * assign23540_e23225) + (locals.var_gov2 * locals.var_sp_ov_d0_dn7))), ((2.0 * locals.var_sp_ov_temp_dn8) + ((locals.var_gov2_dn8 * assign23540_e23225) + (locals.var_gov2 * locals.var_sp_ov_d0_dn8))), ((2.0 * locals.var_sp_ov_temp_dn9) + ((locals.var_gov2_dn9 * assign23540_e23225) + (locals.var_gov2 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_p, locals.var_sp_ov_p_dn4, locals.var_sp_ov_p_dn6, locals.var_sp_ov_p_dn7, locals.var_sp_ov_p_dn8, locals.var_sp_ov_p_dn9,)
    }
};
        locals.var_sp_ov_p = assign23540_e23229;
        locals.var_sp_ov_p_dn4 = assign23540_e23229_d_n4;
        locals.var_sp_ov_p_dn6 = assign23540_e23229_d_n6;
        locals.var_sp_ov_p_dn7 = assign23540_e23229_d_n7;
        locals.var_sp_ov_p_dn8 = assign23540_e23229_d_n8;
        locals.var_sp_ov_p_dn9 = assign23540_e23229_d_n9;
        locals.var_sp_ov_p_rv = 0.0;

        let (assign23550_e23248, assign23550_e23248_d_n4, assign23550_e23248_d_n6, assign23550_e23248_d_n7, assign23550_e23248_d_n8, assign23550_e23248_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23550_e23238: f64 = (locals.var_sp_ov_temp * locals.var_sp_ov_temp);
        let assign23550_e23242: f64 = (locals.var_sp_ov_y0 + 1.0);
        let assign23550_e23244: f64 = (assign23550_e23242 - locals.var_sp_ov_d0);
        let assign23550_e23245: f64 = (locals.var_gov2 * assign23550_e23244);
        let assign23550_e23246: f64 = (assign23550_e23238 + assign23550_e23245);
        (assign23550_e23246, (((locals.var_sp_ov_temp_dn4 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn4)) + ((locals.var_gov2_dn4 * assign23550_e23244) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn4 - locals.var_sp_ov_d0_dn4)))), (((locals.var_sp_ov_temp_dn6 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn6)) + ((locals.var_gov2_dn6 * assign23550_e23244) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn6 - locals.var_sp_ov_d0_dn6)))), (((locals.var_sp_ov_temp_dn7 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn7)) + ((locals.var_gov2_dn7 * assign23550_e23244) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn7 - locals.var_sp_ov_d0_dn7)))), (((locals.var_sp_ov_temp_dn8 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn8)) + ((locals.var_gov2_dn8 * assign23550_e23244) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn8 - locals.var_sp_ov_d0_dn8)))), (((locals.var_sp_ov_temp_dn9 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn9)) + ((locals.var_gov2_dn9 * assign23550_e23244) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn9 - locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_q, locals.var_sp_ov_q_dn4, locals.var_sp_ov_q_dn6, locals.var_sp_ov_q_dn7, locals.var_sp_ov_q_dn8, locals.var_sp_ov_q_dn9,)
    }
};
        locals.var_sp_ov_q = assign23550_e23248;
        locals.var_sp_ov_q_dn4 = assign23550_e23248_d_n4;
        locals.var_sp_ov_q_dn6 = assign23550_e23248_d_n6;
        locals.var_sp_ov_q_dn7 = assign23550_e23248_d_n7;
        locals.var_sp_ov_q_dn8 = assign23550_e23248_d_n8;
        locals.var_sp_ov_q_dn9 = assign23550_e23248_d_n9;
        locals.var_sp_ov_q_rv = 0.0;

        let (assign23560_e23263, assign23560_e23263_d_n4, assign23560_e23263_d_n6, assign23560_e23263_d_n7, assign23560_e23263_d_n8, assign23560_e23263_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23560_e23258: f64 = (locals.var_gov2 * 0.5);
        let assign23560_e23260: f64 = (assign23560_e23258 * locals.var_sp_ov_d0);
        let assign23560_e23261: f64 = (1.0 - assign23560_e23260);
        (assign23560_e23261, (-(((locals.var_gov2_dn4 * 0.5) * locals.var_sp_ov_d0) + (assign23560_e23258 * locals.var_sp_ov_d0_dn4))), (-(((locals.var_gov2_dn6 * 0.5) * locals.var_sp_ov_d0) + (assign23560_e23258 * locals.var_sp_ov_d0_dn6))), (-(((locals.var_gov2_dn7 * 0.5) * locals.var_sp_ov_d0) + (assign23560_e23258 * locals.var_sp_ov_d0_dn7))), (-(((locals.var_gov2_dn8 * 0.5) * locals.var_sp_ov_d0) + (assign23560_e23258 * locals.var_sp_ov_d0_dn8))), (-(((locals.var_gov2_dn9 * 0.5) * locals.var_sp_ov_d0) + (assign23560_e23258 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_xi, locals.var_sp_ov_xi_dn4, locals.var_sp_ov_xi_dn6, locals.var_sp_ov_xi_dn7, locals.var_sp_ov_xi_dn8, locals.var_sp_ov_xi_dn9,)
    }
};
        locals.var_sp_ov_xi = assign23560_e23263;
        locals.var_sp_ov_xi_dn4 = assign23560_e23263_d_n4;
        locals.var_sp_ov_xi_dn6 = assign23560_e23263_d_n6;
        locals.var_sp_ov_xi_dn7 = assign23560_e23263_d_n7;
        locals.var_sp_ov_xi_dn8 = assign23560_e23263_d_n8;
        locals.var_sp_ov_xi_dn9 = assign23560_e23263_d_n9;
        locals.var_sp_ov_xi_rv = 0.0;

        let (assign23570_e23280, assign23570_e23280_d_n4, assign23570_e23280_d_n6, assign23570_e23280_d_n7, assign23570_e23280_d_n8, assign23570_e23280_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23570_e23272: f64 = (locals.var_sp_ov_p * locals.var_sp_ov_p);
        let assign23570_e23276: f64 = (locals.var_sp_ov_xi * locals.var_sp_ov_q);
        let assign23570_e23277: f64 = (4.0 * assign23570_e23276);
        let assign23570_e23278: f64 = (assign23570_e23272 - assign23570_e23277);
        (assign23570_e23278, (((locals.var_sp_ov_p_dn4 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn4)) - (4.0 * ((locals.var_sp_ov_xi_dn4 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn4)))), (((locals.var_sp_ov_p_dn6 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn6)) - (4.0 * ((locals.var_sp_ov_xi_dn6 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn6)))), (((locals.var_sp_ov_p_dn7 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn7)) - (4.0 * ((locals.var_sp_ov_xi_dn7 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn7)))), (((locals.var_sp_ov_p_dn8 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn8)) - (4.0 * ((locals.var_sp_ov_xi_dn8 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn8)))), (((locals.var_sp_ov_p_dn9 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn9)) - (4.0 * ((locals.var_sp_ov_xi_dn9 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign23570_e23280;
        locals.var_sp_ov_temp_dn4 = assign23570_e23280_d_n4;
        locals.var_sp_ov_temp_dn6 = assign23570_e23280_d_n6;
        locals.var_sp_ov_temp_dn7 = assign23570_e23280_d_n7;
        locals.var_sp_ov_temp_dn8 = assign23570_e23280_d_n8;
        locals.var_sp_ov_temp_dn9 = assign23570_e23280_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign23580_e23296, assign23580_e23296_d_n4, assign23580_e23296_d_n6, assign23580_e23296_d_n7, assign23580_e23296_d_n8, assign23580_e23296_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23580_e23289: f64 = (2.0 * locals.var_sp_ov_q);
        let assign23580_e23292: f64 = (locals.var_sp_ov_temp).sqrt();
        let assign23580_e23293: f64 = (locals.var_sp_ov_p + assign23580_e23292);
        let assign23580_e23294: f64 = (assign23580_e23289 / assign23580_e23293);
        (assign23580_e23294, ((((2.0 * locals.var_sp_ov_q_dn4) * assign23580_e23293) - (assign23580_e23289 * (locals.var_sp_ov_p_dn4 + (locals.var_sp_ov_temp_dn4 / (2.0 * assign23580_e23292))))) / (assign23580_e23293 * assign23580_e23293)), ((((2.0 * locals.var_sp_ov_q_dn6) * assign23580_e23293) - (assign23580_e23289 * (locals.var_sp_ov_p_dn6 + (locals.var_sp_ov_temp_dn6 / (2.0 * assign23580_e23292))))) / (assign23580_e23293 * assign23580_e23293)), ((((2.0 * locals.var_sp_ov_q_dn7) * assign23580_e23293) - (assign23580_e23289 * (locals.var_sp_ov_p_dn7 + (locals.var_sp_ov_temp_dn7 / (2.0 * assign23580_e23292))))) / (assign23580_e23293 * assign23580_e23293)), ((((2.0 * locals.var_sp_ov_q_dn8) * assign23580_e23293) - (assign23580_e23289 * (locals.var_sp_ov_p_dn8 + (locals.var_sp_ov_temp_dn8 / (2.0 * assign23580_e23292))))) / (assign23580_e23293 * assign23580_e23293)), ((((2.0 * locals.var_sp_ov_q_dn9) * assign23580_e23293) - (assign23580_e23289 * (locals.var_sp_ov_p_dn9 + (locals.var_sp_ov_temp_dn9 / (2.0 * assign23580_e23292))))) / (assign23580_e23293 * assign23580_e23293)),)
    } else {
        (locals.var_sp_ov_w, locals.var_sp_ov_w_dn4, locals.var_sp_ov_w_dn6, locals.var_sp_ov_w_dn7, locals.var_sp_ov_w_dn8, locals.var_sp_ov_w_dn9,)
    }
};
        locals.var_sp_ov_w = assign23580_e23296;
        locals.var_sp_ov_w_dn4 = assign23580_e23296_d_n4;
        locals.var_sp_ov_w_dn6 = assign23580_e23296_d_n6;
        locals.var_sp_ov_w_dn7 = assign23580_e23296_d_n7;
        locals.var_sp_ov_w_dn8 = assign23580_e23296_d_n8;
        locals.var_sp_ov_w_dn9 = assign23580_e23296_d_n9;
        locals.var_sp_ov_w_rv = 0.0;

        let (assign23590_e23308, assign23590_e23308_d_n4, assign23590_e23308_d_n6, assign23590_e23308_d_n7, assign23590_e23308_d_n8, assign23590_e23308_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 != 0.0)) {
        let assign23590_e23305: f64 = (locals.var_sp_ov_y0 + locals.var_sp_ov_w);
        let assign23590_e23306: f64 = (-assign23590_e23305);
        (assign23590_e23306, (-(locals.var_sp_ov_y0_dn4 + locals.var_sp_ov_w_dn4)), (-(locals.var_sp_ov_y0_dn6 + locals.var_sp_ov_w_dn6)), (-(locals.var_sp_ov_y0_dn7 + locals.var_sp_ov_w_dn7)), (-(locals.var_sp_ov_y0_dn8 + locals.var_sp_ov_w_dn8)), (-(locals.var_sp_ov_y0_dn9 + locals.var_sp_ov_w_dn9)),)
    } else {
        (locals.var_xs_ov, locals.var_xs_ov_dn4, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7, locals.var_xs_ov_dn8, locals.var_xs_ov_dn9,)
    }
};
        locals.var_xs_ov = assign23590_e23308;
        locals.var_xs_ov_dn4 = assign23590_e23308_d_n4;
        locals.var_xs_ov_dn6 = assign23590_e23308_d_n6;
        locals.var_xs_ov_dn7 = assign23590_e23308_d_n7;
        locals.var_xs_ov_dn8 = assign23590_e23308_d_n8;
        locals.var_xs_ov_dn9 = assign23590_e23308_d_n9;
        locals.var_xs_ov_rv = 0.0;

        let (assign23600_e23326, assign23600_e23326_d_n4, assign23600_e23326_d_n6, assign23600_e23326_d_n7, assign23600_e23326_d_n8, assign23600_e23326_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23600_e23318: f64 = (locals.var_xi_ov * 1.25);
        let assign23600_e23320: f64 = (assign23600_e23318 * locals.var_inv_xg1);
        let assign23600_e23322: f64 = (assign23600_e23320 - 1.0);
        let assign23600_e23324: f64 = (assign23600_e23322 * locals.var_inv_xg1);
        (assign23600_e23324, (((((locals.var_xi_ov_dn4 * 1.25) * locals.var_inv_xg1) + (assign23600_e23318 * locals.var_inv_xg1_dn4)) * locals.var_inv_xg1) + (assign23600_e23322 * locals.var_inv_xg1_dn4)), (((((locals.var_xi_ov_dn6 * 1.25) * locals.var_inv_xg1) + (assign23600_e23318 * locals.var_inv_xg1_dn6)) * locals.var_inv_xg1) + (assign23600_e23322 * locals.var_inv_xg1_dn6)), (((((locals.var_xi_ov_dn7 * 1.25) * locals.var_inv_xg1) + (assign23600_e23318 * locals.var_inv_xg1_dn7)) * locals.var_inv_xg1) + (assign23600_e23322 * locals.var_inv_xg1_dn7)), (((((locals.var_xi_ov_dn8 * 1.25) * locals.var_inv_xg1) + (assign23600_e23318 * locals.var_inv_xg1_dn8)) * locals.var_inv_xg1) + (assign23600_e23322 * locals.var_inv_xg1_dn8)), (((((locals.var_xi_ov_dn9 * 1.25) * locals.var_inv_xg1) + (assign23600_e23318 * locals.var_inv_xg1_dn9)) * locals.var_inv_xg1) + (assign23600_e23322 * locals.var_inv_xg1_dn9)),)
    } else {
        (locals.var_sp_ov_afac, locals.var_sp_ov_afac_dn4, locals.var_sp_ov_afac_dn6, locals.var_sp_ov_afac_dn7, locals.var_sp_ov_afac_dn8, locals.var_sp_ov_afac_dn9,)
    }
};
        locals.var_sp_ov_afac = assign23600_e23326;
        locals.var_sp_ov_afac_dn4 = assign23600_e23326_d_n4;
        locals.var_sp_ov_afac_dn6 = assign23600_e23326_d_n6;
        locals.var_sp_ov_afac_dn7 = assign23600_e23326_d_n7;
        locals.var_sp_ov_afac_dn8 = assign23600_e23326_d_n8;
        locals.var_sp_ov_afac_dn9 = assign23600_e23326_d_n9;
        locals.var_sp_ov_afac_rv = 0.0;

        let (assign23610_e23344, assign23610_e23344_d_n4, assign23610_e23344_d_n6, assign23610_e23344_d_n7, assign23610_e23344_d_n8, assign23610_e23344_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23610_e23336: f64 = (locals.var_xgs_ov * locals.var_inv_xi_ov);
        let assign23610_e23340: f64 = (locals.var_sp_ov_afac * locals.var_xgs_ov);
        let assign23610_e23341: f64 = (1.0 + assign23610_e23340);
        let assign23610_e23342: f64 = (assign23610_e23336 * assign23610_e23341);
        (assign23610_e23342, ((((locals.var_xgs_ov_dn4 * locals.var_inv_xi_ov) + (locals.var_xgs_ov * locals.var_inv_xi_ov_dn4)) * assign23610_e23341) + (assign23610_e23336 * ((locals.var_sp_ov_afac_dn4 * locals.var_xgs_ov) + (locals.var_sp_ov_afac * locals.var_xgs_ov_dn4)))), ((((locals.var_xgs_ov_dn6 * locals.var_inv_xi_ov) + (locals.var_xgs_ov * locals.var_inv_xi_ov_dn6)) * assign23610_e23341) + (assign23610_e23336 * ((locals.var_sp_ov_afac_dn6 * locals.var_xgs_ov) + (locals.var_sp_ov_afac * locals.var_xgs_ov_dn6)))), ((((locals.var_xgs_ov_dn7 * locals.var_inv_xi_ov) + (locals.var_xgs_ov * locals.var_inv_xi_ov_dn7)) * assign23610_e23341) + (assign23610_e23336 * ((locals.var_sp_ov_afac_dn7 * locals.var_xgs_ov) + (locals.var_sp_ov_afac * locals.var_xgs_ov_dn7)))), ((((locals.var_xgs_ov_dn8 * locals.var_inv_xi_ov) + (locals.var_xgs_ov * locals.var_inv_xi_ov_dn8)) * assign23610_e23341) + (assign23610_e23336 * ((locals.var_sp_ov_afac_dn8 * locals.var_xgs_ov) + (locals.var_sp_ov_afac * locals.var_xgs_ov_dn8)))), ((((locals.var_xgs_ov_dn9 * locals.var_inv_xi_ov) + (locals.var_xgs_ov * locals.var_inv_xi_ov_dn9)) * assign23610_e23341) + (assign23610_e23336 * ((locals.var_sp_ov_afac_dn9 * locals.var_xgs_ov) + (locals.var_sp_ov_afac * locals.var_xgs_ov_dn9)))),)
    } else {
        (locals.var_sp_ov_xbar, locals.var_sp_ov_xbar_dn4, locals.var_sp_ov_xbar_dn6, locals.var_sp_ov_xbar_dn7, locals.var_sp_ov_xbar_dn8, locals.var_sp_ov_xbar_dn9,)
    }
};
        locals.var_sp_ov_xbar = assign23610_e23344;
        locals.var_sp_ov_xbar_dn4 = assign23610_e23344_d_n4;
        locals.var_sp_ov_xbar_dn6 = assign23610_e23344_d_n6;
        locals.var_sp_ov_xbar_dn7 = assign23610_e23344_d_n7;
        locals.var_sp_ov_xbar_dn8 = assign23610_e23344_d_n8;
        locals.var_sp_ov_xbar_dn9 = assign23610_e23344_d_n9;
        locals.var_sp_ov_xbar_rv = 0.0;

        let assign23620_e23346: f64 = (-locals.var_sp_ov_xbar);
        let assign23620_e23347: f64 = (assign23620_e23346).abs();
        let assign23620_e23349: f64 = if assign23620_e23347 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard690 = assign23620_e23349;
        locals.var_guard690_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_64(
        locals: &mut StampLocals,
    ) {
        let (assign23630_e23363, assign23630_e23363_d_n4, assign23630_e23363_d_n6, assign23630_e23363_d_n7, assign23630_e23363_d_n8, assign23630_e23363_d_n9,) = {
    if ((((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) && (locals.var_guard690 != 0.0)) {
        let assign23630_e23360: f64 = (-locals.var_sp_ov_xbar);
        let assign23630_e23361: f64 = (assign23630_e23360).exp();
        (assign23630_e23361, (assign23630_e23361 * (-locals.var_sp_ov_xbar_dn4)), (assign23630_e23361 * (-locals.var_sp_ov_xbar_dn6)), (assign23630_e23361 * (-locals.var_sp_ov_xbar_dn7)), (assign23630_e23361 * (-locals.var_sp_ov_xbar_dn8)), (assign23630_e23361 * (-locals.var_sp_ov_xbar_dn9)),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign23630_e23363;
        locals.var_sp_ov_temp_dn4 = assign23630_e23363_d_n4;
        locals.var_sp_ov_temp_dn6 = assign23630_e23363_d_n6;
        locals.var_sp_ov_temp_dn7 = assign23630_e23363_d_n7;
        locals.var_sp_ov_temp_dn8 = assign23630_e23363_d_n8;
        locals.var_sp_ov_temp_dn9 = assign23630_e23363_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let assign23640_e23365: f64 = (-locals.var_sp_ov_xbar);
        let assign23640_e23367: f64 = (-80.0);
        let assign23640_e23368: f64 = if assign23640_e23365 < assign23640_e23367 { 1.0 } else { 0.0 };
        locals.var_guard691 = assign23640_e23368;
        locals.var_guard691_rv = 0.0;

        let (assign23650_e23411, assign23650_e23411_d_n4, assign23650_e23411_d_n6, assign23650_e23411_d_n7, assign23650_e23411_d_n8, assign23650_e23411_d_n9,) = {
    if (((((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) && (locals.var_guard690 == 0.0)) && (locals.var_guard691 != 0.0)) {
        let assign23650_e23384: f64 = (-locals.var_sp_ov_xbar);
        let assign23650_e23385: f64 = (-assign23650_e23384);
        let assign23650_e23387: f64 = (assign23650_e23385 - 80.0);
        let assign23650_e23391: f64 = (-locals.var_sp_ov_xbar);
        let assign23650_e23392: f64 = (-assign23650_e23391);
        let assign23650_e23394: f64 = (assign23650_e23392 - 80.0);
        let assign23650_e23395: f64 = (0.5 * assign23650_e23394);
        let assign23650_e23398: f64 = (-locals.var_sp_ov_xbar);
        let assign23650_e23399: f64 = (-assign23650_e23398);
        let assign23650_e23401: f64 = (assign23650_e23399 - 80.0);
        let assign23650_e23403: f64 = (assign23650_e23401 * 0.3333333333333);
        let assign23650_e23404: f64 = (1.0 + assign23650_e23403);
        let assign23650_e23405: f64 = (assign23650_e23395 * assign23650_e23404);
        let assign23650_e23406: f64 = (1.0 + assign23650_e23405);
        let assign23650_e23407: f64 = (assign23650_e23387 * assign23650_e23406);
        let assign23650_e23408: f64 = (1.0 + assign23650_e23407);
        let assign23650_e23409: f64 = (1.80485e-35 / assign23650_e23408);
        (assign23650_e23409, (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn4)) * assign23650_e23406) + (assign23650_e23387 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn4))) * assign23650_e23404) + (assign23650_e23395 * ((-(-locals.var_sp_ov_xbar_dn4)) * 0.3333333333333)))))) / (assign23650_e23408 * assign23650_e23408))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn6)) * assign23650_e23406) + (assign23650_e23387 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn6))) * assign23650_e23404) + (assign23650_e23395 * ((-(-locals.var_sp_ov_xbar_dn6)) * 0.3333333333333)))))) / (assign23650_e23408 * assign23650_e23408))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn7)) * assign23650_e23406) + (assign23650_e23387 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn7))) * assign23650_e23404) + (assign23650_e23395 * ((-(-locals.var_sp_ov_xbar_dn7)) * 0.3333333333333)))))) / (assign23650_e23408 * assign23650_e23408))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn8)) * assign23650_e23406) + (assign23650_e23387 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn8))) * assign23650_e23404) + (assign23650_e23395 * ((-(-locals.var_sp_ov_xbar_dn8)) * 0.3333333333333)))))) / (assign23650_e23408 * assign23650_e23408))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn9)) * assign23650_e23406) + (assign23650_e23387 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn9))) * assign23650_e23404) + (assign23650_e23395 * ((-(-locals.var_sp_ov_xbar_dn9)) * 0.3333333333333)))))) / (assign23650_e23408 * assign23650_e23408))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign23650_e23411;
        locals.var_sp_ov_temp_dn4 = assign23650_e23411_d_n4;
        locals.var_sp_ov_temp_dn6 = assign23650_e23411_d_n6;
        locals.var_sp_ov_temp_dn7 = assign23650_e23411_d_n7;
        locals.var_sp_ov_temp_dn8 = assign23650_e23411_d_n8;
        locals.var_sp_ov_temp_dn9 = assign23650_e23411_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign23660_e23452, assign23660_e23452_d_n4, assign23660_e23452_d_n6, assign23660_e23452_d_n7, assign23660_e23452_d_n8, assign23660_e23452_d_n9,) = {
    if (((((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) && (locals.var_guard690 == 0.0)) && (locals.var_guard691 == 0.0)) {
        let assign23660_e23428: f64 = (-locals.var_sp_ov_xbar);
        let assign23660_e23430: f64 = (assign23660_e23428 - 80.0);
        let assign23660_e23434: f64 = (-locals.var_sp_ov_xbar);
        let assign23660_e23436: f64 = (assign23660_e23434 - 80.0);
        let assign23660_e23437: f64 = (0.5 * assign23660_e23436);
        let assign23660_e23440: f64 = (-locals.var_sp_ov_xbar);
        let assign23660_e23442: f64 = (assign23660_e23440 - 80.0);
        let assign23660_e23444: f64 = (assign23660_e23442 * 0.3333333333333);
        let assign23660_e23445: f64 = (1.0 + assign23660_e23444);
        let assign23660_e23446: f64 = (assign23660_e23437 * assign23660_e23445);
        let assign23660_e23447: f64 = (1.0 + assign23660_e23446);
        let assign23660_e23448: f64 = (assign23660_e23430 * assign23660_e23447);
        let assign23660_e23449: f64 = (1.0 + assign23660_e23448);
        let assign23660_e23450: f64 = (5.54062e34 * assign23660_e23449);
        (assign23660_e23450, (5.54062e34 * (((-locals.var_sp_ov_xbar_dn4) * assign23660_e23447) + (assign23660_e23430 * (((0.5 * (-locals.var_sp_ov_xbar_dn4)) * assign23660_e23445) + (assign23660_e23437 * ((-locals.var_sp_ov_xbar_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn6) * assign23660_e23447) + (assign23660_e23430 * (((0.5 * (-locals.var_sp_ov_xbar_dn6)) * assign23660_e23445) + (assign23660_e23437 * ((-locals.var_sp_ov_xbar_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn7) * assign23660_e23447) + (assign23660_e23430 * (((0.5 * (-locals.var_sp_ov_xbar_dn7)) * assign23660_e23445) + (assign23660_e23437 * ((-locals.var_sp_ov_xbar_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn8) * assign23660_e23447) + (assign23660_e23430 * (((0.5 * (-locals.var_sp_ov_xbar_dn8)) * assign23660_e23445) + (assign23660_e23437 * ((-locals.var_sp_ov_xbar_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn9) * assign23660_e23447) + (assign23660_e23430 * (((0.5 * (-locals.var_sp_ov_xbar_dn9)) * assign23660_e23445) + (assign23660_e23437 * ((-locals.var_sp_ov_xbar_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign23660_e23452;
        locals.var_sp_ov_temp_dn4 = assign23660_e23452_d_n4;
        locals.var_sp_ov_temp_dn6 = assign23660_e23452_d_n6;
        locals.var_sp_ov_temp_dn7 = assign23660_e23452_d_n7;
        locals.var_sp_ov_temp_dn8 = assign23660_e23452_d_n8;
        locals.var_sp_ov_temp_dn9 = assign23660_e23452_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign23670_e23464, assign23670_e23464_d_n4, assign23670_e23464_d_n6, assign23670_e23464_d_n7, assign23670_e23464_d_n8, assign23670_e23464_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23670_e23462: f64 = (1.0 - locals.var_sp_ov_temp);
        (assign23670_e23462, (-locals.var_sp_ov_temp_dn4), (-locals.var_sp_ov_temp_dn6), (-locals.var_sp_ov_temp_dn7), (-locals.var_sp_ov_temp_dn8), (-locals.var_sp_ov_temp_dn9),)
    } else {
        (locals.var_sp_ov_w, locals.var_sp_ov_w_dn4, locals.var_sp_ov_w_dn6, locals.var_sp_ov_w_dn7, locals.var_sp_ov_w_dn8, locals.var_sp_ov_w_dn9,)
    }
};
        locals.var_sp_ov_w = assign23670_e23464;
        locals.var_sp_ov_w_dn4 = assign23670_e23464_d_n4;
        locals.var_sp_ov_w_dn6 = assign23670_e23464_d_n6;
        locals.var_sp_ov_w_dn7 = assign23670_e23464_d_n7;
        locals.var_sp_ov_w_dn8 = assign23670_e23464_d_n8;
        locals.var_sp_ov_w_dn9 = assign23670_e23464_d_n9;
        locals.var_sp_ov_w_rv = 0.0;

        let (assign23680_e23489, assign23680_e23489_d_n4, assign23680_e23489_d_n6, assign23680_e23489_d_n7, assign23680_e23489_d_n8, assign23680_e23489_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23680_e23475: f64 = (locals.var_gov2 * 0.5);
        let assign23680_e23476: f64 = (locals.var_xgs_ov + assign23680_e23475);
        let assign23680_e23481: f64 = (locals.var_gov2 * 0.25);
        let assign23680_e23482: f64 = (locals.var_xgs_ov + assign23680_e23481);
        let assign23680_e23484: f64 = (assign23680_e23482 - locals.var_sp_ov_w);
        let assign23680_e23485: f64 = (assign23680_e23484).sqrt();
        let assign23680_e23486: f64 = (locals.var_gov * assign23680_e23485);
        let assign23680_e23487: f64 = (assign23680_e23476 - assign23680_e23486);
        (assign23680_e23487, ((locals.var_xgs_ov_dn4 + (locals.var_gov2_dn4 * 0.5)) - ((locals.var_gov_dn4 * assign23680_e23485) + (locals.var_gov * (((locals.var_xgs_ov_dn4 + (locals.var_gov2_dn4 * 0.25)) - locals.var_sp_ov_w_dn4) / (2.0 * assign23680_e23485))))), ((locals.var_xgs_ov_dn6 + (locals.var_gov2_dn6 * 0.5)) - ((locals.var_gov_dn6 * assign23680_e23485) + (locals.var_gov * (((locals.var_xgs_ov_dn6 + (locals.var_gov2_dn6 * 0.25)) - locals.var_sp_ov_w_dn6) / (2.0 * assign23680_e23485))))), ((locals.var_xgs_ov_dn7 + (locals.var_gov2_dn7 * 0.5)) - ((locals.var_gov_dn7 * assign23680_e23485) + (locals.var_gov * (((locals.var_xgs_ov_dn7 + (locals.var_gov2_dn7 * 0.25)) - locals.var_sp_ov_w_dn7) / (2.0 * assign23680_e23485))))), ((locals.var_xgs_ov_dn8 + (locals.var_gov2_dn8 * 0.5)) - ((locals.var_gov_dn8 * assign23680_e23485) + (locals.var_gov * (((locals.var_xgs_ov_dn8 + (locals.var_gov2_dn8 * 0.25)) - locals.var_sp_ov_w_dn8) / (2.0 * assign23680_e23485))))), ((locals.var_xgs_ov_dn9 + (locals.var_gov2_dn9 * 0.5)) - ((locals.var_gov_dn9 * assign23680_e23485) + (locals.var_gov * (((locals.var_xgs_ov_dn9 + (locals.var_gov2_dn9 * 0.25)) - locals.var_sp_ov_w_dn9) / (2.0 * assign23680_e23485))))),)
    } else {
        (locals.var_sp_ov_x0, locals.var_sp_ov_x0_dn4, locals.var_sp_ov_x0_dn6, locals.var_sp_ov_x0_dn7, locals.var_sp_ov_x0_dn8, locals.var_sp_ov_x0_dn9,)
    }
};
        locals.var_sp_ov_x0 = assign23680_e23489;
        locals.var_sp_ov_x0_dn4 = assign23680_e23489_d_n4;
        locals.var_sp_ov_x0_dn6 = assign23680_e23489_d_n6;
        locals.var_sp_ov_x0_dn7 = assign23680_e23489_d_n7;
        locals.var_sp_ov_x0_dn8 = assign23680_e23489_d_n8;
        locals.var_sp_ov_x0_dn9 = assign23680_e23489_d_n9;
        locals.var_sp_ov_x0_rv = 0.0;

        let assign23690_e23491: f64 = (-locals.var_sp_ov_x0);
        let assign23690_e23492: f64 = (assign23690_e23491).abs();
        let assign23690_e23494: f64 = if assign23690_e23492 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard692 = assign23690_e23494;
        locals.var_guard692_rv = 0.0;

        let (assign23700_e23508, assign23700_e23508_d_n4, assign23700_e23508_d_n6, assign23700_e23508_d_n7, assign23700_e23508_d_n8, assign23700_e23508_d_n9,) = {
    if ((((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) && (locals.var_guard692 != 0.0)) {
        let assign23700_e23505: f64 = (-locals.var_sp_ov_x0);
        let assign23700_e23506: f64 = (assign23700_e23505).exp();
        (assign23700_e23506, (assign23700_e23506 * (-locals.var_sp_ov_x0_dn4)), (assign23700_e23506 * (-locals.var_sp_ov_x0_dn6)), (assign23700_e23506 * (-locals.var_sp_ov_x0_dn7)), (assign23700_e23506 * (-locals.var_sp_ov_x0_dn8)), (assign23700_e23506 * (-locals.var_sp_ov_x0_dn9)),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign23700_e23508;
        locals.var_sp_ov_d0_dn4 = assign23700_e23508_d_n4;
        locals.var_sp_ov_d0_dn6 = assign23700_e23508_d_n6;
        locals.var_sp_ov_d0_dn7 = assign23700_e23508_d_n7;
        locals.var_sp_ov_d0_dn8 = assign23700_e23508_d_n8;
        locals.var_sp_ov_d0_dn9 = assign23700_e23508_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let assign23710_e23510: f64 = (-locals.var_sp_ov_x0);
        let assign23710_e23512: f64 = (-80.0);
        let assign23710_e23513: f64 = if assign23710_e23510 < assign23710_e23512 { 1.0 } else { 0.0 };
        locals.var_guard693 = assign23710_e23513;
        locals.var_guard693_rv = 0.0;

        let (assign23720_e23556, assign23720_e23556_d_n4, assign23720_e23556_d_n6, assign23720_e23556_d_n7, assign23720_e23556_d_n8, assign23720_e23556_d_n9,) = {
    if (((((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) && (locals.var_guard692 == 0.0)) && (locals.var_guard693 != 0.0)) {
        let assign23720_e23529: f64 = (-locals.var_sp_ov_x0);
        let assign23720_e23530: f64 = (-assign23720_e23529);
        let assign23720_e23532: f64 = (assign23720_e23530 - 80.0);
        let assign23720_e23536: f64 = (-locals.var_sp_ov_x0);
        let assign23720_e23537: f64 = (-assign23720_e23536);
        let assign23720_e23539: f64 = (assign23720_e23537 - 80.0);
        let assign23720_e23540: f64 = (0.5 * assign23720_e23539);
        let assign23720_e23543: f64 = (-locals.var_sp_ov_x0);
        let assign23720_e23544: f64 = (-assign23720_e23543);
        let assign23720_e23546: f64 = (assign23720_e23544 - 80.0);
        let assign23720_e23548: f64 = (assign23720_e23546 * 0.3333333333333);
        let assign23720_e23549: f64 = (1.0 + assign23720_e23548);
        let assign23720_e23550: f64 = (assign23720_e23540 * assign23720_e23549);
        let assign23720_e23551: f64 = (1.0 + assign23720_e23550);
        let assign23720_e23552: f64 = (assign23720_e23532 * assign23720_e23551);
        let assign23720_e23553: f64 = (1.0 + assign23720_e23552);
        let assign23720_e23554: f64 = (1.80485e-35 / assign23720_e23553);
        (assign23720_e23554, (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn4)) * assign23720_e23551) + (assign23720_e23532 * (((0.5 * (-(-locals.var_sp_ov_x0_dn4))) * assign23720_e23549) + (assign23720_e23540 * ((-(-locals.var_sp_ov_x0_dn4)) * 0.3333333333333)))))) / (assign23720_e23553 * assign23720_e23553))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn6)) * assign23720_e23551) + (assign23720_e23532 * (((0.5 * (-(-locals.var_sp_ov_x0_dn6))) * assign23720_e23549) + (assign23720_e23540 * ((-(-locals.var_sp_ov_x0_dn6)) * 0.3333333333333)))))) / (assign23720_e23553 * assign23720_e23553))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn7)) * assign23720_e23551) + (assign23720_e23532 * (((0.5 * (-(-locals.var_sp_ov_x0_dn7))) * assign23720_e23549) + (assign23720_e23540 * ((-(-locals.var_sp_ov_x0_dn7)) * 0.3333333333333)))))) / (assign23720_e23553 * assign23720_e23553))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn8)) * assign23720_e23551) + (assign23720_e23532 * (((0.5 * (-(-locals.var_sp_ov_x0_dn8))) * assign23720_e23549) + (assign23720_e23540 * ((-(-locals.var_sp_ov_x0_dn8)) * 0.3333333333333)))))) / (assign23720_e23553 * assign23720_e23553))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn9)) * assign23720_e23551) + (assign23720_e23532 * (((0.5 * (-(-locals.var_sp_ov_x0_dn9))) * assign23720_e23549) + (assign23720_e23540 * ((-(-locals.var_sp_ov_x0_dn9)) * 0.3333333333333)))))) / (assign23720_e23553 * assign23720_e23553))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign23720_e23556;
        locals.var_sp_ov_d0_dn4 = assign23720_e23556_d_n4;
        locals.var_sp_ov_d0_dn6 = assign23720_e23556_d_n6;
        locals.var_sp_ov_d0_dn7 = assign23720_e23556_d_n7;
        locals.var_sp_ov_d0_dn8 = assign23720_e23556_d_n8;
        locals.var_sp_ov_d0_dn9 = assign23720_e23556_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign23730_e23597, assign23730_e23597_d_n4, assign23730_e23597_d_n6, assign23730_e23597_d_n7, assign23730_e23597_d_n8, assign23730_e23597_d_n9,) = {
    if (((((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) && (locals.var_guard692 == 0.0)) && (locals.var_guard693 == 0.0)) {
        let assign23730_e23573: f64 = (-locals.var_sp_ov_x0);
        let assign23730_e23575: f64 = (assign23730_e23573 - 80.0);
        let assign23730_e23579: f64 = (-locals.var_sp_ov_x0);
        let assign23730_e23581: f64 = (assign23730_e23579 - 80.0);
        let assign23730_e23582: f64 = (0.5 * assign23730_e23581);
        let assign23730_e23585: f64 = (-locals.var_sp_ov_x0);
        let assign23730_e23587: f64 = (assign23730_e23585 - 80.0);
        let assign23730_e23589: f64 = (assign23730_e23587 * 0.3333333333333);
        let assign23730_e23590: f64 = (1.0 + assign23730_e23589);
        let assign23730_e23591: f64 = (assign23730_e23582 * assign23730_e23590);
        let assign23730_e23592: f64 = (1.0 + assign23730_e23591);
        let assign23730_e23593: f64 = (assign23730_e23575 * assign23730_e23592);
        let assign23730_e23594: f64 = (1.0 + assign23730_e23593);
        let assign23730_e23595: f64 = (5.54062e34 * assign23730_e23594);
        (assign23730_e23595, (5.54062e34 * (((-locals.var_sp_ov_x0_dn4) * assign23730_e23592) + (assign23730_e23575 * (((0.5 * (-locals.var_sp_ov_x0_dn4)) * assign23730_e23590) + (assign23730_e23582 * ((-locals.var_sp_ov_x0_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn6) * assign23730_e23592) + (assign23730_e23575 * (((0.5 * (-locals.var_sp_ov_x0_dn6)) * assign23730_e23590) + (assign23730_e23582 * ((-locals.var_sp_ov_x0_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn7) * assign23730_e23592) + (assign23730_e23575 * (((0.5 * (-locals.var_sp_ov_x0_dn7)) * assign23730_e23590) + (assign23730_e23582 * ((-locals.var_sp_ov_x0_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn8) * assign23730_e23592) + (assign23730_e23575 * (((0.5 * (-locals.var_sp_ov_x0_dn8)) * assign23730_e23590) + (assign23730_e23582 * ((-locals.var_sp_ov_x0_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn9) * assign23730_e23592) + (assign23730_e23575 * (((0.5 * (-locals.var_sp_ov_x0_dn9)) * assign23730_e23590) + (assign23730_e23582 * ((-locals.var_sp_ov_x0_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign23730_e23597;
        locals.var_sp_ov_d0_dn4 = assign23730_e23597_d_n4;
        locals.var_sp_ov_d0_dn6 = assign23730_e23597_d_n6;
        locals.var_sp_ov_d0_dn7 = assign23730_e23597_d_n7;
        locals.var_sp_ov_d0_dn8 = assign23730_e23597_d_n8;
        locals.var_sp_ov_d0_dn9 = assign23730_e23597_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign23740_e23617, assign23740_e23617_d_n4, assign23740_e23617_d_n6, assign23740_e23617_d_n7, assign23740_e23617_d_n8, assign23740_e23617_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23740_e23608: f64 = (locals.var_xgs_ov - locals.var_sp_ov_x0);
        let assign23740_e23609: f64 = (2.0 * assign23740_e23608);
        let assign23740_e23613: f64 = (1.0 - locals.var_sp_ov_d0);
        let assign23740_e23614: f64 = (locals.var_gov2 * assign23740_e23613);
        let assign23740_e23615: f64 = (assign23740_e23609 + assign23740_e23614);
        (assign23740_e23615, ((2.0 * (locals.var_xgs_ov_dn4 - locals.var_sp_ov_x0_dn4)) + ((locals.var_gov2_dn4 * assign23740_e23613) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn4)))), ((2.0 * (locals.var_xgs_ov_dn6 - locals.var_sp_ov_x0_dn6)) + ((locals.var_gov2_dn6 * assign23740_e23613) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn6)))), ((2.0 * (locals.var_xgs_ov_dn7 - locals.var_sp_ov_x0_dn7)) + ((locals.var_gov2_dn7 * assign23740_e23613) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn7)))), ((2.0 * (locals.var_xgs_ov_dn8 - locals.var_sp_ov_x0_dn8)) + ((locals.var_gov2_dn8 * assign23740_e23613) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn8)))), ((2.0 * (locals.var_xgs_ov_dn9 - locals.var_sp_ov_x0_dn9)) + ((locals.var_gov2_dn9 * assign23740_e23613) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_p, locals.var_sp_ov_p_dn4, locals.var_sp_ov_p_dn6, locals.var_sp_ov_p_dn7, locals.var_sp_ov_p_dn8, locals.var_sp_ov_p_dn9,)
    }
};
        locals.var_sp_ov_p = assign23740_e23617;
        locals.var_sp_ov_p_dn4 = assign23740_e23617_d_n4;
        locals.var_sp_ov_p_dn6 = assign23740_e23617_d_n6;
        locals.var_sp_ov_p_dn7 = assign23740_e23617_d_n7;
        locals.var_sp_ov_p_dn8 = assign23740_e23617_d_n8;
        locals.var_sp_ov_p_dn9 = assign23740_e23617_d_n9;
        locals.var_sp_ov_p_rv = 0.0;

        let (assign23750_e23641, assign23750_e23641_d_n4, assign23750_e23641_d_n6, assign23750_e23641_d_n7, assign23750_e23641_d_n8, assign23750_e23641_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23750_e23627: f64 = (locals.var_xgs_ov - locals.var_sp_ov_x0);
        let assign23750_e23630: f64 = (locals.var_xgs_ov - locals.var_sp_ov_x0);
        let assign23750_e23631: f64 = (assign23750_e23627 * assign23750_e23630);
        let assign23750_e23635: f64 = (locals.var_sp_ov_x0 - 1.0);
        let assign23750_e23637: f64 = (assign23750_e23635 + locals.var_sp_ov_d0);
        let assign23750_e23638: f64 = (locals.var_gov2 * assign23750_e23637);
        let assign23750_e23639: f64 = (assign23750_e23631 - assign23750_e23638);
        (assign23750_e23639, ((((locals.var_xgs_ov_dn4 - locals.var_sp_ov_x0_dn4) * assign23750_e23630) + (assign23750_e23627 * (locals.var_xgs_ov_dn4 - locals.var_sp_ov_x0_dn4))) - ((locals.var_gov2_dn4 * assign23750_e23637) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn4 + locals.var_sp_ov_d0_dn4)))), ((((locals.var_xgs_ov_dn6 - locals.var_sp_ov_x0_dn6) * assign23750_e23630) + (assign23750_e23627 * (locals.var_xgs_ov_dn6 - locals.var_sp_ov_x0_dn6))) - ((locals.var_gov2_dn6 * assign23750_e23637) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn6 + locals.var_sp_ov_d0_dn6)))), ((((locals.var_xgs_ov_dn7 - locals.var_sp_ov_x0_dn7) * assign23750_e23630) + (assign23750_e23627 * (locals.var_xgs_ov_dn7 - locals.var_sp_ov_x0_dn7))) - ((locals.var_gov2_dn7 * assign23750_e23637) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn7 + locals.var_sp_ov_d0_dn7)))), ((((locals.var_xgs_ov_dn8 - locals.var_sp_ov_x0_dn8) * assign23750_e23630) + (assign23750_e23627 * (locals.var_xgs_ov_dn8 - locals.var_sp_ov_x0_dn8))) - ((locals.var_gov2_dn8 * assign23750_e23637) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn8 + locals.var_sp_ov_d0_dn8)))), ((((locals.var_xgs_ov_dn9 - locals.var_sp_ov_x0_dn9) * assign23750_e23630) + (assign23750_e23627 * (locals.var_xgs_ov_dn9 - locals.var_sp_ov_x0_dn9))) - ((locals.var_gov2_dn9 * assign23750_e23637) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn9 + locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_q, locals.var_sp_ov_q_dn4, locals.var_sp_ov_q_dn6, locals.var_sp_ov_q_dn7, locals.var_sp_ov_q_dn8, locals.var_sp_ov_q_dn9,)
    }
};
        locals.var_sp_ov_q = assign23750_e23641;
        locals.var_sp_ov_q_dn4 = assign23750_e23641_d_n4;
        locals.var_sp_ov_q_dn6 = assign23750_e23641_d_n6;
        locals.var_sp_ov_q_dn7 = assign23750_e23641_d_n7;
        locals.var_sp_ov_q_dn8 = assign23750_e23641_d_n8;
        locals.var_sp_ov_q_dn9 = assign23750_e23641_d_n9;
        locals.var_sp_ov_q_rv = 0.0;

        let (assign23760_e23657, assign23760_e23657_d_n4, assign23760_e23657_d_n6, assign23760_e23657_d_n7, assign23760_e23657_d_n8, assign23760_e23657_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23760_e23652: f64 = (locals.var_gov2 * 0.5);
        let assign23760_e23654: f64 = (assign23760_e23652 * locals.var_sp_ov_d0);
        let assign23760_e23655: f64 = (1.0 - assign23760_e23654);
        (assign23760_e23655, (-(((locals.var_gov2_dn4 * 0.5) * locals.var_sp_ov_d0) + (assign23760_e23652 * locals.var_sp_ov_d0_dn4))), (-(((locals.var_gov2_dn6 * 0.5) * locals.var_sp_ov_d0) + (assign23760_e23652 * locals.var_sp_ov_d0_dn6))), (-(((locals.var_gov2_dn7 * 0.5) * locals.var_sp_ov_d0) + (assign23760_e23652 * locals.var_sp_ov_d0_dn7))), (-(((locals.var_gov2_dn8 * 0.5) * locals.var_sp_ov_d0) + (assign23760_e23652 * locals.var_sp_ov_d0_dn8))), (-(((locals.var_gov2_dn9 * 0.5) * locals.var_sp_ov_d0) + (assign23760_e23652 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_xi, locals.var_sp_ov_xi_dn4, locals.var_sp_ov_xi_dn6, locals.var_sp_ov_xi_dn7, locals.var_sp_ov_xi_dn8, locals.var_sp_ov_xi_dn9,)
    }
};
        locals.var_sp_ov_xi = assign23760_e23657;
        locals.var_sp_ov_xi_dn4 = assign23760_e23657_d_n4;
        locals.var_sp_ov_xi_dn6 = assign23760_e23657_d_n6;
        locals.var_sp_ov_xi_dn7 = assign23760_e23657_d_n7;
        locals.var_sp_ov_xi_dn8 = assign23760_e23657_d_n8;
        locals.var_sp_ov_xi_dn9 = assign23760_e23657_d_n9;
        locals.var_sp_ov_xi_rv = 0.0;

        let (assign23770_e23675, assign23770_e23675_d_n4, assign23770_e23675_d_n6, assign23770_e23675_d_n7, assign23770_e23675_d_n8, assign23770_e23675_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23770_e23667: f64 = (locals.var_sp_ov_p * locals.var_sp_ov_p);
        let assign23770_e23671: f64 = (locals.var_sp_ov_xi * locals.var_sp_ov_q);
        let assign23770_e23672: f64 = (4.0 * assign23770_e23671);
        let assign23770_e23673: f64 = (assign23770_e23667 - assign23770_e23672);
        (assign23770_e23673, (((locals.var_sp_ov_p_dn4 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn4)) - (4.0 * ((locals.var_sp_ov_xi_dn4 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn4)))), (((locals.var_sp_ov_p_dn6 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn6)) - (4.0 * ((locals.var_sp_ov_xi_dn6 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn6)))), (((locals.var_sp_ov_p_dn7 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn7)) - (4.0 * ((locals.var_sp_ov_xi_dn7 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn7)))), (((locals.var_sp_ov_p_dn8 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn8)) - (4.0 * ((locals.var_sp_ov_xi_dn8 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn8)))), (((locals.var_sp_ov_p_dn9 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn9)) - (4.0 * ((locals.var_sp_ov_xi_dn9 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign23770_e23675;
        locals.var_sp_ov_temp_dn4 = assign23770_e23675_d_n4;
        locals.var_sp_ov_temp_dn6 = assign23770_e23675_d_n6;
        locals.var_sp_ov_temp_dn7 = assign23770_e23675_d_n7;
        locals.var_sp_ov_temp_dn8 = assign23770_e23675_d_n8;
        locals.var_sp_ov_temp_dn9 = assign23770_e23675_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign23780_e23692, assign23780_e23692_d_n4, assign23780_e23692_d_n6, assign23780_e23692_d_n7, assign23780_e23692_d_n8, assign23780_e23692_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23780_e23685: f64 = (2.0 * locals.var_sp_ov_q);
        let assign23780_e23688: f64 = (locals.var_sp_ov_temp).sqrt();
        let assign23780_e23689: f64 = (locals.var_sp_ov_p + assign23780_e23688);
        let assign23780_e23690: f64 = (assign23780_e23685 / assign23780_e23689);
        (assign23780_e23690, ((((2.0 * locals.var_sp_ov_q_dn4) * assign23780_e23689) - (assign23780_e23685 * (locals.var_sp_ov_p_dn4 + (locals.var_sp_ov_temp_dn4 / (2.0 * assign23780_e23688))))) / (assign23780_e23689 * assign23780_e23689)), ((((2.0 * locals.var_sp_ov_q_dn6) * assign23780_e23689) - (assign23780_e23685 * (locals.var_sp_ov_p_dn6 + (locals.var_sp_ov_temp_dn6 / (2.0 * assign23780_e23688))))) / (assign23780_e23689 * assign23780_e23689)), ((((2.0 * locals.var_sp_ov_q_dn7) * assign23780_e23689) - (assign23780_e23685 * (locals.var_sp_ov_p_dn7 + (locals.var_sp_ov_temp_dn7 / (2.0 * assign23780_e23688))))) / (assign23780_e23689 * assign23780_e23689)), ((((2.0 * locals.var_sp_ov_q_dn8) * assign23780_e23689) - (assign23780_e23685 * (locals.var_sp_ov_p_dn8 + (locals.var_sp_ov_temp_dn8 / (2.0 * assign23780_e23688))))) / (assign23780_e23689 * assign23780_e23689)), ((((2.0 * locals.var_sp_ov_q_dn9) * assign23780_e23689) - (assign23780_e23685 * (locals.var_sp_ov_p_dn9 + (locals.var_sp_ov_temp_dn9 / (2.0 * assign23780_e23688))))) / (assign23780_e23689 * assign23780_e23689)),)
    } else {
        (locals.var_sp_ov_u, locals.var_sp_ov_u_dn4, locals.var_sp_ov_u_dn6, locals.var_sp_ov_u_dn7, locals.var_sp_ov_u_dn8, locals.var_sp_ov_u_dn9,)
    }
};
        locals.var_sp_ov_u = assign23780_e23692;
        locals.var_sp_ov_u_dn4 = assign23780_e23692_d_n4;
        locals.var_sp_ov_u_dn6 = assign23780_e23692_d_n6;
        locals.var_sp_ov_u_dn7 = assign23780_e23692_d_n7;
        locals.var_sp_ov_u_dn8 = assign23780_e23692_d_n8;
        locals.var_sp_ov_u_dn9 = assign23780_e23692_d_n9;
        locals.var_sp_ov_u_rv = 0.0;

        let (assign23790_e23704, assign23790_e23704_d_n4, assign23790_e23704_d_n6, assign23790_e23704_d_n7, assign23790_e23704_d_n8, assign23790_e23704_d_n9,) = {
    if (((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) && (locals.var_guard687 == 0.0)) {
        let assign23790_e23702: f64 = (locals.var_sp_ov_x0 + locals.var_sp_ov_u);
        (assign23790_e23702, (locals.var_sp_ov_x0_dn4 + locals.var_sp_ov_u_dn4), (locals.var_sp_ov_x0_dn6 + locals.var_sp_ov_u_dn6), (locals.var_sp_ov_x0_dn7 + locals.var_sp_ov_u_dn7), (locals.var_sp_ov_x0_dn8 + locals.var_sp_ov_u_dn8), (locals.var_sp_ov_x0_dn9 + locals.var_sp_ov_u_dn9),)
    } else {
        (locals.var_xs_ov, locals.var_xs_ov_dn4, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7, locals.var_xs_ov_dn8, locals.var_xs_ov_dn9,)
    }
};
        locals.var_xs_ov = assign23790_e23704;
        locals.var_xs_ov_dn4 = assign23790_e23704_d_n4;
        locals.var_xs_ov_dn6 = assign23790_e23704_d_n6;
        locals.var_xs_ov_dn7 = assign23790_e23704_d_n7;
        locals.var_xs_ov_dn8 = assign23790_e23704_d_n8;
        locals.var_xs_ov_dn9 = assign23790_e23704_d_n9;
        locals.var_xs_ov_rv = 0.0;

        let (assign23800_e23712, assign23800_e23712_d_n4, assign23800_e23712_d_n6, assign23800_e23712_d_n7, assign23800_e23712_d_n8, assign23800_e23712_d_n9,) = {
    if ((locals.var_guard685 != 0.0) && (locals.var_guard686 == 0.0)) {
        let assign23800_e23710: f64 = (-locals.var_xs_ov);
        (assign23800_e23710, (-locals.var_xs_ov_dn4), (-locals.var_xs_ov_dn6), (-locals.var_xs_ov_dn7), (-locals.var_xs_ov_dn8), (-locals.var_xs_ov_dn9),)
    } else {
        (locals.var_xs_ov, locals.var_xs_ov_dn4, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7, locals.var_xs_ov_dn8, locals.var_xs_ov_dn9,)
    }
};
        locals.var_xs_ov = assign23800_e23712;
        locals.var_xs_ov_dn4 = assign23800_e23712_d_n4;
        locals.var_xs_ov_dn6 = assign23800_e23712_d_n6;
        locals.var_xs_ov_dn7 = assign23800_e23712_d_n7;
        locals.var_xs_ov_dn8 = assign23800_e23712_d_n8;
        locals.var_xs_ov_dn9 = assign23800_e23712_d_n9;
        locals.var_xs_ov_rv = 0.0;

        let assign23810_e23715: f64 = if locals.var_cov_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard694 = assign23810_e23715;
        locals.var_guard694_rv = 0.0;

        let assign23820_e23717: f64 = (locals.var_xgs_ovcv).abs();
        let assign23820_e23719: f64 = if assign23820_e23717 <= locals.var_x_mrg_ov { 1.0 } else { 0.0 };
        locals.var_guard695 = assign23820_e23719;
        locals.var_guard695_rv = 0.0;

        let (assign23830_e23728, assign23830_e23728_d_n4, assign23830_e23728_d_n6, assign23830_e23728_d_n7, assign23830_e23728_d_n8, assign23830_e23728_d_n9,) = {
    if ((locals.var_guard694 != 0.0) && (locals.var_guard695 != 0.0)) {
        let assign23830_e23724: f64 = (-locals.var_xgs_ovcv);
        let assign23830_e23726: f64 = (assign23830_e23724 * locals.var_inv_xi_ov);
        (assign23830_e23726, (((-locals.var_xgs_ovcv_dn4) * locals.var_inv_xi_ov) + (assign23830_e23724 * locals.var_inv_xi_ov_dn4)), (((-locals.var_xgs_ovcv_dn6) * locals.var_inv_xi_ov) + (assign23830_e23724 * locals.var_inv_xi_ov_dn6)), (((-locals.var_xgs_ovcv_dn7) * locals.var_inv_xi_ov) + (assign23830_e23724 * locals.var_inv_xi_ov_dn7)), (((-locals.var_xgs_ovcv_dn8) * locals.var_inv_xi_ov) + (assign23830_e23724 * locals.var_inv_xi_ov_dn8)), (((-locals.var_xgs_ovcv_dn9) * locals.var_inv_xi_ov) + (assign23830_e23724 * locals.var_inv_xi_ov_dn9)),)
    } else {
        (locals.var_xs_ovcv, locals.var_xs_ovcv_dn4, locals.var_xs_ovcv_dn6, locals.var_xs_ovcv_dn7, locals.var_xs_ovcv_dn8, locals.var_xs_ovcv_dn9,)
    }
};
        locals.var_xs_ovcv = assign23830_e23728;
        locals.var_xs_ovcv_dn4 = assign23830_e23728_d_n4;
        locals.var_xs_ovcv_dn6 = assign23830_e23728_d_n6;
        locals.var_xs_ovcv_dn7 = assign23830_e23728_d_n7;
        locals.var_xs_ovcv_dn8 = assign23830_e23728_d_n8;
        locals.var_xs_ovcv_dn9 = assign23830_e23728_d_n9;
        locals.var_xs_ovcv_rv = 0.0;

        let assign23840_e23731: f64 = (-locals.var_x_mrg_ov);
        let assign23840_e23732: f64 = if locals.var_xgs_ovcv < assign23840_e23731 { 1.0 } else { 0.0 };
        locals.var_guard696 = assign23840_e23732;
        locals.var_guard696_rv = 0.0;

        let (assign23850_e23742, assign23850_e23742_d_n4, assign23850_e23742_d_n6, assign23850_e23742_d_n7, assign23850_e23742_d_n8, assign23850_e23742_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign23850_e23740: f64 = (-locals.var_xgs_ovcv);
        (assign23850_e23740, (-locals.var_xgs_ovcv_dn4), (-locals.var_xgs_ovcv_dn6), (-locals.var_xgs_ovcv_dn7), (-locals.var_xgs_ovcv_dn8), (-locals.var_xgs_ovcv_dn9),)
    } else {
        (locals.var_sp_ov_ygf, locals.var_sp_ov_ygf_dn4, locals.var_sp_ov_ygf_dn6, locals.var_sp_ov_ygf_dn7, locals.var_sp_ov_ygf_dn8, locals.var_sp_ov_ygf_dn9,)
    }
};
        locals.var_sp_ov_ygf = assign23850_e23742;
        locals.var_sp_ov_ygf_dn4 = assign23850_e23742_d_n4;
        locals.var_sp_ov_ygf_dn6 = assign23850_e23742_d_n6;
        locals.var_sp_ov_ygf_dn7 = assign23850_e23742_d_n7;
        locals.var_sp_ov_ygf_dn8 = assign23850_e23742_d_n8;
        locals.var_sp_ov_ygf_dn9 = assign23850_e23742_d_n9;
        locals.var_sp_ov_ygf_rv = 0.0;

        let (assign23860_e23755, assign23860_e23755_d_n4, assign23860_e23755_d_n6, assign23860_e23755_d_n7, assign23860_e23755_d_n8, assign23860_e23755_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign23860_e23751: f64 = (1.25 * locals.var_sp_ov_ygf);
        let assign23860_e23753: f64 = (assign23860_e23751 * locals.var_inv_xi_ov);
        (assign23860_e23753, (((1.25 * locals.var_sp_ov_ygf_dn4) * locals.var_inv_xi_ov) + (assign23860_e23751 * locals.var_inv_xi_ov_dn4)), (((1.25 * locals.var_sp_ov_ygf_dn6) * locals.var_inv_xi_ov) + (assign23860_e23751 * locals.var_inv_xi_ov_dn6)), (((1.25 * locals.var_sp_ov_ygf_dn7) * locals.var_inv_xi_ov) + (assign23860_e23751 * locals.var_inv_xi_ov_dn7)), (((1.25 * locals.var_sp_ov_ygf_dn8) * locals.var_inv_xi_ov) + (assign23860_e23751 * locals.var_inv_xi_ov_dn8)), (((1.25 * locals.var_sp_ov_ygf_dn9) * locals.var_inv_xi_ov) + (assign23860_e23751 * locals.var_inv_xi_ov_dn9)),)
    } else {
        (locals.var_sp_ov_z, locals.var_sp_ov_z_dn4, locals.var_sp_ov_z_dn6, locals.var_sp_ov_z_dn7, locals.var_sp_ov_z_dn8, locals.var_sp_ov_z_dn9,)
    }
};
        locals.var_sp_ov_z = assign23860_e23755;
        locals.var_sp_ov_z_dn4 = assign23860_e23755_d_n4;
        locals.var_sp_ov_z_dn6 = assign23860_e23755_d_n6;
        locals.var_sp_ov_z_dn7 = assign23860_e23755_d_n7;
        locals.var_sp_ov_z_dn8 = assign23860_e23755_d_n8;
        locals.var_sp_ov_z_dn9 = assign23860_e23755_d_n9;
        locals.var_sp_ov_z_rv = 0.0;

        let (assign23870_e23779, assign23870_e23779_d_n4, assign23870_e23779_d_n6, assign23870_e23779_d_n7, assign23870_e23779_d_n8, assign23870_e23779_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign23870_e23765: f64 = (locals.var_sp_ov_z + 10.0);
        let assign23870_e23768: f64 = (locals.var_sp_ov_z - 6.0);
        let assign23870_e23771: f64 = (locals.var_sp_ov_z - 6.0);
        let assign23870_e23772: f64 = (assign23870_e23768 * assign23870_e23771);
        let assign23870_e23774: f64 = (assign23870_e23772 + 64.0);
        let assign23870_e23775: f64 = (assign23870_e23774).sqrt();
        let assign23870_e23776: f64 = (assign23870_e23765 - assign23870_e23775);
        let assign23870_e23777: f64 = (0.5 * assign23870_e23776);
        (assign23870_e23777, (0.5 * (locals.var_sp_ov_z_dn4 - (((locals.var_sp_ov_z_dn4 * assign23870_e23771) + (assign23870_e23768 * locals.var_sp_ov_z_dn4)) / (2.0 * assign23870_e23775)))), (0.5 * (locals.var_sp_ov_z_dn6 - (((locals.var_sp_ov_z_dn6 * assign23870_e23771) + (assign23870_e23768 * locals.var_sp_ov_z_dn6)) / (2.0 * assign23870_e23775)))), (0.5 * (locals.var_sp_ov_z_dn7 - (((locals.var_sp_ov_z_dn7 * assign23870_e23771) + (assign23870_e23768 * locals.var_sp_ov_z_dn7)) / (2.0 * assign23870_e23775)))), (0.5 * (locals.var_sp_ov_z_dn8 - (((locals.var_sp_ov_z_dn8 * assign23870_e23771) + (assign23870_e23768 * locals.var_sp_ov_z_dn8)) / (2.0 * assign23870_e23775)))), (0.5 * (locals.var_sp_ov_z_dn9 - (((locals.var_sp_ov_z_dn9 * assign23870_e23771) + (assign23870_e23768 * locals.var_sp_ov_z_dn9)) / (2.0 * assign23870_e23775)))),)
    } else {
        (locals.var_sp_ov_eta, locals.var_sp_ov_eta_dn4, locals.var_sp_ov_eta_dn6, locals.var_sp_ov_eta_dn7, locals.var_sp_ov_eta_dn8, locals.var_sp_ov_eta_dn9,)
    }
};
        locals.var_sp_ov_eta = assign23870_e23779;
        locals.var_sp_ov_eta_dn4 = assign23870_e23779_d_n4;
        locals.var_sp_ov_eta_dn6 = assign23870_e23779_d_n6;
        locals.var_sp_ov_eta_dn7 = assign23870_e23779_d_n7;
        locals.var_sp_ov_eta_dn8 = assign23870_e23779_d_n8;
        locals.var_sp_ov_eta_dn9 = assign23870_e23779_d_n9;
        locals.var_sp_ov_eta_rv = 0.0;

        let (assign23880_e23800, assign23880_e23800_d_n4, assign23880_e23800_d_n6, assign23880_e23800_d_n7, assign23880_e23800_d_n8, assign23880_e23800_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign23880_e23788: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign23880_e23791: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign23880_e23792: f64 = (assign23880_e23788 * assign23880_e23791);
        let assign23880_e23796: f64 = (locals.var_sp_ov_eta + 1.0);
        let assign23880_e23797: f64 = (locals.var_gov2 * assign23880_e23796);
        let assign23880_e23798: f64 = (assign23880_e23792 + assign23880_e23797);
        (assign23880_e23798, ((((locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4) * assign23880_e23791) + (assign23880_e23788 * (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4))) + ((locals.var_gov2_dn4 * assign23880_e23796) + (locals.var_gov2 * locals.var_sp_ov_eta_dn4))), ((((locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6) * assign23880_e23791) + (assign23880_e23788 * (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6))) + ((locals.var_gov2_dn6 * assign23880_e23796) + (locals.var_gov2 * locals.var_sp_ov_eta_dn6))), ((((locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7) * assign23880_e23791) + (assign23880_e23788 * (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7))) + ((locals.var_gov2_dn7 * assign23880_e23796) + (locals.var_gov2 * locals.var_sp_ov_eta_dn7))), ((((locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8) * assign23880_e23791) + (assign23880_e23788 * (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8))) + ((locals.var_gov2_dn8 * assign23880_e23796) + (locals.var_gov2 * locals.var_sp_ov_eta_dn8))), ((((locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9) * assign23880_e23791) + (assign23880_e23788 * (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9))) + ((locals.var_gov2_dn9 * assign23880_e23796) + (locals.var_gov2 * locals.var_sp_ov_eta_dn9))),)
    } else {
        (locals.var_sp_ov_a, locals.var_sp_ov_a_dn4, locals.var_sp_ov_a_dn6, locals.var_sp_ov_a_dn7, locals.var_sp_ov_a_dn8, locals.var_sp_ov_a_dn9,)
    }
};
        locals.var_sp_ov_a = assign23880_e23800;
        locals.var_sp_ov_a_dn4 = assign23880_e23800_d_n4;
        locals.var_sp_ov_a_dn6 = assign23880_e23800_d_n6;
        locals.var_sp_ov_a_dn7 = assign23880_e23800_d_n7;
        locals.var_sp_ov_a_dn8 = assign23880_e23800_d_n8;
        locals.var_sp_ov_a_dn9 = assign23880_e23800_d_n9;
        locals.var_sp_ov_a_rv = 0.0;

        let (assign23890_e23815, assign23890_e23815_d_n4, assign23890_e23815_d_n6, assign23890_e23815_d_n7, assign23890_e23815_d_n8, assign23890_e23815_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign23890_e23810: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign23890_e23811: f64 = (2.0 * assign23890_e23810);
        let assign23890_e23813: f64 = (assign23890_e23811 - locals.var_gov2);
        (assign23890_e23813, ((2.0 * (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4)) - locals.var_gov2_dn4), ((2.0 * (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6)) - locals.var_gov2_dn6), ((2.0 * (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7)) - locals.var_gov2_dn7), ((2.0 * (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8)) - locals.var_gov2_dn8), ((2.0 * (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9)) - locals.var_gov2_dn9),)
    } else {
        (locals.var_sp_ov_c, locals.var_sp_ov_c_dn4, locals.var_sp_ov_c_dn6, locals.var_sp_ov_c_dn7, locals.var_sp_ov_c_dn8, locals.var_sp_ov_c_dn9,)
    }
};
        locals.var_sp_ov_c = assign23890_e23815;
        locals.var_sp_ov_c_dn4 = assign23890_e23815_d_n4;
        locals.var_sp_ov_c_dn6 = assign23890_e23815_d_n6;
        locals.var_sp_ov_c_dn7 = assign23890_e23815_d_n7;
        locals.var_sp_ov_c_dn8 = assign23890_e23815_d_n8;
        locals.var_sp_ov_c_dn9 = assign23890_e23815_d_n9;
        locals.var_sp_ov_c_rv = 0.0;

        let (assign23900_e23829, assign23900_e23829_d_n4, assign23900_e23829_d_n6, assign23900_e23829_d_n7, assign23900_e23829_d_n8, assign23900_e23829_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign23900_e23824: f64 = (locals.var_sp_ov_a / locals.var_gov2);
        let assign23900_e23825: f64 = (assign23900_e23824).ln();
        let assign23900_e23827: f64 = (assign23900_e23825 - locals.var_sp_ov_eta);
        (assign23900_e23827, (((((locals.var_sp_ov_a_dn4 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn4)) / (locals.var_gov2 * locals.var_gov2)) / assign23900_e23824) - locals.var_sp_ov_eta_dn4), (((((locals.var_sp_ov_a_dn6 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn6)) / (locals.var_gov2 * locals.var_gov2)) / assign23900_e23824) - locals.var_sp_ov_eta_dn6), (((((locals.var_sp_ov_a_dn7 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn7)) / (locals.var_gov2 * locals.var_gov2)) / assign23900_e23824) - locals.var_sp_ov_eta_dn7), (((((locals.var_sp_ov_a_dn8 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn8)) / (locals.var_gov2 * locals.var_gov2)) / assign23900_e23824) - locals.var_sp_ov_eta_dn8), (((((locals.var_sp_ov_a_dn9 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn9)) / (locals.var_gov2 * locals.var_gov2)) / assign23900_e23824) - locals.var_sp_ov_eta_dn9),)
    } else {
        (locals.var_sp_ov_tau, locals.var_sp_ov_tau_dn4, locals.var_sp_ov_tau_dn6, locals.var_sp_ov_tau_dn7, locals.var_sp_ov_tau_dn8, locals.var_sp_ov_tau_dn9,)
    }
};
        locals.var_sp_ov_tau = assign23900_e23829;
        locals.var_sp_ov_tau_dn4 = assign23900_e23829_d_n4;
        locals.var_sp_ov_tau_dn6 = assign23900_e23829_d_n6;
        locals.var_sp_ov_tau_dn7 = assign23900_e23829_d_n7;
        locals.var_sp_ov_tau_dn8 = assign23900_e23829_d_n8;
        locals.var_sp_ov_tau_dn9 = assign23900_e23829_d_n9;
        locals.var_sp_ov_tau_rv = 0.0;

        let (assign23910_e23840, assign23910_e23840_d_n4, assign23910_e23840_d_n6, assign23910_e23840_d_n7, assign23910_e23840_d_n8, assign23910_e23840_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign23910_e23838: f64 = (locals.var_sp_ov_a + locals.var_sp_ov_c);
        (assign23910_e23838, (locals.var_sp_ov_a_dn4 + locals.var_sp_ov_c_dn4), (locals.var_sp_ov_a_dn6 + locals.var_sp_ov_c_dn6), (locals.var_sp_ov_a_dn7 + locals.var_sp_ov_c_dn7), (locals.var_sp_ov_a_dn8 + locals.var_sp_ov_c_dn8), (locals.var_sp_ov_a_dn9 + locals.var_sp_ov_c_dn9),)
    } else {
        (locals.var_sp_ov_nu, locals.var_sp_ov_nu_dn4, locals.var_sp_ov_nu_dn6, locals.var_sp_ov_nu_dn7, locals.var_sp_ov_nu_dn8, locals.var_sp_ov_nu_dn9,)
    }
};
        locals.var_sp_ov_nu = assign23910_e23840;
        locals.var_sp_ov_nu_dn4 = assign23910_e23840_d_n4;
        locals.var_sp_ov_nu_dn6 = assign23910_e23840_d_n6;
        locals.var_sp_ov_nu_dn7 = assign23910_e23840_d_n7;
        locals.var_sp_ov_nu_dn8 = assign23910_e23840_d_n8;
        locals.var_sp_ov_nu_dn9 = assign23910_e23840_d_n9;
        locals.var_sp_ov_nu_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_65(
        locals: &mut StampLocals,
    ) {
        let (assign23920_e23861, assign23920_e23861_d_n4, assign23920_e23861_d_n6, assign23920_e23861_d_n7, assign23920_e23861_d_n8, assign23920_e23861_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign23920_e23849: f64 = (locals.var_sp_ov_nu * locals.var_sp_ov_nu);
        let assign23920_e23853: f64 = (0.5 * locals.var_sp_ov_c);
        let assign23920_e23855: f64 = (assign23920_e23853 * locals.var_sp_ov_c);
        let assign23920_e23857: f64 = (assign23920_e23855 - locals.var_sp_ov_a);
        let assign23920_e23858: f64 = (locals.var_sp_ov_tau * assign23920_e23857);
        let assign23920_e23859: f64 = (assign23920_e23849 + assign23920_e23858);
        (assign23920_e23859, (((locals.var_sp_ov_nu_dn4 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn4)) + ((locals.var_sp_ov_tau_dn4 * assign23920_e23857) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn4) * locals.var_sp_ov_c) + (assign23920_e23853 * locals.var_sp_ov_c_dn4)) - locals.var_sp_ov_a_dn4)))), (((locals.var_sp_ov_nu_dn6 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn6)) + ((locals.var_sp_ov_tau_dn6 * assign23920_e23857) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn6) * locals.var_sp_ov_c) + (assign23920_e23853 * locals.var_sp_ov_c_dn6)) - locals.var_sp_ov_a_dn6)))), (((locals.var_sp_ov_nu_dn7 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn7)) + ((locals.var_sp_ov_tau_dn7 * assign23920_e23857) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn7) * locals.var_sp_ov_c) + (assign23920_e23853 * locals.var_sp_ov_c_dn7)) - locals.var_sp_ov_a_dn7)))), (((locals.var_sp_ov_nu_dn8 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn8)) + ((locals.var_sp_ov_tau_dn8 * assign23920_e23857) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn8) * locals.var_sp_ov_c) + (assign23920_e23853 * locals.var_sp_ov_c_dn8)) - locals.var_sp_ov_a_dn8)))), (((locals.var_sp_ov_nu_dn9 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn9)) + ((locals.var_sp_ov_tau_dn9 * assign23920_e23857) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn9) * locals.var_sp_ov_c) + (assign23920_e23853 * locals.var_sp_ov_c_dn9)) - locals.var_sp_ov_a_dn9)))),)
    } else {
        (locals.var_sp_ov_mutau, locals.var_sp_ov_mutau_dn4, locals.var_sp_ov_mutau_dn6, locals.var_sp_ov_mutau_dn7, locals.var_sp_ov_mutau_dn8, locals.var_sp_ov_mutau_dn9,)
    }
};
        locals.var_sp_ov_mutau = assign23920_e23861;
        locals.var_sp_ov_mutau_dn4 = assign23920_e23861_d_n4;
        locals.var_sp_ov_mutau_dn6 = assign23920_e23861_d_n6;
        locals.var_sp_ov_mutau_dn7 = assign23920_e23861_d_n7;
        locals.var_sp_ov_mutau_dn8 = assign23920_e23861_d_n8;
        locals.var_sp_ov_mutau_dn9 = assign23920_e23861_d_n9;
        locals.var_sp_ov_mutau_rv = 0.0;

        let (assign23930_e23888, assign23930_e23888_d_n4, assign23930_e23888_d_n6, assign23930_e23888_d_n7, assign23930_e23888_d_n8, assign23930_e23888_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign23930_e23871: f64 = (locals.var_sp_ov_nu / locals.var_sp_ov_mutau);
        let assign23930_e23873: f64 = (assign23930_e23871 * locals.var_sp_ov_tau);
        let assign23930_e23875: f64 = (assign23930_e23873 * locals.var_sp_ov_tau);
        let assign23930_e23877: f64 = (assign23930_e23875 * locals.var_sp_ov_c);
        let assign23930_e23880: f64 = (locals.var_sp_ov_c * locals.var_sp_ov_c);
        let assign23930_e23882: f64 = (assign23930_e23880 * 0.3333333333333);
        let assign23930_e23884: f64 = (assign23930_e23882 - locals.var_sp_ov_a);
        let assign23930_e23885: f64 = (assign23930_e23877 * assign23930_e23884);
        let assign23930_e23886: f64 = (locals.var_sp_ov_mutau + assign23930_e23885);
        (assign23930_e23886, (locals.var_sp_ov_mutau_dn4 + (((((((((((locals.var_sp_ov_nu_dn4 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn4)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign23930_e23871 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_tau) + (assign23930_e23873 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_c) + (assign23930_e23875 * locals.var_sp_ov_c_dn4)) * assign23930_e23884) + (assign23930_e23877 * ((((locals.var_sp_ov_c_dn4 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn4)) * 0.3333333333333) - locals.var_sp_ov_a_dn4)))), (locals.var_sp_ov_mutau_dn6 + (((((((((((locals.var_sp_ov_nu_dn6 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn6)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign23930_e23871 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_tau) + (assign23930_e23873 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_c) + (assign23930_e23875 * locals.var_sp_ov_c_dn6)) * assign23930_e23884) + (assign23930_e23877 * ((((locals.var_sp_ov_c_dn6 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn6)) * 0.3333333333333) - locals.var_sp_ov_a_dn6)))), (locals.var_sp_ov_mutau_dn7 + (((((((((((locals.var_sp_ov_nu_dn7 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn7)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign23930_e23871 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_tau) + (assign23930_e23873 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_c) + (assign23930_e23875 * locals.var_sp_ov_c_dn7)) * assign23930_e23884) + (assign23930_e23877 * ((((locals.var_sp_ov_c_dn7 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn7)) * 0.3333333333333) - locals.var_sp_ov_a_dn7)))), (locals.var_sp_ov_mutau_dn8 + (((((((((((locals.var_sp_ov_nu_dn8 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn8)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign23930_e23871 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_tau) + (assign23930_e23873 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_c) + (assign23930_e23875 * locals.var_sp_ov_c_dn8)) * assign23930_e23884) + (assign23930_e23877 * ((((locals.var_sp_ov_c_dn8 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn8)) * 0.3333333333333) - locals.var_sp_ov_a_dn8)))), (locals.var_sp_ov_mutau_dn9 + (((((((((((locals.var_sp_ov_nu_dn9 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn9)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign23930_e23871 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_tau) + (assign23930_e23873 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_c) + (assign23930_e23875 * locals.var_sp_ov_c_dn9)) * assign23930_e23884) + (assign23930_e23877 * ((((locals.var_sp_ov_c_dn9 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn9)) * 0.3333333333333) - locals.var_sp_ov_a_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign23930_e23888;
        locals.var_sp_ov_temp_dn4 = assign23930_e23888_d_n4;
        locals.var_sp_ov_temp_dn6 = assign23930_e23888_d_n6;
        locals.var_sp_ov_temp_dn7 = assign23930_e23888_d_n7;
        locals.var_sp_ov_temp_dn8 = assign23930_e23888_d_n8;
        locals.var_sp_ov_temp_dn9 = assign23930_e23888_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign23940_e23905, assign23940_e23905_d_n4, assign23940_e23905_d_n6, assign23940_e23905_d_n7, assign23940_e23905_d_n8, assign23940_e23905_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign23940_e23898: f64 = (locals.var_sp_ov_a * locals.var_sp_ov_nu);
        let assign23940_e23900: f64 = (assign23940_e23898 * locals.var_sp_ov_tau);
        let assign23940_e23902: f64 = (assign23940_e23900 / locals.var_sp_ov_temp);
        let assign23940_e23903: f64 = (locals.var_sp_ov_eta + assign23940_e23902);
        (assign23940_e23903, (locals.var_sp_ov_eta_dn4 + (((((((locals.var_sp_ov_a_dn4 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn4)) * locals.var_sp_ov_tau) + (assign23940_e23898 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_temp) - (assign23940_e23900 * locals.var_sp_ov_temp_dn4)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn6 + (((((((locals.var_sp_ov_a_dn6 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn6)) * locals.var_sp_ov_tau) + (assign23940_e23898 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_temp) - (assign23940_e23900 * locals.var_sp_ov_temp_dn6)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn7 + (((((((locals.var_sp_ov_a_dn7 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn7)) * locals.var_sp_ov_tau) + (assign23940_e23898 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_temp) - (assign23940_e23900 * locals.var_sp_ov_temp_dn7)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn8 + (((((((locals.var_sp_ov_a_dn8 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn8)) * locals.var_sp_ov_tau) + (assign23940_e23898 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_temp) - (assign23940_e23900 * locals.var_sp_ov_temp_dn8)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn9 + (((((((locals.var_sp_ov_a_dn9 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn9)) * locals.var_sp_ov_tau) + (assign23940_e23898 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_temp) - (assign23940_e23900 * locals.var_sp_ov_temp_dn9)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))),)
    } else {
        (locals.var_sp_ov_y0, locals.var_sp_ov_y0_dn4, locals.var_sp_ov_y0_dn6, locals.var_sp_ov_y0_dn7, locals.var_sp_ov_y0_dn8, locals.var_sp_ov_y0_dn9,)
    }
};
        locals.var_sp_ov_y0 = assign23940_e23905;
        locals.var_sp_ov_y0_dn4 = assign23940_e23905_d_n4;
        locals.var_sp_ov_y0_dn6 = assign23940_e23905_d_n6;
        locals.var_sp_ov_y0_dn7 = assign23940_e23905_d_n7;
        locals.var_sp_ov_y0_dn8 = assign23940_e23905_d_n8;
        locals.var_sp_ov_y0_dn9 = assign23940_e23905_d_n9;
        locals.var_sp_ov_y0_rv = 0.0;

        let assign23950_e23907: f64 = (locals.var_sp_ov_y0).abs();
        let assign23950_e23909: f64 = if assign23950_e23907 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard697 = assign23950_e23909;
        locals.var_guard697_rv = 0.0;

        let (assign23960_e23921, assign23960_e23921_d_n4, assign23960_e23921_d_n6, assign23960_e23921_d_n7, assign23960_e23921_d_n8, assign23960_e23921_d_n9,) = {
    if ((((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) && (locals.var_guard697 != 0.0)) {
        let assign23960_e23919: f64 = (locals.var_sp_ov_y0).exp();
        (assign23960_e23919, (assign23960_e23919 * locals.var_sp_ov_y0_dn4), (assign23960_e23919 * locals.var_sp_ov_y0_dn6), (assign23960_e23919 * locals.var_sp_ov_y0_dn7), (assign23960_e23919 * locals.var_sp_ov_y0_dn8), (assign23960_e23919 * locals.var_sp_ov_y0_dn9),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign23960_e23921;
        locals.var_sp_ov_d0_dn4 = assign23960_e23921_d_n4;
        locals.var_sp_ov_d0_dn6 = assign23960_e23921_d_n6;
        locals.var_sp_ov_d0_dn7 = assign23960_e23921_d_n7;
        locals.var_sp_ov_d0_dn8 = assign23960_e23921_d_n8;
        locals.var_sp_ov_d0_dn9 = assign23960_e23921_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let assign23970_e23924: f64 = (-80.0);
        let assign23970_e23925: f64 = if locals.var_sp_ov_y0 < assign23970_e23924 { 1.0 } else { 0.0 };
        locals.var_guard698 = assign23970_e23925;
        locals.var_guard698_rv = 0.0;

        let (assign23980_e23964, assign23980_e23964_d_n4, assign23980_e23964_d_n6, assign23980_e23964_d_n7, assign23980_e23964_d_n8, assign23980_e23964_d_n9,) = {
    if (((((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) && (locals.var_guard697 == 0.0)) && (locals.var_guard698 != 0.0)) {
        let assign23980_e23940: f64 = (-locals.var_sp_ov_y0);
        let assign23980_e23942: f64 = (assign23980_e23940 - 80.0);
        let assign23980_e23946: f64 = (-locals.var_sp_ov_y0);
        let assign23980_e23948: f64 = (assign23980_e23946 - 80.0);
        let assign23980_e23949: f64 = (0.5 * assign23980_e23948);
        let assign23980_e23952: f64 = (-locals.var_sp_ov_y0);
        let assign23980_e23954: f64 = (assign23980_e23952 - 80.0);
        let assign23980_e23956: f64 = (assign23980_e23954 * 0.3333333333333);
        let assign23980_e23957: f64 = (1.0 + assign23980_e23956);
        let assign23980_e23958: f64 = (assign23980_e23949 * assign23980_e23957);
        let assign23980_e23959: f64 = (1.0 + assign23980_e23958);
        let assign23980_e23960: f64 = (assign23980_e23942 * assign23980_e23959);
        let assign23980_e23961: f64 = (1.0 + assign23980_e23960);
        let assign23980_e23962: f64 = (1.80485e-35 / assign23980_e23961);
        (assign23980_e23962, (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn4) * assign23980_e23959) + (assign23980_e23942 * (((0.5 * (-locals.var_sp_ov_y0_dn4)) * assign23980_e23957) + (assign23980_e23949 * ((-locals.var_sp_ov_y0_dn4) * 0.3333333333333)))))) / (assign23980_e23961 * assign23980_e23961))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn6) * assign23980_e23959) + (assign23980_e23942 * (((0.5 * (-locals.var_sp_ov_y0_dn6)) * assign23980_e23957) + (assign23980_e23949 * ((-locals.var_sp_ov_y0_dn6) * 0.3333333333333)))))) / (assign23980_e23961 * assign23980_e23961))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn7) * assign23980_e23959) + (assign23980_e23942 * (((0.5 * (-locals.var_sp_ov_y0_dn7)) * assign23980_e23957) + (assign23980_e23949 * ((-locals.var_sp_ov_y0_dn7) * 0.3333333333333)))))) / (assign23980_e23961 * assign23980_e23961))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn8) * assign23980_e23959) + (assign23980_e23942 * (((0.5 * (-locals.var_sp_ov_y0_dn8)) * assign23980_e23957) + (assign23980_e23949 * ((-locals.var_sp_ov_y0_dn8) * 0.3333333333333)))))) / (assign23980_e23961 * assign23980_e23961))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn9) * assign23980_e23959) + (assign23980_e23942 * (((0.5 * (-locals.var_sp_ov_y0_dn9)) * assign23980_e23957) + (assign23980_e23949 * ((-locals.var_sp_ov_y0_dn9) * 0.3333333333333)))))) / (assign23980_e23961 * assign23980_e23961))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign23980_e23964;
        locals.var_sp_ov_d0_dn4 = assign23980_e23964_d_n4;
        locals.var_sp_ov_d0_dn6 = assign23980_e23964_d_n6;
        locals.var_sp_ov_d0_dn7 = assign23980_e23964_d_n7;
        locals.var_sp_ov_d0_dn8 = assign23980_e23964_d_n8;
        locals.var_sp_ov_d0_dn9 = assign23980_e23964_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign23990_e24001, assign23990_e24001_d_n4, assign23990_e24001_d_n6, assign23990_e24001_d_n7, assign23990_e24001_d_n8, assign23990_e24001_d_n9,) = {
    if (((((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) && (locals.var_guard697 == 0.0)) && (locals.var_guard698 == 0.0)) {
        let assign23990_e23981: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign23990_e23986: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign23990_e23987: f64 = (0.5 * assign23990_e23986);
        let assign23990_e23991: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign23990_e23993: f64 = (assign23990_e23991 * 0.3333333333333);
        let assign23990_e23994: f64 = (1.0 + assign23990_e23993);
        let assign23990_e23995: f64 = (assign23990_e23987 * assign23990_e23994);
        let assign23990_e23996: f64 = (1.0 + assign23990_e23995);
        let assign23990_e23997: f64 = (assign23990_e23981 * assign23990_e23996);
        let assign23990_e23998: f64 = (1.0 + assign23990_e23997);
        let assign23990_e23999: f64 = (5.54062e34 * assign23990_e23998);
        (assign23990_e23999, (5.54062e34 * ((locals.var_sp_ov_y0_dn4 * assign23990_e23996) + (assign23990_e23981 * (((0.5 * locals.var_sp_ov_y0_dn4) * assign23990_e23994) + (assign23990_e23987 * (locals.var_sp_ov_y0_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn6 * assign23990_e23996) + (assign23990_e23981 * (((0.5 * locals.var_sp_ov_y0_dn6) * assign23990_e23994) + (assign23990_e23987 * (locals.var_sp_ov_y0_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn7 * assign23990_e23996) + (assign23990_e23981 * (((0.5 * locals.var_sp_ov_y0_dn7) * assign23990_e23994) + (assign23990_e23987 * (locals.var_sp_ov_y0_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn8 * assign23990_e23996) + (assign23990_e23981 * (((0.5 * locals.var_sp_ov_y0_dn8) * assign23990_e23994) + (assign23990_e23987 * (locals.var_sp_ov_y0_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn9 * assign23990_e23996) + (assign23990_e23981 * (((0.5 * locals.var_sp_ov_y0_dn9) * assign23990_e23994) + (assign23990_e23987 * (locals.var_sp_ov_y0_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign23990_e24001;
        locals.var_sp_ov_d0_dn4 = assign23990_e24001_d_n4;
        locals.var_sp_ov_d0_dn6 = assign23990_e24001_d_n6;
        locals.var_sp_ov_d0_dn7 = assign23990_e24001_d_n7;
        locals.var_sp_ov_d0_dn8 = assign23990_e24001_d_n8;
        locals.var_sp_ov_d0_dn9 = assign23990_e24001_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign24000_e24012, assign24000_e24012_d_n4, assign24000_e24012_d_n6, assign24000_e24012_d_n7, assign24000_e24012_d_n8, assign24000_e24012_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign24000_e24010: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_y0);
        (assign24000_e24010, (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_y0_dn4), (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_y0_dn6), (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_y0_dn7), (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_y0_dn8), (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_y0_dn9),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24000_e24012;
        locals.var_sp_ov_temp_dn4 = assign24000_e24012_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24000_e24012_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24000_e24012_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24000_e24012_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24000_e24012_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign24010_e24029, assign24010_e24029_d_n4, assign24010_e24029_d_n6, assign24010_e24029_d_n7, assign24010_e24029_d_n8, assign24010_e24029_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign24010_e24021: f64 = (2.0 * locals.var_sp_ov_temp);
        let assign24010_e24025: f64 = (locals.var_sp_ov_d0 - 1.0);
        let assign24010_e24026: f64 = (locals.var_gov2 * assign24010_e24025);
        let assign24010_e24027: f64 = (assign24010_e24021 + assign24010_e24026);
        (assign24010_e24027, ((2.0 * locals.var_sp_ov_temp_dn4) + ((locals.var_gov2_dn4 * assign24010_e24025) + (locals.var_gov2 * locals.var_sp_ov_d0_dn4))), ((2.0 * locals.var_sp_ov_temp_dn6) + ((locals.var_gov2_dn6 * assign24010_e24025) + (locals.var_gov2 * locals.var_sp_ov_d0_dn6))), ((2.0 * locals.var_sp_ov_temp_dn7) + ((locals.var_gov2_dn7 * assign24010_e24025) + (locals.var_gov2 * locals.var_sp_ov_d0_dn7))), ((2.0 * locals.var_sp_ov_temp_dn8) + ((locals.var_gov2_dn8 * assign24010_e24025) + (locals.var_gov2 * locals.var_sp_ov_d0_dn8))), ((2.0 * locals.var_sp_ov_temp_dn9) + ((locals.var_gov2_dn9 * assign24010_e24025) + (locals.var_gov2 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_p, locals.var_sp_ov_p_dn4, locals.var_sp_ov_p_dn6, locals.var_sp_ov_p_dn7, locals.var_sp_ov_p_dn8, locals.var_sp_ov_p_dn9,)
    }
};
        locals.var_sp_ov_p = assign24010_e24029;
        locals.var_sp_ov_p_dn4 = assign24010_e24029_d_n4;
        locals.var_sp_ov_p_dn6 = assign24010_e24029_d_n6;
        locals.var_sp_ov_p_dn7 = assign24010_e24029_d_n7;
        locals.var_sp_ov_p_dn8 = assign24010_e24029_d_n8;
        locals.var_sp_ov_p_dn9 = assign24010_e24029_d_n9;
        locals.var_sp_ov_p_rv = 0.0;

        let (assign24020_e24048, assign24020_e24048_d_n4, assign24020_e24048_d_n6, assign24020_e24048_d_n7, assign24020_e24048_d_n8, assign24020_e24048_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign24020_e24038: f64 = (locals.var_sp_ov_temp * locals.var_sp_ov_temp);
        let assign24020_e24042: f64 = (locals.var_sp_ov_y0 + 1.0);
        let assign24020_e24044: f64 = (assign24020_e24042 - locals.var_sp_ov_d0);
        let assign24020_e24045: f64 = (locals.var_gov2 * assign24020_e24044);
        let assign24020_e24046: f64 = (assign24020_e24038 + assign24020_e24045);
        (assign24020_e24046, (((locals.var_sp_ov_temp_dn4 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn4)) + ((locals.var_gov2_dn4 * assign24020_e24044) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn4 - locals.var_sp_ov_d0_dn4)))), (((locals.var_sp_ov_temp_dn6 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn6)) + ((locals.var_gov2_dn6 * assign24020_e24044) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn6 - locals.var_sp_ov_d0_dn6)))), (((locals.var_sp_ov_temp_dn7 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn7)) + ((locals.var_gov2_dn7 * assign24020_e24044) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn7 - locals.var_sp_ov_d0_dn7)))), (((locals.var_sp_ov_temp_dn8 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn8)) + ((locals.var_gov2_dn8 * assign24020_e24044) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn8 - locals.var_sp_ov_d0_dn8)))), (((locals.var_sp_ov_temp_dn9 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn9)) + ((locals.var_gov2_dn9 * assign24020_e24044) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn9 - locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_q, locals.var_sp_ov_q_dn4, locals.var_sp_ov_q_dn6, locals.var_sp_ov_q_dn7, locals.var_sp_ov_q_dn8, locals.var_sp_ov_q_dn9,)
    }
};
        locals.var_sp_ov_q = assign24020_e24048;
        locals.var_sp_ov_q_dn4 = assign24020_e24048_d_n4;
        locals.var_sp_ov_q_dn6 = assign24020_e24048_d_n6;
        locals.var_sp_ov_q_dn7 = assign24020_e24048_d_n7;
        locals.var_sp_ov_q_dn8 = assign24020_e24048_d_n8;
        locals.var_sp_ov_q_dn9 = assign24020_e24048_d_n9;
        locals.var_sp_ov_q_rv = 0.0;

        let (assign24030_e24063, assign24030_e24063_d_n4, assign24030_e24063_d_n6, assign24030_e24063_d_n7, assign24030_e24063_d_n8, assign24030_e24063_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign24030_e24058: f64 = (locals.var_gov2 * 0.5);
        let assign24030_e24060: f64 = (assign24030_e24058 * locals.var_sp_ov_d0);
        let assign24030_e24061: f64 = (1.0 - assign24030_e24060);
        (assign24030_e24061, (-(((locals.var_gov2_dn4 * 0.5) * locals.var_sp_ov_d0) + (assign24030_e24058 * locals.var_sp_ov_d0_dn4))), (-(((locals.var_gov2_dn6 * 0.5) * locals.var_sp_ov_d0) + (assign24030_e24058 * locals.var_sp_ov_d0_dn6))), (-(((locals.var_gov2_dn7 * 0.5) * locals.var_sp_ov_d0) + (assign24030_e24058 * locals.var_sp_ov_d0_dn7))), (-(((locals.var_gov2_dn8 * 0.5) * locals.var_sp_ov_d0) + (assign24030_e24058 * locals.var_sp_ov_d0_dn8))), (-(((locals.var_gov2_dn9 * 0.5) * locals.var_sp_ov_d0) + (assign24030_e24058 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_xi, locals.var_sp_ov_xi_dn4, locals.var_sp_ov_xi_dn6, locals.var_sp_ov_xi_dn7, locals.var_sp_ov_xi_dn8, locals.var_sp_ov_xi_dn9,)
    }
};
        locals.var_sp_ov_xi = assign24030_e24063;
        locals.var_sp_ov_xi_dn4 = assign24030_e24063_d_n4;
        locals.var_sp_ov_xi_dn6 = assign24030_e24063_d_n6;
        locals.var_sp_ov_xi_dn7 = assign24030_e24063_d_n7;
        locals.var_sp_ov_xi_dn8 = assign24030_e24063_d_n8;
        locals.var_sp_ov_xi_dn9 = assign24030_e24063_d_n9;
        locals.var_sp_ov_xi_rv = 0.0;

        let (assign24040_e24080, assign24040_e24080_d_n4, assign24040_e24080_d_n6, assign24040_e24080_d_n7, assign24040_e24080_d_n8, assign24040_e24080_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign24040_e24072: f64 = (locals.var_sp_ov_p * locals.var_sp_ov_p);
        let assign24040_e24076: f64 = (locals.var_sp_ov_xi * locals.var_sp_ov_q);
        let assign24040_e24077: f64 = (4.0 * assign24040_e24076);
        let assign24040_e24078: f64 = (assign24040_e24072 - assign24040_e24077);
        (assign24040_e24078, (((locals.var_sp_ov_p_dn4 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn4)) - (4.0 * ((locals.var_sp_ov_xi_dn4 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn4)))), (((locals.var_sp_ov_p_dn6 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn6)) - (4.0 * ((locals.var_sp_ov_xi_dn6 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn6)))), (((locals.var_sp_ov_p_dn7 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn7)) - (4.0 * ((locals.var_sp_ov_xi_dn7 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn7)))), (((locals.var_sp_ov_p_dn8 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn8)) - (4.0 * ((locals.var_sp_ov_xi_dn8 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn8)))), (((locals.var_sp_ov_p_dn9 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn9)) - (4.0 * ((locals.var_sp_ov_xi_dn9 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24040_e24080;
        locals.var_sp_ov_temp_dn4 = assign24040_e24080_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24040_e24080_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24040_e24080_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24040_e24080_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24040_e24080_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign24050_e24096, assign24050_e24096_d_n4, assign24050_e24096_d_n6, assign24050_e24096_d_n7, assign24050_e24096_d_n8, assign24050_e24096_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign24050_e24089: f64 = (2.0 * locals.var_sp_ov_q);
        let assign24050_e24092: f64 = (locals.var_sp_ov_temp).sqrt();
        let assign24050_e24093: f64 = (locals.var_sp_ov_p + assign24050_e24092);
        let assign24050_e24094: f64 = (assign24050_e24089 / assign24050_e24093);
        (assign24050_e24094, ((((2.0 * locals.var_sp_ov_q_dn4) * assign24050_e24093) - (assign24050_e24089 * (locals.var_sp_ov_p_dn4 + (locals.var_sp_ov_temp_dn4 / (2.0 * assign24050_e24092))))) / (assign24050_e24093 * assign24050_e24093)), ((((2.0 * locals.var_sp_ov_q_dn6) * assign24050_e24093) - (assign24050_e24089 * (locals.var_sp_ov_p_dn6 + (locals.var_sp_ov_temp_dn6 / (2.0 * assign24050_e24092))))) / (assign24050_e24093 * assign24050_e24093)), ((((2.0 * locals.var_sp_ov_q_dn7) * assign24050_e24093) - (assign24050_e24089 * (locals.var_sp_ov_p_dn7 + (locals.var_sp_ov_temp_dn7 / (2.0 * assign24050_e24092))))) / (assign24050_e24093 * assign24050_e24093)), ((((2.0 * locals.var_sp_ov_q_dn8) * assign24050_e24093) - (assign24050_e24089 * (locals.var_sp_ov_p_dn8 + (locals.var_sp_ov_temp_dn8 / (2.0 * assign24050_e24092))))) / (assign24050_e24093 * assign24050_e24093)), ((((2.0 * locals.var_sp_ov_q_dn9) * assign24050_e24093) - (assign24050_e24089 * (locals.var_sp_ov_p_dn9 + (locals.var_sp_ov_temp_dn9 / (2.0 * assign24050_e24092))))) / (assign24050_e24093 * assign24050_e24093)),)
    } else {
        (locals.var_sp_ov_w, locals.var_sp_ov_w_dn4, locals.var_sp_ov_w_dn6, locals.var_sp_ov_w_dn7, locals.var_sp_ov_w_dn8, locals.var_sp_ov_w_dn9,)
    }
};
        locals.var_sp_ov_w = assign24050_e24096;
        locals.var_sp_ov_w_dn4 = assign24050_e24096_d_n4;
        locals.var_sp_ov_w_dn6 = assign24050_e24096_d_n6;
        locals.var_sp_ov_w_dn7 = assign24050_e24096_d_n7;
        locals.var_sp_ov_w_dn8 = assign24050_e24096_d_n8;
        locals.var_sp_ov_w_dn9 = assign24050_e24096_d_n9;
        locals.var_sp_ov_w_rv = 0.0;

        let (assign24060_e24108, assign24060_e24108_d_n4, assign24060_e24108_d_n6, assign24060_e24108_d_n7, assign24060_e24108_d_n8, assign24060_e24108_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 != 0.0)) {
        let assign24060_e24105: f64 = (locals.var_sp_ov_y0 + locals.var_sp_ov_w);
        let assign24060_e24106: f64 = (-assign24060_e24105);
        (assign24060_e24106, (-(locals.var_sp_ov_y0_dn4 + locals.var_sp_ov_w_dn4)), (-(locals.var_sp_ov_y0_dn6 + locals.var_sp_ov_w_dn6)), (-(locals.var_sp_ov_y0_dn7 + locals.var_sp_ov_w_dn7)), (-(locals.var_sp_ov_y0_dn8 + locals.var_sp_ov_w_dn8)), (-(locals.var_sp_ov_y0_dn9 + locals.var_sp_ov_w_dn9)),)
    } else {
        (locals.var_xs_ovcv, locals.var_xs_ovcv_dn4, locals.var_xs_ovcv_dn6, locals.var_xs_ovcv_dn7, locals.var_xs_ovcv_dn8, locals.var_xs_ovcv_dn9,)
    }
};
        locals.var_xs_ovcv = assign24060_e24108;
        locals.var_xs_ovcv_dn4 = assign24060_e24108_d_n4;
        locals.var_xs_ovcv_dn6 = assign24060_e24108_d_n6;
        locals.var_xs_ovcv_dn7 = assign24060_e24108_d_n7;
        locals.var_xs_ovcv_dn8 = assign24060_e24108_d_n8;
        locals.var_xs_ovcv_dn9 = assign24060_e24108_d_n9;
        locals.var_xs_ovcv_rv = 0.0;

        let (assign24070_e24126, assign24070_e24126_d_n4, assign24070_e24126_d_n6, assign24070_e24126_d_n7, assign24070_e24126_d_n8, assign24070_e24126_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24070_e24118: f64 = (locals.var_xi_ov * 1.25);
        let assign24070_e24120: f64 = (assign24070_e24118 * locals.var_inv_xg1);
        let assign24070_e24122: f64 = (assign24070_e24120 - 1.0);
        let assign24070_e24124: f64 = (assign24070_e24122 * locals.var_inv_xg1);
        (assign24070_e24124, (((((locals.var_xi_ov_dn4 * 1.25) * locals.var_inv_xg1) + (assign24070_e24118 * locals.var_inv_xg1_dn4)) * locals.var_inv_xg1) + (assign24070_e24122 * locals.var_inv_xg1_dn4)), (((((locals.var_xi_ov_dn6 * 1.25) * locals.var_inv_xg1) + (assign24070_e24118 * locals.var_inv_xg1_dn6)) * locals.var_inv_xg1) + (assign24070_e24122 * locals.var_inv_xg1_dn6)), (((((locals.var_xi_ov_dn7 * 1.25) * locals.var_inv_xg1) + (assign24070_e24118 * locals.var_inv_xg1_dn7)) * locals.var_inv_xg1) + (assign24070_e24122 * locals.var_inv_xg1_dn7)), (((((locals.var_xi_ov_dn8 * 1.25) * locals.var_inv_xg1) + (assign24070_e24118 * locals.var_inv_xg1_dn8)) * locals.var_inv_xg1) + (assign24070_e24122 * locals.var_inv_xg1_dn8)), (((((locals.var_xi_ov_dn9 * 1.25) * locals.var_inv_xg1) + (assign24070_e24118 * locals.var_inv_xg1_dn9)) * locals.var_inv_xg1) + (assign24070_e24122 * locals.var_inv_xg1_dn9)),)
    } else {
        (locals.var_sp_ov_afac, locals.var_sp_ov_afac_dn4, locals.var_sp_ov_afac_dn6, locals.var_sp_ov_afac_dn7, locals.var_sp_ov_afac_dn8, locals.var_sp_ov_afac_dn9,)
    }
};
        locals.var_sp_ov_afac = assign24070_e24126;
        locals.var_sp_ov_afac_dn4 = assign24070_e24126_d_n4;
        locals.var_sp_ov_afac_dn6 = assign24070_e24126_d_n6;
        locals.var_sp_ov_afac_dn7 = assign24070_e24126_d_n7;
        locals.var_sp_ov_afac_dn8 = assign24070_e24126_d_n8;
        locals.var_sp_ov_afac_dn9 = assign24070_e24126_d_n9;
        locals.var_sp_ov_afac_rv = 0.0;

        let (assign24080_e24144, assign24080_e24144_d_n4, assign24080_e24144_d_n6, assign24080_e24144_d_n7, assign24080_e24144_d_n8, assign24080_e24144_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24080_e24136: f64 = (locals.var_xgs_ovcv * locals.var_inv_xi_ov);
        let assign24080_e24140: f64 = (locals.var_sp_ov_afac * locals.var_xgs_ovcv);
        let assign24080_e24141: f64 = (1.0 + assign24080_e24140);
        let assign24080_e24142: f64 = (assign24080_e24136 * assign24080_e24141);
        (assign24080_e24142, ((((locals.var_xgs_ovcv_dn4 * locals.var_inv_xi_ov) + (locals.var_xgs_ovcv * locals.var_inv_xi_ov_dn4)) * assign24080_e24141) + (assign24080_e24136 * ((locals.var_sp_ov_afac_dn4 * locals.var_xgs_ovcv) + (locals.var_sp_ov_afac * locals.var_xgs_ovcv_dn4)))), ((((locals.var_xgs_ovcv_dn6 * locals.var_inv_xi_ov) + (locals.var_xgs_ovcv * locals.var_inv_xi_ov_dn6)) * assign24080_e24141) + (assign24080_e24136 * ((locals.var_sp_ov_afac_dn6 * locals.var_xgs_ovcv) + (locals.var_sp_ov_afac * locals.var_xgs_ovcv_dn6)))), ((((locals.var_xgs_ovcv_dn7 * locals.var_inv_xi_ov) + (locals.var_xgs_ovcv * locals.var_inv_xi_ov_dn7)) * assign24080_e24141) + (assign24080_e24136 * ((locals.var_sp_ov_afac_dn7 * locals.var_xgs_ovcv) + (locals.var_sp_ov_afac * locals.var_xgs_ovcv_dn7)))), ((((locals.var_xgs_ovcv_dn8 * locals.var_inv_xi_ov) + (locals.var_xgs_ovcv * locals.var_inv_xi_ov_dn8)) * assign24080_e24141) + (assign24080_e24136 * ((locals.var_sp_ov_afac_dn8 * locals.var_xgs_ovcv) + (locals.var_sp_ov_afac * locals.var_xgs_ovcv_dn8)))), ((((locals.var_xgs_ovcv_dn9 * locals.var_inv_xi_ov) + (locals.var_xgs_ovcv * locals.var_inv_xi_ov_dn9)) * assign24080_e24141) + (assign24080_e24136 * ((locals.var_sp_ov_afac_dn9 * locals.var_xgs_ovcv) + (locals.var_sp_ov_afac * locals.var_xgs_ovcv_dn9)))),)
    } else {
        (locals.var_sp_ov_xbar, locals.var_sp_ov_xbar_dn4, locals.var_sp_ov_xbar_dn6, locals.var_sp_ov_xbar_dn7, locals.var_sp_ov_xbar_dn8, locals.var_sp_ov_xbar_dn9,)
    }
};
        locals.var_sp_ov_xbar = assign24080_e24144;
        locals.var_sp_ov_xbar_dn4 = assign24080_e24144_d_n4;
        locals.var_sp_ov_xbar_dn6 = assign24080_e24144_d_n6;
        locals.var_sp_ov_xbar_dn7 = assign24080_e24144_d_n7;
        locals.var_sp_ov_xbar_dn8 = assign24080_e24144_d_n8;
        locals.var_sp_ov_xbar_dn9 = assign24080_e24144_d_n9;
        locals.var_sp_ov_xbar_rv = 0.0;

        let assign24090_e24146: f64 = (-locals.var_sp_ov_xbar);
        let assign24090_e24147: f64 = (assign24090_e24146).abs();
        let assign24090_e24149: f64 = if assign24090_e24147 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard699 = assign24090_e24149;
        locals.var_guard699_rv = 0.0;

        let (assign24100_e24163, assign24100_e24163_d_n4, assign24100_e24163_d_n6, assign24100_e24163_d_n7, assign24100_e24163_d_n8, assign24100_e24163_d_n9,) = {
    if ((((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) && (locals.var_guard699 != 0.0)) {
        let assign24100_e24160: f64 = (-locals.var_sp_ov_xbar);
        let assign24100_e24161: f64 = (assign24100_e24160).exp();
        (assign24100_e24161, (assign24100_e24161 * (-locals.var_sp_ov_xbar_dn4)), (assign24100_e24161 * (-locals.var_sp_ov_xbar_dn6)), (assign24100_e24161 * (-locals.var_sp_ov_xbar_dn7)), (assign24100_e24161 * (-locals.var_sp_ov_xbar_dn8)), (assign24100_e24161 * (-locals.var_sp_ov_xbar_dn9)),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24100_e24163;
        locals.var_sp_ov_temp_dn4 = assign24100_e24163_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24100_e24163_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24100_e24163_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24100_e24163_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24100_e24163_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let assign24110_e24165: f64 = (-locals.var_sp_ov_xbar);
        let assign24110_e24167: f64 = (-80.0);
        let assign24110_e24168: f64 = if assign24110_e24165 < assign24110_e24167 { 1.0 } else { 0.0 };
        locals.var_guard700 = assign24110_e24168;
        locals.var_guard700_rv = 0.0;

        let (assign24120_e24211, assign24120_e24211_d_n4, assign24120_e24211_d_n6, assign24120_e24211_d_n7, assign24120_e24211_d_n8, assign24120_e24211_d_n9,) = {
    if (((((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) && (locals.var_guard699 == 0.0)) && (locals.var_guard700 != 0.0)) {
        let assign24120_e24184: f64 = (-locals.var_sp_ov_xbar);
        let assign24120_e24185: f64 = (-assign24120_e24184);
        let assign24120_e24187: f64 = (assign24120_e24185 - 80.0);
        let assign24120_e24191: f64 = (-locals.var_sp_ov_xbar);
        let assign24120_e24192: f64 = (-assign24120_e24191);
        let assign24120_e24194: f64 = (assign24120_e24192 - 80.0);
        let assign24120_e24195: f64 = (0.5 * assign24120_e24194);
        let assign24120_e24198: f64 = (-locals.var_sp_ov_xbar);
        let assign24120_e24199: f64 = (-assign24120_e24198);
        let assign24120_e24201: f64 = (assign24120_e24199 - 80.0);
        let assign24120_e24203: f64 = (assign24120_e24201 * 0.3333333333333);
        let assign24120_e24204: f64 = (1.0 + assign24120_e24203);
        let assign24120_e24205: f64 = (assign24120_e24195 * assign24120_e24204);
        let assign24120_e24206: f64 = (1.0 + assign24120_e24205);
        let assign24120_e24207: f64 = (assign24120_e24187 * assign24120_e24206);
        let assign24120_e24208: f64 = (1.0 + assign24120_e24207);
        let assign24120_e24209: f64 = (1.80485e-35 / assign24120_e24208);
        (assign24120_e24209, (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn4)) * assign24120_e24206) + (assign24120_e24187 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn4))) * assign24120_e24204) + (assign24120_e24195 * ((-(-locals.var_sp_ov_xbar_dn4)) * 0.3333333333333)))))) / (assign24120_e24208 * assign24120_e24208))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn6)) * assign24120_e24206) + (assign24120_e24187 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn6))) * assign24120_e24204) + (assign24120_e24195 * ((-(-locals.var_sp_ov_xbar_dn6)) * 0.3333333333333)))))) / (assign24120_e24208 * assign24120_e24208))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn7)) * assign24120_e24206) + (assign24120_e24187 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn7))) * assign24120_e24204) + (assign24120_e24195 * ((-(-locals.var_sp_ov_xbar_dn7)) * 0.3333333333333)))))) / (assign24120_e24208 * assign24120_e24208))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn8)) * assign24120_e24206) + (assign24120_e24187 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn8))) * assign24120_e24204) + (assign24120_e24195 * ((-(-locals.var_sp_ov_xbar_dn8)) * 0.3333333333333)))))) / (assign24120_e24208 * assign24120_e24208))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn9)) * assign24120_e24206) + (assign24120_e24187 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn9))) * assign24120_e24204) + (assign24120_e24195 * ((-(-locals.var_sp_ov_xbar_dn9)) * 0.3333333333333)))))) / (assign24120_e24208 * assign24120_e24208))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24120_e24211;
        locals.var_sp_ov_temp_dn4 = assign24120_e24211_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24120_e24211_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24120_e24211_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24120_e24211_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24120_e24211_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign24130_e24252, assign24130_e24252_d_n4, assign24130_e24252_d_n6, assign24130_e24252_d_n7, assign24130_e24252_d_n8, assign24130_e24252_d_n9,) = {
    if (((((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) && (locals.var_guard699 == 0.0)) && (locals.var_guard700 == 0.0)) {
        let assign24130_e24228: f64 = (-locals.var_sp_ov_xbar);
        let assign24130_e24230: f64 = (assign24130_e24228 - 80.0);
        let assign24130_e24234: f64 = (-locals.var_sp_ov_xbar);
        let assign24130_e24236: f64 = (assign24130_e24234 - 80.0);
        let assign24130_e24237: f64 = (0.5 * assign24130_e24236);
        let assign24130_e24240: f64 = (-locals.var_sp_ov_xbar);
        let assign24130_e24242: f64 = (assign24130_e24240 - 80.0);
        let assign24130_e24244: f64 = (assign24130_e24242 * 0.3333333333333);
        let assign24130_e24245: f64 = (1.0 + assign24130_e24244);
        let assign24130_e24246: f64 = (assign24130_e24237 * assign24130_e24245);
        let assign24130_e24247: f64 = (1.0 + assign24130_e24246);
        let assign24130_e24248: f64 = (assign24130_e24230 * assign24130_e24247);
        let assign24130_e24249: f64 = (1.0 + assign24130_e24248);
        let assign24130_e24250: f64 = (5.54062e34 * assign24130_e24249);
        (assign24130_e24250, (5.54062e34 * (((-locals.var_sp_ov_xbar_dn4) * assign24130_e24247) + (assign24130_e24230 * (((0.5 * (-locals.var_sp_ov_xbar_dn4)) * assign24130_e24245) + (assign24130_e24237 * ((-locals.var_sp_ov_xbar_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn6) * assign24130_e24247) + (assign24130_e24230 * (((0.5 * (-locals.var_sp_ov_xbar_dn6)) * assign24130_e24245) + (assign24130_e24237 * ((-locals.var_sp_ov_xbar_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn7) * assign24130_e24247) + (assign24130_e24230 * (((0.5 * (-locals.var_sp_ov_xbar_dn7)) * assign24130_e24245) + (assign24130_e24237 * ((-locals.var_sp_ov_xbar_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn8) * assign24130_e24247) + (assign24130_e24230 * (((0.5 * (-locals.var_sp_ov_xbar_dn8)) * assign24130_e24245) + (assign24130_e24237 * ((-locals.var_sp_ov_xbar_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn9) * assign24130_e24247) + (assign24130_e24230 * (((0.5 * (-locals.var_sp_ov_xbar_dn9)) * assign24130_e24245) + (assign24130_e24237 * ((-locals.var_sp_ov_xbar_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24130_e24252;
        locals.var_sp_ov_temp_dn4 = assign24130_e24252_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24130_e24252_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24130_e24252_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24130_e24252_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24130_e24252_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign24140_e24264, assign24140_e24264_d_n4, assign24140_e24264_d_n6, assign24140_e24264_d_n7, assign24140_e24264_d_n8, assign24140_e24264_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24140_e24262: f64 = (1.0 - locals.var_sp_ov_temp);
        (assign24140_e24262, (-locals.var_sp_ov_temp_dn4), (-locals.var_sp_ov_temp_dn6), (-locals.var_sp_ov_temp_dn7), (-locals.var_sp_ov_temp_dn8), (-locals.var_sp_ov_temp_dn9),)
    } else {
        (locals.var_sp_ov_w, locals.var_sp_ov_w_dn4, locals.var_sp_ov_w_dn6, locals.var_sp_ov_w_dn7, locals.var_sp_ov_w_dn8, locals.var_sp_ov_w_dn9,)
    }
};
        locals.var_sp_ov_w = assign24140_e24264;
        locals.var_sp_ov_w_dn4 = assign24140_e24264_d_n4;
        locals.var_sp_ov_w_dn6 = assign24140_e24264_d_n6;
        locals.var_sp_ov_w_dn7 = assign24140_e24264_d_n7;
        locals.var_sp_ov_w_dn8 = assign24140_e24264_d_n8;
        locals.var_sp_ov_w_dn9 = assign24140_e24264_d_n9;
        locals.var_sp_ov_w_rv = 0.0;

        let (assign24150_e24289, assign24150_e24289_d_n4, assign24150_e24289_d_n6, assign24150_e24289_d_n7, assign24150_e24289_d_n8, assign24150_e24289_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24150_e24275: f64 = (locals.var_gov2 * 0.5);
        let assign24150_e24276: f64 = (locals.var_xgs_ovcv + assign24150_e24275);
        let assign24150_e24281: f64 = (locals.var_gov2 * 0.25);
        let assign24150_e24282: f64 = (locals.var_xgs_ovcv + assign24150_e24281);
        let assign24150_e24284: f64 = (assign24150_e24282 - locals.var_sp_ov_w);
        let assign24150_e24285: f64 = (assign24150_e24284).sqrt();
        let assign24150_e24286: f64 = (locals.var_gov * assign24150_e24285);
        let assign24150_e24287: f64 = (assign24150_e24276 - assign24150_e24286);
        (assign24150_e24287, ((locals.var_xgs_ovcv_dn4 + (locals.var_gov2_dn4 * 0.5)) - ((locals.var_gov_dn4 * assign24150_e24285) + (locals.var_gov * (((locals.var_xgs_ovcv_dn4 + (locals.var_gov2_dn4 * 0.25)) - locals.var_sp_ov_w_dn4) / (2.0 * assign24150_e24285))))), ((locals.var_xgs_ovcv_dn6 + (locals.var_gov2_dn6 * 0.5)) - ((locals.var_gov_dn6 * assign24150_e24285) + (locals.var_gov * (((locals.var_xgs_ovcv_dn6 + (locals.var_gov2_dn6 * 0.25)) - locals.var_sp_ov_w_dn6) / (2.0 * assign24150_e24285))))), ((locals.var_xgs_ovcv_dn7 + (locals.var_gov2_dn7 * 0.5)) - ((locals.var_gov_dn7 * assign24150_e24285) + (locals.var_gov * (((locals.var_xgs_ovcv_dn7 + (locals.var_gov2_dn7 * 0.25)) - locals.var_sp_ov_w_dn7) / (2.0 * assign24150_e24285))))), ((locals.var_xgs_ovcv_dn8 + (locals.var_gov2_dn8 * 0.5)) - ((locals.var_gov_dn8 * assign24150_e24285) + (locals.var_gov * (((locals.var_xgs_ovcv_dn8 + (locals.var_gov2_dn8 * 0.25)) - locals.var_sp_ov_w_dn8) / (2.0 * assign24150_e24285))))), ((locals.var_xgs_ovcv_dn9 + (locals.var_gov2_dn9 * 0.5)) - ((locals.var_gov_dn9 * assign24150_e24285) + (locals.var_gov * (((locals.var_xgs_ovcv_dn9 + (locals.var_gov2_dn9 * 0.25)) - locals.var_sp_ov_w_dn9) / (2.0 * assign24150_e24285))))),)
    } else {
        (locals.var_sp_ov_x0, locals.var_sp_ov_x0_dn4, locals.var_sp_ov_x0_dn6, locals.var_sp_ov_x0_dn7, locals.var_sp_ov_x0_dn8, locals.var_sp_ov_x0_dn9,)
    }
};
        locals.var_sp_ov_x0 = assign24150_e24289;
        locals.var_sp_ov_x0_dn4 = assign24150_e24289_d_n4;
        locals.var_sp_ov_x0_dn6 = assign24150_e24289_d_n6;
        locals.var_sp_ov_x0_dn7 = assign24150_e24289_d_n7;
        locals.var_sp_ov_x0_dn8 = assign24150_e24289_d_n8;
        locals.var_sp_ov_x0_dn9 = assign24150_e24289_d_n9;
        locals.var_sp_ov_x0_rv = 0.0;

        let assign24160_e24291: f64 = (-locals.var_sp_ov_x0);
        let assign24160_e24292: f64 = (assign24160_e24291).abs();
        let assign24160_e24294: f64 = if assign24160_e24292 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard701 = assign24160_e24294;
        locals.var_guard701_rv = 0.0;

        let (assign24170_e24308, assign24170_e24308_d_n4, assign24170_e24308_d_n6, assign24170_e24308_d_n7, assign24170_e24308_d_n8, assign24170_e24308_d_n9,) = {
    if ((((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) && (locals.var_guard701 != 0.0)) {
        let assign24170_e24305: f64 = (-locals.var_sp_ov_x0);
        let assign24170_e24306: f64 = (assign24170_e24305).exp();
        (assign24170_e24306, (assign24170_e24306 * (-locals.var_sp_ov_x0_dn4)), (assign24170_e24306 * (-locals.var_sp_ov_x0_dn6)), (assign24170_e24306 * (-locals.var_sp_ov_x0_dn7)), (assign24170_e24306 * (-locals.var_sp_ov_x0_dn8)), (assign24170_e24306 * (-locals.var_sp_ov_x0_dn9)),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign24170_e24308;
        locals.var_sp_ov_d0_dn4 = assign24170_e24308_d_n4;
        locals.var_sp_ov_d0_dn6 = assign24170_e24308_d_n6;
        locals.var_sp_ov_d0_dn7 = assign24170_e24308_d_n7;
        locals.var_sp_ov_d0_dn8 = assign24170_e24308_d_n8;
        locals.var_sp_ov_d0_dn9 = assign24170_e24308_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let assign24180_e24310: f64 = (-locals.var_sp_ov_x0);
        let assign24180_e24312: f64 = (-80.0);
        let assign24180_e24313: f64 = if assign24180_e24310 < assign24180_e24312 { 1.0 } else { 0.0 };
        locals.var_guard702 = assign24180_e24313;
        locals.var_guard702_rv = 0.0;

        let (assign24190_e24356, assign24190_e24356_d_n4, assign24190_e24356_d_n6, assign24190_e24356_d_n7, assign24190_e24356_d_n8, assign24190_e24356_d_n9,) = {
    if (((((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) && (locals.var_guard701 == 0.0)) && (locals.var_guard702 != 0.0)) {
        let assign24190_e24329: f64 = (-locals.var_sp_ov_x0);
        let assign24190_e24330: f64 = (-assign24190_e24329);
        let assign24190_e24332: f64 = (assign24190_e24330 - 80.0);
        let assign24190_e24336: f64 = (-locals.var_sp_ov_x0);
        let assign24190_e24337: f64 = (-assign24190_e24336);
        let assign24190_e24339: f64 = (assign24190_e24337 - 80.0);
        let assign24190_e24340: f64 = (0.5 * assign24190_e24339);
        let assign24190_e24343: f64 = (-locals.var_sp_ov_x0);
        let assign24190_e24344: f64 = (-assign24190_e24343);
        let assign24190_e24346: f64 = (assign24190_e24344 - 80.0);
        let assign24190_e24348: f64 = (assign24190_e24346 * 0.3333333333333);
        let assign24190_e24349: f64 = (1.0 + assign24190_e24348);
        let assign24190_e24350: f64 = (assign24190_e24340 * assign24190_e24349);
        let assign24190_e24351: f64 = (1.0 + assign24190_e24350);
        let assign24190_e24352: f64 = (assign24190_e24332 * assign24190_e24351);
        let assign24190_e24353: f64 = (1.0 + assign24190_e24352);
        let assign24190_e24354: f64 = (1.80485e-35 / assign24190_e24353);
        (assign24190_e24354, (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn4)) * assign24190_e24351) + (assign24190_e24332 * (((0.5 * (-(-locals.var_sp_ov_x0_dn4))) * assign24190_e24349) + (assign24190_e24340 * ((-(-locals.var_sp_ov_x0_dn4)) * 0.3333333333333)))))) / (assign24190_e24353 * assign24190_e24353))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn6)) * assign24190_e24351) + (assign24190_e24332 * (((0.5 * (-(-locals.var_sp_ov_x0_dn6))) * assign24190_e24349) + (assign24190_e24340 * ((-(-locals.var_sp_ov_x0_dn6)) * 0.3333333333333)))))) / (assign24190_e24353 * assign24190_e24353))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn7)) * assign24190_e24351) + (assign24190_e24332 * (((0.5 * (-(-locals.var_sp_ov_x0_dn7))) * assign24190_e24349) + (assign24190_e24340 * ((-(-locals.var_sp_ov_x0_dn7)) * 0.3333333333333)))))) / (assign24190_e24353 * assign24190_e24353))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn8)) * assign24190_e24351) + (assign24190_e24332 * (((0.5 * (-(-locals.var_sp_ov_x0_dn8))) * assign24190_e24349) + (assign24190_e24340 * ((-(-locals.var_sp_ov_x0_dn8)) * 0.3333333333333)))))) / (assign24190_e24353 * assign24190_e24353))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn9)) * assign24190_e24351) + (assign24190_e24332 * (((0.5 * (-(-locals.var_sp_ov_x0_dn9))) * assign24190_e24349) + (assign24190_e24340 * ((-(-locals.var_sp_ov_x0_dn9)) * 0.3333333333333)))))) / (assign24190_e24353 * assign24190_e24353))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign24190_e24356;
        locals.var_sp_ov_d0_dn4 = assign24190_e24356_d_n4;
        locals.var_sp_ov_d0_dn6 = assign24190_e24356_d_n6;
        locals.var_sp_ov_d0_dn7 = assign24190_e24356_d_n7;
        locals.var_sp_ov_d0_dn8 = assign24190_e24356_d_n8;
        locals.var_sp_ov_d0_dn9 = assign24190_e24356_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_66(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24200_e24397, assign24200_e24397_d_n4, assign24200_e24397_d_n6, assign24200_e24397_d_n7, assign24200_e24397_d_n8, assign24200_e24397_d_n9,) = {
    if (((((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) && (locals.var_guard701 == 0.0)) && (locals.var_guard702 == 0.0)) {
        let assign24200_e24373: f64 = (-locals.var_sp_ov_x0);
        let assign24200_e24375: f64 = (assign24200_e24373 - 80.0);
        let assign24200_e24379: f64 = (-locals.var_sp_ov_x0);
        let assign24200_e24381: f64 = (assign24200_e24379 - 80.0);
        let assign24200_e24382: f64 = (0.5 * assign24200_e24381);
        let assign24200_e24385: f64 = (-locals.var_sp_ov_x0);
        let assign24200_e24387: f64 = (assign24200_e24385 - 80.0);
        let assign24200_e24389: f64 = (assign24200_e24387 * 0.3333333333333);
        let assign24200_e24390: f64 = (1.0 + assign24200_e24389);
        let assign24200_e24391: f64 = (assign24200_e24382 * assign24200_e24390);
        let assign24200_e24392: f64 = (1.0 + assign24200_e24391);
        let assign24200_e24393: f64 = (assign24200_e24375 * assign24200_e24392);
        let assign24200_e24394: f64 = (1.0 + assign24200_e24393);
        let assign24200_e24395: f64 = (5.54062e34 * assign24200_e24394);
        (assign24200_e24395, (5.54062e34 * (((-locals.var_sp_ov_x0_dn4) * assign24200_e24392) + (assign24200_e24375 * (((0.5 * (-locals.var_sp_ov_x0_dn4)) * assign24200_e24390) + (assign24200_e24382 * ((-locals.var_sp_ov_x0_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn6) * assign24200_e24392) + (assign24200_e24375 * (((0.5 * (-locals.var_sp_ov_x0_dn6)) * assign24200_e24390) + (assign24200_e24382 * ((-locals.var_sp_ov_x0_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn7) * assign24200_e24392) + (assign24200_e24375 * (((0.5 * (-locals.var_sp_ov_x0_dn7)) * assign24200_e24390) + (assign24200_e24382 * ((-locals.var_sp_ov_x0_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn8) * assign24200_e24392) + (assign24200_e24375 * (((0.5 * (-locals.var_sp_ov_x0_dn8)) * assign24200_e24390) + (assign24200_e24382 * ((-locals.var_sp_ov_x0_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn9) * assign24200_e24392) + (assign24200_e24375 * (((0.5 * (-locals.var_sp_ov_x0_dn9)) * assign24200_e24390) + (assign24200_e24382 * ((-locals.var_sp_ov_x0_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign24200_e24397;
        locals.var_sp_ov_d0_dn4 = assign24200_e24397_d_n4;
        locals.var_sp_ov_d0_dn6 = assign24200_e24397_d_n6;
        locals.var_sp_ov_d0_dn7 = assign24200_e24397_d_n7;
        locals.var_sp_ov_d0_dn8 = assign24200_e24397_d_n8;
        locals.var_sp_ov_d0_dn9 = assign24200_e24397_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign24210_e24417, assign24210_e24417_d_n4, assign24210_e24417_d_n6, assign24210_e24417_d_n7, assign24210_e24417_d_n8, assign24210_e24417_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24210_e24408: f64 = (locals.var_xgs_ovcv - locals.var_sp_ov_x0);
        let assign24210_e24409: f64 = (2.0 * assign24210_e24408);
        let assign24210_e24413: f64 = (1.0 - locals.var_sp_ov_d0);
        let assign24210_e24414: f64 = (locals.var_gov2 * assign24210_e24413);
        let assign24210_e24415: f64 = (assign24210_e24409 + assign24210_e24414);
        (assign24210_e24415, ((2.0 * (locals.var_xgs_ovcv_dn4 - locals.var_sp_ov_x0_dn4)) + ((locals.var_gov2_dn4 * assign24210_e24413) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn4)))), ((2.0 * (locals.var_xgs_ovcv_dn6 - locals.var_sp_ov_x0_dn6)) + ((locals.var_gov2_dn6 * assign24210_e24413) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn6)))), ((2.0 * (locals.var_xgs_ovcv_dn7 - locals.var_sp_ov_x0_dn7)) + ((locals.var_gov2_dn7 * assign24210_e24413) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn7)))), ((2.0 * (locals.var_xgs_ovcv_dn8 - locals.var_sp_ov_x0_dn8)) + ((locals.var_gov2_dn8 * assign24210_e24413) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn8)))), ((2.0 * (locals.var_xgs_ovcv_dn9 - locals.var_sp_ov_x0_dn9)) + ((locals.var_gov2_dn9 * assign24210_e24413) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_p, locals.var_sp_ov_p_dn4, locals.var_sp_ov_p_dn6, locals.var_sp_ov_p_dn7, locals.var_sp_ov_p_dn8, locals.var_sp_ov_p_dn9,)
    }
};
        locals.var_sp_ov_p = assign24210_e24417;
        locals.var_sp_ov_p_dn4 = assign24210_e24417_d_n4;
        locals.var_sp_ov_p_dn6 = assign24210_e24417_d_n6;
        locals.var_sp_ov_p_dn7 = assign24210_e24417_d_n7;
        locals.var_sp_ov_p_dn8 = assign24210_e24417_d_n8;
        locals.var_sp_ov_p_dn9 = assign24210_e24417_d_n9;
        locals.var_sp_ov_p_rv = 0.0;

        let (assign24220_e24441, assign24220_e24441_d_n4, assign24220_e24441_d_n6, assign24220_e24441_d_n7, assign24220_e24441_d_n8, assign24220_e24441_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24220_e24427: f64 = (locals.var_xgs_ovcv - locals.var_sp_ov_x0);
        let assign24220_e24430: f64 = (locals.var_xgs_ovcv - locals.var_sp_ov_x0);
        let assign24220_e24431: f64 = (assign24220_e24427 * assign24220_e24430);
        let assign24220_e24435: f64 = (locals.var_sp_ov_x0 - 1.0);
        let assign24220_e24437: f64 = (assign24220_e24435 + locals.var_sp_ov_d0);
        let assign24220_e24438: f64 = (locals.var_gov2 * assign24220_e24437);
        let assign24220_e24439: f64 = (assign24220_e24431 - assign24220_e24438);
        (assign24220_e24439, ((((locals.var_xgs_ovcv_dn4 - locals.var_sp_ov_x0_dn4) * assign24220_e24430) + (assign24220_e24427 * (locals.var_xgs_ovcv_dn4 - locals.var_sp_ov_x0_dn4))) - ((locals.var_gov2_dn4 * assign24220_e24437) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn4 + locals.var_sp_ov_d0_dn4)))), ((((locals.var_xgs_ovcv_dn6 - locals.var_sp_ov_x0_dn6) * assign24220_e24430) + (assign24220_e24427 * (locals.var_xgs_ovcv_dn6 - locals.var_sp_ov_x0_dn6))) - ((locals.var_gov2_dn6 * assign24220_e24437) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn6 + locals.var_sp_ov_d0_dn6)))), ((((locals.var_xgs_ovcv_dn7 - locals.var_sp_ov_x0_dn7) * assign24220_e24430) + (assign24220_e24427 * (locals.var_xgs_ovcv_dn7 - locals.var_sp_ov_x0_dn7))) - ((locals.var_gov2_dn7 * assign24220_e24437) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn7 + locals.var_sp_ov_d0_dn7)))), ((((locals.var_xgs_ovcv_dn8 - locals.var_sp_ov_x0_dn8) * assign24220_e24430) + (assign24220_e24427 * (locals.var_xgs_ovcv_dn8 - locals.var_sp_ov_x0_dn8))) - ((locals.var_gov2_dn8 * assign24220_e24437) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn8 + locals.var_sp_ov_d0_dn8)))), ((((locals.var_xgs_ovcv_dn9 - locals.var_sp_ov_x0_dn9) * assign24220_e24430) + (assign24220_e24427 * (locals.var_xgs_ovcv_dn9 - locals.var_sp_ov_x0_dn9))) - ((locals.var_gov2_dn9 * assign24220_e24437) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn9 + locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_q, locals.var_sp_ov_q_dn4, locals.var_sp_ov_q_dn6, locals.var_sp_ov_q_dn7, locals.var_sp_ov_q_dn8, locals.var_sp_ov_q_dn9,)
    }
};
        locals.var_sp_ov_q = assign24220_e24441;
        locals.var_sp_ov_q_dn4 = assign24220_e24441_d_n4;
        locals.var_sp_ov_q_dn6 = assign24220_e24441_d_n6;
        locals.var_sp_ov_q_dn7 = assign24220_e24441_d_n7;
        locals.var_sp_ov_q_dn8 = assign24220_e24441_d_n8;
        locals.var_sp_ov_q_dn9 = assign24220_e24441_d_n9;
        locals.var_sp_ov_q_rv = 0.0;

        let (assign24230_e24457, assign24230_e24457_d_n4, assign24230_e24457_d_n6, assign24230_e24457_d_n7, assign24230_e24457_d_n8, assign24230_e24457_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24230_e24452: f64 = (locals.var_gov2 * 0.5);
        let assign24230_e24454: f64 = (assign24230_e24452 * locals.var_sp_ov_d0);
        let assign24230_e24455: f64 = (1.0 - assign24230_e24454);
        (assign24230_e24455, (-(((locals.var_gov2_dn4 * 0.5) * locals.var_sp_ov_d0) + (assign24230_e24452 * locals.var_sp_ov_d0_dn4))), (-(((locals.var_gov2_dn6 * 0.5) * locals.var_sp_ov_d0) + (assign24230_e24452 * locals.var_sp_ov_d0_dn6))), (-(((locals.var_gov2_dn7 * 0.5) * locals.var_sp_ov_d0) + (assign24230_e24452 * locals.var_sp_ov_d0_dn7))), (-(((locals.var_gov2_dn8 * 0.5) * locals.var_sp_ov_d0) + (assign24230_e24452 * locals.var_sp_ov_d0_dn8))), (-(((locals.var_gov2_dn9 * 0.5) * locals.var_sp_ov_d0) + (assign24230_e24452 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_xi, locals.var_sp_ov_xi_dn4, locals.var_sp_ov_xi_dn6, locals.var_sp_ov_xi_dn7, locals.var_sp_ov_xi_dn8, locals.var_sp_ov_xi_dn9,)
    }
};
        locals.var_sp_ov_xi = assign24230_e24457;
        locals.var_sp_ov_xi_dn4 = assign24230_e24457_d_n4;
        locals.var_sp_ov_xi_dn6 = assign24230_e24457_d_n6;
        locals.var_sp_ov_xi_dn7 = assign24230_e24457_d_n7;
        locals.var_sp_ov_xi_dn8 = assign24230_e24457_d_n8;
        locals.var_sp_ov_xi_dn9 = assign24230_e24457_d_n9;
        locals.var_sp_ov_xi_rv = 0.0;

        let (assign24240_e24475, assign24240_e24475_d_n4, assign24240_e24475_d_n6, assign24240_e24475_d_n7, assign24240_e24475_d_n8, assign24240_e24475_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24240_e24467: f64 = (locals.var_sp_ov_p * locals.var_sp_ov_p);
        let assign24240_e24471: f64 = (locals.var_sp_ov_xi * locals.var_sp_ov_q);
        let assign24240_e24472: f64 = (4.0 * assign24240_e24471);
        let assign24240_e24473: f64 = (assign24240_e24467 - assign24240_e24472);
        (assign24240_e24473, (((locals.var_sp_ov_p_dn4 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn4)) - (4.0 * ((locals.var_sp_ov_xi_dn4 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn4)))), (((locals.var_sp_ov_p_dn6 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn6)) - (4.0 * ((locals.var_sp_ov_xi_dn6 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn6)))), (((locals.var_sp_ov_p_dn7 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn7)) - (4.0 * ((locals.var_sp_ov_xi_dn7 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn7)))), (((locals.var_sp_ov_p_dn8 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn8)) - (4.0 * ((locals.var_sp_ov_xi_dn8 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn8)))), (((locals.var_sp_ov_p_dn9 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn9)) - (4.0 * ((locals.var_sp_ov_xi_dn9 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24240_e24475;
        locals.var_sp_ov_temp_dn4 = assign24240_e24475_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24240_e24475_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24240_e24475_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24240_e24475_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24240_e24475_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign24250_e24492, assign24250_e24492_d_n4, assign24250_e24492_d_n6, assign24250_e24492_d_n7, assign24250_e24492_d_n8, assign24250_e24492_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24250_e24485: f64 = (2.0 * locals.var_sp_ov_q);
        let assign24250_e24488: f64 = (locals.var_sp_ov_temp).sqrt();
        let assign24250_e24489: f64 = (locals.var_sp_ov_p + assign24250_e24488);
        let assign24250_e24490: f64 = (assign24250_e24485 / assign24250_e24489);
        (assign24250_e24490, ((((2.0 * locals.var_sp_ov_q_dn4) * assign24250_e24489) - (assign24250_e24485 * (locals.var_sp_ov_p_dn4 + (locals.var_sp_ov_temp_dn4 / (2.0 * assign24250_e24488))))) / (assign24250_e24489 * assign24250_e24489)), ((((2.0 * locals.var_sp_ov_q_dn6) * assign24250_e24489) - (assign24250_e24485 * (locals.var_sp_ov_p_dn6 + (locals.var_sp_ov_temp_dn6 / (2.0 * assign24250_e24488))))) / (assign24250_e24489 * assign24250_e24489)), ((((2.0 * locals.var_sp_ov_q_dn7) * assign24250_e24489) - (assign24250_e24485 * (locals.var_sp_ov_p_dn7 + (locals.var_sp_ov_temp_dn7 / (2.0 * assign24250_e24488))))) / (assign24250_e24489 * assign24250_e24489)), ((((2.0 * locals.var_sp_ov_q_dn8) * assign24250_e24489) - (assign24250_e24485 * (locals.var_sp_ov_p_dn8 + (locals.var_sp_ov_temp_dn8 / (2.0 * assign24250_e24488))))) / (assign24250_e24489 * assign24250_e24489)), ((((2.0 * locals.var_sp_ov_q_dn9) * assign24250_e24489) - (assign24250_e24485 * (locals.var_sp_ov_p_dn9 + (locals.var_sp_ov_temp_dn9 / (2.0 * assign24250_e24488))))) / (assign24250_e24489 * assign24250_e24489)),)
    } else {
        (locals.var_sp_ov_u, locals.var_sp_ov_u_dn4, locals.var_sp_ov_u_dn6, locals.var_sp_ov_u_dn7, locals.var_sp_ov_u_dn8, locals.var_sp_ov_u_dn9,)
    }
};
        locals.var_sp_ov_u = assign24250_e24492;
        locals.var_sp_ov_u_dn4 = assign24250_e24492_d_n4;
        locals.var_sp_ov_u_dn6 = assign24250_e24492_d_n6;
        locals.var_sp_ov_u_dn7 = assign24250_e24492_d_n7;
        locals.var_sp_ov_u_dn8 = assign24250_e24492_d_n8;
        locals.var_sp_ov_u_dn9 = assign24250_e24492_d_n9;
        locals.var_sp_ov_u_rv = 0.0;

        let (assign24260_e24504, assign24260_e24504_d_n4, assign24260_e24504_d_n6, assign24260_e24504_d_n7, assign24260_e24504_d_n8, assign24260_e24504_d_n9,) = {
    if (((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) && (locals.var_guard696 == 0.0)) {
        let assign24260_e24502: f64 = (locals.var_sp_ov_x0 + locals.var_sp_ov_u);
        (assign24260_e24502, (locals.var_sp_ov_x0_dn4 + locals.var_sp_ov_u_dn4), (locals.var_sp_ov_x0_dn6 + locals.var_sp_ov_u_dn6), (locals.var_sp_ov_x0_dn7 + locals.var_sp_ov_u_dn7), (locals.var_sp_ov_x0_dn8 + locals.var_sp_ov_u_dn8), (locals.var_sp_ov_x0_dn9 + locals.var_sp_ov_u_dn9),)
    } else {
        (locals.var_xs_ovcv, locals.var_xs_ovcv_dn4, locals.var_xs_ovcv_dn6, locals.var_xs_ovcv_dn7, locals.var_xs_ovcv_dn8, locals.var_xs_ovcv_dn9,)
    }
};
        locals.var_xs_ovcv = assign24260_e24504;
        locals.var_xs_ovcv_dn4 = assign24260_e24504_d_n4;
        locals.var_xs_ovcv_dn6 = assign24260_e24504_d_n6;
        locals.var_xs_ovcv_dn7 = assign24260_e24504_d_n7;
        locals.var_xs_ovcv_dn8 = assign24260_e24504_d_n8;
        locals.var_xs_ovcv_dn9 = assign24260_e24504_d_n9;
        locals.var_xs_ovcv_rv = 0.0;

        let (assign24270_e24512, assign24270_e24512_d_n4, assign24270_e24512_d_n6, assign24270_e24512_d_n7, assign24270_e24512_d_n8, assign24270_e24512_d_n9,) = {
    if ((locals.var_guard694 != 0.0) && (locals.var_guard695 == 0.0)) {
        let assign24270_e24510: f64 = (-locals.var_xs_ovcv);
        (assign24270_e24510, (-locals.var_xs_ovcv_dn4), (-locals.var_xs_ovcv_dn6), (-locals.var_xs_ovcv_dn7), (-locals.var_xs_ovcv_dn8), (-locals.var_xs_ovcv_dn9),)
    } else {
        (locals.var_xs_ovcv, locals.var_xs_ovcv_dn4, locals.var_xs_ovcv_dn6, locals.var_xs_ovcv_dn7, locals.var_xs_ovcv_dn8, locals.var_xs_ovcv_dn9,)
    }
};
        locals.var_xs_ovcv = assign24270_e24512;
        locals.var_xs_ovcv_dn4 = assign24270_e24512_d_n4;
        locals.var_xs_ovcv_dn6 = assign24270_e24512_d_n6;
        locals.var_xs_ovcv_dn7 = assign24270_e24512_d_n7;
        locals.var_xs_ovcv_dn8 = assign24270_e24512_d_n8;
        locals.var_xs_ovcv_dn9 = assign24270_e24512_d_n9;
        locals.var_xs_ovcv_rv = 0.0;

        let assign24280_e24515: f64 = (2.0 * 1.602176565e-19);
        let assign24280_e24517: f64 = (assign24280_e24515 * locals.var_novd_i);
        let assign24280_e24519: f64 = (assign24280_e24517 * locals.var_epsch);
        let assign24280_e24521: f64 = (assign24280_e24519 * locals.var_inv_phit0);
        let assign24280_e24522: f64 = (assign24280_e24521).sqrt();
        let assign24280_e24524: f64 = (assign24280_e24522 / locals.var_cox1prime);
        locals.var_gov = assign24280_e24524;
        locals.var_gov_dn4 = (((assign24280_e24519 * locals.var_inv_phit0_dn4) / (2.0 * assign24280_e24522)) / locals.var_cox1prime);
        locals.var_gov_dn6 = (((assign24280_e24519 * locals.var_inv_phit0_dn6) / (2.0 * assign24280_e24522)) / locals.var_cox1prime);
        locals.var_gov_dn7 = (((assign24280_e24519 * locals.var_inv_phit0_dn7) / (2.0 * assign24280_e24522)) / locals.var_cox1prime);
        locals.var_gov_dn8 = (((assign24280_e24519 * locals.var_inv_phit0_dn8) / (2.0 * assign24280_e24522)) / locals.var_cox1prime);
        locals.var_gov_dn9 = (((assign24280_e24519 * locals.var_inv_phit0_dn9) / (2.0 * assign24280_e24522)) / locals.var_cox1prime);
        locals.var_gov_rv = 0.0;

        let assign24290_e24527: f64 = (locals.var_gov * locals.var_gov);
        locals.var_gov2 = assign24290_e24527;
        locals.var_gov2_dn4 = ((locals.var_gov_dn4 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn4));
        locals.var_gov2_dn6 = ((locals.var_gov_dn6 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn6));
        locals.var_gov2_dn7 = ((locals.var_gov_dn7 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn7));
        locals.var_gov2_dn8 = ((locals.var_gov_dn8 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn8));
        locals.var_gov2_dn9 = ((locals.var_gov_dn9 * locals.var_gov) + (locals.var_gov * locals.var_gov_dn9));
        locals.var_gov2_rv = 0.0;

        let assign24300_e24531: f64 = (locals.var_gov / 1.4142135623731);
        let assign24300_e24532: f64 = (1.0 + assign24300_e24531);
        locals.var_xi_ov = assign24300_e24532;
        locals.var_xi_ov_dn4 = (locals.var_gov_dn4 / 1.4142135623731);
        locals.var_xi_ov_dn6 = (locals.var_gov_dn6 / 1.4142135623731);
        locals.var_xi_ov_dn7 = (locals.var_gov_dn7 / 1.4142135623731);
        locals.var_xi_ov_dn8 = (locals.var_gov_dn8 / 1.4142135623731);
        locals.var_xi_ov_dn9 = (locals.var_gov_dn9 / 1.4142135623731);
        locals.var_xi_ov_rv = 0.0;

        let assign24310_e24535: f64 = (1e-5 * locals.var_xi_ov);
        locals.var_x_mrg_ov = assign24310_e24535;
        locals.var_x_mrg_ov_dn4 = (1e-5 * locals.var_xi_ov_dn4);
        locals.var_x_mrg_ov_dn6 = (1e-5 * locals.var_xi_ov_dn6);
        locals.var_x_mrg_ov_dn7 = (1e-5 * locals.var_xi_ov_dn7);
        locals.var_x_mrg_ov_dn8 = (1e-5 * locals.var_xi_ov_dn8);
        locals.var_x_mrg_ov_dn9 = (1e-5 * locals.var_xi_ov_dn9);
        locals.var_x_mrg_ov_rv = 0.0;

        let assign24320_e24538: f64 = (1.0 / locals.var_xi_ov);
        locals.var_inv_xi_ov = assign24320_e24538;
        locals.var_inv_xi_ov_dn4 = (-(locals.var_xi_ov_dn4 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_dn6 = (-(locals.var_xi_ov_dn6 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_dn7 = (-(locals.var_xi_ov_dn7 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_dn8 = (-(locals.var_xi_ov_dn8 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_dn9 = (-(locals.var_xi_ov_dn9 / (locals.var_xi_ov * locals.var_xi_ov)));
        locals.var_inv_xi_ov_rv = 0.0;

        let assign24330_e24543: f64 = (locals.var_gov * 0.7324648775608221);
        let assign24330_e24544: f64 = (1.25 + assign24330_e24543);
        let assign24330_e24545: f64 = (1.0 / assign24330_e24544);
        locals.var_inv_xg1 = assign24330_e24545;
        locals.var_inv_xg1_dn4 = (-((locals.var_gov_dn4 * 0.7324648775608221) / (assign24330_e24544 * assign24330_e24544)));
        locals.var_inv_xg1_dn6 = (-((locals.var_gov_dn6 * 0.7324648775608221) / (assign24330_e24544 * assign24330_e24544)));
        locals.var_inv_xg1_dn7 = (-((locals.var_gov_dn7 * 0.7324648775608221) / (assign24330_e24544 * assign24330_e24544)));
        locals.var_inv_xg1_dn8 = (-((locals.var_gov_dn8 * 0.7324648775608221) / (assign24330_e24544 * assign24330_e24544)));
        locals.var_inv_xg1_dn9 = (-((locals.var_gov_dn9 * 0.7324648775608221) / (assign24330_e24544 * assign24330_e24544)));
        locals.var_inv_xg1_rv = 0.0;

        let assign24340_e24564: f64 = if (((p.p3 > 0.0) && ((locals.var_igovinvd_i > 0.0) || (locals.var_igovaccd_i > 0.0))) || ((p.p4 > 0.0) && (locals.var_agidld_i > 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard703 = assign24340_e24564;
        locals.var_guard703_rv = 0.0;

        let assign24350_e24566: f64 = (locals.var_xgd_ov).abs();
        let assign24350_e24568: f64 = if assign24350_e24566 <= locals.var_x_mrg_ov { 1.0 } else { 0.0 };
        locals.var_guard704 = assign24350_e24568;
        locals.var_guard704_rv = 0.0;

        let (assign24360_e24577, assign24360_e24577_d_n4, assign24360_e24577_d_n6, assign24360_e24577_d_n7, assign24360_e24577_d_n8, assign24360_e24577_d_n9,) = {
    if ((locals.var_guard703 != 0.0) && (locals.var_guard704 != 0.0)) {
        let assign24360_e24573: f64 = (-locals.var_xgd_ov);
        let assign24360_e24575: f64 = (assign24360_e24573 * locals.var_inv_xi_ov);
        (assign24360_e24575, (((-locals.var_xgd_ov_dn4) * locals.var_inv_xi_ov) + (assign24360_e24573 * locals.var_inv_xi_ov_dn4)), (((-locals.var_xgd_ov_dn6) * locals.var_inv_xi_ov) + (assign24360_e24573 * locals.var_inv_xi_ov_dn6)), (((-locals.var_xgd_ov_dn7) * locals.var_inv_xi_ov) + (assign24360_e24573 * locals.var_inv_xi_ov_dn7)), (((-locals.var_xgd_ov_dn8) * locals.var_inv_xi_ov) + (assign24360_e24573 * locals.var_inv_xi_ov_dn8)), (((-locals.var_xgd_ov_dn9) * locals.var_inv_xi_ov) + (assign24360_e24573 * locals.var_inv_xi_ov_dn9)),)
    } else {
        (locals.var_xd_ov, locals.var_xd_ov_dn4, locals.var_xd_ov_dn6, locals.var_xd_ov_dn7, locals.var_xd_ov_dn8, locals.var_xd_ov_dn9,)
    }
};
        locals.var_xd_ov = assign24360_e24577;
        locals.var_xd_ov_dn4 = assign24360_e24577_d_n4;
        locals.var_xd_ov_dn6 = assign24360_e24577_d_n6;
        locals.var_xd_ov_dn7 = assign24360_e24577_d_n7;
        locals.var_xd_ov_dn8 = assign24360_e24577_d_n8;
        locals.var_xd_ov_dn9 = assign24360_e24577_d_n9;
        locals.var_xd_ov_rv = 0.0;

        let assign24370_e24580: f64 = (-locals.var_x_mrg_ov);
        let assign24370_e24581: f64 = if locals.var_xgd_ov < assign24370_e24580 { 1.0 } else { 0.0 };
        locals.var_guard705 = assign24370_e24581;
        locals.var_guard705_rv = 0.0;

        let (assign24380_e24591, assign24380_e24591_d_n4, assign24380_e24591_d_n6, assign24380_e24591_d_n7, assign24380_e24591_d_n8, assign24380_e24591_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24380_e24589: f64 = (-locals.var_xgd_ov);
        (assign24380_e24589, (-locals.var_xgd_ov_dn4), (-locals.var_xgd_ov_dn6), (-locals.var_xgd_ov_dn7), (-locals.var_xgd_ov_dn8), (-locals.var_xgd_ov_dn9),)
    } else {
        (locals.var_sp_ov_ygf, locals.var_sp_ov_ygf_dn4, locals.var_sp_ov_ygf_dn6, locals.var_sp_ov_ygf_dn7, locals.var_sp_ov_ygf_dn8, locals.var_sp_ov_ygf_dn9,)
    }
};
        locals.var_sp_ov_ygf = assign24380_e24591;
        locals.var_sp_ov_ygf_dn4 = assign24380_e24591_d_n4;
        locals.var_sp_ov_ygf_dn6 = assign24380_e24591_d_n6;
        locals.var_sp_ov_ygf_dn7 = assign24380_e24591_d_n7;
        locals.var_sp_ov_ygf_dn8 = assign24380_e24591_d_n8;
        locals.var_sp_ov_ygf_dn9 = assign24380_e24591_d_n9;
        locals.var_sp_ov_ygf_rv = 0.0;

        let (assign24390_e24604, assign24390_e24604_d_n4, assign24390_e24604_d_n6, assign24390_e24604_d_n7, assign24390_e24604_d_n8, assign24390_e24604_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24390_e24600: f64 = (1.25 * locals.var_sp_ov_ygf);
        let assign24390_e24602: f64 = (assign24390_e24600 * locals.var_inv_xi_ov);
        (assign24390_e24602, (((1.25 * locals.var_sp_ov_ygf_dn4) * locals.var_inv_xi_ov) + (assign24390_e24600 * locals.var_inv_xi_ov_dn4)), (((1.25 * locals.var_sp_ov_ygf_dn6) * locals.var_inv_xi_ov) + (assign24390_e24600 * locals.var_inv_xi_ov_dn6)), (((1.25 * locals.var_sp_ov_ygf_dn7) * locals.var_inv_xi_ov) + (assign24390_e24600 * locals.var_inv_xi_ov_dn7)), (((1.25 * locals.var_sp_ov_ygf_dn8) * locals.var_inv_xi_ov) + (assign24390_e24600 * locals.var_inv_xi_ov_dn8)), (((1.25 * locals.var_sp_ov_ygf_dn9) * locals.var_inv_xi_ov) + (assign24390_e24600 * locals.var_inv_xi_ov_dn9)),)
    } else {
        (locals.var_sp_ov_z, locals.var_sp_ov_z_dn4, locals.var_sp_ov_z_dn6, locals.var_sp_ov_z_dn7, locals.var_sp_ov_z_dn8, locals.var_sp_ov_z_dn9,)
    }
};
        locals.var_sp_ov_z = assign24390_e24604;
        locals.var_sp_ov_z_dn4 = assign24390_e24604_d_n4;
        locals.var_sp_ov_z_dn6 = assign24390_e24604_d_n6;
        locals.var_sp_ov_z_dn7 = assign24390_e24604_d_n7;
        locals.var_sp_ov_z_dn8 = assign24390_e24604_d_n8;
        locals.var_sp_ov_z_dn9 = assign24390_e24604_d_n9;
        locals.var_sp_ov_z_rv = 0.0;

        let (assign24400_e24628, assign24400_e24628_d_n4, assign24400_e24628_d_n6, assign24400_e24628_d_n7, assign24400_e24628_d_n8, assign24400_e24628_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24400_e24614: f64 = (locals.var_sp_ov_z + 10.0);
        let assign24400_e24617: f64 = (locals.var_sp_ov_z - 6.0);
        let assign24400_e24620: f64 = (locals.var_sp_ov_z - 6.0);
        let assign24400_e24621: f64 = (assign24400_e24617 * assign24400_e24620);
        let assign24400_e24623: f64 = (assign24400_e24621 + 64.0);
        let assign24400_e24624: f64 = (assign24400_e24623).sqrt();
        let assign24400_e24625: f64 = (assign24400_e24614 - assign24400_e24624);
        let assign24400_e24626: f64 = (0.5 * assign24400_e24625);
        (assign24400_e24626, (0.5 * (locals.var_sp_ov_z_dn4 - (((locals.var_sp_ov_z_dn4 * assign24400_e24620) + (assign24400_e24617 * locals.var_sp_ov_z_dn4)) / (2.0 * assign24400_e24624)))), (0.5 * (locals.var_sp_ov_z_dn6 - (((locals.var_sp_ov_z_dn6 * assign24400_e24620) + (assign24400_e24617 * locals.var_sp_ov_z_dn6)) / (2.0 * assign24400_e24624)))), (0.5 * (locals.var_sp_ov_z_dn7 - (((locals.var_sp_ov_z_dn7 * assign24400_e24620) + (assign24400_e24617 * locals.var_sp_ov_z_dn7)) / (2.0 * assign24400_e24624)))), (0.5 * (locals.var_sp_ov_z_dn8 - (((locals.var_sp_ov_z_dn8 * assign24400_e24620) + (assign24400_e24617 * locals.var_sp_ov_z_dn8)) / (2.0 * assign24400_e24624)))), (0.5 * (locals.var_sp_ov_z_dn9 - (((locals.var_sp_ov_z_dn9 * assign24400_e24620) + (assign24400_e24617 * locals.var_sp_ov_z_dn9)) / (2.0 * assign24400_e24624)))),)
    } else {
        (locals.var_sp_ov_eta, locals.var_sp_ov_eta_dn4, locals.var_sp_ov_eta_dn6, locals.var_sp_ov_eta_dn7, locals.var_sp_ov_eta_dn8, locals.var_sp_ov_eta_dn9,)
    }
};
        locals.var_sp_ov_eta = assign24400_e24628;
        locals.var_sp_ov_eta_dn4 = assign24400_e24628_d_n4;
        locals.var_sp_ov_eta_dn6 = assign24400_e24628_d_n6;
        locals.var_sp_ov_eta_dn7 = assign24400_e24628_d_n7;
        locals.var_sp_ov_eta_dn8 = assign24400_e24628_d_n8;
        locals.var_sp_ov_eta_dn9 = assign24400_e24628_d_n9;
        locals.var_sp_ov_eta_rv = 0.0;

        let (assign24410_e24649, assign24410_e24649_d_n4, assign24410_e24649_d_n6, assign24410_e24649_d_n7, assign24410_e24649_d_n8, assign24410_e24649_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24410_e24637: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign24410_e24640: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign24410_e24641: f64 = (assign24410_e24637 * assign24410_e24640);
        let assign24410_e24645: f64 = (locals.var_sp_ov_eta + 1.0);
        let assign24410_e24646: f64 = (locals.var_gov2 * assign24410_e24645);
        let assign24410_e24647: f64 = (assign24410_e24641 + assign24410_e24646);
        (assign24410_e24647, ((((locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4) * assign24410_e24640) + (assign24410_e24637 * (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4))) + ((locals.var_gov2_dn4 * assign24410_e24645) + (locals.var_gov2 * locals.var_sp_ov_eta_dn4))), ((((locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6) * assign24410_e24640) + (assign24410_e24637 * (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6))) + ((locals.var_gov2_dn6 * assign24410_e24645) + (locals.var_gov2 * locals.var_sp_ov_eta_dn6))), ((((locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7) * assign24410_e24640) + (assign24410_e24637 * (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7))) + ((locals.var_gov2_dn7 * assign24410_e24645) + (locals.var_gov2 * locals.var_sp_ov_eta_dn7))), ((((locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8) * assign24410_e24640) + (assign24410_e24637 * (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8))) + ((locals.var_gov2_dn8 * assign24410_e24645) + (locals.var_gov2 * locals.var_sp_ov_eta_dn8))), ((((locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9) * assign24410_e24640) + (assign24410_e24637 * (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9))) + ((locals.var_gov2_dn9 * assign24410_e24645) + (locals.var_gov2 * locals.var_sp_ov_eta_dn9))),)
    } else {
        (locals.var_sp_ov_a, locals.var_sp_ov_a_dn4, locals.var_sp_ov_a_dn6, locals.var_sp_ov_a_dn7, locals.var_sp_ov_a_dn8, locals.var_sp_ov_a_dn9,)
    }
};
        locals.var_sp_ov_a = assign24410_e24649;
        locals.var_sp_ov_a_dn4 = assign24410_e24649_d_n4;
        locals.var_sp_ov_a_dn6 = assign24410_e24649_d_n6;
        locals.var_sp_ov_a_dn7 = assign24410_e24649_d_n7;
        locals.var_sp_ov_a_dn8 = assign24410_e24649_d_n8;
        locals.var_sp_ov_a_dn9 = assign24410_e24649_d_n9;
        locals.var_sp_ov_a_rv = 0.0;

        let (assign24420_e24664, assign24420_e24664_d_n4, assign24420_e24664_d_n6, assign24420_e24664_d_n7, assign24420_e24664_d_n8, assign24420_e24664_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24420_e24659: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign24420_e24660: f64 = (2.0 * assign24420_e24659);
        let assign24420_e24662: f64 = (assign24420_e24660 - locals.var_gov2);
        (assign24420_e24662, ((2.0 * (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4)) - locals.var_gov2_dn4), ((2.0 * (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6)) - locals.var_gov2_dn6), ((2.0 * (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7)) - locals.var_gov2_dn7), ((2.0 * (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8)) - locals.var_gov2_dn8), ((2.0 * (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9)) - locals.var_gov2_dn9),)
    } else {
        (locals.var_sp_ov_c, locals.var_sp_ov_c_dn4, locals.var_sp_ov_c_dn6, locals.var_sp_ov_c_dn7, locals.var_sp_ov_c_dn8, locals.var_sp_ov_c_dn9,)
    }
};
        locals.var_sp_ov_c = assign24420_e24664;
        locals.var_sp_ov_c_dn4 = assign24420_e24664_d_n4;
        locals.var_sp_ov_c_dn6 = assign24420_e24664_d_n6;
        locals.var_sp_ov_c_dn7 = assign24420_e24664_d_n7;
        locals.var_sp_ov_c_dn8 = assign24420_e24664_d_n8;
        locals.var_sp_ov_c_dn9 = assign24420_e24664_d_n9;
        locals.var_sp_ov_c_rv = 0.0;

        let (assign24430_e24678, assign24430_e24678_d_n4, assign24430_e24678_d_n6, assign24430_e24678_d_n7, assign24430_e24678_d_n8, assign24430_e24678_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24430_e24673: f64 = (locals.var_sp_ov_a / locals.var_gov2);
        let assign24430_e24674: f64 = (assign24430_e24673).ln();
        let assign24430_e24676: f64 = (assign24430_e24674 - locals.var_sp_ov_eta);
        (assign24430_e24676, (((((locals.var_sp_ov_a_dn4 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn4)) / (locals.var_gov2 * locals.var_gov2)) / assign24430_e24673) - locals.var_sp_ov_eta_dn4), (((((locals.var_sp_ov_a_dn6 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn6)) / (locals.var_gov2 * locals.var_gov2)) / assign24430_e24673) - locals.var_sp_ov_eta_dn6), (((((locals.var_sp_ov_a_dn7 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn7)) / (locals.var_gov2 * locals.var_gov2)) / assign24430_e24673) - locals.var_sp_ov_eta_dn7), (((((locals.var_sp_ov_a_dn8 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn8)) / (locals.var_gov2 * locals.var_gov2)) / assign24430_e24673) - locals.var_sp_ov_eta_dn8), (((((locals.var_sp_ov_a_dn9 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn9)) / (locals.var_gov2 * locals.var_gov2)) / assign24430_e24673) - locals.var_sp_ov_eta_dn9),)
    } else {
        (locals.var_sp_ov_tau, locals.var_sp_ov_tau_dn4, locals.var_sp_ov_tau_dn6, locals.var_sp_ov_tau_dn7, locals.var_sp_ov_tau_dn8, locals.var_sp_ov_tau_dn9,)
    }
};
        locals.var_sp_ov_tau = assign24430_e24678;
        locals.var_sp_ov_tau_dn4 = assign24430_e24678_d_n4;
        locals.var_sp_ov_tau_dn6 = assign24430_e24678_d_n6;
        locals.var_sp_ov_tau_dn7 = assign24430_e24678_d_n7;
        locals.var_sp_ov_tau_dn8 = assign24430_e24678_d_n8;
        locals.var_sp_ov_tau_dn9 = assign24430_e24678_d_n9;
        locals.var_sp_ov_tau_rv = 0.0;

        let (assign24440_e24689, assign24440_e24689_d_n4, assign24440_e24689_d_n6, assign24440_e24689_d_n7, assign24440_e24689_d_n8, assign24440_e24689_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24440_e24687: f64 = (locals.var_sp_ov_a + locals.var_sp_ov_c);
        (assign24440_e24687, (locals.var_sp_ov_a_dn4 + locals.var_sp_ov_c_dn4), (locals.var_sp_ov_a_dn6 + locals.var_sp_ov_c_dn6), (locals.var_sp_ov_a_dn7 + locals.var_sp_ov_c_dn7), (locals.var_sp_ov_a_dn8 + locals.var_sp_ov_c_dn8), (locals.var_sp_ov_a_dn9 + locals.var_sp_ov_c_dn9),)
    } else {
        (locals.var_sp_ov_nu, locals.var_sp_ov_nu_dn4, locals.var_sp_ov_nu_dn6, locals.var_sp_ov_nu_dn7, locals.var_sp_ov_nu_dn8, locals.var_sp_ov_nu_dn9,)
    }
};
        locals.var_sp_ov_nu = assign24440_e24689;
        locals.var_sp_ov_nu_dn4 = assign24440_e24689_d_n4;
        locals.var_sp_ov_nu_dn6 = assign24440_e24689_d_n6;
        locals.var_sp_ov_nu_dn7 = assign24440_e24689_d_n7;
        locals.var_sp_ov_nu_dn8 = assign24440_e24689_d_n8;
        locals.var_sp_ov_nu_dn9 = assign24440_e24689_d_n9;
        locals.var_sp_ov_nu_rv = 0.0;

        let (assign24450_e24710, assign24450_e24710_d_n4, assign24450_e24710_d_n6, assign24450_e24710_d_n7, assign24450_e24710_d_n8, assign24450_e24710_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24450_e24698: f64 = (locals.var_sp_ov_nu * locals.var_sp_ov_nu);
        let assign24450_e24702: f64 = (0.5 * locals.var_sp_ov_c);
        let assign24450_e24704: f64 = (assign24450_e24702 * locals.var_sp_ov_c);
        let assign24450_e24706: f64 = (assign24450_e24704 - locals.var_sp_ov_a);
        let assign24450_e24707: f64 = (locals.var_sp_ov_tau * assign24450_e24706);
        let assign24450_e24708: f64 = (assign24450_e24698 + assign24450_e24707);
        (assign24450_e24708, (((locals.var_sp_ov_nu_dn4 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn4)) + ((locals.var_sp_ov_tau_dn4 * assign24450_e24706) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn4) * locals.var_sp_ov_c) + (assign24450_e24702 * locals.var_sp_ov_c_dn4)) - locals.var_sp_ov_a_dn4)))), (((locals.var_sp_ov_nu_dn6 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn6)) + ((locals.var_sp_ov_tau_dn6 * assign24450_e24706) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn6) * locals.var_sp_ov_c) + (assign24450_e24702 * locals.var_sp_ov_c_dn6)) - locals.var_sp_ov_a_dn6)))), (((locals.var_sp_ov_nu_dn7 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn7)) + ((locals.var_sp_ov_tau_dn7 * assign24450_e24706) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn7) * locals.var_sp_ov_c) + (assign24450_e24702 * locals.var_sp_ov_c_dn7)) - locals.var_sp_ov_a_dn7)))), (((locals.var_sp_ov_nu_dn8 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn8)) + ((locals.var_sp_ov_tau_dn8 * assign24450_e24706) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn8) * locals.var_sp_ov_c) + (assign24450_e24702 * locals.var_sp_ov_c_dn8)) - locals.var_sp_ov_a_dn8)))), (((locals.var_sp_ov_nu_dn9 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn9)) + ((locals.var_sp_ov_tau_dn9 * assign24450_e24706) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn9) * locals.var_sp_ov_c) + (assign24450_e24702 * locals.var_sp_ov_c_dn9)) - locals.var_sp_ov_a_dn9)))),)
    } else {
        (locals.var_sp_ov_mutau, locals.var_sp_ov_mutau_dn4, locals.var_sp_ov_mutau_dn6, locals.var_sp_ov_mutau_dn7, locals.var_sp_ov_mutau_dn8, locals.var_sp_ov_mutau_dn9,)
    }
};
        locals.var_sp_ov_mutau = assign24450_e24710;
        locals.var_sp_ov_mutau_dn4 = assign24450_e24710_d_n4;
        locals.var_sp_ov_mutau_dn6 = assign24450_e24710_d_n6;
        locals.var_sp_ov_mutau_dn7 = assign24450_e24710_d_n7;
        locals.var_sp_ov_mutau_dn8 = assign24450_e24710_d_n8;
        locals.var_sp_ov_mutau_dn9 = assign24450_e24710_d_n9;
        locals.var_sp_ov_mutau_rv = 0.0;

        let (assign24460_e24737, assign24460_e24737_d_n4, assign24460_e24737_d_n6, assign24460_e24737_d_n7, assign24460_e24737_d_n8, assign24460_e24737_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24460_e24720: f64 = (locals.var_sp_ov_nu / locals.var_sp_ov_mutau);
        let assign24460_e24722: f64 = (assign24460_e24720 * locals.var_sp_ov_tau);
        let assign24460_e24724: f64 = (assign24460_e24722 * locals.var_sp_ov_tau);
        let assign24460_e24726: f64 = (assign24460_e24724 * locals.var_sp_ov_c);
        let assign24460_e24729: f64 = (locals.var_sp_ov_c * locals.var_sp_ov_c);
        let assign24460_e24731: f64 = (assign24460_e24729 * 0.3333333333333);
        let assign24460_e24733: f64 = (assign24460_e24731 - locals.var_sp_ov_a);
        let assign24460_e24734: f64 = (assign24460_e24726 * assign24460_e24733);
        let assign24460_e24735: f64 = (locals.var_sp_ov_mutau + assign24460_e24734);
        (assign24460_e24735, (locals.var_sp_ov_mutau_dn4 + (((((((((((locals.var_sp_ov_nu_dn4 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn4)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign24460_e24720 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_tau) + (assign24460_e24722 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_c) + (assign24460_e24724 * locals.var_sp_ov_c_dn4)) * assign24460_e24733) + (assign24460_e24726 * ((((locals.var_sp_ov_c_dn4 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn4)) * 0.3333333333333) - locals.var_sp_ov_a_dn4)))), (locals.var_sp_ov_mutau_dn6 + (((((((((((locals.var_sp_ov_nu_dn6 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn6)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign24460_e24720 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_tau) + (assign24460_e24722 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_c) + (assign24460_e24724 * locals.var_sp_ov_c_dn6)) * assign24460_e24733) + (assign24460_e24726 * ((((locals.var_sp_ov_c_dn6 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn6)) * 0.3333333333333) - locals.var_sp_ov_a_dn6)))), (locals.var_sp_ov_mutau_dn7 + (((((((((((locals.var_sp_ov_nu_dn7 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn7)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign24460_e24720 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_tau) + (assign24460_e24722 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_c) + (assign24460_e24724 * locals.var_sp_ov_c_dn7)) * assign24460_e24733) + (assign24460_e24726 * ((((locals.var_sp_ov_c_dn7 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn7)) * 0.3333333333333) - locals.var_sp_ov_a_dn7)))), (locals.var_sp_ov_mutau_dn8 + (((((((((((locals.var_sp_ov_nu_dn8 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn8)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign24460_e24720 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_tau) + (assign24460_e24722 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_c) + (assign24460_e24724 * locals.var_sp_ov_c_dn8)) * assign24460_e24733) + (assign24460_e24726 * ((((locals.var_sp_ov_c_dn8 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn8)) * 0.3333333333333) - locals.var_sp_ov_a_dn8)))), (locals.var_sp_ov_mutau_dn9 + (((((((((((locals.var_sp_ov_nu_dn9 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn9)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign24460_e24720 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_tau) + (assign24460_e24722 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_c) + (assign24460_e24724 * locals.var_sp_ov_c_dn9)) * assign24460_e24733) + (assign24460_e24726 * ((((locals.var_sp_ov_c_dn9 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn9)) * 0.3333333333333) - locals.var_sp_ov_a_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24460_e24737;
        locals.var_sp_ov_temp_dn4 = assign24460_e24737_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24460_e24737_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24460_e24737_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24460_e24737_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24460_e24737_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign24470_e24754, assign24470_e24754_d_n4, assign24470_e24754_d_n6, assign24470_e24754_d_n7, assign24470_e24754_d_n8, assign24470_e24754_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24470_e24747: f64 = (locals.var_sp_ov_a * locals.var_sp_ov_nu);
        let assign24470_e24749: f64 = (assign24470_e24747 * locals.var_sp_ov_tau);
        let assign24470_e24751: f64 = (assign24470_e24749 / locals.var_sp_ov_temp);
        let assign24470_e24752: f64 = (locals.var_sp_ov_eta + assign24470_e24751);
        (assign24470_e24752, (locals.var_sp_ov_eta_dn4 + (((((((locals.var_sp_ov_a_dn4 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn4)) * locals.var_sp_ov_tau) + (assign24470_e24747 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_temp) - (assign24470_e24749 * locals.var_sp_ov_temp_dn4)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn6 + (((((((locals.var_sp_ov_a_dn6 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn6)) * locals.var_sp_ov_tau) + (assign24470_e24747 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_temp) - (assign24470_e24749 * locals.var_sp_ov_temp_dn6)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn7 + (((((((locals.var_sp_ov_a_dn7 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn7)) * locals.var_sp_ov_tau) + (assign24470_e24747 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_temp) - (assign24470_e24749 * locals.var_sp_ov_temp_dn7)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn8 + (((((((locals.var_sp_ov_a_dn8 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn8)) * locals.var_sp_ov_tau) + (assign24470_e24747 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_temp) - (assign24470_e24749 * locals.var_sp_ov_temp_dn8)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn9 + (((((((locals.var_sp_ov_a_dn9 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn9)) * locals.var_sp_ov_tau) + (assign24470_e24747 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_temp) - (assign24470_e24749 * locals.var_sp_ov_temp_dn9)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))),)
    } else {
        (locals.var_sp_ov_y0, locals.var_sp_ov_y0_dn4, locals.var_sp_ov_y0_dn6, locals.var_sp_ov_y0_dn7, locals.var_sp_ov_y0_dn8, locals.var_sp_ov_y0_dn9,)
    }
};
        locals.var_sp_ov_y0 = assign24470_e24754;
        locals.var_sp_ov_y0_dn4 = assign24470_e24754_d_n4;
        locals.var_sp_ov_y0_dn6 = assign24470_e24754_d_n6;
        locals.var_sp_ov_y0_dn7 = assign24470_e24754_d_n7;
        locals.var_sp_ov_y0_dn8 = assign24470_e24754_d_n8;
        locals.var_sp_ov_y0_dn9 = assign24470_e24754_d_n9;
        locals.var_sp_ov_y0_rv = 0.0;

        let assign24480_e24756: f64 = (locals.var_sp_ov_y0).abs();
        let assign24480_e24758: f64 = if assign24480_e24756 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard706 = assign24480_e24758;
        locals.var_guard706_rv = 0.0;

        let (assign24490_e24770, assign24490_e24770_d_n4, assign24490_e24770_d_n6, assign24490_e24770_d_n7, assign24490_e24770_d_n8, assign24490_e24770_d_n9,) = {
    if ((((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) && (locals.var_guard706 != 0.0)) {
        let assign24490_e24768: f64 = (locals.var_sp_ov_y0).exp();
        (assign24490_e24768, (assign24490_e24768 * locals.var_sp_ov_y0_dn4), (assign24490_e24768 * locals.var_sp_ov_y0_dn6), (assign24490_e24768 * locals.var_sp_ov_y0_dn7), (assign24490_e24768 * locals.var_sp_ov_y0_dn8), (assign24490_e24768 * locals.var_sp_ov_y0_dn9),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign24490_e24770;
        locals.var_sp_ov_d0_dn4 = assign24490_e24770_d_n4;
        locals.var_sp_ov_d0_dn6 = assign24490_e24770_d_n6;
        locals.var_sp_ov_d0_dn7 = assign24490_e24770_d_n7;
        locals.var_sp_ov_d0_dn8 = assign24490_e24770_d_n8;
        locals.var_sp_ov_d0_dn9 = assign24490_e24770_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let assign24500_e24773: f64 = (-80.0);
        let assign24500_e24774: f64 = if locals.var_sp_ov_y0 < assign24500_e24773 { 1.0 } else { 0.0 };
        locals.var_guard707 = assign24500_e24774;
        locals.var_guard707_rv = 0.0;

        let (assign24510_e24813, assign24510_e24813_d_n4, assign24510_e24813_d_n6, assign24510_e24813_d_n7, assign24510_e24813_d_n8, assign24510_e24813_d_n9,) = {
    if (((((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) && (locals.var_guard706 == 0.0)) && (locals.var_guard707 != 0.0)) {
        let assign24510_e24789: f64 = (-locals.var_sp_ov_y0);
        let assign24510_e24791: f64 = (assign24510_e24789 - 80.0);
        let assign24510_e24795: f64 = (-locals.var_sp_ov_y0);
        let assign24510_e24797: f64 = (assign24510_e24795 - 80.0);
        let assign24510_e24798: f64 = (0.5 * assign24510_e24797);
        let assign24510_e24801: f64 = (-locals.var_sp_ov_y0);
        let assign24510_e24803: f64 = (assign24510_e24801 - 80.0);
        let assign24510_e24805: f64 = (assign24510_e24803 * 0.3333333333333);
        let assign24510_e24806: f64 = (1.0 + assign24510_e24805);
        let assign24510_e24807: f64 = (assign24510_e24798 * assign24510_e24806);
        let assign24510_e24808: f64 = (1.0 + assign24510_e24807);
        let assign24510_e24809: f64 = (assign24510_e24791 * assign24510_e24808);
        let assign24510_e24810: f64 = (1.0 + assign24510_e24809);
        let assign24510_e24811: f64 = (1.80485e-35 / assign24510_e24810);
        (assign24510_e24811, (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn4) * assign24510_e24808) + (assign24510_e24791 * (((0.5 * (-locals.var_sp_ov_y0_dn4)) * assign24510_e24806) + (assign24510_e24798 * ((-locals.var_sp_ov_y0_dn4) * 0.3333333333333)))))) / (assign24510_e24810 * assign24510_e24810))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn6) * assign24510_e24808) + (assign24510_e24791 * (((0.5 * (-locals.var_sp_ov_y0_dn6)) * assign24510_e24806) + (assign24510_e24798 * ((-locals.var_sp_ov_y0_dn6) * 0.3333333333333)))))) / (assign24510_e24810 * assign24510_e24810))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn7) * assign24510_e24808) + (assign24510_e24791 * (((0.5 * (-locals.var_sp_ov_y0_dn7)) * assign24510_e24806) + (assign24510_e24798 * ((-locals.var_sp_ov_y0_dn7) * 0.3333333333333)))))) / (assign24510_e24810 * assign24510_e24810))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn8) * assign24510_e24808) + (assign24510_e24791 * (((0.5 * (-locals.var_sp_ov_y0_dn8)) * assign24510_e24806) + (assign24510_e24798 * ((-locals.var_sp_ov_y0_dn8) * 0.3333333333333)))))) / (assign24510_e24810 * assign24510_e24810))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn9) * assign24510_e24808) + (assign24510_e24791 * (((0.5 * (-locals.var_sp_ov_y0_dn9)) * assign24510_e24806) + (assign24510_e24798 * ((-locals.var_sp_ov_y0_dn9) * 0.3333333333333)))))) / (assign24510_e24810 * assign24510_e24810))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign24510_e24813;
        locals.var_sp_ov_d0_dn4 = assign24510_e24813_d_n4;
        locals.var_sp_ov_d0_dn6 = assign24510_e24813_d_n6;
        locals.var_sp_ov_d0_dn7 = assign24510_e24813_d_n7;
        locals.var_sp_ov_d0_dn8 = assign24510_e24813_d_n8;
        locals.var_sp_ov_d0_dn9 = assign24510_e24813_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_67(
        locals: &mut StampLocals,
    ) {
        let (assign24520_e24850, assign24520_e24850_d_n4, assign24520_e24850_d_n6, assign24520_e24850_d_n7, assign24520_e24850_d_n8, assign24520_e24850_d_n9,) = {
    if (((((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) && (locals.var_guard706 == 0.0)) && (locals.var_guard707 == 0.0)) {
        let assign24520_e24830: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign24520_e24835: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign24520_e24836: f64 = (0.5 * assign24520_e24835);
        let assign24520_e24840: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign24520_e24842: f64 = (assign24520_e24840 * 0.3333333333333);
        let assign24520_e24843: f64 = (1.0 + assign24520_e24842);
        let assign24520_e24844: f64 = (assign24520_e24836 * assign24520_e24843);
        let assign24520_e24845: f64 = (1.0 + assign24520_e24844);
        let assign24520_e24846: f64 = (assign24520_e24830 * assign24520_e24845);
        let assign24520_e24847: f64 = (1.0 + assign24520_e24846);
        let assign24520_e24848: f64 = (5.54062e34 * assign24520_e24847);
        (assign24520_e24848, (5.54062e34 * ((locals.var_sp_ov_y0_dn4 * assign24520_e24845) + (assign24520_e24830 * (((0.5 * locals.var_sp_ov_y0_dn4) * assign24520_e24843) + (assign24520_e24836 * (locals.var_sp_ov_y0_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn6 * assign24520_e24845) + (assign24520_e24830 * (((0.5 * locals.var_sp_ov_y0_dn6) * assign24520_e24843) + (assign24520_e24836 * (locals.var_sp_ov_y0_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn7 * assign24520_e24845) + (assign24520_e24830 * (((0.5 * locals.var_sp_ov_y0_dn7) * assign24520_e24843) + (assign24520_e24836 * (locals.var_sp_ov_y0_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn8 * assign24520_e24845) + (assign24520_e24830 * (((0.5 * locals.var_sp_ov_y0_dn8) * assign24520_e24843) + (assign24520_e24836 * (locals.var_sp_ov_y0_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn9 * assign24520_e24845) + (assign24520_e24830 * (((0.5 * locals.var_sp_ov_y0_dn9) * assign24520_e24843) + (assign24520_e24836 * (locals.var_sp_ov_y0_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign24520_e24850;
        locals.var_sp_ov_d0_dn4 = assign24520_e24850_d_n4;
        locals.var_sp_ov_d0_dn6 = assign24520_e24850_d_n6;
        locals.var_sp_ov_d0_dn7 = assign24520_e24850_d_n7;
        locals.var_sp_ov_d0_dn8 = assign24520_e24850_d_n8;
        locals.var_sp_ov_d0_dn9 = assign24520_e24850_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign24530_e24861, assign24530_e24861_d_n4, assign24530_e24861_d_n6, assign24530_e24861_d_n7, assign24530_e24861_d_n8, assign24530_e24861_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24530_e24859: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_y0);
        (assign24530_e24859, (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_y0_dn4), (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_y0_dn6), (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_y0_dn7), (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_y0_dn8), (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_y0_dn9),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24530_e24861;
        locals.var_sp_ov_temp_dn4 = assign24530_e24861_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24530_e24861_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24530_e24861_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24530_e24861_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24530_e24861_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign24540_e24878, assign24540_e24878_d_n4, assign24540_e24878_d_n6, assign24540_e24878_d_n7, assign24540_e24878_d_n8, assign24540_e24878_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24540_e24870: f64 = (2.0 * locals.var_sp_ov_temp);
        let assign24540_e24874: f64 = (locals.var_sp_ov_d0 - 1.0);
        let assign24540_e24875: f64 = (locals.var_gov2 * assign24540_e24874);
        let assign24540_e24876: f64 = (assign24540_e24870 + assign24540_e24875);
        (assign24540_e24876, ((2.0 * locals.var_sp_ov_temp_dn4) + ((locals.var_gov2_dn4 * assign24540_e24874) + (locals.var_gov2 * locals.var_sp_ov_d0_dn4))), ((2.0 * locals.var_sp_ov_temp_dn6) + ((locals.var_gov2_dn6 * assign24540_e24874) + (locals.var_gov2 * locals.var_sp_ov_d0_dn6))), ((2.0 * locals.var_sp_ov_temp_dn7) + ((locals.var_gov2_dn7 * assign24540_e24874) + (locals.var_gov2 * locals.var_sp_ov_d0_dn7))), ((2.0 * locals.var_sp_ov_temp_dn8) + ((locals.var_gov2_dn8 * assign24540_e24874) + (locals.var_gov2 * locals.var_sp_ov_d0_dn8))), ((2.0 * locals.var_sp_ov_temp_dn9) + ((locals.var_gov2_dn9 * assign24540_e24874) + (locals.var_gov2 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_p, locals.var_sp_ov_p_dn4, locals.var_sp_ov_p_dn6, locals.var_sp_ov_p_dn7, locals.var_sp_ov_p_dn8, locals.var_sp_ov_p_dn9,)
    }
};
        locals.var_sp_ov_p = assign24540_e24878;
        locals.var_sp_ov_p_dn4 = assign24540_e24878_d_n4;
        locals.var_sp_ov_p_dn6 = assign24540_e24878_d_n6;
        locals.var_sp_ov_p_dn7 = assign24540_e24878_d_n7;
        locals.var_sp_ov_p_dn8 = assign24540_e24878_d_n8;
        locals.var_sp_ov_p_dn9 = assign24540_e24878_d_n9;
        locals.var_sp_ov_p_rv = 0.0;

        let (assign24550_e24897, assign24550_e24897_d_n4, assign24550_e24897_d_n6, assign24550_e24897_d_n7, assign24550_e24897_d_n8, assign24550_e24897_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24550_e24887: f64 = (locals.var_sp_ov_temp * locals.var_sp_ov_temp);
        let assign24550_e24891: f64 = (locals.var_sp_ov_y0 + 1.0);
        let assign24550_e24893: f64 = (assign24550_e24891 - locals.var_sp_ov_d0);
        let assign24550_e24894: f64 = (locals.var_gov2 * assign24550_e24893);
        let assign24550_e24895: f64 = (assign24550_e24887 + assign24550_e24894);
        (assign24550_e24895, (((locals.var_sp_ov_temp_dn4 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn4)) + ((locals.var_gov2_dn4 * assign24550_e24893) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn4 - locals.var_sp_ov_d0_dn4)))), (((locals.var_sp_ov_temp_dn6 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn6)) + ((locals.var_gov2_dn6 * assign24550_e24893) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn6 - locals.var_sp_ov_d0_dn6)))), (((locals.var_sp_ov_temp_dn7 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn7)) + ((locals.var_gov2_dn7 * assign24550_e24893) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn7 - locals.var_sp_ov_d0_dn7)))), (((locals.var_sp_ov_temp_dn8 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn8)) + ((locals.var_gov2_dn8 * assign24550_e24893) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn8 - locals.var_sp_ov_d0_dn8)))), (((locals.var_sp_ov_temp_dn9 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn9)) + ((locals.var_gov2_dn9 * assign24550_e24893) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn9 - locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_q, locals.var_sp_ov_q_dn4, locals.var_sp_ov_q_dn6, locals.var_sp_ov_q_dn7, locals.var_sp_ov_q_dn8, locals.var_sp_ov_q_dn9,)
    }
};
        locals.var_sp_ov_q = assign24550_e24897;
        locals.var_sp_ov_q_dn4 = assign24550_e24897_d_n4;
        locals.var_sp_ov_q_dn6 = assign24550_e24897_d_n6;
        locals.var_sp_ov_q_dn7 = assign24550_e24897_d_n7;
        locals.var_sp_ov_q_dn8 = assign24550_e24897_d_n8;
        locals.var_sp_ov_q_dn9 = assign24550_e24897_d_n9;
        locals.var_sp_ov_q_rv = 0.0;

        let (assign24560_e24912, assign24560_e24912_d_n4, assign24560_e24912_d_n6, assign24560_e24912_d_n7, assign24560_e24912_d_n8, assign24560_e24912_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24560_e24907: f64 = (locals.var_gov2 * 0.5);
        let assign24560_e24909: f64 = (assign24560_e24907 * locals.var_sp_ov_d0);
        let assign24560_e24910: f64 = (1.0 - assign24560_e24909);
        (assign24560_e24910, (-(((locals.var_gov2_dn4 * 0.5) * locals.var_sp_ov_d0) + (assign24560_e24907 * locals.var_sp_ov_d0_dn4))), (-(((locals.var_gov2_dn6 * 0.5) * locals.var_sp_ov_d0) + (assign24560_e24907 * locals.var_sp_ov_d0_dn6))), (-(((locals.var_gov2_dn7 * 0.5) * locals.var_sp_ov_d0) + (assign24560_e24907 * locals.var_sp_ov_d0_dn7))), (-(((locals.var_gov2_dn8 * 0.5) * locals.var_sp_ov_d0) + (assign24560_e24907 * locals.var_sp_ov_d0_dn8))), (-(((locals.var_gov2_dn9 * 0.5) * locals.var_sp_ov_d0) + (assign24560_e24907 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_xi, locals.var_sp_ov_xi_dn4, locals.var_sp_ov_xi_dn6, locals.var_sp_ov_xi_dn7, locals.var_sp_ov_xi_dn8, locals.var_sp_ov_xi_dn9,)
    }
};
        locals.var_sp_ov_xi = assign24560_e24912;
        locals.var_sp_ov_xi_dn4 = assign24560_e24912_d_n4;
        locals.var_sp_ov_xi_dn6 = assign24560_e24912_d_n6;
        locals.var_sp_ov_xi_dn7 = assign24560_e24912_d_n7;
        locals.var_sp_ov_xi_dn8 = assign24560_e24912_d_n8;
        locals.var_sp_ov_xi_dn9 = assign24560_e24912_d_n9;
        locals.var_sp_ov_xi_rv = 0.0;

        let (assign24570_e24929, assign24570_e24929_d_n4, assign24570_e24929_d_n6, assign24570_e24929_d_n7, assign24570_e24929_d_n8, assign24570_e24929_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24570_e24921: f64 = (locals.var_sp_ov_p * locals.var_sp_ov_p);
        let assign24570_e24925: f64 = (locals.var_sp_ov_xi * locals.var_sp_ov_q);
        let assign24570_e24926: f64 = (4.0 * assign24570_e24925);
        let assign24570_e24927: f64 = (assign24570_e24921 - assign24570_e24926);
        (assign24570_e24927, (((locals.var_sp_ov_p_dn4 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn4)) - (4.0 * ((locals.var_sp_ov_xi_dn4 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn4)))), (((locals.var_sp_ov_p_dn6 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn6)) - (4.0 * ((locals.var_sp_ov_xi_dn6 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn6)))), (((locals.var_sp_ov_p_dn7 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn7)) - (4.0 * ((locals.var_sp_ov_xi_dn7 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn7)))), (((locals.var_sp_ov_p_dn8 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn8)) - (4.0 * ((locals.var_sp_ov_xi_dn8 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn8)))), (((locals.var_sp_ov_p_dn9 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn9)) - (4.0 * ((locals.var_sp_ov_xi_dn9 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24570_e24929;
        locals.var_sp_ov_temp_dn4 = assign24570_e24929_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24570_e24929_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24570_e24929_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24570_e24929_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24570_e24929_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign24580_e24945, assign24580_e24945_d_n4, assign24580_e24945_d_n6, assign24580_e24945_d_n7, assign24580_e24945_d_n8, assign24580_e24945_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24580_e24938: f64 = (2.0 * locals.var_sp_ov_q);
        let assign24580_e24941: f64 = (locals.var_sp_ov_temp).sqrt();
        let assign24580_e24942: f64 = (locals.var_sp_ov_p + assign24580_e24941);
        let assign24580_e24943: f64 = (assign24580_e24938 / assign24580_e24942);
        (assign24580_e24943, ((((2.0 * locals.var_sp_ov_q_dn4) * assign24580_e24942) - (assign24580_e24938 * (locals.var_sp_ov_p_dn4 + (locals.var_sp_ov_temp_dn4 / (2.0 * assign24580_e24941))))) / (assign24580_e24942 * assign24580_e24942)), ((((2.0 * locals.var_sp_ov_q_dn6) * assign24580_e24942) - (assign24580_e24938 * (locals.var_sp_ov_p_dn6 + (locals.var_sp_ov_temp_dn6 / (2.0 * assign24580_e24941))))) / (assign24580_e24942 * assign24580_e24942)), ((((2.0 * locals.var_sp_ov_q_dn7) * assign24580_e24942) - (assign24580_e24938 * (locals.var_sp_ov_p_dn7 + (locals.var_sp_ov_temp_dn7 / (2.0 * assign24580_e24941))))) / (assign24580_e24942 * assign24580_e24942)), ((((2.0 * locals.var_sp_ov_q_dn8) * assign24580_e24942) - (assign24580_e24938 * (locals.var_sp_ov_p_dn8 + (locals.var_sp_ov_temp_dn8 / (2.0 * assign24580_e24941))))) / (assign24580_e24942 * assign24580_e24942)), ((((2.0 * locals.var_sp_ov_q_dn9) * assign24580_e24942) - (assign24580_e24938 * (locals.var_sp_ov_p_dn9 + (locals.var_sp_ov_temp_dn9 / (2.0 * assign24580_e24941))))) / (assign24580_e24942 * assign24580_e24942)),)
    } else {
        (locals.var_sp_ov_w, locals.var_sp_ov_w_dn4, locals.var_sp_ov_w_dn6, locals.var_sp_ov_w_dn7, locals.var_sp_ov_w_dn8, locals.var_sp_ov_w_dn9,)
    }
};
        locals.var_sp_ov_w = assign24580_e24945;
        locals.var_sp_ov_w_dn4 = assign24580_e24945_d_n4;
        locals.var_sp_ov_w_dn6 = assign24580_e24945_d_n6;
        locals.var_sp_ov_w_dn7 = assign24580_e24945_d_n7;
        locals.var_sp_ov_w_dn8 = assign24580_e24945_d_n8;
        locals.var_sp_ov_w_dn9 = assign24580_e24945_d_n9;
        locals.var_sp_ov_w_rv = 0.0;

        let (assign24590_e24957, assign24590_e24957_d_n4, assign24590_e24957_d_n6, assign24590_e24957_d_n7, assign24590_e24957_d_n8, assign24590_e24957_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 != 0.0)) {
        let assign24590_e24954: f64 = (locals.var_sp_ov_y0 + locals.var_sp_ov_w);
        let assign24590_e24955: f64 = (-assign24590_e24954);
        (assign24590_e24955, (-(locals.var_sp_ov_y0_dn4 + locals.var_sp_ov_w_dn4)), (-(locals.var_sp_ov_y0_dn6 + locals.var_sp_ov_w_dn6)), (-(locals.var_sp_ov_y0_dn7 + locals.var_sp_ov_w_dn7)), (-(locals.var_sp_ov_y0_dn8 + locals.var_sp_ov_w_dn8)), (-(locals.var_sp_ov_y0_dn9 + locals.var_sp_ov_w_dn9)),)
    } else {
        (locals.var_xd_ov, locals.var_xd_ov_dn4, locals.var_xd_ov_dn6, locals.var_xd_ov_dn7, locals.var_xd_ov_dn8, locals.var_xd_ov_dn9,)
    }
};
        locals.var_xd_ov = assign24590_e24957;
        locals.var_xd_ov_dn4 = assign24590_e24957_d_n4;
        locals.var_xd_ov_dn6 = assign24590_e24957_d_n6;
        locals.var_xd_ov_dn7 = assign24590_e24957_d_n7;
        locals.var_xd_ov_dn8 = assign24590_e24957_d_n8;
        locals.var_xd_ov_dn9 = assign24590_e24957_d_n9;
        locals.var_xd_ov_rv = 0.0;

        let (assign24600_e24975, assign24600_e24975_d_n4, assign24600_e24975_d_n6, assign24600_e24975_d_n7, assign24600_e24975_d_n8, assign24600_e24975_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 == 0.0)) {
        let assign24600_e24967: f64 = (locals.var_xi_ov * 1.25);
        let assign24600_e24969: f64 = (assign24600_e24967 * locals.var_inv_xg1);
        let assign24600_e24971: f64 = (assign24600_e24969 - 1.0);
        let assign24600_e24973: f64 = (assign24600_e24971 * locals.var_inv_xg1);
        (assign24600_e24973, (((((locals.var_xi_ov_dn4 * 1.25) * locals.var_inv_xg1) + (assign24600_e24967 * locals.var_inv_xg1_dn4)) * locals.var_inv_xg1) + (assign24600_e24971 * locals.var_inv_xg1_dn4)), (((((locals.var_xi_ov_dn6 * 1.25) * locals.var_inv_xg1) + (assign24600_e24967 * locals.var_inv_xg1_dn6)) * locals.var_inv_xg1) + (assign24600_e24971 * locals.var_inv_xg1_dn6)), (((((locals.var_xi_ov_dn7 * 1.25) * locals.var_inv_xg1) + (assign24600_e24967 * locals.var_inv_xg1_dn7)) * locals.var_inv_xg1) + (assign24600_e24971 * locals.var_inv_xg1_dn7)), (((((locals.var_xi_ov_dn8 * 1.25) * locals.var_inv_xg1) + (assign24600_e24967 * locals.var_inv_xg1_dn8)) * locals.var_inv_xg1) + (assign24600_e24971 * locals.var_inv_xg1_dn8)), (((((locals.var_xi_ov_dn9 * 1.25) * locals.var_inv_xg1) + (assign24600_e24967 * locals.var_inv_xg1_dn9)) * locals.var_inv_xg1) + (assign24600_e24971 * locals.var_inv_xg1_dn9)),)
    } else {
        (locals.var_sp_ov_afac, locals.var_sp_ov_afac_dn4, locals.var_sp_ov_afac_dn6, locals.var_sp_ov_afac_dn7, locals.var_sp_ov_afac_dn8, locals.var_sp_ov_afac_dn9,)
    }
};
        locals.var_sp_ov_afac = assign24600_e24975;
        locals.var_sp_ov_afac_dn4 = assign24600_e24975_d_n4;
        locals.var_sp_ov_afac_dn6 = assign24600_e24975_d_n6;
        locals.var_sp_ov_afac_dn7 = assign24600_e24975_d_n7;
        locals.var_sp_ov_afac_dn8 = assign24600_e24975_d_n8;
        locals.var_sp_ov_afac_dn9 = assign24600_e24975_d_n9;
        locals.var_sp_ov_afac_rv = 0.0;

        let (assign24610_e24993, assign24610_e24993_d_n4, assign24610_e24993_d_n6, assign24610_e24993_d_n7, assign24610_e24993_d_n8, assign24610_e24993_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 == 0.0)) {
        let assign24610_e24985: f64 = (locals.var_xgd_ov * locals.var_inv_xi_ov);
        let assign24610_e24989: f64 = (locals.var_sp_ov_afac * locals.var_xgd_ov);
        let assign24610_e24990: f64 = (1.0 + assign24610_e24989);
        let assign24610_e24991: f64 = (assign24610_e24985 * assign24610_e24990);
        (assign24610_e24991, ((((locals.var_xgd_ov_dn4 * locals.var_inv_xi_ov) + (locals.var_xgd_ov * locals.var_inv_xi_ov_dn4)) * assign24610_e24990) + (assign24610_e24985 * ((locals.var_sp_ov_afac_dn4 * locals.var_xgd_ov) + (locals.var_sp_ov_afac * locals.var_xgd_ov_dn4)))), ((((locals.var_xgd_ov_dn6 * locals.var_inv_xi_ov) + (locals.var_xgd_ov * locals.var_inv_xi_ov_dn6)) * assign24610_e24990) + (assign24610_e24985 * ((locals.var_sp_ov_afac_dn6 * locals.var_xgd_ov) + (locals.var_sp_ov_afac * locals.var_xgd_ov_dn6)))), ((((locals.var_xgd_ov_dn7 * locals.var_inv_xi_ov) + (locals.var_xgd_ov * locals.var_inv_xi_ov_dn7)) * assign24610_e24990) + (assign24610_e24985 * ((locals.var_sp_ov_afac_dn7 * locals.var_xgd_ov) + (locals.var_sp_ov_afac * locals.var_xgd_ov_dn7)))), ((((locals.var_xgd_ov_dn8 * locals.var_inv_xi_ov) + (locals.var_xgd_ov * locals.var_inv_xi_ov_dn8)) * assign24610_e24990) + (assign24610_e24985 * ((locals.var_sp_ov_afac_dn8 * locals.var_xgd_ov) + (locals.var_sp_ov_afac * locals.var_xgd_ov_dn8)))), ((((locals.var_xgd_ov_dn9 * locals.var_inv_xi_ov) + (locals.var_xgd_ov * locals.var_inv_xi_ov_dn9)) * assign24610_e24990) + (assign24610_e24985 * ((locals.var_sp_ov_afac_dn9 * locals.var_xgd_ov) + (locals.var_sp_ov_afac * locals.var_xgd_ov_dn9)))),)
    } else {
        (locals.var_sp_ov_xbar, locals.var_sp_ov_xbar_dn4, locals.var_sp_ov_xbar_dn6, locals.var_sp_ov_xbar_dn7, locals.var_sp_ov_xbar_dn8, locals.var_sp_ov_xbar_dn9,)
    }
};
        locals.var_sp_ov_xbar = assign24610_e24993;
        locals.var_sp_ov_xbar_dn4 = assign24610_e24993_d_n4;
        locals.var_sp_ov_xbar_dn6 = assign24610_e24993_d_n6;
        locals.var_sp_ov_xbar_dn7 = assign24610_e24993_d_n7;
        locals.var_sp_ov_xbar_dn8 = assign24610_e24993_d_n8;
        locals.var_sp_ov_xbar_dn9 = assign24610_e24993_d_n9;
        locals.var_sp_ov_xbar_rv = 0.0;

        let assign24620_e24995: f64 = (-locals.var_sp_ov_xbar);
        let assign24620_e24996: f64 = (assign24620_e24995).abs();
        let assign24620_e24998: f64 = if assign24620_e24996 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard708 = assign24620_e24998;
        locals.var_guard708_rv = 0.0;

        let (assign24630_e25012, assign24630_e25012_d_n4, assign24630_e25012_d_n6, assign24630_e25012_d_n7, assign24630_e25012_d_n8, assign24630_e25012_d_n9,) = {
    if ((((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 == 0.0)) && (locals.var_guard708 != 0.0)) {
        let assign24630_e25009: f64 = (-locals.var_sp_ov_xbar);
        let assign24630_e25010: f64 = (assign24630_e25009).exp();
        (assign24630_e25010, (assign24630_e25010 * (-locals.var_sp_ov_xbar_dn4)), (assign24630_e25010 * (-locals.var_sp_ov_xbar_dn6)), (assign24630_e25010 * (-locals.var_sp_ov_xbar_dn7)), (assign24630_e25010 * (-locals.var_sp_ov_xbar_dn8)), (assign24630_e25010 * (-locals.var_sp_ov_xbar_dn9)),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24630_e25012;
        locals.var_sp_ov_temp_dn4 = assign24630_e25012_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24630_e25012_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24630_e25012_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24630_e25012_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24630_e25012_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let assign24640_e25014: f64 = (-locals.var_sp_ov_xbar);
        let assign24640_e25016: f64 = (-80.0);
        let assign24640_e25017: f64 = if assign24640_e25014 < assign24640_e25016 { 1.0 } else { 0.0 };
        locals.var_guard709 = assign24640_e25017;
        locals.var_guard709_rv = 0.0;

        let (assign24650_e25060, assign24650_e25060_d_n4, assign24650_e25060_d_n6, assign24650_e25060_d_n7, assign24650_e25060_d_n8, assign24650_e25060_d_n9,) = {
    if (((((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 == 0.0)) && (locals.var_guard708 == 0.0)) && (locals.var_guard709 != 0.0)) {
        let assign24650_e25033: f64 = (-locals.var_sp_ov_xbar);
        let assign24650_e25034: f64 = (-assign24650_e25033);
        let assign24650_e25036: f64 = (assign24650_e25034 - 80.0);
        let assign24650_e25040: f64 = (-locals.var_sp_ov_xbar);
        let assign24650_e25041: f64 = (-assign24650_e25040);
        let assign24650_e25043: f64 = (assign24650_e25041 - 80.0);
        let assign24650_e25044: f64 = (0.5 * assign24650_e25043);
        let assign24650_e25047: f64 = (-locals.var_sp_ov_xbar);
        let assign24650_e25048: f64 = (-assign24650_e25047);
        let assign24650_e25050: f64 = (assign24650_e25048 - 80.0);
        let assign24650_e25052: f64 = (assign24650_e25050 * 0.3333333333333);
        let assign24650_e25053: f64 = (1.0 + assign24650_e25052);
        let assign24650_e25054: f64 = (assign24650_e25044 * assign24650_e25053);
        let assign24650_e25055: f64 = (1.0 + assign24650_e25054);
        let assign24650_e25056: f64 = (assign24650_e25036 * assign24650_e25055);
        let assign24650_e25057: f64 = (1.0 + assign24650_e25056);
        let assign24650_e25058: f64 = (1.80485e-35 / assign24650_e25057);
        (assign24650_e25058, (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn4)) * assign24650_e25055) + (assign24650_e25036 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn4))) * assign24650_e25053) + (assign24650_e25044 * ((-(-locals.var_sp_ov_xbar_dn4)) * 0.3333333333333)))))) / (assign24650_e25057 * assign24650_e25057))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn6)) * assign24650_e25055) + (assign24650_e25036 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn6))) * assign24650_e25053) + (assign24650_e25044 * ((-(-locals.var_sp_ov_xbar_dn6)) * 0.3333333333333)))))) / (assign24650_e25057 * assign24650_e25057))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn7)) * assign24650_e25055) + (assign24650_e25036 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn7))) * assign24650_e25053) + (assign24650_e25044 * ((-(-locals.var_sp_ov_xbar_dn7)) * 0.3333333333333)))))) / (assign24650_e25057 * assign24650_e25057))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn8)) * assign24650_e25055) + (assign24650_e25036 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn8))) * assign24650_e25053) + (assign24650_e25044 * ((-(-locals.var_sp_ov_xbar_dn8)) * 0.3333333333333)))))) / (assign24650_e25057 * assign24650_e25057))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn9)) * assign24650_e25055) + (assign24650_e25036 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn9))) * assign24650_e25053) + (assign24650_e25044 * ((-(-locals.var_sp_ov_xbar_dn9)) * 0.3333333333333)))))) / (assign24650_e25057 * assign24650_e25057))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24650_e25060;
        locals.var_sp_ov_temp_dn4 = assign24650_e25060_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24650_e25060_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24650_e25060_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24650_e25060_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24650_e25060_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign24660_e25101, assign24660_e25101_d_n4, assign24660_e25101_d_n6, assign24660_e25101_d_n7, assign24660_e25101_d_n8, assign24660_e25101_d_n9,) = {
    if (((((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 == 0.0)) && (locals.var_guard708 == 0.0)) && (locals.var_guard709 == 0.0)) {
        let assign24660_e25077: f64 = (-locals.var_sp_ov_xbar);
        let assign24660_e25079: f64 = (assign24660_e25077 - 80.0);
        let assign24660_e25083: f64 = (-locals.var_sp_ov_xbar);
        let assign24660_e25085: f64 = (assign24660_e25083 - 80.0);
        let assign24660_e25086: f64 = (0.5 * assign24660_e25085);
        let assign24660_e25089: f64 = (-locals.var_sp_ov_xbar);
        let assign24660_e25091: f64 = (assign24660_e25089 - 80.0);
        let assign24660_e25093: f64 = (assign24660_e25091 * 0.3333333333333);
        let assign24660_e25094: f64 = (1.0 + assign24660_e25093);
        let assign24660_e25095: f64 = (assign24660_e25086 * assign24660_e25094);
        let assign24660_e25096: f64 = (1.0 + assign24660_e25095);
        let assign24660_e25097: f64 = (assign24660_e25079 * assign24660_e25096);
        let assign24660_e25098: f64 = (1.0 + assign24660_e25097);
        let assign24660_e25099: f64 = (5.54062e34 * assign24660_e25098);
        (assign24660_e25099, (5.54062e34 * (((-locals.var_sp_ov_xbar_dn4) * assign24660_e25096) + (assign24660_e25079 * (((0.5 * (-locals.var_sp_ov_xbar_dn4)) * assign24660_e25094) + (assign24660_e25086 * ((-locals.var_sp_ov_xbar_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn6) * assign24660_e25096) + (assign24660_e25079 * (((0.5 * (-locals.var_sp_ov_xbar_dn6)) * assign24660_e25094) + (assign24660_e25086 * ((-locals.var_sp_ov_xbar_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn7) * assign24660_e25096) + (assign24660_e25079 * (((0.5 * (-locals.var_sp_ov_xbar_dn7)) * assign24660_e25094) + (assign24660_e25086 * ((-locals.var_sp_ov_xbar_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn8) * assign24660_e25096) + (assign24660_e25079 * (((0.5 * (-locals.var_sp_ov_xbar_dn8)) * assign24660_e25094) + (assign24660_e25086 * ((-locals.var_sp_ov_xbar_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn9) * assign24660_e25096) + (assign24660_e25079 * (((0.5 * (-locals.var_sp_ov_xbar_dn9)) * assign24660_e25094) + (assign24660_e25086 * ((-locals.var_sp_ov_xbar_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24660_e25101;
        locals.var_sp_ov_temp_dn4 = assign24660_e25101_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24660_e25101_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24660_e25101_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24660_e25101_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24660_e25101_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign24670_e25113, assign24670_e25113_d_n4, assign24670_e25113_d_n6, assign24670_e25113_d_n7, assign24670_e25113_d_n8, assign24670_e25113_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 == 0.0)) {
        let assign24670_e25111: f64 = (1.0 - locals.var_sp_ov_temp);
        (assign24670_e25111, (-locals.var_sp_ov_temp_dn4), (-locals.var_sp_ov_temp_dn6), (-locals.var_sp_ov_temp_dn7), (-locals.var_sp_ov_temp_dn8), (-locals.var_sp_ov_temp_dn9),)
    } else {
        (locals.var_sp_ov_w, locals.var_sp_ov_w_dn4, locals.var_sp_ov_w_dn6, locals.var_sp_ov_w_dn7, locals.var_sp_ov_w_dn8, locals.var_sp_ov_w_dn9,)
    }
};
        locals.var_sp_ov_w = assign24670_e25113;
        locals.var_sp_ov_w_dn4 = assign24670_e25113_d_n4;
        locals.var_sp_ov_w_dn6 = assign24670_e25113_d_n6;
        locals.var_sp_ov_w_dn7 = assign24670_e25113_d_n7;
        locals.var_sp_ov_w_dn8 = assign24670_e25113_d_n8;
        locals.var_sp_ov_w_dn9 = assign24670_e25113_d_n9;
        locals.var_sp_ov_w_rv = 0.0;

        let (assign24680_e25138, assign24680_e25138_d_n4, assign24680_e25138_d_n6, assign24680_e25138_d_n7, assign24680_e25138_d_n8, assign24680_e25138_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 == 0.0)) {
        let assign24680_e25124: f64 = (locals.var_gov2 * 0.5);
        let assign24680_e25125: f64 = (locals.var_xgd_ov + assign24680_e25124);
        let assign24680_e25130: f64 = (locals.var_gov2 * 0.25);
        let assign24680_e25131: f64 = (locals.var_xgd_ov + assign24680_e25130);
        let assign24680_e25133: f64 = (assign24680_e25131 - locals.var_sp_ov_w);
        let assign24680_e25134: f64 = (assign24680_e25133).sqrt();
        let assign24680_e25135: f64 = (locals.var_gov * assign24680_e25134);
        let assign24680_e25136: f64 = (assign24680_e25125 - assign24680_e25135);
        (assign24680_e25136, ((locals.var_xgd_ov_dn4 + (locals.var_gov2_dn4 * 0.5)) - ((locals.var_gov_dn4 * assign24680_e25134) + (locals.var_gov * (((locals.var_xgd_ov_dn4 + (locals.var_gov2_dn4 * 0.25)) - locals.var_sp_ov_w_dn4) / (2.0 * assign24680_e25134))))), ((locals.var_xgd_ov_dn6 + (locals.var_gov2_dn6 * 0.5)) - ((locals.var_gov_dn6 * assign24680_e25134) + (locals.var_gov * (((locals.var_xgd_ov_dn6 + (locals.var_gov2_dn6 * 0.25)) - locals.var_sp_ov_w_dn6) / (2.0 * assign24680_e25134))))), ((locals.var_xgd_ov_dn7 + (locals.var_gov2_dn7 * 0.5)) - ((locals.var_gov_dn7 * assign24680_e25134) + (locals.var_gov * (((locals.var_xgd_ov_dn7 + (locals.var_gov2_dn7 * 0.25)) - locals.var_sp_ov_w_dn7) / (2.0 * assign24680_e25134))))), ((locals.var_xgd_ov_dn8 + (locals.var_gov2_dn8 * 0.5)) - ((locals.var_gov_dn8 * assign24680_e25134) + (locals.var_gov * (((locals.var_xgd_ov_dn8 + (locals.var_gov2_dn8 * 0.25)) - locals.var_sp_ov_w_dn8) / (2.0 * assign24680_e25134))))), ((locals.var_xgd_ov_dn9 + (locals.var_gov2_dn9 * 0.5)) - ((locals.var_gov_dn9 * assign24680_e25134) + (locals.var_gov * (((locals.var_xgd_ov_dn9 + (locals.var_gov2_dn9 * 0.25)) - locals.var_sp_ov_w_dn9) / (2.0 * assign24680_e25134))))),)
    } else {
        (locals.var_sp_ov_x0, locals.var_sp_ov_x0_dn4, locals.var_sp_ov_x0_dn6, locals.var_sp_ov_x0_dn7, locals.var_sp_ov_x0_dn8, locals.var_sp_ov_x0_dn9,)
    }
};
        locals.var_sp_ov_x0 = assign24680_e25138;
        locals.var_sp_ov_x0_dn4 = assign24680_e25138_d_n4;
        locals.var_sp_ov_x0_dn6 = assign24680_e25138_d_n6;
        locals.var_sp_ov_x0_dn7 = assign24680_e25138_d_n7;
        locals.var_sp_ov_x0_dn8 = assign24680_e25138_d_n8;
        locals.var_sp_ov_x0_dn9 = assign24680_e25138_d_n9;
        locals.var_sp_ov_x0_rv = 0.0;

        let assign24690_e25140: f64 = (-locals.var_sp_ov_x0);
        let assign24690_e25141: f64 = (assign24690_e25140).abs();
        let assign24690_e25143: f64 = if assign24690_e25141 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard710 = assign24690_e25143;
        locals.var_guard710_rv = 0.0;

        let (assign24700_e25157, assign24700_e25157_d_n4, assign24700_e25157_d_n6, assign24700_e25157_d_n7, assign24700_e25157_d_n8, assign24700_e25157_d_n9,) = {
    if ((((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 == 0.0)) && (locals.var_guard710 != 0.0)) {
        let assign24700_e25154: f64 = (-locals.var_sp_ov_x0);
        let assign24700_e25155: f64 = (assign24700_e25154).exp();
        (assign24700_e25155, (assign24700_e25155 * (-locals.var_sp_ov_x0_dn4)), (assign24700_e25155 * (-locals.var_sp_ov_x0_dn6)), (assign24700_e25155 * (-locals.var_sp_ov_x0_dn7)), (assign24700_e25155 * (-locals.var_sp_ov_x0_dn8)), (assign24700_e25155 * (-locals.var_sp_ov_x0_dn9)),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign24700_e25157;
        locals.var_sp_ov_d0_dn4 = assign24700_e25157_d_n4;
        locals.var_sp_ov_d0_dn6 = assign24700_e25157_d_n6;
        locals.var_sp_ov_d0_dn7 = assign24700_e25157_d_n7;
        locals.var_sp_ov_d0_dn8 = assign24700_e25157_d_n8;
        locals.var_sp_ov_d0_dn9 = assign24700_e25157_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let assign24710_e25159: f64 = (-locals.var_sp_ov_x0);
        let assign24710_e25161: f64 = (-80.0);
        let assign24710_e25162: f64 = if assign24710_e25159 < assign24710_e25161 { 1.0 } else { 0.0 };
        locals.var_guard711 = assign24710_e25162;
        locals.var_guard711_rv = 0.0;

        let (assign24720_e25205, assign24720_e25205_d_n4, assign24720_e25205_d_n6, assign24720_e25205_d_n7, assign24720_e25205_d_n8, assign24720_e25205_d_n9,) = {
    if (((((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 == 0.0)) && (locals.var_guard710 == 0.0)) && (locals.var_guard711 != 0.0)) {
        let assign24720_e25178: f64 = (-locals.var_sp_ov_x0);
        let assign24720_e25179: f64 = (-assign24720_e25178);
        let assign24720_e25181: f64 = (assign24720_e25179 - 80.0);
        let assign24720_e25185: f64 = (-locals.var_sp_ov_x0);
        let assign24720_e25186: f64 = (-assign24720_e25185);
        let assign24720_e25188: f64 = (assign24720_e25186 - 80.0);
        let assign24720_e25189: f64 = (0.5 * assign24720_e25188);
        let assign24720_e25192: f64 = (-locals.var_sp_ov_x0);
        let assign24720_e25193: f64 = (-assign24720_e25192);
        let assign24720_e25195: f64 = (assign24720_e25193 - 80.0);
        let assign24720_e25197: f64 = (assign24720_e25195 * 0.3333333333333);
        let assign24720_e25198: f64 = (1.0 + assign24720_e25197);
        let assign24720_e25199: f64 = (assign24720_e25189 * assign24720_e25198);
        let assign24720_e25200: f64 = (1.0 + assign24720_e25199);
        let assign24720_e25201: f64 = (assign24720_e25181 * assign24720_e25200);
        let assign24720_e25202: f64 = (1.0 + assign24720_e25201);
        let assign24720_e25203: f64 = (1.80485e-35 / assign24720_e25202);
        (assign24720_e25203, (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn4)) * assign24720_e25200) + (assign24720_e25181 * (((0.5 * (-(-locals.var_sp_ov_x0_dn4))) * assign24720_e25198) + (assign24720_e25189 * ((-(-locals.var_sp_ov_x0_dn4)) * 0.3333333333333)))))) / (assign24720_e25202 * assign24720_e25202))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn6)) * assign24720_e25200) + (assign24720_e25181 * (((0.5 * (-(-locals.var_sp_ov_x0_dn6))) * assign24720_e25198) + (assign24720_e25189 * ((-(-locals.var_sp_ov_x0_dn6)) * 0.3333333333333)))))) / (assign24720_e25202 * assign24720_e25202))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn7)) * assign24720_e25200) + (assign24720_e25181 * (((0.5 * (-(-locals.var_sp_ov_x0_dn7))) * assign24720_e25198) + (assign24720_e25189 * ((-(-locals.var_sp_ov_x0_dn7)) * 0.3333333333333)))))) / (assign24720_e25202 * assign24720_e25202))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn8)) * assign24720_e25200) + (assign24720_e25181 * (((0.5 * (-(-locals.var_sp_ov_x0_dn8))) * assign24720_e25198) + (assign24720_e25189 * ((-(-locals.var_sp_ov_x0_dn8)) * 0.3333333333333)))))) / (assign24720_e25202 * assign24720_e25202))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn9)) * assign24720_e25200) + (assign24720_e25181 * (((0.5 * (-(-locals.var_sp_ov_x0_dn9))) * assign24720_e25198) + (assign24720_e25189 * ((-(-locals.var_sp_ov_x0_dn9)) * 0.3333333333333)))))) / (assign24720_e25202 * assign24720_e25202))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign24720_e25205;
        locals.var_sp_ov_d0_dn4 = assign24720_e25205_d_n4;
        locals.var_sp_ov_d0_dn6 = assign24720_e25205_d_n6;
        locals.var_sp_ov_d0_dn7 = assign24720_e25205_d_n7;
        locals.var_sp_ov_d0_dn8 = assign24720_e25205_d_n8;
        locals.var_sp_ov_d0_dn9 = assign24720_e25205_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign24730_e25246, assign24730_e25246_d_n4, assign24730_e25246_d_n6, assign24730_e25246_d_n7, assign24730_e25246_d_n8, assign24730_e25246_d_n9,) = {
    if (((((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 == 0.0)) && (locals.var_guard710 == 0.0)) && (locals.var_guard711 == 0.0)) {
        let assign24730_e25222: f64 = (-locals.var_sp_ov_x0);
        let assign24730_e25224: f64 = (assign24730_e25222 - 80.0);
        let assign24730_e25228: f64 = (-locals.var_sp_ov_x0);
        let assign24730_e25230: f64 = (assign24730_e25228 - 80.0);
        let assign24730_e25231: f64 = (0.5 * assign24730_e25230);
        let assign24730_e25234: f64 = (-locals.var_sp_ov_x0);
        let assign24730_e25236: f64 = (assign24730_e25234 - 80.0);
        let assign24730_e25238: f64 = (assign24730_e25236 * 0.3333333333333);
        let assign24730_e25239: f64 = (1.0 + assign24730_e25238);
        let assign24730_e25240: f64 = (assign24730_e25231 * assign24730_e25239);
        let assign24730_e25241: f64 = (1.0 + assign24730_e25240);
        let assign24730_e25242: f64 = (assign24730_e25224 * assign24730_e25241);
        let assign24730_e25243: f64 = (1.0 + assign24730_e25242);
        let assign24730_e25244: f64 = (5.54062e34 * assign24730_e25243);
        (assign24730_e25244, (5.54062e34 * (((-locals.var_sp_ov_x0_dn4) * assign24730_e25241) + (assign24730_e25224 * (((0.5 * (-locals.var_sp_ov_x0_dn4)) * assign24730_e25239) + (assign24730_e25231 * ((-locals.var_sp_ov_x0_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn6) * assign24730_e25241) + (assign24730_e25224 * (((0.5 * (-locals.var_sp_ov_x0_dn6)) * assign24730_e25239) + (assign24730_e25231 * ((-locals.var_sp_ov_x0_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn7) * assign24730_e25241) + (assign24730_e25224 * (((0.5 * (-locals.var_sp_ov_x0_dn7)) * assign24730_e25239) + (assign24730_e25231 * ((-locals.var_sp_ov_x0_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn8) * assign24730_e25241) + (assign24730_e25224 * (((0.5 * (-locals.var_sp_ov_x0_dn8)) * assign24730_e25239) + (assign24730_e25231 * ((-locals.var_sp_ov_x0_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn9) * assign24730_e25241) + (assign24730_e25224 * (((0.5 * (-locals.var_sp_ov_x0_dn9)) * assign24730_e25239) + (assign24730_e25231 * ((-locals.var_sp_ov_x0_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign24730_e25246;
        locals.var_sp_ov_d0_dn4 = assign24730_e25246_d_n4;
        locals.var_sp_ov_d0_dn6 = assign24730_e25246_d_n6;
        locals.var_sp_ov_d0_dn7 = assign24730_e25246_d_n7;
        locals.var_sp_ov_d0_dn8 = assign24730_e25246_d_n8;
        locals.var_sp_ov_d0_dn9 = assign24730_e25246_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign24740_e25266, assign24740_e25266_d_n4, assign24740_e25266_d_n6, assign24740_e25266_d_n7, assign24740_e25266_d_n8, assign24740_e25266_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 == 0.0)) {
        let assign24740_e25257: f64 = (locals.var_xgd_ov - locals.var_sp_ov_x0);
        let assign24740_e25258: f64 = (2.0 * assign24740_e25257);
        let assign24740_e25262: f64 = (1.0 - locals.var_sp_ov_d0);
        let assign24740_e25263: f64 = (locals.var_gov2 * assign24740_e25262);
        let assign24740_e25264: f64 = (assign24740_e25258 + assign24740_e25263);
        (assign24740_e25264, ((2.0 * (locals.var_xgd_ov_dn4 - locals.var_sp_ov_x0_dn4)) + ((locals.var_gov2_dn4 * assign24740_e25262) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn4)))), ((2.0 * (locals.var_xgd_ov_dn6 - locals.var_sp_ov_x0_dn6)) + ((locals.var_gov2_dn6 * assign24740_e25262) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn6)))), ((2.0 * (locals.var_xgd_ov_dn7 - locals.var_sp_ov_x0_dn7)) + ((locals.var_gov2_dn7 * assign24740_e25262) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn7)))), ((2.0 * (locals.var_xgd_ov_dn8 - locals.var_sp_ov_x0_dn8)) + ((locals.var_gov2_dn8 * assign24740_e25262) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn8)))), ((2.0 * (locals.var_xgd_ov_dn9 - locals.var_sp_ov_x0_dn9)) + ((locals.var_gov2_dn9 * assign24740_e25262) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_p, locals.var_sp_ov_p_dn4, locals.var_sp_ov_p_dn6, locals.var_sp_ov_p_dn7, locals.var_sp_ov_p_dn8, locals.var_sp_ov_p_dn9,)
    }
};
        locals.var_sp_ov_p = assign24740_e25266;
        locals.var_sp_ov_p_dn4 = assign24740_e25266_d_n4;
        locals.var_sp_ov_p_dn6 = assign24740_e25266_d_n6;
        locals.var_sp_ov_p_dn7 = assign24740_e25266_d_n7;
        locals.var_sp_ov_p_dn8 = assign24740_e25266_d_n8;
        locals.var_sp_ov_p_dn9 = assign24740_e25266_d_n9;
        locals.var_sp_ov_p_rv = 0.0;

        let (assign24750_e25290, assign24750_e25290_d_n4, assign24750_e25290_d_n6, assign24750_e25290_d_n7, assign24750_e25290_d_n8, assign24750_e25290_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 == 0.0)) {
        let assign24750_e25276: f64 = (locals.var_xgd_ov - locals.var_sp_ov_x0);
        let assign24750_e25279: f64 = (locals.var_xgd_ov - locals.var_sp_ov_x0);
        let assign24750_e25280: f64 = (assign24750_e25276 * assign24750_e25279);
        let assign24750_e25284: f64 = (locals.var_sp_ov_x0 - 1.0);
        let assign24750_e25286: f64 = (assign24750_e25284 + locals.var_sp_ov_d0);
        let assign24750_e25287: f64 = (locals.var_gov2 * assign24750_e25286);
        let assign24750_e25288: f64 = (assign24750_e25280 - assign24750_e25287);
        (assign24750_e25288, ((((locals.var_xgd_ov_dn4 - locals.var_sp_ov_x0_dn4) * assign24750_e25279) + (assign24750_e25276 * (locals.var_xgd_ov_dn4 - locals.var_sp_ov_x0_dn4))) - ((locals.var_gov2_dn4 * assign24750_e25286) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn4 + locals.var_sp_ov_d0_dn4)))), ((((locals.var_xgd_ov_dn6 - locals.var_sp_ov_x0_dn6) * assign24750_e25279) + (assign24750_e25276 * (locals.var_xgd_ov_dn6 - locals.var_sp_ov_x0_dn6))) - ((locals.var_gov2_dn6 * assign24750_e25286) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn6 + locals.var_sp_ov_d0_dn6)))), ((((locals.var_xgd_ov_dn7 - locals.var_sp_ov_x0_dn7) * assign24750_e25279) + (assign24750_e25276 * (locals.var_xgd_ov_dn7 - locals.var_sp_ov_x0_dn7))) - ((locals.var_gov2_dn7 * assign24750_e25286) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn7 + locals.var_sp_ov_d0_dn7)))), ((((locals.var_xgd_ov_dn8 - locals.var_sp_ov_x0_dn8) * assign24750_e25279) + (assign24750_e25276 * (locals.var_xgd_ov_dn8 - locals.var_sp_ov_x0_dn8))) - ((locals.var_gov2_dn8 * assign24750_e25286) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn8 + locals.var_sp_ov_d0_dn8)))), ((((locals.var_xgd_ov_dn9 - locals.var_sp_ov_x0_dn9) * assign24750_e25279) + (assign24750_e25276 * (locals.var_xgd_ov_dn9 - locals.var_sp_ov_x0_dn9))) - ((locals.var_gov2_dn9 * assign24750_e25286) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn9 + locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_q, locals.var_sp_ov_q_dn4, locals.var_sp_ov_q_dn6, locals.var_sp_ov_q_dn7, locals.var_sp_ov_q_dn8, locals.var_sp_ov_q_dn9,)
    }
};
        locals.var_sp_ov_q = assign24750_e25290;
        locals.var_sp_ov_q_dn4 = assign24750_e25290_d_n4;
        locals.var_sp_ov_q_dn6 = assign24750_e25290_d_n6;
        locals.var_sp_ov_q_dn7 = assign24750_e25290_d_n7;
        locals.var_sp_ov_q_dn8 = assign24750_e25290_d_n8;
        locals.var_sp_ov_q_dn9 = assign24750_e25290_d_n9;
        locals.var_sp_ov_q_rv = 0.0;

        let (assign24760_e25306, assign24760_e25306_d_n4, assign24760_e25306_d_n6, assign24760_e25306_d_n7, assign24760_e25306_d_n8, assign24760_e25306_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 == 0.0)) {
        let assign24760_e25301: f64 = (locals.var_gov2 * 0.5);
        let assign24760_e25303: f64 = (assign24760_e25301 * locals.var_sp_ov_d0);
        let assign24760_e25304: f64 = (1.0 - assign24760_e25303);
        (assign24760_e25304, (-(((locals.var_gov2_dn4 * 0.5) * locals.var_sp_ov_d0) + (assign24760_e25301 * locals.var_sp_ov_d0_dn4))), (-(((locals.var_gov2_dn6 * 0.5) * locals.var_sp_ov_d0) + (assign24760_e25301 * locals.var_sp_ov_d0_dn6))), (-(((locals.var_gov2_dn7 * 0.5) * locals.var_sp_ov_d0) + (assign24760_e25301 * locals.var_sp_ov_d0_dn7))), (-(((locals.var_gov2_dn8 * 0.5) * locals.var_sp_ov_d0) + (assign24760_e25301 * locals.var_sp_ov_d0_dn8))), (-(((locals.var_gov2_dn9 * 0.5) * locals.var_sp_ov_d0) + (assign24760_e25301 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_xi, locals.var_sp_ov_xi_dn4, locals.var_sp_ov_xi_dn6, locals.var_sp_ov_xi_dn7, locals.var_sp_ov_xi_dn8, locals.var_sp_ov_xi_dn9,)
    }
};
        locals.var_sp_ov_xi = assign24760_e25306;
        locals.var_sp_ov_xi_dn4 = assign24760_e25306_d_n4;
        locals.var_sp_ov_xi_dn6 = assign24760_e25306_d_n6;
        locals.var_sp_ov_xi_dn7 = assign24760_e25306_d_n7;
        locals.var_sp_ov_xi_dn8 = assign24760_e25306_d_n8;
        locals.var_sp_ov_xi_dn9 = assign24760_e25306_d_n9;
        locals.var_sp_ov_xi_rv = 0.0;

        let (assign24770_e25324, assign24770_e25324_d_n4, assign24770_e25324_d_n6, assign24770_e25324_d_n7, assign24770_e25324_d_n8, assign24770_e25324_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 == 0.0)) {
        let assign24770_e25316: f64 = (locals.var_sp_ov_p * locals.var_sp_ov_p);
        let assign24770_e25320: f64 = (locals.var_sp_ov_xi * locals.var_sp_ov_q);
        let assign24770_e25321: f64 = (4.0 * assign24770_e25320);
        let assign24770_e25322: f64 = (assign24770_e25316 - assign24770_e25321);
        (assign24770_e25322, (((locals.var_sp_ov_p_dn4 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn4)) - (4.0 * ((locals.var_sp_ov_xi_dn4 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn4)))), (((locals.var_sp_ov_p_dn6 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn6)) - (4.0 * ((locals.var_sp_ov_xi_dn6 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn6)))), (((locals.var_sp_ov_p_dn7 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn7)) - (4.0 * ((locals.var_sp_ov_xi_dn7 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn7)))), (((locals.var_sp_ov_p_dn8 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn8)) - (4.0 * ((locals.var_sp_ov_xi_dn8 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn8)))), (((locals.var_sp_ov_p_dn9 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn9)) - (4.0 * ((locals.var_sp_ov_xi_dn9 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24770_e25324;
        locals.var_sp_ov_temp_dn4 = assign24770_e25324_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24770_e25324_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24770_e25324_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24770_e25324_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24770_e25324_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign24780_e25341, assign24780_e25341_d_n4, assign24780_e25341_d_n6, assign24780_e25341_d_n7, assign24780_e25341_d_n8, assign24780_e25341_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 == 0.0)) {
        let assign24780_e25334: f64 = (2.0 * locals.var_sp_ov_q);
        let assign24780_e25337: f64 = (locals.var_sp_ov_temp).sqrt();
        let assign24780_e25338: f64 = (locals.var_sp_ov_p + assign24780_e25337);
        let assign24780_e25339: f64 = (assign24780_e25334 / assign24780_e25338);
        (assign24780_e25339, ((((2.0 * locals.var_sp_ov_q_dn4) * assign24780_e25338) - (assign24780_e25334 * (locals.var_sp_ov_p_dn4 + (locals.var_sp_ov_temp_dn4 / (2.0 * assign24780_e25337))))) / (assign24780_e25338 * assign24780_e25338)), ((((2.0 * locals.var_sp_ov_q_dn6) * assign24780_e25338) - (assign24780_e25334 * (locals.var_sp_ov_p_dn6 + (locals.var_sp_ov_temp_dn6 / (2.0 * assign24780_e25337))))) / (assign24780_e25338 * assign24780_e25338)), ((((2.0 * locals.var_sp_ov_q_dn7) * assign24780_e25338) - (assign24780_e25334 * (locals.var_sp_ov_p_dn7 + (locals.var_sp_ov_temp_dn7 / (2.0 * assign24780_e25337))))) / (assign24780_e25338 * assign24780_e25338)), ((((2.0 * locals.var_sp_ov_q_dn8) * assign24780_e25338) - (assign24780_e25334 * (locals.var_sp_ov_p_dn8 + (locals.var_sp_ov_temp_dn8 / (2.0 * assign24780_e25337))))) / (assign24780_e25338 * assign24780_e25338)), ((((2.0 * locals.var_sp_ov_q_dn9) * assign24780_e25338) - (assign24780_e25334 * (locals.var_sp_ov_p_dn9 + (locals.var_sp_ov_temp_dn9 / (2.0 * assign24780_e25337))))) / (assign24780_e25338 * assign24780_e25338)),)
    } else {
        (locals.var_sp_ov_u, locals.var_sp_ov_u_dn4, locals.var_sp_ov_u_dn6, locals.var_sp_ov_u_dn7, locals.var_sp_ov_u_dn8, locals.var_sp_ov_u_dn9,)
    }
};
        locals.var_sp_ov_u = assign24780_e25341;
        locals.var_sp_ov_u_dn4 = assign24780_e25341_d_n4;
        locals.var_sp_ov_u_dn6 = assign24780_e25341_d_n6;
        locals.var_sp_ov_u_dn7 = assign24780_e25341_d_n7;
        locals.var_sp_ov_u_dn8 = assign24780_e25341_d_n8;
        locals.var_sp_ov_u_dn9 = assign24780_e25341_d_n9;
        locals.var_sp_ov_u_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_68(
        locals: &mut StampLocals,
    ) {
        let (assign24790_e25353, assign24790_e25353_d_n4, assign24790_e25353_d_n6, assign24790_e25353_d_n7, assign24790_e25353_d_n8, assign24790_e25353_d_n9,) = {
    if (((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) && (locals.var_guard705 == 0.0)) {
        let assign24790_e25351: f64 = (locals.var_sp_ov_x0 + locals.var_sp_ov_u);
        (assign24790_e25351, (locals.var_sp_ov_x0_dn4 + locals.var_sp_ov_u_dn4), (locals.var_sp_ov_x0_dn6 + locals.var_sp_ov_u_dn6), (locals.var_sp_ov_x0_dn7 + locals.var_sp_ov_u_dn7), (locals.var_sp_ov_x0_dn8 + locals.var_sp_ov_u_dn8), (locals.var_sp_ov_x0_dn9 + locals.var_sp_ov_u_dn9),)
    } else {
        (locals.var_xd_ov, locals.var_xd_ov_dn4, locals.var_xd_ov_dn6, locals.var_xd_ov_dn7, locals.var_xd_ov_dn8, locals.var_xd_ov_dn9,)
    }
};
        locals.var_xd_ov = assign24790_e25353;
        locals.var_xd_ov_dn4 = assign24790_e25353_d_n4;
        locals.var_xd_ov_dn6 = assign24790_e25353_d_n6;
        locals.var_xd_ov_dn7 = assign24790_e25353_d_n7;
        locals.var_xd_ov_dn8 = assign24790_e25353_d_n8;
        locals.var_xd_ov_dn9 = assign24790_e25353_d_n9;
        locals.var_xd_ov_rv = 0.0;

        let (assign24800_e25361, assign24800_e25361_d_n4, assign24800_e25361_d_n6, assign24800_e25361_d_n7, assign24800_e25361_d_n8, assign24800_e25361_d_n9,) = {
    if ((locals.var_guard703 != 0.0) && (locals.var_guard704 == 0.0)) {
        let assign24800_e25359: f64 = (-locals.var_xd_ov);
        (assign24800_e25359, (-locals.var_xd_ov_dn4), (-locals.var_xd_ov_dn6), (-locals.var_xd_ov_dn7), (-locals.var_xd_ov_dn8), (-locals.var_xd_ov_dn9),)
    } else {
        (locals.var_xd_ov, locals.var_xd_ov_dn4, locals.var_xd_ov_dn6, locals.var_xd_ov_dn7, locals.var_xd_ov_dn8, locals.var_xd_ov_dn9,)
    }
};
        locals.var_xd_ov = assign24800_e25361;
        locals.var_xd_ov_dn4 = assign24800_e25361_d_n4;
        locals.var_xd_ov_dn6 = assign24800_e25361_d_n6;
        locals.var_xd_ov_dn7 = assign24800_e25361_d_n7;
        locals.var_xd_ov_dn8 = assign24800_e25361_d_n8;
        locals.var_xd_ov_dn9 = assign24800_e25361_d_n9;
        locals.var_xd_ov_rv = 0.0;

        let assign24810_e25364: f64 = if locals.var_covd_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard712 = assign24810_e25364;
        locals.var_guard712_rv = 0.0;

        let assign24820_e25366: f64 = (locals.var_xgd_ovcv).abs();
        let assign24820_e25368: f64 = if assign24820_e25366 <= locals.var_x_mrg_ov { 1.0 } else { 0.0 };
        locals.var_guard713 = assign24820_e25368;
        locals.var_guard713_rv = 0.0;

        let (assign24830_e25377, assign24830_e25377_d_n4, assign24830_e25377_d_n6, assign24830_e25377_d_n7, assign24830_e25377_d_n8, assign24830_e25377_d_n9,) = {
    if ((locals.var_guard712 != 0.0) && (locals.var_guard713 != 0.0)) {
        let assign24830_e25373: f64 = (-locals.var_xgd_ovcv);
        let assign24830_e25375: f64 = (assign24830_e25373 * locals.var_inv_xi_ov);
        (assign24830_e25375, (((-locals.var_xgd_ovcv_dn4) * locals.var_inv_xi_ov) + (assign24830_e25373 * locals.var_inv_xi_ov_dn4)), (((-locals.var_xgd_ovcv_dn6) * locals.var_inv_xi_ov) + (assign24830_e25373 * locals.var_inv_xi_ov_dn6)), (((-locals.var_xgd_ovcv_dn7) * locals.var_inv_xi_ov) + (assign24830_e25373 * locals.var_inv_xi_ov_dn7)), (((-locals.var_xgd_ovcv_dn8) * locals.var_inv_xi_ov) + (assign24830_e25373 * locals.var_inv_xi_ov_dn8)), (((-locals.var_xgd_ovcv_dn9) * locals.var_inv_xi_ov) + (assign24830_e25373 * locals.var_inv_xi_ov_dn9)),)
    } else {
        (locals.var_xd_ovcv, locals.var_xd_ovcv_dn4, locals.var_xd_ovcv_dn6, locals.var_xd_ovcv_dn7, locals.var_xd_ovcv_dn8, locals.var_xd_ovcv_dn9,)
    }
};
        locals.var_xd_ovcv = assign24830_e25377;
        locals.var_xd_ovcv_dn4 = assign24830_e25377_d_n4;
        locals.var_xd_ovcv_dn6 = assign24830_e25377_d_n6;
        locals.var_xd_ovcv_dn7 = assign24830_e25377_d_n7;
        locals.var_xd_ovcv_dn8 = assign24830_e25377_d_n8;
        locals.var_xd_ovcv_dn9 = assign24830_e25377_d_n9;
        locals.var_xd_ovcv_rv = 0.0;

        let assign24840_e25380: f64 = (-locals.var_x_mrg_ov);
        let assign24840_e25381: f64 = if locals.var_xgd_ovcv < assign24840_e25380 { 1.0 } else { 0.0 };
        locals.var_guard714 = assign24840_e25381;
        locals.var_guard714_rv = 0.0;

        let (assign24850_e25391, assign24850_e25391_d_n4, assign24850_e25391_d_n6, assign24850_e25391_d_n7, assign24850_e25391_d_n8, assign24850_e25391_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign24850_e25389: f64 = (-locals.var_xgd_ovcv);
        (assign24850_e25389, (-locals.var_xgd_ovcv_dn4), (-locals.var_xgd_ovcv_dn6), (-locals.var_xgd_ovcv_dn7), (-locals.var_xgd_ovcv_dn8), (-locals.var_xgd_ovcv_dn9),)
    } else {
        (locals.var_sp_ov_ygf, locals.var_sp_ov_ygf_dn4, locals.var_sp_ov_ygf_dn6, locals.var_sp_ov_ygf_dn7, locals.var_sp_ov_ygf_dn8, locals.var_sp_ov_ygf_dn9,)
    }
};
        locals.var_sp_ov_ygf = assign24850_e25391;
        locals.var_sp_ov_ygf_dn4 = assign24850_e25391_d_n4;
        locals.var_sp_ov_ygf_dn6 = assign24850_e25391_d_n6;
        locals.var_sp_ov_ygf_dn7 = assign24850_e25391_d_n7;
        locals.var_sp_ov_ygf_dn8 = assign24850_e25391_d_n8;
        locals.var_sp_ov_ygf_dn9 = assign24850_e25391_d_n9;
        locals.var_sp_ov_ygf_rv = 0.0;

        let (assign24860_e25404, assign24860_e25404_d_n4, assign24860_e25404_d_n6, assign24860_e25404_d_n7, assign24860_e25404_d_n8, assign24860_e25404_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign24860_e25400: f64 = (1.25 * locals.var_sp_ov_ygf);
        let assign24860_e25402: f64 = (assign24860_e25400 * locals.var_inv_xi_ov);
        (assign24860_e25402, (((1.25 * locals.var_sp_ov_ygf_dn4) * locals.var_inv_xi_ov) + (assign24860_e25400 * locals.var_inv_xi_ov_dn4)), (((1.25 * locals.var_sp_ov_ygf_dn6) * locals.var_inv_xi_ov) + (assign24860_e25400 * locals.var_inv_xi_ov_dn6)), (((1.25 * locals.var_sp_ov_ygf_dn7) * locals.var_inv_xi_ov) + (assign24860_e25400 * locals.var_inv_xi_ov_dn7)), (((1.25 * locals.var_sp_ov_ygf_dn8) * locals.var_inv_xi_ov) + (assign24860_e25400 * locals.var_inv_xi_ov_dn8)), (((1.25 * locals.var_sp_ov_ygf_dn9) * locals.var_inv_xi_ov) + (assign24860_e25400 * locals.var_inv_xi_ov_dn9)),)
    } else {
        (locals.var_sp_ov_z, locals.var_sp_ov_z_dn4, locals.var_sp_ov_z_dn6, locals.var_sp_ov_z_dn7, locals.var_sp_ov_z_dn8, locals.var_sp_ov_z_dn9,)
    }
};
        locals.var_sp_ov_z = assign24860_e25404;
        locals.var_sp_ov_z_dn4 = assign24860_e25404_d_n4;
        locals.var_sp_ov_z_dn6 = assign24860_e25404_d_n6;
        locals.var_sp_ov_z_dn7 = assign24860_e25404_d_n7;
        locals.var_sp_ov_z_dn8 = assign24860_e25404_d_n8;
        locals.var_sp_ov_z_dn9 = assign24860_e25404_d_n9;
        locals.var_sp_ov_z_rv = 0.0;

        let (assign24870_e25428, assign24870_e25428_d_n4, assign24870_e25428_d_n6, assign24870_e25428_d_n7, assign24870_e25428_d_n8, assign24870_e25428_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign24870_e25414: f64 = (locals.var_sp_ov_z + 10.0);
        let assign24870_e25417: f64 = (locals.var_sp_ov_z - 6.0);
        let assign24870_e25420: f64 = (locals.var_sp_ov_z - 6.0);
        let assign24870_e25421: f64 = (assign24870_e25417 * assign24870_e25420);
        let assign24870_e25423: f64 = (assign24870_e25421 + 64.0);
        let assign24870_e25424: f64 = (assign24870_e25423).sqrt();
        let assign24870_e25425: f64 = (assign24870_e25414 - assign24870_e25424);
        let assign24870_e25426: f64 = (0.5 * assign24870_e25425);
        (assign24870_e25426, (0.5 * (locals.var_sp_ov_z_dn4 - (((locals.var_sp_ov_z_dn4 * assign24870_e25420) + (assign24870_e25417 * locals.var_sp_ov_z_dn4)) / (2.0 * assign24870_e25424)))), (0.5 * (locals.var_sp_ov_z_dn6 - (((locals.var_sp_ov_z_dn6 * assign24870_e25420) + (assign24870_e25417 * locals.var_sp_ov_z_dn6)) / (2.0 * assign24870_e25424)))), (0.5 * (locals.var_sp_ov_z_dn7 - (((locals.var_sp_ov_z_dn7 * assign24870_e25420) + (assign24870_e25417 * locals.var_sp_ov_z_dn7)) / (2.0 * assign24870_e25424)))), (0.5 * (locals.var_sp_ov_z_dn8 - (((locals.var_sp_ov_z_dn8 * assign24870_e25420) + (assign24870_e25417 * locals.var_sp_ov_z_dn8)) / (2.0 * assign24870_e25424)))), (0.5 * (locals.var_sp_ov_z_dn9 - (((locals.var_sp_ov_z_dn9 * assign24870_e25420) + (assign24870_e25417 * locals.var_sp_ov_z_dn9)) / (2.0 * assign24870_e25424)))),)
    } else {
        (locals.var_sp_ov_eta, locals.var_sp_ov_eta_dn4, locals.var_sp_ov_eta_dn6, locals.var_sp_ov_eta_dn7, locals.var_sp_ov_eta_dn8, locals.var_sp_ov_eta_dn9,)
    }
};
        locals.var_sp_ov_eta = assign24870_e25428;
        locals.var_sp_ov_eta_dn4 = assign24870_e25428_d_n4;
        locals.var_sp_ov_eta_dn6 = assign24870_e25428_d_n6;
        locals.var_sp_ov_eta_dn7 = assign24870_e25428_d_n7;
        locals.var_sp_ov_eta_dn8 = assign24870_e25428_d_n8;
        locals.var_sp_ov_eta_dn9 = assign24870_e25428_d_n9;
        locals.var_sp_ov_eta_rv = 0.0;

        let (assign24880_e25449, assign24880_e25449_d_n4, assign24880_e25449_d_n6, assign24880_e25449_d_n7, assign24880_e25449_d_n8, assign24880_e25449_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign24880_e25437: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign24880_e25440: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign24880_e25441: f64 = (assign24880_e25437 * assign24880_e25440);
        let assign24880_e25445: f64 = (locals.var_sp_ov_eta + 1.0);
        let assign24880_e25446: f64 = (locals.var_gov2 * assign24880_e25445);
        let assign24880_e25447: f64 = (assign24880_e25441 + assign24880_e25446);
        (assign24880_e25447, ((((locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4) * assign24880_e25440) + (assign24880_e25437 * (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4))) + ((locals.var_gov2_dn4 * assign24880_e25445) + (locals.var_gov2 * locals.var_sp_ov_eta_dn4))), ((((locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6) * assign24880_e25440) + (assign24880_e25437 * (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6))) + ((locals.var_gov2_dn6 * assign24880_e25445) + (locals.var_gov2 * locals.var_sp_ov_eta_dn6))), ((((locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7) * assign24880_e25440) + (assign24880_e25437 * (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7))) + ((locals.var_gov2_dn7 * assign24880_e25445) + (locals.var_gov2 * locals.var_sp_ov_eta_dn7))), ((((locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8) * assign24880_e25440) + (assign24880_e25437 * (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8))) + ((locals.var_gov2_dn8 * assign24880_e25445) + (locals.var_gov2 * locals.var_sp_ov_eta_dn8))), ((((locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9) * assign24880_e25440) + (assign24880_e25437 * (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9))) + ((locals.var_gov2_dn9 * assign24880_e25445) + (locals.var_gov2 * locals.var_sp_ov_eta_dn9))),)
    } else {
        (locals.var_sp_ov_a, locals.var_sp_ov_a_dn4, locals.var_sp_ov_a_dn6, locals.var_sp_ov_a_dn7, locals.var_sp_ov_a_dn8, locals.var_sp_ov_a_dn9,)
    }
};
        locals.var_sp_ov_a = assign24880_e25449;
        locals.var_sp_ov_a_dn4 = assign24880_e25449_d_n4;
        locals.var_sp_ov_a_dn6 = assign24880_e25449_d_n6;
        locals.var_sp_ov_a_dn7 = assign24880_e25449_d_n7;
        locals.var_sp_ov_a_dn8 = assign24880_e25449_d_n8;
        locals.var_sp_ov_a_dn9 = assign24880_e25449_d_n9;
        locals.var_sp_ov_a_rv = 0.0;

        let (assign24890_e25464, assign24890_e25464_d_n4, assign24890_e25464_d_n6, assign24890_e25464_d_n7, assign24890_e25464_d_n8, assign24890_e25464_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign24890_e25459: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_eta);
        let assign24890_e25460: f64 = (2.0 * assign24890_e25459);
        let assign24890_e25462: f64 = (assign24890_e25460 - locals.var_gov2);
        (assign24890_e25462, ((2.0 * (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_eta_dn4)) - locals.var_gov2_dn4), ((2.0 * (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_eta_dn6)) - locals.var_gov2_dn6), ((2.0 * (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_eta_dn7)) - locals.var_gov2_dn7), ((2.0 * (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_eta_dn8)) - locals.var_gov2_dn8), ((2.0 * (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_eta_dn9)) - locals.var_gov2_dn9),)
    } else {
        (locals.var_sp_ov_c, locals.var_sp_ov_c_dn4, locals.var_sp_ov_c_dn6, locals.var_sp_ov_c_dn7, locals.var_sp_ov_c_dn8, locals.var_sp_ov_c_dn9,)
    }
};
        locals.var_sp_ov_c = assign24890_e25464;
        locals.var_sp_ov_c_dn4 = assign24890_e25464_d_n4;
        locals.var_sp_ov_c_dn6 = assign24890_e25464_d_n6;
        locals.var_sp_ov_c_dn7 = assign24890_e25464_d_n7;
        locals.var_sp_ov_c_dn8 = assign24890_e25464_d_n8;
        locals.var_sp_ov_c_dn9 = assign24890_e25464_d_n9;
        locals.var_sp_ov_c_rv = 0.0;

        let (assign24900_e25478, assign24900_e25478_d_n4, assign24900_e25478_d_n6, assign24900_e25478_d_n7, assign24900_e25478_d_n8, assign24900_e25478_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign24900_e25473: f64 = (locals.var_sp_ov_a / locals.var_gov2);
        let assign24900_e25474: f64 = (assign24900_e25473).ln();
        let assign24900_e25476: f64 = (assign24900_e25474 - locals.var_sp_ov_eta);
        (assign24900_e25476, (((((locals.var_sp_ov_a_dn4 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn4)) / (locals.var_gov2 * locals.var_gov2)) / assign24900_e25473) - locals.var_sp_ov_eta_dn4), (((((locals.var_sp_ov_a_dn6 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn6)) / (locals.var_gov2 * locals.var_gov2)) / assign24900_e25473) - locals.var_sp_ov_eta_dn6), (((((locals.var_sp_ov_a_dn7 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn7)) / (locals.var_gov2 * locals.var_gov2)) / assign24900_e25473) - locals.var_sp_ov_eta_dn7), (((((locals.var_sp_ov_a_dn8 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn8)) / (locals.var_gov2 * locals.var_gov2)) / assign24900_e25473) - locals.var_sp_ov_eta_dn8), (((((locals.var_sp_ov_a_dn9 * locals.var_gov2) - (locals.var_sp_ov_a * locals.var_gov2_dn9)) / (locals.var_gov2 * locals.var_gov2)) / assign24900_e25473) - locals.var_sp_ov_eta_dn9),)
    } else {
        (locals.var_sp_ov_tau, locals.var_sp_ov_tau_dn4, locals.var_sp_ov_tau_dn6, locals.var_sp_ov_tau_dn7, locals.var_sp_ov_tau_dn8, locals.var_sp_ov_tau_dn9,)
    }
};
        locals.var_sp_ov_tau = assign24900_e25478;
        locals.var_sp_ov_tau_dn4 = assign24900_e25478_d_n4;
        locals.var_sp_ov_tau_dn6 = assign24900_e25478_d_n6;
        locals.var_sp_ov_tau_dn7 = assign24900_e25478_d_n7;
        locals.var_sp_ov_tau_dn8 = assign24900_e25478_d_n8;
        locals.var_sp_ov_tau_dn9 = assign24900_e25478_d_n9;
        locals.var_sp_ov_tau_rv = 0.0;

        let (assign24910_e25489, assign24910_e25489_d_n4, assign24910_e25489_d_n6, assign24910_e25489_d_n7, assign24910_e25489_d_n8, assign24910_e25489_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign24910_e25487: f64 = (locals.var_sp_ov_a + locals.var_sp_ov_c);
        (assign24910_e25487, (locals.var_sp_ov_a_dn4 + locals.var_sp_ov_c_dn4), (locals.var_sp_ov_a_dn6 + locals.var_sp_ov_c_dn6), (locals.var_sp_ov_a_dn7 + locals.var_sp_ov_c_dn7), (locals.var_sp_ov_a_dn8 + locals.var_sp_ov_c_dn8), (locals.var_sp_ov_a_dn9 + locals.var_sp_ov_c_dn9),)
    } else {
        (locals.var_sp_ov_nu, locals.var_sp_ov_nu_dn4, locals.var_sp_ov_nu_dn6, locals.var_sp_ov_nu_dn7, locals.var_sp_ov_nu_dn8, locals.var_sp_ov_nu_dn9,)
    }
};
        locals.var_sp_ov_nu = assign24910_e25489;
        locals.var_sp_ov_nu_dn4 = assign24910_e25489_d_n4;
        locals.var_sp_ov_nu_dn6 = assign24910_e25489_d_n6;
        locals.var_sp_ov_nu_dn7 = assign24910_e25489_d_n7;
        locals.var_sp_ov_nu_dn8 = assign24910_e25489_d_n8;
        locals.var_sp_ov_nu_dn9 = assign24910_e25489_d_n9;
        locals.var_sp_ov_nu_rv = 0.0;

        let (assign24920_e25510, assign24920_e25510_d_n4, assign24920_e25510_d_n6, assign24920_e25510_d_n7, assign24920_e25510_d_n8, assign24920_e25510_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign24920_e25498: f64 = (locals.var_sp_ov_nu * locals.var_sp_ov_nu);
        let assign24920_e25502: f64 = (0.5 * locals.var_sp_ov_c);
        let assign24920_e25504: f64 = (assign24920_e25502 * locals.var_sp_ov_c);
        let assign24920_e25506: f64 = (assign24920_e25504 - locals.var_sp_ov_a);
        let assign24920_e25507: f64 = (locals.var_sp_ov_tau * assign24920_e25506);
        let assign24920_e25508: f64 = (assign24920_e25498 + assign24920_e25507);
        (assign24920_e25508, (((locals.var_sp_ov_nu_dn4 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn4)) + ((locals.var_sp_ov_tau_dn4 * assign24920_e25506) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn4) * locals.var_sp_ov_c) + (assign24920_e25502 * locals.var_sp_ov_c_dn4)) - locals.var_sp_ov_a_dn4)))), (((locals.var_sp_ov_nu_dn6 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn6)) + ((locals.var_sp_ov_tau_dn6 * assign24920_e25506) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn6) * locals.var_sp_ov_c) + (assign24920_e25502 * locals.var_sp_ov_c_dn6)) - locals.var_sp_ov_a_dn6)))), (((locals.var_sp_ov_nu_dn7 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn7)) + ((locals.var_sp_ov_tau_dn7 * assign24920_e25506) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn7) * locals.var_sp_ov_c) + (assign24920_e25502 * locals.var_sp_ov_c_dn7)) - locals.var_sp_ov_a_dn7)))), (((locals.var_sp_ov_nu_dn8 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn8)) + ((locals.var_sp_ov_tau_dn8 * assign24920_e25506) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn8) * locals.var_sp_ov_c) + (assign24920_e25502 * locals.var_sp_ov_c_dn8)) - locals.var_sp_ov_a_dn8)))), (((locals.var_sp_ov_nu_dn9 * locals.var_sp_ov_nu) + (locals.var_sp_ov_nu * locals.var_sp_ov_nu_dn9)) + ((locals.var_sp_ov_tau_dn9 * assign24920_e25506) + (locals.var_sp_ov_tau * ((((0.5 * locals.var_sp_ov_c_dn9) * locals.var_sp_ov_c) + (assign24920_e25502 * locals.var_sp_ov_c_dn9)) - locals.var_sp_ov_a_dn9)))),)
    } else {
        (locals.var_sp_ov_mutau, locals.var_sp_ov_mutau_dn4, locals.var_sp_ov_mutau_dn6, locals.var_sp_ov_mutau_dn7, locals.var_sp_ov_mutau_dn8, locals.var_sp_ov_mutau_dn9,)
    }
};
        locals.var_sp_ov_mutau = assign24920_e25510;
        locals.var_sp_ov_mutau_dn4 = assign24920_e25510_d_n4;
        locals.var_sp_ov_mutau_dn6 = assign24920_e25510_d_n6;
        locals.var_sp_ov_mutau_dn7 = assign24920_e25510_d_n7;
        locals.var_sp_ov_mutau_dn8 = assign24920_e25510_d_n8;
        locals.var_sp_ov_mutau_dn9 = assign24920_e25510_d_n9;
        locals.var_sp_ov_mutau_rv = 0.0;

        let (assign24930_e25537, assign24930_e25537_d_n4, assign24930_e25537_d_n6, assign24930_e25537_d_n7, assign24930_e25537_d_n8, assign24930_e25537_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign24930_e25520: f64 = (locals.var_sp_ov_nu / locals.var_sp_ov_mutau);
        let assign24930_e25522: f64 = (assign24930_e25520 * locals.var_sp_ov_tau);
        let assign24930_e25524: f64 = (assign24930_e25522 * locals.var_sp_ov_tau);
        let assign24930_e25526: f64 = (assign24930_e25524 * locals.var_sp_ov_c);
        let assign24930_e25529: f64 = (locals.var_sp_ov_c * locals.var_sp_ov_c);
        let assign24930_e25531: f64 = (assign24930_e25529 * 0.3333333333333);
        let assign24930_e25533: f64 = (assign24930_e25531 - locals.var_sp_ov_a);
        let assign24930_e25534: f64 = (assign24930_e25526 * assign24930_e25533);
        let assign24930_e25535: f64 = (locals.var_sp_ov_mutau + assign24930_e25534);
        (assign24930_e25535, (locals.var_sp_ov_mutau_dn4 + (((((((((((locals.var_sp_ov_nu_dn4 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn4)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign24930_e25520 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_tau) + (assign24930_e25522 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_c) + (assign24930_e25524 * locals.var_sp_ov_c_dn4)) * assign24930_e25533) + (assign24930_e25526 * ((((locals.var_sp_ov_c_dn4 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn4)) * 0.3333333333333) - locals.var_sp_ov_a_dn4)))), (locals.var_sp_ov_mutau_dn6 + (((((((((((locals.var_sp_ov_nu_dn6 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn6)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign24930_e25520 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_tau) + (assign24930_e25522 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_c) + (assign24930_e25524 * locals.var_sp_ov_c_dn6)) * assign24930_e25533) + (assign24930_e25526 * ((((locals.var_sp_ov_c_dn6 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn6)) * 0.3333333333333) - locals.var_sp_ov_a_dn6)))), (locals.var_sp_ov_mutau_dn7 + (((((((((((locals.var_sp_ov_nu_dn7 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn7)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign24930_e25520 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_tau) + (assign24930_e25522 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_c) + (assign24930_e25524 * locals.var_sp_ov_c_dn7)) * assign24930_e25533) + (assign24930_e25526 * ((((locals.var_sp_ov_c_dn7 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn7)) * 0.3333333333333) - locals.var_sp_ov_a_dn7)))), (locals.var_sp_ov_mutau_dn8 + (((((((((((locals.var_sp_ov_nu_dn8 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn8)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign24930_e25520 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_tau) + (assign24930_e25522 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_c) + (assign24930_e25524 * locals.var_sp_ov_c_dn8)) * assign24930_e25533) + (assign24930_e25526 * ((((locals.var_sp_ov_c_dn8 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn8)) * 0.3333333333333) - locals.var_sp_ov_a_dn8)))), (locals.var_sp_ov_mutau_dn9 + (((((((((((locals.var_sp_ov_nu_dn9 * locals.var_sp_ov_mutau) - (locals.var_sp_ov_nu * locals.var_sp_ov_mutau_dn9)) / (locals.var_sp_ov_mutau * locals.var_sp_ov_mutau)) * locals.var_sp_ov_tau) + (assign24930_e25520 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_tau) + (assign24930_e25522 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_c) + (assign24930_e25524 * locals.var_sp_ov_c_dn9)) * assign24930_e25533) + (assign24930_e25526 * ((((locals.var_sp_ov_c_dn9 * locals.var_sp_ov_c) + (locals.var_sp_ov_c * locals.var_sp_ov_c_dn9)) * 0.3333333333333) - locals.var_sp_ov_a_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign24930_e25537;
        locals.var_sp_ov_temp_dn4 = assign24930_e25537_d_n4;
        locals.var_sp_ov_temp_dn6 = assign24930_e25537_d_n6;
        locals.var_sp_ov_temp_dn7 = assign24930_e25537_d_n7;
        locals.var_sp_ov_temp_dn8 = assign24930_e25537_d_n8;
        locals.var_sp_ov_temp_dn9 = assign24930_e25537_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign24940_e25554, assign24940_e25554_d_n4, assign24940_e25554_d_n6, assign24940_e25554_d_n7, assign24940_e25554_d_n8, assign24940_e25554_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign24940_e25547: f64 = (locals.var_sp_ov_a * locals.var_sp_ov_nu);
        let assign24940_e25549: f64 = (assign24940_e25547 * locals.var_sp_ov_tau);
        let assign24940_e25551: f64 = (assign24940_e25549 / locals.var_sp_ov_temp);
        let assign24940_e25552: f64 = (locals.var_sp_ov_eta + assign24940_e25551);
        (assign24940_e25552, (locals.var_sp_ov_eta_dn4 + (((((((locals.var_sp_ov_a_dn4 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn4)) * locals.var_sp_ov_tau) + (assign24940_e25547 * locals.var_sp_ov_tau_dn4)) * locals.var_sp_ov_temp) - (assign24940_e25549 * locals.var_sp_ov_temp_dn4)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn6 + (((((((locals.var_sp_ov_a_dn6 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn6)) * locals.var_sp_ov_tau) + (assign24940_e25547 * locals.var_sp_ov_tau_dn6)) * locals.var_sp_ov_temp) - (assign24940_e25549 * locals.var_sp_ov_temp_dn6)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn7 + (((((((locals.var_sp_ov_a_dn7 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn7)) * locals.var_sp_ov_tau) + (assign24940_e25547 * locals.var_sp_ov_tau_dn7)) * locals.var_sp_ov_temp) - (assign24940_e25549 * locals.var_sp_ov_temp_dn7)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn8 + (((((((locals.var_sp_ov_a_dn8 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn8)) * locals.var_sp_ov_tau) + (assign24940_e25547 * locals.var_sp_ov_tau_dn8)) * locals.var_sp_ov_temp) - (assign24940_e25549 * locals.var_sp_ov_temp_dn8)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))), (locals.var_sp_ov_eta_dn9 + (((((((locals.var_sp_ov_a_dn9 * locals.var_sp_ov_nu) + (locals.var_sp_ov_a * locals.var_sp_ov_nu_dn9)) * locals.var_sp_ov_tau) + (assign24940_e25547 * locals.var_sp_ov_tau_dn9)) * locals.var_sp_ov_temp) - (assign24940_e25549 * locals.var_sp_ov_temp_dn9)) / (locals.var_sp_ov_temp * locals.var_sp_ov_temp))),)
    } else {
        (locals.var_sp_ov_y0, locals.var_sp_ov_y0_dn4, locals.var_sp_ov_y0_dn6, locals.var_sp_ov_y0_dn7, locals.var_sp_ov_y0_dn8, locals.var_sp_ov_y0_dn9,)
    }
};
        locals.var_sp_ov_y0 = assign24940_e25554;
        locals.var_sp_ov_y0_dn4 = assign24940_e25554_d_n4;
        locals.var_sp_ov_y0_dn6 = assign24940_e25554_d_n6;
        locals.var_sp_ov_y0_dn7 = assign24940_e25554_d_n7;
        locals.var_sp_ov_y0_dn8 = assign24940_e25554_d_n8;
        locals.var_sp_ov_y0_dn9 = assign24940_e25554_d_n9;
        locals.var_sp_ov_y0_rv = 0.0;

        let assign24950_e25556: f64 = (locals.var_sp_ov_y0).abs();
        let assign24950_e25558: f64 = if assign24950_e25556 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard715 = assign24950_e25558;
        locals.var_guard715_rv = 0.0;

        let (assign24960_e25570, assign24960_e25570_d_n4, assign24960_e25570_d_n6, assign24960_e25570_d_n7, assign24960_e25570_d_n8, assign24960_e25570_d_n9,) = {
    if ((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 != 0.0)) {
        let assign24960_e25568: f64 = (locals.var_sp_ov_y0).exp();
        (assign24960_e25568, (assign24960_e25568 * locals.var_sp_ov_y0_dn4), (assign24960_e25568 * locals.var_sp_ov_y0_dn6), (assign24960_e25568 * locals.var_sp_ov_y0_dn7), (assign24960_e25568 * locals.var_sp_ov_y0_dn8), (assign24960_e25568 * locals.var_sp_ov_y0_dn9),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign24960_e25570;
        locals.var_sp_ov_d0_dn4 = assign24960_e25570_d_n4;
        locals.var_sp_ov_d0_dn6 = assign24960_e25570_d_n6;
        locals.var_sp_ov_d0_dn7 = assign24960_e25570_d_n7;
        locals.var_sp_ov_d0_dn8 = assign24960_e25570_d_n8;
        locals.var_sp_ov_d0_dn9 = assign24960_e25570_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let assign24970_e25573: f64 = (-80.0);
        let assign24970_e25574: f64 = if locals.var_sp_ov_y0 < assign24970_e25573 { 1.0 } else { 0.0 };
        locals.var_guard716 = assign24970_e25574;
        locals.var_guard716_rv = 0.0;

        let (assign24980_e25613, assign24980_e25613_d_n4, assign24980_e25613_d_n6, assign24980_e25613_d_n7, assign24980_e25613_d_n8, assign24980_e25613_d_n9,) = {
    if (((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 != 0.0)) {
        let assign24980_e25589: f64 = (-locals.var_sp_ov_y0);
        let assign24980_e25591: f64 = (assign24980_e25589 - 80.0);
        let assign24980_e25595: f64 = (-locals.var_sp_ov_y0);
        let assign24980_e25597: f64 = (assign24980_e25595 - 80.0);
        let assign24980_e25598: f64 = (0.5 * assign24980_e25597);
        let assign24980_e25601: f64 = (-locals.var_sp_ov_y0);
        let assign24980_e25603: f64 = (assign24980_e25601 - 80.0);
        let assign24980_e25605: f64 = (assign24980_e25603 * 0.3333333333333);
        let assign24980_e25606: f64 = (1.0 + assign24980_e25605);
        let assign24980_e25607: f64 = (assign24980_e25598 * assign24980_e25606);
        let assign24980_e25608: f64 = (1.0 + assign24980_e25607);
        let assign24980_e25609: f64 = (assign24980_e25591 * assign24980_e25608);
        let assign24980_e25610: f64 = (1.0 + assign24980_e25609);
        let assign24980_e25611: f64 = (1.80485e-35 / assign24980_e25610);
        (assign24980_e25611, (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn4) * assign24980_e25608) + (assign24980_e25591 * (((0.5 * (-locals.var_sp_ov_y0_dn4)) * assign24980_e25606) + (assign24980_e25598 * ((-locals.var_sp_ov_y0_dn4) * 0.3333333333333)))))) / (assign24980_e25610 * assign24980_e25610))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn6) * assign24980_e25608) + (assign24980_e25591 * (((0.5 * (-locals.var_sp_ov_y0_dn6)) * assign24980_e25606) + (assign24980_e25598 * ((-locals.var_sp_ov_y0_dn6) * 0.3333333333333)))))) / (assign24980_e25610 * assign24980_e25610))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn7) * assign24980_e25608) + (assign24980_e25591 * (((0.5 * (-locals.var_sp_ov_y0_dn7)) * assign24980_e25606) + (assign24980_e25598 * ((-locals.var_sp_ov_y0_dn7) * 0.3333333333333)))))) / (assign24980_e25610 * assign24980_e25610))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn8) * assign24980_e25608) + (assign24980_e25591 * (((0.5 * (-locals.var_sp_ov_y0_dn8)) * assign24980_e25606) + (assign24980_e25598 * ((-locals.var_sp_ov_y0_dn8) * 0.3333333333333)))))) / (assign24980_e25610 * assign24980_e25610))), (-((1.80485e-35 * (((-locals.var_sp_ov_y0_dn9) * assign24980_e25608) + (assign24980_e25591 * (((0.5 * (-locals.var_sp_ov_y0_dn9)) * assign24980_e25606) + (assign24980_e25598 * ((-locals.var_sp_ov_y0_dn9) * 0.3333333333333)))))) / (assign24980_e25610 * assign24980_e25610))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign24980_e25613;
        locals.var_sp_ov_d0_dn4 = assign24980_e25613_d_n4;
        locals.var_sp_ov_d0_dn6 = assign24980_e25613_d_n6;
        locals.var_sp_ov_d0_dn7 = assign24980_e25613_d_n7;
        locals.var_sp_ov_d0_dn8 = assign24980_e25613_d_n8;
        locals.var_sp_ov_d0_dn9 = assign24980_e25613_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign24990_e25650, assign24990_e25650_d_n4, assign24990_e25650_d_n6, assign24990_e25650_d_n7, assign24990_e25650_d_n8, assign24990_e25650_d_n9,) = {
    if (((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) && (locals.var_guard715 == 0.0)) && (locals.var_guard716 == 0.0)) {
        let assign24990_e25630: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign24990_e25635: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign24990_e25636: f64 = (0.5 * assign24990_e25635);
        let assign24990_e25640: f64 = (locals.var_sp_ov_y0 - 80.0);
        let assign24990_e25642: f64 = (assign24990_e25640 * 0.3333333333333);
        let assign24990_e25643: f64 = (1.0 + assign24990_e25642);
        let assign24990_e25644: f64 = (assign24990_e25636 * assign24990_e25643);
        let assign24990_e25645: f64 = (1.0 + assign24990_e25644);
        let assign24990_e25646: f64 = (assign24990_e25630 * assign24990_e25645);
        let assign24990_e25647: f64 = (1.0 + assign24990_e25646);
        let assign24990_e25648: f64 = (5.54062e34 * assign24990_e25647);
        (assign24990_e25648, (5.54062e34 * ((locals.var_sp_ov_y0_dn4 * assign24990_e25645) + (assign24990_e25630 * (((0.5 * locals.var_sp_ov_y0_dn4) * assign24990_e25643) + (assign24990_e25636 * (locals.var_sp_ov_y0_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn6 * assign24990_e25645) + (assign24990_e25630 * (((0.5 * locals.var_sp_ov_y0_dn6) * assign24990_e25643) + (assign24990_e25636 * (locals.var_sp_ov_y0_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn7 * assign24990_e25645) + (assign24990_e25630 * (((0.5 * locals.var_sp_ov_y0_dn7) * assign24990_e25643) + (assign24990_e25636 * (locals.var_sp_ov_y0_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn8 * assign24990_e25645) + (assign24990_e25630 * (((0.5 * locals.var_sp_ov_y0_dn8) * assign24990_e25643) + (assign24990_e25636 * (locals.var_sp_ov_y0_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_sp_ov_y0_dn9 * assign24990_e25645) + (assign24990_e25630 * (((0.5 * locals.var_sp_ov_y0_dn9) * assign24990_e25643) + (assign24990_e25636 * (locals.var_sp_ov_y0_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign24990_e25650;
        locals.var_sp_ov_d0_dn4 = assign24990_e25650_d_n4;
        locals.var_sp_ov_d0_dn6 = assign24990_e25650_d_n6;
        locals.var_sp_ov_d0_dn7 = assign24990_e25650_d_n7;
        locals.var_sp_ov_d0_dn8 = assign24990_e25650_d_n8;
        locals.var_sp_ov_d0_dn9 = assign24990_e25650_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign25000_e25661, assign25000_e25661_d_n4, assign25000_e25661_d_n6, assign25000_e25661_d_n7, assign25000_e25661_d_n8, assign25000_e25661_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign25000_e25659: f64 = (locals.var_sp_ov_ygf - locals.var_sp_ov_y0);
        (assign25000_e25659, (locals.var_sp_ov_ygf_dn4 - locals.var_sp_ov_y0_dn4), (locals.var_sp_ov_ygf_dn6 - locals.var_sp_ov_y0_dn6), (locals.var_sp_ov_ygf_dn7 - locals.var_sp_ov_y0_dn7), (locals.var_sp_ov_ygf_dn8 - locals.var_sp_ov_y0_dn8), (locals.var_sp_ov_ygf_dn9 - locals.var_sp_ov_y0_dn9),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign25000_e25661;
        locals.var_sp_ov_temp_dn4 = assign25000_e25661_d_n4;
        locals.var_sp_ov_temp_dn6 = assign25000_e25661_d_n6;
        locals.var_sp_ov_temp_dn7 = assign25000_e25661_d_n7;
        locals.var_sp_ov_temp_dn8 = assign25000_e25661_d_n8;
        locals.var_sp_ov_temp_dn9 = assign25000_e25661_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign25010_e25678, assign25010_e25678_d_n4, assign25010_e25678_d_n6, assign25010_e25678_d_n7, assign25010_e25678_d_n8, assign25010_e25678_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign25010_e25670: f64 = (2.0 * locals.var_sp_ov_temp);
        let assign25010_e25674: f64 = (locals.var_sp_ov_d0 - 1.0);
        let assign25010_e25675: f64 = (locals.var_gov2 * assign25010_e25674);
        let assign25010_e25676: f64 = (assign25010_e25670 + assign25010_e25675);
        (assign25010_e25676, ((2.0 * locals.var_sp_ov_temp_dn4) + ((locals.var_gov2_dn4 * assign25010_e25674) + (locals.var_gov2 * locals.var_sp_ov_d0_dn4))), ((2.0 * locals.var_sp_ov_temp_dn6) + ((locals.var_gov2_dn6 * assign25010_e25674) + (locals.var_gov2 * locals.var_sp_ov_d0_dn6))), ((2.0 * locals.var_sp_ov_temp_dn7) + ((locals.var_gov2_dn7 * assign25010_e25674) + (locals.var_gov2 * locals.var_sp_ov_d0_dn7))), ((2.0 * locals.var_sp_ov_temp_dn8) + ((locals.var_gov2_dn8 * assign25010_e25674) + (locals.var_gov2 * locals.var_sp_ov_d0_dn8))), ((2.0 * locals.var_sp_ov_temp_dn9) + ((locals.var_gov2_dn9 * assign25010_e25674) + (locals.var_gov2 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_p, locals.var_sp_ov_p_dn4, locals.var_sp_ov_p_dn6, locals.var_sp_ov_p_dn7, locals.var_sp_ov_p_dn8, locals.var_sp_ov_p_dn9,)
    }
};
        locals.var_sp_ov_p = assign25010_e25678;
        locals.var_sp_ov_p_dn4 = assign25010_e25678_d_n4;
        locals.var_sp_ov_p_dn6 = assign25010_e25678_d_n6;
        locals.var_sp_ov_p_dn7 = assign25010_e25678_d_n7;
        locals.var_sp_ov_p_dn8 = assign25010_e25678_d_n8;
        locals.var_sp_ov_p_dn9 = assign25010_e25678_d_n9;
        locals.var_sp_ov_p_rv = 0.0;

        let (assign25020_e25697, assign25020_e25697_d_n4, assign25020_e25697_d_n6, assign25020_e25697_d_n7, assign25020_e25697_d_n8, assign25020_e25697_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign25020_e25687: f64 = (locals.var_sp_ov_temp * locals.var_sp_ov_temp);
        let assign25020_e25691: f64 = (locals.var_sp_ov_y0 + 1.0);
        let assign25020_e25693: f64 = (assign25020_e25691 - locals.var_sp_ov_d0);
        let assign25020_e25694: f64 = (locals.var_gov2 * assign25020_e25693);
        let assign25020_e25695: f64 = (assign25020_e25687 + assign25020_e25694);
        (assign25020_e25695, (((locals.var_sp_ov_temp_dn4 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn4)) + ((locals.var_gov2_dn4 * assign25020_e25693) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn4 - locals.var_sp_ov_d0_dn4)))), (((locals.var_sp_ov_temp_dn6 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn6)) + ((locals.var_gov2_dn6 * assign25020_e25693) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn6 - locals.var_sp_ov_d0_dn6)))), (((locals.var_sp_ov_temp_dn7 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn7)) + ((locals.var_gov2_dn7 * assign25020_e25693) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn7 - locals.var_sp_ov_d0_dn7)))), (((locals.var_sp_ov_temp_dn8 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn8)) + ((locals.var_gov2_dn8 * assign25020_e25693) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn8 - locals.var_sp_ov_d0_dn8)))), (((locals.var_sp_ov_temp_dn9 * locals.var_sp_ov_temp) + (locals.var_sp_ov_temp * locals.var_sp_ov_temp_dn9)) + ((locals.var_gov2_dn9 * assign25020_e25693) + (locals.var_gov2 * (locals.var_sp_ov_y0_dn9 - locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_q, locals.var_sp_ov_q_dn4, locals.var_sp_ov_q_dn6, locals.var_sp_ov_q_dn7, locals.var_sp_ov_q_dn8, locals.var_sp_ov_q_dn9,)
    }
};
        locals.var_sp_ov_q = assign25020_e25697;
        locals.var_sp_ov_q_dn4 = assign25020_e25697_d_n4;
        locals.var_sp_ov_q_dn6 = assign25020_e25697_d_n6;
        locals.var_sp_ov_q_dn7 = assign25020_e25697_d_n7;
        locals.var_sp_ov_q_dn8 = assign25020_e25697_d_n8;
        locals.var_sp_ov_q_dn9 = assign25020_e25697_d_n9;
        locals.var_sp_ov_q_rv = 0.0;

        let (assign25030_e25712, assign25030_e25712_d_n4, assign25030_e25712_d_n6, assign25030_e25712_d_n7, assign25030_e25712_d_n8, assign25030_e25712_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign25030_e25707: f64 = (locals.var_gov2 * 0.5);
        let assign25030_e25709: f64 = (assign25030_e25707 * locals.var_sp_ov_d0);
        let assign25030_e25710: f64 = (1.0 - assign25030_e25709);
        (assign25030_e25710, (-(((locals.var_gov2_dn4 * 0.5) * locals.var_sp_ov_d0) + (assign25030_e25707 * locals.var_sp_ov_d0_dn4))), (-(((locals.var_gov2_dn6 * 0.5) * locals.var_sp_ov_d0) + (assign25030_e25707 * locals.var_sp_ov_d0_dn6))), (-(((locals.var_gov2_dn7 * 0.5) * locals.var_sp_ov_d0) + (assign25030_e25707 * locals.var_sp_ov_d0_dn7))), (-(((locals.var_gov2_dn8 * 0.5) * locals.var_sp_ov_d0) + (assign25030_e25707 * locals.var_sp_ov_d0_dn8))), (-(((locals.var_gov2_dn9 * 0.5) * locals.var_sp_ov_d0) + (assign25030_e25707 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_xi, locals.var_sp_ov_xi_dn4, locals.var_sp_ov_xi_dn6, locals.var_sp_ov_xi_dn7, locals.var_sp_ov_xi_dn8, locals.var_sp_ov_xi_dn9,)
    }
};
        locals.var_sp_ov_xi = assign25030_e25712;
        locals.var_sp_ov_xi_dn4 = assign25030_e25712_d_n4;
        locals.var_sp_ov_xi_dn6 = assign25030_e25712_d_n6;
        locals.var_sp_ov_xi_dn7 = assign25030_e25712_d_n7;
        locals.var_sp_ov_xi_dn8 = assign25030_e25712_d_n8;
        locals.var_sp_ov_xi_dn9 = assign25030_e25712_d_n9;
        locals.var_sp_ov_xi_rv = 0.0;

        let (assign25040_e25729, assign25040_e25729_d_n4, assign25040_e25729_d_n6, assign25040_e25729_d_n7, assign25040_e25729_d_n8, assign25040_e25729_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign25040_e25721: f64 = (locals.var_sp_ov_p * locals.var_sp_ov_p);
        let assign25040_e25725: f64 = (locals.var_sp_ov_xi * locals.var_sp_ov_q);
        let assign25040_e25726: f64 = (4.0 * assign25040_e25725);
        let assign25040_e25727: f64 = (assign25040_e25721 - assign25040_e25726);
        (assign25040_e25727, (((locals.var_sp_ov_p_dn4 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn4)) - (4.0 * ((locals.var_sp_ov_xi_dn4 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn4)))), (((locals.var_sp_ov_p_dn6 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn6)) - (4.0 * ((locals.var_sp_ov_xi_dn6 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn6)))), (((locals.var_sp_ov_p_dn7 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn7)) - (4.0 * ((locals.var_sp_ov_xi_dn7 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn7)))), (((locals.var_sp_ov_p_dn8 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn8)) - (4.0 * ((locals.var_sp_ov_xi_dn8 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn8)))), (((locals.var_sp_ov_p_dn9 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn9)) - (4.0 * ((locals.var_sp_ov_xi_dn9 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign25040_e25729;
        locals.var_sp_ov_temp_dn4 = assign25040_e25729_d_n4;
        locals.var_sp_ov_temp_dn6 = assign25040_e25729_d_n6;
        locals.var_sp_ov_temp_dn7 = assign25040_e25729_d_n7;
        locals.var_sp_ov_temp_dn8 = assign25040_e25729_d_n8;
        locals.var_sp_ov_temp_dn9 = assign25040_e25729_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign25050_e25745, assign25050_e25745_d_n4, assign25050_e25745_d_n6, assign25050_e25745_d_n7, assign25050_e25745_d_n8, assign25050_e25745_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign25050_e25738: f64 = (2.0 * locals.var_sp_ov_q);
        let assign25050_e25741: f64 = (locals.var_sp_ov_temp).sqrt();
        let assign25050_e25742: f64 = (locals.var_sp_ov_p + assign25050_e25741);
        let assign25050_e25743: f64 = (assign25050_e25738 / assign25050_e25742);
        (assign25050_e25743, ((((2.0 * locals.var_sp_ov_q_dn4) * assign25050_e25742) - (assign25050_e25738 * (locals.var_sp_ov_p_dn4 + (locals.var_sp_ov_temp_dn4 / (2.0 * assign25050_e25741))))) / (assign25050_e25742 * assign25050_e25742)), ((((2.0 * locals.var_sp_ov_q_dn6) * assign25050_e25742) - (assign25050_e25738 * (locals.var_sp_ov_p_dn6 + (locals.var_sp_ov_temp_dn6 / (2.0 * assign25050_e25741))))) / (assign25050_e25742 * assign25050_e25742)), ((((2.0 * locals.var_sp_ov_q_dn7) * assign25050_e25742) - (assign25050_e25738 * (locals.var_sp_ov_p_dn7 + (locals.var_sp_ov_temp_dn7 / (2.0 * assign25050_e25741))))) / (assign25050_e25742 * assign25050_e25742)), ((((2.0 * locals.var_sp_ov_q_dn8) * assign25050_e25742) - (assign25050_e25738 * (locals.var_sp_ov_p_dn8 + (locals.var_sp_ov_temp_dn8 / (2.0 * assign25050_e25741))))) / (assign25050_e25742 * assign25050_e25742)), ((((2.0 * locals.var_sp_ov_q_dn9) * assign25050_e25742) - (assign25050_e25738 * (locals.var_sp_ov_p_dn9 + (locals.var_sp_ov_temp_dn9 / (2.0 * assign25050_e25741))))) / (assign25050_e25742 * assign25050_e25742)),)
    } else {
        (locals.var_sp_ov_w, locals.var_sp_ov_w_dn4, locals.var_sp_ov_w_dn6, locals.var_sp_ov_w_dn7, locals.var_sp_ov_w_dn8, locals.var_sp_ov_w_dn9,)
    }
};
        locals.var_sp_ov_w = assign25050_e25745;
        locals.var_sp_ov_w_dn4 = assign25050_e25745_d_n4;
        locals.var_sp_ov_w_dn6 = assign25050_e25745_d_n6;
        locals.var_sp_ov_w_dn7 = assign25050_e25745_d_n7;
        locals.var_sp_ov_w_dn8 = assign25050_e25745_d_n8;
        locals.var_sp_ov_w_dn9 = assign25050_e25745_d_n9;
        locals.var_sp_ov_w_rv = 0.0;

        let (assign25060_e25757, assign25060_e25757_d_n4, assign25060_e25757_d_n6, assign25060_e25757_d_n7, assign25060_e25757_d_n8, assign25060_e25757_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 != 0.0)) {
        let assign25060_e25754: f64 = (locals.var_sp_ov_y0 + locals.var_sp_ov_w);
        let assign25060_e25755: f64 = (-assign25060_e25754);
        (assign25060_e25755, (-(locals.var_sp_ov_y0_dn4 + locals.var_sp_ov_w_dn4)), (-(locals.var_sp_ov_y0_dn6 + locals.var_sp_ov_w_dn6)), (-(locals.var_sp_ov_y0_dn7 + locals.var_sp_ov_w_dn7)), (-(locals.var_sp_ov_y0_dn8 + locals.var_sp_ov_w_dn8)), (-(locals.var_sp_ov_y0_dn9 + locals.var_sp_ov_w_dn9)),)
    } else {
        (locals.var_xd_ovcv, locals.var_xd_ovcv_dn4, locals.var_xd_ovcv_dn6, locals.var_xd_ovcv_dn7, locals.var_xd_ovcv_dn8, locals.var_xd_ovcv_dn9,)
    }
};
        locals.var_xd_ovcv = assign25060_e25757;
        locals.var_xd_ovcv_dn4 = assign25060_e25757_d_n4;
        locals.var_xd_ovcv_dn6 = assign25060_e25757_d_n6;
        locals.var_xd_ovcv_dn7 = assign25060_e25757_d_n7;
        locals.var_xd_ovcv_dn8 = assign25060_e25757_d_n8;
        locals.var_xd_ovcv_dn9 = assign25060_e25757_d_n9;
        locals.var_xd_ovcv_rv = 0.0;

        let (assign25070_e25775, assign25070_e25775_d_n4, assign25070_e25775_d_n6, assign25070_e25775_d_n7, assign25070_e25775_d_n8, assign25070_e25775_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25070_e25767: f64 = (locals.var_xi_ov * 1.25);
        let assign25070_e25769: f64 = (assign25070_e25767 * locals.var_inv_xg1);
        let assign25070_e25771: f64 = (assign25070_e25769 - 1.0);
        let assign25070_e25773: f64 = (assign25070_e25771 * locals.var_inv_xg1);
        (assign25070_e25773, (((((locals.var_xi_ov_dn4 * 1.25) * locals.var_inv_xg1) + (assign25070_e25767 * locals.var_inv_xg1_dn4)) * locals.var_inv_xg1) + (assign25070_e25771 * locals.var_inv_xg1_dn4)), (((((locals.var_xi_ov_dn6 * 1.25) * locals.var_inv_xg1) + (assign25070_e25767 * locals.var_inv_xg1_dn6)) * locals.var_inv_xg1) + (assign25070_e25771 * locals.var_inv_xg1_dn6)), (((((locals.var_xi_ov_dn7 * 1.25) * locals.var_inv_xg1) + (assign25070_e25767 * locals.var_inv_xg1_dn7)) * locals.var_inv_xg1) + (assign25070_e25771 * locals.var_inv_xg1_dn7)), (((((locals.var_xi_ov_dn8 * 1.25) * locals.var_inv_xg1) + (assign25070_e25767 * locals.var_inv_xg1_dn8)) * locals.var_inv_xg1) + (assign25070_e25771 * locals.var_inv_xg1_dn8)), (((((locals.var_xi_ov_dn9 * 1.25) * locals.var_inv_xg1) + (assign25070_e25767 * locals.var_inv_xg1_dn9)) * locals.var_inv_xg1) + (assign25070_e25771 * locals.var_inv_xg1_dn9)),)
    } else {
        (locals.var_sp_ov_afac, locals.var_sp_ov_afac_dn4, locals.var_sp_ov_afac_dn6, locals.var_sp_ov_afac_dn7, locals.var_sp_ov_afac_dn8, locals.var_sp_ov_afac_dn9,)
    }
};
        locals.var_sp_ov_afac = assign25070_e25775;
        locals.var_sp_ov_afac_dn4 = assign25070_e25775_d_n4;
        locals.var_sp_ov_afac_dn6 = assign25070_e25775_d_n6;
        locals.var_sp_ov_afac_dn7 = assign25070_e25775_d_n7;
        locals.var_sp_ov_afac_dn8 = assign25070_e25775_d_n8;
        locals.var_sp_ov_afac_dn9 = assign25070_e25775_d_n9;
        locals.var_sp_ov_afac_rv = 0.0;

        let (assign25080_e25793, assign25080_e25793_d_n4, assign25080_e25793_d_n6, assign25080_e25793_d_n7, assign25080_e25793_d_n8, assign25080_e25793_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25080_e25785: f64 = (locals.var_xgd_ovcv * locals.var_inv_xi_ov);
        let assign25080_e25789: f64 = (locals.var_sp_ov_afac * locals.var_xgd_ovcv);
        let assign25080_e25790: f64 = (1.0 + assign25080_e25789);
        let assign25080_e25791: f64 = (assign25080_e25785 * assign25080_e25790);
        (assign25080_e25791, ((((locals.var_xgd_ovcv_dn4 * locals.var_inv_xi_ov) + (locals.var_xgd_ovcv * locals.var_inv_xi_ov_dn4)) * assign25080_e25790) + (assign25080_e25785 * ((locals.var_sp_ov_afac_dn4 * locals.var_xgd_ovcv) + (locals.var_sp_ov_afac * locals.var_xgd_ovcv_dn4)))), ((((locals.var_xgd_ovcv_dn6 * locals.var_inv_xi_ov) + (locals.var_xgd_ovcv * locals.var_inv_xi_ov_dn6)) * assign25080_e25790) + (assign25080_e25785 * ((locals.var_sp_ov_afac_dn6 * locals.var_xgd_ovcv) + (locals.var_sp_ov_afac * locals.var_xgd_ovcv_dn6)))), ((((locals.var_xgd_ovcv_dn7 * locals.var_inv_xi_ov) + (locals.var_xgd_ovcv * locals.var_inv_xi_ov_dn7)) * assign25080_e25790) + (assign25080_e25785 * ((locals.var_sp_ov_afac_dn7 * locals.var_xgd_ovcv) + (locals.var_sp_ov_afac * locals.var_xgd_ovcv_dn7)))), ((((locals.var_xgd_ovcv_dn8 * locals.var_inv_xi_ov) + (locals.var_xgd_ovcv * locals.var_inv_xi_ov_dn8)) * assign25080_e25790) + (assign25080_e25785 * ((locals.var_sp_ov_afac_dn8 * locals.var_xgd_ovcv) + (locals.var_sp_ov_afac * locals.var_xgd_ovcv_dn8)))), ((((locals.var_xgd_ovcv_dn9 * locals.var_inv_xi_ov) + (locals.var_xgd_ovcv * locals.var_inv_xi_ov_dn9)) * assign25080_e25790) + (assign25080_e25785 * ((locals.var_sp_ov_afac_dn9 * locals.var_xgd_ovcv) + (locals.var_sp_ov_afac * locals.var_xgd_ovcv_dn9)))),)
    } else {
        (locals.var_sp_ov_xbar, locals.var_sp_ov_xbar_dn4, locals.var_sp_ov_xbar_dn6, locals.var_sp_ov_xbar_dn7, locals.var_sp_ov_xbar_dn8, locals.var_sp_ov_xbar_dn9,)
    }
};
        locals.var_sp_ov_xbar = assign25080_e25793;
        locals.var_sp_ov_xbar_dn4 = assign25080_e25793_d_n4;
        locals.var_sp_ov_xbar_dn6 = assign25080_e25793_d_n6;
        locals.var_sp_ov_xbar_dn7 = assign25080_e25793_d_n7;
        locals.var_sp_ov_xbar_dn8 = assign25080_e25793_d_n8;
        locals.var_sp_ov_xbar_dn9 = assign25080_e25793_d_n9;
        locals.var_sp_ov_xbar_rv = 0.0;

        let assign25090_e25795: f64 = (-locals.var_sp_ov_xbar);
        let assign25090_e25796: f64 = (assign25090_e25795).abs();
        let assign25090_e25798: f64 = if assign25090_e25796 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard717 = assign25090_e25798;
        locals.var_guard717_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_69(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25100_e25812, assign25100_e25812_d_n4, assign25100_e25812_d_n6, assign25100_e25812_d_n7, assign25100_e25812_d_n8, assign25100_e25812_d_n9,) = {
    if ((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) && (locals.var_guard717 != 0.0)) {
        let assign25100_e25809: f64 = (-locals.var_sp_ov_xbar);
        let assign25100_e25810: f64 = (assign25100_e25809).exp();
        (assign25100_e25810, (assign25100_e25810 * (-locals.var_sp_ov_xbar_dn4)), (assign25100_e25810 * (-locals.var_sp_ov_xbar_dn6)), (assign25100_e25810 * (-locals.var_sp_ov_xbar_dn7)), (assign25100_e25810 * (-locals.var_sp_ov_xbar_dn8)), (assign25100_e25810 * (-locals.var_sp_ov_xbar_dn9)),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign25100_e25812;
        locals.var_sp_ov_temp_dn4 = assign25100_e25812_d_n4;
        locals.var_sp_ov_temp_dn6 = assign25100_e25812_d_n6;
        locals.var_sp_ov_temp_dn7 = assign25100_e25812_d_n7;
        locals.var_sp_ov_temp_dn8 = assign25100_e25812_d_n8;
        locals.var_sp_ov_temp_dn9 = assign25100_e25812_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let assign25110_e25814: f64 = (-locals.var_sp_ov_xbar);
        let assign25110_e25816: f64 = (-80.0);
        let assign25110_e25817: f64 = if assign25110_e25814 < assign25110_e25816 { 1.0 } else { 0.0 };
        locals.var_guard718 = assign25110_e25817;
        locals.var_guard718_rv = 0.0;

        let (assign25120_e25860, assign25120_e25860_d_n4, assign25120_e25860_d_n6, assign25120_e25860_d_n7, assign25120_e25860_d_n8, assign25120_e25860_d_n9,) = {
    if (((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) && (locals.var_guard717 == 0.0)) && (locals.var_guard718 != 0.0)) {
        let assign25120_e25833: f64 = (-locals.var_sp_ov_xbar);
        let assign25120_e25834: f64 = (-assign25120_e25833);
        let assign25120_e25836: f64 = (assign25120_e25834 - 80.0);
        let assign25120_e25840: f64 = (-locals.var_sp_ov_xbar);
        let assign25120_e25841: f64 = (-assign25120_e25840);
        let assign25120_e25843: f64 = (assign25120_e25841 - 80.0);
        let assign25120_e25844: f64 = (0.5 * assign25120_e25843);
        let assign25120_e25847: f64 = (-locals.var_sp_ov_xbar);
        let assign25120_e25848: f64 = (-assign25120_e25847);
        let assign25120_e25850: f64 = (assign25120_e25848 - 80.0);
        let assign25120_e25852: f64 = (assign25120_e25850 * 0.3333333333333);
        let assign25120_e25853: f64 = (1.0 + assign25120_e25852);
        let assign25120_e25854: f64 = (assign25120_e25844 * assign25120_e25853);
        let assign25120_e25855: f64 = (1.0 + assign25120_e25854);
        let assign25120_e25856: f64 = (assign25120_e25836 * assign25120_e25855);
        let assign25120_e25857: f64 = (1.0 + assign25120_e25856);
        let assign25120_e25858: f64 = (1.80485e-35 / assign25120_e25857);
        (assign25120_e25858, (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn4)) * assign25120_e25855) + (assign25120_e25836 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn4))) * assign25120_e25853) + (assign25120_e25844 * ((-(-locals.var_sp_ov_xbar_dn4)) * 0.3333333333333)))))) / (assign25120_e25857 * assign25120_e25857))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn6)) * assign25120_e25855) + (assign25120_e25836 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn6))) * assign25120_e25853) + (assign25120_e25844 * ((-(-locals.var_sp_ov_xbar_dn6)) * 0.3333333333333)))))) / (assign25120_e25857 * assign25120_e25857))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn7)) * assign25120_e25855) + (assign25120_e25836 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn7))) * assign25120_e25853) + (assign25120_e25844 * ((-(-locals.var_sp_ov_xbar_dn7)) * 0.3333333333333)))))) / (assign25120_e25857 * assign25120_e25857))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn8)) * assign25120_e25855) + (assign25120_e25836 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn8))) * assign25120_e25853) + (assign25120_e25844 * ((-(-locals.var_sp_ov_xbar_dn8)) * 0.3333333333333)))))) / (assign25120_e25857 * assign25120_e25857))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_xbar_dn9)) * assign25120_e25855) + (assign25120_e25836 * (((0.5 * (-(-locals.var_sp_ov_xbar_dn9))) * assign25120_e25853) + (assign25120_e25844 * ((-(-locals.var_sp_ov_xbar_dn9)) * 0.3333333333333)))))) / (assign25120_e25857 * assign25120_e25857))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign25120_e25860;
        locals.var_sp_ov_temp_dn4 = assign25120_e25860_d_n4;
        locals.var_sp_ov_temp_dn6 = assign25120_e25860_d_n6;
        locals.var_sp_ov_temp_dn7 = assign25120_e25860_d_n7;
        locals.var_sp_ov_temp_dn8 = assign25120_e25860_d_n8;
        locals.var_sp_ov_temp_dn9 = assign25120_e25860_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign25130_e25901, assign25130_e25901_d_n4, assign25130_e25901_d_n6, assign25130_e25901_d_n7, assign25130_e25901_d_n8, assign25130_e25901_d_n9,) = {
    if (((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) && (locals.var_guard717 == 0.0)) && (locals.var_guard718 == 0.0)) {
        let assign25130_e25877: f64 = (-locals.var_sp_ov_xbar);
        let assign25130_e25879: f64 = (assign25130_e25877 - 80.0);
        let assign25130_e25883: f64 = (-locals.var_sp_ov_xbar);
        let assign25130_e25885: f64 = (assign25130_e25883 - 80.0);
        let assign25130_e25886: f64 = (0.5 * assign25130_e25885);
        let assign25130_e25889: f64 = (-locals.var_sp_ov_xbar);
        let assign25130_e25891: f64 = (assign25130_e25889 - 80.0);
        let assign25130_e25893: f64 = (assign25130_e25891 * 0.3333333333333);
        let assign25130_e25894: f64 = (1.0 + assign25130_e25893);
        let assign25130_e25895: f64 = (assign25130_e25886 * assign25130_e25894);
        let assign25130_e25896: f64 = (1.0 + assign25130_e25895);
        let assign25130_e25897: f64 = (assign25130_e25879 * assign25130_e25896);
        let assign25130_e25898: f64 = (1.0 + assign25130_e25897);
        let assign25130_e25899: f64 = (5.54062e34 * assign25130_e25898);
        (assign25130_e25899, (5.54062e34 * (((-locals.var_sp_ov_xbar_dn4) * assign25130_e25896) + (assign25130_e25879 * (((0.5 * (-locals.var_sp_ov_xbar_dn4)) * assign25130_e25894) + (assign25130_e25886 * ((-locals.var_sp_ov_xbar_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn6) * assign25130_e25896) + (assign25130_e25879 * (((0.5 * (-locals.var_sp_ov_xbar_dn6)) * assign25130_e25894) + (assign25130_e25886 * ((-locals.var_sp_ov_xbar_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn7) * assign25130_e25896) + (assign25130_e25879 * (((0.5 * (-locals.var_sp_ov_xbar_dn7)) * assign25130_e25894) + (assign25130_e25886 * ((-locals.var_sp_ov_xbar_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn8) * assign25130_e25896) + (assign25130_e25879 * (((0.5 * (-locals.var_sp_ov_xbar_dn8)) * assign25130_e25894) + (assign25130_e25886 * ((-locals.var_sp_ov_xbar_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_xbar_dn9) * assign25130_e25896) + (assign25130_e25879 * (((0.5 * (-locals.var_sp_ov_xbar_dn9)) * assign25130_e25894) + (assign25130_e25886 * ((-locals.var_sp_ov_xbar_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign25130_e25901;
        locals.var_sp_ov_temp_dn4 = assign25130_e25901_d_n4;
        locals.var_sp_ov_temp_dn6 = assign25130_e25901_d_n6;
        locals.var_sp_ov_temp_dn7 = assign25130_e25901_d_n7;
        locals.var_sp_ov_temp_dn8 = assign25130_e25901_d_n8;
        locals.var_sp_ov_temp_dn9 = assign25130_e25901_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign25140_e25913, assign25140_e25913_d_n4, assign25140_e25913_d_n6, assign25140_e25913_d_n7, assign25140_e25913_d_n8, assign25140_e25913_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25140_e25911: f64 = (1.0 - locals.var_sp_ov_temp);
        (assign25140_e25911, (-locals.var_sp_ov_temp_dn4), (-locals.var_sp_ov_temp_dn6), (-locals.var_sp_ov_temp_dn7), (-locals.var_sp_ov_temp_dn8), (-locals.var_sp_ov_temp_dn9),)
    } else {
        (locals.var_sp_ov_w, locals.var_sp_ov_w_dn4, locals.var_sp_ov_w_dn6, locals.var_sp_ov_w_dn7, locals.var_sp_ov_w_dn8, locals.var_sp_ov_w_dn9,)
    }
};
        locals.var_sp_ov_w = assign25140_e25913;
        locals.var_sp_ov_w_dn4 = assign25140_e25913_d_n4;
        locals.var_sp_ov_w_dn6 = assign25140_e25913_d_n6;
        locals.var_sp_ov_w_dn7 = assign25140_e25913_d_n7;
        locals.var_sp_ov_w_dn8 = assign25140_e25913_d_n8;
        locals.var_sp_ov_w_dn9 = assign25140_e25913_d_n9;
        locals.var_sp_ov_w_rv = 0.0;

        let (assign25150_e25938, assign25150_e25938_d_n4, assign25150_e25938_d_n6, assign25150_e25938_d_n7, assign25150_e25938_d_n8, assign25150_e25938_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25150_e25924: f64 = (locals.var_gov2 * 0.5);
        let assign25150_e25925: f64 = (locals.var_xgd_ovcv + assign25150_e25924);
        let assign25150_e25930: f64 = (locals.var_gov2 * 0.25);
        let assign25150_e25931: f64 = (locals.var_xgd_ovcv + assign25150_e25930);
        let assign25150_e25933: f64 = (assign25150_e25931 - locals.var_sp_ov_w);
        let assign25150_e25934: f64 = (assign25150_e25933).sqrt();
        let assign25150_e25935: f64 = (locals.var_gov * assign25150_e25934);
        let assign25150_e25936: f64 = (assign25150_e25925 - assign25150_e25935);
        (assign25150_e25936, ((locals.var_xgd_ovcv_dn4 + (locals.var_gov2_dn4 * 0.5)) - ((locals.var_gov_dn4 * assign25150_e25934) + (locals.var_gov * (((locals.var_xgd_ovcv_dn4 + (locals.var_gov2_dn4 * 0.25)) - locals.var_sp_ov_w_dn4) / (2.0 * assign25150_e25934))))), ((locals.var_xgd_ovcv_dn6 + (locals.var_gov2_dn6 * 0.5)) - ((locals.var_gov_dn6 * assign25150_e25934) + (locals.var_gov * (((locals.var_xgd_ovcv_dn6 + (locals.var_gov2_dn6 * 0.25)) - locals.var_sp_ov_w_dn6) / (2.0 * assign25150_e25934))))), ((locals.var_xgd_ovcv_dn7 + (locals.var_gov2_dn7 * 0.5)) - ((locals.var_gov_dn7 * assign25150_e25934) + (locals.var_gov * (((locals.var_xgd_ovcv_dn7 + (locals.var_gov2_dn7 * 0.25)) - locals.var_sp_ov_w_dn7) / (2.0 * assign25150_e25934))))), ((locals.var_xgd_ovcv_dn8 + (locals.var_gov2_dn8 * 0.5)) - ((locals.var_gov_dn8 * assign25150_e25934) + (locals.var_gov * (((locals.var_xgd_ovcv_dn8 + (locals.var_gov2_dn8 * 0.25)) - locals.var_sp_ov_w_dn8) / (2.0 * assign25150_e25934))))), ((locals.var_xgd_ovcv_dn9 + (locals.var_gov2_dn9 * 0.5)) - ((locals.var_gov_dn9 * assign25150_e25934) + (locals.var_gov * (((locals.var_xgd_ovcv_dn9 + (locals.var_gov2_dn9 * 0.25)) - locals.var_sp_ov_w_dn9) / (2.0 * assign25150_e25934))))),)
    } else {
        (locals.var_sp_ov_x0, locals.var_sp_ov_x0_dn4, locals.var_sp_ov_x0_dn6, locals.var_sp_ov_x0_dn7, locals.var_sp_ov_x0_dn8, locals.var_sp_ov_x0_dn9,)
    }
};
        locals.var_sp_ov_x0 = assign25150_e25938;
        locals.var_sp_ov_x0_dn4 = assign25150_e25938_d_n4;
        locals.var_sp_ov_x0_dn6 = assign25150_e25938_d_n6;
        locals.var_sp_ov_x0_dn7 = assign25150_e25938_d_n7;
        locals.var_sp_ov_x0_dn8 = assign25150_e25938_d_n8;
        locals.var_sp_ov_x0_dn9 = assign25150_e25938_d_n9;
        locals.var_sp_ov_x0_rv = 0.0;

        let assign25160_e25940: f64 = (-locals.var_sp_ov_x0);
        let assign25160_e25941: f64 = (assign25160_e25940).abs();
        let assign25160_e25943: f64 = if assign25160_e25941 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard719 = assign25160_e25943;
        locals.var_guard719_rv = 0.0;

        let (assign25170_e25957, assign25170_e25957_d_n4, assign25170_e25957_d_n6, assign25170_e25957_d_n7, assign25170_e25957_d_n8, assign25170_e25957_d_n9,) = {
    if ((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) && (locals.var_guard719 != 0.0)) {
        let assign25170_e25954: f64 = (-locals.var_sp_ov_x0);
        let assign25170_e25955: f64 = (assign25170_e25954).exp();
        (assign25170_e25955, (assign25170_e25955 * (-locals.var_sp_ov_x0_dn4)), (assign25170_e25955 * (-locals.var_sp_ov_x0_dn6)), (assign25170_e25955 * (-locals.var_sp_ov_x0_dn7)), (assign25170_e25955 * (-locals.var_sp_ov_x0_dn8)), (assign25170_e25955 * (-locals.var_sp_ov_x0_dn9)),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign25170_e25957;
        locals.var_sp_ov_d0_dn4 = assign25170_e25957_d_n4;
        locals.var_sp_ov_d0_dn6 = assign25170_e25957_d_n6;
        locals.var_sp_ov_d0_dn7 = assign25170_e25957_d_n7;
        locals.var_sp_ov_d0_dn8 = assign25170_e25957_d_n8;
        locals.var_sp_ov_d0_dn9 = assign25170_e25957_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let assign25180_e25959: f64 = (-locals.var_sp_ov_x0);
        let assign25180_e25961: f64 = (-80.0);
        let assign25180_e25962: f64 = if assign25180_e25959 < assign25180_e25961 { 1.0 } else { 0.0 };
        locals.var_guard720 = assign25180_e25962;
        locals.var_guard720_rv = 0.0;

        let (assign25190_e26005, assign25190_e26005_d_n4, assign25190_e26005_d_n6, assign25190_e26005_d_n7, assign25190_e26005_d_n8, assign25190_e26005_d_n9,) = {
    if (((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) && (locals.var_guard719 == 0.0)) && (locals.var_guard720 != 0.0)) {
        let assign25190_e25978: f64 = (-locals.var_sp_ov_x0);
        let assign25190_e25979: f64 = (-assign25190_e25978);
        let assign25190_e25981: f64 = (assign25190_e25979 - 80.0);
        let assign25190_e25985: f64 = (-locals.var_sp_ov_x0);
        let assign25190_e25986: f64 = (-assign25190_e25985);
        let assign25190_e25988: f64 = (assign25190_e25986 - 80.0);
        let assign25190_e25989: f64 = (0.5 * assign25190_e25988);
        let assign25190_e25992: f64 = (-locals.var_sp_ov_x0);
        let assign25190_e25993: f64 = (-assign25190_e25992);
        let assign25190_e25995: f64 = (assign25190_e25993 - 80.0);
        let assign25190_e25997: f64 = (assign25190_e25995 * 0.3333333333333);
        let assign25190_e25998: f64 = (1.0 + assign25190_e25997);
        let assign25190_e25999: f64 = (assign25190_e25989 * assign25190_e25998);
        let assign25190_e26000: f64 = (1.0 + assign25190_e25999);
        let assign25190_e26001: f64 = (assign25190_e25981 * assign25190_e26000);
        let assign25190_e26002: f64 = (1.0 + assign25190_e26001);
        let assign25190_e26003: f64 = (1.80485e-35 / assign25190_e26002);
        (assign25190_e26003, (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn4)) * assign25190_e26000) + (assign25190_e25981 * (((0.5 * (-(-locals.var_sp_ov_x0_dn4))) * assign25190_e25998) + (assign25190_e25989 * ((-(-locals.var_sp_ov_x0_dn4)) * 0.3333333333333)))))) / (assign25190_e26002 * assign25190_e26002))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn6)) * assign25190_e26000) + (assign25190_e25981 * (((0.5 * (-(-locals.var_sp_ov_x0_dn6))) * assign25190_e25998) + (assign25190_e25989 * ((-(-locals.var_sp_ov_x0_dn6)) * 0.3333333333333)))))) / (assign25190_e26002 * assign25190_e26002))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn7)) * assign25190_e26000) + (assign25190_e25981 * (((0.5 * (-(-locals.var_sp_ov_x0_dn7))) * assign25190_e25998) + (assign25190_e25989 * ((-(-locals.var_sp_ov_x0_dn7)) * 0.3333333333333)))))) / (assign25190_e26002 * assign25190_e26002))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn8)) * assign25190_e26000) + (assign25190_e25981 * (((0.5 * (-(-locals.var_sp_ov_x0_dn8))) * assign25190_e25998) + (assign25190_e25989 * ((-(-locals.var_sp_ov_x0_dn8)) * 0.3333333333333)))))) / (assign25190_e26002 * assign25190_e26002))), (-((1.80485e-35 * (((-(-locals.var_sp_ov_x0_dn9)) * assign25190_e26000) + (assign25190_e25981 * (((0.5 * (-(-locals.var_sp_ov_x0_dn9))) * assign25190_e25998) + (assign25190_e25989 * ((-(-locals.var_sp_ov_x0_dn9)) * 0.3333333333333)))))) / (assign25190_e26002 * assign25190_e26002))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign25190_e26005;
        locals.var_sp_ov_d0_dn4 = assign25190_e26005_d_n4;
        locals.var_sp_ov_d0_dn6 = assign25190_e26005_d_n6;
        locals.var_sp_ov_d0_dn7 = assign25190_e26005_d_n7;
        locals.var_sp_ov_d0_dn8 = assign25190_e26005_d_n8;
        locals.var_sp_ov_d0_dn9 = assign25190_e26005_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign25200_e26046, assign25200_e26046_d_n4, assign25200_e26046_d_n6, assign25200_e26046_d_n7, assign25200_e26046_d_n8, assign25200_e26046_d_n9,) = {
    if (((((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) && (locals.var_guard719 == 0.0)) && (locals.var_guard720 == 0.0)) {
        let assign25200_e26022: f64 = (-locals.var_sp_ov_x0);
        let assign25200_e26024: f64 = (assign25200_e26022 - 80.0);
        let assign25200_e26028: f64 = (-locals.var_sp_ov_x0);
        let assign25200_e26030: f64 = (assign25200_e26028 - 80.0);
        let assign25200_e26031: f64 = (0.5 * assign25200_e26030);
        let assign25200_e26034: f64 = (-locals.var_sp_ov_x0);
        let assign25200_e26036: f64 = (assign25200_e26034 - 80.0);
        let assign25200_e26038: f64 = (assign25200_e26036 * 0.3333333333333);
        let assign25200_e26039: f64 = (1.0 + assign25200_e26038);
        let assign25200_e26040: f64 = (assign25200_e26031 * assign25200_e26039);
        let assign25200_e26041: f64 = (1.0 + assign25200_e26040);
        let assign25200_e26042: f64 = (assign25200_e26024 * assign25200_e26041);
        let assign25200_e26043: f64 = (1.0 + assign25200_e26042);
        let assign25200_e26044: f64 = (5.54062e34 * assign25200_e26043);
        (assign25200_e26044, (5.54062e34 * (((-locals.var_sp_ov_x0_dn4) * assign25200_e26041) + (assign25200_e26024 * (((0.5 * (-locals.var_sp_ov_x0_dn4)) * assign25200_e26039) + (assign25200_e26031 * ((-locals.var_sp_ov_x0_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn6) * assign25200_e26041) + (assign25200_e26024 * (((0.5 * (-locals.var_sp_ov_x0_dn6)) * assign25200_e26039) + (assign25200_e26031 * ((-locals.var_sp_ov_x0_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn7) * assign25200_e26041) + (assign25200_e26024 * (((0.5 * (-locals.var_sp_ov_x0_dn7)) * assign25200_e26039) + (assign25200_e26031 * ((-locals.var_sp_ov_x0_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn8) * assign25200_e26041) + (assign25200_e26024 * (((0.5 * (-locals.var_sp_ov_x0_dn8)) * assign25200_e26039) + (assign25200_e26031 * ((-locals.var_sp_ov_x0_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_sp_ov_x0_dn9) * assign25200_e26041) + (assign25200_e26024 * (((0.5 * (-locals.var_sp_ov_x0_dn9)) * assign25200_e26039) + (assign25200_e26031 * ((-locals.var_sp_ov_x0_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_sp_ov_d0, locals.var_sp_ov_d0_dn4, locals.var_sp_ov_d0_dn6, locals.var_sp_ov_d0_dn7, locals.var_sp_ov_d0_dn8, locals.var_sp_ov_d0_dn9,)
    }
};
        locals.var_sp_ov_d0 = assign25200_e26046;
        locals.var_sp_ov_d0_dn4 = assign25200_e26046_d_n4;
        locals.var_sp_ov_d0_dn6 = assign25200_e26046_d_n6;
        locals.var_sp_ov_d0_dn7 = assign25200_e26046_d_n7;
        locals.var_sp_ov_d0_dn8 = assign25200_e26046_d_n8;
        locals.var_sp_ov_d0_dn9 = assign25200_e26046_d_n9;
        locals.var_sp_ov_d0_rv = 0.0;

        let (assign25210_e26066, assign25210_e26066_d_n4, assign25210_e26066_d_n6, assign25210_e26066_d_n7, assign25210_e26066_d_n8, assign25210_e26066_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25210_e26057: f64 = (locals.var_xgd_ovcv - locals.var_sp_ov_x0);
        let assign25210_e26058: f64 = (2.0 * assign25210_e26057);
        let assign25210_e26062: f64 = (1.0 - locals.var_sp_ov_d0);
        let assign25210_e26063: f64 = (locals.var_gov2 * assign25210_e26062);
        let assign25210_e26064: f64 = (assign25210_e26058 + assign25210_e26063);
        (assign25210_e26064, ((2.0 * (locals.var_xgd_ovcv_dn4 - locals.var_sp_ov_x0_dn4)) + ((locals.var_gov2_dn4 * assign25210_e26062) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn4)))), ((2.0 * (locals.var_xgd_ovcv_dn6 - locals.var_sp_ov_x0_dn6)) + ((locals.var_gov2_dn6 * assign25210_e26062) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn6)))), ((2.0 * (locals.var_xgd_ovcv_dn7 - locals.var_sp_ov_x0_dn7)) + ((locals.var_gov2_dn7 * assign25210_e26062) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn7)))), ((2.0 * (locals.var_xgd_ovcv_dn8 - locals.var_sp_ov_x0_dn8)) + ((locals.var_gov2_dn8 * assign25210_e26062) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn8)))), ((2.0 * (locals.var_xgd_ovcv_dn9 - locals.var_sp_ov_x0_dn9)) + ((locals.var_gov2_dn9 * assign25210_e26062) + (locals.var_gov2 * (-locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_p, locals.var_sp_ov_p_dn4, locals.var_sp_ov_p_dn6, locals.var_sp_ov_p_dn7, locals.var_sp_ov_p_dn8, locals.var_sp_ov_p_dn9,)
    }
};
        locals.var_sp_ov_p = assign25210_e26066;
        locals.var_sp_ov_p_dn4 = assign25210_e26066_d_n4;
        locals.var_sp_ov_p_dn6 = assign25210_e26066_d_n6;
        locals.var_sp_ov_p_dn7 = assign25210_e26066_d_n7;
        locals.var_sp_ov_p_dn8 = assign25210_e26066_d_n8;
        locals.var_sp_ov_p_dn9 = assign25210_e26066_d_n9;
        locals.var_sp_ov_p_rv = 0.0;

        let (assign25220_e26090, assign25220_e26090_d_n4, assign25220_e26090_d_n6, assign25220_e26090_d_n7, assign25220_e26090_d_n8, assign25220_e26090_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25220_e26076: f64 = (locals.var_xgd_ovcv - locals.var_sp_ov_x0);
        let assign25220_e26079: f64 = (locals.var_xgd_ovcv - locals.var_sp_ov_x0);
        let assign25220_e26080: f64 = (assign25220_e26076 * assign25220_e26079);
        let assign25220_e26084: f64 = (locals.var_sp_ov_x0 - 1.0);
        let assign25220_e26086: f64 = (assign25220_e26084 + locals.var_sp_ov_d0);
        let assign25220_e26087: f64 = (locals.var_gov2 * assign25220_e26086);
        let assign25220_e26088: f64 = (assign25220_e26080 - assign25220_e26087);
        (assign25220_e26088, ((((locals.var_xgd_ovcv_dn4 - locals.var_sp_ov_x0_dn4) * assign25220_e26079) + (assign25220_e26076 * (locals.var_xgd_ovcv_dn4 - locals.var_sp_ov_x0_dn4))) - ((locals.var_gov2_dn4 * assign25220_e26086) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn4 + locals.var_sp_ov_d0_dn4)))), ((((locals.var_xgd_ovcv_dn6 - locals.var_sp_ov_x0_dn6) * assign25220_e26079) + (assign25220_e26076 * (locals.var_xgd_ovcv_dn6 - locals.var_sp_ov_x0_dn6))) - ((locals.var_gov2_dn6 * assign25220_e26086) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn6 + locals.var_sp_ov_d0_dn6)))), ((((locals.var_xgd_ovcv_dn7 - locals.var_sp_ov_x0_dn7) * assign25220_e26079) + (assign25220_e26076 * (locals.var_xgd_ovcv_dn7 - locals.var_sp_ov_x0_dn7))) - ((locals.var_gov2_dn7 * assign25220_e26086) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn7 + locals.var_sp_ov_d0_dn7)))), ((((locals.var_xgd_ovcv_dn8 - locals.var_sp_ov_x0_dn8) * assign25220_e26079) + (assign25220_e26076 * (locals.var_xgd_ovcv_dn8 - locals.var_sp_ov_x0_dn8))) - ((locals.var_gov2_dn8 * assign25220_e26086) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn8 + locals.var_sp_ov_d0_dn8)))), ((((locals.var_xgd_ovcv_dn9 - locals.var_sp_ov_x0_dn9) * assign25220_e26079) + (assign25220_e26076 * (locals.var_xgd_ovcv_dn9 - locals.var_sp_ov_x0_dn9))) - ((locals.var_gov2_dn9 * assign25220_e26086) + (locals.var_gov2 * (locals.var_sp_ov_x0_dn9 + locals.var_sp_ov_d0_dn9)))),)
    } else {
        (locals.var_sp_ov_q, locals.var_sp_ov_q_dn4, locals.var_sp_ov_q_dn6, locals.var_sp_ov_q_dn7, locals.var_sp_ov_q_dn8, locals.var_sp_ov_q_dn9,)
    }
};
        locals.var_sp_ov_q = assign25220_e26090;
        locals.var_sp_ov_q_dn4 = assign25220_e26090_d_n4;
        locals.var_sp_ov_q_dn6 = assign25220_e26090_d_n6;
        locals.var_sp_ov_q_dn7 = assign25220_e26090_d_n7;
        locals.var_sp_ov_q_dn8 = assign25220_e26090_d_n8;
        locals.var_sp_ov_q_dn9 = assign25220_e26090_d_n9;
        locals.var_sp_ov_q_rv = 0.0;

        let (assign25230_e26106, assign25230_e26106_d_n4, assign25230_e26106_d_n6, assign25230_e26106_d_n7, assign25230_e26106_d_n8, assign25230_e26106_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25230_e26101: f64 = (locals.var_gov2 * 0.5);
        let assign25230_e26103: f64 = (assign25230_e26101 * locals.var_sp_ov_d0);
        let assign25230_e26104: f64 = (1.0 - assign25230_e26103);
        (assign25230_e26104, (-(((locals.var_gov2_dn4 * 0.5) * locals.var_sp_ov_d0) + (assign25230_e26101 * locals.var_sp_ov_d0_dn4))), (-(((locals.var_gov2_dn6 * 0.5) * locals.var_sp_ov_d0) + (assign25230_e26101 * locals.var_sp_ov_d0_dn6))), (-(((locals.var_gov2_dn7 * 0.5) * locals.var_sp_ov_d0) + (assign25230_e26101 * locals.var_sp_ov_d0_dn7))), (-(((locals.var_gov2_dn8 * 0.5) * locals.var_sp_ov_d0) + (assign25230_e26101 * locals.var_sp_ov_d0_dn8))), (-(((locals.var_gov2_dn9 * 0.5) * locals.var_sp_ov_d0) + (assign25230_e26101 * locals.var_sp_ov_d0_dn9))),)
    } else {
        (locals.var_sp_ov_xi, locals.var_sp_ov_xi_dn4, locals.var_sp_ov_xi_dn6, locals.var_sp_ov_xi_dn7, locals.var_sp_ov_xi_dn8, locals.var_sp_ov_xi_dn9,)
    }
};
        locals.var_sp_ov_xi = assign25230_e26106;
        locals.var_sp_ov_xi_dn4 = assign25230_e26106_d_n4;
        locals.var_sp_ov_xi_dn6 = assign25230_e26106_d_n6;
        locals.var_sp_ov_xi_dn7 = assign25230_e26106_d_n7;
        locals.var_sp_ov_xi_dn8 = assign25230_e26106_d_n8;
        locals.var_sp_ov_xi_dn9 = assign25230_e26106_d_n9;
        locals.var_sp_ov_xi_rv = 0.0;

        let (assign25240_e26124, assign25240_e26124_d_n4, assign25240_e26124_d_n6, assign25240_e26124_d_n7, assign25240_e26124_d_n8, assign25240_e26124_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25240_e26116: f64 = (locals.var_sp_ov_p * locals.var_sp_ov_p);
        let assign25240_e26120: f64 = (locals.var_sp_ov_xi * locals.var_sp_ov_q);
        let assign25240_e26121: f64 = (4.0 * assign25240_e26120);
        let assign25240_e26122: f64 = (assign25240_e26116 - assign25240_e26121);
        (assign25240_e26122, (((locals.var_sp_ov_p_dn4 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn4)) - (4.0 * ((locals.var_sp_ov_xi_dn4 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn4)))), (((locals.var_sp_ov_p_dn6 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn6)) - (4.0 * ((locals.var_sp_ov_xi_dn6 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn6)))), (((locals.var_sp_ov_p_dn7 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn7)) - (4.0 * ((locals.var_sp_ov_xi_dn7 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn7)))), (((locals.var_sp_ov_p_dn8 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn8)) - (4.0 * ((locals.var_sp_ov_xi_dn8 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn8)))), (((locals.var_sp_ov_p_dn9 * locals.var_sp_ov_p) + (locals.var_sp_ov_p * locals.var_sp_ov_p_dn9)) - (4.0 * ((locals.var_sp_ov_xi_dn9 * locals.var_sp_ov_q) + (locals.var_sp_ov_xi * locals.var_sp_ov_q_dn9)))),)
    } else {
        (locals.var_sp_ov_temp, locals.var_sp_ov_temp_dn4, locals.var_sp_ov_temp_dn6, locals.var_sp_ov_temp_dn7, locals.var_sp_ov_temp_dn8, locals.var_sp_ov_temp_dn9,)
    }
};
        locals.var_sp_ov_temp = assign25240_e26124;
        locals.var_sp_ov_temp_dn4 = assign25240_e26124_d_n4;
        locals.var_sp_ov_temp_dn6 = assign25240_e26124_d_n6;
        locals.var_sp_ov_temp_dn7 = assign25240_e26124_d_n7;
        locals.var_sp_ov_temp_dn8 = assign25240_e26124_d_n8;
        locals.var_sp_ov_temp_dn9 = assign25240_e26124_d_n9;
        locals.var_sp_ov_temp_rv = 0.0;

        let (assign25250_e26141, assign25250_e26141_d_n4, assign25250_e26141_d_n6, assign25250_e26141_d_n7, assign25250_e26141_d_n8, assign25250_e26141_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25250_e26134: f64 = (2.0 * locals.var_sp_ov_q);
        let assign25250_e26137: f64 = (locals.var_sp_ov_temp).sqrt();
        let assign25250_e26138: f64 = (locals.var_sp_ov_p + assign25250_e26137);
        let assign25250_e26139: f64 = (assign25250_e26134 / assign25250_e26138);
        (assign25250_e26139, ((((2.0 * locals.var_sp_ov_q_dn4) * assign25250_e26138) - (assign25250_e26134 * (locals.var_sp_ov_p_dn4 + (locals.var_sp_ov_temp_dn4 / (2.0 * assign25250_e26137))))) / (assign25250_e26138 * assign25250_e26138)), ((((2.0 * locals.var_sp_ov_q_dn6) * assign25250_e26138) - (assign25250_e26134 * (locals.var_sp_ov_p_dn6 + (locals.var_sp_ov_temp_dn6 / (2.0 * assign25250_e26137))))) / (assign25250_e26138 * assign25250_e26138)), ((((2.0 * locals.var_sp_ov_q_dn7) * assign25250_e26138) - (assign25250_e26134 * (locals.var_sp_ov_p_dn7 + (locals.var_sp_ov_temp_dn7 / (2.0 * assign25250_e26137))))) / (assign25250_e26138 * assign25250_e26138)), ((((2.0 * locals.var_sp_ov_q_dn8) * assign25250_e26138) - (assign25250_e26134 * (locals.var_sp_ov_p_dn8 + (locals.var_sp_ov_temp_dn8 / (2.0 * assign25250_e26137))))) / (assign25250_e26138 * assign25250_e26138)), ((((2.0 * locals.var_sp_ov_q_dn9) * assign25250_e26138) - (assign25250_e26134 * (locals.var_sp_ov_p_dn9 + (locals.var_sp_ov_temp_dn9 / (2.0 * assign25250_e26137))))) / (assign25250_e26138 * assign25250_e26138)),)
    } else {
        (locals.var_sp_ov_u, locals.var_sp_ov_u_dn4, locals.var_sp_ov_u_dn6, locals.var_sp_ov_u_dn7, locals.var_sp_ov_u_dn8, locals.var_sp_ov_u_dn9,)
    }
};
        locals.var_sp_ov_u = assign25250_e26141;
        locals.var_sp_ov_u_dn4 = assign25250_e26141_d_n4;
        locals.var_sp_ov_u_dn6 = assign25250_e26141_d_n6;
        locals.var_sp_ov_u_dn7 = assign25250_e26141_d_n7;
        locals.var_sp_ov_u_dn8 = assign25250_e26141_d_n8;
        locals.var_sp_ov_u_dn9 = assign25250_e26141_d_n9;
        locals.var_sp_ov_u_rv = 0.0;

        let (assign25260_e26153, assign25260_e26153_d_n4, assign25260_e26153_d_n6, assign25260_e26153_d_n7, assign25260_e26153_d_n8, assign25260_e26153_d_n9,) = {
    if (((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) && (locals.var_guard714 == 0.0)) {
        let assign25260_e26151: f64 = (locals.var_sp_ov_x0 + locals.var_sp_ov_u);
        (assign25260_e26151, (locals.var_sp_ov_x0_dn4 + locals.var_sp_ov_u_dn4), (locals.var_sp_ov_x0_dn6 + locals.var_sp_ov_u_dn6), (locals.var_sp_ov_x0_dn7 + locals.var_sp_ov_u_dn7), (locals.var_sp_ov_x0_dn8 + locals.var_sp_ov_u_dn8), (locals.var_sp_ov_x0_dn9 + locals.var_sp_ov_u_dn9),)
    } else {
        (locals.var_xd_ovcv, locals.var_xd_ovcv_dn4, locals.var_xd_ovcv_dn6, locals.var_xd_ovcv_dn7, locals.var_xd_ovcv_dn8, locals.var_xd_ovcv_dn9,)
    }
};
        locals.var_xd_ovcv = assign25260_e26153;
        locals.var_xd_ovcv_dn4 = assign25260_e26153_d_n4;
        locals.var_xd_ovcv_dn6 = assign25260_e26153_d_n6;
        locals.var_xd_ovcv_dn7 = assign25260_e26153_d_n7;
        locals.var_xd_ovcv_dn8 = assign25260_e26153_d_n8;
        locals.var_xd_ovcv_dn9 = assign25260_e26153_d_n9;
        locals.var_xd_ovcv_rv = 0.0;

        let (assign25270_e26161, assign25270_e26161_d_n4, assign25270_e26161_d_n6, assign25270_e26161_d_n7, assign25270_e26161_d_n8, assign25270_e26161_d_n9,) = {
    if ((locals.var_guard712 != 0.0) && (locals.var_guard713 == 0.0)) {
        let assign25270_e26159: f64 = (-locals.var_xd_ovcv);
        (assign25270_e26159, (-locals.var_xd_ovcv_dn4), (-locals.var_xd_ovcv_dn6), (-locals.var_xd_ovcv_dn7), (-locals.var_xd_ovcv_dn8), (-locals.var_xd_ovcv_dn9),)
    } else {
        (locals.var_xd_ovcv, locals.var_xd_ovcv_dn4, locals.var_xd_ovcv_dn6, locals.var_xd_ovcv_dn7, locals.var_xd_ovcv_dn8, locals.var_xd_ovcv_dn9,)
    }
};
        locals.var_xd_ovcv = assign25270_e26161;
        locals.var_xd_ovcv_dn4 = assign25270_e26161_d_n4;
        locals.var_xd_ovcv_dn6 = assign25270_e26161_d_n6;
        locals.var_xd_ovcv_dn7 = assign25270_e26161_d_n7;
        locals.var_xd_ovcv_dn8 = assign25270_e26161_d_n8;
        locals.var_xd_ovcv_dn9 = assign25270_e26161_d_n9;
        locals.var_xd_ovcv_rv = 0.0;

        let assign25280_e26163: f64 = (-locals.var_phit0);
        let assign25280_e26166: f64 = (locals.var_xgs_ov + locals.var_xs_ov);
        let assign25280_e26167: f64 = (assign25280_e26163 * assign25280_e26166);
        locals.var_vovs = assign25280_e26167;
        locals.var_vovs_dn4 = (((-locals.var_phit0_dn4) * assign25280_e26166) + (assign25280_e26163 * (locals.var_xgs_ov_dn4 + locals.var_xs_ov_dn4)));
        locals.var_vovs_dn6 = (((-locals.var_phit0_dn6) * assign25280_e26166) + (assign25280_e26163 * (locals.var_xgs_ov_dn6 + locals.var_xs_ov_dn6)));
        locals.var_vovs_dn7 = (((-locals.var_phit0_dn7) * assign25280_e26166) + (assign25280_e26163 * (locals.var_xgs_ov_dn7 + locals.var_xs_ov_dn7)));
        locals.var_vovs_dn8 = (((-locals.var_phit0_dn8) * assign25280_e26166) + (assign25280_e26163 * (locals.var_xgs_ov_dn8 + locals.var_xs_ov_dn8)));
        locals.var_vovs_dn9 = (((-locals.var_phit0_dn9) * assign25280_e26166) + (assign25280_e26163 * (locals.var_xgs_ov_dn9 + locals.var_xs_ov_dn9)));
        locals.var_vovs_rv = 0.0;

        let assign25290_e26169: f64 = (-locals.var_phit0);
        let assign25290_e26172: f64 = (locals.var_xgd_ov + locals.var_xd_ov);
        let assign25290_e26173: f64 = (assign25290_e26169 * assign25290_e26172);
        locals.var_vovd = assign25290_e26173;
        locals.var_vovd_dn4 = (((-locals.var_phit0_dn4) * assign25290_e26172) + (assign25290_e26169 * (locals.var_xgd_ov_dn4 + locals.var_xd_ov_dn4)));
        locals.var_vovd_dn6 = (((-locals.var_phit0_dn6) * assign25290_e26172) + (assign25290_e26169 * (locals.var_xgd_ov_dn6 + locals.var_xd_ov_dn6)));
        locals.var_vovd_dn7 = (((-locals.var_phit0_dn7) * assign25290_e26172) + (assign25290_e26169 * (locals.var_xgd_ov_dn7 + locals.var_xd_ov_dn7)));
        locals.var_vovd_dn8 = (((-locals.var_phit0_dn8) * assign25290_e26172) + (assign25290_e26169 * (locals.var_xgd_ov_dn8 + locals.var_xd_ov_dn8)));
        locals.var_vovd_dn9 = (((-locals.var_phit0_dn9) * assign25290_e26172) + (assign25290_e26169 * (locals.var_xgd_ov_dn9 + locals.var_xd_ov_dn9)));
        locals.var_vovd_rv = 0.0;

        let assign25300_e26175: f64 = (-locals.var_phit0);
        let assign25300_e26178: f64 = (locals.var_xgs_ovcv + locals.var_xs_ovcv);
        let assign25300_e26179: f64 = (assign25300_e26175 * assign25300_e26178);
        locals.var_vovscv = assign25300_e26179;
        locals.var_vovscv_dn4 = (((-locals.var_phit0_dn4) * assign25300_e26178) + (assign25300_e26175 * (locals.var_xgs_ovcv_dn4 + locals.var_xs_ovcv_dn4)));
        locals.var_vovscv_dn6 = (((-locals.var_phit0_dn6) * assign25300_e26178) + (assign25300_e26175 * (locals.var_xgs_ovcv_dn6 + locals.var_xs_ovcv_dn6)));
        locals.var_vovscv_dn7 = (((-locals.var_phit0_dn7) * assign25300_e26178) + (assign25300_e26175 * (locals.var_xgs_ovcv_dn7 + locals.var_xs_ovcv_dn7)));
        locals.var_vovscv_dn8 = (((-locals.var_phit0_dn8) * assign25300_e26178) + (assign25300_e26175 * (locals.var_xgs_ovcv_dn8 + locals.var_xs_ovcv_dn8)));
        locals.var_vovscv_dn9 = (((-locals.var_phit0_dn9) * assign25300_e26178) + (assign25300_e26175 * (locals.var_xgs_ovcv_dn9 + locals.var_xs_ovcv_dn9)));
        locals.var_vovscv_rv = 0.0;

        let assign25310_e26181: f64 = (-locals.var_phit0);
        let assign25310_e26184: f64 = (locals.var_xgd_ovcv + locals.var_xd_ovcv);
        let assign25310_e26185: f64 = (assign25310_e26181 * assign25310_e26184);
        locals.var_vovdcv = assign25310_e26185;
        locals.var_vovdcv_dn4 = (((-locals.var_phit0_dn4) * assign25310_e26184) + (assign25310_e26181 * (locals.var_xgd_ovcv_dn4 + locals.var_xd_ovcv_dn4)));
        locals.var_vovdcv_dn6 = (((-locals.var_phit0_dn6) * assign25310_e26184) + (assign25310_e26181 * (locals.var_xgd_ovcv_dn6 + locals.var_xd_ovcv_dn6)));
        locals.var_vovdcv_dn7 = (((-locals.var_phit0_dn7) * assign25310_e26184) + (assign25310_e26181 * (locals.var_xgd_ovcv_dn7 + locals.var_xd_ovcv_dn7)));
        locals.var_vovdcv_dn8 = (((-locals.var_phit0_dn8) * assign25310_e26184) + (assign25310_e26181 * (locals.var_xgd_ovcv_dn8 + locals.var_xd_ovcv_dn8)));
        locals.var_vovdcv_dn9 = (((-locals.var_phit0_dn9) * assign25310_e26184) + (assign25310_e26181 * (locals.var_xgd_ovcv_dn9 + locals.var_xd_ovcv_dn9)));
        locals.var_vovdcv_rv = 0.0;

        let assign25390_e26195: f64 = if p.p3 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard721 = assign25390_e26195;
        locals.var_guard721_rv = 0.0;

        let assign25400_e26202: f64 = if ((locals.var_igovinv_i > 0.0) || (locals.var_igovacc_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard722 = assign25400_e26202;
        locals.var_guard722_rv = 0.0;

        let (assign25410_e26210, assign25410_e26210_d_n4, assign25410_e26210_d_n6, assign25410_e26210_d_n7, assign25410_e26210_d_n8, assign25410_e26210_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25410_e26208: f64 = (locals.var_vovs + locals.var_dov);
        (assign25410_e26208, (locals.var_vovs_dn4 + locals.var_dov_dn4), (locals.var_vovs_dn6 + locals.var_dov_dn6), (locals.var_vovs_dn7 + locals.var_dov_dn7), (locals.var_vovs_dn8 + locals.var_dov_dn8), (locals.var_vovs_dn9 + locals.var_dov_dn9),)
    } else {
        (locals.var_arg2mina, locals.var_arg2mina_dn4, locals.var_arg2mina_dn6, locals.var_arg2mina_dn7, locals.var_arg2mina_dn8, locals.var_arg2mina_dn9,)
    }
};
        locals.var_arg2mina = assign25410_e26210;
        locals.var_arg2mina_dn4 = assign25410_e26210_d_n4;
        locals.var_arg2mina_dn6 = assign25410_e26210_d_n6;
        locals.var_arg2mina_dn7 = assign25410_e26210_d_n7;
        locals.var_arg2mina_dn8 = assign25410_e26210_d_n8;
        locals.var_arg2mina_dn9 = assign25410_e26210_d_n9;
        locals.var_arg2mina_rv = 0.0;

        let (assign25420_e26231, assign25420_e26231_d_n4, assign25420_e26231_d_n6, assign25420_e26231_d_n7, assign25420_e26231_d_n8, assign25420_e26231_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25420_e26217: f64 = locals.var_arg2mina;
        let assign25420_e26220: f64 = (-locals.var_arg2mina);
        let assign25420_e26223: f64 = (-locals.var_arg2mina);
        let assign25420_e26224: f64 = (assign25420_e26220 * assign25420_e26223);
        let assign25420_e26226: f64 = (assign25420_e26224 + 0.01);
        let assign25420_e26227: f64 = (assign25420_e26226).sqrt();
        let assign25420_e26228: f64 = (assign25420_e26217 - assign25420_e26227);
        let assign25420_e26229: f64 = (0.5 * assign25420_e26228);
        (assign25420_e26229, (0.5 * (locals.var_arg2mina_dn4 - ((((-locals.var_arg2mina_dn4) * assign25420_e26223) + (assign25420_e26220 * (-locals.var_arg2mina_dn4))) / (2.0 * assign25420_e26227)))), (0.5 * (locals.var_arg2mina_dn6 - ((((-locals.var_arg2mina_dn6) * assign25420_e26223) + (assign25420_e26220 * (-locals.var_arg2mina_dn6))) / (2.0 * assign25420_e26227)))), (0.5 * (locals.var_arg2mina_dn7 - ((((-locals.var_arg2mina_dn7) * assign25420_e26223) + (assign25420_e26220 * (-locals.var_arg2mina_dn7))) / (2.0 * assign25420_e26227)))), (0.5 * (locals.var_arg2mina_dn8 - ((((-locals.var_arg2mina_dn8) * assign25420_e26223) + (assign25420_e26220 * (-locals.var_arg2mina_dn8))) / (2.0 * assign25420_e26227)))), (0.5 * (locals.var_arg2mina_dn9 - ((((-locals.var_arg2mina_dn9) * assign25420_e26223) + (assign25420_e26220 * (-locals.var_arg2mina_dn9))) / (2.0 * assign25420_e26227)))),)
    } else {
        (locals.var_psi_t, locals.var_psi_t_dn4, locals.var_psi_t_dn6, locals.var_psi_t_dn7, locals.var_psi_t_dn8, locals.var_psi_t_dn9,)
    }
};
        locals.var_psi_t = assign25420_e26231;
        locals.var_psi_t_dn4 = assign25420_e26231_d_n4;
        locals.var_psi_t_dn6 = assign25420_e26231_d_n6;
        locals.var_psi_t_dn7 = assign25420_e26231_d_n7;
        locals.var_psi_t_dn8 = assign25420_e26231_d_n8;
        locals.var_psi_t_dn9 = assign25420_e26231_d_n9;
        locals.var_psi_t_rv = 0.0;

        let (assign25430_e26244, assign25430_e26244_d_n4, assign25430_e26244_d_n6, assign25430_e26244_d_n7, assign25430_e26244_d_n8, assign25430_e26244_d_n9,) = {
    if ((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) {
        let assign25430_e26237: f64 = (locals.var_vovs * locals.var_vovs);
        let assign25430_e26239: f64 = (assign25430_e26237 + 0.0001);
        let assign25430_e26240: f64 = (assign25430_e26239).sqrt();
        let assign25430_e26242: f64 = (assign25430_e26240 * locals.var_inv_chib);
        (assign25430_e26242, ((((locals.var_vovs_dn4 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn4)) / (2.0 * assign25430_e26240)) * locals.var_inv_chib), ((((locals.var_vovs_dn6 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn6)) / (2.0 * assign25430_e26240)) * locals.var_inv_chib), ((((locals.var_vovs_dn7 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn7)) / (2.0 * assign25430_e26240)) * locals.var_inv_chib), ((((locals.var_vovs_dn8 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn8)) / (2.0 * assign25430_e26240)) * locals.var_inv_chib), ((((locals.var_vovs_dn9 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn9)) / (2.0 * assign25430_e26240)) * locals.var_inv_chib),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign25430_e26244;
        locals.var_zg_dn4 = assign25430_e26244_d_n4;
        locals.var_zg_dn6 = assign25430_e26244_d_n6;
        locals.var_zg_dn7 = assign25430_e26244_d_n7;
        locals.var_zg_dn8 = assign25430_e26244_d_n8;
        locals.var_zg_dn9 = assign25430_e26244_d_n9;
        locals.var_zg_rv = 0.0;

        let assign25440_e26247: f64 = (0.5 * locals.var_xgs_ov);
        let assign25440_e26248: f64 = (assign25440_e26247).abs();
        let assign25440_e26250: f64 = if assign25440_e26248 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard723 = assign25440_e26250;
        locals.var_guard723_rv = 0.0;

        let (assign25450_e26261, assign25450_e26261_d_n4, assign25450_e26261_d_n6, assign25450_e26261_d_n7, assign25450_e26261_d_n8, assign25450_e26261_d_n9,) = {
    if (((locals.var_guard721 != 0.0) && (locals.var_guard722 != 0.0)) && (locals.var_guard723 != 0.0)) {
        let assign25450_e26258: f64 = (0.5 * locals.var_xgs_ov);
        let assign25450_e26259: f64 = (assign25450_e26258).exp();
        (assign25450_e26259, (assign25450_e26259 * (0.5 * locals.var_xgs_ov_dn4)), (assign25450_e26259 * (0.5 * locals.var_xgs_ov_dn6)), (assign25450_e26259 * (0.5 * locals.var_xgs_ov_dn7)), (assign25450_e26259 * (0.5 * locals.var_xgs_ov_dn8)), (assign25450_e26259 * (0.5 * locals.var_xgs_ov_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign25450_e26261;
        locals.var_temp_dn4 = assign25450_e26261_d_n4;
        locals.var_temp_dn6 = assign25450_e26261_d_n6;
        locals.var_temp_dn7 = assign25450_e26261_d_n7;
        locals.var_temp_dn8 = assign25450_e26261_d_n8;
        locals.var_temp_dn9 = assign25450_e26261_d_n9;
        locals.var_temp_rv = 0.0;

        let assign25460_e26264: f64 = (0.5 * locals.var_xgs_ov);
        let assign25460_e26266: f64 = (-80.0);
        let assign25460_e26267: f64 = if assign25460_e26264 < assign25460_e26266 { 1.0 } else { 0.0 };
        locals.var_guard724 = assign25460_e26267;
        locals.var_guard724_rv = 0.0;

    }
}
