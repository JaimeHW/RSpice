#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_80(
        locals: &mut StampLocals,
    ) {
        let (assign30200_e33028, assign30200_e33028_d_n4, assign30200_e33028_d_n6, assign30200_e33028_d_n7, assign30200_e33028_d_n8, assign30200_e33028_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30200_e33023: f64 = (1.0 + locals.var_inv_k1__blk906);
        let assign30200_e33025: f64 = (assign30200_e33023 + locals.var_inv_k2__blk907);
        let assign30200_e33026: f64 = (1.0 / assign30200_e33025);
        (assign30200_e33026, (-((locals.var_inv_k1__blk906_dn4 + locals.var_inv_k2__blk907_dn4) / (assign30200_e33025 * assign30200_e33025))), (-((locals.var_inv_k1__blk906_dn6 + locals.var_inv_k2__blk907_dn6) / (assign30200_e33025 * assign30200_e33025))), (-((locals.var_inv_k1__blk906_dn7 + locals.var_inv_k2__blk907_dn7) / (assign30200_e33025 * assign30200_e33025))), (-((locals.var_inv_k1__blk906_dn8 + locals.var_inv_k2__blk907_dn8) / (assign30200_e33025 * assign30200_e33025))), (-((locals.var_inv_k1__blk906_dn9 + locals.var_inv_k2__blk907_dn9) / (assign30200_e33025 * assign30200_e33025))),)
    } else {
        (locals.var_keq__blk934, locals.var_keq__blk934_dn4, locals.var_keq__blk934_dn6, locals.var_keq__blk934_dn7, locals.var_keq__blk934_dn8, locals.var_keq__blk934_dn9,)
    }
};
        locals.var_keq__blk934 = assign30200_e33028;
        locals.var_keq__blk934_dn4 = assign30200_e33028_d_n4;
        locals.var_keq__blk934_dn6 = assign30200_e33028_d_n6;
        locals.var_keq__blk934_dn7 = assign30200_e33028_d_n7;
        locals.var_keq__blk934_dn8 = assign30200_e33028_d_n8;
        locals.var_keq__blk934_dn9 = assign30200_e33028_d_n9;

        let (assign30210_e33036, assign30210_e33036_d_n4, assign30210_e33036_d_n6, assign30210_e33036_d_n7, assign30210_e33036_d_n8, assign30210_e33036_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30210_e33033: f64 = (locals.var_csiprime__blk919 * locals.var_csiprime__blk919);
        let assign30210_e33034: f64 = (locals.var_a0_csisq / assign30210_e33033);
        (assign30210_e33034, (((locals.var_a0_csisq_dn4 * assign30210_e33033) - (locals.var_a0_csisq * ((locals.var_csiprime__blk919_dn4 * locals.var_csiprime__blk919) + (locals.var_csiprime__blk919 * locals.var_csiprime__blk919_dn4)))) / (assign30210_e33033 * assign30210_e33033)), (((locals.var_a0_csisq_dn6 * assign30210_e33033) - (locals.var_a0_csisq * ((locals.var_csiprime__blk919_dn6 * locals.var_csiprime__blk919) + (locals.var_csiprime__blk919 * locals.var_csiprime__blk919_dn6)))) / (assign30210_e33033 * assign30210_e33033)), (((locals.var_a0_csisq_dn7 * assign30210_e33033) - (locals.var_a0_csisq * ((locals.var_csiprime__blk919_dn7 * locals.var_csiprime__blk919) + (locals.var_csiprime__blk919 * locals.var_csiprime__blk919_dn7)))) / (assign30210_e33033 * assign30210_e33033)), (((locals.var_a0_csisq_dn8 * assign30210_e33033) - (locals.var_a0_csisq * ((locals.var_csiprime__blk919_dn8 * locals.var_csiprime__blk919) + (locals.var_csiprime__blk919 * locals.var_csiprime__blk919_dn8)))) / (assign30210_e33033 * assign30210_e33033)), (((locals.var_a0_csisq_dn9 * assign30210_e33033) - (locals.var_a0_csisq * ((locals.var_csiprime__blk919_dn9 * locals.var_csiprime__blk919) + (locals.var_csiprime__blk919 * locals.var_csiprime__blk919_dn9)))) / (assign30210_e33033 * assign30210_e33033)),)
    } else {
        (locals.var_a0__blk905, locals.var_a0__blk905_dn4, locals.var_a0__blk905_dn6, locals.var_a0__blk905_dn7, locals.var_a0__blk905_dn8, locals.var_a0__blk905_dn9,)
    }
};
        locals.var_a0__blk905 = assign30210_e33036;
        locals.var_a0__blk905_dn4 = assign30210_e33036_d_n4;
        locals.var_a0__blk905_dn6 = assign30210_e33036_d_n6;
        locals.var_a0__blk905_dn7 = assign30210_e33036_d_n7;
        locals.var_a0__blk905_dn8 = assign30210_e33036_d_n8;
        locals.var_a0__blk905_dn9 = assign30210_e33036_d_n9;

        let (assign30220_e33046, assign30220_e33046_d_n4, assign30220_e33046_d_n6, assign30220_e33046_d_n7, assign30220_e33046_d_n8, assign30220_e33046_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30220_e33040: f64 = (1.0 + locals.var_k1__blk932);
        let assign30220_e33043: f64 = (1.0 + locals.var_k2__blk933);
        let assign30220_e33044: f64 = (assign30220_e33040 / assign30220_e33043);
        (assign30220_e33044, (((locals.var_k1__blk932_dn4 * assign30220_e33043) - (assign30220_e33040 * locals.var_k2__blk933_dn4)) / (assign30220_e33043 * assign30220_e33043)), (((locals.var_k1__blk932_dn6 * assign30220_e33043) - (assign30220_e33040 * locals.var_k2__blk933_dn6)) / (assign30220_e33043 * assign30220_e33043)), (((locals.var_k1__blk932_dn7 * assign30220_e33043) - (assign30220_e33040 * locals.var_k2__blk933_dn7)) / (assign30220_e33043 * assign30220_e33043)), (((locals.var_k1__blk932_dn8 * assign30220_e33043) - (assign30220_e33040 * locals.var_k2__blk933_dn8)) / (assign30220_e33043 * assign30220_e33043)), (((locals.var_k1__blk932_dn9 * assign30220_e33043) - (assign30220_e33040 * locals.var_k2__blk933_dn9)) / (assign30220_e33043 * assign30220_e33043)),)
    } else {
        (locals.var_exp_dxth__blk902, locals.var_exp_dxth__blk902_dn4, locals.var_exp_dxth__blk902_dn6, locals.var_exp_dxth__blk902_dn7, locals.var_exp_dxth__blk902_dn8, locals.var_exp_dxth__blk902_dn9,)
    }
};
        locals.var_exp_dxth__blk902 = assign30220_e33046;
        locals.var_exp_dxth__blk902_dn4 = assign30220_e33046_d_n4;
        locals.var_exp_dxth__blk902_dn6 = assign30220_e33046_d_n6;
        locals.var_exp_dxth__blk902_dn7 = assign30220_e33046_d_n7;
        locals.var_exp_dxth__blk902_dn8 = assign30220_e33046_d_n8;
        locals.var_exp_dxth__blk902_dn9 = assign30220_e33046_d_n9;

        let (assign30230_e33051, assign30230_e33051_d_n4, assign30230_e33051_d_n6, assign30230_e33051_d_n7, assign30230_e33051_d_n8, assign30230_e33051_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30230_e33049: f64 = (locals.var_exp_dxth__blk902).ln();
        (assign30230_e33049, (locals.var_exp_dxth__blk902_dn4 / locals.var_exp_dxth__blk902), (locals.var_exp_dxth__blk902_dn6 / locals.var_exp_dxth__blk902), (locals.var_exp_dxth__blk902_dn7 / locals.var_exp_dxth__blk902), (locals.var_exp_dxth__blk902_dn8 / locals.var_exp_dxth__blk902), (locals.var_exp_dxth__blk902_dn9 / locals.var_exp_dxth__blk902),)
    } else {
        (locals.var_dxth__blk903, locals.var_dxth__blk903_dn4, locals.var_dxth__blk903_dn6, locals.var_dxth__blk903_dn7, locals.var_dxth__blk903_dn8, locals.var_dxth__blk903_dn9,)
    }
};
        locals.var_dxth__blk903 = assign30230_e33051;
        locals.var_dxth__blk903_dn4 = assign30230_e33051_d_n4;
        locals.var_dxth__blk903_dn6 = assign30230_e33051_d_n6;
        locals.var_dxth__blk903_dn7 = assign30230_e33051_d_n7;
        locals.var_dxth__blk903_dn8 = assign30230_e33051_d_n8;
        locals.var_dxth__blk903_dn9 = assign30230_e33051_d_n9;

        let assign30240_e33054: f64 = if locals.var_dxth__blk903 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1097 = assign30240_e33054;

        let (assign30250_e33070, assign30250_e33070_d_n4, assign30250_e33070_d_n6, assign30250_e33070_d_n7, assign30250_e33070_d_n8, assign30250_e33070_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1097 != 0.0)) {
        let assign30250_e33060: f64 = (2.0 * locals.var_dxth__blk903);
        let assign30250_e33063: f64 = (locals.var_exp_dxth__blk902 + 1.0);
        let assign30250_e33064: f64 = (assign30250_e33060 * assign30250_e33063);
        let assign30250_e33067: f64 = (locals.var_exp_dxth__blk902 - 1.0);
        let assign30250_e33068: f64 = (assign30250_e33064 / assign30250_e33067);
        (assign30250_e33068, ((((((2.0 * locals.var_dxth__blk903_dn4) * assign30250_e33063) + (assign30250_e33060 * locals.var_exp_dxth__blk902_dn4)) * assign30250_e33067) - (assign30250_e33064 * locals.var_exp_dxth__blk902_dn4)) / (assign30250_e33067 * assign30250_e33067)), ((((((2.0 * locals.var_dxth__blk903_dn6) * assign30250_e33063) + (assign30250_e33060 * locals.var_exp_dxth__blk902_dn6)) * assign30250_e33067) - (assign30250_e33064 * locals.var_exp_dxth__blk902_dn6)) / (assign30250_e33067 * assign30250_e33067)), ((((((2.0 * locals.var_dxth__blk903_dn7) * assign30250_e33063) + (assign30250_e33060 * locals.var_exp_dxth__blk902_dn7)) * assign30250_e33067) - (assign30250_e33064 * locals.var_exp_dxth__blk902_dn7)) / (assign30250_e33067 * assign30250_e33067)), ((((((2.0 * locals.var_dxth__blk903_dn8) * assign30250_e33063) + (assign30250_e33060 * locals.var_exp_dxth__blk902_dn8)) * assign30250_e33067) - (assign30250_e33064 * locals.var_exp_dxth__blk902_dn8)) / (assign30250_e33067 * assign30250_e33067)), ((((((2.0 * locals.var_dxth__blk903_dn9) * assign30250_e33063) + (assign30250_e33060 * locals.var_exp_dxth__blk902_dn9)) * assign30250_e33067) - (assign30250_e33064 * locals.var_exp_dxth__blk902_dn9)) / (assign30250_e33067 * assign30250_e33067)),)
    } else {
        (locals.var_diff_min__blk904, locals.var_diff_min__blk904_dn4, locals.var_diff_min__blk904_dn6, locals.var_diff_min__blk904_dn7, locals.var_diff_min__blk904_dn8, locals.var_diff_min__blk904_dn9,)
    }
};
        locals.var_diff_min__blk904 = assign30250_e33070;
        locals.var_diff_min__blk904_dn4 = assign30250_e33070_d_n4;
        locals.var_diff_min__blk904_dn6 = assign30250_e33070_d_n6;
        locals.var_diff_min__blk904_dn7 = assign30250_e33070_d_n7;
        locals.var_diff_min__blk904_dn8 = assign30250_e33070_d_n8;
        locals.var_diff_min__blk904_dn9 = assign30250_e33070_d_n9;

        let (assign30260_e33081, assign30260_e33081_d_n4, assign30260_e33081_d_n6, assign30260_e33081_d_n7, assign30260_e33081_d_n8, assign30260_e33081_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1097 == 0.0)) {
        let assign30260_e33078: f64 = (2.0 + locals.var_dxth__blk903);
        let assign30260_e33079: f64 = (2.0 * assign30260_e33078);
        (assign30260_e33079, (2.0 * locals.var_dxth__blk903_dn4), (2.0 * locals.var_dxth__blk903_dn6), (2.0 * locals.var_dxth__blk903_dn7), (2.0 * locals.var_dxth__blk903_dn8), (2.0 * locals.var_dxth__blk903_dn9),)
    } else {
        (locals.var_diff_min__blk904, locals.var_diff_min__blk904_dn4, locals.var_diff_min__blk904_dn6, locals.var_diff_min__blk904_dn7, locals.var_diff_min__blk904_dn8, locals.var_diff_min__blk904_dn9,)
    }
};
        locals.var_diff_min__blk904 = assign30260_e33081;
        locals.var_diff_min__blk904_dn4 = assign30260_e33081_d_n4;
        locals.var_diff_min__blk904_dn6 = assign30260_e33081_d_n6;
        locals.var_diff_min__blk904_dn7 = assign30260_e33081_d_n7;
        locals.var_diff_min__blk904_dn8 = assign30260_e33081_d_n8;
        locals.var_diff_min__blk904_dn9 = assign30260_e33081_d_n9;

        let (assign30270_e33089, assign30270_e33089_d_n4, assign30270_e33089_d_n6, assign30270_e33089_d_n7, assign30270_e33089_d_n8, assign30270_e33089_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30270_e33086: f64 = (locals.var_xg1x__blk930 - locals.var_xg2x__blk931);
        let assign30270_e33087: f64 = (locals.var_keq__blk934 * assign30270_e33086);
        (assign30270_e33087, ((locals.var_keq__blk934_dn4 * assign30270_e33086) + (locals.var_keq__blk934 * (locals.var_xg1x__blk930_dn4 - locals.var_xg2x__blk931_dn4))), ((locals.var_keq__blk934_dn6 * assign30270_e33086) + (locals.var_keq__blk934 * (locals.var_xg1x__blk930_dn6 - locals.var_xg2x__blk931_dn6))), ((locals.var_keq__blk934_dn7 * assign30270_e33086) + (locals.var_keq__blk934 * (locals.var_xg1x__blk930_dn7 - locals.var_xg2x__blk931_dn7))), ((locals.var_keq__blk934_dn8 * assign30270_e33086) + (locals.var_keq__blk934 * (locals.var_xg1x__blk930_dn8 - locals.var_xg2x__blk931_dn8))), ((locals.var_keq__blk934_dn9 * assign30270_e33086) + (locals.var_keq__blk934 * (locals.var_xg1x__blk930_dn9 - locals.var_xg2x__blk931_dn9))),)
    } else {
        (locals.var_dx_wi__blk935, locals.var_dx_wi__blk935_dn4, locals.var_dx_wi__blk935_dn6, locals.var_dx_wi__blk935_dn7, locals.var_dx_wi__blk935_dn8, locals.var_dx_wi__blk935_dn9,)
    }
};
        locals.var_dx_wi__blk935 = assign30270_e33089;
        locals.var_dx_wi__blk935_dn4 = assign30270_e33089_d_n4;
        locals.var_dx_wi__blk935_dn6 = assign30270_e33089_d_n6;
        locals.var_dx_wi__blk935_dn7 = assign30270_e33089_d_n7;
        locals.var_dx_wi__blk935_dn8 = assign30270_e33089_d_n8;
        locals.var_dx_wi__blk935_dn9 = assign30270_e33089_d_n9;

        let (assign30280_e33095, assign30280_e33095_d_n4, assign30280_e33095_d_n6, assign30280_e33095_d_n7, assign30280_e33095_d_n8, assign30280_e33095_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30280_e33093: f64 = (locals.var_dx_wi__blk935 * locals.var_dx_wi__blk935);
        (assign30280_e33093, ((locals.var_dx_wi__blk935_dn4 * locals.var_dx_wi__blk935) + (locals.var_dx_wi__blk935 * locals.var_dx_wi__blk935_dn4)), ((locals.var_dx_wi__blk935_dn6 * locals.var_dx_wi__blk935) + (locals.var_dx_wi__blk935 * locals.var_dx_wi__blk935_dn6)), ((locals.var_dx_wi__blk935_dn7 * locals.var_dx_wi__blk935) + (locals.var_dx_wi__blk935 * locals.var_dx_wi__blk935_dn7)), ((locals.var_dx_wi__blk935_dn8 * locals.var_dx_wi__blk935) + (locals.var_dx_wi__blk935 * locals.var_dx_wi__blk935_dn8)), ((locals.var_dx_wi__blk935_dn9 * locals.var_dx_wi__blk935) + (locals.var_dx_wi__blk935 * locals.var_dx_wi__blk935_dn9)),)
    } else {
        (locals.var_dx_wisq__blk936, locals.var_dx_wisq__blk936_dn4, locals.var_dx_wisq__blk936_dn6, locals.var_dx_wisq__blk936_dn7, locals.var_dx_wisq__blk936_dn8, locals.var_dx_wisq__blk936_dn9,)
    }
};
        locals.var_dx_wisq__blk936 = assign30280_e33095;
        locals.var_dx_wisq__blk936_dn4 = assign30280_e33095_d_n4;
        locals.var_dx_wisq__blk936_dn6 = assign30280_e33095_d_n6;
        locals.var_dx_wisq__blk936_dn7 = assign30280_e33095_d_n7;
        locals.var_dx_wisq__blk936_dn8 = assign30280_e33095_d_n8;
        locals.var_dx_wisq__blk936_dn9 = assign30280_e33095_d_n9;

        let (assign30290_e33103, assign30290_e33103_d_n4, assign30290_e33103_d_n6, assign30290_e33103_d_n7, assign30290_e33103_d_n8, assign30290_e33103_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30290_e33100: f64 = (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906);
        let assign30290_e33101: f64 = (locals.var_xg1x__blk930 - assign30290_e33100);
        (assign30290_e33101, (locals.var_xg1x__blk930_dn4 - ((locals.var_dx_wi__blk935_dn4 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn4))), (locals.var_xg1x__blk930_dn6 - ((locals.var_dx_wi__blk935_dn6 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn6))), (locals.var_xg1x__blk930_dn7 - ((locals.var_dx_wi__blk935_dn7 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn7))), (locals.var_xg1x__blk930_dn8 - ((locals.var_dx_wi__blk935_dn8 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn8))), (locals.var_xg1x__blk930_dn9 - ((locals.var_dx_wi__blk935_dn9 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn9))),)
    } else {
        (locals.var_x1_wi0__blk908, locals.var_x1_wi0__blk908_dn4, locals.var_x1_wi0__blk908_dn6, locals.var_x1_wi0__blk908_dn7, locals.var_x1_wi0__blk908_dn8, locals.var_x1_wi0__blk908_dn9,)
    }
};
        locals.var_x1_wi0__blk908 = assign30290_e33103;
        locals.var_x1_wi0__blk908_dn4 = assign30290_e33103_d_n4;
        locals.var_x1_wi0__blk908_dn6 = assign30290_e33103_d_n6;
        locals.var_x1_wi0__blk908_dn7 = assign30290_e33103_d_n7;
        locals.var_x1_wi0__blk908_dn8 = assign30290_e33103_d_n8;
        locals.var_x1_wi0__blk908_dn9 = assign30290_e33103_d_n9;

        let (assign30300_e33111, assign30300_e33111_d_n4, assign30300_e33111_d_n6, assign30300_e33111_d_n7, assign30300_e33111_d_n8, assign30300_e33111_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30300_e33108: f64 = (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907);
        let assign30300_e33109: f64 = (locals.var_xg2x__blk931 + assign30300_e33108);
        (assign30300_e33109, (locals.var_xg2x__blk931_dn4 + ((locals.var_dx_wi__blk935_dn4 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn4))), (locals.var_xg2x__blk931_dn6 + ((locals.var_dx_wi__blk935_dn6 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn6))), (locals.var_xg2x__blk931_dn7 + ((locals.var_dx_wi__blk935_dn7 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn7))), (locals.var_xg2x__blk931_dn8 + ((locals.var_dx_wi__blk935_dn8 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn8))), (locals.var_xg2x__blk931_dn9 + ((locals.var_dx_wi__blk935_dn9 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn9))),)
    } else {
        (locals.var_x2_wi0__blk909, locals.var_x2_wi0__blk909_dn4, locals.var_x2_wi0__blk909_dn6, locals.var_x2_wi0__blk909_dn7, locals.var_x2_wi0__blk909_dn8, locals.var_x2_wi0__blk909_dn9,)
    }
};
        locals.var_x2_wi0__blk909 = assign30300_e33111;
        locals.var_x2_wi0__blk909_dn4 = assign30300_e33111_d_n4;
        locals.var_x2_wi0__blk909_dn6 = assign30300_e33111_d_n6;
        locals.var_x2_wi0__blk909_dn7 = assign30300_e33111_d_n7;
        locals.var_x2_wi0__blk909_dn8 = assign30300_e33111_d_n8;
        locals.var_x2_wi0__blk909_dn9 = assign30300_e33111_d_n9;

        let (assign30310_e33119, assign30310_e33119_d_n4, assign30310_e33119_d_n6, assign30310_e33119_d_n7, assign30310_e33119_d_n8, assign30310_e33119_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30310_e33116: f64 = (locals.var_k1__blk932 + 1.0);
        let assign30310_e33117: f64 = (1.0 / assign30310_e33116);
        (assign30310_e33117, (-(locals.var_k1__blk932_dn4 / (assign30310_e33116 * assign30310_e33116))), (-(locals.var_k1__blk932_dn6 / (assign30310_e33116 * assign30310_e33116))), (-(locals.var_k1__blk932_dn7 / (assign30310_e33116 * assign30310_e33116))), (-(locals.var_k1__blk932_dn8 / (assign30310_e33116 * assign30310_e33116))), (-(locals.var_k1__blk932_dn9 / (assign30310_e33116 * assign30310_e33116))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign30310_e33119;
        locals.var_q_temp1__blk814_dn4 = assign30310_e33119_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign30310_e33119_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign30310_e33119_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign30310_e33119_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign30310_e33119_d_n9;

        let (assign30320_e33127, assign30320_e33127_d_n4, assign30320_e33127_d_n6, assign30320_e33127_d_n7, assign30320_e33127_d_n8, assign30320_e33127_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30320_e33124: f64 = (locals.var_k2__blk933 + 1.0);
        let assign30320_e33125: f64 = (1.0 / assign30320_e33124);
        (assign30320_e33125, (-(locals.var_k2__blk933_dn4 / (assign30320_e33124 * assign30320_e33124))), (-(locals.var_k2__blk933_dn6 / (assign30320_e33124 * assign30320_e33124))), (-(locals.var_k2__blk933_dn7 / (assign30320_e33124 * assign30320_e33124))), (-(locals.var_k2__blk933_dn8 / (assign30320_e33124 * assign30320_e33124))), (-(locals.var_k2__blk933_dn9 / (assign30320_e33124 * assign30320_e33124))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign30320_e33127;
        locals.var_q_temp2__blk815_dn4 = assign30320_e33127_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign30320_e33127_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign30320_e33127_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign30320_e33127_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign30320_e33127_d_n9;

        let (assign30330_e33144, assign30330_e33144_d_n4, assign30330_e33144_d_n6, assign30330_e33144_d_n7, assign30330_e33144_d_n8, assign30330_e33144_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30330_e33132: f64 = (locals.var_k2__blk933 * locals.var_q_temp2__blk815);
        let assign30330_e33133: f64 = (locals.var_k1__blk932 + assign30330_e33132);
        let assign30330_e33135: f64 = (assign30330_e33133 * locals.var_diff_min__blk904);
        let assign30330_e33137: f64 = (assign30330_e33135 / locals.var_a0__blk905);
        let assign30330_e33138: f64 = (assign30330_e33137).ln();
        let assign30330_e33140: f64 = assign30330_e33138;
        let assign30330_e33142: f64 = (assign30330_e33140 + 3.0);
        (assign30330_e33142, (((((((locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_temp2__blk815) + (locals.var_k2__blk933 * locals.var_q_temp2__blk815_dn4))) * locals.var_diff_min__blk904) + (assign30330_e33133 * locals.var_diff_min__blk904_dn4)) * locals.var_a0__blk905) - (assign30330_e33135 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30330_e33137), (((((((locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_temp2__blk815) + (locals.var_k2__blk933 * locals.var_q_temp2__blk815_dn6))) * locals.var_diff_min__blk904) + (assign30330_e33133 * locals.var_diff_min__blk904_dn6)) * locals.var_a0__blk905) - (assign30330_e33135 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30330_e33137), (((((((locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_temp2__blk815) + (locals.var_k2__blk933 * locals.var_q_temp2__blk815_dn7))) * locals.var_diff_min__blk904) + (assign30330_e33133 * locals.var_diff_min__blk904_dn7)) * locals.var_a0__blk905) - (assign30330_e33135 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30330_e33137), (((((((locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_temp2__blk815) + (locals.var_k2__blk933 * locals.var_q_temp2__blk815_dn8))) * locals.var_diff_min__blk904) + (assign30330_e33133 * locals.var_diff_min__blk904_dn8)) * locals.var_a0__blk905) - (assign30330_e33135 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30330_e33137), (((((((locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_temp2__blk815) + (locals.var_k2__blk933 * locals.var_q_temp2__blk815_dn9))) * locals.var_diff_min__blk904) + (assign30330_e33133 * locals.var_diff_min__blk904_dn9)) * locals.var_a0__blk905) - (assign30330_e33135 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30330_e33137),)
    } else {
        (locals.var_q_x1sat__blk817, locals.var_q_x1sat__blk817_dn4, locals.var_q_x1sat__blk817_dn6, locals.var_q_x1sat__blk817_dn7, locals.var_q_x1sat__blk817_dn8, locals.var_q_x1sat__blk817_dn9,)
    }
};
        locals.var_q_x1sat__blk817 = assign30330_e33144;
        locals.var_q_x1sat__blk817_dn4 = assign30330_e33144_d_n4;
        locals.var_q_x1sat__blk817_dn6 = assign30330_e33144_d_n6;
        locals.var_q_x1sat__blk817_dn7 = assign30330_e33144_d_n7;
        locals.var_q_x1sat__blk817_dn8 = assign30330_e33144_d_n8;
        locals.var_q_x1sat__blk817_dn9 = assign30330_e33144_d_n9;

        let (assign30340_e33161, assign30340_e33161_d_n4, assign30340_e33161_d_n6, assign30340_e33161_d_n7, assign30340_e33161_d_n8, assign30340_e33161_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30340_e33149: f64 = (locals.var_k1__blk932 * locals.var_q_temp1__blk814);
        let assign30340_e33150: f64 = (locals.var_k2__blk933 + assign30340_e33149);
        let assign30340_e33152: f64 = (assign30340_e33150 * locals.var_diff_min__blk904);
        let assign30340_e33154: f64 = (assign30340_e33152 / locals.var_a0__blk905);
        let assign30340_e33155: f64 = (assign30340_e33154).ln();
        let assign30340_e33157: f64 = assign30340_e33155;
        let assign30340_e33159: f64 = (assign30340_e33157 + 3.0);
        (assign30340_e33159, (((((((locals.var_k2__blk933_dn4 + ((locals.var_k1__blk932_dn4 * locals.var_q_temp1__blk814) + (locals.var_k1__blk932 * locals.var_q_temp1__blk814_dn4))) * locals.var_diff_min__blk904) + (assign30340_e33150 * locals.var_diff_min__blk904_dn4)) * locals.var_a0__blk905) - (assign30340_e33152 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30340_e33154), (((((((locals.var_k2__blk933_dn6 + ((locals.var_k1__blk932_dn6 * locals.var_q_temp1__blk814) + (locals.var_k1__blk932 * locals.var_q_temp1__blk814_dn6))) * locals.var_diff_min__blk904) + (assign30340_e33150 * locals.var_diff_min__blk904_dn6)) * locals.var_a0__blk905) - (assign30340_e33152 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30340_e33154), (((((((locals.var_k2__blk933_dn7 + ((locals.var_k1__blk932_dn7 * locals.var_q_temp1__blk814) + (locals.var_k1__blk932 * locals.var_q_temp1__blk814_dn7))) * locals.var_diff_min__blk904) + (assign30340_e33150 * locals.var_diff_min__blk904_dn7)) * locals.var_a0__blk905) - (assign30340_e33152 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30340_e33154), (((((((locals.var_k2__blk933_dn8 + ((locals.var_k1__blk932_dn8 * locals.var_q_temp1__blk814) + (locals.var_k1__blk932 * locals.var_q_temp1__blk814_dn8))) * locals.var_diff_min__blk904) + (assign30340_e33150 * locals.var_diff_min__blk904_dn8)) * locals.var_a0__blk905) - (assign30340_e33152 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30340_e33154), (((((((locals.var_k2__blk933_dn9 + ((locals.var_k1__blk932_dn9 * locals.var_q_temp1__blk814) + (locals.var_k1__blk932 * locals.var_q_temp1__blk814_dn9))) * locals.var_diff_min__blk904) + (assign30340_e33150 * locals.var_diff_min__blk904_dn9)) * locals.var_a0__blk905) - (assign30340_e33152 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign30340_e33154),)
    } else {
        (locals.var_q_x2sat__blk818, locals.var_q_x2sat__blk818_dn4, locals.var_q_x2sat__blk818_dn6, locals.var_q_x2sat__blk818_dn7, locals.var_q_x2sat__blk818_dn8, locals.var_q_x2sat__blk818_dn9,)
    }
};
        locals.var_q_x2sat__blk818 = assign30340_e33161;
        locals.var_q_x2sat__blk818_dn4 = assign30340_e33161_d_n4;
        locals.var_q_x2sat__blk818_dn6 = assign30340_e33161_d_n6;
        locals.var_q_x2sat__blk818_dn7 = assign30340_e33161_d_n7;
        locals.var_q_x2sat__blk818_dn8 = assign30340_e33161_d_n8;
        locals.var_q_x2sat__blk818_dn9 = assign30340_e33161_d_n9;

        let assign30350_e33164: f64 = (locals.var_q_x1sat__blk817 - locals.var_x1_wi0__blk908);
        let assign30350_e33166: f64 = (assign30350_e33164 * 0.3333333333333);
        let assign30350_e33168: f64 = if assign30350_e33166 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1098 = assign30350_e33168;

        let (assign30360_e33182, assign30360_e33182_d_n4, assign30360_e33182_d_n6, assign30360_e33182_d_n7, assign30360_e33182_d_n8, assign30360_e33182_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1098 != 0.0)) {
        let assign30360_e33175: f64 = (locals.var_q_x1sat__blk817 - locals.var_x1_wi0__blk908);
        let assign30360_e33177: f64 = (assign30360_e33175 * 0.3333333333333);
        let assign30360_e33178: f64 = (assign30360_e33177).exp();
        let assign30360_e33179: f64 = (1.0 + assign30360_e33178);
        let assign30360_e33180: f64 = (assign30360_e33179).ln();
        (assign30360_e33180, ((assign30360_e33178 * ((locals.var_q_x1sat__blk817_dn4 - locals.var_x1_wi0__blk908_dn4) * 0.3333333333333)) / assign30360_e33179), ((assign30360_e33178 * ((locals.var_q_x1sat__blk817_dn6 - locals.var_x1_wi0__blk908_dn6) * 0.3333333333333)) / assign30360_e33179), ((assign30360_e33178 * ((locals.var_q_x1sat__blk817_dn7 - locals.var_x1_wi0__blk908_dn7) * 0.3333333333333)) / assign30360_e33179), ((assign30360_e33178 * ((locals.var_q_x1sat__blk817_dn8 - locals.var_x1_wi0__blk908_dn8) * 0.3333333333333)) / assign30360_e33179), ((assign30360_e33178 * ((locals.var_q_x1sat__blk817_dn9 - locals.var_x1_wi0__blk908_dn9) * 0.3333333333333)) / assign30360_e33179),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign30360_e33182;
        locals.var_q_temp3__blk816_dn4 = assign30360_e33182_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign30360_e33182_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign30360_e33182_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign30360_e33182_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign30360_e33182_d_n9;

        let (assign30370_e33193, assign30370_e33193_d_n4, assign30370_e33193_d_n6, assign30370_e33193_d_n7, assign30370_e33193_d_n8, assign30370_e33193_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1098 == 0.0)) {
        let assign30370_e33189: f64 = (locals.var_q_x1sat__blk817 - locals.var_x1_wi0__blk908);
        let assign30370_e33191: f64 = (assign30370_e33189 * 0.3333333333333);
        (assign30370_e33191, ((locals.var_q_x1sat__blk817_dn4 - locals.var_x1_wi0__blk908_dn4) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn6 - locals.var_x1_wi0__blk908_dn6) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn7 - locals.var_x1_wi0__blk908_dn7) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn8 - locals.var_x1_wi0__blk908_dn8) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn9 - locals.var_x1_wi0__blk908_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign30370_e33193;
        locals.var_q_temp3__blk816_dn4 = assign30370_e33193_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign30370_e33193_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign30370_e33193_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign30370_e33193_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign30370_e33193_d_n9;

        let (assign30380_e33201, assign30380_e33201_d_n4, assign30380_e33201_d_n6, assign30380_e33201_d_n7, assign30380_e33201_d_n8, assign30380_e33201_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30380_e33198: f64 = (3.0 * locals.var_q_temp3__blk816);
        let assign30380_e33199: f64 = (locals.var_q_x1sat__blk817 - assign30380_e33198);
        (assign30380_e33199, (locals.var_q_x1sat__blk817_dn4 - (3.0 * locals.var_q_temp3__blk816_dn4)), (locals.var_q_x1sat__blk817_dn6 - (3.0 * locals.var_q_temp3__blk816_dn6)), (locals.var_q_x1sat__blk817_dn7 - (3.0 * locals.var_q_temp3__blk816_dn7)), (locals.var_q_x1sat__blk817_dn8 - (3.0 * locals.var_q_temp3__blk816_dn8)), (locals.var_q_x1sat__blk817_dn9 - (3.0 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_x1__blk821, locals.var_q_x1__blk821_dn4, locals.var_q_x1__blk821_dn6, locals.var_q_x1__blk821_dn7, locals.var_q_x1__blk821_dn8, locals.var_q_x1__blk821_dn9,)
    }
};
        locals.var_q_x1__blk821 = assign30380_e33201;
        locals.var_q_x1__blk821_dn4 = assign30380_e33201_d_n4;
        locals.var_q_x1__blk821_dn6 = assign30380_e33201_d_n6;
        locals.var_q_x1__blk821_dn7 = assign30380_e33201_d_n7;
        locals.var_q_x1__blk821_dn8 = assign30380_e33201_d_n8;
        locals.var_q_x1__blk821_dn9 = assign30380_e33201_d_n9;

        let assign30390_e33204: f64 = (locals.var_q_x2sat__blk818 - locals.var_x2_wi0__blk909);
        let assign30390_e33206: f64 = (assign30390_e33204 * 0.3333333333333);
        let assign30390_e33208: f64 = if assign30390_e33206 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1099 = assign30390_e33208;

        let (assign30400_e33222, assign30400_e33222_d_n4, assign30400_e33222_d_n6, assign30400_e33222_d_n7, assign30400_e33222_d_n8, assign30400_e33222_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1099 != 0.0)) {
        let assign30400_e33215: f64 = (locals.var_q_x2sat__blk818 - locals.var_x2_wi0__blk909);
        let assign30400_e33217: f64 = (assign30400_e33215 * 0.3333333333333);
        let assign30400_e33218: f64 = (assign30400_e33217).exp();
        let assign30400_e33219: f64 = (1.0 + assign30400_e33218);
        let assign30400_e33220: f64 = (assign30400_e33219).ln();
        (assign30400_e33220, ((assign30400_e33218 * ((locals.var_q_x2sat__blk818_dn4 - locals.var_x2_wi0__blk909_dn4) * 0.3333333333333)) / assign30400_e33219), ((assign30400_e33218 * ((locals.var_q_x2sat__blk818_dn6 - locals.var_x2_wi0__blk909_dn6) * 0.3333333333333)) / assign30400_e33219), ((assign30400_e33218 * ((locals.var_q_x2sat__blk818_dn7 - locals.var_x2_wi0__blk909_dn7) * 0.3333333333333)) / assign30400_e33219), ((assign30400_e33218 * ((locals.var_q_x2sat__blk818_dn8 - locals.var_x2_wi0__blk909_dn8) * 0.3333333333333)) / assign30400_e33219), ((assign30400_e33218 * ((locals.var_q_x2sat__blk818_dn9 - locals.var_x2_wi0__blk909_dn9) * 0.3333333333333)) / assign30400_e33219),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign30400_e33222;
        locals.var_q_temp3__blk816_dn4 = assign30400_e33222_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign30400_e33222_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign30400_e33222_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign30400_e33222_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign30400_e33222_d_n9;

        let (assign30410_e33233, assign30410_e33233_d_n4, assign30410_e33233_d_n6, assign30410_e33233_d_n7, assign30410_e33233_d_n8, assign30410_e33233_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1099 == 0.0)) {
        let assign30410_e33229: f64 = (locals.var_q_x2sat__blk818 - locals.var_x2_wi0__blk909);
        let assign30410_e33231: f64 = (assign30410_e33229 * 0.3333333333333);
        (assign30410_e33231, ((locals.var_q_x2sat__blk818_dn4 - locals.var_x2_wi0__blk909_dn4) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn6 - locals.var_x2_wi0__blk909_dn6) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn7 - locals.var_x2_wi0__blk909_dn7) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn8 - locals.var_x2_wi0__blk909_dn8) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn9 - locals.var_x2_wi0__blk909_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign30410_e33233;
        locals.var_q_temp3__blk816_dn4 = assign30410_e33233_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign30410_e33233_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign30410_e33233_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign30410_e33233_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign30410_e33233_d_n9;

        let (assign30420_e33241, assign30420_e33241_d_n4, assign30420_e33241_d_n6, assign30420_e33241_d_n7, assign30420_e33241_d_n8, assign30420_e33241_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30420_e33238: f64 = (3.0 * locals.var_q_temp3__blk816);
        let assign30420_e33239: f64 = (locals.var_q_x2sat__blk818 - assign30420_e33238);
        (assign30420_e33239, (locals.var_q_x2sat__blk818_dn4 - (3.0 * locals.var_q_temp3__blk816_dn4)), (locals.var_q_x2sat__blk818_dn6 - (3.0 * locals.var_q_temp3__blk816_dn6)), (locals.var_q_x2sat__blk818_dn7 - (3.0 * locals.var_q_temp3__blk816_dn7)), (locals.var_q_x2sat__blk818_dn8 - (3.0 * locals.var_q_temp3__blk816_dn8)), (locals.var_q_x2sat__blk818_dn9 - (3.0 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_x2__blk822, locals.var_q_x2__blk822_dn4, locals.var_q_x2__blk822_dn6, locals.var_q_x2__blk822_dn7, locals.var_q_x2__blk822_dn8, locals.var_q_x2__blk822_dn9,)
    }
};
        locals.var_q_x2__blk822 = assign30420_e33241;
        locals.var_q_x2__blk822_dn4 = assign30420_e33241_d_n4;
        locals.var_q_x2__blk822_dn6 = assign30420_e33241_d_n6;
        locals.var_q_x2__blk822_dn7 = assign30420_e33241_d_n7;
        locals.var_q_x2__blk822_dn8 = assign30420_e33241_d_n8;
        locals.var_q_x2__blk822_dn9 = assign30420_e33241_d_n9;

        let (assign30430_e33251, assign30430_e33251_d_n4, assign30430_e33251_d_n6, assign30430_e33251_d_n7, assign30430_e33251_d_n8, assign30430_e33251_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30430_e33245: f64 = (locals.var_k1__blk932 * locals.var_xg1x__blk930);
        let assign30430_e33247: f64 = (assign30430_e33245 + locals.var_q_x2__blk822);
        let assign30430_e33249: f64 = (assign30430_e33247 * locals.var_q_temp1__blk814);
        (assign30430_e33249, (((((locals.var_k1__blk932_dn4 * locals.var_xg1x__blk930) + (locals.var_k1__blk932 * locals.var_xg1x__blk930_dn4)) + locals.var_q_x2__blk822_dn4) * locals.var_q_temp1__blk814) + (assign30430_e33247 * locals.var_q_temp1__blk814_dn4)), (((((locals.var_k1__blk932_dn6 * locals.var_xg1x__blk930) + (locals.var_k1__blk932 * locals.var_xg1x__blk930_dn6)) + locals.var_q_x2__blk822_dn6) * locals.var_q_temp1__blk814) + (assign30430_e33247 * locals.var_q_temp1__blk814_dn6)), (((((locals.var_k1__blk932_dn7 * locals.var_xg1x__blk930) + (locals.var_k1__blk932 * locals.var_xg1x__blk930_dn7)) + locals.var_q_x2__blk822_dn7) * locals.var_q_temp1__blk814) + (assign30430_e33247 * locals.var_q_temp1__blk814_dn7)), (((((locals.var_k1__blk932_dn8 * locals.var_xg1x__blk930) + (locals.var_k1__blk932 * locals.var_xg1x__blk930_dn8)) + locals.var_q_x2__blk822_dn8) * locals.var_q_temp1__blk814) + (assign30430_e33247 * locals.var_q_temp1__blk814_dn8)), (((((locals.var_k1__blk932_dn9 * locals.var_xg1x__blk930) + (locals.var_k1__blk932 * locals.var_xg1x__blk930_dn9)) + locals.var_q_x2__blk822_dn9) * locals.var_q_temp1__blk814) + (assign30430_e33247 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_x1_wi__blk819, locals.var_q_x1_wi__blk819_dn4, locals.var_q_x1_wi__blk819_dn6, locals.var_q_x1_wi__blk819_dn7, locals.var_q_x1_wi__blk819_dn8, locals.var_q_x1_wi__blk819_dn9,)
    }
};
        locals.var_q_x1_wi__blk819 = assign30430_e33251;
        locals.var_q_x1_wi__blk819_dn4 = assign30430_e33251_d_n4;
        locals.var_q_x1_wi__blk819_dn6 = assign30430_e33251_d_n6;
        locals.var_q_x1_wi__blk819_dn7 = assign30430_e33251_d_n7;
        locals.var_q_x1_wi__blk819_dn8 = assign30430_e33251_d_n8;
        locals.var_q_x1_wi__blk819_dn9 = assign30430_e33251_d_n9;

        let (assign30440_e33261, assign30440_e33261_d_n4, assign30440_e33261_d_n6, assign30440_e33261_d_n7, assign30440_e33261_d_n8, assign30440_e33261_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30440_e33255: f64 = (locals.var_k2__blk933 * locals.var_xg2x__blk931);
        let assign30440_e33257: f64 = (assign30440_e33255 + locals.var_q_x1__blk821);
        let assign30440_e33259: f64 = (assign30440_e33257 * locals.var_q_temp2__blk815);
        (assign30440_e33259, (((((locals.var_k2__blk933_dn4 * locals.var_xg2x__blk931) + (locals.var_k2__blk933 * locals.var_xg2x__blk931_dn4)) + locals.var_q_x1__blk821_dn4) * locals.var_q_temp2__blk815) + (assign30440_e33257 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_k2__blk933_dn6 * locals.var_xg2x__blk931) + (locals.var_k2__blk933 * locals.var_xg2x__blk931_dn6)) + locals.var_q_x1__blk821_dn6) * locals.var_q_temp2__blk815) + (assign30440_e33257 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_k2__blk933_dn7 * locals.var_xg2x__blk931) + (locals.var_k2__blk933 * locals.var_xg2x__blk931_dn7)) + locals.var_q_x1__blk821_dn7) * locals.var_q_temp2__blk815) + (assign30440_e33257 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_k2__blk933_dn8 * locals.var_xg2x__blk931) + (locals.var_k2__blk933 * locals.var_xg2x__blk931_dn8)) + locals.var_q_x1__blk821_dn8) * locals.var_q_temp2__blk815) + (assign30440_e33257 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_k2__blk933_dn9 * locals.var_xg2x__blk931) + (locals.var_k2__blk933 * locals.var_xg2x__blk931_dn9)) + locals.var_q_x1__blk821_dn9) * locals.var_q_temp2__blk815) + (assign30440_e33257 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_x2_wi__blk820, locals.var_q_x2_wi__blk820_dn4, locals.var_q_x2_wi__blk820_dn6, locals.var_q_x2_wi__blk820_dn7, locals.var_q_x2_wi__blk820_dn8, locals.var_q_x2_wi__blk820_dn9,)
    }
};
        locals.var_q_x2_wi__blk820 = assign30440_e33261;
        locals.var_q_x2_wi__blk820_dn4 = assign30440_e33261_d_n4;
        locals.var_q_x2_wi__blk820_dn6 = assign30440_e33261_d_n6;
        locals.var_q_x2_wi__blk820_dn7 = assign30440_e33261_d_n7;
        locals.var_q_x2_wi__blk820_dn8 = assign30440_e33261_d_n8;
        locals.var_q_x2_wi__blk820_dn9 = assign30440_e33261_d_n9;

        let assign30450_e33264: f64 = (locals.var_q_x1sat__blk817 - locals.var_q_x1_wi__blk819);
        let assign30450_e33266: f64 = (assign30450_e33264 * 0.3333333333333);
        let assign30450_e33268: f64 = if assign30450_e33266 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1100 = assign30450_e33268;

        let (assign30460_e33282, assign30460_e33282_d_n4, assign30460_e33282_d_n6, assign30460_e33282_d_n7, assign30460_e33282_d_n8, assign30460_e33282_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1100 != 0.0)) {
        let assign30460_e33275: f64 = (locals.var_q_x1sat__blk817 - locals.var_q_x1_wi__blk819);
        let assign30460_e33277: f64 = (assign30460_e33275 * 0.3333333333333);
        let assign30460_e33278: f64 = (assign30460_e33277).exp();
        let assign30460_e33279: f64 = (1.0 + assign30460_e33278);
        let assign30460_e33280: f64 = (assign30460_e33279).ln();
        (assign30460_e33280, ((assign30460_e33278 * ((locals.var_q_x1sat__blk817_dn4 - locals.var_q_x1_wi__blk819_dn4) * 0.3333333333333)) / assign30460_e33279), ((assign30460_e33278 * ((locals.var_q_x1sat__blk817_dn6 - locals.var_q_x1_wi__blk819_dn6) * 0.3333333333333)) / assign30460_e33279), ((assign30460_e33278 * ((locals.var_q_x1sat__blk817_dn7 - locals.var_q_x1_wi__blk819_dn7) * 0.3333333333333)) / assign30460_e33279), ((assign30460_e33278 * ((locals.var_q_x1sat__blk817_dn8 - locals.var_q_x1_wi__blk819_dn8) * 0.3333333333333)) / assign30460_e33279), ((assign30460_e33278 * ((locals.var_q_x1sat__blk817_dn9 - locals.var_q_x1_wi__blk819_dn9) * 0.3333333333333)) / assign30460_e33279),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign30460_e33282;
        locals.var_q_temp3__blk816_dn4 = assign30460_e33282_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign30460_e33282_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign30460_e33282_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign30460_e33282_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign30460_e33282_d_n9;

        let (assign30470_e33293, assign30470_e33293_d_n4, assign30470_e33293_d_n6, assign30470_e33293_d_n7, assign30470_e33293_d_n8, assign30470_e33293_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1100 == 0.0)) {
        let assign30470_e33289: f64 = (locals.var_q_x1sat__blk817 - locals.var_q_x1_wi__blk819);
        let assign30470_e33291: f64 = (assign30470_e33289 * 0.3333333333333);
        (assign30470_e33291, ((locals.var_q_x1sat__blk817_dn4 - locals.var_q_x1_wi__blk819_dn4) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn6 - locals.var_q_x1_wi__blk819_dn6) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn7 - locals.var_q_x1_wi__blk819_dn7) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn8 - locals.var_q_x1_wi__blk819_dn8) * 0.3333333333333), ((locals.var_q_x1sat__blk817_dn9 - locals.var_q_x1_wi__blk819_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign30470_e33293;
        locals.var_q_temp3__blk816_dn4 = assign30470_e33293_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign30470_e33293_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign30470_e33293_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign30470_e33293_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign30470_e33293_d_n9;

        let (assign30480_e33301, assign30480_e33301_d_n4, assign30480_e33301_d_n6, assign30480_e33301_d_n7, assign30480_e33301_d_n8, assign30480_e33301_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30480_e33298: f64 = (3.0 * locals.var_q_temp3__blk816);
        let assign30480_e33299: f64 = (locals.var_q_x1sat__blk817 - assign30480_e33298);
        (assign30480_e33299, (locals.var_q_x1sat__blk817_dn4 - (3.0 * locals.var_q_temp3__blk816_dn4)), (locals.var_q_x1sat__blk817_dn6 - (3.0 * locals.var_q_temp3__blk816_dn6)), (locals.var_q_x1sat__blk817_dn7 - (3.0 * locals.var_q_temp3__blk816_dn7)), (locals.var_q_x1sat__blk817_dn8 - (3.0 * locals.var_q_temp3__blk816_dn8)), (locals.var_q_x1sat__blk817_dn9 - (3.0 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_x1__blk821, locals.var_q_x1__blk821_dn4, locals.var_q_x1__blk821_dn6, locals.var_q_x1__blk821_dn7, locals.var_q_x1__blk821_dn8, locals.var_q_x1__blk821_dn9,)
    }
};
        locals.var_q_x1__blk821 = assign30480_e33301;
        locals.var_q_x1__blk821_dn4 = assign30480_e33301_d_n4;
        locals.var_q_x1__blk821_dn6 = assign30480_e33301_d_n6;
        locals.var_q_x1__blk821_dn7 = assign30480_e33301_d_n7;
        locals.var_q_x1__blk821_dn8 = assign30480_e33301_d_n8;
        locals.var_q_x1__blk821_dn9 = assign30480_e33301_d_n9;

        let assign30490_e33304: f64 = (locals.var_q_x2sat__blk818 - locals.var_q_x2_wi__blk820);
        let assign30490_e33306: f64 = (assign30490_e33304 * 0.3333333333333);
        let assign30490_e33308: f64 = if assign30490_e33306 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1101 = assign30490_e33308;

        let (assign30500_e33322, assign30500_e33322_d_n4, assign30500_e33322_d_n6, assign30500_e33322_d_n7, assign30500_e33322_d_n8, assign30500_e33322_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1101 != 0.0)) {
        let assign30500_e33315: f64 = (locals.var_q_x2sat__blk818 - locals.var_q_x2_wi__blk820);
        let assign30500_e33317: f64 = (assign30500_e33315 * 0.3333333333333);
        let assign30500_e33318: f64 = (assign30500_e33317).exp();
        let assign30500_e33319: f64 = (1.0 + assign30500_e33318);
        let assign30500_e33320: f64 = (assign30500_e33319).ln();
        (assign30500_e33320, ((assign30500_e33318 * ((locals.var_q_x2sat__blk818_dn4 - locals.var_q_x2_wi__blk820_dn4) * 0.3333333333333)) / assign30500_e33319), ((assign30500_e33318 * ((locals.var_q_x2sat__blk818_dn6 - locals.var_q_x2_wi__blk820_dn6) * 0.3333333333333)) / assign30500_e33319), ((assign30500_e33318 * ((locals.var_q_x2sat__blk818_dn7 - locals.var_q_x2_wi__blk820_dn7) * 0.3333333333333)) / assign30500_e33319), ((assign30500_e33318 * ((locals.var_q_x2sat__blk818_dn8 - locals.var_q_x2_wi__blk820_dn8) * 0.3333333333333)) / assign30500_e33319), ((assign30500_e33318 * ((locals.var_q_x2sat__blk818_dn9 - locals.var_q_x2_wi__blk820_dn9) * 0.3333333333333)) / assign30500_e33319),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign30500_e33322;
        locals.var_q_temp3__blk816_dn4 = assign30500_e33322_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign30500_e33322_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign30500_e33322_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign30500_e33322_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign30500_e33322_d_n9;

        let (assign30510_e33333, assign30510_e33333_d_n4, assign30510_e33333_d_n6, assign30510_e33333_d_n7, assign30510_e33333_d_n8, assign30510_e33333_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1101 == 0.0)) {
        let assign30510_e33329: f64 = (locals.var_q_x2sat__blk818 - locals.var_q_x2_wi__blk820);
        let assign30510_e33331: f64 = (assign30510_e33329 * 0.3333333333333);
        (assign30510_e33331, ((locals.var_q_x2sat__blk818_dn4 - locals.var_q_x2_wi__blk820_dn4) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn6 - locals.var_q_x2_wi__blk820_dn6) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn7 - locals.var_q_x2_wi__blk820_dn7) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn8 - locals.var_q_x2_wi__blk820_dn8) * 0.3333333333333), ((locals.var_q_x2sat__blk818_dn9 - locals.var_q_x2_wi__blk820_dn9) * 0.3333333333333),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign30510_e33333;
        locals.var_q_temp3__blk816_dn4 = assign30510_e33333_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign30510_e33333_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign30510_e33333_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign30510_e33333_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign30510_e33333_d_n9;

        let (assign30520_e33341, assign30520_e33341_d_n4, assign30520_e33341_d_n6, assign30520_e33341_d_n7, assign30520_e33341_d_n8, assign30520_e33341_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30520_e33338: f64 = (3.0 * locals.var_q_temp3__blk816);
        let assign30520_e33339: f64 = (locals.var_q_x2sat__blk818 - assign30520_e33338);
        (assign30520_e33339, (locals.var_q_x2sat__blk818_dn4 - (3.0 * locals.var_q_temp3__blk816_dn4)), (locals.var_q_x2sat__blk818_dn6 - (3.0 * locals.var_q_temp3__blk816_dn6)), (locals.var_q_x2sat__blk818_dn7 - (3.0 * locals.var_q_temp3__blk816_dn7)), (locals.var_q_x2sat__blk818_dn8 - (3.0 * locals.var_q_temp3__blk816_dn8)), (locals.var_q_x2sat__blk818_dn9 - (3.0 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_x2__blk822, locals.var_q_x2__blk822_dn4, locals.var_q_x2__blk822_dn6, locals.var_q_x2__blk822_dn7, locals.var_q_x2__blk822_dn8, locals.var_q_x2__blk822_dn9,)
    }
};
        locals.var_q_x2__blk822 = assign30520_e33341;
        locals.var_q_x2__blk822_dn4 = assign30520_e33341_d_n4;
        locals.var_q_x2__blk822_dn6 = assign30520_e33341_d_n6;
        locals.var_q_x2__blk822_dn7 = assign30520_e33341_d_n7;
        locals.var_q_x2__blk822_dn8 = assign30520_e33341_d_n8;
        locals.var_q_x2__blk822_dn9 = assign30520_e33341_d_n9;

    }

    pub(super) fn stamp_transient_block_81(
        locals: &mut StampLocals,
    ) {
        let (assign30530_e33347, assign30530_e33347_d_n4, assign30530_e33347_d_n6, assign30530_e33347_d_n7, assign30530_e33347_d_n8, assign30530_e33347_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30530_e33345: f64 = (locals.var_xg1x__blk930 - locals.var_q_x1__blk821);
        (assign30530_e33345, (locals.var_xg1x__blk930_dn4 - locals.var_q_x1__blk821_dn4), (locals.var_xg1x__blk930_dn6 - locals.var_q_x1__blk821_dn6), (locals.var_xg1x__blk930_dn7 - locals.var_q_x1__blk821_dn7), (locals.var_xg1x__blk930_dn8 - locals.var_q_x1__blk821_dn8), (locals.var_xg1x__blk930_dn9 - locals.var_q_x1__blk821_dn9),)
    } else {
        (locals.var_q1s__blk937, locals.var_q1s__blk937_dn4, locals.var_q1s__blk937_dn6, locals.var_q1s__blk937_dn7, locals.var_q1s__blk937_dn8, locals.var_q1s__blk937_dn9,)
    }
};
        locals.var_q1s__blk937 = assign30530_e33347;
        locals.var_q1s__blk937_dn4 = assign30530_e33347_d_n4;
        locals.var_q1s__blk937_dn6 = assign30530_e33347_d_n6;
        locals.var_q1s__blk937_dn7 = assign30530_e33347_d_n7;
        locals.var_q1s__blk937_dn8 = assign30530_e33347_d_n8;
        locals.var_q1s__blk937_dn9 = assign30530_e33347_d_n9;

        let (assign30540_e33353, assign30540_e33353_d_n4, assign30540_e33353_d_n6, assign30540_e33353_d_n7, assign30540_e33353_d_n8, assign30540_e33353_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30540_e33351: f64 = (locals.var_xg2x__blk931 - locals.var_q_x2__blk822);
        (assign30540_e33351, (locals.var_xg2x__blk931_dn4 - locals.var_q_x2__blk822_dn4), (locals.var_xg2x__blk931_dn6 - locals.var_q_x2__blk822_dn6), (locals.var_xg2x__blk931_dn7 - locals.var_q_x2__blk822_dn7), (locals.var_xg2x__blk931_dn8 - locals.var_q_x2__blk822_dn8), (locals.var_xg2x__blk931_dn9 - locals.var_q_x2__blk822_dn9),)
    } else {
        (locals.var_q2s__blk941, locals.var_q2s__blk941_dn4, locals.var_q2s__blk941_dn6, locals.var_q2s__blk941_dn7, locals.var_q2s__blk941_dn8, locals.var_q2s__blk941_dn9,)
    }
};
        locals.var_q2s__blk941 = assign30540_e33353;
        locals.var_q2s__blk941_dn4 = assign30540_e33353_d_n4;
        locals.var_q2s__blk941_dn6 = assign30540_e33353_d_n6;
        locals.var_q2s__blk941_dn7 = assign30540_e33353_d_n7;
        locals.var_q2s__blk941_dn8 = assign30540_e33353_d_n8;
        locals.var_q2s__blk941_dn9 = assign30540_e33353_d_n9;

        let (assign30550_e33357, assign30550_e33357_d_n4, assign30550_e33357_d_n6, assign30550_e33357_d_n7, assign30550_e33357_d_n8, assign30550_e33357_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign30550_e33357;
        locals.var_q_rac_qsq__blk828_dn4 = assign30550_e33357_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign30550_e33357_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign30550_e33357_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign30550_e33357_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign30550_e33357_d_n9;

        let (assign30560_e33361, assign30560_e33361_d_n4, assign30560_e33361_d_n6, assign30560_e33361_d_n7, assign30560_e33361_d_n8, assign30560_e33361_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign30560_e33361;
        locals.var_q_invexpq__blk831_dn4 = assign30560_e33361_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign30560_e33361_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign30560_e33361_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign30560_e33361_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign30560_e33361_d_n9;

        let (assign30570_e33367, assign30570_e33367_d_n4, assign30570_e33367_d_n6, assign30570_e33367_d_n7, assign30570_e33367_d_n8, assign30570_e33367_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30570_e33365: f64 = (locals.var_k1__blk932 * locals.var_q1s__blk937);
        (assign30570_e33365, ((locals.var_k1__blk932_dn4 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn9)),)
    } else {
        (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9,)
    }
};
        locals.var_q_k1q1__blk823 = assign30570_e33367;
        locals.var_q_k1q1__blk823_dn4 = assign30570_e33367_d_n4;
        locals.var_q_k1q1__blk823_dn6 = assign30570_e33367_d_n6;
        locals.var_q_k1q1__blk823_dn7 = assign30570_e33367_d_n7;
        locals.var_q_k1q1__blk823_dn8 = assign30570_e33367_d_n8;
        locals.var_q_k1q1__blk823_dn9 = assign30570_e33367_d_n9;

        let assign30580_e33370: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign30580_e33372: f64 = assign30580_e33370;
        let assign30580_e33374: f64 = if assign30580_e33372 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1102 = assign30580_e33374;

        let (assign30590_e33385, assign30590_e33385_d_n4, assign30590_e33385_d_n6, assign30590_e33385_d_n7, assign30590_e33385_d_n8, assign30590_e33385_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1102 != 0.0)) {
        let assign30590_e33380: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign30590_e33382: f64 = assign30590_e33380;
        let assign30590_e33383: f64 = (assign30590_e33382).exp();
        (assign30590_e33383, (assign30590_e33383 * (locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4)), (assign30590_e33383 * (locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6)), (assign30590_e33383 * (locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7)), (assign30590_e33383 * (locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8)), (assign30590_e33383 * (locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign30590_e33385;
        locals.var_q_temp1__blk814_dn4 = assign30590_e33385_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign30590_e33385_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign30590_e33385_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign30590_e33385_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign30590_e33385_d_n9;

        let (assign30600_e33426, assign30600_e33426_d_n4, assign30600_e33426_d_n6, assign30600_e33426_d_n7, assign30600_e33426_d_n8, assign30600_e33426_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1102 == 0.0)) {
        let assign30600_e33394: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign30600_e33396: f64 = assign30600_e33394;
        let assign30600_e33398: f64 = (assign30600_e33396 - 80.0);
        let assign30600_e33403: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign30600_e33405: f64 = assign30600_e33403;
        let assign30600_e33407: f64 = (assign30600_e33405 - 80.0);
        let assign30600_e33408: f64 = (0.5 * assign30600_e33407);
        let assign30600_e33412: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign30600_e33414: f64 = assign30600_e33412;
        let assign30600_e33416: f64 = (assign30600_e33414 - 80.0);
        let assign30600_e33418: f64 = (assign30600_e33416 * 0.3333333333333);
        let assign30600_e33419: f64 = (1.0 + assign30600_e33418);
        let assign30600_e33420: f64 = (assign30600_e33408 * assign30600_e33419);
        let assign30600_e33421: f64 = (1.0 + assign30600_e33420);
        let assign30600_e33422: f64 = (assign30600_e33398 * assign30600_e33421);
        let assign30600_e33423: f64 = (1.0 + assign30600_e33422);
        let assign30600_e33424: f64 = (5.54062e34 * assign30600_e33423);
        (assign30600_e33424, (5.54062e34 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4) * assign30600_e33421) + (assign30600_e33398 * (((0.5 * (locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4)) * assign30600_e33419) + (assign30600_e33408 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6) * assign30600_e33421) + (assign30600_e33398 * (((0.5 * (locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6)) * assign30600_e33419) + (assign30600_e33408 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7) * assign30600_e33421) + (assign30600_e33398 * (((0.5 * (locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7)) * assign30600_e33419) + (assign30600_e33408 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8) * assign30600_e33421) + (assign30600_e33398 * (((0.5 * (locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8)) * assign30600_e33419) + (assign30600_e33408 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9) * assign30600_e33421) + (assign30600_e33398 * (((0.5 * (locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9)) * assign30600_e33419) + (assign30600_e33408 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign30600_e33426;
        locals.var_q_temp1__blk814_dn4 = assign30600_e33426_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign30600_e33426_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign30600_e33426_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign30600_e33426_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign30600_e33426_d_n9;

        let (assign30610_e33432, assign30610_e33432_d_n4, assign30610_e33432_d_n6, assign30610_e33432_d_n7, assign30610_e33432_d_n8, assign30610_e33432_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30610_e33430: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
        (assign30610_e33430, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_aexp__blk824, locals.var_q_aexp__blk824_dn4, locals.var_q_aexp__blk824_dn6, locals.var_q_aexp__blk824_dn7, locals.var_q_aexp__blk824_dn8, locals.var_q_aexp__blk824_dn9,)
    }
};
        locals.var_q_aexp__blk824 = assign30610_e33432;
        locals.var_q_aexp__blk824_dn4 = assign30610_e33432_d_n4;
        locals.var_q_aexp__blk824_dn6 = assign30610_e33432_d_n6;
        locals.var_q_aexp__blk824_dn7 = assign30610_e33432_d_n7;
        locals.var_q_aexp__blk824_dn8 = assign30610_e33432_d_n8;
        locals.var_q_aexp__blk824_dn9 = assign30610_e33432_d_n9;

        let (assign30620_e33440, assign30620_e33440_d_n4, assign30620_e33440_d_n6, assign30620_e33440_d_n7, assign30620_e33440_d_n8, assign30620_e33440_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30620_e33436: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
        let assign30620_e33438: f64 = (assign30620_e33436 - locals.var_q_aexp__blk824);
        (assign30620_e33438, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign30620_e33440;
        locals.var_q_qsq__blk825_dn4 = assign30620_e33440_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign30620_e33440_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign30620_e33440_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign30620_e33440_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign30620_e33440_d_n9;

        let (assign30630_e33450, assign30630_e33450_d_n4, assign30630_e33450_d_n6, assign30630_e33450_d_n7, assign30630_e33450_d_n8, assign30630_e33450_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30630_e33444: f64 = (2.0 * locals.var_k1__blk932);
        let assign30630_e33446: f64 = (assign30630_e33444 * locals.var_q_k1q1__blk823);
        let assign30630_e33448: f64 = (assign30630_e33446 + locals.var_q_aexp__blk824);
        (assign30630_e33448, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign30630_e33444 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign30630_e33444 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign30630_e33444 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign30630_e33444 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign30630_e33444 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_qsq__blk826, locals.var_q_d1_qsq__blk826_dn4, locals.var_q_d1_qsq__blk826_dn6, locals.var_q_d1_qsq__blk826_dn7, locals.var_q_d1_qsq__blk826_dn8, locals.var_q_d1_qsq__blk826_dn9,)
    }
};
        locals.var_q_d1_qsq__blk826 = assign30630_e33450;
        locals.var_q_d1_qsq__blk826_dn4 = assign30630_e33450_d_n4;
        locals.var_q_d1_qsq__blk826_dn6 = assign30630_e33450_d_n6;
        locals.var_q_d1_qsq__blk826_dn7 = assign30630_e33450_d_n7;
        locals.var_q_d1_qsq__blk826_dn8 = assign30630_e33450_d_n8;
        locals.var_q_d1_qsq__blk826_dn9 = assign30630_e33450_d_n9;

        let (assign30640_e33460, assign30640_e33460_d_n4, assign30640_e33460_d_n6, assign30640_e33460_d_n7, assign30640_e33460_d_n8, assign30640_e33460_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign30640_e33454: f64 = (2.0 * locals.var_k1__blk932);
        let assign30640_e33456: f64 = (assign30640_e33454 * locals.var_k1__blk932);
        let assign30640_e33458: f64 = (assign30640_e33456 - locals.var_q_aexp__blk824);
        (assign30640_e33458, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_k1__blk932) + (assign30640_e33454 * locals.var_k1__blk932_dn4)) - locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_k1__blk932) + (assign30640_e33454 * locals.var_k1__blk932_dn6)) - locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_k1__blk932) + (assign30640_e33454 * locals.var_k1__blk932_dn7)) - locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_k1__blk932) + (assign30640_e33454 * locals.var_k1__blk932_dn8)) - locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_k1__blk932) + (assign30640_e33454 * locals.var_k1__blk932_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_qsq__blk827, locals.var_q_d2_qsq__blk827_dn4, locals.var_q_d2_qsq__blk827_dn6, locals.var_q_d2_qsq__blk827_dn7, locals.var_q_d2_qsq__blk827_dn8, locals.var_q_d2_qsq__blk827_dn9,)
    }
};
        locals.var_q_d2_qsq__blk827 = assign30640_e33460;
        locals.var_q_d2_qsq__blk827_dn4 = assign30640_e33460_d_n4;
        locals.var_q_d2_qsq__blk827_dn6 = assign30640_e33460_d_n6;
        locals.var_q_d2_qsq__blk827_dn7 = assign30640_e33460_d_n7;
        locals.var_q_d2_qsq__blk827_dn8 = assign30640_e33460_d_n8;
        locals.var_q_d2_qsq__blk827_dn9 = assign30640_e33460_d_n9;

        let assign30650_e33463: f64 = (-0.005);
        let assign30650_e33464: f64 = if locals.var_q_qsq__blk825 < assign30650_e33463 { 1.0 } else { 0.0 };
        locals.var_guard1103 = assign30650_e33464;

        let (assign30660_e33472, assign30660_e33472_d_n4, assign30660_e33472_d_n6, assign30660_e33472_d_n7, assign30660_e33472_d_n8, assign30660_e33472_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1103 != 0.0)) {
        let assign30660_e33469: f64 = (locals.var_q_qsq__blk825).abs();
        let assign30660_e33470: f64 = (assign30660_e33469).sqrt();
        (assign30660_e33470, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign30660_e33470)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign30660_e33470)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign30660_e33470)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign30660_e33470)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign30660_e33470)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign30660_e33472;
        locals.var_q_rac_qsq__blk828_dn4 = assign30660_e33472_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign30660_e33472_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign30660_e33472_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign30660_e33472_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign30660_e33472_d_n9;

        let (assign30670_e33483, assign30670_e33483_d_n4, assign30670_e33483_d_n6, assign30670_e33483_d_n7, assign30670_e33483_d_n8, assign30670_e33483_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1103 != 0.0)) {
        let assign30670_e33479: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign30670_e33480: f64 = (assign30670_e33479).tan();
        let assign30670_e33481: f64 = (locals.var_q_rac_qsq__blk828 / assign30670_e33480);
        (assign30670_e33481, (((locals.var_q_rac_qsq__blk828_dn4 * assign30670_e33480) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign30670_e33479).cos() * (assign30670_e33479).cos())))) / (assign30670_e33480 * assign30670_e33480)), (((locals.var_q_rac_qsq__blk828_dn6 * assign30670_e33480) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign30670_e33479).cos() * (assign30670_e33479).cos())))) / (assign30670_e33480 * assign30670_e33480)), (((locals.var_q_rac_qsq__blk828_dn7 * assign30670_e33480) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign30670_e33479).cos() * (assign30670_e33479).cos())))) / (assign30670_e33480 * assign30670_e33480)), (((locals.var_q_rac_qsq__blk828_dn8 * assign30670_e33480) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign30670_e33479).cos() * (assign30670_e33479).cos())))) / (assign30670_e33480 * assign30670_e33480)), (((locals.var_q_rac_qsq__blk828_dn9 * assign30670_e33480) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign30670_e33479).cos() * (assign30670_e33479).cos())))) / (assign30670_e33480 * assign30670_e33480)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign30670_e33483;
        locals.var_q_qcoth__blk829_dn4 = assign30670_e33483_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign30670_e33483_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign30670_e33483_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign30670_e33483_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign30670_e33483_d_n9;

        let (assign30680_e33493, assign30680_e33493_d_n4, assign30680_e33493_d_n6, assign30680_e33493_d_n7, assign30680_e33493_d_n8, assign30680_e33493_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1103 != 0.0)) {
        let assign30680_e33489: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign30680_e33491: f64 = (assign30680_e33489 / locals.var_q_qsq__blk825);
        (assign30680_e33491, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign30680_e33489 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign30680_e33489 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign30680_e33489 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign30680_e33489 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign30680_e33489 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign30680_e33493;
        locals.var_q_temp1__blk814_dn4 = assign30680_e33493_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign30680_e33493_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign30680_e33493_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign30680_e33493_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign30680_e33493_d_n9;

        let (assign30690_e33507, assign30690_e33507_d_n4, assign30690_e33507_d_n6, assign30690_e33507_d_n7, assign30690_e33507_d_n8, assign30690_e33507_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1103 != 0.0)) {
        let assign30690_e33501: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign30690_e33502: f64 = (locals.var_q_qcoth__blk829 * assign30690_e33501);
        let assign30690_e33503: f64 = (locals.var_q_qsq__blk825 + assign30690_e33502);
        let assign30690_e33505: f64 = (assign30690_e33503 * locals.var_q_temp1__blk814);
        (assign30690_e33505, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign30690_e33501) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign30690_e33503 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign30690_e33501) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign30690_e33503 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign30690_e33501) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign30690_e33503 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign30690_e33501) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign30690_e33503 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign30690_e33501) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign30690_e33503 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign30690_e33507;
        locals.var_q_d1_qcoth__blk830_dn4 = assign30690_e33507_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign30690_e33507_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign30690_e33507_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign30690_e33507_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign30690_e33507_d_n9;

        let (assign30700_e33529, assign30700_e33529_d_n4, assign30700_e33529_d_n6, assign30700_e33529_d_n7, assign30700_e33529_d_n8, assign30700_e33529_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1103 != 0.0)) {
        let assign30700_e33514: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign30700_e33517: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign30700_e33518: f64 = (assign30700_e33514 * assign30700_e33517);
        let assign30700_e33519: f64 = (locals.var_q_d1_qsq__blk826 - assign30700_e33518);
        let assign30700_e33521: f64 = (assign30700_e33519 * locals.var_q_temp1__blk814);
        let assign30700_e33524: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign30700_e33526: f64 = (assign30700_e33524 / locals.var_q_d1_qsq__blk826);
        let assign30700_e33527: f64 = (assign30700_e33521 + assign30700_e33526);
        (assign30700_e33527, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign30700_e33517) + (assign30700_e33514 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign30700_e33519 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign30700_e33524 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign30700_e33517) + (assign30700_e33514 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign30700_e33519 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign30700_e33524 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign30700_e33517) + (assign30700_e33514 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign30700_e33519 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign30700_e33524 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign30700_e33517) + (assign30700_e33514 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign30700_e33519 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign30700_e33524 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign30700_e33517) + (assign30700_e33514 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign30700_e33519 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign30700_e33524 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign30700_e33529;
        locals.var_q_d2_qcoth__blk832_dn4 = assign30700_e33529_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign30700_e33529_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign30700_e33529_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign30700_e33529_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign30700_e33529_d_n9;

        let (assign30710_e33539, assign30710_e33539_d_n4, assign30710_e33539_d_n6, assign30710_e33539_d_n7, assign30710_e33539_d_n8, assign30710_e33539_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1103 != 0.0)) {
        let assign30710_e33536: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign30710_e33537: f64 = (1.0 - assign30710_e33536);
        (assign30710_e33537, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign30710_e33539;
        locals.var_q_temp2__blk815_dn4 = assign30710_e33539_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign30710_e33539_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign30710_e33539_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign30710_e33539_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign30710_e33539_d_n9;

        let (assign30720_e33549, assign30720_e33549_d_n4, assign30720_e33549_d_n6, assign30720_e33549_d_n7, assign30720_e33549_d_n8, assign30720_e33549_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1103 != 0.0)) {
        let assign30720_e33545: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign30720_e33547: f64 = (assign30720_e33545 * locals.var_q_temp2__blk815);
        (assign30720_e33547, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30720_e33545 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30720_e33545 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30720_e33545 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30720_e33545 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30720_e33545 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign30720_e33549;
        locals.var_q_d1_ln__blk835_dn4 = assign30720_e33549_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign30720_e33549_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign30720_e33549_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign30720_e33549_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign30720_e33549_d_n9;

        let (assign30730_e33567, assign30730_e33567_d_n4, assign30730_e33567_d_n6, assign30730_e33567_d_n7, assign30730_e33567_d_n8, assign30730_e33567_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1103 != 0.0)) {
        let assign30730_e33555: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign30730_e33560: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign30730_e33561: f64 = (locals.var_q_d1_ln__blk835 + assign30730_e33560);
        let assign30730_e33562: f64 = (locals.var_q_d1_qsq__blk826 * assign30730_e33561);
        let assign30730_e33563: f64 = (assign30730_e33555 - assign30730_e33562);
        let assign30730_e33565: f64 = (assign30730_e33563 / locals.var_q_qsq__blk825);
        (assign30730_e33565, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign30730_e33561) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign30730_e33563 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign30730_e33561) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign30730_e33563 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign30730_e33561) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign30730_e33563 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign30730_e33561) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign30730_e33563 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign30730_e33561) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign30730_e33563 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign30730_e33567;
        locals.var_q_d2_ln__blk836_dn4 = assign30730_e33567_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign30730_e33567_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign30730_e33567_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign30730_e33567_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign30730_e33567_d_n9;

        let assign30740_e33570: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1104 = assign30740_e33570;

        let (assign30750_e33581, assign30750_e33581_d_n4, assign30750_e33581_d_n6, assign30750_e33581_d_n7, assign30750_e33581_d_n8, assign30750_e33581_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 != 0.0)) {
        let assign30750_e33578: f64 = (locals.var_q_qsq__blk825).abs();
        let assign30750_e33579: f64 = (assign30750_e33578).sqrt();
        (assign30750_e33579, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign30750_e33579)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign30750_e33579)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign30750_e33579)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign30750_e33579)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign30750_e33579)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign30750_e33581;
        locals.var_q_rac_qsq__blk828_dn4 = assign30750_e33581_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign30750_e33581_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign30750_e33581_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign30750_e33581_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign30750_e33581_d_n9;

        let (assign30760_e33592, assign30760_e33592_d_n4, assign30760_e33592_d_n6, assign30760_e33592_d_n7, assign30760_e33592_d_n8, assign30760_e33592_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 != 0.0)) {
        let assign30760_e33589: f64 = (-locals.var_q_rac_qsq__blk828);
        let assign30760_e33590: f64 = (assign30760_e33589).exp();
        (assign30760_e33590, (assign30760_e33590 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign30760_e33590 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign30760_e33590 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign30760_e33590 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign30760_e33590 * (-locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign30760_e33592;
        locals.var_q_invexpq__blk831_dn4 = assign30760_e33592_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign30760_e33592_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign30760_e33592_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign30760_e33592_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign30760_e33592_d_n9;

        let (assign30770_e33609, assign30770_e33609_d_n4, assign30770_e33609_d_n6, assign30770_e33609_d_n7, assign30770_e33609_d_n8, assign30770_e33609_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 != 0.0)) {
        let assign30770_e33602: f64 = (1.0 + locals.var_q_invexpq__blk831);
        let assign30770_e33603: f64 = (locals.var_q_rac_qsq__blk828 * assign30770_e33602);
        let assign30770_e33606: f64 = (1.0 - locals.var_q_invexpq__blk831);
        let assign30770_e33607: f64 = (assign30770_e33603 / assign30770_e33606);
        (assign30770_e33607, (((((locals.var_q_rac_qsq__blk828_dn4 * assign30770_e33602) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign30770_e33606) - (assign30770_e33603 * (-locals.var_q_invexpq__blk831_dn4))) / (assign30770_e33606 * assign30770_e33606)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign30770_e33602) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign30770_e33606) - (assign30770_e33603 * (-locals.var_q_invexpq__blk831_dn6))) / (assign30770_e33606 * assign30770_e33606)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign30770_e33602) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign30770_e33606) - (assign30770_e33603 * (-locals.var_q_invexpq__blk831_dn7))) / (assign30770_e33606 * assign30770_e33606)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign30770_e33602) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign30770_e33606) - (assign30770_e33603 * (-locals.var_q_invexpq__blk831_dn8))) / (assign30770_e33606 * assign30770_e33606)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign30770_e33602) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign30770_e33606) - (assign30770_e33603 * (-locals.var_q_invexpq__blk831_dn9))) / (assign30770_e33606 * assign30770_e33606)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign30770_e33609;
        locals.var_q_qcoth__blk829_dn4 = assign30770_e33609_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign30770_e33609_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign30770_e33609_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign30770_e33609_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign30770_e33609_d_n9;

        let (assign30780_e33622, assign30780_e33622_d_n4, assign30780_e33622_d_n6, assign30780_e33622_d_n7, assign30780_e33622_d_n8, assign30780_e33622_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 != 0.0)) {
        let assign30780_e33618: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign30780_e33620: f64 = (assign30780_e33618 / locals.var_q_qsq__blk825);
        (assign30780_e33620, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign30780_e33618 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign30780_e33618 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign30780_e33618 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign30780_e33618 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign30780_e33618 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign30780_e33622;
        locals.var_q_temp1__blk814_dn4 = assign30780_e33622_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign30780_e33622_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign30780_e33622_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign30780_e33622_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign30780_e33622_d_n9;

        let (assign30790_e33639, assign30790_e33639_d_n4, assign30790_e33639_d_n6, assign30790_e33639_d_n7, assign30790_e33639_d_n8, assign30790_e33639_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 != 0.0)) {
        let assign30790_e33633: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign30790_e33634: f64 = (locals.var_q_qcoth__blk829 * assign30790_e33633);
        let assign30790_e33635: f64 = (locals.var_q_qsq__blk825 + assign30790_e33634);
        let assign30790_e33637: f64 = (assign30790_e33635 * locals.var_q_temp1__blk814);
        (assign30790_e33637, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign30790_e33633) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign30790_e33635 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign30790_e33633) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign30790_e33635 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign30790_e33633) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign30790_e33635 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign30790_e33633) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign30790_e33635 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign30790_e33633) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign30790_e33635 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign30790_e33639;
        locals.var_q_d1_qcoth__blk830_dn4 = assign30790_e33639_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign30790_e33639_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign30790_e33639_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign30790_e33639_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign30790_e33639_d_n9;

        let (assign30800_e33664, assign30800_e33664_d_n4, assign30800_e33664_d_n6, assign30800_e33664_d_n7, assign30800_e33664_d_n8, assign30800_e33664_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 != 0.0)) {
        let assign30800_e33649: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign30800_e33652: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign30800_e33653: f64 = (assign30800_e33649 * assign30800_e33652);
        let assign30800_e33654: f64 = (locals.var_q_d1_qsq__blk826 - assign30800_e33653);
        let assign30800_e33656: f64 = (assign30800_e33654 * locals.var_q_temp1__blk814);
        let assign30800_e33659: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign30800_e33661: f64 = (assign30800_e33659 / locals.var_q_d1_qsq__blk826);
        let assign30800_e33662: f64 = (assign30800_e33656 + assign30800_e33661);
        (assign30800_e33662, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign30800_e33652) + (assign30800_e33649 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign30800_e33654 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign30800_e33659 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign30800_e33652) + (assign30800_e33649 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign30800_e33654 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign30800_e33659 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign30800_e33652) + (assign30800_e33649 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign30800_e33654 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign30800_e33659 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign30800_e33652) + (assign30800_e33649 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign30800_e33654 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign30800_e33659 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign30800_e33652) + (assign30800_e33649 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign30800_e33654 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign30800_e33659 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign30800_e33664;
        locals.var_q_d2_qcoth__blk832_dn4 = assign30800_e33664_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign30800_e33664_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign30800_e33664_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign30800_e33664_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign30800_e33664_d_n9;

        let (assign30810_e33677, assign30810_e33677_d_n4, assign30810_e33677_d_n6, assign30810_e33677_d_n7, assign30810_e33677_d_n8, assign30810_e33677_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 != 0.0)) {
        let assign30810_e33674: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign30810_e33675: f64 = (1.0 - assign30810_e33674);
        (assign30810_e33675, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign30810_e33677;
        locals.var_q_temp2__blk815_dn4 = assign30810_e33677_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign30810_e33677_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign30810_e33677_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign30810_e33677_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign30810_e33677_d_n9;

        let (assign30820_e33690, assign30820_e33690_d_n4, assign30820_e33690_d_n6, assign30820_e33690_d_n7, assign30820_e33690_d_n8, assign30820_e33690_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 != 0.0)) {
        let assign30820_e33686: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign30820_e33688: f64 = (assign30820_e33686 * locals.var_q_temp2__blk815);
        (assign30820_e33688, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30820_e33686 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30820_e33686 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30820_e33686 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30820_e33686 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign30820_e33686 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign30820_e33690;
        locals.var_q_d1_ln__blk835_dn4 = assign30820_e33690_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign30820_e33690_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign30820_e33690_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign30820_e33690_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign30820_e33690_d_n9;

        let (assign30830_e33711, assign30830_e33711_d_n4, assign30830_e33711_d_n6, assign30830_e33711_d_n7, assign30830_e33711_d_n8, assign30830_e33711_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 != 0.0)) {
        let assign30830_e33699: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign30830_e33704: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign30830_e33705: f64 = (locals.var_q_d1_ln__blk835 + assign30830_e33704);
        let assign30830_e33706: f64 = (locals.var_q_d1_qsq__blk826 * assign30830_e33705);
        let assign30830_e33707: f64 = (assign30830_e33699 - assign30830_e33706);
        let assign30830_e33709: f64 = (assign30830_e33707 / locals.var_q_qsq__blk825);
        (assign30830_e33709, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign30830_e33705) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign30830_e33707 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign30830_e33705) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign30830_e33707 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign30830_e33705) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign30830_e33707 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign30830_e33705) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign30830_e33707 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign30830_e33705) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign30830_e33707 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign30830_e33711;
        locals.var_q_d2_ln__blk836_dn4 = assign30830_e33711_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign30830_e33711_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign30830_e33711_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign30830_e33711_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign30830_e33711_d_n9;

    }

    pub(super) fn stamp_transient_block_82(
        locals: &mut StampLocals,
    ) {
        let (assign30840_e33739, assign30840_e33739_d_n4, assign30840_e33739_d_n6, assign30840_e33739_d_n7, assign30840_e33739_d_n8, assign30840_e33739_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 == 0.0)) {
        let assign30840_e33723: f64 = (locals.var_q_qsq__blk825 * 0.0166666666667);
        let assign30840_e33727: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign30840_e33731: f64 = (locals.var_q_qsq__blk825 * 0.025);
        let assign30840_e33732: f64 = (1.0 - assign30840_e33731);
        let assign30840_e33733: f64 = (assign30840_e33727 * assign30840_e33732);
        let assign30840_e33734: f64 = (1.0 - assign30840_e33733);
        let assign30840_e33735: f64 = (assign30840_e33723 * assign30840_e33734);
        let assign30840_e33736: f64 = (1.0 - assign30840_e33735);
        let assign30840_e33737: f64 = (0.1666666666667 * assign30840_e33736);
        (assign30840_e33737, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0166666666667) * assign30840_e33734) + (assign30840_e33723 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign30840_e33732) + (assign30840_e33727 * (-(locals.var_q_qsq__blk825_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0166666666667) * assign30840_e33734) + (assign30840_e33723 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign30840_e33732) + (assign30840_e33727 * (-(locals.var_q_qsq__blk825_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0166666666667) * assign30840_e33734) + (assign30840_e33723 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign30840_e33732) + (assign30840_e33727 * (-(locals.var_q_qsq__blk825_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0166666666667) * assign30840_e33734) + (assign30840_e33723 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign30840_e33732) + (assign30840_e33727 * (-(locals.var_q_qsq__blk825_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0166666666667) * assign30840_e33734) + (assign30840_e33723 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign30840_e33732) + (assign30840_e33727 * (-(locals.var_q_qsq__blk825_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign30840_e33739;
        locals.var_q_temp3__blk816_dn4 = assign30840_e33739_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign30840_e33739_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign30840_e33739_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign30840_e33739_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign30840_e33739_d_n9;

        let (assign30850_e33753, assign30850_e33753_d_n4, assign30850_e33753_d_n6, assign30850_e33753_d_n7, assign30850_e33753_d_n8, assign30850_e33753_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 == 0.0)) {
        let assign30850_e33750: f64 = (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816);
        let assign30850_e33751: f64 = (2.0 + assign30850_e33750);
        (assign30850_e33751, ((locals.var_q_qsq__blk825_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn4)), ((locals.var_q_qsq__blk825_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn6)), ((locals.var_q_qsq__blk825_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn7)), ((locals.var_q_qsq__blk825_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn8)), ((locals.var_q_qsq__blk825_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign30850_e33753;
        locals.var_q_qcoth__blk829_dn4 = assign30850_e33753_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign30850_e33753_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign30850_e33753_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign30850_e33753_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign30850_e33753_d_n9;

        let (assign30860_e33781, assign30860_e33781_d_n4, assign30860_e33781_d_n6, assign30860_e33781_d_n7, assign30860_e33781_d_n8, assign30860_e33781_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 == 0.0)) {
        let assign30860_e33765: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign30860_e33769: f64 = (locals.var_q_qsq__blk825 * 0.0357142857143);
        let assign30860_e33773: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign30860_e33774: f64 = (1.0 - assign30860_e33773);
        let assign30860_e33775: f64 = (assign30860_e33769 * assign30860_e33774);
        let assign30860_e33776: f64 = (1.0 - assign30860_e33775);
        let assign30860_e33777: f64 = (assign30860_e33765 * assign30860_e33776);
        let assign30860_e33778: f64 = (1.0 - assign30860_e33777);
        let assign30860_e33779: f64 = (0.1666666666667 * assign30860_e33778);
        (assign30860_e33779, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0333333333333) * assign30860_e33776) + (assign30860_e33765 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0357142857143) * assign30860_e33774) + (assign30860_e33769 * (-(locals.var_q_qsq__blk825_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0333333333333) * assign30860_e33776) + (assign30860_e33765 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0357142857143) * assign30860_e33774) + (assign30860_e33769 * (-(locals.var_q_qsq__blk825_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0333333333333) * assign30860_e33776) + (assign30860_e33765 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0357142857143) * assign30860_e33774) + (assign30860_e33769 * (-(locals.var_q_qsq__blk825_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0333333333333) * assign30860_e33776) + (assign30860_e33765 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0357142857143) * assign30860_e33774) + (assign30860_e33769 * (-(locals.var_q_qsq__blk825_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0333333333333) * assign30860_e33776) + (assign30860_e33765 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0357142857143) * assign30860_e33774) + (assign30860_e33769 * (-(locals.var_q_qsq__blk825_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign30860_e33781;
        locals.var_q_temp1__blk814_dn4 = assign30860_e33781_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign30860_e33781_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign30860_e33781_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign30860_e33781_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign30860_e33781_d_n9;

        let (assign30870_e33793, assign30870_e33793_d_n4, assign30870_e33793_d_n6, assign30870_e33793_d_n7, assign30870_e33793_d_n8, assign30870_e33793_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 == 0.0)) {
        let assign30870_e33791: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814);
        (assign30870_e33791, ((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign30870_e33793;
        locals.var_q_d1_qcoth__blk830_dn4 = assign30870_e33793_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign30870_e33793_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign30870_e33793_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign30870_e33793_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign30870_e33793_d_n9;

        let (assign30880_e33821, assign30880_e33821_d_n4, assign30880_e33821_d_n6, assign30880_e33821_d_n7, assign30880_e33821_d_n8, assign30880_e33821_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 == 0.0)) {
        let assign30880_e33805: f64 = (locals.var_q_qsq__blk825 * 0.0714285714286);
        let assign30880_e33809: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign30880_e33813: f64 = (0.0420875420875421 * locals.var_q_qsq__blk825);
        let assign30880_e33814: f64 = (1.0 - assign30880_e33813);
        let assign30880_e33815: f64 = (assign30880_e33809 * assign30880_e33814);
        let assign30880_e33816: f64 = (1.0 - assign30880_e33815);
        let assign30880_e33817: f64 = (assign30880_e33805 * assign30880_e33816);
        let assign30880_e33818: f64 = (1.0 - assign30880_e33817);
        let assign30880_e33819: f64 = (0.0055555555556 * assign30880_e33818);
        (assign30880_e33819, (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0714285714286) * assign30880_e33816) + (assign30880_e33805 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign30880_e33814) + (assign30880_e33809 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0714285714286) * assign30880_e33816) + (assign30880_e33805 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign30880_e33814) + (assign30880_e33809 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0714285714286) * assign30880_e33816) + (assign30880_e33805 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign30880_e33814) + (assign30880_e33809 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0714285714286) * assign30880_e33816) + (assign30880_e33805 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign30880_e33814) + (assign30880_e33809 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0714285714286) * assign30880_e33816) + (assign30880_e33805 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign30880_e33814) + (assign30880_e33809 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn9))))))))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign30880_e33821;
        locals.var_q_temp2__blk815_dn4 = assign30880_e33821_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign30880_e33821_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign30880_e33821_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign30880_e33821_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign30880_e33821_d_n9;

        let (assign30890_e33839, assign30890_e33839_d_n4, assign30890_e33839_d_n6, assign30890_e33839_d_n7, assign30890_e33839_d_n8, assign30890_e33839_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 == 0.0)) {
        let assign30890_e33831: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814);
        let assign30890_e33834: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826);
        let assign30890_e33836: f64 = (assign30890_e33834 * locals.var_q_temp2__blk815);
        let assign30890_e33837: f64 = (assign30890_e33831 - assign30890_e33836);
        (assign30890_e33837, (((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn4)) - ((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn4)) * locals.var_q_temp2__blk815) + (assign30890_e33834 * locals.var_q_temp2__blk815_dn4))), (((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn6)) - ((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn6)) * locals.var_q_temp2__blk815) + (assign30890_e33834 * locals.var_q_temp2__blk815_dn6))), (((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn7)) - ((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn7)) * locals.var_q_temp2__blk815) + (assign30890_e33834 * locals.var_q_temp2__blk815_dn7))), (((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn8)) - ((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn8)) * locals.var_q_temp2__blk815) + (assign30890_e33834 * locals.var_q_temp2__blk815_dn8))), (((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn9)) - ((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn9)) * locals.var_q_temp2__blk815) + (assign30890_e33834 * locals.var_q_temp2__blk815_dn9))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign30890_e33839;
        locals.var_q_d2_qcoth__blk832_dn4 = assign30890_e33839_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign30890_e33839_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign30890_e33839_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign30890_e33839_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign30890_e33839_d_n9;

        let (assign30900_e33854, assign30900_e33854_d_n4, assign30900_e33854_d_n6, assign30900_e33854_d_n7, assign30900_e33854_d_n8, assign30900_e33854_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 == 0.0)) {
        let assign30900_e33848: f64 = (-0.5);
        let assign30900_e33850: f64 = (assign30900_e33848 * locals.var_q_d1_qsq__blk826);
        let assign30900_e33852: f64 = (assign30900_e33850 * locals.var_q_temp3__blk816);
        (assign30900_e33852, (((assign30900_e33848 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_temp3__blk816) + (assign30900_e33850 * locals.var_q_temp3__blk816_dn4)), (((assign30900_e33848 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_temp3__blk816) + (assign30900_e33850 * locals.var_q_temp3__blk816_dn6)), (((assign30900_e33848 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_temp3__blk816) + (assign30900_e33850 * locals.var_q_temp3__blk816_dn7)), (((assign30900_e33848 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_temp3__blk816) + (assign30900_e33850 * locals.var_q_temp3__blk816_dn8)), (((assign30900_e33848 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_temp3__blk816) + (assign30900_e33850 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign30900_e33854;
        locals.var_q_d1_ln__blk835_dn4 = assign30900_e33854_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign30900_e33854_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign30900_e33854_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign30900_e33854_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign30900_e33854_d_n9;

        let (assign30910_e33889, assign30910_e33889_d_n4, assign30910_e33889_d_n6, assign30910_e33889_d_n7, assign30910_e33889_d_n8, assign30910_e33889_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1103 == 0.0)) && (locals.var_guard1104 == 0.0)) {
        let assign30910_e33863: f64 = (-0.5);
        let assign30910_e33865: f64 = (assign30910_e33863 * locals.var_q_d2_qsq__blk827);
        let assign30910_e33867: f64 = (assign30910_e33865 * locals.var_q_temp3__blk816);
        let assign30910_e33870: f64 = (0.25 * 0.0055555555556);
        let assign30910_e33872: f64 = (assign30910_e33870 * locals.var_q_d1_qsq__blk826);
        let assign30910_e33874: f64 = (assign30910_e33872 * locals.var_q_d1_qsq__blk826);
        let assign30910_e33878: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign30910_e33882: f64 = (0.075 * locals.var_q_qsq__blk825);
        let assign30910_e33883: f64 = (2.0 - assign30910_e33882);
        let assign30910_e33884: f64 = (assign30910_e33878 * assign30910_e33883);
        let assign30910_e33885: f64 = (1.0 - assign30910_e33884);
        let assign30910_e33886: f64 = (assign30910_e33874 * assign30910_e33885);
        let assign30910_e33887: f64 = (assign30910_e33867 + assign30910_e33886);
        (assign30910_e33887, ((((assign30910_e33863 * locals.var_q_d2_qsq__blk827_dn4) * locals.var_q_temp3__blk816) + (assign30910_e33865 * locals.var_q_temp3__blk816_dn4)) + (((((assign30910_e33870 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_d1_qsq__blk826) + (assign30910_e33872 * locals.var_q_d1_qsq__blk826_dn4)) * assign30910_e33885) + (assign30910_e33874 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign30910_e33883) + (assign30910_e33878 * (-(0.075 * locals.var_q_qsq__blk825_dn4)))))))), ((((assign30910_e33863 * locals.var_q_d2_qsq__blk827_dn6) * locals.var_q_temp3__blk816) + (assign30910_e33865 * locals.var_q_temp3__blk816_dn6)) + (((((assign30910_e33870 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_d1_qsq__blk826) + (assign30910_e33872 * locals.var_q_d1_qsq__blk826_dn6)) * assign30910_e33885) + (assign30910_e33874 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign30910_e33883) + (assign30910_e33878 * (-(0.075 * locals.var_q_qsq__blk825_dn6)))))))), ((((assign30910_e33863 * locals.var_q_d2_qsq__blk827_dn7) * locals.var_q_temp3__blk816) + (assign30910_e33865 * locals.var_q_temp3__blk816_dn7)) + (((((assign30910_e33870 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_d1_qsq__blk826) + (assign30910_e33872 * locals.var_q_d1_qsq__blk826_dn7)) * assign30910_e33885) + (assign30910_e33874 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign30910_e33883) + (assign30910_e33878 * (-(0.075 * locals.var_q_qsq__blk825_dn7)))))))), ((((assign30910_e33863 * locals.var_q_d2_qsq__blk827_dn8) * locals.var_q_temp3__blk816) + (assign30910_e33865 * locals.var_q_temp3__blk816_dn8)) + (((((assign30910_e33870 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_d1_qsq__blk826) + (assign30910_e33872 * locals.var_q_d1_qsq__blk826_dn8)) * assign30910_e33885) + (assign30910_e33874 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign30910_e33883) + (assign30910_e33878 * (-(0.075 * locals.var_q_qsq__blk825_dn8)))))))), ((((assign30910_e33863 * locals.var_q_d2_qsq__blk827_dn9) * locals.var_q_temp3__blk816) + (assign30910_e33865 * locals.var_q_temp3__blk816_dn9)) + (((((assign30910_e33870 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_d1_qsq__blk826) + (assign30910_e33872 * locals.var_q_d1_qsq__blk826_dn9)) * assign30910_e33885) + (assign30910_e33874 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign30910_e33883) + (assign30910_e33878 * (-(0.075 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign30910_e33889;
        locals.var_q_d2_ln__blk836_dn4 = assign30910_e33889_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign30910_e33889_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign30910_e33889_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign30910_e33889_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign30910_e33889_d_n9;

        let assign30920_e33892: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1105 = assign30920_e33892;

        let (assign30930_e33908, assign30930_e33908_d_n4, assign30930_e33908_d_n6, assign30930_e33908_d_n7, assign30930_e33908_d_n8, assign30930_e33908_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1105 != 0.0)) {
        let assign30930_e33898: f64 = (4.0 * locals.var_q_qsq__blk825);
        let assign30930_e33903: f64 = (2.0 - locals.var_q_invexpq__blk831);
        let assign30930_e33904: f64 = (locals.var_q_invexpq__blk831 * assign30930_e33903);
        let assign30930_e33905: f64 = (1.0 - assign30930_e33904);
        let assign30930_e33906: f64 = (assign30930_e33898 / assign30930_e33905);
        (assign30930_e33906, ((((4.0 * locals.var_q_qsq__blk825_dn4) * assign30930_e33905) - (assign30930_e33898 * (-((locals.var_q_invexpq__blk831_dn4 * assign30930_e33903) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign30930_e33905 * assign30930_e33905)), ((((4.0 * locals.var_q_qsq__blk825_dn6) * assign30930_e33905) - (assign30930_e33898 * (-((locals.var_q_invexpq__blk831_dn6 * assign30930_e33903) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign30930_e33905 * assign30930_e33905)), ((((4.0 * locals.var_q_qsq__blk825_dn7) * assign30930_e33905) - (assign30930_e33898 * (-((locals.var_q_invexpq__blk831_dn7 * assign30930_e33903) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign30930_e33905 * assign30930_e33905)), ((((4.0 * locals.var_q_qsq__blk825_dn8) * assign30930_e33905) - (assign30930_e33898 * (-((locals.var_q_invexpq__blk831_dn8 * assign30930_e33903) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign30930_e33905 * assign30930_e33905)), ((((4.0 * locals.var_q_qsq__blk825_dn9) * assign30930_e33905) - (assign30930_e33898 * (-((locals.var_q_invexpq__blk831_dn9 * assign30930_e33903) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign30930_e33905 * assign30930_e33905)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign30930_e33908;
        locals.var_q_temp2__blk815_dn4 = assign30930_e33908_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign30930_e33908_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign30930_e33908_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign30930_e33908_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign30930_e33908_d_n9;

        let (assign30940_e33916, assign30940_e33916_d_n4, assign30940_e33916_d_n6, assign30940_e33916_d_n7, assign30940_e33916_d_n8, assign30940_e33916_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1105 != 0.0)) {
        let assign30940_e33914: f64 = (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831);
        (assign30940_e33914, ((locals.var_q_temp2__blk815_dn4 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn4)), ((locals.var_q_temp2__blk815_dn6 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn6)), ((locals.var_q_temp2__blk815_dn7 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn7)), ((locals.var_q_temp2__blk815_dn8 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn8)), ((locals.var_q_temp2__blk815_dn9 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn9)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign30940_e33916;
        locals.var_q_sh_term__blk833_dn4 = assign30940_e33916_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign30940_e33916_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign30940_e33916_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign30940_e33916_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign30940_e33916_d_n9;

        let (assign30950_e33925, assign30950_e33925_d_n4, assign30950_e33925_d_n6, assign30950_e33925_d_n7, assign30950_e33925_d_n8, assign30950_e33925_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1105 != 0.0)) {
        let assign30950_e33921: f64 = (locals.var_q_temp2__blk815).ln();
        let assign30950_e33923: f64 = (assign30950_e33921 - locals.var_q_rac_qsq__blk828);
        (assign30950_e33923, ((locals.var_q_temp2__blk815_dn4 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn4), ((locals.var_q_temp2__blk815_dn6 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn6), ((locals.var_q_temp2__blk815_dn7 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn7), ((locals.var_q_temp2__blk815_dn8 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn8), ((locals.var_q_temp2__blk815_dn9 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn9),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign30950_e33925;
        locals.var_q_ln_term__blk834_dn4 = assign30950_e33925_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign30950_e33925_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign30950_e33925_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign30950_e33925_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign30950_e33925_d_n9;

        let assign30960_e33928: f64 = (-0.005);
        let assign30960_e33929: f64 = if locals.var_q_qsq__blk825 < assign30960_e33928 { 1.0 } else { 0.0 };
        locals.var_guard1106 = assign30960_e33929;

        let (assign30970_e33941, assign30970_e33941_d_n4, assign30970_e33941_d_n6, assign30970_e33941_d_n7, assign30970_e33941_d_n8, assign30970_e33941_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1105 == 0.0)) && (locals.var_guard1106 != 0.0)) {
        let assign30970_e33938: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign30970_e33939: f64 = (assign30970_e33938).sin();
        (assign30970_e33939, ((assign30970_e33938).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign30970_e33938).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign30970_e33938).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign30970_e33938).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign30970_e33938).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign30970_e33941;
        locals.var_q_temp2__blk815_dn4 = assign30970_e33941_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign30970_e33941_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign30970_e33941_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign30970_e33941_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign30970_e33941_d_n9;

        let (assign30980_e33955, assign30980_e33955_d_n4, assign30980_e33955_d_n6, assign30980_e33955_d_n7, assign30980_e33955_d_n8, assign30980_e33955_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1105 == 0.0)) && (locals.var_guard1106 != 0.0)) {
        let assign30980_e33949: f64 = (-locals.var_q_qsq__blk825);
        let assign30980_e33952: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
        let assign30980_e33953: f64 = (assign30980_e33949 / assign30980_e33952);
        (assign30980_e33953, ((((-locals.var_q_qsq__blk825_dn4) * assign30980_e33952) - (assign30980_e33949 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign30980_e33952 * assign30980_e33952)), ((((-locals.var_q_qsq__blk825_dn6) * assign30980_e33952) - (assign30980_e33949 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign30980_e33952 * assign30980_e33952)), ((((-locals.var_q_qsq__blk825_dn7) * assign30980_e33952) - (assign30980_e33949 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign30980_e33952 * assign30980_e33952)), ((((-locals.var_q_qsq__blk825_dn8) * assign30980_e33952) - (assign30980_e33949 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign30980_e33952 * assign30980_e33952)), ((((-locals.var_q_qsq__blk825_dn9) * assign30980_e33952) - (assign30980_e33949 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign30980_e33952 * assign30980_e33952)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign30980_e33955;
        locals.var_q_sh_term__blk833_dn4 = assign30980_e33955_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign30980_e33955_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign30980_e33955_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign30980_e33955_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign30980_e33955_d_n9;

        let (assign30990_e33965, assign30990_e33965_d_n4, assign30990_e33965_d_n6, assign30990_e33965_d_n7, assign30990_e33965_d_n8, assign30990_e33965_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1105 == 0.0)) && (locals.var_guard1106 != 0.0)) {
        let assign30990_e33963: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign30990_e33963, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign30990_e33965;
        locals.var_q_ln_term__blk834_dn4 = assign30990_e33965_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign30990_e33965_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign30990_e33965_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign30990_e33965_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign30990_e33965_d_n9;

        let (assign31000_e33991, assign31000_e33991_d_n4, assign31000_e33991_d_n6, assign31000_e33991_d_n7, assign31000_e33991_d_n8, assign31000_e33991_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1105 == 0.0)) && (locals.var_guard1106 == 0.0)) {
        let assign31000_e33976: f64 = (locals.var_q_qsq__blk825 * 0.3333333333333);
        let assign31000_e33980: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign31000_e33984: f64 = (0.0396825396825397 * locals.var_q_qsq__blk825);
        let assign31000_e33985: f64 = (1.0 - assign31000_e33984);
        let assign31000_e33986: f64 = (assign31000_e33980 * assign31000_e33985);
        let assign31000_e33987: f64 = (1.0 - assign31000_e33986);
        let assign31000_e33988: f64 = (assign31000_e33976 * assign31000_e33987);
        let assign31000_e33989: f64 = (4.0 - assign31000_e33988);
        (assign31000_e33989, (-(((locals.var_q_qsq__blk825_dn4 * 0.3333333333333) * assign31000_e33987) + (assign31000_e33976 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign31000_e33985) + (assign31000_e33980 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn4)))))))), (-(((locals.var_q_qsq__blk825_dn6 * 0.3333333333333) * assign31000_e33987) + (assign31000_e33976 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign31000_e33985) + (assign31000_e33980 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn6)))))))), (-(((locals.var_q_qsq__blk825_dn7 * 0.3333333333333) * assign31000_e33987) + (assign31000_e33976 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign31000_e33985) + (assign31000_e33980 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn7)))))))), (-(((locals.var_q_qsq__blk825_dn8 * 0.3333333333333) * assign31000_e33987) + (assign31000_e33976 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign31000_e33985) + (assign31000_e33980 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn8)))))))), (-(((locals.var_q_qsq__blk825_dn9 * 0.3333333333333) * assign31000_e33987) + (assign31000_e33976 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign31000_e33985) + (assign31000_e33980 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign31000_e33991;
        locals.var_q_sh_term__blk833_dn4 = assign31000_e33991_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign31000_e33991_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign31000_e33991_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign31000_e33991_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign31000_e33991_d_n9;

        let (assign31010_e34002, assign31010_e34002_d_n4, assign31010_e34002_d_n6, assign31010_e34002_d_n7, assign31010_e34002_d_n8, assign31010_e34002_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1105 == 0.0)) && (locals.var_guard1106 == 0.0)) {
        let assign31010_e34000: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign31010_e34000, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign31010_e34002;
        locals.var_q_ln_term__blk834_dn4 = assign31010_e34002_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign31010_e34002_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign31010_e34002_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign31010_e34002_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign31010_e34002_d_n9;

        let assign31020_e34005: f64 = (1.01 * locals.var_q_k1q1__blk823);
        let assign31020_e34007: f64 = (assign31020_e34005 + locals.var_q_qcoth__blk829);
        let assign31020_e34009: f64 = if assign31020_e34007 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1107 = assign31020_e34009;

        let (assign31030_e34017, assign31030_e34017_d_n4, assign31030_e34017_d_n6, assign31030_e34017_d_n7, assign31030_e34017_d_n8, assign31030_e34017_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1107 != 0.0)) {
        let assign31030_e34015: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_qcoth__blk829);
        (assign31030_e34015, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_qcoth__blk829_dn9),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign31030_e34017;
        locals.var_q_expnum__blk837_dn4 = assign31030_e34017_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign31030_e34017_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign31030_e34017_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign31030_e34017_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign31030_e34017_d_n9;

        let (assign31040_e34025, assign31040_e34025_d_n4, assign31040_e34025_d_n6, assign31040_e34025_d_n7, assign31040_e34025_d_n8, assign31040_e34025_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1107 != 0.0)) {
        let assign31040_e34023: f64 = (locals.var_k1__blk932 + locals.var_q_d1_qcoth__blk830);
        (assign31040_e34023, (locals.var_k1__blk932_dn4 + locals.var_q_d1_qcoth__blk830_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_d1_qcoth__blk830_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_d1_qcoth__blk830_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_d1_qcoth__blk830_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_d1_qcoth__blk830_dn9),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign31040_e34025;
        locals.var_q_d1_expnum__blk838_dn4 = assign31040_e34025_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign31040_e34025_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign31040_e34025_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign31040_e34025_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign31040_e34025_d_n9;

        let (assign31050_e34031, assign31050_e34031_d_n4, assign31050_e34031_d_n6, assign31050_e34031_d_n7, assign31050_e34031_d_n8, assign31050_e34031_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1107 != 0.0)) {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign31050_e34031;
        locals.var_q_d2_expnum__blk839_dn4 = assign31050_e34031_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign31050_e34031_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign31050_e34031_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign31050_e34031_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign31050_e34031_d_n9;

        let (assign31060_e34042, assign31060_e34042_d_n4, assign31060_e34042_d_n6, assign31060_e34042_d_n7, assign31060_e34042_d_n8, assign31060_e34042_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1107 == 0.0)) {
        let assign31060_e34039: f64 = (locals.var_q_k1q1__blk823 - locals.var_q_qcoth__blk829);
        let assign31060_e34040: f64 = (1.0 / assign31060_e34039);
        (assign31060_e34040, (-((locals.var_q_k1q1__blk823_dn4 - locals.var_q_qcoth__blk829_dn4) / (assign31060_e34039 * assign31060_e34039))), (-((locals.var_q_k1q1__blk823_dn6 - locals.var_q_qcoth__blk829_dn6) / (assign31060_e34039 * assign31060_e34039))), (-((locals.var_q_k1q1__blk823_dn7 - locals.var_q_qcoth__blk829_dn7) / (assign31060_e34039 * assign31060_e34039))), (-((locals.var_q_k1q1__blk823_dn8 - locals.var_q_qcoth__blk829_dn8) / (assign31060_e34039 * assign31060_e34039))), (-((locals.var_q_k1q1__blk823_dn9 - locals.var_q_qcoth__blk829_dn9) / (assign31060_e34039 * assign31060_e34039))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign31060_e34042;
        locals.var_q_temp2__blk815_dn4 = assign31060_e34042_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign31060_e34042_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign31060_e34042_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign31060_e34042_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign31060_e34042_d_n9;

        let (assign31070_e34051, assign31070_e34051_d_n4, assign31070_e34051_d_n6, assign31070_e34051_d_n7, assign31070_e34051_d_n8, assign31070_e34051_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1107 == 0.0)) {
        let assign31070_e34049: f64 = (locals.var_q_d1_qcoth__blk830 - locals.var_k1__blk932);
        (assign31070_e34049, (locals.var_q_d1_qcoth__blk830_dn4 - locals.var_k1__blk932_dn4), (locals.var_q_d1_qcoth__blk830_dn6 - locals.var_k1__blk932_dn6), (locals.var_q_d1_qcoth__blk830_dn7 - locals.var_k1__blk932_dn7), (locals.var_q_d1_qcoth__blk830_dn8 - locals.var_k1__blk932_dn8), (locals.var_q_d1_qcoth__blk830_dn9 - locals.var_k1__blk932_dn9),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign31070_e34051;
        locals.var_q_temp3__blk816_dn4 = assign31070_e34051_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign31070_e34051_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign31070_e34051_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign31070_e34051_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign31070_e34051_d_n9;

        let (assign31080_e34062, assign31080_e34062_d_n4, assign31080_e34062_d_n6, assign31080_e34062_d_n7, assign31080_e34062_d_n8, assign31080_e34062_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1107 == 0.0)) {
        let assign31080_e34058: f64 = (locals.var_q_aexp__blk824 - locals.var_q_sh_term__blk833);
        let assign31080_e34060: f64 = (assign31080_e34058 * locals.var_q_temp2__blk815);
        (assign31080_e34060, (((locals.var_q_aexp__blk824_dn4 - locals.var_q_sh_term__blk833_dn4) * locals.var_q_temp2__blk815) + (assign31080_e34058 * locals.var_q_temp2__blk815_dn4)), (((locals.var_q_aexp__blk824_dn6 - locals.var_q_sh_term__blk833_dn6) * locals.var_q_temp2__blk815) + (assign31080_e34058 * locals.var_q_temp2__blk815_dn6)), (((locals.var_q_aexp__blk824_dn7 - locals.var_q_sh_term__blk833_dn7) * locals.var_q_temp2__blk815) + (assign31080_e34058 * locals.var_q_temp2__blk815_dn7)), (((locals.var_q_aexp__blk824_dn8 - locals.var_q_sh_term__blk833_dn8) * locals.var_q_temp2__blk815) + (assign31080_e34058 * locals.var_q_temp2__blk815_dn8)), (((locals.var_q_aexp__blk824_dn9 - locals.var_q_sh_term__blk833_dn9) * locals.var_q_temp2__blk815) + (assign31080_e34058 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign31080_e34062;
        locals.var_q_expnum__blk837_dn4 = assign31080_e34062_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign31080_e34062_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign31080_e34062_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign31080_e34062_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign31080_e34062_d_n9;

        let (assign31090_e34079, assign31090_e34079_d_n4, assign31090_e34079_d_n6, assign31090_e34079_d_n7, assign31090_e34079_d_n8, assign31090_e34079_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1107 == 0.0)) {
        let assign31090_e34069: f64 = (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837);
        let assign31090_e34071: f64 = (assign31090_e34069 - locals.var_q_aexp__blk824);
        let assign31090_e34074: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833);
        let assign31090_e34075: f64 = (assign31090_e34071 - assign31090_e34074);
        let assign31090_e34077: f64 = (assign31090_e34075 * locals.var_q_temp2__blk815);
        (assign31090_e34077, ((((((locals.var_q_temp3__blk816_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4) - ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign31090_e34075 * locals.var_q_temp2__blk815_dn4)), ((((((locals.var_q_temp3__blk816_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6) - ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign31090_e34075 * locals.var_q_temp2__blk815_dn6)), ((((((locals.var_q_temp3__blk816_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7) - ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign31090_e34075 * locals.var_q_temp2__blk815_dn7)), ((((((locals.var_q_temp3__blk816_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8) - ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign31090_e34075 * locals.var_q_temp2__blk815_dn8)), ((((((locals.var_q_temp3__blk816_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9) - ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign31090_e34075 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign31090_e34079;
        locals.var_q_d1_expnum__blk838_dn4 = assign31090_e34079_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign31090_e34079_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign31090_e34079_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign31090_e34079_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign31090_e34079_d_n9;

        let (assign31100_e34106, assign31100_e34106_d_n4, assign31100_e34106_d_n6, assign31100_e34106_d_n7, assign31100_e34106_d_n8, assign31100_e34106_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1107 == 0.0)) {
        let assign31100_e34086: f64 = (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837);
        let assign31100_e34089: f64 = (2.0 * locals.var_q_temp3__blk816);
        let assign31100_e34091: f64 = (assign31100_e34089 * locals.var_q_d1_expnum__blk838);
        let assign31100_e34092: f64 = (assign31100_e34086 + assign31100_e34091);
        let assign31100_e34094: f64 = (assign31100_e34092 + locals.var_q_aexp__blk824);
        let assign31100_e34098: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835);
        let assign31100_e34099: f64 = (locals.var_q_d2_ln__blk836 + assign31100_e34098);
        let assign31100_e34101: f64 = (assign31100_e34099 * locals.var_q_sh_term__blk833);
        let assign31100_e34102: f64 = (assign31100_e34094 - assign31100_e34101);
        let assign31100_e34104: f64 = (assign31100_e34102 * locals.var_q_temp2__blk815);
        (assign31100_e34104, (((((((locals.var_q_d2_qcoth__blk832_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_temp3__blk816_dn4) * locals.var_q_d1_expnum__blk838) + (assign31100_e34089 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4) - (((locals.var_q_d2_ln__blk836_dn4 + ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn4))) * locals.var_q_sh_term__blk833) + (assign31100_e34099 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign31100_e34102 * locals.var_q_temp2__blk815_dn4)), (((((((locals.var_q_d2_qcoth__blk832_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_temp3__blk816_dn6) * locals.var_q_d1_expnum__blk838) + (assign31100_e34089 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6) - (((locals.var_q_d2_ln__blk836_dn6 + ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn6))) * locals.var_q_sh_term__blk833) + (assign31100_e34099 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign31100_e34102 * locals.var_q_temp2__blk815_dn6)), (((((((locals.var_q_d2_qcoth__blk832_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_temp3__blk816_dn7) * locals.var_q_d1_expnum__blk838) + (assign31100_e34089 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7) - (((locals.var_q_d2_ln__blk836_dn7 + ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn7))) * locals.var_q_sh_term__blk833) + (assign31100_e34099 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign31100_e34102 * locals.var_q_temp2__blk815_dn7)), (((((((locals.var_q_d2_qcoth__blk832_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_temp3__blk816_dn8) * locals.var_q_d1_expnum__blk838) + (assign31100_e34089 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8) - (((locals.var_q_d2_ln__blk836_dn8 + ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn8))) * locals.var_q_sh_term__blk833) + (assign31100_e34099 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign31100_e34102 * locals.var_q_temp2__blk815_dn8)), (((((((locals.var_q_d2_qcoth__blk832_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_temp3__blk816_dn9) * locals.var_q_d1_expnum__blk838) + (assign31100_e34089 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9) - (((locals.var_q_d2_ln__blk836_dn9 + ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn9))) * locals.var_q_sh_term__blk833) + (assign31100_e34099 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign31100_e34102 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign31100_e34106;
        locals.var_q_d2_expnum__blk839_dn4 = assign31100_e34106_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign31100_e34106_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign31100_e34106_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign31100_e34106_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign31100_e34106_d_n9;

        let assign31110_e34109: f64 = if locals.var_q_expnum__blk837 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1108 = assign31110_e34109;

        let (assign31120_e34116, assign31120_e34116_d_n4, assign31120_e34116_d_n6, assign31120_e34116_d_n7, assign31120_e34116_d_n8, assign31120_e34116_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1108 != 0.0)) {
        let assign31120_e34114: f64 = (locals.var_q_expnum__blk837).ln();
        (assign31120_e34114, (locals.var_q_expnum__blk837_dn4 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn6 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn7 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn8 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn9 / locals.var_q_expnum__blk837),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign31120_e34116;
        locals.var_q_lnexpnum__blk840_dn4 = assign31120_e34116_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign31120_e34116_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign31120_e34116_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign31120_e34116_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign31120_e34116_d_n9;

        let (assign31130_e34124, assign31130_e34124_d_n4, assign31130_e34124_d_n6, assign31130_e34124_d_n7, assign31130_e34124_d_n8, assign31130_e34124_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1108 != 0.0)) {
        let assign31130_e34122: f64 = (1.0 / locals.var_q_expnum__blk837);
        (assign31130_e34122, (-(locals.var_q_expnum__blk837_dn4 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn6 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn7 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn8 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn9 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign31130_e34124;
        locals.var_q_temp1__blk814_dn4 = assign31130_e34124_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign31130_e34124_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign31130_e34124_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign31130_e34124_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign31130_e34124_d_n9;

        let (assign31140_e34132, assign31140_e34132_d_n4, assign31140_e34132_d_n6, assign31140_e34132_d_n7, assign31140_e34132_d_n8, assign31140_e34132_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1108 != 0.0)) {
        let assign31140_e34130: f64 = (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814);
        (assign31140_e34130, ((locals.var_q_d1_expnum__blk838_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_expnum__blk838_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_expnum__blk838_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_expnum__blk838_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_expnum__blk838_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign31140_e34132;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign31140_e34132_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign31140_e34132_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign31140_e34132_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign31140_e34132_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign31140_e34132_d_n9;

        let (assign31150_e34144, assign31150_e34144_d_n4, assign31150_e34144_d_n6, assign31150_e34144_d_n7, assign31150_e34144_d_n8, assign31150_e34144_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1108 != 0.0)) {
        let assign31150_e34138: f64 = (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814);
        let assign31150_e34141: f64 = (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841);
        let assign31150_e34142: f64 = (assign31150_e34138 - assign31150_e34141);
        (assign31150_e34142, (((locals.var_q_d2_expnum__blk839_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn4)) - ((locals.var_q_d1_lnexpnum__blk841_dn4 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn4))), (((locals.var_q_d2_expnum__blk839_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn6)) - ((locals.var_q_d1_lnexpnum__blk841_dn6 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn6))), (((locals.var_q_d2_expnum__blk839_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn7)) - ((locals.var_q_d1_lnexpnum__blk841_dn7 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn7))), (((locals.var_q_d2_expnum__blk839_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn8)) - ((locals.var_q_d1_lnexpnum__blk841_dn8 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn8))), (((locals.var_q_d2_expnum__blk839_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn9)) - ((locals.var_q_d1_lnexpnum__blk841_dn9 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign31150_e34144;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign31150_e34144_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign31150_e34144_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign31150_e34144_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign31150_e34144_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign31150_e34144_d_n9;

    }

    pub(super) fn stamp_transient_block_83(
        locals: &mut StampLocals,
    ) {
        let (assign31160_e34157, assign31160_e34157_d_n4, assign31160_e34157_d_n6, assign31160_e34157_d_n7, assign31160_e34157_d_n8, assign31160_e34157_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1108 == 0.0)) {
        let assign31160_e34151: f64 = (locals.var_q_k1q1__blk823 + 0.6931471805599);
        let assign31160_e34153: f64 = (-locals.var_q_k1q1__blk823);
        let assign31160_e34154: f64 = (assign31160_e34153).ln();
        let assign31160_e34155: f64 = (assign31160_e34151 + assign31160_e34154);
        (assign31160_e34155, (locals.var_q_k1q1__blk823_dn4 + ((-locals.var_q_k1q1__blk823_dn4) / assign31160_e34153)), (locals.var_q_k1q1__blk823_dn6 + ((-locals.var_q_k1q1__blk823_dn6) / assign31160_e34153)), (locals.var_q_k1q1__blk823_dn7 + ((-locals.var_q_k1q1__blk823_dn7) / assign31160_e34153)), (locals.var_q_k1q1__blk823_dn8 + ((-locals.var_q_k1q1__blk823_dn8) / assign31160_e34153)), (locals.var_q_k1q1__blk823_dn9 + ((-locals.var_q_k1q1__blk823_dn9) / assign31160_e34153)),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign31160_e34157;
        locals.var_q_lnexpnum__blk840_dn4 = assign31160_e34157_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign31160_e34157_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign31160_e34157_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign31160_e34157_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign31160_e34157_d_n9;

        let (assign31170_e34166, assign31170_e34166_d_n4, assign31170_e34166_d_n6, assign31170_e34166_d_n7, assign31170_e34166_d_n8, assign31170_e34166_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1108 == 0.0)) {
        let assign31170_e34164: f64 = (1.0 / locals.var_q1s__blk937);
        (assign31170_e34164, (-(locals.var_q1s__blk937_dn4 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))), (-(locals.var_q1s__blk937_dn6 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))), (-(locals.var_q1s__blk937_dn7 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))), (-(locals.var_q1s__blk937_dn8 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))), (-(locals.var_q1s__blk937_dn9 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign31170_e34166;
        locals.var_q_temp1__blk814_dn4 = assign31170_e34166_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign31170_e34166_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign31170_e34166_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign31170_e34166_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign31170_e34166_d_n9;

        let (assign31180_e34175, assign31180_e34175_d_n4, assign31180_e34175_d_n6, assign31180_e34175_d_n7, assign31180_e34175_d_n8, assign31180_e34175_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1108 == 0.0)) {
        let assign31180_e34173: f64 = (locals.var_k1__blk932 + locals.var_q_temp1__blk814);
        (assign31180_e34173, (locals.var_k1__blk932_dn4 + locals.var_q_temp1__blk814_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_temp1__blk814_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_temp1__blk814_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_temp1__blk814_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_temp1__blk814_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign31180_e34175;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign31180_e34175_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign31180_e34175_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign31180_e34175_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign31180_e34175_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign31180_e34175_d_n9;

        let (assign31190_e34185, assign31190_e34185_d_n4, assign31190_e34185_d_n6, assign31190_e34185_d_n7, assign31190_e34185_d_n8, assign31190_e34185_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1108 == 0.0)) {
        let assign31190_e34181: f64 = (-locals.var_q_temp1__blk814);
        let assign31190_e34183: f64 = (assign31190_e34181 * locals.var_q_temp1__blk814);
        (assign31190_e34183, (((-locals.var_q_temp1__blk814_dn4) * locals.var_q_temp1__blk814) + (assign31190_e34181 * locals.var_q_temp1__blk814_dn4)), (((-locals.var_q_temp1__blk814_dn6) * locals.var_q_temp1__blk814) + (assign31190_e34181 * locals.var_q_temp1__blk814_dn6)), (((-locals.var_q_temp1__blk814_dn7) * locals.var_q_temp1__blk814) + (assign31190_e34181 * locals.var_q_temp1__blk814_dn7)), (((-locals.var_q_temp1__blk814_dn8) * locals.var_q_temp1__blk814) + (assign31190_e34181 * locals.var_q_temp1__blk814_dn8)), (((-locals.var_q_temp1__blk814_dn9) * locals.var_q_temp1__blk814) + (assign31190_e34181 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign31190_e34185;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign31190_e34185_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign31190_e34185_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign31190_e34185_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign31190_e34185_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign31190_e34185_d_n9;

        let (assign31200_e34199, assign31200_e34199_d_n4, assign31200_e34199_d_n6, assign31200_e34199_d_n7, assign31200_e34199_d_n8, assign31200_e34199_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31200_e34189: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
        let assign31200_e34191: f64 = (assign31200_e34189 + locals.var_q1s__blk937);
        let assign31200_e34194: f64 = (2.0 * locals.var_q_lnexpnum__blk840);
        let assign31200_e34195: f64 = (assign31200_e34191 + assign31200_e34194);
        let assign31200_e34197: f64 = (assign31200_e34195 - locals.var_q_ln_term__blk834);
        (assign31200_e34197, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1s__blk937_dn4) + (2.0 * locals.var_q_lnexpnum__blk840_dn4)) - locals.var_q_ln_term__blk834_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1s__blk937_dn6) + (2.0 * locals.var_q_lnexpnum__blk840_dn6)) - locals.var_q_ln_term__blk834_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1s__blk937_dn7) + (2.0 * locals.var_q_lnexpnum__blk840_dn7)) - locals.var_q_ln_term__blk834_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1s__blk937_dn8) + (2.0 * locals.var_q_lnexpnum__blk840_dn8)) - locals.var_q_ln_term__blk834_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1s__blk937_dn9) + (2.0 * locals.var_q_lnexpnum__blk840_dn9)) - locals.var_q_ln_term__blk834_dn9),)
    } else {
        (locals.var_q_q2_int__blk843, locals.var_q_q2_int__blk843_dn4, locals.var_q_q2_int__blk843_dn6, locals.var_q_q2_int__blk843_dn7, locals.var_q_q2_int__blk843_dn8, locals.var_q_q2_int__blk843_dn9,)
    }
};
        locals.var_q_q2_int__blk843 = assign31200_e34199;
        locals.var_q_q2_int__blk843_dn4 = assign31200_e34199_d_n4;
        locals.var_q_q2_int__blk843_dn6 = assign31200_e34199_d_n6;
        locals.var_q_q2_int__blk843_dn7 = assign31200_e34199_d_n7;
        locals.var_q_q2_int__blk843_dn8 = assign31200_e34199_d_n8;
        locals.var_q_q2_int__blk843_dn9 = assign31200_e34199_d_n9;

        let (assign31210_e34209, assign31210_e34209_d_n4, assign31210_e34209_d_n6, assign31210_e34209_d_n7, assign31210_e34209_d_n8, assign31210_e34209_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31210_e34204: f64 = (2.0 * locals.var_q_d1_lnexpnum__blk841);
        let assign31210_e34205: f64 = (1.0 + assign31210_e34204);
        let assign31210_e34207: f64 = (assign31210_e34205 - locals.var_q_d1_ln__blk835);
        (assign31210_e34207, ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn4) - locals.var_q_d1_ln__blk835_dn4), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn6) - locals.var_q_d1_ln__blk835_dn6), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn7) - locals.var_q_d1_ln__blk835_dn7), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn8) - locals.var_q_d1_ln__blk835_dn8), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn9) - locals.var_q_d1_ln__blk835_dn9),)
    } else {
        (locals.var_q_d1_q2__blk844, locals.var_q_d1_q2__blk844_dn4, locals.var_q_d1_q2__blk844_dn6, locals.var_q_d1_q2__blk844_dn7, locals.var_q_d1_q2__blk844_dn8, locals.var_q_d1_q2__blk844_dn9,)
    }
};
        locals.var_q_d1_q2__blk844 = assign31210_e34209;
        locals.var_q_d1_q2__blk844_dn4 = assign31210_e34209_d_n4;
        locals.var_q_d1_q2__blk844_dn6 = assign31210_e34209_d_n6;
        locals.var_q_d1_q2__blk844_dn7 = assign31210_e34209_d_n7;
        locals.var_q_d1_q2__blk844_dn8 = assign31210_e34209_d_n8;
        locals.var_q_d1_q2__blk844_dn9 = assign31210_e34209_d_n9;

        let (assign31220_e34217, assign31220_e34217_d_n4, assign31220_e34217_d_n6, assign31220_e34217_d_n7, assign31220_e34217_d_n8, assign31220_e34217_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31220_e34213: f64 = (2.0 * locals.var_q_d2_lnexpnum__blk842);
        let assign31220_e34215: f64 = (assign31220_e34213 - locals.var_q_d2_ln__blk836);
        (assign31220_e34215, ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn4) - locals.var_q_d2_ln__blk836_dn4), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn6) - locals.var_q_d2_ln__blk836_dn6), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn7) - locals.var_q_d2_ln__blk836_dn7), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn8) - locals.var_q_d2_ln__blk836_dn8), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn9) - locals.var_q_d2_ln__blk836_dn9),)
    } else {
        (locals.var_q_d2_q2__blk845, locals.var_q_d2_q2__blk845_dn4, locals.var_q_d2_q2__blk845_dn6, locals.var_q_d2_q2__blk845_dn7, locals.var_q_d2_q2__blk845_dn8, locals.var_q_d2_q2__blk845_dn9,)
    }
};
        locals.var_q_d2_q2__blk845 = assign31220_e34217;
        locals.var_q_d2_q2__blk845_dn4 = assign31220_e34217_d_n4;
        locals.var_q_d2_q2__blk845_dn6 = assign31220_e34217_d_n6;
        locals.var_q_d2_q2__blk845_dn7 = assign31220_e34217_d_n7;
        locals.var_q_d2_q2__blk845_dn8 = assign31220_e34217_d_n8;
        locals.var_q_d2_q2__blk845_dn9 = assign31220_e34217_d_n9;

        let (assign31230_e34225, assign31230_e34225_d_n4, assign31230_e34225_d_n6, assign31230_e34225_d_n7, assign31230_e34225_d_n8, assign31230_e34225_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31230_e34222: f64 = (locals.var_k2__blk933 * locals.var_q_q2_int__blk843);
        let assign31230_e34223: f64 = (locals.var_q_k1q1__blk823 + assign31230_e34222);
        (assign31230_e34223, (locals.var_q_k1q1__blk823_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn4))), (locals.var_q_k1q1__blk823_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn6))), (locals.var_q_k1q1__blk823_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn7))), (locals.var_q_k1q1__blk823_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn8))), (locals.var_q_k1q1__blk823_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn9))),)
    } else {
        (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9,)
    }
};
        locals.var_q_qi_int__blk846 = assign31230_e34225;
        locals.var_q_qi_int__blk846_dn4 = assign31230_e34225_d_n4;
        locals.var_q_qi_int__blk846_dn6 = assign31230_e34225_d_n6;
        locals.var_q_qi_int__blk846_dn7 = assign31230_e34225_d_n7;
        locals.var_q_qi_int__blk846_dn8 = assign31230_e34225_d_n8;
        locals.var_q_qi_int__blk846_dn9 = assign31230_e34225_d_n9;

        let (assign31240_e34233, assign31240_e34233_d_n4, assign31240_e34233_d_n6, assign31240_e34233_d_n7, assign31240_e34233_d_n8, assign31240_e34233_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31240_e34230: f64 = (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844);
        let assign31240_e34231: f64 = (locals.var_k1__blk932 + assign31240_e34230);
        (assign31240_e34231, (locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn4))), (locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn6))), (locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn7))), (locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn8))), (locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn9))),)
    } else {
        (locals.var_q_d1_qi__blk847, locals.var_q_d1_qi__blk847_dn4, locals.var_q_d1_qi__blk847_dn6, locals.var_q_d1_qi__blk847_dn7, locals.var_q_d1_qi__blk847_dn8, locals.var_q_d1_qi__blk847_dn9,)
    }
};
        locals.var_q_d1_qi__blk847 = assign31240_e34233;
        locals.var_q_d1_qi__blk847_dn4 = assign31240_e34233_d_n4;
        locals.var_q_d1_qi__blk847_dn6 = assign31240_e34233_d_n6;
        locals.var_q_d1_qi__blk847_dn7 = assign31240_e34233_d_n7;
        locals.var_q_d1_qi__blk847_dn8 = assign31240_e34233_d_n8;
        locals.var_q_d1_qi__blk847_dn9 = assign31240_e34233_d_n9;

        let (assign31250_e34239, assign31250_e34239_d_n4, assign31250_e34239_d_n6, assign31250_e34239_d_n7, assign31250_e34239_d_n8, assign31250_e34239_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31250_e34237: f64 = (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845);
        (assign31250_e34237, ((locals.var_k2__blk933_dn4 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn9)),)
    } else {
        (locals.var_q_d2_qi__blk848, locals.var_q_d2_qi__blk848_dn4, locals.var_q_d2_qi__blk848_dn6, locals.var_q_d2_qi__blk848_dn7, locals.var_q_d2_qi__blk848_dn8, locals.var_q_d2_qi__blk848_dn9,)
    }
};
        locals.var_q_d2_qi__blk848 = assign31250_e34239;
        locals.var_q_d2_qi__blk848_dn4 = assign31250_e34239_d_n4;
        locals.var_q_d2_qi__blk848_dn6 = assign31250_e34239_d_n6;
        locals.var_q_d2_qi__blk848_dn7 = assign31250_e34239_d_n7;
        locals.var_q_d2_qi__blk848_dn8 = assign31250_e34239_d_n8;
        locals.var_q_d2_qi__blk848_dn9 = assign31250_e34239_d_n9;

        let (assign31260_e34247, assign31260_e34247_d_n4, assign31260_e34247_d_n6, assign31260_e34247_d_n7, assign31260_e34247_d_n8, assign31260_e34247_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31260_e34243: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837);
        let assign31260_e34245: f64 = (assign31260_e34243 - locals.var_q_aexp__blk824);
        (assign31260_e34245, (((locals.var_q_qi_int__blk846_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_qi_int__blk846_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_qi_int__blk846_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_qi_int__blk846_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_qi_int__blk846_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9,)
    }
};
        locals.var_q_zero__blk849 = assign31260_e34247;
        locals.var_q_zero__blk849_dn4 = assign31260_e34247_d_n4;
        locals.var_q_zero__blk849_dn6 = assign31260_e34247_d_n6;
        locals.var_q_zero__blk849_dn7 = assign31260_e34247_d_n7;
        locals.var_q_zero__blk849_dn8 = assign31260_e34247_d_n8;
        locals.var_q_zero__blk849_dn9 = assign31260_e34247_d_n9;

        let (assign31270_e34259, assign31270_e34259_d_n4, assign31270_e34259_d_n6, assign31270_e34259_d_n7, assign31270_e34259_d_n8, assign31270_e34259_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31270_e34251: f64 = (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837);
        let assign31270_e34254: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838);
        let assign31270_e34255: f64 = (assign31270_e34251 + assign31270_e34254);
        let assign31270_e34257: f64 = (assign31270_e34255 + locals.var_q_aexp__blk824);
        (assign31270_e34257, ((((locals.var_q_d1_qi__blk847_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn4)) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4), ((((locals.var_q_d1_qi__blk847_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn6)) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6), ((((locals.var_q_d1_qi__blk847_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn7)) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7), ((((locals.var_q_d1_qi__blk847_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn8)) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8), ((((locals.var_q_d1_qi__blk847_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn9)) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9,)
    }
};
        locals.var_q_d1_zero__blk850 = assign31270_e34259;
        locals.var_q_d1_zero__blk850_dn4 = assign31270_e34259_d_n4;
        locals.var_q_d1_zero__blk850_dn6 = assign31270_e34259_d_n6;
        locals.var_q_d1_zero__blk850_dn7 = assign31270_e34259_d_n7;
        locals.var_q_d1_zero__blk850_dn8 = assign31270_e34259_d_n8;
        locals.var_q_d1_zero__blk850_dn9 = assign31270_e34259_d_n9;

        let (assign31280_e34277, assign31280_e34277_d_n4, assign31280_e34277_d_n6, assign31280_e34277_d_n7, assign31280_e34277_d_n8, assign31280_e34277_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31280_e34263: f64 = (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837);
        let assign31280_e34266: f64 = (2.0 * locals.var_q_d1_qi__blk847);
        let assign31280_e34268: f64 = (assign31280_e34266 * locals.var_q_d1_expnum__blk838);
        let assign31280_e34269: f64 = (assign31280_e34263 + assign31280_e34268);
        let assign31280_e34272: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839);
        let assign31280_e34273: f64 = (assign31280_e34269 + assign31280_e34272);
        let assign31280_e34275: f64 = (assign31280_e34273 - locals.var_q_aexp__blk824);
        (assign31280_e34275, (((((locals.var_q_d2_qi__blk848_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_d1_qi__blk847_dn4) * locals.var_q_d1_expnum__blk838) + (assign31280_e34266 * locals.var_q_d1_expnum__blk838_dn4))) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn4))) - locals.var_q_aexp__blk824_dn4), (((((locals.var_q_d2_qi__blk848_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_d1_qi__blk847_dn6) * locals.var_q_d1_expnum__blk838) + (assign31280_e34266 * locals.var_q_d1_expnum__blk838_dn6))) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn6))) - locals.var_q_aexp__blk824_dn6), (((((locals.var_q_d2_qi__blk848_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_d1_qi__blk847_dn7) * locals.var_q_d1_expnum__blk838) + (assign31280_e34266 * locals.var_q_d1_expnum__blk838_dn7))) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn7))) - locals.var_q_aexp__blk824_dn7), (((((locals.var_q_d2_qi__blk848_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_d1_qi__blk847_dn8) * locals.var_q_d1_expnum__blk838) + (assign31280_e34266 * locals.var_q_d1_expnum__blk838_dn8))) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn8))) - locals.var_q_aexp__blk824_dn8), (((((locals.var_q_d2_qi__blk848_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_d1_qi__blk847_dn9) * locals.var_q_d1_expnum__blk838) + (assign31280_e34266 * locals.var_q_d1_expnum__blk838_dn9))) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn9))) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_zero__blk851, locals.var_q_d2_zero__blk851_dn4, locals.var_q_d2_zero__blk851_dn6, locals.var_q_d2_zero__blk851_dn7, locals.var_q_d2_zero__blk851_dn8, locals.var_q_d2_zero__blk851_dn9,)
    }
};
        locals.var_q_d2_zero__blk851 = assign31280_e34277;
        locals.var_q_d2_zero__blk851_dn4 = assign31280_e34277_d_n4;
        locals.var_q_d2_zero__blk851_dn6 = assign31280_e34277_d_n6;
        locals.var_q_d2_zero__blk851_dn7 = assign31280_e34277_d_n7;
        locals.var_q_d2_zero__blk851_dn8 = assign31280_e34277_d_n8;
        locals.var_q_d2_zero__blk851_dn9 = assign31280_e34277_d_n9;

        let (assign31290_e34289, assign31290_e34289_d_n4, assign31290_e34289_d_n6, assign31290_e34289_d_n7, assign31290_e34289_d_n8, assign31290_e34289_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31290_e34281: f64 = (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850);
        let assign31290_e34284: f64 = (0.5 * locals.var_q_zero__blk849);
        let assign31290_e34286: f64 = (assign31290_e34284 * locals.var_q_d2_zero__blk851);
        let assign31290_e34287: f64 = (assign31290_e34281 - assign31290_e34286);
        (assign31290_e34287, (((locals.var_q_d1_zero__blk850_dn4 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn4)) - (((0.5 * locals.var_q_zero__blk849_dn4) * locals.var_q_d2_zero__blk851) + (assign31290_e34284 * locals.var_q_d2_zero__blk851_dn4))), (((locals.var_q_d1_zero__blk850_dn6 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn6)) - (((0.5 * locals.var_q_zero__blk849_dn6) * locals.var_q_d2_zero__blk851) + (assign31290_e34284 * locals.var_q_d2_zero__blk851_dn6))), (((locals.var_q_d1_zero__blk850_dn7 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn7)) - (((0.5 * locals.var_q_zero__blk849_dn7) * locals.var_q_d2_zero__blk851) + (assign31290_e34284 * locals.var_q_d2_zero__blk851_dn7))), (((locals.var_q_d1_zero__blk850_dn8 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn8)) - (((0.5 * locals.var_q_zero__blk849_dn8) * locals.var_q_d2_zero__blk851) + (assign31290_e34284 * locals.var_q_d2_zero__blk851_dn8))), (((locals.var_q_d1_zero__blk850_dn9 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn9)) - (((0.5 * locals.var_q_zero__blk849_dn9) * locals.var_q_d2_zero__blk851) + (assign31290_e34284 * locals.var_q_d2_zero__blk851_dn9))),)
    } else {
        (locals.var_q_temp__blk860, locals.var_q_temp__blk860_dn4, locals.var_q_temp__blk860_dn6, locals.var_q_temp__blk860_dn7, locals.var_q_temp__blk860_dn8, locals.var_q_temp__blk860_dn9,)
    }
};
        locals.var_q_temp__blk860 = assign31290_e34289;
        locals.var_q_temp__blk860_dn4 = assign31290_e34289_d_n4;
        locals.var_q_temp__blk860_dn6 = assign31290_e34289_d_n6;
        locals.var_q_temp__blk860_dn7 = assign31290_e34289_d_n7;
        locals.var_q_temp__blk860_dn8 = assign31290_e34289_d_n8;
        locals.var_q_temp__blk860_dn9 = assign31290_e34289_d_n9;

        let (assign31300_e34304, assign31300_e34304_d_n4, assign31300_e34304_d_n6, assign31300_e34304_d_n7, assign31300_e34304_d_n8, assign31300_e34304_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31300_e34292: f64 = (-locals.var_q_zero__blk849);
        let assign31300_e34294: f64 = (assign31300_e34292 * locals.var_q_d1_zero__blk850);
        let assign31300_e34296: f64 = (assign31300_e34294 * locals.var_q_temp__blk860);
        let assign31300_e34299: f64 = (locals.var_q_temp__blk860 * locals.var_q_temp__blk860);
        let assign31300_e34301: f64 = (assign31300_e34299 + 1e-200);
        let assign31300_e34302: f64 = (assign31300_e34296 / assign31300_e34301);
        (assign31300_e34302, ((((((((-locals.var_q_zero__blk849_dn4) * locals.var_q_d1_zero__blk850) + (assign31300_e34292 * locals.var_q_d1_zero__blk850_dn4)) * locals.var_q_temp__blk860) + (assign31300_e34294 * locals.var_q_temp__blk860_dn4)) * assign31300_e34301) - (assign31300_e34296 * ((locals.var_q_temp__blk860_dn4 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn4)))) / (assign31300_e34301 * assign31300_e34301)), ((((((((-locals.var_q_zero__blk849_dn6) * locals.var_q_d1_zero__blk850) + (assign31300_e34292 * locals.var_q_d1_zero__blk850_dn6)) * locals.var_q_temp__blk860) + (assign31300_e34294 * locals.var_q_temp__blk860_dn6)) * assign31300_e34301) - (assign31300_e34296 * ((locals.var_q_temp__blk860_dn6 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn6)))) / (assign31300_e34301 * assign31300_e34301)), ((((((((-locals.var_q_zero__blk849_dn7) * locals.var_q_d1_zero__blk850) + (assign31300_e34292 * locals.var_q_d1_zero__blk850_dn7)) * locals.var_q_temp__blk860) + (assign31300_e34294 * locals.var_q_temp__blk860_dn7)) * assign31300_e34301) - (assign31300_e34296 * ((locals.var_q_temp__blk860_dn7 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn7)))) / (assign31300_e34301 * assign31300_e34301)), ((((((((-locals.var_q_zero__blk849_dn8) * locals.var_q_d1_zero__blk850) + (assign31300_e34292 * locals.var_q_d1_zero__blk850_dn8)) * locals.var_q_temp__blk860) + (assign31300_e34294 * locals.var_q_temp__blk860_dn8)) * assign31300_e34301) - (assign31300_e34296 * ((locals.var_q_temp__blk860_dn8 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn8)))) / (assign31300_e34301 * assign31300_e34301)), ((((((((-locals.var_q_zero__blk849_dn9) * locals.var_q_d1_zero__blk850) + (assign31300_e34292 * locals.var_q_d1_zero__blk850_dn9)) * locals.var_q_temp__blk860) + (assign31300_e34294 * locals.var_q_temp__blk860_dn9)) * assign31300_e34301) - (assign31300_e34296 * ((locals.var_q_temp__blk860_dn9 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn9)))) / (assign31300_e34301 * assign31300_e34301)),)
    } else {
        (locals.var_q_eps2__blk852, locals.var_q_eps2__blk852_dn4, locals.var_q_eps2__blk852_dn6, locals.var_q_eps2__blk852_dn7, locals.var_q_eps2__blk852_dn8, locals.var_q_eps2__blk852_dn9,)
    }
};
        locals.var_q_eps2__blk852 = assign31300_e34304;
        locals.var_q_eps2__blk852_dn4 = assign31300_e34304_d_n4;
        locals.var_q_eps2__blk852_dn6 = assign31300_e34304_d_n6;
        locals.var_q_eps2__blk852_dn7 = assign31300_e34304_d_n7;
        locals.var_q_eps2__blk852_dn8 = assign31300_e34304_d_n8;
        locals.var_q_eps2__blk852_dn9 = assign31300_e34304_d_n9;

        let (assign31310_e34310, assign31310_e34310_d_n4, assign31310_e34310_d_n6, assign31310_e34310_d_n7, assign31310_e34310_d_n8, assign31310_e34310_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31310_e34308: f64 = (locals.var_q1s__blk937 + locals.var_q_eps2__blk852);
        (assign31310_e34308, (locals.var_q1s__blk937_dn4 + locals.var_q_eps2__blk852_dn4), (locals.var_q1s__blk937_dn6 + locals.var_q_eps2__blk852_dn6), (locals.var_q1s__blk937_dn7 + locals.var_q_eps2__blk852_dn7), (locals.var_q1s__blk937_dn8 + locals.var_q_eps2__blk852_dn8), (locals.var_q1s__blk937_dn9 + locals.var_q_eps2__blk852_dn9),)
    } else {
        (locals.var_q1s__blk937, locals.var_q1s__blk937_dn4, locals.var_q1s__blk937_dn6, locals.var_q1s__blk937_dn7, locals.var_q1s__blk937_dn8, locals.var_q1s__blk937_dn9,)
    }
};
        locals.var_q1s__blk937 = assign31310_e34310;
        locals.var_q1s__blk937_dn4 = assign31310_e34310_d_n4;
        locals.var_q1s__blk937_dn6 = assign31310_e34310_d_n6;
        locals.var_q1s__blk937_dn7 = assign31310_e34310_d_n7;
        locals.var_q1s__blk937_dn8 = assign31310_e34310_d_n8;
        locals.var_q1s__blk937_dn9 = assign31310_e34310_d_n9;

        let (assign31320_e34316, assign31320_e34316_d_n4, assign31320_e34316_d_n6, assign31320_e34316_d_n7, assign31320_e34316_d_n8, assign31320_e34316_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31320_e34314: f64 = (locals.var_k1__blk932 * locals.var_q1s__blk937);
        (assign31320_e34314, ((locals.var_k1__blk932_dn4 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn9)),)
    } else {
        (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9,)
    }
};
        locals.var_q_k1q1__blk823 = assign31320_e34316;
        locals.var_q_k1q1__blk823_dn4 = assign31320_e34316_d_n4;
        locals.var_q_k1q1__blk823_dn6 = assign31320_e34316_d_n6;
        locals.var_q_k1q1__blk823_dn7 = assign31320_e34316_d_n7;
        locals.var_q_k1q1__blk823_dn8 = assign31320_e34316_d_n8;
        locals.var_q_k1q1__blk823_dn9 = assign31320_e34316_d_n9;

        let (assign31330_e34322, assign31330_e34322_d_n4, assign31330_e34322_d_n6, assign31330_e34322_d_n7, assign31330_e34322_d_n8, assign31330_e34322_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31330_e34320: f64 = (locals.var_k2__blk933 * locals.var_q2s__blk941);
        (assign31330_e34320, ((locals.var_k2__blk933_dn4 * locals.var_q2s__blk941) + (locals.var_k2__blk933 * locals.var_q2s__blk941_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q2s__blk941) + (locals.var_k2__blk933 * locals.var_q2s__blk941_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q2s__blk941) + (locals.var_k2__blk933 * locals.var_q2s__blk941_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q2s__blk941) + (locals.var_k2__blk933 * locals.var_q2s__blk941_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q2s__blk941) + (locals.var_k2__blk933 * locals.var_q2s__blk941_dn9)),)
    } else {
        (locals.var_q_k2q2__blk853, locals.var_q_k2q2__blk853_dn4, locals.var_q_k2q2__blk853_dn6, locals.var_q_k2q2__blk853_dn7, locals.var_q_k2q2__blk853_dn8, locals.var_q_k2q2__blk853_dn9,)
    }
};
        locals.var_q_k2q2__blk853 = assign31330_e34322;
        locals.var_q_k2q2__blk853_dn4 = assign31330_e34322_d_n4;
        locals.var_q_k2q2__blk853_dn6 = assign31330_e34322_d_n6;
        locals.var_q_k2q2__blk853_dn7 = assign31330_e34322_d_n7;
        locals.var_q_k2q2__blk853_dn8 = assign31330_e34322_d_n8;
        locals.var_q_k2q2__blk853_dn9 = assign31330_e34322_d_n9;

        let (assign31340_e34328, assign31340_e34328_d_n4, assign31340_e34328_d_n6, assign31340_e34328_d_n7, assign31340_e34328_d_n8, assign31340_e34328_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31340_e34326: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_k2q2__blk853);
        (assign31340_e34326, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_k2q2__blk853_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_k2q2__blk853_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_k2q2__blk853_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_k2q2__blk853_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_k2q2__blk853_dn9),)
    } else {
        (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9,)
    }
};
        locals.var_q_qi_int__blk846 = assign31340_e34328;
        locals.var_q_qi_int__blk846_dn4 = assign31340_e34328_d_n4;
        locals.var_q_qi_int__blk846_dn6 = assign31340_e34328_d_n6;
        locals.var_q_qi_int__blk846_dn7 = assign31340_e34328_d_n7;
        locals.var_q_qi_int__blk846_dn8 = assign31340_e34328_d_n8;
        locals.var_q_qi_int__blk846_dn9 = assign31340_e34328_d_n9;

        let (assign31350_e34336, assign31350_e34336_d_n4, assign31350_e34336_d_n6, assign31350_e34336_d_n7, assign31350_e34336_d_n8, assign31350_e34336_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31350_e34333: f64 = (0.065345483024 * locals.var_q_qi_int__blk846);
        let assign31350_e34334: f64 = (1.0 + assign31350_e34333);
        (assign31350_e34334, (0.065345483024 * locals.var_q_qi_int__blk846_dn4), (0.065345483024 * locals.var_q_qi_int__blk846_dn6), (0.065345483024 * locals.var_q_qi_int__blk846_dn7), (0.065345483024 * locals.var_q_qi_int__blk846_dn8), (0.065345483024 * locals.var_q_qi_int__blk846_dn9),)
    } else {
        (locals.var_q_a__blk854, locals.var_q_a__blk854_dn4, locals.var_q_a__blk854_dn6, locals.var_q_a__blk854_dn7, locals.var_q_a__blk854_dn8, locals.var_q_a__blk854_dn9,)
    }
};
        locals.var_q_a__blk854 = assign31350_e34336;
        locals.var_q_a__blk854_dn4 = assign31350_e34336_d_n4;
        locals.var_q_a__blk854_dn6 = assign31350_e34336_d_n6;
        locals.var_q_a__blk854_dn7 = assign31350_e34336_d_n7;
        locals.var_q_a__blk854_dn8 = assign31350_e34336_d_n8;
        locals.var_q_a__blk854_dn9 = assign31350_e34336_d_n9;

        let (assign31360_e34348, assign31360_e34348_d_n4, assign31360_e34348_d_n6, assign31360_e34348_d_n7, assign31360_e34348_d_n8, assign31360_e34348_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31360_e34341: f64 = (8.5797362674 * locals.var_q_qi_int__blk846);
        let assign31360_e34342: f64 = (39.478417604 + assign31360_e34341);
        let assign31360_e34345: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853);
        let assign31360_e34346: f64 = (assign31360_e34342 + assign31360_e34345);
        (assign31360_e34346, ((8.5797362674 * locals.var_q_qi_int__blk846_dn4) + ((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn4))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn6) + ((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn6))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn7) + ((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn7))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn8) + ((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn8))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn9) + ((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn9))),)
    } else {
        (locals.var_q_b__blk855, locals.var_q_b__blk855_dn4, locals.var_q_b__blk855_dn6, locals.var_q_b__blk855_dn7, locals.var_q_b__blk855_dn8, locals.var_q_b__blk855_dn9,)
    }
};
        locals.var_q_b__blk855 = assign31360_e34348;
        locals.var_q_b__blk855_dn4 = assign31360_e34348_d_n4;
        locals.var_q_b__blk855_dn6 = assign31360_e34348_d_n6;
        locals.var_q_b__blk855_dn7 = assign31360_e34348_d_n7;
        locals.var_q_b__blk855_dn8 = assign31360_e34348_d_n8;
        locals.var_q_b__blk855_dn9 = assign31360_e34348_d_n9;

        let (assign31370_e34360, assign31370_e34360_d_n4, assign31370_e34360_d_n6, assign31370_e34360_d_n7, assign31370_e34360_d_n8, assign31370_e34360_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31370_e34353: f64 = (2.0 * locals.var_q_qi_int__blk846);
        let assign31370_e34356: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853);
        let assign31370_e34357: f64 = (assign31370_e34353 + assign31370_e34356);
        let assign31370_e34358: f64 = (39.478417604 * assign31370_e34357);
        (assign31370_e34358, (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn4) + ((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn4)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn6) + ((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn6)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn7) + ((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn7)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn8) + ((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn8)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn9) + ((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn9)))),)
    } else {
        (locals.var_q_c__blk856, locals.var_q_c__blk856_dn4, locals.var_q_c__blk856_dn6, locals.var_q_c__blk856_dn7, locals.var_q_c__blk856_dn8, locals.var_q_c__blk856_dn9,)
    }
};
        locals.var_q_c__blk856 = assign31370_e34360;
        locals.var_q_c__blk856_dn4 = assign31370_e34360_d_n4;
        locals.var_q_c__blk856_dn6 = assign31370_e34360_d_n6;
        locals.var_q_c__blk856_dn7 = assign31370_e34360_d_n7;
        locals.var_q_c__blk856_dn8 = assign31370_e34360_d_n8;
        locals.var_q_c__blk856_dn9 = assign31370_e34360_d_n9;

        let (assign31380_e34373, assign31380_e34373_d_n4, assign31380_e34373_d_n6, assign31380_e34373_d_n7, assign31380_e34373_d_n8, assign31380_e34373_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31380_e34364: f64 = (locals.var_q_b__blk855 * locals.var_q_b__blk855);
        let assign31380_e34367: f64 = (4.0 * locals.var_q_a__blk854);
        let assign31380_e34369: f64 = (assign31380_e34367 * locals.var_q_c__blk856);
        let assign31380_e34370: f64 = (assign31380_e34364 - assign31380_e34369);
        let assign31380_e34371: f64 = (assign31380_e34370).sqrt();
        (assign31380_e34371, ((((locals.var_q_b__blk855_dn4 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn4)) - (((4.0 * locals.var_q_a__blk854_dn4) * locals.var_q_c__blk856) + (assign31380_e34367 * locals.var_q_c__blk856_dn4))) / (2.0 * assign31380_e34371)), ((((locals.var_q_b__blk855_dn6 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn6)) - (((4.0 * locals.var_q_a__blk854_dn6) * locals.var_q_c__blk856) + (assign31380_e34367 * locals.var_q_c__blk856_dn6))) / (2.0 * assign31380_e34371)), ((((locals.var_q_b__blk855_dn7 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn7)) - (((4.0 * locals.var_q_a__blk854_dn7) * locals.var_q_c__blk856) + (assign31380_e34367 * locals.var_q_c__blk856_dn7))) / (2.0 * assign31380_e34371)), ((((locals.var_q_b__blk855_dn8 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn8)) - (((4.0 * locals.var_q_a__blk854_dn8) * locals.var_q_c__blk856) + (assign31380_e34367 * locals.var_q_c__blk856_dn8))) / (2.0 * assign31380_e34371)), ((((locals.var_q_b__blk855_dn9 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn9)) - (((4.0 * locals.var_q_a__blk854_dn9) * locals.var_q_c__blk856) + (assign31380_e34367 * locals.var_q_c__blk856_dn9))) / (2.0 * assign31380_e34371)),)
    } else {
        (locals.var_q_disc__blk857, locals.var_q_disc__blk857_dn4, locals.var_q_disc__blk857_dn6, locals.var_q_disc__blk857_dn7, locals.var_q_disc__blk857_dn8, locals.var_q_disc__blk857_dn9,)
    }
};
        locals.var_q_disc__blk857 = assign31380_e34373;
        locals.var_q_disc__blk857_dn4 = assign31380_e34373_d_n4;
        locals.var_q_disc__blk857_dn6 = assign31380_e34373_d_n6;
        locals.var_q_disc__blk857_dn7 = assign31380_e34373_d_n7;
        locals.var_q_disc__blk857_dn8 = assign31380_e34373_d_n8;
        locals.var_q_disc__blk857_dn9 = assign31380_e34373_d_n9;

        let (assign31390_e34383, assign31390_e34383_d_n4, assign31390_e34383_d_n6, assign31390_e34383_d_n7, assign31390_e34383_d_n8, assign31390_e34383_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31390_e34377: f64 = (locals.var_q_disc__blk857 - locals.var_q_b__blk855);
        let assign31390_e34380: f64 = (2.0 * locals.var_q_a__blk854);
        let assign31390_e34381: f64 = (assign31390_e34377 / assign31390_e34380);
        (assign31390_e34381, ((((locals.var_q_disc__blk857_dn4 - locals.var_q_b__blk855_dn4) * assign31390_e34380) - (assign31390_e34377 * (2.0 * locals.var_q_a__blk854_dn4))) / (assign31390_e34380 * assign31390_e34380)), ((((locals.var_q_disc__blk857_dn6 - locals.var_q_b__blk855_dn6) * assign31390_e34380) - (assign31390_e34377 * (2.0 * locals.var_q_a__blk854_dn6))) / (assign31390_e34380 * assign31390_e34380)), ((((locals.var_q_disc__blk857_dn7 - locals.var_q_b__blk855_dn7) * assign31390_e34380) - (assign31390_e34377 * (2.0 * locals.var_q_a__blk854_dn7))) / (assign31390_e34380 * assign31390_e34380)), ((((locals.var_q_disc__blk857_dn8 - locals.var_q_b__blk855_dn8) * assign31390_e34380) - (assign31390_e34377 * (2.0 * locals.var_q_a__blk854_dn8))) / (assign31390_e34380 * assign31390_e34380)), ((((locals.var_q_disc__blk857_dn9 - locals.var_q_b__blk855_dn9) * assign31390_e34380) - (assign31390_e34377 * (2.0 * locals.var_q_a__blk854_dn9))) / (assign31390_e34380 * assign31390_e34380)),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign31390_e34383;
        locals.var_q_qsq__blk825_dn4 = assign31390_e34383_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign31390_e34383_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign31390_e34383_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign31390_e34383_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign31390_e34383_d_n9;

        let (assign31400_e34391, assign31400_e34391_d_n4, assign31400_e34391_d_n6, assign31400_e34391_d_n7, assign31400_e34391_d_n8, assign31400_e34391_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31400_e34387: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
        let assign31400_e34389: f64 = (assign31400_e34387 - locals.var_q_qsq__blk825);
        (assign31400_e34389, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_qsq__blk825_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_qsq__blk825_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_qsq__blk825_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_qsq__blk825_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_qsq__blk825_dn9),)
    } else {
        (locals.var_q_delta__blk858, locals.var_q_delta__blk858_dn4, locals.var_q_delta__blk858_dn6, locals.var_q_delta__blk858_dn7, locals.var_q_delta__blk858_dn8, locals.var_q_delta__blk858_dn9,)
    }
};
        locals.var_q_delta__blk858 = assign31400_e34391;
        locals.var_q_delta__blk858_dn4 = assign31400_e34391_d_n4;
        locals.var_q_delta__blk858_dn6 = assign31400_e34391_d_n6;
        locals.var_q_delta__blk858_dn7 = assign31400_e34391_d_n7;
        locals.var_q_delta__blk858_dn8 = assign31400_e34391_d_n8;
        locals.var_q_delta__blk858_dn9 = assign31400_e34391_d_n9;

        let assign31410_e34394: f64 = if locals.var_q_delta__blk858 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1109 = assign31410_e34394;

        let (assign31420_e34411, assign31420_e34411_d_n4, assign31420_e34411_d_n6, assign31420_e34411_d_n7, assign31420_e34411_d_n8, assign31420_e34411_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1109 != 0.0)) {
        let assign31420_e34401: f64 = (locals.var_q_delta__blk858 / locals.var_a0__blk905);
        let assign31420_e34402: f64 = (assign31420_e34401).ln();
        let assign31420_e34404: f64 = assign31420_e34402;
        let assign31420_e34406: f64 = (assign31420_e34404 - locals.var_xg1x__blk930);
        let assign31420_e34408: f64 = (assign31420_e34406 + locals.var_q1s__blk937);
        let assign31420_e34409: f64 = (locals.var_q_delta__blk858 * assign31420_e34408);
        (assign31420_e34409, ((locals.var_q_delta__blk858_dn4 * assign31420_e34408) + (locals.var_q_delta__blk858 * ((((((locals.var_q_delta__blk858_dn4 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign31420_e34401) - locals.var_xg1x__blk930_dn4) + locals.var_q1s__blk937_dn4))), ((locals.var_q_delta__blk858_dn6 * assign31420_e34408) + (locals.var_q_delta__blk858 * ((((((locals.var_q_delta__blk858_dn6 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign31420_e34401) - locals.var_xg1x__blk930_dn6) + locals.var_q1s__blk937_dn6))), ((locals.var_q_delta__blk858_dn7 * assign31420_e34408) + (locals.var_q_delta__blk858 * ((((((locals.var_q_delta__blk858_dn7 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign31420_e34401) - locals.var_xg1x__blk930_dn7) + locals.var_q1s__blk937_dn7))), ((locals.var_q_delta__blk858_dn8 * assign31420_e34408) + (locals.var_q_delta__blk858 * ((((((locals.var_q_delta__blk858_dn8 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign31420_e34401) - locals.var_xg1x__blk930_dn8) + locals.var_q1s__blk937_dn8))), ((locals.var_q_delta__blk858_dn9 * assign31420_e34408) + (locals.var_q_delta__blk858 * ((((((locals.var_q_delta__blk858_dn9 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign31420_e34401) - locals.var_xg1x__blk930_dn9) + locals.var_q1s__blk937_dn9))),)
    } else {
        (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9,)
    }
};
        locals.var_q_zero__blk849 = assign31420_e34411;
        locals.var_q_zero__blk849_dn4 = assign31420_e34411_d_n4;
        locals.var_q_zero__blk849_dn6 = assign31420_e34411_d_n6;
        locals.var_q_zero__blk849_dn7 = assign31420_e34411_d_n7;
        locals.var_q_zero__blk849_dn8 = assign31420_e34411_d_n8;
        locals.var_q_zero__blk849_dn9 = assign31420_e34411_d_n9;

        let (assign31430_e34423, assign31430_e34423_d_n4, assign31430_e34423_d_n6, assign31430_e34423_d_n7, assign31430_e34423_d_n8, assign31430_e34423_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1109 != 0.0)) {
        let assign31430_e34417: f64 = (2.0 * locals.var_k1__blk932);
        let assign31430_e34419: f64 = (assign31430_e34417 * locals.var_q_k1q1__blk823);
        let assign31430_e34421: f64 = (assign31430_e34419 + locals.var_q_delta__blk858);
        (assign31430_e34421, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign31430_e34417 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_delta__blk858_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign31430_e34417 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_delta__blk858_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign31430_e34417 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_delta__blk858_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign31430_e34417 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_delta__blk858_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign31430_e34417 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_delta__blk858_dn9),)
    } else {
        (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9,)
    }
};
        locals.var_q_d1_zero__blk850 = assign31430_e34423;
        locals.var_q_d1_zero__blk850_dn4 = assign31430_e34423_d_n4;
        locals.var_q_d1_zero__blk850_dn6 = assign31430_e34423_d_n6;
        locals.var_q_d1_zero__blk850_dn7 = assign31430_e34423_d_n7;
        locals.var_q_d1_zero__blk850_dn8 = assign31430_e34423_d_n8;
        locals.var_q_d1_zero__blk850_dn9 = assign31430_e34423_d_n9;

        let (assign31440_e34433,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1109 != 0.0)) {
        let assign31440_e34429: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign31440_e34431: f64 = (assign31440_e34429 - locals.var_q_x1sat__blk817);
        (assign31440_e34431,)
    } else {
        (locals.var_q_dx1__blk859,)
    }
};
        locals.var_q_dx1__blk859 = assign31440_e34433;

        let assign31450_e34443: f64 = (locals.var_q_dx1__blk859 + 2.3025850929941);
        let assign31450_e34445: f64 = (locals.var_k1__blk932).ln();
        let assign31450_e34446: f64 = (assign31450_e34443 + assign31450_e34445);
        let assign31450_e34453: f64 = if ((((locals.var_q_zero__blk849 < 0.0) && (locals.var_q_d1_zero__blk850 > 0.0)) && (assign31450_e34446 > 0.0)) || (locals.var_q_dx1__blk859 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1110 = assign31450_e34453;

        let (assign31460_e34465, assign31460_e34465_d_n4, assign31460_e34465_d_n6, assign31460_e34465_d_n7, assign31460_e34465_d_n8, assign31460_e34465_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1109 != 0.0)) && (locals.var_guard1110 != 0.0)) {
        let assign31460_e34462: f64 = (locals.var_q_zero__blk849 / locals.var_q_d1_zero__blk850);
        let assign31460_e34463: f64 = (locals.var_q1s__blk937 - assign31460_e34462);
        (assign31460_e34463, (locals.var_q1s__blk937_dn4 - (((locals.var_q_zero__blk849_dn4 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn4)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1s__blk937_dn6 - (((locals.var_q_zero__blk849_dn6 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn6)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1s__blk937_dn7 - (((locals.var_q_zero__blk849_dn7 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn7)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1s__blk937_dn8 - (((locals.var_q_zero__blk849_dn8 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn8)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1s__blk937_dn9 - (((locals.var_q_zero__blk849_dn9 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn9)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))),)
    } else {
        (locals.var_q1s__blk937, locals.var_q1s__blk937_dn4, locals.var_q1s__blk937_dn6, locals.var_q1s__blk937_dn7, locals.var_q1s__blk937_dn8, locals.var_q1s__blk937_dn9,)
    }
};
        locals.var_q1s__blk937 = assign31460_e34465;
        locals.var_q1s__blk937_dn4 = assign31460_e34465_d_n4;
        locals.var_q1s__blk937_dn6 = assign31460_e34465_d_n6;
        locals.var_q1s__blk937_dn7 = assign31460_e34465_d_n7;
        locals.var_q1s__blk937_dn8 = assign31460_e34465_d_n8;
        locals.var_q1s__blk937_dn9 = assign31460_e34465_d_n9;

        let (assign31470_e34471, assign31470_e34471_d_n4, assign31470_e34471_d_n6, assign31470_e34471_d_n7, assign31470_e34471_d_n8, assign31470_e34471_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31470_e34469: f64 = (locals.var_k1__blk932 * locals.var_q1s__blk937);
        (assign31470_e34469, ((locals.var_k1__blk932_dn4 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn9)),)
    } else {
        (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9,)
    }
};
        locals.var_q_k1q1__blk823 = assign31470_e34471;
        locals.var_q_k1q1__blk823_dn4 = assign31470_e34471_d_n4;
        locals.var_q_k1q1__blk823_dn6 = assign31470_e34471_d_n6;
        locals.var_q_k1q1__blk823_dn7 = assign31470_e34471_d_n7;
        locals.var_q_k1q1__blk823_dn8 = assign31470_e34471_d_n8;
        locals.var_q_k1q1__blk823_dn9 = assign31470_e34471_d_n9;

    }

    pub(super) fn stamp_transient_block_84(
        locals: &mut StampLocals,
    ) {
        let (assign31480_e34477, assign31480_e34477_d_n4, assign31480_e34477_d_n6, assign31480_e34477_d_n7, assign31480_e34477_d_n8, assign31480_e34477_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31480_e34475: f64 = (locals.var_k2__blk933 * locals.var_q2s__blk941);
        (assign31480_e34475, ((locals.var_k2__blk933_dn4 * locals.var_q2s__blk941) + (locals.var_k2__blk933 * locals.var_q2s__blk941_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q2s__blk941) + (locals.var_k2__blk933 * locals.var_q2s__blk941_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q2s__blk941) + (locals.var_k2__blk933 * locals.var_q2s__blk941_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q2s__blk941) + (locals.var_k2__blk933 * locals.var_q2s__blk941_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q2s__blk941) + (locals.var_k2__blk933 * locals.var_q2s__blk941_dn9)),)
    } else {
        (locals.var_q_k2q2__blk853, locals.var_q_k2q2__blk853_dn4, locals.var_q_k2q2__blk853_dn6, locals.var_q_k2q2__blk853_dn7, locals.var_q_k2q2__blk853_dn8, locals.var_q_k2q2__blk853_dn9,)
    }
};
        locals.var_q_k2q2__blk853 = assign31480_e34477;
        locals.var_q_k2q2__blk853_dn4 = assign31480_e34477_d_n4;
        locals.var_q_k2q2__blk853_dn6 = assign31480_e34477_d_n6;
        locals.var_q_k2q2__blk853_dn7 = assign31480_e34477_d_n7;
        locals.var_q_k2q2__blk853_dn8 = assign31480_e34477_d_n8;
        locals.var_q_k2q2__blk853_dn9 = assign31480_e34477_d_n9;

        let (assign31490_e34483, assign31490_e34483_d_n4, assign31490_e34483_d_n6, assign31490_e34483_d_n7, assign31490_e34483_d_n8, assign31490_e34483_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31490_e34481: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_k2q2__blk853);
        (assign31490_e34481, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_k2q2__blk853_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_k2q2__blk853_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_k2q2__blk853_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_k2q2__blk853_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_k2q2__blk853_dn9),)
    } else {
        (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9,)
    }
};
        locals.var_q_qi_int__blk846 = assign31490_e34483;
        locals.var_q_qi_int__blk846_dn4 = assign31490_e34483_d_n4;
        locals.var_q_qi_int__blk846_dn6 = assign31490_e34483_d_n6;
        locals.var_q_qi_int__blk846_dn7 = assign31490_e34483_d_n7;
        locals.var_q_qi_int__blk846_dn8 = assign31490_e34483_d_n8;
        locals.var_q_qi_int__blk846_dn9 = assign31490_e34483_d_n9;

        let (assign31500_e34491, assign31500_e34491_d_n4, assign31500_e34491_d_n6, assign31500_e34491_d_n7, assign31500_e34491_d_n8, assign31500_e34491_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31500_e34488: f64 = (0.065345483024 * locals.var_q_qi_int__blk846);
        let assign31500_e34489: f64 = (1.0 + assign31500_e34488);
        (assign31500_e34489, (0.065345483024 * locals.var_q_qi_int__blk846_dn4), (0.065345483024 * locals.var_q_qi_int__blk846_dn6), (0.065345483024 * locals.var_q_qi_int__blk846_dn7), (0.065345483024 * locals.var_q_qi_int__blk846_dn8), (0.065345483024 * locals.var_q_qi_int__blk846_dn9),)
    } else {
        (locals.var_q_a__blk854, locals.var_q_a__blk854_dn4, locals.var_q_a__blk854_dn6, locals.var_q_a__blk854_dn7, locals.var_q_a__blk854_dn8, locals.var_q_a__blk854_dn9,)
    }
};
        locals.var_q_a__blk854 = assign31500_e34491;
        locals.var_q_a__blk854_dn4 = assign31500_e34491_d_n4;
        locals.var_q_a__blk854_dn6 = assign31500_e34491_d_n6;
        locals.var_q_a__blk854_dn7 = assign31500_e34491_d_n7;
        locals.var_q_a__blk854_dn8 = assign31500_e34491_d_n8;
        locals.var_q_a__blk854_dn9 = assign31500_e34491_d_n9;

        let (assign31510_e34503, assign31510_e34503_d_n4, assign31510_e34503_d_n6, assign31510_e34503_d_n7, assign31510_e34503_d_n8, assign31510_e34503_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31510_e34496: f64 = (8.5797362674 * locals.var_q_qi_int__blk846);
        let assign31510_e34497: f64 = (39.478417604 + assign31510_e34496);
        let assign31510_e34500: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853);
        let assign31510_e34501: f64 = (assign31510_e34497 + assign31510_e34500);
        (assign31510_e34501, ((8.5797362674 * locals.var_q_qi_int__blk846_dn4) + ((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn4))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn6) + ((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn6))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn7) + ((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn7))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn8) + ((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn8))), ((8.5797362674 * locals.var_q_qi_int__blk846_dn9) + ((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn9))),)
    } else {
        (locals.var_q_b__blk855, locals.var_q_b__blk855_dn4, locals.var_q_b__blk855_dn6, locals.var_q_b__blk855_dn7, locals.var_q_b__blk855_dn8, locals.var_q_b__blk855_dn9,)
    }
};
        locals.var_q_b__blk855 = assign31510_e34503;
        locals.var_q_b__blk855_dn4 = assign31510_e34503_d_n4;
        locals.var_q_b__blk855_dn6 = assign31510_e34503_d_n6;
        locals.var_q_b__blk855_dn7 = assign31510_e34503_d_n7;
        locals.var_q_b__blk855_dn8 = assign31510_e34503_d_n8;
        locals.var_q_b__blk855_dn9 = assign31510_e34503_d_n9;

        let (assign31520_e34515, assign31520_e34515_d_n4, assign31520_e34515_d_n6, assign31520_e34515_d_n7, assign31520_e34515_d_n8, assign31520_e34515_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31520_e34508: f64 = (2.0 * locals.var_q_qi_int__blk846);
        let assign31520_e34511: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853);
        let assign31520_e34512: f64 = (assign31520_e34508 + assign31520_e34511);
        let assign31520_e34513: f64 = (39.478417604 * assign31520_e34512);
        (assign31520_e34513, (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn4) + ((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn4)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn6) + ((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn6)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn7) + ((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn7)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn8) + ((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn8)))), (39.478417604 * ((2.0 * locals.var_q_qi_int__blk846_dn9) + ((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn9)))),)
    } else {
        (locals.var_q_c__blk856, locals.var_q_c__blk856_dn4, locals.var_q_c__blk856_dn6, locals.var_q_c__blk856_dn7, locals.var_q_c__blk856_dn8, locals.var_q_c__blk856_dn9,)
    }
};
        locals.var_q_c__blk856 = assign31520_e34515;
        locals.var_q_c__blk856_dn4 = assign31520_e34515_d_n4;
        locals.var_q_c__blk856_dn6 = assign31520_e34515_d_n6;
        locals.var_q_c__blk856_dn7 = assign31520_e34515_d_n7;
        locals.var_q_c__blk856_dn8 = assign31520_e34515_d_n8;
        locals.var_q_c__blk856_dn9 = assign31520_e34515_d_n9;

        let (assign31530_e34528, assign31530_e34528_d_n4, assign31530_e34528_d_n6, assign31530_e34528_d_n7, assign31530_e34528_d_n8, assign31530_e34528_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31530_e34519: f64 = (locals.var_q_b__blk855 * locals.var_q_b__blk855);
        let assign31530_e34522: f64 = (4.0 * locals.var_q_a__blk854);
        let assign31530_e34524: f64 = (assign31530_e34522 * locals.var_q_c__blk856);
        let assign31530_e34525: f64 = (assign31530_e34519 - assign31530_e34524);
        let assign31530_e34526: f64 = (assign31530_e34525).sqrt();
        (assign31530_e34526, ((((locals.var_q_b__blk855_dn4 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn4)) - (((4.0 * locals.var_q_a__blk854_dn4) * locals.var_q_c__blk856) + (assign31530_e34522 * locals.var_q_c__blk856_dn4))) / (2.0 * assign31530_e34526)), ((((locals.var_q_b__blk855_dn6 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn6)) - (((4.0 * locals.var_q_a__blk854_dn6) * locals.var_q_c__blk856) + (assign31530_e34522 * locals.var_q_c__blk856_dn6))) / (2.0 * assign31530_e34526)), ((((locals.var_q_b__blk855_dn7 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn7)) - (((4.0 * locals.var_q_a__blk854_dn7) * locals.var_q_c__blk856) + (assign31530_e34522 * locals.var_q_c__blk856_dn7))) / (2.0 * assign31530_e34526)), ((((locals.var_q_b__blk855_dn8 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn8)) - (((4.0 * locals.var_q_a__blk854_dn8) * locals.var_q_c__blk856) + (assign31530_e34522 * locals.var_q_c__blk856_dn8))) / (2.0 * assign31530_e34526)), ((((locals.var_q_b__blk855_dn9 * locals.var_q_b__blk855) + (locals.var_q_b__blk855 * locals.var_q_b__blk855_dn9)) - (((4.0 * locals.var_q_a__blk854_dn9) * locals.var_q_c__blk856) + (assign31530_e34522 * locals.var_q_c__blk856_dn9))) / (2.0 * assign31530_e34526)),)
    } else {
        (locals.var_q_disc__blk857, locals.var_q_disc__blk857_dn4, locals.var_q_disc__blk857_dn6, locals.var_q_disc__blk857_dn7, locals.var_q_disc__blk857_dn8, locals.var_q_disc__blk857_dn9,)
    }
};
        locals.var_q_disc__blk857 = assign31530_e34528;
        locals.var_q_disc__blk857_dn4 = assign31530_e34528_d_n4;
        locals.var_q_disc__blk857_dn6 = assign31530_e34528_d_n6;
        locals.var_q_disc__blk857_dn7 = assign31530_e34528_d_n7;
        locals.var_q_disc__blk857_dn8 = assign31530_e34528_d_n8;
        locals.var_q_disc__blk857_dn9 = assign31530_e34528_d_n9;

        let (assign31540_e34538, assign31540_e34538_d_n4, assign31540_e34538_d_n6, assign31540_e34538_d_n7, assign31540_e34538_d_n8, assign31540_e34538_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31540_e34532: f64 = (locals.var_q_disc__blk857 - locals.var_q_b__blk855);
        let assign31540_e34535: f64 = (2.0 * locals.var_q_a__blk854);
        let assign31540_e34536: f64 = (assign31540_e34532 / assign31540_e34535);
        (assign31540_e34536, ((((locals.var_q_disc__blk857_dn4 - locals.var_q_b__blk855_dn4) * assign31540_e34535) - (assign31540_e34532 * (2.0 * locals.var_q_a__blk854_dn4))) / (assign31540_e34535 * assign31540_e34535)), ((((locals.var_q_disc__blk857_dn6 - locals.var_q_b__blk855_dn6) * assign31540_e34535) - (assign31540_e34532 * (2.0 * locals.var_q_a__blk854_dn6))) / (assign31540_e34535 * assign31540_e34535)), ((((locals.var_q_disc__blk857_dn7 - locals.var_q_b__blk855_dn7) * assign31540_e34535) - (assign31540_e34532 * (2.0 * locals.var_q_a__blk854_dn7))) / (assign31540_e34535 * assign31540_e34535)), ((((locals.var_q_disc__blk857_dn8 - locals.var_q_b__blk855_dn8) * assign31540_e34535) - (assign31540_e34532 * (2.0 * locals.var_q_a__blk854_dn8))) / (assign31540_e34535 * assign31540_e34535)), ((((locals.var_q_disc__blk857_dn9 - locals.var_q_b__blk855_dn9) * assign31540_e34535) - (assign31540_e34532 * (2.0 * locals.var_q_a__blk854_dn9))) / (assign31540_e34535 * assign31540_e34535)),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign31540_e34538;
        locals.var_q_qsq__blk825_dn4 = assign31540_e34538_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign31540_e34538_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign31540_e34538_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign31540_e34538_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign31540_e34538_d_n9;

        let assign31550_e34541: f64 = (-0.005);
        let assign31550_e34542: f64 = if locals.var_q_qsq__blk825 < assign31550_e34541 { 1.0 } else { 0.0 };
        locals.var_guard1111 = assign31550_e34542;

        let (assign31560_e34550, assign31560_e34550_d_n4, assign31560_e34550_d_n6, assign31560_e34550_d_n7, assign31560_e34550_d_n8, assign31560_e34550_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1111 != 0.0)) {
        let assign31560_e34547: f64 = (locals.var_q_qsq__blk825).abs();
        let assign31560_e34548: f64 = (assign31560_e34547).sqrt();
        (assign31560_e34548, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign31560_e34548)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign31560_e34548)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign31560_e34548)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign31560_e34548)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign31560_e34548)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign31560_e34550;
        locals.var_q_rac_qsq__blk828_dn4 = assign31560_e34550_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign31560_e34550_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign31560_e34550_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign31560_e34550_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign31560_e34550_d_n9;

        let (assign31570_e34561, assign31570_e34561_d_n4, assign31570_e34561_d_n6, assign31570_e34561_d_n7, assign31570_e34561_d_n8, assign31570_e34561_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1111 != 0.0)) {
        let assign31570_e34557: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign31570_e34558: f64 = (assign31570_e34557).tan();
        let assign31570_e34559: f64 = (locals.var_q_rac_qsq__blk828 / assign31570_e34558);
        (assign31570_e34559, (((locals.var_q_rac_qsq__blk828_dn4 * assign31570_e34558) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign31570_e34557).cos() * (assign31570_e34557).cos())))) / (assign31570_e34558 * assign31570_e34558)), (((locals.var_q_rac_qsq__blk828_dn6 * assign31570_e34558) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign31570_e34557).cos() * (assign31570_e34557).cos())))) / (assign31570_e34558 * assign31570_e34558)), (((locals.var_q_rac_qsq__blk828_dn7 * assign31570_e34558) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign31570_e34557).cos() * (assign31570_e34557).cos())))) / (assign31570_e34558 * assign31570_e34558)), (((locals.var_q_rac_qsq__blk828_dn8 * assign31570_e34558) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign31570_e34557).cos() * (assign31570_e34557).cos())))) / (assign31570_e34558 * assign31570_e34558)), (((locals.var_q_rac_qsq__blk828_dn9 * assign31570_e34558) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign31570_e34557).cos() * (assign31570_e34557).cos())))) / (assign31570_e34558 * assign31570_e34558)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign31570_e34561;
        locals.var_q_qcoth__blk829_dn4 = assign31570_e34561_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign31570_e34561_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign31570_e34561_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign31570_e34561_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign31570_e34561_d_n9;

        let (assign31580_e34577, assign31580_e34577_d_n4, assign31580_e34577_d_n6, assign31580_e34577_d_n7, assign31580_e34577_d_n8, assign31580_e34577_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1111 != 0.0)) {
        let assign31580_e34570: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign31580_e34571: f64 = (locals.var_q_qcoth__blk829 * assign31580_e34570);
        let assign31580_e34572: f64 = (locals.var_q_qsq__blk825 + assign31580_e34571);
        let assign31580_e34573: f64 = (0.25 * assign31580_e34572);
        let assign31580_e34575: f64 = (assign31580_e34573 / locals.var_q_qsq__blk825);
        (assign31580_e34575, ((((0.25 * (locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign31580_e34570) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4))))) * locals.var_q_qsq__blk825) - (assign31580_e34573 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign31580_e34570) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6))))) * locals.var_q_qsq__blk825) - (assign31580_e34573 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign31580_e34570) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7))))) * locals.var_q_qsq__blk825) - (assign31580_e34573 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign31580_e34570) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8))))) * locals.var_q_qsq__blk825) - (assign31580_e34573 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign31580_e34570) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9))))) * locals.var_q_qsq__blk825) - (assign31580_e34573 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign31580_e34577;
        locals.var_q_d1_qcoth__blk830_dn4 = assign31580_e34577_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign31580_e34577_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign31580_e34577_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign31580_e34577_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign31580_e34577_d_n9;

        let assign31590_e34580: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1112 = assign31590_e34580;

        let (assign31600_e34591, assign31600_e34591_d_n4, assign31600_e34591_d_n6, assign31600_e34591_d_n7, assign31600_e34591_d_n8, assign31600_e34591_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1111 == 0.0)) && (locals.var_guard1112 != 0.0)) {
        let assign31600_e34588: f64 = (locals.var_q_qsq__blk825).abs();
        let assign31600_e34589: f64 = (assign31600_e34588).sqrt();
        (assign31600_e34589, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign31600_e34589)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign31600_e34589)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign31600_e34589)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign31600_e34589)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign31600_e34589)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign31600_e34591;
        locals.var_q_rac_qsq__blk828_dn4 = assign31600_e34591_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign31600_e34591_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign31600_e34591_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign31600_e34591_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign31600_e34591_d_n9;

        let (assign31610_e34602, assign31610_e34602_d_n4, assign31610_e34602_d_n6, assign31610_e34602_d_n7, assign31610_e34602_d_n8, assign31610_e34602_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1111 == 0.0)) && (locals.var_guard1112 != 0.0)) {
        let assign31610_e34599: f64 = (-locals.var_q_rac_qsq__blk828);
        let assign31610_e34600: f64 = (assign31610_e34599).exp();
        (assign31610_e34600, (assign31610_e34600 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign31610_e34600 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign31610_e34600 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign31610_e34600 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign31610_e34600 * (-locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign31610_e34602;
        locals.var_q_invexpq__blk831_dn4 = assign31610_e34602_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign31610_e34602_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign31610_e34602_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign31610_e34602_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign31610_e34602_d_n9;

        let (assign31620_e34619, assign31620_e34619_d_n4, assign31620_e34619_d_n6, assign31620_e34619_d_n7, assign31620_e34619_d_n8, assign31620_e34619_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1111 == 0.0)) && (locals.var_guard1112 != 0.0)) {
        let assign31620_e34612: f64 = (1.0 + locals.var_q_invexpq__blk831);
        let assign31620_e34613: f64 = (locals.var_q_rac_qsq__blk828 * assign31620_e34612);
        let assign31620_e34616: f64 = (1.0 - locals.var_q_invexpq__blk831);
        let assign31620_e34617: f64 = (assign31620_e34613 / assign31620_e34616);
        (assign31620_e34617, (((((locals.var_q_rac_qsq__blk828_dn4 * assign31620_e34612) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign31620_e34616) - (assign31620_e34613 * (-locals.var_q_invexpq__blk831_dn4))) / (assign31620_e34616 * assign31620_e34616)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign31620_e34612) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign31620_e34616) - (assign31620_e34613 * (-locals.var_q_invexpq__blk831_dn6))) / (assign31620_e34616 * assign31620_e34616)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign31620_e34612) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign31620_e34616) - (assign31620_e34613 * (-locals.var_q_invexpq__blk831_dn7))) / (assign31620_e34616 * assign31620_e34616)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign31620_e34612) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign31620_e34616) - (assign31620_e34613 * (-locals.var_q_invexpq__blk831_dn8))) / (assign31620_e34616 * assign31620_e34616)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign31620_e34612) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign31620_e34616) - (assign31620_e34613 * (-locals.var_q_invexpq__blk831_dn9))) / (assign31620_e34616 * assign31620_e34616)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign31620_e34619;
        locals.var_q_qcoth__blk829_dn4 = assign31620_e34619_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign31620_e34619_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign31620_e34619_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign31620_e34619_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign31620_e34619_d_n9;

        let (assign31630_e34638, assign31630_e34638_d_n4, assign31630_e34638_d_n6, assign31630_e34638_d_n7, assign31630_e34638_d_n8, assign31630_e34638_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1111 == 0.0)) && (locals.var_guard1112 != 0.0)) {
        let assign31630_e34631: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign31630_e34632: f64 = (locals.var_q_qcoth__blk829 * assign31630_e34631);
        let assign31630_e34633: f64 = (locals.var_q_qsq__blk825 + assign31630_e34632);
        let assign31630_e34634: f64 = (0.25 * assign31630_e34633);
        let assign31630_e34636: f64 = (assign31630_e34634 / locals.var_q_qsq__blk825);
        (assign31630_e34636, ((((0.25 * (locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign31630_e34631) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4))))) * locals.var_q_qsq__blk825) - (assign31630_e34634 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign31630_e34631) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6))))) * locals.var_q_qsq__blk825) - (assign31630_e34634 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign31630_e34631) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7))))) * locals.var_q_qsq__blk825) - (assign31630_e34634 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign31630_e34631) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8))))) * locals.var_q_qsq__blk825) - (assign31630_e34634 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * (locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign31630_e34631) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9))))) * locals.var_q_qsq__blk825) - (assign31630_e34634 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign31630_e34638;
        locals.var_q_d1_qcoth__blk830_dn4 = assign31630_e34638_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign31630_e34638_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign31630_e34638_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign31630_e34638_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign31630_e34638_d_n9;

        let (assign31640_e34664, assign31640_e34664_d_n4, assign31640_e34664_d_n6, assign31640_e34664_d_n7, assign31640_e34664_d_n8, assign31640_e34664_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1111 == 0.0)) && (locals.var_guard1112 == 0.0)) {
        let assign31640_e34649: f64 = (locals.var_q_qsq__blk825 * 0.1666666666667);
        let assign31640_e34653: f64 = (locals.var_q_qsq__blk825 * 0.0166666666667);
        let assign31640_e34657: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign31640_e34658: f64 = (1.0 - assign31640_e34657);
        let assign31640_e34659: f64 = (assign31640_e34653 * assign31640_e34658);
        let assign31640_e34660: f64 = (1.0 - assign31640_e34659);
        let assign31640_e34661: f64 = (assign31640_e34649 * assign31640_e34660);
        let assign31640_e34662: f64 = (2.0 + assign31640_e34661);
        (assign31640_e34662, (((locals.var_q_qsq__blk825_dn4 * 0.1666666666667) * assign31640_e34660) + (assign31640_e34649 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0166666666667) * assign31640_e34658) + (assign31640_e34653 * (-(locals.var_q_qsq__blk825_dn4 * 0.0238095238095))))))), (((locals.var_q_qsq__blk825_dn6 * 0.1666666666667) * assign31640_e34660) + (assign31640_e34649 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0166666666667) * assign31640_e34658) + (assign31640_e34653 * (-(locals.var_q_qsq__blk825_dn6 * 0.0238095238095))))))), (((locals.var_q_qsq__blk825_dn7 * 0.1666666666667) * assign31640_e34660) + (assign31640_e34649 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0166666666667) * assign31640_e34658) + (assign31640_e34653 * (-(locals.var_q_qsq__blk825_dn7 * 0.0238095238095))))))), (((locals.var_q_qsq__blk825_dn8 * 0.1666666666667) * assign31640_e34660) + (assign31640_e34649 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0166666666667) * assign31640_e34658) + (assign31640_e34653 * (-(locals.var_q_qsq__blk825_dn8 * 0.0238095238095))))))), (((locals.var_q_qsq__blk825_dn9 * 0.1666666666667) * assign31640_e34660) + (assign31640_e34649 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0166666666667) * assign31640_e34658) + (assign31640_e34653 * (-(locals.var_q_qsq__blk825_dn9 * 0.0238095238095))))))),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign31640_e34664;
        locals.var_q_qcoth__blk829_dn4 = assign31640_e34664_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign31640_e34664_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign31640_e34664_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign31640_e34664_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign31640_e34664_d_n9;

        let (assign31650_e34692, assign31650_e34692_d_n4, assign31650_e34692_d_n6, assign31650_e34692_d_n7, assign31650_e34692_d_n8, assign31650_e34692_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1111 == 0.0)) && (locals.var_guard1112 == 0.0)) {
        let assign31650_e34676: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign31650_e34680: f64 = (locals.var_q_qsq__blk825 * 0.0357142857143);
        let assign31650_e34684: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign31650_e34685: f64 = (1.0 - assign31650_e34684);
        let assign31650_e34686: f64 = (assign31650_e34680 * assign31650_e34685);
        let assign31650_e34687: f64 = (1.0 - assign31650_e34686);
        let assign31650_e34688: f64 = (assign31650_e34676 * assign31650_e34687);
        let assign31650_e34689: f64 = (1.0 - assign31650_e34688);
        let assign31650_e34690: f64 = (0.1666666666667 * assign31650_e34689);
        (assign31650_e34690, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0333333333333) * assign31650_e34687) + (assign31650_e34676 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0357142857143) * assign31650_e34685) + (assign31650_e34680 * (-(locals.var_q_qsq__blk825_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0333333333333) * assign31650_e34687) + (assign31650_e34676 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0357142857143) * assign31650_e34685) + (assign31650_e34680 * (-(locals.var_q_qsq__blk825_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0333333333333) * assign31650_e34687) + (assign31650_e34676 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0357142857143) * assign31650_e34685) + (assign31650_e34680 * (-(locals.var_q_qsq__blk825_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0333333333333) * assign31650_e34687) + (assign31650_e34676 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0357142857143) * assign31650_e34685) + (assign31650_e34680 * (-(locals.var_q_qsq__blk825_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0333333333333) * assign31650_e34687) + (assign31650_e34676 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0357142857143) * assign31650_e34685) + (assign31650_e34680 * (-(locals.var_q_qsq__blk825_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign31650_e34692;
        locals.var_q_d1_qcoth__blk830_dn4 = assign31650_e34692_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign31650_e34692_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign31650_e34692_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign31650_e34692_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign31650_e34692_d_n9;

        let (assign31660_e34712, assign31660_e34712_d_n4, assign31660_e34712_d_n6, assign31660_e34712_d_n7, assign31660_e34712_d_n8, assign31660_e34712_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31660_e34697: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_qcoth__blk829);
        let assign31660_e34700: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853);
        let assign31660_e34701: f64 = (assign31660_e34697 + assign31660_e34700);
        let assign31660_e34703: f64 = (assign31660_e34701 + locals.var_q_qsq__blk825);
        let assign31660_e34706: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_qcoth__blk830);
        let assign31660_e34708: f64 = (assign31660_e34706 + 1.0);
        let assign31660_e34709: f64 = (assign31660_e34703 / assign31660_e34708);
        let assign31660_e34710: f64 = (locals.var_q_qsq__blk825 - assign31660_e34709);
        (assign31660_e34710, (locals.var_q_qsq__blk825_dn4 - (((((((locals.var_q_qi_int__blk846_dn4 * locals.var_q_qcoth__blk829) + (locals.var_q_qi_int__blk846 * locals.var_q_qcoth__blk829_dn4)) + ((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn4))) + locals.var_q_qsq__blk825_dn4) * assign31660_e34708) - (assign31660_e34703 * ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_qcoth__blk830) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_qcoth__blk830_dn4)))) / (assign31660_e34708 * assign31660_e34708))), (locals.var_q_qsq__blk825_dn6 - (((((((locals.var_q_qi_int__blk846_dn6 * locals.var_q_qcoth__blk829) + (locals.var_q_qi_int__blk846 * locals.var_q_qcoth__blk829_dn6)) + ((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn6))) + locals.var_q_qsq__blk825_dn6) * assign31660_e34708) - (assign31660_e34703 * ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_qcoth__blk830) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_qcoth__blk830_dn6)))) / (assign31660_e34708 * assign31660_e34708))), (locals.var_q_qsq__blk825_dn7 - (((((((locals.var_q_qi_int__blk846_dn7 * locals.var_q_qcoth__blk829) + (locals.var_q_qi_int__blk846 * locals.var_q_qcoth__blk829_dn7)) + ((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn7))) + locals.var_q_qsq__blk825_dn7) * assign31660_e34708) - (assign31660_e34703 * ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_qcoth__blk830) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_qcoth__blk830_dn7)))) / (assign31660_e34708 * assign31660_e34708))), (locals.var_q_qsq__blk825_dn8 - (((((((locals.var_q_qi_int__blk846_dn8 * locals.var_q_qcoth__blk829) + (locals.var_q_qi_int__blk846 * locals.var_q_qcoth__blk829_dn8)) + ((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn8))) + locals.var_q_qsq__blk825_dn8) * assign31660_e34708) - (assign31660_e34703 * ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_qcoth__blk830) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_qcoth__blk830_dn8)))) / (assign31660_e34708 * assign31660_e34708))), (locals.var_q_qsq__blk825_dn9 - (((((((locals.var_q_qi_int__blk846_dn9 * locals.var_q_qcoth__blk829) + (locals.var_q_qi_int__blk846 * locals.var_q_qcoth__blk829_dn9)) + ((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k2q2__blk853) + (locals.var_q_k1q1__blk823 * locals.var_q_k2q2__blk853_dn9))) + locals.var_q_qsq__blk825_dn9) * assign31660_e34708) - (assign31660_e34703 * ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_qcoth__blk830) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_qcoth__blk830_dn9)))) / (assign31660_e34708 * assign31660_e34708))),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign31660_e34712;
        locals.var_q_qsq__blk825_dn4 = assign31660_e34712_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign31660_e34712_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign31660_e34712_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign31660_e34712_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign31660_e34712_d_n9;

        let (assign31670_e34720, assign31670_e34720_d_n4, assign31670_e34720_d_n6, assign31670_e34720_d_n7, assign31670_e34720_d_n8, assign31670_e34720_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31670_e34716: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
        let assign31670_e34718: f64 = (assign31670_e34716 - locals.var_q_qsq__blk825);
        (assign31670_e34718, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_qsq__blk825_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_qsq__blk825_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_qsq__blk825_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_qsq__blk825_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_qsq__blk825_dn9),)
    } else {
        (locals.var_q_delta__blk858, locals.var_q_delta__blk858_dn4, locals.var_q_delta__blk858_dn6, locals.var_q_delta__blk858_dn7, locals.var_q_delta__blk858_dn8, locals.var_q_delta__blk858_dn9,)
    }
};
        locals.var_q_delta__blk858 = assign31670_e34720;
        locals.var_q_delta__blk858_dn4 = assign31670_e34720_d_n4;
        locals.var_q_delta__blk858_dn6 = assign31670_e34720_d_n6;
        locals.var_q_delta__blk858_dn7 = assign31670_e34720_d_n7;
        locals.var_q_delta__blk858_dn8 = assign31670_e34720_d_n8;
        locals.var_q_delta__blk858_dn9 = assign31670_e34720_d_n9;

        let assign31680_e34723: f64 = if locals.var_q_delta__blk858 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1113 = assign31680_e34723;

        let (assign31690_e34740, assign31690_e34740_d_n4, assign31690_e34740_d_n6, assign31690_e34740_d_n7, assign31690_e34740_d_n8, assign31690_e34740_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1113 != 0.0)) {
        let assign31690_e34730: f64 = (locals.var_q_delta__blk858 / locals.var_a0__blk905);
        let assign31690_e34731: f64 = (assign31690_e34730).ln();
        let assign31690_e34733: f64 = assign31690_e34731;
        let assign31690_e34735: f64 = (assign31690_e34733 - locals.var_xg1x__blk930);
        let assign31690_e34737: f64 = (assign31690_e34735 + locals.var_q1s__blk937);
        let assign31690_e34738: f64 = (locals.var_q_delta__blk858 * assign31690_e34737);
        (assign31690_e34738, ((locals.var_q_delta__blk858_dn4 * assign31690_e34737) + (locals.var_q_delta__blk858 * ((((((locals.var_q_delta__blk858_dn4 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign31690_e34730) - locals.var_xg1x__blk930_dn4) + locals.var_q1s__blk937_dn4))), ((locals.var_q_delta__blk858_dn6 * assign31690_e34737) + (locals.var_q_delta__blk858 * ((((((locals.var_q_delta__blk858_dn6 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign31690_e34730) - locals.var_xg1x__blk930_dn6) + locals.var_q1s__blk937_dn6))), ((locals.var_q_delta__blk858_dn7 * assign31690_e34737) + (locals.var_q_delta__blk858 * ((((((locals.var_q_delta__blk858_dn7 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign31690_e34730) - locals.var_xg1x__blk930_dn7) + locals.var_q1s__blk937_dn7))), ((locals.var_q_delta__blk858_dn8 * assign31690_e34737) + (locals.var_q_delta__blk858 * ((((((locals.var_q_delta__blk858_dn8 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign31690_e34730) - locals.var_xg1x__blk930_dn8) + locals.var_q1s__blk937_dn8))), ((locals.var_q_delta__blk858_dn9 * assign31690_e34737) + (locals.var_q_delta__blk858 * ((((((locals.var_q_delta__blk858_dn9 * locals.var_a0__blk905) - (locals.var_q_delta__blk858 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)) / assign31690_e34730) - locals.var_xg1x__blk930_dn9) + locals.var_q1s__blk937_dn9))),)
    } else {
        (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9,)
    }
};
        locals.var_q_zero__blk849 = assign31690_e34740;
        locals.var_q_zero__blk849_dn4 = assign31690_e34740_d_n4;
        locals.var_q_zero__blk849_dn6 = assign31690_e34740_d_n6;
        locals.var_q_zero__blk849_dn7 = assign31690_e34740_d_n7;
        locals.var_q_zero__blk849_dn8 = assign31690_e34740_d_n8;
        locals.var_q_zero__blk849_dn9 = assign31690_e34740_d_n9;

        let (assign31700_e34752, assign31700_e34752_d_n4, assign31700_e34752_d_n6, assign31700_e34752_d_n7, assign31700_e34752_d_n8, assign31700_e34752_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1113 != 0.0)) {
        let assign31700_e34746: f64 = (2.0 * locals.var_k1__blk932);
        let assign31700_e34748: f64 = (assign31700_e34746 * locals.var_q_k1q1__blk823);
        let assign31700_e34750: f64 = (assign31700_e34748 + locals.var_q_delta__blk858);
        (assign31700_e34750, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign31700_e34746 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_delta__blk858_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign31700_e34746 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_delta__blk858_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign31700_e34746 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_delta__blk858_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign31700_e34746 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_delta__blk858_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign31700_e34746 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_delta__blk858_dn9),)
    } else {
        (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9,)
    }
};
        locals.var_q_d1_zero__blk850 = assign31700_e34752;
        locals.var_q_d1_zero__blk850_dn4 = assign31700_e34752_d_n4;
        locals.var_q_d1_zero__blk850_dn6 = assign31700_e34752_d_n6;
        locals.var_q_d1_zero__blk850_dn7 = assign31700_e34752_d_n7;
        locals.var_q_d1_zero__blk850_dn8 = assign31700_e34752_d_n8;
        locals.var_q_d1_zero__blk850_dn9 = assign31700_e34752_d_n9;

        let (assign31710_e34762,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1113 != 0.0)) {
        let assign31710_e34758: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign31710_e34760: f64 = (assign31710_e34758 - locals.var_q_x1sat__blk817);
        (assign31710_e34760,)
    } else {
        (locals.var_q_dx1__blk859,)
    }
};
        locals.var_q_dx1__blk859 = assign31710_e34762;

        let assign31720_e34772: f64 = (locals.var_q_dx1__blk859 + 2.3025850929941);
        let assign31720_e34774: f64 = (locals.var_k1__blk932).ln();
        let assign31720_e34775: f64 = (assign31720_e34772 + assign31720_e34774);
        let assign31720_e34782: f64 = if ((((locals.var_q_zero__blk849 < 0.0) && (locals.var_q_d1_zero__blk850 > 0.0)) && (assign31720_e34775 > 0.0)) || (locals.var_q_dx1__blk859 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1114 = assign31720_e34782;

        let (assign31730_e34794, assign31730_e34794_d_n4, assign31730_e34794_d_n6, assign31730_e34794_d_n7, assign31730_e34794_d_n8, assign31730_e34794_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1113 != 0.0)) && (locals.var_guard1114 != 0.0)) {
        let assign31730_e34791: f64 = (locals.var_q_zero__blk849 / locals.var_q_d1_zero__blk850);
        let assign31730_e34792: f64 = (locals.var_q1s__blk937 - assign31730_e34791);
        (assign31730_e34792, (locals.var_q1s__blk937_dn4 - (((locals.var_q_zero__blk849_dn4 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn4)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1s__blk937_dn6 - (((locals.var_q_zero__blk849_dn6 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn6)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1s__blk937_dn7 - (((locals.var_q_zero__blk849_dn7 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn7)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1s__blk937_dn8 - (((locals.var_q_zero__blk849_dn8 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn8)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))), (locals.var_q1s__blk937_dn9 - (((locals.var_q_zero__blk849_dn9 * locals.var_q_d1_zero__blk850) - (locals.var_q_zero__blk849 * locals.var_q_d1_zero__blk850_dn9)) / (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850))),)
    } else {
        (locals.var_q1s__blk937, locals.var_q1s__blk937_dn4, locals.var_q1s__blk937_dn6, locals.var_q1s__blk937_dn7, locals.var_q1s__blk937_dn8, locals.var_q1s__blk937_dn9,)
    }
};
        locals.var_q1s__blk937 = assign31730_e34794;
        locals.var_q1s__blk937_dn4 = assign31730_e34794_d_n4;
        locals.var_q1s__blk937_dn6 = assign31730_e34794_d_n6;
        locals.var_q1s__blk937_dn7 = assign31730_e34794_d_n7;
        locals.var_q1s__blk937_dn8 = assign31730_e34794_d_n8;
        locals.var_q1s__blk937_dn9 = assign31730_e34794_d_n9;

        let (assign31740_e34800, assign31740_e34800_d_n4, assign31740_e34800_d_n6, assign31740_e34800_d_n7, assign31740_e34800_d_n8, assign31740_e34800_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31740_e34798: f64 = (locals.var_k1__blk932 * locals.var_q1s__blk937);
        (assign31740_e34798, ((locals.var_k1__blk932_dn4 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn9)),)
    } else {
        (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9,)
    }
};
        locals.var_q_k1q1__blk823 = assign31740_e34800;
        locals.var_q_k1q1__blk823_dn4 = assign31740_e34800_d_n4;
        locals.var_q_k1q1__blk823_dn6 = assign31740_e34800_d_n6;
        locals.var_q_k1q1__blk823_dn7 = assign31740_e34800_d_n7;
        locals.var_q_k1q1__blk823_dn8 = assign31740_e34800_d_n8;
        locals.var_q_k1q1__blk823_dn9 = assign31740_e34800_d_n9;

        let assign31750_e34803: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign31750_e34805: f64 = assign31750_e34803;
        let assign31750_e34807: f64 = if assign31750_e34805 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1115 = assign31750_e34807;

        let (assign31760_e34818, assign31760_e34818_d_n4, assign31760_e34818_d_n6, assign31760_e34818_d_n7, assign31760_e34818_d_n8, assign31760_e34818_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1115 != 0.0)) {
        let assign31760_e34813: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign31760_e34815: f64 = assign31760_e34813;
        let assign31760_e34816: f64 = (assign31760_e34815).exp();
        (assign31760_e34816, (assign31760_e34816 * (locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4)), (assign31760_e34816 * (locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6)), (assign31760_e34816 * (locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7)), (assign31760_e34816 * (locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8)), (assign31760_e34816 * (locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign31760_e34818;
        locals.var_q_temp1__blk814_dn4 = assign31760_e34818_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign31760_e34818_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign31760_e34818_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign31760_e34818_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign31760_e34818_d_n9;

        let (assign31770_e34859, assign31770_e34859_d_n4, assign31770_e34859_d_n6, assign31770_e34859_d_n7, assign31770_e34859_d_n8, assign31770_e34859_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1115 == 0.0)) {
        let assign31770_e34827: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign31770_e34829: f64 = assign31770_e34827;
        let assign31770_e34831: f64 = (assign31770_e34829 - 80.0);
        let assign31770_e34836: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign31770_e34838: f64 = assign31770_e34836;
        let assign31770_e34840: f64 = (assign31770_e34838 - 80.0);
        let assign31770_e34841: f64 = (0.5 * assign31770_e34840);
        let assign31770_e34845: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign31770_e34847: f64 = assign31770_e34845;
        let assign31770_e34849: f64 = (assign31770_e34847 - 80.0);
        let assign31770_e34851: f64 = (assign31770_e34849 * 0.3333333333333);
        let assign31770_e34852: f64 = (1.0 + assign31770_e34851);
        let assign31770_e34853: f64 = (assign31770_e34841 * assign31770_e34852);
        let assign31770_e34854: f64 = (1.0 + assign31770_e34853);
        let assign31770_e34855: f64 = (assign31770_e34831 * assign31770_e34854);
        let assign31770_e34856: f64 = (1.0 + assign31770_e34855);
        let assign31770_e34857: f64 = (5.54062e34 * assign31770_e34856);
        (assign31770_e34857, (5.54062e34 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4) * assign31770_e34854) + (assign31770_e34831 * (((0.5 * (locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4)) * assign31770_e34852) + (assign31770_e34841 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6) * assign31770_e34854) + (assign31770_e34831 * (((0.5 * (locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6)) * assign31770_e34852) + (assign31770_e34841 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7) * assign31770_e34854) + (assign31770_e34831 * (((0.5 * (locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7)) * assign31770_e34852) + (assign31770_e34841 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8) * assign31770_e34854) + (assign31770_e34831 * (((0.5 * (locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8)) * assign31770_e34852) + (assign31770_e34841 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9) * assign31770_e34854) + (assign31770_e34831 * (((0.5 * (locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9)) * assign31770_e34852) + (assign31770_e34841 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign31770_e34859;
        locals.var_q_temp1__blk814_dn4 = assign31770_e34859_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign31770_e34859_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign31770_e34859_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign31770_e34859_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign31770_e34859_d_n9;

        let (assign31780_e34865, assign31780_e34865_d_n4, assign31780_e34865_d_n6, assign31780_e34865_d_n7, assign31780_e34865_d_n8, assign31780_e34865_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31780_e34863: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
        (assign31780_e34863, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_aexp__blk824, locals.var_q_aexp__blk824_dn4, locals.var_q_aexp__blk824_dn6, locals.var_q_aexp__blk824_dn7, locals.var_q_aexp__blk824_dn8, locals.var_q_aexp__blk824_dn9,)
    }
};
        locals.var_q_aexp__blk824 = assign31780_e34865;
        locals.var_q_aexp__blk824_dn4 = assign31780_e34865_d_n4;
        locals.var_q_aexp__blk824_dn6 = assign31780_e34865_d_n6;
        locals.var_q_aexp__blk824_dn7 = assign31780_e34865_d_n7;
        locals.var_q_aexp__blk824_dn8 = assign31780_e34865_d_n8;
        locals.var_q_aexp__blk824_dn9 = assign31780_e34865_d_n9;

        let (assign31790_e34873, assign31790_e34873_d_n4, assign31790_e34873_d_n6, assign31790_e34873_d_n7, assign31790_e34873_d_n8, assign31790_e34873_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31790_e34869: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
        let assign31790_e34871: f64 = (assign31790_e34869 - locals.var_q_aexp__blk824);
        (assign31790_e34871, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign31790_e34873;
        locals.var_q_qsq__blk825_dn4 = assign31790_e34873_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign31790_e34873_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign31790_e34873_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign31790_e34873_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign31790_e34873_d_n9;

    }

    pub(super) fn stamp_transient_block_85(
        locals: &mut StampLocals,
    ) {
        let (assign31800_e34883, assign31800_e34883_d_n4, assign31800_e34883_d_n6, assign31800_e34883_d_n7, assign31800_e34883_d_n8, assign31800_e34883_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31800_e34877: f64 = (2.0 * locals.var_k1__blk932);
        let assign31800_e34879: f64 = (assign31800_e34877 * locals.var_q_k1q1__blk823);
        let assign31800_e34881: f64 = (assign31800_e34879 + locals.var_q_aexp__blk824);
        (assign31800_e34881, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign31800_e34877 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign31800_e34877 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign31800_e34877 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign31800_e34877 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign31800_e34877 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_qsq__blk826, locals.var_q_d1_qsq__blk826_dn4, locals.var_q_d1_qsq__blk826_dn6, locals.var_q_d1_qsq__blk826_dn7, locals.var_q_d1_qsq__blk826_dn8, locals.var_q_d1_qsq__blk826_dn9,)
    }
};
        locals.var_q_d1_qsq__blk826 = assign31800_e34883;
        locals.var_q_d1_qsq__blk826_dn4 = assign31800_e34883_d_n4;
        locals.var_q_d1_qsq__blk826_dn6 = assign31800_e34883_d_n6;
        locals.var_q_d1_qsq__blk826_dn7 = assign31800_e34883_d_n7;
        locals.var_q_d1_qsq__blk826_dn8 = assign31800_e34883_d_n8;
        locals.var_q_d1_qsq__blk826_dn9 = assign31800_e34883_d_n9;

        let (assign31810_e34893, assign31810_e34893_d_n4, assign31810_e34893_d_n6, assign31810_e34893_d_n7, assign31810_e34893_d_n8, assign31810_e34893_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign31810_e34887: f64 = (2.0 * locals.var_k1__blk932);
        let assign31810_e34889: f64 = (assign31810_e34887 * locals.var_k1__blk932);
        let assign31810_e34891: f64 = (assign31810_e34889 - locals.var_q_aexp__blk824);
        (assign31810_e34891, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_k1__blk932) + (assign31810_e34887 * locals.var_k1__blk932_dn4)) - locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_k1__blk932) + (assign31810_e34887 * locals.var_k1__blk932_dn6)) - locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_k1__blk932) + (assign31810_e34887 * locals.var_k1__blk932_dn7)) - locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_k1__blk932) + (assign31810_e34887 * locals.var_k1__blk932_dn8)) - locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_k1__blk932) + (assign31810_e34887 * locals.var_k1__blk932_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_qsq__blk827, locals.var_q_d2_qsq__blk827_dn4, locals.var_q_d2_qsq__blk827_dn6, locals.var_q_d2_qsq__blk827_dn7, locals.var_q_d2_qsq__blk827_dn8, locals.var_q_d2_qsq__blk827_dn9,)
    }
};
        locals.var_q_d2_qsq__blk827 = assign31810_e34893;
        locals.var_q_d2_qsq__blk827_dn4 = assign31810_e34893_d_n4;
        locals.var_q_d2_qsq__blk827_dn6 = assign31810_e34893_d_n6;
        locals.var_q_d2_qsq__blk827_dn7 = assign31810_e34893_d_n7;
        locals.var_q_d2_qsq__blk827_dn8 = assign31810_e34893_d_n8;
        locals.var_q_d2_qsq__blk827_dn9 = assign31810_e34893_d_n9;

        let assign31820_e34896: f64 = (-0.005);
        let assign31820_e34897: f64 = if locals.var_q_qsq__blk825 < assign31820_e34896 { 1.0 } else { 0.0 };
        locals.var_guard1116 = assign31820_e34897;

        let (assign31830_e34905, assign31830_e34905_d_n4, assign31830_e34905_d_n6, assign31830_e34905_d_n7, assign31830_e34905_d_n8, assign31830_e34905_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1116 != 0.0)) {
        let assign31830_e34902: f64 = (locals.var_q_qsq__blk825).abs();
        let assign31830_e34903: f64 = (assign31830_e34902).sqrt();
        (assign31830_e34903, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign31830_e34903)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign31830_e34903)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign31830_e34903)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign31830_e34903)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign31830_e34903)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign31830_e34905;
        locals.var_q_rac_qsq__blk828_dn4 = assign31830_e34905_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign31830_e34905_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign31830_e34905_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign31830_e34905_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign31830_e34905_d_n9;

        let (assign31840_e34916, assign31840_e34916_d_n4, assign31840_e34916_d_n6, assign31840_e34916_d_n7, assign31840_e34916_d_n8, assign31840_e34916_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1116 != 0.0)) {
        let assign31840_e34912: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign31840_e34913: f64 = (assign31840_e34912).tan();
        let assign31840_e34914: f64 = (locals.var_q_rac_qsq__blk828 / assign31840_e34913);
        (assign31840_e34914, (((locals.var_q_rac_qsq__blk828_dn4 * assign31840_e34913) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign31840_e34912).cos() * (assign31840_e34912).cos())))) / (assign31840_e34913 * assign31840_e34913)), (((locals.var_q_rac_qsq__blk828_dn6 * assign31840_e34913) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign31840_e34912).cos() * (assign31840_e34912).cos())))) / (assign31840_e34913 * assign31840_e34913)), (((locals.var_q_rac_qsq__blk828_dn7 * assign31840_e34913) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign31840_e34912).cos() * (assign31840_e34912).cos())))) / (assign31840_e34913 * assign31840_e34913)), (((locals.var_q_rac_qsq__blk828_dn8 * assign31840_e34913) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign31840_e34912).cos() * (assign31840_e34912).cos())))) / (assign31840_e34913 * assign31840_e34913)), (((locals.var_q_rac_qsq__blk828_dn9 * assign31840_e34913) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign31840_e34912).cos() * (assign31840_e34912).cos())))) / (assign31840_e34913 * assign31840_e34913)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign31840_e34916;
        locals.var_q_qcoth__blk829_dn4 = assign31840_e34916_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign31840_e34916_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign31840_e34916_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign31840_e34916_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign31840_e34916_d_n9;

        let (assign31850_e34926, assign31850_e34926_d_n4, assign31850_e34926_d_n6, assign31850_e34926_d_n7, assign31850_e34926_d_n8, assign31850_e34926_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1116 != 0.0)) {
        let assign31850_e34922: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign31850_e34924: f64 = (assign31850_e34922 / locals.var_q_qsq__blk825);
        (assign31850_e34924, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign31850_e34922 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign31850_e34922 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign31850_e34922 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign31850_e34922 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign31850_e34922 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign31850_e34926;
        locals.var_q_temp1__blk814_dn4 = assign31850_e34926_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign31850_e34926_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign31850_e34926_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign31850_e34926_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign31850_e34926_d_n9;

        let (assign31860_e34940, assign31860_e34940_d_n4, assign31860_e34940_d_n6, assign31860_e34940_d_n7, assign31860_e34940_d_n8, assign31860_e34940_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1116 != 0.0)) {
        let assign31860_e34934: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign31860_e34935: f64 = (locals.var_q_qcoth__blk829 * assign31860_e34934);
        let assign31860_e34936: f64 = (locals.var_q_qsq__blk825 + assign31860_e34935);
        let assign31860_e34938: f64 = (assign31860_e34936 * locals.var_q_temp1__blk814);
        (assign31860_e34938, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign31860_e34934) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign31860_e34936 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign31860_e34934) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign31860_e34936 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign31860_e34934) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign31860_e34936 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign31860_e34934) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign31860_e34936 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign31860_e34934) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign31860_e34936 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign31860_e34940;
        locals.var_q_d1_qcoth__blk830_dn4 = assign31860_e34940_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign31860_e34940_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign31860_e34940_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign31860_e34940_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign31860_e34940_d_n9;

        let (assign31870_e34962, assign31870_e34962_d_n4, assign31870_e34962_d_n6, assign31870_e34962_d_n7, assign31870_e34962_d_n8, assign31870_e34962_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1116 != 0.0)) {
        let assign31870_e34947: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign31870_e34950: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign31870_e34951: f64 = (assign31870_e34947 * assign31870_e34950);
        let assign31870_e34952: f64 = (locals.var_q_d1_qsq__blk826 - assign31870_e34951);
        let assign31870_e34954: f64 = (assign31870_e34952 * locals.var_q_temp1__blk814);
        let assign31870_e34957: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign31870_e34959: f64 = (assign31870_e34957 / locals.var_q_d1_qsq__blk826);
        let assign31870_e34960: f64 = (assign31870_e34954 + assign31870_e34959);
        (assign31870_e34960, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign31870_e34950) + (assign31870_e34947 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign31870_e34952 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign31870_e34957 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign31870_e34950) + (assign31870_e34947 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign31870_e34952 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign31870_e34957 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign31870_e34950) + (assign31870_e34947 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign31870_e34952 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign31870_e34957 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign31870_e34950) + (assign31870_e34947 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign31870_e34952 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign31870_e34957 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign31870_e34950) + (assign31870_e34947 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign31870_e34952 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign31870_e34957 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign31870_e34962;
        locals.var_q_d2_qcoth__blk832_dn4 = assign31870_e34962_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign31870_e34962_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign31870_e34962_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign31870_e34962_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign31870_e34962_d_n9;

        let (assign31880_e34972, assign31880_e34972_d_n4, assign31880_e34972_d_n6, assign31880_e34972_d_n7, assign31880_e34972_d_n8, assign31880_e34972_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1116 != 0.0)) {
        let assign31880_e34969: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign31880_e34970: f64 = (1.0 - assign31880_e34969);
        (assign31880_e34970, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign31880_e34972;
        locals.var_q_temp2__blk815_dn4 = assign31880_e34972_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign31880_e34972_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign31880_e34972_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign31880_e34972_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign31880_e34972_d_n9;

        let (assign31890_e34982, assign31890_e34982_d_n4, assign31890_e34982_d_n6, assign31890_e34982_d_n7, assign31890_e34982_d_n8, assign31890_e34982_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1116 != 0.0)) {
        let assign31890_e34978: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign31890_e34980: f64 = (assign31890_e34978 * locals.var_q_temp2__blk815);
        (assign31890_e34980, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign31890_e34978 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign31890_e34978 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign31890_e34978 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign31890_e34978 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign31890_e34978 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign31890_e34982;
        locals.var_q_d1_ln__blk835_dn4 = assign31890_e34982_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign31890_e34982_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign31890_e34982_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign31890_e34982_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign31890_e34982_d_n9;

        let (assign31900_e35000, assign31900_e35000_d_n4, assign31900_e35000_d_n6, assign31900_e35000_d_n7, assign31900_e35000_d_n8, assign31900_e35000_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1116 != 0.0)) {
        let assign31900_e34988: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign31900_e34993: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign31900_e34994: f64 = (locals.var_q_d1_ln__blk835 + assign31900_e34993);
        let assign31900_e34995: f64 = (locals.var_q_d1_qsq__blk826 * assign31900_e34994);
        let assign31900_e34996: f64 = (assign31900_e34988 - assign31900_e34995);
        let assign31900_e34998: f64 = (assign31900_e34996 / locals.var_q_qsq__blk825);
        (assign31900_e34998, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign31900_e34994) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign31900_e34996 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign31900_e34994) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign31900_e34996 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign31900_e34994) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign31900_e34996 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign31900_e34994) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign31900_e34996 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign31900_e34994) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign31900_e34996 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign31900_e35000;
        locals.var_q_d2_ln__blk836_dn4 = assign31900_e35000_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign31900_e35000_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign31900_e35000_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign31900_e35000_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign31900_e35000_d_n9;

        let assign31910_e35003: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1117 = assign31910_e35003;

        let (assign31920_e35014, assign31920_e35014_d_n4, assign31920_e35014_d_n6, assign31920_e35014_d_n7, assign31920_e35014_d_n8, assign31920_e35014_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1116 == 0.0)) && (locals.var_guard1117 != 0.0)) {
        let assign31920_e35011: f64 = (locals.var_q_qsq__blk825).abs();
        let assign31920_e35012: f64 = (assign31920_e35011).sqrt();
        (assign31920_e35012, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign31920_e35012)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign31920_e35012)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign31920_e35012)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign31920_e35012)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign31920_e35012)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign31920_e35014;
        locals.var_q_rac_qsq__blk828_dn4 = assign31920_e35014_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign31920_e35014_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign31920_e35014_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign31920_e35014_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign31920_e35014_d_n9;

        let (assign31930_e35025, assign31930_e35025_d_n4, assign31930_e35025_d_n6, assign31930_e35025_d_n7, assign31930_e35025_d_n8, assign31930_e35025_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1116 == 0.0)) && (locals.var_guard1117 != 0.0)) {
        let assign31930_e35022: f64 = (-locals.var_q_rac_qsq__blk828);
        let assign31930_e35023: f64 = (assign31930_e35022).exp();
        (assign31930_e35023, (assign31930_e35023 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign31930_e35023 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign31930_e35023 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign31930_e35023 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign31930_e35023 * (-locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign31930_e35025;
        locals.var_q_invexpq__blk831_dn4 = assign31930_e35025_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign31930_e35025_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign31930_e35025_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign31930_e35025_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign31930_e35025_d_n9;

        let (assign31940_e35042, assign31940_e35042_d_n4, assign31940_e35042_d_n6, assign31940_e35042_d_n7, assign31940_e35042_d_n8, assign31940_e35042_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1116 == 0.0)) && (locals.var_guard1117 != 0.0)) {
        let assign31940_e35035: f64 = (1.0 + locals.var_q_invexpq__blk831);
        let assign31940_e35036: f64 = (locals.var_q_rac_qsq__blk828 * assign31940_e35035);
        let assign31940_e35039: f64 = (1.0 - locals.var_q_invexpq__blk831);
        let assign31940_e35040: f64 = (assign31940_e35036 / assign31940_e35039);
        (assign31940_e35040, (((((locals.var_q_rac_qsq__blk828_dn4 * assign31940_e35035) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign31940_e35039) - (assign31940_e35036 * (-locals.var_q_invexpq__blk831_dn4))) / (assign31940_e35039 * assign31940_e35039)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign31940_e35035) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign31940_e35039) - (assign31940_e35036 * (-locals.var_q_invexpq__blk831_dn6))) / (assign31940_e35039 * assign31940_e35039)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign31940_e35035) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign31940_e35039) - (assign31940_e35036 * (-locals.var_q_invexpq__blk831_dn7))) / (assign31940_e35039 * assign31940_e35039)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign31940_e35035) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign31940_e35039) - (assign31940_e35036 * (-locals.var_q_invexpq__blk831_dn8))) / (assign31940_e35039 * assign31940_e35039)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign31940_e35035) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign31940_e35039) - (assign31940_e35036 * (-locals.var_q_invexpq__blk831_dn9))) / (assign31940_e35039 * assign31940_e35039)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign31940_e35042;
        locals.var_q_qcoth__blk829_dn4 = assign31940_e35042_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign31940_e35042_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign31940_e35042_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign31940_e35042_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign31940_e35042_d_n9;

        let (assign31950_e35055, assign31950_e35055_d_n4, assign31950_e35055_d_n6, assign31950_e35055_d_n7, assign31950_e35055_d_n8, assign31950_e35055_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1116 == 0.0)) && (locals.var_guard1117 != 0.0)) {
        let assign31950_e35051: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign31950_e35053: f64 = (assign31950_e35051 / locals.var_q_qsq__blk825);
        (assign31950_e35053, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign31950_e35051 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign31950_e35051 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign31950_e35051 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign31950_e35051 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign31950_e35051 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign31950_e35055;
        locals.var_q_temp1__blk814_dn4 = assign31950_e35055_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign31950_e35055_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign31950_e35055_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign31950_e35055_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign31950_e35055_d_n9;

        let (assign31960_e35072, assign31960_e35072_d_n4, assign31960_e35072_d_n6, assign31960_e35072_d_n7, assign31960_e35072_d_n8, assign31960_e35072_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1116 == 0.0)) && (locals.var_guard1117 != 0.0)) {
        let assign31960_e35066: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign31960_e35067: f64 = (locals.var_q_qcoth__blk829 * assign31960_e35066);
        let assign31960_e35068: f64 = (locals.var_q_qsq__blk825 + assign31960_e35067);
        let assign31960_e35070: f64 = (assign31960_e35068 * locals.var_q_temp1__blk814);
        (assign31960_e35070, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign31960_e35066) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign31960_e35068 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign31960_e35066) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign31960_e35068 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign31960_e35066) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign31960_e35068 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign31960_e35066) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign31960_e35068 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign31960_e35066) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign31960_e35068 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign31960_e35072;
        locals.var_q_d1_qcoth__blk830_dn4 = assign31960_e35072_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign31960_e35072_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign31960_e35072_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign31960_e35072_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign31960_e35072_d_n9;

        let (assign31970_e35097, assign31970_e35097_d_n4, assign31970_e35097_d_n6, assign31970_e35097_d_n7, assign31970_e35097_d_n8, assign31970_e35097_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1116 == 0.0)) && (locals.var_guard1117 != 0.0)) {
        let assign31970_e35082: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign31970_e35085: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign31970_e35086: f64 = (assign31970_e35082 * assign31970_e35085);
        let assign31970_e35087: f64 = (locals.var_q_d1_qsq__blk826 - assign31970_e35086);
        let assign31970_e35089: f64 = (assign31970_e35087 * locals.var_q_temp1__blk814);
        let assign31970_e35092: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign31970_e35094: f64 = (assign31970_e35092 / locals.var_q_d1_qsq__blk826);
        let assign31970_e35095: f64 = (assign31970_e35089 + assign31970_e35094);
        (assign31970_e35095, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign31970_e35085) + (assign31970_e35082 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign31970_e35087 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign31970_e35092 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign31970_e35085) + (assign31970_e35082 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign31970_e35087 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign31970_e35092 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign31970_e35085) + (assign31970_e35082 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign31970_e35087 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign31970_e35092 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign31970_e35085) + (assign31970_e35082 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign31970_e35087 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign31970_e35092 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign31970_e35085) + (assign31970_e35082 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign31970_e35087 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign31970_e35092 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign31970_e35097;
        locals.var_q_d2_qcoth__blk832_dn4 = assign31970_e35097_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign31970_e35097_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign31970_e35097_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign31970_e35097_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign31970_e35097_d_n9;

        let (assign31980_e35110, assign31980_e35110_d_n4, assign31980_e35110_d_n6, assign31980_e35110_d_n7, assign31980_e35110_d_n8, assign31980_e35110_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1116 == 0.0)) && (locals.var_guard1117 != 0.0)) {
        let assign31980_e35107: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign31980_e35108: f64 = (1.0 - assign31980_e35107);
        (assign31980_e35108, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign31980_e35110;
        locals.var_q_temp2__blk815_dn4 = assign31980_e35110_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign31980_e35110_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign31980_e35110_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign31980_e35110_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign31980_e35110_d_n9;

        let (assign31990_e35123, assign31990_e35123_d_n4, assign31990_e35123_d_n6, assign31990_e35123_d_n7, assign31990_e35123_d_n8, assign31990_e35123_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1116 == 0.0)) && (locals.var_guard1117 != 0.0)) {
        let assign31990_e35119: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign31990_e35121: f64 = (assign31990_e35119 * locals.var_q_temp2__blk815);
        (assign31990_e35121, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign31990_e35119 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign31990_e35119 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign31990_e35119 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign31990_e35119 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign31990_e35119 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign31990_e35123;
        locals.var_q_d1_ln__blk835_dn4 = assign31990_e35123_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign31990_e35123_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign31990_e35123_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign31990_e35123_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign31990_e35123_d_n9;

        let (assign32000_e35144, assign32000_e35144_d_n4, assign32000_e35144_d_n6, assign32000_e35144_d_n7, assign32000_e35144_d_n8, assign32000_e35144_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1116 == 0.0)) && (locals.var_guard1117 != 0.0)) {
        let assign32000_e35132: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign32000_e35137: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign32000_e35138: f64 = (locals.var_q_d1_ln__blk835 + assign32000_e35137);
        let assign32000_e35139: f64 = (locals.var_q_d1_qsq__blk826 * assign32000_e35138);
        let assign32000_e35140: f64 = (assign32000_e35132 - assign32000_e35139);
        let assign32000_e35142: f64 = (assign32000_e35140 / locals.var_q_qsq__blk825);
        (assign32000_e35142, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign32000_e35138) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign32000_e35140 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign32000_e35138) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign32000_e35140 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign32000_e35138) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign32000_e35140 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign32000_e35138) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign32000_e35140 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign32000_e35138) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign32000_e35140 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign32000_e35144;
        locals.var_q_d2_ln__blk836_dn4 = assign32000_e35144_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign32000_e35144_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign32000_e35144_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign32000_e35144_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign32000_e35144_d_n9;

        let (assign32010_e35172, assign32010_e35172_d_n4, assign32010_e35172_d_n6, assign32010_e35172_d_n7, assign32010_e35172_d_n8, assign32010_e35172_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1116 == 0.0)) && (locals.var_guard1117 == 0.0)) {
        let assign32010_e35156: f64 = (locals.var_q_qsq__blk825 * 0.0166666666667);
        let assign32010_e35160: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign32010_e35164: f64 = (locals.var_q_qsq__blk825 * 0.025);
        let assign32010_e35165: f64 = (1.0 - assign32010_e35164);
        let assign32010_e35166: f64 = (assign32010_e35160 * assign32010_e35165);
        let assign32010_e35167: f64 = (1.0 - assign32010_e35166);
        let assign32010_e35168: f64 = (assign32010_e35156 * assign32010_e35167);
        let assign32010_e35169: f64 = (1.0 - assign32010_e35168);
        let assign32010_e35170: f64 = (0.1666666666667 * assign32010_e35169);
        (assign32010_e35170, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0166666666667) * assign32010_e35167) + (assign32010_e35156 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign32010_e35165) + (assign32010_e35160 * (-(locals.var_q_qsq__blk825_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0166666666667) * assign32010_e35167) + (assign32010_e35156 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign32010_e35165) + (assign32010_e35160 * (-(locals.var_q_qsq__blk825_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0166666666667) * assign32010_e35167) + (assign32010_e35156 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign32010_e35165) + (assign32010_e35160 * (-(locals.var_q_qsq__blk825_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0166666666667) * assign32010_e35167) + (assign32010_e35156 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign32010_e35165) + (assign32010_e35160 * (-(locals.var_q_qsq__blk825_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0166666666667) * assign32010_e35167) + (assign32010_e35156 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign32010_e35165) + (assign32010_e35160 * (-(locals.var_q_qsq__blk825_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign32010_e35172;
        locals.var_q_temp3__blk816_dn4 = assign32010_e35172_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign32010_e35172_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign32010_e35172_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign32010_e35172_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign32010_e35172_d_n9;

        let (assign32020_e35186, assign32020_e35186_d_n4, assign32020_e35186_d_n6, assign32020_e35186_d_n7, assign32020_e35186_d_n8, assign32020_e35186_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1116 == 0.0)) && (locals.var_guard1117 == 0.0)) {
        let assign32020_e35183: f64 = (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816);
        let assign32020_e35184: f64 = (2.0 + assign32020_e35183);
        (assign32020_e35184, ((locals.var_q_qsq__blk825_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn4)), ((locals.var_q_qsq__blk825_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn6)), ((locals.var_q_qsq__blk825_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn7)), ((locals.var_q_qsq__blk825_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn8)), ((locals.var_q_qsq__blk825_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign32020_e35186;
        locals.var_q_qcoth__blk829_dn4 = assign32020_e35186_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign32020_e35186_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign32020_e35186_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign32020_e35186_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign32020_e35186_d_n9;

        let (assign32030_e35214, assign32030_e35214_d_n4, assign32030_e35214_d_n6, assign32030_e35214_d_n7, assign32030_e35214_d_n8, assign32030_e35214_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1116 == 0.0)) && (locals.var_guard1117 == 0.0)) {
        let assign32030_e35198: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign32030_e35202: f64 = (locals.var_q_qsq__blk825 * 0.0357142857143);
        let assign32030_e35206: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign32030_e35207: f64 = (1.0 - assign32030_e35206);
        let assign32030_e35208: f64 = (assign32030_e35202 * assign32030_e35207);
        let assign32030_e35209: f64 = (1.0 - assign32030_e35208);
        let assign32030_e35210: f64 = (assign32030_e35198 * assign32030_e35209);
        let assign32030_e35211: f64 = (1.0 - assign32030_e35210);
        let assign32030_e35212: f64 = (0.1666666666667 * assign32030_e35211);
        (assign32030_e35212, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0333333333333) * assign32030_e35209) + (assign32030_e35198 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0357142857143) * assign32030_e35207) + (assign32030_e35202 * (-(locals.var_q_qsq__blk825_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0333333333333) * assign32030_e35209) + (assign32030_e35198 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0357142857143) * assign32030_e35207) + (assign32030_e35202 * (-(locals.var_q_qsq__blk825_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0333333333333) * assign32030_e35209) + (assign32030_e35198 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0357142857143) * assign32030_e35207) + (assign32030_e35202 * (-(locals.var_q_qsq__blk825_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0333333333333) * assign32030_e35209) + (assign32030_e35198 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0357142857143) * assign32030_e35207) + (assign32030_e35202 * (-(locals.var_q_qsq__blk825_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0333333333333) * assign32030_e35209) + (assign32030_e35198 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0357142857143) * assign32030_e35207) + (assign32030_e35202 * (-(locals.var_q_qsq__blk825_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign32030_e35214;
        locals.var_q_temp1__blk814_dn4 = assign32030_e35214_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign32030_e35214_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign32030_e35214_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign32030_e35214_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign32030_e35214_d_n9;

        let (assign32040_e35226, assign32040_e35226_d_n4, assign32040_e35226_d_n6, assign32040_e35226_d_n7, assign32040_e35226_d_n8, assign32040_e35226_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1116 == 0.0)) && (locals.var_guard1117 == 0.0)) {
        let assign32040_e35224: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814);
        (assign32040_e35224, ((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign32040_e35226;
        locals.var_q_d1_qcoth__blk830_dn4 = assign32040_e35226_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign32040_e35226_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign32040_e35226_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign32040_e35226_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign32040_e35226_d_n9;

        let (assign32050_e35254, assign32050_e35254_d_n4, assign32050_e35254_d_n6, assign32050_e35254_d_n7, assign32050_e35254_d_n8, assign32050_e35254_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1116 == 0.0)) && (locals.var_guard1117 == 0.0)) {
        let assign32050_e35238: f64 = (locals.var_q_qsq__blk825 * 0.0714285714286);
        let assign32050_e35242: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign32050_e35246: f64 = (0.0420875420875421 * locals.var_q_qsq__blk825);
        let assign32050_e35247: f64 = (1.0 - assign32050_e35246);
        let assign32050_e35248: f64 = (assign32050_e35242 * assign32050_e35247);
        let assign32050_e35249: f64 = (1.0 - assign32050_e35248);
        let assign32050_e35250: f64 = (assign32050_e35238 * assign32050_e35249);
        let assign32050_e35251: f64 = (1.0 - assign32050_e35250);
        let assign32050_e35252: f64 = (0.0055555555556 * assign32050_e35251);
        (assign32050_e35252, (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0714285714286) * assign32050_e35249) + (assign32050_e35238 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign32050_e35247) + (assign32050_e35242 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0714285714286) * assign32050_e35249) + (assign32050_e35238 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign32050_e35247) + (assign32050_e35242 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0714285714286) * assign32050_e35249) + (assign32050_e35238 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign32050_e35247) + (assign32050_e35242 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0714285714286) * assign32050_e35249) + (assign32050_e35238 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign32050_e35247) + (assign32050_e35242 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0714285714286) * assign32050_e35249) + (assign32050_e35238 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign32050_e35247) + (assign32050_e35242 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn9))))))))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign32050_e35254;
        locals.var_q_temp2__blk815_dn4 = assign32050_e35254_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign32050_e35254_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign32050_e35254_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign32050_e35254_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign32050_e35254_d_n9;

        let (assign32060_e35272, assign32060_e35272_d_n4, assign32060_e35272_d_n6, assign32060_e35272_d_n7, assign32060_e35272_d_n8, assign32060_e35272_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1116 == 0.0)) && (locals.var_guard1117 == 0.0)) {
        let assign32060_e35264: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814);
        let assign32060_e35267: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826);
        let assign32060_e35269: f64 = (assign32060_e35267 * locals.var_q_temp2__blk815);
        let assign32060_e35270: f64 = (assign32060_e35264 - assign32060_e35269);
        (assign32060_e35270, (((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn4)) - ((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn4)) * locals.var_q_temp2__blk815) + (assign32060_e35267 * locals.var_q_temp2__blk815_dn4))), (((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn6)) - ((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn6)) * locals.var_q_temp2__blk815) + (assign32060_e35267 * locals.var_q_temp2__blk815_dn6))), (((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn7)) - ((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn7)) * locals.var_q_temp2__blk815) + (assign32060_e35267 * locals.var_q_temp2__blk815_dn7))), (((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn8)) - ((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn8)) * locals.var_q_temp2__blk815) + (assign32060_e35267 * locals.var_q_temp2__blk815_dn8))), (((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn9)) - ((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn9)) * locals.var_q_temp2__blk815) + (assign32060_e35267 * locals.var_q_temp2__blk815_dn9))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign32060_e35272;
        locals.var_q_d2_qcoth__blk832_dn4 = assign32060_e35272_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign32060_e35272_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign32060_e35272_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign32060_e35272_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign32060_e35272_d_n9;

        let (assign32070_e35287, assign32070_e35287_d_n4, assign32070_e35287_d_n6, assign32070_e35287_d_n7, assign32070_e35287_d_n8, assign32070_e35287_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1116 == 0.0)) && (locals.var_guard1117 == 0.0)) {
        let assign32070_e35281: f64 = (-0.5);
        let assign32070_e35283: f64 = (assign32070_e35281 * locals.var_q_d1_qsq__blk826);
        let assign32070_e35285: f64 = (assign32070_e35283 * locals.var_q_temp3__blk816);
        (assign32070_e35285, (((assign32070_e35281 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_temp3__blk816) + (assign32070_e35283 * locals.var_q_temp3__blk816_dn4)), (((assign32070_e35281 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_temp3__blk816) + (assign32070_e35283 * locals.var_q_temp3__blk816_dn6)), (((assign32070_e35281 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_temp3__blk816) + (assign32070_e35283 * locals.var_q_temp3__blk816_dn7)), (((assign32070_e35281 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_temp3__blk816) + (assign32070_e35283 * locals.var_q_temp3__blk816_dn8)), (((assign32070_e35281 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_temp3__blk816) + (assign32070_e35283 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign32070_e35287;
        locals.var_q_d1_ln__blk835_dn4 = assign32070_e35287_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign32070_e35287_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign32070_e35287_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign32070_e35287_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign32070_e35287_d_n9;

        let (assign32080_e35322, assign32080_e35322_d_n4, assign32080_e35322_d_n6, assign32080_e35322_d_n7, assign32080_e35322_d_n8, assign32080_e35322_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1116 == 0.0)) && (locals.var_guard1117 == 0.0)) {
        let assign32080_e35296: f64 = (-0.5);
        let assign32080_e35298: f64 = (assign32080_e35296 * locals.var_q_d2_qsq__blk827);
        let assign32080_e35300: f64 = (assign32080_e35298 * locals.var_q_temp3__blk816);
        let assign32080_e35303: f64 = (0.25 * 0.0055555555556);
        let assign32080_e35305: f64 = (assign32080_e35303 * locals.var_q_d1_qsq__blk826);
        let assign32080_e35307: f64 = (assign32080_e35305 * locals.var_q_d1_qsq__blk826);
        let assign32080_e35311: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign32080_e35315: f64 = (0.075 * locals.var_q_qsq__blk825);
        let assign32080_e35316: f64 = (2.0 - assign32080_e35315);
        let assign32080_e35317: f64 = (assign32080_e35311 * assign32080_e35316);
        let assign32080_e35318: f64 = (1.0 - assign32080_e35317);
        let assign32080_e35319: f64 = (assign32080_e35307 * assign32080_e35318);
        let assign32080_e35320: f64 = (assign32080_e35300 + assign32080_e35319);
        (assign32080_e35320, ((((assign32080_e35296 * locals.var_q_d2_qsq__blk827_dn4) * locals.var_q_temp3__blk816) + (assign32080_e35298 * locals.var_q_temp3__blk816_dn4)) + (((((assign32080_e35303 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_d1_qsq__blk826) + (assign32080_e35305 * locals.var_q_d1_qsq__blk826_dn4)) * assign32080_e35318) + (assign32080_e35307 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign32080_e35316) + (assign32080_e35311 * (-(0.075 * locals.var_q_qsq__blk825_dn4)))))))), ((((assign32080_e35296 * locals.var_q_d2_qsq__blk827_dn6) * locals.var_q_temp3__blk816) + (assign32080_e35298 * locals.var_q_temp3__blk816_dn6)) + (((((assign32080_e35303 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_d1_qsq__blk826) + (assign32080_e35305 * locals.var_q_d1_qsq__blk826_dn6)) * assign32080_e35318) + (assign32080_e35307 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign32080_e35316) + (assign32080_e35311 * (-(0.075 * locals.var_q_qsq__blk825_dn6)))))))), ((((assign32080_e35296 * locals.var_q_d2_qsq__blk827_dn7) * locals.var_q_temp3__blk816) + (assign32080_e35298 * locals.var_q_temp3__blk816_dn7)) + (((((assign32080_e35303 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_d1_qsq__blk826) + (assign32080_e35305 * locals.var_q_d1_qsq__blk826_dn7)) * assign32080_e35318) + (assign32080_e35307 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign32080_e35316) + (assign32080_e35311 * (-(0.075 * locals.var_q_qsq__blk825_dn7)))))))), ((((assign32080_e35296 * locals.var_q_d2_qsq__blk827_dn8) * locals.var_q_temp3__blk816) + (assign32080_e35298 * locals.var_q_temp3__blk816_dn8)) + (((((assign32080_e35303 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_d1_qsq__blk826) + (assign32080_e35305 * locals.var_q_d1_qsq__blk826_dn8)) * assign32080_e35318) + (assign32080_e35307 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign32080_e35316) + (assign32080_e35311 * (-(0.075 * locals.var_q_qsq__blk825_dn8)))))))), ((((assign32080_e35296 * locals.var_q_d2_qsq__blk827_dn9) * locals.var_q_temp3__blk816) + (assign32080_e35298 * locals.var_q_temp3__blk816_dn9)) + (((((assign32080_e35303 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_d1_qsq__blk826) + (assign32080_e35305 * locals.var_q_d1_qsq__blk826_dn9)) * assign32080_e35318) + (assign32080_e35307 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign32080_e35316) + (assign32080_e35311 * (-(0.075 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign32080_e35322;
        locals.var_q_d2_ln__blk836_dn4 = assign32080_e35322_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign32080_e35322_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign32080_e35322_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign32080_e35322_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign32080_e35322_d_n9;

        let assign32090_e35325: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1118 = assign32090_e35325;

    }

    pub(super) fn stamp_transient_block_86(
        locals: &mut StampLocals,
    ) {
        let (assign32100_e35341, assign32100_e35341_d_n4, assign32100_e35341_d_n6, assign32100_e35341_d_n7, assign32100_e35341_d_n8, assign32100_e35341_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1118 != 0.0)) {
        let assign32100_e35331: f64 = (4.0 * locals.var_q_qsq__blk825);
        let assign32100_e35336: f64 = (2.0 - locals.var_q_invexpq__blk831);
        let assign32100_e35337: f64 = (locals.var_q_invexpq__blk831 * assign32100_e35336);
        let assign32100_e35338: f64 = (1.0 - assign32100_e35337);
        let assign32100_e35339: f64 = (assign32100_e35331 / assign32100_e35338);
        (assign32100_e35339, ((((4.0 * locals.var_q_qsq__blk825_dn4) * assign32100_e35338) - (assign32100_e35331 * (-((locals.var_q_invexpq__blk831_dn4 * assign32100_e35336) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign32100_e35338 * assign32100_e35338)), ((((4.0 * locals.var_q_qsq__blk825_dn6) * assign32100_e35338) - (assign32100_e35331 * (-((locals.var_q_invexpq__blk831_dn6 * assign32100_e35336) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign32100_e35338 * assign32100_e35338)), ((((4.0 * locals.var_q_qsq__blk825_dn7) * assign32100_e35338) - (assign32100_e35331 * (-((locals.var_q_invexpq__blk831_dn7 * assign32100_e35336) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign32100_e35338 * assign32100_e35338)), ((((4.0 * locals.var_q_qsq__blk825_dn8) * assign32100_e35338) - (assign32100_e35331 * (-((locals.var_q_invexpq__blk831_dn8 * assign32100_e35336) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign32100_e35338 * assign32100_e35338)), ((((4.0 * locals.var_q_qsq__blk825_dn9) * assign32100_e35338) - (assign32100_e35331 * (-((locals.var_q_invexpq__blk831_dn9 * assign32100_e35336) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign32100_e35338 * assign32100_e35338)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign32100_e35341;
        locals.var_q_temp2__blk815_dn4 = assign32100_e35341_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign32100_e35341_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign32100_e35341_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign32100_e35341_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign32100_e35341_d_n9;

        let (assign32110_e35349, assign32110_e35349_d_n4, assign32110_e35349_d_n6, assign32110_e35349_d_n7, assign32110_e35349_d_n8, assign32110_e35349_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1118 != 0.0)) {
        let assign32110_e35347: f64 = (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831);
        (assign32110_e35347, ((locals.var_q_temp2__blk815_dn4 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn4)), ((locals.var_q_temp2__blk815_dn6 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn6)), ((locals.var_q_temp2__blk815_dn7 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn7)), ((locals.var_q_temp2__blk815_dn8 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn8)), ((locals.var_q_temp2__blk815_dn9 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn9)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign32110_e35349;
        locals.var_q_sh_term__blk833_dn4 = assign32110_e35349_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign32110_e35349_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign32110_e35349_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign32110_e35349_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign32110_e35349_d_n9;

        let (assign32120_e35358, assign32120_e35358_d_n4, assign32120_e35358_d_n6, assign32120_e35358_d_n7, assign32120_e35358_d_n8, assign32120_e35358_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1118 != 0.0)) {
        let assign32120_e35354: f64 = (locals.var_q_temp2__blk815).ln();
        let assign32120_e35356: f64 = (assign32120_e35354 - locals.var_q_rac_qsq__blk828);
        (assign32120_e35356, ((locals.var_q_temp2__blk815_dn4 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn4), ((locals.var_q_temp2__blk815_dn6 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn6), ((locals.var_q_temp2__blk815_dn7 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn7), ((locals.var_q_temp2__blk815_dn8 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn8), ((locals.var_q_temp2__blk815_dn9 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn9),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign32120_e35358;
        locals.var_q_ln_term__blk834_dn4 = assign32120_e35358_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign32120_e35358_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign32120_e35358_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign32120_e35358_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign32120_e35358_d_n9;

        let assign32130_e35361: f64 = (-0.005);
        let assign32130_e35362: f64 = if locals.var_q_qsq__blk825 < assign32130_e35361 { 1.0 } else { 0.0 };
        locals.var_guard1119 = assign32130_e35362;

        let (assign32140_e35374, assign32140_e35374_d_n4, assign32140_e35374_d_n6, assign32140_e35374_d_n7, assign32140_e35374_d_n8, assign32140_e35374_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1118 == 0.0)) && (locals.var_guard1119 != 0.0)) {
        let assign32140_e35371: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign32140_e35372: f64 = (assign32140_e35371).sin();
        (assign32140_e35372, ((assign32140_e35371).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign32140_e35371).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign32140_e35371).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign32140_e35371).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign32140_e35371).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign32140_e35374;
        locals.var_q_temp2__blk815_dn4 = assign32140_e35374_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign32140_e35374_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign32140_e35374_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign32140_e35374_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign32140_e35374_d_n9;

        let (assign32150_e35388, assign32150_e35388_d_n4, assign32150_e35388_d_n6, assign32150_e35388_d_n7, assign32150_e35388_d_n8, assign32150_e35388_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1118 == 0.0)) && (locals.var_guard1119 != 0.0)) {
        let assign32150_e35382: f64 = (-locals.var_q_qsq__blk825);
        let assign32150_e35385: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
        let assign32150_e35386: f64 = (assign32150_e35382 / assign32150_e35385);
        (assign32150_e35386, ((((-locals.var_q_qsq__blk825_dn4) * assign32150_e35385) - (assign32150_e35382 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign32150_e35385 * assign32150_e35385)), ((((-locals.var_q_qsq__blk825_dn6) * assign32150_e35385) - (assign32150_e35382 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign32150_e35385 * assign32150_e35385)), ((((-locals.var_q_qsq__blk825_dn7) * assign32150_e35385) - (assign32150_e35382 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign32150_e35385 * assign32150_e35385)), ((((-locals.var_q_qsq__blk825_dn8) * assign32150_e35385) - (assign32150_e35382 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign32150_e35385 * assign32150_e35385)), ((((-locals.var_q_qsq__blk825_dn9) * assign32150_e35385) - (assign32150_e35382 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign32150_e35385 * assign32150_e35385)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign32150_e35388;
        locals.var_q_sh_term__blk833_dn4 = assign32150_e35388_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign32150_e35388_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign32150_e35388_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign32150_e35388_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign32150_e35388_d_n9;

        let (assign32160_e35398, assign32160_e35398_d_n4, assign32160_e35398_d_n6, assign32160_e35398_d_n7, assign32160_e35398_d_n8, assign32160_e35398_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1118 == 0.0)) && (locals.var_guard1119 != 0.0)) {
        let assign32160_e35396: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign32160_e35396, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign32160_e35398;
        locals.var_q_ln_term__blk834_dn4 = assign32160_e35398_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign32160_e35398_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign32160_e35398_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign32160_e35398_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign32160_e35398_d_n9;

        let (assign32170_e35424, assign32170_e35424_d_n4, assign32170_e35424_d_n6, assign32170_e35424_d_n7, assign32170_e35424_d_n8, assign32170_e35424_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1118 == 0.0)) && (locals.var_guard1119 == 0.0)) {
        let assign32170_e35409: f64 = (locals.var_q_qsq__blk825 * 0.3333333333333);
        let assign32170_e35413: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign32170_e35417: f64 = (0.0396825396825397 * locals.var_q_qsq__blk825);
        let assign32170_e35418: f64 = (1.0 - assign32170_e35417);
        let assign32170_e35419: f64 = (assign32170_e35413 * assign32170_e35418);
        let assign32170_e35420: f64 = (1.0 - assign32170_e35419);
        let assign32170_e35421: f64 = (assign32170_e35409 * assign32170_e35420);
        let assign32170_e35422: f64 = (4.0 - assign32170_e35421);
        (assign32170_e35422, (-(((locals.var_q_qsq__blk825_dn4 * 0.3333333333333) * assign32170_e35420) + (assign32170_e35409 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign32170_e35418) + (assign32170_e35413 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn4)))))))), (-(((locals.var_q_qsq__blk825_dn6 * 0.3333333333333) * assign32170_e35420) + (assign32170_e35409 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign32170_e35418) + (assign32170_e35413 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn6)))))))), (-(((locals.var_q_qsq__blk825_dn7 * 0.3333333333333) * assign32170_e35420) + (assign32170_e35409 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign32170_e35418) + (assign32170_e35413 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn7)))))))), (-(((locals.var_q_qsq__blk825_dn8 * 0.3333333333333) * assign32170_e35420) + (assign32170_e35409 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign32170_e35418) + (assign32170_e35413 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn8)))))))), (-(((locals.var_q_qsq__blk825_dn9 * 0.3333333333333) * assign32170_e35420) + (assign32170_e35409 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign32170_e35418) + (assign32170_e35413 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign32170_e35424;
        locals.var_q_sh_term__blk833_dn4 = assign32170_e35424_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign32170_e35424_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign32170_e35424_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign32170_e35424_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign32170_e35424_d_n9;

        let (assign32180_e35435, assign32180_e35435_d_n4, assign32180_e35435_d_n6, assign32180_e35435_d_n7, assign32180_e35435_d_n8, assign32180_e35435_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1118 == 0.0)) && (locals.var_guard1119 == 0.0)) {
        let assign32180_e35433: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign32180_e35433, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign32180_e35435;
        locals.var_q_ln_term__blk834_dn4 = assign32180_e35435_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign32180_e35435_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign32180_e35435_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign32180_e35435_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign32180_e35435_d_n9;

        let assign32190_e35438: f64 = (1.01 * locals.var_q_k1q1__blk823);
        let assign32190_e35440: f64 = (assign32190_e35438 + locals.var_q_qcoth__blk829);
        let assign32190_e35442: f64 = if assign32190_e35440 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1120 = assign32190_e35442;

        let (assign32200_e35450, assign32200_e35450_d_n4, assign32200_e35450_d_n6, assign32200_e35450_d_n7, assign32200_e35450_d_n8, assign32200_e35450_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1120 != 0.0)) {
        let assign32200_e35448: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_qcoth__blk829);
        (assign32200_e35448, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_qcoth__blk829_dn9),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign32200_e35450;
        locals.var_q_expnum__blk837_dn4 = assign32200_e35450_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign32200_e35450_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign32200_e35450_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign32200_e35450_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign32200_e35450_d_n9;

        let (assign32210_e35458, assign32210_e35458_d_n4, assign32210_e35458_d_n6, assign32210_e35458_d_n7, assign32210_e35458_d_n8, assign32210_e35458_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1120 != 0.0)) {
        let assign32210_e35456: f64 = (locals.var_k1__blk932 + locals.var_q_d1_qcoth__blk830);
        (assign32210_e35456, (locals.var_k1__blk932_dn4 + locals.var_q_d1_qcoth__blk830_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_d1_qcoth__blk830_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_d1_qcoth__blk830_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_d1_qcoth__blk830_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_d1_qcoth__blk830_dn9),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign32210_e35458;
        locals.var_q_d1_expnum__blk838_dn4 = assign32210_e35458_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign32210_e35458_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign32210_e35458_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign32210_e35458_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign32210_e35458_d_n9;

        let (assign32220_e35464, assign32220_e35464_d_n4, assign32220_e35464_d_n6, assign32220_e35464_d_n7, assign32220_e35464_d_n8, assign32220_e35464_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1120 != 0.0)) {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign32220_e35464;
        locals.var_q_d2_expnum__blk839_dn4 = assign32220_e35464_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign32220_e35464_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign32220_e35464_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign32220_e35464_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign32220_e35464_d_n9;

        let (assign32230_e35475, assign32230_e35475_d_n4, assign32230_e35475_d_n6, assign32230_e35475_d_n7, assign32230_e35475_d_n8, assign32230_e35475_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1120 == 0.0)) {
        let assign32230_e35472: f64 = (locals.var_q_k1q1__blk823 - locals.var_q_qcoth__blk829);
        let assign32230_e35473: f64 = (1.0 / assign32230_e35472);
        (assign32230_e35473, (-((locals.var_q_k1q1__blk823_dn4 - locals.var_q_qcoth__blk829_dn4) / (assign32230_e35472 * assign32230_e35472))), (-((locals.var_q_k1q1__blk823_dn6 - locals.var_q_qcoth__blk829_dn6) / (assign32230_e35472 * assign32230_e35472))), (-((locals.var_q_k1q1__blk823_dn7 - locals.var_q_qcoth__blk829_dn7) / (assign32230_e35472 * assign32230_e35472))), (-((locals.var_q_k1q1__blk823_dn8 - locals.var_q_qcoth__blk829_dn8) / (assign32230_e35472 * assign32230_e35472))), (-((locals.var_q_k1q1__blk823_dn9 - locals.var_q_qcoth__blk829_dn9) / (assign32230_e35472 * assign32230_e35472))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign32230_e35475;
        locals.var_q_temp2__blk815_dn4 = assign32230_e35475_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign32230_e35475_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign32230_e35475_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign32230_e35475_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign32230_e35475_d_n9;

        let (assign32240_e35484, assign32240_e35484_d_n4, assign32240_e35484_d_n6, assign32240_e35484_d_n7, assign32240_e35484_d_n8, assign32240_e35484_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1120 == 0.0)) {
        let assign32240_e35482: f64 = (locals.var_q_d1_qcoth__blk830 - locals.var_k1__blk932);
        (assign32240_e35482, (locals.var_q_d1_qcoth__blk830_dn4 - locals.var_k1__blk932_dn4), (locals.var_q_d1_qcoth__blk830_dn6 - locals.var_k1__blk932_dn6), (locals.var_q_d1_qcoth__blk830_dn7 - locals.var_k1__blk932_dn7), (locals.var_q_d1_qcoth__blk830_dn8 - locals.var_k1__blk932_dn8), (locals.var_q_d1_qcoth__blk830_dn9 - locals.var_k1__blk932_dn9),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign32240_e35484;
        locals.var_q_temp3__blk816_dn4 = assign32240_e35484_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign32240_e35484_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign32240_e35484_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign32240_e35484_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign32240_e35484_d_n9;

        let (assign32250_e35495, assign32250_e35495_d_n4, assign32250_e35495_d_n6, assign32250_e35495_d_n7, assign32250_e35495_d_n8, assign32250_e35495_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1120 == 0.0)) {
        let assign32250_e35491: f64 = (locals.var_q_aexp__blk824 - locals.var_q_sh_term__blk833);
        let assign32250_e35493: f64 = (assign32250_e35491 * locals.var_q_temp2__blk815);
        (assign32250_e35493, (((locals.var_q_aexp__blk824_dn4 - locals.var_q_sh_term__blk833_dn4) * locals.var_q_temp2__blk815) + (assign32250_e35491 * locals.var_q_temp2__blk815_dn4)), (((locals.var_q_aexp__blk824_dn6 - locals.var_q_sh_term__blk833_dn6) * locals.var_q_temp2__blk815) + (assign32250_e35491 * locals.var_q_temp2__blk815_dn6)), (((locals.var_q_aexp__blk824_dn7 - locals.var_q_sh_term__blk833_dn7) * locals.var_q_temp2__blk815) + (assign32250_e35491 * locals.var_q_temp2__blk815_dn7)), (((locals.var_q_aexp__blk824_dn8 - locals.var_q_sh_term__blk833_dn8) * locals.var_q_temp2__blk815) + (assign32250_e35491 * locals.var_q_temp2__blk815_dn8)), (((locals.var_q_aexp__blk824_dn9 - locals.var_q_sh_term__blk833_dn9) * locals.var_q_temp2__blk815) + (assign32250_e35491 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign32250_e35495;
        locals.var_q_expnum__blk837_dn4 = assign32250_e35495_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign32250_e35495_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign32250_e35495_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign32250_e35495_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign32250_e35495_d_n9;

        let (assign32260_e35512, assign32260_e35512_d_n4, assign32260_e35512_d_n6, assign32260_e35512_d_n7, assign32260_e35512_d_n8, assign32260_e35512_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1120 == 0.0)) {
        let assign32260_e35502: f64 = (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837);
        let assign32260_e35504: f64 = (assign32260_e35502 - locals.var_q_aexp__blk824);
        let assign32260_e35507: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833);
        let assign32260_e35508: f64 = (assign32260_e35504 - assign32260_e35507);
        let assign32260_e35510: f64 = (assign32260_e35508 * locals.var_q_temp2__blk815);
        (assign32260_e35510, ((((((locals.var_q_temp3__blk816_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4) - ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign32260_e35508 * locals.var_q_temp2__blk815_dn4)), ((((((locals.var_q_temp3__blk816_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6) - ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign32260_e35508 * locals.var_q_temp2__blk815_dn6)), ((((((locals.var_q_temp3__blk816_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7) - ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign32260_e35508 * locals.var_q_temp2__blk815_dn7)), ((((((locals.var_q_temp3__blk816_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8) - ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign32260_e35508 * locals.var_q_temp2__blk815_dn8)), ((((((locals.var_q_temp3__blk816_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9) - ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign32260_e35508 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign32260_e35512;
        locals.var_q_d1_expnum__blk838_dn4 = assign32260_e35512_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign32260_e35512_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign32260_e35512_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign32260_e35512_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign32260_e35512_d_n9;

        let (assign32270_e35539, assign32270_e35539_d_n4, assign32270_e35539_d_n6, assign32270_e35539_d_n7, assign32270_e35539_d_n8, assign32270_e35539_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1120 == 0.0)) {
        let assign32270_e35519: f64 = (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837);
        let assign32270_e35522: f64 = (2.0 * locals.var_q_temp3__blk816);
        let assign32270_e35524: f64 = (assign32270_e35522 * locals.var_q_d1_expnum__blk838);
        let assign32270_e35525: f64 = (assign32270_e35519 + assign32270_e35524);
        let assign32270_e35527: f64 = (assign32270_e35525 + locals.var_q_aexp__blk824);
        let assign32270_e35531: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835);
        let assign32270_e35532: f64 = (locals.var_q_d2_ln__blk836 + assign32270_e35531);
        let assign32270_e35534: f64 = (assign32270_e35532 * locals.var_q_sh_term__blk833);
        let assign32270_e35535: f64 = (assign32270_e35527 - assign32270_e35534);
        let assign32270_e35537: f64 = (assign32270_e35535 * locals.var_q_temp2__blk815);
        (assign32270_e35537, (((((((locals.var_q_d2_qcoth__blk832_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_temp3__blk816_dn4) * locals.var_q_d1_expnum__blk838) + (assign32270_e35522 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4) - (((locals.var_q_d2_ln__blk836_dn4 + ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn4))) * locals.var_q_sh_term__blk833) + (assign32270_e35532 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign32270_e35535 * locals.var_q_temp2__blk815_dn4)), (((((((locals.var_q_d2_qcoth__blk832_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_temp3__blk816_dn6) * locals.var_q_d1_expnum__blk838) + (assign32270_e35522 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6) - (((locals.var_q_d2_ln__blk836_dn6 + ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn6))) * locals.var_q_sh_term__blk833) + (assign32270_e35532 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign32270_e35535 * locals.var_q_temp2__blk815_dn6)), (((((((locals.var_q_d2_qcoth__blk832_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_temp3__blk816_dn7) * locals.var_q_d1_expnum__blk838) + (assign32270_e35522 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7) - (((locals.var_q_d2_ln__blk836_dn7 + ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn7))) * locals.var_q_sh_term__blk833) + (assign32270_e35532 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign32270_e35535 * locals.var_q_temp2__blk815_dn7)), (((((((locals.var_q_d2_qcoth__blk832_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_temp3__blk816_dn8) * locals.var_q_d1_expnum__blk838) + (assign32270_e35522 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8) - (((locals.var_q_d2_ln__blk836_dn8 + ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn8))) * locals.var_q_sh_term__blk833) + (assign32270_e35532 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign32270_e35535 * locals.var_q_temp2__blk815_dn8)), (((((((locals.var_q_d2_qcoth__blk832_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_temp3__blk816_dn9) * locals.var_q_d1_expnum__blk838) + (assign32270_e35522 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9) - (((locals.var_q_d2_ln__blk836_dn9 + ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn9))) * locals.var_q_sh_term__blk833) + (assign32270_e35532 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign32270_e35535 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign32270_e35539;
        locals.var_q_d2_expnum__blk839_dn4 = assign32270_e35539_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign32270_e35539_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign32270_e35539_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign32270_e35539_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign32270_e35539_d_n9;

        let assign32280_e35542: f64 = if locals.var_q_expnum__blk837 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1121 = assign32280_e35542;

        let (assign32290_e35549, assign32290_e35549_d_n4, assign32290_e35549_d_n6, assign32290_e35549_d_n7, assign32290_e35549_d_n8, assign32290_e35549_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1121 != 0.0)) {
        let assign32290_e35547: f64 = (locals.var_q_expnum__blk837).ln();
        (assign32290_e35547, (locals.var_q_expnum__blk837_dn4 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn6 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn7 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn8 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn9 / locals.var_q_expnum__blk837),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign32290_e35549;
        locals.var_q_lnexpnum__blk840_dn4 = assign32290_e35549_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign32290_e35549_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign32290_e35549_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign32290_e35549_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign32290_e35549_d_n9;

        let (assign32300_e35557, assign32300_e35557_d_n4, assign32300_e35557_d_n6, assign32300_e35557_d_n7, assign32300_e35557_d_n8, assign32300_e35557_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1121 != 0.0)) {
        let assign32300_e35555: f64 = (1.0 / locals.var_q_expnum__blk837);
        (assign32300_e35555, (-(locals.var_q_expnum__blk837_dn4 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn6 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn7 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn8 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn9 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign32300_e35557;
        locals.var_q_temp1__blk814_dn4 = assign32300_e35557_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign32300_e35557_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign32300_e35557_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign32300_e35557_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign32300_e35557_d_n9;

        let (assign32310_e35565, assign32310_e35565_d_n4, assign32310_e35565_d_n6, assign32310_e35565_d_n7, assign32310_e35565_d_n8, assign32310_e35565_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1121 != 0.0)) {
        let assign32310_e35563: f64 = (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814);
        (assign32310_e35563, ((locals.var_q_d1_expnum__blk838_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_expnum__blk838_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_expnum__blk838_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_expnum__blk838_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_expnum__blk838_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign32310_e35565;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign32310_e35565_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign32310_e35565_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign32310_e35565_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign32310_e35565_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign32310_e35565_d_n9;

        let (assign32320_e35577, assign32320_e35577_d_n4, assign32320_e35577_d_n6, assign32320_e35577_d_n7, assign32320_e35577_d_n8, assign32320_e35577_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1121 != 0.0)) {
        let assign32320_e35571: f64 = (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814);
        let assign32320_e35574: f64 = (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841);
        let assign32320_e35575: f64 = (assign32320_e35571 - assign32320_e35574);
        (assign32320_e35575, (((locals.var_q_d2_expnum__blk839_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn4)) - ((locals.var_q_d1_lnexpnum__blk841_dn4 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn4))), (((locals.var_q_d2_expnum__blk839_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn6)) - ((locals.var_q_d1_lnexpnum__blk841_dn6 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn6))), (((locals.var_q_d2_expnum__blk839_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn7)) - ((locals.var_q_d1_lnexpnum__blk841_dn7 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn7))), (((locals.var_q_d2_expnum__blk839_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn8)) - ((locals.var_q_d1_lnexpnum__blk841_dn8 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn8))), (((locals.var_q_d2_expnum__blk839_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn9)) - ((locals.var_q_d1_lnexpnum__blk841_dn9 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign32320_e35577;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign32320_e35577_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign32320_e35577_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign32320_e35577_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign32320_e35577_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign32320_e35577_d_n9;

        let (assign32330_e35590, assign32330_e35590_d_n4, assign32330_e35590_d_n6, assign32330_e35590_d_n7, assign32330_e35590_d_n8, assign32330_e35590_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1121 == 0.0)) {
        let assign32330_e35584: f64 = (locals.var_q_k1q1__blk823 + 0.6931471805599);
        let assign32330_e35586: f64 = (-locals.var_q_k1q1__blk823);
        let assign32330_e35587: f64 = (assign32330_e35586).ln();
        let assign32330_e35588: f64 = (assign32330_e35584 + assign32330_e35587);
        (assign32330_e35588, (locals.var_q_k1q1__blk823_dn4 + ((-locals.var_q_k1q1__blk823_dn4) / assign32330_e35586)), (locals.var_q_k1q1__blk823_dn6 + ((-locals.var_q_k1q1__blk823_dn6) / assign32330_e35586)), (locals.var_q_k1q1__blk823_dn7 + ((-locals.var_q_k1q1__blk823_dn7) / assign32330_e35586)), (locals.var_q_k1q1__blk823_dn8 + ((-locals.var_q_k1q1__blk823_dn8) / assign32330_e35586)), (locals.var_q_k1q1__blk823_dn9 + ((-locals.var_q_k1q1__blk823_dn9) / assign32330_e35586)),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign32330_e35590;
        locals.var_q_lnexpnum__blk840_dn4 = assign32330_e35590_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign32330_e35590_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign32330_e35590_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign32330_e35590_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign32330_e35590_d_n9;

        let (assign32340_e35599, assign32340_e35599_d_n4, assign32340_e35599_d_n6, assign32340_e35599_d_n7, assign32340_e35599_d_n8, assign32340_e35599_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1121 == 0.0)) {
        let assign32340_e35597: f64 = (1.0 / locals.var_q1s__blk937);
        (assign32340_e35597, (-(locals.var_q1s__blk937_dn4 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))), (-(locals.var_q1s__blk937_dn6 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))), (-(locals.var_q1s__blk937_dn7 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))), (-(locals.var_q1s__blk937_dn8 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))), (-(locals.var_q1s__blk937_dn9 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign32340_e35599;
        locals.var_q_temp1__blk814_dn4 = assign32340_e35599_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign32340_e35599_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign32340_e35599_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign32340_e35599_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign32340_e35599_d_n9;

        let (assign32350_e35608, assign32350_e35608_d_n4, assign32350_e35608_d_n6, assign32350_e35608_d_n7, assign32350_e35608_d_n8, assign32350_e35608_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1121 == 0.0)) {
        let assign32350_e35606: f64 = (locals.var_k1__blk932 + locals.var_q_temp1__blk814);
        (assign32350_e35606, (locals.var_k1__blk932_dn4 + locals.var_q_temp1__blk814_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_temp1__blk814_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_temp1__blk814_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_temp1__blk814_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_temp1__blk814_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign32350_e35608;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign32350_e35608_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign32350_e35608_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign32350_e35608_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign32350_e35608_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign32350_e35608_d_n9;

        let (assign32360_e35618, assign32360_e35618_d_n4, assign32360_e35618_d_n6, assign32360_e35618_d_n7, assign32360_e35618_d_n8, assign32360_e35618_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1121 == 0.0)) {
        let assign32360_e35614: f64 = (-locals.var_q_temp1__blk814);
        let assign32360_e35616: f64 = (assign32360_e35614 * locals.var_q_temp1__blk814);
        (assign32360_e35616, (((-locals.var_q_temp1__blk814_dn4) * locals.var_q_temp1__blk814) + (assign32360_e35614 * locals.var_q_temp1__blk814_dn4)), (((-locals.var_q_temp1__blk814_dn6) * locals.var_q_temp1__blk814) + (assign32360_e35614 * locals.var_q_temp1__blk814_dn6)), (((-locals.var_q_temp1__blk814_dn7) * locals.var_q_temp1__blk814) + (assign32360_e35614 * locals.var_q_temp1__blk814_dn7)), (((-locals.var_q_temp1__blk814_dn8) * locals.var_q_temp1__blk814) + (assign32360_e35614 * locals.var_q_temp1__blk814_dn8)), (((-locals.var_q_temp1__blk814_dn9) * locals.var_q_temp1__blk814) + (assign32360_e35614 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign32360_e35618;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign32360_e35618_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign32360_e35618_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign32360_e35618_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign32360_e35618_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign32360_e35618_d_n9;

        let (assign32370_e35632, assign32370_e35632_d_n4, assign32370_e35632_d_n6, assign32370_e35632_d_n7, assign32370_e35632_d_n8, assign32370_e35632_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign32370_e35622: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
        let assign32370_e35624: f64 = (assign32370_e35622 + locals.var_q1s__blk937);
        let assign32370_e35627: f64 = (2.0 * locals.var_q_lnexpnum__blk840);
        let assign32370_e35628: f64 = (assign32370_e35624 + assign32370_e35627);
        let assign32370_e35630: f64 = (assign32370_e35628 - locals.var_q_ln_term__blk834);
        (assign32370_e35630, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1s__blk937_dn4) + (2.0 * locals.var_q_lnexpnum__blk840_dn4)) - locals.var_q_ln_term__blk834_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1s__blk937_dn6) + (2.0 * locals.var_q_lnexpnum__blk840_dn6)) - locals.var_q_ln_term__blk834_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1s__blk937_dn7) + (2.0 * locals.var_q_lnexpnum__blk840_dn7)) - locals.var_q_ln_term__blk834_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1s__blk937_dn8) + (2.0 * locals.var_q_lnexpnum__blk840_dn8)) - locals.var_q_ln_term__blk834_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1s__blk937_dn9) + (2.0 * locals.var_q_lnexpnum__blk840_dn9)) - locals.var_q_ln_term__blk834_dn9),)
    } else {
        (locals.var_q_q2_int__blk843, locals.var_q_q2_int__blk843_dn4, locals.var_q_q2_int__blk843_dn6, locals.var_q_q2_int__blk843_dn7, locals.var_q_q2_int__blk843_dn8, locals.var_q_q2_int__blk843_dn9,)
    }
};
        locals.var_q_q2_int__blk843 = assign32370_e35632;
        locals.var_q_q2_int__blk843_dn4 = assign32370_e35632_d_n4;
        locals.var_q_q2_int__blk843_dn6 = assign32370_e35632_d_n6;
        locals.var_q_q2_int__blk843_dn7 = assign32370_e35632_d_n7;
        locals.var_q_q2_int__blk843_dn8 = assign32370_e35632_d_n8;
        locals.var_q_q2_int__blk843_dn9 = assign32370_e35632_d_n9;

        let (assign32380_e35642, assign32380_e35642_d_n4, assign32380_e35642_d_n6, assign32380_e35642_d_n7, assign32380_e35642_d_n8, assign32380_e35642_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign32380_e35637: f64 = (2.0 * locals.var_q_d1_lnexpnum__blk841);
        let assign32380_e35638: f64 = (1.0 + assign32380_e35637);
        let assign32380_e35640: f64 = (assign32380_e35638 - locals.var_q_d1_ln__blk835);
        (assign32380_e35640, ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn4) - locals.var_q_d1_ln__blk835_dn4), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn6) - locals.var_q_d1_ln__blk835_dn6), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn7) - locals.var_q_d1_ln__blk835_dn7), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn8) - locals.var_q_d1_ln__blk835_dn8), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn9) - locals.var_q_d1_ln__blk835_dn9),)
    } else {
        (locals.var_q_d1_q2__blk844, locals.var_q_d1_q2__blk844_dn4, locals.var_q_d1_q2__blk844_dn6, locals.var_q_d1_q2__blk844_dn7, locals.var_q_d1_q2__blk844_dn8, locals.var_q_d1_q2__blk844_dn9,)
    }
};
        locals.var_q_d1_q2__blk844 = assign32380_e35642;
        locals.var_q_d1_q2__blk844_dn4 = assign32380_e35642_d_n4;
        locals.var_q_d1_q2__blk844_dn6 = assign32380_e35642_d_n6;
        locals.var_q_d1_q2__blk844_dn7 = assign32380_e35642_d_n7;
        locals.var_q_d1_q2__blk844_dn8 = assign32380_e35642_d_n8;
        locals.var_q_d1_q2__blk844_dn9 = assign32380_e35642_d_n9;

        let (assign32390_e35650, assign32390_e35650_d_n4, assign32390_e35650_d_n6, assign32390_e35650_d_n7, assign32390_e35650_d_n8, assign32390_e35650_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign32390_e35646: f64 = (2.0 * locals.var_q_d2_lnexpnum__blk842);
        let assign32390_e35648: f64 = (assign32390_e35646 - locals.var_q_d2_ln__blk836);
        (assign32390_e35648, ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn4) - locals.var_q_d2_ln__blk836_dn4), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn6) - locals.var_q_d2_ln__blk836_dn6), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn7) - locals.var_q_d2_ln__blk836_dn7), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn8) - locals.var_q_d2_ln__blk836_dn8), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn9) - locals.var_q_d2_ln__blk836_dn9),)
    } else {
        (locals.var_q_d2_q2__blk845, locals.var_q_d2_q2__blk845_dn4, locals.var_q_d2_q2__blk845_dn6, locals.var_q_d2_q2__blk845_dn7, locals.var_q_d2_q2__blk845_dn8, locals.var_q_d2_q2__blk845_dn9,)
    }
};
        locals.var_q_d2_q2__blk845 = assign32390_e35650;
        locals.var_q_d2_q2__blk845_dn4 = assign32390_e35650_d_n4;
        locals.var_q_d2_q2__blk845_dn6 = assign32390_e35650_d_n6;
        locals.var_q_d2_q2__blk845_dn7 = assign32390_e35650_d_n7;
        locals.var_q_d2_q2__blk845_dn8 = assign32390_e35650_d_n8;
        locals.var_q_d2_q2__blk845_dn9 = assign32390_e35650_d_n9;

        let (assign32400_e35658, assign32400_e35658_d_n4, assign32400_e35658_d_n6, assign32400_e35658_d_n7, assign32400_e35658_d_n8, assign32400_e35658_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign32400_e35655: f64 = (locals.var_k2__blk933 * locals.var_q_q2_int__blk843);
        let assign32400_e35656: f64 = (locals.var_q_k1q1__blk823 + assign32400_e35655);
        (assign32400_e35656, (locals.var_q_k1q1__blk823_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn4))), (locals.var_q_k1q1__blk823_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn6))), (locals.var_q_k1q1__blk823_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn7))), (locals.var_q_k1q1__blk823_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn8))), (locals.var_q_k1q1__blk823_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn9))),)
    } else {
        (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9,)
    }
};
        locals.var_q_qi_int__blk846 = assign32400_e35658;
        locals.var_q_qi_int__blk846_dn4 = assign32400_e35658_d_n4;
        locals.var_q_qi_int__blk846_dn6 = assign32400_e35658_d_n6;
        locals.var_q_qi_int__blk846_dn7 = assign32400_e35658_d_n7;
        locals.var_q_qi_int__blk846_dn8 = assign32400_e35658_d_n8;
        locals.var_q_qi_int__blk846_dn9 = assign32400_e35658_d_n9;

        let (assign32410_e35666, assign32410_e35666_d_n4, assign32410_e35666_d_n6, assign32410_e35666_d_n7, assign32410_e35666_d_n8, assign32410_e35666_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign32410_e35663: f64 = (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844);
        let assign32410_e35664: f64 = (locals.var_k1__blk932 + assign32410_e35663);
        (assign32410_e35664, (locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn4))), (locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn6))), (locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn7))), (locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn8))), (locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn9))),)
    } else {
        (locals.var_q_d1_qi__blk847, locals.var_q_d1_qi__blk847_dn4, locals.var_q_d1_qi__blk847_dn6, locals.var_q_d1_qi__blk847_dn7, locals.var_q_d1_qi__blk847_dn8, locals.var_q_d1_qi__blk847_dn9,)
    }
};
        locals.var_q_d1_qi__blk847 = assign32410_e35666;
        locals.var_q_d1_qi__blk847_dn4 = assign32410_e35666_d_n4;
        locals.var_q_d1_qi__blk847_dn6 = assign32410_e35666_d_n6;
        locals.var_q_d1_qi__blk847_dn7 = assign32410_e35666_d_n7;
        locals.var_q_d1_qi__blk847_dn8 = assign32410_e35666_d_n8;
        locals.var_q_d1_qi__blk847_dn9 = assign32410_e35666_d_n9;

        let (assign32420_e35672, assign32420_e35672_d_n4, assign32420_e35672_d_n6, assign32420_e35672_d_n7, assign32420_e35672_d_n8, assign32420_e35672_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign32420_e35670: f64 = (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845);
        (assign32420_e35670, ((locals.var_k2__blk933_dn4 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn9)),)
    } else {
        (locals.var_q_d2_qi__blk848, locals.var_q_d2_qi__blk848_dn4, locals.var_q_d2_qi__blk848_dn6, locals.var_q_d2_qi__blk848_dn7, locals.var_q_d2_qi__blk848_dn8, locals.var_q_d2_qi__blk848_dn9,)
    }
};
        locals.var_q_d2_qi__blk848 = assign32420_e35672;
        locals.var_q_d2_qi__blk848_dn4 = assign32420_e35672_d_n4;
        locals.var_q_d2_qi__blk848_dn6 = assign32420_e35672_d_n6;
        locals.var_q_d2_qi__blk848_dn7 = assign32420_e35672_d_n7;
        locals.var_q_d2_qi__blk848_dn8 = assign32420_e35672_d_n8;
        locals.var_q_d2_qi__blk848_dn9 = assign32420_e35672_d_n9;

    }

    pub(super) fn stamp_transient_block_87(
        locals: &mut StampLocals,
    ) {
        let (assign32430_e35680, assign32430_e35680_d_n4, assign32430_e35680_d_n6, assign32430_e35680_d_n7, assign32430_e35680_d_n8, assign32430_e35680_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign32430_e35676: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837);
        let assign32430_e35678: f64 = (assign32430_e35676 - locals.var_q_aexp__blk824);
        (assign32430_e35678, (((locals.var_q_qi_int__blk846_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_qi_int__blk846_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_qi_int__blk846_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_qi_int__blk846_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_qi_int__blk846_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9,)
    }
};
        locals.var_q_zero__blk849 = assign32430_e35680;
        locals.var_q_zero__blk849_dn4 = assign32430_e35680_d_n4;
        locals.var_q_zero__blk849_dn6 = assign32430_e35680_d_n6;
        locals.var_q_zero__blk849_dn7 = assign32430_e35680_d_n7;
        locals.var_q_zero__blk849_dn8 = assign32430_e35680_d_n8;
        locals.var_q_zero__blk849_dn9 = assign32430_e35680_d_n9;

        let (assign32440_e35692, assign32440_e35692_d_n4, assign32440_e35692_d_n6, assign32440_e35692_d_n7, assign32440_e35692_d_n8, assign32440_e35692_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign32440_e35684: f64 = (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837);
        let assign32440_e35687: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838);
        let assign32440_e35688: f64 = (assign32440_e35684 + assign32440_e35687);
        let assign32440_e35690: f64 = (assign32440_e35688 + locals.var_q_aexp__blk824);
        (assign32440_e35690, ((((locals.var_q_d1_qi__blk847_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn4)) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4), ((((locals.var_q_d1_qi__blk847_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn6)) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6), ((((locals.var_q_d1_qi__blk847_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn7)) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7), ((((locals.var_q_d1_qi__blk847_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn8)) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8), ((((locals.var_q_d1_qi__blk847_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn9)) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9,)
    }
};
        locals.var_q_d1_zero__blk850 = assign32440_e35692;
        locals.var_q_d1_zero__blk850_dn4 = assign32440_e35692_d_n4;
        locals.var_q_d1_zero__blk850_dn6 = assign32440_e35692_d_n6;
        locals.var_q_d1_zero__blk850_dn7 = assign32440_e35692_d_n7;
        locals.var_q_d1_zero__blk850_dn8 = assign32440_e35692_d_n8;
        locals.var_q_d1_zero__blk850_dn9 = assign32440_e35692_d_n9;

        let (assign32450_e35710, assign32450_e35710_d_n4, assign32450_e35710_d_n6, assign32450_e35710_d_n7, assign32450_e35710_d_n8, assign32450_e35710_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign32450_e35696: f64 = (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837);
        let assign32450_e35699: f64 = (2.0 * locals.var_q_d1_qi__blk847);
        let assign32450_e35701: f64 = (assign32450_e35699 * locals.var_q_d1_expnum__blk838);
        let assign32450_e35702: f64 = (assign32450_e35696 + assign32450_e35701);
        let assign32450_e35705: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839);
        let assign32450_e35706: f64 = (assign32450_e35702 + assign32450_e35705);
        let assign32450_e35708: f64 = (assign32450_e35706 - locals.var_q_aexp__blk824);
        (assign32450_e35708, (((((locals.var_q_d2_qi__blk848_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_d1_qi__blk847_dn4) * locals.var_q_d1_expnum__blk838) + (assign32450_e35699 * locals.var_q_d1_expnum__blk838_dn4))) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn4))) - locals.var_q_aexp__blk824_dn4), (((((locals.var_q_d2_qi__blk848_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_d1_qi__blk847_dn6) * locals.var_q_d1_expnum__blk838) + (assign32450_e35699 * locals.var_q_d1_expnum__blk838_dn6))) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn6))) - locals.var_q_aexp__blk824_dn6), (((((locals.var_q_d2_qi__blk848_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_d1_qi__blk847_dn7) * locals.var_q_d1_expnum__blk838) + (assign32450_e35699 * locals.var_q_d1_expnum__blk838_dn7))) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn7))) - locals.var_q_aexp__blk824_dn7), (((((locals.var_q_d2_qi__blk848_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_d1_qi__blk847_dn8) * locals.var_q_d1_expnum__blk838) + (assign32450_e35699 * locals.var_q_d1_expnum__blk838_dn8))) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn8))) - locals.var_q_aexp__blk824_dn8), (((((locals.var_q_d2_qi__blk848_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_d1_qi__blk847_dn9) * locals.var_q_d1_expnum__blk838) + (assign32450_e35699 * locals.var_q_d1_expnum__blk838_dn9))) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn9))) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_zero__blk851, locals.var_q_d2_zero__blk851_dn4, locals.var_q_d2_zero__blk851_dn6, locals.var_q_d2_zero__blk851_dn7, locals.var_q_d2_zero__blk851_dn8, locals.var_q_d2_zero__blk851_dn9,)
    }
};
        locals.var_q_d2_zero__blk851 = assign32450_e35710;
        locals.var_q_d2_zero__blk851_dn4 = assign32450_e35710_d_n4;
        locals.var_q_d2_zero__blk851_dn6 = assign32450_e35710_d_n6;
        locals.var_q_d2_zero__blk851_dn7 = assign32450_e35710_d_n7;
        locals.var_q_d2_zero__blk851_dn8 = assign32450_e35710_d_n8;
        locals.var_q_d2_zero__blk851_dn9 = assign32450_e35710_d_n9;

        let (assign32460_e35722, assign32460_e35722_d_n4, assign32460_e35722_d_n6, assign32460_e35722_d_n7, assign32460_e35722_d_n8, assign32460_e35722_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign32460_e35714: f64 = (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850);
        let assign32460_e35717: f64 = (0.5 * locals.var_q_zero__blk849);
        let assign32460_e35719: f64 = (assign32460_e35717 * locals.var_q_d2_zero__blk851);
        let assign32460_e35720: f64 = (assign32460_e35714 - assign32460_e35719);
        (assign32460_e35720, (((locals.var_q_d1_zero__blk850_dn4 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn4)) - (((0.5 * locals.var_q_zero__blk849_dn4) * locals.var_q_d2_zero__blk851) + (assign32460_e35717 * locals.var_q_d2_zero__blk851_dn4))), (((locals.var_q_d1_zero__blk850_dn6 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn6)) - (((0.5 * locals.var_q_zero__blk849_dn6) * locals.var_q_d2_zero__blk851) + (assign32460_e35717 * locals.var_q_d2_zero__blk851_dn6))), (((locals.var_q_d1_zero__blk850_dn7 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn7)) - (((0.5 * locals.var_q_zero__blk849_dn7) * locals.var_q_d2_zero__blk851) + (assign32460_e35717 * locals.var_q_d2_zero__blk851_dn7))), (((locals.var_q_d1_zero__blk850_dn8 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn8)) - (((0.5 * locals.var_q_zero__blk849_dn8) * locals.var_q_d2_zero__blk851) + (assign32460_e35717 * locals.var_q_d2_zero__blk851_dn8))), (((locals.var_q_d1_zero__blk850_dn9 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn9)) - (((0.5 * locals.var_q_zero__blk849_dn9) * locals.var_q_d2_zero__blk851) + (assign32460_e35717 * locals.var_q_d2_zero__blk851_dn9))),)
    } else {
        (locals.var_q_temp__blk860, locals.var_q_temp__blk860_dn4, locals.var_q_temp__blk860_dn6, locals.var_q_temp__blk860_dn7, locals.var_q_temp__blk860_dn8, locals.var_q_temp__blk860_dn9,)
    }
};
        locals.var_q_temp__blk860 = assign32460_e35722;
        locals.var_q_temp__blk860_dn4 = assign32460_e35722_d_n4;
        locals.var_q_temp__blk860_dn6 = assign32460_e35722_d_n6;
        locals.var_q_temp__blk860_dn7 = assign32460_e35722_d_n7;
        locals.var_q_temp__blk860_dn8 = assign32460_e35722_d_n8;
        locals.var_q_temp__blk860_dn9 = assign32460_e35722_d_n9;

        let (assign32470_e35737, assign32470_e35737_d_n4, assign32470_e35737_d_n6, assign32470_e35737_d_n7, assign32470_e35737_d_n8, assign32470_e35737_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign32470_e35725: f64 = (-locals.var_q_zero__blk849);
        let assign32470_e35727: f64 = (assign32470_e35725 * locals.var_q_d1_zero__blk850);
        let assign32470_e35729: f64 = (assign32470_e35727 * locals.var_q_temp__blk860);
        let assign32470_e35732: f64 = (locals.var_q_temp__blk860 * locals.var_q_temp__blk860);
        let assign32470_e35734: f64 = (assign32470_e35732 + 1e-200);
        let assign32470_e35735: f64 = (assign32470_e35729 / assign32470_e35734);
        (assign32470_e35735, ((((((((-locals.var_q_zero__blk849_dn4) * locals.var_q_d1_zero__blk850) + (assign32470_e35725 * locals.var_q_d1_zero__blk850_dn4)) * locals.var_q_temp__blk860) + (assign32470_e35727 * locals.var_q_temp__blk860_dn4)) * assign32470_e35734) - (assign32470_e35729 * ((locals.var_q_temp__blk860_dn4 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn4)))) / (assign32470_e35734 * assign32470_e35734)), ((((((((-locals.var_q_zero__blk849_dn6) * locals.var_q_d1_zero__blk850) + (assign32470_e35725 * locals.var_q_d1_zero__blk850_dn6)) * locals.var_q_temp__blk860) + (assign32470_e35727 * locals.var_q_temp__blk860_dn6)) * assign32470_e35734) - (assign32470_e35729 * ((locals.var_q_temp__blk860_dn6 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn6)))) / (assign32470_e35734 * assign32470_e35734)), ((((((((-locals.var_q_zero__blk849_dn7) * locals.var_q_d1_zero__blk850) + (assign32470_e35725 * locals.var_q_d1_zero__blk850_dn7)) * locals.var_q_temp__blk860) + (assign32470_e35727 * locals.var_q_temp__blk860_dn7)) * assign32470_e35734) - (assign32470_e35729 * ((locals.var_q_temp__blk860_dn7 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn7)))) / (assign32470_e35734 * assign32470_e35734)), ((((((((-locals.var_q_zero__blk849_dn8) * locals.var_q_d1_zero__blk850) + (assign32470_e35725 * locals.var_q_d1_zero__blk850_dn8)) * locals.var_q_temp__blk860) + (assign32470_e35727 * locals.var_q_temp__blk860_dn8)) * assign32470_e35734) - (assign32470_e35729 * ((locals.var_q_temp__blk860_dn8 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn8)))) / (assign32470_e35734 * assign32470_e35734)), ((((((((-locals.var_q_zero__blk849_dn9) * locals.var_q_d1_zero__blk850) + (assign32470_e35725 * locals.var_q_d1_zero__blk850_dn9)) * locals.var_q_temp__blk860) + (assign32470_e35727 * locals.var_q_temp__blk860_dn9)) * assign32470_e35734) - (assign32470_e35729 * ((locals.var_q_temp__blk860_dn9 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn9)))) / (assign32470_e35734 * assign32470_e35734)),)
    } else {
        (locals.var_q_eps2__blk852, locals.var_q_eps2__blk852_dn4, locals.var_q_eps2__blk852_dn6, locals.var_q_eps2__blk852_dn7, locals.var_q_eps2__blk852_dn8, locals.var_q_eps2__blk852_dn9,)
    }
};
        locals.var_q_eps2__blk852 = assign32470_e35737;
        locals.var_q_eps2__blk852_dn4 = assign32470_e35737_d_n4;
        locals.var_q_eps2__blk852_dn6 = assign32470_e35737_d_n6;
        locals.var_q_eps2__blk852_dn7 = assign32470_e35737_d_n7;
        locals.var_q_eps2__blk852_dn8 = assign32470_e35737_d_n8;
        locals.var_q_eps2__blk852_dn9 = assign32470_e35737_d_n9;

        let (assign32480_e35743, assign32480_e35743_d_n4, assign32480_e35743_d_n6, assign32480_e35743_d_n7, assign32480_e35743_d_n8, assign32480_e35743_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign32480_e35741: f64 = (locals.var_q1s__blk937 + locals.var_q_eps2__blk852);
        (assign32480_e35741, (locals.var_q1s__blk937_dn4 + locals.var_q_eps2__blk852_dn4), (locals.var_q1s__blk937_dn6 + locals.var_q_eps2__blk852_dn6), (locals.var_q1s__blk937_dn7 + locals.var_q_eps2__blk852_dn7), (locals.var_q1s__blk937_dn8 + locals.var_q_eps2__blk852_dn8), (locals.var_q1s__blk937_dn9 + locals.var_q_eps2__blk852_dn9),)
    } else {
        (locals.var_q1s__blk937, locals.var_q1s__blk937_dn4, locals.var_q1s__blk937_dn6, locals.var_q1s__blk937_dn7, locals.var_q1s__blk937_dn8, locals.var_q1s__blk937_dn9,)
    }
};
        locals.var_q1s__blk937 = assign32480_e35743;
        locals.var_q1s__blk937_dn4 = assign32480_e35743_d_n4;
        locals.var_q1s__blk937_dn6 = assign32480_e35743_d_n6;
        locals.var_q1s__blk937_dn7 = assign32480_e35743_d_n7;
        locals.var_q1s__blk937_dn8 = assign32480_e35743_d_n8;
        locals.var_q1s__blk937_dn9 = assign32480_e35743_d_n9;

        let (assign32490_e35749, assign32490_e35749_d_n4, assign32490_e35749_d_n6, assign32490_e35749_d_n7, assign32490_e35749_d_n8, assign32490_e35749_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign32490_e35747: f64 = (locals.var_k1__blk932 * locals.var_q1s__blk937);
        (assign32490_e35747, ((locals.var_k1__blk932_dn4 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn9)),)
    } else {
        (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9,)
    }
};
        locals.var_q_k1q1__blk823 = assign32490_e35749;
        locals.var_q_k1q1__blk823_dn4 = assign32490_e35749_d_n4;
        locals.var_q_k1q1__blk823_dn6 = assign32490_e35749_d_n6;
        locals.var_q_k1q1__blk823_dn7 = assign32490_e35749_d_n7;
        locals.var_q_k1q1__blk823_dn8 = assign32490_e35749_d_n8;
        locals.var_q_k1q1__blk823_dn9 = assign32490_e35749_d_n9;

        let assign32500_e35752: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign32500_e35754: f64 = assign32500_e35752;
        let assign32500_e35756: f64 = if assign32500_e35754 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1122 = assign32500_e35756;

        let (assign32510_e35767, assign32510_e35767_d_n4, assign32510_e35767_d_n6, assign32510_e35767_d_n7, assign32510_e35767_d_n8, assign32510_e35767_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1122 != 0.0)) {
        let assign32510_e35762: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign32510_e35764: f64 = assign32510_e35762;
        let assign32510_e35765: f64 = (assign32510_e35764).exp();
        (assign32510_e35765, (assign32510_e35765 * (locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4)), (assign32510_e35765 * (locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6)), (assign32510_e35765 * (locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7)), (assign32510_e35765 * (locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8)), (assign32510_e35765 * (locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign32510_e35767;
        locals.var_q_temp1__blk814_dn4 = assign32510_e35767_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign32510_e35767_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign32510_e35767_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign32510_e35767_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign32510_e35767_d_n9;

        let (assign32520_e35808, assign32520_e35808_d_n4, assign32520_e35808_d_n6, assign32520_e35808_d_n7, assign32520_e35808_d_n8, assign32520_e35808_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1122 == 0.0)) {
        let assign32520_e35776: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign32520_e35778: f64 = assign32520_e35776;
        let assign32520_e35780: f64 = (assign32520_e35778 - 80.0);
        let assign32520_e35785: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign32520_e35787: f64 = assign32520_e35785;
        let assign32520_e35789: f64 = (assign32520_e35787 - 80.0);
        let assign32520_e35790: f64 = (0.5 * assign32520_e35789);
        let assign32520_e35794: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign32520_e35796: f64 = assign32520_e35794;
        let assign32520_e35798: f64 = (assign32520_e35796 - 80.0);
        let assign32520_e35800: f64 = (assign32520_e35798 * 0.3333333333333);
        let assign32520_e35801: f64 = (1.0 + assign32520_e35800);
        let assign32520_e35802: f64 = (assign32520_e35790 * assign32520_e35801);
        let assign32520_e35803: f64 = (1.0 + assign32520_e35802);
        let assign32520_e35804: f64 = (assign32520_e35780 * assign32520_e35803);
        let assign32520_e35805: f64 = (1.0 + assign32520_e35804);
        let assign32520_e35806: f64 = (5.54062e34 * assign32520_e35805);
        (assign32520_e35806, (5.54062e34 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4) * assign32520_e35803) + (assign32520_e35780 * (((0.5 * (locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4)) * assign32520_e35801) + (assign32520_e35790 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6) * assign32520_e35803) + (assign32520_e35780 * (((0.5 * (locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6)) * assign32520_e35801) + (assign32520_e35790 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7) * assign32520_e35803) + (assign32520_e35780 * (((0.5 * (locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7)) * assign32520_e35801) + (assign32520_e35790 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8) * assign32520_e35803) + (assign32520_e35780 * (((0.5 * (locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8)) * assign32520_e35801) + (assign32520_e35790 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9) * assign32520_e35803) + (assign32520_e35780 * (((0.5 * (locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9)) * assign32520_e35801) + (assign32520_e35790 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign32520_e35808;
        locals.var_q_temp1__blk814_dn4 = assign32520_e35808_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign32520_e35808_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign32520_e35808_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign32520_e35808_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign32520_e35808_d_n9;

        let (assign32530_e35814, assign32530_e35814_d_n4, assign32530_e35814_d_n6, assign32530_e35814_d_n7, assign32530_e35814_d_n8, assign32530_e35814_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign32530_e35812: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
        (assign32530_e35812, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_aexp__blk824, locals.var_q_aexp__blk824_dn4, locals.var_q_aexp__blk824_dn6, locals.var_q_aexp__blk824_dn7, locals.var_q_aexp__blk824_dn8, locals.var_q_aexp__blk824_dn9,)
    }
};
        locals.var_q_aexp__blk824 = assign32530_e35814;
        locals.var_q_aexp__blk824_dn4 = assign32530_e35814_d_n4;
        locals.var_q_aexp__blk824_dn6 = assign32530_e35814_d_n6;
        locals.var_q_aexp__blk824_dn7 = assign32530_e35814_d_n7;
        locals.var_q_aexp__blk824_dn8 = assign32530_e35814_d_n8;
        locals.var_q_aexp__blk824_dn9 = assign32530_e35814_d_n9;

        let (assign32540_e35822, assign32540_e35822_d_n4, assign32540_e35822_d_n6, assign32540_e35822_d_n7, assign32540_e35822_d_n8, assign32540_e35822_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign32540_e35818: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
        let assign32540_e35820: f64 = (assign32540_e35818 - locals.var_q_aexp__blk824);
        (assign32540_e35820, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign32540_e35822;
        locals.var_q_qsq__blk825_dn4 = assign32540_e35822_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign32540_e35822_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign32540_e35822_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign32540_e35822_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign32540_e35822_d_n9;

        let (assign32550_e35832, assign32550_e35832_d_n4, assign32550_e35832_d_n6, assign32550_e35832_d_n7, assign32550_e35832_d_n8, assign32550_e35832_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign32550_e35826: f64 = (2.0 * locals.var_k1__blk932);
        let assign32550_e35828: f64 = (assign32550_e35826 * locals.var_q_k1q1__blk823);
        let assign32550_e35830: f64 = (assign32550_e35828 + locals.var_q_aexp__blk824);
        (assign32550_e35830, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign32550_e35826 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign32550_e35826 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign32550_e35826 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign32550_e35826 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign32550_e35826 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_qsq__blk826, locals.var_q_d1_qsq__blk826_dn4, locals.var_q_d1_qsq__blk826_dn6, locals.var_q_d1_qsq__blk826_dn7, locals.var_q_d1_qsq__blk826_dn8, locals.var_q_d1_qsq__blk826_dn9,)
    }
};
        locals.var_q_d1_qsq__blk826 = assign32550_e35832;
        locals.var_q_d1_qsq__blk826_dn4 = assign32550_e35832_d_n4;
        locals.var_q_d1_qsq__blk826_dn6 = assign32550_e35832_d_n6;
        locals.var_q_d1_qsq__blk826_dn7 = assign32550_e35832_d_n7;
        locals.var_q_d1_qsq__blk826_dn8 = assign32550_e35832_d_n8;
        locals.var_q_d1_qsq__blk826_dn9 = assign32550_e35832_d_n9;

        let (assign32560_e35842, assign32560_e35842_d_n4, assign32560_e35842_d_n6, assign32560_e35842_d_n7, assign32560_e35842_d_n8, assign32560_e35842_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign32560_e35836: f64 = (2.0 * locals.var_k1__blk932);
        let assign32560_e35838: f64 = (assign32560_e35836 * locals.var_k1__blk932);
        let assign32560_e35840: f64 = (assign32560_e35838 - locals.var_q_aexp__blk824);
        (assign32560_e35840, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_k1__blk932) + (assign32560_e35836 * locals.var_k1__blk932_dn4)) - locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_k1__blk932) + (assign32560_e35836 * locals.var_k1__blk932_dn6)) - locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_k1__blk932) + (assign32560_e35836 * locals.var_k1__blk932_dn7)) - locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_k1__blk932) + (assign32560_e35836 * locals.var_k1__blk932_dn8)) - locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_k1__blk932) + (assign32560_e35836 * locals.var_k1__blk932_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_qsq__blk827, locals.var_q_d2_qsq__blk827_dn4, locals.var_q_d2_qsq__blk827_dn6, locals.var_q_d2_qsq__blk827_dn7, locals.var_q_d2_qsq__blk827_dn8, locals.var_q_d2_qsq__blk827_dn9,)
    }
};
        locals.var_q_d2_qsq__blk827 = assign32560_e35842;
        locals.var_q_d2_qsq__blk827_dn4 = assign32560_e35842_d_n4;
        locals.var_q_d2_qsq__blk827_dn6 = assign32560_e35842_d_n6;
        locals.var_q_d2_qsq__blk827_dn7 = assign32560_e35842_d_n7;
        locals.var_q_d2_qsq__blk827_dn8 = assign32560_e35842_d_n8;
        locals.var_q_d2_qsq__blk827_dn9 = assign32560_e35842_d_n9;

        let assign32570_e35845: f64 = (-0.005);
        let assign32570_e35846: f64 = if locals.var_q_qsq__blk825 < assign32570_e35845 { 1.0 } else { 0.0 };
        locals.var_guard1123 = assign32570_e35846;

        let (assign32580_e35854, assign32580_e35854_d_n4, assign32580_e35854_d_n6, assign32580_e35854_d_n7, assign32580_e35854_d_n8, assign32580_e35854_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1123 != 0.0)) {
        let assign32580_e35851: f64 = (locals.var_q_qsq__blk825).abs();
        let assign32580_e35852: f64 = (assign32580_e35851).sqrt();
        (assign32580_e35852, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign32580_e35852)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign32580_e35852)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign32580_e35852)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign32580_e35852)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign32580_e35852)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign32580_e35854;
        locals.var_q_rac_qsq__blk828_dn4 = assign32580_e35854_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign32580_e35854_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign32580_e35854_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign32580_e35854_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign32580_e35854_d_n9;

        let (assign32590_e35865, assign32590_e35865_d_n4, assign32590_e35865_d_n6, assign32590_e35865_d_n7, assign32590_e35865_d_n8, assign32590_e35865_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1123 != 0.0)) {
        let assign32590_e35861: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign32590_e35862: f64 = (assign32590_e35861).tan();
        let assign32590_e35863: f64 = (locals.var_q_rac_qsq__blk828 / assign32590_e35862);
        (assign32590_e35863, (((locals.var_q_rac_qsq__blk828_dn4 * assign32590_e35862) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign32590_e35861).cos() * (assign32590_e35861).cos())))) / (assign32590_e35862 * assign32590_e35862)), (((locals.var_q_rac_qsq__blk828_dn6 * assign32590_e35862) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign32590_e35861).cos() * (assign32590_e35861).cos())))) / (assign32590_e35862 * assign32590_e35862)), (((locals.var_q_rac_qsq__blk828_dn7 * assign32590_e35862) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign32590_e35861).cos() * (assign32590_e35861).cos())))) / (assign32590_e35862 * assign32590_e35862)), (((locals.var_q_rac_qsq__blk828_dn8 * assign32590_e35862) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign32590_e35861).cos() * (assign32590_e35861).cos())))) / (assign32590_e35862 * assign32590_e35862)), (((locals.var_q_rac_qsq__blk828_dn9 * assign32590_e35862) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign32590_e35861).cos() * (assign32590_e35861).cos())))) / (assign32590_e35862 * assign32590_e35862)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign32590_e35865;
        locals.var_q_qcoth__blk829_dn4 = assign32590_e35865_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign32590_e35865_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign32590_e35865_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign32590_e35865_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign32590_e35865_d_n9;

        let (assign32600_e35875, assign32600_e35875_d_n4, assign32600_e35875_d_n6, assign32600_e35875_d_n7, assign32600_e35875_d_n8, assign32600_e35875_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1123 != 0.0)) {
        let assign32600_e35871: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign32600_e35873: f64 = (assign32600_e35871 / locals.var_q_qsq__blk825);
        (assign32600_e35873, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign32600_e35871 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign32600_e35871 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign32600_e35871 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign32600_e35871 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign32600_e35871 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign32600_e35875;
        locals.var_q_temp1__blk814_dn4 = assign32600_e35875_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign32600_e35875_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign32600_e35875_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign32600_e35875_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign32600_e35875_d_n9;

        let (assign32610_e35889, assign32610_e35889_d_n4, assign32610_e35889_d_n6, assign32610_e35889_d_n7, assign32610_e35889_d_n8, assign32610_e35889_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1123 != 0.0)) {
        let assign32610_e35883: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign32610_e35884: f64 = (locals.var_q_qcoth__blk829 * assign32610_e35883);
        let assign32610_e35885: f64 = (locals.var_q_qsq__blk825 + assign32610_e35884);
        let assign32610_e35887: f64 = (assign32610_e35885 * locals.var_q_temp1__blk814);
        (assign32610_e35887, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign32610_e35883) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign32610_e35885 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign32610_e35883) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign32610_e35885 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign32610_e35883) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign32610_e35885 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign32610_e35883) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign32610_e35885 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign32610_e35883) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign32610_e35885 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign32610_e35889;
        locals.var_q_d1_qcoth__blk830_dn4 = assign32610_e35889_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign32610_e35889_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign32610_e35889_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign32610_e35889_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign32610_e35889_d_n9;

        let (assign32620_e35911, assign32620_e35911_d_n4, assign32620_e35911_d_n6, assign32620_e35911_d_n7, assign32620_e35911_d_n8, assign32620_e35911_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1123 != 0.0)) {
        let assign32620_e35896: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign32620_e35899: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign32620_e35900: f64 = (assign32620_e35896 * assign32620_e35899);
        let assign32620_e35901: f64 = (locals.var_q_d1_qsq__blk826 - assign32620_e35900);
        let assign32620_e35903: f64 = (assign32620_e35901 * locals.var_q_temp1__blk814);
        let assign32620_e35906: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign32620_e35908: f64 = (assign32620_e35906 / locals.var_q_d1_qsq__blk826);
        let assign32620_e35909: f64 = (assign32620_e35903 + assign32620_e35908);
        (assign32620_e35909, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign32620_e35899) + (assign32620_e35896 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign32620_e35901 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign32620_e35906 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign32620_e35899) + (assign32620_e35896 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign32620_e35901 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign32620_e35906 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign32620_e35899) + (assign32620_e35896 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign32620_e35901 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign32620_e35906 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign32620_e35899) + (assign32620_e35896 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign32620_e35901 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign32620_e35906 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign32620_e35899) + (assign32620_e35896 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign32620_e35901 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign32620_e35906 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign32620_e35911;
        locals.var_q_d2_qcoth__blk832_dn4 = assign32620_e35911_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign32620_e35911_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign32620_e35911_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign32620_e35911_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign32620_e35911_d_n9;

        let (assign32630_e35921, assign32630_e35921_d_n4, assign32630_e35921_d_n6, assign32630_e35921_d_n7, assign32630_e35921_d_n8, assign32630_e35921_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1123 != 0.0)) {
        let assign32630_e35918: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign32630_e35919: f64 = (1.0 - assign32630_e35918);
        (assign32630_e35919, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign32630_e35921;
        locals.var_q_temp2__blk815_dn4 = assign32630_e35921_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign32630_e35921_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign32630_e35921_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign32630_e35921_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign32630_e35921_d_n9;

        let (assign32640_e35931, assign32640_e35931_d_n4, assign32640_e35931_d_n6, assign32640_e35931_d_n7, assign32640_e35931_d_n8, assign32640_e35931_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1123 != 0.0)) {
        let assign32640_e35927: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign32640_e35929: f64 = (assign32640_e35927 * locals.var_q_temp2__blk815);
        (assign32640_e35929, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign32640_e35927 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign32640_e35927 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign32640_e35927 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign32640_e35927 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign32640_e35927 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign32640_e35931;
        locals.var_q_d1_ln__blk835_dn4 = assign32640_e35931_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign32640_e35931_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign32640_e35931_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign32640_e35931_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign32640_e35931_d_n9;

        let (assign32650_e35949, assign32650_e35949_d_n4, assign32650_e35949_d_n6, assign32650_e35949_d_n7, assign32650_e35949_d_n8, assign32650_e35949_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1123 != 0.0)) {
        let assign32650_e35937: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign32650_e35942: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign32650_e35943: f64 = (locals.var_q_d1_ln__blk835 + assign32650_e35942);
        let assign32650_e35944: f64 = (locals.var_q_d1_qsq__blk826 * assign32650_e35943);
        let assign32650_e35945: f64 = (assign32650_e35937 - assign32650_e35944);
        let assign32650_e35947: f64 = (assign32650_e35945 / locals.var_q_qsq__blk825);
        (assign32650_e35947, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign32650_e35943) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign32650_e35945 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign32650_e35943) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign32650_e35945 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign32650_e35943) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign32650_e35945 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign32650_e35943) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign32650_e35945 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign32650_e35943) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign32650_e35945 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign32650_e35949;
        locals.var_q_d2_ln__blk836_dn4 = assign32650_e35949_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign32650_e35949_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign32650_e35949_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign32650_e35949_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign32650_e35949_d_n9;

        let assign32660_e35952: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1124 = assign32660_e35952;

        let (assign32670_e35963, assign32670_e35963_d_n4, assign32670_e35963_d_n6, assign32670_e35963_d_n7, assign32670_e35963_d_n8, assign32670_e35963_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 != 0.0)) {
        let assign32670_e35960: f64 = (locals.var_q_qsq__blk825).abs();
        let assign32670_e35961: f64 = (assign32670_e35960).sqrt();
        (assign32670_e35961, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign32670_e35961)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign32670_e35961)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign32670_e35961)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign32670_e35961)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign32670_e35961)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign32670_e35963;
        locals.var_q_rac_qsq__blk828_dn4 = assign32670_e35963_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign32670_e35963_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign32670_e35963_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign32670_e35963_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign32670_e35963_d_n9;

        let (assign32680_e35974, assign32680_e35974_d_n4, assign32680_e35974_d_n6, assign32680_e35974_d_n7, assign32680_e35974_d_n8, assign32680_e35974_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 != 0.0)) {
        let assign32680_e35971: f64 = (-locals.var_q_rac_qsq__blk828);
        let assign32680_e35972: f64 = (assign32680_e35971).exp();
        (assign32680_e35972, (assign32680_e35972 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign32680_e35972 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign32680_e35972 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign32680_e35972 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign32680_e35972 * (-locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign32680_e35974;
        locals.var_q_invexpq__blk831_dn4 = assign32680_e35974_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign32680_e35974_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign32680_e35974_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign32680_e35974_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign32680_e35974_d_n9;

        let (assign32690_e35991, assign32690_e35991_d_n4, assign32690_e35991_d_n6, assign32690_e35991_d_n7, assign32690_e35991_d_n8, assign32690_e35991_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 != 0.0)) {
        let assign32690_e35984: f64 = (1.0 + locals.var_q_invexpq__blk831);
        let assign32690_e35985: f64 = (locals.var_q_rac_qsq__blk828 * assign32690_e35984);
        let assign32690_e35988: f64 = (1.0 - locals.var_q_invexpq__blk831);
        let assign32690_e35989: f64 = (assign32690_e35985 / assign32690_e35988);
        (assign32690_e35989, (((((locals.var_q_rac_qsq__blk828_dn4 * assign32690_e35984) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign32690_e35988) - (assign32690_e35985 * (-locals.var_q_invexpq__blk831_dn4))) / (assign32690_e35988 * assign32690_e35988)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign32690_e35984) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign32690_e35988) - (assign32690_e35985 * (-locals.var_q_invexpq__blk831_dn6))) / (assign32690_e35988 * assign32690_e35988)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign32690_e35984) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign32690_e35988) - (assign32690_e35985 * (-locals.var_q_invexpq__blk831_dn7))) / (assign32690_e35988 * assign32690_e35988)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign32690_e35984) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign32690_e35988) - (assign32690_e35985 * (-locals.var_q_invexpq__blk831_dn8))) / (assign32690_e35988 * assign32690_e35988)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign32690_e35984) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign32690_e35988) - (assign32690_e35985 * (-locals.var_q_invexpq__blk831_dn9))) / (assign32690_e35988 * assign32690_e35988)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign32690_e35991;
        locals.var_q_qcoth__blk829_dn4 = assign32690_e35991_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign32690_e35991_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign32690_e35991_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign32690_e35991_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign32690_e35991_d_n9;

        let (assign32700_e36004, assign32700_e36004_d_n4, assign32700_e36004_d_n6, assign32700_e36004_d_n7, assign32700_e36004_d_n8, assign32700_e36004_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 != 0.0)) {
        let assign32700_e36000: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign32700_e36002: f64 = (assign32700_e36000 / locals.var_q_qsq__blk825);
        (assign32700_e36002, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign32700_e36000 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign32700_e36000 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign32700_e36000 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign32700_e36000 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign32700_e36000 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign32700_e36004;
        locals.var_q_temp1__blk814_dn4 = assign32700_e36004_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign32700_e36004_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign32700_e36004_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign32700_e36004_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign32700_e36004_d_n9;

        let (assign32710_e36021, assign32710_e36021_d_n4, assign32710_e36021_d_n6, assign32710_e36021_d_n7, assign32710_e36021_d_n8, assign32710_e36021_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 != 0.0)) {
        let assign32710_e36015: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign32710_e36016: f64 = (locals.var_q_qcoth__blk829 * assign32710_e36015);
        let assign32710_e36017: f64 = (locals.var_q_qsq__blk825 + assign32710_e36016);
        let assign32710_e36019: f64 = (assign32710_e36017 * locals.var_q_temp1__blk814);
        (assign32710_e36019, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign32710_e36015) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign32710_e36017 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign32710_e36015) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign32710_e36017 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign32710_e36015) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign32710_e36017 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign32710_e36015) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign32710_e36017 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign32710_e36015) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign32710_e36017 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign32710_e36021;
        locals.var_q_d1_qcoth__blk830_dn4 = assign32710_e36021_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign32710_e36021_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign32710_e36021_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign32710_e36021_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign32710_e36021_d_n9;

        let (assign32720_e36046, assign32720_e36046_d_n4, assign32720_e36046_d_n6, assign32720_e36046_d_n7, assign32720_e36046_d_n8, assign32720_e36046_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 != 0.0)) {
        let assign32720_e36031: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign32720_e36034: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign32720_e36035: f64 = (assign32720_e36031 * assign32720_e36034);
        let assign32720_e36036: f64 = (locals.var_q_d1_qsq__blk826 - assign32720_e36035);
        let assign32720_e36038: f64 = (assign32720_e36036 * locals.var_q_temp1__blk814);
        let assign32720_e36041: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign32720_e36043: f64 = (assign32720_e36041 / locals.var_q_d1_qsq__blk826);
        let assign32720_e36044: f64 = (assign32720_e36038 + assign32720_e36043);
        (assign32720_e36044, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign32720_e36034) + (assign32720_e36031 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign32720_e36036 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign32720_e36041 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign32720_e36034) + (assign32720_e36031 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign32720_e36036 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign32720_e36041 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign32720_e36034) + (assign32720_e36031 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign32720_e36036 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign32720_e36041 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign32720_e36034) + (assign32720_e36031 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign32720_e36036 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign32720_e36041 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign32720_e36034) + (assign32720_e36031 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign32720_e36036 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign32720_e36041 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign32720_e36046;
        locals.var_q_d2_qcoth__blk832_dn4 = assign32720_e36046_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign32720_e36046_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign32720_e36046_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign32720_e36046_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign32720_e36046_d_n9;

        let (assign32730_e36059, assign32730_e36059_d_n4, assign32730_e36059_d_n6, assign32730_e36059_d_n7, assign32730_e36059_d_n8, assign32730_e36059_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 != 0.0)) {
        let assign32730_e36056: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign32730_e36057: f64 = (1.0 - assign32730_e36056);
        (assign32730_e36057, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign32730_e36059;
        locals.var_q_temp2__blk815_dn4 = assign32730_e36059_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign32730_e36059_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign32730_e36059_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign32730_e36059_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign32730_e36059_d_n9;

    }

    pub(super) fn stamp_transient_block_88(
        locals: &mut StampLocals,
    ) {
        let (assign32740_e36072, assign32740_e36072_d_n4, assign32740_e36072_d_n6, assign32740_e36072_d_n7, assign32740_e36072_d_n8, assign32740_e36072_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 != 0.0)) {
        let assign32740_e36068: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign32740_e36070: f64 = (assign32740_e36068 * locals.var_q_temp2__blk815);
        (assign32740_e36070, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign32740_e36068 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign32740_e36068 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign32740_e36068 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign32740_e36068 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign32740_e36068 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign32740_e36072;
        locals.var_q_d1_ln__blk835_dn4 = assign32740_e36072_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign32740_e36072_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign32740_e36072_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign32740_e36072_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign32740_e36072_d_n9;

        let (assign32750_e36093, assign32750_e36093_d_n4, assign32750_e36093_d_n6, assign32750_e36093_d_n7, assign32750_e36093_d_n8, assign32750_e36093_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 != 0.0)) {
        let assign32750_e36081: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign32750_e36086: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign32750_e36087: f64 = (locals.var_q_d1_ln__blk835 + assign32750_e36086);
        let assign32750_e36088: f64 = (locals.var_q_d1_qsq__blk826 * assign32750_e36087);
        let assign32750_e36089: f64 = (assign32750_e36081 - assign32750_e36088);
        let assign32750_e36091: f64 = (assign32750_e36089 / locals.var_q_qsq__blk825);
        (assign32750_e36091, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign32750_e36087) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign32750_e36089 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign32750_e36087) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign32750_e36089 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign32750_e36087) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign32750_e36089 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign32750_e36087) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign32750_e36089 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign32750_e36087) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign32750_e36089 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign32750_e36093;
        locals.var_q_d2_ln__blk836_dn4 = assign32750_e36093_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign32750_e36093_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign32750_e36093_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign32750_e36093_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign32750_e36093_d_n9;

        let (assign32760_e36121, assign32760_e36121_d_n4, assign32760_e36121_d_n6, assign32760_e36121_d_n7, assign32760_e36121_d_n8, assign32760_e36121_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 == 0.0)) {
        let assign32760_e36105: f64 = (locals.var_q_qsq__blk825 * 0.0166666666667);
        let assign32760_e36109: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign32760_e36113: f64 = (locals.var_q_qsq__blk825 * 0.025);
        let assign32760_e36114: f64 = (1.0 - assign32760_e36113);
        let assign32760_e36115: f64 = (assign32760_e36109 * assign32760_e36114);
        let assign32760_e36116: f64 = (1.0 - assign32760_e36115);
        let assign32760_e36117: f64 = (assign32760_e36105 * assign32760_e36116);
        let assign32760_e36118: f64 = (1.0 - assign32760_e36117);
        let assign32760_e36119: f64 = (0.1666666666667 * assign32760_e36118);
        (assign32760_e36119, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0166666666667) * assign32760_e36116) + (assign32760_e36105 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign32760_e36114) + (assign32760_e36109 * (-(locals.var_q_qsq__blk825_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0166666666667) * assign32760_e36116) + (assign32760_e36105 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign32760_e36114) + (assign32760_e36109 * (-(locals.var_q_qsq__blk825_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0166666666667) * assign32760_e36116) + (assign32760_e36105 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign32760_e36114) + (assign32760_e36109 * (-(locals.var_q_qsq__blk825_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0166666666667) * assign32760_e36116) + (assign32760_e36105 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign32760_e36114) + (assign32760_e36109 * (-(locals.var_q_qsq__blk825_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0166666666667) * assign32760_e36116) + (assign32760_e36105 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign32760_e36114) + (assign32760_e36109 * (-(locals.var_q_qsq__blk825_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign32760_e36121;
        locals.var_q_temp3__blk816_dn4 = assign32760_e36121_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign32760_e36121_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign32760_e36121_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign32760_e36121_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign32760_e36121_d_n9;

        let (assign32770_e36135, assign32770_e36135_d_n4, assign32770_e36135_d_n6, assign32770_e36135_d_n7, assign32770_e36135_d_n8, assign32770_e36135_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 == 0.0)) {
        let assign32770_e36132: f64 = (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816);
        let assign32770_e36133: f64 = (2.0 + assign32770_e36132);
        (assign32770_e36133, ((locals.var_q_qsq__blk825_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn4)), ((locals.var_q_qsq__blk825_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn6)), ((locals.var_q_qsq__blk825_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn7)), ((locals.var_q_qsq__blk825_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn8)), ((locals.var_q_qsq__blk825_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign32770_e36135;
        locals.var_q_qcoth__blk829_dn4 = assign32770_e36135_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign32770_e36135_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign32770_e36135_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign32770_e36135_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign32770_e36135_d_n9;

        let (assign32780_e36163, assign32780_e36163_d_n4, assign32780_e36163_d_n6, assign32780_e36163_d_n7, assign32780_e36163_d_n8, assign32780_e36163_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 == 0.0)) {
        let assign32780_e36147: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign32780_e36151: f64 = (locals.var_q_qsq__blk825 * 0.0357142857143);
        let assign32780_e36155: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign32780_e36156: f64 = (1.0 - assign32780_e36155);
        let assign32780_e36157: f64 = (assign32780_e36151 * assign32780_e36156);
        let assign32780_e36158: f64 = (1.0 - assign32780_e36157);
        let assign32780_e36159: f64 = (assign32780_e36147 * assign32780_e36158);
        let assign32780_e36160: f64 = (1.0 - assign32780_e36159);
        let assign32780_e36161: f64 = (0.1666666666667 * assign32780_e36160);
        (assign32780_e36161, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0333333333333) * assign32780_e36158) + (assign32780_e36147 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0357142857143) * assign32780_e36156) + (assign32780_e36151 * (-(locals.var_q_qsq__blk825_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0333333333333) * assign32780_e36158) + (assign32780_e36147 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0357142857143) * assign32780_e36156) + (assign32780_e36151 * (-(locals.var_q_qsq__blk825_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0333333333333) * assign32780_e36158) + (assign32780_e36147 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0357142857143) * assign32780_e36156) + (assign32780_e36151 * (-(locals.var_q_qsq__blk825_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0333333333333) * assign32780_e36158) + (assign32780_e36147 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0357142857143) * assign32780_e36156) + (assign32780_e36151 * (-(locals.var_q_qsq__blk825_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0333333333333) * assign32780_e36158) + (assign32780_e36147 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0357142857143) * assign32780_e36156) + (assign32780_e36151 * (-(locals.var_q_qsq__blk825_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign32780_e36163;
        locals.var_q_temp1__blk814_dn4 = assign32780_e36163_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign32780_e36163_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign32780_e36163_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign32780_e36163_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign32780_e36163_d_n9;

        let (assign32790_e36175, assign32790_e36175_d_n4, assign32790_e36175_d_n6, assign32790_e36175_d_n7, assign32790_e36175_d_n8, assign32790_e36175_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 == 0.0)) {
        let assign32790_e36173: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814);
        (assign32790_e36173, ((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign32790_e36175;
        locals.var_q_d1_qcoth__blk830_dn4 = assign32790_e36175_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign32790_e36175_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign32790_e36175_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign32790_e36175_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign32790_e36175_d_n9;

        let (assign32800_e36203, assign32800_e36203_d_n4, assign32800_e36203_d_n6, assign32800_e36203_d_n7, assign32800_e36203_d_n8, assign32800_e36203_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 == 0.0)) {
        let assign32800_e36187: f64 = (locals.var_q_qsq__blk825 * 0.0714285714286);
        let assign32800_e36191: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign32800_e36195: f64 = (0.0420875420875421 * locals.var_q_qsq__blk825);
        let assign32800_e36196: f64 = (1.0 - assign32800_e36195);
        let assign32800_e36197: f64 = (assign32800_e36191 * assign32800_e36196);
        let assign32800_e36198: f64 = (1.0 - assign32800_e36197);
        let assign32800_e36199: f64 = (assign32800_e36187 * assign32800_e36198);
        let assign32800_e36200: f64 = (1.0 - assign32800_e36199);
        let assign32800_e36201: f64 = (0.0055555555556 * assign32800_e36200);
        (assign32800_e36201, (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0714285714286) * assign32800_e36198) + (assign32800_e36187 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign32800_e36196) + (assign32800_e36191 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0714285714286) * assign32800_e36198) + (assign32800_e36187 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign32800_e36196) + (assign32800_e36191 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0714285714286) * assign32800_e36198) + (assign32800_e36187 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign32800_e36196) + (assign32800_e36191 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0714285714286) * assign32800_e36198) + (assign32800_e36187 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign32800_e36196) + (assign32800_e36191 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0714285714286) * assign32800_e36198) + (assign32800_e36187 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign32800_e36196) + (assign32800_e36191 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn9))))))))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign32800_e36203;
        locals.var_q_temp2__blk815_dn4 = assign32800_e36203_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign32800_e36203_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign32800_e36203_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign32800_e36203_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign32800_e36203_d_n9;

        let (assign32810_e36221, assign32810_e36221_d_n4, assign32810_e36221_d_n6, assign32810_e36221_d_n7, assign32810_e36221_d_n8, assign32810_e36221_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 == 0.0)) {
        let assign32810_e36213: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814);
        let assign32810_e36216: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826);
        let assign32810_e36218: f64 = (assign32810_e36216 * locals.var_q_temp2__blk815);
        let assign32810_e36219: f64 = (assign32810_e36213 - assign32810_e36218);
        (assign32810_e36219, (((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn4)) - ((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn4)) * locals.var_q_temp2__blk815) + (assign32810_e36216 * locals.var_q_temp2__blk815_dn4))), (((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn6)) - ((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn6)) * locals.var_q_temp2__blk815) + (assign32810_e36216 * locals.var_q_temp2__blk815_dn6))), (((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn7)) - ((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn7)) * locals.var_q_temp2__blk815) + (assign32810_e36216 * locals.var_q_temp2__blk815_dn7))), (((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn8)) - ((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn8)) * locals.var_q_temp2__blk815) + (assign32810_e36216 * locals.var_q_temp2__blk815_dn8))), (((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn9)) - ((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn9)) * locals.var_q_temp2__blk815) + (assign32810_e36216 * locals.var_q_temp2__blk815_dn9))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign32810_e36221;
        locals.var_q_d2_qcoth__blk832_dn4 = assign32810_e36221_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign32810_e36221_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign32810_e36221_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign32810_e36221_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign32810_e36221_d_n9;

        let (assign32820_e36236, assign32820_e36236_d_n4, assign32820_e36236_d_n6, assign32820_e36236_d_n7, assign32820_e36236_d_n8, assign32820_e36236_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 == 0.0)) {
        let assign32820_e36230: f64 = (-0.5);
        let assign32820_e36232: f64 = (assign32820_e36230 * locals.var_q_d1_qsq__blk826);
        let assign32820_e36234: f64 = (assign32820_e36232 * locals.var_q_temp3__blk816);
        (assign32820_e36234, (((assign32820_e36230 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_temp3__blk816) + (assign32820_e36232 * locals.var_q_temp3__blk816_dn4)), (((assign32820_e36230 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_temp3__blk816) + (assign32820_e36232 * locals.var_q_temp3__blk816_dn6)), (((assign32820_e36230 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_temp3__blk816) + (assign32820_e36232 * locals.var_q_temp3__blk816_dn7)), (((assign32820_e36230 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_temp3__blk816) + (assign32820_e36232 * locals.var_q_temp3__blk816_dn8)), (((assign32820_e36230 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_temp3__blk816) + (assign32820_e36232 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign32820_e36236;
        locals.var_q_d1_ln__blk835_dn4 = assign32820_e36236_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign32820_e36236_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign32820_e36236_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign32820_e36236_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign32820_e36236_d_n9;

        let (assign32830_e36271, assign32830_e36271_d_n4, assign32830_e36271_d_n6, assign32830_e36271_d_n7, assign32830_e36271_d_n8, assign32830_e36271_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1123 == 0.0)) && (locals.var_guard1124 == 0.0)) {
        let assign32830_e36245: f64 = (-0.5);
        let assign32830_e36247: f64 = (assign32830_e36245 * locals.var_q_d2_qsq__blk827);
        let assign32830_e36249: f64 = (assign32830_e36247 * locals.var_q_temp3__blk816);
        let assign32830_e36252: f64 = (0.25 * 0.0055555555556);
        let assign32830_e36254: f64 = (assign32830_e36252 * locals.var_q_d1_qsq__blk826);
        let assign32830_e36256: f64 = (assign32830_e36254 * locals.var_q_d1_qsq__blk826);
        let assign32830_e36260: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign32830_e36264: f64 = (0.075 * locals.var_q_qsq__blk825);
        let assign32830_e36265: f64 = (2.0 - assign32830_e36264);
        let assign32830_e36266: f64 = (assign32830_e36260 * assign32830_e36265);
        let assign32830_e36267: f64 = (1.0 - assign32830_e36266);
        let assign32830_e36268: f64 = (assign32830_e36256 * assign32830_e36267);
        let assign32830_e36269: f64 = (assign32830_e36249 + assign32830_e36268);
        (assign32830_e36269, ((((assign32830_e36245 * locals.var_q_d2_qsq__blk827_dn4) * locals.var_q_temp3__blk816) + (assign32830_e36247 * locals.var_q_temp3__blk816_dn4)) + (((((assign32830_e36252 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_d1_qsq__blk826) + (assign32830_e36254 * locals.var_q_d1_qsq__blk826_dn4)) * assign32830_e36267) + (assign32830_e36256 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign32830_e36265) + (assign32830_e36260 * (-(0.075 * locals.var_q_qsq__blk825_dn4)))))))), ((((assign32830_e36245 * locals.var_q_d2_qsq__blk827_dn6) * locals.var_q_temp3__blk816) + (assign32830_e36247 * locals.var_q_temp3__blk816_dn6)) + (((((assign32830_e36252 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_d1_qsq__blk826) + (assign32830_e36254 * locals.var_q_d1_qsq__blk826_dn6)) * assign32830_e36267) + (assign32830_e36256 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign32830_e36265) + (assign32830_e36260 * (-(0.075 * locals.var_q_qsq__blk825_dn6)))))))), ((((assign32830_e36245 * locals.var_q_d2_qsq__blk827_dn7) * locals.var_q_temp3__blk816) + (assign32830_e36247 * locals.var_q_temp3__blk816_dn7)) + (((((assign32830_e36252 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_d1_qsq__blk826) + (assign32830_e36254 * locals.var_q_d1_qsq__blk826_dn7)) * assign32830_e36267) + (assign32830_e36256 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign32830_e36265) + (assign32830_e36260 * (-(0.075 * locals.var_q_qsq__blk825_dn7)))))))), ((((assign32830_e36245 * locals.var_q_d2_qsq__blk827_dn8) * locals.var_q_temp3__blk816) + (assign32830_e36247 * locals.var_q_temp3__blk816_dn8)) + (((((assign32830_e36252 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_d1_qsq__blk826) + (assign32830_e36254 * locals.var_q_d1_qsq__blk826_dn8)) * assign32830_e36267) + (assign32830_e36256 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign32830_e36265) + (assign32830_e36260 * (-(0.075 * locals.var_q_qsq__blk825_dn8)))))))), ((((assign32830_e36245 * locals.var_q_d2_qsq__blk827_dn9) * locals.var_q_temp3__blk816) + (assign32830_e36247 * locals.var_q_temp3__blk816_dn9)) + (((((assign32830_e36252 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_d1_qsq__blk826) + (assign32830_e36254 * locals.var_q_d1_qsq__blk826_dn9)) * assign32830_e36267) + (assign32830_e36256 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign32830_e36265) + (assign32830_e36260 * (-(0.075 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign32830_e36271;
        locals.var_q_d2_ln__blk836_dn4 = assign32830_e36271_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign32830_e36271_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign32830_e36271_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign32830_e36271_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign32830_e36271_d_n9;

        let assign32840_e36274: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1125 = assign32840_e36274;

        let (assign32850_e36290, assign32850_e36290_d_n4, assign32850_e36290_d_n6, assign32850_e36290_d_n7, assign32850_e36290_d_n8, assign32850_e36290_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1125 != 0.0)) {
        let assign32850_e36280: f64 = (4.0 * locals.var_q_qsq__blk825);
        let assign32850_e36285: f64 = (2.0 - locals.var_q_invexpq__blk831);
        let assign32850_e36286: f64 = (locals.var_q_invexpq__blk831 * assign32850_e36285);
        let assign32850_e36287: f64 = (1.0 - assign32850_e36286);
        let assign32850_e36288: f64 = (assign32850_e36280 / assign32850_e36287);
        (assign32850_e36288, ((((4.0 * locals.var_q_qsq__blk825_dn4) * assign32850_e36287) - (assign32850_e36280 * (-((locals.var_q_invexpq__blk831_dn4 * assign32850_e36285) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign32850_e36287 * assign32850_e36287)), ((((4.0 * locals.var_q_qsq__blk825_dn6) * assign32850_e36287) - (assign32850_e36280 * (-((locals.var_q_invexpq__blk831_dn6 * assign32850_e36285) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign32850_e36287 * assign32850_e36287)), ((((4.0 * locals.var_q_qsq__blk825_dn7) * assign32850_e36287) - (assign32850_e36280 * (-((locals.var_q_invexpq__blk831_dn7 * assign32850_e36285) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign32850_e36287 * assign32850_e36287)), ((((4.0 * locals.var_q_qsq__blk825_dn8) * assign32850_e36287) - (assign32850_e36280 * (-((locals.var_q_invexpq__blk831_dn8 * assign32850_e36285) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign32850_e36287 * assign32850_e36287)), ((((4.0 * locals.var_q_qsq__blk825_dn9) * assign32850_e36287) - (assign32850_e36280 * (-((locals.var_q_invexpq__blk831_dn9 * assign32850_e36285) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign32850_e36287 * assign32850_e36287)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign32850_e36290;
        locals.var_q_temp2__blk815_dn4 = assign32850_e36290_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign32850_e36290_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign32850_e36290_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign32850_e36290_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign32850_e36290_d_n9;

        let (assign32860_e36298, assign32860_e36298_d_n4, assign32860_e36298_d_n6, assign32860_e36298_d_n7, assign32860_e36298_d_n8, assign32860_e36298_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1125 != 0.0)) {
        let assign32860_e36296: f64 = (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831);
        (assign32860_e36296, ((locals.var_q_temp2__blk815_dn4 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn4)), ((locals.var_q_temp2__blk815_dn6 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn6)), ((locals.var_q_temp2__blk815_dn7 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn7)), ((locals.var_q_temp2__blk815_dn8 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn8)), ((locals.var_q_temp2__blk815_dn9 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn9)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign32860_e36298;
        locals.var_q_sh_term__blk833_dn4 = assign32860_e36298_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign32860_e36298_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign32860_e36298_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign32860_e36298_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign32860_e36298_d_n9;

        let (assign32870_e36307, assign32870_e36307_d_n4, assign32870_e36307_d_n6, assign32870_e36307_d_n7, assign32870_e36307_d_n8, assign32870_e36307_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1125 != 0.0)) {
        let assign32870_e36303: f64 = (locals.var_q_temp2__blk815).ln();
        let assign32870_e36305: f64 = (assign32870_e36303 - locals.var_q_rac_qsq__blk828);
        (assign32870_e36305, ((locals.var_q_temp2__blk815_dn4 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn4), ((locals.var_q_temp2__blk815_dn6 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn6), ((locals.var_q_temp2__blk815_dn7 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn7), ((locals.var_q_temp2__blk815_dn8 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn8), ((locals.var_q_temp2__blk815_dn9 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn9),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign32870_e36307;
        locals.var_q_ln_term__blk834_dn4 = assign32870_e36307_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign32870_e36307_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign32870_e36307_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign32870_e36307_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign32870_e36307_d_n9;

        let assign32880_e36310: f64 = (-0.005);
        let assign32880_e36311: f64 = if locals.var_q_qsq__blk825 < assign32880_e36310 { 1.0 } else { 0.0 };
        locals.var_guard1126 = assign32880_e36311;

        let (assign32890_e36323, assign32890_e36323_d_n4, assign32890_e36323_d_n6, assign32890_e36323_d_n7, assign32890_e36323_d_n8, assign32890_e36323_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1125 == 0.0)) && (locals.var_guard1126 != 0.0)) {
        let assign32890_e36320: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign32890_e36321: f64 = (assign32890_e36320).sin();
        (assign32890_e36321, ((assign32890_e36320).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign32890_e36320).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign32890_e36320).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign32890_e36320).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign32890_e36320).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign32890_e36323;
        locals.var_q_temp2__blk815_dn4 = assign32890_e36323_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign32890_e36323_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign32890_e36323_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign32890_e36323_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign32890_e36323_d_n9;

        let (assign32900_e36337, assign32900_e36337_d_n4, assign32900_e36337_d_n6, assign32900_e36337_d_n7, assign32900_e36337_d_n8, assign32900_e36337_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1125 == 0.0)) && (locals.var_guard1126 != 0.0)) {
        let assign32900_e36331: f64 = (-locals.var_q_qsq__blk825);
        let assign32900_e36334: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
        let assign32900_e36335: f64 = (assign32900_e36331 / assign32900_e36334);
        (assign32900_e36335, ((((-locals.var_q_qsq__blk825_dn4) * assign32900_e36334) - (assign32900_e36331 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign32900_e36334 * assign32900_e36334)), ((((-locals.var_q_qsq__blk825_dn6) * assign32900_e36334) - (assign32900_e36331 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign32900_e36334 * assign32900_e36334)), ((((-locals.var_q_qsq__blk825_dn7) * assign32900_e36334) - (assign32900_e36331 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign32900_e36334 * assign32900_e36334)), ((((-locals.var_q_qsq__blk825_dn8) * assign32900_e36334) - (assign32900_e36331 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign32900_e36334 * assign32900_e36334)), ((((-locals.var_q_qsq__blk825_dn9) * assign32900_e36334) - (assign32900_e36331 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign32900_e36334 * assign32900_e36334)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign32900_e36337;
        locals.var_q_sh_term__blk833_dn4 = assign32900_e36337_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign32900_e36337_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign32900_e36337_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign32900_e36337_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign32900_e36337_d_n9;

        let (assign32910_e36347, assign32910_e36347_d_n4, assign32910_e36347_d_n6, assign32910_e36347_d_n7, assign32910_e36347_d_n8, assign32910_e36347_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1125 == 0.0)) && (locals.var_guard1126 != 0.0)) {
        let assign32910_e36345: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign32910_e36345, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign32910_e36347;
        locals.var_q_ln_term__blk834_dn4 = assign32910_e36347_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign32910_e36347_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign32910_e36347_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign32910_e36347_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign32910_e36347_d_n9;

        let (assign32920_e36373, assign32920_e36373_d_n4, assign32920_e36373_d_n6, assign32920_e36373_d_n7, assign32920_e36373_d_n8, assign32920_e36373_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1125 == 0.0)) && (locals.var_guard1126 == 0.0)) {
        let assign32920_e36358: f64 = (locals.var_q_qsq__blk825 * 0.3333333333333);
        let assign32920_e36362: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign32920_e36366: f64 = (0.0396825396825397 * locals.var_q_qsq__blk825);
        let assign32920_e36367: f64 = (1.0 - assign32920_e36366);
        let assign32920_e36368: f64 = (assign32920_e36362 * assign32920_e36367);
        let assign32920_e36369: f64 = (1.0 - assign32920_e36368);
        let assign32920_e36370: f64 = (assign32920_e36358 * assign32920_e36369);
        let assign32920_e36371: f64 = (4.0 - assign32920_e36370);
        (assign32920_e36371, (-(((locals.var_q_qsq__blk825_dn4 * 0.3333333333333) * assign32920_e36369) + (assign32920_e36358 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign32920_e36367) + (assign32920_e36362 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn4)))))))), (-(((locals.var_q_qsq__blk825_dn6 * 0.3333333333333) * assign32920_e36369) + (assign32920_e36358 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign32920_e36367) + (assign32920_e36362 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn6)))))))), (-(((locals.var_q_qsq__blk825_dn7 * 0.3333333333333) * assign32920_e36369) + (assign32920_e36358 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign32920_e36367) + (assign32920_e36362 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn7)))))))), (-(((locals.var_q_qsq__blk825_dn8 * 0.3333333333333) * assign32920_e36369) + (assign32920_e36358 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign32920_e36367) + (assign32920_e36362 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn8)))))))), (-(((locals.var_q_qsq__blk825_dn9 * 0.3333333333333) * assign32920_e36369) + (assign32920_e36358 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign32920_e36367) + (assign32920_e36362 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign32920_e36373;
        locals.var_q_sh_term__blk833_dn4 = assign32920_e36373_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign32920_e36373_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign32920_e36373_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign32920_e36373_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign32920_e36373_d_n9;

        let (assign32930_e36384, assign32930_e36384_d_n4, assign32930_e36384_d_n6, assign32930_e36384_d_n7, assign32930_e36384_d_n8, assign32930_e36384_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1125 == 0.0)) && (locals.var_guard1126 == 0.0)) {
        let assign32930_e36382: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign32930_e36382, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign32930_e36384;
        locals.var_q_ln_term__blk834_dn4 = assign32930_e36384_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign32930_e36384_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign32930_e36384_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign32930_e36384_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign32930_e36384_d_n9;

        let assign32940_e36387: f64 = (1.01 * locals.var_q_k1q1__blk823);
        let assign32940_e36389: f64 = (assign32940_e36387 + locals.var_q_qcoth__blk829);
        let assign32940_e36391: f64 = if assign32940_e36389 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1127 = assign32940_e36391;

        let (assign32950_e36399, assign32950_e36399_d_n4, assign32950_e36399_d_n6, assign32950_e36399_d_n7, assign32950_e36399_d_n8, assign32950_e36399_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1127 != 0.0)) {
        let assign32950_e36397: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_qcoth__blk829);
        (assign32950_e36397, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_qcoth__blk829_dn9),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign32950_e36399;
        locals.var_q_expnum__blk837_dn4 = assign32950_e36399_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign32950_e36399_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign32950_e36399_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign32950_e36399_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign32950_e36399_d_n9;

        let (assign32960_e36407, assign32960_e36407_d_n4, assign32960_e36407_d_n6, assign32960_e36407_d_n7, assign32960_e36407_d_n8, assign32960_e36407_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1127 != 0.0)) {
        let assign32960_e36405: f64 = (locals.var_k1__blk932 + locals.var_q_d1_qcoth__blk830);
        (assign32960_e36405, (locals.var_k1__blk932_dn4 + locals.var_q_d1_qcoth__blk830_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_d1_qcoth__blk830_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_d1_qcoth__blk830_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_d1_qcoth__blk830_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_d1_qcoth__blk830_dn9),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign32960_e36407;
        locals.var_q_d1_expnum__blk838_dn4 = assign32960_e36407_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign32960_e36407_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign32960_e36407_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign32960_e36407_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign32960_e36407_d_n9;

        let (assign32970_e36413, assign32970_e36413_d_n4, assign32970_e36413_d_n6, assign32970_e36413_d_n7, assign32970_e36413_d_n8, assign32970_e36413_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1127 != 0.0)) {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign32970_e36413;
        locals.var_q_d2_expnum__blk839_dn4 = assign32970_e36413_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign32970_e36413_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign32970_e36413_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign32970_e36413_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign32970_e36413_d_n9;

        let (assign32980_e36424, assign32980_e36424_d_n4, assign32980_e36424_d_n6, assign32980_e36424_d_n7, assign32980_e36424_d_n8, assign32980_e36424_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1127 == 0.0)) {
        let assign32980_e36421: f64 = (locals.var_q_k1q1__blk823 - locals.var_q_qcoth__blk829);
        let assign32980_e36422: f64 = (1.0 / assign32980_e36421);
        (assign32980_e36422, (-((locals.var_q_k1q1__blk823_dn4 - locals.var_q_qcoth__blk829_dn4) / (assign32980_e36421 * assign32980_e36421))), (-((locals.var_q_k1q1__blk823_dn6 - locals.var_q_qcoth__blk829_dn6) / (assign32980_e36421 * assign32980_e36421))), (-((locals.var_q_k1q1__blk823_dn7 - locals.var_q_qcoth__blk829_dn7) / (assign32980_e36421 * assign32980_e36421))), (-((locals.var_q_k1q1__blk823_dn8 - locals.var_q_qcoth__blk829_dn8) / (assign32980_e36421 * assign32980_e36421))), (-((locals.var_q_k1q1__blk823_dn9 - locals.var_q_qcoth__blk829_dn9) / (assign32980_e36421 * assign32980_e36421))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign32980_e36424;
        locals.var_q_temp2__blk815_dn4 = assign32980_e36424_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign32980_e36424_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign32980_e36424_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign32980_e36424_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign32980_e36424_d_n9;

        let (assign32990_e36433, assign32990_e36433_d_n4, assign32990_e36433_d_n6, assign32990_e36433_d_n7, assign32990_e36433_d_n8, assign32990_e36433_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1127 == 0.0)) {
        let assign32990_e36431: f64 = (locals.var_q_d1_qcoth__blk830 - locals.var_k1__blk932);
        (assign32990_e36431, (locals.var_q_d1_qcoth__blk830_dn4 - locals.var_k1__blk932_dn4), (locals.var_q_d1_qcoth__blk830_dn6 - locals.var_k1__blk932_dn6), (locals.var_q_d1_qcoth__blk830_dn7 - locals.var_k1__blk932_dn7), (locals.var_q_d1_qcoth__blk830_dn8 - locals.var_k1__blk932_dn8), (locals.var_q_d1_qcoth__blk830_dn9 - locals.var_k1__blk932_dn9),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign32990_e36433;
        locals.var_q_temp3__blk816_dn4 = assign32990_e36433_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign32990_e36433_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign32990_e36433_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign32990_e36433_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign32990_e36433_d_n9;

        let (assign33000_e36444, assign33000_e36444_d_n4, assign33000_e36444_d_n6, assign33000_e36444_d_n7, assign33000_e36444_d_n8, assign33000_e36444_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1127 == 0.0)) {
        let assign33000_e36440: f64 = (locals.var_q_aexp__blk824 - locals.var_q_sh_term__blk833);
        let assign33000_e36442: f64 = (assign33000_e36440 * locals.var_q_temp2__blk815);
        (assign33000_e36442, (((locals.var_q_aexp__blk824_dn4 - locals.var_q_sh_term__blk833_dn4) * locals.var_q_temp2__blk815) + (assign33000_e36440 * locals.var_q_temp2__blk815_dn4)), (((locals.var_q_aexp__blk824_dn6 - locals.var_q_sh_term__blk833_dn6) * locals.var_q_temp2__blk815) + (assign33000_e36440 * locals.var_q_temp2__blk815_dn6)), (((locals.var_q_aexp__blk824_dn7 - locals.var_q_sh_term__blk833_dn7) * locals.var_q_temp2__blk815) + (assign33000_e36440 * locals.var_q_temp2__blk815_dn7)), (((locals.var_q_aexp__blk824_dn8 - locals.var_q_sh_term__blk833_dn8) * locals.var_q_temp2__blk815) + (assign33000_e36440 * locals.var_q_temp2__blk815_dn8)), (((locals.var_q_aexp__blk824_dn9 - locals.var_q_sh_term__blk833_dn9) * locals.var_q_temp2__blk815) + (assign33000_e36440 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign33000_e36444;
        locals.var_q_expnum__blk837_dn4 = assign33000_e36444_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign33000_e36444_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign33000_e36444_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign33000_e36444_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign33000_e36444_d_n9;

        let (assign33010_e36461, assign33010_e36461_d_n4, assign33010_e36461_d_n6, assign33010_e36461_d_n7, assign33010_e36461_d_n8, assign33010_e36461_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1127 == 0.0)) {
        let assign33010_e36451: f64 = (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837);
        let assign33010_e36453: f64 = (assign33010_e36451 - locals.var_q_aexp__blk824);
        let assign33010_e36456: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833);
        let assign33010_e36457: f64 = (assign33010_e36453 - assign33010_e36456);
        let assign33010_e36459: f64 = (assign33010_e36457 * locals.var_q_temp2__blk815);
        (assign33010_e36459, ((((((locals.var_q_temp3__blk816_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4) - ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign33010_e36457 * locals.var_q_temp2__blk815_dn4)), ((((((locals.var_q_temp3__blk816_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6) - ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign33010_e36457 * locals.var_q_temp2__blk815_dn6)), ((((((locals.var_q_temp3__blk816_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7) - ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign33010_e36457 * locals.var_q_temp2__blk815_dn7)), ((((((locals.var_q_temp3__blk816_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8) - ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign33010_e36457 * locals.var_q_temp2__blk815_dn8)), ((((((locals.var_q_temp3__blk816_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9) - ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign33010_e36457 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign33010_e36461;
        locals.var_q_d1_expnum__blk838_dn4 = assign33010_e36461_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign33010_e36461_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign33010_e36461_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign33010_e36461_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign33010_e36461_d_n9;

        let (assign33020_e36488, assign33020_e36488_d_n4, assign33020_e36488_d_n6, assign33020_e36488_d_n7, assign33020_e36488_d_n8, assign33020_e36488_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1127 == 0.0)) {
        let assign33020_e36468: f64 = (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837);
        let assign33020_e36471: f64 = (2.0 * locals.var_q_temp3__blk816);
        let assign33020_e36473: f64 = (assign33020_e36471 * locals.var_q_d1_expnum__blk838);
        let assign33020_e36474: f64 = (assign33020_e36468 + assign33020_e36473);
        let assign33020_e36476: f64 = (assign33020_e36474 + locals.var_q_aexp__blk824);
        let assign33020_e36480: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835);
        let assign33020_e36481: f64 = (locals.var_q_d2_ln__blk836 + assign33020_e36480);
        let assign33020_e36483: f64 = (assign33020_e36481 * locals.var_q_sh_term__blk833);
        let assign33020_e36484: f64 = (assign33020_e36476 - assign33020_e36483);
        let assign33020_e36486: f64 = (assign33020_e36484 * locals.var_q_temp2__blk815);
        (assign33020_e36486, (((((((locals.var_q_d2_qcoth__blk832_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_temp3__blk816_dn4) * locals.var_q_d1_expnum__blk838) + (assign33020_e36471 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4) - (((locals.var_q_d2_ln__blk836_dn4 + ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn4))) * locals.var_q_sh_term__blk833) + (assign33020_e36481 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign33020_e36484 * locals.var_q_temp2__blk815_dn4)), (((((((locals.var_q_d2_qcoth__blk832_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_temp3__blk816_dn6) * locals.var_q_d1_expnum__blk838) + (assign33020_e36471 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6) - (((locals.var_q_d2_ln__blk836_dn6 + ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn6))) * locals.var_q_sh_term__blk833) + (assign33020_e36481 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign33020_e36484 * locals.var_q_temp2__blk815_dn6)), (((((((locals.var_q_d2_qcoth__blk832_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_temp3__blk816_dn7) * locals.var_q_d1_expnum__blk838) + (assign33020_e36471 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7) - (((locals.var_q_d2_ln__blk836_dn7 + ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn7))) * locals.var_q_sh_term__blk833) + (assign33020_e36481 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign33020_e36484 * locals.var_q_temp2__blk815_dn7)), (((((((locals.var_q_d2_qcoth__blk832_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_temp3__blk816_dn8) * locals.var_q_d1_expnum__blk838) + (assign33020_e36471 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8) - (((locals.var_q_d2_ln__blk836_dn8 + ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn8))) * locals.var_q_sh_term__blk833) + (assign33020_e36481 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign33020_e36484 * locals.var_q_temp2__blk815_dn8)), (((((((locals.var_q_d2_qcoth__blk832_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_temp3__blk816_dn9) * locals.var_q_d1_expnum__blk838) + (assign33020_e36471 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9) - (((locals.var_q_d2_ln__blk836_dn9 + ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn9))) * locals.var_q_sh_term__blk833) + (assign33020_e36481 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign33020_e36484 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign33020_e36488;
        locals.var_q_d2_expnum__blk839_dn4 = assign33020_e36488_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign33020_e36488_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign33020_e36488_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign33020_e36488_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign33020_e36488_d_n9;

        let assign33030_e36491: f64 = if locals.var_q_expnum__blk837 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1128 = assign33030_e36491;

        let (assign33040_e36498, assign33040_e36498_d_n4, assign33040_e36498_d_n6, assign33040_e36498_d_n7, assign33040_e36498_d_n8, assign33040_e36498_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1128 != 0.0)) {
        let assign33040_e36496: f64 = (locals.var_q_expnum__blk837).ln();
        (assign33040_e36496, (locals.var_q_expnum__blk837_dn4 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn6 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn7 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn8 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn9 / locals.var_q_expnum__blk837),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign33040_e36498;
        locals.var_q_lnexpnum__blk840_dn4 = assign33040_e36498_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign33040_e36498_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign33040_e36498_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign33040_e36498_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign33040_e36498_d_n9;

        let (assign33050_e36506, assign33050_e36506_d_n4, assign33050_e36506_d_n6, assign33050_e36506_d_n7, assign33050_e36506_d_n8, assign33050_e36506_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1128 != 0.0)) {
        let assign33050_e36504: f64 = (1.0 / locals.var_q_expnum__blk837);
        (assign33050_e36504, (-(locals.var_q_expnum__blk837_dn4 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn6 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn7 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn8 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn9 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign33050_e36506;
        locals.var_q_temp1__blk814_dn4 = assign33050_e36506_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign33050_e36506_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign33050_e36506_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign33050_e36506_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign33050_e36506_d_n9;

    }

    pub(super) fn stamp_transient_block_89(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign33060_e36514, assign33060_e36514_d_n4, assign33060_e36514_d_n6, assign33060_e36514_d_n7, assign33060_e36514_d_n8, assign33060_e36514_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1128 != 0.0)) {
        let assign33060_e36512: f64 = (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814);
        (assign33060_e36512, ((locals.var_q_d1_expnum__blk838_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_expnum__blk838_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_expnum__blk838_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_expnum__blk838_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_expnum__blk838_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign33060_e36514;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign33060_e36514_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign33060_e36514_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign33060_e36514_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign33060_e36514_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign33060_e36514_d_n9;

        let (assign33070_e36526, assign33070_e36526_d_n4, assign33070_e36526_d_n6, assign33070_e36526_d_n7, assign33070_e36526_d_n8, assign33070_e36526_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1128 != 0.0)) {
        let assign33070_e36520: f64 = (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814);
        let assign33070_e36523: f64 = (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841);
        let assign33070_e36524: f64 = (assign33070_e36520 - assign33070_e36523);
        (assign33070_e36524, (((locals.var_q_d2_expnum__blk839_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn4)) - ((locals.var_q_d1_lnexpnum__blk841_dn4 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn4))), (((locals.var_q_d2_expnum__blk839_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn6)) - ((locals.var_q_d1_lnexpnum__blk841_dn6 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn6))), (((locals.var_q_d2_expnum__blk839_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn7)) - ((locals.var_q_d1_lnexpnum__blk841_dn7 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn7))), (((locals.var_q_d2_expnum__blk839_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn8)) - ((locals.var_q_d1_lnexpnum__blk841_dn8 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn8))), (((locals.var_q_d2_expnum__blk839_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn9)) - ((locals.var_q_d1_lnexpnum__blk841_dn9 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign33070_e36526;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign33070_e36526_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign33070_e36526_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign33070_e36526_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign33070_e36526_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign33070_e36526_d_n9;

        let (assign33080_e36539, assign33080_e36539_d_n4, assign33080_e36539_d_n6, assign33080_e36539_d_n7, assign33080_e36539_d_n8, assign33080_e36539_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1128 == 0.0)) {
        let assign33080_e36533: f64 = (locals.var_q_k1q1__blk823 + 0.6931471805599);
        let assign33080_e36535: f64 = (-locals.var_q_k1q1__blk823);
        let assign33080_e36536: f64 = (assign33080_e36535).ln();
        let assign33080_e36537: f64 = (assign33080_e36533 + assign33080_e36536);
        (assign33080_e36537, (locals.var_q_k1q1__blk823_dn4 + ((-locals.var_q_k1q1__blk823_dn4) / assign33080_e36535)), (locals.var_q_k1q1__blk823_dn6 + ((-locals.var_q_k1q1__blk823_dn6) / assign33080_e36535)), (locals.var_q_k1q1__blk823_dn7 + ((-locals.var_q_k1q1__blk823_dn7) / assign33080_e36535)), (locals.var_q_k1q1__blk823_dn8 + ((-locals.var_q_k1q1__blk823_dn8) / assign33080_e36535)), (locals.var_q_k1q1__blk823_dn9 + ((-locals.var_q_k1q1__blk823_dn9) / assign33080_e36535)),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign33080_e36539;
        locals.var_q_lnexpnum__blk840_dn4 = assign33080_e36539_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign33080_e36539_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign33080_e36539_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign33080_e36539_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign33080_e36539_d_n9;

        let (assign33090_e36548, assign33090_e36548_d_n4, assign33090_e36548_d_n6, assign33090_e36548_d_n7, assign33090_e36548_d_n8, assign33090_e36548_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1128 == 0.0)) {
        let assign33090_e36546: f64 = (1.0 / locals.var_q1s__blk937);
        (assign33090_e36546, (-(locals.var_q1s__blk937_dn4 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))), (-(locals.var_q1s__blk937_dn6 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))), (-(locals.var_q1s__blk937_dn7 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))), (-(locals.var_q1s__blk937_dn8 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))), (-(locals.var_q1s__blk937_dn9 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign33090_e36548;
        locals.var_q_temp1__blk814_dn4 = assign33090_e36548_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign33090_e36548_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign33090_e36548_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign33090_e36548_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign33090_e36548_d_n9;

        let (assign33100_e36557, assign33100_e36557_d_n4, assign33100_e36557_d_n6, assign33100_e36557_d_n7, assign33100_e36557_d_n8, assign33100_e36557_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1128 == 0.0)) {
        let assign33100_e36555: f64 = (locals.var_k1__blk932 + locals.var_q_temp1__blk814);
        (assign33100_e36555, (locals.var_k1__blk932_dn4 + locals.var_q_temp1__blk814_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_temp1__blk814_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_temp1__blk814_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_temp1__blk814_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_temp1__blk814_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign33100_e36557;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign33100_e36557_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign33100_e36557_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign33100_e36557_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign33100_e36557_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign33100_e36557_d_n9;

        let (assign33110_e36567, assign33110_e36567_d_n4, assign33110_e36567_d_n6, assign33110_e36567_d_n7, assign33110_e36567_d_n8, assign33110_e36567_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1128 == 0.0)) {
        let assign33110_e36563: f64 = (-locals.var_q_temp1__blk814);
        let assign33110_e36565: f64 = (assign33110_e36563 * locals.var_q_temp1__blk814);
        (assign33110_e36565, (((-locals.var_q_temp1__blk814_dn4) * locals.var_q_temp1__blk814) + (assign33110_e36563 * locals.var_q_temp1__blk814_dn4)), (((-locals.var_q_temp1__blk814_dn6) * locals.var_q_temp1__blk814) + (assign33110_e36563 * locals.var_q_temp1__blk814_dn6)), (((-locals.var_q_temp1__blk814_dn7) * locals.var_q_temp1__blk814) + (assign33110_e36563 * locals.var_q_temp1__blk814_dn7)), (((-locals.var_q_temp1__blk814_dn8) * locals.var_q_temp1__blk814) + (assign33110_e36563 * locals.var_q_temp1__blk814_dn8)), (((-locals.var_q_temp1__blk814_dn9) * locals.var_q_temp1__blk814) + (assign33110_e36563 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign33110_e36567;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign33110_e36567_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign33110_e36567_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign33110_e36567_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign33110_e36567_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign33110_e36567_d_n9;

        let (assign33120_e36581, assign33120_e36581_d_n4, assign33120_e36581_d_n6, assign33120_e36581_d_n7, assign33120_e36581_d_n8, assign33120_e36581_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign33120_e36571: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
        let assign33120_e36573: f64 = (assign33120_e36571 + locals.var_q1s__blk937);
        let assign33120_e36576: f64 = (2.0 * locals.var_q_lnexpnum__blk840);
        let assign33120_e36577: f64 = (assign33120_e36573 + assign33120_e36576);
        let assign33120_e36579: f64 = (assign33120_e36577 - locals.var_q_ln_term__blk834);
        (assign33120_e36579, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1s__blk937_dn4) + (2.0 * locals.var_q_lnexpnum__blk840_dn4)) - locals.var_q_ln_term__blk834_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1s__blk937_dn6) + (2.0 * locals.var_q_lnexpnum__blk840_dn6)) - locals.var_q_ln_term__blk834_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1s__blk937_dn7) + (2.0 * locals.var_q_lnexpnum__blk840_dn7)) - locals.var_q_ln_term__blk834_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1s__blk937_dn8) + (2.0 * locals.var_q_lnexpnum__blk840_dn8)) - locals.var_q_ln_term__blk834_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1s__blk937_dn9) + (2.0 * locals.var_q_lnexpnum__blk840_dn9)) - locals.var_q_ln_term__blk834_dn9),)
    } else {
        (locals.var_q_q2_int__blk843, locals.var_q_q2_int__blk843_dn4, locals.var_q_q2_int__blk843_dn6, locals.var_q_q2_int__blk843_dn7, locals.var_q_q2_int__blk843_dn8, locals.var_q_q2_int__blk843_dn9,)
    }
};
        locals.var_q_q2_int__blk843 = assign33120_e36581;
        locals.var_q_q2_int__blk843_dn4 = assign33120_e36581_d_n4;
        locals.var_q_q2_int__blk843_dn6 = assign33120_e36581_d_n6;
        locals.var_q_q2_int__blk843_dn7 = assign33120_e36581_d_n7;
        locals.var_q_q2_int__blk843_dn8 = assign33120_e36581_d_n8;
        locals.var_q_q2_int__blk843_dn9 = assign33120_e36581_d_n9;

        let (assign33130_e36591, assign33130_e36591_d_n4, assign33130_e36591_d_n6, assign33130_e36591_d_n7, assign33130_e36591_d_n8, assign33130_e36591_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign33130_e36586: f64 = (2.0 * locals.var_q_d1_lnexpnum__blk841);
        let assign33130_e36587: f64 = (1.0 + assign33130_e36586);
        let assign33130_e36589: f64 = (assign33130_e36587 - locals.var_q_d1_ln__blk835);
        (assign33130_e36589, ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn4) - locals.var_q_d1_ln__blk835_dn4), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn6) - locals.var_q_d1_ln__blk835_dn6), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn7) - locals.var_q_d1_ln__blk835_dn7), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn8) - locals.var_q_d1_ln__blk835_dn8), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn9) - locals.var_q_d1_ln__blk835_dn9),)
    } else {
        (locals.var_q_d1_q2__blk844, locals.var_q_d1_q2__blk844_dn4, locals.var_q_d1_q2__blk844_dn6, locals.var_q_d1_q2__blk844_dn7, locals.var_q_d1_q2__blk844_dn8, locals.var_q_d1_q2__blk844_dn9,)
    }
};
        locals.var_q_d1_q2__blk844 = assign33130_e36591;
        locals.var_q_d1_q2__blk844_dn4 = assign33130_e36591_d_n4;
        locals.var_q_d1_q2__blk844_dn6 = assign33130_e36591_d_n6;
        locals.var_q_d1_q2__blk844_dn7 = assign33130_e36591_d_n7;
        locals.var_q_d1_q2__blk844_dn8 = assign33130_e36591_d_n8;
        locals.var_q_d1_q2__blk844_dn9 = assign33130_e36591_d_n9;

        let (assign33140_e36599, assign33140_e36599_d_n4, assign33140_e36599_d_n6, assign33140_e36599_d_n7, assign33140_e36599_d_n8, assign33140_e36599_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign33140_e36595: f64 = (2.0 * locals.var_q_d2_lnexpnum__blk842);
        let assign33140_e36597: f64 = (assign33140_e36595 - locals.var_q_d2_ln__blk836);
        (assign33140_e36597, ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn4) - locals.var_q_d2_ln__blk836_dn4), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn6) - locals.var_q_d2_ln__blk836_dn6), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn7) - locals.var_q_d2_ln__blk836_dn7), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn8) - locals.var_q_d2_ln__blk836_dn8), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn9) - locals.var_q_d2_ln__blk836_dn9),)
    } else {
        (locals.var_q_d2_q2__blk845, locals.var_q_d2_q2__blk845_dn4, locals.var_q_d2_q2__blk845_dn6, locals.var_q_d2_q2__blk845_dn7, locals.var_q_d2_q2__blk845_dn8, locals.var_q_d2_q2__blk845_dn9,)
    }
};
        locals.var_q_d2_q2__blk845 = assign33140_e36599;
        locals.var_q_d2_q2__blk845_dn4 = assign33140_e36599_d_n4;
        locals.var_q_d2_q2__blk845_dn6 = assign33140_e36599_d_n6;
        locals.var_q_d2_q2__blk845_dn7 = assign33140_e36599_d_n7;
        locals.var_q_d2_q2__blk845_dn8 = assign33140_e36599_d_n8;
        locals.var_q_d2_q2__blk845_dn9 = assign33140_e36599_d_n9;

        let (assign33150_e36607, assign33150_e36607_d_n4, assign33150_e36607_d_n6, assign33150_e36607_d_n7, assign33150_e36607_d_n8, assign33150_e36607_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign33150_e36604: f64 = (locals.var_k2__blk933 * locals.var_q_q2_int__blk843);
        let assign33150_e36605: f64 = (locals.var_q_k1q1__blk823 + assign33150_e36604);
        (assign33150_e36605, (locals.var_q_k1q1__blk823_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn4))), (locals.var_q_k1q1__blk823_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn6))), (locals.var_q_k1q1__blk823_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn7))), (locals.var_q_k1q1__blk823_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn8))), (locals.var_q_k1q1__blk823_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn9))),)
    } else {
        (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9,)
    }
};
        locals.var_q_qi_int__blk846 = assign33150_e36607;
        locals.var_q_qi_int__blk846_dn4 = assign33150_e36607_d_n4;
        locals.var_q_qi_int__blk846_dn6 = assign33150_e36607_d_n6;
        locals.var_q_qi_int__blk846_dn7 = assign33150_e36607_d_n7;
        locals.var_q_qi_int__blk846_dn8 = assign33150_e36607_d_n8;
        locals.var_q_qi_int__blk846_dn9 = assign33150_e36607_d_n9;

        let (assign33160_e36615, assign33160_e36615_d_n4, assign33160_e36615_d_n6, assign33160_e36615_d_n7, assign33160_e36615_d_n8, assign33160_e36615_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign33160_e36612: f64 = (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844);
        let assign33160_e36613: f64 = (locals.var_k1__blk932 + assign33160_e36612);
        (assign33160_e36613, (locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn4))), (locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn6))), (locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn7))), (locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn8))), (locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn9))),)
    } else {
        (locals.var_q_d1_qi__blk847, locals.var_q_d1_qi__blk847_dn4, locals.var_q_d1_qi__blk847_dn6, locals.var_q_d1_qi__blk847_dn7, locals.var_q_d1_qi__blk847_dn8, locals.var_q_d1_qi__blk847_dn9,)
    }
};
        locals.var_q_d1_qi__blk847 = assign33160_e36615;
        locals.var_q_d1_qi__blk847_dn4 = assign33160_e36615_d_n4;
        locals.var_q_d1_qi__blk847_dn6 = assign33160_e36615_d_n6;
        locals.var_q_d1_qi__blk847_dn7 = assign33160_e36615_d_n7;
        locals.var_q_d1_qi__blk847_dn8 = assign33160_e36615_d_n8;
        locals.var_q_d1_qi__blk847_dn9 = assign33160_e36615_d_n9;

        let (assign33170_e36621, assign33170_e36621_d_n4, assign33170_e36621_d_n6, assign33170_e36621_d_n7, assign33170_e36621_d_n8, assign33170_e36621_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign33170_e36619: f64 = (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845);
        (assign33170_e36619, ((locals.var_k2__blk933_dn4 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn9)),)
    } else {
        (locals.var_q_d2_qi__blk848, locals.var_q_d2_qi__blk848_dn4, locals.var_q_d2_qi__blk848_dn6, locals.var_q_d2_qi__blk848_dn7, locals.var_q_d2_qi__blk848_dn8, locals.var_q_d2_qi__blk848_dn9,)
    }
};
        locals.var_q_d2_qi__blk848 = assign33170_e36621;
        locals.var_q_d2_qi__blk848_dn4 = assign33170_e36621_d_n4;
        locals.var_q_d2_qi__blk848_dn6 = assign33170_e36621_d_n6;
        locals.var_q_d2_qi__blk848_dn7 = assign33170_e36621_d_n7;
        locals.var_q_d2_qi__blk848_dn8 = assign33170_e36621_d_n8;
        locals.var_q_d2_qi__blk848_dn9 = assign33170_e36621_d_n9;

        let (assign33180_e36629, assign33180_e36629_d_n4, assign33180_e36629_d_n6, assign33180_e36629_d_n7, assign33180_e36629_d_n8, assign33180_e36629_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign33180_e36625: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837);
        let assign33180_e36627: f64 = (assign33180_e36625 - locals.var_q_aexp__blk824);
        (assign33180_e36627, (((locals.var_q_qi_int__blk846_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_qi_int__blk846_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_qi_int__blk846_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_qi_int__blk846_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_qi_int__blk846_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9,)
    }
};
        locals.var_q_zero__blk849 = assign33180_e36629;
        locals.var_q_zero__blk849_dn4 = assign33180_e36629_d_n4;
        locals.var_q_zero__blk849_dn6 = assign33180_e36629_d_n6;
        locals.var_q_zero__blk849_dn7 = assign33180_e36629_d_n7;
        locals.var_q_zero__blk849_dn8 = assign33180_e36629_d_n8;
        locals.var_q_zero__blk849_dn9 = assign33180_e36629_d_n9;

        let (assign33190_e36641, assign33190_e36641_d_n4, assign33190_e36641_d_n6, assign33190_e36641_d_n7, assign33190_e36641_d_n8, assign33190_e36641_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign33190_e36633: f64 = (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837);
        let assign33190_e36636: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838);
        let assign33190_e36637: f64 = (assign33190_e36633 + assign33190_e36636);
        let assign33190_e36639: f64 = (assign33190_e36637 + locals.var_q_aexp__blk824);
        (assign33190_e36639, ((((locals.var_q_d1_qi__blk847_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn4)) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4), ((((locals.var_q_d1_qi__blk847_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn6)) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6), ((((locals.var_q_d1_qi__blk847_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn7)) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7), ((((locals.var_q_d1_qi__blk847_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn8)) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8), ((((locals.var_q_d1_qi__blk847_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn9)) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9,)
    }
};
        locals.var_q_d1_zero__blk850 = assign33190_e36641;
        locals.var_q_d1_zero__blk850_dn4 = assign33190_e36641_d_n4;
        locals.var_q_d1_zero__blk850_dn6 = assign33190_e36641_d_n6;
        locals.var_q_d1_zero__blk850_dn7 = assign33190_e36641_d_n7;
        locals.var_q_d1_zero__blk850_dn8 = assign33190_e36641_d_n8;
        locals.var_q_d1_zero__blk850_dn9 = assign33190_e36641_d_n9;

        let (assign33200_e36659, assign33200_e36659_d_n4, assign33200_e36659_d_n6, assign33200_e36659_d_n7, assign33200_e36659_d_n8, assign33200_e36659_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign33200_e36645: f64 = (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837);
        let assign33200_e36648: f64 = (2.0 * locals.var_q_d1_qi__blk847);
        let assign33200_e36650: f64 = (assign33200_e36648 * locals.var_q_d1_expnum__blk838);
        let assign33200_e36651: f64 = (assign33200_e36645 + assign33200_e36650);
        let assign33200_e36654: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839);
        let assign33200_e36655: f64 = (assign33200_e36651 + assign33200_e36654);
        let assign33200_e36657: f64 = (assign33200_e36655 - locals.var_q_aexp__blk824);
        (assign33200_e36657, (((((locals.var_q_d2_qi__blk848_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_d1_qi__blk847_dn4) * locals.var_q_d1_expnum__blk838) + (assign33200_e36648 * locals.var_q_d1_expnum__blk838_dn4))) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn4))) - locals.var_q_aexp__blk824_dn4), (((((locals.var_q_d2_qi__blk848_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_d1_qi__blk847_dn6) * locals.var_q_d1_expnum__blk838) + (assign33200_e36648 * locals.var_q_d1_expnum__blk838_dn6))) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn6))) - locals.var_q_aexp__blk824_dn6), (((((locals.var_q_d2_qi__blk848_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_d1_qi__blk847_dn7) * locals.var_q_d1_expnum__blk838) + (assign33200_e36648 * locals.var_q_d1_expnum__blk838_dn7))) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn7))) - locals.var_q_aexp__blk824_dn7), (((((locals.var_q_d2_qi__blk848_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_d1_qi__blk847_dn8) * locals.var_q_d1_expnum__blk838) + (assign33200_e36648 * locals.var_q_d1_expnum__blk838_dn8))) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn8))) - locals.var_q_aexp__blk824_dn8), (((((locals.var_q_d2_qi__blk848_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_d1_qi__blk847_dn9) * locals.var_q_d1_expnum__blk838) + (assign33200_e36648 * locals.var_q_d1_expnum__blk838_dn9))) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn9))) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_zero__blk851, locals.var_q_d2_zero__blk851_dn4, locals.var_q_d2_zero__blk851_dn6, locals.var_q_d2_zero__blk851_dn7, locals.var_q_d2_zero__blk851_dn8, locals.var_q_d2_zero__blk851_dn9,)
    }
};
        locals.var_q_d2_zero__blk851 = assign33200_e36659;
        locals.var_q_d2_zero__blk851_dn4 = assign33200_e36659_d_n4;
        locals.var_q_d2_zero__blk851_dn6 = assign33200_e36659_d_n6;
        locals.var_q_d2_zero__blk851_dn7 = assign33200_e36659_d_n7;
        locals.var_q_d2_zero__blk851_dn8 = assign33200_e36659_d_n8;
        locals.var_q_d2_zero__blk851_dn9 = assign33200_e36659_d_n9;

        let (assign33210_e36671, assign33210_e36671_d_n4, assign33210_e36671_d_n6, assign33210_e36671_d_n7, assign33210_e36671_d_n8, assign33210_e36671_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign33210_e36663: f64 = (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850);
        let assign33210_e36666: f64 = (0.5 * locals.var_q_zero__blk849);
        let assign33210_e36668: f64 = (assign33210_e36666 * locals.var_q_d2_zero__blk851);
        let assign33210_e36669: f64 = (assign33210_e36663 - assign33210_e36668);
        (assign33210_e36669, (((locals.var_q_d1_zero__blk850_dn4 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn4)) - (((0.5 * locals.var_q_zero__blk849_dn4) * locals.var_q_d2_zero__blk851) + (assign33210_e36666 * locals.var_q_d2_zero__blk851_dn4))), (((locals.var_q_d1_zero__blk850_dn6 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn6)) - (((0.5 * locals.var_q_zero__blk849_dn6) * locals.var_q_d2_zero__blk851) + (assign33210_e36666 * locals.var_q_d2_zero__blk851_dn6))), (((locals.var_q_d1_zero__blk850_dn7 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn7)) - (((0.5 * locals.var_q_zero__blk849_dn7) * locals.var_q_d2_zero__blk851) + (assign33210_e36666 * locals.var_q_d2_zero__blk851_dn7))), (((locals.var_q_d1_zero__blk850_dn8 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn8)) - (((0.5 * locals.var_q_zero__blk849_dn8) * locals.var_q_d2_zero__blk851) + (assign33210_e36666 * locals.var_q_d2_zero__blk851_dn8))), (((locals.var_q_d1_zero__blk850_dn9 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn9)) - (((0.5 * locals.var_q_zero__blk849_dn9) * locals.var_q_d2_zero__blk851) + (assign33210_e36666 * locals.var_q_d2_zero__blk851_dn9))),)
    } else {
        (locals.var_q_temp__blk860, locals.var_q_temp__blk860_dn4, locals.var_q_temp__blk860_dn6, locals.var_q_temp__blk860_dn7, locals.var_q_temp__blk860_dn8, locals.var_q_temp__blk860_dn9,)
    }
};
        locals.var_q_temp__blk860 = assign33210_e36671;
        locals.var_q_temp__blk860_dn4 = assign33210_e36671_d_n4;
        locals.var_q_temp__blk860_dn6 = assign33210_e36671_d_n6;
        locals.var_q_temp__blk860_dn7 = assign33210_e36671_d_n7;
        locals.var_q_temp__blk860_dn8 = assign33210_e36671_d_n8;
        locals.var_q_temp__blk860_dn9 = assign33210_e36671_d_n9;

        let (assign33220_e36686, assign33220_e36686_d_n4, assign33220_e36686_d_n6, assign33220_e36686_d_n7, assign33220_e36686_d_n8, assign33220_e36686_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign33220_e36674: f64 = (-locals.var_q_zero__blk849);
        let assign33220_e36676: f64 = (assign33220_e36674 * locals.var_q_d1_zero__blk850);
        let assign33220_e36678: f64 = (assign33220_e36676 * locals.var_q_temp__blk860);
        let assign33220_e36681: f64 = (locals.var_q_temp__blk860 * locals.var_q_temp__blk860);
        let assign33220_e36683: f64 = (assign33220_e36681 + 1e-200);
        let assign33220_e36684: f64 = (assign33220_e36678 / assign33220_e36683);
        (assign33220_e36684, ((((((((-locals.var_q_zero__blk849_dn4) * locals.var_q_d1_zero__blk850) + (assign33220_e36674 * locals.var_q_d1_zero__blk850_dn4)) * locals.var_q_temp__blk860) + (assign33220_e36676 * locals.var_q_temp__blk860_dn4)) * assign33220_e36683) - (assign33220_e36678 * ((locals.var_q_temp__blk860_dn4 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn4)))) / (assign33220_e36683 * assign33220_e36683)), ((((((((-locals.var_q_zero__blk849_dn6) * locals.var_q_d1_zero__blk850) + (assign33220_e36674 * locals.var_q_d1_zero__blk850_dn6)) * locals.var_q_temp__blk860) + (assign33220_e36676 * locals.var_q_temp__blk860_dn6)) * assign33220_e36683) - (assign33220_e36678 * ((locals.var_q_temp__blk860_dn6 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn6)))) / (assign33220_e36683 * assign33220_e36683)), ((((((((-locals.var_q_zero__blk849_dn7) * locals.var_q_d1_zero__blk850) + (assign33220_e36674 * locals.var_q_d1_zero__blk850_dn7)) * locals.var_q_temp__blk860) + (assign33220_e36676 * locals.var_q_temp__blk860_dn7)) * assign33220_e36683) - (assign33220_e36678 * ((locals.var_q_temp__blk860_dn7 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn7)))) / (assign33220_e36683 * assign33220_e36683)), ((((((((-locals.var_q_zero__blk849_dn8) * locals.var_q_d1_zero__blk850) + (assign33220_e36674 * locals.var_q_d1_zero__blk850_dn8)) * locals.var_q_temp__blk860) + (assign33220_e36676 * locals.var_q_temp__blk860_dn8)) * assign33220_e36683) - (assign33220_e36678 * ((locals.var_q_temp__blk860_dn8 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn8)))) / (assign33220_e36683 * assign33220_e36683)), ((((((((-locals.var_q_zero__blk849_dn9) * locals.var_q_d1_zero__blk850) + (assign33220_e36674 * locals.var_q_d1_zero__blk850_dn9)) * locals.var_q_temp__blk860) + (assign33220_e36676 * locals.var_q_temp__blk860_dn9)) * assign33220_e36683) - (assign33220_e36678 * ((locals.var_q_temp__blk860_dn9 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn9)))) / (assign33220_e36683 * assign33220_e36683)),)
    } else {
        (locals.var_q_eps2__blk852, locals.var_q_eps2__blk852_dn4, locals.var_q_eps2__blk852_dn6, locals.var_q_eps2__blk852_dn7, locals.var_q_eps2__blk852_dn8, locals.var_q_eps2__blk852_dn9,)
    }
};
        locals.var_q_eps2__blk852 = assign33220_e36686;
        locals.var_q_eps2__blk852_dn4 = assign33220_e36686_d_n4;
        locals.var_q_eps2__blk852_dn6 = assign33220_e36686_d_n6;
        locals.var_q_eps2__blk852_dn7 = assign33220_e36686_d_n7;
        locals.var_q_eps2__blk852_dn8 = assign33220_e36686_d_n8;
        locals.var_q_eps2__blk852_dn9 = assign33220_e36686_d_n9;

        let (assign33230_e36692, assign33230_e36692_d_n4, assign33230_e36692_d_n6, assign33230_e36692_d_n7, assign33230_e36692_d_n8, assign33230_e36692_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign33230_e36690: f64 = (locals.var_q1s__blk937 + locals.var_q_eps2__blk852);
        (assign33230_e36690, (locals.var_q1s__blk937_dn4 + locals.var_q_eps2__blk852_dn4), (locals.var_q1s__blk937_dn6 + locals.var_q_eps2__blk852_dn6), (locals.var_q1s__blk937_dn7 + locals.var_q_eps2__blk852_dn7), (locals.var_q1s__blk937_dn8 + locals.var_q_eps2__blk852_dn8), (locals.var_q1s__blk937_dn9 + locals.var_q_eps2__blk852_dn9),)
    } else {
        (locals.var_q1s__blk937, locals.var_q1s__blk937_dn4, locals.var_q1s__blk937_dn6, locals.var_q1s__blk937_dn7, locals.var_q1s__blk937_dn8, locals.var_q1s__blk937_dn9,)
    }
};
        locals.var_q1s__blk937 = assign33230_e36692;
        locals.var_q1s__blk937_dn4 = assign33230_e36692_d_n4;
        locals.var_q1s__blk937_dn6 = assign33230_e36692_d_n6;
        locals.var_q1s__blk937_dn7 = assign33230_e36692_d_n7;
        locals.var_q1s__blk937_dn8 = assign33230_e36692_d_n8;
        locals.var_q1s__blk937_dn9 = assign33230_e36692_d_n9;

        let assign33240_e36695: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1129 = assign33240_e36695;

        let assign33250_e36697: f64 = (locals.var_q_eps2__blk852).abs();
        let assign33250_e36699: f64 = if assign33250_e36697 > 0.01 { 1.0 } else { 0.0 };
        locals.var_guard1130 = assign33250_e36699;

        let (assign33260_e36709, assign33260_e36709_d_n4, assign33260_e36709_d_n6, assign33260_e36709_d_n7, assign33260_e36709_d_n8, assign33260_e36709_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) {
        let assign33260_e36707: f64 = (locals.var_k1__blk932 * locals.var_q1s__blk937);
        (assign33260_e36707, ((locals.var_k1__blk932_dn4 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn9)),)
    } else {
        (locals.var_q_k1q1__blk823, locals.var_q_k1q1__blk823_dn4, locals.var_q_k1q1__blk823_dn6, locals.var_q_k1q1__blk823_dn7, locals.var_q_k1q1__blk823_dn8, locals.var_q_k1q1__blk823_dn9,)
    }
};
        locals.var_q_k1q1__blk823 = assign33260_e36709;
        locals.var_q_k1q1__blk823_dn4 = assign33260_e36709_d_n4;
        locals.var_q_k1q1__blk823_dn6 = assign33260_e36709_d_n6;
        locals.var_q_k1q1__blk823_dn7 = assign33260_e36709_d_n7;
        locals.var_q_k1q1__blk823_dn8 = assign33260_e36709_d_n8;
        locals.var_q_k1q1__blk823_dn9 = assign33260_e36709_d_n9;

        let assign33270_e36712: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign33270_e36714: f64 = assign33270_e36712;
        let assign33270_e36716: f64 = if assign33270_e36714 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1131 = assign33270_e36716;

        let (assign33280_e36731, assign33280_e36731_d_n4, assign33280_e36731_d_n6, assign33280_e36731_d_n7, assign33280_e36731_d_n8, assign33280_e36731_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1131 != 0.0)) {
        let assign33280_e36726: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign33280_e36728: f64 = assign33280_e36726;
        let assign33280_e36729: f64 = (assign33280_e36728).exp();
        (assign33280_e36729, (assign33280_e36729 * (locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4)), (assign33280_e36729 * (locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6)), (assign33280_e36729 * (locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7)), (assign33280_e36729 * (locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8)), (assign33280_e36729 * (locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign33280_e36731;
        locals.var_q_temp1__blk814_dn4 = assign33280_e36731_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign33280_e36731_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign33280_e36731_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign33280_e36731_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign33280_e36731_d_n9;

        let (assign33290_e36776, assign33290_e36776_d_n4, assign33290_e36776_d_n6, assign33290_e36776_d_n7, assign33290_e36776_d_n8, assign33290_e36776_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1131 == 0.0)) {
        let assign33290_e36744: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign33290_e36746: f64 = assign33290_e36744;
        let assign33290_e36748: f64 = (assign33290_e36746 - 80.0);
        let assign33290_e36753: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign33290_e36755: f64 = assign33290_e36753;
        let assign33290_e36757: f64 = (assign33290_e36755 - 80.0);
        let assign33290_e36758: f64 = (0.5 * assign33290_e36757);
        let assign33290_e36762: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign33290_e36764: f64 = assign33290_e36762;
        let assign33290_e36766: f64 = (assign33290_e36764 - 80.0);
        let assign33290_e36768: f64 = (assign33290_e36766 * 0.3333333333333);
        let assign33290_e36769: f64 = (1.0 + assign33290_e36768);
        let assign33290_e36770: f64 = (assign33290_e36758 * assign33290_e36769);
        let assign33290_e36771: f64 = (1.0 + assign33290_e36770);
        let assign33290_e36772: f64 = (assign33290_e36748 * assign33290_e36771);
        let assign33290_e36773: f64 = (1.0 + assign33290_e36772);
        let assign33290_e36774: f64 = (5.54062e34 * assign33290_e36773);
        (assign33290_e36774, (5.54062e34 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4) * assign33290_e36771) + (assign33290_e36748 * (((0.5 * (locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4)) * assign33290_e36769) + (assign33290_e36758 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6) * assign33290_e36771) + (assign33290_e36748 * (((0.5 * (locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6)) * assign33290_e36769) + (assign33290_e36758 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7) * assign33290_e36771) + (assign33290_e36748 * (((0.5 * (locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7)) * assign33290_e36769) + (assign33290_e36758 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8) * assign33290_e36771) + (assign33290_e36748 * (((0.5 * (locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8)) * assign33290_e36769) + (assign33290_e36758 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9) * assign33290_e36771) + (assign33290_e36748 * (((0.5 * (locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9)) * assign33290_e36769) + (assign33290_e36758 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign33290_e36776;
        locals.var_q_temp1__blk814_dn4 = assign33290_e36776_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign33290_e36776_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign33290_e36776_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign33290_e36776_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign33290_e36776_d_n9;

        let (assign33300_e36786, assign33300_e36786_d_n4, assign33300_e36786_d_n6, assign33300_e36786_d_n7, assign33300_e36786_d_n8, assign33300_e36786_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) {
        let assign33300_e36784: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
        (assign33300_e36784, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_aexp__blk824, locals.var_q_aexp__blk824_dn4, locals.var_q_aexp__blk824_dn6, locals.var_q_aexp__blk824_dn7, locals.var_q_aexp__blk824_dn8, locals.var_q_aexp__blk824_dn9,)
    }
};
        locals.var_q_aexp__blk824 = assign33300_e36786;
        locals.var_q_aexp__blk824_dn4 = assign33300_e36786_d_n4;
        locals.var_q_aexp__blk824_dn6 = assign33300_e36786_d_n6;
        locals.var_q_aexp__blk824_dn7 = assign33300_e36786_d_n7;
        locals.var_q_aexp__blk824_dn8 = assign33300_e36786_d_n8;
        locals.var_q_aexp__blk824_dn9 = assign33300_e36786_d_n9;

        let (assign33310_e36798, assign33310_e36798_d_n4, assign33310_e36798_d_n6, assign33310_e36798_d_n7, assign33310_e36798_d_n8, assign33310_e36798_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) {
        let assign33310_e36794: f64 = (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823);
        let assign33310_e36796: f64 = (assign33310_e36794 - locals.var_q_aexp__blk824);
        (assign33310_e36796, (((locals.var_q_k1q1__blk823_dn4 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_k1q1__blk823_dn6 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_k1q1__blk823_dn7 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_k1q1__blk823_dn8 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_k1q1__blk823_dn9 * locals.var_q_k1q1__blk823) + (locals.var_q_k1q1__blk823 * locals.var_q_k1q1__blk823_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_qsq__blk825, locals.var_q_qsq__blk825_dn4, locals.var_q_qsq__blk825_dn6, locals.var_q_qsq__blk825_dn7, locals.var_q_qsq__blk825_dn8, locals.var_q_qsq__blk825_dn9,)
    }
};
        locals.var_q_qsq__blk825 = assign33310_e36798;
        locals.var_q_qsq__blk825_dn4 = assign33310_e36798_d_n4;
        locals.var_q_qsq__blk825_dn6 = assign33310_e36798_d_n6;
        locals.var_q_qsq__blk825_dn7 = assign33310_e36798_d_n7;
        locals.var_q_qsq__blk825_dn8 = assign33310_e36798_d_n8;
        locals.var_q_qsq__blk825_dn9 = assign33310_e36798_d_n9;

        let (assign33320_e36812, assign33320_e36812_d_n4, assign33320_e36812_d_n6, assign33320_e36812_d_n7, assign33320_e36812_d_n8, assign33320_e36812_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) {
        let assign33320_e36806: f64 = (2.0 * locals.var_k1__blk932);
        let assign33320_e36808: f64 = (assign33320_e36806 * locals.var_q_k1q1__blk823);
        let assign33320_e36810: f64 = (assign33320_e36808 + locals.var_q_aexp__blk824);
        (assign33320_e36810, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_q_k1q1__blk823) + (assign33320_e36806 * locals.var_q_k1q1__blk823_dn4)) + locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_q_k1q1__blk823) + (assign33320_e36806 * locals.var_q_k1q1__blk823_dn6)) + locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_q_k1q1__blk823) + (assign33320_e36806 * locals.var_q_k1q1__blk823_dn7)) + locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_q_k1q1__blk823) + (assign33320_e36806 * locals.var_q_k1q1__blk823_dn8)) + locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_q_k1q1__blk823) + (assign33320_e36806 * locals.var_q_k1q1__blk823_dn9)) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_qsq__blk826, locals.var_q_d1_qsq__blk826_dn4, locals.var_q_d1_qsq__blk826_dn6, locals.var_q_d1_qsq__blk826_dn7, locals.var_q_d1_qsq__blk826_dn8, locals.var_q_d1_qsq__blk826_dn9,)
    }
};
        locals.var_q_d1_qsq__blk826 = assign33320_e36812;
        locals.var_q_d1_qsq__blk826_dn4 = assign33320_e36812_d_n4;
        locals.var_q_d1_qsq__blk826_dn6 = assign33320_e36812_d_n6;
        locals.var_q_d1_qsq__blk826_dn7 = assign33320_e36812_d_n7;
        locals.var_q_d1_qsq__blk826_dn8 = assign33320_e36812_d_n8;
        locals.var_q_d1_qsq__blk826_dn9 = assign33320_e36812_d_n9;

        let (assign33330_e36826, assign33330_e36826_d_n4, assign33330_e36826_d_n6, assign33330_e36826_d_n7, assign33330_e36826_d_n8, assign33330_e36826_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) {
        let assign33330_e36820: f64 = (2.0 * locals.var_k1__blk932);
        let assign33330_e36822: f64 = (assign33330_e36820 * locals.var_k1__blk932);
        let assign33330_e36824: f64 = (assign33330_e36822 - locals.var_q_aexp__blk824);
        (assign33330_e36824, ((((2.0 * locals.var_k1__blk932_dn4) * locals.var_k1__blk932) + (assign33330_e36820 * locals.var_k1__blk932_dn4)) - locals.var_q_aexp__blk824_dn4), ((((2.0 * locals.var_k1__blk932_dn6) * locals.var_k1__blk932) + (assign33330_e36820 * locals.var_k1__blk932_dn6)) - locals.var_q_aexp__blk824_dn6), ((((2.0 * locals.var_k1__blk932_dn7) * locals.var_k1__blk932) + (assign33330_e36820 * locals.var_k1__blk932_dn7)) - locals.var_q_aexp__blk824_dn7), ((((2.0 * locals.var_k1__blk932_dn8) * locals.var_k1__blk932) + (assign33330_e36820 * locals.var_k1__blk932_dn8)) - locals.var_q_aexp__blk824_dn8), ((((2.0 * locals.var_k1__blk932_dn9) * locals.var_k1__blk932) + (assign33330_e36820 * locals.var_k1__blk932_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_qsq__blk827, locals.var_q_d2_qsq__blk827_dn4, locals.var_q_d2_qsq__blk827_dn6, locals.var_q_d2_qsq__blk827_dn7, locals.var_q_d2_qsq__blk827_dn8, locals.var_q_d2_qsq__blk827_dn9,)
    }
};
        locals.var_q_d2_qsq__blk827 = assign33330_e36826;
        locals.var_q_d2_qsq__blk827_dn4 = assign33330_e36826_d_n4;
        locals.var_q_d2_qsq__blk827_dn6 = assign33330_e36826_d_n6;
        locals.var_q_d2_qsq__blk827_dn7 = assign33330_e36826_d_n7;
        locals.var_q_d2_qsq__blk827_dn8 = assign33330_e36826_d_n8;
        locals.var_q_d2_qsq__blk827_dn9 = assign33330_e36826_d_n9;

        let assign33340_e36829: f64 = (-0.005);
        let assign33340_e36830: f64 = if locals.var_q_qsq__blk825 < assign33340_e36829 { 1.0 } else { 0.0 };
        locals.var_guard1132 = assign33340_e36830;

        let (assign33350_e36842, assign33350_e36842_d_n4, assign33350_e36842_d_n6, assign33350_e36842_d_n7, assign33350_e36842_d_n8, assign33350_e36842_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 != 0.0)) {
        let assign33350_e36839: f64 = (locals.var_q_qsq__blk825).abs();
        let assign33350_e36840: f64 = (assign33350_e36839).sqrt();
        (assign33350_e36840, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign33350_e36840)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign33350_e36840)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign33350_e36840)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign33350_e36840)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign33350_e36840)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign33350_e36842;
        locals.var_q_rac_qsq__blk828_dn4 = assign33350_e36842_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign33350_e36842_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign33350_e36842_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign33350_e36842_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign33350_e36842_d_n9;

        let (assign33360_e36857, assign33360_e36857_d_n4, assign33360_e36857_d_n6, assign33360_e36857_d_n7, assign33360_e36857_d_n8, assign33360_e36857_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 != 0.0)) {
        let assign33360_e36853: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign33360_e36854: f64 = (assign33360_e36853).tan();
        let assign33360_e36855: f64 = (locals.var_q_rac_qsq__blk828 / assign33360_e36854);
        (assign33360_e36855, (((locals.var_q_rac_qsq__blk828_dn4 * assign33360_e36854) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign33360_e36853).cos() * (assign33360_e36853).cos())))) / (assign33360_e36854 * assign33360_e36854)), (((locals.var_q_rac_qsq__blk828_dn6 * assign33360_e36854) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign33360_e36853).cos() * (assign33360_e36853).cos())))) / (assign33360_e36854 * assign33360_e36854)), (((locals.var_q_rac_qsq__blk828_dn7 * assign33360_e36854) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign33360_e36853).cos() * (assign33360_e36853).cos())))) / (assign33360_e36854 * assign33360_e36854)), (((locals.var_q_rac_qsq__blk828_dn8 * assign33360_e36854) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign33360_e36853).cos() * (assign33360_e36853).cos())))) / (assign33360_e36854 * assign33360_e36854)), (((locals.var_q_rac_qsq__blk828_dn9 * assign33360_e36854) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign33360_e36853).cos() * (assign33360_e36853).cos())))) / (assign33360_e36854 * assign33360_e36854)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign33360_e36857;
        locals.var_q_qcoth__blk829_dn4 = assign33360_e36857_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign33360_e36857_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign33360_e36857_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign33360_e36857_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign33360_e36857_d_n9;

        let (assign33370_e36871, assign33370_e36871_d_n4, assign33370_e36871_d_n6, assign33370_e36871_d_n7, assign33370_e36871_d_n8, assign33370_e36871_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 != 0.0)) {
        let assign33370_e36867: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign33370_e36869: f64 = (assign33370_e36867 / locals.var_q_qsq__blk825);
        (assign33370_e36869, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign33370_e36867 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign33370_e36867 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign33370_e36867 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign33370_e36867 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign33370_e36867 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign33370_e36871;
        locals.var_q_temp1__blk814_dn4 = assign33370_e36871_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign33370_e36871_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign33370_e36871_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign33370_e36871_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign33370_e36871_d_n9;

    }

    pub(super) fn stamp_transient_block_90(
        locals: &mut StampLocals,
    ) {
        let (assign33380_e36889, assign33380_e36889_d_n4, assign33380_e36889_d_n6, assign33380_e36889_d_n7, assign33380_e36889_d_n8, assign33380_e36889_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 != 0.0)) {
        let assign33380_e36883: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign33380_e36884: f64 = (locals.var_q_qcoth__blk829 * assign33380_e36883);
        let assign33380_e36885: f64 = (locals.var_q_qsq__blk825 + assign33380_e36884);
        let assign33380_e36887: f64 = (assign33380_e36885 * locals.var_q_temp1__blk814);
        (assign33380_e36887, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign33380_e36883) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign33380_e36885 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign33380_e36883) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign33380_e36885 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign33380_e36883) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign33380_e36885 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign33380_e36883) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign33380_e36885 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign33380_e36883) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign33380_e36885 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign33380_e36889;
        locals.var_q_d1_qcoth__blk830_dn4 = assign33380_e36889_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign33380_e36889_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign33380_e36889_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign33380_e36889_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign33380_e36889_d_n9;

        let (assign33390_e36915, assign33390_e36915_d_n4, assign33390_e36915_d_n6, assign33390_e36915_d_n7, assign33390_e36915_d_n8, assign33390_e36915_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 != 0.0)) {
        let assign33390_e36900: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign33390_e36903: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign33390_e36904: f64 = (assign33390_e36900 * assign33390_e36903);
        let assign33390_e36905: f64 = (locals.var_q_d1_qsq__blk826 - assign33390_e36904);
        let assign33390_e36907: f64 = (assign33390_e36905 * locals.var_q_temp1__blk814);
        let assign33390_e36910: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign33390_e36912: f64 = (assign33390_e36910 / locals.var_q_d1_qsq__blk826);
        let assign33390_e36913: f64 = (assign33390_e36907 + assign33390_e36912);
        (assign33390_e36913, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign33390_e36903) + (assign33390_e36900 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign33390_e36905 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign33390_e36910 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign33390_e36903) + (assign33390_e36900 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign33390_e36905 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign33390_e36910 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign33390_e36903) + (assign33390_e36900 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign33390_e36905 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign33390_e36910 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign33390_e36903) + (assign33390_e36900 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign33390_e36905 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign33390_e36910 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign33390_e36903) + (assign33390_e36900 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign33390_e36905 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign33390_e36910 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign33390_e36915;
        locals.var_q_d2_qcoth__blk832_dn4 = assign33390_e36915_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign33390_e36915_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign33390_e36915_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign33390_e36915_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign33390_e36915_d_n9;

        let (assign33400_e36929, assign33400_e36929_d_n4, assign33400_e36929_d_n6, assign33400_e36929_d_n7, assign33400_e36929_d_n8, assign33400_e36929_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 != 0.0)) {
        let assign33400_e36926: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign33400_e36927: f64 = (1.0 - assign33400_e36926);
        (assign33400_e36927, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign33400_e36929;
        locals.var_q_temp2__blk815_dn4 = assign33400_e36929_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign33400_e36929_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign33400_e36929_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign33400_e36929_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign33400_e36929_d_n9;

        let (assign33410_e36943, assign33410_e36943_d_n4, assign33410_e36943_d_n6, assign33410_e36943_d_n7, assign33410_e36943_d_n8, assign33410_e36943_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 != 0.0)) {
        let assign33410_e36939: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign33410_e36941: f64 = (assign33410_e36939 * locals.var_q_temp2__blk815);
        (assign33410_e36941, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign33410_e36939 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign33410_e36939 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign33410_e36939 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign33410_e36939 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign33410_e36939 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign33410_e36943;
        locals.var_q_d1_ln__blk835_dn4 = assign33410_e36943_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign33410_e36943_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign33410_e36943_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign33410_e36943_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign33410_e36943_d_n9;

        let (assign33420_e36965, assign33420_e36965_d_n4, assign33420_e36965_d_n6, assign33420_e36965_d_n7, assign33420_e36965_d_n8, assign33420_e36965_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 != 0.0)) {
        let assign33420_e36953: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign33420_e36958: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign33420_e36959: f64 = (locals.var_q_d1_ln__blk835 + assign33420_e36958);
        let assign33420_e36960: f64 = (locals.var_q_d1_qsq__blk826 * assign33420_e36959);
        let assign33420_e36961: f64 = (assign33420_e36953 - assign33420_e36960);
        let assign33420_e36963: f64 = (assign33420_e36961 / locals.var_q_qsq__blk825);
        (assign33420_e36963, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign33420_e36959) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign33420_e36961 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign33420_e36959) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign33420_e36961 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign33420_e36959) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign33420_e36961 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign33420_e36959) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign33420_e36961 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign33420_e36959) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign33420_e36961 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign33420_e36965;
        locals.var_q_d2_ln__blk836_dn4 = assign33420_e36965_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign33420_e36965_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign33420_e36965_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign33420_e36965_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign33420_e36965_d_n9;

        let assign33430_e36968: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1133 = assign33430_e36968;

        let (assign33440_e36983, assign33440_e36983_d_n4, assign33440_e36983_d_n6, assign33440_e36983_d_n7, assign33440_e36983_d_n8, assign33440_e36983_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 == 0.0)) && (locals.var_guard1133 != 0.0)) {
        let assign33440_e36980: f64 = (locals.var_q_qsq__blk825).abs();
        let assign33440_e36981: f64 = (assign33440_e36980).sqrt();
        (assign33440_e36981, (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn4 } else { (-locals.var_q_qsq__blk825_dn4) } / (2.0 * assign33440_e36981)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn6 } else { (-locals.var_q_qsq__blk825_dn6) } / (2.0 * assign33440_e36981)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn7 } else { (-locals.var_q_qsq__blk825_dn7) } / (2.0 * assign33440_e36981)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn8 } else { (-locals.var_q_qsq__blk825_dn8) } / (2.0 * assign33440_e36981)), (if locals.var_q_qsq__blk825 >= 0.0 { locals.var_q_qsq__blk825_dn9 } else { (-locals.var_q_qsq__blk825_dn9) } / (2.0 * assign33440_e36981)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign33440_e36983;
        locals.var_q_rac_qsq__blk828_dn4 = assign33440_e36983_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign33440_e36983_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign33440_e36983_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign33440_e36983_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign33440_e36983_d_n9;

        let (assign33450_e36998, assign33450_e36998_d_n4, assign33450_e36998_d_n6, assign33450_e36998_d_n7, assign33450_e36998_d_n8, assign33450_e36998_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 == 0.0)) && (locals.var_guard1133 != 0.0)) {
        let assign33450_e36995: f64 = (-locals.var_q_rac_qsq__blk828);
        let assign33450_e36996: f64 = (assign33450_e36995).exp();
        (assign33450_e36996, (assign33450_e36996 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign33450_e36996 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign33450_e36996 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign33450_e36996 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign33450_e36996 * (-locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign33450_e36998;
        locals.var_q_invexpq__blk831_dn4 = assign33450_e36998_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign33450_e36998_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign33450_e36998_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign33450_e36998_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign33450_e36998_d_n9;

        let (assign33460_e37019, assign33460_e37019_d_n4, assign33460_e37019_d_n6, assign33460_e37019_d_n7, assign33460_e37019_d_n8, assign33460_e37019_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 == 0.0)) && (locals.var_guard1133 != 0.0)) {
        let assign33460_e37012: f64 = (1.0 + locals.var_q_invexpq__blk831);
        let assign33460_e37013: f64 = (locals.var_q_rac_qsq__blk828 * assign33460_e37012);
        let assign33460_e37016: f64 = (1.0 - locals.var_q_invexpq__blk831);
        let assign33460_e37017: f64 = (assign33460_e37013 / assign33460_e37016);
        (assign33460_e37017, (((((locals.var_q_rac_qsq__blk828_dn4 * assign33460_e37012) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign33460_e37016) - (assign33460_e37013 * (-locals.var_q_invexpq__blk831_dn4))) / (assign33460_e37016 * assign33460_e37016)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign33460_e37012) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign33460_e37016) - (assign33460_e37013 * (-locals.var_q_invexpq__blk831_dn6))) / (assign33460_e37016 * assign33460_e37016)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign33460_e37012) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign33460_e37016) - (assign33460_e37013 * (-locals.var_q_invexpq__blk831_dn7))) / (assign33460_e37016 * assign33460_e37016)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign33460_e37012) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign33460_e37016) - (assign33460_e37013 * (-locals.var_q_invexpq__blk831_dn8))) / (assign33460_e37016 * assign33460_e37016)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign33460_e37012) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign33460_e37016) - (assign33460_e37013 * (-locals.var_q_invexpq__blk831_dn9))) / (assign33460_e37016 * assign33460_e37016)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign33460_e37019;
        locals.var_q_qcoth__blk829_dn4 = assign33460_e37019_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign33460_e37019_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign33460_e37019_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign33460_e37019_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign33460_e37019_d_n9;

        let (assign33470_e37036, assign33470_e37036_d_n4, assign33470_e37036_d_n6, assign33470_e37036_d_n7, assign33470_e37036_d_n8, assign33470_e37036_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 == 0.0)) && (locals.var_guard1133 != 0.0)) {
        let assign33470_e37032: f64 = (0.25 * locals.var_q_d1_qsq__blk826);
        let assign33470_e37034: f64 = (assign33470_e37032 / locals.var_q_qsq__blk825);
        (assign33470_e37034, ((((0.25 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_qsq__blk825) - (assign33470_e37032 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_qsq__blk825) - (assign33470_e37032 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_qsq__blk825) - (assign33470_e37032 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_qsq__blk825) - (assign33470_e37032 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((0.25 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_qsq__blk825) - (assign33470_e37032 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign33470_e37036;
        locals.var_q_temp1__blk814_dn4 = assign33470_e37036_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign33470_e37036_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign33470_e37036_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign33470_e37036_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign33470_e37036_d_n9;

        let (assign33480_e37057, assign33480_e37057_d_n4, assign33480_e37057_d_n6, assign33480_e37057_d_n7, assign33480_e37057_d_n8, assign33480_e37057_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 == 0.0)) && (locals.var_guard1133 != 0.0)) {
        let assign33480_e37051: f64 = (2.0 - locals.var_q_qcoth__blk829);
        let assign33480_e37052: f64 = (locals.var_q_qcoth__blk829 * assign33480_e37051);
        let assign33480_e37053: f64 = (locals.var_q_qsq__blk825 + assign33480_e37052);
        let assign33480_e37055: f64 = (assign33480_e37053 * locals.var_q_temp1__blk814);
        (assign33480_e37055, (((locals.var_q_qsq__blk825_dn4 + ((locals.var_q_qcoth__blk829_dn4 * assign33480_e37051) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn4)))) * locals.var_q_temp1__blk814) + (assign33480_e37053 * locals.var_q_temp1__blk814_dn4)), (((locals.var_q_qsq__blk825_dn6 + ((locals.var_q_qcoth__blk829_dn6 * assign33480_e37051) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn6)))) * locals.var_q_temp1__blk814) + (assign33480_e37053 * locals.var_q_temp1__blk814_dn6)), (((locals.var_q_qsq__blk825_dn7 + ((locals.var_q_qcoth__blk829_dn7 * assign33480_e37051) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn7)))) * locals.var_q_temp1__blk814) + (assign33480_e37053 * locals.var_q_temp1__blk814_dn7)), (((locals.var_q_qsq__blk825_dn8 + ((locals.var_q_qcoth__blk829_dn8 * assign33480_e37051) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn8)))) * locals.var_q_temp1__blk814) + (assign33480_e37053 * locals.var_q_temp1__blk814_dn8)), (((locals.var_q_qsq__blk825_dn9 + ((locals.var_q_qcoth__blk829_dn9 * assign33480_e37051) + (locals.var_q_qcoth__blk829 * (-locals.var_q_qcoth__blk829_dn9)))) * locals.var_q_temp1__blk814) + (assign33480_e37053 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign33480_e37057;
        locals.var_q_d1_qcoth__blk830_dn4 = assign33480_e37057_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign33480_e37057_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign33480_e37057_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign33480_e37057_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign33480_e37057_d_n9;

        let (assign33490_e37086, assign33490_e37086_d_n4, assign33490_e37086_d_n6, assign33490_e37086_d_n7, assign33490_e37086_d_n8, assign33490_e37086_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 == 0.0)) && (locals.var_guard1133 != 0.0)) {
        let assign33490_e37071: f64 = (2.0 * locals.var_q_d1_qcoth__blk830);
        let assign33490_e37074: f64 = (1.0 + locals.var_q_qcoth__blk829);
        let assign33490_e37075: f64 = (assign33490_e37071 * assign33490_e37074);
        let assign33490_e37076: f64 = (locals.var_q_d1_qsq__blk826 - assign33490_e37075);
        let assign33490_e37078: f64 = (assign33490_e37076 * locals.var_q_temp1__blk814);
        let assign33490_e37081: f64 = (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827);
        let assign33490_e37083: f64 = (assign33490_e37081 / locals.var_q_d1_qsq__blk826);
        let assign33490_e37084: f64 = (assign33490_e37078 + assign33490_e37083);
        (assign33490_e37084, ((((locals.var_q_d1_qsq__blk826_dn4 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn4) * assign33490_e37074) + (assign33490_e37071 * locals.var_q_qcoth__blk829_dn4))) * locals.var_q_temp1__blk814) + (assign33490_e37076 * locals.var_q_temp1__blk814_dn4)) + (((((locals.var_q_d1_qcoth__blk830_dn4 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn4)) * locals.var_q_d1_qsq__blk826) - (assign33490_e37081 * locals.var_q_d1_qsq__blk826_dn4)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn6 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn6) * assign33490_e37074) + (assign33490_e37071 * locals.var_q_qcoth__blk829_dn6))) * locals.var_q_temp1__blk814) + (assign33490_e37076 * locals.var_q_temp1__blk814_dn6)) + (((((locals.var_q_d1_qcoth__blk830_dn6 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn6)) * locals.var_q_d1_qsq__blk826) - (assign33490_e37081 * locals.var_q_d1_qsq__blk826_dn6)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn7 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn7) * assign33490_e37074) + (assign33490_e37071 * locals.var_q_qcoth__blk829_dn7))) * locals.var_q_temp1__blk814) + (assign33490_e37076 * locals.var_q_temp1__blk814_dn7)) + (((((locals.var_q_d1_qcoth__blk830_dn7 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn7)) * locals.var_q_d1_qsq__blk826) - (assign33490_e37081 * locals.var_q_d1_qsq__blk826_dn7)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn8 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn8) * assign33490_e37074) + (assign33490_e37071 * locals.var_q_qcoth__blk829_dn8))) * locals.var_q_temp1__blk814) + (assign33490_e37076 * locals.var_q_temp1__blk814_dn8)) + (((((locals.var_q_d1_qcoth__blk830_dn8 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn8)) * locals.var_q_d1_qsq__blk826) - (assign33490_e37081 * locals.var_q_d1_qsq__blk826_dn8)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))), ((((locals.var_q_d1_qsq__blk826_dn9 - (((2.0 * locals.var_q_d1_qcoth__blk830_dn9) * assign33490_e37074) + (assign33490_e37071 * locals.var_q_qcoth__blk829_dn9))) * locals.var_q_temp1__blk814) + (assign33490_e37076 * locals.var_q_temp1__blk814_dn9)) + (((((locals.var_q_d1_qcoth__blk830_dn9 * locals.var_q_d2_qsq__blk827) + (locals.var_q_d1_qcoth__blk830 * locals.var_q_d2_qsq__blk827_dn9)) * locals.var_q_d1_qsq__blk826) - (assign33490_e37081 * locals.var_q_d1_qsq__blk826_dn9)) / (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign33490_e37086;
        locals.var_q_d2_qcoth__blk832_dn4 = assign33490_e37086_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign33490_e37086_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign33490_e37086_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign33490_e37086_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign33490_e37086_d_n9;

        let (assign33500_e37103, assign33500_e37103_d_n4, assign33500_e37103_d_n6, assign33500_e37103_d_n7, assign33500_e37103_d_n8, assign33500_e37103_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 == 0.0)) && (locals.var_guard1133 != 0.0)) {
        let assign33500_e37100: f64 = (0.5 * locals.var_q_qcoth__blk829);
        let assign33500_e37101: f64 = (1.0 - assign33500_e37100);
        (assign33500_e37101, (-(0.5 * locals.var_q_qcoth__blk829_dn4)), (-(0.5 * locals.var_q_qcoth__blk829_dn6)), (-(0.5 * locals.var_q_qcoth__blk829_dn7)), (-(0.5 * locals.var_q_qcoth__blk829_dn8)), (-(0.5 * locals.var_q_qcoth__blk829_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign33500_e37103;
        locals.var_q_temp2__blk815_dn4 = assign33500_e37103_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign33500_e37103_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign33500_e37103_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign33500_e37103_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign33500_e37103_d_n9;

        let (assign33510_e37120, assign33510_e37120_d_n4, assign33510_e37120_d_n6, assign33510_e37120_d_n7, assign33510_e37120_d_n8, assign33510_e37120_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 == 0.0)) && (locals.var_guard1133 != 0.0)) {
        let assign33510_e37116: f64 = (locals.var_q_d1_qsq__blk826 / locals.var_q_qsq__blk825);
        let assign33510_e37118: f64 = (assign33510_e37116 * locals.var_q_temp2__blk815);
        (assign33510_e37118, (((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign33510_e37116 * locals.var_q_temp2__blk815_dn4)), (((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign33510_e37116 * locals.var_q_temp2__blk815_dn6)), (((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign33510_e37116 * locals.var_q_temp2__blk815_dn7)), (((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign33510_e37116 * locals.var_q_temp2__blk815_dn8)), (((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_qsq__blk825) - (locals.var_q_d1_qsq__blk826 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)) * locals.var_q_temp2__blk815) + (assign33510_e37116 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign33510_e37120;
        locals.var_q_d1_ln__blk835_dn4 = assign33510_e37120_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign33510_e37120_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign33510_e37120_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign33510_e37120_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign33510_e37120_d_n9;

        let (assign33520_e37145, assign33520_e37145_d_n4, assign33520_e37145_d_n6, assign33520_e37145_d_n7, assign33520_e37145_d_n8, assign33520_e37145_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 == 0.0)) && (locals.var_guard1133 != 0.0)) {
        let assign33520_e37133: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815);
        let assign33520_e37138: f64 = (0.5 * locals.var_q_d1_qcoth__blk830);
        let assign33520_e37139: f64 = (locals.var_q_d1_ln__blk835 + assign33520_e37138);
        let assign33520_e37140: f64 = (locals.var_q_d1_qsq__blk826 * assign33520_e37139);
        let assign33520_e37141: f64 = (assign33520_e37133 - assign33520_e37140);
        let assign33520_e37143: f64 = (assign33520_e37141 / locals.var_q_qsq__blk825);
        (assign33520_e37143, ((((((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn4)) - ((locals.var_q_d1_qsq__blk826_dn4 * assign33520_e37139) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn4 + (0.5 * locals.var_q_d1_qcoth__blk830_dn4))))) * locals.var_q_qsq__blk825) - (assign33520_e37141 * locals.var_q_qsq__blk825_dn4)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn6)) - ((locals.var_q_d1_qsq__blk826_dn6 * assign33520_e37139) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn6 + (0.5 * locals.var_q_d1_qcoth__blk830_dn6))))) * locals.var_q_qsq__blk825) - (assign33520_e37141 * locals.var_q_qsq__blk825_dn6)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn7)) - ((locals.var_q_d1_qsq__blk826_dn7 * assign33520_e37139) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn7 + (0.5 * locals.var_q_d1_qcoth__blk830_dn7))))) * locals.var_q_qsq__blk825) - (assign33520_e37141 * locals.var_q_qsq__blk825_dn7)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn8)) - ((locals.var_q_d1_qsq__blk826_dn8 * assign33520_e37139) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn8 + (0.5 * locals.var_q_d1_qcoth__blk830_dn8))))) * locals.var_q_qsq__blk825) - (assign33520_e37141 * locals.var_q_qsq__blk825_dn8)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)), ((((((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp2__blk815_dn9)) - ((locals.var_q_d1_qsq__blk826_dn9 * assign33520_e37139) + (locals.var_q_d1_qsq__blk826 * (locals.var_q_d1_ln__blk835_dn9 + (0.5 * locals.var_q_d1_qcoth__blk830_dn9))))) * locals.var_q_qsq__blk825) - (assign33520_e37141 * locals.var_q_qsq__blk825_dn9)) / (locals.var_q_qsq__blk825 * locals.var_q_qsq__blk825)),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign33520_e37145;
        locals.var_q_d2_ln__blk836_dn4 = assign33520_e37145_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign33520_e37145_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign33520_e37145_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign33520_e37145_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign33520_e37145_d_n9;

        let (assign33530_e37177, assign33530_e37177_d_n4, assign33530_e37177_d_n6, assign33530_e37177_d_n7, assign33530_e37177_d_n8, assign33530_e37177_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 == 0.0)) && (locals.var_guard1133 == 0.0)) {
        let assign33530_e37161: f64 = (locals.var_q_qsq__blk825 * 0.0166666666667);
        let assign33530_e37165: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign33530_e37169: f64 = (locals.var_q_qsq__blk825 * 0.025);
        let assign33530_e37170: f64 = (1.0 - assign33530_e37169);
        let assign33530_e37171: f64 = (assign33530_e37165 * assign33530_e37170);
        let assign33530_e37172: f64 = (1.0 - assign33530_e37171);
        let assign33530_e37173: f64 = (assign33530_e37161 * assign33530_e37172);
        let assign33530_e37174: f64 = (1.0 - assign33530_e37173);
        let assign33530_e37175: f64 = (0.1666666666667 * assign33530_e37174);
        (assign33530_e37175, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0166666666667) * assign33530_e37172) + (assign33530_e37161 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign33530_e37170) + (assign33530_e37165 * (-(locals.var_q_qsq__blk825_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0166666666667) * assign33530_e37172) + (assign33530_e37161 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign33530_e37170) + (assign33530_e37165 * (-(locals.var_q_qsq__blk825_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0166666666667) * assign33530_e37172) + (assign33530_e37161 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign33530_e37170) + (assign33530_e37165 * (-(locals.var_q_qsq__blk825_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0166666666667) * assign33530_e37172) + (assign33530_e37161 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign33530_e37170) + (assign33530_e37165 * (-(locals.var_q_qsq__blk825_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0166666666667) * assign33530_e37172) + (assign33530_e37161 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign33530_e37170) + (assign33530_e37165 * (-(locals.var_q_qsq__blk825_dn9 * 0.025))))))))),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign33530_e37177;
        locals.var_q_temp3__blk816_dn4 = assign33530_e37177_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign33530_e37177_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign33530_e37177_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign33530_e37177_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign33530_e37177_d_n9;

        let (assign33540_e37195, assign33540_e37195_d_n4, assign33540_e37195_d_n6, assign33540_e37195_d_n7, assign33540_e37195_d_n8, assign33540_e37195_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 == 0.0)) && (locals.var_guard1133 == 0.0)) {
        let assign33540_e37192: f64 = (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816);
        let assign33540_e37193: f64 = (2.0 + assign33540_e37192);
        (assign33540_e37193, ((locals.var_q_qsq__blk825_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn4)), ((locals.var_q_qsq__blk825_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn6)), ((locals.var_q_qsq__blk825_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn7)), ((locals.var_q_qsq__blk825_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn8)), ((locals.var_q_qsq__blk825_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_qsq__blk825 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign33540_e37195;
        locals.var_q_qcoth__blk829_dn4 = assign33540_e37195_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign33540_e37195_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign33540_e37195_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign33540_e37195_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign33540_e37195_d_n9;

        let (assign33550_e37227, assign33550_e37227_d_n4, assign33550_e37227_d_n6, assign33550_e37227_d_n7, assign33550_e37227_d_n8, assign33550_e37227_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 == 0.0)) && (locals.var_guard1133 == 0.0)) {
        let assign33550_e37211: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign33550_e37215: f64 = (locals.var_q_qsq__blk825 * 0.0357142857143);
        let assign33550_e37219: f64 = (locals.var_q_qsq__blk825 * 0.0333333333333);
        let assign33550_e37220: f64 = (1.0 - assign33550_e37219);
        let assign33550_e37221: f64 = (assign33550_e37215 * assign33550_e37220);
        let assign33550_e37222: f64 = (1.0 - assign33550_e37221);
        let assign33550_e37223: f64 = (assign33550_e37211 * assign33550_e37222);
        let assign33550_e37224: f64 = (1.0 - assign33550_e37223);
        let assign33550_e37225: f64 = (0.1666666666667 * assign33550_e37224);
        (assign33550_e37225, (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0333333333333) * assign33550_e37222) + (assign33550_e37211 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0357142857143) * assign33550_e37220) + (assign33550_e37215 * (-(locals.var_q_qsq__blk825_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0333333333333) * assign33550_e37222) + (assign33550_e37211 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0357142857143) * assign33550_e37220) + (assign33550_e37215 * (-(locals.var_q_qsq__blk825_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0333333333333) * assign33550_e37222) + (assign33550_e37211 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0357142857143) * assign33550_e37220) + (assign33550_e37215 * (-(locals.var_q_qsq__blk825_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0333333333333) * assign33550_e37222) + (assign33550_e37211 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0357142857143) * assign33550_e37220) + (assign33550_e37215 * (-(locals.var_q_qsq__blk825_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0333333333333) * assign33550_e37222) + (assign33550_e37211 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0357142857143) * assign33550_e37220) + (assign33550_e37215 * (-(locals.var_q_qsq__blk825_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign33550_e37227;
        locals.var_q_temp1__blk814_dn4 = assign33550_e37227_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign33550_e37227_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign33550_e37227_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign33550_e37227_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign33550_e37227_d_n9;

        let (assign33560_e37243, assign33560_e37243_d_n4, assign33560_e37243_d_n6, assign33560_e37243_d_n7, assign33560_e37243_d_n8, assign33560_e37243_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 == 0.0)) && (locals.var_guard1133 == 0.0)) {
        let assign33560_e37241: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814);
        (assign33560_e37241, ((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_qsq__blk826 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_qcoth__blk830, locals.var_q_d1_qcoth__blk830_dn4, locals.var_q_d1_qcoth__blk830_dn6, locals.var_q_d1_qcoth__blk830_dn7, locals.var_q_d1_qcoth__blk830_dn8, locals.var_q_d1_qcoth__blk830_dn9,)
    }
};
        locals.var_q_d1_qcoth__blk830 = assign33560_e37243;
        locals.var_q_d1_qcoth__blk830_dn4 = assign33560_e37243_d_n4;
        locals.var_q_d1_qcoth__blk830_dn6 = assign33560_e37243_d_n6;
        locals.var_q_d1_qcoth__blk830_dn7 = assign33560_e37243_d_n7;
        locals.var_q_d1_qcoth__blk830_dn8 = assign33560_e37243_d_n8;
        locals.var_q_d1_qcoth__blk830_dn9 = assign33560_e37243_d_n9;

        let (assign33570_e37275, assign33570_e37275_d_n4, assign33570_e37275_d_n6, assign33570_e37275_d_n7, assign33570_e37275_d_n8, assign33570_e37275_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 == 0.0)) && (locals.var_guard1133 == 0.0)) {
        let assign33570_e37259: f64 = (locals.var_q_qsq__blk825 * 0.0714285714286);
        let assign33570_e37263: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign33570_e37267: f64 = (0.0420875420875421 * locals.var_q_qsq__blk825);
        let assign33570_e37268: f64 = (1.0 - assign33570_e37267);
        let assign33570_e37269: f64 = (assign33570_e37263 * assign33570_e37268);
        let assign33570_e37270: f64 = (1.0 - assign33570_e37269);
        let assign33570_e37271: f64 = (assign33570_e37259 * assign33570_e37270);
        let assign33570_e37272: f64 = (1.0 - assign33570_e37271);
        let assign33570_e37273: f64 = (0.0055555555556 * assign33570_e37272);
        (assign33570_e37273, (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0714285714286) * assign33570_e37270) + (assign33570_e37259 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign33570_e37268) + (assign33570_e37263 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0714285714286) * assign33570_e37270) + (assign33570_e37259 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign33570_e37268) + (assign33570_e37263 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0714285714286) * assign33570_e37270) + (assign33570_e37259 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign33570_e37268) + (assign33570_e37263 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0714285714286) * assign33570_e37270) + (assign33570_e37259 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign33570_e37268) + (assign33570_e37263 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0714285714286) * assign33570_e37270) + (assign33570_e37259 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign33570_e37268) + (assign33570_e37263 * (-(0.0420875420875421 * locals.var_q_qsq__blk825_dn9))))))))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign33570_e37275;
        locals.var_q_temp2__blk815_dn4 = assign33570_e37275_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign33570_e37275_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign33570_e37275_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign33570_e37275_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign33570_e37275_d_n9;

        let (assign33580_e37297, assign33580_e37297_d_n4, assign33580_e37297_d_n6, assign33580_e37297_d_n7, assign33580_e37297_d_n8, assign33580_e37297_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 == 0.0)) && (locals.var_guard1133 == 0.0)) {
        let assign33580_e37289: f64 = (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814);
        let assign33580_e37292: f64 = (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826);
        let assign33580_e37294: f64 = (assign33580_e37292 * locals.var_q_temp2__blk815);
        let assign33580_e37295: f64 = (assign33580_e37289 - assign33580_e37294);
        (assign33580_e37295, (((locals.var_q_d2_qsq__blk827_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn4)) - ((((locals.var_q_d1_qsq__blk826_dn4 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn4)) * locals.var_q_temp2__blk815) + (assign33580_e37292 * locals.var_q_temp2__blk815_dn4))), (((locals.var_q_d2_qsq__blk827_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn6)) - ((((locals.var_q_d1_qsq__blk826_dn6 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn6)) * locals.var_q_temp2__blk815) + (assign33580_e37292 * locals.var_q_temp2__blk815_dn6))), (((locals.var_q_d2_qsq__blk827_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn7)) - ((((locals.var_q_d1_qsq__blk826_dn7 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn7)) * locals.var_q_temp2__blk815) + (assign33580_e37292 * locals.var_q_temp2__blk815_dn7))), (((locals.var_q_d2_qsq__blk827_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn8)) - ((((locals.var_q_d1_qsq__blk826_dn8 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn8)) * locals.var_q_temp2__blk815) + (assign33580_e37292 * locals.var_q_temp2__blk815_dn8))), (((locals.var_q_d2_qsq__blk827_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_qsq__blk827 * locals.var_q_temp1__blk814_dn9)) - ((((locals.var_q_d1_qsq__blk826_dn9 * locals.var_q_d1_qsq__blk826) + (locals.var_q_d1_qsq__blk826 * locals.var_q_d1_qsq__blk826_dn9)) * locals.var_q_temp2__blk815) + (assign33580_e37292 * locals.var_q_temp2__blk815_dn9))),)
    } else {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    }
};
        locals.var_q_d2_qcoth__blk832 = assign33580_e37297;
        locals.var_q_d2_qcoth__blk832_dn4 = assign33580_e37297_d_n4;
        locals.var_q_d2_qcoth__blk832_dn6 = assign33580_e37297_d_n6;
        locals.var_q_d2_qcoth__blk832_dn7 = assign33580_e37297_d_n7;
        locals.var_q_d2_qcoth__blk832_dn8 = assign33580_e37297_d_n8;
        locals.var_q_d2_qcoth__blk832_dn9 = assign33580_e37297_d_n9;

        let (assign33590_e37316, assign33590_e37316_d_n4, assign33590_e37316_d_n6, assign33590_e37316_d_n7, assign33590_e37316_d_n8, assign33590_e37316_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 == 0.0)) && (locals.var_guard1133 == 0.0)) {
        let assign33590_e37310: f64 = (-0.5);
        let assign33590_e37312: f64 = (assign33590_e37310 * locals.var_q_d1_qsq__blk826);
        let assign33590_e37314: f64 = (assign33590_e37312 * locals.var_q_temp3__blk816);
        (assign33590_e37314, (((assign33590_e37310 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_temp3__blk816) + (assign33590_e37312 * locals.var_q_temp3__blk816_dn4)), (((assign33590_e37310 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_temp3__blk816) + (assign33590_e37312 * locals.var_q_temp3__blk816_dn6)), (((assign33590_e37310 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_temp3__blk816) + (assign33590_e37312 * locals.var_q_temp3__blk816_dn7)), (((assign33590_e37310 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_temp3__blk816) + (assign33590_e37312 * locals.var_q_temp3__blk816_dn8)), (((assign33590_e37310 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_temp3__blk816) + (assign33590_e37312 * locals.var_q_temp3__blk816_dn9)),)
    } else {
        (locals.var_q_d1_ln__blk835, locals.var_q_d1_ln__blk835_dn4, locals.var_q_d1_ln__blk835_dn6, locals.var_q_d1_ln__blk835_dn7, locals.var_q_d1_ln__blk835_dn8, locals.var_q_d1_ln__blk835_dn9,)
    }
};
        locals.var_q_d1_ln__blk835 = assign33590_e37316;
        locals.var_q_d1_ln__blk835_dn4 = assign33590_e37316_d_n4;
        locals.var_q_d1_ln__blk835_dn6 = assign33590_e37316_d_n6;
        locals.var_q_d1_ln__blk835_dn7 = assign33590_e37316_d_n7;
        locals.var_q_d1_ln__blk835_dn8 = assign33590_e37316_d_n8;
        locals.var_q_d1_ln__blk835_dn9 = assign33590_e37316_d_n9;

        let (assign33600_e37355, assign33600_e37355_d_n4, assign33600_e37355_d_n6, assign33600_e37355_d_n7, assign33600_e37355_d_n8, assign33600_e37355_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1132 == 0.0)) && (locals.var_guard1133 == 0.0)) {
        let assign33600_e37329: f64 = (-0.5);
        let assign33600_e37331: f64 = (assign33600_e37329 * locals.var_q_d2_qsq__blk827);
        let assign33600_e37333: f64 = (assign33600_e37331 * locals.var_q_temp3__blk816);
        let assign33600_e37336: f64 = (0.25 * 0.0055555555556);
        let assign33600_e37338: f64 = (assign33600_e37336 * locals.var_q_d1_qsq__blk826);
        let assign33600_e37340: f64 = (assign33600_e37338 * locals.var_q_d1_qsq__blk826);
        let assign33600_e37344: f64 = (locals.var_q_qsq__blk825 * 0.0238095238095);
        let assign33600_e37348: f64 = (0.075 * locals.var_q_qsq__blk825);
        let assign33600_e37349: f64 = (2.0 - assign33600_e37348);
        let assign33600_e37350: f64 = (assign33600_e37344 * assign33600_e37349);
        let assign33600_e37351: f64 = (1.0 - assign33600_e37350);
        let assign33600_e37352: f64 = (assign33600_e37340 * assign33600_e37351);
        let assign33600_e37353: f64 = (assign33600_e37333 + assign33600_e37352);
        (assign33600_e37353, ((((assign33600_e37329 * locals.var_q_d2_qsq__blk827_dn4) * locals.var_q_temp3__blk816) + (assign33600_e37331 * locals.var_q_temp3__blk816_dn4)) + (((((assign33600_e37336 * locals.var_q_d1_qsq__blk826_dn4) * locals.var_q_d1_qsq__blk826) + (assign33600_e37338 * locals.var_q_d1_qsq__blk826_dn4)) * assign33600_e37351) + (assign33600_e37340 * (-(((locals.var_q_qsq__blk825_dn4 * 0.0238095238095) * assign33600_e37349) + (assign33600_e37344 * (-(0.075 * locals.var_q_qsq__blk825_dn4)))))))), ((((assign33600_e37329 * locals.var_q_d2_qsq__blk827_dn6) * locals.var_q_temp3__blk816) + (assign33600_e37331 * locals.var_q_temp3__blk816_dn6)) + (((((assign33600_e37336 * locals.var_q_d1_qsq__blk826_dn6) * locals.var_q_d1_qsq__blk826) + (assign33600_e37338 * locals.var_q_d1_qsq__blk826_dn6)) * assign33600_e37351) + (assign33600_e37340 * (-(((locals.var_q_qsq__blk825_dn6 * 0.0238095238095) * assign33600_e37349) + (assign33600_e37344 * (-(0.075 * locals.var_q_qsq__blk825_dn6)))))))), ((((assign33600_e37329 * locals.var_q_d2_qsq__blk827_dn7) * locals.var_q_temp3__blk816) + (assign33600_e37331 * locals.var_q_temp3__blk816_dn7)) + (((((assign33600_e37336 * locals.var_q_d1_qsq__blk826_dn7) * locals.var_q_d1_qsq__blk826) + (assign33600_e37338 * locals.var_q_d1_qsq__blk826_dn7)) * assign33600_e37351) + (assign33600_e37340 * (-(((locals.var_q_qsq__blk825_dn7 * 0.0238095238095) * assign33600_e37349) + (assign33600_e37344 * (-(0.075 * locals.var_q_qsq__blk825_dn7)))))))), ((((assign33600_e37329 * locals.var_q_d2_qsq__blk827_dn8) * locals.var_q_temp3__blk816) + (assign33600_e37331 * locals.var_q_temp3__blk816_dn8)) + (((((assign33600_e37336 * locals.var_q_d1_qsq__blk826_dn8) * locals.var_q_d1_qsq__blk826) + (assign33600_e37338 * locals.var_q_d1_qsq__blk826_dn8)) * assign33600_e37351) + (assign33600_e37340 * (-(((locals.var_q_qsq__blk825_dn8 * 0.0238095238095) * assign33600_e37349) + (assign33600_e37344 * (-(0.075 * locals.var_q_qsq__blk825_dn8)))))))), ((((assign33600_e37329 * locals.var_q_d2_qsq__blk827_dn9) * locals.var_q_temp3__blk816) + (assign33600_e37331 * locals.var_q_temp3__blk816_dn9)) + (((((assign33600_e37336 * locals.var_q_d1_qsq__blk826_dn9) * locals.var_q_d1_qsq__blk826) + (assign33600_e37338 * locals.var_q_d1_qsq__blk826_dn9)) * assign33600_e37351) + (assign33600_e37340 * (-(((locals.var_q_qsq__blk825_dn9 * 0.0238095238095) * assign33600_e37349) + (assign33600_e37344 * (-(0.075 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_d2_ln__blk836, locals.var_q_d2_ln__blk836_dn4, locals.var_q_d2_ln__blk836_dn6, locals.var_q_d2_ln__blk836_dn7, locals.var_q_d2_ln__blk836_dn8, locals.var_q_d2_ln__blk836_dn9,)
    }
};
        locals.var_q_d2_ln__blk836 = assign33600_e37355;
        locals.var_q_d2_ln__blk836_dn4 = assign33600_e37355_d_n4;
        locals.var_q_d2_ln__blk836_dn6 = assign33600_e37355_d_n6;
        locals.var_q_d2_ln__blk836_dn7 = assign33600_e37355_d_n7;
        locals.var_q_d2_ln__blk836_dn8 = assign33600_e37355_d_n8;
        locals.var_q_d2_ln__blk836_dn9 = assign33600_e37355_d_n9;

        let assign33610_e37358: f64 = if locals.var_q_qsq__blk825 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1134 = assign33610_e37358;

        let (assign33620_e37378, assign33620_e37378_d_n4, assign33620_e37378_d_n6, assign33620_e37378_d_n7, assign33620_e37378_d_n8, assign33620_e37378_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1134 != 0.0)) {
        let assign33620_e37368: f64 = (4.0 * locals.var_q_qsq__blk825);
        let assign33620_e37373: f64 = (2.0 - locals.var_q_invexpq__blk831);
        let assign33620_e37374: f64 = (locals.var_q_invexpq__blk831 * assign33620_e37373);
        let assign33620_e37375: f64 = (1.0 - assign33620_e37374);
        let assign33620_e37376: f64 = (assign33620_e37368 / assign33620_e37375);
        (assign33620_e37376, ((((4.0 * locals.var_q_qsq__blk825_dn4) * assign33620_e37375) - (assign33620_e37368 * (-((locals.var_q_invexpq__blk831_dn4 * assign33620_e37373) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign33620_e37375 * assign33620_e37375)), ((((4.0 * locals.var_q_qsq__blk825_dn6) * assign33620_e37375) - (assign33620_e37368 * (-((locals.var_q_invexpq__blk831_dn6 * assign33620_e37373) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign33620_e37375 * assign33620_e37375)), ((((4.0 * locals.var_q_qsq__blk825_dn7) * assign33620_e37375) - (assign33620_e37368 * (-((locals.var_q_invexpq__blk831_dn7 * assign33620_e37373) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign33620_e37375 * assign33620_e37375)), ((((4.0 * locals.var_q_qsq__blk825_dn8) * assign33620_e37375) - (assign33620_e37368 * (-((locals.var_q_invexpq__blk831_dn8 * assign33620_e37373) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign33620_e37375 * assign33620_e37375)), ((((4.0 * locals.var_q_qsq__blk825_dn9) * assign33620_e37375) - (assign33620_e37368 * (-((locals.var_q_invexpq__blk831_dn9 * assign33620_e37373) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign33620_e37375 * assign33620_e37375)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign33620_e37378;
        locals.var_q_temp2__blk815_dn4 = assign33620_e37378_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign33620_e37378_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign33620_e37378_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign33620_e37378_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign33620_e37378_d_n9;

        let (assign33630_e37390, assign33630_e37390_d_n4, assign33630_e37390_d_n6, assign33630_e37390_d_n7, assign33630_e37390_d_n8, assign33630_e37390_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1134 != 0.0)) {
        let assign33630_e37388: f64 = (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831);
        (assign33630_e37388, ((locals.var_q_temp2__blk815_dn4 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn4)), ((locals.var_q_temp2__blk815_dn6 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn6)), ((locals.var_q_temp2__blk815_dn7 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn7)), ((locals.var_q_temp2__blk815_dn8 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn8)), ((locals.var_q_temp2__blk815_dn9 * locals.var_q_invexpq__blk831) + (locals.var_q_temp2__blk815 * locals.var_q_invexpq__blk831_dn9)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign33630_e37390;
        locals.var_q_sh_term__blk833_dn4 = assign33630_e37390_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign33630_e37390_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign33630_e37390_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign33630_e37390_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign33630_e37390_d_n9;

        let (assign33640_e37403, assign33640_e37403_d_n4, assign33640_e37403_d_n6, assign33640_e37403_d_n7, assign33640_e37403_d_n8, assign33640_e37403_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1134 != 0.0)) {
        let assign33640_e37399: f64 = (locals.var_q_temp2__blk815).ln();
        let assign33640_e37401: f64 = (assign33640_e37399 - locals.var_q_rac_qsq__blk828);
        (assign33640_e37401, ((locals.var_q_temp2__blk815_dn4 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn4), ((locals.var_q_temp2__blk815_dn6 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn6), ((locals.var_q_temp2__blk815_dn7 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn7), ((locals.var_q_temp2__blk815_dn8 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn8), ((locals.var_q_temp2__blk815_dn9 / locals.var_q_temp2__blk815) - locals.var_q_rac_qsq__blk828_dn9),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign33640_e37403;
        locals.var_q_ln_term__blk834_dn4 = assign33640_e37403_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign33640_e37403_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign33640_e37403_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign33640_e37403_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign33640_e37403_d_n9;

        let assign33650_e37406: f64 = (-0.005);
        let assign33650_e37407: f64 = if locals.var_q_qsq__blk825 < assign33650_e37406 { 1.0 } else { 0.0 };
        locals.var_guard1135 = assign33650_e37407;

        let (assign33660_e37423, assign33660_e37423_d_n4, assign33660_e37423_d_n6, assign33660_e37423_d_n7, assign33660_e37423_d_n8, assign33660_e37423_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1134 == 0.0)) && (locals.var_guard1135 != 0.0)) {
        let assign33660_e37420: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign33660_e37421: f64 = (assign33660_e37420).sin();
        (assign33660_e37421, ((assign33660_e37420).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign33660_e37420).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign33660_e37420).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign33660_e37420).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign33660_e37420).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign33660_e37423;
        locals.var_q_temp2__blk815_dn4 = assign33660_e37423_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign33660_e37423_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign33660_e37423_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign33660_e37423_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign33660_e37423_d_n9;

        let (assign33670_e37441, assign33670_e37441_d_n4, assign33670_e37441_d_n6, assign33670_e37441_d_n7, assign33670_e37441_d_n8, assign33670_e37441_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1134 == 0.0)) && (locals.var_guard1135 != 0.0)) {
        let assign33670_e37435: f64 = (-locals.var_q_qsq__blk825);
        let assign33670_e37438: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
        let assign33670_e37439: f64 = (assign33670_e37435 / assign33670_e37438);
        (assign33670_e37439, ((((-locals.var_q_qsq__blk825_dn4) * assign33670_e37438) - (assign33670_e37435 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign33670_e37438 * assign33670_e37438)), ((((-locals.var_q_qsq__blk825_dn6) * assign33670_e37438) - (assign33670_e37435 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign33670_e37438 * assign33670_e37438)), ((((-locals.var_q_qsq__blk825_dn7) * assign33670_e37438) - (assign33670_e37435 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign33670_e37438 * assign33670_e37438)), ((((-locals.var_q_qsq__blk825_dn8) * assign33670_e37438) - (assign33670_e37435 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign33670_e37438 * assign33670_e37438)), ((((-locals.var_q_qsq__blk825_dn9) * assign33670_e37438) - (assign33670_e37435 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign33670_e37438 * assign33670_e37438)),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign33670_e37441;
        locals.var_q_sh_term__blk833_dn4 = assign33670_e37441_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign33670_e37441_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign33670_e37441_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign33670_e37441_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign33670_e37441_d_n9;

    }

    pub(super) fn stamp_transient_block_91(
        locals: &mut StampLocals,
    ) {
        let (assign33680_e37455, assign33680_e37455_d_n4, assign33680_e37455_d_n6, assign33680_e37455_d_n7, assign33680_e37455_d_n8, assign33680_e37455_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1134 == 0.0)) && (locals.var_guard1135 != 0.0)) {
        let assign33680_e37453: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign33680_e37453, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign33680_e37455;
        locals.var_q_ln_term__blk834_dn4 = assign33680_e37455_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign33680_e37455_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign33680_e37455_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign33680_e37455_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign33680_e37455_d_n9;

        let (assign33690_e37485, assign33690_e37485_d_n4, assign33690_e37485_d_n6, assign33690_e37485_d_n7, assign33690_e37485_d_n8, assign33690_e37485_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1134 == 0.0)) && (locals.var_guard1135 == 0.0)) {
        let assign33690_e37470: f64 = (locals.var_q_qsq__blk825 * 0.3333333333333);
        let assign33690_e37474: f64 = (0.05 * locals.var_q_qsq__blk825);
        let assign33690_e37478: f64 = (0.0396825396825397 * locals.var_q_qsq__blk825);
        let assign33690_e37479: f64 = (1.0 - assign33690_e37478);
        let assign33690_e37480: f64 = (assign33690_e37474 * assign33690_e37479);
        let assign33690_e37481: f64 = (1.0 - assign33690_e37480);
        let assign33690_e37482: f64 = (assign33690_e37470 * assign33690_e37481);
        let assign33690_e37483: f64 = (4.0 - assign33690_e37482);
        (assign33690_e37483, (-(((locals.var_q_qsq__blk825_dn4 * 0.3333333333333) * assign33690_e37481) + (assign33690_e37470 * (-(((0.05 * locals.var_q_qsq__blk825_dn4) * assign33690_e37479) + (assign33690_e37474 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn4)))))))), (-(((locals.var_q_qsq__blk825_dn6 * 0.3333333333333) * assign33690_e37481) + (assign33690_e37470 * (-(((0.05 * locals.var_q_qsq__blk825_dn6) * assign33690_e37479) + (assign33690_e37474 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn6)))))))), (-(((locals.var_q_qsq__blk825_dn7 * 0.3333333333333) * assign33690_e37481) + (assign33690_e37470 * (-(((0.05 * locals.var_q_qsq__blk825_dn7) * assign33690_e37479) + (assign33690_e37474 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn7)))))))), (-(((locals.var_q_qsq__blk825_dn8 * 0.3333333333333) * assign33690_e37481) + (assign33690_e37470 * (-(((0.05 * locals.var_q_qsq__blk825_dn8) * assign33690_e37479) + (assign33690_e37474 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn8)))))))), (-(((locals.var_q_qsq__blk825_dn9 * 0.3333333333333) * assign33690_e37481) + (assign33690_e37470 * (-(((0.05 * locals.var_q_qsq__blk825_dn9) * assign33690_e37479) + (assign33690_e37474 * (-(0.0396825396825397 * locals.var_q_qsq__blk825_dn9)))))))),)
    } else {
        (locals.var_q_sh_term__blk833, locals.var_q_sh_term__blk833_dn4, locals.var_q_sh_term__blk833_dn6, locals.var_q_sh_term__blk833_dn7, locals.var_q_sh_term__blk833_dn8, locals.var_q_sh_term__blk833_dn9,)
    }
};
        locals.var_q_sh_term__blk833 = assign33690_e37485;
        locals.var_q_sh_term__blk833_dn4 = assign33690_e37485_d_n4;
        locals.var_q_sh_term__blk833_dn6 = assign33690_e37485_d_n6;
        locals.var_q_sh_term__blk833_dn7 = assign33690_e37485_d_n7;
        locals.var_q_sh_term__blk833_dn8 = assign33690_e37485_d_n8;
        locals.var_q_sh_term__blk833_dn9 = assign33690_e37485_d_n9;

        let (assign33700_e37500, assign33700_e37500_d_n4, assign33700_e37500_d_n6, assign33700_e37500_d_n7, assign33700_e37500_d_n8, assign33700_e37500_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1134 == 0.0)) && (locals.var_guard1135 == 0.0)) {
        let assign33700_e37498: f64 = (locals.var_q_sh_term__blk833).ln();
        (assign33700_e37498, (locals.var_q_sh_term__blk833_dn4 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn6 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn7 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn8 / locals.var_q_sh_term__blk833), (locals.var_q_sh_term__blk833_dn9 / locals.var_q_sh_term__blk833),)
    } else {
        (locals.var_q_ln_term__blk834, locals.var_q_ln_term__blk834_dn4, locals.var_q_ln_term__blk834_dn6, locals.var_q_ln_term__blk834_dn7, locals.var_q_ln_term__blk834_dn8, locals.var_q_ln_term__blk834_dn9,)
    }
};
        locals.var_q_ln_term__blk834 = assign33700_e37500;
        locals.var_q_ln_term__blk834_dn4 = assign33700_e37500_d_n4;
        locals.var_q_ln_term__blk834_dn6 = assign33700_e37500_d_n6;
        locals.var_q_ln_term__blk834_dn7 = assign33700_e37500_d_n7;
        locals.var_q_ln_term__blk834_dn8 = assign33700_e37500_d_n8;
        locals.var_q_ln_term__blk834_dn9 = assign33700_e37500_d_n9;

        let assign33710_e37503: f64 = (1.01 * locals.var_q_k1q1__blk823);
        let assign33710_e37505: f64 = (assign33710_e37503 + locals.var_q_qcoth__blk829);
        let assign33710_e37507: f64 = if assign33710_e37505 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1136 = assign33710_e37507;

        let (assign33720_e37519, assign33720_e37519_d_n4, assign33720_e37519_d_n6, assign33720_e37519_d_n7, assign33720_e37519_d_n8, assign33720_e37519_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1136 != 0.0)) {
        let assign33720_e37517: f64 = (locals.var_q_k1q1__blk823 + locals.var_q_qcoth__blk829);
        (assign33720_e37517, (locals.var_q_k1q1__blk823_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_q_k1q1__blk823_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_q_k1q1__blk823_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_q_k1q1__blk823_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_q_k1q1__blk823_dn9 + locals.var_q_qcoth__blk829_dn9),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign33720_e37519;
        locals.var_q_expnum__blk837_dn4 = assign33720_e37519_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign33720_e37519_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign33720_e37519_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign33720_e37519_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign33720_e37519_d_n9;

        let (assign33730_e37531, assign33730_e37531_d_n4, assign33730_e37531_d_n6, assign33730_e37531_d_n7, assign33730_e37531_d_n8, assign33730_e37531_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1136 != 0.0)) {
        let assign33730_e37529: f64 = (locals.var_k1__blk932 + locals.var_q_d1_qcoth__blk830);
        (assign33730_e37529, (locals.var_k1__blk932_dn4 + locals.var_q_d1_qcoth__blk830_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_d1_qcoth__blk830_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_d1_qcoth__blk830_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_d1_qcoth__blk830_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_d1_qcoth__blk830_dn9),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign33730_e37531;
        locals.var_q_d1_expnum__blk838_dn4 = assign33730_e37531_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign33730_e37531_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign33730_e37531_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign33730_e37531_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign33730_e37531_d_n9;

        let (assign33740_e37541, assign33740_e37541_d_n4, assign33740_e37541_d_n6, assign33740_e37541_d_n7, assign33740_e37541_d_n8, assign33740_e37541_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1136 != 0.0)) {
        (locals.var_q_d2_qcoth__blk832, locals.var_q_d2_qcoth__blk832_dn4, locals.var_q_d2_qcoth__blk832_dn6, locals.var_q_d2_qcoth__blk832_dn7, locals.var_q_d2_qcoth__blk832_dn8, locals.var_q_d2_qcoth__blk832_dn9,)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign33740_e37541;
        locals.var_q_d2_expnum__blk839_dn4 = assign33740_e37541_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign33740_e37541_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign33740_e37541_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign33740_e37541_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign33740_e37541_d_n9;

        let (assign33750_e37556, assign33750_e37556_d_n4, assign33750_e37556_d_n6, assign33750_e37556_d_n7, assign33750_e37556_d_n8, assign33750_e37556_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1136 == 0.0)) {
        let assign33750_e37553: f64 = (locals.var_q_k1q1__blk823 - locals.var_q_qcoth__blk829);
        let assign33750_e37554: f64 = (1.0 / assign33750_e37553);
        (assign33750_e37554, (-((locals.var_q_k1q1__blk823_dn4 - locals.var_q_qcoth__blk829_dn4) / (assign33750_e37553 * assign33750_e37553))), (-((locals.var_q_k1q1__blk823_dn6 - locals.var_q_qcoth__blk829_dn6) / (assign33750_e37553 * assign33750_e37553))), (-((locals.var_q_k1q1__blk823_dn7 - locals.var_q_qcoth__blk829_dn7) / (assign33750_e37553 * assign33750_e37553))), (-((locals.var_q_k1q1__blk823_dn8 - locals.var_q_qcoth__blk829_dn8) / (assign33750_e37553 * assign33750_e37553))), (-((locals.var_q_k1q1__blk823_dn9 - locals.var_q_qcoth__blk829_dn9) / (assign33750_e37553 * assign33750_e37553))),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign33750_e37556;
        locals.var_q_temp2__blk815_dn4 = assign33750_e37556_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign33750_e37556_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign33750_e37556_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign33750_e37556_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign33750_e37556_d_n9;

        let (assign33760_e37569, assign33760_e37569_d_n4, assign33760_e37569_d_n6, assign33760_e37569_d_n7, assign33760_e37569_d_n8, assign33760_e37569_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1136 == 0.0)) {
        let assign33760_e37567: f64 = (locals.var_q_d1_qcoth__blk830 - locals.var_k1__blk932);
        (assign33760_e37567, (locals.var_q_d1_qcoth__blk830_dn4 - locals.var_k1__blk932_dn4), (locals.var_q_d1_qcoth__blk830_dn6 - locals.var_k1__blk932_dn6), (locals.var_q_d1_qcoth__blk830_dn7 - locals.var_k1__blk932_dn7), (locals.var_q_d1_qcoth__blk830_dn8 - locals.var_k1__blk932_dn8), (locals.var_q_d1_qcoth__blk830_dn9 - locals.var_k1__blk932_dn9),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign33760_e37569;
        locals.var_q_temp3__blk816_dn4 = assign33760_e37569_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign33760_e37569_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign33760_e37569_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign33760_e37569_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign33760_e37569_d_n9;

        let (assign33770_e37584, assign33770_e37584_d_n4, assign33770_e37584_d_n6, assign33770_e37584_d_n7, assign33770_e37584_d_n8, assign33770_e37584_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1136 == 0.0)) {
        let assign33770_e37580: f64 = (locals.var_q_aexp__blk824 - locals.var_q_sh_term__blk833);
        let assign33770_e37582: f64 = (assign33770_e37580 * locals.var_q_temp2__blk815);
        (assign33770_e37582, (((locals.var_q_aexp__blk824_dn4 - locals.var_q_sh_term__blk833_dn4) * locals.var_q_temp2__blk815) + (assign33770_e37580 * locals.var_q_temp2__blk815_dn4)), (((locals.var_q_aexp__blk824_dn6 - locals.var_q_sh_term__blk833_dn6) * locals.var_q_temp2__blk815) + (assign33770_e37580 * locals.var_q_temp2__blk815_dn6)), (((locals.var_q_aexp__blk824_dn7 - locals.var_q_sh_term__blk833_dn7) * locals.var_q_temp2__blk815) + (assign33770_e37580 * locals.var_q_temp2__blk815_dn7)), (((locals.var_q_aexp__blk824_dn8 - locals.var_q_sh_term__blk833_dn8) * locals.var_q_temp2__blk815) + (assign33770_e37580 * locals.var_q_temp2__blk815_dn8)), (((locals.var_q_aexp__blk824_dn9 - locals.var_q_sh_term__blk833_dn9) * locals.var_q_temp2__blk815) + (assign33770_e37580 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_expnum__blk837, locals.var_q_expnum__blk837_dn4, locals.var_q_expnum__blk837_dn6, locals.var_q_expnum__blk837_dn7, locals.var_q_expnum__blk837_dn8, locals.var_q_expnum__blk837_dn9,)
    }
};
        locals.var_q_expnum__blk837 = assign33770_e37584;
        locals.var_q_expnum__blk837_dn4 = assign33770_e37584_d_n4;
        locals.var_q_expnum__blk837_dn6 = assign33770_e37584_d_n6;
        locals.var_q_expnum__blk837_dn7 = assign33770_e37584_d_n7;
        locals.var_q_expnum__blk837_dn8 = assign33770_e37584_d_n8;
        locals.var_q_expnum__blk837_dn9 = assign33770_e37584_d_n9;

        let (assign33780_e37605, assign33780_e37605_d_n4, assign33780_e37605_d_n6, assign33780_e37605_d_n7, assign33780_e37605_d_n8, assign33780_e37605_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1136 == 0.0)) {
        let assign33780_e37595: f64 = (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837);
        let assign33780_e37597: f64 = (assign33780_e37595 - locals.var_q_aexp__blk824);
        let assign33780_e37600: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833);
        let assign33780_e37601: f64 = (assign33780_e37597 - assign33780_e37600);
        let assign33780_e37603: f64 = (assign33780_e37601 * locals.var_q_temp2__blk815);
        (assign33780_e37603, ((((((locals.var_q_temp3__blk816_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4) - ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign33780_e37601 * locals.var_q_temp2__blk815_dn4)), ((((((locals.var_q_temp3__blk816_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6) - ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign33780_e37601 * locals.var_q_temp2__blk815_dn6)), ((((((locals.var_q_temp3__blk816_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7) - ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign33780_e37601 * locals.var_q_temp2__blk815_dn7)), ((((((locals.var_q_temp3__blk816_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8) - ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign33780_e37601 * locals.var_q_temp2__blk815_dn8)), ((((((locals.var_q_temp3__blk816_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_temp3__blk816 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9) - ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_sh_term__blk833) + (locals.var_q_d1_ln__blk835 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign33780_e37601 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d1_expnum__blk838, locals.var_q_d1_expnum__blk838_dn4, locals.var_q_d1_expnum__blk838_dn6, locals.var_q_d1_expnum__blk838_dn7, locals.var_q_d1_expnum__blk838_dn8, locals.var_q_d1_expnum__blk838_dn9,)
    }
};
        locals.var_q_d1_expnum__blk838 = assign33780_e37605;
        locals.var_q_d1_expnum__blk838_dn4 = assign33780_e37605_d_n4;
        locals.var_q_d1_expnum__blk838_dn6 = assign33780_e37605_d_n6;
        locals.var_q_d1_expnum__blk838_dn7 = assign33780_e37605_d_n7;
        locals.var_q_d1_expnum__blk838_dn8 = assign33780_e37605_d_n8;
        locals.var_q_d1_expnum__blk838_dn9 = assign33780_e37605_d_n9;

        let (assign33790_e37636, assign33790_e37636_d_n4, assign33790_e37636_d_n6, assign33790_e37636_d_n7, assign33790_e37636_d_n8, assign33790_e37636_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1136 == 0.0)) {
        let assign33790_e37616: f64 = (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837);
        let assign33790_e37619: f64 = (2.0 * locals.var_q_temp3__blk816);
        let assign33790_e37621: f64 = (assign33790_e37619 * locals.var_q_d1_expnum__blk838);
        let assign33790_e37622: f64 = (assign33790_e37616 + assign33790_e37621);
        let assign33790_e37624: f64 = (assign33790_e37622 + locals.var_q_aexp__blk824);
        let assign33790_e37628: f64 = (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835);
        let assign33790_e37629: f64 = (locals.var_q_d2_ln__blk836 + assign33790_e37628);
        let assign33790_e37631: f64 = (assign33790_e37629 * locals.var_q_sh_term__blk833);
        let assign33790_e37632: f64 = (assign33790_e37624 - assign33790_e37631);
        let assign33790_e37634: f64 = (assign33790_e37632 * locals.var_q_temp2__blk815);
        (assign33790_e37634, (((((((locals.var_q_d2_qcoth__blk832_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_temp3__blk816_dn4) * locals.var_q_d1_expnum__blk838) + (assign33790_e37619 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4) - (((locals.var_q_d2_ln__blk836_dn4 + ((locals.var_q_d1_ln__blk835_dn4 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn4))) * locals.var_q_sh_term__blk833) + (assign33790_e37629 * locals.var_q_sh_term__blk833_dn4))) * locals.var_q_temp2__blk815) + (assign33790_e37632 * locals.var_q_temp2__blk815_dn4)), (((((((locals.var_q_d2_qcoth__blk832_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_temp3__blk816_dn6) * locals.var_q_d1_expnum__blk838) + (assign33790_e37619 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6) - (((locals.var_q_d2_ln__blk836_dn6 + ((locals.var_q_d1_ln__blk835_dn6 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn6))) * locals.var_q_sh_term__blk833) + (assign33790_e37629 * locals.var_q_sh_term__blk833_dn6))) * locals.var_q_temp2__blk815) + (assign33790_e37632 * locals.var_q_temp2__blk815_dn6)), (((((((locals.var_q_d2_qcoth__blk832_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_temp3__blk816_dn7) * locals.var_q_d1_expnum__blk838) + (assign33790_e37619 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7) - (((locals.var_q_d2_ln__blk836_dn7 + ((locals.var_q_d1_ln__blk835_dn7 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn7))) * locals.var_q_sh_term__blk833) + (assign33790_e37629 * locals.var_q_sh_term__blk833_dn7))) * locals.var_q_temp2__blk815) + (assign33790_e37632 * locals.var_q_temp2__blk815_dn7)), (((((((locals.var_q_d2_qcoth__blk832_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_temp3__blk816_dn8) * locals.var_q_d1_expnum__blk838) + (assign33790_e37619 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8) - (((locals.var_q_d2_ln__blk836_dn8 + ((locals.var_q_d1_ln__blk835_dn8 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn8))) * locals.var_q_sh_term__blk833) + (assign33790_e37629 * locals.var_q_sh_term__blk833_dn8))) * locals.var_q_temp2__blk815) + (assign33790_e37632 * locals.var_q_temp2__blk815_dn8)), (((((((locals.var_q_d2_qcoth__blk832_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qcoth__blk832 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_temp3__blk816_dn9) * locals.var_q_d1_expnum__blk838) + (assign33790_e37619 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9) - (((locals.var_q_d2_ln__blk836_dn9 + ((locals.var_q_d1_ln__blk835_dn9 * locals.var_q_d1_ln__blk835) + (locals.var_q_d1_ln__blk835 * locals.var_q_d1_ln__blk835_dn9))) * locals.var_q_sh_term__blk833) + (assign33790_e37629 * locals.var_q_sh_term__blk833_dn9))) * locals.var_q_temp2__blk815) + (assign33790_e37632 * locals.var_q_temp2__blk815_dn9)),)
    } else {
        (locals.var_q_d2_expnum__blk839, locals.var_q_d2_expnum__blk839_dn4, locals.var_q_d2_expnum__blk839_dn6, locals.var_q_d2_expnum__blk839_dn7, locals.var_q_d2_expnum__blk839_dn8, locals.var_q_d2_expnum__blk839_dn9,)
    }
};
        locals.var_q_d2_expnum__blk839 = assign33790_e37636;
        locals.var_q_d2_expnum__blk839_dn4 = assign33790_e37636_d_n4;
        locals.var_q_d2_expnum__blk839_dn6 = assign33790_e37636_d_n6;
        locals.var_q_d2_expnum__blk839_dn7 = assign33790_e37636_d_n7;
        locals.var_q_d2_expnum__blk839_dn8 = assign33790_e37636_d_n8;
        locals.var_q_d2_expnum__blk839_dn9 = assign33790_e37636_d_n9;

        let assign33800_e37639: f64 = if locals.var_q_expnum__blk837 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1137 = assign33800_e37639;

        let (assign33810_e37650, assign33810_e37650_d_n4, assign33810_e37650_d_n6, assign33810_e37650_d_n7, assign33810_e37650_d_n8, assign33810_e37650_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1137 != 0.0)) {
        let assign33810_e37648: f64 = (locals.var_q_expnum__blk837).ln();
        (assign33810_e37648, (locals.var_q_expnum__blk837_dn4 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn6 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn7 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn8 / locals.var_q_expnum__blk837), (locals.var_q_expnum__blk837_dn9 / locals.var_q_expnum__blk837),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign33810_e37650;
        locals.var_q_lnexpnum__blk840_dn4 = assign33810_e37650_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign33810_e37650_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign33810_e37650_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign33810_e37650_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign33810_e37650_d_n9;

        let (assign33820_e37662, assign33820_e37662_d_n4, assign33820_e37662_d_n6, assign33820_e37662_d_n7, assign33820_e37662_d_n8, assign33820_e37662_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1137 != 0.0)) {
        let assign33820_e37660: f64 = (1.0 / locals.var_q_expnum__blk837);
        (assign33820_e37660, (-(locals.var_q_expnum__blk837_dn4 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn6 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn7 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn8 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))), (-(locals.var_q_expnum__blk837_dn9 / (locals.var_q_expnum__blk837 * locals.var_q_expnum__blk837))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign33820_e37662;
        locals.var_q_temp1__blk814_dn4 = assign33820_e37662_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign33820_e37662_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign33820_e37662_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign33820_e37662_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign33820_e37662_d_n9;

        let (assign33830_e37674, assign33830_e37674_d_n4, assign33830_e37674_d_n6, assign33830_e37674_d_n7, assign33830_e37674_d_n8, assign33830_e37674_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1137 != 0.0)) {
        let assign33830_e37672: f64 = (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814);
        (assign33830_e37672, ((locals.var_q_d1_expnum__blk838_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn4)), ((locals.var_q_d1_expnum__blk838_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn6)), ((locals.var_q_d1_expnum__blk838_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn7)), ((locals.var_q_d1_expnum__blk838_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn8)), ((locals.var_q_d1_expnum__blk838_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d1_expnum__blk838 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign33830_e37674;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign33830_e37674_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign33830_e37674_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign33830_e37674_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign33830_e37674_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign33830_e37674_d_n9;

        let (assign33840_e37690, assign33840_e37690_d_n4, assign33840_e37690_d_n6, assign33840_e37690_d_n7, assign33840_e37690_d_n8, assign33840_e37690_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1137 != 0.0)) {
        let assign33840_e37684: f64 = (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814);
        let assign33840_e37687: f64 = (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841);
        let assign33840_e37688: f64 = (assign33840_e37684 - assign33840_e37687);
        (assign33840_e37688, (((locals.var_q_d2_expnum__blk839_dn4 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn4)) - ((locals.var_q_d1_lnexpnum__blk841_dn4 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn4))), (((locals.var_q_d2_expnum__blk839_dn6 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn6)) - ((locals.var_q_d1_lnexpnum__blk841_dn6 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn6))), (((locals.var_q_d2_expnum__blk839_dn7 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn7)) - ((locals.var_q_d1_lnexpnum__blk841_dn7 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn7))), (((locals.var_q_d2_expnum__blk839_dn8 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn8)) - ((locals.var_q_d1_lnexpnum__blk841_dn8 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn8))), (((locals.var_q_d2_expnum__blk839_dn9 * locals.var_q_temp1__blk814) + (locals.var_q_d2_expnum__blk839 * locals.var_q_temp1__blk814_dn9)) - ((locals.var_q_d1_lnexpnum__blk841_dn9 * locals.var_q_d1_lnexpnum__blk841) + (locals.var_q_d1_lnexpnum__blk841 * locals.var_q_d1_lnexpnum__blk841_dn9))),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign33840_e37690;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign33840_e37690_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign33840_e37690_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign33840_e37690_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign33840_e37690_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign33840_e37690_d_n9;

        let (assign33850_e37707, assign33850_e37707_d_n4, assign33850_e37707_d_n6, assign33850_e37707_d_n7, assign33850_e37707_d_n8, assign33850_e37707_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1137 == 0.0)) {
        let assign33850_e37701: f64 = (locals.var_q_k1q1__blk823 + 0.6931471805599);
        let assign33850_e37703: f64 = (-locals.var_q_k1q1__blk823);
        let assign33850_e37704: f64 = (assign33850_e37703).ln();
        let assign33850_e37705: f64 = (assign33850_e37701 + assign33850_e37704);
        (assign33850_e37705, (locals.var_q_k1q1__blk823_dn4 + ((-locals.var_q_k1q1__blk823_dn4) / assign33850_e37703)), (locals.var_q_k1q1__blk823_dn6 + ((-locals.var_q_k1q1__blk823_dn6) / assign33850_e37703)), (locals.var_q_k1q1__blk823_dn7 + ((-locals.var_q_k1q1__blk823_dn7) / assign33850_e37703)), (locals.var_q_k1q1__blk823_dn8 + ((-locals.var_q_k1q1__blk823_dn8) / assign33850_e37703)), (locals.var_q_k1q1__blk823_dn9 + ((-locals.var_q_k1q1__blk823_dn9) / assign33850_e37703)),)
    } else {
        (locals.var_q_lnexpnum__blk840, locals.var_q_lnexpnum__blk840_dn4, locals.var_q_lnexpnum__blk840_dn6, locals.var_q_lnexpnum__blk840_dn7, locals.var_q_lnexpnum__blk840_dn8, locals.var_q_lnexpnum__blk840_dn9,)
    }
};
        locals.var_q_lnexpnum__blk840 = assign33850_e37707;
        locals.var_q_lnexpnum__blk840_dn4 = assign33850_e37707_d_n4;
        locals.var_q_lnexpnum__blk840_dn6 = assign33850_e37707_d_n6;
        locals.var_q_lnexpnum__blk840_dn7 = assign33850_e37707_d_n7;
        locals.var_q_lnexpnum__blk840_dn8 = assign33850_e37707_d_n8;
        locals.var_q_lnexpnum__blk840_dn9 = assign33850_e37707_d_n9;

        let (assign33860_e37720, assign33860_e37720_d_n4, assign33860_e37720_d_n6, assign33860_e37720_d_n7, assign33860_e37720_d_n8, assign33860_e37720_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1137 == 0.0)) {
        let assign33860_e37718: f64 = (1.0 / locals.var_q1s__blk937);
        (assign33860_e37718, (-(locals.var_q1s__blk937_dn4 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))), (-(locals.var_q1s__blk937_dn6 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))), (-(locals.var_q1s__blk937_dn7 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))), (-(locals.var_q1s__blk937_dn8 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))), (-(locals.var_q1s__blk937_dn9 / (locals.var_q1s__blk937 * locals.var_q1s__blk937))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign33860_e37720;
        locals.var_q_temp1__blk814_dn4 = assign33860_e37720_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign33860_e37720_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign33860_e37720_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign33860_e37720_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign33860_e37720_d_n9;

        let (assign33870_e37733, assign33870_e37733_d_n4, assign33870_e37733_d_n6, assign33870_e37733_d_n7, assign33870_e37733_d_n8, assign33870_e37733_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1137 == 0.0)) {
        let assign33870_e37731: f64 = (locals.var_k1__blk932 + locals.var_q_temp1__blk814);
        (assign33870_e37731, (locals.var_k1__blk932_dn4 + locals.var_q_temp1__blk814_dn4), (locals.var_k1__blk932_dn6 + locals.var_q_temp1__blk814_dn6), (locals.var_k1__blk932_dn7 + locals.var_q_temp1__blk814_dn7), (locals.var_k1__blk932_dn8 + locals.var_q_temp1__blk814_dn8), (locals.var_k1__blk932_dn9 + locals.var_q_temp1__blk814_dn9),)
    } else {
        (locals.var_q_d1_lnexpnum__blk841, locals.var_q_d1_lnexpnum__blk841_dn4, locals.var_q_d1_lnexpnum__blk841_dn6, locals.var_q_d1_lnexpnum__blk841_dn7, locals.var_q_d1_lnexpnum__blk841_dn8, locals.var_q_d1_lnexpnum__blk841_dn9,)
    }
};
        locals.var_q_d1_lnexpnum__blk841 = assign33870_e37733;
        locals.var_q_d1_lnexpnum__blk841_dn4 = assign33870_e37733_d_n4;
        locals.var_q_d1_lnexpnum__blk841_dn6 = assign33870_e37733_d_n6;
        locals.var_q_d1_lnexpnum__blk841_dn7 = assign33870_e37733_d_n7;
        locals.var_q_d1_lnexpnum__blk841_dn8 = assign33870_e37733_d_n8;
        locals.var_q_d1_lnexpnum__blk841_dn9 = assign33870_e37733_d_n9;

        let (assign33880_e37747, assign33880_e37747_d_n4, assign33880_e37747_d_n6, assign33880_e37747_d_n7, assign33880_e37747_d_n8, assign33880_e37747_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) && (locals.var_guard1137 == 0.0)) {
        let assign33880_e37743: f64 = (-locals.var_q_temp1__blk814);
        let assign33880_e37745: f64 = (assign33880_e37743 * locals.var_q_temp1__blk814);
        (assign33880_e37745, (((-locals.var_q_temp1__blk814_dn4) * locals.var_q_temp1__blk814) + (assign33880_e37743 * locals.var_q_temp1__blk814_dn4)), (((-locals.var_q_temp1__blk814_dn6) * locals.var_q_temp1__blk814) + (assign33880_e37743 * locals.var_q_temp1__blk814_dn6)), (((-locals.var_q_temp1__blk814_dn7) * locals.var_q_temp1__blk814) + (assign33880_e37743 * locals.var_q_temp1__blk814_dn7)), (((-locals.var_q_temp1__blk814_dn8) * locals.var_q_temp1__blk814) + (assign33880_e37743 * locals.var_q_temp1__blk814_dn8)), (((-locals.var_q_temp1__blk814_dn9) * locals.var_q_temp1__blk814) + (assign33880_e37743 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_q_d2_lnexpnum__blk842, locals.var_q_d2_lnexpnum__blk842_dn4, locals.var_q_d2_lnexpnum__blk842_dn6, locals.var_q_d2_lnexpnum__blk842_dn7, locals.var_q_d2_lnexpnum__blk842_dn8, locals.var_q_d2_lnexpnum__blk842_dn9,)
    }
};
        locals.var_q_d2_lnexpnum__blk842 = assign33880_e37747;
        locals.var_q_d2_lnexpnum__blk842_dn4 = assign33880_e37747_d_n4;
        locals.var_q_d2_lnexpnum__blk842_dn6 = assign33880_e37747_d_n6;
        locals.var_q_d2_lnexpnum__blk842_dn7 = assign33880_e37747_d_n7;
        locals.var_q_d2_lnexpnum__blk842_dn8 = assign33880_e37747_d_n8;
        locals.var_q_d2_lnexpnum__blk842_dn9 = assign33880_e37747_d_n9;

        let (assign33890_e37765, assign33890_e37765_d_n4, assign33890_e37765_d_n6, assign33890_e37765_d_n7, assign33890_e37765_d_n8, assign33890_e37765_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) {
        let assign33890_e37755: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
        let assign33890_e37757: f64 = (assign33890_e37755 + locals.var_q1s__blk937);
        let assign33890_e37760: f64 = (2.0 * locals.var_q_lnexpnum__blk840);
        let assign33890_e37761: f64 = (assign33890_e37757 + assign33890_e37760);
        let assign33890_e37763: f64 = (assign33890_e37761 - locals.var_q_ln_term__blk834);
        (assign33890_e37763, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1s__blk937_dn4) + (2.0 * locals.var_q_lnexpnum__blk840_dn4)) - locals.var_q_ln_term__blk834_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1s__blk937_dn6) + (2.0 * locals.var_q_lnexpnum__blk840_dn6)) - locals.var_q_ln_term__blk834_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1s__blk937_dn7) + (2.0 * locals.var_q_lnexpnum__blk840_dn7)) - locals.var_q_ln_term__blk834_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1s__blk937_dn8) + (2.0 * locals.var_q_lnexpnum__blk840_dn8)) - locals.var_q_ln_term__blk834_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1s__blk937_dn9) + (2.0 * locals.var_q_lnexpnum__blk840_dn9)) - locals.var_q_ln_term__blk834_dn9),)
    } else {
        (locals.var_q_q2_int__blk843, locals.var_q_q2_int__blk843_dn4, locals.var_q_q2_int__blk843_dn6, locals.var_q_q2_int__blk843_dn7, locals.var_q_q2_int__blk843_dn8, locals.var_q_q2_int__blk843_dn9,)
    }
};
        locals.var_q_q2_int__blk843 = assign33890_e37765;
        locals.var_q_q2_int__blk843_dn4 = assign33890_e37765_d_n4;
        locals.var_q_q2_int__blk843_dn6 = assign33890_e37765_d_n6;
        locals.var_q_q2_int__blk843_dn7 = assign33890_e37765_d_n7;
        locals.var_q_q2_int__blk843_dn8 = assign33890_e37765_d_n8;
        locals.var_q_q2_int__blk843_dn9 = assign33890_e37765_d_n9;

        let (assign33900_e37779, assign33900_e37779_d_n4, assign33900_e37779_d_n6, assign33900_e37779_d_n7, assign33900_e37779_d_n8, assign33900_e37779_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) {
        let assign33900_e37774: f64 = (2.0 * locals.var_q_d1_lnexpnum__blk841);
        let assign33900_e37775: f64 = (1.0 + assign33900_e37774);
        let assign33900_e37777: f64 = (assign33900_e37775 - locals.var_q_d1_ln__blk835);
        (assign33900_e37777, ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn4) - locals.var_q_d1_ln__blk835_dn4), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn6) - locals.var_q_d1_ln__blk835_dn6), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn7) - locals.var_q_d1_ln__blk835_dn7), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn8) - locals.var_q_d1_ln__blk835_dn8), ((2.0 * locals.var_q_d1_lnexpnum__blk841_dn9) - locals.var_q_d1_ln__blk835_dn9),)
    } else {
        (locals.var_q_d1_q2__blk844, locals.var_q_d1_q2__blk844_dn4, locals.var_q_d1_q2__blk844_dn6, locals.var_q_d1_q2__blk844_dn7, locals.var_q_d1_q2__blk844_dn8, locals.var_q_d1_q2__blk844_dn9,)
    }
};
        locals.var_q_d1_q2__blk844 = assign33900_e37779;
        locals.var_q_d1_q2__blk844_dn4 = assign33900_e37779_d_n4;
        locals.var_q_d1_q2__blk844_dn6 = assign33900_e37779_d_n6;
        locals.var_q_d1_q2__blk844_dn7 = assign33900_e37779_d_n7;
        locals.var_q_d1_q2__blk844_dn8 = assign33900_e37779_d_n8;
        locals.var_q_d1_q2__blk844_dn9 = assign33900_e37779_d_n9;

        let (assign33910_e37791, assign33910_e37791_d_n4, assign33910_e37791_d_n6, assign33910_e37791_d_n7, assign33910_e37791_d_n8, assign33910_e37791_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) {
        let assign33910_e37787: f64 = (2.0 * locals.var_q_d2_lnexpnum__blk842);
        let assign33910_e37789: f64 = (assign33910_e37787 - locals.var_q_d2_ln__blk836);
        (assign33910_e37789, ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn4) - locals.var_q_d2_ln__blk836_dn4), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn6) - locals.var_q_d2_ln__blk836_dn6), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn7) - locals.var_q_d2_ln__blk836_dn7), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn8) - locals.var_q_d2_ln__blk836_dn8), ((2.0 * locals.var_q_d2_lnexpnum__blk842_dn9) - locals.var_q_d2_ln__blk836_dn9),)
    } else {
        (locals.var_q_d2_q2__blk845, locals.var_q_d2_q2__blk845_dn4, locals.var_q_d2_q2__blk845_dn6, locals.var_q_d2_q2__blk845_dn7, locals.var_q_d2_q2__blk845_dn8, locals.var_q_d2_q2__blk845_dn9,)
    }
};
        locals.var_q_d2_q2__blk845 = assign33910_e37791;
        locals.var_q_d2_q2__blk845_dn4 = assign33910_e37791_d_n4;
        locals.var_q_d2_q2__blk845_dn6 = assign33910_e37791_d_n6;
        locals.var_q_d2_q2__blk845_dn7 = assign33910_e37791_d_n7;
        locals.var_q_d2_q2__blk845_dn8 = assign33910_e37791_d_n8;
        locals.var_q_d2_q2__blk845_dn9 = assign33910_e37791_d_n9;

        let (assign33920_e37803, assign33920_e37803_d_n4, assign33920_e37803_d_n6, assign33920_e37803_d_n7, assign33920_e37803_d_n8, assign33920_e37803_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) {
        let assign33920_e37800: f64 = (locals.var_k2__blk933 * locals.var_q_q2_int__blk843);
        let assign33920_e37801: f64 = (locals.var_q_k1q1__blk823 + assign33920_e37800);
        (assign33920_e37801, (locals.var_q_k1q1__blk823_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn4))), (locals.var_q_k1q1__blk823_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn6))), (locals.var_q_k1q1__blk823_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn7))), (locals.var_q_k1q1__blk823_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn8))), (locals.var_q_k1q1__blk823_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_q2_int__blk843) + (locals.var_k2__blk933 * locals.var_q_q2_int__blk843_dn9))),)
    } else {
        (locals.var_q_qi_int__blk846, locals.var_q_qi_int__blk846_dn4, locals.var_q_qi_int__blk846_dn6, locals.var_q_qi_int__blk846_dn7, locals.var_q_qi_int__blk846_dn8, locals.var_q_qi_int__blk846_dn9,)
    }
};
        locals.var_q_qi_int__blk846 = assign33920_e37803;
        locals.var_q_qi_int__blk846_dn4 = assign33920_e37803_d_n4;
        locals.var_q_qi_int__blk846_dn6 = assign33920_e37803_d_n6;
        locals.var_q_qi_int__blk846_dn7 = assign33920_e37803_d_n7;
        locals.var_q_qi_int__blk846_dn8 = assign33920_e37803_d_n8;
        locals.var_q_qi_int__blk846_dn9 = assign33920_e37803_d_n9;

        let (assign33930_e37815, assign33930_e37815_d_n4, assign33930_e37815_d_n6, assign33930_e37815_d_n7, assign33930_e37815_d_n8, assign33930_e37815_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) {
        let assign33930_e37812: f64 = (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844);
        let assign33930_e37813: f64 = (locals.var_k1__blk932 + assign33930_e37812);
        (assign33930_e37813, (locals.var_k1__blk932_dn4 + ((locals.var_k2__blk933_dn4 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn4))), (locals.var_k1__blk932_dn6 + ((locals.var_k2__blk933_dn6 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn6))), (locals.var_k1__blk932_dn7 + ((locals.var_k2__blk933_dn7 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn7))), (locals.var_k1__blk932_dn8 + ((locals.var_k2__blk933_dn8 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn8))), (locals.var_k1__blk932_dn9 + ((locals.var_k2__blk933_dn9 * locals.var_q_d1_q2__blk844) + (locals.var_k2__blk933 * locals.var_q_d1_q2__blk844_dn9))),)
    } else {
        (locals.var_q_d1_qi__blk847, locals.var_q_d1_qi__blk847_dn4, locals.var_q_d1_qi__blk847_dn6, locals.var_q_d1_qi__blk847_dn7, locals.var_q_d1_qi__blk847_dn8, locals.var_q_d1_qi__blk847_dn9,)
    }
};
        locals.var_q_d1_qi__blk847 = assign33930_e37815;
        locals.var_q_d1_qi__blk847_dn4 = assign33930_e37815_d_n4;
        locals.var_q_d1_qi__blk847_dn6 = assign33930_e37815_d_n6;
        locals.var_q_d1_qi__blk847_dn7 = assign33930_e37815_d_n7;
        locals.var_q_d1_qi__blk847_dn8 = assign33930_e37815_d_n8;
        locals.var_q_d1_qi__blk847_dn9 = assign33930_e37815_d_n9;

        let (assign33940_e37825, assign33940_e37825_d_n4, assign33940_e37825_d_n6, assign33940_e37825_d_n7, assign33940_e37825_d_n8, assign33940_e37825_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) {
        let assign33940_e37823: f64 = (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845);
        (assign33940_e37823, ((locals.var_k2__blk933_dn4 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q_d2_q2__blk845) + (locals.var_k2__blk933 * locals.var_q_d2_q2__blk845_dn9)),)
    } else {
        (locals.var_q_d2_qi__blk848, locals.var_q_d2_qi__blk848_dn4, locals.var_q_d2_qi__blk848_dn6, locals.var_q_d2_qi__blk848_dn7, locals.var_q_d2_qi__blk848_dn8, locals.var_q_d2_qi__blk848_dn9,)
    }
};
        locals.var_q_d2_qi__blk848 = assign33940_e37825;
        locals.var_q_d2_qi__blk848_dn4 = assign33940_e37825_d_n4;
        locals.var_q_d2_qi__blk848_dn6 = assign33940_e37825_d_n6;
        locals.var_q_d2_qi__blk848_dn7 = assign33940_e37825_d_n7;
        locals.var_q_d2_qi__blk848_dn8 = assign33940_e37825_d_n8;
        locals.var_q_d2_qi__blk848_dn9 = assign33940_e37825_d_n9;

        let (assign33950_e37837, assign33950_e37837_d_n4, assign33950_e37837_d_n6, assign33950_e37837_d_n7, assign33950_e37837_d_n8, assign33950_e37837_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) {
        let assign33950_e37833: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837);
        let assign33950_e37835: f64 = (assign33950_e37833 - locals.var_q_aexp__blk824);
        (assign33950_e37835, (((locals.var_q_qi_int__blk846_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn4)) - locals.var_q_aexp__blk824_dn4), (((locals.var_q_qi_int__blk846_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn6)) - locals.var_q_aexp__blk824_dn6), (((locals.var_q_qi_int__blk846_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn7)) - locals.var_q_aexp__blk824_dn7), (((locals.var_q_qi_int__blk846_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn8)) - locals.var_q_aexp__blk824_dn8), (((locals.var_q_qi_int__blk846_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_qi_int__blk846 * locals.var_q_expnum__blk837_dn9)) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_zero__blk849, locals.var_q_zero__blk849_dn4, locals.var_q_zero__blk849_dn6, locals.var_q_zero__blk849_dn7, locals.var_q_zero__blk849_dn8, locals.var_q_zero__blk849_dn9,)
    }
};
        locals.var_q_zero__blk849 = assign33950_e37837;
        locals.var_q_zero__blk849_dn4 = assign33950_e37837_d_n4;
        locals.var_q_zero__blk849_dn6 = assign33950_e37837_d_n6;
        locals.var_q_zero__blk849_dn7 = assign33950_e37837_d_n7;
        locals.var_q_zero__blk849_dn8 = assign33950_e37837_d_n8;
        locals.var_q_zero__blk849_dn9 = assign33950_e37837_d_n9;

        let (assign33960_e37853, assign33960_e37853_d_n4, assign33960_e37853_d_n6, assign33960_e37853_d_n7, assign33960_e37853_d_n8, assign33960_e37853_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) {
        let assign33960_e37845: f64 = (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837);
        let assign33960_e37848: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838);
        let assign33960_e37849: f64 = (assign33960_e37845 + assign33960_e37848);
        let assign33960_e37851: f64 = (assign33960_e37849 + locals.var_q_aexp__blk824);
        (assign33960_e37851, ((((locals.var_q_d1_qi__blk847_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn4)) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn4))) + locals.var_q_aexp__blk824_dn4), ((((locals.var_q_d1_qi__blk847_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn6)) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn6))) + locals.var_q_aexp__blk824_dn6), ((((locals.var_q_d1_qi__blk847_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn7)) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn7))) + locals.var_q_aexp__blk824_dn7), ((((locals.var_q_d1_qi__blk847_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn8)) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn8))) + locals.var_q_aexp__blk824_dn8), ((((locals.var_q_d1_qi__blk847_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d1_qi__blk847 * locals.var_q_expnum__blk837_dn9)) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d1_expnum__blk838) + (locals.var_q_qi_int__blk846 * locals.var_q_d1_expnum__blk838_dn9))) + locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d1_zero__blk850, locals.var_q_d1_zero__blk850_dn4, locals.var_q_d1_zero__blk850_dn6, locals.var_q_d1_zero__blk850_dn7, locals.var_q_d1_zero__blk850_dn8, locals.var_q_d1_zero__blk850_dn9,)
    }
};
        locals.var_q_d1_zero__blk850 = assign33960_e37853;
        locals.var_q_d1_zero__blk850_dn4 = assign33960_e37853_d_n4;
        locals.var_q_d1_zero__blk850_dn6 = assign33960_e37853_d_n6;
        locals.var_q_d1_zero__blk850_dn7 = assign33960_e37853_d_n7;
        locals.var_q_d1_zero__blk850_dn8 = assign33960_e37853_d_n8;
        locals.var_q_d1_zero__blk850_dn9 = assign33960_e37853_d_n9;

        let (assign33970_e37875, assign33970_e37875_d_n4, assign33970_e37875_d_n6, assign33970_e37875_d_n7, assign33970_e37875_d_n8, assign33970_e37875_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) {
        let assign33970_e37861: f64 = (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837);
        let assign33970_e37864: f64 = (2.0 * locals.var_q_d1_qi__blk847);
        let assign33970_e37866: f64 = (assign33970_e37864 * locals.var_q_d1_expnum__blk838);
        let assign33970_e37867: f64 = (assign33970_e37861 + assign33970_e37866);
        let assign33970_e37870: f64 = (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839);
        let assign33970_e37871: f64 = (assign33970_e37867 + assign33970_e37870);
        let assign33970_e37873: f64 = (assign33970_e37871 - locals.var_q_aexp__blk824);
        (assign33970_e37873, (((((locals.var_q_d2_qi__blk848_dn4 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn4)) + (((2.0 * locals.var_q_d1_qi__blk847_dn4) * locals.var_q_d1_expnum__blk838) + (assign33970_e37864 * locals.var_q_d1_expnum__blk838_dn4))) + ((locals.var_q_qi_int__blk846_dn4 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn4))) - locals.var_q_aexp__blk824_dn4), (((((locals.var_q_d2_qi__blk848_dn6 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn6)) + (((2.0 * locals.var_q_d1_qi__blk847_dn6) * locals.var_q_d1_expnum__blk838) + (assign33970_e37864 * locals.var_q_d1_expnum__blk838_dn6))) + ((locals.var_q_qi_int__blk846_dn6 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn6))) - locals.var_q_aexp__blk824_dn6), (((((locals.var_q_d2_qi__blk848_dn7 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn7)) + (((2.0 * locals.var_q_d1_qi__blk847_dn7) * locals.var_q_d1_expnum__blk838) + (assign33970_e37864 * locals.var_q_d1_expnum__blk838_dn7))) + ((locals.var_q_qi_int__blk846_dn7 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn7))) - locals.var_q_aexp__blk824_dn7), (((((locals.var_q_d2_qi__blk848_dn8 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn8)) + (((2.0 * locals.var_q_d1_qi__blk847_dn8) * locals.var_q_d1_expnum__blk838) + (assign33970_e37864 * locals.var_q_d1_expnum__blk838_dn8))) + ((locals.var_q_qi_int__blk846_dn8 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn8))) - locals.var_q_aexp__blk824_dn8), (((((locals.var_q_d2_qi__blk848_dn9 * locals.var_q_expnum__blk837) + (locals.var_q_d2_qi__blk848 * locals.var_q_expnum__blk837_dn9)) + (((2.0 * locals.var_q_d1_qi__blk847_dn9) * locals.var_q_d1_expnum__blk838) + (assign33970_e37864 * locals.var_q_d1_expnum__blk838_dn9))) + ((locals.var_q_qi_int__blk846_dn9 * locals.var_q_d2_expnum__blk839) + (locals.var_q_qi_int__blk846 * locals.var_q_d2_expnum__blk839_dn9))) - locals.var_q_aexp__blk824_dn9),)
    } else {
        (locals.var_q_d2_zero__blk851, locals.var_q_d2_zero__blk851_dn4, locals.var_q_d2_zero__blk851_dn6, locals.var_q_d2_zero__blk851_dn7, locals.var_q_d2_zero__blk851_dn8, locals.var_q_d2_zero__blk851_dn9,)
    }
};
        locals.var_q_d2_zero__blk851 = assign33970_e37875;
        locals.var_q_d2_zero__blk851_dn4 = assign33970_e37875_d_n4;
        locals.var_q_d2_zero__blk851_dn6 = assign33970_e37875_d_n6;
        locals.var_q_d2_zero__blk851_dn7 = assign33970_e37875_d_n7;
        locals.var_q_d2_zero__blk851_dn8 = assign33970_e37875_d_n8;
        locals.var_q_d2_zero__blk851_dn9 = assign33970_e37875_d_n9;

        let (assign33980_e37891, assign33980_e37891_d_n4, assign33980_e37891_d_n6, assign33980_e37891_d_n7, assign33980_e37891_d_n8, assign33980_e37891_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) {
        let assign33980_e37883: f64 = (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850);
        let assign33980_e37886: f64 = (0.5 * locals.var_q_zero__blk849);
        let assign33980_e37888: f64 = (assign33980_e37886 * locals.var_q_d2_zero__blk851);
        let assign33980_e37889: f64 = (assign33980_e37883 - assign33980_e37888);
        (assign33980_e37889, (((locals.var_q_d1_zero__blk850_dn4 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn4)) - (((0.5 * locals.var_q_zero__blk849_dn4) * locals.var_q_d2_zero__blk851) + (assign33980_e37886 * locals.var_q_d2_zero__blk851_dn4))), (((locals.var_q_d1_zero__blk850_dn6 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn6)) - (((0.5 * locals.var_q_zero__blk849_dn6) * locals.var_q_d2_zero__blk851) + (assign33980_e37886 * locals.var_q_d2_zero__blk851_dn6))), (((locals.var_q_d1_zero__blk850_dn7 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn7)) - (((0.5 * locals.var_q_zero__blk849_dn7) * locals.var_q_d2_zero__blk851) + (assign33980_e37886 * locals.var_q_d2_zero__blk851_dn7))), (((locals.var_q_d1_zero__blk850_dn8 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn8)) - (((0.5 * locals.var_q_zero__blk849_dn8) * locals.var_q_d2_zero__blk851) + (assign33980_e37886 * locals.var_q_d2_zero__blk851_dn8))), (((locals.var_q_d1_zero__blk850_dn9 * locals.var_q_d1_zero__blk850) + (locals.var_q_d1_zero__blk850 * locals.var_q_d1_zero__blk850_dn9)) - (((0.5 * locals.var_q_zero__blk849_dn9) * locals.var_q_d2_zero__blk851) + (assign33980_e37886 * locals.var_q_d2_zero__blk851_dn9))),)
    } else {
        (locals.var_q_temp__blk860, locals.var_q_temp__blk860_dn4, locals.var_q_temp__blk860_dn6, locals.var_q_temp__blk860_dn7, locals.var_q_temp__blk860_dn8, locals.var_q_temp__blk860_dn9,)
    }
};
        locals.var_q_temp__blk860 = assign33980_e37891;
        locals.var_q_temp__blk860_dn4 = assign33980_e37891_d_n4;
        locals.var_q_temp__blk860_dn6 = assign33980_e37891_d_n6;
        locals.var_q_temp__blk860_dn7 = assign33980_e37891_d_n7;
        locals.var_q_temp__blk860_dn8 = assign33980_e37891_d_n8;
        locals.var_q_temp__blk860_dn9 = assign33980_e37891_d_n9;

        let (assign33990_e37910, assign33990_e37910_d_n4, assign33990_e37910_d_n6, assign33990_e37910_d_n7, assign33990_e37910_d_n8, assign33990_e37910_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) {
        let assign33990_e37898: f64 = (-locals.var_q_zero__blk849);
        let assign33990_e37900: f64 = (assign33990_e37898 * locals.var_q_d1_zero__blk850);
        let assign33990_e37902: f64 = (assign33990_e37900 * locals.var_q_temp__blk860);
        let assign33990_e37905: f64 = (locals.var_q_temp__blk860 * locals.var_q_temp__blk860);
        let assign33990_e37907: f64 = (assign33990_e37905 + 1e-200);
        let assign33990_e37908: f64 = (assign33990_e37902 / assign33990_e37907);
        (assign33990_e37908, ((((((((-locals.var_q_zero__blk849_dn4) * locals.var_q_d1_zero__blk850) + (assign33990_e37898 * locals.var_q_d1_zero__blk850_dn4)) * locals.var_q_temp__blk860) + (assign33990_e37900 * locals.var_q_temp__blk860_dn4)) * assign33990_e37907) - (assign33990_e37902 * ((locals.var_q_temp__blk860_dn4 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn4)))) / (assign33990_e37907 * assign33990_e37907)), ((((((((-locals.var_q_zero__blk849_dn6) * locals.var_q_d1_zero__blk850) + (assign33990_e37898 * locals.var_q_d1_zero__blk850_dn6)) * locals.var_q_temp__blk860) + (assign33990_e37900 * locals.var_q_temp__blk860_dn6)) * assign33990_e37907) - (assign33990_e37902 * ((locals.var_q_temp__blk860_dn6 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn6)))) / (assign33990_e37907 * assign33990_e37907)), ((((((((-locals.var_q_zero__blk849_dn7) * locals.var_q_d1_zero__blk850) + (assign33990_e37898 * locals.var_q_d1_zero__blk850_dn7)) * locals.var_q_temp__blk860) + (assign33990_e37900 * locals.var_q_temp__blk860_dn7)) * assign33990_e37907) - (assign33990_e37902 * ((locals.var_q_temp__blk860_dn7 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn7)))) / (assign33990_e37907 * assign33990_e37907)), ((((((((-locals.var_q_zero__blk849_dn8) * locals.var_q_d1_zero__blk850) + (assign33990_e37898 * locals.var_q_d1_zero__blk850_dn8)) * locals.var_q_temp__blk860) + (assign33990_e37900 * locals.var_q_temp__blk860_dn8)) * assign33990_e37907) - (assign33990_e37902 * ((locals.var_q_temp__blk860_dn8 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn8)))) / (assign33990_e37907 * assign33990_e37907)), ((((((((-locals.var_q_zero__blk849_dn9) * locals.var_q_d1_zero__blk850) + (assign33990_e37898 * locals.var_q_d1_zero__blk850_dn9)) * locals.var_q_temp__blk860) + (assign33990_e37900 * locals.var_q_temp__blk860_dn9)) * assign33990_e37907) - (assign33990_e37902 * ((locals.var_q_temp__blk860_dn9 * locals.var_q_temp__blk860) + (locals.var_q_temp__blk860 * locals.var_q_temp__blk860_dn9)))) / (assign33990_e37907 * assign33990_e37907)),)
    } else {
        (locals.var_q_eps2__blk852, locals.var_q_eps2__blk852_dn4, locals.var_q_eps2__blk852_dn6, locals.var_q_eps2__blk852_dn7, locals.var_q_eps2__blk852_dn8, locals.var_q_eps2__blk852_dn9,)
    }
};
        locals.var_q_eps2__blk852 = assign33990_e37910;
        locals.var_q_eps2__blk852_dn4 = assign33990_e37910_d_n4;
        locals.var_q_eps2__blk852_dn6 = assign33990_e37910_d_n6;
        locals.var_q_eps2__blk852_dn7 = assign33990_e37910_d_n7;
        locals.var_q_eps2__blk852_dn8 = assign33990_e37910_d_n8;
        locals.var_q_eps2__blk852_dn9 = assign33990_e37910_d_n9;

    }

    pub(super) fn stamp_transient_block_92(
        locals: &mut StampLocals,
    ) {
        let (assign34000_e37920, assign34000_e37920_d_n4, assign34000_e37920_d_n6, assign34000_e37920_d_n7, assign34000_e37920_d_n8, assign34000_e37920_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1129 != 0.0)) && (locals.var_guard1130 != 0.0)) {
        let assign34000_e37918: f64 = (locals.var_q1s__blk937 + locals.var_q_eps2__blk852);
        (assign34000_e37918, (locals.var_q1s__blk937_dn4 + locals.var_q_eps2__blk852_dn4), (locals.var_q1s__blk937_dn6 + locals.var_q_eps2__blk852_dn6), (locals.var_q1s__blk937_dn7 + locals.var_q_eps2__blk852_dn7), (locals.var_q1s__blk937_dn8 + locals.var_q_eps2__blk852_dn8), (locals.var_q1s__blk937_dn9 + locals.var_q_eps2__blk852_dn9),)
    } else {
        (locals.var_q1s__blk937, locals.var_q1s__blk937_dn4, locals.var_q1s__blk937_dn6, locals.var_q1s__blk937_dn7, locals.var_q1s__blk937_dn8, locals.var_q1s__blk937_dn9,)
    }
};
        locals.var_q1s__blk937 = assign34000_e37920;
        locals.var_q1s__blk937_dn4 = assign34000_e37920_d_n4;
        locals.var_q1s__blk937_dn6 = assign34000_e37920_d_n6;
        locals.var_q1s__blk937_dn7 = assign34000_e37920_d_n7;
        locals.var_q1s__blk937_dn8 = assign34000_e37920_d_n8;
        locals.var_q1s__blk937_dn9 = assign34000_e37920_d_n9;

        let (assign34010_e37926, assign34010_e37926_d_n4, assign34010_e37926_d_n6, assign34010_e37926_d_n7, assign34010_e37926_d_n8, assign34010_e37926_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34010_e37924: f64 = (locals.var_k1__blk932 * locals.var_q1s__blk937);
        (assign34010_e37924, ((locals.var_k1__blk932_dn4 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn4)), ((locals.var_k1__blk932_dn6 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn6)), ((locals.var_k1__blk932_dn7 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn7)), ((locals.var_k1__blk932_dn8 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn8)), ((locals.var_k1__blk932_dn9 * locals.var_q1s__blk937) + (locals.var_k1__blk932 * locals.var_q1s__blk937_dn9)),)
    } else {
        (locals.var_k1q1s__blk939, locals.var_k1q1s__blk939_dn4, locals.var_k1q1s__blk939_dn6, locals.var_k1q1s__blk939_dn7, locals.var_k1q1s__blk939_dn8, locals.var_k1q1s__blk939_dn9,)
    }
};
        locals.var_k1q1s__blk939 = assign34010_e37926;
        locals.var_k1q1s__blk939_dn4 = assign34010_e37926_d_n4;
        locals.var_k1q1s__blk939_dn6 = assign34010_e37926_d_n6;
        locals.var_k1q1s__blk939_dn7 = assign34010_e37926_d_n7;
        locals.var_k1q1s__blk939_dn8 = assign34010_e37926_d_n8;
        locals.var_k1q1s__blk939_dn9 = assign34010_e37926_d_n9;

        let assign34020_e37929: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign34020_e37931: f64 = assign34020_e37929;
        let assign34020_e37933: f64 = if assign34020_e37931 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1138 = assign34020_e37933;

        let (assign34030_e37944, assign34030_e37944_d_n4, assign34030_e37944_d_n6, assign34030_e37944_d_n7, assign34030_e37944_d_n8, assign34030_e37944_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1138 != 0.0)) {
        let assign34030_e37939: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign34030_e37941: f64 = assign34030_e37939;
        let assign34030_e37942: f64 = (assign34030_e37941).exp();
        (assign34030_e37942, (assign34030_e37942 * (locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4)), (assign34030_e37942 * (locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6)), (assign34030_e37942 * (locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7)), (assign34030_e37942 * (locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8)), (assign34030_e37942 * (locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign34030_e37944;
        locals.var_q_temp1__blk814_dn4 = assign34030_e37944_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign34030_e37944_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign34030_e37944_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign34030_e37944_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign34030_e37944_d_n9;

        let (assign34040_e37985, assign34040_e37985_d_n4, assign34040_e37985_d_n6, assign34040_e37985_d_n7, assign34040_e37985_d_n8, assign34040_e37985_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1138 == 0.0)) {
        let assign34040_e37953: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign34040_e37955: f64 = assign34040_e37953;
        let assign34040_e37957: f64 = (assign34040_e37955 - 80.0);
        let assign34040_e37962: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign34040_e37964: f64 = assign34040_e37962;
        let assign34040_e37966: f64 = (assign34040_e37964 - 80.0);
        let assign34040_e37967: f64 = (0.5 * assign34040_e37966);
        let assign34040_e37971: f64 = (locals.var_xg1x__blk930 - locals.var_q1s__blk937);
        let assign34040_e37973: f64 = assign34040_e37971;
        let assign34040_e37975: f64 = (assign34040_e37973 - 80.0);
        let assign34040_e37977: f64 = (assign34040_e37975 * 0.3333333333333);
        let assign34040_e37978: f64 = (1.0 + assign34040_e37977);
        let assign34040_e37979: f64 = (assign34040_e37967 * assign34040_e37978);
        let assign34040_e37980: f64 = (1.0 + assign34040_e37979);
        let assign34040_e37981: f64 = (assign34040_e37957 * assign34040_e37980);
        let assign34040_e37982: f64 = (1.0 + assign34040_e37981);
        let assign34040_e37983: f64 = (5.54062e34 * assign34040_e37982);
        (assign34040_e37983, (5.54062e34 * (((locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4) * assign34040_e37980) + (assign34040_e37957 * (((0.5 * (locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4)) * assign34040_e37978) + (assign34040_e37967 * ((locals.var_xg1x__blk930_dn4 - locals.var_q1s__blk937_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6) * assign34040_e37980) + (assign34040_e37957 * (((0.5 * (locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6)) * assign34040_e37978) + (assign34040_e37967 * ((locals.var_xg1x__blk930_dn6 - locals.var_q1s__blk937_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7) * assign34040_e37980) + (assign34040_e37957 * (((0.5 * (locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7)) * assign34040_e37978) + (assign34040_e37967 * ((locals.var_xg1x__blk930_dn7 - locals.var_q1s__blk937_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8) * assign34040_e37980) + (assign34040_e37957 * (((0.5 * (locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8)) * assign34040_e37978) + (assign34040_e37967 * ((locals.var_xg1x__blk930_dn8 - locals.var_q1s__blk937_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9) * assign34040_e37980) + (assign34040_e37957 * (((0.5 * (locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9)) * assign34040_e37978) + (assign34040_e37967 * ((locals.var_xg1x__blk930_dn9 - locals.var_q1s__blk937_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign34040_e37985;
        locals.var_q_temp1__blk814_dn4 = assign34040_e37985_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign34040_e37985_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign34040_e37985_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign34040_e37985_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign34040_e37985_d_n9;

        let (assign34050_e37991, assign34050_e37991_d_n4, assign34050_e37991_d_n6, assign34050_e37991_d_n7, assign34050_e37991_d_n8, assign34050_e37991_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34050_e37989: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
        (assign34050_e37989, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_aexp1s__blk943, locals.var_aexp1s__blk943_dn4, locals.var_aexp1s__blk943_dn6, locals.var_aexp1s__blk943_dn7, locals.var_aexp1s__blk943_dn8, locals.var_aexp1s__blk943_dn9,)
    }
};
        locals.var_aexp1s__blk943 = assign34050_e37991;
        locals.var_aexp1s__blk943_dn4 = assign34050_e37991_d_n4;
        locals.var_aexp1s__blk943_dn6 = assign34050_e37991_d_n6;
        locals.var_aexp1s__blk943_dn7 = assign34050_e37991_d_n7;
        locals.var_aexp1s__blk943_dn8 = assign34050_e37991_d_n8;
        locals.var_aexp1s__blk943_dn9 = assign34050_e37991_d_n9;

        let (assign34060_e37999, assign34060_e37999_d_n4, assign34060_e37999_d_n6, assign34060_e37999_d_n7, assign34060_e37999_d_n8, assign34060_e37999_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34060_e37995: f64 = (locals.var_k1q1s__blk939 * locals.var_k1q1s__blk939);
        let assign34060_e37997: f64 = (assign34060_e37995 - locals.var_aexp1s__blk943);
        (assign34060_e37997, (((locals.var_k1q1s__blk939_dn4 * locals.var_k1q1s__blk939) + (locals.var_k1q1s__blk939 * locals.var_k1q1s__blk939_dn4)) - locals.var_aexp1s__blk943_dn4), (((locals.var_k1q1s__blk939_dn6 * locals.var_k1q1s__blk939) + (locals.var_k1q1s__blk939 * locals.var_k1q1s__blk939_dn6)) - locals.var_aexp1s__blk943_dn6), (((locals.var_k1q1s__blk939_dn7 * locals.var_k1q1s__blk939) + (locals.var_k1q1s__blk939 * locals.var_k1q1s__blk939_dn7)) - locals.var_aexp1s__blk943_dn7), (((locals.var_k1q1s__blk939_dn8 * locals.var_k1q1s__blk939) + (locals.var_k1q1s__blk939 * locals.var_k1q1s__blk939_dn8)) - locals.var_aexp1s__blk943_dn8), (((locals.var_k1q1s__blk939_dn9 * locals.var_k1q1s__blk939) + (locals.var_k1q1s__blk939 * locals.var_k1q1s__blk939_dn9)) - locals.var_aexp1s__blk943_dn9),)
    } else {
        (locals.var_qsqs__blk942, locals.var_qsqs__blk942_dn4, locals.var_qsqs__blk942_dn6, locals.var_qsqs__blk942_dn7, locals.var_qsqs__blk942_dn8, locals.var_qsqs__blk942_dn9,)
    }
};
        locals.var_qsqs__blk942 = assign34060_e37999;
        locals.var_qsqs__blk942_dn4 = assign34060_e37999_d_n4;
        locals.var_qsqs__blk942_dn6 = assign34060_e37999_d_n6;
        locals.var_qsqs__blk942_dn7 = assign34060_e37999_d_n7;
        locals.var_qsqs__blk942_dn8 = assign34060_e37999_d_n8;
        locals.var_qsqs__blk942_dn9 = assign34060_e37999_d_n9;

        let assign34070_e38002: f64 = if locals.var_aexp1s__blk943 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1139 = assign34070_e38002;

        let (assign34080_e38008, assign34080_e38008_d_n4, assign34080_e38008_d_n6, assign34080_e38008_d_n7, assign34080_e38008_d_n8, assign34080_e38008_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1139 != 0.0)) {
        (1e-80, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qis__blk938, locals.var_qis__blk938_dn4, locals.var_qis__blk938_dn6, locals.var_qis__blk938_dn7, locals.var_qis__blk938_dn8, locals.var_qis__blk938_dn9,)
    }
};
        locals.var_qis__blk938 = assign34080_e38008;
        locals.var_qis__blk938_dn4 = assign34080_e38008_d_n4;
        locals.var_qis__blk938_dn6 = assign34080_e38008_d_n6;
        locals.var_qis__blk938_dn7 = assign34080_e38008_d_n7;
        locals.var_qis__blk938_dn8 = assign34080_e38008_d_n8;
        locals.var_qis__blk938_dn9 = assign34080_e38008_d_n9;

        let (assign34090_e38016, assign34090_e38016_d_n4, assign34090_e38016_d_n6, assign34090_e38016_d_n7, assign34090_e38016_d_n8, assign34090_e38016_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1139 != 0.0)) {
        let assign34090_e38014: f64 = (locals.var_qis__blk938 - locals.var_k1q1s__blk939);
        (assign34090_e38014, (locals.var_qis__blk938_dn4 - locals.var_k1q1s__blk939_dn4), (locals.var_qis__blk938_dn6 - locals.var_k1q1s__blk939_dn6), (locals.var_qis__blk938_dn7 - locals.var_k1q1s__blk939_dn7), (locals.var_qis__blk938_dn8 - locals.var_k1q1s__blk939_dn8), (locals.var_qis__blk938_dn9 - locals.var_k1q1s__blk939_dn9),)
    } else {
        (locals.var_k2q2s__blk940, locals.var_k2q2s__blk940_dn4, locals.var_k2q2s__blk940_dn6, locals.var_k2q2s__blk940_dn7, locals.var_k2q2s__blk940_dn8, locals.var_k2q2s__blk940_dn9,)
    }
};
        locals.var_k2q2s__blk940 = assign34090_e38016;
        locals.var_k2q2s__blk940_dn4 = assign34090_e38016_d_n4;
        locals.var_k2q2s__blk940_dn6 = assign34090_e38016_d_n6;
        locals.var_k2q2s__blk940_dn7 = assign34090_e38016_d_n7;
        locals.var_k2q2s__blk940_dn8 = assign34090_e38016_d_n8;
        locals.var_k2q2s__blk940_dn9 = assign34090_e38016_d_n9;

        let (assign34100_e38024, assign34100_e38024_d_n4, assign34100_e38024_d_n6, assign34100_e38024_d_n7, assign34100_e38024_d_n8, assign34100_e38024_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1139 != 0.0)) {
        let assign34100_e38022: f64 = (locals.var_k2q2s__blk940 / locals.var_k2__blk933);
        (assign34100_e38022, (((locals.var_k2q2s__blk940_dn4 * locals.var_k2__blk933) - (locals.var_k2q2s__blk940 * locals.var_k2__blk933_dn4)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2s__blk940_dn6 * locals.var_k2__blk933) - (locals.var_k2q2s__blk940 * locals.var_k2__blk933_dn6)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2s__blk940_dn7 * locals.var_k2__blk933) - (locals.var_k2q2s__blk940 * locals.var_k2__blk933_dn7)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2s__blk940_dn8 * locals.var_k2__blk933) - (locals.var_k2q2s__blk940 * locals.var_k2__blk933_dn8)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2s__blk940_dn9 * locals.var_k2__blk933) - (locals.var_k2q2s__blk940 * locals.var_k2__blk933_dn9)) / (locals.var_k2__blk933 * locals.var_k2__blk933)),)
    } else {
        (locals.var_q2s__blk941, locals.var_q2s__blk941_dn4, locals.var_q2s__blk941_dn6, locals.var_q2s__blk941_dn7, locals.var_q2s__blk941_dn8, locals.var_q2s__blk941_dn9,)
    }
};
        locals.var_q2s__blk941 = assign34100_e38024;
        locals.var_q2s__blk941_dn4 = assign34100_e38024_d_n4;
        locals.var_q2s__blk941_dn6 = assign34100_e38024_d_n6;
        locals.var_q2s__blk941_dn7 = assign34100_e38024_d_n7;
        locals.var_q2s__blk941_dn8 = assign34100_e38024_d_n8;
        locals.var_q2s__blk941_dn9 = assign34100_e38024_d_n9;

        let assign34110_e38027: f64 = (-0.005);
        let assign34110_e38028: f64 = if locals.var_qsqs__blk942 < assign34110_e38027 { 1.0 } else { 0.0 };
        locals.var_guard1140 = assign34110_e38028;

        let (assign34120_e38039, assign34120_e38039_d_n4, assign34120_e38039_d_n6, assign34120_e38039_d_n7, assign34120_e38039_d_n8, assign34120_e38039_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1140 != 0.0)) {
        let assign34120_e38036: f64 = (locals.var_qsqs__blk942).abs();
        let assign34120_e38037: f64 = (assign34120_e38036).sqrt();
        (assign34120_e38037, (if locals.var_qsqs__blk942 >= 0.0 { locals.var_qsqs__blk942_dn4 } else { (-locals.var_qsqs__blk942_dn4) } / (2.0 * assign34120_e38037)), (if locals.var_qsqs__blk942 >= 0.0 { locals.var_qsqs__blk942_dn6 } else { (-locals.var_qsqs__blk942_dn6) } / (2.0 * assign34120_e38037)), (if locals.var_qsqs__blk942 >= 0.0 { locals.var_qsqs__blk942_dn7 } else { (-locals.var_qsqs__blk942_dn7) } / (2.0 * assign34120_e38037)), (if locals.var_qsqs__blk942 >= 0.0 { locals.var_qsqs__blk942_dn8 } else { (-locals.var_qsqs__blk942_dn8) } / (2.0 * assign34120_e38037)), (if locals.var_qsqs__blk942 >= 0.0 { locals.var_qsqs__blk942_dn9 } else { (-locals.var_qsqs__blk942_dn9) } / (2.0 * assign34120_e38037)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign34120_e38039;
        locals.var_q_rac_qsq__blk828_dn4 = assign34120_e38039_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign34120_e38039_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign34120_e38039_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign34120_e38039_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign34120_e38039_d_n9;

        let (assign34130_e38053, assign34130_e38053_d_n4, assign34130_e38053_d_n6, assign34130_e38053_d_n7, assign34130_e38053_d_n8, assign34130_e38053_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1140 != 0.0)) {
        let assign34130_e38049: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign34130_e38050: f64 = (assign34130_e38049).tan();
        let assign34130_e38051: f64 = (locals.var_q_rac_qsq__blk828 / assign34130_e38050);
        (assign34130_e38051, (((locals.var_q_rac_qsq__blk828_dn4 * assign34130_e38050) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn4) / ((assign34130_e38049).cos() * (assign34130_e38049).cos())))) / (assign34130_e38050 * assign34130_e38050)), (((locals.var_q_rac_qsq__blk828_dn6 * assign34130_e38050) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn6) / ((assign34130_e38049).cos() * (assign34130_e38049).cos())))) / (assign34130_e38050 * assign34130_e38050)), (((locals.var_q_rac_qsq__blk828_dn7 * assign34130_e38050) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn7) / ((assign34130_e38049).cos() * (assign34130_e38049).cos())))) / (assign34130_e38050 * assign34130_e38050)), (((locals.var_q_rac_qsq__blk828_dn8 * assign34130_e38050) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn8) / ((assign34130_e38049).cos() * (assign34130_e38049).cos())))) / (assign34130_e38050 * assign34130_e38050)), (((locals.var_q_rac_qsq__blk828_dn9 * assign34130_e38050) - (locals.var_q_rac_qsq__blk828 * ((0.5 * locals.var_q_rac_qsq__blk828_dn9) / ((assign34130_e38049).cos() * (assign34130_e38049).cos())))) / (assign34130_e38050 * assign34130_e38050)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign34130_e38053;
        locals.var_q_qcoth__blk829_dn4 = assign34130_e38053_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign34130_e38053_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign34130_e38053_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign34130_e38053_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign34130_e38053_d_n9;

        let assign34140_e38056: f64 = if locals.var_qsqs__blk942 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1141 = assign34140_e38056;

        let (assign34150_e38070, assign34150_e38070_d_n4, assign34150_e38070_d_n6, assign34150_e38070_d_n7, assign34150_e38070_d_n8, assign34150_e38070_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1140 == 0.0)) && (locals.var_guard1141 != 0.0)) {
        let assign34150_e38067: f64 = (locals.var_qsqs__blk942).abs();
        let assign34150_e38068: f64 = (assign34150_e38067).sqrt();
        (assign34150_e38068, (if locals.var_qsqs__blk942 >= 0.0 { locals.var_qsqs__blk942_dn4 } else { (-locals.var_qsqs__blk942_dn4) } / (2.0 * assign34150_e38068)), (if locals.var_qsqs__blk942 >= 0.0 { locals.var_qsqs__blk942_dn6 } else { (-locals.var_qsqs__blk942_dn6) } / (2.0 * assign34150_e38068)), (if locals.var_qsqs__blk942 >= 0.0 { locals.var_qsqs__blk942_dn7 } else { (-locals.var_qsqs__blk942_dn7) } / (2.0 * assign34150_e38068)), (if locals.var_qsqs__blk942 >= 0.0 { locals.var_qsqs__blk942_dn8 } else { (-locals.var_qsqs__blk942_dn8) } / (2.0 * assign34150_e38068)), (if locals.var_qsqs__blk942 >= 0.0 { locals.var_qsqs__blk942_dn9 } else { (-locals.var_qsqs__blk942_dn9) } / (2.0 * assign34150_e38068)),)
    } else {
        (locals.var_q_rac_qsq__blk828, locals.var_q_rac_qsq__blk828_dn4, locals.var_q_rac_qsq__blk828_dn6, locals.var_q_rac_qsq__blk828_dn7, locals.var_q_rac_qsq__blk828_dn8, locals.var_q_rac_qsq__blk828_dn9,)
    }
};
        locals.var_q_rac_qsq__blk828 = assign34150_e38070;
        locals.var_q_rac_qsq__blk828_dn4 = assign34150_e38070_d_n4;
        locals.var_q_rac_qsq__blk828_dn6 = assign34150_e38070_d_n6;
        locals.var_q_rac_qsq__blk828_dn7 = assign34150_e38070_d_n7;
        locals.var_q_rac_qsq__blk828_dn8 = assign34150_e38070_d_n8;
        locals.var_q_rac_qsq__blk828_dn9 = assign34150_e38070_d_n9;

        let (assign34160_e38084, assign34160_e38084_d_n4, assign34160_e38084_d_n6, assign34160_e38084_d_n7, assign34160_e38084_d_n8, assign34160_e38084_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1140 == 0.0)) && (locals.var_guard1141 != 0.0)) {
        let assign34160_e38081: f64 = (-locals.var_q_rac_qsq__blk828);
        let assign34160_e38082: f64 = (assign34160_e38081).exp();
        (assign34160_e38082, (assign34160_e38082 * (-locals.var_q_rac_qsq__blk828_dn4)), (assign34160_e38082 * (-locals.var_q_rac_qsq__blk828_dn6)), (assign34160_e38082 * (-locals.var_q_rac_qsq__blk828_dn7)), (assign34160_e38082 * (-locals.var_q_rac_qsq__blk828_dn8)), (assign34160_e38082 * (-locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_invexpq__blk831, locals.var_q_invexpq__blk831_dn4, locals.var_q_invexpq__blk831_dn6, locals.var_q_invexpq__blk831_dn7, locals.var_q_invexpq__blk831_dn8, locals.var_q_invexpq__blk831_dn9,)
    }
};
        locals.var_q_invexpq__blk831 = assign34160_e38084;
        locals.var_q_invexpq__blk831_dn4 = assign34160_e38084_d_n4;
        locals.var_q_invexpq__blk831_dn6 = assign34160_e38084_d_n6;
        locals.var_q_invexpq__blk831_dn7 = assign34160_e38084_d_n7;
        locals.var_q_invexpq__blk831_dn8 = assign34160_e38084_d_n8;
        locals.var_q_invexpq__blk831_dn9 = assign34160_e38084_d_n9;

        let (assign34170_e38104, assign34170_e38104_d_n4, assign34170_e38104_d_n6, assign34170_e38104_d_n7, assign34170_e38104_d_n8, assign34170_e38104_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1140 == 0.0)) && (locals.var_guard1141 != 0.0)) {
        let assign34170_e38097: f64 = (1.0 + locals.var_q_invexpq__blk831);
        let assign34170_e38098: f64 = (locals.var_q_rac_qsq__blk828 * assign34170_e38097);
        let assign34170_e38101: f64 = (1.0 - locals.var_q_invexpq__blk831);
        let assign34170_e38102: f64 = (assign34170_e38098 / assign34170_e38101);
        (assign34170_e38102, (((((locals.var_q_rac_qsq__blk828_dn4 * assign34170_e38097) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn4)) * assign34170_e38101) - (assign34170_e38098 * (-locals.var_q_invexpq__blk831_dn4))) / (assign34170_e38101 * assign34170_e38101)), (((((locals.var_q_rac_qsq__blk828_dn6 * assign34170_e38097) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn6)) * assign34170_e38101) - (assign34170_e38098 * (-locals.var_q_invexpq__blk831_dn6))) / (assign34170_e38101 * assign34170_e38101)), (((((locals.var_q_rac_qsq__blk828_dn7 * assign34170_e38097) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn7)) * assign34170_e38101) - (assign34170_e38098 * (-locals.var_q_invexpq__blk831_dn7))) / (assign34170_e38101 * assign34170_e38101)), (((((locals.var_q_rac_qsq__blk828_dn8 * assign34170_e38097) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn8)) * assign34170_e38101) - (assign34170_e38098 * (-locals.var_q_invexpq__blk831_dn8))) / (assign34170_e38101 * assign34170_e38101)), (((((locals.var_q_rac_qsq__blk828_dn9 * assign34170_e38097) + (locals.var_q_rac_qsq__blk828 * locals.var_q_invexpq__blk831_dn9)) * assign34170_e38101) - (assign34170_e38098 * (-locals.var_q_invexpq__blk831_dn9))) / (assign34170_e38101 * assign34170_e38101)),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign34170_e38104;
        locals.var_q_qcoth__blk829_dn4 = assign34170_e38104_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign34170_e38104_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign34170_e38104_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign34170_e38104_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign34170_e38104_d_n9;

        let (assign34180_e38133, assign34180_e38133_d_n4, assign34180_e38133_d_n6, assign34180_e38133_d_n7, assign34180_e38133_d_n8, assign34180_e38133_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1140 == 0.0)) && (locals.var_guard1141 == 0.0)) {
        let assign34180_e38118: f64 = (locals.var_qsqs__blk942 * 0.1666666666667);
        let assign34180_e38122: f64 = (locals.var_qsqs__blk942 * 0.0166666666667);
        let assign34180_e38126: f64 = (locals.var_qsqs__blk942 * 0.0238095238095);
        let assign34180_e38127: f64 = (1.0 - assign34180_e38126);
        let assign34180_e38128: f64 = (assign34180_e38122 * assign34180_e38127);
        let assign34180_e38129: f64 = (1.0 - assign34180_e38128);
        let assign34180_e38130: f64 = (assign34180_e38118 * assign34180_e38129);
        let assign34180_e38131: f64 = (2.0 + assign34180_e38130);
        (assign34180_e38131, (((locals.var_qsqs__blk942_dn4 * 0.1666666666667) * assign34180_e38129) + (assign34180_e38118 * (-(((locals.var_qsqs__blk942_dn4 * 0.0166666666667) * assign34180_e38127) + (assign34180_e38122 * (-(locals.var_qsqs__blk942_dn4 * 0.0238095238095))))))), (((locals.var_qsqs__blk942_dn6 * 0.1666666666667) * assign34180_e38129) + (assign34180_e38118 * (-(((locals.var_qsqs__blk942_dn6 * 0.0166666666667) * assign34180_e38127) + (assign34180_e38122 * (-(locals.var_qsqs__blk942_dn6 * 0.0238095238095))))))), (((locals.var_qsqs__blk942_dn7 * 0.1666666666667) * assign34180_e38129) + (assign34180_e38118 * (-(((locals.var_qsqs__blk942_dn7 * 0.0166666666667) * assign34180_e38127) + (assign34180_e38122 * (-(locals.var_qsqs__blk942_dn7 * 0.0238095238095))))))), (((locals.var_qsqs__blk942_dn8 * 0.1666666666667) * assign34180_e38129) + (assign34180_e38118 * (-(((locals.var_qsqs__blk942_dn8 * 0.0166666666667) * assign34180_e38127) + (assign34180_e38122 * (-(locals.var_qsqs__blk942_dn8 * 0.0238095238095))))))), (((locals.var_qsqs__blk942_dn9 * 0.1666666666667) * assign34180_e38129) + (assign34180_e38118 * (-(((locals.var_qsqs__blk942_dn9 * 0.0166666666667) * assign34180_e38127) + (assign34180_e38122 * (-(locals.var_qsqs__blk942_dn9 * 0.0238095238095))))))),)
    } else {
        (locals.var_q_qcoth__blk829, locals.var_q_qcoth__blk829_dn4, locals.var_q_qcoth__blk829_dn6, locals.var_q_qcoth__blk829_dn7, locals.var_q_qcoth__blk829_dn8, locals.var_q_qcoth__blk829_dn9,)
    }
};
        locals.var_q_qcoth__blk829 = assign34180_e38133;
        locals.var_q_qcoth__blk829_dn4 = assign34180_e38133_d_n4;
        locals.var_q_qcoth__blk829_dn6 = assign34180_e38133_d_n6;
        locals.var_q_qcoth__blk829_dn7 = assign34180_e38133_d_n7;
        locals.var_q_qcoth__blk829_dn8 = assign34180_e38133_d_n8;
        locals.var_q_qcoth__blk829_dn9 = assign34180_e38133_d_n9;

        let assign34190_e38136: f64 = (1.01 * locals.var_k1q1s__blk939);
        let assign34190_e38138: f64 = (assign34190_e38136 + locals.var_q_qcoth__blk829);
        let assign34190_e38140: f64 = if assign34190_e38138 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1142 = assign34190_e38140;

        let (assign34200_e38151, assign34200_e38151_d_n4, assign34200_e38151_d_n6, assign34200_e38151_d_n7, assign34200_e38151_d_n8, assign34200_e38151_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 != 0.0)) {
        let assign34200_e38149: f64 = (locals.var_k1q1s__blk939 + locals.var_q_qcoth__blk829);
        (assign34200_e38149, (locals.var_k1q1s__blk939_dn4 + locals.var_q_qcoth__blk829_dn4), (locals.var_k1q1s__blk939_dn6 + locals.var_q_qcoth__blk829_dn6), (locals.var_k1q1s__blk939_dn7 + locals.var_q_qcoth__blk829_dn7), (locals.var_k1q1s__blk939_dn8 + locals.var_q_qcoth__blk829_dn8), (locals.var_k1q1s__blk939_dn9 + locals.var_q_qcoth__blk829_dn9),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign34200_e38151;
        locals.var_q_temp1__blk814_dn4 = assign34200_e38151_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign34200_e38151_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign34200_e38151_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign34200_e38151_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign34200_e38151_d_n9;

        let assign34210_e38154: f64 = (locals.var_aexp1s__blk943 * locals.var_k1q1s__blk939);
        let assign34210_e38157: f64 = (0.9 * locals.var_k1q1s__blk939);
        let assign34210_e38159: f64 = (assign34210_e38157 * locals.var_k1q1s__blk939);
        let assign34210_e38161: f64 = (assign34210_e38159 * locals.var_q_temp1__blk814);
        let assign34210_e38162: f64 = if assign34210_e38154 < assign34210_e38161 { 1.0 } else { 0.0 };
        locals.var_guard1143 = assign34210_e38162;

        let (assign34220_e38177, assign34220_e38177_d_n4, assign34220_e38177_d_n6, assign34220_e38177_d_n7, assign34220_e38177_d_n8, assign34220_e38177_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 != 0.0)) && (locals.var_guard1143 != 0.0)) {
        let assign34220_e38173: f64 = (locals.var_aexp1s__blk943 / locals.var_q_temp1__blk814);
        let assign34220_e38175: f64 = (assign34220_e38173 + 1e-80);
        (assign34220_e38175, (((locals.var_aexp1s__blk943_dn4 * locals.var_q_temp1__blk814) - (locals.var_aexp1s__blk943 * locals.var_q_temp1__blk814_dn4)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1s__blk943_dn6 * locals.var_q_temp1__blk814) - (locals.var_aexp1s__blk943 * locals.var_q_temp1__blk814_dn6)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1s__blk943_dn7 * locals.var_q_temp1__blk814) - (locals.var_aexp1s__blk943 * locals.var_q_temp1__blk814_dn7)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1s__blk943_dn8 * locals.var_q_temp1__blk814) - (locals.var_aexp1s__blk943 * locals.var_q_temp1__blk814_dn8)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)), (((locals.var_aexp1s__blk943_dn9 * locals.var_q_temp1__blk814) - (locals.var_aexp1s__blk943 * locals.var_q_temp1__blk814_dn9)) / (locals.var_q_temp1__blk814 * locals.var_q_temp1__blk814)),)
    } else {
        (locals.var_qis__blk938, locals.var_qis__blk938_dn4, locals.var_qis__blk938_dn6, locals.var_qis__blk938_dn7, locals.var_qis__blk938_dn8, locals.var_qis__blk938_dn9,)
    }
};
        locals.var_qis__blk938 = assign34220_e38177;
        locals.var_qis__blk938_dn4 = assign34220_e38177_d_n4;
        locals.var_qis__blk938_dn6 = assign34220_e38177_d_n6;
        locals.var_qis__blk938_dn7 = assign34220_e38177_d_n7;
        locals.var_qis__blk938_dn8 = assign34220_e38177_d_n8;
        locals.var_qis__blk938_dn9 = assign34220_e38177_d_n9;

        let (assign34230_e38190, assign34230_e38190_d_n4, assign34230_e38190_d_n6, assign34230_e38190_d_n7, assign34230_e38190_d_n8, assign34230_e38190_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 != 0.0)) && (locals.var_guard1143 != 0.0)) {
        let assign34230_e38188: f64 = (locals.var_qis__blk938 - locals.var_k1q1s__blk939);
        (assign34230_e38188, (locals.var_qis__blk938_dn4 - locals.var_k1q1s__blk939_dn4), (locals.var_qis__blk938_dn6 - locals.var_k1q1s__blk939_dn6), (locals.var_qis__blk938_dn7 - locals.var_k1q1s__blk939_dn7), (locals.var_qis__blk938_dn8 - locals.var_k1q1s__blk939_dn8), (locals.var_qis__blk938_dn9 - locals.var_k1q1s__blk939_dn9),)
    } else {
        (locals.var_k2q2s__blk940, locals.var_k2q2s__blk940_dn4, locals.var_k2q2s__blk940_dn6, locals.var_k2q2s__blk940_dn7, locals.var_k2q2s__blk940_dn8, locals.var_k2q2s__blk940_dn9,)
    }
};
        locals.var_k2q2s__blk940 = assign34230_e38190;
        locals.var_k2q2s__blk940_dn4 = assign34230_e38190_d_n4;
        locals.var_k2q2s__blk940_dn6 = assign34230_e38190_d_n6;
        locals.var_k2q2s__blk940_dn7 = assign34230_e38190_d_n7;
        locals.var_k2q2s__blk940_dn8 = assign34230_e38190_d_n8;
        locals.var_k2q2s__blk940_dn9 = assign34230_e38190_d_n9;

        let (assign34240_e38203, assign34240_e38203_d_n4, assign34240_e38203_d_n6, assign34240_e38203_d_n7, assign34240_e38203_d_n8, assign34240_e38203_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 != 0.0)) && (locals.var_guard1143 != 0.0)) {
        let assign34240_e38201: f64 = (locals.var_k2q2s__blk940 / locals.var_k2__blk933);
        (assign34240_e38201, (((locals.var_k2q2s__blk940_dn4 * locals.var_k2__blk933) - (locals.var_k2q2s__blk940 * locals.var_k2__blk933_dn4)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2s__blk940_dn6 * locals.var_k2__blk933) - (locals.var_k2q2s__blk940 * locals.var_k2__blk933_dn6)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2s__blk940_dn7 * locals.var_k2__blk933) - (locals.var_k2q2s__blk940 * locals.var_k2__blk933_dn7)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2s__blk940_dn8 * locals.var_k2__blk933) - (locals.var_k2q2s__blk940 * locals.var_k2__blk933_dn8)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2s__blk940_dn9 * locals.var_k2__blk933) - (locals.var_k2q2s__blk940 * locals.var_k2__blk933_dn9)) / (locals.var_k2__blk933 * locals.var_k2__blk933)),)
    } else {
        (locals.var_q2s__blk941, locals.var_q2s__blk941_dn4, locals.var_q2s__blk941_dn6, locals.var_q2s__blk941_dn7, locals.var_q2s__blk941_dn8, locals.var_q2s__blk941_dn9,)
    }
};
        locals.var_q2s__blk941 = assign34240_e38203;
        locals.var_q2s__blk941_dn4 = assign34240_e38203_d_n4;
        locals.var_q2s__blk941_dn6 = assign34240_e38203_d_n6;
        locals.var_q2s__blk941_dn7 = assign34240_e38203_d_n7;
        locals.var_q2s__blk941_dn8 = assign34240_e38203_d_n8;
        locals.var_q2s__blk941_dn9 = assign34240_e38203_d_n9;

        let assign34250_e38206: f64 = if locals.var_qsqs__blk942 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1144 = assign34250_e38206;

        let (assign34260_e38233, assign34260_e38233_d_n4, assign34260_e38233_d_n6, assign34260_e38233_d_n7, assign34260_e38233_d_n8, assign34260_e38233_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 != 0.0)) && (locals.var_guard1143 == 0.0)) && (locals.var_guard1144 != 0.0)) {
        let assign34260_e38220: f64 = (4.0 * locals.var_qsqs__blk942);
        let assign34260_e38225: f64 = (2.0 - locals.var_q_invexpq__blk831);
        let assign34260_e38226: f64 = (locals.var_q_invexpq__blk831 * assign34260_e38225);
        let assign34260_e38227: f64 = (1.0 - assign34260_e38226);
        let assign34260_e38228: f64 = (assign34260_e38220 / assign34260_e38227);
        let assign34260_e38229: f64 = (assign34260_e38228).ln();
        let assign34260_e38231: f64 = (assign34260_e38229 - locals.var_q_rac_qsq__blk828);
        (assign34260_e38231, ((((((4.0 * locals.var_qsqs__blk942_dn4) * assign34260_e38227) - (assign34260_e38220 * (-((locals.var_q_invexpq__blk831_dn4 * assign34260_e38225) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign34260_e38227 * assign34260_e38227)) / assign34260_e38228) - locals.var_q_rac_qsq__blk828_dn4), ((((((4.0 * locals.var_qsqs__blk942_dn6) * assign34260_e38227) - (assign34260_e38220 * (-((locals.var_q_invexpq__blk831_dn6 * assign34260_e38225) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign34260_e38227 * assign34260_e38227)) / assign34260_e38228) - locals.var_q_rac_qsq__blk828_dn6), ((((((4.0 * locals.var_qsqs__blk942_dn7) * assign34260_e38227) - (assign34260_e38220 * (-((locals.var_q_invexpq__blk831_dn7 * assign34260_e38225) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign34260_e38227 * assign34260_e38227)) / assign34260_e38228) - locals.var_q_rac_qsq__blk828_dn7), ((((((4.0 * locals.var_qsqs__blk942_dn8) * assign34260_e38227) - (assign34260_e38220 * (-((locals.var_q_invexpq__blk831_dn8 * assign34260_e38225) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign34260_e38227 * assign34260_e38227)) / assign34260_e38228) - locals.var_q_rac_qsq__blk828_dn8), ((((((4.0 * locals.var_qsqs__blk942_dn9) * assign34260_e38227) - (assign34260_e38220 * (-((locals.var_q_invexpq__blk831_dn9 * assign34260_e38225) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign34260_e38227 * assign34260_e38227)) / assign34260_e38228) - locals.var_q_rac_qsq__blk828_dn9),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign34260_e38233;
        locals.var_q_temp2__blk815_dn4 = assign34260_e38233_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign34260_e38233_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign34260_e38233_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign34260_e38233_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign34260_e38233_d_n9;

        let assign34270_e38236: f64 = (-0.005);
        let assign34270_e38237: f64 = if locals.var_qsqs__blk942 < assign34270_e38236 { 1.0 } else { 0.0 };
        locals.var_guard1145 = assign34270_e38237;

        let (assign34280_e38257, assign34280_e38257_d_n4, assign34280_e38257_d_n6, assign34280_e38257_d_n7, assign34280_e38257_d_n8, assign34280_e38257_d_n9,) = {
    if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 != 0.0)) && (locals.var_guard1143 == 0.0)) && (locals.var_guard1144 == 0.0)) && (locals.var_guard1145 != 0.0)) {
        let assign34280_e38254: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign34280_e38255: f64 = (assign34280_e38254).sin();
        (assign34280_e38255, ((assign34280_e38254).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign34280_e38254).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign34280_e38254).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign34280_e38254).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign34280_e38254).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign34280_e38257;
        locals.var_q_temp3__blk816_dn4 = assign34280_e38257_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign34280_e38257_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign34280_e38257_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign34280_e38257_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign34280_e38257_d_n9;

        let (assign34290_e38280, assign34290_e38280_d_n4, assign34290_e38280_d_n6, assign34290_e38280_d_n7, assign34290_e38280_d_n8, assign34290_e38280_d_n9,) = {
    if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 != 0.0)) && (locals.var_guard1143 == 0.0)) && (locals.var_guard1144 == 0.0)) && (locals.var_guard1145 != 0.0)) {
        let assign34290_e38273: f64 = (-locals.var_qsqs__blk942);
        let assign34290_e38276: f64 = (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816);
        let assign34290_e38277: f64 = (assign34290_e38273 / assign34290_e38276);
        let assign34290_e38278: f64 = (assign34290_e38277).ln();
        (assign34290_e38278, (((((-locals.var_qsqs__blk942_dn4) * assign34290_e38276) - (assign34290_e38273 * ((locals.var_q_temp3__blk816_dn4 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn4)))) / (assign34290_e38276 * assign34290_e38276)) / assign34290_e38277), (((((-locals.var_qsqs__blk942_dn6) * assign34290_e38276) - (assign34290_e38273 * ((locals.var_q_temp3__blk816_dn6 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn6)))) / (assign34290_e38276 * assign34290_e38276)) / assign34290_e38277), (((((-locals.var_qsqs__blk942_dn7) * assign34290_e38276) - (assign34290_e38273 * ((locals.var_q_temp3__blk816_dn7 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn7)))) / (assign34290_e38276 * assign34290_e38276)) / assign34290_e38277), (((((-locals.var_qsqs__blk942_dn8) * assign34290_e38276) - (assign34290_e38273 * ((locals.var_q_temp3__blk816_dn8 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn8)))) / (assign34290_e38276 * assign34290_e38276)) / assign34290_e38277), (((((-locals.var_qsqs__blk942_dn9) * assign34290_e38276) - (assign34290_e38273 * ((locals.var_q_temp3__blk816_dn9 * locals.var_q_temp3__blk816) + (locals.var_q_temp3__blk816 * locals.var_q_temp3__blk816_dn9)))) / (assign34290_e38276 * assign34290_e38276)) / assign34290_e38277),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign34290_e38280;
        locals.var_q_temp2__blk815_dn4 = assign34290_e38280_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign34290_e38280_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign34290_e38280_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign34290_e38280_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign34290_e38280_d_n9;

        let (assign34300_e38315, assign34300_e38315_d_n4, assign34300_e38315_d_n6, assign34300_e38315_d_n7, assign34300_e38315_d_n8, assign34300_e38315_d_n9,) = {
    if ((((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 != 0.0)) && (locals.var_guard1143 == 0.0)) && (locals.var_guard1144 == 0.0)) && (locals.var_guard1145 == 0.0)) {
        let assign34300_e38299: f64 = (locals.var_qsqs__blk942 * 0.3333333333333);
        let assign34300_e38303: f64 = (0.05 * locals.var_qsqs__blk942);
        let assign34300_e38307: f64 = (0.0396825396825397 * locals.var_qsqs__blk942);
        let assign34300_e38308: f64 = (1.0 - assign34300_e38307);
        let assign34300_e38309: f64 = (assign34300_e38303 * assign34300_e38308);
        let assign34300_e38310: f64 = (1.0 - assign34300_e38309);
        let assign34300_e38311: f64 = (assign34300_e38299 * assign34300_e38310);
        let assign34300_e38312: f64 = (4.0 - assign34300_e38311);
        let assign34300_e38313: f64 = (assign34300_e38312).ln();
        (assign34300_e38313, ((-(((locals.var_qsqs__blk942_dn4 * 0.3333333333333) * assign34300_e38310) + (assign34300_e38299 * (-(((0.05 * locals.var_qsqs__blk942_dn4) * assign34300_e38308) + (assign34300_e38303 * (-(0.0396825396825397 * locals.var_qsqs__blk942_dn4)))))))) / assign34300_e38312), ((-(((locals.var_qsqs__blk942_dn6 * 0.3333333333333) * assign34300_e38310) + (assign34300_e38299 * (-(((0.05 * locals.var_qsqs__blk942_dn6) * assign34300_e38308) + (assign34300_e38303 * (-(0.0396825396825397 * locals.var_qsqs__blk942_dn6)))))))) / assign34300_e38312), ((-(((locals.var_qsqs__blk942_dn7 * 0.3333333333333) * assign34300_e38310) + (assign34300_e38299 * (-(((0.05 * locals.var_qsqs__blk942_dn7) * assign34300_e38308) + (assign34300_e38303 * (-(0.0396825396825397 * locals.var_qsqs__blk942_dn7)))))))) / assign34300_e38312), ((-(((locals.var_qsqs__blk942_dn8 * 0.3333333333333) * assign34300_e38310) + (assign34300_e38299 * (-(((0.05 * locals.var_qsqs__blk942_dn8) * assign34300_e38308) + (assign34300_e38303 * (-(0.0396825396825397 * locals.var_qsqs__blk942_dn8)))))))) / assign34300_e38312), ((-(((locals.var_qsqs__blk942_dn9 * 0.3333333333333) * assign34300_e38310) + (assign34300_e38299 * (-(((0.05 * locals.var_qsqs__blk942_dn9) * assign34300_e38308) + (assign34300_e38303 * (-(0.0396825396825397 * locals.var_qsqs__blk942_dn9)))))))) / assign34300_e38312),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign34300_e38315;
        locals.var_q_temp2__blk815_dn4 = assign34300_e38315_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign34300_e38315_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign34300_e38315_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign34300_e38315_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign34300_e38315_d_n9;

        let (assign34310_e38338, assign34310_e38338_d_n4, assign34310_e38338_d_n6, assign34310_e38338_d_n7, assign34310_e38338_d_n8, assign34310_e38338_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 != 0.0)) && (locals.var_guard1143 == 0.0)) {
        let assign34310_e38327: f64 = (locals.var_xg2x__blk931 - locals.var_xg1x__blk930);
        let assign34310_e38329: f64 = (assign34310_e38327 + locals.var_q1s__blk937);
        let assign34310_e38332: f64 = (locals.var_q_temp1__blk814).ln();
        let assign34310_e38333: f64 = (2.0 * assign34310_e38332);
        let assign34310_e38334: f64 = (assign34310_e38329 + assign34310_e38333);
        let assign34310_e38336: f64 = (assign34310_e38334 - locals.var_q_temp2__blk815);
        (assign34310_e38336, ((((locals.var_xg2x__blk931_dn4 - locals.var_xg1x__blk930_dn4) + locals.var_q1s__blk937_dn4) + (2.0 * (locals.var_q_temp1__blk814_dn4 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn4), ((((locals.var_xg2x__blk931_dn6 - locals.var_xg1x__blk930_dn6) + locals.var_q1s__blk937_dn6) + (2.0 * (locals.var_q_temp1__blk814_dn6 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn6), ((((locals.var_xg2x__blk931_dn7 - locals.var_xg1x__blk930_dn7) + locals.var_q1s__blk937_dn7) + (2.0 * (locals.var_q_temp1__blk814_dn7 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn7), ((((locals.var_xg2x__blk931_dn8 - locals.var_xg1x__blk930_dn8) + locals.var_q1s__blk937_dn8) + (2.0 * (locals.var_q_temp1__blk814_dn8 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn8), ((((locals.var_xg2x__blk931_dn9 - locals.var_xg1x__blk930_dn9) + locals.var_q1s__blk937_dn9) + (2.0 * (locals.var_q_temp1__blk814_dn9 / locals.var_q_temp1__blk814))) - locals.var_q_temp2__blk815_dn9),)
    } else {
        (locals.var_q2s__blk941, locals.var_q2s__blk941_dn4, locals.var_q2s__blk941_dn6, locals.var_q2s__blk941_dn7, locals.var_q2s__blk941_dn8, locals.var_q2s__blk941_dn9,)
    }
};
        locals.var_q2s__blk941 = assign34310_e38338;
        locals.var_q2s__blk941_dn4 = assign34310_e38338_d_n4;
        locals.var_q2s__blk941_dn6 = assign34310_e38338_d_n6;
        locals.var_q2s__blk941_dn7 = assign34310_e38338_d_n7;
        locals.var_q2s__blk941_dn8 = assign34310_e38338_d_n8;
        locals.var_q2s__blk941_dn9 = assign34310_e38338_d_n9;

        let (assign34320_e38352, assign34320_e38352_d_n4, assign34320_e38352_d_n6, assign34320_e38352_d_n7, assign34320_e38352_d_n8, assign34320_e38352_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 != 0.0)) && (locals.var_guard1143 == 0.0)) {
        let assign34320_e38350: f64 = (locals.var_k2__blk933 * locals.var_q2s__blk941);
        (assign34320_e38350, ((locals.var_k2__blk933_dn4 * locals.var_q2s__blk941) + (locals.var_k2__blk933 * locals.var_q2s__blk941_dn4)), ((locals.var_k2__blk933_dn6 * locals.var_q2s__blk941) + (locals.var_k2__blk933 * locals.var_q2s__blk941_dn6)), ((locals.var_k2__blk933_dn7 * locals.var_q2s__blk941) + (locals.var_k2__blk933 * locals.var_q2s__blk941_dn7)), ((locals.var_k2__blk933_dn8 * locals.var_q2s__blk941) + (locals.var_k2__blk933 * locals.var_q2s__blk941_dn8)), ((locals.var_k2__blk933_dn9 * locals.var_q2s__blk941) + (locals.var_k2__blk933 * locals.var_q2s__blk941_dn9)),)
    } else {
        (locals.var_k2q2s__blk940, locals.var_k2q2s__blk940_dn4, locals.var_k2q2s__blk940_dn6, locals.var_k2q2s__blk940_dn7, locals.var_k2q2s__blk940_dn8, locals.var_k2q2s__blk940_dn9,)
    }
};
        locals.var_k2q2s__blk940 = assign34320_e38352;
        locals.var_k2q2s__blk940_dn4 = assign34320_e38352_d_n4;
        locals.var_k2q2s__blk940_dn6 = assign34320_e38352_d_n6;
        locals.var_k2q2s__blk940_dn7 = assign34320_e38352_d_n7;
        locals.var_k2q2s__blk940_dn8 = assign34320_e38352_d_n8;
        locals.var_k2q2s__blk940_dn9 = assign34320_e38352_d_n9;

        let (assign34330_e38366, assign34330_e38366_d_n4, assign34330_e38366_d_n6, assign34330_e38366_d_n7, assign34330_e38366_d_n8, assign34330_e38366_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 != 0.0)) && (locals.var_guard1143 == 0.0)) {
        let assign34330_e38364: f64 = (locals.var_k1q1s__blk939 + locals.var_k2q2s__blk940);
        (assign34330_e38364, (locals.var_k1q1s__blk939_dn4 + locals.var_k2q2s__blk940_dn4), (locals.var_k1q1s__blk939_dn6 + locals.var_k2q2s__blk940_dn6), (locals.var_k1q1s__blk939_dn7 + locals.var_k2q2s__blk940_dn7), (locals.var_k1q1s__blk939_dn8 + locals.var_k2q2s__blk940_dn8), (locals.var_k1q1s__blk939_dn9 + locals.var_k2q2s__blk940_dn9),)
    } else {
        (locals.var_qis__blk938, locals.var_qis__blk938_dn4, locals.var_qis__blk938_dn6, locals.var_qis__blk938_dn7, locals.var_qis__blk938_dn8, locals.var_qis__blk938_dn9,)
    }
};
        locals.var_qis__blk938 = assign34330_e38366;
        locals.var_qis__blk938_dn4 = assign34330_e38366_d_n4;
        locals.var_qis__blk938_dn6 = assign34330_e38366_d_n6;
        locals.var_qis__blk938_dn7 = assign34330_e38366_d_n7;
        locals.var_qis__blk938_dn8 = assign34330_e38366_d_n8;
        locals.var_qis__blk938_dn9 = assign34330_e38366_d_n9;

        let assign34340_e38369: f64 = if locals.var_qsqs__blk942 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1146 = assign34340_e38369;

        let assign34350_e38372: f64 = locals.var_q1s__blk937;
        let assign34350_e38374: f64 = (assign34350_e38372 - locals.var_xg1x__blk930);
        let assign34350_e38376: f64 = (assign34350_e38374 - locals.var_q_rac_qsq__blk828);
        let assign34350_e38378: f64 = if assign34350_e38376 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1147 = assign34350_e38378;

        let (assign34360_e38399, assign34360_e38399_d_n4, assign34360_e38399_d_n6, assign34360_e38399_d_n7, assign34360_e38399_d_n8, assign34360_e38399_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 == 0.0)) && (locals.var_guard1146 != 0.0)) && (locals.var_guard1147 != 0.0)) {
        let assign34360_e38392: f64 = locals.var_q1s__blk937;
        let assign34360_e38394: f64 = (assign34360_e38392 - locals.var_xg1x__blk930);
        let assign34360_e38396: f64 = (assign34360_e38394 - locals.var_q_rac_qsq__blk828);
        let assign34360_e38397: f64 = (assign34360_e38396).exp();
        (assign34360_e38397, (assign34360_e38397 * ((locals.var_q1s__blk937_dn4 - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4)), (assign34360_e38397 * ((locals.var_q1s__blk937_dn6 - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6)), (assign34360_e38397 * ((locals.var_q1s__blk937_dn7 - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7)), (assign34360_e38397 * ((locals.var_q1s__blk937_dn8 - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8)), (assign34360_e38397 * ((locals.var_q1s__blk937_dn9 - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign34360_e38399;
        locals.var_q_temp3__blk816_dn4 = assign34360_e38399_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign34360_e38399_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign34360_e38399_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign34360_e38399_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign34360_e38399_d_n9;

    }

    pub(super) fn stamp_transient_block_93(
        locals: &mut StampLocals,
    ) {
        let (assign34370_e38454, assign34370_e38454_d_n4, assign34370_e38454_d_n6, assign34370_e38454_d_n7, assign34370_e38454_d_n8, assign34370_e38454_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 == 0.0)) && (locals.var_guard1146 != 0.0)) && (locals.var_guard1147 == 0.0)) {
        let assign34370_e38416: f64 = locals.var_q1s__blk937;
        let assign34370_e38418: f64 = (assign34370_e38416 - locals.var_xg1x__blk930);
        let assign34370_e38420: f64 = (assign34370_e38418 - locals.var_q_rac_qsq__blk828);
        let assign34370_e38422: f64 = (assign34370_e38420 - 80.0);
        let assign34370_e38427: f64 = locals.var_q1s__blk937;
        let assign34370_e38429: f64 = (assign34370_e38427 - locals.var_xg1x__blk930);
        let assign34370_e38431: f64 = (assign34370_e38429 - locals.var_q_rac_qsq__blk828);
        let assign34370_e38433: f64 = (assign34370_e38431 - 80.0);
        let assign34370_e38434: f64 = (0.5 * assign34370_e38433);
        let assign34370_e38438: f64 = locals.var_q1s__blk937;
        let assign34370_e38440: f64 = (assign34370_e38438 - locals.var_xg1x__blk930);
        let assign34370_e38442: f64 = (assign34370_e38440 - locals.var_q_rac_qsq__blk828);
        let assign34370_e38444: f64 = (assign34370_e38442 - 80.0);
        let assign34370_e38446: f64 = (assign34370_e38444 * 0.3333333333333);
        let assign34370_e38447: f64 = (1.0 + assign34370_e38446);
        let assign34370_e38448: f64 = (assign34370_e38434 * assign34370_e38447);
        let assign34370_e38449: f64 = (1.0 + assign34370_e38448);
        let assign34370_e38450: f64 = (assign34370_e38422 * assign34370_e38449);
        let assign34370_e38451: f64 = (1.0 + assign34370_e38450);
        let assign34370_e38452: f64 = (5.54062e34 * assign34370_e38451);
        (assign34370_e38452, (5.54062e34 * ((((locals.var_q1s__blk937_dn4 - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4) * assign34370_e38449) + (assign34370_e38422 * (((0.5 * ((locals.var_q1s__blk937_dn4 - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4)) * assign34370_e38447) + (assign34370_e38434 * (((locals.var_q1s__blk937_dn4 - locals.var_xg1x__blk930_dn4) - locals.var_q_rac_qsq__blk828_dn4) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_q1s__blk937_dn6 - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6) * assign34370_e38449) + (assign34370_e38422 * (((0.5 * ((locals.var_q1s__blk937_dn6 - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6)) * assign34370_e38447) + (assign34370_e38434 * (((locals.var_q1s__blk937_dn6 - locals.var_xg1x__blk930_dn6) - locals.var_q_rac_qsq__blk828_dn6) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_q1s__blk937_dn7 - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7) * assign34370_e38449) + (assign34370_e38422 * (((0.5 * ((locals.var_q1s__blk937_dn7 - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7)) * assign34370_e38447) + (assign34370_e38434 * (((locals.var_q1s__blk937_dn7 - locals.var_xg1x__blk930_dn7) - locals.var_q_rac_qsq__blk828_dn7) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_q1s__blk937_dn8 - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8) * assign34370_e38449) + (assign34370_e38422 * (((0.5 * ((locals.var_q1s__blk937_dn8 - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8)) * assign34370_e38447) + (assign34370_e38434 * (((locals.var_q1s__blk937_dn8 - locals.var_xg1x__blk930_dn8) - locals.var_q_rac_qsq__blk828_dn8) * 0.3333333333333)))))), (5.54062e34 * ((((locals.var_q1s__blk937_dn9 - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9) * assign34370_e38449) + (assign34370_e38422 * (((0.5 * ((locals.var_q1s__blk937_dn9 - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9)) * assign34370_e38447) + (assign34370_e38434 * (((locals.var_q1s__blk937_dn9 - locals.var_xg1x__blk930_dn9) - locals.var_q_rac_qsq__blk828_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp3__blk816, locals.var_q_temp3__blk816_dn4, locals.var_q_temp3__blk816_dn6, locals.var_q_temp3__blk816_dn7, locals.var_q_temp3__blk816_dn8, locals.var_q_temp3__blk816_dn9,)
    }
};
        locals.var_q_temp3__blk816 = assign34370_e38454;
        locals.var_q_temp3__blk816_dn4 = assign34370_e38454_d_n4;
        locals.var_q_temp3__blk816_dn6 = assign34370_e38454_d_n6;
        locals.var_q_temp3__blk816_dn7 = assign34370_e38454_d_n7;
        locals.var_q_temp3__blk816_dn8 = assign34370_e38454_d_n8;
        locals.var_q_temp3__blk816_dn9 = assign34370_e38454_d_n9;

        let (assign34380_e38468, assign34380_e38468_d_n4, assign34380_e38468_d_n6, assign34380_e38468_d_n7, assign34380_e38468_d_n8, assign34380_e38468_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 == 0.0)) && (locals.var_guard1146 != 0.0)) {
        let assign34380_e38466: f64 = (locals.var_q_temp3__blk816 / locals.var_a0__blk905);
        (assign34380_e38466, (((locals.var_q_temp3__blk816_dn4 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn4)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn6 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn6)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn7 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn7)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn8 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn8)) / (locals.var_a0__blk905 * locals.var_a0__blk905)), (((locals.var_q_temp3__blk816_dn9 * locals.var_a0__blk905) - (locals.var_q_temp3__blk816 * locals.var_a0__blk905_dn9)) / (locals.var_a0__blk905 * locals.var_a0__blk905)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign34380_e38468;
        locals.var_q_temp2__blk815_dn4 = assign34380_e38468_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign34380_e38468_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign34380_e38468_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign34380_e38468_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign34380_e38468_d_n9;

        let (assign34390_e38492, assign34390_e38492_d_n4, assign34390_e38492_d_n6, assign34390_e38492_d_n7, assign34390_e38492_d_n8, assign34390_e38492_d_n9,) = {
    if ((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 == 0.0)) && (locals.var_guard1146 != 0.0)) {
        let assign34390_e38480: f64 = (4.0 * locals.var_qsqs__blk942);
        let assign34390_e38482: f64 = (assign34390_e38480 * locals.var_q_temp2__blk815);
        let assign34390_e38487: f64 = (2.0 - locals.var_q_invexpq__blk831);
        let assign34390_e38488: f64 = (locals.var_q_invexpq__blk831 * assign34390_e38487);
        let assign34390_e38489: f64 = (1.0 - assign34390_e38488);
        let assign34390_e38490: f64 = (assign34390_e38482 / assign34390_e38489);
        (assign34390_e38490, ((((((4.0 * locals.var_qsqs__blk942_dn4) * locals.var_q_temp2__blk815) + (assign34390_e38480 * locals.var_q_temp2__blk815_dn4)) * assign34390_e38489) - (assign34390_e38482 * (-((locals.var_q_invexpq__blk831_dn4 * assign34390_e38487) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn4)))))) / (assign34390_e38489 * assign34390_e38489)), ((((((4.0 * locals.var_qsqs__blk942_dn6) * locals.var_q_temp2__blk815) + (assign34390_e38480 * locals.var_q_temp2__blk815_dn6)) * assign34390_e38489) - (assign34390_e38482 * (-((locals.var_q_invexpq__blk831_dn6 * assign34390_e38487) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn6)))))) / (assign34390_e38489 * assign34390_e38489)), ((((((4.0 * locals.var_qsqs__blk942_dn7) * locals.var_q_temp2__blk815) + (assign34390_e38480 * locals.var_q_temp2__blk815_dn7)) * assign34390_e38489) - (assign34390_e38482 * (-((locals.var_q_invexpq__blk831_dn7 * assign34390_e38487) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn7)))))) / (assign34390_e38489 * assign34390_e38489)), ((((((4.0 * locals.var_qsqs__blk942_dn8) * locals.var_q_temp2__blk815) + (assign34390_e38480 * locals.var_q_temp2__blk815_dn8)) * assign34390_e38489) - (assign34390_e38482 * (-((locals.var_q_invexpq__blk831_dn8 * assign34390_e38487) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn8)))))) / (assign34390_e38489 * assign34390_e38489)), ((((((4.0 * locals.var_qsqs__blk942_dn9) * locals.var_q_temp2__blk815) + (assign34390_e38480 * locals.var_q_temp2__blk815_dn9)) * assign34390_e38489) - (assign34390_e38482 * (-((locals.var_q_invexpq__blk831_dn9 * assign34390_e38487) + (locals.var_q_invexpq__blk831 * (-locals.var_q_invexpq__blk831_dn9)))))) / (assign34390_e38489 * assign34390_e38489)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign34390_e38492;
        locals.var_q_temp1__blk814_dn4 = assign34390_e38492_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign34390_e38492_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign34390_e38492_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign34390_e38492_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign34390_e38492_d_n9;

        let assign34400_e38495: f64 = (-0.005);
        let assign34400_e38496: f64 = if locals.var_qsqs__blk942 < assign34400_e38495 { 1.0 } else { 0.0 };
        locals.var_guard1148 = assign34400_e38496;

        let (assign34410_e38514, assign34410_e38514_d_n4, assign34410_e38514_d_n6, assign34410_e38514_d_n7, assign34410_e38514_d_n8, assign34410_e38514_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 == 0.0)) && (locals.var_guard1146 == 0.0)) && (locals.var_guard1148 != 0.0)) {
        let assign34410_e38511: f64 = (0.5 * locals.var_q_rac_qsq__blk828);
        let assign34410_e38512: f64 = (assign34410_e38511).sin();
        (assign34410_e38512, ((assign34410_e38511).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn4)), ((assign34410_e38511).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn6)), ((assign34410_e38511).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn7)), ((assign34410_e38511).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn8)), ((assign34410_e38511).cos() * (0.5 * locals.var_q_rac_qsq__blk828_dn9)),)
    } else {
        (locals.var_q_temp2__blk815, locals.var_q_temp2__blk815_dn4, locals.var_q_temp2__blk815_dn6, locals.var_q_temp2__blk815_dn7, locals.var_q_temp2__blk815_dn8, locals.var_q_temp2__blk815_dn9,)
    }
};
        locals.var_q_temp2__blk815 = assign34410_e38514;
        locals.var_q_temp2__blk815_dn4 = assign34410_e38514_d_n4;
        locals.var_q_temp2__blk815_dn6 = assign34410_e38514_d_n6;
        locals.var_q_temp2__blk815_dn7 = assign34410_e38514_d_n7;
        locals.var_q_temp2__blk815_dn8 = assign34410_e38514_d_n8;
        locals.var_q_temp2__blk815_dn9 = assign34410_e38514_d_n9;

        let (assign34420_e38536, assign34420_e38536_d_n4, assign34420_e38536_d_n6, assign34420_e38536_d_n7, assign34420_e38536_d_n8, assign34420_e38536_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 == 0.0)) && (locals.var_guard1146 == 0.0)) && (locals.var_guard1148 != 0.0)) {
        let assign34420_e38528: f64 = (-locals.var_qsqs__blk942);
        let assign34420_e38531: f64 = (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815);
        let assign34420_e38532: f64 = (assign34420_e38528 / assign34420_e38531);
        let assign34420_e38534: f64 = (assign34420_e38532 / locals.var_aexp1s__blk943);
        (assign34420_e38534, (((((((-locals.var_qsqs__blk942_dn4) * assign34420_e38531) - (assign34420_e38528 * ((locals.var_q_temp2__blk815_dn4 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn4)))) / (assign34420_e38531 * assign34420_e38531)) * locals.var_aexp1s__blk943) - (assign34420_e38532 * locals.var_aexp1s__blk943_dn4)) / (locals.var_aexp1s__blk943 * locals.var_aexp1s__blk943)), (((((((-locals.var_qsqs__blk942_dn6) * assign34420_e38531) - (assign34420_e38528 * ((locals.var_q_temp2__blk815_dn6 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn6)))) / (assign34420_e38531 * assign34420_e38531)) * locals.var_aexp1s__blk943) - (assign34420_e38532 * locals.var_aexp1s__blk943_dn6)) / (locals.var_aexp1s__blk943 * locals.var_aexp1s__blk943)), (((((((-locals.var_qsqs__blk942_dn7) * assign34420_e38531) - (assign34420_e38528 * ((locals.var_q_temp2__blk815_dn7 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn7)))) / (assign34420_e38531 * assign34420_e38531)) * locals.var_aexp1s__blk943) - (assign34420_e38532 * locals.var_aexp1s__blk943_dn7)) / (locals.var_aexp1s__blk943 * locals.var_aexp1s__blk943)), (((((((-locals.var_qsqs__blk942_dn8) * assign34420_e38531) - (assign34420_e38528 * ((locals.var_q_temp2__blk815_dn8 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn8)))) / (assign34420_e38531 * assign34420_e38531)) * locals.var_aexp1s__blk943) - (assign34420_e38532 * locals.var_aexp1s__blk943_dn8)) / (locals.var_aexp1s__blk943 * locals.var_aexp1s__blk943)), (((((((-locals.var_qsqs__blk942_dn9) * assign34420_e38531) - (assign34420_e38528 * ((locals.var_q_temp2__blk815_dn9 * locals.var_q_temp2__blk815) + (locals.var_q_temp2__blk815 * locals.var_q_temp2__blk815_dn9)))) / (assign34420_e38531 * assign34420_e38531)) * locals.var_aexp1s__blk943) - (assign34420_e38532 * locals.var_aexp1s__blk943_dn9)) / (locals.var_aexp1s__blk943 * locals.var_aexp1s__blk943)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign34420_e38536;
        locals.var_q_temp1__blk814_dn4 = assign34420_e38536_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign34420_e38536_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign34420_e38536_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign34420_e38536_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign34420_e38536_d_n9;

        let (assign34430_e38570, assign34430_e38570_d_n4, assign34430_e38570_d_n6, assign34430_e38570_d_n7, assign34430_e38570_d_n8, assign34430_e38570_d_n9,) = {
    if (((((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 == 0.0)) && (locals.var_guard1146 == 0.0)) && (locals.var_guard1148 == 0.0)) {
        let assign34430_e38553: f64 = (locals.var_qsqs__blk942 * 0.3333333333333);
        let assign34430_e38557: f64 = (0.05 * locals.var_qsqs__blk942);
        let assign34430_e38561: f64 = (0.0396825396825397 * locals.var_qsqs__blk942);
        let assign34430_e38562: f64 = (1.0 - assign34430_e38561);
        let assign34430_e38563: f64 = (assign34430_e38557 * assign34430_e38562);
        let assign34430_e38564: f64 = (1.0 - assign34430_e38563);
        let assign34430_e38565: f64 = (assign34430_e38553 * assign34430_e38564);
        let assign34430_e38566: f64 = (4.0 - assign34430_e38565);
        let assign34430_e38568: f64 = (assign34430_e38566 / locals.var_aexp1s__blk943);
        (assign34430_e38568, ((((-(((locals.var_qsqs__blk942_dn4 * 0.3333333333333) * assign34430_e38564) + (assign34430_e38553 * (-(((0.05 * locals.var_qsqs__blk942_dn4) * assign34430_e38562) + (assign34430_e38557 * (-(0.0396825396825397 * locals.var_qsqs__blk942_dn4)))))))) * locals.var_aexp1s__blk943) - (assign34430_e38566 * locals.var_aexp1s__blk943_dn4)) / (locals.var_aexp1s__blk943 * locals.var_aexp1s__blk943)), ((((-(((locals.var_qsqs__blk942_dn6 * 0.3333333333333) * assign34430_e38564) + (assign34430_e38553 * (-(((0.05 * locals.var_qsqs__blk942_dn6) * assign34430_e38562) + (assign34430_e38557 * (-(0.0396825396825397 * locals.var_qsqs__blk942_dn6)))))))) * locals.var_aexp1s__blk943) - (assign34430_e38566 * locals.var_aexp1s__blk943_dn6)) / (locals.var_aexp1s__blk943 * locals.var_aexp1s__blk943)), ((((-(((locals.var_qsqs__blk942_dn7 * 0.3333333333333) * assign34430_e38564) + (assign34430_e38553 * (-(((0.05 * locals.var_qsqs__blk942_dn7) * assign34430_e38562) + (assign34430_e38557 * (-(0.0396825396825397 * locals.var_qsqs__blk942_dn7)))))))) * locals.var_aexp1s__blk943) - (assign34430_e38566 * locals.var_aexp1s__blk943_dn7)) / (locals.var_aexp1s__blk943 * locals.var_aexp1s__blk943)), ((((-(((locals.var_qsqs__blk942_dn8 * 0.3333333333333) * assign34430_e38564) + (assign34430_e38553 * (-(((0.05 * locals.var_qsqs__blk942_dn8) * assign34430_e38562) + (assign34430_e38557 * (-(0.0396825396825397 * locals.var_qsqs__blk942_dn8)))))))) * locals.var_aexp1s__blk943) - (assign34430_e38566 * locals.var_aexp1s__blk943_dn8)) / (locals.var_aexp1s__blk943 * locals.var_aexp1s__blk943)), ((((-(((locals.var_qsqs__blk942_dn9 * 0.3333333333333) * assign34430_e38564) + (assign34430_e38553 * (-(((0.05 * locals.var_qsqs__blk942_dn9) * assign34430_e38562) + (assign34430_e38557 * (-(0.0396825396825397 * locals.var_qsqs__blk942_dn9)))))))) * locals.var_aexp1s__blk943) - (assign34430_e38566 * locals.var_aexp1s__blk943_dn9)) / (locals.var_aexp1s__blk943 * locals.var_aexp1s__blk943)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign34430_e38570;
        locals.var_q_temp1__blk814_dn4 = assign34430_e38570_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign34430_e38570_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign34430_e38570_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign34430_e38570_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign34430_e38570_d_n9;

        let (assign34440_e38588, assign34440_e38588_d_n4, assign34440_e38588_d_n6, assign34440_e38588_d_n7, assign34440_e38588_d_n8, assign34440_e38588_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 == 0.0)) {
        let assign34440_e38580: f64 = (locals.var_k1q1s__blk939 - locals.var_q_qcoth__blk829);
        let assign34440_e38583: f64 = (1.0 - locals.var_q_temp1__blk814);
        let assign34440_e38584: f64 = (assign34440_e38580 / assign34440_e38583);
        let assign34440_e38586: f64 = (assign34440_e38584 + 1e-80);
        (assign34440_e38586, ((((locals.var_k1q1s__blk939_dn4 - locals.var_q_qcoth__blk829_dn4) * assign34440_e38583) - (assign34440_e38580 * (-locals.var_q_temp1__blk814_dn4))) / (assign34440_e38583 * assign34440_e38583)), ((((locals.var_k1q1s__blk939_dn6 - locals.var_q_qcoth__blk829_dn6) * assign34440_e38583) - (assign34440_e38580 * (-locals.var_q_temp1__blk814_dn6))) / (assign34440_e38583 * assign34440_e38583)), ((((locals.var_k1q1s__blk939_dn7 - locals.var_q_qcoth__blk829_dn7) * assign34440_e38583) - (assign34440_e38580 * (-locals.var_q_temp1__blk814_dn7))) / (assign34440_e38583 * assign34440_e38583)), ((((locals.var_k1q1s__blk939_dn8 - locals.var_q_qcoth__blk829_dn8) * assign34440_e38583) - (assign34440_e38580 * (-locals.var_q_temp1__blk814_dn8))) / (assign34440_e38583 * assign34440_e38583)), ((((locals.var_k1q1s__blk939_dn9 - locals.var_q_qcoth__blk829_dn9) * assign34440_e38583) - (assign34440_e38580 * (-locals.var_q_temp1__blk814_dn9))) / (assign34440_e38583 * assign34440_e38583)),)
    } else {
        (locals.var_qis__blk938, locals.var_qis__blk938_dn4, locals.var_qis__blk938_dn6, locals.var_qis__blk938_dn7, locals.var_qis__blk938_dn8, locals.var_qis__blk938_dn9,)
    }
};
        locals.var_qis__blk938 = assign34440_e38588;
        locals.var_qis__blk938_dn4 = assign34440_e38588_d_n4;
        locals.var_qis__blk938_dn6 = assign34440_e38588_d_n6;
        locals.var_qis__blk938_dn7 = assign34440_e38588_d_n7;
        locals.var_qis__blk938_dn8 = assign34440_e38588_d_n8;
        locals.var_qis__blk938_dn9 = assign34440_e38588_d_n9;

        let (assign34450_e38600, assign34450_e38600_d_n4, assign34450_e38600_d_n6, assign34450_e38600_d_n7, assign34450_e38600_d_n8, assign34450_e38600_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 == 0.0)) {
        let assign34450_e38598: f64 = (locals.var_qis__blk938 - locals.var_k1q1s__blk939);
        (assign34450_e38598, (locals.var_qis__blk938_dn4 - locals.var_k1q1s__blk939_dn4), (locals.var_qis__blk938_dn6 - locals.var_k1q1s__blk939_dn6), (locals.var_qis__blk938_dn7 - locals.var_k1q1s__blk939_dn7), (locals.var_qis__blk938_dn8 - locals.var_k1q1s__blk939_dn8), (locals.var_qis__blk938_dn9 - locals.var_k1q1s__blk939_dn9),)
    } else {
        (locals.var_k2q2s__blk940, locals.var_k2q2s__blk940_dn4, locals.var_k2q2s__blk940_dn6, locals.var_k2q2s__blk940_dn7, locals.var_k2q2s__blk940_dn8, locals.var_k2q2s__blk940_dn9,)
    }
};
        locals.var_k2q2s__blk940 = assign34450_e38600;
        locals.var_k2q2s__blk940_dn4 = assign34450_e38600_d_n4;
        locals.var_k2q2s__blk940_dn6 = assign34450_e38600_d_n6;
        locals.var_k2q2s__blk940_dn7 = assign34450_e38600_d_n7;
        locals.var_k2q2s__blk940_dn8 = assign34450_e38600_d_n8;
        locals.var_k2q2s__blk940_dn9 = assign34450_e38600_d_n9;

        let (assign34460_e38612, assign34460_e38612_d_n4, assign34460_e38612_d_n6, assign34460_e38612_d_n7, assign34460_e38612_d_n8, assign34460_e38612_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1142 == 0.0)) {
        let assign34460_e38610: f64 = (locals.var_k2q2s__blk940 / locals.var_k2__blk933);
        (assign34460_e38610, (((locals.var_k2q2s__blk940_dn4 * locals.var_k2__blk933) - (locals.var_k2q2s__blk940 * locals.var_k2__blk933_dn4)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2s__blk940_dn6 * locals.var_k2__blk933) - (locals.var_k2q2s__blk940 * locals.var_k2__blk933_dn6)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2s__blk940_dn7 * locals.var_k2__blk933) - (locals.var_k2q2s__blk940 * locals.var_k2__blk933_dn7)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2s__blk940_dn8 * locals.var_k2__blk933) - (locals.var_k2q2s__blk940 * locals.var_k2__blk933_dn8)) / (locals.var_k2__blk933 * locals.var_k2__blk933)), (((locals.var_k2q2s__blk940_dn9 * locals.var_k2__blk933) - (locals.var_k2q2s__blk940 * locals.var_k2__blk933_dn9)) / (locals.var_k2__blk933 * locals.var_k2__blk933)),)
    } else {
        (locals.var_q2s__blk941, locals.var_q2s__blk941_dn4, locals.var_q2s__blk941_dn6, locals.var_q2s__blk941_dn7, locals.var_q2s__blk941_dn8, locals.var_q2s__blk941_dn9,)
    }
};
        locals.var_q2s__blk941 = assign34460_e38612;
        locals.var_q2s__blk941_dn4 = assign34460_e38612_d_n4;
        locals.var_q2s__blk941_dn6 = assign34460_e38612_d_n6;
        locals.var_q2s__blk941_dn7 = assign34460_e38612_d_n7;
        locals.var_q2s__blk941_dn8 = assign34460_e38612_d_n8;
        locals.var_q2s__blk941_dn9 = assign34460_e38612_d_n9;

        let assign34470_e38615: f64 = (locals.var_xg2x__blk931 - locals.var_q2s__blk941);
        let assign34470_e38617: f64 = assign34470_e38615;
        let assign34470_e38619: f64 = if assign34470_e38617 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1149 = assign34470_e38619;

        let (assign34480_e38630, assign34480_e38630_d_n4, assign34480_e38630_d_n6, assign34480_e38630_d_n7, assign34480_e38630_d_n8, assign34480_e38630_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1149 != 0.0)) {
        let assign34480_e38625: f64 = (locals.var_xg2x__blk931 - locals.var_q2s__blk941);
        let assign34480_e38627: f64 = assign34480_e38625;
        let assign34480_e38628: f64 = (assign34480_e38627).exp();
        (assign34480_e38628, (assign34480_e38628 * (locals.var_xg2x__blk931_dn4 - locals.var_q2s__blk941_dn4)), (assign34480_e38628 * (locals.var_xg2x__blk931_dn6 - locals.var_q2s__blk941_dn6)), (assign34480_e38628 * (locals.var_xg2x__blk931_dn7 - locals.var_q2s__blk941_dn7)), (assign34480_e38628 * (locals.var_xg2x__blk931_dn8 - locals.var_q2s__blk941_dn8)), (assign34480_e38628 * (locals.var_xg2x__blk931_dn9 - locals.var_q2s__blk941_dn9)),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign34480_e38630;
        locals.var_q_temp1__blk814_dn4 = assign34480_e38630_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign34480_e38630_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign34480_e38630_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign34480_e38630_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign34480_e38630_d_n9;

        let (assign34490_e38671, assign34490_e38671_d_n4, assign34490_e38671_d_n6, assign34490_e38671_d_n7, assign34490_e38671_d_n8, assign34490_e38671_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1149 == 0.0)) {
        let assign34490_e38639: f64 = (locals.var_xg2x__blk931 - locals.var_q2s__blk941);
        let assign34490_e38641: f64 = assign34490_e38639;
        let assign34490_e38643: f64 = (assign34490_e38641 - 80.0);
        let assign34490_e38648: f64 = (locals.var_xg2x__blk931 - locals.var_q2s__blk941);
        let assign34490_e38650: f64 = assign34490_e38648;
        let assign34490_e38652: f64 = (assign34490_e38650 - 80.0);
        let assign34490_e38653: f64 = (0.5 * assign34490_e38652);
        let assign34490_e38657: f64 = (locals.var_xg2x__blk931 - locals.var_q2s__blk941);
        let assign34490_e38659: f64 = assign34490_e38657;
        let assign34490_e38661: f64 = (assign34490_e38659 - 80.0);
        let assign34490_e38663: f64 = (assign34490_e38661 * 0.3333333333333);
        let assign34490_e38664: f64 = (1.0 + assign34490_e38663);
        let assign34490_e38665: f64 = (assign34490_e38653 * assign34490_e38664);
        let assign34490_e38666: f64 = (1.0 + assign34490_e38665);
        let assign34490_e38667: f64 = (assign34490_e38643 * assign34490_e38666);
        let assign34490_e38668: f64 = (1.0 + assign34490_e38667);
        let assign34490_e38669: f64 = (5.54062e34 * assign34490_e38668);
        (assign34490_e38669, (5.54062e34 * (((locals.var_xg2x__blk931_dn4 - locals.var_q2s__blk941_dn4) * assign34490_e38666) + (assign34490_e38643 * (((0.5 * (locals.var_xg2x__blk931_dn4 - locals.var_q2s__blk941_dn4)) * assign34490_e38664) + (assign34490_e38653 * ((locals.var_xg2x__blk931_dn4 - locals.var_q2s__blk941_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg2x__blk931_dn6 - locals.var_q2s__blk941_dn6) * assign34490_e38666) + (assign34490_e38643 * (((0.5 * (locals.var_xg2x__blk931_dn6 - locals.var_q2s__blk941_dn6)) * assign34490_e38664) + (assign34490_e38653 * ((locals.var_xg2x__blk931_dn6 - locals.var_q2s__blk941_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg2x__blk931_dn7 - locals.var_q2s__blk941_dn7) * assign34490_e38666) + (assign34490_e38643 * (((0.5 * (locals.var_xg2x__blk931_dn7 - locals.var_q2s__blk941_dn7)) * assign34490_e38664) + (assign34490_e38653 * ((locals.var_xg2x__blk931_dn7 - locals.var_q2s__blk941_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg2x__blk931_dn8 - locals.var_q2s__blk941_dn8) * assign34490_e38666) + (assign34490_e38643 * (((0.5 * (locals.var_xg2x__blk931_dn8 - locals.var_q2s__blk941_dn8)) * assign34490_e38664) + (assign34490_e38653 * ((locals.var_xg2x__blk931_dn8 - locals.var_q2s__blk941_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg2x__blk931_dn9 - locals.var_q2s__blk941_dn9) * assign34490_e38666) + (assign34490_e38643 * (((0.5 * (locals.var_xg2x__blk931_dn9 - locals.var_q2s__blk941_dn9)) * assign34490_e38664) + (assign34490_e38653 * ((locals.var_xg2x__blk931_dn9 - locals.var_q2s__blk941_dn9) * 0.3333333333333)))))),)
    } else {
        (locals.var_q_temp1__blk814, locals.var_q_temp1__blk814_dn4, locals.var_q_temp1__blk814_dn6, locals.var_q_temp1__blk814_dn7, locals.var_q_temp1__blk814_dn8, locals.var_q_temp1__blk814_dn9,)
    }
};
        locals.var_q_temp1__blk814 = assign34490_e38671;
        locals.var_q_temp1__blk814_dn4 = assign34490_e38671_d_n4;
        locals.var_q_temp1__blk814_dn6 = assign34490_e38671_d_n6;
        locals.var_q_temp1__blk814_dn7 = assign34490_e38671_d_n7;
        locals.var_q_temp1__blk814_dn8 = assign34490_e38671_d_n8;
        locals.var_q_temp1__blk814_dn9 = assign34490_e38671_d_n9;

        let (assign34500_e38677, assign34500_e38677_d_n4, assign34500_e38677_d_n6, assign34500_e38677_d_n7, assign34500_e38677_d_n8, assign34500_e38677_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34500_e38675: f64 = (locals.var_a0__blk905 * locals.var_q_temp1__blk814);
        (assign34500_e38675, ((locals.var_a0__blk905_dn4 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn4)), ((locals.var_a0__blk905_dn6 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn6)), ((locals.var_a0__blk905_dn7 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn7)), ((locals.var_a0__blk905_dn8 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn8)), ((locals.var_a0__blk905_dn9 * locals.var_q_temp1__blk814) + (locals.var_a0__blk905 * locals.var_q_temp1__blk814_dn9)),)
    } else {
        (locals.var_aexp2s__blk944, locals.var_aexp2s__blk944_dn4, locals.var_aexp2s__blk944_dn6, locals.var_aexp2s__blk944_dn7, locals.var_aexp2s__blk944_dn8, locals.var_aexp2s__blk944_dn9,)
    }
};
        locals.var_aexp2s__blk944 = assign34500_e38677;
        locals.var_aexp2s__blk944_dn4 = assign34500_e38677_d_n4;
        locals.var_aexp2s__blk944_dn6 = assign34500_e38677_d_n6;
        locals.var_aexp2s__blk944_dn7 = assign34500_e38677_d_n7;
        locals.var_aexp2s__blk944_dn8 = assign34500_e38677_d_n8;
        locals.var_aexp2s__blk944_dn9 = assign34500_e38677_d_n9;

        let (assign34510_e38681, assign34510_e38681_d_n4, assign34510_e38681_d_n6, assign34510_e38681_d_n7, assign34510_e38681_d_n8, assign34510_e38681_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_a1s__blk947, locals.var_a1s__blk947_dn4, locals.var_a1s__blk947_dn6, locals.var_a1s__blk947_dn7, locals.var_a1s__blk947_dn8, locals.var_a1s__blk947_dn9,)
    }
};
        locals.var_a1s__blk947 = assign34510_e38681;
        locals.var_a1s__blk947_dn4 = assign34510_e38681_d_n4;
        locals.var_a1s__blk947_dn6 = assign34510_e38681_d_n6;
        locals.var_a1s__blk947_dn7 = assign34510_e38681_d_n7;
        locals.var_a1s__blk947_dn8 = assign34510_e38681_d_n8;
        locals.var_a1s__blk947_dn9 = assign34510_e38681_d_n9;

        let (assign34520_e38685, assign34520_e38685_d_n4, assign34520_e38685_d_n6, assign34520_e38685_d_n7, assign34520_e38685_d_n8, assign34520_e38685_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_a2s__blk948, locals.var_a2s__blk948_dn4, locals.var_a2s__blk948_dn6, locals.var_a2s__blk948_dn7, locals.var_a2s__blk948_dn8, locals.var_a2s__blk948_dn9,)
    }
};
        locals.var_a2s__blk948 = assign34520_e38685;
        locals.var_a2s__blk948_dn4 = assign34520_e38685_d_n4;
        locals.var_a2s__blk948_dn6 = assign34520_e38685_d_n6;
        locals.var_a2s__blk948_dn7 = assign34520_e38685_d_n7;
        locals.var_a2s__blk948_dn8 = assign34520_e38685_d_n8;
        locals.var_a2s__blk948_dn9 = assign34520_e38685_d_n9;

        let (assign34530_e38689, assign34530_e38689_d_n4, assign34530_e38689_d_n6, assign34530_e38689_d_n7, assign34530_e38689_d_n8, assign34530_e38689_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b1s__blk945, locals.var_b1s__blk945_dn4, locals.var_b1s__blk945_dn6, locals.var_b1s__blk945_dn7, locals.var_b1s__blk945_dn8, locals.var_b1s__blk945_dn9,)
    }
};
        locals.var_b1s__blk945 = assign34530_e38689;
        locals.var_b1s__blk945_dn4 = assign34530_e38689_d_n4;
        locals.var_b1s__blk945_dn6 = assign34530_e38689_d_n6;
        locals.var_b1s__blk945_dn7 = assign34530_e38689_d_n7;
        locals.var_b1s__blk945_dn8 = assign34530_e38689_d_n8;
        locals.var_b1s__blk945_dn9 = assign34530_e38689_d_n9;

        let (assign34540_e38693, assign34540_e38693_d_n4, assign34540_e38693_d_n6, assign34540_e38693_d_n7, assign34540_e38693_d_n8, assign34540_e38693_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b2s__blk946, locals.var_b2s__blk946_dn4, locals.var_b2s__blk946_dn6, locals.var_b2s__blk946_dn7, locals.var_b2s__blk946_dn8, locals.var_b2s__blk946_dn9,)
    }
};
        locals.var_b2s__blk946 = assign34540_e38693;
        locals.var_b2s__blk946_dn4 = assign34540_e38693_d_n4;
        locals.var_b2s__blk946_dn6 = assign34540_e38693_d_n6;
        locals.var_b2s__blk946_dn7 = assign34540_e38693_d_n7;
        locals.var_b2s__blk946_dn8 = assign34540_e38693_d_n8;
        locals.var_b2s__blk946_dn9 = assign34540_e38693_d_n9;

        let (assign34550_e38697, assign34550_e38697_d_n4, assign34550_e38697_d_n6, assign34550_e38697_d_n7, assign34550_e38697_d_n8, assign34550_e38697_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sums__blk949, locals.var_sums__blk949_dn4, locals.var_sums__blk949_dn6, locals.var_sums__blk949_dn7, locals.var_sums__blk949_dn8, locals.var_sums__blk949_dn9,)
    }
};
        locals.var_sums__blk949 = assign34550_e38697;
        locals.var_sums__blk949_dn4 = assign34550_e38697_d_n4;
        locals.var_sums__blk949_dn6 = assign34550_e38697_d_n6;
        locals.var_sums__blk949_dn7 = assign34550_e38697_d_n7;
        locals.var_sums__blk949_dn8 = assign34550_e38697_d_n8;
        locals.var_sums__blk949_dn9 = assign34550_e38697_d_n9;

        let (assign34560_e38701, assign34560_e38701_d_n4, assign34560_e38701_d_n6, assign34560_e38701_d_n7, assign34560_e38701_d_n8, assign34560_e38701_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dqsqs_dxn_qi__blk950, locals.var_dqsqs_dxn_qi__blk950_dn4, locals.var_dqsqs_dxn_qi__blk950_dn6, locals.var_dqsqs_dxn_qi__blk950_dn7, locals.var_dqsqs_dxn_qi__blk950_dn8, locals.var_dqsqs_dxn_qi__blk950_dn9,)
    }
};
        locals.var_dqsqs_dxn_qi__blk950 = assign34560_e38701;
        locals.var_dqsqs_dxn_qi__blk950_dn4 = assign34560_e38701_d_n4;
        locals.var_dqsqs_dxn_qi__blk950_dn6 = assign34560_e38701_d_n6;
        locals.var_dqsqs_dxn_qi__blk950_dn7 = assign34560_e38701_d_n7;
        locals.var_dqsqs_dxn_qi__blk950_dn8 = assign34560_e38701_d_n8;
        locals.var_dqsqs_dxn_qi__blk950_dn9 = assign34560_e38701_d_n9;

        let assign34570_e38704: f64 = if locals.var_qis__blk938 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1150 = assign34570_e38704;

        let (assign34580_e38712, assign34580_e38712_d_n4, assign34580_e38712_d_n6, assign34580_e38712_d_n7, assign34580_e38712_d_n8, assign34580_e38712_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1150 != 0.0)) {
        let assign34580_e38710: f64 = (locals.var_aexp1s__blk943 * locals.var_inv_k1__blk906);
        (assign34580_e38710, ((locals.var_aexp1s__blk943_dn4 * locals.var_inv_k1__blk906) + (locals.var_aexp1s__blk943 * locals.var_inv_k1__blk906_dn4)), ((locals.var_aexp1s__blk943_dn6 * locals.var_inv_k1__blk906) + (locals.var_aexp1s__blk943 * locals.var_inv_k1__blk906_dn6)), ((locals.var_aexp1s__blk943_dn7 * locals.var_inv_k1__blk906) + (locals.var_aexp1s__blk943 * locals.var_inv_k1__blk906_dn7)), ((locals.var_aexp1s__blk943_dn8 * locals.var_inv_k1__blk906) + (locals.var_aexp1s__blk943 * locals.var_inv_k1__blk906_dn8)), ((locals.var_aexp1s__blk943_dn9 * locals.var_inv_k1__blk906) + (locals.var_aexp1s__blk943 * locals.var_inv_k1__blk906_dn9)),)
    } else {
        (locals.var_b1s__blk945, locals.var_b1s__blk945_dn4, locals.var_b1s__blk945_dn6, locals.var_b1s__blk945_dn7, locals.var_b1s__blk945_dn8, locals.var_b1s__blk945_dn9,)
    }
};
        locals.var_b1s__blk945 = assign34580_e38712;
        locals.var_b1s__blk945_dn4 = assign34580_e38712_d_n4;
        locals.var_b1s__blk945_dn6 = assign34580_e38712_d_n6;
        locals.var_b1s__blk945_dn7 = assign34580_e38712_d_n7;
        locals.var_b1s__blk945_dn8 = assign34580_e38712_d_n8;
        locals.var_b1s__blk945_dn9 = assign34580_e38712_d_n9;

        let (assign34590_e38720, assign34590_e38720_d_n4, assign34590_e38720_d_n6, assign34590_e38720_d_n7, assign34590_e38720_d_n8, assign34590_e38720_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1150 != 0.0)) {
        let assign34590_e38718: f64 = (locals.var_aexp2s__blk944 * locals.var_inv_k2__blk907);
        (assign34590_e38718, ((locals.var_aexp2s__blk944_dn4 * locals.var_inv_k2__blk907) + (locals.var_aexp2s__blk944 * locals.var_inv_k2__blk907_dn4)), ((locals.var_aexp2s__blk944_dn6 * locals.var_inv_k2__blk907) + (locals.var_aexp2s__blk944 * locals.var_inv_k2__blk907_dn6)), ((locals.var_aexp2s__blk944_dn7 * locals.var_inv_k2__blk907) + (locals.var_aexp2s__blk944 * locals.var_inv_k2__blk907_dn7)), ((locals.var_aexp2s__blk944_dn8 * locals.var_inv_k2__blk907) + (locals.var_aexp2s__blk944 * locals.var_inv_k2__blk907_dn8)), ((locals.var_aexp2s__blk944_dn9 * locals.var_inv_k2__blk907) + (locals.var_aexp2s__blk944 * locals.var_inv_k2__blk907_dn9)),)
    } else {
        (locals.var_b2s__blk946, locals.var_b2s__blk946_dn4, locals.var_b2s__blk946_dn6, locals.var_b2s__blk946_dn7, locals.var_b2s__blk946_dn8, locals.var_b2s__blk946_dn9,)
    }
};
        locals.var_b2s__blk946 = assign34590_e38720;
        locals.var_b2s__blk946_dn4 = assign34590_e38720_d_n4;
        locals.var_b2s__blk946_dn6 = assign34590_e38720_d_n6;
        locals.var_b2s__blk946_dn7 = assign34590_e38720_d_n7;
        locals.var_b2s__blk946_dn8 = assign34590_e38720_d_n8;
        locals.var_b2s__blk946_dn9 = assign34590_e38720_d_n9;

        let (assign34600_e38730, assign34600_e38730_d_n4, assign34600_e38730_d_n6, assign34600_e38730_d_n7, assign34600_e38730_d_n8, assign34600_e38730_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1150 != 0.0)) {
        let assign34600_e38727: f64 = (2.0 * locals.var_k1q1s__blk939);
        let assign34600_e38728: f64 = (locals.var_b1s__blk945 + assign34600_e38727);
        (assign34600_e38728, (locals.var_b1s__blk945_dn4 + (2.0 * locals.var_k1q1s__blk939_dn4)), (locals.var_b1s__blk945_dn6 + (2.0 * locals.var_k1q1s__blk939_dn6)), (locals.var_b1s__blk945_dn7 + (2.0 * locals.var_k1q1s__blk939_dn7)), (locals.var_b1s__blk945_dn8 + (2.0 * locals.var_k1q1s__blk939_dn8)), (locals.var_b1s__blk945_dn9 + (2.0 * locals.var_k1q1s__blk939_dn9)),)
    } else {
        (locals.var_a1s__blk947, locals.var_a1s__blk947_dn4, locals.var_a1s__blk947_dn6, locals.var_a1s__blk947_dn7, locals.var_a1s__blk947_dn8, locals.var_a1s__blk947_dn9,)
    }
};
        locals.var_a1s__blk947 = assign34600_e38730;
        locals.var_a1s__blk947_dn4 = assign34600_e38730_d_n4;
        locals.var_a1s__blk947_dn6 = assign34600_e38730_d_n6;
        locals.var_a1s__blk947_dn7 = assign34600_e38730_d_n7;
        locals.var_a1s__blk947_dn8 = assign34600_e38730_d_n8;
        locals.var_a1s__blk947_dn9 = assign34600_e38730_d_n9;

        let (assign34610_e38740, assign34610_e38740_d_n4, assign34610_e38740_d_n6, assign34610_e38740_d_n7, assign34610_e38740_d_n8, assign34610_e38740_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1150 != 0.0)) {
        let assign34610_e38737: f64 = (2.0 * locals.var_k2q2s__blk940);
        let assign34610_e38738: f64 = (locals.var_b2s__blk946 + assign34610_e38737);
        (assign34610_e38738, (locals.var_b2s__blk946_dn4 + (2.0 * locals.var_k2q2s__blk940_dn4)), (locals.var_b2s__blk946_dn6 + (2.0 * locals.var_k2q2s__blk940_dn6)), (locals.var_b2s__blk946_dn7 + (2.0 * locals.var_k2q2s__blk940_dn7)), (locals.var_b2s__blk946_dn8 + (2.0 * locals.var_k2q2s__blk940_dn8)), (locals.var_b2s__blk946_dn9 + (2.0 * locals.var_k2q2s__blk940_dn9)),)
    } else {
        (locals.var_a2s__blk948, locals.var_a2s__blk948_dn4, locals.var_a2s__blk948_dn6, locals.var_a2s__blk948_dn7, locals.var_a2s__blk948_dn8, locals.var_a2s__blk948_dn9,)
    }
};
        locals.var_a2s__blk948 = assign34610_e38740;
        locals.var_a2s__blk948_dn4 = assign34610_e38740_d_n4;
        locals.var_a2s__blk948_dn6 = assign34610_e38740_d_n6;
        locals.var_a2s__blk948_dn7 = assign34610_e38740_d_n7;
        locals.var_a2s__blk948_dn8 = assign34610_e38740_d_n8;
        locals.var_a2s__blk948_dn9 = assign34610_e38740_d_n9;

        let (assign34620_e38752, assign34620_e38752_d_n4, assign34620_e38752_d_n6, assign34620_e38752_d_n7, assign34620_e38752_d_n8, assign34620_e38752_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1150 != 0.0)) {
        let assign34620_e38746: f64 = (2.0 * locals.var_qis__blk938);
        let assign34620_e38748: f64 = (assign34620_e38746 + locals.var_b1s__blk945);
        let assign34620_e38750: f64 = (assign34620_e38748 + locals.var_b2s__blk946);
        (assign34620_e38750, (((2.0 * locals.var_qis__blk938_dn4) + locals.var_b1s__blk945_dn4) + locals.var_b2s__blk946_dn4), (((2.0 * locals.var_qis__blk938_dn6) + locals.var_b1s__blk945_dn6) + locals.var_b2s__blk946_dn6), (((2.0 * locals.var_qis__blk938_dn7) + locals.var_b1s__blk945_dn7) + locals.var_b2s__blk946_dn7), (((2.0 * locals.var_qis__blk938_dn8) + locals.var_b1s__blk945_dn8) + locals.var_b2s__blk946_dn8), (((2.0 * locals.var_qis__blk938_dn9) + locals.var_b1s__blk945_dn9) + locals.var_b2s__blk946_dn9),)
    } else {
        (locals.var_sums__blk949, locals.var_sums__blk949_dn4, locals.var_sums__blk949_dn6, locals.var_sums__blk949_dn7, locals.var_sums__blk949_dn8, locals.var_sums__blk949_dn9,)
    }
};
        locals.var_sums__blk949 = assign34620_e38752;
        locals.var_sums__blk949_dn4 = assign34620_e38752_d_n4;
        locals.var_sums__blk949_dn6 = assign34620_e38752_d_n6;
        locals.var_sums__blk949_dn7 = assign34620_e38752_d_n7;
        locals.var_sums__blk949_dn8 = assign34620_e38752_d_n8;
        locals.var_sums__blk949_dn9 = assign34620_e38752_d_n9;

        let assign34630_e38754: f64 = (locals.var_qsqs__blk942).abs();
        let assign34630_e38756: f64 = if assign34630_e38754 > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard1151 = assign34630_e38756;

        let (assign34640_e38782, assign34640_e38782_d_n4, assign34640_e38782_d_n6, assign34640_e38782_d_n7, assign34640_e38782_d_n8, assign34640_e38782_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1150 != 0.0)) && (locals.var_guard1151 != 0.0)) {
        let assign34640_e38764: f64 = (locals.var_a1s__blk947 * locals.var_a2s__blk948);
        let assign34640_e38768: f64 = (locals.var_q1s__blk937 + 2.0);
        let assign34640_e38769: f64 = (2.0 * assign34640_e38768);
        let assign34640_e38771: f64 = (assign34640_e38769 * locals.var_a2s__blk948);
        let assign34640_e38772: f64 = (assign34640_e38764 + assign34640_e38771);
        let assign34640_e38776: f64 = (locals.var_q2s__blk941 + 2.0);
        let assign34640_e38777: f64 = (2.0 * assign34640_e38776);
        let assign34640_e38779: f64 = (assign34640_e38777 * locals.var_a1s__blk947);
        let assign34640_e38780: f64 = (assign34640_e38772 + assign34640_e38779);
        (assign34640_e38780, ((((locals.var_a1s__blk947_dn4 * locals.var_a2s__blk948) + (locals.var_a1s__blk947 * locals.var_a2s__blk948_dn4)) + (((2.0 * locals.var_q1s__blk937_dn4) * locals.var_a2s__blk948) + (assign34640_e38769 * locals.var_a2s__blk948_dn4))) + (((2.0 * locals.var_q2s__blk941_dn4) * locals.var_a1s__blk947) + (assign34640_e38777 * locals.var_a1s__blk947_dn4))), ((((locals.var_a1s__blk947_dn6 * locals.var_a2s__blk948) + (locals.var_a1s__blk947 * locals.var_a2s__blk948_dn6)) + (((2.0 * locals.var_q1s__blk937_dn6) * locals.var_a2s__blk948) + (assign34640_e38769 * locals.var_a2s__blk948_dn6))) + (((2.0 * locals.var_q2s__blk941_dn6) * locals.var_a1s__blk947) + (assign34640_e38777 * locals.var_a1s__blk947_dn6))), ((((locals.var_a1s__blk947_dn7 * locals.var_a2s__blk948) + (locals.var_a1s__blk947 * locals.var_a2s__blk948_dn7)) + (((2.0 * locals.var_q1s__blk937_dn7) * locals.var_a2s__blk948) + (assign34640_e38769 * locals.var_a2s__blk948_dn7))) + (((2.0 * locals.var_q2s__blk941_dn7) * locals.var_a1s__blk947) + (assign34640_e38777 * locals.var_a1s__blk947_dn7))), ((((locals.var_a1s__blk947_dn8 * locals.var_a2s__blk948) + (locals.var_a1s__blk947 * locals.var_a2s__blk948_dn8)) + (((2.0 * locals.var_q1s__blk937_dn8) * locals.var_a2s__blk948) + (assign34640_e38769 * locals.var_a2s__blk948_dn8))) + (((2.0 * locals.var_q2s__blk941_dn8) * locals.var_a1s__blk947) + (assign34640_e38777 * locals.var_a1s__blk947_dn8))), ((((locals.var_a1s__blk947_dn9 * locals.var_a2s__blk948) + (locals.var_a1s__blk947 * locals.var_a2s__blk948_dn9)) + (((2.0 * locals.var_q1s__blk937_dn9) * locals.var_a2s__blk948) + (assign34640_e38769 * locals.var_a2s__blk948_dn9))) + (((2.0 * locals.var_q2s__blk941_dn9) * locals.var_a1s__blk947) + (assign34640_e38777 * locals.var_a1s__blk947_dn9))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign34640_e38782;
        locals.var_temp1_dn4 = assign34640_e38782_d_n4;
        locals.var_temp1_dn6 = assign34640_e38782_d_n6;
        locals.var_temp1_dn7 = assign34640_e38782_d_n7;
        locals.var_temp1_dn8 = assign34640_e38782_d_n8;
        locals.var_temp1_dn9 = assign34640_e38782_d_n9;

        let (assign34650_e38799, assign34650_e38799_d_n4, assign34650_e38799_d_n6, assign34650_e38799_d_n7, assign34650_e38799_d_n8, assign34650_e38799_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1150 != 0.0)) && (locals.var_guard1151 != 0.0)) {
        let assign34650_e38789: f64 = (-4.0);
        let assign34650_e38791: f64 = (assign34650_e38789 * locals.var_qsqs__blk942);
        let assign34650_e38793: f64 = (assign34650_e38791 * locals.var_sums__blk949);
        let assign34650_e38796: f64 = (locals.var_qis__blk938 * locals.var_temp1);
        let assign34650_e38797: f64 = (assign34650_e38793 / assign34650_e38796);
        (assign34650_e38797, ((((((assign34650_e38789 * locals.var_qsqs__blk942_dn4) * locals.var_sums__blk949) + (assign34650_e38791 * locals.var_sums__blk949_dn4)) * assign34650_e38796) - (assign34650_e38793 * ((locals.var_qis__blk938_dn4 * locals.var_temp1) + (locals.var_qis__blk938 * locals.var_temp1_dn4)))) / (assign34650_e38796 * assign34650_e38796)), ((((((assign34650_e38789 * locals.var_qsqs__blk942_dn6) * locals.var_sums__blk949) + (assign34650_e38791 * locals.var_sums__blk949_dn6)) * assign34650_e38796) - (assign34650_e38793 * ((locals.var_qis__blk938_dn6 * locals.var_temp1) + (locals.var_qis__blk938 * locals.var_temp1_dn6)))) / (assign34650_e38796 * assign34650_e38796)), ((((((assign34650_e38789 * locals.var_qsqs__blk942_dn7) * locals.var_sums__blk949) + (assign34650_e38791 * locals.var_sums__blk949_dn7)) * assign34650_e38796) - (assign34650_e38793 * ((locals.var_qis__blk938_dn7 * locals.var_temp1) + (locals.var_qis__blk938 * locals.var_temp1_dn7)))) / (assign34650_e38796 * assign34650_e38796)), ((((((assign34650_e38789 * locals.var_qsqs__blk942_dn8) * locals.var_sums__blk949) + (assign34650_e38791 * locals.var_sums__blk949_dn8)) * assign34650_e38796) - (assign34650_e38793 * ((locals.var_qis__blk938_dn8 * locals.var_temp1) + (locals.var_qis__blk938 * locals.var_temp1_dn8)))) / (assign34650_e38796 * assign34650_e38796)), ((((((assign34650_e38789 * locals.var_qsqs__blk942_dn9) * locals.var_sums__blk949) + (assign34650_e38791 * locals.var_sums__blk949_dn9)) * assign34650_e38796) - (assign34650_e38793 * ((locals.var_qis__blk938_dn9 * locals.var_temp1) + (locals.var_qis__blk938 * locals.var_temp1_dn9)))) / (assign34650_e38796 * assign34650_e38796)),)
    } else {
        (locals.var_dqsqs_dxn_qi__blk950, locals.var_dqsqs_dxn_qi__blk950_dn4, locals.var_dqsqs_dxn_qi__blk950_dn6, locals.var_dqsqs_dxn_qi__blk950_dn7, locals.var_dqsqs_dxn_qi__blk950_dn8, locals.var_dqsqs_dxn_qi__blk950_dn9,)
    }
};
        locals.var_dqsqs_dxn_qi__blk950 = assign34650_e38799;
        locals.var_dqsqs_dxn_qi__blk950_dn4 = assign34650_e38799_d_n4;
        locals.var_dqsqs_dxn_qi__blk950_dn6 = assign34650_e38799_d_n6;
        locals.var_dqsqs_dxn_qi__blk950_dn7 = assign34650_e38799_d_n7;
        locals.var_dqsqs_dxn_qi__blk950_dn8 = assign34650_e38799_d_n8;
        locals.var_dqsqs_dxn_qi__blk950_dn9 = assign34650_e38799_d_n9;

        let (assign34660_e38826, assign34660_e38826_d_n4, assign34660_e38826_d_n6, assign34660_e38826_d_n7, assign34660_e38826_d_n8, assign34660_e38826_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1150 != 0.0)) && (locals.var_guard1151 == 0.0)) {
        let assign34660_e38810: f64 = (locals.var_qsqs__blk942 * 0.0333333333333);
        let assign34660_e38814: f64 = (locals.var_qsqs__blk942 * 0.0357142857143);
        let assign34660_e38818: f64 = (locals.var_qsqs__blk942 * 0.0333333333333);
        let assign34660_e38819: f64 = (1.0 - assign34660_e38818);
        let assign34660_e38820: f64 = (assign34660_e38814 * assign34660_e38819);
        let assign34660_e38821: f64 = (1.0 - assign34660_e38820);
        let assign34660_e38822: f64 = (assign34660_e38810 * assign34660_e38821);
        let assign34660_e38823: f64 = (1.0 - assign34660_e38822);
        let assign34660_e38824: f64 = (0.1666666666667 * assign34660_e38823);
        (assign34660_e38824, (0.1666666666667 * (-(((locals.var_qsqs__blk942_dn4 * 0.0333333333333) * assign34660_e38821) + (assign34660_e38810 * (-(((locals.var_qsqs__blk942_dn4 * 0.0357142857143) * assign34660_e38819) + (assign34660_e38814 * (-(locals.var_qsqs__blk942_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqs__blk942_dn6 * 0.0333333333333) * assign34660_e38821) + (assign34660_e38810 * (-(((locals.var_qsqs__blk942_dn6 * 0.0357142857143) * assign34660_e38819) + (assign34660_e38814 * (-(locals.var_qsqs__blk942_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqs__blk942_dn7 * 0.0333333333333) * assign34660_e38821) + (assign34660_e38810 * (-(((locals.var_qsqs__blk942_dn7 * 0.0357142857143) * assign34660_e38819) + (assign34660_e38814 * (-(locals.var_qsqs__blk942_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqs__blk942_dn8 * 0.0333333333333) * assign34660_e38821) + (assign34660_e38810 * (-(((locals.var_qsqs__blk942_dn8 * 0.0357142857143) * assign34660_e38819) + (assign34660_e38814 * (-(locals.var_qsqs__blk942_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_qsqs__blk942_dn9 * 0.0333333333333) * assign34660_e38821) + (assign34660_e38810 * (-(((locals.var_qsqs__blk942_dn9 * 0.0357142857143) * assign34660_e38819) + (assign34660_e38814 * (-(locals.var_qsqs__blk942_dn9 * 0.0333333333333))))))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign34660_e38826;
        locals.var_temp1_dn4 = assign34660_e38826_d_n4;
        locals.var_temp1_dn6 = assign34660_e38826_d_n6;
        locals.var_temp1_dn7 = assign34660_e38826_d_n7;
        locals.var_temp1_dn8 = assign34660_e38826_d_n8;
        locals.var_temp1_dn9 = assign34660_e38826_d_n9;

        let (assign34670_e38853, assign34670_e38853_d_n4, assign34670_e38853_d_n6, assign34670_e38853_d_n7, assign34670_e38853_d_n8, assign34670_e38853_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1150 != 0.0)) && (locals.var_guard1151 == 0.0)) {
        let assign34670_e38835: f64 = (locals.var_a1s__blk947 * locals.var_aexp1s__blk943);
        let assign34670_e38838: f64 = (locals.var_a2s__blk948 * locals.var_aexp2s__blk944);
        let assign34670_e38839: f64 = (assign34670_e38835 + assign34670_e38838);
        let assign34670_e38842: f64 = (locals.var_a1s__blk947 * locals.var_a2s__blk948);
        let assign34670_e38844: f64 = (assign34670_e38842 * locals.var_qis__blk938);
        let assign34670_e38848: f64 = (locals.var_qis__blk938 * locals.var_temp1);
        let assign34670_e38849: f64 = (1.0 + assign34670_e38848);
        let assign34670_e38850: f64 = (assign34670_e38844 * assign34670_e38849);
        let assign34670_e38851: f64 = (assign34670_e38839 + assign34670_e38850);
        (assign34670_e38851, ((((locals.var_a1s__blk947_dn4 * locals.var_aexp1s__blk943) + (locals.var_a1s__blk947 * locals.var_aexp1s__blk943_dn4)) + ((locals.var_a2s__blk948_dn4 * locals.var_aexp2s__blk944) + (locals.var_a2s__blk948 * locals.var_aexp2s__blk944_dn4))) + ((((((locals.var_a1s__blk947_dn4 * locals.var_a2s__blk948) + (locals.var_a1s__blk947 * locals.var_a2s__blk948_dn4)) * locals.var_qis__blk938) + (assign34670_e38842 * locals.var_qis__blk938_dn4)) * assign34670_e38849) + (assign34670_e38844 * ((locals.var_qis__blk938_dn4 * locals.var_temp1) + (locals.var_qis__blk938 * locals.var_temp1_dn4))))), ((((locals.var_a1s__blk947_dn6 * locals.var_aexp1s__blk943) + (locals.var_a1s__blk947 * locals.var_aexp1s__blk943_dn6)) + ((locals.var_a2s__blk948_dn6 * locals.var_aexp2s__blk944) + (locals.var_a2s__blk948 * locals.var_aexp2s__blk944_dn6))) + ((((((locals.var_a1s__blk947_dn6 * locals.var_a2s__blk948) + (locals.var_a1s__blk947 * locals.var_a2s__blk948_dn6)) * locals.var_qis__blk938) + (assign34670_e38842 * locals.var_qis__blk938_dn6)) * assign34670_e38849) + (assign34670_e38844 * ((locals.var_qis__blk938_dn6 * locals.var_temp1) + (locals.var_qis__blk938 * locals.var_temp1_dn6))))), ((((locals.var_a1s__blk947_dn7 * locals.var_aexp1s__blk943) + (locals.var_a1s__blk947 * locals.var_aexp1s__blk943_dn7)) + ((locals.var_a2s__blk948_dn7 * locals.var_aexp2s__blk944) + (locals.var_a2s__blk948 * locals.var_aexp2s__blk944_dn7))) + ((((((locals.var_a1s__blk947_dn7 * locals.var_a2s__blk948) + (locals.var_a1s__blk947 * locals.var_a2s__blk948_dn7)) * locals.var_qis__blk938) + (assign34670_e38842 * locals.var_qis__blk938_dn7)) * assign34670_e38849) + (assign34670_e38844 * ((locals.var_qis__blk938_dn7 * locals.var_temp1) + (locals.var_qis__blk938 * locals.var_temp1_dn7))))), ((((locals.var_a1s__blk947_dn8 * locals.var_aexp1s__blk943) + (locals.var_a1s__blk947 * locals.var_aexp1s__blk943_dn8)) + ((locals.var_a2s__blk948_dn8 * locals.var_aexp2s__blk944) + (locals.var_a2s__blk948 * locals.var_aexp2s__blk944_dn8))) + ((((((locals.var_a1s__blk947_dn8 * locals.var_a2s__blk948) + (locals.var_a1s__blk947 * locals.var_a2s__blk948_dn8)) * locals.var_qis__blk938) + (assign34670_e38842 * locals.var_qis__blk938_dn8)) * assign34670_e38849) + (assign34670_e38844 * ((locals.var_qis__blk938_dn8 * locals.var_temp1) + (locals.var_qis__blk938 * locals.var_temp1_dn8))))), ((((locals.var_a1s__blk947_dn9 * locals.var_aexp1s__blk943) + (locals.var_a1s__blk947 * locals.var_aexp1s__blk943_dn9)) + ((locals.var_a2s__blk948_dn9 * locals.var_aexp2s__blk944) + (locals.var_a2s__blk948 * locals.var_aexp2s__blk944_dn9))) + ((((((locals.var_a1s__blk947_dn9 * locals.var_a2s__blk948) + (locals.var_a1s__blk947 * locals.var_a2s__blk948_dn9)) * locals.var_qis__blk938) + (assign34670_e38842 * locals.var_qis__blk938_dn9)) * assign34670_e38849) + (assign34670_e38844 * ((locals.var_qis__blk938_dn9 * locals.var_temp1) + (locals.var_qis__blk938 * locals.var_temp1_dn9))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign34670_e38853;
        locals.var_temp2_dn4 = assign34670_e38853_d_n4;
        locals.var_temp2_dn6 = assign34670_e38853_d_n6;
        locals.var_temp2_dn7 = assign34670_e38853_d_n7;
        locals.var_temp2_dn8 = assign34670_e38853_d_n8;
        locals.var_temp2_dn9 = assign34670_e38853_d_n9;

    }

    pub(super) fn stamp_transient_block_94(
        locals: &mut StampLocals,
    ) {
        let (assign34680_e38870, assign34680_e38870_d_n4, assign34680_e38870_d_n6, assign34680_e38870_d_n7, assign34680_e38870_d_n8, assign34680_e38870_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1150 != 0.0)) && (locals.var_guard1151 == 0.0)) {
        let assign34680_e38862: f64 = (locals.var_aexp1s__blk943 * locals.var_aexp2s__blk944);
        let assign34680_e38864: f64 = (assign34680_e38862 * locals.var_sums__blk949);
        let assign34680_e38867: f64 = (locals.var_qis__blk938 * locals.var_temp2);
        let assign34680_e38868: f64 = (assign34680_e38864 / assign34680_e38867);
        (assign34680_e38868, (((((((locals.var_aexp1s__blk943_dn4 * locals.var_aexp2s__blk944) + (locals.var_aexp1s__blk943 * locals.var_aexp2s__blk944_dn4)) * locals.var_sums__blk949) + (assign34680_e38862 * locals.var_sums__blk949_dn4)) * assign34680_e38867) - (assign34680_e38864 * ((locals.var_qis__blk938_dn4 * locals.var_temp2) + (locals.var_qis__blk938 * locals.var_temp2_dn4)))) / (assign34680_e38867 * assign34680_e38867)), (((((((locals.var_aexp1s__blk943_dn6 * locals.var_aexp2s__blk944) + (locals.var_aexp1s__blk943 * locals.var_aexp2s__blk944_dn6)) * locals.var_sums__blk949) + (assign34680_e38862 * locals.var_sums__blk949_dn6)) * assign34680_e38867) - (assign34680_e38864 * ((locals.var_qis__blk938_dn6 * locals.var_temp2) + (locals.var_qis__blk938 * locals.var_temp2_dn6)))) / (assign34680_e38867 * assign34680_e38867)), (((((((locals.var_aexp1s__blk943_dn7 * locals.var_aexp2s__blk944) + (locals.var_aexp1s__blk943 * locals.var_aexp2s__blk944_dn7)) * locals.var_sums__blk949) + (assign34680_e38862 * locals.var_sums__blk949_dn7)) * assign34680_e38867) - (assign34680_e38864 * ((locals.var_qis__blk938_dn7 * locals.var_temp2) + (locals.var_qis__blk938 * locals.var_temp2_dn7)))) / (assign34680_e38867 * assign34680_e38867)), (((((((locals.var_aexp1s__blk943_dn8 * locals.var_aexp2s__blk944) + (locals.var_aexp1s__blk943 * locals.var_aexp2s__blk944_dn8)) * locals.var_sums__blk949) + (assign34680_e38862 * locals.var_sums__blk949_dn8)) * assign34680_e38867) - (assign34680_e38864 * ((locals.var_qis__blk938_dn8 * locals.var_temp2) + (locals.var_qis__blk938 * locals.var_temp2_dn8)))) / (assign34680_e38867 * assign34680_e38867)), (((((((locals.var_aexp1s__blk943_dn9 * locals.var_aexp2s__blk944) + (locals.var_aexp1s__blk943 * locals.var_aexp2s__blk944_dn9)) * locals.var_sums__blk949) + (assign34680_e38862 * locals.var_sums__blk949_dn9)) * assign34680_e38867) - (assign34680_e38864 * ((locals.var_qis__blk938_dn9 * locals.var_temp2) + (locals.var_qis__blk938 * locals.var_temp2_dn9)))) / (assign34680_e38867 * assign34680_e38867)),)
    } else {
        (locals.var_dqsqs_dxn_qi__blk950, locals.var_dqsqs_dxn_qi__blk950_dn4, locals.var_dqsqs_dxn_qi__blk950_dn6, locals.var_dqsqs_dxn_qi__blk950_dn7, locals.var_dqsqs_dxn_qi__blk950_dn8, locals.var_dqsqs_dxn_qi__blk950_dn9,)
    }
};
        locals.var_dqsqs_dxn_qi__blk950 = assign34680_e38870;
        locals.var_dqsqs_dxn_qi__blk950_dn4 = assign34680_e38870_d_n4;
        locals.var_dqsqs_dxn_qi__blk950_dn6 = assign34680_e38870_d_n6;
        locals.var_dqsqs_dxn_qi__blk950_dn7 = assign34680_e38870_d_n7;
        locals.var_dqsqs_dxn_qi__blk950_dn8 = assign34680_e38870_d_n8;
        locals.var_dqsqs_dxn_qi__blk950_dn9 = assign34680_e38870_d_n9;

        let (assign34690_e38875, assign34690_e38875_d_n4, assign34690_e38875_d_n6, assign34690_e38875_d_n7, assign34690_e38875_d_n8, assign34690_e38875_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34690_e38873: f64 = (locals.var_qis__blk938).ln();
        (assign34690_e38873, (locals.var_qis__blk938_dn4 / locals.var_qis__blk938), (locals.var_qis__blk938_dn6 / locals.var_qis__blk938), (locals.var_qis__blk938_dn7 / locals.var_qis__blk938), (locals.var_qis__blk938_dn8 / locals.var_qis__blk938), (locals.var_qis__blk938_dn9 / locals.var_qis__blk938),)
    } else {
        (locals.var_xdrifts__blk951, locals.var_xdrifts__blk951_dn4, locals.var_xdrifts__blk951_dn6, locals.var_xdrifts__blk951_dn7, locals.var_xdrifts__blk951_dn8, locals.var_xdrifts__blk951_dn9,)
    }
};
        locals.var_xdrifts__blk951 = assign34690_e38875;
        locals.var_xdrifts__blk951_dn4 = assign34690_e38875_d_n4;
        locals.var_xdrifts__blk951_dn6 = assign34690_e38875_d_n6;
        locals.var_xdrifts__blk951_dn7 = assign34690_e38875_d_n7;
        locals.var_xdrifts__blk951_dn8 = assign34690_e38875_d_n8;
        locals.var_xdrifts__blk951_dn9 = assign34690_e38875_d_n9;

        let assign34700_e38878: f64 = (locals.var_k1q1s__blk939 / 2.0);
        let assign34700_e38880: f64 = if assign34700_e38878 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1152 = assign34700_e38880;

        let (assign34710_e38892, assign34710_e38892_d_n4, assign34710_e38892_d_n6, assign34710_e38892_d_n7, assign34710_e38892_d_n8, assign34710_e38892_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1152 != 0.0)) {
        let assign34710_e38887: f64 = (locals.var_k1q1s__blk939 / 2.0);
        let assign34710_e38888: f64 = (assign34710_e38887).exp();
        let assign34710_e38889: f64 = (1.0 + assign34710_e38888);
        let assign34710_e38890: f64 = (assign34710_e38889).ln();
        (assign34710_e38890, ((assign34710_e38888 * (locals.var_k1q1s__blk939_dn4 / 2.0)) / assign34710_e38889), ((assign34710_e38888 * (locals.var_k1q1s__blk939_dn6 / 2.0)) / assign34710_e38889), ((assign34710_e38888 * (locals.var_k1q1s__blk939_dn7 / 2.0)) / assign34710_e38889), ((assign34710_e38888 * (locals.var_k1q1s__blk939_dn8 / 2.0)) / assign34710_e38889), ((assign34710_e38888 * (locals.var_k1q1s__blk939_dn9 / 2.0)) / assign34710_e38889),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign34710_e38892;
        locals.var_temp1_dn4 = assign34710_e38892_d_n4;
        locals.var_temp1_dn6 = assign34710_e38892_d_n6;
        locals.var_temp1_dn7 = assign34710_e38892_d_n7;
        locals.var_temp1_dn8 = assign34710_e38892_d_n8;
        locals.var_temp1_dn9 = assign34710_e38892_d_n9;

        let (assign34720_e38901, assign34720_e38901_d_n4, assign34720_e38901_d_n6, assign34720_e38901_d_n7, assign34720_e38901_d_n8, assign34720_e38901_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1152 == 0.0)) {
        let assign34720_e38899: f64 = (locals.var_k1q1s__blk939 / 2.0);
        (assign34720_e38899, (locals.var_k1q1s__blk939_dn4 / 2.0), (locals.var_k1q1s__blk939_dn6 / 2.0), (locals.var_k1q1s__blk939_dn7 / 2.0), (locals.var_k1q1s__blk939_dn8 / 2.0), (locals.var_k1q1s__blk939_dn9 / 2.0),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign34720_e38901;
        locals.var_temp1_dn4 = assign34720_e38901_d_n4;
        locals.var_temp1_dn6 = assign34720_e38901_d_n6;
        locals.var_temp1_dn7 = assign34720_e38901_d_n7;
        locals.var_temp1_dn8 = assign34720_e38901_d_n8;
        locals.var_temp1_dn9 = assign34720_e38901_d_n9;

        let (assign34730_e38907, assign34730_e38907_d_n4, assign34730_e38907_d_n6, assign34730_e38907_d_n7, assign34730_e38907_d_n8, assign34730_e38907_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34730_e38905: f64 = (2.0 * locals.var_temp1);
        (assign34730_e38905, (2.0 * locals.var_temp1_dn4), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), (2.0 * locals.var_temp1_dn9),)
    } else {
        (locals.var_esurf1s__blk952, locals.var_esurf1s__blk952_dn4, locals.var_esurf1s__blk952_dn6, locals.var_esurf1s__blk952_dn7, locals.var_esurf1s__blk952_dn8, locals.var_esurf1s__blk952_dn9,)
    }
};
        locals.var_esurf1s__blk952 = assign34730_e38907;
        locals.var_esurf1s__blk952_dn4 = assign34730_e38907_d_n4;
        locals.var_esurf1s__blk952_dn6 = assign34730_e38907_d_n6;
        locals.var_esurf1s__blk952_dn7 = assign34730_e38907_d_n7;
        locals.var_esurf1s__blk952_dn8 = assign34730_e38907_d_n8;
        locals.var_esurf1s__blk952_dn9 = assign34730_e38907_d_n9;

        let assign34740_e38910: f64 = (locals.var_k2q2s__blk940 / 2.0);
        let assign34740_e38912: f64 = if assign34740_e38910 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1153 = assign34740_e38912;

        let (assign34750_e38924, assign34750_e38924_d_n4, assign34750_e38924_d_n6, assign34750_e38924_d_n7, assign34750_e38924_d_n8, assign34750_e38924_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1153 != 0.0)) {
        let assign34750_e38919: f64 = (locals.var_k2q2s__blk940 / 2.0);
        let assign34750_e38920: f64 = (assign34750_e38919).exp();
        let assign34750_e38921: f64 = (1.0 + assign34750_e38920);
        let assign34750_e38922: f64 = (assign34750_e38921).ln();
        (assign34750_e38922, ((assign34750_e38920 * (locals.var_k2q2s__blk940_dn4 / 2.0)) / assign34750_e38921), ((assign34750_e38920 * (locals.var_k2q2s__blk940_dn6 / 2.0)) / assign34750_e38921), ((assign34750_e38920 * (locals.var_k2q2s__blk940_dn7 / 2.0)) / assign34750_e38921), ((assign34750_e38920 * (locals.var_k2q2s__blk940_dn8 / 2.0)) / assign34750_e38921), ((assign34750_e38920 * (locals.var_k2q2s__blk940_dn9 / 2.0)) / assign34750_e38921),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign34750_e38924;
        locals.var_temp2_dn4 = assign34750_e38924_d_n4;
        locals.var_temp2_dn6 = assign34750_e38924_d_n6;
        locals.var_temp2_dn7 = assign34750_e38924_d_n7;
        locals.var_temp2_dn8 = assign34750_e38924_d_n8;
        locals.var_temp2_dn9 = assign34750_e38924_d_n9;

        let (assign34760_e38933, assign34760_e38933_d_n4, assign34760_e38933_d_n6, assign34760_e38933_d_n7, assign34760_e38933_d_n8, assign34760_e38933_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1153 == 0.0)) {
        let assign34760_e38931: f64 = (locals.var_k2q2s__blk940 / 2.0);
        (assign34760_e38931, (locals.var_k2q2s__blk940_dn4 / 2.0), (locals.var_k2q2s__blk940_dn6 / 2.0), (locals.var_k2q2s__blk940_dn7 / 2.0), (locals.var_k2q2s__blk940_dn8 / 2.0), (locals.var_k2q2s__blk940_dn9 / 2.0),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign34760_e38933;
        locals.var_temp2_dn4 = assign34760_e38933_d_n4;
        locals.var_temp2_dn6 = assign34760_e38933_d_n6;
        locals.var_temp2_dn7 = assign34760_e38933_d_n7;
        locals.var_temp2_dn8 = assign34760_e38933_d_n8;
        locals.var_temp2_dn9 = assign34760_e38933_d_n9;

        let (assign34770_e38939, assign34770_e38939_d_n4, assign34770_e38939_d_n6, assign34770_e38939_d_n7, assign34770_e38939_d_n8, assign34770_e38939_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34770_e38937: f64 = (2.0 * locals.var_temp2);
        (assign34770_e38937, (2.0 * locals.var_temp2_dn4), (2.0 * locals.var_temp2_dn6), (2.0 * locals.var_temp2_dn7), (2.0 * locals.var_temp2_dn8), (2.0 * locals.var_temp2_dn9),)
    } else {
        (locals.var_esurf2s__blk953, locals.var_esurf2s__blk953_dn4, locals.var_esurf2s__blk953_dn6, locals.var_esurf2s__blk953_dn7, locals.var_esurf2s__blk953_dn8, locals.var_esurf2s__blk953_dn9,)
    }
};
        locals.var_esurf2s__blk953 = assign34770_e38939;
        locals.var_esurf2s__blk953_dn4 = assign34770_e38939_d_n4;
        locals.var_esurf2s__blk953_dn6 = assign34770_e38939_d_n6;
        locals.var_esurf2s__blk953_dn7 = assign34770_e38939_d_n7;
        locals.var_esurf2s__blk953_dn8 = assign34770_e38939_d_n8;
        locals.var_esurf2s__blk953_dn9 = assign34770_e38939_d_n9;

        let (assign34780_e38945, assign34780_e38945_d_n4, assign34780_e38945_d_n6, assign34780_e38945_d_n7, assign34780_e38945_d_n8, assign34780_e38945_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34780_e38943: f64 = (locals.var_esurf2s__blk953 - locals.var_k2q2s__blk940);
        (assign34780_e38943, (locals.var_esurf2s__blk953_dn4 - locals.var_k2q2s__blk940_dn4), (locals.var_esurf2s__blk953_dn6 - locals.var_k2q2s__blk940_dn6), (locals.var_esurf2s__blk953_dn7 - locals.var_k2q2s__blk940_dn7), (locals.var_esurf2s__blk953_dn8 - locals.var_k2q2s__blk940_dn8), (locals.var_esurf2s__blk953_dn9 - locals.var_k2q2s__blk940_dn9),)
    } else {
        (locals.var_ecpl1s__blk954, locals.var_ecpl1s__blk954_dn4, locals.var_ecpl1s__blk954_dn6, locals.var_ecpl1s__blk954_dn7, locals.var_ecpl1s__blk954_dn8, locals.var_ecpl1s__blk954_dn9,)
    }
};
        locals.var_ecpl1s__blk954 = assign34780_e38945;
        locals.var_ecpl1s__blk954_dn4 = assign34780_e38945_d_n4;
        locals.var_ecpl1s__blk954_dn6 = assign34780_e38945_d_n6;
        locals.var_ecpl1s__blk954_dn7 = assign34780_e38945_d_n7;
        locals.var_ecpl1s__blk954_dn8 = assign34780_e38945_d_n8;
        locals.var_ecpl1s__blk954_dn9 = assign34780_e38945_d_n9;

        let (assign34790_e38951, assign34790_e38951_d_n4, assign34790_e38951_d_n6, assign34790_e38951_d_n7, assign34790_e38951_d_n8, assign34790_e38951_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34790_e38949: f64 = (locals.var_esurf1s__blk952 - locals.var_k1q1s__blk939);
        (assign34790_e38949, (locals.var_esurf1s__blk952_dn4 - locals.var_k1q1s__blk939_dn4), (locals.var_esurf1s__blk952_dn6 - locals.var_k1q1s__blk939_dn6), (locals.var_esurf1s__blk952_dn7 - locals.var_k1q1s__blk939_dn7), (locals.var_esurf1s__blk952_dn8 - locals.var_k1q1s__blk939_dn8), (locals.var_esurf1s__blk952_dn9 - locals.var_k1q1s__blk939_dn9),)
    } else {
        (locals.var_ecpl2s__blk955, locals.var_ecpl2s__blk955_dn4, locals.var_ecpl2s__blk955_dn6, locals.var_ecpl2s__blk955_dn7, locals.var_ecpl2s__blk955_dn8, locals.var_ecpl2s__blk955_dn9,)
    }
};
        locals.var_ecpl2s__blk955 = assign34790_e38951;
        locals.var_ecpl2s__blk955_dn4 = assign34790_e38951_d_n4;
        locals.var_ecpl2s__blk955_dn6 = assign34790_e38951_d_n6;
        locals.var_ecpl2s__blk955_dn7 = assign34790_e38951_d_n7;
        locals.var_ecpl2s__blk955_dn8 = assign34790_e38951_d_n8;
        locals.var_ecpl2s__blk955_dn9 = assign34790_e38951_d_n9;

        let (assign34800_e38961, assign34800_e38961_d_n4, assign34800_e38961_d_n6, assign34800_e38961_d_n7, assign34800_e38961_d_n8, assign34800_e38961_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34800_e38955: f64 = (locals.var_eta_mu * locals.var_esurf1s__blk952);
        let assign34800_e38958: f64 = (locals.var_one_m_eta * locals.var_ecpl1s__blk954);
        let assign34800_e38959: f64 = (assign34800_e38955 + assign34800_e38958);
        (assign34800_e38959, ((locals.var_eta_mu * locals.var_esurf1s__blk952_dn4) + (locals.var_one_m_eta * locals.var_ecpl1s__blk954_dn4)), ((locals.var_eta_mu * locals.var_esurf1s__blk952_dn6) + (locals.var_one_m_eta * locals.var_ecpl1s__blk954_dn6)), ((locals.var_eta_mu * locals.var_esurf1s__blk952_dn7) + (locals.var_one_m_eta * locals.var_ecpl1s__blk954_dn7)), ((locals.var_eta_mu * locals.var_esurf1s__blk952_dn8) + (locals.var_one_m_eta * locals.var_ecpl1s__blk954_dn8)), ((locals.var_eta_mu * locals.var_esurf1s__blk952_dn9) + (locals.var_one_m_eta * locals.var_ecpl1s__blk954_dn9)),)
    } else {
        (locals.var_eeff1s__blk956, locals.var_eeff1s__blk956_dn4, locals.var_eeff1s__blk956_dn6, locals.var_eeff1s__blk956_dn7, locals.var_eeff1s__blk956_dn8, locals.var_eeff1s__blk956_dn9,)
    }
};
        locals.var_eeff1s__blk956 = assign34800_e38961;
        locals.var_eeff1s__blk956_dn4 = assign34800_e38961_d_n4;
        locals.var_eeff1s__blk956_dn6 = assign34800_e38961_d_n6;
        locals.var_eeff1s__blk956_dn7 = assign34800_e38961_d_n7;
        locals.var_eeff1s__blk956_dn8 = assign34800_e38961_d_n8;
        locals.var_eeff1s__blk956_dn9 = assign34800_e38961_d_n9;

        let (assign34810_e38971, assign34810_e38971_d_n4, assign34810_e38971_d_n6, assign34810_e38971_d_n7, assign34810_e38971_d_n8, assign34810_e38971_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34810_e38965: f64 = (locals.var_eta_mu * locals.var_esurf2s__blk953);
        let assign34810_e38968: f64 = (locals.var_one_m_eta * locals.var_ecpl2s__blk955);
        let assign34810_e38969: f64 = (assign34810_e38965 + assign34810_e38968);
        (assign34810_e38969, ((locals.var_eta_mu * locals.var_esurf2s__blk953_dn4) + (locals.var_one_m_eta * locals.var_ecpl2s__blk955_dn4)), ((locals.var_eta_mu * locals.var_esurf2s__blk953_dn6) + (locals.var_one_m_eta * locals.var_ecpl2s__blk955_dn6)), ((locals.var_eta_mu * locals.var_esurf2s__blk953_dn7) + (locals.var_one_m_eta * locals.var_ecpl2s__blk955_dn7)), ((locals.var_eta_mu * locals.var_esurf2s__blk953_dn8) + (locals.var_one_m_eta * locals.var_ecpl2s__blk955_dn8)), ((locals.var_eta_mu * locals.var_esurf2s__blk953_dn9) + (locals.var_one_m_eta * locals.var_ecpl2s__blk955_dn9)),)
    } else {
        (locals.var_eeff2s__blk957, locals.var_eeff2s__blk957_dn4, locals.var_eeff2s__blk957_dn6, locals.var_eeff2s__blk957_dn7, locals.var_eeff2s__blk957_dn8, locals.var_eeff2s__blk957_dn9,)
    }
};
        locals.var_eeff2s__blk957 = assign34810_e38971;
        locals.var_eeff2s__blk957_dn4 = assign34810_e38971_d_n4;
        locals.var_eeff2s__blk957_dn6 = assign34810_e38971_d_n6;
        locals.var_eeff2s__blk957_dn7 = assign34810_e38971_d_n7;
        locals.var_eeff2s__blk957_dn8 = assign34810_e38971_d_n8;
        locals.var_eeff2s__blk957_dn9 = assign34810_e38971_d_n9;

        let (assign34820_e38979, assign34820_e38979_d_n4, assign34820_e38979_d_n6, assign34820_e38979_d_n7, assign34820_e38979_d_n8, assign34820_e38979_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34820_e38976: f64 = (locals.var_esurf1s__blk952 + locals.var_esurf2s__blk953);
        let assign34820_e38977: f64 = (locals.var_qis__blk938 / assign34820_e38976);
        (assign34820_e38977, (((locals.var_qis__blk938_dn4 * assign34820_e38976) - (locals.var_qis__blk938 * (locals.var_esurf1s__blk952_dn4 + locals.var_esurf2s__blk953_dn4))) / (assign34820_e38976 * assign34820_e38976)), (((locals.var_qis__blk938_dn6 * assign34820_e38976) - (locals.var_qis__blk938 * (locals.var_esurf1s__blk952_dn6 + locals.var_esurf2s__blk953_dn6))) / (assign34820_e38976 * assign34820_e38976)), (((locals.var_qis__blk938_dn7 * assign34820_e38976) - (locals.var_qis__blk938 * (locals.var_esurf1s__blk952_dn7 + locals.var_esurf2s__blk953_dn7))) / (assign34820_e38976 * assign34820_e38976)), (((locals.var_qis__blk938_dn8 * assign34820_e38976) - (locals.var_qis__blk938 * (locals.var_esurf1s__blk952_dn8 + locals.var_esurf2s__blk953_dn8))) / (assign34820_e38976 * assign34820_e38976)), (((locals.var_qis__blk938_dn9 * assign34820_e38976) - (locals.var_qis__blk938 * (locals.var_esurf1s__blk952_dn9 + locals.var_esurf2s__blk953_dn9))) / (assign34820_e38976 * assign34820_e38976)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign34820_e38979;
        locals.var_temp_dn4 = assign34820_e38979_d_n4;
        locals.var_temp_dn6 = assign34820_e38979_d_n6;
        locals.var_temp_dn7 = assign34820_e38979_d_n7;
        locals.var_temp_dn8 = assign34820_e38979_d_n8;
        locals.var_temp_dn9 = assign34820_e38979_d_n9;

        let (assign34830_e38985, assign34830_e38985_d_n4, assign34830_e38985_d_n6, assign34830_e38985_d_n7, assign34830_e38985_d_n8, assign34830_e38985_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34830_e38983: f64 = (locals.var_esurf1s__blk952 * locals.var_temp);
        (assign34830_e38983, ((locals.var_esurf1s__blk952_dn4 * locals.var_temp) + (locals.var_esurf1s__blk952 * locals.var_temp_dn4)), ((locals.var_esurf1s__blk952_dn6 * locals.var_temp) + (locals.var_esurf1s__blk952 * locals.var_temp_dn6)), ((locals.var_esurf1s__blk952_dn7 * locals.var_temp) + (locals.var_esurf1s__blk952 * locals.var_temp_dn7)), ((locals.var_esurf1s__blk952_dn8 * locals.var_temp) + (locals.var_esurf1s__blk952 * locals.var_temp_dn8)), ((locals.var_esurf1s__blk952_dn9 * locals.var_temp) + (locals.var_esurf1s__blk952 * locals.var_temp_dn9)),)
    } else {
        (locals.var_qi1s__blk958, locals.var_qi1s__blk958_dn4, locals.var_qi1s__blk958_dn6, locals.var_qi1s__blk958_dn7, locals.var_qi1s__blk958_dn8, locals.var_qi1s__blk958_dn9,)
    }
};
        locals.var_qi1s__blk958 = assign34830_e38985;
        locals.var_qi1s__blk958_dn4 = assign34830_e38985_d_n4;
        locals.var_qi1s__blk958_dn6 = assign34830_e38985_d_n6;
        locals.var_qi1s__blk958_dn7 = assign34830_e38985_d_n7;
        locals.var_qi1s__blk958_dn8 = assign34830_e38985_d_n8;
        locals.var_qi1s__blk958_dn9 = assign34830_e38985_d_n9;

        let (assign34840_e38991, assign34840_e38991_d_n4, assign34840_e38991_d_n6, assign34840_e38991_d_n7, assign34840_e38991_d_n8, assign34840_e38991_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34840_e38989: f64 = (locals.var_esurf2s__blk953 * locals.var_temp);
        (assign34840_e38989, ((locals.var_esurf2s__blk953_dn4 * locals.var_temp) + (locals.var_esurf2s__blk953 * locals.var_temp_dn4)), ((locals.var_esurf2s__blk953_dn6 * locals.var_temp) + (locals.var_esurf2s__blk953 * locals.var_temp_dn6)), ((locals.var_esurf2s__blk953_dn7 * locals.var_temp) + (locals.var_esurf2s__blk953 * locals.var_temp_dn7)), ((locals.var_esurf2s__blk953_dn8 * locals.var_temp) + (locals.var_esurf2s__blk953 * locals.var_temp_dn8)), ((locals.var_esurf2s__blk953_dn9 * locals.var_temp) + (locals.var_esurf2s__blk953 * locals.var_temp_dn9)),)
    } else {
        (locals.var_qi2s__blk959, locals.var_qi2s__blk959_dn4, locals.var_qi2s__blk959_dn6, locals.var_qi2s__blk959_dn7, locals.var_qi2s__blk959_dn8, locals.var_qi2s__blk959_dn9,)
    }
};
        locals.var_qi2s__blk959 = assign34840_e38991;
        locals.var_qi2s__blk959_dn4 = assign34840_e38991_d_n4;
        locals.var_qi2s__blk959_dn6 = assign34840_e38991_d_n6;
        locals.var_qi2s__blk959_dn7 = assign34840_e38991_d_n7;
        locals.var_qi2s__blk959_dn8 = assign34840_e38991_d_n8;
        locals.var_qi2s__blk959_dn9 = assign34840_e38991_d_n9;

        let (assign34850_e39002, assign34850_e39002_d_n4, assign34850_e39002_d_n6, assign34850_e39002_d_n7, assign34850_e39002_d_n8, assign34850_e39002_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34850_e38995: f64 = (locals.var_esurf1s__blk952 * locals.var_betn1_t);
        let assign34850_e38998: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign34850_e38999: f64 = (assign34850_e38998).exp();
        let assign34850_e39000: f64 = (assign34850_e38995 * assign34850_e38999);
        (assign34850_e39000, ((((locals.var_esurf1s__blk952_dn4 * locals.var_betn1_t) + (locals.var_esurf1s__blk952 * locals.var_betn1_t_dn4)) * assign34850_e38999) + (assign34850_e38995 * (assign34850_e38999 * (locals.var_stbet_i * locals.var_lnrtn_dn4)))), ((((locals.var_esurf1s__blk952_dn6 * locals.var_betn1_t) + (locals.var_esurf1s__blk952 * locals.var_betn1_t_dn6)) * assign34850_e38999) + (assign34850_e38995 * (assign34850_e38999 * (locals.var_stbet_i * locals.var_lnrtn_dn6)))), ((((locals.var_esurf1s__blk952_dn7 * locals.var_betn1_t) + (locals.var_esurf1s__blk952 * locals.var_betn1_t_dn7)) * assign34850_e38999) + (assign34850_e38995 * (assign34850_e38999 * (locals.var_stbet_i * locals.var_lnrtn_dn7)))), ((((locals.var_esurf1s__blk952_dn8 * locals.var_betn1_t) + (locals.var_esurf1s__blk952 * locals.var_betn1_t_dn8)) * assign34850_e38999) + (assign34850_e38995 * (assign34850_e38999 * (locals.var_stbet_i * locals.var_lnrtn_dn8)))), ((((locals.var_esurf1s__blk952_dn9 * locals.var_betn1_t) + (locals.var_esurf1s__blk952 * locals.var_betn1_t_dn9)) * assign34850_e38999) + (assign34850_e38995 * (assign34850_e38999 * (locals.var_stbet_i * locals.var_lnrtn_dn9)))),)
    } else {
        (locals.var_c1s__blk960, locals.var_c1s__blk960_dn4, locals.var_c1s__blk960_dn6, locals.var_c1s__blk960_dn7, locals.var_c1s__blk960_dn8, locals.var_c1s__blk960_dn9,)
    }
};
        locals.var_c1s__blk960 = assign34850_e39002;
        locals.var_c1s__blk960_dn4 = assign34850_e39002_d_n4;
        locals.var_c1s__blk960_dn6 = assign34850_e39002_d_n6;
        locals.var_c1s__blk960_dn7 = assign34850_e39002_d_n7;
        locals.var_c1s__blk960_dn8 = assign34850_e39002_d_n8;
        locals.var_c1s__blk960_dn9 = assign34850_e39002_d_n9;

        let (assign34860_e39013, assign34860_e39013_d_n4, assign34860_e39013_d_n6, assign34860_e39013_d_n7, assign34860_e39013_d_n8, assign34860_e39013_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34860_e39006: f64 = (locals.var_esurf2s__blk953 * locals.var_betn2_t);
        let assign34860_e39009: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign34860_e39010: f64 = (assign34860_e39009).exp();
        let assign34860_e39011: f64 = (assign34860_e39006 * assign34860_e39010);
        (assign34860_e39011, ((((locals.var_esurf2s__blk953_dn4 * locals.var_betn2_t) + (locals.var_esurf2s__blk953 * locals.var_betn2_t_dn4)) * assign34860_e39010) + (assign34860_e39006 * (assign34860_e39010 * (locals.var_stbet_i * locals.var_lnrtn_dn4)))), ((((locals.var_esurf2s__blk953_dn6 * locals.var_betn2_t) + (locals.var_esurf2s__blk953 * locals.var_betn2_t_dn6)) * assign34860_e39010) + (assign34860_e39006 * (assign34860_e39010 * (locals.var_stbet_i * locals.var_lnrtn_dn6)))), ((((locals.var_esurf2s__blk953_dn7 * locals.var_betn2_t) + (locals.var_esurf2s__blk953 * locals.var_betn2_t_dn7)) * assign34860_e39010) + (assign34860_e39006 * (assign34860_e39010 * (locals.var_stbet_i * locals.var_lnrtn_dn7)))), ((((locals.var_esurf2s__blk953_dn8 * locals.var_betn2_t) + (locals.var_esurf2s__blk953 * locals.var_betn2_t_dn8)) * assign34860_e39010) + (assign34860_e39006 * (assign34860_e39010 * (locals.var_stbet_i * locals.var_lnrtn_dn8)))), ((((locals.var_esurf2s__blk953_dn9 * locals.var_betn2_t) + (locals.var_esurf2s__blk953 * locals.var_betn2_t_dn9)) * assign34860_e39010) + (assign34860_e39006 * (assign34860_e39010 * (locals.var_stbet_i * locals.var_lnrtn_dn9)))),)
    } else {
        (locals.var_c2s__blk961, locals.var_c2s__blk961_dn4, locals.var_c2s__blk961_dn6, locals.var_c2s__blk961_dn7, locals.var_c2s__blk961_dn8, locals.var_c2s__blk961_dn9,)
    }
};
        locals.var_c2s__blk961 = assign34860_e39013;
        locals.var_c2s__blk961_dn4 = assign34860_e39013_d_n4;
        locals.var_c2s__blk961_dn6 = assign34860_e39013_d_n6;
        locals.var_c2s__blk961_dn7 = assign34860_e39013_d_n7;
        locals.var_c2s__blk961_dn8 = assign34860_e39013_d_n8;
        locals.var_c2s__blk961_dn9 = assign34860_e39013_d_n9;

        let (assign34870_e39023, assign34870_e39023_d_n4, assign34870_e39023_d_n6, assign34870_e39023_d_n7, assign34870_e39023_d_n8, assign34870_e39023_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34870_e39019: f64 = (locals.var_xcorb_i * locals.var_ecpl2s__blk955);
        let assign34870_e39020: f64 = (locals.var_ecpl1s__blk954 + assign34870_e39019);
        let assign34870_e39021: f64 = (locals.var_xcor_i * assign34870_e39020);
        (assign34870_e39021, ((locals.var_xcor_i_dn4 * assign34870_e39020) + (locals.var_xcor_i * (locals.var_ecpl1s__blk954_dn4 + (locals.var_xcorb_i * locals.var_ecpl2s__blk955_dn4)))), ((locals.var_xcor_i_dn6 * assign34870_e39020) + (locals.var_xcor_i * (locals.var_ecpl1s__blk954_dn6 + (locals.var_xcorb_i * locals.var_ecpl2s__blk955_dn6)))), ((locals.var_xcor_i_dn7 * assign34870_e39020) + (locals.var_xcor_i * (locals.var_ecpl1s__blk954_dn7 + (locals.var_xcorb_i * locals.var_ecpl2s__blk955_dn7)))), ((locals.var_xcor_i_dn8 * assign34870_e39020) + (locals.var_xcor_i * (locals.var_ecpl1s__blk954_dn8 + (locals.var_xcorb_i * locals.var_ecpl2s__blk955_dn8)))), ((locals.var_xcor_i_dn9 * assign34870_e39020) + (locals.var_xcor_i * (locals.var_ecpl1s__blk954_dn9 + (locals.var_xcorb_i * locals.var_ecpl2s__blk955_dn9)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign34870_e39023;
        locals.var_temp1_dn4 = assign34870_e39023_d_n4;
        locals.var_temp1_dn6 = assign34870_e39023_d_n6;
        locals.var_temp1_dn7 = assign34870_e39023_d_n7;
        locals.var_temp1_dn8 = assign34870_e39023_d_n8;
        locals.var_temp1_dn9 = assign34870_e39023_d_n9;

        let (assign34880_e39048, assign34880_e39048_d_n4, assign34880_e39048_d_n6, assign34880_e39048_d_n7, assign34880_e39048_d_n8, assign34880_e39048_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34880_e39028: f64 = (1.0 + locals.var_temp1);
        let assign34880_e39030: f64 = assign34880_e39028;
        let assign34880_e39033: f64 = (1.0 + locals.var_temp1);
        let assign34880_e39035: f64 = assign34880_e39033;
        let assign34880_e39038: f64 = (1.0 + locals.var_temp1);
        let assign34880_e39040: f64 = assign34880_e39038;
        let assign34880_e39041: f64 = (assign34880_e39035 * assign34880_e39040);
        let assign34880_e39043: f64 = (assign34880_e39041 + 0.01);
        let assign34880_e39044: f64 = (assign34880_e39043).sqrt();
        let assign34880_e39045: f64 = (assign34880_e39030 + assign34880_e39044);
        let assign34880_e39046: f64 = (0.5 * assign34880_e39045);
        (assign34880_e39046, (0.5 * (locals.var_temp1_dn4 + (((locals.var_temp1_dn4 * assign34880_e39040) + (assign34880_e39035 * locals.var_temp1_dn4)) / (2.0 * assign34880_e39044)))), (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign34880_e39040) + (assign34880_e39035 * locals.var_temp1_dn6)) / (2.0 * assign34880_e39044)))), (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign34880_e39040) + (assign34880_e39035 * locals.var_temp1_dn7)) / (2.0 * assign34880_e39044)))), (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign34880_e39040) + (assign34880_e39035 * locals.var_temp1_dn8)) / (2.0 * assign34880_e39044)))), (0.5 * (locals.var_temp1_dn9 + (((locals.var_temp1_dn9 * assign34880_e39040) + (assign34880_e39035 * locals.var_temp1_dn9)) / (2.0 * assign34880_e39044)))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign34880_e39048;
        locals.var_temp2_dn4 = assign34880_e39048_d_n4;
        locals.var_temp2_dn6 = assign34880_e39048_d_n6;
        locals.var_temp2_dn7 = assign34880_e39048_d_n7;
        locals.var_temp2_dn8 = assign34880_e39048_d_n8;
        locals.var_temp2_dn9 = assign34880_e39048_d_n9;

        let (assign34890_e39079, assign34890_e39079_d_n4, assign34890_e39079_d_n6, assign34890_e39079_d_n7, assign34890_e39079_d_n8, assign34890_e39079_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34890_e39054: f64 = (0.2 * locals.var_temp1);
        let assign34890_e39055: f64 = (1.0 + assign34890_e39054);
        let assign34890_e39057: f64 = assign34890_e39055;
        let assign34890_e39061: f64 = (0.2 * locals.var_temp1);
        let assign34890_e39062: f64 = (1.0 + assign34890_e39061);
        let assign34890_e39064: f64 = assign34890_e39062;
        let assign34890_e39068: f64 = (0.2 * locals.var_temp1);
        let assign34890_e39069: f64 = (1.0 + assign34890_e39068);
        let assign34890_e39071: f64 = assign34890_e39069;
        let assign34890_e39072: f64 = (assign34890_e39064 * assign34890_e39071);
        let assign34890_e39074: f64 = (assign34890_e39072 + 0.01);
        let assign34890_e39075: f64 = (assign34890_e39074).sqrt();
        let assign34890_e39076: f64 = (assign34890_e39057 + assign34890_e39075);
        let assign34890_e39077: f64 = (0.5 * assign34890_e39076);
        (assign34890_e39077, (0.5 * ((0.2 * locals.var_temp1_dn4) + ((((0.2 * locals.var_temp1_dn4) * assign34890_e39071) + (assign34890_e39064 * (0.2 * locals.var_temp1_dn4))) / (2.0 * assign34890_e39075)))), (0.5 * ((0.2 * locals.var_temp1_dn6) + ((((0.2 * locals.var_temp1_dn6) * assign34890_e39071) + (assign34890_e39064 * (0.2 * locals.var_temp1_dn6))) / (2.0 * assign34890_e39075)))), (0.5 * ((0.2 * locals.var_temp1_dn7) + ((((0.2 * locals.var_temp1_dn7) * assign34890_e39071) + (assign34890_e39064 * (0.2 * locals.var_temp1_dn7))) / (2.0 * assign34890_e39075)))), (0.5 * ((0.2 * locals.var_temp1_dn8) + ((((0.2 * locals.var_temp1_dn8) * assign34890_e39071) + (assign34890_e39064 * (0.2 * locals.var_temp1_dn8))) / (2.0 * assign34890_e39075)))), (0.5 * ((0.2 * locals.var_temp1_dn9) + ((((0.2 * locals.var_temp1_dn9) * assign34890_e39071) + (assign34890_e39064 * (0.2 * locals.var_temp1_dn9))) / (2.0 * assign34890_e39075)))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign34890_e39079;
        locals.var_temp3_dn4 = assign34890_e39079_d_n4;
        locals.var_temp3_dn6 = assign34890_e39079_d_n6;
        locals.var_temp3_dn7 = assign34890_e39079_d_n7;
        locals.var_temp3_dn8 = assign34890_e39079_d_n8;
        locals.var_temp3_dn9 = assign34890_e39079_d_n9;

        let (assign34900_e39085, assign34900_e39085_d_n4, assign34900_e39085_d_n6, assign34900_e39085_d_n7, assign34900_e39085_d_n8, assign34900_e39085_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34900_e39083: f64 = (locals.var_temp2 / locals.var_temp3);
        (assign34900_e39083, (((locals.var_temp2_dn4 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn4)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn6 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn6)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn7 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn7)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn8 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn8)) / (locals.var_temp3 * locals.var_temp3)), (((locals.var_temp2_dn9 * locals.var_temp3) - (locals.var_temp2 * locals.var_temp3_dn9)) / (locals.var_temp3 * locals.var_temp3)),)
    } else {
        (locals.var_fcors__blk962, locals.var_fcors__blk962_dn4, locals.var_fcors__blk962_dn6, locals.var_fcors__blk962_dn7, locals.var_fcors__blk962_dn8, locals.var_fcors__blk962_dn9,)
    }
};
        locals.var_fcors__blk962 = assign34900_e39085;
        locals.var_fcors__blk962_dn4 = assign34900_e39085_d_n4;
        locals.var_fcors__blk962_dn6 = assign34900_e39085_d_n6;
        locals.var_fcors__blk962_dn7 = assign34900_e39085_d_n7;
        locals.var_fcors__blk962_dn8 = assign34900_e39085_d_n8;
        locals.var_fcors__blk962_dn9 = assign34900_e39085_d_n9;

        let (assign34910_e39114, assign34910_e39114_d_n4, assign34910_e39114_d_n6, assign34910_e39114_d_n7, assign34910_e39114_d_n8, assign34910_e39114_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34910_e39091: f64 = (locals.var_csfi_i * locals.var_ecpl1s__blk954);
        let assign34910_e39092: f64 = (1.0 + assign34910_e39091);
        let assign34910_e39095: f64 = (locals.var_csbi_i * locals.var_ecpl2s__blk955);
        let assign34910_e39096: f64 = (assign34910_e39092 + assign34910_e39095);
        let assign34910_e39097: f64 = (locals.var_cs_i * assign34910_e39096);
        let assign34910_e39099: f64 = (-locals.var_thecs_i);
        let assign34910_e39103: f64 = (locals.var_qi1s__blk958 * locals.var_inv_qi1cs);
        let assign34910_e39104: f64 = (1.0 + assign34910_e39103);
        let assign34910_e39107: f64 = (locals.var_qi2s__blk959 * locals.var_inv_qi2cs);
        let assign34910_e39108: f64 = (assign34910_e39104 + assign34910_e39107);
        let assign34910_e39109: f64 = (assign34910_e39108).ln();
        let assign34910_e39110: f64 = (assign34910_e39099 * assign34910_e39109);
        let assign34910_e39111: f64 = (assign34910_e39110).exp();
        let assign34910_e39112: f64 = (assign34910_e39097 * assign34910_e39111);
        (assign34910_e39112, ((((locals.var_cs_i_dn4 * assign34910_e39096) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1s__blk954_dn4) + (locals.var_csbi_i * locals.var_ecpl2s__blk955_dn4)))) * assign34910_e39111) + (assign34910_e39097 * (assign34910_e39111 * (((-locals.var_thecs_i_dn4) * assign34910_e39109) + (assign34910_e39099 * (((locals.var_qi1s__blk958_dn4 * locals.var_inv_qi1cs) + (locals.var_qi2s__blk959_dn4 * locals.var_inv_qi2cs)) / assign34910_e39108)))))), ((((locals.var_cs_i_dn6 * assign34910_e39096) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1s__blk954_dn6) + (locals.var_csbi_i * locals.var_ecpl2s__blk955_dn6)))) * assign34910_e39111) + (assign34910_e39097 * (assign34910_e39111 * (((-locals.var_thecs_i_dn6) * assign34910_e39109) + (assign34910_e39099 * (((locals.var_qi1s__blk958_dn6 * locals.var_inv_qi1cs) + (locals.var_qi2s__blk959_dn6 * locals.var_inv_qi2cs)) / assign34910_e39108)))))), ((((locals.var_cs_i_dn7 * assign34910_e39096) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1s__blk954_dn7) + (locals.var_csbi_i * locals.var_ecpl2s__blk955_dn7)))) * assign34910_e39111) + (assign34910_e39097 * (assign34910_e39111 * (((-locals.var_thecs_i_dn7) * assign34910_e39109) + (assign34910_e39099 * (((locals.var_qi1s__blk958_dn7 * locals.var_inv_qi1cs) + (locals.var_qi2s__blk959_dn7 * locals.var_inv_qi2cs)) / assign34910_e39108)))))), ((((locals.var_cs_i_dn8 * assign34910_e39096) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1s__blk954_dn8) + (locals.var_csbi_i * locals.var_ecpl2s__blk955_dn8)))) * assign34910_e39111) + (assign34910_e39097 * (assign34910_e39111 * (((-locals.var_thecs_i_dn8) * assign34910_e39109) + (assign34910_e39099 * (((locals.var_qi1s__blk958_dn8 * locals.var_inv_qi1cs) + (locals.var_qi2s__blk959_dn8 * locals.var_inv_qi2cs)) / assign34910_e39108)))))), ((((locals.var_cs_i_dn9 * assign34910_e39096) + (locals.var_cs_i * ((locals.var_csfi_i * locals.var_ecpl1s__blk954_dn9) + (locals.var_csbi_i * locals.var_ecpl2s__blk955_dn9)))) * assign34910_e39111) + (assign34910_e39097 * (assign34910_e39111 * (((-locals.var_thecs_i_dn9) * assign34910_e39109) + (assign34910_e39099 * (((locals.var_qi1s__blk958_dn9 * locals.var_inv_qi1cs) + (locals.var_qi2s__blk959_dn9 * locals.var_inv_qi2cs)) / assign34910_e39108)))))),)
    } else {
        (locals.var_gcss__blk963, locals.var_gcss__blk963_dn4, locals.var_gcss__blk963_dn6, locals.var_gcss__blk963_dn7, locals.var_gcss__blk963_dn8, locals.var_gcss__blk963_dn9,)
    }
};
        locals.var_gcss__blk963 = assign34910_e39114;
        locals.var_gcss__blk963_dn4 = assign34910_e39114_d_n4;
        locals.var_gcss__blk963_dn6 = assign34910_e39114_d_n6;
        locals.var_gcss__blk963_dn7 = assign34910_e39114_d_n7;
        locals.var_gcss__blk963_dn8 = assign34910_e39114_d_n8;
        locals.var_gcss__blk963_dn9 = assign34910_e39114_d_n9;

        let assign34920_e39117: f64 = if locals.var_rsg_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1154 = assign34920_e39117;

        let (assign34930_e39123, assign34930_e39123_d_n4, assign34930_e39123_d_n6, assign34930_e39123_d_n7, assign34930_e39123_d_n8, assign34930_e39123_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1154 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign34930_e39123;
        locals.var_temp3_dn4 = assign34930_e39123_d_n4;
        locals.var_temp3_dn6 = assign34930_e39123_d_n6;
        locals.var_temp3_dn7 = assign34930_e39123_d_n7;
        locals.var_temp3_dn8 = assign34930_e39123_d_n8;
        locals.var_temp3_dn9 = assign34930_e39123_d_n9;

        let assign34940_e39126: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1155 = assign34940_e39126;

        let (assign34950_e39143, assign34950_e39143_d_n4, assign34950_e39143_d_n6, assign34950_e39143_d_n7, assign34950_e39143_d_n8, assign34950_e39143_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1154 == 0.0)) && (locals.var_guard1155 != 0.0)) {
        let assign34950_e39137: f64 = (locals.var_qis__blk938 + 1e-12);
        let assign34950_e39138: f64 = (assign34950_e39137).ln();
        let assign34950_e39139: f64 = (locals.var_thersg_i * assign34950_e39138);
        let assign34950_e39140: f64 = (assign34950_e39139).exp();
        let assign34950_e39141: f64 = (locals.var_rsg_i * assign34950_e39140);
        (assign34950_e39141, (locals.var_rsg_i * (assign34950_e39140 * (locals.var_thersg_i * (locals.var_qis__blk938_dn4 / assign34950_e39137)))), (locals.var_rsg_i * (assign34950_e39140 * (locals.var_thersg_i * (locals.var_qis__blk938_dn6 / assign34950_e39137)))), (locals.var_rsg_i * (assign34950_e39140 * (locals.var_thersg_i * (locals.var_qis__blk938_dn7 / assign34950_e39137)))), (locals.var_rsg_i * (assign34950_e39140 * (locals.var_thersg_i * (locals.var_qis__blk938_dn8 / assign34950_e39137)))), (locals.var_rsg_i * (assign34950_e39140 * (locals.var_thersg_i * (locals.var_qis__blk938_dn9 / assign34950_e39137)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign34950_e39143;
        locals.var_temp1_dn4 = assign34950_e39143_d_n4;
        locals.var_temp1_dn6 = assign34950_e39143_d_n6;
        locals.var_temp1_dn7 = assign34950_e39143_d_n7;
        locals.var_temp1_dn8 = assign34950_e39143_d_n8;
        locals.var_temp1_dn9 = assign34950_e39143_d_n9;

        let (assign34960_e39154, assign34960_e39154_d_n4, assign34960_e39154_d_n6, assign34960_e39154_d_n7, assign34960_e39154_d_n8, assign34960_e39154_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1154 == 0.0)) && (locals.var_guard1155 != 0.0)) {
        let assign34960_e39152: f64 = (1.0 - locals.var_temp1);
        (assign34960_e39152, (-locals.var_temp1_dn4), (-locals.var_temp1_dn6), (-locals.var_temp1_dn7), (-locals.var_temp1_dn8), (-locals.var_temp1_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign34960_e39154;
        locals.var_temp3_dn4 = assign34960_e39154_d_n4;
        locals.var_temp3_dn6 = assign34960_e39154_d_n6;
        locals.var_temp3_dn7 = assign34960_e39154_d_n7;
        locals.var_temp3_dn8 = assign34960_e39154_d_n8;
        locals.var_temp3_dn9 = assign34960_e39154_d_n9;

        let (assign34970_e39172, assign34970_e39172_d_n4, assign34970_e39172_d_n6, assign34970_e39172_d_n7, assign34970_e39172_d_n8, assign34970_e39172_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1154 == 0.0)) && (locals.var_guard1155 == 0.0)) {
        let assign34970_e39166: f64 = (locals.var_qis__blk938 + 1e-12);
        let assign34970_e39167: f64 = (assign34970_e39166).ln();
        let assign34970_e39168: f64 = (locals.var_thersg_i * assign34970_e39167);
        let assign34970_e39169: f64 = (assign34970_e39168).exp();
        let assign34970_e39170: f64 = (locals.var_rsg_i * assign34970_e39169);
        (assign34970_e39170, (locals.var_rsg_i * (assign34970_e39169 * (locals.var_thersg_i * (locals.var_qis__blk938_dn4 / assign34970_e39166)))), (locals.var_rsg_i * (assign34970_e39169 * (locals.var_thersg_i * (locals.var_qis__blk938_dn6 / assign34970_e39166)))), (locals.var_rsg_i * (assign34970_e39169 * (locals.var_thersg_i * (locals.var_qis__blk938_dn7 / assign34970_e39166)))), (locals.var_rsg_i * (assign34970_e39169 * (locals.var_thersg_i * (locals.var_qis__blk938_dn8 / assign34970_e39166)))), (locals.var_rsg_i * (assign34970_e39169 * (locals.var_thersg_i * (locals.var_qis__blk938_dn9 / assign34970_e39166)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign34970_e39172;
        locals.var_temp1_dn4 = assign34970_e39172_d_n4;
        locals.var_temp1_dn6 = assign34970_e39172_d_n6;
        locals.var_temp1_dn7 = assign34970_e39172_d_n7;
        locals.var_temp1_dn8 = assign34970_e39172_d_n8;
        locals.var_temp1_dn9 = assign34970_e39172_d_n9;

        let (assign34980_e39186, assign34980_e39186_d_n4, assign34980_e39186_d_n6, assign34980_e39186_d_n7, assign34980_e39186_d_n8, assign34980_e39186_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1154 == 0.0)) && (locals.var_guard1155 == 0.0)) {
        let assign34980_e39183: f64 = (1.0 + locals.var_temp1);
        let assign34980_e39184: f64 = (1.0 / assign34980_e39183);
        (assign34980_e39184, (-(locals.var_temp1_dn4 / (assign34980_e39183 * assign34980_e39183))), (-(locals.var_temp1_dn6 / (assign34980_e39183 * assign34980_e39183))), (-(locals.var_temp1_dn7 / (assign34980_e39183 * assign34980_e39183))), (-(locals.var_temp1_dn8 / (assign34980_e39183 * assign34980_e39183))), (-(locals.var_temp1_dn9 / (assign34980_e39183 * assign34980_e39183))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign34980_e39186;
        locals.var_temp3_dn4 = assign34980_e39186_d_n4;
        locals.var_temp3_dn6 = assign34980_e39186_d_n6;
        locals.var_temp3_dn7 = assign34980_e39186_d_n7;
        locals.var_temp3_dn8 = assign34980_e39186_d_n8;
        locals.var_temp3_dn9 = assign34980_e39186_d_n9;

    }

    pub(super) fn stamp_transient_block_95(
        locals: &mut StampLocals,
    ) {
        let (assign34990_e39221, assign34990_e39221_d_n4, assign34990_e39221_d_n6, assign34990_e39221_d_n7, assign34990_e39221_d_n8, assign34990_e39221_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign34990_e39190: f64 = (locals.var_frs * locals.var_csiprime__blk919);
        let assign34990_e39192: f64 = (assign34990_e39190 * 0.5);
        let assign34990_e39196: f64 = (locals.var_rsb_i * locals.var_xg20shift__blk900);
        let assign34990_e39197: f64 = (1.0 - assign34990_e39196);
        let assign34990_e39199: f64 = assign34990_e39197;
        let assign34990_e39203: f64 = (locals.var_rsb_i * locals.var_xg20shift__blk900);
        let assign34990_e39204: f64 = (1.0 - assign34990_e39203);
        let assign34990_e39206: f64 = assign34990_e39204;
        let assign34990_e39210: f64 = (locals.var_rsb_i * locals.var_xg20shift__blk900);
        let assign34990_e39211: f64 = (1.0 - assign34990_e39210);
        let assign34990_e39213: f64 = assign34990_e39211;
        let assign34990_e39214: f64 = (assign34990_e39206 * assign34990_e39213);
        let assign34990_e39216: f64 = (assign34990_e39214 + 0.01);
        let assign34990_e39217: f64 = (assign34990_e39216).sqrt();
        let assign34990_e39218: f64 = (assign34990_e39199 + assign34990_e39217);
        let assign34990_e39219: f64 = (assign34990_e39192 * assign34990_e39218);
        (assign34990_e39219, (((((locals.var_frs_dn4 * locals.var_csiprime__blk919) + (locals.var_frs * locals.var_csiprime__blk919_dn4)) * 0.5) * assign34990_e39218) + (assign34990_e39192 * ((-(locals.var_rsb_i * locals.var_xg20shift__blk900_dn4)) + ((((-(locals.var_rsb_i * locals.var_xg20shift__blk900_dn4)) * assign34990_e39213) + (assign34990_e39206 * (-(locals.var_rsb_i * locals.var_xg20shift__blk900_dn4)))) / (2.0 * assign34990_e39217))))), (((((locals.var_frs_dn6 * locals.var_csiprime__blk919) + (locals.var_frs * locals.var_csiprime__blk919_dn6)) * 0.5) * assign34990_e39218) + (assign34990_e39192 * ((-(locals.var_rsb_i * locals.var_xg20shift__blk900_dn6)) + ((((-(locals.var_rsb_i * locals.var_xg20shift__blk900_dn6)) * assign34990_e39213) + (assign34990_e39206 * (-(locals.var_rsb_i * locals.var_xg20shift__blk900_dn6)))) / (2.0 * assign34990_e39217))))), (((((locals.var_frs_dn7 * locals.var_csiprime__blk919) + (locals.var_frs * locals.var_csiprime__blk919_dn7)) * 0.5) * assign34990_e39218) + (assign34990_e39192 * ((-(locals.var_rsb_i * locals.var_xg20shift__blk900_dn7)) + ((((-(locals.var_rsb_i * locals.var_xg20shift__blk900_dn7)) * assign34990_e39213) + (assign34990_e39206 * (-(locals.var_rsb_i * locals.var_xg20shift__blk900_dn7)))) / (2.0 * assign34990_e39217))))), (((((locals.var_frs_dn8 * locals.var_csiprime__blk919) + (locals.var_frs * locals.var_csiprime__blk919_dn8)) * 0.5) * assign34990_e39218) + (assign34990_e39192 * ((-(locals.var_rsb_i * locals.var_xg20shift__blk900_dn8)) + ((((-(locals.var_rsb_i * locals.var_xg20shift__blk900_dn8)) * assign34990_e39213) + (assign34990_e39206 * (-(locals.var_rsb_i * locals.var_xg20shift__blk900_dn8)))) / (2.0 * assign34990_e39217))))), (((((locals.var_frs_dn9 * locals.var_csiprime__blk919) + (locals.var_frs * locals.var_csiprime__blk919_dn9)) * 0.5) * assign34990_e39218) + (assign34990_e39192 * ((-(locals.var_rsb_i * locals.var_xg20shift__blk900_dn9)) + ((((-(locals.var_rsb_i * locals.var_xg20shift__blk900_dn9)) * assign34990_e39213) + (assign34990_e39206 * (-(locals.var_rsb_i * locals.var_xg20shift__blk900_dn9)))) / (2.0 * assign34990_e39217))))),)
    } else {
        (locals.var_frscsi__blk964, locals.var_frscsi__blk964_dn4, locals.var_frscsi__blk964_dn6, locals.var_frscsi__blk964_dn7, locals.var_frscsi__blk964_dn8, locals.var_frscsi__blk964_dn9,)
    }
};
        locals.var_frscsi__blk964 = assign34990_e39221;
        locals.var_frscsi__blk964_dn4 = assign34990_e39221_d_n4;
        locals.var_frscsi__blk964_dn6 = assign34990_e39221_d_n6;
        locals.var_frscsi__blk964_dn7 = assign34990_e39221_d_n7;
        locals.var_frscsi__blk964_dn8 = assign34990_e39221_d_n8;
        locals.var_frscsi__blk964_dn9 = assign34990_e39221_d_n9;

        let (assign35000_e39231, assign35000_e39231_d_n4, assign35000_e39231_d_n6, assign35000_e39231_d_n7, assign35000_e39231_d_n8, assign35000_e39231_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign35000_e39226: f64 = (locals.var_qis__blk938 * locals.var_temp3);
        let assign35000_e39228: f64 = (assign35000_e39226 + locals.var_rsig_i);
        let assign35000_e39229: f64 = (locals.var_frscsi__blk964 * assign35000_e39228);
        (assign35000_e39229, ((locals.var_frscsi__blk964_dn4 * assign35000_e39228) + (locals.var_frscsi__blk964 * ((locals.var_qis__blk938_dn4 * locals.var_temp3) + (locals.var_qis__blk938 * locals.var_temp3_dn4)))), ((locals.var_frscsi__blk964_dn6 * assign35000_e39228) + (locals.var_frscsi__blk964 * ((locals.var_qis__blk938_dn6 * locals.var_temp3) + (locals.var_qis__blk938 * locals.var_temp3_dn6)))), ((locals.var_frscsi__blk964_dn7 * assign35000_e39228) + (locals.var_frscsi__blk964 * ((locals.var_qis__blk938_dn7 * locals.var_temp3) + (locals.var_qis__blk938 * locals.var_temp3_dn7)))), ((locals.var_frscsi__blk964_dn8 * assign35000_e39228) + (locals.var_frscsi__blk964 * ((locals.var_qis__blk938_dn8 * locals.var_temp3) + (locals.var_qis__blk938 * locals.var_temp3_dn8)))), ((locals.var_frscsi__blk964_dn9 * assign35000_e39228) + (locals.var_frscsi__blk964 * ((locals.var_qis__blk938_dn9 * locals.var_temp3) + (locals.var_qis__blk938 * locals.var_temp3_dn9)))),)
    } else {
        (locals.var_grss__blk965, locals.var_grss__blk965_dn4, locals.var_grss__blk965_dn6, locals.var_grss__blk965_dn7, locals.var_grss__blk965_dn8, locals.var_grss__blk965_dn9,)
    }
};
        locals.var_grss__blk965 = assign35000_e39231;
        locals.var_grss__blk965_dn4 = assign35000_e39231_d_n4;
        locals.var_grss__blk965_dn6 = assign35000_e39231_d_n6;
        locals.var_grss__blk965_dn7 = assign35000_e39231_d_n7;
        locals.var_grss__blk965_dn8 = assign35000_e39231_d_n8;
        locals.var_grss__blk965_dn9 = assign35000_e39231_d_n9;

        let (assign35010_e39251, assign35010_e39251_d_n4, assign35010_e39251_d_n6, assign35010_e39251_d_n7, assign35010_e39251_d_n8, assign35010_e39251_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign35010_e39237: f64 = (locals.var_fmue * locals.var_eeff1s__blk956);
        let assign35010_e39239: f64 = (assign35010_e39237 + 1e-6);
        let assign35010_e39240: f64 = (assign35010_e39239).ln();
        let assign35010_e39241: f64 = (locals.var_themu_i * assign35010_e39240);
        let assign35010_e39242: f64 = (assign35010_e39241).exp();
        let assign35010_e39243: f64 = (1.0 + assign35010_e39242);
        let assign35010_e39245: f64 = (assign35010_e39243 + locals.var_gcss__blk963);
        let assign35010_e39248: f64 = (locals.var_betn1_i * locals.var_grss__blk965);
        let assign35010_e39249: f64 = (assign35010_e39245 + assign35010_e39248);
        (assign35010_e39249, (((assign35010_e39242 * ((locals.var_themu_i_dn4 * assign35010_e39240) + (locals.var_themu_i * (((locals.var_fmue_dn4 * locals.var_eeff1s__blk956) + (locals.var_fmue * locals.var_eeff1s__blk956_dn4)) / assign35010_e39239)))) + locals.var_gcss__blk963_dn4) + ((locals.var_betn1_i_dn4 * locals.var_grss__blk965) + (locals.var_betn1_i * locals.var_grss__blk965_dn4))), (((assign35010_e39242 * ((locals.var_themu_i_dn6 * assign35010_e39240) + (locals.var_themu_i * (((locals.var_fmue_dn6 * locals.var_eeff1s__blk956) + (locals.var_fmue * locals.var_eeff1s__blk956_dn6)) / assign35010_e39239)))) + locals.var_gcss__blk963_dn6) + ((locals.var_betn1_i_dn6 * locals.var_grss__blk965) + (locals.var_betn1_i * locals.var_grss__blk965_dn6))), (((assign35010_e39242 * ((locals.var_themu_i_dn7 * assign35010_e39240) + (locals.var_themu_i * (((locals.var_fmue_dn7 * locals.var_eeff1s__blk956) + (locals.var_fmue * locals.var_eeff1s__blk956_dn7)) / assign35010_e39239)))) + locals.var_gcss__blk963_dn7) + ((locals.var_betn1_i_dn7 * locals.var_grss__blk965) + (locals.var_betn1_i * locals.var_grss__blk965_dn7))), (((assign35010_e39242 * ((locals.var_themu_i_dn8 * assign35010_e39240) + (locals.var_themu_i * (((locals.var_fmue_dn8 * locals.var_eeff1s__blk956) + (locals.var_fmue * locals.var_eeff1s__blk956_dn8)) / assign35010_e39239)))) + locals.var_gcss__blk963_dn8) + ((locals.var_betn1_i_dn8 * locals.var_grss__blk965) + (locals.var_betn1_i * locals.var_grss__blk965_dn8))), (((assign35010_e39242 * ((locals.var_themu_i_dn9 * assign35010_e39240) + (locals.var_themu_i * (((locals.var_fmue_dn9 * locals.var_eeff1s__blk956) + (locals.var_fmue * locals.var_eeff1s__blk956_dn9)) / assign35010_e39239)))) + locals.var_gcss__blk963_dn9) + ((locals.var_betn1_i_dn9 * locals.var_grss__blk965) + (locals.var_betn1_i * locals.var_grss__blk965_dn9))),)
    } else {
        (locals.var_gmob1s__blk966, locals.var_gmob1s__blk966_dn4, locals.var_gmob1s__blk966_dn6, locals.var_gmob1s__blk966_dn7, locals.var_gmob1s__blk966_dn8, locals.var_gmob1s__blk966_dn9,)
    }
};
        locals.var_gmob1s__blk966 = assign35010_e39251;
        locals.var_gmob1s__blk966_dn4 = assign35010_e39251_d_n4;
        locals.var_gmob1s__blk966_dn6 = assign35010_e39251_d_n6;
        locals.var_gmob1s__blk966_dn7 = assign35010_e39251_d_n7;
        locals.var_gmob1s__blk966_dn8 = assign35010_e39251_d_n8;
        locals.var_gmob1s__blk966_dn9 = assign35010_e39251_d_n9;

        let (assign35020_e39271, assign35020_e39271_d_n4, assign35020_e39271_d_n6, assign35020_e39271_d_n7, assign35020_e39271_d_n8, assign35020_e39271_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign35020_e39257: f64 = (locals.var_fmue * locals.var_eeff2s__blk957);
        let assign35020_e39259: f64 = (assign35020_e39257 + 1e-6);
        let assign35020_e39260: f64 = (assign35020_e39259).ln();
        let assign35020_e39261: f64 = (locals.var_themu_i * assign35020_e39260);
        let assign35020_e39262: f64 = (assign35020_e39261).exp();
        let assign35020_e39263: f64 = (1.0 + assign35020_e39262);
        let assign35020_e39265: f64 = (assign35020_e39263 + locals.var_gcss__blk963);
        let assign35020_e39268: f64 = (locals.var_betn2_i * locals.var_grss__blk965);
        let assign35020_e39269: f64 = (assign35020_e39265 + assign35020_e39268);
        (assign35020_e39269, (((assign35020_e39262 * ((locals.var_themu_i_dn4 * assign35020_e39260) + (locals.var_themu_i * (((locals.var_fmue_dn4 * locals.var_eeff2s__blk957) + (locals.var_fmue * locals.var_eeff2s__blk957_dn4)) / assign35020_e39259)))) + locals.var_gcss__blk963_dn4) + ((locals.var_betn2_i_dn4 * locals.var_grss__blk965) + (locals.var_betn2_i * locals.var_grss__blk965_dn4))), (((assign35020_e39262 * ((locals.var_themu_i_dn6 * assign35020_e39260) + (locals.var_themu_i * (((locals.var_fmue_dn6 * locals.var_eeff2s__blk957) + (locals.var_fmue * locals.var_eeff2s__blk957_dn6)) / assign35020_e39259)))) + locals.var_gcss__blk963_dn6) + ((locals.var_betn2_i_dn6 * locals.var_grss__blk965) + (locals.var_betn2_i * locals.var_grss__blk965_dn6))), (((assign35020_e39262 * ((locals.var_themu_i_dn7 * assign35020_e39260) + (locals.var_themu_i * (((locals.var_fmue_dn7 * locals.var_eeff2s__blk957) + (locals.var_fmue * locals.var_eeff2s__blk957_dn7)) / assign35020_e39259)))) + locals.var_gcss__blk963_dn7) + ((locals.var_betn2_i_dn7 * locals.var_grss__blk965) + (locals.var_betn2_i * locals.var_grss__blk965_dn7))), (((assign35020_e39262 * ((locals.var_themu_i_dn8 * assign35020_e39260) + (locals.var_themu_i * (((locals.var_fmue_dn8 * locals.var_eeff2s__blk957) + (locals.var_fmue * locals.var_eeff2s__blk957_dn8)) / assign35020_e39259)))) + locals.var_gcss__blk963_dn8) + ((locals.var_betn2_i_dn8 * locals.var_grss__blk965) + (locals.var_betn2_i * locals.var_grss__blk965_dn8))), (((assign35020_e39262 * ((locals.var_themu_i_dn9 * assign35020_e39260) + (locals.var_themu_i * (((locals.var_fmue_dn9 * locals.var_eeff2s__blk957) + (locals.var_fmue * locals.var_eeff2s__blk957_dn9)) / assign35020_e39259)))) + locals.var_gcss__blk963_dn9) + ((locals.var_betn2_i_dn9 * locals.var_grss__blk965) + (locals.var_betn2_i * locals.var_grss__blk965_dn9))),)
    } else {
        (locals.var_gmob2s__blk967, locals.var_gmob2s__blk967_dn4, locals.var_gmob2s__blk967_dn6, locals.var_gmob2s__blk967_dn7, locals.var_gmob2s__blk967_dn8, locals.var_gmob2s__blk967_dn9,)
    }
};
        locals.var_gmob2s__blk967 = assign35020_e39271;
        locals.var_gmob2s__blk967_dn4 = assign35020_e39271_d_n4;
        locals.var_gmob2s__blk967_dn6 = assign35020_e39271_d_n6;
        locals.var_gmob2s__blk967_dn7 = assign35020_e39271_d_n7;
        locals.var_gmob2s__blk967_dn8 = assign35020_e39271_d_n8;
        locals.var_gmob2s__blk967_dn9 = assign35020_e39271_d_n9;

        let (assign35030_e39287, assign35030_e39287_d_n4, assign35030_e39287_d_n6, assign35030_e39287_d_n7, assign35030_e39287_d_n8, assign35030_e39287_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign35030_e39276: f64 = (locals.var_c1s__blk960 + locals.var_c2s__blk961);
        let assign35030_e39277: f64 = (locals.var_fcors__blk962 * assign35030_e39276);
        let assign35030_e39280: f64 = (locals.var_c1s__blk960 / locals.var_gmob1s__blk966);
        let assign35030_e39283: f64 = (locals.var_c2s__blk961 / locals.var_gmob2s__blk967);
        let assign35030_e39284: f64 = (assign35030_e39280 + assign35030_e39283);
        let assign35030_e39285: f64 = (assign35030_e39277 / assign35030_e39284);
        (assign35030_e39285, (((((locals.var_fcors__blk962_dn4 * assign35030_e39276) + (locals.var_fcors__blk962 * (locals.var_c1s__blk960_dn4 + locals.var_c2s__blk961_dn4))) * assign35030_e39284) - (assign35030_e39277 * ((((locals.var_c1s__blk960_dn4 * locals.var_gmob1s__blk966) - (locals.var_c1s__blk960 * locals.var_gmob1s__blk966_dn4)) / (locals.var_gmob1s__blk966 * locals.var_gmob1s__blk966)) + (((locals.var_c2s__blk961_dn4 * locals.var_gmob2s__blk967) - (locals.var_c2s__blk961 * locals.var_gmob2s__blk967_dn4)) / (locals.var_gmob2s__blk967 * locals.var_gmob2s__blk967))))) / (assign35030_e39284 * assign35030_e39284)), (((((locals.var_fcors__blk962_dn6 * assign35030_e39276) + (locals.var_fcors__blk962 * (locals.var_c1s__blk960_dn6 + locals.var_c2s__blk961_dn6))) * assign35030_e39284) - (assign35030_e39277 * ((((locals.var_c1s__blk960_dn6 * locals.var_gmob1s__blk966) - (locals.var_c1s__blk960 * locals.var_gmob1s__blk966_dn6)) / (locals.var_gmob1s__blk966 * locals.var_gmob1s__blk966)) + (((locals.var_c2s__blk961_dn6 * locals.var_gmob2s__blk967) - (locals.var_c2s__blk961 * locals.var_gmob2s__blk967_dn6)) / (locals.var_gmob2s__blk967 * locals.var_gmob2s__blk967))))) / (assign35030_e39284 * assign35030_e39284)), (((((locals.var_fcors__blk962_dn7 * assign35030_e39276) + (locals.var_fcors__blk962 * (locals.var_c1s__blk960_dn7 + locals.var_c2s__blk961_dn7))) * assign35030_e39284) - (assign35030_e39277 * ((((locals.var_c1s__blk960_dn7 * locals.var_gmob1s__blk966) - (locals.var_c1s__blk960 * locals.var_gmob1s__blk966_dn7)) / (locals.var_gmob1s__blk966 * locals.var_gmob1s__blk966)) + (((locals.var_c2s__blk961_dn7 * locals.var_gmob2s__blk967) - (locals.var_c2s__blk961 * locals.var_gmob2s__blk967_dn7)) / (locals.var_gmob2s__blk967 * locals.var_gmob2s__blk967))))) / (assign35030_e39284 * assign35030_e39284)), (((((locals.var_fcors__blk962_dn8 * assign35030_e39276) + (locals.var_fcors__blk962 * (locals.var_c1s__blk960_dn8 + locals.var_c2s__blk961_dn8))) * assign35030_e39284) - (assign35030_e39277 * ((((locals.var_c1s__blk960_dn8 * locals.var_gmob1s__blk966) - (locals.var_c1s__blk960 * locals.var_gmob1s__blk966_dn8)) / (locals.var_gmob1s__blk966 * locals.var_gmob1s__blk966)) + (((locals.var_c2s__blk961_dn8 * locals.var_gmob2s__blk967) - (locals.var_c2s__blk961 * locals.var_gmob2s__blk967_dn8)) / (locals.var_gmob2s__blk967 * locals.var_gmob2s__blk967))))) / (assign35030_e39284 * assign35030_e39284)), (((((locals.var_fcors__blk962_dn9 * assign35030_e39276) + (locals.var_fcors__blk962 * (locals.var_c1s__blk960_dn9 + locals.var_c2s__blk961_dn9))) * assign35030_e39284) - (assign35030_e39277 * ((((locals.var_c1s__blk960_dn9 * locals.var_gmob1s__blk966) - (locals.var_c1s__blk960 * locals.var_gmob1s__blk966_dn9)) / (locals.var_gmob1s__blk966 * locals.var_gmob1s__blk966)) + (((locals.var_c2s__blk961_dn9 * locals.var_gmob2s__blk967) - (locals.var_c2s__blk961 * locals.var_gmob2s__blk967_dn9)) / (locals.var_gmob2s__blk967 * locals.var_gmob2s__blk967))))) / (assign35030_e39284 * assign35030_e39284)),)
    } else {
        (locals.var_gmobs__blk968, locals.var_gmobs__blk968_dn4, locals.var_gmobs__blk968_dn6, locals.var_gmobs__blk968_dn7, locals.var_gmobs__blk968_dn8, locals.var_gmobs__blk968_dn9,)
    }
};
        locals.var_gmobs__blk968 = assign35030_e39287;
        locals.var_gmobs__blk968_dn4 = assign35030_e39287_d_n4;
        locals.var_gmobs__blk968_dn6 = assign35030_e39287_d_n6;
        locals.var_gmobs__blk968_dn7 = assign35030_e39287_d_n7;
        locals.var_gmobs__blk968_dn8 = assign35030_e39287_d_n8;
        locals.var_gmobs__blk968_dn9 = assign35030_e39287_d_n9;

        let assign35040_e39289: f64 = (locals.var_dx_wi__blk935).abs();
        let assign35040_e39291: f64 = if assign35040_e39289 > 0.007 { 1.0 } else { 0.0 };
        locals.var_guard1156 = assign35040_e39291;

        let assign35050_e39294: f64 = if locals.var_dx_wi__blk935 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1157 = assign35050_e39294;

        let (assign35060_e39304, assign35060_e39304_d_n4, assign35060_e39304_d_n6, assign35060_e39304_d_n7, assign35060_e39304_d_n8, assign35060_e39304_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1156 != 0.0)) && (locals.var_guard1157 != 0.0)) {
        let assign35060_e39301: f64 = (-locals.var_dx_wi__blk935);
        let assign35060_e39302: f64 = (assign35060_e39301).exp();
        (assign35060_e39302, (assign35060_e39302 * (-locals.var_dx_wi__blk935_dn4)), (assign35060_e39302 * (-locals.var_dx_wi__blk935_dn6)), (assign35060_e39302 * (-locals.var_dx_wi__blk935_dn7)), (assign35060_e39302 * (-locals.var_dx_wi__blk935_dn8)), (assign35060_e39302 * (-locals.var_dx_wi__blk935_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign35060_e39304;
        locals.var_temp_dn4 = assign35060_e39304_d_n4;
        locals.var_temp_dn6 = assign35060_e39304_d_n6;
        locals.var_temp_dn7 = assign35060_e39304_d_n7;
        locals.var_temp_dn8 = assign35060_e39304_d_n8;
        locals.var_temp_dn9 = assign35060_e39304_d_n9;

        let (assign35070_e39316, assign35070_e39316_d_n4, assign35070_e39316_d_n6, assign35070_e39316_d_n7, assign35070_e39316_d_n8, assign35070_e39316_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1156 != 0.0)) && (locals.var_guard1157 != 0.0)) {
        let assign35070_e39313: f64 = (1.0 - locals.var_temp);
        let assign35070_e39314: f64 = (locals.var_dx_wi__blk935 / assign35070_e39313);
        (assign35070_e39314, (((locals.var_dx_wi__blk935_dn4 * assign35070_e39313) - (locals.var_dx_wi__blk935 * (-locals.var_temp_dn4))) / (assign35070_e39313 * assign35070_e39313)), (((locals.var_dx_wi__blk935_dn6 * assign35070_e39313) - (locals.var_dx_wi__blk935 * (-locals.var_temp_dn6))) / (assign35070_e39313 * assign35070_e39313)), (((locals.var_dx_wi__blk935_dn7 * assign35070_e39313) - (locals.var_dx_wi__blk935 * (-locals.var_temp_dn7))) / (assign35070_e39313 * assign35070_e39313)), (((locals.var_dx_wi__blk935_dn8 * assign35070_e39313) - (locals.var_dx_wi__blk935 * (-locals.var_temp_dn8))) / (assign35070_e39313 * assign35070_e39313)), (((locals.var_dx_wi__blk935_dn9 * assign35070_e39313) - (locals.var_dx_wi__blk935 * (-locals.var_temp_dn9))) / (assign35070_e39313 * assign35070_e39313)),)
    } else {
        (locals.var_s1__blk969, locals.var_s1__blk969_dn4, locals.var_s1__blk969_dn6, locals.var_s1__blk969_dn7, locals.var_s1__blk969_dn8, locals.var_s1__blk969_dn9,)
    }
};
        locals.var_s1__blk969 = assign35070_e39316;
        locals.var_s1__blk969_dn4 = assign35070_e39316_d_n4;
        locals.var_s1__blk969_dn6 = assign35070_e39316_d_n6;
        locals.var_s1__blk969_dn7 = assign35070_e39316_d_n7;
        locals.var_s1__blk969_dn8 = assign35070_e39316_d_n8;
        locals.var_s1__blk969_dn9 = assign35070_e39316_d_n9;

        let (assign35080_e39326, assign35080_e39326_d_n4, assign35080_e39326_d_n6, assign35080_e39326_d_n7, assign35080_e39326_d_n8, assign35080_e39326_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1156 != 0.0)) && (locals.var_guard1157 != 0.0)) {
        let assign35080_e39324: f64 = (locals.var_temp * locals.var_s1__blk969);
        (assign35080_e39324, ((locals.var_temp_dn4 * locals.var_s1__blk969) + (locals.var_temp * locals.var_s1__blk969_dn4)), ((locals.var_temp_dn6 * locals.var_s1__blk969) + (locals.var_temp * locals.var_s1__blk969_dn6)), ((locals.var_temp_dn7 * locals.var_s1__blk969) + (locals.var_temp * locals.var_s1__blk969_dn7)), ((locals.var_temp_dn8 * locals.var_s1__blk969) + (locals.var_temp * locals.var_s1__blk969_dn8)), ((locals.var_temp_dn9 * locals.var_s1__blk969) + (locals.var_temp * locals.var_s1__blk969_dn9)),)
    } else {
        (locals.var_s2__blk970, locals.var_s2__blk970_dn4, locals.var_s2__blk970_dn6, locals.var_s2__blk970_dn7, locals.var_s2__blk970_dn8, locals.var_s2__blk970_dn9,)
    }
};
        locals.var_s2__blk970 = assign35080_e39326;
        locals.var_s2__blk970_dn4 = assign35080_e39326_d_n4;
        locals.var_s2__blk970_dn6 = assign35080_e39326_d_n6;
        locals.var_s2__blk970_dn7 = assign35080_e39326_d_n7;
        locals.var_s2__blk970_dn8 = assign35080_e39326_d_n8;
        locals.var_s2__blk970_dn9 = assign35080_e39326_d_n9;

        let (assign35090_e39343, assign35090_e39343_d_n4, assign35090_e39343_d_n6, assign35090_e39343_d_n7, assign35090_e39343_d_n8, assign35090_e39343_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1156 != 0.0)) && (locals.var_guard1157 != 0.0)) {
        let assign35090_e39335: f64 = (locals.var_qis__blk938 * locals.var_s1__blk969);
        let assign35090_e39336: f64 = (locals.var_a0__blk905 / assign35090_e39335);
        let assign35090_e39337: f64 = (assign35090_e39336).ln();
        let assign35090_e39339: f64 = (assign35090_e39337 - 0.6931471805599);
        let assign35090_e39341: f64 = (assign35090_e39339 + locals.var_x1_wi0__blk908);
        (assign35090_e39341, (((((locals.var_a0__blk905_dn4 * assign35090_e39335) - (locals.var_a0__blk905 * ((locals.var_qis__blk938_dn4 * locals.var_s1__blk969) + (locals.var_qis__blk938 * locals.var_s1__blk969_dn4)))) / (assign35090_e39335 * assign35090_e39335)) / assign35090_e39336) + locals.var_x1_wi0__blk908_dn4), (((((locals.var_a0__blk905_dn6 * assign35090_e39335) - (locals.var_a0__blk905 * ((locals.var_qis__blk938_dn6 * locals.var_s1__blk969) + (locals.var_qis__blk938 * locals.var_s1__blk969_dn6)))) / (assign35090_e39335 * assign35090_e39335)) / assign35090_e39336) + locals.var_x1_wi0__blk908_dn6), (((((locals.var_a0__blk905_dn7 * assign35090_e39335) - (locals.var_a0__blk905 * ((locals.var_qis__blk938_dn7 * locals.var_s1__blk969) + (locals.var_qis__blk938 * locals.var_s1__blk969_dn7)))) / (assign35090_e39335 * assign35090_e39335)) / assign35090_e39336) + locals.var_x1_wi0__blk908_dn7), (((((locals.var_a0__blk905_dn8 * assign35090_e39335) - (locals.var_a0__blk905 * ((locals.var_qis__blk938_dn8 * locals.var_s1__blk969) + (locals.var_qis__blk938 * locals.var_s1__blk969_dn8)))) / (assign35090_e39335 * assign35090_e39335)) / assign35090_e39336) + locals.var_x1_wi0__blk908_dn8), (((((locals.var_a0__blk905_dn9 * assign35090_e39335) - (locals.var_a0__blk905 * ((locals.var_qis__blk938_dn9 * locals.var_s1__blk969) + (locals.var_qis__blk938 * locals.var_s1__blk969_dn9)))) / (assign35090_e39335 * assign35090_e39335)) / assign35090_e39336) + locals.var_x1_wi0__blk908_dn9),)
    } else {
        (locals.var_deltaxinf__blk971, locals.var_deltaxinf__blk971_dn4, locals.var_deltaxinf__blk971_dn6, locals.var_deltaxinf__blk971_dn7, locals.var_deltaxinf__blk971_dn8, locals.var_deltaxinf__blk971_dn9,)
    }
};
        locals.var_deltaxinf__blk971 = assign35090_e39343;
        locals.var_deltaxinf__blk971_dn4 = assign35090_e39343_d_n4;
        locals.var_deltaxinf__blk971_dn6 = assign35090_e39343_d_n6;
        locals.var_deltaxinf__blk971_dn7 = assign35090_e39343_d_n7;
        locals.var_deltaxinf__blk971_dn8 = assign35090_e39343_d_n8;
        locals.var_deltaxinf__blk971_dn9 = assign35090_e39343_d_n9;

        let (assign35100_e39353, assign35100_e39353_d_n4, assign35100_e39353_d_n6, assign35100_e39353_d_n7, assign35100_e39353_d_n8, assign35100_e39353_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1156 != 0.0)) && (locals.var_guard1157 == 0.0)) {
        let assign35100_e39351: f64 = (locals.var_dx_wi__blk935).exp();
        (assign35100_e39351, (assign35100_e39351 * locals.var_dx_wi__blk935_dn4), (assign35100_e39351 * locals.var_dx_wi__blk935_dn6), (assign35100_e39351 * locals.var_dx_wi__blk935_dn7), (assign35100_e39351 * locals.var_dx_wi__blk935_dn8), (assign35100_e39351 * locals.var_dx_wi__blk935_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign35100_e39353;
        locals.var_temp_dn4 = assign35100_e39353_d_n4;
        locals.var_temp_dn6 = assign35100_e39353_d_n6;
        locals.var_temp_dn7 = assign35100_e39353_d_n7;
        locals.var_temp_dn8 = assign35100_e39353_d_n8;
        locals.var_temp_dn9 = assign35100_e39353_d_n9;

        let (assign35110_e39366, assign35110_e39366_d_n4, assign35110_e39366_d_n6, assign35110_e39366_d_n7, assign35110_e39366_d_n8, assign35110_e39366_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1156 != 0.0)) && (locals.var_guard1157 == 0.0)) {
        let assign35110_e39363: f64 = (locals.var_temp - 1.0);
        let assign35110_e39364: f64 = (locals.var_dx_wi__blk935 / assign35110_e39363);
        (assign35110_e39364, (((locals.var_dx_wi__blk935_dn4 * assign35110_e39363) - (locals.var_dx_wi__blk935 * locals.var_temp_dn4)) / (assign35110_e39363 * assign35110_e39363)), (((locals.var_dx_wi__blk935_dn6 * assign35110_e39363) - (locals.var_dx_wi__blk935 * locals.var_temp_dn6)) / (assign35110_e39363 * assign35110_e39363)), (((locals.var_dx_wi__blk935_dn7 * assign35110_e39363) - (locals.var_dx_wi__blk935 * locals.var_temp_dn7)) / (assign35110_e39363 * assign35110_e39363)), (((locals.var_dx_wi__blk935_dn8 * assign35110_e39363) - (locals.var_dx_wi__blk935 * locals.var_temp_dn8)) / (assign35110_e39363 * assign35110_e39363)), (((locals.var_dx_wi__blk935_dn9 * assign35110_e39363) - (locals.var_dx_wi__blk935 * locals.var_temp_dn9)) / (assign35110_e39363 * assign35110_e39363)),)
    } else {
        (locals.var_s2__blk970, locals.var_s2__blk970_dn4, locals.var_s2__blk970_dn6, locals.var_s2__blk970_dn7, locals.var_s2__blk970_dn8, locals.var_s2__blk970_dn9,)
    }
};
        locals.var_s2__blk970 = assign35110_e39366;
        locals.var_s2__blk970_dn4 = assign35110_e39366_d_n4;
        locals.var_s2__blk970_dn6 = assign35110_e39366_d_n6;
        locals.var_s2__blk970_dn7 = assign35110_e39366_d_n7;
        locals.var_s2__blk970_dn8 = assign35110_e39366_d_n8;
        locals.var_s2__blk970_dn9 = assign35110_e39366_d_n9;

        let (assign35120_e39377, assign35120_e39377_d_n4, assign35120_e39377_d_n6, assign35120_e39377_d_n7, assign35120_e39377_d_n8, assign35120_e39377_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1156 != 0.0)) && (locals.var_guard1157 == 0.0)) {
        let assign35120_e39375: f64 = (locals.var_temp * locals.var_s2__blk970);
        (assign35120_e39375, ((locals.var_temp_dn4 * locals.var_s2__blk970) + (locals.var_temp * locals.var_s2__blk970_dn4)), ((locals.var_temp_dn6 * locals.var_s2__blk970) + (locals.var_temp * locals.var_s2__blk970_dn6)), ((locals.var_temp_dn7 * locals.var_s2__blk970) + (locals.var_temp * locals.var_s2__blk970_dn7)), ((locals.var_temp_dn8 * locals.var_s2__blk970) + (locals.var_temp * locals.var_s2__blk970_dn8)), ((locals.var_temp_dn9 * locals.var_s2__blk970) + (locals.var_temp * locals.var_s2__blk970_dn9)),)
    } else {
        (locals.var_s1__blk969, locals.var_s1__blk969_dn4, locals.var_s1__blk969_dn6, locals.var_s1__blk969_dn7, locals.var_s1__blk969_dn8, locals.var_s1__blk969_dn9,)
    }
};
        locals.var_s1__blk969 = assign35120_e39377;
        locals.var_s1__blk969_dn4 = assign35120_e39377_d_n4;
        locals.var_s1__blk969_dn6 = assign35120_e39377_d_n6;
        locals.var_s1__blk969_dn7 = assign35120_e39377_d_n7;
        locals.var_s1__blk969_dn8 = assign35120_e39377_d_n8;
        locals.var_s1__blk969_dn9 = assign35120_e39377_d_n9;

        let (assign35130_e39395, assign35130_e39395_d_n4, assign35130_e39395_d_n6, assign35130_e39395_d_n7, assign35130_e39395_d_n8, assign35130_e39395_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1156 != 0.0)) && (locals.var_guard1157 == 0.0)) {
        let assign35130_e39387: f64 = (locals.var_qis__blk938 * locals.var_s2__blk970);
        let assign35130_e39388: f64 = (locals.var_a0__blk905 / assign35130_e39387);
        let assign35130_e39389: f64 = (assign35130_e39388).ln();
        let assign35130_e39391: f64 = (assign35130_e39389 - 0.6931471805599);
        let assign35130_e39393: f64 = (assign35130_e39391 + locals.var_x2_wi0__blk909);
        (assign35130_e39393, (((((locals.var_a0__blk905_dn4 * assign35130_e39387) - (locals.var_a0__blk905 * ((locals.var_qis__blk938_dn4 * locals.var_s2__blk970) + (locals.var_qis__blk938 * locals.var_s2__blk970_dn4)))) / (assign35130_e39387 * assign35130_e39387)) / assign35130_e39388) + locals.var_x2_wi0__blk909_dn4), (((((locals.var_a0__blk905_dn6 * assign35130_e39387) - (locals.var_a0__blk905 * ((locals.var_qis__blk938_dn6 * locals.var_s2__blk970) + (locals.var_qis__blk938 * locals.var_s2__blk970_dn6)))) / (assign35130_e39387 * assign35130_e39387)) / assign35130_e39388) + locals.var_x2_wi0__blk909_dn6), (((((locals.var_a0__blk905_dn7 * assign35130_e39387) - (locals.var_a0__blk905 * ((locals.var_qis__blk938_dn7 * locals.var_s2__blk970) + (locals.var_qis__blk938 * locals.var_s2__blk970_dn7)))) / (assign35130_e39387 * assign35130_e39387)) / assign35130_e39388) + locals.var_x2_wi0__blk909_dn7), (((((locals.var_a0__blk905_dn8 * assign35130_e39387) - (locals.var_a0__blk905 * ((locals.var_qis__blk938_dn8 * locals.var_s2__blk970) + (locals.var_qis__blk938 * locals.var_s2__blk970_dn8)))) / (assign35130_e39387 * assign35130_e39387)) / assign35130_e39388) + locals.var_x2_wi0__blk909_dn8), (((((locals.var_a0__blk905_dn9 * assign35130_e39387) - (locals.var_a0__blk905 * ((locals.var_qis__blk938_dn9 * locals.var_s2__blk970) + (locals.var_qis__blk938 * locals.var_s2__blk970_dn9)))) / (assign35130_e39387 * assign35130_e39387)) / assign35130_e39388) + locals.var_x2_wi0__blk909_dn9),)
    } else {
        (locals.var_deltaxinf__blk971, locals.var_deltaxinf__blk971_dn4, locals.var_deltaxinf__blk971_dn6, locals.var_deltaxinf__blk971_dn7, locals.var_deltaxinf__blk971_dn8, locals.var_deltaxinf__blk971_dn9,)
    }
};
        locals.var_deltaxinf__blk971 = assign35130_e39395;
        locals.var_deltaxinf__blk971_dn4 = assign35130_e39395_d_n4;
        locals.var_deltaxinf__blk971_dn6 = assign35130_e39395_d_n6;
        locals.var_deltaxinf__blk971_dn7 = assign35130_e39395_d_n7;
        locals.var_deltaxinf__blk971_dn8 = assign35130_e39395_d_n8;
        locals.var_deltaxinf__blk971_dn9 = assign35130_e39395_d_n9;

        let (assign35140_e39412, assign35140_e39412_d_n4, assign35140_e39412_d_n6, assign35140_e39412_d_n7, assign35140_e39412_d_n8, assign35140_e39412_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1156 != 0.0)) {
        let assign35140_e39400: f64 = (-locals.var_dx_wi__blk935);
        let assign35140_e39404: f64 = (1.0 - locals.var_s1__blk969);
        let assign35140_e39407: f64 = (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907);
        let assign35140_e39408: f64 = (assign35140_e39404 - assign35140_e39407);
        let assign35140_e39409: f64 = (locals.var_keq__blk934 * assign35140_e39408);
        let assign35140_e39410: f64 = (assign35140_e39400 / assign35140_e39409);
        (assign35140_e39410, ((((-locals.var_dx_wi__blk935_dn4) * assign35140_e39409) - (assign35140_e39400 * ((locals.var_keq__blk934_dn4 * assign35140_e39408) + (locals.var_keq__blk934 * ((-locals.var_s1__blk969_dn4) - ((locals.var_dx_wi__blk935_dn4 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn4))))))) / (assign35140_e39409 * assign35140_e39409)), ((((-locals.var_dx_wi__blk935_dn6) * assign35140_e39409) - (assign35140_e39400 * ((locals.var_keq__blk934_dn6 * assign35140_e39408) + (locals.var_keq__blk934 * ((-locals.var_s1__blk969_dn6) - ((locals.var_dx_wi__blk935_dn6 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn6))))))) / (assign35140_e39409 * assign35140_e39409)), ((((-locals.var_dx_wi__blk935_dn7) * assign35140_e39409) - (assign35140_e39400 * ((locals.var_keq__blk934_dn7 * assign35140_e39408) + (locals.var_keq__blk934 * ((-locals.var_s1__blk969_dn7) - ((locals.var_dx_wi__blk935_dn7 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn7))))))) / (assign35140_e39409 * assign35140_e39409)), ((((-locals.var_dx_wi__blk935_dn8) * assign35140_e39409) - (assign35140_e39400 * ((locals.var_keq__blk934_dn8 * assign35140_e39408) + (locals.var_keq__blk934 * ((-locals.var_s1__blk969_dn8) - ((locals.var_dx_wi__blk935_dn8 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn8))))))) / (assign35140_e39409 * assign35140_e39409)), ((((-locals.var_dx_wi__blk935_dn9) * assign35140_e39409) - (assign35140_e39400 * ((locals.var_keq__blk934_dn9 * assign35140_e39408) + (locals.var_keq__blk934 * ((-locals.var_s1__blk969_dn9) - ((locals.var_dx_wi__blk935_dn9 * locals.var_inv_k2__blk907) + (locals.var_dx_wi__blk935 * locals.var_inv_k2__blk907_dn9))))))) / (assign35140_e39409 * assign35140_e39409)),)
    } else {
        (locals.var_q1chapinf__blk972, locals.var_q1chapinf__blk972_dn4, locals.var_q1chapinf__blk972_dn6, locals.var_q1chapinf__blk972_dn7, locals.var_q1chapinf__blk972_dn8, locals.var_q1chapinf__blk972_dn9,)
    }
};
        locals.var_q1chapinf__blk972 = assign35140_e39412;
        locals.var_q1chapinf__blk972_dn4 = assign35140_e39412_d_n4;
        locals.var_q1chapinf__blk972_dn6 = assign35140_e39412_d_n6;
        locals.var_q1chapinf__blk972_dn7 = assign35140_e39412_d_n7;
        locals.var_q1chapinf__blk972_dn8 = assign35140_e39412_d_n8;
        locals.var_q1chapinf__blk972_dn9 = assign35140_e39412_d_n9;

        let (assign35150_e39428, assign35150_e39428_d_n4, assign35150_e39428_d_n6, assign35150_e39428_d_n7, assign35150_e39428_d_n8, assign35150_e39428_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1156 != 0.0)) {
        let assign35150_e39420: f64 = (1.0 - locals.var_s2__blk970);
        let assign35150_e39423: f64 = (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906);
        let assign35150_e39424: f64 = (assign35150_e39420 + assign35150_e39423);
        let assign35150_e39425: f64 = (locals.var_keq__blk934 * assign35150_e39424);
        let assign35150_e39426: f64 = (locals.var_dx_wi__blk935 / assign35150_e39425);
        (assign35150_e39426, (((locals.var_dx_wi__blk935_dn4 * assign35150_e39425) - (locals.var_dx_wi__blk935 * ((locals.var_keq__blk934_dn4 * assign35150_e39424) + (locals.var_keq__blk934 * ((-locals.var_s2__blk970_dn4) + ((locals.var_dx_wi__blk935_dn4 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn4))))))) / (assign35150_e39425 * assign35150_e39425)), (((locals.var_dx_wi__blk935_dn6 * assign35150_e39425) - (locals.var_dx_wi__blk935 * ((locals.var_keq__blk934_dn6 * assign35150_e39424) + (locals.var_keq__blk934 * ((-locals.var_s2__blk970_dn6) + ((locals.var_dx_wi__blk935_dn6 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn6))))))) / (assign35150_e39425 * assign35150_e39425)), (((locals.var_dx_wi__blk935_dn7 * assign35150_e39425) - (locals.var_dx_wi__blk935 * ((locals.var_keq__blk934_dn7 * assign35150_e39424) + (locals.var_keq__blk934 * ((-locals.var_s2__blk970_dn7) + ((locals.var_dx_wi__blk935_dn7 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn7))))))) / (assign35150_e39425 * assign35150_e39425)), (((locals.var_dx_wi__blk935_dn8 * assign35150_e39425) - (locals.var_dx_wi__blk935 * ((locals.var_keq__blk934_dn8 * assign35150_e39424) + (locals.var_keq__blk934 * ((-locals.var_s2__blk970_dn8) + ((locals.var_dx_wi__blk935_dn8 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn8))))))) / (assign35150_e39425 * assign35150_e39425)), (((locals.var_dx_wi__blk935_dn9 * assign35150_e39425) - (locals.var_dx_wi__blk935 * ((locals.var_keq__blk934_dn9 * assign35150_e39424) + (locals.var_keq__blk934 * ((-locals.var_s2__blk970_dn9) + ((locals.var_dx_wi__blk935_dn9 * locals.var_inv_k1__blk906) + (locals.var_dx_wi__blk935 * locals.var_inv_k1__blk906_dn9))))))) / (assign35150_e39425 * assign35150_e39425)),)
    } else {
        (locals.var_q2chapinf__blk973, locals.var_q2chapinf__blk973_dn4, locals.var_q2chapinf__blk973_dn6, locals.var_q2chapinf__blk973_dn7, locals.var_q2chapinf__blk973_dn8, locals.var_q2chapinf__blk973_dn9,)
    }
};
        locals.var_q2chapinf__blk973 = assign35150_e39428;
        locals.var_q2chapinf__blk973_dn4 = assign35150_e39428_d_n4;
        locals.var_q2chapinf__blk973_dn6 = assign35150_e39428_d_n6;
        locals.var_q2chapinf__blk973_dn7 = assign35150_e39428_d_n7;
        locals.var_q2chapinf__blk973_dn8 = assign35150_e39428_d_n8;
        locals.var_q2chapinf__blk973_dn9 = assign35150_e39428_d_n9;

        let (assign35160_e39450, assign35160_e39450_d_n4, assign35160_e39450_d_n6, assign35160_e39450_d_n7, assign35160_e39450_d_n8, assign35160_e39450_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1156 != 0.0)) {
        let assign35160_e39435: f64 = (locals.var_s2__blk970 * locals.var_inv_k2__blk907);
        let assign35160_e39437: f64 = (assign35160_e39435 + 0.5);
        let assign35160_e39439: f64 = (assign35160_e39437 / locals.var_q2chapinf__blk973);
        let assign35160_e39442: f64 = (locals.var_s1__blk969 * locals.var_inv_k1__blk906);
        let assign35160_e39444: f64 = (assign35160_e39442 + 0.5);
        let assign35160_e39446: f64 = (assign35160_e39444 / locals.var_q1chapinf__blk972);
        let assign35160_e39447: f64 = (assign35160_e39439 - assign35160_e39446);
        let assign35160_e39448: f64 = (locals.var_dx_wi__blk935 / assign35160_e39447);
        (assign35160_e39448, (((locals.var_dx_wi__blk935_dn4 * assign35160_e39447) - (locals.var_dx_wi__blk935 * ((((((locals.var_s2__blk970_dn4 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn4)) * locals.var_q2chapinf__blk973) - (assign35160_e39437 * locals.var_q2chapinf__blk973_dn4)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) - (((((locals.var_s1__blk969_dn4 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn4)) * locals.var_q1chapinf__blk972) - (assign35160_e39444 * locals.var_q1chapinf__blk972_dn4)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))))) / (assign35160_e39447 * assign35160_e39447)), (((locals.var_dx_wi__blk935_dn6 * assign35160_e39447) - (locals.var_dx_wi__blk935 * ((((((locals.var_s2__blk970_dn6 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn6)) * locals.var_q2chapinf__blk973) - (assign35160_e39437 * locals.var_q2chapinf__blk973_dn6)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) - (((((locals.var_s1__blk969_dn6 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn6)) * locals.var_q1chapinf__blk972) - (assign35160_e39444 * locals.var_q1chapinf__blk972_dn6)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))))) / (assign35160_e39447 * assign35160_e39447)), (((locals.var_dx_wi__blk935_dn7 * assign35160_e39447) - (locals.var_dx_wi__blk935 * ((((((locals.var_s2__blk970_dn7 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn7)) * locals.var_q2chapinf__blk973) - (assign35160_e39437 * locals.var_q2chapinf__blk973_dn7)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) - (((((locals.var_s1__blk969_dn7 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn7)) * locals.var_q1chapinf__blk972) - (assign35160_e39444 * locals.var_q1chapinf__blk972_dn7)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))))) / (assign35160_e39447 * assign35160_e39447)), (((locals.var_dx_wi__blk935_dn8 * assign35160_e39447) - (locals.var_dx_wi__blk935 * ((((((locals.var_s2__blk970_dn8 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn8)) * locals.var_q2chapinf__blk973) - (assign35160_e39437 * locals.var_q2chapinf__blk973_dn8)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) - (((((locals.var_s1__blk969_dn8 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn8)) * locals.var_q1chapinf__blk972) - (assign35160_e39444 * locals.var_q1chapinf__blk972_dn8)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))))) / (assign35160_e39447 * assign35160_e39447)), (((locals.var_dx_wi__blk935_dn9 * assign35160_e39447) - (locals.var_dx_wi__blk935 * ((((((locals.var_s2__blk970_dn9 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn9)) * locals.var_q2chapinf__blk973) - (assign35160_e39437 * locals.var_q2chapinf__blk973_dn9)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) - (((((locals.var_s1__blk969_dn9 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn9)) * locals.var_q1chapinf__blk972) - (assign35160_e39444 * locals.var_q1chapinf__blk972_dn9)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))))) / (assign35160_e39447 * assign35160_e39447)),)
    } else {
        (locals.var_dinf__blk974, locals.var_dinf__blk974_dn4, locals.var_dinf__blk974_dn6, locals.var_dinf__blk974_dn7, locals.var_dinf__blk974_dn8, locals.var_dinf__blk974_dn9,)
    }
};
        locals.var_dinf__blk974 = assign35160_e39450;
        locals.var_dinf__blk974_dn4 = assign35160_e39450_d_n4;
        locals.var_dinf__blk974_dn6 = assign35160_e39450_d_n6;
        locals.var_dinf__blk974_dn7 = assign35160_e39450_d_n7;
        locals.var_dinf__blk974_dn8 = assign35160_e39450_d_n8;
        locals.var_dinf__blk974_dn9 = assign35160_e39450_d_n9;

        let (assign35170_e39461, assign35170_e39461_d_n4, assign35170_e39461_d_n6, assign35170_e39461_d_n7, assign35170_e39461_d_n8, assign35170_e39461_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1156 == 0.0)) {
        let assign35170_e39457: f64 = (0.5 * 0.1666666666667);
        let assign35170_e39459: f64 = (assign35170_e39457 * locals.var_dx_wisq__blk936);
        (assign35170_e39459, (assign35170_e39457 * locals.var_dx_wisq__blk936_dn4), (assign35170_e39457 * locals.var_dx_wisq__blk936_dn6), (assign35170_e39457 * locals.var_dx_wisq__blk936_dn7), (assign35170_e39457 * locals.var_dx_wisq__blk936_dn8), (assign35170_e39457 * locals.var_dx_wisq__blk936_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign35170_e39461;
        locals.var_temp_dn4 = assign35170_e39461_d_n4;
        locals.var_temp_dn6 = assign35170_e39461_d_n6;
        locals.var_temp_dn7 = assign35170_e39461_d_n7;
        locals.var_temp_dn8 = assign35170_e39461_d_n8;
        locals.var_temp_dn9 = assign35170_e39461_d_n9;

        let (assign35180_e39470, assign35180_e39470_d_n4, assign35180_e39470_d_n6, assign35180_e39470_d_n7, assign35180_e39470_d_n8, assign35180_e39470_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1156 == 0.0)) {
        let assign35180_e39468: f64 = (0.5 * locals.var_dx_wi__blk935);
        (assign35180_e39468, (0.5 * locals.var_dx_wi__blk935_dn4), (0.5 * locals.var_dx_wi__blk935_dn6), (0.5 * locals.var_dx_wi__blk935_dn7), (0.5 * locals.var_dx_wi__blk935_dn8), (0.5 * locals.var_dx_wi__blk935_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign35180_e39470;
        locals.var_temp1_dn4 = assign35180_e39470_d_n4;
        locals.var_temp1_dn6 = assign35180_e39470_d_n6;
        locals.var_temp1_dn7 = assign35180_e39470_d_n7;
        locals.var_temp1_dn8 = assign35180_e39470_d_n8;
        locals.var_temp1_dn9 = assign35180_e39470_d_n9;

        let (assign35190_e39481, assign35190_e39481_d_n4, assign35190_e39481_d_n6, assign35190_e39481_d_n7, assign35190_e39481_d_n8, assign35190_e39481_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1156 == 0.0)) {
        let assign35190_e39477: f64 = (1.0 + locals.var_temp1);
        let assign35190_e39479: f64 = (assign35190_e39477 + locals.var_temp);
        (assign35190_e39479, (locals.var_temp1_dn4 + locals.var_temp_dn4), (locals.var_temp1_dn6 + locals.var_temp_dn6), (locals.var_temp1_dn7 + locals.var_temp_dn7), (locals.var_temp1_dn8 + locals.var_temp_dn8), (locals.var_temp1_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_s1__blk969, locals.var_s1__blk969_dn4, locals.var_s1__blk969_dn6, locals.var_s1__blk969_dn7, locals.var_s1__blk969_dn8, locals.var_s1__blk969_dn9,)
    }
};
        locals.var_s1__blk969 = assign35190_e39481;
        locals.var_s1__blk969_dn4 = assign35190_e39481_d_n4;
        locals.var_s1__blk969_dn6 = assign35190_e39481_d_n6;
        locals.var_s1__blk969_dn7 = assign35190_e39481_d_n7;
        locals.var_s1__blk969_dn8 = assign35190_e39481_d_n8;
        locals.var_s1__blk969_dn9 = assign35190_e39481_d_n9;

        let (assign35200_e39492, assign35200_e39492_d_n4, assign35200_e39492_d_n6, assign35200_e39492_d_n7, assign35200_e39492_d_n8, assign35200_e39492_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1156 == 0.0)) {
        let assign35200_e39488: f64 = (1.0 - locals.var_temp1);
        let assign35200_e39490: f64 = (assign35200_e39488 + locals.var_temp);
        (assign35200_e39490, ((-locals.var_temp1_dn4) + locals.var_temp_dn4), ((-locals.var_temp1_dn6) + locals.var_temp_dn6), ((-locals.var_temp1_dn7) + locals.var_temp_dn7), ((-locals.var_temp1_dn8) + locals.var_temp_dn8), ((-locals.var_temp1_dn9) + locals.var_temp_dn9),)
    } else {
        (locals.var_s2__blk970, locals.var_s2__blk970_dn4, locals.var_s2__blk970_dn6, locals.var_s2__blk970_dn7, locals.var_s2__blk970_dn8, locals.var_s2__blk970_dn9,)
    }
};
        locals.var_s2__blk970 = assign35200_e39492;
        locals.var_s2__blk970_dn4 = assign35200_e39492_d_n4;
        locals.var_s2__blk970_dn6 = assign35200_e39492_d_n6;
        locals.var_s2__blk970_dn7 = assign35200_e39492_d_n7;
        locals.var_s2__blk970_dn8 = assign35200_e39492_d_n8;
        locals.var_s2__blk970_dn9 = assign35200_e39492_d_n9;

        let (assign35210_e39501, assign35210_e39501_d_n4, assign35210_e39501_d_n6, assign35210_e39501_d_n7, assign35210_e39501_d_n8, assign35210_e39501_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1156 == 0.0)) {
        let assign35210_e39499: f64 = (0.1666666666667 * locals.var_temp1);
        (assign35210_e39499, (0.1666666666667 * locals.var_temp1_dn4), (0.1666666666667 * locals.var_temp1_dn6), (0.1666666666667 * locals.var_temp1_dn7), (0.1666666666667 * locals.var_temp1_dn8), (0.1666666666667 * locals.var_temp1_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign35210_e39501;
        locals.var_temp2_dn4 = assign35210_e39501_d_n4;
        locals.var_temp2_dn6 = assign35210_e39501_d_n6;
        locals.var_temp2_dn7 = assign35210_e39501_d_n7;
        locals.var_temp2_dn8 = assign35210_e39501_d_n8;
        locals.var_temp2_dn9 = assign35210_e39501_d_n9;

        let (assign35220_e39516, assign35220_e39516_d_n4, assign35220_e39516_d_n6, assign35220_e39516_d_n7, assign35220_e39516_d_n8, assign35220_e39516_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1156 == 0.0)) {
        let assign35220_e39510: f64 = (0.5 + locals.var_inv_k2__blk907);
        let assign35220_e39512: f64 = (assign35220_e39510 + locals.var_temp2);
        let assign35220_e39513: f64 = (locals.var_keq__blk934 * assign35220_e39512);
        let assign35220_e39514: f64 = (1.0 / assign35220_e39513);
        (assign35220_e39514, (-(((locals.var_keq__blk934_dn4 * assign35220_e39512) + (locals.var_keq__blk934 * (locals.var_inv_k2__blk907_dn4 + locals.var_temp2_dn4))) / (assign35220_e39513 * assign35220_e39513))), (-(((locals.var_keq__blk934_dn6 * assign35220_e39512) + (locals.var_keq__blk934 * (locals.var_inv_k2__blk907_dn6 + locals.var_temp2_dn6))) / (assign35220_e39513 * assign35220_e39513))), (-(((locals.var_keq__blk934_dn7 * assign35220_e39512) + (locals.var_keq__blk934 * (locals.var_inv_k2__blk907_dn7 + locals.var_temp2_dn7))) / (assign35220_e39513 * assign35220_e39513))), (-(((locals.var_keq__blk934_dn8 * assign35220_e39512) + (locals.var_keq__blk934 * (locals.var_inv_k2__blk907_dn8 + locals.var_temp2_dn8))) / (assign35220_e39513 * assign35220_e39513))), (-(((locals.var_keq__blk934_dn9 * assign35220_e39512) + (locals.var_keq__blk934 * (locals.var_inv_k2__blk907_dn9 + locals.var_temp2_dn9))) / (assign35220_e39513 * assign35220_e39513))),)
    } else {
        (locals.var_q1chapinf__blk972, locals.var_q1chapinf__blk972_dn4, locals.var_q1chapinf__blk972_dn6, locals.var_q1chapinf__blk972_dn7, locals.var_q1chapinf__blk972_dn8, locals.var_q1chapinf__blk972_dn9,)
    }
};
        locals.var_q1chapinf__blk972 = assign35220_e39516;
        locals.var_q1chapinf__blk972_dn4 = assign35220_e39516_d_n4;
        locals.var_q1chapinf__blk972_dn6 = assign35220_e39516_d_n6;
        locals.var_q1chapinf__blk972_dn7 = assign35220_e39516_d_n7;
        locals.var_q1chapinf__blk972_dn8 = assign35220_e39516_d_n8;
        locals.var_q1chapinf__blk972_dn9 = assign35220_e39516_d_n9;

        let (assign35230_e39531, assign35230_e39531_d_n4, assign35230_e39531_d_n6, assign35230_e39531_d_n7, assign35230_e39531_d_n8, assign35230_e39531_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1156 == 0.0)) {
        let assign35230_e39525: f64 = (0.5 + locals.var_inv_k1__blk906);
        let assign35230_e39527: f64 = (assign35230_e39525 - locals.var_temp2);
        let assign35230_e39528: f64 = (locals.var_keq__blk934 * assign35230_e39527);
        let assign35230_e39529: f64 = (1.0 / assign35230_e39528);
        (assign35230_e39529, (-(((locals.var_keq__blk934_dn4 * assign35230_e39527) + (locals.var_keq__blk934 * (locals.var_inv_k1__blk906_dn4 - locals.var_temp2_dn4))) / (assign35230_e39528 * assign35230_e39528))), (-(((locals.var_keq__blk934_dn6 * assign35230_e39527) + (locals.var_keq__blk934 * (locals.var_inv_k1__blk906_dn6 - locals.var_temp2_dn6))) / (assign35230_e39528 * assign35230_e39528))), (-(((locals.var_keq__blk934_dn7 * assign35230_e39527) + (locals.var_keq__blk934 * (locals.var_inv_k1__blk906_dn7 - locals.var_temp2_dn7))) / (assign35230_e39528 * assign35230_e39528))), (-(((locals.var_keq__blk934_dn8 * assign35230_e39527) + (locals.var_keq__blk934 * (locals.var_inv_k1__blk906_dn8 - locals.var_temp2_dn8))) / (assign35230_e39528 * assign35230_e39528))), (-(((locals.var_keq__blk934_dn9 * assign35230_e39527) + (locals.var_keq__blk934 * (locals.var_inv_k1__blk906_dn9 - locals.var_temp2_dn9))) / (assign35230_e39528 * assign35230_e39528))),)
    } else {
        (locals.var_q2chapinf__blk973, locals.var_q2chapinf__blk973_dn4, locals.var_q2chapinf__blk973_dn6, locals.var_q2chapinf__blk973_dn7, locals.var_q2chapinf__blk973_dn8, locals.var_q2chapinf__blk973_dn9,)
    }
};
        locals.var_q2chapinf__blk973 = assign35230_e39531;
        locals.var_q2chapinf__blk973_dn4 = assign35230_e39531_d_n4;
        locals.var_q2chapinf__blk973_dn6 = assign35230_e39531_d_n6;
        locals.var_q2chapinf__blk973_dn7 = assign35230_e39531_d_n7;
        locals.var_q2chapinf__blk973_dn8 = assign35230_e39531_d_n8;
        locals.var_q2chapinf__blk973_dn9 = assign35230_e39531_d_n9;

        let (assign35240_e39555, assign35240_e39555_d_n4, assign35240_e39555_d_n6, assign35240_e39555_d_n7, assign35240_e39555_d_n8, assign35240_e39555_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1156 == 0.0)) {
        let assign35240_e39541: f64 = (0.5 * locals.var_temp);
        let assign35240_e39542: f64 = (1.0 - assign35240_e39541);
        let assign35240_e39543: f64 = (locals.var_qis__blk938 * assign35240_e39542);
        let assign35240_e39544: f64 = (locals.var_a0__blk905 / assign35240_e39543);
        let assign35240_e39545: f64 = (assign35240_e39544).ln();
        let assign35240_e39547: f64 = (assign35240_e39545 - 0.6931471805599);
        let assign35240_e39551: f64 = (locals.var_x1_wi0__blk908 + locals.var_x2_wi0__blk909);
        let assign35240_e39552: f64 = (0.5 * assign35240_e39551);
        let assign35240_e39553: f64 = (assign35240_e39547 + assign35240_e39552);
        (assign35240_e39553, (((((locals.var_a0__blk905_dn4 * assign35240_e39543) - (locals.var_a0__blk905 * ((locals.var_qis__blk938_dn4 * assign35240_e39542) + (locals.var_qis__blk938 * (-(0.5 * locals.var_temp_dn4)))))) / (assign35240_e39543 * assign35240_e39543)) / assign35240_e39544) + (0.5 * (locals.var_x1_wi0__blk908_dn4 + locals.var_x2_wi0__blk909_dn4))), (((((locals.var_a0__blk905_dn6 * assign35240_e39543) - (locals.var_a0__blk905 * ((locals.var_qis__blk938_dn6 * assign35240_e39542) + (locals.var_qis__blk938 * (-(0.5 * locals.var_temp_dn6)))))) / (assign35240_e39543 * assign35240_e39543)) / assign35240_e39544) + (0.5 * (locals.var_x1_wi0__blk908_dn6 + locals.var_x2_wi0__blk909_dn6))), (((((locals.var_a0__blk905_dn7 * assign35240_e39543) - (locals.var_a0__blk905 * ((locals.var_qis__blk938_dn7 * assign35240_e39542) + (locals.var_qis__blk938 * (-(0.5 * locals.var_temp_dn7)))))) / (assign35240_e39543 * assign35240_e39543)) / assign35240_e39544) + (0.5 * (locals.var_x1_wi0__blk908_dn7 + locals.var_x2_wi0__blk909_dn7))), (((((locals.var_a0__blk905_dn8 * assign35240_e39543) - (locals.var_a0__blk905 * ((locals.var_qis__blk938_dn8 * assign35240_e39542) + (locals.var_qis__blk938 * (-(0.5 * locals.var_temp_dn8)))))) / (assign35240_e39543 * assign35240_e39543)) / assign35240_e39544) + (0.5 * (locals.var_x1_wi0__blk908_dn8 + locals.var_x2_wi0__blk909_dn8))), (((((locals.var_a0__blk905_dn9 * assign35240_e39543) - (locals.var_a0__blk905 * ((locals.var_qis__blk938_dn9 * assign35240_e39542) + (locals.var_qis__blk938 * (-(0.5 * locals.var_temp_dn9)))))) / (assign35240_e39543 * assign35240_e39543)) / assign35240_e39544) + (0.5 * (locals.var_x1_wi0__blk908_dn9 + locals.var_x2_wi0__blk909_dn9))),)
    } else {
        (locals.var_deltaxinf__blk971, locals.var_deltaxinf__blk971_dn4, locals.var_deltaxinf__blk971_dn6, locals.var_deltaxinf__blk971_dn7, locals.var_deltaxinf__blk971_dn8, locals.var_deltaxinf__blk971_dn9,)
    }
};
        locals.var_deltaxinf__blk971 = assign35240_e39555;
        locals.var_deltaxinf__blk971_dn4 = assign35240_e39555_d_n4;
        locals.var_deltaxinf__blk971_dn6 = assign35240_e39555_d_n6;
        locals.var_deltaxinf__blk971_dn7 = assign35240_e39555_d_n7;
        locals.var_deltaxinf__blk971_dn8 = assign35240_e39555_d_n8;
        locals.var_deltaxinf__blk971_dn9 = assign35240_e39555_d_n9;

        let (assign35250_e39595, assign35250_e39595_d_n4, assign35250_e39595_d_n6, assign35250_e39595_d_n7, assign35250_e39595_d_n8, assign35250_e39595_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1156 == 0.0)) {
        let assign35250_e39561: f64 = (-12.0);
        let assign35250_e39565: f64 = (3.0 * locals.var_keq__blk934);
        let assign35250_e39566: f64 = (4.0 - assign35250_e39565);
        let assign35250_e39569: f64 = (12.0 * locals.var_keq__blk934);
        let assign35250_e39572: f64 = (locals.var_k1__blk932 * locals.var_k2__blk933);
        let assign35250_e39573: f64 = (assign35250_e39569 / assign35250_e39572);
        let assign35250_e39574: f64 = (assign35250_e39566 + assign35250_e39573);
        let assign35250_e39578: f64 = (locals.var_inv_k1__blk906 - locals.var_inv_k2__blk907);
        let assign35250_e39579: f64 = (locals.var_keq__blk934 * assign35250_e39578);
        let assign35250_e39581: f64 = (assign35250_e39579 * locals.var_dx_wi__blk935);
        let assign35250_e39582: f64 = (assign35250_e39574 + assign35250_e39581);
        let assign35250_e39587: f64 = (0.25 * locals.var_keq__blk934);
        let assign35250_e39588: f64 = (0.2 - assign35250_e39587);
        let assign35250_e39589: f64 = (0.3333333333333 * assign35250_e39588);
        let assign35250_e39591: f64 = (assign35250_e39589 * locals.var_dx_wisq__blk936);
        let assign35250_e39592: f64 = (assign35250_e39582 + assign35250_e39591);
        let assign35250_e39593: f64 = (assign35250_e39561 / assign35250_e39592);
        (assign35250_e39593, (-((assign35250_e39561 * ((((-(3.0 * locals.var_keq__blk934_dn4)) + ((((12.0 * locals.var_keq__blk934_dn4) * assign35250_e39572) - (assign35250_e39569 * ((locals.var_k1__blk932_dn4 * locals.var_k2__blk933) + (locals.var_k1__blk932 * locals.var_k2__blk933_dn4)))) / (assign35250_e39572 * assign35250_e39572))) + ((((locals.var_keq__blk934_dn4 * assign35250_e39578) + (locals.var_keq__blk934 * (locals.var_inv_k1__blk906_dn4 - locals.var_inv_k2__blk907_dn4))) * locals.var_dx_wi__blk935) + (assign35250_e39579 * locals.var_dx_wi__blk935_dn4))) + (((0.3333333333333 * (-(0.25 * locals.var_keq__blk934_dn4))) * locals.var_dx_wisq__blk936) + (assign35250_e39589 * locals.var_dx_wisq__blk936_dn4)))) / (assign35250_e39592 * assign35250_e39592))), (-((assign35250_e39561 * ((((-(3.0 * locals.var_keq__blk934_dn6)) + ((((12.0 * locals.var_keq__blk934_dn6) * assign35250_e39572) - (assign35250_e39569 * ((locals.var_k1__blk932_dn6 * locals.var_k2__blk933) + (locals.var_k1__blk932 * locals.var_k2__blk933_dn6)))) / (assign35250_e39572 * assign35250_e39572))) + ((((locals.var_keq__blk934_dn6 * assign35250_e39578) + (locals.var_keq__blk934 * (locals.var_inv_k1__blk906_dn6 - locals.var_inv_k2__blk907_dn6))) * locals.var_dx_wi__blk935) + (assign35250_e39579 * locals.var_dx_wi__blk935_dn6))) + (((0.3333333333333 * (-(0.25 * locals.var_keq__blk934_dn6))) * locals.var_dx_wisq__blk936) + (assign35250_e39589 * locals.var_dx_wisq__blk936_dn6)))) / (assign35250_e39592 * assign35250_e39592))), (-((assign35250_e39561 * ((((-(3.0 * locals.var_keq__blk934_dn7)) + ((((12.0 * locals.var_keq__blk934_dn7) * assign35250_e39572) - (assign35250_e39569 * ((locals.var_k1__blk932_dn7 * locals.var_k2__blk933) + (locals.var_k1__blk932 * locals.var_k2__blk933_dn7)))) / (assign35250_e39572 * assign35250_e39572))) + ((((locals.var_keq__blk934_dn7 * assign35250_e39578) + (locals.var_keq__blk934 * (locals.var_inv_k1__blk906_dn7 - locals.var_inv_k2__blk907_dn7))) * locals.var_dx_wi__blk935) + (assign35250_e39579 * locals.var_dx_wi__blk935_dn7))) + (((0.3333333333333 * (-(0.25 * locals.var_keq__blk934_dn7))) * locals.var_dx_wisq__blk936) + (assign35250_e39589 * locals.var_dx_wisq__blk936_dn7)))) / (assign35250_e39592 * assign35250_e39592))), (-((assign35250_e39561 * ((((-(3.0 * locals.var_keq__blk934_dn8)) + ((((12.0 * locals.var_keq__blk934_dn8) * assign35250_e39572) - (assign35250_e39569 * ((locals.var_k1__blk932_dn8 * locals.var_k2__blk933) + (locals.var_k1__blk932 * locals.var_k2__blk933_dn8)))) / (assign35250_e39572 * assign35250_e39572))) + ((((locals.var_keq__blk934_dn8 * assign35250_e39578) + (locals.var_keq__blk934 * (locals.var_inv_k1__blk906_dn8 - locals.var_inv_k2__blk907_dn8))) * locals.var_dx_wi__blk935) + (assign35250_e39579 * locals.var_dx_wi__blk935_dn8))) + (((0.3333333333333 * (-(0.25 * locals.var_keq__blk934_dn8))) * locals.var_dx_wisq__blk936) + (assign35250_e39589 * locals.var_dx_wisq__blk936_dn8)))) / (assign35250_e39592 * assign35250_e39592))), (-((assign35250_e39561 * ((((-(3.0 * locals.var_keq__blk934_dn9)) + ((((12.0 * locals.var_keq__blk934_dn9) * assign35250_e39572) - (assign35250_e39569 * ((locals.var_k1__blk932_dn9 * locals.var_k2__blk933) + (locals.var_k1__blk932 * locals.var_k2__blk933_dn9)))) / (assign35250_e39572 * assign35250_e39572))) + ((((locals.var_keq__blk934_dn9 * assign35250_e39578) + (locals.var_keq__blk934 * (locals.var_inv_k1__blk906_dn9 - locals.var_inv_k2__blk907_dn9))) * locals.var_dx_wi__blk935) + (assign35250_e39579 * locals.var_dx_wi__blk935_dn9))) + (((0.3333333333333 * (-(0.25 * locals.var_keq__blk934_dn9))) * locals.var_dx_wisq__blk936) + (assign35250_e39589 * locals.var_dx_wisq__blk936_dn9)))) / (assign35250_e39592 * assign35250_e39592))),)
    } else {
        (locals.var_dinf__blk974, locals.var_dinf__blk974_dn4, locals.var_dinf__blk974_dn6, locals.var_dinf__blk974_dn7, locals.var_dinf__blk974_dn8, locals.var_dinf__blk974_dn9,)
    }
};
        locals.var_dinf__blk974 = assign35250_e39595;
        locals.var_dinf__blk974_dn4 = assign35250_e39595_d_n4;
        locals.var_dinf__blk974_dn6 = assign35250_e39595_d_n6;
        locals.var_dinf__blk974_dn7 = assign35250_e39595_d_n7;
        locals.var_dinf__blk974_dn8 = assign35250_e39595_d_n8;
        locals.var_dinf__blk974_dn9 = assign35250_e39595_d_n9;

        let (assign35260_e39601, assign35260_e39601_d_n4, assign35260_e39601_d_n6, assign35260_e39601_d_n7, assign35260_e39601_d_n8, assign35260_e39601_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign35260_e39599: f64 = (1.0 / locals.var_dinf__blk974);
        (assign35260_e39599, (-(locals.var_dinf__blk974_dn4 / (locals.var_dinf__blk974 * locals.var_dinf__blk974))), (-(locals.var_dinf__blk974_dn6 / (locals.var_dinf__blk974 * locals.var_dinf__blk974))), (-(locals.var_dinf__blk974_dn7 / (locals.var_dinf__blk974 * locals.var_dinf__blk974))), (-(locals.var_dinf__blk974_dn8 / (locals.var_dinf__blk974 * locals.var_dinf__blk974))), (-(locals.var_dinf__blk974_dn9 / (locals.var_dinf__blk974 * locals.var_dinf__blk974))),)
    } else {
        (locals.var_inv_dinf__blk975, locals.var_inv_dinf__blk975_dn4, locals.var_inv_dinf__blk975_dn6, locals.var_inv_dinf__blk975_dn7, locals.var_inv_dinf__blk975_dn8, locals.var_inv_dinf__blk975_dn9,)
    }
};
        locals.var_inv_dinf__blk975 = assign35260_e39601;
        locals.var_inv_dinf__blk975_dn4 = assign35260_e39601_d_n4;
        locals.var_inv_dinf__blk975_dn6 = assign35260_e39601_d_n6;
        locals.var_inv_dinf__blk975_dn7 = assign35260_e39601_d_n7;
        locals.var_inv_dinf__blk975_dn8 = assign35260_e39601_d_n8;
        locals.var_inv_dinf__blk975_dn9 = assign35260_e39601_d_n9;

        let assign35270_e39604: f64 = if locals.var_qis__blk938 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1158 = assign35270_e39604;

    }
}
